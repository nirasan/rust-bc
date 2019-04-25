use crate::ast::*;
use crate::object::Object;

use std::rc::Rc;
use core::borrow::Borrow;

pub fn eval_program(program: Program) -> Vec<Rc<Object>> {
    let mut results = vec![];
    for statement in program.statements {
        let result = eval_statement(statement);
        if let Some(result) = result {
            results.push(result);
        }
    }
    return results;
}

pub fn eval_statement(statement: Rc<Statement>) -> Option<Rc<Object>> {
    match statement.borrow() {
        Statement::ExpressionStatement {expression} => eval_expression(expression),
        _ => None,
    }
}

pub fn eval_expression(expression: &Rc<Expression>) -> Option<Rc<Object>> {
    match expression.borrow() {
        Expression::NumberLiteral(n) => Some(Rc::new(Object::Number(*n))),
        Expression::InfixExpression {left, operator, right} => eval_infix_expression(left, operator, right),
        _ => None,
    }
}

pub fn eval_infix_expression(left: &Rc<Expression>, operator: &str, right: &Rc<Expression>) -> Option<Rc<Object>> {
    let left = eval_expression(left)?;
    let right = eval_expression(right)?;

    if let &Object::Number(left) = left.borrow() {
        if let &Object::Number(right) = right.borrow() {
            return match operator {
                "+" => Some(Rc::new(Object::Number(left + right))),
                "-" => Some(Rc::new(Object::Number(left - right))),
                "*" => Some(Rc::new(Object::Number(left * right))),
                "/" => Some(Rc::new(Object::Number(left / right))),
                _ => None,
            }
        }
    }

    return None;
}

