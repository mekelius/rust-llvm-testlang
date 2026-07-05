use crate::{
    ast::{
        BinopExpression, Call, Case, Expression, Function, NodeRef, Program, Statement,
        UnopExpression,
    },
    ast_store::{ASTStore, ExpressionID, FunctionID, HasID, StatementID, Store},
    span::SourceIDSpanned,
};

pub trait ASTVisitor<R> {
    fn enter_program(&mut self, _program: &SourceIDSpanned<Program>) -> Option<R> {
        None
    }
    fn enter_function(&mut self, _function: (&SourceIDSpanned<Function>, FunctionID)) -> Option<R> {
        None
    }
    fn enter_statement(
        &mut self,
        _statement: (&SourceIDSpanned<Statement>, StatementID),
    ) -> Option<R> {
        None
    }
    fn enter_expression(
        &mut self,
        _expression: (&SourceIDSpanned<Expression>, ExpressionID),
    ) -> Option<R> {
        None
    }
    fn exit_program(&mut self, _program: &SourceIDSpanned<Program>) -> Option<R> {
        None
    }
    fn exit_function(&mut self, _function: (&SourceIDSpanned<Function>, FunctionID)) -> Option<R> {
        None
    }
    fn exit_statement(
        &mut self,
        _statement: (&SourceIDSpanned<Statement>, StatementID),
    ) -> Option<R> {
        None
    }
    fn exit_expression(
        &mut self,
        _expression: (&SourceIDSpanned<Expression>, ExpressionID),
    ) -> Option<R> {
        None
    }
}

pub trait ASTVisitorMut<R> {
    fn visit_program(&mut self, program: &mut SourceIDSpanned<Program>) -> Option<R>;
    fn visit_function(&mut self, function: &mut SourceIDSpanned<Function>) -> Option<R>;
    fn visit_statement(&mut self, statement: &mut SourceIDSpanned<Statement>) -> Option<R>;
    fn visit_expression(&mut self, expression: &mut SourceIDSpanned<Expression>) -> Option<R>;
}

pub trait MatchingASTVisitor<R>: ASTVisitor<R> {
    fn visit_node(&mut self, case: NodeRef) -> Option<R>;
}

