use paste::paste;
use std::fmt;

use crate::{
    value::LiteralType,
    expr_visitor,
    token::{ Token, TokenType }
};

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>
    },
    Grouping {
        expression: Box<Expr>
    },
    Literal {
        value: LiteralType
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Var {
        name: Token
    },
    Assign {
        name: Token,
        value: Box<Expr>
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>
    },
    Alteration {
        name: Token,
        alteration_type: TokenType
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Binary{left, operator, right} => write!(f, "Binary({left} {operator} {right})"),
            Expr::Grouping { expression } => write!(f, "Grouping({expression})"),
            Expr::Literal { value } => write!(f, "Literal({value})"),
            Expr::Unary { operator, right } => write!(f, "Unary({operator} {right})"),
            Expr::Var { name } => write!(f, "Var({name})"),
            Expr::Assign { name, value } => write!(f, "Assign({name} = {value}"),
            Expr::Logical { left, operator, right } => write!(f, "Logical({left} {operator} {right})"),
            Expr::Alteration { name, alteration_type } => write!(f, "Alteration({name} {alteration_type})"),
            Expr::Call { callee, paren, arguments: arguements } => write!(f, "Call({callee} {paren} {arguements:?})")
        }
    }
}

expr_visitor!(Binary, Grouping, Literal, Unary, Var, Assign, Logical, Alteration, Call);