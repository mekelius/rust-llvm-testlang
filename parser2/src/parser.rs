use crate::lexer::Token;
use crate::lexer::TokenData;
use chumsky::prelude::*;

#[derive(Debug, Clone)]
pub enum Node {
    Program(Vec<Node>),
    Function {
        name: String,
        formals: Vec<Node>,
        body: Box<Node>,
    },
    Block(Vec<Node>),
    Expression(Vec<Node>),
    Identifier(String),
    Formals(Vec<Node>),
    Formal(String),
    FunctionBody(Vec<Node>),
    While {
        condition: Box<Node>,
        body: Box<Node>,
    },
    EmptyStatement,
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
}

fn parenthesized<'src>(
    a: impl Parser<'src, &'src [Token], Node, chumsky::extra::Err<Rich<'src, Token>>> + Clone,
) -> impl Parser<'src, &'src [Token], Node, chumsky::extra::Err<Rich<'src, Token>>> + Clone {
    a.delimited_by(just(Token::LParenthesis), just(Token::RParenthesis))
}

fn in_curly_braces<'src>(
    a: impl Parser<'src, &'src [Token], Node, chumsky::extra::Err<Rich<'src, Token>>> + Clone,
) -> impl Parser<'src, &'src [Token], Node, chumsky::extra::Err<Rich<'src, Token>>> + Clone {
    a.delimited_by(just(Token::LCurlyBrace), just(Token::RCurlyBrace))
}

pub fn parser<'src>()
-> impl Parser<'src, &'src [Token], Node, chumsky::extra::Err<Rich<'src, Token>>> + Clone {
    let identifier = select! {
        Token::Identifier(TokenData {value}) => Node::Identifier(value),
    };

    let unary_operator = just(Token::Minus).to(Node::UnaryMinus);

    let binary_op_1 = choice((
        just(Token::Equals),
        just(Token::GreaterThan),
        just(Token::LessThan),
        just(Token::GreaterThanOrEquals),
        just(Token::LessThanOrEquals),
    ));

    // let binary_op_2 = choice((
    //     just(Token::Times),
    //     just(Token::Division),
    // ));

    // let binary_op_3 = choice((
    //     just(Token::Plus),
    //     just(Token::Minus),
    // ));

    let expression = recursive(|expr| {
        let argument_list = parenthesized(
            expr.clone()
                .separated_by(just(Token::Comma))
                .collect::<Vec<Node>>()
                .map(|e| Node::ArgumentList(e)),
        );

        let function_call = identifier
            .then(argument_list)
            .map(|(identifier, arguments)| {
                let callee = match identifier {
                    Node::Identifier(value) => value,
                    _ => todo!(),
                };

                let argument_list = match arguments {
                    Node::ArgumentList(argument_list) => argument_list,
                    _ => todo!(),
                };

                Node::FunctionCall {
                    callee,
                    argument_list,
                }
            });

        let unary_expression =
            unary_operator
                .clone()
                .then(expr.clone())
                .map(|(op, rhs)| Node::UnaryOperator {
                    op: Box::new(op),
                    rhs: Box::new(rhs),
                });

        let term = choice((
            parenthesized(expr.clone()),
            unary_expression.clone(),
            function_call.clone(),
            identifier.clone(),
        ));

        let binary_expression_1 = term.clone().foldl(
            binary_op_1
                .clone()
                .then(term.clone())
                .repeated(),
            |lhs, (op, rhs)| match op {
                Token::Equals => Node::Equals(Box::new(lhs), Box::new(rhs)),
                Token::GreaterThan => Node::GreaterThan(Box::new(lhs), Box::new(rhs)),
                Token::LessThan => Node::LessThan(Box::new(lhs), Box::new(rhs)),
                Token::GreaterThanOrEquals => Node::GreaterThanOrEquals(Box::new(lhs), Box::new(rhs)),
                Token::LessThanOrEquals => Node::LessThanOrEquals(Box::new(lhs), Box::new(rhs)),
                _ => unreachable!(),
            },
        );

        let expression = choice((
            parenthesized(expr.clone()),
            function_call,
            binary_expression_1,
            unary_expression,
            identifier,
        ));

        expression
    });

    let statement = recursive(|p| {
        let while_ = just(Token::While)
            .ignore_then(parenthesized(expression.clone()))
            .then(p.clone())
            .map(|(condition, body)| Node::While {
                condition: Box::new(condition),
                body: Box::new(body),
            });

        let empty_statement = empty().to(Node::EmptyStatement);

        let block = in_curly_braces(
            p.clone()
                .repeated()
                .collect::<Vec<Node>>()
                .map(|e| Node::Block(e)),
        );

        let single_statement =
            ((expression.clone()).or(empty_statement)).then_ignore(just(Token::Semicolon));

        let complex_statement = while_;

        single_statement.or(block).or(complex_statement)
    });

    let formal = select! {
        Token::Identifier(TokenData {value}) => Node::Formal(value)
    };

    let function_body = in_curly_braces(
        statement
            .repeated()
            .collect::<Vec<Node>>()
            .map(|e| Node::FunctionBody(e)),
    );

    let formals = parenthesized(
        formal
            .separated_by(just(Token::Comma))
            .collect::<Vec<Node>>()
            .map(|e| Node::Formals(e)),
    );

    let function = just(Token::Function)
        .ignore_then(identifier)
        .then(formals)
        .then(function_body)
        .map(|((name, formals), function_body)| {
            let name = match name {
                Node::Identifier(value) => value,
                _ => unreachable!(),
            };

            let formals = match formals {
                Node::Formals(formals) => formals,
                _ => unreachable!(),
            };

            Node::Function {
                name,
                formals,
                body: Box::new(function_body),
            }
        });

    let program = function
        .repeated()
        .collect::<Vec<Node>>()
        .map(|e| Node::Program(e));

    program
}
