use inkwell::{
    types::BasicTypeEnum,
    values::{AnyValue, BasicValueEnum},
};

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

    pub fn handle_for(&mut self, init: &Node, condition: &Node, step: &Node, body: &Node) {        
        let from_block = self.ir.builder.get_insert_block().unwrap();
        let current_function = from_block.get_parent().unwrap();

        self.scopes.push_new_scope();

        let header_block = self
            .ir
            .context
            .append_basic_block(current_function, "loop_condition");
        let body_block = self
            .ir
            .context
            .append_basic_block(current_function, "loop_body");
        let step_block = self
            .ir
            .context
            .append_basic_block(current_function, "loop_step");
        let after_block = self
            .ir
            .context
            .append_basic_block(current_function, "loop_after");

        // Loop init
        self.handle_statement(init);
        let condition_value_initial = self.handle_expression(condition);
        self.ir
            .builder
            .build_unconditional_branch(header_block)
            .unwrap();

        // Loop step
        self.ir.builder.position_at_end(step_block);
        self.handle_statement(step);
        let condition_value_updated = self.handle_expression(condition);
        self.ir
            .builder
            .build_unconditional_branch(header_block)
            .unwrap();

        // Loop header
        self.ir.builder.position_at_end(header_block);

        let condition_value_initial: BasicValueEnum<'ctx> =
            condition_value_initial.try_into().unwrap();
        let condition_value_updated: BasicValueEnum<'ctx> =
            condition_value_updated.try_into().unwrap();
        let phi_type: BasicTypeEnum<'ctx> = condition_value_initial.get_type().try_into().unwrap();
        let phi = self
            .ir
            .builder
            .build_phi(phi_type, "for_loop_condition")
            .unwrap();
        phi.add_incoming(&[
            (&condition_value_initial, from_block),
            (&condition_value_updated, step_block),
        ]);
        self.ir
            .builder
            .build_conditional_branch(
                phi.as_any_value_enum().into_int_value(),
                body_block,
                after_block,
            )
            .unwrap();

        // Loop body
        self.ir.builder.position_at_end(body_block);
        self.handle_statement(body);
        self.ir.builder.build_unconditional_branch(step_block).unwrap();

        // clean up
        self.ir.builder.position_at_end(after_block);
        self.scopes.pop_scope();
    }
}
