use chumsky::prelude::*;

use crate::{
    ast::{Parameter, Function, Statement},
    in_curly_braces, parenthesized,
    parser::{
        ParserError,
        common::{identifier_as_string, type_expression},
        lexer::Token,
        statement::statement,
    },
    span::{SourceIDSpan, SourceIDSpanned},
};

fn typed_formal<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<Parameter>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    select! {Token::TypeIdentifier(value) => value}
        .spanned()
        .then(
            select! {
                Token::Identifier(value) => value
            }
            .spanned(),
        )
        .map(|(type_, name)| Parameter::Typed(type_, name))
        .spanned()
}

fn untyped_formal<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<Parameter>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    select! {
        Token::Identifier(value) => Parameter::Untyped(value)
    }
    .spanned()
}

fn formal<'src, I>() -> impl Parser<'src, I, SourceIDSpanned<Parameter>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    choice((typed_formal(), untyped_formal()))
}

fn formals<'src, I>()
-> impl Parser<'src, I, Vec<SourceIDSpanned<Parameter>>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    parenthesized!(
        formal()
            .separated_by(just(Token::Comma))
            .collect::<Vec<SourceIDSpanned<Parameter>>>()
    )
}

fn maybe_return_type<'src, I>()
-> impl Parser<'src, I, Option<SourceIDSpanned<String>>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    (just(Token::ArrowSingle)
        .ignore_then(type_expression())
        .map(|type_| Some(type_)))
    .or(empty().to(None))
}

fn function_body<'src, I>()
-> impl Parser<'src, I, Vec<SourceIDSpanned<Statement>>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    in_curly_braces!(
        statement()
            .repeated()
            .collect::<Vec<SourceIDSpanned<Statement>>>()
            .map(|body| body)
    )
}

pub fn function<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<Function>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    identifier_as_string()
        .then(formals())
        .then(maybe_return_type())
        .then(function_body())
        .map(
            |(((name, formals), return_type_string), body)| Function {
                name,
                return_type_string,
                formals,
                body,
            },
        )
        .spanned()
}
