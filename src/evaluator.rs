use crate::errors::RuntimeError;
use crate::expr::Expr;
use crate::token::{Token, TokenType};
use crate::values::Literal;
use crate::stmt::Stmt;
use crate::environment::Environment;

use std::cmp::Ordering;

pub struct Evaluator {
    environment: Environment
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator { environment: Environment::new() }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), RuntimeError> {
        for statement in statements {
            self.execute(statement)?;
        }
        Ok(())
    }

    fn execute(&mut self, stmt: Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Expression { expression } => self.expression_stmt(expression),
            Stmt::Print { expression } => self.print_stmt(expression),
            Stmt::Var { name, initializer } => self.var_stmt(name, initializer),
        }
    }

    fn expression_stmt(&self, expression: Box<Expr>) -> Result<(), RuntimeError> {
        self.evaluate(&expression)?;
        Ok(())
    }

    fn print_stmt(&self, expression: Box<Expr>) -> Result<(), RuntimeError> {
        let value = self.evaluate(&expression)?;
        println!("{}", value);
        Ok(())
    }

    fn var_stmt(&mut self, name: Token, initializer: Box<Expr>) -> Result<(), RuntimeError> {
        let value = self.evaluate(&initializer)?;
        self.environment.define(name.lexeme().to_string(), value);
        Ok(())
    }

    fn evaluate(&self, expr: &Expr) -> Result<Literal, RuntimeError> {
        match expr {
            Expr::Binary { left, operator, right } => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;
                self.evaluate_binary(&left_val, operator, &right_val)
            }
            Expr::Grouping { expression } => self.evaluate(expression),
            Expr::Literal { value } => Ok(value.clone()),
            Expr::Unary { operator, right } => {
                let right_val = self.evaluate(right)?;
                self.evaluate_unary(operator, &right_val)
            }
            Expr::Ternary { condition, then_branch, else_branch } => {
                let condition_val = self.evaluate(condition)?;
                self.evaluate_ternary(&condition_val, then_branch, else_branch)
            }
            Expr::Variable { name } => self.environment.get(name),
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

    /// Evaluates a binary expression based on the operator and operands.
    fn evaluate_binary(&self, left: &Literal, operator: &Token, right: &Literal) -> Result<Literal, RuntimeError> {
        match operator.token_type() {
            TokenType::Plus => self.addition_binary(left, operator, right),
            TokenType::Minus => self.numeric_binary(left, operator, right, |l, r| Literal::Number(l - r)),
            TokenType::Star => self.numeric_binary(left, operator, right, |l, r| Literal::Number(l * r)),
            TokenType::Slash => self.division_binary(left, operator, right),
            TokenType::Greater => self.comparison_binary(left, operator, right, |ord| ord == Ordering::Greater),
            TokenType::GreaterEqual => self.comparison_binary(left, operator, right, |ord| ord != Ordering::Less),
            TokenType::Less => self.comparison_binary(left, operator, right, |ord| ord == Ordering::Less),
            TokenType::LessEqual => self.comparison_binary(left, operator, right, |ord| ord != Ordering::Greater),
            TokenType::EqualEqual => Ok(Literal::Bool(self.is_equal(left, right))),
            TokenType::BangEqual => Ok(Literal::Bool(!self.is_equal(left, right))),
            _ => Err(self.runtime_error(operator, "Unknown binary operator.")),
        }
    }

    /// Handles addition for numbers and string concatenation.
    fn addition_binary(&self, left: &Literal, operator: &Token, right: &Literal) -> Result<Literal, RuntimeError> {
        match (left, right) {
                (Literal::Number(l), Literal::Number(r)) => {
                    Ok(Literal::Number(l + r))
                }
                (Literal::String(l), Literal::String(r)) => {
                    Ok(Literal::String(format!("{}{}", l, r)))
                }
                (Literal::String(l), Literal::Number(r)) => {
                    Ok(Literal::String(format!("{}{}", l, r)))
                }
                (Literal::Number(l), Literal::String(r)) => {
                    Ok(Literal::String(format!("{}{}", l, r)))
                }
                _ => Err(self.runtime_error(operator, "Operands must be two numbers or atleast one string.")),
        }
    }

    /// Handles numeric binary operations like subtraction, multiplication, and division.
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

    /// Handles division and checks for division by zero.
    fn division_binary(&self, left: &Literal, operator: &Token, right: &Literal) -> Result<Literal, RuntimeError> {
        match (left, right) {
            (Literal::Number(l), Literal::Number(r)) if *r == 0.0 => {
                Err(self.runtime_error(operator, "Division by zero."))
            }
            _ => self.numeric_binary(left, operator, right, |l, r| Literal::Number(l / r)),
        }
    }

    /// Uses the std::cmp::Ordering to compare two literals and applies the provided comparison function.
    fn comparison_binary<F>(&self, left: &Literal, operator: &Token, right: &Literal, combine: F) -> Result<Literal, RuntimeError>
    where
        F: Fn(Ordering) -> bool,
    {
        let ordering = match (left, right) {
            (Literal::Number(l), Literal::Number(r)) => l.partial_cmp(r),
            (Literal::String(l), Literal::String(r)) => l.partial_cmp(r),
            _ => None,
        };

        match ordering {
            Some(ord) => Ok(Literal::Bool(combine(ord))),
            None => Err(self.runtime_error(operator, "Operands must be two numbers or two strings.")),
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
