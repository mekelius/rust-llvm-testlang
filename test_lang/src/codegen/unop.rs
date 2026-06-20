use inkwell::values::{AnyValue, AnyValueEnum, IntValue};

use crate::{ast::Node, codegen::CodeGen};

impl<'ctx> CodeGen<'ctx> {
    pub fn handle_unary_minus(&self, rhs: &Node) -> AnyValueEnum<'ctx> {
        self.negate_int(self.handle_expression(rhs).into_int_value())
    }

    pub fn negate_int(&self, value: IntValue<'ctx>) -> AnyValueEnum<'ctx> {
        self.ir
            .builder
            .build_int_nsw_sub(self.ir.context.i32_type().const_int(0, false), value, "")
            .unwrap()
            .as_any_value_enum()
    }

    pub fn handle_unary_not(&self, rhs: &Node) -> AnyValueEnum<'ctx> {
        self.ir
            .builder
            .build_not(self.handle_expression(rhs).into_int_value(), "")
            .unwrap()
            .as_any_value_enum()
    }
}
