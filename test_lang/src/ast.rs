#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Program(Vec<Node>),
    Function {
        name: String,
        return_type_string: Option<String>,
        formals: Vec<Node>,
        body: Box<Node>,
    },
    Block(Vec<Node>),
    ExpressionStatement(Box<Node>),
    Identifier(String),

    NumberLiteral(String),
    StringLiteral(String),
    BooleanLiteral(bool),
    UnitLiteral,

    TypedExpression(String, Box<Node>),

    Formals(Vec<Node>),
    UntypedFormal(String),
    TypedFormal(String, String),
    FunctionBody(Vec<Node>),

    WhileStatement {
        condition: Box<Node>,
        body: Box<Node>,
    },
    ForStatement {
        init: Box<Node>,
        condition: Box<Node>,
        step: Box<Node>,
        body: Box<Node>,
    },
    IfStatement {
        condition: Box<Node>,
        body: Box<Node>,
    },
    IfElseStatement(Box<Node>, Box<Node>),
    SwitchStatement {
        expression: Box<Node>,
        cases: Vec<Node>,
    },
    Case {
        case: Box<Node>,
        body: Box<Node>,
    },
    DefaultCase(Box<Node>),

    ContinueStatement,
    BreakStatement,

    EmptyStatement,
    LetStatement(String, Box<Node>),
    ReturnStatement(Box<Node>),
    ValuelessReturnStatement,

    ArgumentList(Vec<Node>),
    FunctionCall {
        callee: String,
        argument_list: Vec<Node>,
    },

    UnaryOperator {
        op: Box<Node>,
        rhs: Box<Node>,
    },
    UnaryMinus,

    Equals(Box<Node>, Box<Node>),
    GreaterThan(Box<Node>, Box<Node>),
    LessThan(Box<Node>, Box<Node>),
    GreaterThanOrEquals(Box<Node>, Box<Node>),
    LessThanOrEquals(Box<Node>, Box<Node>),
    NotEquals(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
}
