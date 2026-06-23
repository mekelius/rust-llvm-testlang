use chumsky::prelude::*;

use crate::{
    ast::{Node, SourceIDSpan, SpannedNode},
    parenthesized,
    parser::{
        ParserError,
        common::{identifier, type_expression},
        lexer::Token,
    },
};

pub fn number_literal<'src, I>() -> impl Parser<'src, I, SpannedNode, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    select! {
        Token::NumberLiteral(value) => Node::NumberLiteral(value),
    }
    .spanned()
}

pub fn string_literal<'src, I>() -> impl Parser<'src, I, SpannedNode, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    select! {
        Token::StringLiteral(value) => Node::StringLiteral(value),
    }
    .spanned()
}

pub fn boolean_literal<'src, I>() -> impl Parser<'src, I, SpannedNode, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    select! {
        Token::True => Node::BooleanLiteral(true),
        Token::False => Node::BooleanLiteral(false),
    }
    .spanned()
}

pub fn unit_literal<'src, I>() -> impl Parser<'src, I, SpannedNode, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    just(Token::LParenthesis)
        .ignore_then(just(Token::RParenthesis))
        .to(Node::UnitLiteral)
        .spanned()
}
pub fn binary_op_1<'src, I>() -> impl Parser<'src, I, Spanned<Token, SourceIDSpan>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    choice((
        just(Token::Asterisk),
        just(Token::Slash),
        just(Token::Percent),
    ))
    .spanned()
}

pub fn binary_op_2<'src, I>() -> impl Parser<'src, I, Spanned<Token, SourceIDSpan>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    choice((
        just(Token::Plus),
        just(Token::Minus),
        just(Token::DoubleAmpersand),
        just(Token::DoublePipe),
    ))
    .spanned()
}

pub fn binary_op_3<'src, I>() -> impl Parser<'src, I, Spanned<Token, SourceIDSpan>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    choice((
        just(Token::DoubleEquals),
        just(Token::GreaterThan),
        just(Token::LessThan),
        just(Token::GreaterThanOrEquals),
        just(Token::LessThanOrEquals),
        just(Token::NotEquals),
    ))
    .spanned()
}

pub fn literal<'src, I>() -> impl Parser<'src, I, SpannedNode, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    choice((
        string_literal(),
        number_literal(),
        boolean_literal(),
        unit_literal(),
    ))
}

pub fn expression<'src, I>() -> impl Parser<'src, I, SpannedNode, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    let identifier = identifier();

    recursive(|expr| {
        let argument_list = parenthesized!(
            expr.clone()
                .separated_by(just(Token::Comma))
                .collect::<Vec<SpannedNode>>()
                .map(|e| Node::ArgumentList(e))
        );

        let function_call = identifier
            .clone()
            .then(argument_list)
            .map(|(callee_expression, arguments)| {
                let callee = match callee_expression.inner {
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
            .spanned();

        let typed_expression = type_expression()
            .clone()
            .then(expr.clone())
            .map(|(type_, expression)| Node::TypedExpression(type_.inner, Box::new(expression)))
            .spanned();

        let term = recursive(|term| {
            let unary_not_expression = just(Token::Minus)
                .ignore_then(term.clone())
                .map(|expression| Node::UnaryMinus(Box::new(expression)))
                .spanned();

            let unary_minus_expression = just(Token::Bang)
                .ignore_then(term.clone())
                .map(|expression| Node::UnaryNot(Box::new(expression)))
                .spanned();

            let unary_expression = choice((unary_minus_expression, unary_not_expression));

            choice((
                unary_expression.clone(),
                function_call,
                identifier.clone(),
                literal(),
                typed_expression,
                parenthesized!(expr.clone()),
            ))
        });

        let binary_expression_1 = term.clone().foldl(
            binary_op_1().then(term.clone()).repeated(),
            |lhs, (op, rhs)| {
                let span = lhs.span.union(op.span).union(rhs.span);
                match op.inner {
                    Token::Asterisk => Node::Mul(Box::new(lhs), Box::new(rhs)),
                    Token::Slash => Node::Div(Box::new(lhs), Box::new(rhs)),
                    Token::Percent => Node::Mod(Box::new(lhs), Box::new(rhs)),
                    _ => unreachable!(),
                }
                .with_span(span)
            },
        );

        let binary_expression_2 = binary_expression_1.clone().foldl(
            binary_op_2().then(binary_expression_1.clone()).repeated(),
            |lhs, (op, rhs)| {
                let span = lhs.span.union(op.span).union(rhs.span);
                match op.inner {
                    Token::Plus => Node::Add(Box::new(lhs), Box::new(rhs)),
                    Token::Minus => Node::Sub(Box::new(lhs), Box::new(rhs)),
                    Token::DoubleAmpersand => Node::And(Box::new(lhs), Box::new(rhs)),
                    Token::DoublePipe => Node::Or(Box::new(lhs), Box::new(rhs)),
                    _ => unreachable!(),
                }
                .with_span(span)
            },
        );

        let binary_expression_3 = binary_expression_2.clone().foldl(
            binary_op_3().then(binary_expression_2.clone()).repeated(),
            |lhs, (op, rhs)| {
                let span = lhs.span.union(op.span).union(rhs.span);
                match op.inner {
                    Token::DoubleEquals => Node::Equals(Box::new(lhs), Box::new(rhs)),
                    Token::GreaterThan => Node::GreaterThan(Box::new(lhs), Box::new(rhs)),
                    Token::LessThan => Node::LessThan(Box::new(lhs), Box::new(rhs)),
                    Token::GreaterThanOrEquals => {
                        Node::GreaterThanOrEquals(Box::new(lhs), Box::new(rhs))
                    }
                    Token::LessThanOrEquals => Node::LessThanOrEquals(Box::new(lhs), Box::new(rhs)),
                    Token::NotEquals => Node::NotEquals(Box::new(lhs), Box::new(rhs)),
                    _ => unreachable!(),
                }
                .with_span(span)
            },
        );

        choice((
            binary_expression_3,
            binary_expression_2,
            binary_expression_1,
            term,
            parenthesized!(expr.clone()),
        ))
    })
}
