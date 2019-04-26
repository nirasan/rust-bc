#[derive(Debug, PartialEq)]
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


