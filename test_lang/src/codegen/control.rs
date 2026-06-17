use super::CodeGen;
use crate::{ast::Node, codegen::scope::Scopes};

impl<'ctx> CodeGen<'ctx> {
    pub fn handle_if(&self, condition: &Node, body: &Node, scopes: &mut Scopes<'ctx>) {
        let condition_value = self.handle_expression(condition, scopes);

        let current_function = self
            .builder
            .get_insert_block()
            .unwrap()
            .get_parent()
            .unwrap();
        let then_block = self.context.append_basic_block(current_function, "if_then");
        // let else_ = self.context.append_basic_block(current_function, "if_else");
        let after_block = self
            .context
            .append_basic_block(current_function, "if_after");
        self.builder
            .build_conditional_branch(condition_value.into_int_value(), then_block, after_block)
            .unwrap_or_else(|e| {
                panic!(
                    "Failed to build conditional branch from {:?}, {}",
                    condition, e
                )
            });

        self.builder.position_at_end(then_block);
        self.handle_statement(body, scopes);

        self.builder
            .build_unconditional_branch(after_block)
            .unwrap();
        self.builder.position_at_end(after_block);
    }

    pub fn handle_while(&self, condition: &Node, body: &Node, scopes: &mut Scopes<'ctx>) {
        let from_block = self.builder.get_insert_block().unwrap();
        let current_function = from_block.get_parent().unwrap();

        let loop_header = self
            .context
            .append_basic_block(current_function, "loop_header");
        let loop_body = self
            .context
            .append_basic_block(current_function, "loop_body");
        let after_block = self
            .context
            .append_basic_block(current_function, "loop_after");

        self.builder.build_unconditional_branch(loop_header).unwrap();

        self.builder.position_at_end(loop_header);
        let condition_value = self.handle_expression(condition, scopes);
        self.builder
            .build_conditional_branch(condition_value.into_int_value(), loop_body, after_block)
            .unwrap_or_else(|e| {
                panic!(
                    "Failed to build conditional branch from {:?}, {}",
                    condition, e
                )
            });

        self.builder.position_at_end(loop_body);
        self.handle_statement(body, scopes);
        self.builder.build_unconditional_branch(loop_header).unwrap();

        self.builder.position_at_end(after_block);
    }

    pub fn handle_for(&self, _init: &Node, _condition: &Node, _step: &Node, _body: &Node) {
        todo!("For loops");
    }
}
