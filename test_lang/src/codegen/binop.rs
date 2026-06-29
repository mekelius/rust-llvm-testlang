use inkwell::{
    IntPredicate::{EQ, NE, SGE, SGT, SLE, SLT},
    values::{AnyValue, AnyValueEnum},
};

use super::CodeGen;
use crate::ast::{BinaryOperator, BinopExpression, Expression};

impl<'ctx> CodeGen<'ctx> {
    pub fn handle_binop(&self, binop: &BinopExpression) -> AnyValueEnum<'ctx> {
        match binop.op {
            BinaryOperator::Equals => self.handle_eq(&binop.lhs.inner, &binop.rhs.inner),
            BinaryOperator::GreaterThan => self.handle_gt(&binop.lhs.inner, &binop.rhs.inner),
            BinaryOperator::LessThan => self.handle_lt(&binop.lhs.inner, &binop.rhs.inner),
            BinaryOperator::GreaterThanOrEquals => {
                self.handle_gteq(&binop.lhs.inner, &binop.rhs.inner)
            }
            BinaryOperator::LessThanOrEquals => {
                self.handle_lteq(&binop.lhs.inner, &binop.rhs.inner)
            }
            BinaryOperator::NotEquals => self.handle_neq(&binop.lhs.inner, &binop.rhs.inner),
            BinaryOperator::And => todo!("binary and"), //self.handle_(&binop.lhs.inner, &binop.rhs.inner),
            BinaryOperator::Or => todo!("binary or"), //self.handle_(&binop.lhs.inner, &binop.rhs.inner),
            BinaryOperator::Add => self.handle_add(&binop.lhs.inner, &binop.rhs.inner),
            BinaryOperator::Sub => self.handle_sub(&binop.lhs.inner, &binop.rhs.inner),
            BinaryOperator::Mul => self.handle_mul(&binop.lhs.inner, &binop.rhs.inner),
            BinaryOperator::Div => self.handle_div(&binop.lhs.inner, &binop.rhs.inner),
            BinaryOperator::Mod => self.handle_mod(&binop.lhs.inner, &binop.rhs.inner),
        }
    }

    pub fn handle_add(&self, lhs: &Expression, rhs: &Expression) -> AnyValueEnum<'ctx> {
        let lhs_value = self.handle_expression(lhs).into_int_value();
        let rhs_value = self.handle_expression(rhs).into_int_value();

        self.ir
            .builder
            .build_int_add(lhs_value, rhs_value, "")
            .unwrap()
            .as_any_value_enum()
    }

    pub fn handle_sub(&self, lhs: &Expression, rhs: &Expression) -> AnyValueEnum<'ctx> {
        let lhs_value = self.handle_expression(lhs).into_int_value();
        let rhs_value = self.handle_expression(rhs).into_int_value();

        self.ir
            .builder
            .build_int_sub(lhs_value, rhs_value, "")
            .unwrap()
            .as_any_value_enum()
    }

    pub fn handle_mul(&self, lhs: &Expression, rhs: &Expression) -> AnyValueEnum<'ctx> {
        let lhs_value = self.handle_expression(lhs).into_int_value();
        let rhs_value = self.handle_expression(rhs).into_int_value();

        self.ir
            .builder
            .build_int_mul(lhs_value, rhs_value, "")
            .unwrap()
            .as_any_value_enum()
    }

    pub fn handle_div(&self, lhs: &Expression, rhs: &Expression) -> AnyValueEnum<'ctx> {
        let lhs_value = self.handle_expression(lhs).into_int_value();
        let rhs_value = self.handle_expression(rhs).into_int_value();

        self.ir
            .builder
            .build_int_signed_div(lhs_value, rhs_value, "")
            .unwrap()
            .as_any_value_enum()
    }

    pub fn handle_mod(&self, lhs: &Expression, rhs: &Expression) -> AnyValueEnum<'ctx> {
        let lhs_value = self.handle_expression(lhs).into_int_value();
        let rhs_value = self.handle_expression(rhs).into_int_value();

        self.ir
            .builder
            .build_int_signed_rem(lhs_value, rhs_value, "")
            .unwrap()
            .as_any_value_enum()
    }

    pub fn handle_eq(&self, lhs: &Expression, rhs: &Expression) -> AnyValueEnum<'ctx> {
        let lhs_value = self.handle_expression(lhs).into_int_value();
        let rhs_value = self.handle_expression(rhs).into_int_value();

        self.ir
            .builder
            .build_int_compare(EQ, lhs_value, rhs_value, "")
            .unwrap()
            .as_any_value_enum()
    }

    pub fn handle_neq(&self, lhs: &Expression, rhs: &Expression) -> AnyValueEnum<'ctx> {
        let lhs_value = self.handle_expression(lhs).into_int_value();
        let rhs_value = self.handle_expression(rhs).into_int_value();

        self.ir
            .builder
            .build_int_compare(NE, lhs_value, rhs_value, "")
            .unwrap()
            .as_any_value_enum()
    }

    pub fn handle_gt(&self, lhs: &Expression, rhs: &Expression) -> AnyValueEnum<'ctx> {
        let lhs_value = self.handle_expression(lhs).into_int_value();
        let rhs_value = self.handle_expression(rhs).into_int_value();

        self.ir
            .builder
            .build_int_compare(SGT, lhs_value, rhs_value, "")
            .unwrap()
            .as_any_value_enum()
    }

    pub fn handle_lt(&self, lhs: &Expression, rhs: &Expression) -> AnyValueEnum<'ctx> {
        let lhs_value = self.handle_expression(lhs).into_int_value();
        let rhs_value = self.handle_expression(rhs).into_int_value();

        self.ir
            .builder
            .build_int_compare(SLT, lhs_value, rhs_value, "")
            .unwrap()
            .as_any_value_enum()
    }

    pub fn handle_gteq(&self, lhs: &Expression, rhs: &Expression) -> AnyValueEnum<'ctx> {
        let lhs_value = self.handle_expression(lhs).into_int_value();
        let rhs_value = self.handle_expression(rhs).into_int_value();

        self.ir
            .builder
            .build_int_compare(SGE, lhs_value, rhs_value, "")
            .unwrap()
            .as_any_value_enum()
    }

    pub fn handle_lteq(&self, lhs: &Expression, rhs: &Expression) -> AnyValueEnum<'ctx> {
        let lhs_value = self.handle_expression(lhs).into_int_value();
        let rhs_value = self.handle_expression(rhs).into_int_value();

        self.ir
            .builder
            .build_int_compare(SLE, lhs_value, rhs_value, "")
            .unwrap()
            .as_any_value_enum()
    }
}
