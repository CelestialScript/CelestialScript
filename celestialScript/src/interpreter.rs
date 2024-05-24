use crate::parser::{Expr, Statement, BinaryOperator};
use std::collections::HashMap;

pub struct Interpreter {
    variables: HashMap<String, i64>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Statement>) {
        for statement in statements {
            self.execute_statement(statement);
        }
    }

    fn execute_statement(&mut self, statement: Statement) {
        match statement {
            Statement::Let(name, expr) => {
                let value = self.evaluate_expression(expr);
                self.variables.insert(name, value);
            }
            Statement::Print(expr) => {
                let value = self.evaluate_expression(expr);
                println!("{}", value);
            }
        }
    }

    fn evaluate_expression(&mut self, expr: Expr) -> i64 {
        match expr {
            Expr::Number(n) => n,
            Expr::Identifier(name) => {
                *self.variables.get(&name).expect("Undefined variable")
            }
            Expr::BinaryOp(left, op, right) => {
                let left_value = self.evaluate_expression(*left);
                let right_value = self.evaluate_expression(*right);
                match op {
                    BinaryOperator::Plus => left_value + right_value,
                }
            }
        }
    }
}
