use chumsky::span::{SimpleSpan, Spanned};

pub type ByteOffset = usize;
pub type SourceID = usize;
pub type SourceIDSpan = SimpleSpan<ByteOffset, SourceID>;
pub type SpannedNode = Spanned<Node, SourceIDSpan>;
pub type SpannedString = Spanned<String, SourceIDSpan>;

#[derive(Debug, Clone, PartialEq)]
pub struct SourceLocation {
    pub file: String,
    pub line: i32,
    pub row: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Program(Vec<SpannedNode>),

    Function {
        name: SpannedString,
        return_type_string: Option<SpannedString>,
        formals: Vec<SpannedNode>,
        body: Box<SpannedNode>,
    },
    
    Identifier(String),
    
    NumberLiteral(String),
    StringLiteral(String),
    BooleanLiteral(bool),
    UnitLiteral,

    TypedExpression(String, Box<SpannedNode>),

    Formals(Vec<SpannedNode>),
    UntypedFormal(String),
    TypedFormal(String, String),
    FunctionBody(Vec<SpannedNode>),
    
    Block(Vec<SpannedNode>),
    ExpressionStatement(Box<SpannedNode>),

    WhileStatement {
        condition: Box<SpannedNode>,
        body: Box<SpannedNode>,
    },
    ForStatement {
        init: Box<SpannedNode>,
        condition: Box<SpannedNode>,
        step: Box<SpannedNode>,
        body: Box<SpannedNode>,
    },
    IfStatement {
        condition: Box<SpannedNode>,
        body: Box<SpannedNode>,
    },
    IfElseStatement(Box<SpannedNode>, Box<SpannedNode>),
    SwitchStatement {
        matched_value_expression: Box<SpannedNode>,
        cases: Vec<SpannedNode>,
    },
    Case {
        value: String,
        body: Vec<SpannedNode>,
    },
    DefaultCase(Vec<SpannedNode>),

    ContinueStatement,
    BreakStatement,

    EmptyStatement,

    LetStatement(String, Box<SpannedNode>),
    ConstStatement(String, Box<SpannedNode>),
    AssignmentStatement(String, Box<SpannedNode>),

    ReturnStatement(Box<SpannedNode>),
    ValuelessReturnStatement,

    ArgumentList(Vec<SpannedNode>),
    FunctionCall {
        callee: String,
        argument_list: Vec<SpannedNode>,
    },

    UnaryMinus(Box<SpannedNode>),
    UnaryNot(Box<SpannedNode>),

    Equals(Box<SpannedNode>, Box<SpannedNode>),
    GreaterThan(Box<SpannedNode>, Box<SpannedNode>),
    LessThan(Box<SpannedNode>, Box<SpannedNode>),
    GreaterThanOrEquals(Box<SpannedNode>, Box<SpannedNode>),
    LessThanOrEquals(Box<SpannedNode>, Box<SpannedNode>),
    NotEquals(Box<SpannedNode>, Box<SpannedNode>),
    And(Box<SpannedNode>, Box<SpannedNode>),
    Or(Box<SpannedNode>, Box<SpannedNode>),

    Mul(Box<SpannedNode>, Box<SpannedNode>),
    Div(Box<SpannedNode>, Box<SpannedNode>),
    Add(Box<SpannedNode>, Box<SpannedNode>),
    Sub(Box<SpannedNode>, Box<SpannedNode>),
    Mod(Box<SpannedNode>, Box<SpannedNode>),
}