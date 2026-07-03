use chumsky::prelude::*;

use crate::{
    ast::{Case, DEFAULT_CASE, Statement},
    ast_store::{StatementID, UNIT_LITERAL},
    in_curly_braces, parenthesized,
    parser::{
        Extras,
        assignment::{assignment, assignment_statement},
        common::number_literal_as_string,
        expression::expression,
        lexer::Token,
        store_node::StoreStatement,
    },
    span::{SourceIDSpan, SourceIDSpanned},
};

pub fn empty_statement<'tokens, I>()
-> impl Parser<'tokens, I, SourceIDSpanned<Statement>, Extras<'tokens>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
{
    empty().to(Statement::Empty).spanned()
}

pub fn expression_statement<'tokens, I>()
-> impl Parser<'tokens, I, SourceIDSpanned<Statement>, Extras<'tokens>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
{
    expression()
        .map(|expression| Statement::Expression(expression))
        .spanned()
}

pub fn const_statement<'tokens, I>()
-> impl Parser<'tokens, I, SourceIDSpanned<Statement>, Extras<'tokens>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
{
    just(Token::Const)
        .ignore_then(assignment())
        .map(|(lvalue, value)| Statement::Const(lvalue, value))
        .spanned()
}

pub fn let_statement<'tokens, I>() -> impl Parser<'tokens, I, StatementID, Extras<'tokens>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
{
    just(Token::Let)
        .ignore_then(assignment())
        .map(|(lvalue, value)| Statement::Let(lvalue, value))
        .spanned()
        .store_statement()
}

pub fn return_statement<'tokens, I>()
-> impl Parser<'tokens, I, SourceIDSpanned<Statement>, Extras<'tokens>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
{
    just(Token::Return)
        .ignore_then(expression())
        .map(|expression| Statement::Return(expression))
        .spanned()
}

/**
 * Valueless "return" is desugared to "return ()"
 */
pub fn valueless_return_statement<'tokens, I>()
-> impl Parser<'tokens, I, SourceIDSpanned<Statement>, Extras<'tokens>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
{
    just(Token::Return)
        .to(UNIT_LITERAL)
        .map(|unit| Statement::Return(unit))
        .spanned()
}

pub fn continue_statement<'tokens, I>()
-> impl Parser<'tokens, I, SourceIDSpanned<Statement>, Extras<'tokens>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
{
    just(Token::Continue).to(Statement::Continue).spanned()
}

pub fn break_statement<'tokens, I>()
-> impl Parser<'tokens, I, SourceIDSpanned<Statement>, Extras<'tokens>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
{
    just(Token::Break).to(Statement::Break).spanned()
}

pub fn statement<'tokens, I>() -> impl Parser<'tokens, I, StatementID, Extras<'tokens>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
{
    let expression = expression();
    let let_statement = let_statement();
    let assignment_statement = assignment_statement();

    recursive(|p| {
        let if_statement = just(Token::If)
            .ignore_then(parenthesized!(expression.clone()))
            .then(p.clone())
            .map(|(condition, body)| Statement::If { condition, body })
            .spanned()
            .store_statement();

        let if_else_statement = if_statement
            .clone()
            .then_ignore(just(Token::Else))
            .then(p.clone())
            .map(|(if_branch, else_branch)| Statement::IfElse(if_branch, else_branch))
            .spanned()
            .store_statement();

        let statement_list = p.clone().repeated().collect::<Vec<StatementID>>();

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
                matched_value_expression: expression,
                cases,
            })
            .spanned()
            .store_statement();

        let block = in_curly_braces!(statement_list.clone().map(|e| Statement::Block(e)))
            .spanned()
            .store_statement();

        let simple_statement = choice((
            let_statement.clone(),
            const_statement().store_statement(),
            assignment_statement.clone(),
            return_statement().store_statement(),
            continue_statement().store_statement(),
            break_statement().store_statement(),
            valueless_return_statement().store_statement(),
            expression_statement().store_statement(),
            empty_statement().store_statement(),
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
                init,
                condition,
                step,
                body,
            })
            .spanned()
            .store_statement();

        let while_statement = just(Token::While)
            .ignore_then(parenthesized!(expression.clone()))
            .then(p.clone())
            .map(|(condition, body)| Statement::While { condition, body })
            .spanned()
            .store_statement();

        let complex_statement = choice((
            if_else_statement,
            if_statement,
            switch_statement,
            while_statement,
            for_statement,
        ));

        choice((single_statement, block, complex_statement))
        // .map_with(|statement, e| e.state().statements.add(statement))
    })
}
