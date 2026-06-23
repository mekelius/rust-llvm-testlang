use chumsky::span::Spanned;

#[derive(Debug, Clone, PartialEq)]
pub struct SourceLocation {
    pub file: String,
    pub line: i32,
    pub row: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Program(Vec<Spanned<Node>>),

    Function {
        name: Spanned<String>,
        return_type_string: Option<Spanned<String>>,
        formals: Vec<Spanned<Node>>,
        body: Box<Spanned<Node>>,
    },
    
    Identifier(String),
    
    NumberLiteral(String),
    StringLiteral(String),
    BooleanLiteral(bool),
    UnitLiteral,

    TypedExpression(String, Box<Spanned<Node>>),

    Formals(Vec<Spanned<Node>>),
    UntypedFormal(String),
    TypedFormal(String, String),
    FunctionBody(Vec<Spanned<Node>>),
    
    Block(Vec<Spanned<Node>>),
    ExpressionStatement(Box<Spanned<Node>>),

    WhileStatement {
        condition: Box<Spanned<Node>>,
        body: Box<Spanned<Node>>,
    },
    ForStatement {
        init: Box<Spanned<Node>>,
        condition: Box<Spanned<Node>>,
        step: Box<Spanned<Node>>,
        body: Box<Spanned<Node>>,
    },
    IfStatement {
        condition: Box<Spanned<Node>>,
        body: Box<Spanned<Node>>,
    },
    IfElseStatement(Box<Spanned<Node>>, Box<Spanned<Node>>),
    SwitchStatement {
        matched_value_expression: Box<Spanned<Node>>,
        cases: Vec<Spanned<Node>>,
    },
    Case {
        value: String,
        body: Vec<Spanned<Node>>,
    },
    DefaultCase(Vec<Spanned<Node>>),

    ContinueStatement,
    BreakStatement,

    EmptyStatement,

    LetStatement(String, Box<Spanned<Node>>),
    ConstStatement(String, Box<Spanned<Node>>),
    AssignmentStatement(String, Box<Spanned<Node>>),

    ReturnStatement(Box<Spanned<Node>>),
    ValuelessReturnStatement,

    ArgumentList(Vec<Spanned<Node>>),
    FunctionCall {
        callee: String,
        argument_list: Vec<Spanned<Node>>,
    },

    UnaryMinus(Box<Spanned<Node>>),
    UnaryNot(Box<Spanned<Node>>),

    Equals(Box<Spanned<Node>>, Box<Spanned<Node>>),
    GreaterThan(Box<Spanned<Node>>, Box<Spanned<Node>>),
    LessThan(Box<Spanned<Node>>, Box<Spanned<Node>>),
    GreaterThanOrEquals(Box<Spanned<Node>>, Box<Spanned<Node>>),
    LessThanOrEquals(Box<Spanned<Node>>, Box<Spanned<Node>>),
    NotEquals(Box<Spanned<Node>>, Box<Spanned<Node>>),
    And(Box<Spanned<Node>>, Box<Spanned<Node>>),
    Or(Box<Spanned<Node>>, Box<Spanned<Node>>),

    Mul(Box<Spanned<Node>>, Box<Spanned<Node>>),
    Div(Box<Spanned<Node>>, Box<Spanned<Node>>),
    Add(Box<Spanned<Node>>, Box<Spanned<Node>>),
    Sub(Box<Spanned<Node>>, Box<Spanned<Node>>),
    Mod(Box<Spanned<Node>>, Box<Spanned<Node>>),
}