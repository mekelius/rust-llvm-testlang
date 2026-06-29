use chumsky::prelude::*;

use crate::{
    ast::{
        BinaryOperator, BinopExpression, Expression, Literal, Statement, UnaryOperator,
        UnopExpression,
    },
    parser::{ParserError, expression::expression, lexer::Token},
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

pub fn assignment<'src, I>()
-> impl Parser<'src, I, (SourceIDSpanned<Expression>, SourceIDSpanned<Expression>), ParserError<'src>>
+ Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    expression()
        .then_ignore(just(Token::SingleEquals))
        .then(expression())
}

pub fn shorthand_assignment<'src, I>()
-> impl Parser<'src, I, (SourceIDSpanned<Expression>, SourceIDSpanned<Expression>), ParserError<'src>>
+ Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    expression()
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
                lhs: Box::new(lvalue.clone()),
                rhs: Box::new(rhs),
            })
            .with_span(operator.span.union(rhs_span));

            (lvalue, rhs)
        })
}

pub fn postfix_assignment<'src, I>()
-> impl Parser<'src, I, (SourceIDSpanned<Expression>, SourceIDSpanned<Expression>), ParserError<'src>>
+ Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    expression()
        .then(postfix_assignment_operator())
        .map(|(lvalue, operator)| {
            (
                lvalue.clone(),
                match operator.inner {
                    PostfixAssignment::Increment => Expression::Binop(BinopExpression {
                        op: BinaryOperator::Add,
                        lhs: Box::new(lvalue),
                        rhs: Box::new(
                            Expression::Literal(Literal::Number("1".into()))
                                .with_span(operator.span),
                        ),
                    }),
                    PostfixAssignment::Decrement => Expression::Binop(BinopExpression {
                        op: BinaryOperator::Sub,
                        lhs: Box::new(lvalue),
                        rhs: Box::new(
                            Expression::Literal(Literal::Number("1".into()))
                                .with_span(operator.span),
                        ),
                    }),
                    PostfixAssignment::Negate => Expression::Unop(UnopExpression {
                        op: UnaryOperator::UnaryNot,
                        term: Box::new(lvalue),
                    }),
                }
                .with_span(operator.span),
            )
        })
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
