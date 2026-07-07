use crate::values::Literal;
use crate::errors::RuntimeError;
use crate::token::Token;
use std::collections::HashMap;

pub struct Environment {
    values: HashMap<String, Literal>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn get(&self, name: &Token) -> Result<Literal, RuntimeError> {
        self.values
            .get(name.lexeme())
            .cloned()
            .ok_or_else(|| RuntimeError {
                token: name.clone(),
                message: "Undefined variable.".to_string(),
            })
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }
}