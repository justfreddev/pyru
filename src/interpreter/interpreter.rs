use crate::{
    environment::Environment,
    expr::{self, Expr, LiteralType},
    stmt::{self, Stmt},
    tokens::{Token, TokenType}
};

pub struct Interpreter {
    environment: Environment
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(None)
        }
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

    }

    pub fn evaluate(&mut self, expr: &Expr) -> LiteralType {
        expr.accept(self)
    }

    pub fn execute(&mut self, stmt: &Stmt) {
        stmt.accept(self);
    }

    pub fn execute_block(&mut self, statements: Vec<Stmt>, environment: Environment) {
        let previous = self.environment.clone();

        self.environment = environment;

        for statement in statements {
            self.execute(&statement);
        }

        self.environment = previous;
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

    fn visit_variable_expr(&mut self, expr: &Expr) -> LiteralType {
        match expr {
            Expr::Var { name } => self.environment.get(name.clone()),
            _ => panic!("Expected a variable expression")
        }
    }
    
    fn visit_assign_expr(&mut self, expr: &Expr) -> LiteralType {
        match expr {
            Expr::Assign { name, value } => {
                let assign_value = self.evaluate(value);
                self.environment.assign(name.clone(), &assign_value);
                return assign_value;
            }
            _ => panic!("Expected an assignment expression")
        }
    }
}

impl stmt::Visitor<()> for Interpreter {
    fn visit_expression_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expression { expression } => {
                self.evaluate(&expression.clone());
            },
            _ => panic!("Expected an expression statement"),
        }
    }

    fn visit_print_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Print { expression } => {
                let value = self.evaluate(&expression.clone());
                println!("{}", self.stringify(value));
            },
            _ => panic!("Exepcted a print statement")
        }
    }

    fn visit_var_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Var { name, initializer } => {
                let mut value = LiteralType::Nil;

                if let Some(initializer_expr) = initializer {
                    value = self.evaluate(initializer_expr);
                }

                self.environment.define(name.lexeme.clone(), value);

            },
            _ => panic!("Expected a var statement")
        }
    }

    fn visit_block_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Block { statements } => self.execute_block(statements.clone(), Environment::new(Some(self.environment.clone()))),
            _ => panic!("Expected a block statement")
        }
    }
}