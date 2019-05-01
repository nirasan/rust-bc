use crate::ast::*;
use crate::environment::Environment;
use crate::object::Object;

use core::borrow::Borrow;
use std::rc::Rc;

pub fn eval_program(program: Program, env: &mut Environment) -> Vec<Rc<Object>> {
    let mut results = vec![];
    for statement in program.statements {
        let result = eval_statement(statement, env);
        if let Some(result) = result {
            results.push(result);
        }
    }
    return results;
}

pub fn eval_statement(statement: Rc<Statement>, env: &mut Environment) -> Option<Rc<Object>> {
    match statement.borrow() {
        Statement::ExpressionStatement { expression } => eval_expression(expression, env),
        Statement::AssignStatement { name, expression } => {
            eval_assign_statement(name.to_owned(), expression, env)
        }
    }
}

pub fn eval_assign_statement(
    name: String,
    expression: &Rc<Expression>,
    env: &mut Environment,
) -> Option<Rc<Object>> {
    let value = eval_expression(expression, env)?;
    env.set(name, value);
    return Some(Rc::new(Object::Empty));
}

pub fn eval_expression(expression: &Rc<Expression>, env: &mut Environment) -> Option<Rc<Object>> {
    match expression.borrow() {
        Expression::NumberLiteral(n) => Some(Rc::new(Object::Number(*n))),
        Expression::Identifier(s) => eval_identifier(s, env),
        Expression::InfixExpression {
            left,
            operator,
            right,
        } => eval_infix_expression(left, operator, right, env),
        _ => None,
    }
}

pub fn eval_infix_expression(
    left: &Rc<Expression>,
    operator: &str,
    right: &Rc<Expression>,
    env: &mut Environment,
) -> Option<Rc<Object>> {
    let left = eval_expression(left, env)?;
    let right = eval_expression(right, env)?;

    if let &Object::Number(left) = left.borrow() {
        if let &Object::Number(right) = right.borrow() {
            return match operator {
                "+" => Some(Rc::new(Object::Number(left + right))),
                "-" => Some(Rc::new(Object::Number(left - right))),
                "*" => Some(Rc::new(Object::Number(left * right))),
                "/" => Some(Rc::new(Object::Number(left / right))),
                _ => None,
            };
        }
    }

    return None;
}

pub fn eval_identifier(name: &String, env: &mut Environment) -> Option<Rc<Object>> {
    env.get(name)
}
