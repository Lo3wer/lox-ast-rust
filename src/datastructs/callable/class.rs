use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::datastructs::literal::Literal;
use crate::datastructs::callable::Callable;
use crate::datastructs::instance::Instance;
use crate::datastructs::exceptions::RuntimeException;
use crate::evaluator::Evaluator;

pub struct Class {
    pub name: String,
    methods: HashMap<String, Rc<dyn Callable>>,
}

impl Class {
    pub fn new(name: String, methods: HashMap<String, Rc<dyn Callable>>) -> Self {
        Class { name, methods }
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
    fn bind(&self, _instance: Rc<RefCell<Instance>>) -> Rc<dyn Callable> {
        Rc::new(Class { name: self.name.clone(), methods: self.methods.clone() })
    }

    fn arity(&self) -> usize {
        if let Some(initializer) = self.find_method("init") {
            initializer.arity()
        } else {
            0
        }
    }

    fn call(&self, evaluator: &mut Evaluator, arguments: &[Literal]) -> Result<Literal, RuntimeException> {
        let instance = Rc::new(RefCell::new(Instance::new(self.name.clone(), self.methods.clone())));
        let initializer = self.find_method("init");
        if let Some(initializer) = initializer {
            initializer.bind(instance.clone()).call(evaluator, arguments)?;
        }
        Ok(Literal::Instance(instance.clone()))
    }
}