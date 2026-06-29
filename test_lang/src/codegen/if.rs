use super::CodeGen;
use crate::ast::{Expression, Statement};

impl<'ctx> CodeGen<'ctx> {
    pub fn handle_if(&mut self, condition: &Expression, body: &Statement) {
        let condition_value = self.handle_expression(condition);

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
        self.ir
            .builder
            .build_conditional_branch(condition_value.into_int_value(), then_block, after_block)
            .unwrap_or_else(|e| {
                panic!(
                    "Failed to build conditional branch from {:?}, {}",
                    condition, e
                )
            });

        self.ir.builder.position_at_end(then_block);
        self.handle_statement(body);

        if !self.ir.at_terminator() {
            self.ir
                .builder
                .build_unconditional_branch(after_block)
                .unwrap();
        }
        self.ir.builder.position_at_end(after_block);
    }

    pub fn handle_if_else(&mut self, if_branch: &Statement, else_branch: &Statement) {
        let Statement::If {
            condition,
            body: if_branch_body,
        } = if_branch
        else {
            unreachable!("Encountered if-else-statement with non if-statement first branch");
        };

        let condition_value = self.handle_expression(condition);

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
        self.handle_statement(if_branch_body);

        self.ir
            .builder
            .build_unconditional_branch(after_block)
            .unwrap();

        self.ir.builder.position_at_end(else_block);

        self.handle_statement(else_branch);

        if !self.ir.at_terminator() {
            self.ir
                .builder
                .build_unconditional_branch(after_block)
                .unwrap();
        }
        self.ir.builder.position_at_end(after_block);
    }
}
