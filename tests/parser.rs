extern crate rust_bc;

use core::borrow::Borrow;
use std::rc::Rc;

use rust_bc::ast;
use rust_bc::lexer::Lexer;
use rust_bc::parser::Parser;

#[test]
fn test_parse_integer_literal() {
    let mut parser = Parser::new(Lexer::new("1 + 2".chars().collect()));
    let exp = parser.parse_integer_literal();
    assert_eq!(exp.is_some(), true);
    let exp = exp.unwrap();
    if let ast::Expression::NumberLiteral(n) = exp.borrow() {
        assert_eq!(n, &1_f64);
    }

    parser.next_token();
    assert_eq!(parser.parse_integer_literal().is_none(), true);
}

#[test]
fn test_parse_program() {
    let mut parser = Parser::new(Lexer::new("1 * 2 * 3".chars().collect()));
    let program = parser.parse_program();
    println!("{:?}", program);
    assert_eq!(format!("{:?}", program), r#"Some(Program { statements: [ExpressionStatement { expression: InfixExpression { left: InfixExpression { left: NumberLiteral(1.0), operator: "Asterisk", right: NumberLiteral(2.0) }, operator: "Asterisk", right: NumberLiteral(3.0) } }] })"#);

    let mut parser = Parser::new(Lexer::new("1 + 2 * 3".chars().collect()));
    let program = parser.parse_program();
    println!("{:?}", program);
    assert_eq!(format!("{:?}", program), r#"Some(Program { statements: [ExpressionStatement { expression: InfixExpression { left: NumberLiteral(1.0), operator: "Plus", right: InfixExpression { left: NumberLiteral(2.0), operator: "Asterisk", right: NumberLiteral(3.0) } } }] })"#);
}
