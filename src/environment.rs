use crate::values::Literal;
use crate::errors::RuntimeError;
use crate::token::Token;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub type EnvRef = Rc<RefCell<Environment>>;

pub struct Environment {
    enclosing: Option<EnvRef>,
    values: HashMap<String, Literal>,
}

impl Environment {
    // global constructor
    pub fn new() -> EnvRef {
        Rc::new(RefCell::new(Environment {
            enclosing: None,
            values: HashMap::new(),
        }))
    }

    // local constructor
    pub fn new_enclosed(enclosing: EnvRef) -> EnvRef {
        Rc::new(RefCell::new(Environment {
            enclosing: Some(enclosing),
            values: HashMap::new(),
        }))
    }

    pub fn get(&self, name: &Token) -> Result<Literal, RuntimeError> {
        if let Some(value) = self.values.get(name.lexeme()) {
            return Ok(value.clone());
        } else if let Some(parent) = &self.enclosing {
            return parent.borrow().get(name);
        }
        Err(RuntimeError {
            token: name.clone(),
            message: format!("Undefined variable '{}'.", name.lexeme()),
        })
    }

    pub fn define(&mut self, name: &Token, value: Literal) {
        self.values.insert(name.lexeme().to_string(), value);
    }

    pub fn define_str(&mut self, name: &str, value: Literal) {
        self.values.insert(name.to_string(), value);
    }

    pub fn assign(&mut self, name: &Token, value: Literal) -> Result<(), RuntimeError> {
        if self.values.contains_key(name.lexeme()) {
            self.values.insert(name.lexeme().to_string(), value);
            Ok(())
        } else if let Some(parent) = &self.enclosing {
            parent.borrow_mut().assign(name, value)
        } else {
            Err(RuntimeError {
                token: name.clone(),
                message: format!("Undefined variable '{}'.", name.lexeme()),
            })
        }
    }
}