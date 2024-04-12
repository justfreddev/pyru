use std::fmt;

use crate::{callable::{Func, NativeFunc}, class::{Instance, Klass}};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Class(Klass),
    Function(Func),
    Instance(Instance),
    Literal(LiteralType),
    NativeFunction(NativeFunc),
}

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralType {
    Str(String),
    Num(f64),
    True,
    False,
    Null
}



impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            Value::Class(cls) => write!(f, "Class({cls})"),
            Value::Function(fun) => write!(f, "Function({fun})"),
            Value::Instance(instance) => write!(f, "Instance({instance})"),
            Value::Literal(literal) => write!(f, "Literal({literal})"),
            Value::NativeFunction(nf) => write!(f, "NativeFunction({nf})"),
        };
    }
}


impl fmt::Display for LiteralType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            LiteralType::Str(s) => write!(f, "Str({s})"),
            LiteralType::Num(n) => write!(f, "Num({n})"),
            LiteralType::True => write!(f, "True"),
            LiteralType::False => write!(f, "False"),
            LiteralType::Null => write!(f, "Null")
        };
    }
}