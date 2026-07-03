use chumsky::prelude::*;

use crate::{
    ast::Expression,
    parser::{Extras, lexer::Token},
    span::{SourceIDSpan, SourceIDSpanned},
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

pub fn type_expression<'tokens, I>()
-> impl Parser<'tokens, I, SourceIDSpanned<String>, Extras<'tokens>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
{
    select! {
        Token::TypeIdentifier(value) => value
    }
    .spanned()
}

pub fn identifier<'tokens, I>()
-> impl Parser<'tokens, I, SourceIDSpanned<Expression>, Extras<'tokens>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
{
    select! {
        Token::Identifier(value) => Expression::Identifier(value),
    }
    .spanned()
}

pub fn identifier_as_string<'tokens, I>()
-> impl Parser<'tokens, I, SourceIDSpanned<String>, Extras<'tokens>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
{
    select! {
        Token::Identifier(value) => value,
    }
    .spanned()
}

pub fn number_literal_as_string<'tokens, I>()
-> impl Parser<'tokens, I, SourceIDSpanned<String>, Extras<'tokens>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
{
    select! {
        Token::NumberLiteral(value) => value,
    }
    .spanned()
}
