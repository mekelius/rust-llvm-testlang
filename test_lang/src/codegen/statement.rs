use super::CodeGen;
use crate::ast::Node;

impl<'ctx> CodeGen<'ctx> {
    /* Return true if was a return statement */
    pub fn handle_statement(&mut self, statement: &Node) {
        match statement {
            Node::ConstStatement(identifier, expression) => self.handle_const(identifier, expression),
            Node::LetStatement(_identifier, _expression) => todo!("Let statement"),
            Node::AssignmentStatement(_identifier, _expression) => todo!("Assignment statement"),

            Node::ReturnStatement(expression) => {
                self.handle_return(expression);
            }
            Node::ValuelessReturnStatement => {
                self.handle_valueless_return();
            }
            Node::ExpressionStatement(expression) => {
                self.handle_expression(&expression);
            }
            Node::EmptyStatement => return,

            Node::IfStatement { condition, body } => self.handle_if(condition, body),
            Node::IfElseStatement(if_branch, else_branch) => {
                self.handle_if_else(if_branch, else_branch)
            }
            Node::WhileStatement { condition, body } => self.handle_while(condition, body),
            Node::ForStatement {
                init,
                condition,
                step,
                body,
            } => self.handle_for(init, condition, step, body),

            Node::Block(statements) => self.handle_block(statements),

            Node::SwitchStatement{matched_value_expression, cases} => self.handle_switch(matched_value_expression, cases),
            _ => unreachable!("Unknown statement type {:?}", statement),
        };
    }

    pub fn handle_block(&mut self, statements: &Vec<Node>) {
        for statement in statements {
            self.handle_statement(statement);
        }
    }
}
