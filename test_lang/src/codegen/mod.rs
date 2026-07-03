pub mod ir;
pub mod scope;

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

use crate::ast::{Expression, Function, Program, Statement};
use crate::ast_store::{ASTStore, ExpressionID, FunctionID, StatementID};
use crate::ast_visitor::ASTVisitor;
use crate::codegen::ir::IR;
use crate::codegen::scope::Scopes;
use crate::span::SourceIDSpanned;

pub struct CodeGen<'ctx> {
    pub ir: IR<'ctx>,
    pub scopes: Scopes<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &'ctx str) -> CodeGen<'ctx> {
        let mut codegen = Self {
            ir: IR::new(context, module_name),
            scopes: Scopes::new(),
        };

        codegen.init_builtins();
        codegen
    }

    pub fn run(
        &mut self,
        (program, store): (&mut SourceIDSpanned<Program>, &mut ASTStore),
    ) -> Result<(), Box<dyn Error>> {
        store.walk_program(self, program);

        Ok(())
    }
}

macro_rules! add_visit {
    ($visit_method:ident, $node_type:ty) => {
        fn $visit_method(&mut self, node: $node_type) -> Option<Box<dyn Error>> {
            self.$visit_method(node)
        }
    };
}

impl<'ctx> ASTVisitor<Box<dyn Error>> for CodeGen<'ctx> {
    add_visit!(enter_function, (&SourceIDSpanned<Function>, FunctionID));
    add_visit!(exit_function, (&SourceIDSpanned<Function>, FunctionID));
    add_visit!(enter_statement, (&SourceIDSpanned<Statement>, StatementID));
    add_visit!(exit_statement, (&SourceIDSpanned<Statement>, StatementID));
    add_visit!(enter_expression, (&SourceIDSpanned<Expression>, ExpressionID));
    add_visit!(exit_expression, (&SourceIDSpanned<Expression>, ExpressionID));
}
