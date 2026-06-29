use inkwell::values::{AnyValue, AnyValueEnum, BasicMetadataValueEnum};

use super::CodeGen;
use crate::ast::{Call, Expression, LValue, Literal};

impl<'ctx> CodeGen<'ctx> {
    pub fn handle_expression(&self, expression: &Expression) -> AnyValueEnum<'ctx> {
        match expression {
            Expression::TypedExpression(type_identifier, expression) => {
                self.handle_typed_expresssion(type_identifier, expression)
            }

            Expression::Binop(binop) => self.handle_binop(binop),
            Expression::Unop(unop) => self.handle_unop(unop),
            Expression::Call(call) => self.handle_function_call(&call),
            Expression::LValue(value) => self.handle_lvalue(&*value),
            
            Expression::Literal(Literal::Number(value)) => self.handle_number_literal(&value),
            Expression::Literal(Literal::String(value)) => self.handle_string_literal(&value),
            Expression::Literal(Literal::Boolean(value)) => self.handle_boolean_literal(&value),
            Expression::Literal(Literal::Unit) => {
                panic!("UnitLiteral only allowed as return value atm")
            }
        }
    }

    fn handle_typed_expresssion(
        &self,
        _type_identifier: &str,
        expression: &Expression,
    ) -> AnyValueEnum<'ctx> {
        // let type_ = scopes.resolve_type(type_identifier);
        // TODO: type check here
        self.handle_expression(expression)
    }

    fn handle_function_call(&self, call_expression: &Call) -> AnyValueEnum<'ctx> {
        let Call {
            callee: callee_expression,
            args: argument_list,
        } = call_expression;

        let lvalue: &LValue = match &callee_expression.inner {
            Expression::LValue(lvalue) => lvalue,
            _ => todo!("Callee expressions"),
        };

        let callee_name = match lvalue {
            LValue::Identifier(callee_name) => callee_name,
            _ => todo!("Non identifier lvalues"),
        };

        let args: Vec<BasicMetadataValueEnum> = argument_list
            .into_iter()
            .map(|arg| BasicMetadataValueEnum::try_from(self.handle_expression(arg)).unwrap())
            .collect();

        let callee = self
            .scopes
            .resolve_function(&callee_name)
            .unwrap_or_else(|| panic!("Attempt to call nonexistent function {}", callee_name));

        self.ir
            .builder
            .build_call(callee, &args, "")
            .unwrap()
            .as_any_value_enum()
    }
}
