use std::fmt;

use crate::callable::{Func, NativeFunc};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Literal(LiteralType),
    Function(Func),
    NativeFunction(NativeFunc)
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
        match self {
            Value::Literal(literal) => write!(f, "Literal({literal})"),
            Value::Function(fun) => write!(f, "Function({fun})"),
            Value::NativeFunction(nf) => write!(f, "NativeFunction({nf})")
        }
    }
}


impl fmt::Display for LiteralType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralType::Str(s) => write!(f, "Str({s})"),
            LiteralType::Num(n) => write!(f, "Num({n})"),
            LiteralType::True => write!(f, "True"),
            LiteralType::False => write!(f, "False"),
            LiteralType::Null => write!(f, "Null")
        }
    }
}