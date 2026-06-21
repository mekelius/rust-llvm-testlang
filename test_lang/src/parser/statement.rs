use chumsky::prelude::*;

use crate::{ast::Node, in_curly_braces, parenthesized, parser::{ParserError, expression::expression, lexer::{Token, TokenData}}};

pub fn statement<'src>() -> impl Parser<'src, &'src [Token], Node, ParserError<'src>> + Clone {
    let expression = expression();
    recursive(|p| {
        let if_statement = just(Token::If)
            .ignore_then(parenthesized!(expression.clone()))
            .then(p.clone())
            .map(|(condition, body)| Node::IfStatement {
                condition: Box::new(condition),
                body: Box::new(body),
            })
            .boxed();

        let if_else_statement = if_statement
            .clone()
            .then_ignore(just(Token::Else))
            .then(p.clone())
            .map(|(if_branch, else_branch)| {
                Node::IfElseStatement(Box::new(if_branch), Box::new(else_branch))
            });

        let statement_list = p.clone().repeated().collect::<Vec<Node>>();

        let case = just(Token::Case)
            .ignore_then(select! {
                Token::NumberLiteral(TokenData{value}) => value,
            })
            .then_ignore(just(Token::Colon))
            .then(statement_list.clone())
            .map(|(value, body)| Node::Case { value, body });

        let default_case = just(Token::Default)
            .ignore_then(just(Token::Colon))
            .ignore_then(statement_list.clone())
            .map(|case_body| Node::DefaultCase(case_body));

        let switch_statement = just(Token::Switch)
            .ignore_then(expression.clone())
            .then(in_curly_braces!(
                (case.or(default_case)).repeated().collect()
            ))
            .map(|(expression, cases)| Node::SwitchStatement {
                matched_value_expression: Box::new(expression),
                cases,
            });

        let empty_statement = empty().to(Node::EmptyStatement).boxed();

        let block = in_curly_braces!(statement_list.clone().map(|e| Node::Block(e))).boxed();

        let assignment = (select! {
            Token::Identifier(TokenData {value}) => value,
        })
        .then_ignore(just(Token::SingleEquals))
        .then(expression.clone());

        let const_statement = just(Token::Const)
            .ignore_then(assignment.clone())
            .map(|(name, value)| Node::ConstStatement(name, Box::new(value)))
            .boxed();

        let let_statement = just(Token::Let)
            .ignore_then(assignment.clone())
            .map(|(name, value)| Node::LetStatement(name, Box::new(value)))
            .boxed();

        let assignment_statement = assignment
            .clone()
            .map(|(name, value)| Node::AssignmentStatement(name, Box::new(value)))
            .boxed();

        let valueless_return_statement = just(Token::Return).to(Node::ValuelessReturnStatement);

        let return_statement = just(Token::Return)
            .ignore_then(expression.clone())
            .map(|expr| Node::ReturnStatement(Box::new(expr)))
            .boxed();

        let continue_statement = just(Token::Continue).to(Node::ContinueStatement);
        let break_statement = just(Token::Break).to(Node::BreakStatement);

        let expression_statement = expression
            .clone()
            .map(|e| Node::ExpressionStatement(Box::new(e)));

        let simple_statement = choice((
            let_statement.clone(),
            const_statement.clone(),
            assignment_statement.clone(),
            return_statement.clone(),
            continue_statement.clone(),
            break_statement.clone(),
            valueless_return_statement.clone(),
            expression_statement.clone(),
            empty_statement.clone(),
        ))
        .boxed();
        let single_statement = simple_statement.clone().then_ignore(just(Token::Semicolon)).boxed();

        // For loop
        let for_init = let_statement.clone().or(assignment_statement.clone());
        let for_condition = expression.clone();
        let for_step = simple_statement.clone();

        let for_statement = just(Token::For)
            .ignore_then(parenthesized!(
                for_init
                    .clone()
                    .then_ignore(just(Token::Semicolon))
                    .then(for_condition.clone())
                    .then_ignore(just(Token::Semicolon))
                    .then(for_step.clone())
            ))
            .then(p.clone())
            .map(|(((init, condition), step), body)| Node::ForStatement {
                init: Box::new(init),
                condition: Box::new(condition),
                step: Box::new(step),
                body: Box::new(body),
            })
            .boxed();

        let while_statement = just(Token::While)
            .ignore_then(parenthesized!(expression.clone()))
            .then(p.clone())
            .map(|(condition, body)| Node::WhileStatement {
                condition: Box::new(condition),
                body: Box::new(body),
            })
            .boxed();

        let complex_statement = choice((
            if_else_statement.clone(),
            if_statement.clone(),
            switch_statement.clone(),
            while_statement.clone(),
            for_statement.clone(),
        ))
        .boxed();

        single_statement.or(block).or(complex_statement).boxed()
    })
    .boxed()
}
