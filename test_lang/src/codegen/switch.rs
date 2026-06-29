use chumsky::span::Spanned;
use inkwell::{basic_block::BasicBlock, types::StringRadix::Decimal, values::IntValue};

use crate::{
    ast::{Case, DEFAULT_CASE, Expression, Statement},
    codegen::CodeGen,
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
        matched_value_expression: &Expression,
        body: &Vec<SourceIDSpanned<Case>>,
    ) {
        let matched_value = self.handle_expression(matched_value_expression);

        self.scopes.push_new_scope();

        let entry_block = self.ir.builder.get_insert_block().unwrap();
        let current_function = entry_block.get_parent().unwrap();

        // TODO: Check the type of the matched value

        // Create basic blocks
        let after_block = self
            .ir
            .context
            .append_basic_block(current_function, "after");

        let HandleCasesReturn {
            cases,
            default_block,
        } = self.handle_cases(body, after_block);

        self.ir.builder.position_at_end(entry_block);

        let cases: Vec<(IntValue<'ctx>, BasicBlock<'ctx>)> = cases
            .iter()
            .map(|(case_string, case_block)| {
                (
                    self.ir
                        .context
                        .i32_type()
                        .const_int_from_string(&case_string, Decimal)
                        .unwrap(),
                    *case_block,
                )
            })
            .collect();

        let matched_value = matched_value.into_int_value();

        self.ir
            .builder
            .build_switch(matched_value, default_block, &cases)
            .unwrap();

        self.ir.builder.position_at_end(after_block);
        self.scopes.pop_scope();
    }

    /** Returned default block will be the after_block if no default case was encountered */
    fn handle_cases<'a>(
        &mut self,
        cases_body: &Vec<SourceIDSpanned<Case>>,
        after_block: BasicBlock<'ctx>,
    ) -> HandleCasesReturn<'ctx> {
        let entry_block = self.ir.builder.get_insert_block().unwrap();
        let current_function = entry_block.get_parent().unwrap();
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
                    self.handle_case(&body, &next_block, &after_block);
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
                    self.handle_case(&body, &next_block, &after_block);
                }
            };
        }

        // Slightly hacky way to implement fallthrough from cases
        next_block.replace_all_uses_with(&after_block);
        unsafe {
            next_block.delete().unwrap();
        }

        HandleCasesReturn {
            cases,
            default_block: default_block.unwrap_or(after_block),
        }
    }

    fn handle_case<S>(
        &mut self,
        body: &Vec<Spanned<Statement, S>>,
        next_block: &BasicBlock,
        after_block: &BasicBlock,
    ) {
        for statement in body {
            match statement.inner {
                Statement::BreakStatement => {
                    self.ir
                        .builder
                        .build_unconditional_branch(*after_block)
                        .unwrap();
                    return;
                }
                _ => self.handle_statement(&statement),
            }
        }

        self.ir
            .builder
            .build_unconditional_branch(*next_block)
            .unwrap();
    }
}
