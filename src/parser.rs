use crate::ast;
use crate::lexer::Lexer;
use crate::token::Token;

use core::borrow::Borrow;
use std::mem;
use std::rc::Rc;

pub struct Parser {
    lexer: Lexer,
    curr_token: Option<Token>,
    peek_token: Option<Token>,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Parser {
        let curr_token = lexer.token();
        let peek_token = lexer.token();
        Parser {
            lexer,
            curr_token,
            peek_token,
            errors: vec![],
        }
    }

    pub fn next_token(&mut self) {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.lexer.token();
    }

    pub fn is_curr_token(&self, token: &Token) -> bool {
        if self.curr_token.is_none() {
            return false;
        }
        mem::discriminant(self.curr_token.as_ref().unwrap()) == mem::discriminant(token)
    }

    pub fn is_peek_token(&self, token: &Token) -> bool {
        if self.peek_token.is_none() {
            return false;
        }
        mem::discriminant(self.peek_token.as_ref().unwrap()) == mem::discriminant(token)
    }

    pub fn expect_peek(&mut self, token: &Token) -> bool {
        if self.is_peek_token(token) {
            self.next_token();
            return true;
        } else {
            self.peek_error(token);
            return false;
        }
    }

    fn peek_error(&mut self, token: &Token) {
        self.errors.push(format!(
            "expected next token to be {:?}, got {:?} insted",
            token, self.peek_token
        ))
    }

    fn peek_precedence(&self) -> Precedence {
        let token = self.peek_token.borrow();
        if token.is_none() {
            return Precedence::LOWEST;
        }
        return Precedence::token_precedence(token.as_ref().unwrap());
    }

    /**

    Parser parse functions

    **/

    pub fn parse_program(&mut self) -> Option<ast::Program> {
        let mut statements = vec![];
        while self.curr_token.is_some() && !self.is_curr_token(&Token::Eof) {
            let statement = self.parse_statement();
            if let Some(statement) = statement {
                statements.push(statement);
            }
            self.next_token();
        }
        return Some(ast::Program { statements });
    }

    fn parse_statement(&mut self) -> Option<Rc<ast::Statement>> {
        let expression = self.parse_expression(Precedence::LOWEST)?;
        self.expect_peek(&Token::SemiColon);
        return Some(Rc::new(ast::Statement::ExpressionStatement { expression }));
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Rc<ast::Expression>> {
        let mut left = self.parse_prefix()?;

        while !self.is_peek_token(&Token::SemiColon) && precedence < self.peek_precedence() {
            self.next_token();
            left = self.parse_infix(left)?;
        }

        return Some(left);
    }

    fn parse_prefix(&mut self) -> Option<Rc<ast::Expression>> {
        match self.curr_token.as_ref()? {
            Token::Number(_) => self.parse_integer_literal(),
            _ => None,
        }
    }

    fn parse_infix(&mut self, left: Rc<ast::Expression>) -> Option<Rc<ast::Expression>> {
        let token = self.curr_token.as_ref()?;
        match token {
            Token::Plus | Token::Minus | Token::Asterisk | Token::Slash => {
                self.parse_infix_expression(left)
            }
            _ => None,
        }
    }

    pub fn parse_integer_literal(&mut self) -> Option<Rc<ast::Expression>> {
        if let Some(Token::Number(n)) = self.curr_token {
            Some(Rc::new(ast::Expression::NumberLiteral(n)))
        } else {
            None
        }
    }

    pub fn parse_infix_expression(&mut self, left: Rc<ast::Expression>) -> Option<Rc<ast::Expression>> {
        let token = self.curr_token.as_ref()?;
        let operator = format!("{}", token);
        let precedence = Precedence::token_precedence(token);
        self.next_token();
        let right = self.parse_expression(precedence)?;
        return Some(Rc::new(ast::Expression::InfixExpression {left, operator, right}));
    }
}

#[derive(PartialOrd, PartialEq)]
enum Precedence {
    LOWEST,
    SUM,
    PRODUCT,
    PREFIX,
}

impl Precedence {
    fn token_precedence(token: &Token) -> Precedence {
        match token {
            Token::Plus => Precedence::SUM,
            Token::Minus => Precedence::SUM,
            Token::Slash => Precedence::PRODUCT,
            Token::Asterisk => Precedence::PRODUCT,
            _ => Precedence::LOWEST,
        }
    }
}
