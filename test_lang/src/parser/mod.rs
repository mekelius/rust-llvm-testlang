pub mod common;
pub mod expression;
pub mod function;
pub mod lexer;
pub mod statement;

use self::lexer::Token;
use chumsky::prelude::*;
use std::error::Error;

use crate::ast::Node;
use crate::parser::function::function;

type ParserError<'src> = chumsky::extra::Err<Rich<'src, Token>>;

fn parser<'src>() -> impl Parser<'src, &'src [Token], Node, ParserError<'src>> + Clone {
    let function = function();

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
