use std::fmt;
use crate::{
    callable::Callable,
    error::InterpreterError,
    interpreter::Interpreter,
    value::Value
};

#[derive(Clone, Debug, PartialEq)]
pub struct Klass {
    pub name: String,
    pub arity: usize
}

impl Klass {
    pub fn new(name: String) -> Self {
        return Self { name, arity: 0 };
    }
}

impl Callable for Klass {
    fn call(&self, _: &mut Interpreter, _: Vec<Value>) -> Result<Value, InterpreterError> {
        return Ok(Value::Instance(Instance::new(self.clone())));
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Instance {
    class: Klass
}

impl Instance {
    pub fn new(class: Klass) -> Self {
        return Self { class };
    }
}

impl fmt::Display for Klass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "Klass({}({}))", self.name, self.arity);
    }
}

impl fmt::Display for Instance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "Instance({})", self.class.name);
    }
}