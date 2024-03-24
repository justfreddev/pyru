use std::{
    cell::RefCell,
    collections::HashMap,
    fmt,
    rc::Rc
};

use crate::{ expr::Value, tokens::Token };

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Rc<RefCell<Value>>>,
    enclosing: Option<Rc<RefCell<Environment>>>
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Self { values: HashMap::new(), enclosing }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, Rc::new(RefCell::new(value)));
    }

    pub fn get(&self, name: Token) -> Value {
        match self.values.get(&name.lexeme) {
            Some(v) => return v.borrow().clone(),
            _ => {
                if let Some(enclosing) = &self.enclosing {
                    return enclosing.borrow().get(name);
                }
            },
        }

        println!("{:#?}", self.values);
        panic!("Undefined variable {}.", name.lexeme)
    }

    pub fn assign(&mut self, name: Token, value: Value) {
        if let Some(x) = self.values.get_mut(&name.lexeme) {
            let mut y = x.borrow_mut();
            *y = value;
            return;
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow_mut().assign(name, value);
            return;
        }

        panic!("Undefined variable {}.", name.lexeme)
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Environment(values: {:#?}, enclosing: {})", self.values, self.enclosing.is_some())
    }
}