use std::fmt;
use std::rc::Rc;
use std::collections::HashMap;
use std::cell::RefCell;
use crate::datastructs::values::{Callable, Literal};
use crate::datastructs::exceptions::RuntimeException;
use crate::evaluator::Evaluator;
use crate::datastructs::instance::Instance;

pub struct Class {
    pub name: String,
    methods: HashMap<String, Rc<dyn Callable>>,
}

impl Class {
    pub fn new(name: String, methods: HashMap<String, Rc<dyn Callable>>) -> Self {
        Class { name, methods }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn find_method(&self, name: &str) -> Option<Rc<dyn Callable>> {
        self.methods.get(name).cloned()
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
        Ok(Literal::Instance(Rc::new(RefCell::new(Instance::new(self.name.clone(), self.methods.clone())))))
    }
}