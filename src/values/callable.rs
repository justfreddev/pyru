use std::{cell::RefCell, fmt, rc::Rc};

use crate::{
    enviromnent::{Environment, LocalEnvironment},
    error::InterpreterError,
    interpreter::Interpreter,
    stmt::Stmt,
    value::{LiteralType, Value},
};

pub trait Callable {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, InterpreterError>;
    fn _fn_to_string(&self) -> String;
}

#[derive(Clone, Debug)]
pub struct Func {
    name: String,
    pub arity: usize,
    declaration: Stmt,
    closure: Rc<RefCell<dyn Environment>>,
}

impl PartialEq for Func {
    fn eq(&self, other: &Self) -> bool {
        println!("Should never be called");
        self.name == other.name
            && self.arity == other.arity
            && self.declaration == other.declaration
    }
}

impl Func {
    pub fn new(
        declaration: Stmt,
        closure: Rc<RefCell<dyn Environment>>,
    ) -> Result<Self, InterpreterError> {
        match &declaration {
            Stmt::Function {
                name,
                params,
                body: _,
            } => Ok(Self {
                name: name.lexeme.clone(),
                arity: params.len(),
                declaration,
                closure,
            }),
            _ => Err(InterpreterError::ExpectedFunctionStatementForDeclaration),
        }
    }
}

impl Callable for Func {
    fn call(
        &self,
        interpreter: &mut crate::interpreter::Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, InterpreterError> {
        match &self.declaration {
            Stmt::Function {
                name: _,
                params,
                body,
            } => {
                let environment = Rc::new(RefCell::new(LocalEnvironment::new(Some(Rc::clone(
                    &self.closure,
                )))));

                for i in 0..params.len() {
                    environment
                        .borrow_mut()
                        .define(params[i].lexeme.clone(), arguments[i].clone());
                }

                return match interpreter.execute_block(body.clone(), environment) {
                    Ok(_) => Ok(Value::Literal(LiteralType::Null)),
                    Err(r) => match r {
                        Ok(v) => Ok(v),
                        Err(e) => Err(e),
                    },
                };
            }
            _ => return Err(InterpreterError::ExpectedDeclarationToBeAFunction),
        }
    }

    fn _fn_to_string(&self) -> String {
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NativeFunc {
    name: String,
    pub arity: usize,
    fun: fn(&mut Interpreter, Vec<Value>) -> Result<Value, InterpreterError>,
}

impl NativeFunc {
    pub fn new(
        name: String,
        arity: usize,
        fun: fn(&mut Interpreter, Vec<Value>) -> Result<Value, InterpreterError>,
    ) -> Self {
        Self { name, arity, fun }
    }
}

impl Callable for NativeFunc {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, InterpreterError> {
        (self.fun)(interpreter, arguments)
    }

    fn _fn_to_string(&self) -> String {
        "<native fn>".to_string()
    }
}

impl fmt::Display for NativeFunc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({}) {{{:?}}}", self.name, self.arity, self.fun)
    }
}

impl fmt::Display for Func {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({}) {{{}}}", self.name, self.arity, self.declaration)
    }
}