use inkwell::basic_block::BasicBlock;

use super::CodeGen;
use crate::ast::Node;

impl<'ctx> CodeGen<'ctx> {
    pub fn handle_if(&mut self, condition: &Node, body: &Node) {
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
        let returned = self.handle_statement(body);

        if !returned {
            self.ir
                .builder
                .build_unconditional_branch(after_block)
                .unwrap();
        }
        self.ir.builder.position_at_end(after_block);
    }

    pub fn handle_if_else(&mut self, if_branch: &Node, else_branch: &Node) {
        let Node::IfStatement {
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

        let returned = self.handle_statement(else_branch);

        if !returned {
            self.ir.builder.build_unconditional_branch(after_block).unwrap();
        }
        self.ir.builder.position_at_end(after_block);
    }

    pub fn handle_while(&mut self, condition: &Node, body: &Node) {
        let from_block = self.ir.builder.get_insert_block().unwrap();
        let current_function = from_block.get_parent().unwrap();

        let loop_header = self
            .ir
            .context
            .append_basic_block(current_function, "loop_header");
        let loop_body = self
            .ir
            .context
            .append_basic_block(current_function, "loop_body");
        let after_block = self
            .ir
            .context
            .append_basic_block(current_function, "loop_after");

        self.ir
            .builder
            .build_unconditional_branch(loop_header)
            .unwrap();

        self.ir.builder.position_at_end(loop_header);
        let condition_value = self.handle_expression(condition);
        self.ir
            .builder
            .build_conditional_branch(condition_value.into_int_value(), loop_body, after_block)
            .unwrap_or_else(|e| {
                panic!(
                    "Failed to build conditional branch from {:?}, {}",
                    condition, e
                )
            });

        self.ir.builder.position_at_end(loop_body);
        self.handle_statement(body);
        self.ir
            .builder
            .build_unconditional_branch(loop_header)
            .unwrap();

        self.ir.builder.position_at_end(after_block);
    }

    pub fn handle_for(&self, _init: &Node, _condition: &Node, _step: &Node, _body: &Node) {
        todo!("For loops");
    }
}
