use chumsky::prelude::*;

use crate::{
    ast::{
        BinaryOperator, BinopExpression, Case, DEFAULT_CASE, Expression, Literal, Statement, UnaryOperator, UnopExpression,
    }, in_curly_braces, parenthesized, parser::{
        ParserError,
        common::{identifier_as_string, number_literal_as_string},
        expression::expression,
        lexer::Token,
    }, span::{SourceIDSpan, SourceIDSpanned},
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
    just(Token::Break).to(Statement::BreakStatement).spanned()
}

pub fn empty_statement<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<Statement>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    empty().to(Statement::EmptyStatement).spanned()
}

pub fn expression_statement<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<Statement>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    expression()
        .map(|e| Statement::ExpressionStatement(Box::new(e)))
        .spanned()
}

pub fn assignment<'src, I>()
-> impl Parser<'src, I, (SourceIDSpanned<String>, SourceIDSpanned<Expression>), ParserError<'src>>
+ Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    identifier_as_string()
        .then_ignore(just(Token::SingleEquals))
        .then(expression())
}

pub fn shorthand_assignment<'src, I>()
-> impl Parser<'src, I, (SourceIDSpanned<String>, SourceIDSpanned<Expression>), ParserError<'src>>
+ Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    identifier_as_string()
        .then(shorthand_assignment_operator())
        .then(expression())
        .map(|((name, operator), rhs)| {
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
                lhs: Box::new(Expression::Identifier(name.inner.clone()).with_span(name.span)),
                rhs: Box::new(rhs),
            })
            .with_span(operator.span.union(rhs_span));

            (name, rhs)
        })
}

pub fn postfix_assignment<'src, I>()
-> impl Parser<'src, I, (SourceIDSpanned<String>, SourceIDSpanned<Expression>), ParserError<'src>>
+ Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    identifier_as_string()
        .then(postfix_assignment_operator())
        .map(|(name, operator)| {
            (
                name.clone(),
                match operator.inner {
                    PostfixAssignment::Increment => Expression::Binop(BinopExpression {
                        op: BinaryOperator::Add,
                        lhs: Box::new(Expression::Identifier(name.inner).with_span(name.span)),
                        rhs: Box::new(
                            Expression::Literal(Literal::NumberLiteral("1".into()))
                                .with_span(operator.span),
                        ),
                    }),
                    PostfixAssignment::Decrement => Expression::Binop(BinopExpression {
                        op: BinaryOperator::Sub,
                        lhs: Box::new(Expression::Identifier(name.inner).with_span(name.span)),
                        rhs: Box::new(
                            Expression::Literal(Literal::NumberLiteral("1".into()))
                                .with_span(operator.span),
                        ),
                    }),
                    PostfixAssignment::Negate => Expression::Unop(UnopExpression {
                        op: UnaryOperator::UnaryNot,
                        term: Box::new(Expression::Identifier(name.inner).with_span(name.span)),
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
        .map(|(name, value)| Statement::ConstStatement(name.inner, Box::new(value)))
        .spanned()
}

pub fn let_statement<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<Statement>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    just(Token::Let)
        .ignore_then(assignment())
        .map(|(name, value)| Statement::LetStatement(name.inner, Box::new(value)))
        .spanned()
}

pub fn assignment_statement<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<Statement>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    choice((assignment(), shorthand_assignment(), postfix_assignment()))
        .map(|(name, value)| Statement::AssignmentStatement(name.inner, Box::new(value)))
        .spanned()
}

pub fn valueless_return_statement<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<Statement>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    just(Token::Return)
        .to(Statement::ValuelessReturnStatement)
        .spanned()
}

pub fn return_statement<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<Statement>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    just(Token::Return)
        .ignore_then(expression())
        .map(|expr| Statement::ReturnStatement(Box::new(expr)))
        .spanned()
}

pub fn continue_statement<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<Statement>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    just(Token::Continue)
        .to(Statement::ContinueStatement)
        .spanned()
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
            .map(|(condition, body)| Statement::IfStatement {
                condition: Box::new(condition),
                body: Box::new(body),
            })
            .spanned();

        let if_else_statement = if_statement
            .clone()
            .then_ignore(just(Token::Else))
            .then(p.clone())
            .map(|(if_branch, else_branch)| {
                Statement::IfElseStatement(Box::new(if_branch), Box::new(else_branch))
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
            .map(|(expression, cases)| Statement::SwitchStatement {
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
            .map(
                |(((init, condition), step), body)| Statement::ForStatement {
                    init: Box::new(init),
                    condition: Box::new(condition),
                    step: Box::new(step),
                    body: Box::new(body),
                },
            )
            .spanned();

        let while_statement = just(Token::While)
            .ignore_then(parenthesized!(expression.clone()))
            .then(p.clone())
            .map(|(condition, body)| Statement::WhileStatement {
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
