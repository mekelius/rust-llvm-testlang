use std::error::Error;
use dict::DictIface;
use inkwell::values::{AnyValue, AnyValueEnum, BasicMetadataValueEnum};

use crate::ast::Node;
use super::CodeGen;

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

        let void_t = self.context.void_type();
        // let i64_i64_i64_ft = i64_t.fn_type(&[i64_t.into(), i64_t.into(), i64_t.into()], false);
        let void_ft = void_t.fn_type(&[], false);

        let test_f = self.module.add_function(&name, void_ft, None);
        self.function_identifiers.add(name.to_string(), test_f);

        let entry_b = self.context.append_basic_block(test_f, "Entry");
        self.builder.position_at_end(entry_b);

        let Node::FunctionBody(body) = &**body else {
            unreachable!();
        };
        for statement in body {
            self.handle_statement(&statement);
        }

        // self.builder.build_return(Some(&c1))?;
        self.builder.build_return(None)?;

        Ok(())
    }

    fn handle_statement(&self, statement: &Node) {
        /*  CURRENTLY POSSIBLE STATEMENTS DEFINED IN THIS CLUSTEFUCK:

            let simple_statement = choice((
                let_statement.clone(),
                return_statement.clone(),
                expression.clone(),
                empty_statement.clone(),
            )).boxed();
            let single_statement = simple_statement.then_ignore(just(Token::Semicolon)).boxed();

            let complex_statement = while_;

            single_statement.or(block).or(complex_statement).boxed()
        */

        match statement {
            Node::LetStatement(_, _) => todo!("Let"), // self.let_statement(statement),
            Node::ReturnStatement(_) => todo!("Return"), // self.return_statement(statement),
            Node::ExpressionStatement(expression) => self.handle_expression(&expression),
            Node::EmptyStatement => return,
            Node::While {
                condition: _,
                body: _,
            } => todo!(), //self.while_(statement),
            Node::Block(_) => todo!(), //self.block(statement),
            _ => unreachable!(),
        };
    }

    pub fn handle_expression(&self, expression: &Node) -> AnyValueEnum<'_> {
        match expression {
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

            Node::Identifier(_value) => todo!("Identifier"),
            
            Node::NumberLiteral(value) => self.handle_number_literal(&value),
            Node::StringLiteral(value) => self.handle_string_literal(&value),
            Node::BooleanLiteral(value) => self.handle_boolean_literal(&value),
            _ => unreachable!("Unknown AST node type {:?}", expression),
        }
    }

    fn handle_function_call(&self, call_expression: &Node) -> AnyValueEnum<'_> {
        let Node::FunctionCall {
            callee: callee_name,
            argument_list,
        } = call_expression
        else {
            unreachable!();
        };

        // !!!PLACEHOLDER
        // let i64_t = self.context.i64_type();
        // let i64_ft = i64_t.fn_type(&[], false);
        // let callee_ref_PLACEHOLDER = self.module.add_function(callee, i64_ft, None);
        // !!!

        // let mut args = Vec<LLVMVa>::new;
        let args: Vec<BasicMetadataValueEnum> = argument_list
            .into_iter()
            .map(|arg| BasicMetadataValueEnum::try_from(self.handle_expression(arg)).unwrap())
            .collect();

        let callee = self
            .resolve_function(callee_name)
            .unwrap_or_else(|| panic!("Attempt to call nonexistent function {}", callee_name));

        self.builder
            .build_call(*callee, &args, "")
            .unwrap()
            .as_any_value_enum()
    }
}