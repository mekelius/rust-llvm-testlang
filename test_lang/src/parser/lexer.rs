use std::error::Error;

use logos::Logos;
use logos::skip;

use crate::span::SourceID;
use crate::span::SourceIDSpan;

#[derive(Logos, Debug, PartialEq, Eq, Hash, Clone)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(extras = String)]
pub enum Token {
    #[regex(r"//[^\n]*", skip, allow_greedy = true)]
    LineComment,
    #[regex(r"(?s)/\*.*\*/", skip, allow_greedy = true)]
    BlockComment,

    #[token("if")]
    If,
    #[token("else")]
    Else,

    #[token("switch")]
    Switch,
    #[token("case")]
    Case,
    #[token("default")]
    Default,
    #[token("break")]
    Break,
    #[token("return")]
    Return,
    #[token("continue")]
    Continue,

    #[token("while")]
    While,
    #[token("for")]
    For,

    #[token("->")]
    ArrowSingle,

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Asterisk,
    #[regex("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("!")]
    Bang,

    #[token("(")]
    LParenthesis,
    #[token(")")]
    RParenthesis,

    #[token("{")]
    LCurlyBrace,
    #[token("}")]
    RCurlyBrace,

    #[token("[")]
    LSquareBracket,
    #[token("]")]
    RSquareBracket,

    #[token(".")]
    Period,
    #[token(",")]
    Comma,

    #[token("==")]
    DoubleEquals,
    #[token("<=")]
    LessThanOrEquals,
    #[token(">=")]
    GreaterThanOrEquals,
    #[token("<")]
    LessThan,
    #[token(">")]
    GreaterThan,
    #[token("!=")]
    NotEquals,

    #[token("+=")]
    PlusEquals,
    #[token("-=")]
    MinusEquals,
    #[token("*=")]
    AsteriskEquals,
    #[token("/=")]
    SlashEquals,
    #[token("%=")]
    PercentEquals,

    #[token("&&")]
    DoubleAmpersand,
    #[token("||")]
    DoublePipe,

    #[token("&")]
    Ampersand,
    #[token("|")]
    Pipe,
    #[token("~")]
    Tilde,
    #[token("^")]
    Caret,

    #[token("=")]
    SingleEquals,

    #[token("let")]
    Let,
    #[token("const")]
    Const,

    #[token("true")]
    True,
    #[token("false")]
    False,

    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,

    #[regex("[a-z_][a-zA-Z0-9_']*", |lex| lex.slice().to_string())]
    Identifier(String),
    #[regex("[A-Z][a-zA-Z0-9_']*", |lex| lex.slice().to_string())]
    TypeIdentifier(String),

    #[regex("\"[^\"]+\"", |lex| {
        let string = lex.slice();
        // String the quotes
        string[1..string.len()-1].to_string()})]
    #[regex("\'[^\']+\'", |lex| {
        let string = lex.slice();
        // String the quotes
        string[1..string.len()-1].to_string()})]
    StringLiteral(String),
    #[regex("[0-9]+", |lex| lex.slice().to_string())]
    NumberLiteral(String),
}

pub fn run(src: &str, source_id: SourceID) -> Result<Vec<(Token, SourceIDSpan)>, Box<dyn Error>> {
    let lexer = Token::lexer(&src);

    let mut tokens = vec![];
    for (token, span) in lexer.spanned() {
        match token {
            Ok(token) => tokens.push((
                token,
                SourceIDSpan {
                    start: span.start,
                    end: span.end,
                    context: source_id,
                },
            )),
            Err(e) => {
                panic!("lexer error at {:?}: {:?}", span, e);
            }
        }
    }

    Ok(tokens)
}
