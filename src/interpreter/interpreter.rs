use std::{
    cell::RefCell,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH}
};

use crate::{
    alteration,
    arithmetic,
    callable::{Callable, Func, NativeFunc},
    comparison,
    enviromnent::{Environment, GlobalEnvironment, LocalEnvironment},
    error::InterpreterError,
    expr::{self, Expr},
    stmt::{self, Stmt},
    token::TokenType,
    value::{LiteralType, Value}
};

pub struct Interpreter {
    pub globals: Rc<RefCell<GlobalEnvironment>>,
    pub environment: Rc<RefCell<dyn Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let global = Rc::new(RefCell::new(GlobalEnvironment::new()));

        let clock = NativeFunc::new(
            "clock".to_string(),
            0,
            |_, _| {
                Ok(Value::Literal(LiteralType::Num(
                    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64()
                )))
            }
        );

        global.borrow_mut().define("clock".to_string(), Value::NativeFunction(clock));

        Self {
            globals: Rc::clone(&global),
            environment: Rc::clone(&global) as Rc<RefCell<dyn Environment>>
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), InterpreterError> {
        for stmt in statements {
            let _ = match self.execute(&stmt) {
                Ok(()) => {},
                Err(r) => match r {
                    Ok(_) => {},
                    Err(e) => return Err(e)
                }
            };
        }
        Ok(())
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, InterpreterError> {
        match expr.accept_expr(self) {
            Ok(v) => Ok(v),
            Err(e) => Err(e)
        }
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), Result<Value, InterpreterError>> {
        stmt.accept_stmt(self)
    }

    pub fn execute_block(&mut self, statements: Vec<Stmt>, environment: Rc<RefCell<LocalEnvironment>>) -> Result<(), Result<Value, InterpreterError>> {
        let previous = Rc::clone(&self.environment);

        self.environment = Rc::clone(&environment) as Rc<RefCell<dyn Environment>>;

        for statement in statements {
            match self.execute(&statement) {
                Ok(_) => {},
                Err(r) => match r {
                    Ok(v) => {self.environment = previous; return Err(Ok(v))},
                    Err(e) => return Err(Err(e))
                }
            }
        }
        self.environment = previous;
        Ok(())
    }

    fn is_truthy(&mut self, object: &Value) -> Result<bool, InterpreterError> {
        match object {
            Value::Literal(literal) => Ok(!matches!(literal, LiteralType::Nil | LiteralType::False)),
            _ => Err(InterpreterError::ExpectedLiteralValue)
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
            },
            LiteralType::Str(s) => s,
            LiteralType::True => "true".to_string(),
            LiteralType::False => "false".to_string(),
            LiteralType::Nil => "nil".to_string()
        }
    }


}
impl expr::ExprVisitor<Result<Value, InterpreterError>> for Interpreter {
    fn visit_literal_expr(&mut self, expr: &Expr) -> Result<Value, InterpreterError> {
        match expr {
            Expr::Literal { value } => return Ok(Value::Literal(value.clone())),
            _ => Err(InterpreterError::ExpectedLiteralValue)
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) -> Result<Value, InterpreterError> {
        match expr {
            Expr::Grouping { expression } => self.evaluate(expression),
            _ => Err(InterpreterError::ExpectedGroupExpression)
        }
    }

    fn visit_unary_expr(&mut self, expr: &Expr) -> Result<Value, InterpreterError> {
        match expr {
            Expr::Unary { operator, right } => {
                let right = match self.evaluate(right) {
                    Ok(v) => v,
                    Err(e) => return Err(e)
                };

                match operator.token_type {
                    TokenType::Bang => {
                        match self.is_truthy(&right) {
                            Ok(v) => {
                                if v {
                                    return Ok(Value::Literal(LiteralType::False));
                                }
                                return Ok(Value::Literal(LiteralType::True));
                            },
                            Err(e) => return Err(e)
                        }
                    },
                    TokenType::Minus => {
                        if let Value::Literal(LiteralType::Num(n)) = right {
                            return Ok(Value::Literal(LiteralType::Num(-n)));
                        }
                        return Err(InterpreterError::UnableToNegate)
                    },
                    _ => return Err(InterpreterError::ExpectedMinus)
                }
            },
            _ => return Err(InterpreterError::ExpectedUnaryExpression)
        }
    }

    fn visit_binary_expr(&mut self, expr: &Expr) -> Result<Value, InterpreterError> {
        match expr {
            Expr::Binary { left, operator, right } => {
                let left = match self.evaluate(&left) {
                    Ok(v) => v,
                    Err(e) => return Err(e)
                };
                let right = match self.evaluate(&right) {
                    Ok(v) => v,
                    Err(e) => return Err(e)
                };

                match operator.token_type {
                    TokenType::Greater => {
                        comparison!( > ; left ; right);
                        return Err(InterpreterError::ExpectedNumber)
                    },
                    TokenType::GreaterEqual => {
                        comparison!( >= ; left ; right);
                        return Err(InterpreterError::ExpectedNumber)
                    },
                    TokenType::Less => {
                        comparison!( < ; left ; right);
                        return Err(InterpreterError::ExpectedNumber)
                    },
                    TokenType::LessEqual => {
                        comparison!( <= ; left ; right);
                        return Err(InterpreterError::ExpectedNumber)
                    },
                    TokenType::BangEqual => {
                        if !self.is_equal(&left, &right) {
                            return Ok(Value::Literal(LiteralType::True))
                        }
                        return Ok(Value::Literal(LiteralType::False))
                    },
                    TokenType::EqualEqual => {
                        if self.is_equal(&left, &right) {
                            return Ok(Value::Literal(LiteralType::True))
                        }
                        return Ok(Value::Literal(LiteralType::False))
                    },
                    TokenType::Plus => {
                        arithmetic!( + ; left ; right );
                        return Err(InterpreterError::ExpectedNumber)
                    },
                    TokenType::Minus => {
                        arithmetic!( - ; left ; right);
                        return Err(InterpreterError::ExpectedNumber)
                    },
                    TokenType::FSlash => {
                        arithmetic!( / ; left ; right);
                        return Err(InterpreterError::ExpectedNumber)
                    },
                    TokenType::Asterisk => {
                        arithmetic!( * ; left ; right);
                        return Err(InterpreterError::ExpectedNumber)
                    },
                    _ => return Err(InterpreterError::ExpectedValidBinaryOperator)
                }
            },
            _ => return Err(InterpreterError::ExpectedBinaryExpression)
        }
    }

    fn visit_var_expr(&mut self, expr: &Expr) -> Result<Value, InterpreterError> {
        match expr {
            Expr::Var { name } => {
                self.environment.borrow().get(name.clone())
            },
            _ => Err(InterpreterError::ExpectedVariableExpression)
        }
    }

    fn visit_assign_expr(&mut self, expr: &Expr) -> Result<Value, InterpreterError> {
        match expr {
            Expr::Assign { name, value } => {
                let value = match self.evaluate(value) {
                    Ok(v) => v,
                    Err(e) => return Err(e)
                };
                self.environment.borrow_mut().assign(name.clone(), value.clone())
            },
            _ => return Err(InterpreterError::ExpectedAssignmentExpression)
        }
    }

    fn visit_logical_expr(&mut self, expr: &Expr) -> Result<Value, InterpreterError> {
        match expr {
            Expr::Logical { left, operator, right } => {
                let left = match self.evaluate(left) {
                    Ok(v) => v,
                    Err(e) => return Err(e)
                };
                
                if operator.token_type == TokenType::Or {
                    match self.is_truthy(&left) {
                        Ok(v) => {
                            if v {
                                return Ok(left)
                            }
                        },
                        Err(e) => return Err(e)
                    }
                } else {
                    match self.is_truthy(&left) {
                        Ok(v) => {
                            if !v {
                                return Ok(left)
                            }
                        },
                        Err(e) => return Err(e)
                    }
                }

                self.evaluate(right)
            },
            _ => return Err(InterpreterError::ExpectedLogicalExpression)
        }
    }

    fn visit_alteration_expr(&mut self, expr: &Expr) -> Result<Value, InterpreterError> {
        match expr {
            Expr::Alteration { name, alteration_type } => {
                let curr_value = match self.environment.borrow().get(name.clone()) {
                    Ok(v) => v,
                    Err(e) => return Err(e)
                };

                match alteration_type {
                    TokenType::Incr => {
                        alteration!( self ;  + ; name ; curr_value);
                    },
                    TokenType::Decr => {
                        alteration!( self ; - ; name ; curr_value);
                    },
                    _ => return Err(InterpreterError::ExpectedAlterationToken)
                }
            },
            _ => return Err(InterpreterError::ExpectedAlterationExpression)
        }
    }

    fn visit_call_expr(&mut self, expr: &Expr) -> Result<Value, InterpreterError> {
        match expr {
            Expr::Call { callee, paren: _, arguments } => {
                let callee = match self.evaluate(callee) {
                    Ok(v) => v,
                    Err(e) => return Err(e)
                };

                let mut args: Vec<Value> = Vec::new();

                for argument in arguments {
                    let arg = match self.evaluate(argument) {
                        Ok(v) => v,
                        Err(e) => return Err(e)
                    };
                    args.push(arg);
                }

                match callee {
                    Value::Function(f) => {
                        if args.len() != f.arity {
                            return Err(
                                InterpreterError::ArgsDifferFromArity{
                                    args: args.len(),
                                    arity: f.arity
                                }
                            )
                        }
                        return f.call(self, args)
                    },
                    Value::NativeFunction(nf) => {
                        if args.len() != nf.arity {
                            return Err(
                                InterpreterError::ArgsDifferFromArity{
                                    args: args.len(),
                                    arity: nf.arity
                                }
                            )
                        }
                        return nf.call(self, args)
                    }
                    Value::Literal(_) => return Err(InterpreterError::ExpectedFunctionOrClass)
                }
            }
            _ => return Err(InterpreterError::ExpectedCallExpression)
        }
    }
}

impl stmt::StmtVisitor<Result<(), Result<Value, InterpreterError>>> for Interpreter {
    fn visit_expression_stmt(&mut self, stmt: &Stmt) -> Result<(), Result<Value, InterpreterError>> {
        match stmt {
            Stmt::Expression { expression } => {
                return match self.evaluate(&expression.clone()) {
                    Ok(_) => Ok(()), // MAY NEED TO CHANGE
                    Err(e) => return Err(Err(e))
                }
            },
            _ => return Err(Err(InterpreterError::ExpectedExpressionStatement))
        }
    }

