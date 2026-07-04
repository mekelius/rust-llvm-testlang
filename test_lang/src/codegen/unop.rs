use inkwell::values::{AnyValue, AnyValueEnum, IntValue};

use crate::{
    ast::{UnaryOperator, UnopExpression},
    ast_store::{ASTStore, ExpressionID},
    codegen::{CodeGen, CodegenError},
};

impl<'ctx> CodeGen<'ctx> {
    pub fn handle_unop(
        &mut self,
        ast_store: &ASTStore,
        UnopExpression { op, term }: &UnopExpression,
    ) -> Result<AnyValueEnum<'ctx>, CodegenError> {
        match op {
            UnaryOperator::UnaryMinus => self.handle_unary_minus(ast_store, *term),
            UnaryOperator::UnaryNot => self.handle_unary_not(ast_store, *term),
        }
    }

    pub fn handle_unary_minus(
        &mut self,
        ast_store: &ASTStore,
        rhs: ExpressionID,
    ) -> Result<AnyValueEnum<'ctx>, CodegenError> {
        let term = self.handle_expression(ast_store, rhs)?;
        self.negate_int(term.into_int_value())
    }

    pub fn negate_int(&self, value: IntValue<'ctx>) -> Result<AnyValueEnum<'ctx>, CodegenError> {
        let value = self.ir.builder.build_int_nsw_sub(
            self.ir.context.i32_type().const_int(0, false),
            value,
            "",
        )?;

        Ok(value.as_any_value_enum())
    }

    pub fn handle_unary_not(
        &mut self,
        ast_store: &ASTStore,
        rhs: ExpressionID,
    ) -> Result<AnyValueEnum<'ctx>, CodegenError> {
        let term = self.handle_expression(ast_store, rhs)?;

        let value = self.ir.builder.build_not(term.into_int_value(), "")?;

        Ok(value.as_any_value_enum())
    }
}
