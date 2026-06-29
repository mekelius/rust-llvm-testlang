use crate::{
    span::SourceIDSpanned,
    types::SimpleType::{self, Boolean, Int, Unknown, Void},
};

// macro_rules! node_types {
//     ($node: item) => {
//         #[derive(Debug, Clone, PartialEq)]
//         pub enum Node {
//             $node($node),
//         }

//         #[derive(Debug, Clone, PartialEq)]
//         pub enum NodeRef {
//             $node($node)
//         }
//     };
// }

// node_types!(Program, Function);

// node_types!(Program, Function, Statement, Expression, Case);

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Program(SourceIDSpanned<Program>),
    Function(SourceIDSpanned<Function>),
    Statement(SourceIDSpanned<Statement>),
    Expression(SourceIDSpanned<Expression>),
    Case(SourceIDSpanned<Case>),
    /*
    Literal
    Binop
    Unop
    Identifier
    Assignment
    Let
    Const
    Type
    Call
    ControlStatement
    ExpressionStatement
    EmptyStatement
    Block
    */
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeRef<'a> {
    Program(&'a SourceIDSpanned<Program>),
    Function(&'a SourceIDSpanned<Function>),
    Statement(&'a SourceIDSpanned<Statement>),
    Expression(&'a SourceIDSpanned<Expression>),
    Case(&'a SourceIDSpanned<Case>),
}

#[derive(Debug, PartialEq)]
pub enum NodeRefMut<'a> {
    Program(&'a mut SourceIDSpanned<Program>),
    Function(&'a mut SourceIDSpanned<Function>),
    Statement(&'a mut SourceIDSpanned<Statement>),
    Expression(&'a mut SourceIDSpanned<Expression>),
    Case(&'a mut SourceIDSpanned<Case>),
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
    Expression(Box<SourceIDSpanned<Expression>>),

    Let(Expression, Box<SourceIDSpanned<Expression>>),
    Const(Expression, Box<SourceIDSpanned<Expression>>),
    Assignment(Expression, Box<SourceIDSpanned<Expression>>),
    Return(Box<SourceIDSpanned<Expression>>),

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
    PropertyAccess(PropertyAccess),
    Literal(Literal),
}

impl Expression {
    pub fn get_type(&self) -> SimpleType {
        match self {
            Expression::TypedExpression(type_string, _expression) => {
                SimpleType::from_type_string(type_string)
            }
            Expression::Call(_call) => Unknown,
            Expression::Binop(expression) => expression.get_type(),
            Expression::Unop(expression) => expression.get_type(),
            Expression::Identifier(_value) => Unknown,
            Expression::PropertyAccess(_) => Unknown,
            Expression::Literal(Literal::Boolean(_)) => Boolean,
            Expression::Literal(Literal::String(_)) => SimpleType::String,
            Expression::Literal(Literal::Number(_)) => Int,
            Expression::Literal(Literal::Unit) => Void,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropertyAccess {
    pub object_expression: Box<SourceIDSpanned<Expression>>,
    pub property_name: SourceIDSpanned<String>,
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
    pub callee: Box<SourceIDSpanned<Expression>>,
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

impl BinopExpression {
    pub fn get_type(&self) -> SimpleType {
        match self.op {
            BinaryOperator::Equals => Boolean,
            BinaryOperator::GreaterThan => Boolean,
            BinaryOperator::LessThan => Boolean,
            BinaryOperator::GreaterThanOrEquals => Boolean,
            BinaryOperator::LessThanOrEquals => Boolean,
            BinaryOperator::NotEquals => Boolean,
            BinaryOperator::And => Boolean,
            BinaryOperator::Or => Boolean,
            BinaryOperator::Mul => Int,
            BinaryOperator::Div => Int,
            BinaryOperator::Add => Int,
            BinaryOperator::Sub => Int,
            BinaryOperator::Mod => Int,
        }
    }
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

impl UnopExpression {
    pub fn get_type(&self) -> SimpleType {
        match self.op {
            UnaryOperator::UnaryMinus => Int,
            UnaryOperator::UnaryNot => Boolean,
        }
    }
}
