use crate::expr::Expr;
use crate::values::Literal;
use crate::token::{Token, TokenType};

pub struct Evaluator {}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {}
    }

    pub fn interpret(&self, expr: &Expr) -> Option<Literal> {
        self.evaluate(expr)
    }
    
    fn evaluate(&self, expr: &Expr) -> Option<Literal> {
        match expr {
            Expr::Binary { left, operator, right } => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;
                self.evaluate_binary(&left_val, operator, &right_val)
            }
            Expr::Grouping { expression } => self.evaluate(expression),
            Expr::Literal { value } => value.clone(),
            Expr::Unary { operator, right } => {
                let right_val = self.evaluate(right)?;
                self.evaluate_unary(operator, &right_val)
            }
            Expr::Ternary { condition, then_branch, else_branch } => {
                let condition_val = self.evaluate(condition)?;
                if let Literal::Bool(true) = condition_val {
                    self.evaluate(then_branch)
                } else {
                    self.evaluate(else_branch)
                }
            }
        }
    }

    fn evaluate_unary(&self, operator: &Token, right: &Literal) -> Option<Literal> {
        match operator.token_type() {
            TokenType::Minus => {
                if let Literal::Number(n) = right {
                    Some(Literal::Number(-n))
                } else {
                    None
                }
            }
            TokenType::Bang => {
                Some(Literal::Bool(!self.is_truthy(right)))
            }
            _ => None,
        }
    }

    fn evaluate_binary(&self, left: &Literal, operator: &Token, right: &Literal) -> Option<Literal> {
        match operator.token_type() {
            // Arithmetic operations
            TokenType::Plus => {
                if let (Literal::Number(l), Literal::Number(r)) = (left, right) {
                    Some(Literal::Number(l + r))
                } else if let (Literal::String(l), Literal::String(r)) = (left, right) {
                    Some(Literal::String(format!("{}{}", l, r)))
                } else {
                    None
                }
            }
            TokenType::Minus => {
                if let (Literal::Number(l), Literal::Number(r)) = (left, right) {
                    Some(Literal::Number(l - r))
                } else {
                    None
                }
            }
            TokenType::Star => {
                if let (Literal::Number(l), Literal::Number(r)) = (left, right) {
                    Some(Literal::Number(l * r))
                } else {
                    None
                }
            }
            TokenType::Slash => {
                if let (Literal::Number(l), Literal::Number(r)) = (left, right) {
                    Some(Literal::Number(l / r))
                } else {
                    None
                }
            }
            // Comparison operations
            TokenType::Greater => {
                if let (Literal::Number(l), Literal::Number(r)) = (left, right) {
                    Some(Literal::Bool(l > r))
                } else {
                    None
                }
            }
            TokenType::Less => {
                if let (Literal::Number(l), Literal::Number(r)) = (left, right) {
                    Some(Literal::Bool(l < r))
                } else {
                    None
                }
            }
            TokenType::GreaterEqual => {
                if let (Literal::Number(l), Literal::Number(r)) = (left, right) {
                    Some(Literal::Bool(l >= r))
                } else {
                    None
                }
            }
            TokenType::LessEqual => {
                if let (Literal::Number(l), Literal::Number(r)) = (left, right) {
                    Some(Literal::Bool(l <= r))
                } else {
                    None
                }
            }
            TokenType::EqualEqual => {
                Some(Literal::Bool(self.is_equal(left, right)))
            }
            TokenType::BangEqual => {
                Some(Literal::Bool(!self.is_equal(left, right)))
            }
            _ => None,
        }
    }
    
    fn evaluate_ternary(&self, condition: &Literal, then_branch: &Expr, else_branch: &Expr) -> Option<Literal> {
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