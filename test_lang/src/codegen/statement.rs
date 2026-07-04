use std::error::Error;

use crate::{
    ast::Statement,
    ast_store::{ASTStore, StatementID},
    codegen::CodegenError,
};

use super::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub fn handle_statement(
        &mut self,
        ast_store: &ASTStore,
        statement_id: StatementID,
    ) -> Result<(), CodegenError> {
        match &ast_store.get_statement(statement_id).inner {
            Statement::Block(ids) => self.handle_block(ast_store, ids),

            Statement::Expression(expression_id) => self
                .handle_expression(ast_store, *expression_id)
                .map(|_| ()),
            Statement::Const(lvalue, expression) => {
                self.handle_const(ast_store, *lvalue, *expression)
            }
            Statement::Let(lvalue, expression) => self.handle_let(ast_store, *lvalue, *expression),
            Statement::Assignment(lvalue, expression) => {
                self.handle_assignment(ast_store, *lvalue, *expression)
            }
            Statement::Return(expression_id) => self.handle_return(ast_store, *expression_id),

            Statement::If { condition, body } => self.handle_if(ast_store, *condition, *body),
            Statement::IfElse(if_branch, else_branch) => {
                self.handle_if_else(ast_store, *if_branch, *else_branch)
            }
            Statement::While { condition, body } => self.handle_while(ast_store, *condition, *body),
            Statement::For {
                init,
                condition,
                step,
                body,
            } => self.handle_for(ast_store, *init, *condition, *step, *body),

            Statement::Switch {
                matched_value_expression,
                cases,
            } => self.handle_switch(ast_store, *matched_value_expression, cases),
            _ => todo!("Other statement types"),
        }
    }
}

impl<'ctx> CodeGen<'ctx> {
    pub fn handle_block(
        &mut self,
        ast_store: &ASTStore,
        statement_ids: &Vec<StatementID>,
    ) -> Result<(), Box<dyn Error>> {
        // TODO: pop scope
        for statement_id in statement_ids {
            self.handle_statement(ast_store, *statement_id)?;
        }
        Ok(())
    }
}
