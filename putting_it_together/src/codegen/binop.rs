use inkwell::values::{AnyValue, AnyValueEnum};

use crate::ast::Node;
use super::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub fn handle_add(&self, lhs: &Node, rhs: &Node) -> AnyValueEnum<'_> {
        let lhs_value = self.handle_expression(lhs).into_int_value();
        let rhs_value = self.handle_expression(rhs).into_int_value();

        self.builder
            .build_int_add(lhs_value, rhs_value, "")
            .unwrap()
            .as_any_value_enum()
    }

    pub fn handle_subtr(&self, lhs: &Node, rhs: &Node) -> AnyValueEnum<'_> {
        let lhs_value = self.handle_expression(lhs).into_int_value();
        let rhs_value = self.handle_expression(rhs).into_int_value();

        self.builder
            .build_int_sub(lhs_value, rhs_value, "")
            .unwrap()
            .as_any_value_enum()
    }

    pub fn handle_mult(&self, lhs: &Node, rhs: &Node) -> AnyValueEnum<'_> {
        let lhs_value = self.handle_expression(lhs).into_int_value();
        let rhs_value = self.handle_expression(rhs).into_int_value();

        self.builder
            .build_int_mul(lhs_value, rhs_value, "")
            .unwrap()
            .as_any_value_enum()
    }

    pub fn handle_div(&self, lhs: &Node, rhs: &Node) -> AnyValueEnum<'_> {
        let lhs_value = self.handle_expression(lhs).into_int_value();
        let rhs_value = self.handle_expression(rhs).into_int_value();

        self.builder
            .build_int_signed_div(lhs_value, rhs_value, "")
            .unwrap()
            .as_any_value_enum()
    }
}