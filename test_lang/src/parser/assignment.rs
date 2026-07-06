use chumsky::prelude::*;

use crate::{
    ast::{BinaryOperator, BinopExpression, Expression, Statement, UnaryOperator, UnopExpression},
    ast_store::{ExpressionID, NUMBER_1_LITERAL, StatementID, Store},
    parser::{Extras, expression::expression, lexer::Token, store_node::StoreStatement},
    span::{SourceIDSpan, SourceIDSpanned},
};

#[derive(Clone)]
enum PostfixAssignment {
    Increment,
    Decrement,
    Negate,
}

fn plus_plus<'tokens, I>()
-> impl Parser<'tokens, I, SourceIDSpanned<PostfixAssignment>, Extras<'tokens>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
{
    just(Token::Plus)
        .then(just(Token::Plus))
        .to(PostfixAssignment::Increment)
        .spanned()
}

fn minus_minus<'tokens, I>()
-> impl Parser<'tokens, I, SourceIDSpanned<PostfixAssignment>, Extras<'tokens>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
{
    just(Token::Minus)
        .then(just(Token::Minus))
        .to(PostfixAssignment::Decrement)
        .spanned()
}

fn bang_bang<'tokens, I>()
-> impl Parser<'tokens, I, SourceIDSpanned<PostfixAssignment>, Extras<'tokens>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
{
    just(Token::Bang)
        .then(just(Token::Bang))
        .to(PostfixAssignment::Negate)
        .spanned()
}

pub fn shorthand_assignment_operator<'tokens, I>()
-> impl Parser<'tokens, I, SourceIDSpanned<Token>, Extras<'tokens>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
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

fn postfix_assignment_operator<'tokens, I>()
-> impl Parser<'tokens, I, SourceIDSpanned<PostfixAssignment>, Extras<'tokens>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
{
    choice((plus_plus(), minus_minus(), bang_bang()))
}

pub fn assignment<'tokens, I>()
-> impl Parser<'tokens, I, (ExpressionID, ExpressionID), Extras<'tokens>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
{
    expression()
        .then_ignore(just(Token::SingleEquals))
        .then(expression())
}

pub fn shorthand_assignment<'tokens, I>()
-> impl Parser<'tokens, I, (ExpressionID, ExpressionID), Extras<'tokens>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
{
    let expression = expression();

    expression
        .clone()
        .then(shorthand_assignment_operator())
        .then::<SourceIDSpanned<ExpressionID>, _>(expression.spanned())
        .map_with(|((lvalue, operator), rhs), extras| {
            let rhs_span = rhs.span.clone();
            let op = match operator.inner {
                Token::PlusEquals => BinaryOperator::Add,
                Token::MinusEquals => BinaryOperator::Sub,
                Token::AsteriskEquals => BinaryOperator::Mul,
                Token::SlashEquals => BinaryOperator::Div,
                Token::PercentEquals => BinaryOperator::Mod,
                _ => todo!("Unhandled shorthand assignment operator"),
            };
            let rhs = Expression::Binop(BinopExpression {
                op,
                lhs: lvalue.clone(),
                rhs: rhs.inner,
            })
            .with_span(operator.span.union(rhs_span));

            let rhs = extras.state().expressions.add(rhs);

            (lvalue, rhs)
        })
}

pub fn postfix_assignment<'tokens, I>()
-> impl Parser<'tokens, I, (ExpressionID, ExpressionID), Extras<'tokens>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
{
    expression()
        .then(postfix_assignment_operator())
        .map_with(|(lvalue, operator), extras| {
            let store = &mut extras.state().expressions;
            let rhs = match operator.inner {
                PostfixAssignment::Increment => Expression::Binop(BinopExpression {
                    op: BinaryOperator::Add,
                    lhs: lvalue,
                    rhs: NUMBER_1_LITERAL,
                }),
                PostfixAssignment::Decrement => Expression::Binop(BinopExpression {
                    op: BinaryOperator::Sub,
                    lhs: lvalue,
                    rhs: NUMBER_1_LITERAL,
                }),
                PostfixAssignment::Negate => Expression::Unop(UnopExpression {
                    op: UnaryOperator::UnaryNot,
                    term: lvalue,
                }),
            }
            .with_span(operator.span);

            let rhs = store.add(rhs);

            (lvalue, rhs)
        })
}

pub fn assignment_statement<'tokens, I>()
-> impl Parser<'tokens, I, StatementID, Extras<'tokens>> + Clone
where
    I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
{
    choice((assignment(), shorthand_assignment(), postfix_assignment()))
        .map(|(lvalue, value)| Statement::Assignment(lvalue, value))
        .spanned()
        .store_statement()
}
