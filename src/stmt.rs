use crate::expr::Expr;
use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Stmt {
    Block {
        statements: Vec<Stmt>,
    },
    Expression {
        expression: Box<Expr>,
    },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
    If {
        condition: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    Print {
        expression: Box<Expr>,
    },
    Var {
        name: Token,
        initializer: Box<Expr>,
    },
    While {
        condition: Box<Expr>,
        body: Box<Stmt>,
    },
}