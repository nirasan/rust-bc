use std::rc::Rc;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Program {
    pub statements: Vec<Rc<Statement>>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Statement {
    ExpressionStatement { expression: Rc<Expression> },
    AssignStatement { name: String, expression: Rc<Expression> },
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Expression {
    NumberLiteral(f64),
    Identifier(String),
    InfixExpression {
        left: Rc<Expression>,
        operator: String,
        right: Rc<Expression>,
    },
}
