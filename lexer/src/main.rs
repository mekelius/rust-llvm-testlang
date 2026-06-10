#[derive(Debug, PartialEq, Default)]
struct TokenData {
    value: String,
}

use logos::Logos;
use logos::Lexer;

fn identifier_callback(lex: &mut Lexer<Token>) -> TokenData {
    // let line = lex.extras.0;
    // let column = lex.span().start - lex.extras.1;

    TokenData{
        value: lex.slice().to_string(),
    }
}

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(extras = TokenData)]
enum Token {
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

    #[token("=")]
    Equals,
    #[token("<")]
    LessThan,
    #[token(">")]
    GreaterThan,

    #[token("let")]
    Let,
    #[token("function")]
    Function,
    #[token("return")]
    Return,

    #[token(";")]
    Semicolon,

    #[regex("[a-zA-Z]+", identifier_callback)]
    Identifier(TokenData),
    #[regex("[0-9]+", identifier_callback)]
    NumberLiteral(TokenData),
}

fn main() {
    let source = "return asdj;  \n\nasok = let x+231*2";
    for result in Token::lexer(&source) {
        match result {
            Ok(token) => println!("{:#?}", token),
            Err(e) => panic!("Lexing failed: {:?}", e),
        }
    }
}
