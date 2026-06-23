use chumsky::prelude::*;

use crate::{
    ast::Node,
    parser::{ParserError, lexer::Token},
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

pub fn type_expression<'src, I>() -> impl Parser<'src, I, Spanned<String>> + Clone
where
    I: Input<'src, Token = Token, Span = SimpleSpan>,
{
    select! {
        Token::TypeIdentifier(value) => value
    }.spanned()
}

pub fn identifier<'src, I>() -> impl Parser<'src, I, Spanned<Node>> + Clone
where
    I: Input<'src, Token = Token, Span = SimpleSpan>,
{
    select! {
        Token::Identifier(value) => Node::Identifier(value),
    }
    .spanned()
}

pub fn identifier_as_string<'src, I>() -> impl Parser<'src, I, Spanned<String>> + Clone
where
    I: Input<'src, Token = Token, Span = SimpleSpan>,
{
    select! {
        Token::Identifier(value) => value,
    }
    .spanned()
}
