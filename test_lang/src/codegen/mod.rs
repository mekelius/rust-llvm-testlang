pub mod ir;
pub mod scope;

mod builtins;

mod function;
mod expression;
mod statement;
mod binop;
mod control;
mod identifier;
mod literal;
mod variable;

use inkwell::context::Context;
use std::error::Error;

use crate::ast::Node;
use crate::codegen::ir::IR;
use crate::codegen::scope::Scopes;

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
}
