use interpreter_v1::tokens::{Token, TokenType};

use crate::expr::{self, Expr, LiteralType};
use crate::stmt::{self, Stmt};

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Self
    }

    pub fn line_error(line: usize, message: &str) {
        Interpreter::report(line, "", message);
    }
    
    pub fn report(line: usize, where_about: &str, message: &str) {
        println!("[line {line}] Error {where_about}: {message}");
    }

    pub fn token_error(token: &Token, message: &str) {
        if token.token_type == TokenType::Eof {
            Interpreter::report(token.line, " at end", message);
        } else {
            Interpreter::report(token.line, format!(" at '{}'", token.lexeme).as_str(), message);
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) {

        for stmt in statements {
            self.execute(&stmt);
        }

        // let result = self.evaluate(expression);
        // match result {
        //     LiteralType::Str(s) => println!("{s}"),
        //     LiteralType::Num(n) => println!("{}", n.to_string()),
        //     LiteralType::True => println!("True"),
        //     LiteralType::False => println!("False"),
        //     LiteralType::Nil => println!("Nil"),
        // }
    }

    pub fn evaluate(&mut self, expr: &Expr) -> LiteralType {
        expr.accept(self)
    }

    pub fn execute(&mut self, stmt: &Stmt) {
        stmt.accept(self);
    }

    fn unbox<T>(&mut self, value: T) -> T {
        value
    }

    fn is_truthy(&mut self, object: &LiteralType) -> bool {
        !matches!(object, LiteralType::Nil | LiteralType::False)
    }

    fn is_equal(&mut self, a: &LiteralType, b: &LiteralType) -> bool {
        *a == *b
    }

    fn stringify(&self, object: LiteralType) -> String {
        match object {
            LiteralType::Num(n) => {
                let mut text = n.to_string();
                if text.ends_with(".0") {
                    text.truncate(text.len() - 2);
                }
                text
            }
            LiteralType::Str(s) => s,
            LiteralType::True => "true".to_string(),
            LiteralType::False => "false".to_string(),
            LiteralType::Nil => "nil".to_string(),
        }
    }
}

impl expr::Visitor<LiteralType> for Interpreter {
    fn visit_literal_expr(&mut self, expr: &Expr) -> LiteralType {
        match expr {
            Expr::Literal { value } => value.clone(),
            _ => panic!("Expected a literal value")
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) -> LiteralType {
        match expr {
            Expr::Grouping { expression } => {
                let expression_value = self.unbox(expression.clone());
                self.evaluate(&expression_value)
            },
            _ => panic!("Expected a group expression")
        }
    }

    fn visit_binary_expr(&mut self, expr: &Expr) -> LiteralType {
        match expr {
            Expr::Binary { left, operator, right } => {
                let left_value = self.unbox(left.clone());
                let left = self.evaluate(&left_value);
                let right_value = self.unbox(right.clone());
                let right = self.evaluate(&right_value);
                match operator.token_type {
                    TokenType::Greater => {
                        if let LiteralType::Num(ln) = left {
                            if let LiteralType::Num(rn) = right {
                                if ln > rn { return LiteralType::True } return LiteralType::False
                            }
                        }
                        panic!("Expected a number")
                    },
                    TokenType::GreaterEqual => {
                        if let LiteralType::Num(ln) = left {
                            if let LiteralType::Num(rn) = right {
                                if ln >= rn { return LiteralType::True } return LiteralType::False
                            }
                        }
                        panic!("Expected a number")
                    },
                    TokenType::Less => {
                        if let LiteralType::Num(ln) = left {
                            if let LiteralType::Num(rn) = right {
                                if ln < rn { return LiteralType::True } return LiteralType::False
                            }
                        }
                        panic!("Expected a number")
                    },
                    TokenType::LessEqual => {
                        if let LiteralType::Num(ln) = left {
                            if let LiteralType::Num(rn) = right {
                                if ln <= rn { return LiteralType::True } return LiteralType::False
                            }
                        }
                        panic!("Expected a number")
                    },
                    TokenType::BangEqual => {
                        if !self.is_equal(&left, &right) { return LiteralType::True } return LiteralType::False
                    },
                    TokenType::EqualEqual => {
                        if self.is_equal(&left, &right) { return LiteralType::True } return LiteralType::False
                    },
                    TokenType::Minus => {
                        if let LiteralType::Num(ln) = left {
                            if let LiteralType::Num(rn) = right {
                                return LiteralType::Num(ln - rn);
                            };
                        };
                        panic!("Expected a number")
                    },
                    TokenType::Plus => {
                        if let LiteralType::Num(ln) = left {
                            if let LiteralType::Num(rn) = right {
                                return LiteralType::Num(ln + rn);
                            }
                        }
                        if let LiteralType::Str(ls) = left {
                            if let LiteralType::Str(rs) = right {
                                return LiteralType::Str(ls + &rs);
                            }
                        }
                        return LiteralType::Nil;
                    },
                    TokenType::FSlash => {
                        if let LiteralType::Num(ln) = left {
                            if let LiteralType::Num(rn) = right {
                                return LiteralType::Num(ln / rn);
                            }
                        }
                        panic!("Expected a number")
                    },
                    TokenType::Asterisk => {
                        if let LiteralType::Num(ln) = left {
                            if let LiteralType::Num(rn) = right {
                                return LiteralType::Num(ln * rn);
                            };
                        };
                        panic!("Expected a number")
                    },
                    _ => return LiteralType::Nil,
                };
            }
            _ => panic!("Expected a binary expression")
        }
    }

    fn visit_unary_expr(&mut self, expr: &Expr) -> LiteralType {
        match expr {
            Expr::Unary { operator, right } => {
                let right_value = self.unbox(right.clone());
                let right = self.evaluate(&right_value);
                match operator.token_type {
                    TokenType::Bang => if self.is_truthy(&right) { LiteralType::False } else { LiteralType::True },
                    TokenType::Minus => right,
                    _ => panic!("Expected a minus"),
                };
                return LiteralType::Nil
            },
            _ => panic!("Expected a unary expression")
        }
    }
}

impl stmt::Visitor<()> for Interpreter {
    fn visit_expression_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expression { expression } => {
                self.evaluate(&expression.clone());
            },
            Stmt::Print { .. } => panic!("Expected an expression statement")
        }
    }

    fn visit_print_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Print { expression } => {
                let value = self.evaluate(&expression.clone());
                println!("{}", self.stringify(value));
            },
            Stmt::Expression { .. } => panic!("Exepcted a print statement")
        }
    }
}