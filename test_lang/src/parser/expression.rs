use chumsky::prelude::*;

use crate::{
    ast::{BinaryOperator, BinopExpression, Call, Expression, UnaryOperator, UnopExpression},
    parenthesized,
    parser::{
        ParserError,
        common::{identifier, type_expression},
        lexer::Token,
        literal::literal,
    },
    span::{SourceIDSpan, SourceIDSpanned},
};

pub fn binary_operator_1<'src, I>()
-> impl Parser<'src, I, Spanned<Token, SourceIDSpan>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    choice((
        just(Token::Asterisk),
        just(Token::Slash),
        just(Token::Percent),
    ))
    .spanned()
}

pub fn binary_operator_2<'src, I>()
-> impl Parser<'src, I, Spanned<Token, SourceIDSpan>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    choice((
        just(Token::Plus),
        just(Token::Minus),
        just(Token::DoubleAmpersand),
        just(Token::DoublePipe),
    ))
    .spanned()
}

pub fn binary_operator_3<'src, I>()
-> impl Parser<'src, I, Spanned<Token, SourceIDSpan>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    choice((
        just(Token::DoubleEquals),
        just(Token::GreaterThan),
        just(Token::LessThan),
        just(Token::GreaterThanOrEquals),
        just(Token::LessThanOrEquals),
        just(Token::NotEquals),
    ))
    .spanned()
}

pub fn expression<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<Expression>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    let identifier = identifier();

    recursive(|expression| {
        let argument_list = parenthesized!(
            expression
                .clone()
                .separated_by(just(Token::Comma))
                .collect::<Vec<SourceIDSpanned<Expression>>>()
        );

        let callee_expression = parenthesized!(expression.clone());
        let callee = choice((callee_expression, identifier.clone()));

        let function_call = callee
            .then(argument_list)
            .map(|(callee, argument_list)| {
                Expression::Call(Call {
                    callee: Box::new(callee),
                    args: argument_list,
                })
            })
            .spanned();

        let typed_expression = type_expression()
            .clone()
            .then(expression.clone())
            .map(|(type_, expression)| Expression::TypedExpression(type_, Box::new(expression)))
            .spanned();

        let term = recursive(|term| {
            let unary_not_expression = just(Token::Minus)
                .ignore_then(term.clone())
                .map(|expression| {
                    Expression::Unop(UnopExpression {
                        op: UnaryOperator::UnaryMinus,
                        term: Box::new(expression),
                    })
                })
                .spanned();

            let unary_minus_expression = just(Token::Bang)
                .ignore_then(term.clone())
                .map(|expression| {
                    Expression::Unop(UnopExpression {
                        op: UnaryOperator::UnaryNot,
                        term: Box::new(expression),
                    })
                })
                .spanned();

            let unary_expression = choice((unary_minus_expression, unary_not_expression));

            choice((
                unary_expression.clone(),
                function_call,
                identifier.clone(),
                literal(),
                typed_expression,
                parenthesized!(expression.clone()),
            ))
        });

        let binary_expression_1 = term.clone().foldl(
            binary_operator_1().then(term.clone()).repeated(),
            |lhs, (op, rhs)| {
                let span = lhs.span.union(op.span).union(rhs.span);
                let op = match op.inner {
                    Token::Asterisk => BinaryOperator::Mul,
                    Token::Slash => BinaryOperator::Div,
                    Token::Percent => BinaryOperator::Mod,
                    _ => unreachable!(),
                };

                Expression::Binop(BinopExpression {
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                })
                .with_span(span)
            },
        );

        let binary_expression_2 = binary_expression_1.clone().foldl(
            binary_operator_2().then(binary_expression_1.clone()).repeated(),
            |lhs, (op, rhs)| {
                let span = lhs.span.union(op.span).union(rhs.span);
                let op = match op.inner {
                    Token::Plus => BinaryOperator::Add,
                    Token::Minus => BinaryOperator::Sub,
                    Token::DoubleAmpersand => BinaryOperator::And,
                    Token::DoublePipe => BinaryOperator::Or,
                    _ => unreachable!(),
                };

                Expression::Binop(BinopExpression {
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                })
                .with_span(span)
            },
        );

        let binary_expression_3 = binary_expression_2.clone().foldl(
            binary_operator_3().then(binary_expression_2.clone()).repeated(),
            |lhs, (op, rhs)| {
                let span = lhs.span.union(op.span).union(rhs.span);
                let op = match op.inner {
                    Token::DoubleEquals => BinaryOperator::Equals,
                    Token::GreaterThan => BinaryOperator::GreaterThan,
                    Token::LessThan => BinaryOperator::LessThan,
                    Token::GreaterThanOrEquals => BinaryOperator::GreaterThanOrEquals,
                    Token::LessThanOrEquals => BinaryOperator::LessThanOrEquals,
                    Token::NotEquals => BinaryOperator::NotEquals,
                    _ => unreachable!(),
                };

                Expression::Binop(BinopExpression {
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                })
                .with_span(span)
            },
        );

        choice((
            binary_expression_3,
            binary_expression_2,
            binary_expression_1,
            term,
            parenthesized!(expression.clone()),
        ))
    })
}
