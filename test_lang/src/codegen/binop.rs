use inkwell::{
    IntPredicate::{EQ, NE, SGE, SGT, SLE, SLT},
    values::{AnyValue, AnyValueEnum},
};

use super::CodeGen;
use crate::{
    ast::{BinaryOperator, BinopExpression},
    ast_store::ASTStore,
    codegen::CodegenError,
};

impl<'ctx> CodeGen<'ctx> {
    pub fn handle_binop(
        &mut self,
        ast_store: &ASTStore,
        binop: &BinopExpression,
    ) -> Result<AnyValueEnum<'ctx>, CodegenError> {
        let lhs_value = self.handle_expression(ast_store, binop.lhs)?;
        let rhs_value = self.handle_expression(ast_store, binop.rhs)?;

        Ok(match binop.op {
            BinaryOperator::Equals => self.handle_eq(lhs_value, rhs_value),
            BinaryOperator::GreaterThan => self.handle_gt(lhs_value, rhs_value),
            BinaryOperator::LessThan => self.handle_lt(lhs_value, rhs_value),
            BinaryOperator::GreaterThanOrEquals => self.handle_gteq(lhs_value, rhs_value),
            BinaryOperator::LessThanOrEquals => self.handle_lteq(lhs_value, rhs_value),
            BinaryOperator::NotEquals => self.handle_neq(lhs_value, rhs_value),
            BinaryOperator::And => todo!("binary and"), //self.handle_(lhs_value, rhs_value),
            BinaryOperator::Or => todo!("binary or"),   //self.handle_(lhs_value, rhs_value),
            BinaryOperator::Add => self.handle_add(lhs_value, rhs_value),
            BinaryOperator::Sub => self.handle_sub(lhs_value, rhs_value),
            BinaryOperator::Mul => self.handle_mul(lhs_value, rhs_value),
            BinaryOperator::Div => self.handle_div(lhs_value, rhs_value),
            BinaryOperator::Mod => self.handle_mod(lhs_value, rhs_value),
        })
    }

    pub fn handle_add(
        &self,
        lhs: AnyValueEnum<'ctx>,
        rhs: AnyValueEnum<'ctx>,
    ) -> AnyValueEnum<'ctx> {
        self.ir
            .builder
            .build_int_add(lhs.into_int_value(), rhs.into_int_value(), "")
            .unwrap()
            .as_any_value_enum()
    }

    pub fn handle_sub(
        &self,
        lhs: AnyValueEnum<'ctx>,
        rhs: AnyValueEnum<'ctx>,
    ) -> AnyValueEnum<'ctx> {
        self.ir
            .builder
            .build_int_sub(lhs.into_int_value(), rhs.into_int_value(), "")
            .unwrap()
            .as_any_value_enum()
    }

    pub fn handle_mul(
        &self,
        lhs: AnyValueEnum<'ctx>,
        rhs: AnyValueEnum<'ctx>,
    ) -> AnyValueEnum<'ctx> {
        self.ir
            .builder
            .build_int_mul(lhs.into_int_value(), rhs.into_int_value(), "")
            .unwrap()
            .as_any_value_enum()
    }

    pub fn handle_div(
        &self,
        lhs: AnyValueEnum<'ctx>,
        rhs: AnyValueEnum<'ctx>,
    ) -> AnyValueEnum<'ctx> {
        self.ir
            .builder
            .build_int_signed_div(lhs.into_int_value(), rhs.into_int_value(), "")
            .unwrap()
            .as_any_value_enum()
    }

    pub fn handle_mod(
        &self,
        lhs: AnyValueEnum<'ctx>,
        rhs: AnyValueEnum<'ctx>,
    ) -> AnyValueEnum<'ctx> {
        self.ir
            .builder
            .build_int_signed_rem(lhs.into_int_value(), rhs.into_int_value(), "")
            .unwrap()
            .as_any_value_enum()
    }

    pub fn handle_eq(
        &self,
        lhs: AnyValueEnum<'ctx>,
        rhs: AnyValueEnum<'ctx>,
    ) -> AnyValueEnum<'ctx> {
        self.ir
            .builder
            .build_int_compare(EQ, lhs.into_int_value(), rhs.into_int_value(), "")
            .unwrap()
            .as_any_value_enum()
    }

    pub fn handle_neq(
        &self,
        lhs: AnyValueEnum<'ctx>,
        rhs: AnyValueEnum<'ctx>,
    ) -> AnyValueEnum<'ctx> {
        self.ir
            .builder
            .build_int_compare(NE, lhs.into_int_value(), rhs.into_int_value(), "")
            .unwrap()
            .as_any_value_enum()
    }

    pub fn handle_gt(
        &self,
        lhs: AnyValueEnum<'ctx>,
        rhs: AnyValueEnum<'ctx>,
    ) -> AnyValueEnum<'ctx> {
        self.ir
            .builder
            .build_int_compare(SGT, lhs.into_int_value(), rhs.into_int_value(), "")
            .unwrap()
            .as_any_value_enum()
    }

    pub fn handle_lt(
        &self,
        lhs: AnyValueEnum<'ctx>,
        rhs: AnyValueEnum<'ctx>,
    ) -> AnyValueEnum<'ctx> {
        self.ir
            .builder
            .build_int_compare(SLT, lhs.into_int_value(), rhs.into_int_value(), "")
            .unwrap()
            .as_any_value_enum()
    }

    pub fn handle_gteq(
        &self,
        lhs: AnyValueEnum<'ctx>,
        rhs: AnyValueEnum<'ctx>,
    ) -> AnyValueEnum<'ctx> {
        self.ir
            .builder
            .build_int_compare(SGE, lhs.into_int_value(), rhs.into_int_value(), "")
            .unwrap()
            .as_any_value_enum()
    }

    pub fn handle_lteq(
        &self,
        lhs: AnyValueEnum<'ctx>,
        rhs: AnyValueEnum<'ctx>,
    ) -> AnyValueEnum<'ctx> {
        self.ir
            .builder
            .build_int_compare(SLE, lhs.into_int_value(), rhs.into_int_value(), "")
            .unwrap()
            .as_any_value_enum()
    }
}
