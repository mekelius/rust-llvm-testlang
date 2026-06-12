use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::support::LLVMString;
use std::error::Error;

use crate::ast::Node;

pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn run(&self, ast: &Node) -> Result<(), Box<dyn Error>> {
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

    ///////////////////////////////////////////////////////////////////////////////////////////////
    fn handle_function(&self, function: &Node) -> Result<(), Box<dyn Error>> {
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

        let entry_b = self.context.append_basic_block(test_f, "entry");
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
            Node::LetStatement(_, _) => todo!(), // self.let_statement(statement),
            Node::ReturnStatement(_) => todo!(), // self.return_statement(statement),
            Node::ExpressionStatement(expression) => self.handle_expression(&expression),
            Node::EmptyStatement => return,
            Node::While {
                condition: _,
                body: _,
            } => todo!(), //self.while_(statement),
            Node::Block(_) => todo!(), //self.block(statement),
            _ => unreachable!(),
        }
    }

    fn handle_expression(&self, expression: &Node) {
        match expression {
            Node::Equals(_lhs, _rhs) => todo!(),
            Node::GreaterThan(_lhs, _rhs) => todo!(),
            Node::LessThan(_lhs, _rhs) => todo!(),
            Node::GreaterThanOrEquals(_lhs, _rhs) => todo!(),
            Node::LessThanOrEquals(_lhs, _rhs) => todo!(),
            Node::NotEquals(_lhs, _rhs) => todo!(),
            Node::Times(_lhs, _rhs) => todo!(),
            Node::Divided(_lhs, _rhs) => todo!(),

            Node::FunctionCall {
                callee: _,
                argument_list: _,
            } => self.handle_function_call(&expression),

            Node::Identifier(_value) => todo!(),
            Node::NumberLiteral(_value) => todo!(),
            _ => unreachable!(),
        }
    }

    fn handle_function_call(&self, call_expression: &Node) {
        let Node::FunctionCall {
            callee,
            argument_list,
        } = call_expression
        else {
            unreachable!();
        };
        
        // !!!PLACEHOLDER
        let i64_t = self.context.i64_type();
        let i64_ft = i64_t.fn_type(&[], false);
        let callee_ref_PLACEHOLDER = self.module.add_function(callee, i64_ft, None);
        // !!!
        
        self.builder.build_call(callee_ref_PLACEHOLDER, &[], "");
    }
}