    fn visit_print_stmt(&mut self, stmt: &Stmt) -> Result<(), Result<Value, InterpreterError>> {
        match stmt {
            Stmt::Print { expression } => {
                let value = match self.evaluate(expression) {
                    Ok(v) => v,
                    Err(e) => return Err(Err(e))
                };
                match value {
                    Value::Literal(literal) => {
                        println!("{}", self.stringify(literal));
                        Ok(())
                    },
                    _ => return Err(Err(InterpreterError::ExpectedToPrintLiteralValue))
                }
            },
            _ => return Err(Err(InterpreterError::ExpectedPrintStatement))
        }
    }

    fn visit_var_stmt(&mut self, stmt: &Stmt) -> Result<(), Result<Value, InterpreterError>> {
        match stmt {
            Stmt::Var { name, initializer } => {
                let mut value = Value::Literal(LiteralType::Nil);

                if let Some(initializer_expr) = initializer {
                    value = match self.evaluate(initializer_expr) {
                        Ok(v) => v,
                        Err(e) => return Err(Err(e))
                    };
                }
                self.environment.borrow_mut().define(name.lexeme.clone(), value);
                Ok(())
            },
            _ => Err(Err(InterpreterError::ExpectedVarStatement))
        }
    }

    fn visit_block_stmt(&mut self, stmt: &Stmt) -> Result<(), Result<Value, InterpreterError>> {
        match stmt {
            Stmt::Block { statements } => {
                let _ = match self.execute_block(
                    statements.clone(),
                    Rc::new(RefCell::new(LocalEnvironment::new(Some(self.environment.clone()))))
                ) {
                    Ok(_) => {},
                    Err(r) => match r {
                        Ok(v) => return Err(Ok(v)),
                        Err(e) => return Err(Err(e))
                    }
                };
                Ok(())
            },
            _ => Err(Err(InterpreterError::ExpectedBlockStatement))
        }
    }

