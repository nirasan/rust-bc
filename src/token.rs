use std::fmt;
use std::str::FromStr;
use std::mem;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum Token {
    Number(f64),

    Plus,
    Minus,
    Asterisk,
    Slash,

    SemiColon,

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
            Token::Number(n) => write!(f, "Number({})", n),
        }
    }
}

impl Token {
    pub fn is_same(&self, other: &Token) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }
}
