use std::fmt;
use std::mem;
use std::str::FromStr;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum Token {
    Number(f64),
    Identifier(String),

    Plus,
    Minus,
    Asterisk,
    Slash,

    SemiColon,

    Assign,

    LParen,
    RParen,

    Eof,
    Illegal,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Illegal => write!(f, "Illegal"),
            Token::Eof => write!(f, "EOF"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Asterisk => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::SemiColon => write!(f, ";"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::Assign => write!(f, "="),
            Token::Number(n) => write!(f, "Number({})", n),
            Token::Identifier(s) => write!(f, "Identifier({})", s),
        }
    }
}

impl Token {
    pub fn is_same(&self, other: &Token) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }

    pub fn is_identifier(&self) -> bool {
        self.is_same(&Token::Identifier(String::new()))
    }
}
