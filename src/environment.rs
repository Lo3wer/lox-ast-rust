use crate::datastructs::literal::Literal;
use crate::datastructs::exceptions::RuntimeException;
use crate::datastructs::token::Token;
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

    pub fn get(&self, name: &Token) -> Result<Literal, RuntimeException> {
        if let Some(value) = self.values.get(name.lexeme()) {
            return Ok(value.clone());
        } else if let Some(parent) = &self.enclosing {
            return parent.borrow().get(name);
        }
        Err(RuntimeException::Error {
            token: name.clone(),
            message: format!("Undefined variable '{}'.", name.lexeme()),
        })
    }

    pub fn get_at(&self, distance: usize, name: &Token) -> Result<Literal, RuntimeException> {
        if distance == 0 {
            self.get(name)
        } else if let Some(parent) = &self.enclosing {
            parent.borrow().get_at(distance - 1, name)
        } else {
            Err(RuntimeException::Error {
                token: name.clone(),
                message: format!("Undefined variable '{}'.", name.lexeme()),
            })
        }
    }

    pub fn define(&mut self, name: &Token, value: Literal) {
        self.values.insert(name.lexeme().to_string(), value);
    }

    pub fn assign(&mut self, name: &Token, value: Literal) -> Result<(), RuntimeException> {
        if self.values.contains_key(name.lexeme()) {
            self.values.insert(name.lexeme().to_string(), value);
            Ok(())
        } else if let Some(parent) = &self.enclosing {
            parent.borrow_mut().assign(name, value)
        } else {
            Err(RuntimeException::Error {
                token: name.clone(),
                message: format!("Undefined variable '{}'.", name.lexeme()),
            })
        }
    }

    pub fn assign_at(&mut self, distance: usize, name: &Token, value: Literal) -> Result<(), RuntimeException> {
        if distance == 0 {
            self.assign(name, value)
        } else if let Some(parent) = &self.enclosing {
            parent.borrow_mut().assign_at(distance - 1, name, value)
        } else {
            Err(RuntimeException::Error {
                token: name.clone(),
                message: format!("Undefined variable '{}'.", name.lexeme()),
            })
        }
    }
}