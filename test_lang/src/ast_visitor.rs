use crate::{ast::Node, span::SourceIDSpanned};

pub trait ASTVisitor<R> {
    fn visit_program(&mut self, program: &SourceIDSpanned<Node>) -> Option<R>;
    fn visit_function(&mut self, function: &SourceIDSpanned<Node>) -> Option<R>;
    fn visit_statement(&mut self, statement: &SourceIDSpanned<Node>) -> Option<R>;
    fn visit_expression(&mut self, expression: &SourceIDSpanned<Node>) -> Option<R>;
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
    fn visit_statement(&mut self, statement: &SourceIDSpanned<Node>) -> Option<R> {
        self.visit_node(statement)
    }
    fn visit_expression(&mut self, expression: &SourceIDSpanned<Node>) -> Option<R> {
        self.visit_node(expression)
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

            Self::walk_statement(visitor, body)
        });
    }

    pub fn walk_statement<R>(
        visitor: &mut impl ASTVisitor<R>,
        statement: &mut SourceIDSpanned<Node>,
    ) -> Option<R> {
        println!("{:?}", statement);

        return visitor
            .visit_statement(statement)
            .or_else(|| match &mut statement.inner {
                Node::Block(statements) => statements
                    .iter_mut()
                    .find_map(|statement| Self::walk_statement(visitor, statement)),

                Node::EmptyStatement => None,
                Node::ExpressionStatement(expression) => {
                    Self::walk_expression(visitor, &mut **expression)
                }

                Node::WhileStatement { condition, body } => {
                    Self::walk_expression(visitor, &mut **condition)
                        .or_else(|| Self::walk_statement(visitor, body))
                }
                Node::ForStatement {
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

                Node::IfStatement { condition, body } => {
                    return Self::walk_expression(visitor, &mut **condition)
                        .or_else(|| Self::walk_statement(visitor, body));
                }
                Node::IfElseStatement(if_statement, else_statement) => {
                    return Self::walk_statement(visitor, if_statement)
                        .or_else(|| Self::walk_statement(visitor, else_statement));
                }
                Node::SwitchStatement {
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

                Node::LetStatement(_, expression) => {
                    return Self::walk_expression(visitor, &mut **expression);
                }
                Node::ConstStatement(_, expression) => {
                    return Self::walk_expression(visitor, &mut **expression);
                }
                Node::AssignmentStatement(_identifier, expression) => {
                    return Self::walk_expression(visitor, &mut **expression);
                }
                Node::ReturnStatement(expression) => {
                    return Self::walk_expression(visitor, &mut **expression);
                }
                _ => {
                    unreachable!("Node::walk_statement called with a non-statement node")
                }
            });
    }

    pub fn walk_expression<R>(
        visitor: &mut impl ASTVisitor<R>,
        expression: &mut SourceIDSpanned<Node>,
    ) -> Option<R> {
        return visitor
            .visit_expression(expression)
            .or_else(|| match &mut expression.inner {
                Node::TypedExpression(_type_, expression) => {
                    Self::walk_expression(visitor, expression)
                }
                Node::Identifier(_value) => None,
                Node::NumberLiteral(_value) => None,
                Node::StringLiteral(_value) => None,
                Node::BooleanLiteral(_value) => None,
                Node::UnaryMinus(rhs) => Self::walk_expression(visitor, rhs),
                Node::UnaryNot(rhs) => Self::walk_expression(visitor, rhs),
                Node::Equals(lhs, rhs) => {
                    Self::walk_binary_expression(visitor, &mut **lhs, &mut **rhs)
                }
                Node::GreaterThan(lhs, rhs) => {
                    Self::walk_binary_expression(visitor, &mut **lhs, &mut **rhs)
                }
                Node::LessThan(lhs, rhs) => {
                    Self::walk_binary_expression(visitor, &mut **lhs, &mut **rhs)
                }
                Node::GreaterThanOrEquals(lhs, rhs) => {
                    Self::walk_binary_expression(visitor, lhs, rhs)
                }
                Node::LessThanOrEquals(lhs, rhs) => {
                    Self::walk_binary_expression(visitor, &mut **lhs, &mut **rhs)
                }
                Node::NotEquals(lhs, rhs) => {
                    Self::walk_binary_expression(visitor, &mut **lhs, &mut **rhs)
                }
                Node::And(lhs, rhs) => {
                    Self::walk_binary_expression(visitor, &mut **lhs, &mut **rhs)
                }
                Node::Or(lhs, rhs) => Self::walk_binary_expression(visitor, &mut **lhs, &mut **rhs),
                Node::Mul(lhs, rhs) => {
                    Self::walk_binary_expression(visitor, &mut **lhs, &mut **rhs)
                }
                Node::Div(lhs, rhs) => {
                    Self::walk_binary_expression(visitor, &mut **lhs, &mut **rhs)
                }
                Node::Add(lhs, rhs) => {
                    Self::walk_binary_expression(visitor, &mut **lhs, &mut **rhs)
                }
                Node::Sub(lhs, rhs) => {
                    Self::walk_binary_expression(visitor, &mut **lhs, &mut **rhs)
                }
                Node::Mod(lhs, rhs) => {
                    Self::walk_binary_expression(visitor, &mut **lhs, &mut **rhs)
                }
                Node::UnitLiteral => None,
                Node::FunctionCall {
                    callee: _,
                    argument_list,
                } => argument_list.iter_mut().find_map(|argument_expression| {
                    Self::walk_expression(visitor, argument_expression)
                }),
                _ => unreachable!("Node::walk_expression called with a non-expression node"),
            });
    }

    /**
     * Assumes that the expression as a whole has already been visited
     */
    fn walk_binary_expression<R>(
        visitor: &mut impl ASTVisitor<R>,
        lhs: &mut SourceIDSpanned<Node>,
        rhs: &mut SourceIDSpanned<Node>,
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

    use crate::span::SourceIDSpan;
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
            body: Box::new(Node::EmptyStatement.with_span(DUMMY_SPAN)),
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
        let string_literal = Node::StringLiteral("test_string".to_string()).with_span(DUMMY_SPAN);
        let function1 = Node::Function {
            name: "f1".to_string().with_span(DUMMY_SPAN),
            return_type_string: None,
            formals: vec![],
            body: Box::new(Node::EmptyStatement.with_span(DUMMY_SPAN)),
        }
        .with_span(DUMMY_SPAN);
        let function2 = Node::Function {
            name: "f2".to_string().with_span(DUMMY_SPAN),
            return_type_string: None,
            formals: vec![],
            body: Box::new(
                Node::ExpressionStatement(Box::new(string_literal.clone())).with_span(DUMMY_SPAN),
            ),
        }
        .with_span(DUMMY_SPAN);
        let mut node = Node::Program(vec![function1, function2]).with_span(DUMMY_SPAN);

        let first_string_literal_value = Node::walk_program(
            &mut |node: &SourceIDSpanned<Node>| match &node.inner {
                Node::StringLiteral(value) => Some(value.clone()),
                _ => None,
            },
            &mut node,
        )
        .unwrap();

        assert_eq!(first_string_literal_value, "test_string");
    }
}
