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

pub trait Callable {
    fn call(&self, evaluator: &mut Evaluator, arguments: Vec<Value>) -> Result<Value, EvaluatorError>;
}

#[derive(Clone, Debug)]
pub struct Func {
    name: String,
    pub arity: usize,
    declaration: Stmt,
    closure: Env,
}

impl PartialEq for Func {
    fn eq(&self, other: &Self) -> bool {
        println!("Should never be called");
        return self.name == other.name
            && self.arity == other.arity
            && self.declaration == other.declaration;
    }
}

impl PartialOrd for Func {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Func {
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

impl Callable for Func {
    fn call(&self, evaluator: &mut Evaluator, arguments: Vec<Value>) -> Result<Value, EvaluatorError> {
        match &self.declaration {
            Stmt::Function { name: _, params, body } => {
                let environment = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(
                    &self.closure,
                )))));

                for i in 0..params.len() {
                    environment
                        .borrow_mut()
                        .define(params[i].lexeme.clone(), arguments[i].clone());
                }

                return match evaluator.execute_block(body.clone(), environment) {
                    Ok(_) => Ok(Value::Literal(LiteralType::Null)),
                    Err(r) => Ok(r?)
                }
            }
            _ => return Err(EvaluatorError::ExpectedDeclarationToBeAFunction),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NativeFunc {
    name: String,
    pub arity: usize,
    fun: fn(&mut Evaluator, Vec<Value>) -> Result<Value, EvaluatorError>,
}

impl NativeFunc {
    pub fn new(name: String, arity: usize, fun: fn(&mut Evaluator, Vec<Value>) -> Result<Value, EvaluatorError>) -> Self {
        return Self { name, arity, fun };
    }
}

impl Callable for NativeFunc {
    fn call(&self, evaluator: &mut Evaluator, arguments: Vec<Value>) -> Result<Value, EvaluatorError> {
        return (self.fun)(evaluator, arguments);
    }
}

impl fmt::Display for NativeFunc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}({}) {{{:?}}}", self.name, self.arity, self.fun);
    }
}

impl fmt::Display for Func {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}({}) {{{}}}", self.name, self.arity, self.declaration);
    }
}