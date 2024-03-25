use paste::paste;
use std::{ cell::RefCell, fmt, rc::Rc };

use crate::{
    environment::Environment,
    expr::{ Expr, LiteralType, Value },
    interpreter::Interpreter,
    stmt_visitor,
    tokens::Token
};

pub trait Callable {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Value;
    fn arity(&self) -> usize;
    fn _fn_to_string(&self) -> String;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    declaration: Stmt,
}

impl Function {
    pub fn new(declaration: Stmt) -> Self {
        match declaration {
            Stmt::Function { .. } => Self { declaration },
            _ => panic!("Expected declaration to be a function statement")
        }
    }
}

impl Callable for Function {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Value {
        match &self.declaration {
            Stmt::Function { name: _, params, body } => {
                let mut environment = Environment::new(Some(Rc::new(RefCell::new(interpreter.globals.clone()))));
                for i in 0..params.len() {
                    environment.define(params[i].lexeme.clone(), arguments[i].clone());
                }

                interpreter.execute_block(body.clone(), environment);
                Value::Literal(LiteralType::Nil)
            },
            _ => panic!("Expected declaration to be a function statement")
        }
        
        
    }
    
    fn arity(&self) -> usize {
        match &self.declaration {
            Stmt::Function { name: _, params, body: _ } => params.len(),
            _ => panic!("Expected declaration to be a function statement")
        }
    }
    
    fn _fn_to_string(&self) -> String {
        match &self.declaration {
            Stmt::Function { name, params: _, body: _ } => format!("<fn {}>", name.lexeme),
            _ => panic!("Expected declaration to be a function statement")
        }
        
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.declaration)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NativeFunction {
    name: String,
    arity: usize,
    fun: fn(&mut Interpreter, Vec<Value>) -> Value
}

impl NativeFunction {
    pub fn new(name: String, arity: usize, fun: fn(&mut Interpreter, Vec<Value>) -> Value) -> Self {
        Self {
            name,
            arity,
            fun
        }
    }
}

impl Callable for NativeFunction {
    fn arity(&self) -> usize {
        self.arity
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Value {
        (self.fun)(interpreter, arguments)
    }
    
    fn _fn_to_string(&self) -> String {
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Expression {
        expression: Expr
    },
    Print {
        expression: Expr
    },
    Var {
        name: Token,
        initializer: Option<Expr>
    },
    Block {
        statements: Vec<Stmt>
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>
    },
    While {
        condition: Expr,
        body: Box<Stmt>
    },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Block { statements } => write!(f, "Block({statements:?}"),
            Stmt::Expression { expression } => write!(f, "Expression({expression})"),
            Stmt::If { condition, then_branch, else_branch } => {
                if else_branch.is_some() {
                    write!(f, "If({condition} {then_branch} {})", else_branch.as_ref().unwrap())
                } else {
                    write!(f, "If({condition} {then_branch})")
                }
                
            },
            Stmt::Print { expression } => write!(f, "Print({expression})"),
            Stmt::Var { name, initializer } => {
                if initializer.is_some() {
                    write!(f, "Var({name} {}", initializer.as_ref().unwrap())
                } else {
                    write!(f, "Var({name})")
                }
                
            },
            Stmt::While { condition, body } => write!(f, "While({condition} {body})"),
            Stmt::Function { name, params, body } => write!(f, "Function({name} {params:?} {body:?})"),
        }
    }
}

stmt_visitor!(Expression, Print, Var, Block, If, While, Function);