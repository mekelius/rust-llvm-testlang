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

    // Values
    Identifier(String),

    NumberLiteral(String),
    StringLiteral(String),
    BooleanLiteral(bool),
    UnitLiteral,

    TypedExpression(SourceIDSpanned<String>, Box<SourceIDSpanned<Node>>),

    Formals(Vec<SourceIDSpanned<Node>>),
    UntypedFormal(String),
    TypedFormal(SourceIDSpanned<String>, SourceIDSpanned<String>),
    FunctionBody(Vec<SourceIDSpanned<Node>>),

    // Statements
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
        value: SourceIDSpanned<String>,
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

    // Expressions
    ArgumentList(Vec<SourceIDSpanned<Node>>),
    FunctionCall {
        callee: SourceIDSpanned<String>,
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

pub trait ASTVisitor {
    fn visit_program(&mut self, program: &SourceIDSpanned<Node>);
    fn visit_function(&mut self, function: &SourceIDSpanned<Node>);
    fn visit_statement(&mut self, statement: &SourceIDSpanned<Node>);
    fn visit_expression(&mut self, expression: &SourceIDSpanned<Node>);
    fn visit_case(&mut self, case: &SourceIDSpanned<Node>);
}

impl Node {
    pub fn walk_program(visitor: &mut impl ASTVisitor, program: &SourceIDSpanned<Node>) {
        visitor.visit_program(program);

        let Node::Program(functions) = &program.inner else {
            unreachable!("Node::walk_program called with a non-program node");
        };
        for function in functions {
            Self::walk_function(visitor, &function);
        }
    }

    pub fn walk_function(visitor: &mut impl ASTVisitor, function: &SourceIDSpanned<Node>) {
        visitor.visit_function(function);

        let Node::Function {
            name: _,
            return_type_string: _,
            formals: _,
            body,
        } = &function.inner
        else {
            unreachable!("Node::walk_function called with a non-function node")
        };
        Self::walk_statement(visitor, &body);
    }

    pub fn walk_statement(visitor: &mut impl ASTVisitor, statement: &SourceIDSpanned<Node>) {
        visitor.visit_statement(statement);

        match &statement.inner {
            Node::Block(statements) => {
                for statement in statements {
                    Self::walk_statement(visitor, &statement);
                }
            }
            Node::ExpressionStatement(expression) => {
                Self::walk_expression(visitor, &expression);
            }

            Node::WhileStatement { condition, body } => {
                Self::walk_expression(visitor, &condition);
                Self::walk_statement(visitor, &body);
            }
            Node::ForStatement {
                init,
                condition,
                step,
                body,
            } => {
                Self::walk_statement(visitor, &init);
                Self::walk_expression(visitor, &condition);
                Self::walk_statement(visitor, &step);
                Self::walk_statement(visitor, &body);
            }

            Node::IfStatement { condition, body } => {
                Self::walk_expression(visitor, &condition);
                Self::walk_statement(visitor, &body);
            }
            Node::IfElseStatement(if_statement, else_statement) => {
                Self::walk_statement(visitor, &if_statement);
                Self::walk_statement(visitor, &else_statement);
            }
            Node::SwitchStatement {
                matched_value_expression,
                cases,
            } => {
                Self::walk_expression(visitor, &matched_value_expression);

                for case in cases {
                    Self::walk_case(visitor, &case);
                }
            }

            Node::LetStatement(_, expression) => {
                Self::walk_expression(visitor, &expression);
            }
            Node::ConstStatement(_, expression) => {
                Self::walk_expression(visitor, &expression);
            }
            Node::AssignmentStatement(_identifier, expression) => {
                Self::walk_expression(visitor, &expression);
            }
            Node::ReturnStatement(expression) => {
                Self::walk_expression(visitor, &expression);
            }
            _ => {
                unreachable!("Node::walk_statement called with a non-statement node")
            }
        };
    }

    pub fn walk_expression(_visitor: &mut impl ASTVisitor, _expression: &SourceIDSpanned<Node>) {}
    pub fn walk_case(visitor: &mut impl ASTVisitor, case: &SourceIDSpanned<Node>) {
        visitor.visit_case(case);

        match &case.inner {
            Node::Case {
                value: _value,
                body,
            } => {
                for statement in body {
                    Self::walk_statement(visitor, &statement);
                }
            }
            Node::DefaultCase(_body) => {}
            _ => {
                unreachable!("Node::walk_case called with a non-case node")
            }
        }
    }
}
