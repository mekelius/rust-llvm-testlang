use chumsky::span::SpanWrap;

use crate::{
    ast::{BinopExpression, Expression, FunctionCall, Node, Statement, UnopExpression},
    span::SourceIDSpanned,
};

pub trait ASTVisitor<R> {
    fn visit_program(&mut self, program: &SourceIDSpanned<Node>) -> Option<R>;
    fn visit_function(&mut self, function: &SourceIDSpanned<Node>) -> Option<R>;
    fn visit_statement(&mut self, statement: &SourceIDSpanned<Statement>) -> Option<R>;
    fn visit_expression(&mut self, expression: &SourceIDSpanned<Expression>) -> Option<R>;
    fn visit_case(&mut self, case: &SourceIDSpanned<Node>) -> Option<R>;
}

pub trait DumbASTVisitor<R>: ASTVisitor<R> {
    fn visit_node(&mut self, case: &SourceIDSpanned<Node>) -> Option<R>;
}

impl<T, R> ASTVisitor<R> for T
where
    T: DumbASTVisitor<R>,
{
    fn visit_program(&mut self, program: &SourceIDSpanned<Node>) -> Option<R> {
        self.visit_node(program)
    }
    fn visit_function(&mut self, function: &SourceIDSpanned<Node>) -> Option<R> {
        self.visit_node(function)
    }
    fn visit_statement(&mut self, statement: &SourceIDSpanned<Statement>) -> Option<R> {
        self.visit_node(&Node::Statement(statement.inner.clone()).with_span(statement.span))
    }
    fn visit_expression(&mut self, expression: &SourceIDSpanned<Expression>) -> Option<R> {
        self.visit_node(&Node::Expression(expression.inner.clone()).with_span(expression.span))
    }
    fn visit_case(&mut self, case: &SourceIDSpanned<Node>) -> Option<R> {
        self.visit_node(case)
    }
}

/**
 * Helper trait
 * */
trait ASTVisitResult<R> {
    fn into_option(self) -> Option<R>;
}

impl<T, R, RH> DumbASTVisitor<R> for T
where
    T: Fn(&SourceIDSpanned<Node>) -> RH,
    RH: ASTVisitResult<R>,
{
    fn visit_node(&mut self, node: &SourceIDSpanned<Node>) -> Option<R> {
        self(node).into_option()
    }
}

impl ASTVisitResult<()> for bool {
    fn into_option(self) -> Option<()> {
        self.then_some(())
    }
}

impl<R> ASTVisitResult<R> for Option<R> {
    fn into_option(self) -> Option<R> {
        self
    }
}

impl Node {
    pub fn walk_program<R>(
        visitor: &mut impl ASTVisitor<R>,
        program: &mut SourceIDSpanned<Node>,
    ) -> Option<R> {
        let value = visitor.visit_program(program);
        if value.is_some() {
            return value;
        }

        let Node::Program(functions) = &mut program.inner else {
            unreachable!("Node::walk_program called with a non-program node");
        };

        functions
            .iter_mut()
            .find_map(|function| Self::walk_function(visitor, function))
    }

    pub fn walk_function<R>(
        visitor: &mut impl ASTVisitor<R>,
        function: &mut SourceIDSpanned<Node>,
    ) -> Option<R> {
        return visitor.visit_function(function).or_else(|| {
            let Node::Function {
                name: _,
                return_type_string: _,
                formals: _,
                body,
            } = &mut function.inner
            else {
                unreachable!("Node::walk_function called with a non-function node")
            };

            let Node::FunctionBody(statements) = &mut body.inner else {
                unreachable!(
                    "Node::walk_function called with function node that had no function body"
                )
            };

            statements
                .iter_mut()
                .find_map(|statement| Self::walk_statement(visitor, statement))
        });
    }

    pub fn walk_statement<R>(
        visitor: &mut impl ASTVisitor<R>,
        statement: &mut SourceIDSpanned<Statement>,
    ) -> Option<R> {
        println!("{:?}", statement);

        return visitor
            .visit_statement(statement)
            .or_else(|| match &mut statement.inner {
                Statement::Block(statements) => statements
                    .iter_mut()
                    .find_map(|statement| Self::walk_statement(visitor, statement)),

                Statement::EmptyStatement => None,
                Statement::ExpressionStatement(expression) => {
                    Self::walk_expression(visitor, &mut **expression)
                }

                Statement::WhileStatement { condition, body } => {
                    Self::walk_expression(visitor, &mut **condition)
                        .or_else(|| Self::walk_statement(visitor, body))
                }
                Statement::ForStatement {
                    init,
                    condition,
                    step,
                    body,
                } => {
                    return Self::walk_statement(visitor, init)
                        .or_else(|| Self::walk_expression(visitor, &mut **condition))
                        .or_else(|| Self::walk_statement(visitor, step))
                        .or_else(|| Self::walk_statement(visitor, body));
                }

                Statement::IfStatement { condition, body } => {
                    return Self::walk_expression(visitor, &mut **condition)
                        .or_else(|| Self::walk_statement(visitor, body));
                }
                Statement::IfElseStatement(if_statement, else_statement) => {
                    return Self::walk_statement(visitor, if_statement)
                        .or_else(|| Self::walk_statement(visitor, else_statement));
                }
                Statement::SwitchStatement {
                    matched_value_expression,
                    cases,
                } => {
                    return Self::walk_expression(visitor, &mut **matched_value_expression)
                        .or_else(|| {
                            cases
                                .iter_mut()
                                .find_map(|case| Self::walk_case(visitor, case))
                        });
                }

                Statement::LetStatement(_, expression) => {
                    return Self::walk_expression(visitor, &mut **expression);
                }
                Statement::ConstStatement(_, expression) => {
                    return Self::walk_expression(visitor, &mut **expression);
                }
                Statement::AssignmentStatement(_identifier, expression) => {
                    return Self::walk_expression(visitor, &mut **expression);
                }
                Statement::ReturnStatement(expression) => {
                    return Self::walk_expression(visitor, &mut **expression);
                }
                _ => {
                    unreachable!("Node::walk_statement called with a non-statement node")
                }
            });
    }

