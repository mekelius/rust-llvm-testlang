use inkwell::values::{AnyValue, AnyValueEnum, BasicMetadataValueEnum};

use super::CodeGen;
use crate::ast::Node;

impl<'ctx> CodeGen<'ctx> {
    pub fn handle_expression(&self, expression: &Node) -> AnyValueEnum<'ctx> {
        match expression {
            Node::TypedExpression(type_identifier, expression) => {
                self.handle_typed_expresssion(type_identifier, expression)
            }

            Node::Equals(lhs, rhs) => self.handle_eq(lhs, rhs),
            Node::NotEquals(lhs, rhs) => self.handle_neq(lhs, rhs),
            Node::GreaterThan(lhs, rhs) => self.handle_gt(lhs, rhs),
            Node::LessThan(lhs, rhs) => self.handle_lt(lhs, rhs),
            Node::GreaterThanOrEquals(lhs, rhs) => self.handle_gteq(lhs, rhs),
            Node::LessThanOrEquals(lhs, rhs) => self.handle_lteq(lhs, rhs),

            Node::Mul(lhs, rhs) => self.handle_mul(lhs, rhs),
            Node::Div(lhs, rhs) => self.handle_div(lhs, rhs),
            Node::Add(lhs, rhs) => self.handle_add(lhs, rhs),
            Node::Sub(lhs, rhs) => self.handle_sub(lhs, rhs),
            Node::Mod(lhs, rhs) => self.handle_mod(lhs, rhs),

            Node::UnaryMinus(expression) => self.handle_unary_minus(expression),
            Node::UnaryNot(expression) => self.handle_unary_not(expression),

            Node::FunctionCall {
                callee: _,
                argument_list: _,
            } => self.handle_function_call(&expression),

            Node::Identifier(value) => self.handle_identifier(value).as_any_value_enum(),

            Node::NumberLiteral(value) => self.handle_number_literal(&value),
            Node::StringLiteral(value) => self.handle_string_literal(&value),
            Node::BooleanLiteral(value) => self.handle_boolean_literal(&value),
            Node::UnitLiteral => panic!("UnitLiteral only allowed as return value atm"),
            _ => unreachable!("Unknown AST node type {:?}", expression),
        }
    }

    fn handle_typed_expresssion(
        &self,
        _type_identifier: &str,
        expression: &Node,
    ) -> AnyValueEnum<'ctx> {
        // let type_ = scopes.resolve_type(type_identifier);
        // TODO: type check here
        self.handle_expression(expression)
    }

    fn handle_function_call(&self, call_expression: &Node) -> AnyValueEnum<'ctx> {
        let Node::FunctionCall {
            callee: callee_name,
            argument_list,
        } = call_expression
        else {
            unreachable!();
        };

        let args: Vec<BasicMetadataValueEnum> = argument_list
            .into_iter()
            .map(|arg| BasicMetadataValueEnum::try_from(self.handle_expression(arg)).unwrap())
            .collect();

        let callee = self
            .scopes
            .resolve_function(callee_name)
            .unwrap_or_else(|| panic!("Attempt to call nonexistent function {}", callee_name));

        self.ir
            .builder
            .build_call(callee, &args, "")
            .unwrap()
            .as_any_value_enum()
    }
}
