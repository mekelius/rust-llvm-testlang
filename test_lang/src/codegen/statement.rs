use chumsky::span::Spanned;

use crate::ast::Statement;

use super::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    /* Return true if was a return statement */
    pub fn handle_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Const(lvalue, expression) => {
                self.handle_const(lvalue, expression)
            }
            Statement::Let(lvalue, expression) => {
                self.handle_let(lvalue, expression)
            }
            Statement::Assignment(lvalue, expression) => {
                self.handle_assignment(lvalue, expression)
            }

            Statement::Return(expression) => {
                self.handle_return(expression);
            }
            Statement::Expression(expression) => {
                self.handle_expression(&expression);
            }
            Statement::Empty => return,

            Statement::If { condition, body } => self.handle_if(condition, body),
            Statement::IfElse(if_branch, else_branch) => {
                self.handle_if_else(if_branch, else_branch)
            }
            Statement::While { condition, body } => self.handle_while(condition, body),
            Statement::For {
                init,
                condition,
                step,
                body,
            } => self.handle_for(init, condition, step, body),

            Statement::Block(statements) => self.handle_block(statements),

            Statement::Switch {
                matched_value_expression,
                cases,
            } => self.handle_switch(matched_value_expression, cases),
            _ => unreachable!("Unknown statement type {:?}", statement),
        };
    }

    pub fn handle_block<S>(&mut self, statements: &Vec<Spanned<Statement, S>>) {
        for statement in statements {
            self.handle_statement(statement);
        }
    }
}
