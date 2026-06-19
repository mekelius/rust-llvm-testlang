use chumsky::prelude::*;

use crate::{ast::Node, in_curly_braces, parenthesized, parser::{ParserError, common::{identifier, type_expression}, lexer::{Token, TokenData}, statement::statement}};

pub fn function<'src>() -> impl Parser<'src, &'src [Token], Node, ParserError<'src>> + Clone {
    let identifier = identifier();
    let statement = statement();

    let untyped_formal = select! {
        Token::Identifier(TokenData {value}) => Node::UntypedFormal(value)
    };

    let typed_formal = select! {Token::TypeIdentifier(TokenData {value}) => value}
        .clone()
        .then(select! {
            Token::Identifier(TokenData {value}) => value
        })
        .map(|(type_, name)| Node::TypedFormal(type_, name));

    let formal = typed_formal.clone().or(untyped_formal.clone());

    let function_body = in_curly_braces!(
        statement
            .repeated()
            .collect::<Vec<Node>>()
            .map(|e| Node::FunctionBody(e))
    );

    let formals = parenthesized!(
        formal
            .separated_by(just(Token::Comma))
            .collect::<Vec<Node>>()
            .map(|e| Node::Formals(e))
    );

    let maybe_return_type = (just(Token::ArrowSingle)
        .ignore_then(type_expression())
        .map(|type_| Some(type_)))
    .or(empty().to(None));

    let function = identifier
        .clone()
        .then(formals)
        .then(maybe_return_type)
        .then(function_body)
        .map(|(((name, formals), return_type_string), function_body)| {
            let name = match name {
                Node::Identifier(value) => value,
                _ => unreachable!(),
            };

            let formals = match formals {
                Node::Formals(formals) => formals,
                _ => unreachable!(),
            };

            Node::Function {
                name,
                return_type_string,
                formals,
                body: Box::new(function_body),
            }
        })
        .boxed();

    function
}
