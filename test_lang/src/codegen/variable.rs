use inkwell::{
    types::BasicTypeEnum,
    values::{AnyValue, AnyValueEnum, BasicValueEnum, PointerValue},
};

use super::CodeGen;
use crate::{
    ast::{Expression, LValue},
    codegen::identifier::Symbol,
    types::SimpleType,
};

impl<'ctx> CodeGen<'ctx> {
    pub fn handle_const(&mut self, lvalue: &LValue, expression: &Expression) {
        let LValue::Identifier(identifier) = lvalue else {
            todo!("Non-identifier lvalues");
        };

        let value = self.handle_expression(expression);
        let symbol = Symbol::Value(value);
        let current_scope = self.scopes.get_current_scope_mut();

        match current_scope.identifiers.get(identifier) {
            Some(_) => panic!("Attempting to redefine identifier \"{}\"", identifier),
            // NOTE: Only values defined with let atm
            None => current_scope
                .identifiers
                .insert(identifier.to_string(), symbol),
        };
    }

    pub fn handle_let(&mut self, lvalue: &LValue, expression: &Expression) {
        let LValue::Identifier(identifier) = lvalue else {
            todo!("Non-identifier lvalues");
        };

        let simple_type = match expression {
            Expression::TypedExpression(type_identifier, _) => {
                SimpleType::from_type_string(type_identifier)
            }
            _ => SimpleType::Int,
        };
        let ir_type: BasicTypeEnum<'ctx> = self
            .ir
            .simple_type_to_ir_type(simple_type)
            .expect("Void/Unit type not allowed as a variable type")
            .try_into()
            .unwrap();
        let value: BasicValueEnum<'ctx> = self.handle_expression(expression).try_into().unwrap();
        let pointer = self.ir.builder.build_alloca(ir_type, identifier).unwrap();
        self.ir.builder.build_store(pointer, value).unwrap();
        self.scopes.define_identifier(
            identifier,
            Symbol::Variable {
                pointer,
                type_: simple_type,
            },
        );
    }

    pub fn load_variable(
        &self,
        pointer: PointerValue<'ctx>,
        type_: &SimpleType,
    ) -> AnyValueEnum<'ctx> {
        let ir_type: BasicTypeEnum<'ctx> = self
            .ir
            .simple_type_to_ir_type(*type_)
            .expect("Void/Unit value not allowed to be stored in variables")
            .try_into()
            .unwrap();
        self.ir
            .builder
            .build_load(ir_type, pointer, "")
            .unwrap()
            .as_any_value_enum()
    }

    pub fn handle_assignment(&self, lvalue: &LValue, new_value_expression: &Expression) {
        let LValue::Identifier(identifier) = lvalue else {
            todo!("Non-identifier lvalues");
        };

        let Symbol::Variable {
            pointer,
            type_: old_type,
        } = self.scopes.resolve_identifier(identifier).unwrap()
        else {
            panic!();
        };

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
            .handle_expression(new_value_expression)
            .try_into()
            .unwrap();
        self.ir.builder.build_store(*pointer, new_value).unwrap();
    }
}
