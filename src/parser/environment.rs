use std::collections::HashMap;

use crate::{expr::LiteralType, tokens::Token};

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, LiteralType>,
    enclosing: Option<Box<Environment>>
}

impl Environment {
    pub fn new(enclosing: Option<Environment>) -> Self {
        match enclosing {
            Some(environment) => Self { values: HashMap::new(), enclosing: Some(Box::new(environment)) },
            None => Self { values: HashMap::new(), enclosing: None }
        }
    }

    pub fn define(&mut self, name: String, value: LiteralType) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: Token) -> LiteralType {
        if self.values.contains_key(&name.lexeme) {
            match self.values.get(&name.lexeme) {
                Some(v) => return v.clone(),
                _ => panic!("Undefined variable {}.", name.lexeme),
            }
        }

        if let Some(enclosing) = &self.enclosing {
            return enclosing.get(name);
        }

        panic!("Undefined variable {}.", name.lexeme)
    }

    pub fn assign(&mut self, name: Token, value: &LiteralType) {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme, value.clone());
            return;
        }

        if let Some(enclosing) = &mut self.enclosing {
            enclosing.assign(name, value);
            return;
        }

        panic!("Undefined variable {}.", name.lexeme)
    }
}