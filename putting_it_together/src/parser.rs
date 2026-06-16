use crate::lexer;
use crate::lexer::Token;
use crate::lexer::TokenData;
use chumsky::prelude::*;
use std::error::Error;

use crate::ast::Node;

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

fn parser<'src>()
-> impl Parser<'src, &'src [Token], Node, chumsky::extra::Err<Rich<'src, Token>>> + Clone {
    let identifier = select! {
        Token::Identifier(TokenData {value}) => Node::Identifier(value),
    };

    let number_literal = select! {
        Token::NumberLiteral(TokenData {value}) => Node::NumberLiteral(value),
    };

    let string_literal = select! {
        Token::StringLiteral(TokenData {value}) => Node::StringLiteral(value),
    };

    let boolean_literal = select! {
        Token::True => Node::BooleanLiteral(true),
        Token::False => Node::BooleanLiteral(false),
    };

    let unary_operator = just(Token::Minus).to(Node::UnaryMinus);

    let binary_op_1 = choice((just(Token::Times), just(Token::Divided)));
    let binary_op_2 = choice((just(Token::Plus), just(Token::Minus)));
    let binary_op_3 = choice((
        just(Token::Equals),
        just(Token::GreaterThan),
        just(Token::LessThan),
        just(Token::GreaterThanOrEquals),
        just(Token::LessThanOrEquals),
        just(Token::NotEquals),
    ));

    let expression = recursive(|expr| {
        let argument_list = parenthesized(
            expr.clone()
                .separated_by(just(Token::Comma))
                .collect::<Vec<Node>>()
                .map(|e| Node::ArgumentList(e)),
        )
        .boxed();

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
            })
            .boxed();

        let unary_expression = unary_operator
            .clone()
            .then(expr.clone())
            .map(|(op, rhs)| Node::UnaryOperator {
                op: Box::new(op),
                rhs: Box::new(rhs),
            })
            .boxed();

        let literal = choice((
            string_literal.clone(),
            number_literal.clone(),
            boolean_literal.clone(),
        )).boxed();

        let typed_expression = select! {
            Token::TypeIdentifier(TokenData{value}) => value
        }
        .then(expr.clone())
        .map(|(type_, expression)| Node::TypedExpression(type_, Box::new(expression)));

        let term = choice((
            unary_expression.clone(),
            function_call.clone(),
            identifier.clone(),
            literal.clone(),
            typed_expression.clone(),
            parenthesized(expr.clone()),
        ))
        .boxed();

        let binary_expression_1 = term
            .clone()
            .foldl(
                binary_op_1.clone().then(term.clone()).repeated(),
                |lhs, (op, rhs)| match op {
                    Token::Times => Node::Mul(Box::new(lhs), Box::new(rhs)),
                    Token::Divided => Node::Div(Box::new(lhs), Box::new(rhs)),
                    _ => unreachable!(),
                },
            )
            .boxed();

        let binary_expression_2 = binary_expression_1
            .clone()
            .foldl(
                binary_op_2
                    .clone()
                    .then(binary_expression_1.clone())
                    .repeated(),
                |lhs, (op, rhs)| match op {
                    Token::Plus => Node::Add(Box::new(lhs), Box::new(rhs)),
                    Token::Minus => Node::Sub(Box::new(lhs), Box::new(rhs)),
                    _ => unreachable!(),
                },
            )
            .boxed();

        let binary_expression_3 = binary_expression_2
            .clone()
            .foldl(
                binary_op_3
                    .clone()
                    .then(binary_expression_2.clone())
                    .repeated(),
                |lhs, (op, rhs)| match op {
                    Token::Equals => Node::Equals(Box::new(lhs), Box::new(rhs)),
                    Token::GreaterThan => Node::GreaterThan(Box::new(lhs), Box::new(rhs)),
                    Token::LessThan => Node::LessThan(Box::new(lhs), Box::new(rhs)),
                    Token::GreaterThanOrEquals => {
                        Node::GreaterThanOrEquals(Box::new(lhs), Box::new(rhs))
                    }
                    Token::LessThanOrEquals => Node::LessThanOrEquals(Box::new(lhs), Box::new(rhs)),
                    Token::NotEquals => Node::NotEquals(Box::new(lhs), Box::new(rhs)),
                    _ => unreachable!(),
                },
            )
            .boxed();

        let expression = choice((
            binary_expression_3,
            binary_expression_2,
            binary_expression_1,
            unary_expression,
            function_call,
            string_literal,
            number_literal,
            identifier,
            parenthesized(expr.clone()),
        ))
        .boxed();

        expression
    })
    .boxed();

    let statement = recursive(|p| {
        let while_ = just(Token::While)
            .ignore_then(parenthesized(expression.clone()))
            .then(p.clone())
            .map(|(condition, body)| Node::While {
                condition: Box::new(condition),
                body: Box::new(body),
            })
            .boxed();

        let empty_statement = empty().to(Node::EmptyStatement).boxed();

        let block = in_curly_braces(
            p.clone()
                .repeated()
                .collect::<Vec<Node>>()
                .map(|e| Node::Block(e)),
        )
        .boxed();

        let let_statement = just(Token::Let)
            .ignore_then(identifier.clone())
            .then_ignore(just(Token::Assign))
            .then(expression.clone())
            .map(|(name, value)| {
                let name = match name {
                    Node::Identifier(name) => name,
                    _ => unreachable!(),
                };

                Node::LetStatement(name, Box::new(value))
            })
            .boxed();

        let return_statement = just(Token::Return)
            .ignore_then(expression.clone())
            .map(|expr| Node::ReturnStatement(Box::new(expr)))
            .boxed();

        let expression_statement = expression
            .clone()
            .map(|e| Node::ExpressionStatement(Box::new(e)));

        let simple_statement = choice((
            let_statement.clone(),
            return_statement.clone(),
            expression_statement.clone(),
            empty_statement.clone(),
        ))
        .boxed();
        let single_statement = simple_statement.then_ignore(just(Token::Semicolon)).boxed();

        let complex_statement = while_;

        single_statement.or(block).or(complex_statement).boxed()
    })
    .boxed();

    let untyped_formal = select! {
        Token::Identifier(TokenData {value}) => Node::UntypedFormal(value)
    };

    let typed_formal = select! {Token::TypeIdentifier(TokenData {value}) => value}
        .clone()
        .then(select! {
            Token::Identifier(TokenData {value}) => value
        })
        .map(|(type_, name)| Node::TypedFormal(type_, name));

    let formal = typed_formal.clone().or(untyped_formal.clone());

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
        })
        .boxed();

    let program = function
        .repeated()
        .collect::<Vec<Node>>()
        .map(|e| Node::Program(e));

    program.boxed()
}

pub fn run(src: &str) -> Result<Node, Box<dyn Error>> {
    let tokens = match lexer::run(src) {
        Ok(tokens) => tokens,
        Err(_) => unreachable!(),
    };

    match parser().parse(&tokens).into_result() {
        Ok(result) => Ok(result),
        Err(e) => panic!("parse error: {:#?}", e),
    }
}
