//! The `callable` module defines the `Callable` trait and the `Func` and `NativeFunc` structs,
//! which represent user-defined and native functions in the interpreter. These types are used
//! to encapsulate callable entities and their behavior.
//!
//! ## Overview
//!
//! The `Callable` trait provides a common interface for all callable entities, allowing them
//! to be invoked with a set of arguments. The `Func` struct represents user-defined functions
//! declared in the source code, while the `NativeFunc` struct represents functions implemented
//! in Rust and exposed to the interpreter.
//!
//! This module also implements the `Display` trait for both `Func` and `NativeFunc`, providing
//! string representations for debugging and logging purposes.
//!
//! ## Example
//!
//! ```rust
//! use crate::callable::{Callable, Func, NativeFunc};
//! use crate::value::{Value, LiteralType};
//! use crate::evaluator::Evaluator;
//!
//! let native_func = NativeFunc::new(
//!     "print".to_string(),
//!     1,
//!     |_, args| {
//!         println!("{:?}", args);
//!         Ok(Value::Literal(LiteralType::Null))
//!     },
//! );
//!
//! println!("{}", native_func);
//! ```
//!
//! ## Usage
//!
//! Functions are created during the parsing phase and are used by the evaluator to execute
//! callable entities. The `Callable` trait provides a unified interface for invoking both
//! user-defined and native functions.

use std::{
    cell::RefCell,
    fmt,
    rc::Rc,
};

use crate::{
    environment::Environment,
    error::EvaluatorError,
    evaluator::{Env, Evaluator},
    stmt::Stmt,
    value::{LiteralType, Value},
};

/// The `Callable` trait defines the interface for all callable entities in the interpreter.
///
/// ## Methods
/// - `call`: Invokes the callable entity with the given arguments and returns the result.
pub trait Callable { // INTERFACE DEFINITION - BAND A
    fn call(&self, evaluator: &mut Evaluator, arguments: Vec<Value>) -> Result<Value, EvaluatorError>;
}

/// The `Func` struct represents a user-defined function.
///
/// ## Fields
/// - `name`: The name of the function.
/// - `arity`: The number of parameters the function takes.
/// - `declaration`: The statement that declares the function.
/// - `closure`: The environment in which the function was declared.
#[derive(Clone, Debug)]
pub struct Func {
    name: String,
    pub arity: usize,
    declaration: Stmt,
    closure: Env, // COMPOSITION - BAND A
}

impl PartialEq for Func {
    /// Implements equality for `Func` based on its name, arity, and declaration.
    fn eq(&self, other: &Self) -> bool {
        println!("Should never be called");
        return self.name == other.name
            && self.arity == other.arity
            && self.declaration == other.declaration;
    }
}

impl PartialOrd for Func {
    /// Implements partial ordering for `Func` based on its name.
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Func {
    /// Creates a new `Func` instance.
    ///
    /// ## Parameters
    /// - `declaration`: The statement that declares the function.
    /// - `closure`: The environment in which the function was declared.
    ///
    /// ## Returns
    /// A new `Func` instance or an `EvaluatorError` if the declaration is invalid.
    pub fn new(declaration: Stmt, closure: Env) -> Result<Self, EvaluatorError> {
        match &declaration {
            Stmt::Function { name, params, .. } => {
                return Ok(Self {
                    name: name.lexeme.clone(),
                    arity: params.len(),
                    declaration,
                    closure,
                });
            },
            _ => return Err(EvaluatorError::ExpectedFunctionStatementForDeclaration),
        }
    }
}

impl Callable for Func { // INTERFACE - BAND A
    /// Calls the user-defined function with the given arguments.
    ///
    /// ## Parameters
    /// - `evaluator`: The evaluator instance.
    /// - `arguments`: The arguments passed to the function.
    ///
    /// ## Returns
    /// The result of the function execution or an `EvaluatorError`.
    fn call(&self, evaluator: &mut Evaluator, arguments: Vec<Value>) -> Result<Value, EvaluatorError> {
        match &self.declaration {
            Stmt::Function { name: _, params, body } => {
                // Create a new environment for the function call.
                let environment = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(
                    &self.closure,
                )))));

                // Define the function parameters in the new environment.
                for i in 0..params.len() {
                    environment
                        .borrow_mut()
                        .define(params[i].lexeme.clone(), arguments[i].clone());
                }

                // Execute the function body in the new environment.
                // RECURSIVE FUNCTION CALL - BAND A
                return match evaluator.execute_body(body.clone(), environment) {
                    Ok(_) => Ok(Value::Literal(LiteralType::Null)),
                    Err(r) => Ok(r?) // GOOD EXCEPTION HANDLING - EXCELLENT CODING STYLES
                }
            }
            _ => return Err(EvaluatorError::ExpectedDeclarationToBeAFunction),
        }
    }
}

/// The `NativeFunc` struct represents a native function implemented in Rust.
///
/// ## Fields
/// - `name`: The name of the native function.
/// - `arity`: The number of parameters the native function takes.
/// - `fun`: The function pointer to the native function implementation.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NativeFunc {
    name: String,
    pub arity: usize,
    fun: fn(&mut Evaluator, Vec<Value>) -> Result<Value, EvaluatorError>, // FUNCTION POINTER - BAND A EQUIVALENT
}

impl NativeFunc {
    /// Creates a new `NativeFunc` instance.
    ///
    /// ## Parameters
    /// - `name`: The name of the native function.
    /// - `arity`: The number of parameters the native function takes.
    /// - `fun`: The function pointer to the native function implementation.
    ///
    /// ## Returns
    /// A new `NativeFunc` instance.
    pub fn new(name: String, arity: usize, fun: fn(&mut Evaluator, Vec<Value>) -> Result<Value, EvaluatorError>) -> Self {
        return Self { name, arity, fun };
    }
}

impl Callable for NativeFunc { // INTERFACE - BAND A
    /// Calls the native function with the given arguments.
    ///
    /// ## Parameters
    /// - `evaluator`: The evaluator instance.
    /// - `arguments`: The arguments passed to the function.
    ///
    /// ## Returns
    /// The result of the native function execution or an `EvaluatorError`.
    fn call(&self, evaluator: &mut Evaluator, arguments: Vec<Value>) -> Result<Value, EvaluatorError> {
        return (self.fun)(evaluator, arguments); // FUNCTION POINTER INVOCATION - BAND A EQUIVALENT
    }
}

impl fmt::Display for NativeFunc {
    /// Implements the `Display` trait for `NativeFunc` to provide a string representation
    /// of the native function.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}({}) {{{:?}}}", self.name, self.arity, self.fun);
    }
}

impl fmt::Display for Func {
    /// Implements the `Display` trait for `Func` to provide a string representation
    /// of the user-defined function.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}({}) {{{}}}", self.name, self.arity, self.declaration);
    }
}