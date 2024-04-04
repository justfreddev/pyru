use std::{
    cell::RefCell,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH}
};

use crate::{
    arithmetic, comparison, environment::Environment, expr::{self, Expr, LiteralType, Value}, getresult, returncheck, stmt::{self, Callable, Function, NativeFunction, Stmt}, tokens::{Token, TokenType}
};

pub struct Interpreter {
    pub globals: Environment,
    pub environment: Environment
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
            let _ = self.execute(&stmt);
        }
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Result<(), Value> {
        let result_result = expr.accept_expr(self);
        let result = getresult!(result_result);
        Err(result)
    }

    pub fn execute(&mut self, stmt: &Stmt) -> Result<(), Value> {
        stmt.accept_stmt(self)

    }

    pub fn execute_block(&mut self, statements: Vec<Stmt>, environment: Environment) -> Result<(), Value> {
        let previous = self.environment.clone();

        self.environment = environment;

        for statement in statements {
            match self.execute(&statement) {
                Err(v) => {
                    match statement {
                        Stmt::Return { .. } => return Err(v),
                        _ => {},
                    }
                },
                Ok(_) => {} 
            }
        }
        self.environment = previous;
        Ok(())
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

impl expr::ExprVisitor<Result<(), Value>> for Interpreter {
    fn visit_literal_expr(&mut self, expr: &Expr) -> Result<(), Value> {
        match expr {
            Expr::Literal { value } => Err(Value::Literal(value.clone())),
            _ => panic!("Expected a literal value")
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) -> Result<(), Value> {
        match expr {
            Expr::Grouping { expression } => {
                let expression_value = self.unbox(expression.clone());
                self.evaluate(&expression_value)
            },
            _ => panic!("Expected a group expression")
        }
    }

    fn visit_binary_expr(&mut self, expr: &Expr) -> Result<(), Value> {
        match expr {
            Expr::Binary { left, operator, right } => {
                let left_value = self.unbox(left.clone());
                let left_result = self.evaluate(&left_value);
                let right_value = self.unbox(right.clone());
                let right_result = self.evaluate(&right_value);
                let left = getresult!(left_result);
                let right = getresult!(right_result);
                

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
                        if !self.is_equal(&left, &right) { return Err(Value::Literal(LiteralType::True)) } return Err(Value::Literal(LiteralType::False))
                    },
                    TokenType::EqualEqual => {
                        if self.is_equal(&left, &right) { return Err(Value::Literal(LiteralType::True)) } return Err(Value::Literal(LiteralType::False))
                    },
                    TokenType::Minus => {
                        arithmetic!( - ; left ; right );
                        panic!("Expected a number")
                    },
                    TokenType::Plus => {
                        arithmetic!( + ; left ; right );
                        panic!("Expected a number")
                    },
                    TokenType::FSlash => {
                        arithmetic!( / ; left ; right );
                        panic!("Expected a number")
                    },
                    TokenType::Asterisk => {
                        arithmetic!( * ; left ; right );
                        panic!("Expected a number")
                    },
                    _ => return Err(Value::Literal(LiteralType::Nil)),
                };
            }
            _ => panic!("Expected a binary expression")
        }
    }

    fn visit_unary_expr(&mut self, expr: &Expr) -> Result<(), Value> {
        match expr {
            Expr::Unary { operator, right } => {
                let right_value = self.unbox(*right.clone());
                let right_result = self.evaluate(&right_value);
                let right = getresult!(right_result);
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
                            return Err(Value::Literal(LiteralType::Num(-n)));
                        }
                        panic!("Couldn't negate number??")
                    },
                    _ => panic!("Expected a minus"),
                };
                Err(Value::Literal(LiteralType::Nil))
            },
            _ => panic!("Expected a unary expression")
        }
    }

    fn visit_var_expr(&mut self, expr: &Expr) -> Result<(), Value> {
        match expr {
            Expr::Var { name } => {
                match self.environment.get(name.clone()) {
                    Value::Literal(l) => Err(Value::Literal(l)),
                    Value::Func(f) => Err(Value::Func(f)),
                    Value::NativeFunc(nf) => Err(Value::NativeFunc(nf)),
                }
            },
            _ => panic!("Expected a variable expression")
        }
    }
    
    fn visit_assign_expr(&mut self, expr: &Expr) -> Result<(), Value> {
        match expr {
            Expr::Assign { name, value } => {
                let assign_value_result = self.evaluate(value);
                let assign_value = getresult!(assign_value_result);
                self.environment.assign(name.clone(), assign_value.clone());
                Err(assign_value)
            }
            _ => panic!("Expected an assignment expression")
        }
    }
    
    fn visit_logical_expr(&mut self, expr: &Expr) -> Result<(), Value> {
        match expr {
            Expr::Logical { left, operator, right } => {
                let left_result = self.evaluate(left);
                let left = getresult!(left_result);
                if operator.token_type == TokenType::Or {
                    if self.is_truthy(&left) { return Err(left) }
                    if !self.is_truthy(&left) { return Err(left) };
                    
                }
                self.evaluate(right)
            }
            _ => panic!("Expected a logical expression")
        }
    }
    
    fn visit_alteration_expr(&mut self, expr: &Expr) -> Result<(), Value> {
        match expr {
            Expr::Alteration { name, alteration_type } => {
                let curr_value = self.environment.get(name.clone());
                match alteration_type {
                    TokenType::Incr => {
                        if let Value::Literal(LiteralType::Num(n)) = curr_value {
                            self.environment.assign(name.clone(), Value::Literal(LiteralType::Num(n + 1.0)));
                            return Err(Value::Literal(LiteralType::Num(n + 1.0)));
                        }
                        panic!("Why is the current value not a number?? 1")
                    },
                    TokenType::Decr => {
                        if let Value::Literal(LiteralType::Num(n)) = curr_value {
                            self.environment.assign(name.clone(), Value::Literal(LiteralType::Num(n - 1.0)));
                            return Err(Value::Literal(LiteralType::Num(n - 1.0)));
                        }
                        panic!("Why is the current value not a number?? 2")
                    },
                    _ => panic!("Why is it not an increment or decrement token?")
                }
            },
            _ => panic!("Expected an alteration expression")
        }
    }
    
    fn visit_call_expr(&mut self, expr: &Expr) -> Result<(), Value> {
        println!("visit_call_expr()");
        match expr {
            Expr::Call { callee, paren: _paren, arguments: arguements } => {
                let callee_result = self.evaluate(callee);
                let callee = getresult!(callee_result);
                
                let mut args: Vec<Value> = Vec::new();
                for argument in arguements {
                    let arg_result = self.evaluate(argument);
                    let arg = getresult!(arg_result);
                    args.push(arg);
                }

                match callee {
                    Value::Func(f) => {
                        assert!(!args.len() != f.arity(), "expected {} arguments but got {}.", args.len(), f.arity());
                        let res = f.call(self, args);
                        Err(res)
                    },
                    Value::NativeFunc(nf) => {
                        assert!(!args.len() != nf.arity(), "expected {} arguments but got {}.", args.len(), nf.arity());
                        Err(nf.call(self, args))
                    },
                    _ => panic!("Can only call functions and classes"),
                }
            },
            _ => panic!("Expected a call expression")
        }
    }
}

