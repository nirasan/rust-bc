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
    do_parse_program(
        "1 * 2 * 3",
        r#"Some(Program { statements: [ExpressionStatement { expression: InfixExpression { left: InfixExpression { left: NumberLiteral(1.0), operator: "*", right: NumberLiteral(2.0) }, operator: "*", right: NumberLiteral(3.0) } }] })"#
    );

    do_parse_program(
        "1 + 2 * 3",
        r#"Some(Program { statements: [ExpressionStatement { expression: InfixExpression { left: NumberLiteral(1.0), operator: "+", right: InfixExpression { left: NumberLiteral(2.0), operator: "*", right: NumberLiteral(3.0) } } }] })"#
    );

    do_parse_program(
        "val = 4",
        r#"Some(Program { statements: [AssignStatement { name: "val", expression: NumberLiteral(4.0) }] })"#
    );

    do_parse_program(
        "val + 5 * 6",
        r#"Some(Program { statements: [ExpressionStatement { expression: InfixExpression { left: Identifier("val"), operator: "+", right: InfixExpression { left: NumberLiteral(5.0), operator: "*", right: NumberLiteral(6.0) } } }] })"#
    );

    do_parse_program(
        "1 / (2 + 3)",
        r#"Some(Program { statements: [ExpressionStatement { expression: InfixExpression { left: NumberLiteral(1.0), operator: "/", right: InfixExpression { left: NumberLiteral(2.0), operator: "+", right: NumberLiteral(3.0) } } }] })"#
    );
}

fn do_parse_program(input: &str, expect: &str) {
    let mut parser = Parser::new(Lexer::new(input.chars().collect()));
    let program = parser.parse_program();
    assert_eq!(format!("{:?}", program), expect);
}
