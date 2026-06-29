use crate::span::SourceIDSpanned;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Program(Program),
    Function(Function),
    Statement(Statement),
    Expression(Expression),
    Case(Case),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub functions: Vec<SourceIDSpanned<Function>>,
}

// ******************************************* FUNCTION *******************************************

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: SourceIDSpanned<String>,
    pub return_type_string: Option<SourceIDSpanned<String>>,
    pub formals: Vec<SourceIDSpanned<Parameter>>,
    pub body: Vec<SourceIDSpanned<Statement>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Parameter {
    Untyped(String),
    Typed(SourceIDSpanned<String>, SourceIDSpanned<String>),
}

// ******************************************* STATEMENT ******************************************

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Empty,
    Block(Vec<SourceIDSpanned<Statement>>),
    ExpressionStatement(Box<SourceIDSpanned<Expression>>),
    
    Let(String, Box<SourceIDSpanned<Expression>>),
    Const(String, Box<SourceIDSpanned<Expression>>),
    Assignment(String, Box<SourceIDSpanned<Expression>>),
    Return(Box<SourceIDSpanned<Expression>>),
    ValuelessReturn,

    While {
        condition: Box<SourceIDSpanned<Expression>>,
        body: Box<SourceIDSpanned<Statement>>,
    },
    For {
        init: Box<SourceIDSpanned<Statement>>,
        condition: Box<SourceIDSpanned<Expression>>,
        step: Box<SourceIDSpanned<Statement>>,
        body: Box<SourceIDSpanned<Statement>>,
    },
    If {
        condition: Box<SourceIDSpanned<Expression>>,
        body: Box<SourceIDSpanned<Statement>>,
    },
    IfElse(
        Box<SourceIDSpanned<Statement>>,
        Box<SourceIDSpanned<Statement>>,
    ),
    Switch {
        matched_value_expression: Box<SourceIDSpanned<Expression>>,
        cases: Vec<SourceIDSpanned<Case>>,
    },

    Continue,
    Break,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Case {
    pub matched_value: Option<SourceIDSpanned<String>>,
    pub body: Vec<SourceIDSpanned<Statement>>,
}

pub const DEFAULT_CASE: Option<SourceIDSpanned<String>> = None;

// ****************************************** EXPRESSION ******************************************

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    TypedExpression(SourceIDSpanned<String>, Box<SourceIDSpanned<Expression>>),
    Call(Call),
    Binop(BinopExpression),
    Unop(UnopExpression),
    Identifier(String),
    Literal(Literal),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(String),
    String(String),
    Boolean(bool),
    Unit,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Call {
    pub callee: SourceIDSpanned<String>,
    pub args: Vec<SourceIDSpanned<Expression>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinopExpression {
    pub op: BinaryOperator,
    pub lhs: Box<SourceIDSpanned<Expression>>,
    pub rhs: Box<SourceIDSpanned<Expression>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Equals,
    GreaterThan,
    LessThan,
    GreaterThanOrEquals,
    LessThanOrEquals,
    NotEquals,
    And,
    Or,
    Mul,
    Div,
    Add,
    Sub,
    Mod,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnopExpression {
    pub op: UnaryOperator,
    pub term: Box<SourceIDSpanned<Expression>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    UnaryMinus,
    UnaryNot,
}
