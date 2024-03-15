#[path ="../lexer/tokens.rs"]
mod tokens;

use tokens::{Token, TokenType};

enum LiteralType {
    Str,
    Num
}

enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>
    },
    Grouping {
        expression: Box<Expr>
    },
    Literal {
        value: String,
        value_type: LiteralType
    },
    Unary {
        operator: Token,
        right: Box<Expr>
    }
}

trait Visitor<T> {
    fn visit_binary_expr(&mut self, expr: &Expr) -> T;
    fn visit_grouping_expr(&mut self, expr: &Expr) -> T;
    fn visit_literal_expr(&mut self, expr: &Expr) -> T;
    fn visit_unary_expr(&mut self, expr: &Expr) -> T;
}

impl Expr {
    fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Expr::Binary { .. } => visitor.visit_binary_expr(self),
            Expr::Grouping { .. } => visitor.visit_grouping_expr(self),
            Expr::Literal { .. } => visitor.visit_literal_expr(self),
            Expr::Unary { .. } => visitor.visit_unary_expr(self),
        }
    }
}

struct AstPrinter;


impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Binary { left, operator, right } => self.parenthesize(operator.lexeme.clone(), vec![&*left, &*right]),
            _ => panic!("Expected a binary expression")
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Grouping { expression } => self.parenthesize(String::from(""), vec![&*expression]),
            _ => panic!("Expected a group expression")
        }
    }

    fn visit_literal_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Literal { value, value_type } => match value_type {
                LiteralType::Str => format!("\"{value}\""),
                _ => {
                    if value == "" {
                        return String::from("nil");
                    }
                    value.clone()
                }
            },
            _ => panic!("Expcted a literal expression"),
        }
    }

    fn visit_unary_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Unary { operator, right } => self.parenthesize(operator.lexeme.clone(), vec![&*right]),
            _ => panic!("Expected a unary expression")
        }
    }
}

impl AstPrinter {
    fn print(&mut self, expr: &Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&mut self, name: String, exprs: Vec<&Expr>) -> String {
        let mut string = String::from("(");
        string.push_str(&name);

        for expr in exprs {
            string.push_str(" ");
            string.push_str(expr.accept(self).as_str());
        }

        string.push_str(")");

        string
    }
}

pub fn run_ast() {
    let expr = Expr::Binary{
        left: Box::from(Expr::Unary {
            operator: Token::new(TokenType::Minus, String::from("-"), String::from(""), 1),
            right: Box::new(Expr::Literal { value: "123".to_string(), value_type: LiteralType::Num })}),
        operator: Token::new(TokenType::Asterisk, String::from("*"), String::from(""), 1),
        right: Box::new(Expr::Grouping { expression: Box::new(Expr::Literal{ value: "45.67".to_string(), value_type: LiteralType::Str })})
    };

    let ast_expr = AstPrinter.print(&expr);
    println!("{}", ast_expr);
}