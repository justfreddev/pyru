use interpreter_v1::tokens::Token;

use crate::expr::LiteralType;

use std::collections::HashMap;

pub struct Environment {
    values: HashMap<String, LiteralType>
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new()
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
        panic!("Undefined variable {}.", name.lexeme)
    }
}