use chumsky::prelude::*;

use crate::{
    ast::{
        BinaryOperator, BinopExpression, Call, DotAccessExpression, Expression, UnaryOperator,
        UnopExpression,
    },
    parenthesized,
    parser::{
        ParserError,
        common::{identifier, identifier_as_string, type_expression},
        lexer::Token,
        literal::literal,
    },
    span::{SourceIDSpan, SourceIDSpanned},
};

pub fn binary_operator_1<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<BinaryOperator>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    choice((
        just(Token::Asterisk).to(BinaryOperator::Mul),
        just(Token::Slash).to(BinaryOperator::Div),
        just(Token::Percent).to(BinaryOperator::Mod),
    ))
    .spanned()
}

pub fn binary_operator_2<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<BinaryOperator>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    choice((
        just(Token::Plus).to(BinaryOperator::Add),
        just(Token::Minus).to(BinaryOperator::Sub),
        just(Token::DoubleAmpersand).to(BinaryOperator::And),
        just(Token::DoublePipe).to(BinaryOperator::Or),
    ))
    .spanned()
}

pub fn binary_operator_3<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<BinaryOperator>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    choice((
        just(Token::DoubleEquals).to(BinaryOperator::Equals),
        just(Token::GreaterThan).to(BinaryOperator::GreaterThan),
        just(Token::LessThan).to(BinaryOperator::LessThan),
        just(Token::GreaterThanOrEquals).to(BinaryOperator::GreaterThanOrEquals),
        just(Token::LessThanOrEquals).to(BinaryOperator::LessThanOrEquals),
        just(Token::NotEquals).to(BinaryOperator::NotEquals),
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

            // let dot_access_chain = choice((identifier_as_string(), function_call.clone()));

            let dot_access_expression = choice((identifier.clone(), function_call.clone()))
                .then_ignore(just(Token::Period))
                .then(identifier_as_string())
                .map(|(lhs, property_name)| {
                    Expression::DotAccess(DotAccessExpression {
                        lhs: Box::new(lhs),
                        property_name,
                    })
                })
                .spanned();

            choice((
                unary_expression.clone(),
                function_call,
                dot_access_expression,
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

                Expression::Binop(BinopExpression {
                    op: op.inner,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                })
                .with_span(span)
            },
        );

        let binary_expression_2 = binary_expression_1.clone().foldl(
            binary_operator_2()
                .then(binary_expression_1.clone())
                .repeated(),
            |lhs, (op, rhs)| {
                let span = lhs.span.union(op.span).union(rhs.span);

                Expression::Binop(BinopExpression {
                    op: op.inner,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                })
                .with_span(span)
            },
        );

        let binary_expression_3 = binary_expression_2.clone().foldl(
            binary_operator_3()
                .then(binary_expression_2.clone())
                .repeated(),
            |lhs, (op, rhs)| {
                let span = lhs.span.union(op.span).union(rhs.span);

                Expression::Binop(BinopExpression {
                    op: op.inner,
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
