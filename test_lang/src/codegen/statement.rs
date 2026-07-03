use std::error::Error;

use crate::{
    ast::Statement, ast_store::StatementID, ast_visitor::ASTVisitor, span::SourceIDSpanned,
};

use super::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    fn enter_statement(
        &mut self,
        (statement, statement_id): (&SourceIDSpanned<Statement>, StatementID),
    ) -> Option<Box<dyn Error>> {
        match &statement.inner {
            Statement::Block(_) => {
                self.enter_block();
                None
            }

            // Statement::Const(lvalue, expression) => self.handle_const(lvalue, expression),
            // Statement::Let(lvalue, expression) => self.handle_let(lvalue, expression),
            // Statement::Assignment(lvalue, expression) => self.handle_assignment(lvalue, expression),

            // Statement::If { condition, body } => self.handle_if(*condition, *body),
            // Statement::IfElse(if_branch, else_branch) => {
            //     self.handle_if_else(*if_branch, *else_branch)
            // }
            // Statement::While { condition, body } => self.handle_while(*condition, *body),
            // Statement::For {
            //     init,
            //     condition,
            //     step,
            //     body,
            // } => self.handle_for(init, condition, step, body),

            // Statement::Switch {
            //     matched_value_expression,
            //     cases,
            // } => self.handle_switch(matched_value_expression, cases),
            _ => None,
        }
    }

    fn exit_statement(
        &mut self,
        (statement, _): (&SourceIDSpanned<Statement>, StatementID),
    ) -> Option<Box<dyn Error>> {
        match &statement.inner {
            Statement::Block(_) => self.exit_block(),

            // Statement::Const(lvalue, expression) => self.handle_const(lvalue, expression),
            // Statement::Let(lvalue, expression) => self.handle_let(lvalue, expression),
            // Statement::Assignment(lvalue, expression) => self.handle_assignment(lvalue, expression),
            Statement::Return(_) => self.exit_return(),
            // Statement::If { condition, body } => self.handle_if(*condition, *body),
            // Statement::IfElse(if_branch, else_branch) => {
            //     self.handle_if_else(*if_branch, *else_branch)
            // }
            // Statement::While { condition, body } => self.handle_while(*condition, *body),
            // Statement::For {
            //     init,
            //     condition,
            //     step,
            //     body,
            // } => self.handle_for(init, condition, step, body),

            // Statement::Switch {
            //     matched_value_expression,
            //     cases,
            // } => self.handle_switch(matched_value_expression, cases),
            _ => None,
        }
    }
}

impl<'ctx> CodeGen<'ctx> {
    pub fn enter_block(&mut self) {
        // TODO: push scope
    }

    pub fn exit_block(&mut self) -> Option<Box<dyn Error>> {
        // TODO: pop scope
        None
    }
}
