use std::error::Error;
use std::io;
use testl::{ast_store::ASTStore, parser};

fn main() -> Result<(), Box<dyn Error>> {
    let src = io::read_to_string(io::stdin())?;

    match parser::run(&src, 1, ASTStore::new()) {
        Ok((program, ast_store)) => {
            println!("[AST]\n{:#?}", program);
        }
        Err(e) => {
            panic!("parse error: {:#?}", e);
        }
    };

    Ok(())
}
