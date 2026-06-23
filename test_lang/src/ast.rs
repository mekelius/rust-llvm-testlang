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

    Identifier(String),

    NumberLiteral(String),
    StringLiteral(String),
    BooleanLiteral(bool),
    UnitLiteral,

    TypedExpression(String, Box<SourceIDSpanned<Node>>),

    Formals(Vec<SourceIDSpanned<Node>>),
    UntypedFormal(String),
    TypedFormal(String, String),
    FunctionBody(Vec<SourceIDSpanned<Node>>),

    Block(Vec<SourceIDSpanned<Node>>),
    ExpressionStatement(Box<SourceIDSpanned<Node>>),

    WhileStatement {
        condition: Box<SourceIDSpanned<Node>>,
        body: Box<SourceIDSpanned<Node>>,
    },
    ForStatement {
        init: Box<SourceIDSpanned<Node>>,
        condition: Box<SourceIDSpanned<Node>>,
        step: Box<SourceIDSpanned<Node>>,
        body: Box<SourceIDSpanned<Node>>,
    },
    IfStatement {
        condition: Box<SourceIDSpanned<Node>>,
        body: Box<SourceIDSpanned<Node>>,
    },
    IfElseStatement(Box<SourceIDSpanned<Node>>, Box<SourceIDSpanned<Node>>),
    SwitchStatement {
        matched_value_expression: Box<SourceIDSpanned<Node>>,
        cases: Vec<SourceIDSpanned<Node>>,
    },
    Case {
        value: String,
        body: Vec<SourceIDSpanned<Node>>,
    },
    DefaultCase(Vec<SourceIDSpanned<Node>>),

    ContinueStatement,
    BreakStatement,

    EmptyStatement,

    LetStatement(String, Box<SourceIDSpanned<Node>>),
    ConstStatement(String, Box<SourceIDSpanned<Node>>),
    AssignmentStatement(String, Box<SourceIDSpanned<Node>>),

    ReturnStatement(Box<SourceIDSpanned<Node>>),
    ValuelessReturnStatement,

    ArgumentList(Vec<SourceIDSpanned<Node>>),
    FunctionCall {
        callee: String,
        argument_list: Vec<SourceIDSpanned<Node>>,
    },

    UnaryMinus(Box<SourceIDSpanned<Node>>),
    UnaryNot(Box<SourceIDSpanned<Node>>),

    Equals(Box<SourceIDSpanned<Node>>, Box<SourceIDSpanned<Node>>),
    GreaterThan(Box<SourceIDSpanned<Node>>, Box<SourceIDSpanned<Node>>),
    LessThan(Box<SourceIDSpanned<Node>>, Box<SourceIDSpanned<Node>>),
    GreaterThanOrEquals(Box<SourceIDSpanned<Node>>, Box<SourceIDSpanned<Node>>),
    LessThanOrEquals(Box<SourceIDSpanned<Node>>, Box<SourceIDSpanned<Node>>),
    NotEquals(Box<SourceIDSpanned<Node>>, Box<SourceIDSpanned<Node>>),
    And(Box<SourceIDSpanned<Node>>, Box<SourceIDSpanned<Node>>),
    Or(Box<SourceIDSpanned<Node>>, Box<SourceIDSpanned<Node>>),

    Mul(Box<SourceIDSpanned<Node>>, Box<SourceIDSpanned<Node>>),
    Div(Box<SourceIDSpanned<Node>>, Box<SourceIDSpanned<Node>>),
    Add(Box<SourceIDSpanned<Node>>, Box<SourceIDSpanned<Node>>),
    Sub(Box<SourceIDSpanned<Node>>, Box<SourceIDSpanned<Node>>),
    Mod(Box<SourceIDSpanned<Node>>, Box<SourceIDSpanned<Node>>),
}
