use logos::Logos;
use std::error::Error;
use std::io;
use testl::parser::lexer::Token;

fn main() -> Result<(), Box<dyn Error>> {
    let source = io::read_to_string(io::stdin())?;

    for token in Token::lexer(&source) {
        println!("{:#?}", token.map_err(|_| "Lexing failed")?)
    }

    Ok(())
}
