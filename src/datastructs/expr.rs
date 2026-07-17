use super::token::Token;
use super::literal::Literal;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub enum Expr {
    Assign{
        name: Token,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
    Get {
        object: Box<Expr>,
        name: Token,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Literal,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Set {
        object: Box<Expr>,
        name: Token,
        value: Box<Expr>,
    },
    This {
        keyword: Token,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Ternary {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Box<Expr>,
    },
    Variable {
        name: Token,
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Expr::Assign { name: a1, value: v1 }, Expr::Assign { name: a2, value: v2 }) => a1 == a2 && v1 == v2,
            (Expr::Binary { left: l1, operator: o1, right: r1 }, Expr::Binary { left: l2, operator: o2, right: r2 }) => l1 == l2 && o1 == o2 && r1 == r2,
            (Expr::Call { callee: c1, paren: p1, arguments: a1 }, Expr::Call { callee: c2, paren: p2, arguments: a2 }) => c1 == c2 && p1 == p2 && a1 == a2,
            (Expr::Get { object: o1, name: n1 }, Expr::Get { object: o2, name: n2 }) => o1 == o2 && n1 == n2,
            (Expr::Grouping { expression: e1 }, Expr::Grouping { expression: e2 }) => e1 == e2,
            (Expr::Literal { .. }, Expr::Literal { .. }) => std::mem::discriminant(self) == std::mem::discriminant(other),
            (Expr::Logical { left: l1, operator: o1, right: r1 }, Expr::Logical { left: l2, operator: o2, right: r2 }) => l1 == l2 && o1 == o2 && r1 == r2,
            (Expr::Set { object: o1, name: n1, value: v1 }, Expr::Set { object: o2, name: n2, value: v2 }) => o1 == o2 && n1 == n2 && v1 == v2,
            (Expr::This { keyword: k1 }, Expr::This { keyword: k2 }) => k1 == k2,
            (Expr::Unary { operator: o1, right: r1 }, Expr::Unary { operator: o2, right: r2 }) => o1 == o2 && r1 == r2,
            (Expr::Ternary { condition: c1, then_branch: t1, else_branch: e1 }, Expr::Ternary { condition: c2, then_branch: t2, else_branch: e2 }) => c1 == c2 && t1 == t2 && e1 == e2,
            (Expr::Variable { name: n1 }, Expr::Variable { name: n2 }) => n1 == n2,
            _ => false,
        }
    }
}

impl Eq for Expr {}

impl Hash for Expr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            Expr::Assign { name, value } => { name.hash(state); value.hash(state); }
            Expr::Binary { left, operator, right } => { left.hash(state); operator.hash(state); right.hash(state); }
            Expr::Call { callee, paren, arguments } => { callee.hash(state); paren.hash(state); arguments.hash(state); }
            Expr::Get { object, name } => { object.hash(state); name.hash(state); }
            Expr::Grouping { expression } => { expression.hash(state); }
            Expr::Literal { .. } => {}
            Expr::Logical { left, operator, right } => { left.hash(state); operator.hash(state); right.hash(state); }
            Expr::Set { object, name, value } => { object.hash(state); name.hash(state); value.hash(state); }
            Expr::This { keyword } => { keyword.hash(state); }
            Expr::Unary { operator, right } => { operator.hash(state); right.hash(state); }
            Expr::Ternary { condition, then_branch, else_branch } => { condition.hash(state); then_branch.hash(state); else_branch.hash(state); }
            Expr::Variable { name } => { name.hash(state); }
        }
    }
}