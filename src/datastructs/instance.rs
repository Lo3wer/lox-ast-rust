use std::fmt;
use std::collections::HashMap;

use crate::datastructs::class::Class;
use crate::datastructs::values::Literal;
use crate::datastructs::exceptions::RuntimeException;
use crate::datastructs::token::Token;

pub struct Instance {
    class: Class,
    fields: HashMap<String, Literal>,
}

impl Instance {
    pub fn new(class: Class) -> Self {
        Instance { class, fields: HashMap::new() }
    }

    pub fn get(&self, name: &Token) -> Result<Literal, RuntimeException> {
        if let Some(value) = self.fields.get(name.lexeme()) {
            return Ok(value.clone());
        }
        Err(RuntimeException::Error {
            token: name.clone(),
            message: format!("Undefined property '{}'.", name.lexeme()),
        })
    }

    pub fn set(&mut self, name: &Token, value: Literal) {
        self.fields.insert(name.lexeme().to_string(), value);
    }
}

impl fmt::Display for Instance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} instance", self.class.name())
    }
}