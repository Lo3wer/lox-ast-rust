use std::fmt;
use crate::datastructs::values::{Callable, Literal};
use crate::datastructs::exceptions::RuntimeException;
use crate::evaluator::Evaluator;

pub struct Class {
    pub name: String,
}

impl Class {
    pub fn new(name: String) -> Self {
        Class { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<class {}>", self.name)
    }
}

impl Callable for Class {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self, _evaluator: &mut Evaluator, _arguments: &[Literal]) -> Result<Literal, RuntimeException> {
        Ok(Literal::Nil)
    }
}