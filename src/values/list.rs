use std::fmt;

use crate::value::Value;

#[derive(Clone, Debug, PartialEq)]
pub struct List {
    values: Vec<Value>
}

impl List {
    pub fn new(values: Vec<Value>) -> Self {
        return Self { values };
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, value) in self.values.iter().enumerate() {
            if i > 0 {
            write!(f, ", ")?;
            }
            write!(f, "{}", value)?;
        }
        write!(f, "]")
    }
}