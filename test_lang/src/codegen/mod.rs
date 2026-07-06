pub mod ir;
pub mod scope;
pub mod helpers;

mod builtins;
mod types;

mod binop;
mod expression;
mod function;
mod identifier;
mod r#if;
mod literal;
mod r#loop;
mod statement;
mod switch;
mod unop;
mod variable;

use inkwell::context::Context;
use std::error::Error;

use crate::ast::Program;
use crate::ast_store::ASTStore;
use crate::codegen::ir::IR;
use crate::codegen::scope::Scopes;

pub struct CodeGen<'ctx> {
    pub ir: IR<'ctx>,
    pub scopes: Scopes<'ctx>,
}

pub type CodegenError = Box<dyn Error>;

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

    pub fn run(&mut self, ast_store: &ASTStore, Program { functions }: &Program) -> Result<(), Box<dyn Error>> {
        for function in functions {
            self.handle_function(ast_store, *function)?;
        }

        Ok(())
    }
}
