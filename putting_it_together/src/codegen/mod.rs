mod builtins;
mod identifier;
mod handlers;
mod binop;
mod literal;

use dict::Dict;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::support::LLVMString;
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
}
