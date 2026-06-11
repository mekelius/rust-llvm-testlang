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
    Formals(Vec<Expr>),
    Formal(String),
    While {
        condition: Box<Expr>,
        body: Box<Expr>,
    },
}

fn parenthesized<'src>(
    a: impl Parser<'src, &'src [Token], Expr> + Clone,
) -> impl Parser<'src, &'src [Token], Expr> + Clone {
    a.delimited_by(just(Token::LParenthesis), just(Token::RParenthesis))
}

pub fn parser<'src>() -> impl Parser<'src, &'src [Token], Expr> + Clone {
    let identifier = select! {
    Token::Identifier(TokenData {value}) => Expr::Identifier(value)
    };

    let expression = identifier;

    let block = recursive(|p| {
        let while_ = just(Token::While)
            .ignore_then(parenthesized(expression))
            .then(p.clone())
            .map(|(condition, body)| Expr::While {
                condition: Box::new(condition),
                body: Box::new(body),
            });

        let statement = (expression.or(while_)).then_ignore(just(Token::Semicolon));

        let block = (statement
            .repeated()
            .collect::<Vec<Expr>>()
            .map(|e| Expr::Block(e)))
        .delimited_by(just(Token::LCurlyBrace), just(Token::RCurlyBrace));

        block
    });

    let formal = select! {
        Token::Identifier(TokenData {value}) => Expr::Formal(value)
    };

    let formals = (formal
        .separated_by(just(Token::Comma))
        .collect::<Vec<Expr>>()
        .map(|e| Expr::Formals(e)))
    .delimited_by(just(Token::LParenthesis), just(Token::RParenthesis));

    let function = just(Token::Function)
        .ignore_then(identifier)
        .then(formals)
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