impl<T, R> ASTVisitor<R> for T
where
    T: MatchingASTVisitor<R>,
{
    fn enter_program(&mut self, program: &SourceIDSpanned<Program>) -> Option<R> {
        self.visit_node(NodeRef::Program(program))
    }
    fn enter_function(&mut self, function: (&SourceIDSpanned<Function>, FunctionID)) -> Option<R> {
        self.visit_node(NodeRef::Function(function.value()))
    }
    fn enter_statement(
        &mut self,
        statement: (&SourceIDSpanned<Statement>, StatementID),
    ) -> Option<R> {
        self.visit_node(NodeRef::Statement(statement.value()))
    }
    fn enter_expression(
        &mut self,
        expression: (&SourceIDSpanned<Expression>, ExpressionID),
    ) -> Option<R> {
        self.visit_node(NodeRef::Expression(expression.value()))
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

impl ASTStore {
    pub fn walk_program<R>(
        &mut self,
        visitor: &mut impl ASTVisitor<R>,
        program: &mut SourceIDSpanned<Program>,
    ) -> Option<R> {
        program
            .inner
            .functions
            .iter_mut()
            .find_map(|function| self.walk_function(visitor, *function))
            .or_else(|| visitor.exit_program(program))
    }

    pub fn walk_function<R>(
        &mut self,
        visitor: &mut impl ASTVisitor<R>,
        function_id: FunctionID,
    ) -> Option<R> {
        let function = self.functions.get_node_mut(function_id);

        let result = visitor.enter_function((function, function_id));
        if result.is_some() {
            return result;
        }

        let body = function.inner.body.clone();

        let result = body
            .iter()
            .find_map(|statement| self.walk_statement(visitor, *statement));

        if result.is_some() {
            return result;
        }

        visitor.exit_function((self.functions.get_node(function_id), function_id))
    }

    pub fn walk_statement<R>(
        &mut self,
        visitor: &mut impl ASTVisitor<R>,
        statement_id: StatementID,
    ) -> Option<R> {
        let statement = self.statements.get_node(statement_id);

        let result = visitor.enter_statement((statement, statement_id));
        if result.is_some() {
            return result;
        }

        let result = match &statement.inner {
            Statement::Switch {
                matched_value_expression: _,
                cases: _,
            } => todo!("Switch"),
            _ => {
                let statement = statement.inner.clone();
                match statement {
                    Statement::Block(statements) => statements
                        .iter()
                        .find_map(|statement_id| self.walk_statement(visitor, *statement_id)),

                    Statement::Expression(expression) => self.walk_expression(visitor, expression),

                    Statement::While { condition, body } => self
                        .walk_expression(visitor, condition)
                        .or_else(|| self.walk_statement(visitor, body)),
                    Statement::For {
                        init,
                        condition,
                        step,
                        body,
                    } => self
                        .walk_statement(visitor, init)
                        .or_else(|| self.walk_expression(visitor, condition))
                        .or_else(|| self.walk_statement(visitor, step))
                        .or_else(|| self.walk_statement(visitor, body)),

                    Statement::If { condition, body } => self
                        .walk_expression(visitor, condition)
                        .or_else(|| self.walk_statement(visitor, body)),
                    Statement::IfElse(if_statement, else_statement) => self
                        .walk_statement(visitor, if_statement)
                        .or_else(|| self.walk_statement(visitor, else_statement)),
                    Statement::Switch {
                        matched_value_expression,
                        cases,
                    } => self
                        .walk_expression(visitor, matched_value_expression)
                        .or_else(|| cases.iter().find_map(|case| self.walk_case(visitor, case))),

                    Statement::Let(_, expression) => self.walk_expression(visitor, expression),
                    Statement::Const(_, expression) => self.walk_expression(visitor, expression),
                    Statement::Assignment(_identifier, expression) => {
                        self.walk_expression(visitor, expression)
                    }
                    Statement::Return(expression) => self.walk_expression(visitor, expression),

                    Statement::Empty => None,
                    Statement::Break => None,
                    Statement::Continue => None,
                }
            }
        };

        if result.is_some() {
            return result;
        }

        let statement = self.statements.get_node(statement_id);
        visitor.exit_statement((statement, statement_id))
    }

    fn walk_case<R>(&mut self, _visitor: &mut impl ASTVisitor<R>, _case: &Case) -> Option<R> {
        todo!("Walking cases");
    }

    pub fn walk_expression<R>(
        &mut self,
        visitor: &mut impl ASTVisitor<R>,
        expression_id: ExpressionID,
    ) -> Option<R> {
        let expression = self.expressions.get_node(expression_id);

        let result = visitor.enter_expression((expression, expression_id));
        if result.is_some() {
            return result;
        }

        let result = match &expression.inner {
            Expression::Binop(BinopExpression { op: _, lhs, rhs }) => {
                let lhs = lhs.clone();
                let rhs = rhs.clone();
                self.walk_binary_expression(visitor, lhs, rhs)
            }
            Expression::Call(Call { callee: _, args }) => {
                let args = args.clone();
                args.iter().find_map(|argument_expression| {
                    self.walk_expression(visitor, *argument_expression)
                })
            }
            Expression::Unop(UnopExpression { op: _, term }) => {
                let term = term.clone();
                self.walk_expression(visitor, term)
            }
            Expression::Literal(_) => None,
            Expression::Identifier(_) => None,
            Expression::PropertyAccess(dot_access) => {
                let expression_id = dot_access.dot_subscriptable.clone();
                self.walk_expression(visitor, expression_id)
            }
            Expression::TypedExpression(_type_, expression) => {
                let expression_id = expression.clone();
                self.walk_expression(visitor, expression_id)
            }
        };

        if result.is_some() {
            return result;
        }

        let expression = self.expressions.get_node(expression_id);
        visitor.exit_expression((expression, expression_id))
    }

    /**
     * Assumes that the expression as a whole has already been visited
     */
    fn walk_binary_expression<R>(
        &mut self,
        visitor: &mut impl ASTVisitor<R>,
        lhs: ExpressionID,
        rhs: ExpressionID,
    ) -> Option<R> {
        self.walk_expression(visitor, lhs)
            .or_else(|| self.walk_expression(visitor, rhs))
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
        let mut store = ASTStore::new();

        let function1 = Function {
            name: "f1".to_string().with_span(DUMMY_SPAN),
            return_type_string: None,
            formals: vec![],
            body: vec![store.statements.add(Statement::Empty.with_span(DUMMY_SPAN))],
        }
        .with_span(DUMMY_SPAN);
        let mut node = Program {
            functions: vec![store.functions.add(function1)],
        }
        .with_span(DUMMY_SPAN);

        let first_function_name = store
            .walk_program(
                &mut |node: NodeRef| match node {
                    NodeRef::Function(function) => function.inner.name.inner.clone().into(),
                    _ => None,
                },
                &mut node,
            )
            .expect("should have a function");

        assert_eq!(first_function_name, "f1");
    }

    #[test]
    fn finds_a_node() {
        let mut store = ASTStore::new();

        let string_literal = store.expressions.add(
            Expression::Literal(Literal::String("test_string".to_string())).with_span(DUMMY_SPAN),
        );
        let function1 = store.functions.add(
            Function {
                name: "f1".to_string().with_span(DUMMY_SPAN),
                return_type_string: None,
                formals: vec![],
                body: vec![store.statements.add(Statement::Empty.with_span(DUMMY_SPAN))],
            }
            .with_span(DUMMY_SPAN),
        );
        let function2 = store.functions.add(
            Function {
                name: "f2".to_string().with_span(DUMMY_SPAN),
                return_type_string: None,
                formals: vec![],
                body: vec![
                    store
                        .statements
                        .add(Statement::Expression(string_literal.clone()).with_span(DUMMY_SPAN)),
                ],
            }
            .with_span(DUMMY_SPAN),
        );
        let mut node = Program {
            functions: vec![function1, function2],
        }
        .with_span(DUMMY_SPAN);

        let first_string_literal_value = store
            .walk_program(
                &mut |node: NodeRef| match node {
                    NodeRef::Expression(SourceIDSpanned {
                        inner: Expression::Literal(Literal::String(value)),
                        span: _,
                    }) => Some(value.clone()),
                    _ => None,
                },
                &mut node,
            )
            .expect("should find a string");

        assert_eq!(first_string_literal_value, "test_string");
    }
}
