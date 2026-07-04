use chumsky::span::SpanWrap;

use crate::{
    ast::{Expression, Function, Literal, Statement},
    source::BUILTINS_SOURCE_ID,
    span::{SourceIDSpan, SourceIDSpanned},
};

pub trait HasID {
    type Value;
    type ID;

    fn id(self) -> Self::ID;
    fn value(self) -> Self::Value;
}

impl<Value, ID> HasID for (Value, ID)
where
    ID: Copy,
{
    type Value = Value;
    type ID = ID;

    fn id(self) -> Self::ID {
        self.1
    }
    fn value(self) -> Self::Value {
        self.0
    }
}

pub trait Store<N> {
    type ID;

    fn add(&mut self, node: N) -> Self::ID;
    fn get_node(&self, id: Self::ID) -> &N;
    fn get_node_mut(&mut self, id: Self::ID) -> &mut N;
}

impl<N> Store<N> for Vec<N> {
    type ID = usize;

    fn add(&mut self, node: N) -> Self::ID {
        self.push(node);
        self.len() - 1
    }
    fn get_node(&self, id: Self::ID) -> &N {
        &self[id]
    }
    fn get_node_mut(&mut self, id: Self::ID) -> &mut N {
        &mut self[id]
    }
}

pub type FunctionID = usize;
pub type StatementID = usize;
pub type ExpressionID = usize;

pub struct ASTStore {
    pub functions: Vec<SourceIDSpanned<Function>>,
    pub statements: Vec<SourceIDSpanned<Statement>>,
    pub expressions: Vec<SourceIDSpanned<Expression>>,
}

pub const PREDEFINED_LITERAL_SPAN: SourceIDSpan = SourceIDSpan {
    context: BUILTINS_SOURCE_ID,
    start: 0,
    end: 0,
};

impl ASTStore {
    pub fn new() -> ASTStore {
        let mut store = ASTStore {
            functions: vec![],
            statements: vec![],
            expressions: vec![],
        };

        store
            .expressions
            .add(Expression::Literal(Literal::Unit).with_span(PREDEFINED_LITERAL_SPAN));
        store
            .expressions
            .add(Expression::Literal(Literal::Boolean(true)).with_span(PREDEFINED_LITERAL_SPAN));
        store
            .expressions
            .add(Expression::Literal(Literal::Boolean(false)).with_span(PREDEFINED_LITERAL_SPAN));
        store.expressions.add(
            Expression::Literal(Literal::Number("1".to_string()))
                .with_span(PREDEFINED_LITERAL_SPAN),
        );

        store
    }
}

pub const UNIT_LITERAL: ExpressionID = 0;
pub const TRUE_LITERAL: ExpressionID = 1;
pub const FALSE_LITERAL: ExpressionID = 2;
pub const NUMBER_1_LITERAL: ExpressionID = 3;

impl ASTStore {
    pub fn get_function(&self, id: FunctionID) -> &Function {
        &self.functions[id]
    }
    pub fn get_function_mut(&mut self, id: FunctionID) -> &mut Function {
        &mut self.functions[id]
    }

    pub fn get_statement(&self, id: StatementID) -> &SourceIDSpanned<Statement> {
        &self.statements[id]
    }
    pub fn get_statement_mut(&mut self, id: StatementID) -> &mut Statement {
        &mut self.statements[id]
    }

    pub fn get_expression(&self, id: ExpressionID) -> &Expression {
        &self.expressions[id]
    }
    pub fn get_expression_mut(&mut self, id: ExpressionID) -> &mut Expression {
        &mut self.expressions[id]
    }
}
