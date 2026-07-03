pub mod assignment;
pub mod common;
pub mod expression;
pub mod function;
pub mod lexer;
pub mod literal;
pub mod lvalue;
pub mod statement;
pub mod store_node;

use self::lexer::Token;
use chumsky::input::IterInput;
use chumsky::prelude::*;
use std::error::Error;

use crate::ast::{Function, Program};
use crate::ast_store::{self, ASTStore, FunctionID};
use crate::parser::function::function;
use crate::source::SourceID;
use crate::span::{SourceIDSpan, SourceIDSpanned};

type ParserError<'tokens> = extra::Err<Rich<'tokens, Token, SourceIDSpan>>;
type Extras<'tokens> =
    extra::Full<Rich<'tokens, Token, SourceIDSpan>, extra::SimpleState<ASTStore>, ()>;

fn parser<'tokens, I>() -> impl Parser<'tokens, I, SourceIDSpanned<Program>, Extras<'tokens>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
{
    let function = function();

    let program = function
        .repeated()
        .collect::<Vec<FunctionID>>()
        .map(|functions| Program { functions });

    program.spanned()
}

pub fn run(
    src: &str,
    source_id: SourceID,
    ast_store: ASTStore,
) -> Result<(SourceIDSpanned<Program>, ASTStore), Box<dyn Error>> {
    if source_id == 0 {
        panic!("SourceID 0 is reserved for builtins");
    }

    let tokens = match lexer::run(src, source_id) {
        Ok(tokens) => tokens,
        Err(_) => unreachable!(),
    };

    let input = IterInput::new(tokens.iter().cloned(), tokens.last().unwrap().1);
    let mut state = extra::SimpleState(ast_store);

    match parser()
        .parse_with_state(input, &mut state)
        .into_result()
    {
        Ok(result) => {Ok((result, state.0))},
        Err(e) => panic!("parse error: {:#?}", e),
    }
}
