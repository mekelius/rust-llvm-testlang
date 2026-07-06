use std::error::Error;

use crate::{
    ast_store::{ExpressionID, FunctionID, StatementID},
    span::SourceIDSpanned,
    types::SimpleType::{self, Boolean, Int, Unknown},
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
}

#[derive(Debug, PartialEq)]
pub enum NodeRefMut<'a> {
    Program(&'a mut SourceIDSpanned<Program>),
    Function(&'a mut SourceIDSpanned<Function>),
    Statement(&'a mut SourceIDSpanned<Statement>),
    Expression(&'a mut SourceIDSpanned<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub functions: Vec<FunctionID>,
}

// ******************************************* FUNCTION *******************************************

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: SourceIDSpanned<String>,
    pub return_type_string: Option<SourceIDSpanned<String>>,
    pub params: Vec<SourceIDSpanned<Parameter>>,
    pub body: Vec<StatementID>,
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
    Block(Vec<StatementID>),
    Expression(ExpressionID),

    Let(ExpressionID, ExpressionID),
    Const(ExpressionID, ExpressionID),
    Assignment(ExpressionID, ExpressionID),
    Return(ExpressionID),

    While {
        condition: ExpressionID,
        body: StatementID,
    },
    For {
        init: StatementID,
        condition: ExpressionID,
        step: StatementID,
        body: StatementID,
    },
    If {
        condition: ExpressionID,
        body: StatementID,
    },
    IfElse(StatementID, StatementID),
    Switch {
        matched_value_expression: ExpressionID,
        cases: Vec<SourceIDSpanned<Case>>,
    },

    Continue,
    Break,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Case {
    pub matched_value: Option<SourceIDSpanned<String>>,
    pub body: Vec<StatementID>,
}

pub const DEFAULT_CASE: Option<SourceIDSpanned<String>> = None;

// ****************************************** EXPRESSION ******************************************

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    TypedExpression(SourceIDSpanned<String>, ExpressionID),
    Call(Call),
    Binop(BinopExpression),
    Unop(UnopExpression),
    Identifier(String),
    PropertyAccess(PropertyAccess),
    Literal(Literal),
}

impl Expression {
    pub fn get_actual_type(&self) -> Result<SimpleType, Box<dyn Error>> {
        match self {
            Expression::TypedExpression(type_string, _expression) => {
                SimpleType::from_type_string(type_string)
                    .ok_or_else(|| format!("invalid type {}", type_string.inner).into())
            }
            Expression::Call(_call) => Ok(Unknown),
            Expression::Binop(expression) => Ok(expression.get_actual_type()),
            Expression::Unop(expression) => Ok(expression.get_actual_type()),
            Expression::Identifier(_value) => Ok(Unknown),
            Expression::PropertyAccess(_) => Ok(Unknown),
            Expression::Literal(literal) => Ok(literal.get_type()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropertyAccess {
    pub dot_subscriptable: ExpressionID,
    pub property_name: SourceIDSpanned<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(String),
    String(String),
    Boolean(bool),
    Unit,
}

impl Literal {
    pub fn get_type(&self) -> SimpleType {
        match self {
            Literal::Boolean(_) => SimpleType::Boolean,
            Literal::String(_) => SimpleType::String,
            Literal::Number(_) => SimpleType::Int,
            Literal::Unit => SimpleType::Void,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Call {
    pub callee: ExpressionID,
    pub args: Vec<ExpressionID>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinopExpression {
    pub op: BinaryOperator,
    pub lhs: ExpressionID,
    pub rhs: ExpressionID,
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
    pub fn get_actual_type(&self) -> SimpleType {
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
    pub term: ExpressionID,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    UnaryMinus,
    UnaryNot,
}

impl UnopExpression {
    pub fn get_actual_type(&self) -> SimpleType {
        match self.op {
            UnaryOperator::UnaryMinus => Int,
            UnaryOperator::UnaryNot => Boolean,
        }
    }
}
