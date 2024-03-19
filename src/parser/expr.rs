use std::fmt;
use interpreter_v1::tokens::Token;

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralType {
    Str(String),
    Num(f64),
    True,
    False,
    Nil
}

#[derive(Clone, Debug)]
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
        right: Box<Expr>
    },
    Var {
        name: Token
    }
}

impl fmt::Display for LiteralType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralType::Str(s) => write!(f, "Str({s})"),
            LiteralType::Num(n) => write!(f, "Num({n})"),
            LiteralType::True => write!(f, "True"),
            LiteralType::False => write!(f, "False"),
            LiteralType::Nil => write!(f, "Nil"),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Binary{left, operator, right} => write!(f, "Binary({left} {operator} {right})"),
            Expr::Grouping { expression } => write!(f, "Grouping({expression})"),
            Expr::Literal { value } => write!(f, "Literal({value})"),
            Expr::Unary { operator, right } => write!(f, "Unary({operator} {right})"),
            Expr::Var { name } => write!(f, "Var({name})")
        }
    }
}

pub trait Visitor<T> {
    fn visit_binary_expr(&mut self, expr: &Expr) -> T;
    fn visit_grouping_expr(&mut self, expr: &Expr) -> T;
    fn visit_literal_expr(&mut self, expr: &Expr) -> T;
    fn visit_unary_expr(&mut self, expr: &Expr) -> T;
    fn visit_variable_expr(&mut self, expr: &Expr) -> T;
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Expr::Binary { .. } => visitor.visit_binary_expr(self),
            Expr::Grouping { .. } => visitor.visit_grouping_expr(self),
            Expr::Literal { .. } => visitor.visit_literal_expr(self),
            Expr::Unary { .. } => visitor.visit_unary_expr(self),
            Expr::Var { .. } => visitor.visit_variable_expr(self)
        }
    }
}

pub struct AstPrinter;


impl Visitor<String> for AstPrinter {
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

    fn visit_variable_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Var { name } => {
                let mut name_string = String::from("Var");
                name_string.push_str(name.to_string().as_str());
                name_string
            }
            _ => panic!("Expected a variable expression")
        }
    }
}

impl AstPrinter {
    pub fn _print(&mut self, expr: &Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&mut self, name: &str, exprs: Vec<&Expr>) -> String {
        let mut string = String::from("(");
        string.push_str(name);

        for expr in exprs {
            string.push(' ');
            string.push_str(expr.accept(self).as_str());
        }

        string.push(')');

        string
    }
}