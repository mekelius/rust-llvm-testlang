use inkwell::{
    types::BasicTypeEnum,
    values::{AnyValue, AnyValueEnum, BasicValueEnum, PointerValue},
};

use super::CodeGen;
use crate::{
    ast::Expression,
    ast_store::{ASTStore, ExpressionID},
    codegen::{CodegenError, helpers::TryIntoOverride, identifier::Symbol},
    types::SimpleType,
};

impl<'ctx> CodeGen<'ctx> {
    pub fn handle_const(
        &mut self,
        ast_store: &ASTStore,
        lvalue_id: ExpressionID,
        expression: ExpressionID,
    ) -> Result<(), CodegenError> {
        let lvalue = ast_store.get_expression(lvalue_id);

        let Expression::Identifier(identifier) = lvalue else {
            todo!("Non-identifier lvalues");
        };

        let value = self.handle_expression(ast_store, expression)?;
        let symbol = Symbol::Value(value);
        let current_scope = self.scopes.get_current_scope_mut();

        match current_scope.identifiers.get(identifier) {
            Some(_) => return Err("Attempt to redefine identifier".into()),
            // NOTE: Only values defined with let atm
            None => current_scope
                .identifiers
                .insert(identifier.to_string(), symbol),
        };

        Ok(())
    }

    pub fn handle_let(
        &mut self,
        ast_store: &ASTStore,
        lvalue_id: ExpressionID,
        expression_id: ExpressionID,
    ) -> Result<(), CodegenError> {
        let lvalue = ast_store.get_expression(lvalue_id);

        let Expression::Identifier(identifier) = lvalue else {
            todo!("Non-identifier lvalues");
        };

        let simple_type = match ast_store.get_expression(expression_id) {
            Expression::TypedExpression(type_identifier, _) => {
                SimpleType::from_type_string(type_identifier)
            }
            _ => SimpleType::Int,
        };
        let ir_type: BasicTypeEnum<'ctx> = self
            .ir
            .simple_type_to_ir_type(simple_type)
            .expect("Void/Unit type not allowed as a variable type")
            .try_into_override()?;
        let value: BasicValueEnum<'ctx> = self
            .handle_expression(ast_store, expression_id)?
            .try_into_override()?;
        let pointer = self.ir.builder.build_alloca(ir_type, identifier)?;

        self.ir.builder.build_store(pointer, value)?;
        self.scopes.define_identifier(
            identifier,
            Symbol::Variable {
                pointer,
                type_: simple_type,
            },
        );

        Ok(())
    }

    pub fn load_variable(
        &self,
        pointer: PointerValue<'ctx>,
        type_: &SimpleType,
    ) -> Result<AnyValueEnum<'ctx>, CodegenError> {
        let ir_type: BasicTypeEnum<'ctx> = self
            .ir
            .simple_type_to_ir_type(*type_)
            .expect("Void/Unit value not allowed to be stored in variables")
            .try_into_override()?;

        Ok(self
            .ir
            .builder
            .build_load(ir_type, pointer, "")?
            .as_any_value_enum())
    }

    pub fn handle_assignment(
        &mut self,
        ast_store: &ASTStore,
        lvalue_id: ExpressionID,
        new_value_expression_id: ExpressionID,
    ) -> Result<(), CodegenError> {
        let lvalue = ast_store.get_expression(lvalue_id);

        let Expression::Identifier(identifier) = lvalue else {
            todo!("Non-identifier lvalues");
        };

        let Symbol::Variable {
            pointer,
            type_: old_type,
        } = self
            .scopes
            .resolve_identifier(identifier)
            .ok_or::<CodegenError>(format!("unknown identifier {}", identifier).into())?
        else {
            return Err(format!("{} is not a variable", identifier).into());
        };

        let pointer = pointer.clone();

        let new_value_expression = ast_store.get_expression(new_value_expression_id);

        let new_type = match new_value_expression {
            Expression::TypedExpression(type_string, _) => {
                SimpleType::from_type_string(type_string)
            }
            _ => SimpleType::Int,
        };

        if new_type != *old_type {
            todo!("Type casts on assignments");
        }
        let new_value: BasicValueEnum<'ctx> = self
            .handle_expression(ast_store, new_value_expression_id)?
            .try_into_override()?;

        self.ir.builder.build_store(pointer, new_value)?;
        Ok(())
    }
}
