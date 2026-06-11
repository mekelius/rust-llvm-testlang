use crate::lexer::Token;
use crate::lexer::TokenData;
use chumsky::prelude::*;

#[derive(Debug, Clone)]
pub enum Expr {
    Program(Vec<Expr>),
    Function((Box<Expr>, Box<Expr>, Box<Expr>)),
    Block(Vec<Expr>),
    Expression(Vec<Expr>),
    Identifier(String),
    Formals(Vec<Expr>),
    Formal(String),
    FunctionBody(Vec<Expr>),
    While {
        condition: Box<Expr>,
        body: Box<Expr>,
    },
    EmptyStatement,
}

fn parenthesized<'src>(
    a: impl Parser<'src, &'src [Token], Expr, chumsky::extra::Err<Rich<'src, Token>>> + Clone,
) -> impl Parser<'src, &'src [Token], Expr, chumsky::extra::Err<Rich<'src, Token>>> + Clone {
    a.delimited_by(just(Token::LParenthesis), just(Token::RParenthesis))
}

fn in_curly_braces<'src>(
    a: impl Parser<'src, &'src [Token], Expr, chumsky::extra::Err<Rich<'src, Token>>> + Clone,
) -> impl Parser<'src, &'src [Token], Expr, chumsky::extra::Err<Rich<'src, Token>>> + Clone {
    a.delimited_by(just(Token::LCurlyBrace), just(Token::RCurlyBrace))
}

pub fn parser<'src>()
-> impl Parser<'src, &'src [Token], Expr, chumsky::extra::Err<Rich<'src, Token>>> + Clone {
    let identifier = select! {
        Token::Identifier(TokenData {value}) => Expr::Identifier(value),
    };

    let expression = identifier;

    let statement = recursive(|p| {
        let while_ = just(Token::While)
            .ignore_then(parenthesized(expression))
            .then(p.clone())
            .map(|(condition, body)| Expr::While {
                condition: Box::new(condition),
                body: Box::new(body),
            });

        let empty_statement = empty().to(Expr::EmptyStatement);

        let block = in_curly_braces(
            p.clone()
                .repeated()
                .collect::<Vec<Expr>>()
                .map(|e| Expr::Block(e)),
        );

        let single_statement =
            ((expression).or(empty_statement)).then_ignore(just(Token::Semicolon));

        let complex_statement = while_;

        single_statement.or(block).or(complex_statement)
    });

    let formal = select! {
        Token::Identifier(TokenData {value}) => Expr::Formal(value)
    };

    let function_body = in_curly_braces(
        statement
            .repeated()
            .collect::<Vec<Expr>>()
            .map(|e| Expr::FunctionBody(e)),
    );

    let formals = parenthesized(
        formal
            .separated_by(just(Token::Comma))
            .collect::<Vec<Expr>>()
            .map(|e| Expr::Formals(e)),
    );

    let function = just(Token::Function)
        .ignore_then(identifier)
        .then(formals)
        .then(function_body)
        .map(|((name, formals), function_body)| {
            Expr::Function((Box::new(name), Box::new(formals), Box::new(function_body)))
        });

    let program = function
        .repeated()
        .collect::<Vec<Expr>>()
        .map(|e| Expr::Program(e));

    program
}