impl stmt::StmtVisitor<Result<(), Value>> for Interpreter {
    fn visit_expression_stmt(&mut self, stmt: &Stmt) -> Result<(), Value> {
        match stmt {
            Stmt::Expression { expression } => {
                return self.evaluate(&expression.clone());
            },
            _ => panic!("Expected an expression statement"),
        }
    }

    fn visit_print_stmt(&mut self, stmt: &Stmt) -> Result<(), Value> {
        match stmt {
            Stmt::Print { expression } => {
                let value_result = self.evaluate(&expression.clone());
                let value = getresult!(value_result);
                match value {
                    Value::Literal(literal) => {
                        println!("{}", self.stringify(literal));
                        Ok(())
                    },
                    _ => panic!("Expected a literal value")
                }
                
            },
            _ => panic!("Exepcted a print statement")
        }
    }

    fn visit_var_stmt(&mut self, stmt: &Stmt) -> Result<(), Value> {
        match stmt {
            Stmt::Var { name, initializer } => {
                let mut value = Value::Literal(LiteralType::Nil);

                if let Some(initializer_expr) = initializer {
                    let evaluation_result = self.evaluate(initializer_expr);
                    value = getresult!(evaluation_result);
                }

                self.environment.define(name.lexeme.clone(), value);
                Ok(())
            },
            _ => panic!("Expected a var statement")
        }
    }

