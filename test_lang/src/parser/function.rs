use chumsky::prelude::*;

use crate::{
    ast::{Function, Parameter},
    ast_store::{FunctionID, StatementID, Store},
    in_curly_braces, parenthesized,
    parser::{
        Extras,
        common::{identifier_as_string, type_expression},
        lexer::Token,
        statement::statement,
    },
    span::{SourceIDSpan, SourceIDSpanned},
};

fn typed_formal<'tokens, I>()
-> impl Parser<'tokens, I, SourceIDSpanned<Parameter>, Extras<'tokens>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
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

fn untyped_formal<'tokens, I>()
-> impl Parser<'tokens, I, SourceIDSpanned<Parameter>, Extras<'tokens>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
{
    select! {
        Token::Identifier(value) => Parameter::Untyped(value)
    }
    .spanned()
}

fn formal<'tokens, I>()
-> impl Parser<'tokens, I, SourceIDSpanned<Parameter>, Extras<'tokens>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
{
    choice((typed_formal(), untyped_formal()))
}

fn parameter_list<'tokens, I>()
-> impl Parser<'tokens, I, Vec<SourceIDSpanned<Parameter>>, Extras<'tokens>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
{
    parenthesized!(
        formal()
            .separated_by(just(Token::Comma))
            .collect::<Vec<SourceIDSpanned<Parameter>>>()
    )
}

fn maybe_return_type<'tokens, I>()
-> impl Parser<'tokens, I, Option<SourceIDSpanned<String>>, Extras<'tokens>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
{
    (just(Token::ArrowSingle)
        .ignore_then(type_expression())
        .map(|type_| Some(type_)))
    .or(empty().to(None))
}

fn function_body<'tokens, I>() -> impl Parser<'tokens, I, Vec<StatementID>, Extras<'tokens>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
{
    in_curly_braces!(
        statement()
            .repeated()
            .collect::<Vec<StatementID>>()
            .map(|body| body)
    )
}

pub fn function<'tokens, I>() -> impl Parser<'tokens, I, FunctionID, Extras<'tokens>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
{
    identifier_as_string()
        .then(parameter_list())
        .then(maybe_return_type())
        .then(function_body())
        .map(|(((name, params), return_type_string), body)| Function {
            name,
            return_type_string,
            params,
            body,
        })
        .spanned()
        .map_with(|function, e| e.state().functions.add(function))
}
