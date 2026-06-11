use crate::lexer::Token;
use crate::lexer::TokenData;
use chumsky::prelude::*;

#[derive(Debug, Clone)]
pub enum Expr {
    Program(Vec<Expr>),
    Function((Box<Expr>, Box<Expr>, Box<Expr>)),
    Block(Vec<Expr>),
    Statement(Vec<Expr>),
    Expression(Vec<Expr>),
    Identifier(String),
    Arguments,
}

pub fn parser<'src>() -> impl Parser<'src, &'src [Token], Expr> {
    // recursive(|p| {

    // })

    let identifier = select! {
        Token::Identifier(TokenData {value}) => Expr::Identifier(value)
    };

    let expression = identifier;

    let statement = expression.then_ignore(just(Token::Semicolon));

    let block = just(Token::LCurlyBrace)
        .ignore_then(statement.repeated().collect::<Vec<Expr>>())
        .then_ignore(just(Token::RCurlyBrace))
        .map(|e| Expr::Block(e));

    let args = just([Token::LParenthesis, Token::RParenthesis]).to(Expr::Arguments);

    let function = just(Token::Function)
        .ignore_then(identifier)
        .then(args)
        .then(block)
        .map(|((name, args), body)| {
            Expr::Function((Box::new(name), Box::new(args), Box::new(body)))
        });

    let program = function
        .repeated()
        .collect::<Vec<Expr>>()
        .map(|e| Expr::Program(e));

    program
}

// pub fn parser<'src>()
// -> impl Parser<'src, &'src [Token], Expr, chumsky::extra::Err<chumsky::error::Simple<'src, Token>>>
// {
//     let program = just(Token::Identifier).repeated();
//     // recursive(|p| {

//     // })
//     program
// }
