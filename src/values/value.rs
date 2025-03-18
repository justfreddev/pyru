//! This module defines the `Value` and `LiteralType` enums, which represent the different types of
//! values that can be used in the interpreter. These include functions, lists, literals, and native
//! functions. The module also implements the `Display` trait for these types to provide string
//! representations of their values.

use std::fmt;

use crate::{callable::{Func, NativeFunc}, list::List};

/// Represents the different types of values that can be used in the interpreter.
/// 
/// ## Variants
/// - `Function(Func)`: Represents a user-defined function.
/// - `List(List)`: Represents a list of values.
/// - `Literal(LiteralType)`: Represents a literal value (e.g., string, number, boolean, null).
/// - `NativeFunction(NativeFunc)`: Represents a native function implemented in Rust.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Value {
    Function(Func),
    List(List),
    Literal(LiteralType),
    NativeFunction(NativeFunc),
}

/// Represents the different types of literal values that can be used in the interpreter.
/// 
/// ## Variants
/// - `Str(String)`: Represents a string literal.
/// - `Num(f64)`: Represents a numeric literal.
/// - `True`: Represents the boolean value `true`.
/// - `False`: Represents the boolean value `false`.
/// - `Null`: Represents the absence of a value.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum LiteralType {
    Str(String),
    Num(f64),
    True,
    False,
    Null,
}

/// Implements the `Display` trait for the `Value` enum to provide a string representation
/// of each variant.
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

/// Implements the `Display` trait for the `LiteralType` enum to provide a string representation
/// of each variant.
impl fmt::Display for LiteralType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            LiteralType::Str(s) => write!(f, "{s}"),
            LiteralType::Num(n) => write!(f, "{n}"),
            LiteralType::True => write!(f, "true"),
            LiteralType::False => write!(f, "false"),
            LiteralType::Null => write!(f, "null"),
        };
    }
}