    fn visit_if_stmt(&mut self, stmt: &Stmt) -> Result<(), Result<Value, InterpreterError>> {
        match stmt {
            Stmt::If { condition, then_branch, else_branch } => {
                let condition_evaluation = match self.evaluate(&condition) {
                    Ok(v) => v,
                    Err(e) => return Err(Err(e))
                };

                let condition_evaluation_result = match self.is_truthy(&condition_evaluation) {
                    Ok(v) => v,
                    Err(e) => return Err(Err(e))
                };
                if condition_evaluation_result {
                    let _ = match self.execute(&then_branch) {
                        Ok(_) => {},
                        Err(r) => match r {
                            Ok(v) => return Err(Ok(v)),
                            Err(e) => return Err(Err(e))
                        }
                    };
                } else if else_branch.is_some() {
                    let _ = match self.execute(&else_branch.as_ref().unwrap()) {
                        Ok(_) => {},
                        Err(r) => match r {
                            Ok(v) => return Err(Ok(v)),
                            Err(e) => return Err(Err(e))
                        }
                    };
                }
                Ok(())
            },
            _ => return Err(Err(InterpreterError::ExpectedIfStatement))
        }
    }

    fn visit_while_stmt(&mut self, stmt: &Stmt) -> Result<(), Result<Value, InterpreterError>> {
        match stmt {
            Stmt::While { condition, body } => {
                let body = *body.clone();
                let mut condition_evaluation = match self.evaluate(condition) {
                    Ok(v) => v,
                    Err(e) => return Err(Err(e))
                };
                let mut condition_result = match self.is_truthy(&condition_evaluation) {
                    Ok(v) => v,
                    Err(e) => return Err(Err(e))
                };
                while condition_result {
                    let _ = match self.execute(&body) {
                        Ok(_) => {},
                        Err(r) => match r {
                            Ok(v) => return Err(Ok(v)),
                            Err(e) => return Err(Err(e))
                        }
                    };
                    condition_evaluation = match self.evaluate(condition) {
                        Ok(v) => v,
                        Err(e) => return Err(Err(e))
                    };
                    condition_result = match self.is_truthy(&condition_evaluation) {
                        Ok(v) => v,
                        Err(e) => return Err(Err(e))
                    };
                }
                Ok(())
            }
            _ => panic!("Expected a while statement")
        }
    }

