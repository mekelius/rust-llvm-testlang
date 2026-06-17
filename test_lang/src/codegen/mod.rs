mod binop;
mod builtins;
mod control;
mod handlers;
mod identifier;
mod literal;
pub mod scope;
mod variable;

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::support::LLVMString;
use inkwell::builder::Builder;
use std::error::Error;

use crate::ast::Node;
use crate::codegen::scope::Scopes;

pub struct IR<'ctx> {
        pub context: &'ctx Context,
        pub module: Module<'ctx>,
        pub builder: Builder<'ctx>,
    }

pub struct CodeGen<'ctx> {
    pub ir: IR<'ctx>,
    pub scopes: Scopes<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, name: &'ctx str) -> CodeGen<'ctx> {
        let mut codegen = Self {
            ir: IR {
                context,
                module: context.create_module(name),
                builder: context.create_builder(),
            },
            scopes: Scopes::new(),
        };
        
        codegen.init_builtins();
        codegen
    }

    pub fn run(&mut self, ast: &Node) -> Result<(), Box<dyn Error>> {
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
        let passed = self.ir.module.verify();

        let dump = self.ir.module.print_to_string().to_string();
        print!("{}", &dump);

        match passed {
            Err(err) => {
                print!("{:?}", err);
                panic!("Module verification failed");
            }
            Ok(_) => return Ok(()),
        };
    }

    pub fn print_to_file(&self, output_file: &str) -> Result<(), LLVMString> {
        self.ir.module.verify()?;
        self.ir.module.print_to_file(output_file)?;
        Ok(())
    }
}
