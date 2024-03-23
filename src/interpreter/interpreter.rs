use std::{ rc::Rc, cell::RefCell };

use crate::{
    arithmetic,
    comparison,
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
        expr.accept_expr(self)
    }

    pub fn execute(&mut self, stmt: &Stmt) {
        stmt.accept_stmt(self);
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

impl expr::ExprVisitor<LiteralType> for Interpreter {
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
                        comparison!( > ; left ; right );
                        panic!("Expected a number")
                    },
                    TokenType::GreaterEqual => {
                        comparison!( >= ; left ; right );
                        panic!("Expected a number")
                    },
                    TokenType::Less => {
                        comparison!( < ; left ; right );
                        panic!("Expected a number")
                    },
                    TokenType::LessEqual => {
                        comparison!( <= ; left ; right );
                        panic!("Expected a number")
                    },
                    TokenType::BangEqual => {
                        if !self.is_equal(&left, &right) { return LiteralType::True } return LiteralType::False
                    },
                    TokenType::EqualEqual => {
                        if self.is_equal(&left, &right) { return LiteralType::True } return LiteralType::False
                    },
                    TokenType::Minus => {
                        arithmetic!( - ; left ; right );
                        panic!("Expected a number")
                    },
                    TokenType::Plus => {
                        arithmetic!( + ; left ; right );
                        if let LiteralType::Str(ls) = left {
                            if let LiteralType::Str(rs) = right {
                                return LiteralType::Str(ls + &rs);
                            }
                        }
                        return LiteralType::Nil;
                    },
                    TokenType::FSlash => {
                        arithmetic!( / ; left ; right );
                        panic!("Expected a number")
                    },
                    TokenType::Asterisk => {
                        arithmetic!( * ; left ; right );
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
                let right_value = self.unbox(*right.clone());
                let right = self.evaluate(&right_value);
                match operator.token_type {
                    TokenType::Bang => if self.is_truthy(&right) { LiteralType::False } else { LiteralType::True },
                    TokenType::Minus => if let LiteralType::Num(n) = right { return LiteralType::Num(-n) } else { panic!("Couldn't negate number??") },
                    _ => panic!("Expected a minus"),
                };
                return LiteralType::Nil
            },
            _ => panic!("Expected a unary expression")
        }
    }

    fn visit_var_expr(&mut self, expr: &Expr) -> LiteralType {
        match expr {
            Expr::Var { name } => self.environment.get(name.clone()),
            _ => panic!("Expected a variable expression")
        }
    }
    
    fn visit_assign_expr(&mut self, expr: &Expr) -> LiteralType {
        match expr {
            Expr::Assign { name, value } => {
                let assign_value = self.evaluate(value);
                self.environment.assign(name.clone(), assign_value.clone());
                assign_value
            }
            _ => panic!("Expected an assignment expression")
        }
    }
    
    fn visit_logical_expr(&mut self, expr: &Expr) -> LiteralType {
        match expr {
            Expr::Logical { left, operator, right } => {
                let left = self.evaluate(left);
                if operator.token_type == TokenType::Or {
                    if self.is_truthy(&left) {
                        return left
                    } else {
                        if !self.is_truthy(&left) { return left };
                    }
                }
                self.evaluate(right)
            }
            _ => panic!("Expected a logical expression")
        }
    }
}

impl stmt::StmtVisitor<()> for Interpreter {
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
            Stmt::Block { statements } => self.execute_block(statements.clone(), Environment::new(Some(Rc::new(RefCell::new(self.environment.clone()))))),
            _ => panic!("Expected a block statement")
        }
    }
    
    fn visit_if_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::If { condition, then_branch, else_branch } => {
                let condition_evaluation = &self.evaluate(condition);
                let else_branch_copy = else_branch.clone();
                if self.is_truthy(condition_evaluation) {
                    self.execute(&then_branch);
                } else if else_branch.is_some() {
                    self.execute(&else_branch_copy.unwrap());
                }
            }
            _ => panic!("Expected an if statement")
        }
        
    }
    
    fn visit_while_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::While { condition, body } => {
                let body = *body.clone();
                let mut condition_evaluation = self.evaluate(condition);
                let mut condition_result = self.is_truthy(&condition_evaluation);
                while condition_result {
                    self.execute(&body);
                    condition_evaluation = self.evaluate(condition);
                    condition_result = self.is_truthy(&condition_evaluation);
                }
            }
            _ => panic!("Expected a while statement")
        }
    }
}