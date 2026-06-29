use chumsky::prelude::*;

use crate::{
    ast::{
        BinaryOperator, BinopExpression, Case, DEFAULT_CASE, Expression, LValue, Literal,
        Statement, UnaryOperator, UnopExpression,
    },
    in_curly_braces, parenthesized,
    parser::{
        ParserError, common::number_literal_as_string, expression::expression, lexer::Token,
        lvalue::lvalue,
    },
    span::{SourceIDSpan, SourceIDSpanned},
};

#[derive(Clone)]
enum PostfixAssignment {
    Increment,
    Decrement,
    Negate,
}

fn plus_plus<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<PostfixAssignment>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    just(Token::Plus)
        .then(just(Token::Plus))
        .to(PostfixAssignment::Increment)
        .spanned()
}

fn minus_minus<'src, I>()
-> impl Parser<'src, I, Spanned<PostfixAssignment, SourceIDSpan>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    just(Token::Minus)
        .then(just(Token::Minus))
        .to(PostfixAssignment::Decrement)
        .spanned()
}

fn bang_bang<'src, I>()
-> impl Parser<'src, I, Spanned<PostfixAssignment, SourceIDSpan>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    just(Token::Bang)
        .then(just(Token::Bang))
        .to(PostfixAssignment::Negate)
        .spanned()
}

pub fn shorthand_assignment_operator<'src, I>()
-> impl Parser<'src, I, Spanned<Token, SourceIDSpan>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    choice((
        just(Token::PlusEquals),
        just(Token::MinusEquals),
        just(Token::AsteriskEquals),
        just(Token::SlashEquals),
        just(Token::PercentEquals),
    ))
    .spanned()
}

fn postfix_assignment_operator<'src, I>()
-> impl Parser<'src, I, Spanned<PostfixAssignment, SourceIDSpan>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    choice((plus_plus(), minus_minus(), bang_bang()))
}

pub fn break_statement<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<Statement>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    just(Token::Break).to(Statement::Break).spanned()
}

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

pub fn assignment<'src, I>()
-> impl Parser<'src, I, (SourceIDSpanned<LValue>, SourceIDSpanned<Expression>), ParserError<'src>>
+ Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    lvalue()
        .then_ignore(just(Token::SingleEquals))
        .then(expression())
}

pub fn shorthand_assignment<'src, I>()
-> impl Parser<'src, I, (SourceIDSpanned<LValue>, SourceIDSpanned<Expression>), ParserError<'src>>
+ Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    lvalue()
        .then(shorthand_assignment_operator())
        .then(expression())
        .map(|((lvalue, operator), rhs)| {
            let rhs_span = rhs.span.clone();
            let op = match operator.inner {
                Token::PlusEquals => BinaryOperator::Add,
                Token::MinusEquals => BinaryOperator::Sub,
                Token::AsteriskEquals => BinaryOperator::Mul,
                Token::SlashEquals => BinaryOperator::Div,
                Token::PercentEquals => BinaryOperator::Mod,
                _ => unreachable!("Unhandled shorthand assignment operator"),
            };
            let rhs = Expression::Binop(BinopExpression {
                op,
                lhs: Box::new(lvalue.to_expression().with_span(lvalue.span)),
                rhs: Box::new(rhs),
            })
            .with_span(operator.span.union(rhs_span));

            (lvalue, rhs)
        })
}

pub fn postfix_assignment<'src, I>()
-> impl Parser<'src, I, (SourceIDSpanned<LValue>, SourceIDSpanned<Expression>), ParserError<'src>>
+ Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    lvalue()
        .then(postfix_assignment_operator())
        .map(|(lvalue, operator)| {
            (
                lvalue.clone(),
                match operator.inner {
                    PostfixAssignment::Increment => Expression::Binop(BinopExpression {
                        op: BinaryOperator::Add,
                        lhs: Box::new(lvalue.to_expression().with_span(lvalue.span)),
                        rhs: Box::new(
                            Expression::Literal(Literal::Number("1".into()))
                                .with_span(operator.span),
                        ),
                    }),
                    PostfixAssignment::Decrement => Expression::Binop(BinopExpression {
                        op: BinaryOperator::Sub,
                        lhs: Box::new(lvalue.to_expression().with_span(lvalue.span)),
                        rhs: Box::new(
                            Expression::Literal(Literal::Number("1".into()))
                                .with_span(operator.span),
                        ),
                    }),
                    PostfixAssignment::Negate => Expression::Unop(UnopExpression {
                        op: UnaryOperator::UnaryNot,
                        term: Box::new(lvalue.to_expression().with_span(lvalue.span)),
                    }),
                }
                .with_span(operator.span),
            )
        })
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

pub fn assignment_statement<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<Statement>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    choice((assignment(), shorthand_assignment(), postfix_assignment()))
        .map(|(name, value)| Statement::Assignment(name.inner, Box::new(value)))
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

pub fn continue_statement<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<Statement>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    just(Token::Continue).to(Statement::Continue).spanned()
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
