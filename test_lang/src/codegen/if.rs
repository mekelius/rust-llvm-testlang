use super::CodeGen;
use crate::{
    ast::Statement,
    ast_store::{ASTStore, ExpressionID, StatementID},
    codegen::CodegenError,
};

impl<'ctx> CodeGen<'ctx> {
    pub fn handle_if(
        &mut self,
        ast_store: &ASTStore,
        condition: ExpressionID,
        body: StatementID,
    ) -> Result<(), CodegenError> {
        let condition_value = self.handle_expression(ast_store, condition)?;

        let current_function = self
            .ir
            .builder
            .get_insert_block()
            .unwrap()
            .get_parent()
            .unwrap();

        // Create basic blocks
        let then_block = self
            .ir
            .context
            .append_basic_block(current_function, "if_then");
        let after_block = self
            .ir
            .context
            .append_basic_block(current_function, "if_after");

        // Jump
        self.ir.builder.build_conditional_branch(
            condition_value.into_int_value(),
            then_block,
            after_block,
        )?;

        self.ir.builder.position_at_end(then_block);
        self.handle_statement(ast_store, body)?;

        if !self.ir.at_terminator() {
            self.ir.builder.build_unconditional_branch(after_block)?;
        }

        self.ir.builder.position_at_end(after_block);
        Ok(())
    }

    pub fn handle_if_else(
        &mut self,
        ast_store: &ASTStore,
        if_branch: StatementID,
        else_branch: StatementID,
    ) -> Result<(), CodegenError> {
        let Statement::If {
            condition,
            body: if_branch_body,
        } = ast_store.get_statement(if_branch).inner
        else {
            unreachable!("Encountered if-else-statement with non if-statement first branch");
        };

        let condition_value = self.handle_expression(ast_store, condition)?;

        let current_function = self
            .ir
            .builder
            .get_insert_block()
            .unwrap()
            .get_parent()
            .unwrap();

        // Create basic blocks
        let if_block = self
            .ir
            .context
            .append_basic_block(current_function, "if_branch_body");
        let else_block = self
            .ir
            .context
            .append_basic_block(current_function, "else_branch_body");
        let after_block = self
            .ir
            .context
            .append_basic_block(current_function, "after");

        // Jump
        self.ir
            .builder
            .build_conditional_branch(condition_value.into_int_value(), if_block, else_block)
            .unwrap_or_else(|e| {
                panic!(
                    "Failed to build conditional branch from {:?}, {}",
                    condition, e
                )
            });

        self.ir.builder.position_at_end(if_block);
        self.handle_statement(ast_store, if_branch_body)?;

        self.ir
            .builder
            .build_unconditional_branch(after_block)
            .unwrap();

        self.ir.builder.position_at_end(else_block);

        self.handle_statement(ast_store, else_branch)?;

        if !self.ir.at_terminator() {
            self.ir
                .builder
                .build_unconditional_branch(after_block)
                .unwrap();
        }
        self.ir.builder.position_at_end(after_block);

        Ok(())
    }
}
