use std::error::Error;

#[derive(Debug, PartialEq, Default, Eq, Hash, Clone)]
pub struct TokenData {
    pub value: String,
}

use logos::Logos;
use logos::skip;

#[derive(Logos, Debug, PartialEq, Eq, Hash, Clone)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(extras = TokenData)]
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
    Times,
    #[regex("/")]
    Divided,
    #[token("%")]
    Modulo,

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
    Equals,
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

    #[token("=")]
    Assign,

    #[token("let")]
    Let,
    #[token("function")]
    Function,
    #[token("return")]
    Return,

    #[token("true")]
    True,
    #[token("false")]
    False,

    #[token(";")]
    Semicolon,

    #[regex("[a-z_][a-zA-Z0-9_']*", |lex| TokenData {value: lex.slice().to_string()})]
    Identifier(TokenData),
    #[regex("[A-Z][a-zA-Z0-9_']*", |lex| TokenData {value: lex.slice().to_string()})]
    TypeIdentifier(TokenData),

    #[regex("\"[^\"]+\"", |lex| {
        let string = lex.slice();
        // String the quotes
        TokenData {value: string[1..string.len()-1].to_string()}})]
    #[regex("\'[^\']+\'", |lex| {
        let string = lex.slice();
        // String the quotes
        TokenData {value: string[1..string.len()-1].to_string()}})]
    StringLiteral(TokenData),
    #[regex("[0-9]+", |lex| TokenData {value: lex.slice().to_string()})]
    NumberLiteral(TokenData),
}

pub fn run(src: &str) -> Result<Vec<Token>, Box<dyn Error>> {
    let lexer = Token::lexer(&src);

    let mut tokens = vec![];
    for (token, span) in lexer.spanned() {
        match token {
            Ok(token) => tokens.push(token),
            Err(e) => {
                panic!("lexer error at {:?}: {:?}", span, e);
            }
        }
    }

    Ok(tokens)
}
