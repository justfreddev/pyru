use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{self, Debug},
    rc::Rc,
};

use crate::{
    error::EvaluatorError,
    evaluator::Env,
    token::Token,
    value::Value,
};

/// The `Environment` struct represents a scope in which variables are defined and stored.
/// It supports nested scopes by maintaining a reference to an enclosing environment.
/// 
/// ## Fields
/// - `values`: A `HashMap` that stores variable names and their corresponding values.
/// - `enclosing`: An optional reference to an enclosing environment, allowing for nested scopes.
#[derive(Debug)]
pub struct Environment {
    values: HashMap<String, Rc<RefCell<Value>>>,
    enclosing: Option<Env> // Composition
}

impl Environment {
    /// Creates a new `Environment` instance.
    pub fn new(enclosing: Option<Env>) -> Self {
        return Self {
            values: HashMap::new(),
            enclosing,
        }
    }

    /// Defines a new variable in the current environment.
    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, Rc::new(RefCell::new(value)));
    }

    /// Retrieves the value of a variable from the current or enclosing environments.
    pub fn get(&self, name: &Token) -> Result<Value, EvaluatorError> {
        return match self.values.get(&name.lexeme) {
            Some(v) => Ok(v.borrow().clone()),
            None => {
                if let Some(enclosing) = &self.enclosing {
                    return enclosing.borrow().get(name);
                } else {
                    return Err(EvaluatorError::UndefinedVariable {
                        name: name.lexeme.clone(),
                        start: name.start,
                        end: name.end,
                        line: name.line,
                    });
                }
            }
        }
    }

    /// Assigns a new value to an existing variable in the current or enclosing environments.
    pub fn assign(&mut self, name: &Token, value: Value) -> Result<Value, EvaluatorError> {
        if self.values.contains_key(&name.lexeme) {
            self.values
                .insert(name.lexeme.clone(), Rc::new(RefCell::new(value.clone())));
            return Ok(value);
        }

        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow_mut().assign(name, value);
        }

        return Err(EvaluatorError::UndefinedVariable {
            name: name.lexeme.clone(),
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
