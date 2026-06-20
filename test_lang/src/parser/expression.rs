use chumsky::prelude::*;

use crate::{
    ast::Node,
    parenthesized,
    parser::{
        ParserError,
        common::{identifier, type_expression},
        lexer::{Token, TokenData},
    },
};

pub fn expression<'src>() -> impl Parser<'src, &'src [Token], Node, ParserError<'src>> + Clone {
    let identifier = identifier();

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

    let unit_literal = just(Token::LParenthesis)
        .ignore_then(just(Token::RParenthesis))
        .to(Node::UnitLiteral);

    let binary_op_1 = choice((just(Token::Asterisk), just(Token::Slash)));
    let binary_op_2 = choice((just(Token::Plus), just(Token::Minus)));
    let binary_op_3 = choice((
        just(Token::DoubleEquals),
        just(Token::GreaterThan),
        just(Token::LessThan),
        just(Token::GreaterThanOrEquals),
        just(Token::LessThanOrEquals),
        just(Token::NotEquals),
    ));

    recursive(|expr| {
        let argument_list = parenthesized!(
            expr.clone()
                .separated_by(just(Token::Comma))
                .collect::<Vec<Node>>()
                .map(|e| Node::ArgumentList(e))
        )
        .boxed();

        let function_call = identifier
            .clone()
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

        let literal = choice((
            string_literal.clone(),
            number_literal.clone(),
            boolean_literal.clone(),
            unit_literal,
        ))
        .boxed();

        let typed_expression = type_expression()
            .clone()
            .then(expr.clone())
            .map(|(type_, expression)| Node::TypedExpression(type_, Box::new(expression)));

        let term = recursive(|term| { 
            let unary_minus_expression = just(Token::Minus)
                .ignore_then(term.clone())
                .map(|expression| Node::UnaryMinus(Box::new(expression)));

            let unary_expression = unary_minus_expression;

            choice((
                unary_expression.clone(),
                function_call.clone(),
                identifier.clone(),
                literal.clone(),
                typed_expression.clone(),
                parenthesized!(expr.clone()),
            ))
        });

        let binary_expression_1 = term
            .clone()
            .foldl(
                binary_op_1.clone().then(term.clone()).repeated(),
                |lhs, (op, rhs)| match op {
                    Token::Asterisk => Node::Mul(Box::new(lhs), Box::new(rhs)),
                    Token::Slash => Node::Div(Box::new(lhs), Box::new(rhs)),
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
                    Token::DoubleEquals => Node::Equals(Box::new(lhs), Box::new(rhs)),
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
            term,
            parenthesized!(expr.clone()),
        ))
        .boxed();

        expression
    })
}
