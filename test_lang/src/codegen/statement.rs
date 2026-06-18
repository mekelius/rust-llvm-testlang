use super::CodeGen;
use crate::ast::Node;

impl<'ctx> CodeGen<'ctx> {
    pub fn handle_statement(&mut self, statement: &Node) {
        match statement {
            Node::LetStatement(identifier, expression) => self.handle_let(identifier, expression),
            Node::ReturnStatement(expression) => self.handle_return(expression),
            Node::ValuelessReturnStatement => self.handle_valueless_return(),
            Node::ExpressionStatement(expression) => {
                self.handle_expression(&expression);
            }
            Node::EmptyStatement => return,

            Node::If { condition, body } => self.handle_if(condition, body),
            Node::While { condition, body } => self.handle_while(condition, body),
            Node::For {
                init,
                condition,
                step,
                body,
            } => self.handle_for(init, condition, step, body),

            Node::Block(statements) => self.handle_block(statements),
            _ => unreachable!("Unknown statement type {:?}", statement),
        };
    }

    pub fn handle_block(&mut self, statements: &Vec<Node>) {
        for statement in statements {
            self.handle_statement(statement);
        }
    }
}
