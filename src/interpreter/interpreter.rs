use interpreter_v1::tokens::{Token, TokenType};

use crate::expr::{Expr, LiteralType, Visitor};

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

    pub fn interpret(&mut self, expression: Expr) {
        let result = self.evaluate(expression);
        match result {
            LiteralType::Str(s) => println!("{s}"),
            LiteralType::Num(n) => println!("{}", n.to_string()),
            LiteralType::True => println!("True"),
            LiteralType::False => println!("False"),
            LiteralType::Nil => println!("Nil"),
        }
    }

    pub fn evaluate(&mut self, expr: Expr) -> LiteralType {
        expr.accept(self)
    }

    fn unbox<T>(&mut self, value: Box<T>) -> T {
        *value
    }

    fn is_truthy(&mut self, object: LiteralType) -> bool {
        match object {
            LiteralType::Nil | LiteralType::False => false,
            _ => true,
        }
    }

    fn is_equal(&mut self, a: LiteralType, b: LiteralType) -> bool {
        a == b
    }
}

impl Visitor<LiteralType> for Interpreter {
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
                self.evaluate(expression_value)
            },
            _ => panic!("Expected a group expression")
        }
    }

    fn visit_binary_expr(&mut self, expr: &Expr) -> LiteralType {
        match expr {
            Expr::Binary { left, operator, right } => {
                let left_value = self.unbox(left.clone());
                let left = self.evaluate(left_value);
                let right_value = self.unbox(right.clone());
                let right = self.evaluate(right_value);
                match operator.token_type {
                    TokenType::Greater => {
                        if let LiteralType::Num(ln) = left {
                            if let LiteralType::Num(rn) = right {
                                match ln > rn {
                                    true => return LiteralType::True,
                                    false => return LiteralType::False
                                }
                            }
                        }
                        panic!("Expected a number 6")
                    },
                    TokenType::GreaterEqual => {
                        if let LiteralType::Num(ln) = left {
                            if let LiteralType::Num(rn) = right {
                                match ln >= rn {
                                    true => return LiteralType::True,
                                    false => return LiteralType::False
                                }
                            }
                        }
                        panic!("Expected a number 7")
                    },
                    TokenType::Less => {
                        if let LiteralType::Num(ln) = left {
                            if let LiteralType::Num(rn) = right {
                                match ln < rn {
                                    true => return LiteralType::True,
                                    false => return LiteralType::False
                                }
                            }
                        }
                        panic!("Expected a number 8")
                    },
                    TokenType::LessEqual => {
                        if let LiteralType::Num(ln) = left {
                            if let LiteralType::Num(rn) = right {
                                match ln <= rn {
                                    true => return LiteralType::True,
                                    false => return LiteralType::False
                                }
                            }
                        }
                        panic!("Expected a number 6")
                    },
                    TokenType::BangEqual => {
                        
                        match !self.is_equal(left, right) {
                            true => return LiteralType::True,
                            false => return LiteralType::False
                        }
                    },
                    TokenType::EqualEqual => {
                        match self.is_equal(left, right) {
                            true => return LiteralType::True,
                            false => return LiteralType::False
                        }
                    },
                    TokenType::Minus => {
                        if let LiteralType::Num(ln) = left {
                            if let LiteralType::Num(rn) = right {
                                return LiteralType::Num(ln - rn);
                            };
                        };
                        panic!("Expected a number 3")
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
                        panic!("Expected a number 4")
                    },
                    TokenType::Asterisk => {
                        if let LiteralType::Num(ln) = left {
                            if let LiteralType::Num(rn) = right {
                                return LiteralType::Num(ln * rn);
                            };
                        };
                        panic!("Expected a number 5")
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
                let right = self.evaluate(right_value);
                match operator.token_type {
                    TokenType::Bang => match !self.is_truthy(right) {
                        true => LiteralType::True,
                        false => LiteralType::False,
                    },
                    TokenType::Minus => right,
                    _ => panic!("Expected a minus"),
                };
                return LiteralType::Nil
            },
            _ => panic!("Expected a unary expression")
        }
    }
}