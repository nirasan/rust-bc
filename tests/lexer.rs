extern crate rust_bc;

use rust_bc::lexer::Lexer;

#[test]
fn test_new() {
    let input = "1 + 2 + 3 + 4".to_string();
    let mut lexer = Lexer::new(input.chars().peekable());
    assert_eq!(lexer.curr(), None);
    assert_eq!(lexer.peek(), Some(&'1'));

    lexer.next();
    assert_eq!(lexer.curr(), Some('1'));
    assert_eq!(lexer.peek(), Some(&' '));

    lexer.next();
    assert_eq!(lexer.curr(), Some(' '));
    assert_eq!(lexer.peek(), Some(&'+'));

    assert_eq!(lexer.is_peek('+'), true)
}