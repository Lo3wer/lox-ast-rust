use super::token::Token;
use super::literal::Literal;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub enum Expr {
    Assign{
        name: Token,
        value: Box<Expr>,
        id: usize,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
        id: usize,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
        id: usize,
    },
    Get {
        object: Box<Expr>,
        name: Token,
        id: usize,
    },
    Grouping {
        expression: Box<Expr>,
        id: usize,
    },
    Literal {
        value: Literal,
        id: usize,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
        id: usize,
    },
    Set {
        object: Box<Expr>,
        name: Token,
        value: Box<Expr>,
        id: usize,
    },
    Super {
        keyword: Token,
        method: Token,
        id: usize,
    },
    This {
        keyword: Token,
        id: usize,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
        id: usize,
    },
    Ternary {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Box<Expr>,
        id: usize,
    },
    Variable {
        name: Token,
        id: usize,
    }
}

impl Expr {
    pub fn id(&self) -> usize {
        match self {
            Expr::Assign { id, .. }
            | Expr::Binary { id, .. }
            | Expr::Call { id, .. }
            | Expr::Get { id, .. }
            | Expr::Grouping { id, .. }
            | Expr::Literal { id, .. }
            | Expr::Logical { id, .. }
            | Expr::Set { id, .. }
            | Expr::Super { id, .. }
            | Expr::This { id, .. }
            | Expr::Unary { id, .. }
            | Expr::Ternary { id, .. }
            | Expr::Variable { id, .. } => *id,
        }
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for Expr {}

impl Hash for Expr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}