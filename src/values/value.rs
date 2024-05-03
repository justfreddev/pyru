use std::fmt;

use crate::{callable::{Func, NativeFunc}, list::List};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Value {
    Function(Func),
    List(List),
    Literal(LiteralType),
    NativeFunction(NativeFunc),
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
            Value::Function(fun) => write!(f, "Function({fun})"),
            Value::List(list) => write!(f, "{list}"),
            Value::Literal(literal) => write!(f, "{literal}"),
            Value::NativeFunction(nf) => write!(f, "NativeFunction({nf})"),
        };
    }
}

impl fmt::Display for LiteralType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            LiteralType::Str(s) => write!(f, "{s}"),
            LiteralType::Num(n) => write!(f, "{n}"),
            LiteralType::True => write!(f, "true"),
            LiteralType::False => write!(f, "false"),
            LiteralType::Null => write!(f, "null")
        };
    }
}