use crate::{
    ast::{Expression, Statement},
    ast_store::{ExpressionID, StatementID, Store},
    parser::{Extras, lexer::Token},
    span::{SourceIDSpan, SourceIDSpanned},
};
use chumsky::prelude::*;

macro_rules! store_node {
    ($trait_name:ident, $store:ident, $function_name:ident, $NodeType:ty, $IDType:ty) => {
        pub trait $trait_name<'tokens, I>:
            Parser<'tokens, I, $NodeType, Extras<'tokens>> + Clone + Sized
        where
            I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
        {
            fn $function_name(
                self,
            ) -> impl Parser<'tokens, I, $IDType, Extras<'tokens>> + Clone + Sized {
                self.map_with(|node, extras| extras.state().$store.add(node))
            }
        }

        impl<'tokens, I, P> $trait_name<'tokens, I> for P
        where
            P: Parser<'tokens, I, $NodeType, Extras<'tokens>> + Clone + Sized,
            I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
        {
        }
    };
}

store_node!(
    StoreExpression,
    expressions,
    store_expression,
    SourceIDSpanned<Expression>,
    ExpressionID
);
store_node!(
    StoreStatement,
    statements,
    store_statement,
    SourceIDSpanned<Statement>,
    StatementID
);

// pub trait StoreExpression<'tokens, I>:
//     Parser<'tokens, I, SourceIDSpanned<Expression>, Extras<'tokens>> + Clone + Sized
// where
//     I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
// {
//     fn store_expression(
//         self,
//     ) -> impl Parser<'tokens, I, ExpressionID, Extras<'tokens>> + Clone + Sized {
//         self.map_with(|expression, extras| extras.state().expressions.add(expression))
//     }
// }

// impl<'tokens, I, P> StoreExpression<'tokens, I> for P
// where
//     P: Parser<'tokens, I, SourceIDSpanned<Expression>, Extras<'tokens>> + Clone + Sized,
//     I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
// {
// }

// pub trait StoreStatement<'tokens, I>:
//     Parser<'tokens, I, SourceIDSpanned<Statement>, Extras<'tokens>> + Clone + Sized
// where
//     I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
// {
//     fn store_statement(
//         self,
//     ) -> impl Parser<'tokens, I, StatementID, Extras<'tokens>> + Clone + Sized {
//         self.map_with(|node, extras| extras.state().statements.add(node))
//     }
// }

// impl<'tokens, I, P> StoreStatement<'tokens, I> for P
// where
//     P: Parser<'tokens, I, SourceIDSpanned<Statement>, Extras<'tokens>> + Clone + Sized,
//     I: Input<'tokens, Token = Token, Span = SourceIDSpan>,
// {
// }
