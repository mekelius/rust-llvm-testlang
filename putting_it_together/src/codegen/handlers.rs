use inkwell::values::{AnyValue, AnyValueEnum, BasicMetadataValueEnum};
use std::error::Error;

use super::CodeGen;
use crate::{ast::Node, codegen::{identifier::Symbol, scope::Scopes}};

impl<'ctx> CodeGen<'ctx> {
    ////////////////////////////////////////// Handlers ///////////////////////////////////////////
    pub fn handle_function(&self, function: &Node, scopes: &mut Scopes<'ctx>) -> Result<(), Box<dyn Error>> {
        let Node::Function {
            name,
            formals: _,
            body,
        } = function
        else {
            unreachable!();
        };

        let void_t = self.context.void_type();
        // let i64_i64_i64_ft = i64_t.fn_type(&[i64_t.into(), i64_t.into(), i64_t.into()], false);
        let void_ft = void_t.fn_type(&[], false);

        let test_f = self.module.add_function(&name, void_ft, None);
        scopes.define_identifier(&name.to_string(), Symbol::Function(test_f));

        let entry_b = self.context.append_basic_block(test_f, "Entry");
        self.builder.position_at_end(entry_b);


        // Prepare function scope -----------------------------------------------------

        let function_scope = scopes.push_new_scope();

        // ----------------------------------------------------------------------------------------

        let Node::FunctionBody(body) = &**body else {
            unreachable!();
        };
        for statement in body {
            self.handle_statement(&statement, scopes);
        }

        self.builder.build_return(None)?;
        scopes.pop_scope();

        Ok(())
    }

    pub fn handle_statement(&self, statement: &Node, scopes: &mut Scopes<'ctx>) {
        match statement {
            Node::LetStatement(_, _) => todo!("Let"), // self.let_statement(statement),
            Node::ReturnStatement(_) => todo!("Return"), // self.return_statement(statement),
            Node::ExpressionStatement(expression) => {
                self.handle_expression(&expression, scopes);
            }
            Node::EmptyStatement => return,

            Node::If { condition, body } => self.handle_if(condition, body, scopes),
            Node::While { condition, body } => self.handle_while(condition, body, scopes),
            Node::For {
                init,
                condition,
                step,
                body,
            } => self.handle_for(init, condition, step, body),

            Node::Block(statements) => self.handle_block(statements, scopes),
            _ => unreachable!("Unknown statement type {:?}", statement),
        };
    }

    pub fn handle_block(&self, statements: &Vec<Node>, scopes: &mut Scopes<'ctx>) {
        for statement in statements {
            self.handle_statement(statement, scopes);
        }
    }

    pub fn handle_expression(&self, expression: &Node, scopes: &mut Scopes<'ctx>) -> AnyValueEnum<'_> {
        match expression {
            Node::Equals(lhs, rhs) => self.handle_eq(lhs, rhs, scopes),
            Node::NotEquals(lhs, rhs) => self.handle_neq(lhs, rhs, scopes),
            Node::GreaterThan(lhs, rhs) => self.handle_gt(lhs, rhs, scopes),
            Node::LessThan(lhs, rhs) => self.handle_lt(lhs, rhs, scopes),
            Node::GreaterThanOrEquals(lhs, rhs) => self.handle_gteq(lhs, rhs, scopes),
            Node::LessThanOrEquals(lhs, rhs) => self.handle_lteq(lhs, rhs, scopes),

            Node::Mul(lhs, rhs) => self.handle_mul(lhs, rhs, scopes),
            Node::Div(lhs, rhs) => self.handle_div(lhs, rhs, scopes),
            Node::Add(lhs, rhs) => self.handle_add(lhs, rhs, scopes),
            Node::Sub(lhs, rhs) => self.handle_sub(lhs, rhs, scopes),

            Node::FunctionCall {
                callee: _,
                argument_list: _,
            } => self.handle_function_call(&expression, scopes),

            Node::Identifier(_value) => todo!("Identifier"),

            Node::NumberLiteral(value) => self.handle_number_literal(&value),
            Node::StringLiteral(value) => self.handle_string_literal(&value),
            Node::BooleanLiteral(value) => self.handle_boolean_literal(&value),
            _ => unreachable!("Unknown AST node type {:?}", expression),
        }
    }

    fn handle_function_call(&self, call_expression: &Node, scopes: &mut Scopes<'ctx>) -> AnyValueEnum<'_> {
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
            .map(|arg| BasicMetadataValueEnum::try_from(self.handle_expression(arg, scopes)).unwrap())
            .collect();

        print!("{:?}", *scopes);

        let callee = self
            .resolve_function(callee_name, &scopes)
            .unwrap_or_else(|| panic!("Attempt to call nonexistent function {}", callee_name));

        self.builder
            .build_call(callee, &args, "")
            .unwrap()
            .as_any_value_enum()
    }
}
