#[derive(Debug, PartialEq, Default, Eq, Hash, Clone)]
pub struct TokenData {
    pub value: String,
}

use logos::Logos;

#[derive(Logos, Debug, PartialEq, Eq, Hash, Clone)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(extras = TokenData)]
pub enum Token {
    #[token("if")]
    If,
    #[token("else")]
    Else,

    #[token("while")]
    While,
    #[token("for")]
    For,

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Times,
    #[token("/")]
    Division,
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

    #[token("=")]
    Assign,

    #[token("let")]
    Let,
    #[token("function")]
    Function,
    #[token("return")]
    Return,

    #[token(";")]
    Semicolon,

    #[regex("[a-zA-Z]+", |lex| TokenData {value: lex.slice().to_string()})]
    Identifier(TokenData),
    #[regex("[0-9]+", |lex| TokenData {value: lex.slice().to_string()})]
    NumberLiteral(TokenData),
}
