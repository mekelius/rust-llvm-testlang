use std::error::Error;
use std::io;
use testl::{ast_store::ASTStore, parser, source::SourceID};

fn main() -> Result<(), Box<dyn Error>> {
    let src = io::read_to_string(io::stdin())?;

    match parser::run(&src, SourceID::new(1), ASTStore::new()) {
        Ok((program, _ast_store)) => {
            println!("[AST]\n{:#?}", program);
        }
        Err(e) => {
            panic!("parse error: {:#?}", e);
        }
    };

    Ok(())
}
