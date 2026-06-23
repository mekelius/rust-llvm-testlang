pub mod common;
pub mod expression;
pub mod function;
pub mod lexer;
pub mod statement;

use self::lexer::Token;
use chumsky::input::IterInput;
use chumsky::prelude::*;
use std::error::Error;

use crate::ast::{Node, SourceID, SourceIDSpan, SpannedNode};
use crate::parser::function::function;

type ParserError<'tokens> = chumsky::extra::Err<Rich<'tokens, Token, SourceIDSpan>>;

fn parser<'tokens, I>() -> impl Parser<'tokens, I, SpannedNode, ParserError<'tokens>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
{
    let function = function();

    let program = function
        .repeated()
        .collect::<Vec<SpannedNode>>()
        .map(|e| Node::Program(e));

    program.spanned()
}

pub fn run(src: &str, source_id: SourceID) -> Result<SpannedNode, Box<dyn Error>> {
    let tokens = match lexer::run(src, source_id) {
        Ok(tokens) => tokens,
        Err(_) => unreachable!(),
    };

    let input = IterInput::new(tokens.iter().cloned(), tokens.last().unwrap().1);

    match parser().parse(input).into_result() {
        Ok(result) => Ok(result),
        Err(e) => panic!("parse error: {:#?}", e),
    }
}
