// use chumsky::Parser;
use std::error::Error;
// use std::io;
// use testl::parser::expression::expression;
// use testl::parser::lexer;

fn main() -> Result<(), Box<dyn Error>> {
    // let src = io::read_to_string(io::stdin())?;

    // let tokens = match lexer::run(&src) {
    //     Ok(tokens) => tokens,
    //     Err(_) => unreachable!(),
    // };

    // let ast = expression()
    //     .parse(&tokens)
    //     .into_result()
    //     .unwrap_or_else(|e| panic!("parse error: {:#?}", e));

    // println!("[AST]\n{:#?}", ast);

    Ok(())
}
