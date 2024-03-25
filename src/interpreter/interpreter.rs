use std::{
    cell::RefCell,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH}
};

use crate::{
    arithmetic,
    comparison,
    environment::Environment,
    expr::{self, Expr, LiteralType, Value},
    stmt::{self, Callable, Function, NativeFunction, Stmt},
    tokens::{Token, TokenType}
};

pub struct Interpreter {
    pub globals: Environment,
    environment: Environment
}

impl Interpreter {
    pub fn new() -> Self {
        let mut globals = Environment::new(None);


        let clock = NativeFunction::new(
            "clock".to_string(),
            0,
            |_, _| {
                Value::Literal(LiteralType::Num(
                    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64()
                ))
            }
        );

        globals.define("clock".to_string(), Value::NativeFunc(clock));

        Self {
            globals: globals.clone(),
            environment: globals
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

    pub fn evaluate(&mut self, expr: &Expr) -> Value {
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

    fn is_truthy(&mut self, object: &Value) -> bool {
        match object {
            Value::Literal(literal) => !matches!(literal, LiteralType::Nil | LiteralType::False),
            _ => panic!("Expected a literal value")
        }
        
    }

    fn is_equal(&mut self, a: &Value, b: &Value) -> bool {
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
            LiteralType::Nil => "nil".to_string()
        }
    }
}

impl expr::ExprVisitor<Value> for Interpreter {
    fn visit_literal_expr(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Literal { value } => Value::Literal(value.clone()),
            _ => panic!("Expected a literal value")
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Grouping { expression } => {
                let expression_value = self.unbox(expression.clone());
                self.evaluate(&expression_value)
            },
            _ => panic!("Expected a group expression")
        }
    }

    fn visit_binary_expr(&mut self, expr: &Expr) -> Value {
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
                        if !self.is_equal(&left, &right) { return Value::Literal(LiteralType::True) } return Value::Literal(LiteralType::False)
                    },
                    TokenType::EqualEqual => {
                        if self.is_equal(&left, &right) { return Value::Literal(LiteralType::True) } return Value::Literal(LiteralType::False)
                    },
                    TokenType::Minus => {
                        arithmetic!( - ; left ; right );
                        panic!("Expected a number")
                    },
                    TokenType::Plus => {
                        arithmetic!( + ; left ; right );
                        if let Value::Literal(LiteralType::Str(ls)) = left {
                            if let Value::Literal(LiteralType::Str(rs)) = right {
                                return Value::Literal(LiteralType::Str(ls + &rs));
                            }
                        }
                        return Value::Literal(LiteralType::Nil);
                    },
                    TokenType::FSlash => {
                        arithmetic!( / ; left ; right );
                        panic!("Expected a number")
                    },
                    TokenType::Asterisk => {
                        arithmetic!( * ; left ; right );
                        panic!("Expected a number")
                    },
                    _ => return Value::Literal(LiteralType::Nil),
                };
            }
            _ => panic!("Expected a binary expression")
        }
    }

    fn visit_unary_expr(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Unary { operator, right } => {
                let right_value = self.unbox(*right.clone());
                let right = self.evaluate(&right_value);
                match operator.token_type {
                    TokenType::Bang => {
                            if self.is_truthy(&right) {
                                Value::Literal(LiteralType::False)
                            } else {
                                Value::Literal(LiteralType::True)
                            }
                        }
                    TokenType::Minus => {
                        if let Value::Literal(LiteralType::Num(n)) = right {
                            return Value::Literal(LiteralType::Num(-n));
                        }
                        panic!("Couldn't negate number??")
                    },
                    _ => panic!("Expected a minus"),
                };
                Value::Literal(LiteralType::Nil)
            },
            _ => panic!("Expected a unary expression")
        }
    }

    fn visit_var_expr(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Var { name } => {
                match self.environment.get(name.clone()) {
                    Value::Literal(l) => Value::Literal(l),
                    Value::Func(f) => Value::Func(f),
                    Value::NativeFunc(nf) => Value::NativeFunc(nf),
                }
            },
            _ => panic!("Expected a variable expression")
        }
    }
    
    fn visit_assign_expr(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Assign { name, value } => {
                let assign_value = self.evaluate(value);
                self.environment.assign(name.clone(), assign_value.clone());
                assign_value
            }
            _ => panic!("Expected an assignment expression")
        }
    }
    
    fn visit_logical_expr(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Logical { left, operator, right } => {
                let left = self.evaluate(left);
                if operator.token_type == TokenType::Or {
                    if self.is_truthy(&left) { return left }
                    if !self.is_truthy(&left) { return left };
                    
                }
                self.evaluate(right)
            }
            _ => panic!("Expected a logical expression")
        }
    }
    
    fn visit_alteration_expr(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Alteration { name, alteration_type } => {
                let curr_value = self.environment.get(name.clone());
                match alteration_type {
                    TokenType::Incr => {
                        if let Value::Literal(LiteralType::Num(n)) = curr_value {
                            self.environment.assign(name.clone(), Value::Literal(LiteralType::Num(n + 1.0)));
                            return Value::Literal(LiteralType::Num(n + 1.0));
                        }
                        panic!("Why is the current value not a number?? 1")
                    },
                    TokenType::Decr => {
                        if let Value::Literal(LiteralType::Num(n)) = curr_value {
                            self.environment.assign(name.clone(), Value::Literal(LiteralType::Num(n - 1.0)));
                            return Value::Literal(LiteralType::Num(n - 1.0));
                        }
                        panic!("Why is the current value not a number?? 2")
                    },
                    _ => panic!("Why is it not an increment or decrement token?")
                }
            },
            _ => panic!("Expected an alteration expression")
        }
    }
    
    fn visit_call_expr(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Call { callee, paren: _paren, arguments: arguements } => {
                let callee = self.evaluate(callee);
                
                let mut args: Vec<Value> = Vec::new();
                for argument in arguements {
                    args.push(self.evaluate(argument));
                }

                match callee {
                    Value::Func(f) => {
                        assert!(!args.len() != f.arity(), "expected {} arguments but got {}.", args.len(), f.arity());
                        f.call(self, args)
                    },
                    Value::NativeFunc(nf) => {
                        assert!(!args.len() != nf.arity(), "expected {} arguments but got {}.", args.len(), nf.arity());
                        nf.call(self, args)
                    },
                    _ => panic!("Can only call functions and classes"),
                }
            },
            _ => panic!("Expected a call expression")
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
                match value {
                    Value::Literal(literal) => println!("{}", self.stringify(literal)),
                    _ => panic!("Expected a literal value")
                }
                
            },
            _ => panic!("Exepcted a print statement")
        }
    }

    fn visit_var_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Var { name, initializer } => {
                let mut value = Value::Literal(LiteralType::Nil);

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
                    self.execute(then_branch);
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
    
    fn visit_function_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Function { name, params: _, body: _ } => {
                let function = Value::Func(Function::new(stmt.clone()));
                self.environment.define(name.lexeme.clone(), function);
            },
            _ => panic!("Expected a function statement")
        }
    }
}