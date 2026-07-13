use std::fmt;
use std::rc::Rc;
use crate::evaluator::Evaluator;
use crate::errors::RuntimeError;
use crate::stmt::Stmt;
use crate::token::Token;
use crate::environment::Environment;

#[derive(Clone)]
pub enum Literal {
    Bool(bool),
    String(String),
    Number(f64),
    Callable(Rc<dyn Callable>),
    Nil,
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Literal::Bool(a), Literal::Bool(b)) => a == b,
            (Literal::String(a), Literal::String(b)) => a == b,
            (Literal::Number(a), Literal::Number(b)) => a == b,
            (Literal::Nil, Literal::Nil) => true,
            (Literal::Callable(_), Literal::Callable(_)) => false,
            _ => false,
        }
    }
}

impl fmt::Debug for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Bool(v) => write!(f, "{:?}", v),
            Literal::String(v) => write!(f, "{:?}", v),
            Literal::Number(v) => write!(f, "{:?}", v),
            Literal::Callable(_) => write!(f, "<function>"),
            Literal::Nil => write!(f, "Nil"),
        }
    }
}

pub trait Callable {
    fn arity(&self) -> usize;
    fn call(&self, evaluator: &mut Evaluator, arguments: &[Literal]) -> Result<Literal, RuntimeError>;
}

pub struct FunctionCallable{
    params: Vec<Token>,
    body: Vec<Stmt>,
}

impl FunctionCallable {
    pub fn new(params: Vec<Token>, body: Vec<Stmt>) -> Self {
        FunctionCallable { params, body }
    }
}

impl Callable for FunctionCallable {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(&self, evaluator: &mut Evaluator, arguments: &[Literal]) -> Result<Literal, RuntimeError> {
        let function_env = Environment::new_enclosed(evaluator.globals());
        for (param, arg) in self.params.iter().zip(arguments.iter()) {
            function_env.borrow_mut().define(param, arg.clone());
        }
        evaluator.execute_block(&self.body, function_env)?;
        Ok(Literal::Nil)
    }
}


impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Bool(value) => write!(f, "{}", value),
            Literal::String(value) => write!(f, "{}", value),
            Literal::Number(value) => {
                if value.fract() == 0.0 {
                    write!(f, "{:.0}", value)
                } else {
                    write!(f, "{}", value)
                }
            }
            Literal::Nil => write!(f, "nil"),
            Literal::Callable(_) => write!(f, "<function>"),
        }
    }
}
