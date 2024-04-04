use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{self, Debug},
    rc::Rc, time::{SystemTime, UNIX_EPOCH}
};

use crate::{
    callable::NativeFunc,
    error::InterpreterError,
    token::Token,
    value::{LiteralType, Value}
};


pub trait Environment: Debug {
    fn define(&mut self, name: String, value: Value);
    fn get(&self, name: Token) -> Result<Value, InterpreterError>;
    fn assign(&mut self, name: Token, value: Value) -> Result<Value, InterpreterError>;
}

#[derive(Debug)]
pub struct GlobalEnvironment {
    values: HashMap<String, Rc<RefCell<Value>>>
}

impl GlobalEnvironment {
    pub fn new() -> Self {
        let clock = NativeFunc::new(
            "clock".to_string(),
            0,
            |_, _| {
                Ok(Value::Literal(LiteralType::Num(
                    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64()
                )))
            }
        );
        let mut values: HashMap<String, Rc<RefCell<Value>>> = HashMap::new();
        values.insert("clock".to_string(), Rc::new(RefCell::new(Value::NativeFunction(clock))));
        Self {
            values
        }
    }
}

impl Environment for GlobalEnvironment {
    fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, Rc::new(RefCell::new(value)));
    }

    fn get(&self, name: Token) -> Result<Value, InterpreterError> {
        match self.values.get(&name.lexeme) {
            Some(v) => return Ok(v.borrow().clone()),
            _ => {
                return Err(
                    InterpreterError::UndefinedVariable{
                        name: name.lexeme,
                        start: name.start,
                        end: name.end, line:
                        name.line
                    }
                )
            }
        }
    }

    fn assign(&mut self, name: Token, value: Value) -> Result<Value, InterpreterError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme, Rc::new(RefCell::new(value.clone())));
            return Ok(value);
        }

        Err(
            InterpreterError::UndefinedVariable{
                name: name.lexeme,
                start: name.start,
                end: name.end,
                line: name.line
            }
        )
    }
}

#[derive(Clone, Debug)]
pub struct LocalEnvironment {
    values: HashMap<String, Rc<RefCell<Value>>>,
    enclosing: Option<Rc<RefCell<dyn Environment>>>
}


impl LocalEnvironment {
    pub fn new(enclosing: Option<Rc<RefCell<dyn Environment>>>) -> Self {
        Self { values: HashMap::new(), enclosing }
    }
}

impl Environment for LocalEnvironment {
    fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, Rc::new(RefCell::new(value)));
    }

    fn get(&self, name: Token) -> Result<Value, InterpreterError> {
        match self.values.get(&name.lexeme) {
            Some(v) => return Ok(v.borrow().clone()),
            None => {
                if let Some(enclosing) = &self.enclosing {
                    return enclosing.borrow().get(name)
                }
            }
        }
        return Err(
            InterpreterError::UndefinedVariable{
                name: name.lexeme,
                start: name.start,
                end: name.end,
                line: name.line
            }
        )
    }

    fn assign(&mut self, name: Token, value: Value) -> Result<Value, InterpreterError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme, Rc::new(RefCell::new(value.clone())));
            return Ok(value);
        }

        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow_mut().assign(name.clone(), value);
        }
        Err(
            InterpreterError::UndefinedVariable{
                name: name.lexeme,
                start: name.start,
                end: name.end,
                line: name.line
            }
        )
    }
}

impl fmt::Display for LocalEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Local(values: {:#?})", self.values)
    }
}

impl fmt::Display for GlobalEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Global(values: {:#?})", self.values)
    }
}