use chumsky::prelude::*;

use crate::{
    ast::{Case, DEFAULT_CASE, Expression, Literal, Statement},
    in_curly_braces, parenthesized,
    parser::{
        ParserError,
        assignment::{assignment, assignment_statement},
        common::number_literal_as_string,
        expression::expression,
        lexer::Token,
    },
    span::{SourceIDSpan, SourceIDSpanned},
};

pub fn empty_statement<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<Statement>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    empty().to(Statement::Empty).spanned()
}

pub fn expression_statement<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<Statement>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    expression()
        .map(|e| Statement::Expression(Box::new(e)))
        .spanned()
}

pub fn const_statement<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<Statement>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    just(Token::Const)
        .ignore_then(assignment())
        .map(|(name, value)| Statement::Const(name.inner, Box::new(value)))
        .spanned()
}

pub fn let_statement<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<Statement>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    just(Token::Let)
        .ignore_then(assignment())
        .map(|(name, value)| Statement::Let(name.inner, Box::new(value)))
        .spanned()
}

pub fn return_statement<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<Statement>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    just(Token::Return)
        .ignore_then(expression())
        .map(|expr| Statement::Return(Box::new(expr)))
        .spanned()
}

/**
 * Valueless "return" is desugared to "return ()"
 */
pub fn valueless_return_statement<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<Statement>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    just(Token::Return)
        .to(Expression::Literal(Literal::Unit))
        .spanned()
        .map(|unit| Statement::Return(Box::new(unit)))
        .spanned()
}

pub fn continue_statement<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<Statement>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    just(Token::Continue).to(Statement::Continue).spanned()
}

pub fn break_statement<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<Statement>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    just(Token::Break).to(Statement::Break).spanned()
}

pub fn statement<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<Statement>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    let expression = expression();
    let let_statement = let_statement();
    let assignment_statement = assignment_statement();

    recursive(|p| {
        let if_statement = just(Token::If)
            .ignore_then(parenthesized!(expression.clone()))
            .then(p.clone())
            .map(|(condition, body)| Statement::If {
                condition: Box::new(condition),
                body: Box::new(body),
            })
            .spanned();

        let if_else_statement = if_statement
            .clone()
            .then_ignore(just(Token::Else))
            .then(p.clone())
            .map(|(if_branch, else_branch)| {
                Statement::IfElse(Box::new(if_branch), Box::new(else_branch))
            })
            .spanned();

        let statement_list = p
            .clone()
            .repeated()
            .collect::<Vec<SourceIDSpanned<Statement>>>();

        let case = just(Token::Case)
            .ignore_then(number_literal_as_string())
            .then_ignore(just(Token::Colon))
            .then(statement_list.clone())
            .map(|(matched_value, body)| Case {
                matched_value: Some(matched_value),
                body,
            })
            .spanned();

        let default_case = just(Token::Default)
            .ignore_then(just(Token::Colon))
            .ignore_then(statement_list.clone())
            .map(|body| Case {
                matched_value: DEFAULT_CASE,
                body,
            })
            .spanned();

        let switch_statement = just(Token::Switch)
            .ignore_then(expression.clone())
            .then(in_curly_braces!(
                (case.or(default_case)).repeated().collect()
            ))
            .map(|(expression, cases)| Statement::Switch {
                matched_value_expression: Box::new(expression),
                cases,
            })
            .spanned();

        let block = in_curly_braces!(statement_list.clone().map(|e| Statement::Block(e))).spanned();

        let simple_statement = choice((
            let_statement.clone(),
            const_statement(),
            assignment_statement.clone(),
            return_statement(),
            continue_statement(),
            break_statement(),
            valueless_return_statement(),
            expression_statement(),
            empty_statement(),
        ));

        let single_statement = simple_statement.clone().then_ignore(just(Token::Semicolon));

        // For loop
        let for_init = choice((let_statement.clone(), assignment_statement.clone()));
        let for_condition = expression.clone();
        let for_step = simple_statement.clone();

        let for_statement = just(Token::For)
            .ignore_then(parenthesized!(
                for_init
                    .then_ignore(just(Token::Semicolon))
                    .then(for_condition)
                    .then_ignore(just(Token::Semicolon))
                    .then(for_step)
            ))
            .then(p.clone())
            .map(|(((init, condition), step), body)| Statement::For {
                init: Box::new(init),
                condition: Box::new(condition),
                step: Box::new(step),
                body: Box::new(body),
            })
            .spanned();

        let while_statement = just(Token::While)
            .ignore_then(parenthesized!(expression.clone()))
            .then(p.clone())
            .map(|(condition, body)| Statement::While {
                condition: Box::new(condition),
                body: Box::new(body),
            })
            .spanned();

        let complex_statement = choice((
            if_else_statement,
            if_statement,
            switch_statement,
            while_statement,
            for_statement,
        ));

        choice((single_statement, block, complex_statement))
    })
}
