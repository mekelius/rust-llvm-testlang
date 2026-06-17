mod binop;
mod builtins;
mod control;
mod handlers;
mod identifier;
mod literal;
pub mod scope;
mod variable;

use dict::Dict;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::support::LLVMString;
use inkwell::{builder::Builder, values::FunctionValue};
use std::error::Error;

use crate::ast::Node;
use crate::codegen::scope::Scopes;

pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub builtins: Dict<FunctionValue<'ctx>>,
    pub function_identifiers: Dict<FunctionValue<'ctx>>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, name: &'ctx str) -> CodeGen<'ctx> {
        let codegen = Self {
            context,
            module: context.create_module(name),
            builder: context.create_builder(),
            builtins: Dict::new(),
            function_identifiers: Dict::new(),
        };
        
        // codegen.init_builtins(scopes);
        codegen
    }

    pub fn run(&self, ast: &Node) -> Result<(), Box<dyn Error>> {
        let mut scopes = Scopes::new();
        self.init_builtins(&mut scopes);

        match ast {
            Node::Program(program) => {
                for function in program {
                    self.handle_function(function, &mut scopes)?;
                }
            }
            _ => unreachable!(),
        };

        Ok(())
    }

    pub fn print(&self) -> Result<(), LLVMString> {
        let passed = self.module.verify();

        let dump = self.module.print_to_string().to_string();
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
        self.module.verify()?;
        self.module.print_to_file(output_file)?;
        Ok(())
    }
}
