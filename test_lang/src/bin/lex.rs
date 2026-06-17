use std::error::Error;
use std::io;
use testl::parser::lexer::Token;
use logos::Logos;

fn main() -> Result<(), Box<dyn Error>> {
    let source = io::read_to_string(io::stdin())?;

    for result in Token::lexer(&source) {
        match result {
            Ok(token) => println!("{:#?}", token),
            Err(e) => panic!("Lexing failed: {:?}", e),
        }
    };

    Ok(())
}