    fn visit_block_stmt(&mut self, stmt: &Stmt) -> Result<(), Value> {
        match stmt {
            Stmt::Block { statements } => {
                let _ = self.execute_block(
                    statements.clone(),
                    Environment::new(Some(Rc::new(RefCell::new(self.environment.clone()))))
                );
                Ok(())
            },
            _ => panic!("Expected a block statement")
        }
    }
    
    fn visit_if_stmt(&mut self, stmt: &Stmt) -> Result<(), Value> {
        match stmt {
            Stmt::If { condition, then_branch, else_branch } => {
                let condition_evaluation_result = &self.evaluate(condition);
                let condition_evaluation = getresult!(condition_evaluation_result);
                let else_branch_copy = else_branch.clone();
                if self.is_truthy(condition_evaluation) {
                    let _ = self.execute(then_branch);
                } else if else_branch.is_some() {
                    let _ = self.execute(&else_branch_copy.unwrap());
                }
                Ok(())
            }
            _ => panic!("Expected an if statement")
        }
        
    }
    
    fn visit_while_stmt(&mut self, stmt: &Stmt) -> Result<(), Value> {
        match stmt {
            Stmt::While { condition, body } => {
                let body = *body.clone();
                let mut condition_evaluation_result = self.evaluate(condition);
                let mut condition_evaluation = getresult!(condition_evaluation_result);
                let mut condition_result = self.is_truthy(&condition_evaluation);
                while condition_result {
                    let _ = self.execute(&body);
                    condition_evaluation_result = self.evaluate(condition);
                    condition_evaluation = getresult!(condition_evaluation_result);
                    condition_result = self.is_truthy(&condition_evaluation);
                }
                Ok(())
            }
            _ => panic!("Expected a while statement")
        }
    }
    
    fn visit_for_stmt(&mut self, stmt: &Stmt) -> Result<(), Value> {
        match stmt {
            Stmt::For { initializer, condition, increment, body } => {
                if initializer.is_some() {
                    let x = self.execute(*&initializer.as_ref().unwrap());
                    returncheck!(x);
                }
                let mut condition_evaluation_result = self.evaluate(condition);
                let mut condition_evaluation = getresult!(condition_evaluation_result);
                let mut condition_result = self.is_truthy(&condition_evaluation);
                while condition_result {
                    let y = self.execute(&body);
                    returncheck!(y);
                    if increment.is_some() {
                        let _ = self.evaluate(*&increment.as_ref().unwrap());
                    }
                    condition_evaluation_result = self.evaluate(condition);
                    condition_evaluation = getresult!(condition_evaluation_result);
                    condition_result = self.is_truthy(&condition_evaluation);
                }
                Ok(())
            },
            _ => panic!("Expected a for statement")
        }
    }
    
    fn visit_function_stmt(&mut self, stmt: &Stmt) -> Result<(), Value> {
        println!("visit_function_stmt()");
        match stmt {
            Stmt::Function { name, params: _, body: _ } => {
                self.environment.define(name.lexeme.clone(), Value::Literal(LiteralType::Nil));
                let function = Value::Func(Function::new(stmt.clone()));
                self.environment.assign(name.clone(), function);
                Ok(())
            },
            _ => panic!("Expected a function statement")
        }
    }
    
    fn visit_return_stmt(&mut self, stmt: &Stmt) -> Result<(), Value> {
        match stmt {
            Stmt::Return { keyword: _, value } => {
                let mut return_value = Value::Literal(LiteralType::Nil);
                if value.is_some() {
                    let return_value_result = self.evaluate(value.as_ref().unwrap());
                    return_value = getresult!(return_value_result);
                }
                Err(return_value)
            },
            _ => panic!("Expected a return statement")
        }
    }
}