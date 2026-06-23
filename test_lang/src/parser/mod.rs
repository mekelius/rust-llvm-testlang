pub mod common;
pub mod expression;
pub mod function;
pub mod lexer;
pub mod statement;

use self::lexer::Token;
use chumsky::input::IterInput;
use chumsky::input::ValueInput;
use chumsky::prelude::*;
use chumsky::span::Span;
use std::error::Error;

use crate::ast::Node;
use crate::parser::function::function;

type ParserError<'tokens> = chumsky::extra::Err<Rich<'tokens, Token>>;

fn parser<'tokens, I>() -> impl Parser<'tokens, I, Spanned<Node>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SimpleSpan>,
{
    let function = function();

    let program = function
        .repeated()
        .collect::<Vec<Spanned<Node>>>()
        .map(|e| Node::Program(e));

    program.spanned()
}

pub fn run(src: &str) -> Result<Spanned<Node>, Box<dyn Error>> {
    let tokens = match lexer::run(src) {
        Ok(tokens) => tokens,
        Err(_) => unreachable!(),
    };

    let input = IterInput::new(tokens.iter().cloned(), tokens.last().unwrap().1);

    match parser().parse(input).into_result() {
        Ok(result) => Ok(result),
        Err(e) => panic!("parse error: {:#?}", e),
    }
}
