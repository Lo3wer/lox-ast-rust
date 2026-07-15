use std::fmt;
use crate::datastructs::class::Class;

pub struct Instance {
    pub class: Class,
}

impl Instance {
    pub fn new(class: Class) -> Self {
        Instance { class }
    }
}

impl fmt::Display for Instance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} instance", self.class.name())
    }
}