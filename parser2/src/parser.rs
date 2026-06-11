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
    ArgumentList(Vec<Expr>),
    FunctionCall {
        callee: String,
        argument_list: Vec<Expr>,
    },
    UnaryOperator {
        operator: Box<Expr>,
        rhs: Box<Expr>,
    },
    UnaryMinus,
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

    let unary_operator = just(Token::Minus).to(Expr::UnaryMinus);

    let expression = recursive(|expr| {
        let argument_list = parenthesized(
            expr.clone()
                .separated_by(just(Token::Comma))
                .collect::<Vec<Expr>>()
                .map(|e| Expr::ArgumentList(e)),
        );

        let unary_expression =
            unary_operator
                .then(expr.clone())
                .map(|(operator, rhs)| Expr::UnaryOperator {
                    operator: Box::new(operator),
                    rhs: Box::new(rhs),
                });

        // let binary_expression = todo();

        // let function_call = identifier.then(argument_list).map(
        //     |(Expr::Identifier(value), Expr::ArgumentList(argument_list))| Expr::FunctionCall {
        //         callee: value,
        //         argument_list,
        //     },
        // );

        let function_call = identifier
            .then(argument_list)
            .map(|(identifier, arguments)| {
                let callee = match identifier {
                    Expr::Identifier(value) => value,
                    _ => todo!(),
                };

                let argument_list = match arguments {
                    Expr::ArgumentList(argument_list) => argument_list,
                    _ => todo!(),
                };

                Expr::FunctionCall {
                    callee,
                    argument_list,
                }
            });

        let expression = choice((
            function_call,
            identifier,
            unary_expression,
            // binary_expression,
            parenthesized(expr.clone()),
        ));

        expression
    });

    let statement = recursive(|p| {
        let while_ = just(Token::While)
            .ignore_then(parenthesized(expression.clone()))
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
            ((expression.clone()).or(empty_statement)).then_ignore(just(Token::Semicolon));

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
