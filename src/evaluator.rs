use crate::errors::RuntimeError;
use crate::expr::Expr;
use crate::token::{Token, TokenType};
use crate::values::Literal;

pub struct Evaluator {}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {}
    }

    pub fn interpret(&self, expr: &Expr) -> Result<Literal, RuntimeError> {
        self.evaluate(expr)
    }

    fn evaluate(&self, expr: &Expr) -> Result<Literal, RuntimeError> {
        match expr {
            Expr::Binary { left, operator, right } => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;
                self.evaluate_binary(&left_val, operator, &right_val)
            }
            Expr::Grouping { expression } => self.evaluate(expression),
            Expr::Literal { value } => Ok(value.clone().unwrap_or(Literal::Nil)),
            Expr::Unary { operator, right } => {
                let right_val = self.evaluate(right)?;
                self.evaluate_unary(operator, &right_val)
            }
            Expr::Ternary { condition, then_branch, else_branch } => {
                let condition_val = self.evaluate(condition)?;
                self.evaluate_ternary(&condition_val, then_branch, else_branch)
            }
        }
    }

    fn evaluate_unary(&self, operator: &Token, right: &Literal) -> Result<Literal, RuntimeError> {
        match operator.token_type() {
            TokenType::Minus => match right {
                Literal::Number(n) => Ok(Literal::Number(-n)),
                _ => Err(self.runtime_error(operator, "Operand must be a number.")),
            },
            TokenType::Bang => Ok(Literal::Bool(!self.is_truthy(right))),
            _ => Err(self.runtime_error(operator, "Unknown unary operator.")),
        }
    }

    fn evaluate_binary(&self, left: &Literal, operator: &Token, right: &Literal) -> Result<Literal, RuntimeError> {
        match operator.token_type() {
            TokenType::Plus => match (left, right) {
                (Literal::Number(l), Literal::Number(r)) => {
                    Ok(Literal::Number(l + r))
                }
                (Literal::String(l), Literal::String(r)) => {
                    Ok(Literal::String(format!("{}{}", l, r)))
                }
                _ => Err(self.runtime_error(operator, "Operands must be two numbers or two strings.")),
            },
            TokenType::Minus => self.numeric_binary(left, operator, right, |l, r| {
                Literal::Number(l - r)
            }),
            TokenType::Star => self.numeric_binary(left, operator, right, |l, r| {
                Literal::Number(l * r)
            }),
            TokenType::Slash => self.numeric_binary(left, operator, right, |l, r| {
                Literal::Number(l / r)
            }),
            TokenType::Greater => self.comparison_binary(left, operator, right, |l, r| {
                Literal::Bool(l > r)
            }),
            TokenType::GreaterEqual => self.comparison_binary(left, operator, right, |l, r| {
                Literal::Bool(l >= r)
            }),
            TokenType::Less => self.comparison_binary(left, operator, right, |l, r| {
                Literal::Bool(l < r)
            }),
            TokenType::LessEqual => self.comparison_binary(left, operator, right, |l, r| {
                Literal::Bool(l <= r)
            }),
            TokenType::EqualEqual => Ok(Literal::Bool(self.is_equal(left, right))),
            TokenType::BangEqual => Ok(Literal::Bool(!self.is_equal(left, right))),
            _ => Err(self.runtime_error(operator, "Unknown binary operator.")),
        }
    }

    fn numeric_binary<F>(&self, left: &Literal, operator: &Token, right: &Literal, combine: F) -> Result<Literal, RuntimeError>
    where
        F: Fn(f64, f64) -> Literal,
    {
        match (left, right) {
            (Literal::Number(l), Literal::Number(r)) => {
                Ok(combine(*l, *r))
            }
            _ => Err(self.runtime_error(operator, "Operands must be numbers.")),
        }
    }

    fn comparison_binary<F>(&self, left: &Literal, operator: &Token, right: &Literal, combine: F) -> Result<Literal, RuntimeError>
    where
        F: Fn(f64, f64) -> Literal,
    {
        match (left, right) {
            (Literal::Number(l), Literal::Number(r)) => {
                Ok(combine(*l, *r))
            }
            _ => Err(self.runtime_error(operator, "Operands must be numbers.")),
        }
    }

    fn runtime_error(&self, token: &Token, message: &str) -> RuntimeError {
        RuntimeError {
            token: token.clone(),
            message: message.to_string(),
        }
    }

    fn evaluate_ternary(&self, condition: &Literal, then_branch: &Expr, else_branch: &Expr) -> Result<Literal, RuntimeError> {
        if let Literal::Bool(true) = condition {
            self.evaluate(then_branch)
        } else {
            self.evaluate(else_branch)
        }
    }

    fn is_truthy(&self, literal: &Literal) -> bool {
        match literal {
            Literal::Bool(b) => *b,
            Literal::Nil => false,
            _ => true,
        }
    }

    fn is_equal(&self, a: &Literal, b: &Literal) -> bool {
        match (a, b) {
            (Literal::Number(l), Literal::Number(r)) => l == r,
            (Literal::String(l), Literal::String(r)) => l == r,
            (Literal::Bool(l), Literal::Bool(r)) => l == r,
            (Literal::Nil, Literal::Nil) => true,
            _ => false,
        }
    }
}