    pub fn walk_expression<R>(
        visitor: &mut impl ASTVisitor<R>,
        expression: &mut SourceIDSpanned<Expression>,
    ) -> Option<R> {
        return visitor
            .visit_expression(expression)
            .or_else(|| match &mut expression.inner {
                Expression::Binop(BinopExpression { op: _, lhs, rhs }) => {
                    Self::walk_binary_expression(visitor, &mut **lhs, &mut **rhs)
                }
                Expression::FunctionCall(FunctionCall {
                    callee: _,
                    argument_list,
                }) => argument_list.iter_mut().find_map(|argument_expression| {
                    Self::walk_expression(visitor, argument_expression)
                }),
                Expression::Unop(UnopExpression { op: _, term }) => {
                    Self::walk_expression(visitor, term)
                }
                Expression::Literal(_) => None,
                Expression::Identifier(_) => None,
                Expression::TypedExpression(_type_, expression) => {
                    Self::walk_expression(visitor, expression)
                }
            });
    }

    /**
     * Assumes that the expression as a whole has already been visited
     */
    fn walk_binary_expression<R>(
        visitor: &mut impl ASTVisitor<R>,
        lhs: &mut SourceIDSpanned<Expression>,
        rhs: &mut SourceIDSpanned<Expression>,
    ) -> Option<R> {
        Self::walk_expression(visitor, lhs).or_else(|| Self::walk_expression(visitor, rhs))
    }

    pub fn walk_case<R>(
        visitor: &mut impl ASTVisitor<R>,
        case: &mut SourceIDSpanned<Node>,
    ) -> Option<R> {
        return visitor.visit_case(case).or_else(|| match &mut case.inner {
            Node::Case {
                value: _value,
                body,
            } => {
                return body
                    .iter_mut()
                    .find_map(|statement| Self::walk_statement(visitor, statement));
            }
            Node::DefaultCase(_body) => {
                todo!("default case")
            }
            _ => {
                unreachable!("Node::walk_case called with a non-case node")
            }
        });
    }
}

// ****************************************** TESTS ***********************************************

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{ast::Literal, span::SourceIDSpan};
    use chumsky::span::SpanWrap;

    const DUMMY_SPAN: SourceIDSpan = SourceIDSpan {
        context: 0,
        start: 0,
        end: 0,
    };

    #[test]
    fn finds_a_function() {
        let function1 = Node::Function {
            name: "f1".to_string().with_span(DUMMY_SPAN),
            return_type_string: None,
            formals: vec![],
            body: Box::new(
                Node::FunctionBody(vec![Statement::EmptyStatement.with_span(DUMMY_SPAN)])
                    .with_span(DUMMY_SPAN),
            ),
        }
        .with_span(DUMMY_SPAN);
        let mut node = Node::Program(vec![function1]).with_span(DUMMY_SPAN);

        let first_function_name = Node::walk_program(
            &mut |node: &SourceIDSpanned<Node>| match &node.inner {
                Node::Function {
                    name,
                    return_type_string: _,
                    formals: _,
                    body: _,
                } => Some(name.inner.clone()),
                _ => None,
            },
            &mut node,
        )
        .unwrap();

        assert_eq!(first_function_name, "f1");
    }

    #[test]
    fn finds_a_node() {
        let string_literal = Expression::Literal(Literal::StringLiteral("test_string".to_string()))
            .with_span(DUMMY_SPAN);
        let function1 = Node::Function {
            name: "f1".to_string().with_span(DUMMY_SPAN),
            return_type_string: None,
            formals: vec![],
            body: Box::new(
                Node::FunctionBody(vec![Statement::EmptyStatement.with_span(DUMMY_SPAN)])
                    .with_span(DUMMY_SPAN),
            ),
        }
        .with_span(DUMMY_SPAN);
        let function2 = Node::Function {
            name: "f2".to_string().with_span(DUMMY_SPAN),
            return_type_string: None,
            formals: vec![],
            body: Box::new(
                Node::FunctionBody(vec![
                    Statement::ExpressionStatement(Box::new(string_literal.clone()))
                        .with_span(DUMMY_SPAN),
                ])
                .with_span(DUMMY_SPAN),
            ),
        }
        .with_span(DUMMY_SPAN);
        let mut node = Node::Program(vec![function1, function2]).with_span(DUMMY_SPAN);

        let first_string_literal_value = Node::walk_program(
            &mut |node: &SourceIDSpanned<Node>| match &node.inner {
                Node::Expression(Expression::Literal(Literal::StringLiteral(value))) => {
                    Some(value.clone())
                }
                _ => None,
            },
            &mut node,
        )
        .unwrap();

        assert_eq!(first_string_literal_value, "test_string");
    }
}
