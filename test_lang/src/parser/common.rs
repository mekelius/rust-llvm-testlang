use chumsky::prelude::*;

use crate::{
    ast::{Expression, Node}, parser::{ParserError, lexer::Token}, span::{SourceIDSpan, SourceIDSpanned},
};

#[macro_export]
macro_rules! parenthesized {
    ($p:expr) => {
        ($p).delimited_by(just(Token::LParenthesis), just(Token::RParenthesis))
    };
}

#[macro_export]
macro_rules! in_curly_braces {
    ($p:expr) => {
        ($p).delimited_by(just(Token::LCurlyBrace), just(Token::RCurlyBrace))
    };
}

pub fn type_expression<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<String>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    select! {
        Token::TypeIdentifier(value) => value
    }
    .spanned()
}

pub fn identifier<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<Expression>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    select! {
        Token::Identifier(value) => Expression::Identifier(value),
    }
    .spanned()
}

pub fn identifier_as_string<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<String>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    select! {
        Token::Identifier(value) => value,
    }
    .spanned()
}

pub fn number_literal_as_string<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<String>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    select! {
        Token::NumberLiteral(value)=>value,
    }
    .spanned()
}
