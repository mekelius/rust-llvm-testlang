#[derive(Debug, Clone)]
pub enum Node {
    Program(Vec<Node>),
    Function {
        name: String,
        formals: Vec<Node>,
        body: Box<Node>,
    },
    Block(Vec<Node>),
    ExpressionStatement(Box<Node>),
    Identifier(String),
    NumberLiteral(String),
    StringLiteral(String),
    Formals(Vec<Node>),
    Formal(String),
    FunctionBody(Vec<Node>),

    While {
        condition: Box<Node>,
        body: Box<Node>,
    },
    EmptyStatement,
    LetStatement(String, Box<Node>),
    ReturnStatement(Box<Node>),

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
    Times(Box<Node>, Box<Node>),
    Divided(Box<Node>, Box<Node>),
    Plus(Box<Node>, Box<Node>),
    Minus(Box<Node>, Box<Node>),
}