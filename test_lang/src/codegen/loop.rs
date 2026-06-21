use super::CodeGen;
use crate::ast::Node;

impl<'ctx> CodeGen<'ctx> {
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
