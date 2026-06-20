use inkwell::{
    IntPredicate::{EQ, NE, SGE, SGT, SLE, SLT},
    values::{AnyValue, AnyValueEnum},
};

use super::CodeGen;
use crate::ast::Node;

impl<'ctx> CodeGen<'ctx> {
    pub fn handle_add(
        &self,
        lhs: &Node,
        rhs: &Node,
    ) -> AnyValueEnum<'ctx> {
        let lhs_value = self.handle_expression(lhs).into_int_value();
        let rhs_value = self.handle_expression(rhs).into_int_value();

        self.ir
            .builder
            .build_int_add(lhs_value, rhs_value, "")
            .unwrap()
            .as_any_value_enum()
    }

    pub fn handle_sub(
        &self,
        lhs: &Node,
        rhs: &Node,
    ) -> AnyValueEnum<'ctx> {
        let lhs_value = self.handle_expression(lhs).into_int_value();
        let rhs_value = self.handle_expression(rhs).into_int_value();

        self.ir
            .builder
            .build_int_sub(lhs_value, rhs_value, "")
            .unwrap()
            .as_any_value_enum()
    }

    pub fn handle_mul(
        &self,
        lhs: &Node,
        rhs: &Node,
    ) -> AnyValueEnum<'ctx> {
        let lhs_value = self.handle_expression(lhs).into_int_value();
        let rhs_value = self.handle_expression(rhs).into_int_value();

        self.ir
            .builder
            .build_int_mul(lhs_value, rhs_value, "")
            .unwrap()
            .as_any_value_enum()
    }

    pub fn handle_div(
        &self,
        lhs: &Node,
        rhs: &Node,
    ) -> AnyValueEnum<'ctx> {
        let lhs_value = self.handle_expression(lhs).into_int_value();
        let rhs_value = self.handle_expression(rhs).into_int_value();

        self.ir
            .builder
            .build_int_signed_div(lhs_value, rhs_value, "")
            .unwrap()
            .as_any_value_enum()
    }

    pub fn handle_mod(
        &self,
        lhs: &Node,
        rhs: &Node,
    ) -> AnyValueEnum<'ctx> {
        let lhs_value = self.handle_expression(lhs).into_int_value();
        let rhs_value = self.handle_expression(rhs).into_int_value();

        self.ir
            .builder
            .build_int_signed_rem(lhs_value, rhs_value, "")
            .unwrap()
            .as_any_value_enum()
    }

    pub fn handle_eq(
        &self,
        lhs: &Node,
        rhs: &Node,
    ) -> AnyValueEnum<'ctx> {
        let lhs_value = self.handle_expression(lhs).into_int_value();
        let rhs_value = self.handle_expression(rhs).into_int_value();

        self.ir
            .builder
            .build_int_compare(EQ, lhs_value, rhs_value, "")
            .unwrap()
            .as_any_value_enum()
    }

    pub fn handle_neq(
        &self,
        lhs: &Node,
        rhs: &Node,
    ) -> AnyValueEnum<'ctx> {
        let lhs_value = self.handle_expression(lhs).into_int_value();
        let rhs_value = self.handle_expression(rhs).into_int_value();

        self.ir
            .builder
            .build_int_compare(NE, lhs_value, rhs_value, "")
            .unwrap()
            .as_any_value_enum()
    }

    pub fn handle_gt(
        &self,
        lhs: &Node,
        rhs: &Node,
    ) -> AnyValueEnum<'ctx> {
        let lhs_value = self.handle_expression(lhs).into_int_value();
        let rhs_value = self.handle_expression(rhs).into_int_value();

        self.ir
            .builder
            .build_int_compare(SGT, lhs_value, rhs_value, "")
            .unwrap()
            .as_any_value_enum()
    }

    pub fn handle_lt(
        &self,
        lhs: &Node,
        rhs: &Node,
    ) -> AnyValueEnum<'ctx> {
        let lhs_value = self.handle_expression(lhs).into_int_value();
        let rhs_value = self.handle_expression(rhs).into_int_value();

        self.ir
            .builder
            .build_int_compare(SLT, lhs_value, rhs_value, "")
            .unwrap()
            .as_any_value_enum()
    }

    pub fn handle_gteq(
        &self,
        lhs: &Node,
        rhs: &Node,
    ) -> AnyValueEnum<'ctx> {
        let lhs_value = self.handle_expression(lhs).into_int_value();
        let rhs_value = self.handle_expression(rhs).into_int_value();

        self.ir
            .builder
            .build_int_compare(SGE, lhs_value, rhs_value, "")
            .unwrap()
            .as_any_value_enum()
    }

    pub fn handle_lteq(
        &self,
        lhs: &Node,
        rhs: &Node,
    ) -> AnyValueEnum<'ctx> {
        let lhs_value = self.handle_expression(lhs).into_int_value();
        let rhs_value = self.handle_expression(rhs).into_int_value();

        self.ir
            .builder
            .build_int_compare(SLE, lhs_value, rhs_value, "")
            .unwrap()
            .as_any_value_enum()
    }
}
