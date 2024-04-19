use paste::paste;
use std::fmt;

use crate::{
    expr_visitor,
    token::{Token, TokenType},
    value::LiteralType,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Alteration {
        name: Token,
        alteration_type: TokenType,
    },
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    List {
        items: Vec<Expr>,
    },
    Literal {
        value: LiteralType,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Var {
        name: Token,
    },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Alteration { name, alteration_type } => {
                return write!(f, "Alteration({name} {alteration_type})");
            },
            Expr::Assign { name, value } => return write!(f, "Assign({name} = {value}"),
            Expr::Binary { left, operator, right } => {
                return write!(f, "Binary({left} {operator} {right})");
            },
            Expr::Call { callee, paren, arguments } => {
                return write!(f, "Call({callee} {paren} {arguments:?})");
            },
            Expr::Grouping { expression } => return write!(f, "Grouping({expression})"),
            Expr::List { items } => return write!(f, "[{items:?}]"),
            Expr::Literal { value } => return write!(f, "Literal({value})"),
            Expr::Logical { left, operator, right } => {
                return write!(f, "Logical({left} {operator} {right})");
            },
            Expr::Unary { operator, right } => return write!(f, "Unary({operator} {right})"),
            Expr::Var { name } => return write!(f, "Var({name})"),
        }
    }
}

expr_visitor!(Alteration, Assign, Binary, Call, Grouping, List, Literal, Logical, Unary, Var);
