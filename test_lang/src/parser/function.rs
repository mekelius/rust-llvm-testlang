use chumsky::prelude::*;

use crate::{
    ast::{Node, SourceIDSpan, SpannedString},
    in_curly_braces, parenthesized,
    parser::{
        ParserError, SpannedNode,
        common::{identifier_as_string, type_expression},
        lexer::Token,
        statement::statement,
    },
};

pub fn function<'src, I>() -> impl Parser<'src, I, SpannedNode, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    let statement = statement();

    let untyped_formal = select! {
        Token::Identifier(value) => Node::UntypedFormal(value)
    }
    .spanned();

    let typed_formal = select! {Token::TypeIdentifier(value) => value}
        .clone()
        .then(select! {
            Token::Identifier(value) => value
        })
        .map(|(type_, name)| Node::TypedFormal(type_, name))
        .spanned();

    let formal = choice((typed_formal, untyped_formal));

    let function_body = in_curly_braces!(
        statement
            .repeated()
            .collect::<Vec<SpannedNode>>()
            .map(|e| Node::FunctionBody(e))
    )
    .spanned();

    let formals = parenthesized!(
        formal
            .separated_by(just(Token::Comma))
            .collect::<Vec<SpannedNode>>()
            .map(|e| Node::Formals(e))
    )
    .spanned();

    let maybe_return_type = (just(Token::ArrowSingle)
        .ignore_then(type_expression())
        .map(|type_| Some(type_)))
    .or(empty().to(None));

    let function = identifier_as_string()
        .then(formals)
        .then(maybe_return_type)
        .then(function_body)
        .map(
            |(((name, formals), return_type_string), function_body): (
                ((SpannedString, SpannedNode), Option<SpannedString>),
                SpannedNode,
            )| {
                let formals = match formals.inner {
                    Node::Formals(formals) => formals,
                    _ => unreachable!(),
                };

                Node::Function {
                    name,
                    return_type_string,
                    formals,
                    body: Box::new(function_body),
                }
            },
        )
        .spanned();

    function
}
