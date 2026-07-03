use std::error::Error;

use inkwell::values::{AnyValue, AnyValueEnum, BasicMetadataValueEnum};

use super::CodeGen;
use crate::{
    ast::{
        Call,
        Expression::{self},
        Literal,
    },
    ast_store::ExpressionID,
    span::SourceIDSpanned,
};

impl<'ctx> CodeGen<'ctx> {
    pub fn enter_expression(
        &mut self,
        (expression, expression_id): (&SourceIDSpanned<Expression>, ExpressionID),
    ) -> Option<Box<dyn Error>> {
        match &expression.inner {
            // Expression::TypedExpression(type_identifier, expression) => {
            //     self.handle_typed_expresssion(type_identifier, expression)
            // }

            // Expression::Binop(binop) => self.handle_binop(binop),
            // Expression::Unop(unop) => self.handle_unop(unop),
            // Expression::Call(call) => self.handle_function_call(&call),
            // Expression::Identifier(value) => self.handle_identifier(&value),
            Expression::PropertyAccess(_) => todo!("Dot access expressions"),

            Expression::Literal(Literal::Number(value)) => self
                .ir
                .expression_stack
                .push(Some(self.handle_number_literal(&value))),
            Expression::Literal(Literal::String(value)) => self
                .ir
                .expression_stack
                .push(Some(self.handle_string_literal(&value))),
            Expression::Literal(Literal::Boolean(value)) => self
                .ir
                .expression_stack
                .push(Some(self.handle_boolean_literal(&value))),
            Expression::Literal(Literal::Unit) => {
                panic!("UnitLiteral only allowed as return value atm")
            }

            _ => todo!("other expression types"),
        };

        None
    }

    // fn handle_typed_expresssion(
    //     &self,
    //     _type_identifier: &str,
    //     expression: &Expression,
    // ) -> AnyValueEnum<'ctx> {
    //     // let type_ = scopes.resolve_type(type_identifier);
    //     // TODO: type check here
    //     self.handle_expression(expression)
    // }

    // fn handle_function_call(&self, call_expression: &Call) -> AnyValueEnum<'ctx> {
    //     let Call {
    //         callee: callee_expression,
    //         args: argument_list,
    //     } = call_expression;

    //     let Expression::Identifier(callee_name) = &callee_expression.inner else {
    //         todo!("Callee expressions")
    //     };

    //     let args: Vec<BasicMetadataValueEnum> = argument_list
    //         .into_iter()
    //         .map(|arg| BasicMetadataValueEnum::try_from(self.handle_expression(arg)).unwrap())
    //         .collect();

    //     let callee = self
    //         .scopes
    //         .resolve_function(&callee_name)
    //         .unwrap_or_else(|| panic!("Attempt to call nonexistent function {}", callee_name));

    //     self.ir
    //         .builder
    //         .build_call(callee, &args, "")
    //         .unwrap()
    //         .as_any_value_enum()
    // }
}
