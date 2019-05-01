extern crate rust_bc;

use core::borrow::Borrow;
use rust_bc::environment::Environment;
use rust_bc::evaluator::*;
use rust_bc::lexer::Lexer;
use rust_bc::object::Object;
use rust_bc::parser::Parser;
use std::rc::Rc;

#[test]
fn test_eval_program() {
    let results = do_eval("1 + 2 + 3");
    assert_eq!(results.len(), 1);
    let result = results.get(0).unwrap();
    if let Object::Number(n) = result.borrow() {
        assert_eq!(n, &6_f64);
    }

    let results = do_eval("1 + 2 * 3; 6 / 3 * 2");
    assert_eq!(results.len(), 2);
    let result = results.get(0).unwrap();
    if let Object::Number(n) = result.borrow() {
        assert_eq!(n, &7_f64);
    }
    let result = results.get(1).unwrap();
    if let Object::Number(n) = result.borrow() {
        assert_eq!(n, &4_f64);
    }

    let results = do_eval("val = 100; 1 + val");
    assert_eq!(results.len(), 2);
    let result = results.get(0).unwrap();
    assert!(match result.borrow() {
        Object::Empty => true,
        _ => false,
    });
    let result = results.get(1).unwrap();
    if let Object::Number(n) = result.borrow() {
        assert_eq!(n, &101_f64);
    }
}

fn do_eval(input: &str) -> Vec<Rc<Object>> {
    let mut parser = Parser::new(Lexer::new(input.chars().collect()));
    let program = parser.parse_program().unwrap();
    let mut env = Environment::new();
    return eval_program(program, &mut env);
}
