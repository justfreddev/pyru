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
        callee: Box<Expr>, // The name of the call, e.g. the function name
        arguments: Vec<Expr>, // The arguments passed in the parenthesise
    },
    Grouping {
        expression: Box<Expr>, // The expresion in brackets, usually binary
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
    Splice {
        list: Token, // The name of the variable for the list
        is_splice: bool, // Check if it is a splice to see if returning list or value
        start: Option<Box<Expr>>, // The start index INCLUSIVE
        end: Option<Box<Expr>>, // The end index INCLUSIVE
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Var {
        name: Token, // The name of the variable which the value is gotten from
    },
}

impl fmt::Display for Expr {
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
            Expr::Splice { list, is_splice: _, start, end } => {
                write!(f, "{list}[{start:?}:{end:?}]")
            },
            Expr::Unary { operator, right } => write!(f, "Unary({operator} {right})"),
            Expr::Var { name } => write!(f, "Var({name})"),
        }
    }
}

expr_visitor!(Alteration, Assign, Binary, Call, Grouping, List, ListMethodCall, Literal, Logical, Splice, Unary, Var);
