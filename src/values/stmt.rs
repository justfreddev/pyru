//! The `stmt` module defines the `Stmt` enum, which represents the different types of statements
//! that can be encountered in the source code. Statements are the building blocks of the program's
//! control flow and behavior.
//!
//! ## Overview
//!
//! The `Stmt` enum includes variants for expressions, loops, conditionals, variable declarations,
//! function declarations, and more. Each variant corresponds to a specific type of statement in the
//! language and contains the necessary fields to represent its structure and behavior.
//!
//! This module also implements the `Display` trait for the `Stmt` enum, providing a string
//! representation of each statement for debugging and logging purposes.
//!
//! Additionally, the `stmt_visitor!` macro is used to generate the visitor design pattern for
//! traversing and processing statements.
//!
//! ## Example
//!
//! ```rust
//! use crate::stmt::Stmt;
//! use crate::expr::Expr;
//! use crate::token::Token;
//!
//! let stmt = Stmt::Print {
//!     expression: Expr::Literal("Hello, world!".into()),
//! };
//!
//! println!("{}", stmt);
//! ```
//!
//! ## Usage
//!
//! Statements are created during the parsing phase and are used by the evaluator to execute the
//! program. The `Stmt` enum provides a comprehensive representation of all possible statement
//! types in the language.

use paste::paste;
use std::fmt;

use crate::{
    expr::Expr,
    stmt_visitor,
    token::Token
};

/// Represents the different types of statements that can be encountered in the source code.
///
/// ## Variants
/// - `Expression`: Represents an expression statement.
/// - `For`: Represents a `for` loop.
/// - `Function`: Represents a function declaration.
/// - `If`: Represents an `if` statement with optional `else` branch.
/// - `Print`: Represents a `print` statement.
/// - `Return`: Represents a `return` statement.
/// - `Var`: Represents a variable declaration.
/// - `While`: Represents a `while` loop.
#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Expression {
        expression: Expr,
    },
    For {
        initializer: Box<Stmt>,
        condition: Expr,
        step: Expr,
        body: Vec<Stmt>,
    },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    Print {
        expression: Expr,
    },
    Return {
        keyword: Token,
        value: Option<Expr>,
    },
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
}

impl fmt::Display for Stmt {
    /// Implements the `Display` trait for `Stmt` to provide a string representation
    /// of each statement variant.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Expression { expression } => write!(f, "Expression({expression})"),
            Stmt::For { initializer, condition, step, body } => {
                return write!(f, "For({initializer:?} {condition} {step:?} {body:?})");
            },
            Stmt::Function { name, params, body } => {
                return write!(f, "Function({name} {params:?} {body:?})")
            },
            Stmt::If { condition, then_branch, else_branch } => {
                if else_branch.is_some() {
                    return write!(
                        f,
                        "If({condition} {then_branch:?} {})",
                        else_branch.as_ref().unwrap()
                    );
                } else {
                    return write!(f, "If({condition} {then_branch:?})");
                }
            },
            Stmt::Print { expression } => write!(f, "Print({expression})"),
            Stmt::Return { keyword: _, value } => return write!(f, "Return({value:?})"),
            Stmt::Var { name, initializer } => {
                if initializer.is_some() {
                    return write!(f, "Var({name} {}", initializer.as_ref().unwrap());
                } else {
                    return write!(f, "Var({name})");
                }
            }
            Stmt::While { condition, body } => return write!(f, "While({condition} {body:?})"),
        }
    }
}

// Generates the visitor design pattern for statements.
//
// This macro defines a `StmtVisitor` trait with methods for visiting each statement type.
// It also implements the `accept_stmt` method for the `Stmt` enum, which dispatches the
// appropriate visitor method based on the statement type.
stmt_visitor!(Expression, For, Function, If, Print, Return, Var, While);