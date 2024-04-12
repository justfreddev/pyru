use std::{collections::HashMap, fmt};
use crate::{
    callable::{Callable, Func},
    error::InterpreterError,
    interpreter::Interpreter,
    token::Token,
    value::Value,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Klass {
    pub name: String,
    pub arity: usize,
    methods: HashMap<String, Func>
}

impl Klass {
    pub fn new(name: String, methods: HashMap<String, Func>) -> Self {
        return Self { name, arity: 0, methods };
    }

    pub fn find_method(&self, name: String) -> Result<Func, InterpreterError> {
        if self.methods.contains_key(&name) {
            return Ok(self.methods.get(&name).unwrap().clone());
        }

        return Err(InterpreterError::UndefinedProperty { name });
    }
}

impl Callable for Klass {
    fn call(&self, _: &mut Interpreter, _: Vec<Value>) -> Result<Value, InterpreterError> {
        return Ok(Value::Instance(Instance::new(self.clone())));
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Instance {
    class: Klass,
    fields: HashMap<String, Value>,
}

impl Instance {
    pub fn new(class: Klass) -> Self {
        return Self { class, fields: HashMap::<String, Value>::new() };
    }

    pub fn get(&self, name: Token) -> Result<Value, InterpreterError> {
        if self.fields.contains_key(&name.lexeme.clone()) {
            match self.fields.get(&name.lexeme.clone()) {
                Some(v) => return Ok(v.clone()),
                None => {}
            }
        }

        let method = self.class.find_method(name.lexeme)?;
        return Ok(Value::Function(method));
    }

    pub fn set(&mut self, name: Token, value: Value) -> Result<Value, InterpreterError> {
        match self.fields.insert(name.lexeme.clone(), value) {
            Some(v) => Ok(v),
            None => Err(InterpreterError::UndefinedField { name: name.lexeme.clone() })
        }
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