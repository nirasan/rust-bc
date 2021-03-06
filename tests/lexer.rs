extern crate rust_bc;

use rust_bc::lexer::Lexer;
use rust_bc::token::Token;

#[test]
fn test_new() {
    let input = "1 + 2 + 3 + 4";
    let mut lexer = Lexer::new(input.chars().collect());
    assert_eq!(lexer.curr(), Some(&'1'));
    assert_eq!(lexer.peek(), Some(&' '));

    lexer.next();
    assert_eq!(lexer.curr(), Some(&' '));
    assert_eq!(lexer.peek(), Some(&'+'));

    assert_eq!(lexer.is_peek('+'), true)
}

#[test]
fn test_token() {
    let input = "1 + 2 - 3 * 4 / 5;";
    let mut lexer = Lexer::new(input.chars().collect());

    assert_eq!(lexer.token(), Some(Token::Number(1_f64)));
    assert_eq!(lexer.token(), Some(Token::Plus));
    assert_eq!(lexer.token(), Some(Token::Number(2_f64)));
    assert_eq!(lexer.token(), Some(Token::Minus));
    assert_eq!(lexer.token(), Some(Token::Number(3_f64)));
    assert_eq!(lexer.token(), Some(Token::Asterisk));
    assert_eq!(lexer.token(), Some(Token::Number(4_f64)));
    assert_eq!(lexer.token(), Some(Token::Slash));
    assert_eq!(lexer.token(), Some(Token::Number(5_f64)));
    assert_eq!(lexer.token(), Some(Token::SemiColon));
    assert_eq!(lexer.token(), Some(Token::Eof));
}

#[test]
fn test_assign() {
    let input = "val = 100";
    let mut lexer = Lexer::new(input.chars().collect());

    assert_eq!(lexer.token(), Some(Token::Identifier("val".to_string())));
    assert_eq!(lexer.token(), Some(Token::Assign));
    assert_eq!(lexer.token(), Some(Token::Number(100_f64)));
    assert_eq!(lexer.token(), Some(Token::Eof));
}
