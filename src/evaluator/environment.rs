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
    values: HashMap<String, Rc<RefCell<Value>>>, // HASH TABLE - BAND A
    enclosing: Option<Env> // COMPOSITION - BAND A
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
        // Insert the variable name and its value into the hash map.
        self.values.insert(name, Rc::new(RefCell::new(value)));
    }

    /// Retrieves the value of a variable from the current or enclosing environments.
    pub fn get(&self, name: &Token) -> Result<Value, EvaluatorError> {
        // Check if the variable exists in the current environment.
        return match self.values.get(&name.lexeme) {
            // If it does, return its value.
            Some(v) => Ok(v.borrow().clone()),
            // If not, check if the enclosing environment is present.
            None => {
                if let Some(enclosing) = &self.enclosing {
                    // RECURSIVE FUNCTION CALL - BAND A
                    return enclosing.borrow().get(name);
                // If the enclosing environment is not present, the variable is undefined.
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
        // Check if the variable exists in the current environment.
        if self.values.contains_key(&name.lexeme) {
            // Inserts the new value into the current environment.
            self.values
                .insert(name.lexeme.clone(), Rc::new(RefCell::new(value.clone())));
            return Ok(value);
        }

        // If the variable is not in the current environment, check the enclosing environment.
        // RECURSIVE FUNCTION CALL - BAND A
        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow_mut().assign(name, value);
        }

        // If the variable is not in the enclosing environment, it is undefined.
        return Err(EvaluatorError::UndefinedVariable { // EXCELLING ERROR HANDLING - BAND A
            name: name.lexeme.clone(),
            start: name.start,
            end: name.end,
            line: name.line
        });
    }
}

impl fmt::Display for Environment { // INTERFACES - BAND A
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "Environment(values: {:#?}, enclosing: {})", self.values, self.enclosing.is_some());
    }
}
