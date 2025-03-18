//! The `expr` module defines the `Expr` enum, which represents the different types of expressions
//! that can be encountered in the source code. Expressions are the fundamental units of computation
//! in the language and are used to evaluate values, perform operations, and control program behavior.
//!
//! ## Overview
//!
//! The `Expr` enum includes variants for literals, variables, binary operations, function calls,
//! list operations, and more. Each variant corresponds to a specific type of expression in the
//! language and contains the necessary fields to represent its structure and behavior.
//!
//! This module also implements the `Display` trait for the `Expr` enum, providing a string
//! representation of each expression for debugging and logging purposes.
//!
//! Additionally, the `expr_visitor!` macro is used to generate the visitor design pattern for
//! traversing and processing expressions.
//!
//! ## Example
//!
//! ```rust
//! use crate::expr::Expr;
//! use crate::token::{Token, TokenType};
//! use crate::value::LiteralType;
//!
//! let expr = Expr::Binary {
//!     left: Box::new(Expr::Literal { value: LiteralType::Num(5.0) }),
//!     operator: Token {
//!         token_type: TokenType::Plus,
//!         lexeme: "+".to_string(),
//!         literal: "".to_string(),
//!         line: 1,
//!         start: 0,
//!         end: 1,
//!     },
//!     right: Box::new(Expr::Literal { value: LiteralType::Num(3.0) }),
//! };
//!
//! println!("{}", expr);
//! ```
//!
//! ## Usage
//!
//! Expressions are created during the parsing phase and are used by the evaluator to compute
//! values and execute operations. The `Expr` enum provides a comprehensive representation of
//! all possible expression types in the language.

use paste::paste;
use std::fmt;

use crate::{
    expr_visitor,
    token::{Token, TokenType},
    value::LiteralType,
};

/// Represents the different types of expressions that can be encountered in the source code.
///
/// ## Variants
/// - `Alteration`: Represents an increment or decrement operation on a variable.
/// - `Assign`: Represents an assignment of a value to a variable.
/// - `Binary`: Represents a binary operation (e.g., addition, subtraction).
/// - `Call`: Represents a function or method call.
/// - `Grouping`: Represents a grouped expression (e.g., expressions in parentheses).
/// - `List`: Represents a list literal.
/// - `ListMethodCall`: Represents a method call on a list.
/// - `Literal`: Represents a literal value (e.g., string, number, boolean).
/// - `Logical`: Represents a logical operation (e.g., `and`, `or`).
/// - `Membership`: Represents a membership test (e.g., `in`, `not in`).
/// - `Splice`: Represents a list slicing operation.
/// - `Unary`: Represents a unary operation (e.g., negation).
/// - `Var`: Represents a variable reference.
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Alteration {
        name: Token, // Variable name
        alteration_type: TokenType, // Incr or Decr tokens
    },
    Assign {
        name: Token, // Variable name
        value: Box<Expr>, // The expression to be assigned
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>, // The name of the call, e.g., the function name
        arguments: Vec<Expr>, // The arguments passed in the parentheses
    },
    Grouping {
        expression: Box<Expr>, // The expression in parentheses, usually binary
    },
    List {
        items: Vec<Expr>, // The items to be in the created list
    },
    ListMethodCall {
        object: Token, // The name of the instance that the method is being called on
        call: Box<Expr>, // A call expression for the method call
    },
    Literal {
        value: LiteralType,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Membership {
        left: Box<Expr>, // The expression to be searched for
        not: bool, // Whether the membership test is negated
        right: Box<Expr>, // The list
    },
    Splice {
        list: Token, // The name of the variable for the list
        is_splice: bool, // Whether it is a splice (returns a list or value)
        start: Option<Box<Expr>>, // The start index (inclusive)
        end: Option<Box<Expr>>, // The end index (inclusive)
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Var {
        name: Token, // The name of the variable whose value is retrieved
    },
}

impl fmt::Display for Expr {
    /// Implements the `Display` trait for `Expr` to provide a string representation
    /// of each expression variant.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            Expr::Alteration { name, alteration_type } => {
                write!(f, "Alteration({name} {alteration_type})")
            },
            Expr::Assign { name, value } => write!(f, "Assign({name} = {value}"),
            Expr::Binary { left, operator, right } => {
                write!(f, "Binary({left} {operator} {right})")
            },
            Expr::Call { callee, arguments } => write!(f, "Call({callee} {arguments:?})"),
            Expr::Grouping { expression } => write!(f, "Grouping({expression})"),
            Expr::List { items } => write!(f, "[{items:?}]"),
            Expr::ListMethodCall { object, call } => write!(f, "{object}.{call}"),
            Expr::Literal { value } => write!(f, "{value}"),
            Expr::Logical { left, operator, right } => {
                write!(f, "Logical({left} {operator} {right})")
            },
            Expr::Membership { left, not, right } => {
                if *not {
                    return write!(f, "{left} not in {right}");
                };
                write!(f, "{left} in {right}")
            },
            Expr::Splice { list, is_splice: _, start, end } => {
                write!(f, "{list}[{start:?}:{end:?}]")
            },
            Expr::Unary { operator, right } => write!(f, "Unary({operator} {right})"),
            Expr::Var { name } => write!(f, "Var({name})"),
        }
    }
}

// Generates the visitor design pattern for expressions.
//
// This macro defines an `ExprVisitor` trait with methods for visiting each expression type.
// It also implements the `accept_expr` method for the `Expr` enum, which dispatches the
// appropriate visitor method based on the expression type.
expr_visitor!(Alteration, Assign, Binary, Call, Grouping, List, ListMethodCall, Literal, Logical, Membership, Splice, Unary, Var);