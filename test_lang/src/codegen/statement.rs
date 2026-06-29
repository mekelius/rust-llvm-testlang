use chumsky::span::Spanned;

use crate::ast::Statement;

use super::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    /* Return true if was a return statement */
    pub fn handle_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::ConstStatement(identifier, expression) => {
                self.handle_const(identifier, expression)
            }
            Statement::LetStatement(identifier, expression) => {
                self.handle_let(identifier, expression)
            }
            Statement::AssignmentStatement(identifier, expression) => {
                self.handle_assignment(identifier, expression)
            }

            Statement::ReturnStatement(expression) => {
                self.handle_return(expression);
            }
            Statement::ValuelessReturnStatement => {
                self.handle_valueless_return();
            }
            Statement::ExpressionStatement(expression) => {
                self.handle_expression(&expression);
            }
            Statement::EmptyStatement => return,

            Statement::IfStatement { condition, body } => self.handle_if(condition, body),
            Statement::IfElseStatement(if_branch, else_branch) => {
                self.handle_if_else(if_branch, else_branch)
            }
            Statement::WhileStatement { condition, body } => self.handle_while(condition, body),
            Statement::ForStatement {
                init,
                condition,
                step,
                body,
            } => self.handle_for(init, condition, step, body),

            Statement::Block(statements) => self.handle_block(statements),

            Statement::SwitchStatement {
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
