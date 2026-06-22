use chumsky::prelude::*;

use crate::{
    ast::Node,
    parser::{
        ParserError,
        lexer::{Token, TokenData},
    },
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

pub fn type_expression<'src>() -> impl Parser<'src, &'src [Token], String, ParserError<'src>> + Clone
{
    select! {
        Token::TypeIdentifier(TokenData{value}) => value
    }
}

pub fn identifier<'src>() -> impl Parser<'src, &'src [Token], Node, ParserError<'src>> + Clone {
    select! {
        Token::Identifier(TokenData {value}) => Node::Identifier(value),
    }
}

pub fn identifier_as_string<'src>()
-> impl Parser<'src, &'src [Token], String, ParserError<'src>> + Clone {
    select! {
        Token::Identifier(TokenData {value}) => value,
    }
}