    fn visit_for_stmt(&mut self, stmt: &Stmt) -> Result<(), Result<Value, InterpreterError>> {
        match stmt {
            Stmt::For { initializer, condition, increment, body } => {
                if initializer.is_some() {
                    let _ = match self.execute(&initializer.as_ref().unwrap()) {
                        Ok(_) => {},
                        Err(r) => match r {
                            Ok(v) => return Err(Ok(v)),
                            Err(e) => return Err(Err(e))
                        }
                    };
                }
                let mut condition_evaluation = match self.evaluate(condition) {
                    Ok(v) => v,
                    Err(e) => return Err(Err(e))
                };
                let mut condition_result = match self.is_truthy(&condition_evaluation) {
                    Ok(v) => v,
                    Err(e) => return Err(Err(e))
                };

                while condition_result {
                    let _ = match self.execute(&body) {
                        Ok(_) => {},
                        Err(r) => match r{
                            Ok(v) => return Err(Ok(v)),
                            Err(e) => return Err(Err(e))
                        }
                    };
                    if increment.is_some() {
                        let _ = match self.evaluate(&increment.as_ref().unwrap()) {
                            Ok(v) => v,
                            Err(e) => return Err(Err(e))
                        };
                    }
                    condition_evaluation = match self.evaluate(condition) {
                        Ok(v) => v,
                        Err(e) => return Err(Err(e))
                    };
                    condition_result = match self.is_truthy(&condition_evaluation) {
                        Ok(v) => v,
                        Err(e) => return Err(Err(e))
                    };
                }
                Ok(())
            },
            _ => return Err(Err(InterpreterError::ExpectedForStatement))
        }
    }

    fn visit_function_stmt(&mut self, stmt: &Stmt) -> Result<(), Result<Value, InterpreterError>> {
        match stmt {
            Stmt::Function { name, params: _, body: _ } => {
                let function = match Func::new(stmt.clone(), self.environment.clone()) {
                    Ok(v) => v,
                    Err(e) => return Err(Err(e))
                };
                self.environment.borrow_mut().define(name.lexeme.clone(), Value::Function(function));
                Ok(())
            },
            _ => Err(Err(InterpreterError::ExpectedFunctionStatement))
        }
    }

    fn visit_return_stmt(&mut self, stmt: &Stmt) -> Result<(), Result<Value, InterpreterError>> {
        match stmt {
            Stmt::Return { keyword: _, value } => {
                let mut return_value = Value::Literal(LiteralType::Nil);
                if value.is_some() {
                    return_value = match self.evaluate(&value.as_ref().unwrap()) {
                        Ok(v) => v,
                        Err(e) => return Err(Err(e))
                    };
                }
                Err(Ok(return_value))
            },
            _ => return Err(Err(InterpreterError::ExpectedReturnStatement))
        }
    }
}