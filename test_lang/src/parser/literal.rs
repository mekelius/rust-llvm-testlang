use chumsky::prelude::*;

use crate::{
    ast::{Expression, Literal}, ast_store::{ExpressionID, FALSE_LITERAL, TRUE_LITERAL, UNIT_LITERAL}, parser::{Extras, lexer::Token, store_node::StoreExpression}, span::SourceIDSpan,
};

pub fn number_literal<'tokens, I>() -> impl Parser<'tokens, I, ExpressionID, Extras<'tokens>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
{
    select! {
        Token::NumberLiteral(value) => Expression::Literal(Literal::Number(value)),
    }
    .spanned()
    .store_expression()
}

pub fn string_literal<'tokens, I>() -> impl Parser<'tokens, I, ExpressionID, Extras<'tokens>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
{
    select! {
        Token::StringLiteral(value) => Expression::Literal(Literal::String(value)),
    }
    .spanned()
    .store_expression()
}

pub fn boolean_literal<'tokens, I>()
-> impl Parser<'tokens, I, ExpressionID, Extras<'tokens>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
{
    select! {
        Token::True => TRUE_LITERAL,
        Token::False => FALSE_LITERAL,
    }
}

pub fn unit_literal<'tokens, I>() -> impl Parser<'tokens, I, ExpressionID, Extras<'tokens>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
{
    just([Token::LParenthesis, Token::RParenthesis]).to(UNIT_LITERAL)
}

pub fn literal<'tokens, I>() -> impl Parser<'tokens, I, ExpressionID, Extras<'tokens>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
{
    choice((
        string_literal(),
        number_literal(),
        boolean_literal(),
        unit_literal(),
    ))
}
