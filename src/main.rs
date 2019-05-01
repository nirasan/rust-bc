use core::borrow::Borrow;
use std::mem;

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LParen,
    RParen,
}

struct Lexer {
    input: Vec<char>,
    position: usize
}

impl Lexer {
    fn new(input: Vec<char>) -> Lexer {
        Lexer{
            input,
            position: 0
        }
    }
    fn token(&mut self) -> Option<Token> {
        use std::iter::FromIterator;
        while self.curr().is_some() && self.curr().unwrap().is_whitespace() {
            self.next();
        }
        let curr = self.curr()?;
        let token = if Self::is_number(curr) {
            let mut number = vec![*curr];
            while self.peek().is_some() && Self::is_number(self.peek().unwrap()) {
                self.next();
                number.push(*self.curr().unwrap());
            }
            String::from_iter(number).parse::<f64>().ok().and_then(|n| Some(Token::Number(n)))
        } else {
            match curr {
                &'+' => Some(Token::Plus),
                &'-' => Some(Token::Minus),
                &'*' => Some(Token::Asterisk),
                &'/' => Some(Token::Slash),
                &'(' => Some(Token::LParen),
                &')' => Some(Token::RParen),
                _ => None,
            }
        };
        self.next();
        return token;
    }
    fn next(&mut self) {
        self.position += 1;
    }
    fn curr(&mut self) -> Option<&char> {
        self.input.get(self.position)
    }
    fn peek(&mut self) -> Option<&char> {
        self.input.get(self.position + 1)
    }
    fn is_number(c: &char) -> bool {
        c.is_ascii_digit() || c == &'.'
    }
}

#[test]
fn test_lexer() {
    let mut lexer = Lexer::new("1 + 2".chars().collect());
    assert_eq!(lexer.token(), Some(Token::Number(1_f64)));
    assert_eq!(lexer.token(), Some(Token::Plus));
    assert_eq!(lexer.token(), Some(Token::Number(2_f64)));
    assert_eq!(lexer.token(), None);
}

#[derive(Debug)]
enum Expr {
    Number(f64),
    PrefixExpr{operator: String, right: Box<Expr>},
    InfixExpr{left: Box<Expr>, operator: String, right: Box<Expr>},
}

#[derive(PartialOrd, PartialEq)]
enum Precedence {
    LOWEST,
    SUM,
    PRODUCT,
    PREFIX,
}

struct Parser {
    lexer: Lexer,
    curr: Option<Token>,
    peek: Option<Token>,
}

impl Parser {
    fn new(mut lexer: Lexer) -> Parser {
        let curr = lexer.token();
        let peek = lexer.token();
        Parser {
            lexer,
            curr,
            peek,
        }
    }
    fn parse(&mut self, precedence: Precedence) -> Option<Box<Expr>> {
        let mut left = self.parse_prefix()?;

        while self.peek.is_some() && precedence < self.peek_precedence() {
            self.next();
            left = self.parse_infix(left)?;
        }

        return Some(left);
    }
    fn parse_prefix(&mut self) -> Option<Box<Expr>> {
        match self.curr.as_ref()? {
            Token::Minus => self.parse_minus(),
            Token::Number(_) => self.parse_number(),
            Token::LParen => self.parse_grouped_expression(),
            _ => None,
        }
    }
    pub fn parse_minus(&mut self) -> Option<Box<Expr>> {
        self.next();
        let number = self.parse(Precedence::PREFIX)?;
        return Some(Box::new(Expr::PrefixExpr {operator: "Minus".to_string(), right: number}));
    }
    pub fn parse_number(&mut self) -> Option<Box<Expr>> {
        match self.curr.borrow() {
            Some(Token::Number(n)) => Some(Box::new(Expr::Number(*n))),
            _ => None,
        }
    }
    pub fn parse_grouped_expression(&mut self) -> Option<Box<Expr>> {
        self.next();
        let expression = self.parse(Precedence::LOWEST);
        if self.peek_is(&Token::RParen) {
            self.next();
            return expression;
        } else {
            return None;
        }
    }
    fn parse_infix(&mut self, left: Box<Expr>) -> Option<Box<Expr>> {
        let token = self.curr.as_ref()?;
        match token {
            Token::Plus | Token::Minus | Token::Asterisk | Token::Slash => {
                self.parse_infix_expression(left)
            }
            _ => Some(left),
        }
    }
    pub fn parse_infix_expression(&mut self, left: Box<Expr>) -> Option<Box<Expr>> {
        let token = self.curr.as_ref()?;
        let operator = format!("{:?}", token);
        let precedence = Self::token_precedence(token);
        self.next();
        let right = self.parse(precedence)?;
        return Some(Box::new(Expr::InfixExpr {left, operator, right}));
    }
    fn next(&mut self) {
        self.curr = self.peek.clone();
        self.peek = self.lexer.token();
    }
    fn peek_is(&self, token: &Token) -> bool {
        if self.peek.is_none() {
            return false;
        }
        mem::discriminant(self.peek.as_ref().unwrap()) == mem::discriminant(token)
    }
    fn peek_precedence(&self) -> Precedence {
        let token = self.peek.borrow();
        if token.is_none() {
            return Precedence::LOWEST;
        }
        return Self::token_precedence(token.as_ref().unwrap());
    }
    fn token_precedence(token: &Token) -> Precedence {
        match token {
            Token::Plus | Token::Minus => Precedence::SUM,
            Token::Slash | Token::Asterisk => Precedence::PRODUCT,
            _ => Precedence::LOWEST,
        }
    }
}

#[test]
fn test_parser() {
    do_parser("1 + 2",
        r#"Some(InfixExpr { left: Number(1.0), operator: "Plus", right: Number(2.0) })"#);
    do_parser("- 1 + 2 * 3",
             r#"Some(InfixExpr { left: PrefixExpr { operator: "Minus", right: Number(1.0) }, operator: "Plus", right: InfixExpr { left: Number(2.0), operator: "Asterisk", right: Number(3.0) } })"#);
}

fn do_parser(input: &str, expect: &str) {
    let lexer = Lexer::new(input.chars().collect());
    let mut parser = Parser::new(lexer);
    assert_eq!(format!("{:?}", parser.parse(Precedence::LOWEST)), expect);
}

fn eval(expr: &Expr) -> f64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::PrefixExpr {operator: _, right} => { - eval(right) },
        Expr::InfixExpr {left, operator, right} => {
            let left = eval(left);
            let right = eval(right);
            match operator.as_str() {
                "Plus" => left + right,
                "Minus" => left - right,
                "Asterisk" => left * right,
                "Slash" => left / right,
                _ => panic!("invalid operator"),
            }
        },
    }
}

#[test]
fn test_eval() {
    do_eval("1 + 2", 3_f64);
    do_eval("1 + 2 * 3", 7_f64);
    do_eval("1 + (2 + 3) * -(3 / 3)", -4_f64);
}

fn do_eval(input: &str, expect: f64) {
    let lexer = Lexer::new(input.chars().collect());
    let mut parser = Parser::new(lexer);
    let result = eval(parser.parse(Precedence::LOWEST).unwrap().borrow());
    assert_eq!(result, expect);
}
