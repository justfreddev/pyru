use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{self, Debug},
    rc::Rc,
};

use crate::{
    error::InterpreterError,
    interpreter::Env,
    token::Token,
    value::Value,
};

#[derive(Debug)]
pub struct Environment {
    values: HashMap<String, Rc<RefCell<Value>>>,
    enclosing: Option<Env>
}

impl Environment {
    pub fn new(enclosing: Option<Env>) -> Self {
        return Self {
            values: HashMap::new(),
            enclosing,
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, Rc::new(RefCell::new(value)));
    }

    pub fn get(&self, name: Token) -> Result<Value, InterpreterError> {
        return match self.values.get(&name.lexeme) {
            Some(v) => Ok(v.borrow().clone()),
            None => {
                if let Some(enclosing) = &self.enclosing {
                    return enclosing.borrow().get(name);
                } else {
                    return Err(InterpreterError::UndefinedVariable {
                        name: name.lexeme,
                        start: name.start,
                        end: name.end,
                        line: name.line,
                    });
                }
            }
        }
    }

    pub fn assign(&mut self, name: Token, value: Value) -> Result<Value, InterpreterError> {
        if self.values.contains_key(&name.lexeme) {
            self.values
                .insert(name.lexeme, Rc::new(RefCell::new(value.clone())));
            return Ok(value);
        }

        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow_mut().assign(name.clone(), value);
        }

        return Err(InterpreterError::UndefinedVariable {
            name: name.lexeme,
            start: name.start,
            end: name.end,
            line: name.line
        });
    }
}


impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "Environment(values: {:#?}, enclosing: {})", self.values, self.enclosing.is_some());
    }
}
