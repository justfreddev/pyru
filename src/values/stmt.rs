use paste::paste;
use std::fmt;

use crate::{ expr::Expr, stmt_visitor, token::Token };

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
    For {
        initializer: Option<Box<Stmt>>,
        condition: Expr,
        increment: Option<Expr>,
        body: Box<Stmt>
    },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>
    },
    Return {
        keyword: Token,
        value: Option<Expr>
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
            Stmt::For { initializer, condition, increment, body } => {
                write!(f, "For({initializer:?} {condition} {increment:?} {body})")
            },
            Stmt::Return { keyword: _, value } => write!(f, "Return({value:?})")
        }
    }
}

stmt_visitor!(Expression, Print, Var, Block, If, While, For, Function, Return);

