use std::error::Error;
use chumsky::Parser;
use logos::Logos;
use parser2::lexer::Token;
use parser2::parser::parser;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let source = io::read_to_string(io::stdin())?;

    let lexer = Token::lexer(&source);

    let mut tokens = vec![];
    for (token, span) in lexer.spanned() {
        match token {
            Ok(token) => tokens.push(token),
            Err(e) => {
                panic!("lexer error at {:?}: {:?}", span, e);
            }
        }
    }

    match parser().parse(&tokens).into_result() {
        Ok(expr) => {
            println!("[AST]\n{:#?}", expr);
            expr
        }
        Err(e) => {
            panic!("parse error: {:#?}", e);
        }
    };

    Ok(())
}