mod builtins;
mod identifier;
mod handlers;

use dict::{Dict, DictIface};
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::support::LLVMString;
use inkwell::types::StringRadix::Decimal;
use inkwell::values::{AnyValue, AnyValueEnum, BasicMetadataValueEnum};
use inkwell::{builder::Builder, values::FunctionValue};
use std::error::Error;

use crate::ast::Node;

pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub builtins: Dict<FunctionValue<'ctx>>,
    pub function_identifiers: Dict<FunctionValue<'ctx>>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, name: &'ctx str) -> CodeGen<'ctx> {
        let mut codegen = Self {
            context,
            module: context.create_module(name),
            builder: context.create_builder(),
            builtins: Dict::new(),
            function_identifiers: Dict::new(),
        };

        codegen.init_builtins();
        codegen
    }

    pub fn run(&mut self, ast: &'ctx Node) -> Result<(), Box<dyn Error>> {
        match ast {
            Node::Program(program) => {
                for function in program {
                    self.handle_function(function)?;
                }
            }
            _ => unreachable!(),
        };

        Ok(())
    }

    pub fn print(&self) -> Result<(), LLVMString> {
        self.module.verify()?;
        let dump = self.module.print_to_string().to_string();
        print!("{}", &dump);
        Ok(())
    }

    pub fn print_to_file(&self, output_file: &str) -> Result<(), LLVMString> {
        self.module.verify()?;
        self.module.print_to_file(output_file)?;
        Ok(())
    }

    ////////////////////////////////////////// Handlers ///////////////////////////////////////////
    fn handle_function(&mut self, function: &Node) -> Result<(), Box<dyn Error>> {
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

    fn handle_expression(&self, expression: &Node) -> AnyValueEnum<'_> {
        match expression {
            Node::Equals(_lhs, _rhs) => todo!("Equals"),
            Node::GreaterThan(_lhs, _rhs) => todo!("GT"),
            Node::LessThan(_lhs, _rhs) => todo!("LT"),
            Node::GreaterThanOrEquals(_lhs, _rhs) => todo!("GTorEQ"),
            Node::LessThanOrEquals(_lhs, _rhs) => todo!("LTorEQ"),
            Node::NotEquals(_lhs, _rhs) => todo!("NEQ"),
            Node::Mult(lhs, rhs) => self.handle_mult(lhs, rhs),
            Node::Div(lhs, rhs) => self.handle_div(lhs, rhs),
            Node::Add(lhs, rhs) => self.handle_add(lhs, rhs),
            Node::Subtr(lhs, rhs) => self.handle_subtr(lhs, rhs),

            Node::FunctionCall {
                callee: _,
                argument_list: _,
            } => self.handle_function_call(&expression),

            Node::Identifier(_value) => todo!("Identifier"),
            Node::NumberLiteral(value) => self.handle_number_literal(&value),
            Node::StringLiteral(value) => self.handle_string_literal(&value),
            _ => unreachable!(),
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

    fn handle_number_literal(&self, value: &str) -> AnyValueEnum<'_> {
        self.context
            .i64_type()
            .const_int_from_string(value, Decimal)
            .unwrap_or_else(|| panic!("Could not create integer from {}", value))
            .as_any_value_enum()
    }

    fn handle_string_literal(&self, value: &str) -> AnyValueEnum<'_> {
        self.builder
            .build_global_string_ptr(value, "string_literal")
            .unwrap_or_else(|_| panic!("Creating global string from {} failed", value))
            .as_any_value_enum()
    }

    fn handle_add(&self, lhs: &Node, rhs: &Node) -> AnyValueEnum<'_> {
        let lhs_value = self.handle_expression(lhs).into_int_value();
        let rhs_value = self.handle_expression(rhs).into_int_value();

        self.builder
            .build_int_add(lhs_value, rhs_value, "")
            .unwrap()
            .as_any_value_enum()
    }

    fn handle_subtr(&self, lhs: &Node, rhs: &Node) -> AnyValueEnum<'_> {
        let lhs_value = self.handle_expression(lhs).into_int_value();
        let rhs_value = self.handle_expression(rhs).into_int_value();

        self.builder
            .build_int_sub(lhs_value, rhs_value, "")
            .unwrap()
            .as_any_value_enum()
    }

    fn handle_mult(&self, lhs: &Node, rhs: &Node) -> AnyValueEnum<'_> {
        let lhs_value = self.handle_expression(lhs).into_int_value();
        let rhs_value = self.handle_expression(rhs).into_int_value();

        self.builder
            .build_int_mul(lhs_value, rhs_value, "")
            .unwrap()
            .as_any_value_enum()
    }

    fn handle_div(&self, lhs: &Node, rhs: &Node) -> AnyValueEnum<'_> {
        let lhs_value = self.handle_expression(lhs).into_int_value();
        let rhs_value = self.handle_expression(rhs).into_int_value();

        self.builder
            .build_int_signed_div(lhs_value, rhs_value, "")
            .unwrap()
            .as_any_value_enum()
    }
}
