pub mod assignment;
pub mod common;
pub mod expression;
pub mod function;
pub mod lexer;
pub mod literal;
pub mod statement;
pub mod store_node;

use self::lexer::Token;
use chumsky::input::IterInput;
use chumsky::prelude::*;
use std::error::Error;

use crate::ast::Program;
use crate::ast_store::{ASTStore, FunctionID};
use crate::parser::function::function;
use crate::source::{BUILTINS_SOURCE_ID, SourceID};
use crate::span::{SourceIDSpan, SourceIDSpanned};

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
    if source_id == BUILTINS_SOURCE_ID {
        return Err("BUILTINS_SOURCE_ID is reserved for builtins".into());
    }

    let tokens = lexer::run(src, source_id)?;

    if tokens.is_empty() {
        return Ok((
            Program { functions: vec![] }.with_span(SourceIDSpan {
                context: source_id,
                start: 0,
                end: 0,
            }),
            ast_store,
        ));
    }

    let input = IterInput::new(
        tokens.iter().cloned(),
        tokens.last().expect("empty input is handled above").1,
    );
    let mut state = extra::SimpleState(ast_store);

    match parser().parse_with_state(input, &mut state).into_result() {
        Ok(result) => Ok((result, state.0)),
        Err(e) => Err(format!("parse error: {:#?}", e).into()),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn handles_empty_source() {
        let store = ASTStore::new();
        let source_id = SourceID::new(3);
        let (program, _store) = run("", source_id, store).expect("should parse");

        assert_eq!(program.inner, Program { functions: vec![] });
        assert_eq!(
            program.span,
            SourceIDSpan {
                context: source_id,
                start: 0,
                end: 0
            }
        );
    }

    #[test]
    fn handles_nonempty_zero_token_source_with_zero_span() {
        let store = ASTStore::new();
        let source_id = SourceID::new(3);

        let source = "\n\n   //jeejee    \n\n      /*somethingsomething(){\n\njoo\n  } kyl    \n\n    */    \n\n ";
        let (program, _store) = run(source, source_id, store).expect("should parse");

        assert_eq!(program.inner, Program { functions: vec![] });
        assert_eq!(
            program.span,
            SourceIDSpan {
                context: source_id,
                start: 0,
                end: 0
            }
        );
    }
}
