use crate::span::SourceIDSpanned;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Program(Vec<SourceIDSpanned<Node>>),

    Function {
        name: SourceIDSpanned<String>,
        return_type_string: Option<SourceIDSpanned<String>>,
        formals: Vec<SourceIDSpanned<Node>>,
        body: Box<SourceIDSpanned<Node>>,
    },

    Formals(Vec<SourceIDSpanned<Node>>),
    UntypedFormal(String),
    TypedFormal(SourceIDSpanned<String>, SourceIDSpanned<String>),
    FunctionBody(Vec<SourceIDSpanned<Statement>>),

    Statement(Statement),
    Expression(Expression),

    Case {
        value: SourceIDSpanned<String>,
        body: Vec<SourceIDSpanned<Statement>>,
    },
    DefaultCase(Vec<SourceIDSpanned<Statement>>),
    ArgumentList(Vec<SourceIDSpanned<Expression>>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    TypedExpression(SourceIDSpanned<String>, Box<SourceIDSpanned<Expression>>),
    FunctionCall(FunctionCall),
    Binop(BinopExpression),
    Unop(UnopExpression),

    Identifier(String),
    Literal(Literal),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    NumberLiteral(String),
    StringLiteral(String),
    BooleanLiteral(bool),
    UnitLiteral,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub callee: SourceIDSpanned<String>,
    pub argument_list: Vec<SourceIDSpanned<Expression>>,
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
pub enum UnaryOperator {
    UnaryMinus,
    UnaryNot,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnopExpression {
    pub op: UnaryOperator,
    pub term: Box<SourceIDSpanned<Expression>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    ContinueStatement,
    BreakStatement,

    EmptyStatement,

    LetStatement(String, Box<SourceIDSpanned<Expression>>),
    ConstStatement(String, Box<SourceIDSpanned<Expression>>),
    AssignmentStatement(String, Box<SourceIDSpanned<Expression>>),

    ReturnStatement(Box<SourceIDSpanned<Expression>>),
    ValuelessReturnStatement,

    Block(Vec<SourceIDSpanned<Statement>>),
    ExpressionStatement(Box<SourceIDSpanned<Expression>>),

    WhileStatement {
        condition: Box<SourceIDSpanned<Expression>>,
        body: Box<SourceIDSpanned<Statement>>,
    },
    ForStatement {
        init: Box<SourceIDSpanned<Statement>>,
        condition: Box<SourceIDSpanned<Expression>>,
        step: Box<SourceIDSpanned<Statement>>,
        body: Box<SourceIDSpanned<Statement>>,
    },
    IfStatement {
        condition: Box<SourceIDSpanned<Expression>>,
        body: Box<SourceIDSpanned<Statement>>,
    },
    IfElseStatement(
        Box<SourceIDSpanned<Statement>>,
        Box<SourceIDSpanned<Statement>>,
    ),
    SwitchStatement {
        matched_value_expression: Box<SourceIDSpanned<Expression>>,
        cases: Vec<SourceIDSpanned<Node>>,
    },
}
