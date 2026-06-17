use std::error::Error;
use testl::parser;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let src = io::read_to_string(io::stdin())?;

    let ast = parser::run(&src);

    match ast {
        Ok(ast) => {
            println!("[AST]\n{:#?}", ast);
            ast
        }
        Err(e) => {
            panic!("parse error: {:#?}", e);
        }
    };

    Ok(())
}