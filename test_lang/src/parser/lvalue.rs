use chumsky::prelude::*;

use crate::{
    ast::LValue,
    parser::{ParserError, common::identifier_as_string, expression::expression, lexer::Token},
    span::{SourceIDSpan, SourceIDSpanned},
};

pub fn dot_access_lvalue<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<LValue>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    expression()
        .then_ignore(just(Token::Period))
        .then(identifier_as_string())
        .map(|(lhs, property_name)| LValue::DotAccess { lhs, property_name })
        .spanned()
}

pub fn lvalue<'src, I>() -> impl Parser<'src, I, SourceIDSpanned<LValue>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    choice((
        dot_access_lvalue(),
        identifier_as_string()
            .map(|identifier| LValue::Identifier(identifier.inner).with_span(identifier.span)),
    ))
}
