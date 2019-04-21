use crate::token::Token;
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    iter: Peekable<Chars<'a>>,
    curr: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(iter: Peekable<Chars<'a>>) -> Lexer<'a> {
        Lexer{
            iter,
            curr: None,
        }
    }

    pub fn curr(&self) -> Option<char> {
        self.curr
    }

    pub fn next(&mut self) {
        let curr = self.iter.next();
        self.curr = curr;
    }

    pub fn peek(&mut self) -> Option<&char> {
        self.iter.peek()
    }

    pub fn is_peek(&mut self, char: char) -> bool {
        let peek = self.peek();
        peek.is_some() && peek.unwrap() == &char
    }
}