use inkwell::values::{AnyValue, AnyValueEnum, BasicMetadataValueEnum};
use std::error::Error;

use super::CodeGen;
use crate::{
    ast::Node,
    codegen::{identifier::Symbol, scope::Scopes},
};

impl<'ctx> CodeGen<'ctx> {
    ////////////////////////////////////////// Handlers ///////////////////////////////////////////
    pub fn handle_function(&mut self, function: &Node) -> Result<(), Box<dyn Error>> {
        let Node::Function {
            name,
            formals: _,
            body,
        } = function
        else {
            unreachable!();
        };

        {
            let CodeGen { ir, scopes } = self;

            let void_t = ir.context.void_type();
            // let i64_i64_i64_ft = i64_t.fn_type(&[i64_t.into(), i64_t.into(), i64_t.into()], false);
            let void_ft = void_t.fn_type(&[], false);

            let test_f = ir.module.add_function(&name, void_ft, None);
            scopes.define_identifier(&name.to_string(), Symbol::Function(test_f));

            let entry_b = ir.context.append_basic_block(test_f, "Entry");
            ir.builder.position_at_end(entry_b);

            // Prepare function scope -----------------------------------------------------

            let function_scope = scopes.push_new_scope();

            // ----------------------------------------------------------------------------------------
        }

        let Node::FunctionBody(body) = &**body else {
            unreachable!();
        };
        for statement in body {
            self.handle_statement(&statement);
        }

        {
            let CodeGen { ir, scopes } = self;

            ir.builder.build_return(None)?;
            scopes.pop_scope();
        }
        Ok(())
    }

    pub fn handle_statement(&mut self, statement: &Node) {
        match statement {
            Node::LetStatement(identifier, expression) => self.handle_let(identifier, expression),
            Node::ReturnStatement(_) => todo!("Return"), // self.return_statement(statement),
            Node::ExpressionStatement(expression) => {
                self.handle_expression(&expression);
            }
            Node::EmptyStatement => return,

            Node::If { condition, body } => self.handle_if(condition, body),
            Node::While { condition, body } => self.handle_while(condition, body),
            Node::For {
                init,
                condition,
                step,
                body,
            } => self.handle_for(init, condition, step, body),

            Node::Block(statements) => self.handle_block(statements),
            _ => unreachable!("Unknown statement type {:?}", statement),
        };
    }

    pub fn handle_block(&mut self, statements: &Vec<Node>) {
        for statement in statements {
            self.handle_statement(statement);
        }
    }

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

            Node::FunctionCall {
                callee: _,
                argument_list: _,
            } => self.handle_function_call(&expression),

            Node::Identifier(value) => self.handle_identifier(value).as_any_value_enum(),

            Node::NumberLiteral(value) => self.handle_number_literal(&value),
            Node::StringLiteral(value) => self.handle_string_literal(&value),
            Node::BooleanLiteral(value) => self.handle_boolean_literal(&value),
            _ => unreachable!("Unknown AST node type {:?}", expression),
        }
    }

    fn handle_typed_expresssion(
        &self,
        type_identifier: &str,
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

        // let mut args = Vec<LLVMVa>::new;
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
