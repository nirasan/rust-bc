use crate::token::Token;
use std::iter::FromIterator;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Lexer {
        Lexer { input, position: 0 }
    }

    pub fn token(&mut self) -> Option<Token> {
        let token = self.get_token();
        self.next();
        return token;
    }

    fn get_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if self.curr().is_none() {
            return Some(Token::Eof);
        }

        let curr = *self.curr().unwrap();

        // Token::Number
        if curr.is_ascii_digit() {
            let mut number = vec![curr];
            while self.is_peek_digit() {
                self.next();
                number.push(*self.curr().unwrap());
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
        while self.curr().is_some() && self.curr().unwrap().is_whitespace() {
            self.next();
        }
    }

    pub fn next(&mut self) {
        self.position += 1;
    }

    pub fn curr(&self) -> Option<&char> {
        self.input.get(self.position)
    }

    pub fn peek(&self) -> Option<&char> {
        self.input.get(self.position + 1)
    }

    pub fn is_curr(&self, c: char) -> bool {
        self.curr().is_some() && self.curr().unwrap() == &c
    }

    pub fn is_peek(&self, c: char) -> bool {
        self.peek().is_some() && self.peek().unwrap() == &c
    }

    fn is_peek_digit(&self) -> bool {
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
