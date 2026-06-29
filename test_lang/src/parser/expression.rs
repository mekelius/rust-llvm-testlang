use chumsky::prelude::*;

use crate::{
    ast::{
        BinaryOperator, BinopExpression, Expression, FunctionCall, Literal, Node, UnaryOperator,
        UnopExpression,
    },
    parenthesized,
    parser::{
        ParserError,
        common::{identifier, type_expression},
        lexer::Token,
    },
    span::{SourceIDSpan, SourceIDSpanned},
};

pub fn number_literal<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<Expression>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    select! {
        Token::NumberLiteral(value) => Expression::Literal(Literal::NumberLiteral(value)),
    }
    .spanned()
}

pub fn string_literal<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<Expression>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    select! {
        Token::StringLiteral(value) => Expression::Literal(Literal::StringLiteral(value)),
    }
    .spanned()
}

pub fn boolean_literal<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<Expression>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    select! {
        Token::True => Expression::Literal(Literal::BooleanLiteral(true)),
        Token::False => Expression::Literal(Literal::BooleanLiteral(false)),
    }
    .spanned()
}

pub fn unit_literal<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<Expression>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    just(Token::LParenthesis)
        .ignore_then(just(Token::RParenthesis))
        .to(Expression::Literal(Literal::UnitLiteral))
        .spanned()
}
pub fn binary_op_1<'src, I>()
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

pub fn binary_op_2<'src, I>()
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

pub fn binary_op_3<'src, I>()
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

pub fn literal<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<Expression>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    choice((
        string_literal(),
        number_literal(),
        boolean_literal(),
        unit_literal(),
    ))
}

pub fn expression<'src, I>()
-> impl Parser<'src, I, SourceIDSpanned<Expression>, ParserError<'src>> + Clone
where
    I: Input<'src, Token = Token, Span = SourceIDSpan>,
{
    let identifier = identifier();

    recursive(|expr| {
        let argument_list = parenthesized!(
            expr.clone()
                .separated_by(just(Token::Comma))
                .collect::<Vec<SourceIDSpanned<Expression>>>()
                .map(|e| Node::ArgumentList(e))
        );

        let function_call = identifier
            .clone()
            .then(argument_list)
            .map(|(callee_expression, arguments)| {
                let callee = match callee_expression.inner {
                    Expression::Identifier(value) => value.with_span(callee_expression.span),
                    _ => todo!(),
                };

                let argument_list = match arguments {
                    Node::ArgumentList(argument_list) => argument_list,
                    _ => todo!(),
                };

                Expression::FunctionCall(FunctionCall {
                    callee,
                    argument_list,
                })
            })
            .spanned();

        let typed_expression = type_expression()
            .clone()
            .then(expr.clone())
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
                parenthesized!(expr.clone()),
            ))
        });

        let binary_expression_1 = term.clone().foldl(
            binary_op_1().then(term.clone()).repeated(),
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
            binary_op_2().then(binary_expression_1.clone()).repeated(),
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
            binary_op_3().then(binary_expression_2.clone()).repeated(),
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
            parenthesized!(expr.clone()),
        ))
    })
}
