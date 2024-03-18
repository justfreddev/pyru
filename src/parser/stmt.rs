use crate::expr::Expr;

pub enum Stmt {
    Expression {
        expression: Expr
    },
    Print {
        expression: Expr
    }
}

pub trait Visitor<T> {
    fn visit_expression_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_print_stmt(&mut self, stmt: &Stmt) -> T;
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Stmt::Expression { .. } => visitor.visit_expression_stmt(self),
            Stmt::Print { .. } => visitor.visit_print_stmt(self)
        }
    }
}