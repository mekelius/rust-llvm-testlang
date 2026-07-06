use inkwell::{basic_block::BasicBlock, types::StringRadix::Decimal, values::IntValue};

use crate::{
    ast::{Case, DEFAULT_CASE, Statement},
    ast_store::{ASTStore, ExpressionID, StatementID},
    codegen::{CodeGen, CodegenError},
    span::SourceIDSpanned,
};

struct HandleCasesReturn<'ctx> {
    cases: Vec<(String, BasicBlock<'ctx>)>,
    default_block: BasicBlock<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    /** Returns true if the block all_cases_returned */
    pub fn handle_switch(
        &mut self,
        ast_store: &ASTStore,
        matched_value_expression: ExpressionID,
        body: &Vec<SourceIDSpanned<Case>>,
    ) -> Result<(), CodegenError> {
        let matched_value = self.handle_expression(ast_store, matched_value_expression)?;

        self.scopes.push_new_scope();

        let entry_block = self
            .ir
            .builder
            .get_insert_block()
            .expect("statement should have an insert block");
        let current_function = entry_block
            .get_parent()
            .expect("statement should be inside a function");

        // TODO: Check the type of the matched value

        // Create basic blocks
        let after_block = self
            .ir
            .context
            .append_basic_block(current_function, "after");

        let HandleCasesReturn {
            cases,
            default_block,
        } = self.handle_cases(ast_store, body, after_block)?;

        self.ir.builder.position_at_end(entry_block);

        let cases = cases
            .iter()
            .map(|(case_string, case_block)| {
                Ok((
                    self.ir
                        .context
                        .i32_type()
                        .const_int_from_string(&case_string, Decimal)
                        .ok_or_else(|| format!("{} does not produce an Int", case_string))?,
                    *case_block,
                ))
            })
            .collect::<Result<Vec<(IntValue<'ctx>, BasicBlock<'ctx>)>, CodegenError>>()?;

        let matched_value = matched_value.into_int_value();

        self.ir
            .builder
            .build_switch(matched_value, default_block, &cases)?;

        self.ir.builder.position_at_end(after_block);
        self.scopes.pop_scope();

        Ok(())
    }

    /** Returned default block will be the after_block if no default case was encountered */
    fn handle_cases<'a>(
        &mut self,
        ast_store: &ASTStore,
        cases_body: &Vec<SourceIDSpanned<Case>>,
        after_block: BasicBlock<'ctx>,
    ) -> Result<HandleCasesReturn<'ctx>, CodegenError> {
        let entry_block = self
            .ir
            .builder
            .get_insert_block()
            .expect("statement should have an insert block");
        let current_function = entry_block
            .get_parent()
            .expect("statement should be inside a function");
        let mut next_block = self.ir.context.append_basic_block(current_function, "case");
        let mut default_block = None;
        let mut cases = Vec::<(String, BasicBlock)>::new();

        for case in cases_body {
            let case_block = next_block;
            next_block = self.ir.context.append_basic_block(current_function, "case");

            match &case.inner {
                Case {
                    matched_value: Some(matched_value),
                    body,
                } => {
                    cases.push((matched_value.inner.clone(), case_block));
                    self.ir.builder.position_at_end(case_block);
                    self.handle_case(ast_store, body, &next_block, &after_block)?;
                }
                Case {
                    matched_value: DEFAULT_CASE,
                    body,
                } => {
                    // Check no duplicate default
                    if default_block.is_some() {
                        panic!("Multiple default cases in one switch statement");
                    }

                    default_block = Some(case_block.clone());
                    self.ir.builder.position_at_end(case_block);
                    self.handle_case(ast_store, body, &next_block, &after_block)?;
                }
            };
        }

        // Slightly hacky way to implement fallthrough from cases
        next_block.replace_all_uses_with(&after_block);
        unsafe {
            next_block
                .delete()
                .map_err(|_| "deleting basic block failed")?;
        }

        Ok(HandleCasesReturn {
            cases,
            default_block: default_block.unwrap_or(after_block),
        })
    }

    fn handle_case(
        &mut self,
        ast_store: &ASTStore,
        body: &Vec<StatementID>,
        next_block: &BasicBlock,
        after_block: &BasicBlock,
    ) -> Result<(), CodegenError> {
        for statement_id in body {
            match ast_store.get_statement(*statement_id).inner {
                Statement::Break => {
                    self.ir.builder.build_unconditional_branch(*after_block)?;
                    return Ok(());
                }
                _ => self.handle_statement(ast_store, *statement_id)?,
            }
        }

        self.ir.builder.build_unconditional_branch(*next_block)?;
        Ok(())
    }
}
