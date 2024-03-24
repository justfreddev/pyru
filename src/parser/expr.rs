use paste::paste;
use std::{fmt, vec};

use crate::{
    expr_visitor,
    stmt::{Function, NativeFunction},
    tokens::{ Token, TokenType }
};

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralType {
    Str(String),
    Num(f64),
    True,
    False,
    Nil
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Literal(LiteralType),
    Func(Function),
    NativeFunc(NativeFunction),
}

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

// Displays types in the format: <type>(<value>)
impl fmt::Display for LiteralType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralType::Str(s) => write!(f, "Str({s})"),
            LiteralType::Num(n) => write!(f, "Num({n})"),
            LiteralType::True => write!(f, "True"),
            LiteralType::False => write!(f, "False"),
            LiteralType::Nil => write!(f, "Nil")
        }
    }
}

// Displays expressions in the format: <type>(<attributes,>)
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

// Use the expr_visitor! macro to generate the visitor design pattern for expressions
expr_visitor!(Binary, Grouping, Literal, Unary, Var, Assign, Logical, Alteration, Call);

pub struct AstPrinter;


impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Binary { left, operator, right } => self.parenthesize(operator.lexeme.as_str(), vec![left, right]),
            _ => panic!("Expected a binary expression")
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Grouping { expression } => self.parenthesize("", vec![expression]),
            _ => panic!("Expected a group expression")
        }
    }

    fn visit_literal_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Literal { value } => match value {
                LiteralType::Num(n) => n.to_string(),
                LiteralType::Str(s) => s.to_string(),
                LiteralType::True => "true".to_string(),
                LiteralType::False => "false".to_string(),
                LiteralType::Nil => "nil".to_string()
            }
            _ => panic!("Expcted a literal expression"),
        }
    }

    fn visit_unary_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Unary { operator, right } => self.parenthesize(operator.lexeme.as_str(), vec![right]),
            _ => panic!("Expected a unary expression")
        }
    }

    fn visit_var_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Var { name } => {
                let mut name_string = String::from("Var");
                name_string.push_str(name.to_string().as_str());
                name_string
            }
            _ => panic!("Expected a variable expression")
        }
    }

    fn visit_assign_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Assign { name, value } => format!("{name} = {value}"),
            _ => panic!("Expected an assignment expression")
        }
    }
    
    fn visit_logical_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Logical { left, operator, right } => self.parenthesize(operator.lexeme.as_str(), vec![left, right]),
            _ => panic!("Expected a logical expression")
        }
    }
    
    fn visit_alteration_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Alteration { name, alteration_type } => format!("{name}{alteration_type}"),
            _ => panic!("Expected an alteration expression")
        }
    }

    fn visit_call_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Call { callee, paren: _paren, arguments: arguements } => format!("{callee}({arguements:?})"),
            _ => panic!("Expected a call expression")
        }
    }
}

impl AstPrinter {
    pub fn _print(&mut self, expr: &Expr) -> String {
        expr.accept_expr(self)
    }

    fn parenthesize(&mut self, name: &str, exprs: Vec<&Expr>) -> String {
        let mut string = String::from("(");
        string.push_str(name);

        for expr in exprs {
            string.push(' ');
            string.push_str(expr.accept_expr(self).as_str());
        }

        string.push(')');

        string
    }
}