use paste::paste;
use std::fmt;

use crate::{
    expr::Expr,
    stmt_visitor,
    token::Token
};

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

stmt_visitor!(Expression, For, Function, If, Print, Return, Var, While);
