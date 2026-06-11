use crate::lexer::Token;
use crate::lexer::TokenData;
use chumsky::prelude::*;

#[derive(Debug, Clone)]
pub enum Node {
    Program(Vec<Node>),
    Function((Box<Node>, Box<Node>, Box<Node>)),
    Block(Vec<Node>),
    Expression(Vec<Node>),
    Identifier(String),
    Formals(Vec<Node>),
    Formal(String),
    FunctionBody(Vec<Node>),
    While {
        condition: Box<Node>,
        body: Box<Node>,
    },
    EmptyStatement,
    ArgumentList(Vec<Node>),
    FunctionCall {
        callee: String,
        argument_list: Vec<Node>,
    },
    UnaryOperator {
        operator: Box<Node>,
        rhs: Box<Node>,
    },
    UnaryMinus,
}

fn parenthesized<'src>(
    a: impl Parser<'src, &'src [Token], Node, chumsky::extra::Err<Rich<'src, Token>>> + Clone,
) -> impl Parser<'src, &'src [Token], Node, chumsky::extra::Err<Rich<'src, Token>>> + Clone {
    a.delimited_by(just(Token::LParenthesis), just(Token::RParenthesis))
}

fn in_curly_braces<'src>(
    a: impl Parser<'src, &'src [Token], Node, chumsky::extra::Err<Rich<'src, Token>>> + Clone,
) -> impl Parser<'src, &'src [Token], Node, chumsky::extra::Err<Rich<'src, Token>>> + Clone {
    a.delimited_by(just(Token::LCurlyBrace), just(Token::RCurlyBrace))
}

pub fn parser<'src>()
-> impl Parser<'src, &'src [Token], Node, chumsky::extra::Err<Rich<'src, Token>>> + Clone {
    let identifier = select! {
        Token::Identifier(TokenData {value}) => Node::Identifier(value),
    };

    let unary_operator = just(Token::Minus).to(Node::UnaryMinus);

    // let binary_operator = just(Token::Minus).to(Node::UnaryMinus);

    let expression = recursive(|expr| {
        let argument_list = parenthesized(
            expr.clone()
                .separated_by(just(Token::Comma))
                .collect::<Vec<Node>>()
                .map(|e| Node::ArgumentList(e)),
        );

        let unary_expression =
            unary_operator
                .then(expr.clone())
                .map(|(operator, rhs)| Node::UnaryOperator {
                    operator: Box::new(operator),
                    rhs: Box::new(rhs),
                });

        // let binary_expression = todo();

        // let function_call = identifier.then(argument_list).map(
        //     |(Node::Identifier(value), Node::ArgumentList(argument_list))| Node::FunctionCall {
        //         callee: value,
        //         argument_list,
        //     },
        // );

        let function_call = identifier
            .then(argument_list)
            .map(|(identifier, arguments)| {
                let callee = match identifier {
                    Node::Identifier(value) => value,
                    _ => todo!(),
                };

                let argument_list = match arguments {
                    Node::ArgumentList(argument_list) => argument_list,
                    _ => todo!(),
                };

                Node::FunctionCall {
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
            .map(|(condition, body)| Node::While {
                condition: Box::new(condition),
                body: Box::new(body),
            });

        let empty_statement = empty().to(Node::EmptyStatement);

        let block = in_curly_braces(
            p.clone()
                .repeated()
                .collect::<Vec<Node>>()
                .map(|e| Node::Block(e)),
        );

        let single_statement =
            ((expression.clone()).or(empty_statement)).then_ignore(just(Token::Semicolon));

        let complex_statement = while_;

        single_statement.or(block).or(complex_statement)
    });

    let formal = select! {
        Token::Identifier(TokenData {value}) => Node::Formal(value)
    };

    let function_body = in_curly_braces(
        statement
            .repeated()
            .collect::<Vec<Node>>()
            .map(|e| Node::FunctionBody(e)),
    );

    let formals = parenthesized(
        formal
            .separated_by(just(Token::Comma))
            .collect::<Vec<Node>>()
            .map(|e| Node::Formals(e)),
    );

    let function = just(Token::Function)
        .ignore_then(identifier)
        .then(formals)
        .then(function_body)
        .map(|((name, formals), function_body)| {
            Node::Function((Box::new(name), Box::new(formals), Box::new(function_body)))
        });

    let program = function
        .repeated()
        .collect::<Vec<Node>>()
        .map(|e| Node::Program(e));

    program
}
