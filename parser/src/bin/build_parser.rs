use cfgrammar::yacc::YaccKind;
use lrpar::CTParserBuilder;
use parser::lexer;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let source = io::read_to_string(io::stdin())?;

    CTParserBuilder::<lexer::Token>::new()
        .grammar_in_src_dir("grm.y")?
        .build()?;

    Ok(())
}
