use std::error::Error;

use crate::{
    ast::Statement,
    ast_store::{ASTStore, StatementID},
    span::SourceIDSpanned,
};

use super::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub fn handle_statement(
        &mut self,
        ast_store: &ASTStore,
        statement_id: StatementID,
    ) -> Result<(), Box<dyn Error>> {
        match &ast_store.get_statement(statement_id).inner {
            Statement::Block(id) => self.handle_block(ast_store, id),

            Statement::Expression(expression_id) => self
                .handle_expression(ast_store, *expression_id)
                .map(|_| ()),
            // Statement::Const(lvalue, expression) => self.handle_const(lvalue, expression),
            // Statement::Let(lvalue, expression) => self.handle_let(lvalue, expression),
            // Statement::Assignment(lvalue, expression) => self.handle_assignment(lvalue, expression),
            Statement::Return(expression_id) => self.handle_return(ast_store, *expression_id),

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
            _ => todo!("Other statement types"),
        }
    }

    // fn exit_statement(
    //     &mut self,
    //     (statement, _): (&SourceIDSpanned<Statement>, StatementID),
    // ) -> result((), <Box<dyn Error>) {
    //     match &statement.inner {
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
    // _ => None,
    // }
    // }
}

impl<'ctx> CodeGen<'ctx> {
    pub fn handle_block(
        &mut self,
        ast_store: &ASTStore,
        statement_ids: &Vec<StatementID>,
    ) -> Result<(), Box<dyn Error>> {
        // TODO: pop scope
        Ok(())
    }
}
