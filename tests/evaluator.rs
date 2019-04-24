extern crate rust_bc;

use rust_bc::lexer::Lexer;
use rust_bc::parser::Parser;
use rust_bc::evaluator::*;
use rust_bc::object::Object;
use core::borrow::Borrow;

#[test]
fn test_eval_program() {
    let mut parser = Parser::new(Lexer::new("1 + 2 * 3".chars().collect()));
    let program = parser.parse_program().unwrap();
    let results = eval_program(program);
    assert_eq!(results.len(), 1);
    let result = results.get(0).unwrap();
    if let Object::Number(n) = result.borrow() {
        assert_eq!(n, &7_f64);
    }

    let mut parser = Parser::new(Lexer::new("1 + 2 * 3; 6 / 3 * 2".chars().collect()));
    let program = parser.parse_program().unwrap();
    let results = eval_program(program);
    assert_eq!(results.len(), 2);
    let result = results.get(0).unwrap();
    if let Object::Number(n) = result.borrow() {
        assert_eq!(n, &7_f64);
    }
    let result = results.get(1).unwrap();
    if let Object::Number(n) = result.borrow() {
        assert_eq!(n, &4_f64);
    }

}