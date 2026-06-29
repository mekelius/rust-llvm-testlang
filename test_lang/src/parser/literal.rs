use chumsky::prelude::*;

use crate::{
    ast::{Expression, Literal},
    parser::{ParserError, lexer::Token},
    span::{SourceIDSpan, SourceIDSpanned},
};

pub fn number_literal<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<Expression>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    select! {
        Token::NumberLiteral(value) => Expression::Literal(Literal::Number(value)),
    }
    .spanned()
}

pub fn string_literal<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<Expression>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    select! {
        Token::StringLiteral(value) => Expression::Literal(Literal::String(value)),
    }
    .spanned()
}

pub fn boolean_literal<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<Expression>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    select! {
        Token::True => Expression::Literal(Literal::Boolean(true)),
        Token::False => Expression::Literal(Literal::Boolean(false)),
    }
    .spanned()
}

pub fn unit_literal<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<Expression>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    just(Token::LParenthesis)
        .ignore_then(just(Token::RParenthesis))
        .to(Expression::Literal(Literal::Unit))
        .spanned()
}

pub fn literal<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<Expression>, ParserError<'src>> + Clone
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
