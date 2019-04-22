use crate::token::Token;
use std::iter::FromIterator;
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    iter: Peekable<Chars<'a>>,
    curr: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(iter: Peekable<Chars<'a>>) -> Lexer<'a> {
        Lexer { iter, curr: None }
    }

    pub fn token(&mut self) -> Option<Token> {
        self.next();

        self.skip_whitespace();

        if self.curr.is_none() {
            return Some(Token::Eof);
        }

        let curr = self.curr.unwrap();

        // Token::Number
        if curr.is_ascii_digit() {
            let mut number = vec![curr];
            while self.is_peek_digit() {
                self.next();
                number.push(self.curr.unwrap());
            }
            let number = String::from_iter(number);
            let number = number.parse::<f64>();
            if let Ok(number) = number {
                return Some(Token::Number(number));
            }
            return None;
        }

        let t = match curr {
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Asterisk),
            '/' => Some(Token::Slash),
            ';' => Some(Token::SemiColon),
            _ => None,
        };
        if let Some(t) = t {
            return Some(t);
        }
        return None;
    }

    fn skip_whitespace(&mut self) {
        while self.curr.is_some() && self.curr.unwrap().is_whitespace() {
            self.next();
        }
    }

    pub fn next(&mut self) {
        let curr = self.iter.next();
        self.curr = curr;
    }

    pub fn curr(&self) -> Option<char> {
        self.curr
    }

    pub fn peek(&mut self) -> Option<&char> {
        self.iter.peek()
    }

    pub fn is_curr(&self, c: char) -> bool {
        self.curr.is_some() && self.curr.unwrap() == c
    }

    pub fn is_peek(&mut self, c: char) -> bool {
        let peek = self.peek();
        peek.is_some() && peek.unwrap() == &c
    }

    fn is_peek_digit(&mut self) -> bool {
        let peek = self.peek();
        if peek.is_none() {
            return false;
        }
        let peek = peek.unwrap();
        if peek.is_ascii_digit() || peek == &'.' {
            return true;
        }
        return false;
    }
}
