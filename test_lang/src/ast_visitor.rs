use crate::{
    ast::{
        BinopExpression, Call, Case, DEFAULT_CASE, Expression, Function, Node, NodeRef, Program,
        Statement, UnopExpression,
    },
    span::SourceIDSpanned,
};

pub trait ASTVisitor<R> {
    fn visit_program(&mut self, program: &SourceIDSpanned<Program>) -> Option<R>;
    fn visit_function(&mut self, function: &SourceIDSpanned<Function>) -> Option<R>;
    fn visit_statement(&mut self, statement: &SourceIDSpanned<Statement>) -> Option<R>;
    fn visit_expression(&mut self, expression: &SourceIDSpanned<Expression>) -> Option<R>;
    fn visit_case(&mut self, case: &SourceIDSpanned<Case>) -> Option<R>;
}

pub trait MatchingASTVisitor<R>: ASTVisitor<R> {
    fn visit_node(&mut self, case: NodeRef) -> Option<R>;
}

impl<T, R> ASTVisitor<R> for T
where
    T: MatchingASTVisitor<R>,
{
    fn visit_program(&mut self, program: &SourceIDSpanned<Program>) -> Option<R> {
        self.visit_node(NodeRef::Program(program))
    }
    fn visit_function(&mut self, function: &SourceIDSpanned<Function>) -> Option<R> {
        self.visit_node(NodeRef::Function(function))
    }
    fn visit_statement(&mut self, statement: &SourceIDSpanned<Statement>) -> Option<R> {
        self.visit_node(NodeRef::Statement(statement))
    }
    fn visit_expression(&mut self, expression: &SourceIDSpanned<Expression>) -> Option<R> {
        self.visit_node(NodeRef::Expression(expression))
    }
    fn visit_case(&mut self, case: &SourceIDSpanned<Case>) -> Option<R> {
        self.visit_node(NodeRef::Case(case))
    }
}

/**
 * Helper trait
 * */
trait ASTVisitResult<R> {
    fn into_option(self) -> Option<R>;
}

impl<T, R, RH> MatchingASTVisitor<R> for T
where
    T: Fn(NodeRef) -> RH,
    RH: ASTVisitResult<R>,
{
    fn visit_node(&mut self, node: NodeRef) -> Option<R> {
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
        program: &mut SourceIDSpanned<Program>,
    ) -> Option<R> {
        visitor.visit_program(program).or_else(|| {
            program
                .inner
                .functions
                .iter_mut()
                .find_map(|function| Self::walk_function(visitor, function))
        })
    }

    pub fn walk_function<R>(
        visitor: &mut impl ASTVisitor<R>,
        function: &mut SourceIDSpanned<Function>,
    ) -> Option<R> {
        return visitor.visit_function(function).or_else(|| {
            function
                .inner
                .body
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

                Statement::Empty => None,
                Statement::ExpressionStatement(expression) => {
                    Self::walk_expression(visitor, &mut **expression)
                }

                Statement::While { condition, body } => {
                    Self::walk_expression(visitor, &mut **condition)
                        .or_else(|| Self::walk_statement(visitor, body))
                }
                Statement::For {
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

                Statement::If { condition, body } => {
                    return Self::walk_expression(visitor, &mut **condition)
                        .or_else(|| Self::walk_statement(visitor, body));
                }
                Statement::IfElse(if_statement, else_statement) => {
                    return Self::walk_statement(visitor, if_statement)
                        .or_else(|| Self::walk_statement(visitor, else_statement));
                }
                Statement::Switch {
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

                Statement::Let(_, expression) => {
                    return Self::walk_expression(visitor, &mut **expression);
                }
                Statement::Const(_, expression) => {
                    return Self::walk_expression(visitor, &mut **expression);
                }
                Statement::Assignment(_identifier, expression) => {
                    return Self::walk_expression(visitor, &mut **expression);
                }
                Statement::Return(expression) => {
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
                Expression::Call(Call {
                    callee: _,
                    args: argument_list,
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
        case: &mut SourceIDSpanned<Case>,
    ) -> Option<R> {
        return visitor.visit_case(case).or_else(|| match &mut case.inner {
            Case {
                matched_value: Some(_),
                body,
            } => {
                return body
                    .iter_mut()
                    .find_map(|statement| Self::walk_statement(visitor, statement));
            }

            Case {
                matched_value: DEFAULT_CASE,
                body: _,
            } => {
                todo!("default case")
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
        let function1 = Function {
            name: "f1".to_string().with_span(DUMMY_SPAN),
            return_type_string: None,
            formals: vec![],
            body: vec![Statement::Empty.with_span(DUMMY_SPAN)],
        }
        .with_span(DUMMY_SPAN);
        let mut node = Program {
            functions: vec![function1],
        }
        .with_span(DUMMY_SPAN);

        let first_function_name = Node::walk_program(
            &mut |node: NodeRef| match node {
                NodeRef::Function(function) => function.inner.name.inner.clone().into(),
                _ => None,
            },
            &mut node,
        )
        .unwrap();

        assert_eq!(first_function_name, "f1");
    }

    #[test]
    fn finds_a_node() {
        let string_literal =
            Expression::Literal(Literal::String("test_string".to_string())).with_span(DUMMY_SPAN);
        let function1 = Function {
            name: "f1".to_string().with_span(DUMMY_SPAN),
            return_type_string: None,
            formals: vec![],
            body: vec![Statement::Empty.with_span(DUMMY_SPAN)],
        }
        .with_span(DUMMY_SPAN);
        let function2 = Function {
            name: "f2".to_string().with_span(DUMMY_SPAN),
            return_type_string: None,
            formals: vec![],
            body: vec![
                Statement::ExpressionStatement(Box::new(string_literal.clone()))
                    .with_span(DUMMY_SPAN),
            ],
        }
        .with_span(DUMMY_SPAN);
        let mut node = Program {
            functions: vec![function1, function2],
        }
        .with_span(DUMMY_SPAN);

        let first_string_literal_value = Node::walk_program(
            &mut |node: NodeRef| match node {
                NodeRef::Expression(SourceIDSpanned {
                    inner: Expression::Literal(Literal::String(value)),
                    span: _,
                }) => Some(value.clone()),
                _ => None,
            },
            &mut node,
        )
        .unwrap();

        assert_eq!(first_string_literal_value, "test_string");
    }
}
