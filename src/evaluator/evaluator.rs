use std::{
    cell::RefCell,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};
use sha2::{Sha256, Digest};

use crate::{
    alteration,
    arithmetic,
    callable::{Callable, Func, NativeFunc},
    comparison,
    environment::Environment,
    error::EvaluatorError,
    expr::{self, Expr},
    list::List,
    stmt::{self, Stmt},
    token::TokenType,
    value::{LiteralType, Value},
};

pub type ExprResult = Result<Value, EvaluatorError>;
pub type StmtResult = Result<(), Result<Value, EvaluatorError>>;
pub type Env = Rc<RefCell<Environment>>;

pub struct Evaluator {
    pub environment: Env,
    pub globals: Env,
    output: Vec<String>,
}

impl Evaluator {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new(None)));

        let clock = NativeFunc::new("clock".to_string(), 0, |_, _| {
            Ok(Value::Literal(LiteralType::Num(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64(),
            )))
        });

        let hash = NativeFunc::new("hash".to_string(), 1, |_, args| {
            if let Value::Literal(LiteralType::Str(s)) = &args[0] {
                let mut hasher = Sha256::new();
                hasher.update(s);
                return Ok(Value::Literal(LiteralType::Str(format!("{:x}", hasher.finalize()))));
            }
            return Err(EvaluatorError::CannotHashValue);
        });

        globals.borrow_mut().define("clock".to_string(), Value::NativeFunction(clock));
        globals.borrow_mut().define("hash".to_string(), Value::NativeFunction(hash));

        return Self {
            environment: Rc::clone(&globals),
            globals,
            output: Vec::new()
        };
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<Vec<String>, EvaluatorError> {
        for stmt in statements {
            match self.execute(&stmt) {
                Ok(()) => {}
                Err(r) => match r {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                },
            };
        }
        return Ok(self.output.clone());
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, EvaluatorError> {
        return match expr.accept_expr(self) {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        }
    }

    fn execute(&mut self, stmt: &Stmt) -> StmtResult {
        return stmt.accept_stmt(self);
    }

    pub fn execute_block(&mut self, statements: Vec<Stmt>, environment: Env) -> StmtResult {
        let previous = Rc::clone(&self.environment);

        self.environment = Rc::clone(&environment);

        for statement in statements {
            match self.execute(&statement) {
                Ok(_) => {}
                Err(r) => match r {
                    Ok(v) => {
                        self.environment = previous;
                        return Err(Ok(v));
                    }
                    Err(e) => return Err(Err(e)),
                },
            }
        }
        self.environment = previous;
        return Ok(());
    }

    fn is_truthy(&mut self, object: &Value) -> Result<bool, EvaluatorError> {
        match object {
            Value::Literal(literal) => {
                return Ok(!matches!(literal, LiteralType::Null | LiteralType::False))
            }
            _ => return Err(EvaluatorError::ExpectedLiteralValue),
        }
    }

    fn is_equal(&mut self, a: &Value, b: &Value) -> bool {
        return *a == *b;
    }

    fn stringify(&self, object: &LiteralType) -> String {
        return match object {
            LiteralType::Num(n) => {
                let mut text = n.to_string();
                if text.ends_with(".0") {
                    text.truncate(text.len() - 2);
                }
                text
            }
            LiteralType::Str(s) => s.clone(),
            LiteralType::True => "true".to_string(),
            LiteralType::False => "false".to_string(),
            LiteralType::Null => "null".to_string(),
        }
    }
}
impl expr::ExprVisitor<ExprResult> for Evaluator {
    fn visit_alteration_expr(&mut self, expr: &Expr) -> ExprResult {
        match expr {
            Expr::Alteration { name, alteration_type } => {
                let curr_value = self.environment.borrow().get(name)?;

                match alteration_type {
                    TokenType::Incr => {
                        alteration!( self ;  + ; name ; curr_value);
                    }
                    TokenType::Decr => {
                        alteration!( self ; - ; name ; curr_value);
                    }
                    _ => return Err(EvaluatorError::ExpectedAlterationToken),
                }
            }
            _ => return Err(EvaluatorError::DifferentExpression {
                expr: expr.clone(),
                expected: "alteration".to_string(),
            }),
        }
    }

    fn visit_assign_expr(&mut self, expr: &Expr) -> ExprResult {
        match expr {
            Expr::Assign { name, value } => {
                let value = self.evaluate(value)?;

                return self.environment
                    .borrow_mut()
                    .assign(name, value);
            }
            _ => return Err(EvaluatorError::DifferentExpression {
                expr: expr.clone(),
                expected: "assign".to_string(),
            }),
        }
    }

    fn visit_binary_expr(&mut self, expr: &Expr) -> ExprResult {
        match expr {
            Expr::Binary { left, operator, right } => {
                let left = self.evaluate(&left)?;
                let right = self.evaluate(&right)?;

                match operator.token_type {
                    TokenType::Greater => {
                        comparison!( > ; left ; right);
                        return Err(EvaluatorError::ExpectedNumber);
                    }
                    TokenType::GreaterEqual => {
                        comparison!( >= ; left ; right);
                        return Err(EvaluatorError::ExpectedNumber);
                    }
                    TokenType::Less => {
                        comparison!( < ; left ; right);
                        return Err(EvaluatorError::ExpectedNumber);
                    }
                    TokenType::LessEqual => {
                        comparison!( <= ; left ; right);
                        return Err(EvaluatorError::ExpectedNumber);
                    }
                    TokenType::BangEqual => {
                        if !self.is_equal(&left, &right) {
                            return Ok(Value::Literal(LiteralType::True));
                        }
                        return Ok(Value::Literal(LiteralType::False));
                    }
                    TokenType::EqualEqual => {
                        if self.is_equal(&left, &right) {
                            return Ok(Value::Literal(LiteralType::True));
                        }
                        return Ok(Value::Literal(LiteralType::False));
                    }
                    TokenType::Plus => {
                        arithmetic!( + ; left ; right );
                        return Err(EvaluatorError::ExpectedNumber);
                    }
                    TokenType::Minus => {
                        arithmetic!( - ; left ; right);
                        return Err(EvaluatorError::ExpectedNumber);
                    }
                    TokenType::FSlash => {
                        arithmetic!( / ; left ; right);
                        return Err(EvaluatorError::ExpectedNumber);
                    }
                    TokenType::Asterisk => {
                        arithmetic!( * ; left ; right);
                        return Err(EvaluatorError::ExpectedNumber);
                    }
                    _ => return Err(EvaluatorError::ExpectedValidBinaryOperator),
                }
            }
            _ => return Err(EvaluatorError::DifferentExpression {
                expr: expr.clone(),
                expected: "binary".to_string(),
            }),
        }
    }

    fn visit_call_expr(&mut self, expr: &Expr) -> ExprResult {
        match expr {
            Expr::Call { callee, arguments } => {
                let callee = self.evaluate(callee)?;

                let mut args: Vec<Value> = Vec::new();

                for argument in arguments {
                    let arg = self.evaluate(argument)?;
                    args.push(arg);
                }

                match callee {
                    Value::Function(f) => {
                        if args.len() != f.arity {
                            return Err(EvaluatorError::ArgsDifferFromArity {
                                args: args.len(),
                                arity: f.arity,
                            });
                        }
                        return f.call(self, args);
                    }
                    Value::NativeFunction(nf) => {
                        if args.len() != nf.arity {
                            return Err(EvaluatorError::ArgsDifferFromArity {
                                args: args.len(),
                                arity: nf.arity,
                            });
                        }
                        return nf.call(self, args);
                    }
                    _ => return Err(EvaluatorError::ExpectedFunctionOrClass),
                }
            }
            _ => return Err(EvaluatorError::DifferentExpression {
                expr: expr.clone(),
                expected: "call".to_string(),
            }),
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) -> ExprResult {
        match expr {
            Expr::Grouping { expression } => return self.evaluate(expression),
            _ => return Err(EvaluatorError::DifferentExpression {
                expr: expr.clone(),
                expected: "group".to_string(),
            }),
        }
    }

    fn visit_list_expr(&mut self, expr: &Expr) -> ExprResult {
        match expr {
            Expr::List { items } => {
                let mut list: Vec<Value> = Vec::new();
                for item in items {
                    list.push(self.evaluate(item)?);
                }
                Ok(Value::List(List::new(list)))
            },
            _ => return Err(EvaluatorError::DifferentExpression {
                expr: expr.clone(),
                expected: "list".to_string(),
            }),
        }
    }

    fn visit_listmethodcall_expr(&mut self, expr: &Expr) -> ExprResult {
        match expr {
            Expr::ListMethodCall { object, call } => {
                if let Expr::Call { callee, arguments } = &**call {
                    if let Expr::Var { name } = &**callee {
                        let mut args: Vec<Value> = Vec::new();

                        for argument in arguments {
                            let arg = self.evaluate(argument)?;
                            args.push(arg);
                        }

                        let list = self.environment.borrow().get(object)?;
                        let mut result_value: Option<Value> = None;
                        let new_list;

                        if let Value::List(mut list) = list {
                            new_list = match name.lexeme.as_str() {
                                "push" => list.push(args)?,
                                "pop" => {
                                    let temp = list.pop();
                                    if temp.0.is_some() {
                                        result_value = temp.0;
                                    }
                                    temp.1
                                },
                                "remove" => {
                                    let temp = list.remove(args)?;
                                    result_value = Some(temp.0);
                                    temp.1
                                },
                                "insertAt" => list.insert_at(args)?,
                                "index" => return Ok(Value::Literal(LiteralType::Num(list.index(args)? as f64))),
                                "len" => return Ok(Value::Literal(LiteralType::Num(list.len() as f64))),
                                "sort" => return Ok(Value::List(list.tim_sort()?)),
                                _ => return Err(EvaluatorError::InvalidListMethod)
                            };

                            self.environment.borrow_mut().assign(object, Value::List(new_list.clone()))?;
                            if let Some(v) = result_value {
                                return Ok(v);
                            }
                        }
                    }
                }

                return Ok(Value::Literal(LiteralType::Null));
            },
            _ => return Err(EvaluatorError::DifferentExpression {
                expr: expr.clone(),
                expected: "listmethodcall".to_string(),
            }),
        }
    }

    fn visit_literal_expr(&mut self, expr: &Expr) -> ExprResult {
        match expr {
            Expr::Literal { value } => return Ok(Value::Literal(value.clone())),
            _ => return Err(EvaluatorError::DifferentExpression {
                expr: expr.clone(),
                expected: "literal".to_string(),
            }),
        }
    }

    fn visit_logical_expr(&mut self, expr: &Expr) -> ExprResult {
        match expr {
            Expr::Logical { left, operator, right } => {
                let left = self.evaluate(left)?;

                if operator.token_type == TokenType::Or {
                    match self.is_truthy(&left) {
                        Ok(v) => {
                            if v {
                                return Ok(left);
                            }
                        }
                        Err(e) => return Err(e),
                    }
                } else {
                    match self.is_truthy(&left) {
                        Ok(v) => {
                            if !v {
                                return Ok(left);
                            }
                        }
                        Err(e) => return Err(e),
                    }
                }

                return self.evaluate(right);
            }
            _ => return Err(EvaluatorError::DifferentExpression {
                expr: expr.clone(),
                expected: "logical".to_string(),
            }),
        }
    }

    fn visit_splice_expr(&mut self, expr: &Expr) -> ExprResult {
        match expr {
            Expr::Splice { list, is_splice, start, end } => {
                let mut start_idx_expr: Option<Value> = None;
                let mut end_idx_expr: Option<Value> = None;

                if let Some(start) = start {
                    start_idx_expr = Some(self.evaluate(start)?);
                }
                if let Some(end) = end {
                    end_idx_expr = Some(self.evaluate(end)?);
                }

                let mut start_idx: usize = 0;
                let mut end_idx: Option<usize> = None;

                if let Some(Value::Literal(ref v)) = start_idx_expr {
                    if let LiteralType::Num(num) = v {
                        start_idx = *num as usize;
                    } else {
                        return Err(EvaluatorError::ExpectedIndexToBeANum);
                    }
                } else if end_idx_expr.is_none() {
                    return Err(EvaluatorError::ExpectedIndexToBeANum)
                }

                if let Some(Value::Literal(v)) = end_idx_expr {
                    if let LiteralType::Num(num) = v {
                        end_idx = Some(num as usize);
                    } else {
                        return Err(EvaluatorError::ExpectedIndexToBeANum);
                    }
                } else if end_idx.is_some() {
                    return Err(EvaluatorError::ExpectedIndexToBeANum)
                }

                let value = self.environment.borrow().get(list)?;

                if let Value::List(list) = value {
                    if let Some(end_idx) = end_idx {
                        if end_idx >= list.values.len() {
                            return Err(EvaluatorError::IndexOutOfRange);
                        }
                        if start_idx_expr.is_none() {
                            return Ok(Value::List(List::new(list.values[0..end_idx + 1].to_vec())));
                        }

                        return Ok(Value::List(List::new(list.values[start_idx..end_idx + 1].to_vec())));
                    }
                    if start_idx >= list.values.len() {
                        return Err(EvaluatorError::IndexOutOfRange);
                    }
                    if *is_splice {
                        return Ok(Value::List(List::new(list.values[start_idx..list.values.len()].to_vec())));
                    }
                    return Ok(list.values[start_idx].clone());
                }

                return Err(EvaluatorError::ValueWasNotAList);
            },
            _ => return Err(EvaluatorError::DifferentExpression {
                expr: expr.clone(),
                expected: "splice".to_string(),
            }),
        }
    }

    fn visit_unary_expr(&mut self, expr: &Expr) -> ExprResult {
        match expr {
            Expr::Unary { operator, right } => {
                let right = self.evaluate(right)?;

                match operator.token_type {
                    TokenType::Bang => match self.is_truthy(&right) {
                        Ok(v) => {
                            if v {
                                return Ok(Value::Literal(LiteralType::False));
                            }
                            return Ok(Value::Literal(LiteralType::True));
                        }
                        Err(e) => return Err(e),
                    },
                    TokenType::Minus => {
                        if let Value::Literal(LiteralType::Num(n)) = right {
                            return Ok(Value::Literal(LiteralType::Num(-n)));
                        }
                        return Err(EvaluatorError::UnableToNegate)
                    }
                    _ => return Err(EvaluatorError::ExpectedMinus),
                }
            }
            _ => return Err(EvaluatorError::DifferentExpression {
                expr: expr.clone(),
                expected: "unary".to_string(),
            }),
        }
    }

    fn visit_var_expr(&mut self, expr: &Expr) -> ExprResult {
        match expr {
            Expr::Var { name } => {
                return self.environment.borrow().get(name);
            },
            _ => return Err(EvaluatorError::DifferentExpression {
                expr: expr.clone(),
                expected: "variable".to_string(),
            }),
        }
    }
}

impl stmt::StmtVisitor<StmtResult> for Evaluator {
    fn visit_expression_stmt(&mut self, stmt: &Stmt) -> StmtResult {
        match stmt {
            Stmt::Expression { expression } => {
                return match self.evaluate(expression) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(Err(e)),
                }
            }
            _ => return Err(Err(EvaluatorError::DifferentStatement {
                stmt: stmt.clone(),
                expected: "expression".to_string(),
            })),
        }
    }

    fn visit_for_stmt(&mut self, stmt: &Stmt) -> StmtResult {
        match stmt {
            Stmt::For { initializer, condition, step, body } => {
                match self.execute(initializer) {
                    Ok(_) => {},
                    Err(r) => return Err(Ok(r)?),
                };

                let mut condition_evaluation = match self.evaluate(condition) {
                    Ok(v) => v,
                    Err(e) => return Err(Err(e)),
                };
                let mut condition_result = match self.is_truthy(&condition_evaluation) {
                    Ok(v) => v,
                    Err(e) => return Err(Err(e)),
                };
                
                while condition_result {
                    for stmt in body {
                        match self.execute(stmt) {
                            Ok(_) => {}
                            Err(r) => return Err(Ok(r)?)
                        };
                    }

                    match self.evaluate(step) {
                        Ok(_) => {},
                        Err(e) => return Err(Err(e)),
                    };
                    
                    condition_evaluation = match self.evaluate(condition) {
                        Ok(v) => v,
                        Err(e) => return Err(Err(e)),
                    };
                    condition_result = match self.is_truthy(&condition_evaluation) {
                        Ok(v) => v,
                        Err(e) => return Err(Err(e)),
                    };
                }

                return Ok(());
            }
            _ => return Err(Err(EvaluatorError::DifferentStatement {
                stmt: stmt.clone(),
                expected: "for".to_string(),
            })),
        }
    }

    fn visit_function_stmt(&mut self, stmt: &Stmt) -> StmtResult {
        match stmt {
            Stmt::Function { name, .. } => {
                let function = match Func::new(stmt.clone(), self.environment.clone()) {
                    Ok(v) => v,
                    Err(e) => return Err(Err(e)),
                };
                self.environment
                    .borrow_mut()
                    .define(name.lexeme.clone(), Value::Function(function));

                return Ok(());
            }
            _ => return Err(Err(EvaluatorError::DifferentStatement {
                stmt: stmt.clone(),
                expected: "function".to_string(),
            })),
        }
    }

    fn visit_if_stmt(&mut self, stmt: &Stmt) -> StmtResult {
        match stmt {
            Stmt::If { condition, then_branch, else_branch } => {
                let condition_evaluation = match self.evaluate(condition) {
                    Ok(v) => v,
                    Err(e) => return Err(Err(e)),
                };

                let condition_evaluation_result = match self.is_truthy(&condition_evaluation) {
                    Ok(v) => v,
                    Err(e) => return Err(Err(e)),
                };

                if condition_evaluation_result {
                    for stmt in then_branch {
                        match self.execute(stmt) {
                            Ok(_) => {}
                            Err(r) => match r {
                                Ok(v) => return Err(Ok(v)),
                                Err(e) => return Err(Err(e)),
                            },
                        };
                    }
                } else if else_branch.is_some() {
                    match self.execute(&else_branch.as_ref().unwrap()) {
                        Ok(_) => {}
                        Err(r) => return Err(Ok(r)?)
                    };
                }

                return Ok(());
            }
            _ => return Err(Err(EvaluatorError::DifferentStatement {
                stmt: stmt.clone(),
                expected: "if".to_string(),
            })),
        }
    }
    
    fn visit_print_stmt(&mut self, stmt: &Stmt) -> StmtResult {
        match stmt {
            Stmt::Print { expression } => {
                let value = match self.evaluate(expression) {
                    Ok(v) => v,
                    Err(e) => return Err(Err(e)),
                };
                match value {
                    Value::Literal(literal) => {
                        println!("{}", self.stringify(&literal));
                        self.output.push(self.stringify(&literal));
                        return Ok(());
                    },
                    Value::List(list) => {
                        println!("{list}");
                        self.output.push(format!("{list}"));
                        return Ok(());
                    },
                    _ => return Err(Err(EvaluatorError::ExpectedToPrintLiteralValue)),
                }
            }
            _ => return Err(Err(EvaluatorError::DifferentStatement {
                stmt: stmt.clone(),
                expected: "print".to_string(),
            })),
        }
    }

    fn visit_return_stmt(&mut self, stmt: &Stmt) -> StmtResult {
        match stmt {
            Stmt::Return { keyword: _, value } => {
                let mut return_value = Value::Literal(LiteralType::Null);
                if value.is_some() {
                    return_value = match self.evaluate(value.as_ref().unwrap()) {
                        Ok(v) => v,
                        Err(e) => return Err(Err(e)),
                    };
                }
                return Err(Ok(return_value));
            }
            _ => return Err(Err(EvaluatorError::DifferentStatement {
                stmt: stmt.clone(),
                expected: "return".to_string(),
            })),
        }
    }

    fn visit_var_stmt(&mut self, stmt: &Stmt) -> StmtResult {
        match stmt {
            Stmt::Var { name, initializer } => {
                let mut value = Value::Literal(LiteralType::Null);
                
                if let Some(initializer_expr) = initializer {
                    value = match self.evaluate(initializer_expr) {
                        Ok(v) => v,
                        Err(e) => return Err(Err(e)),
                    };
                }
                
                self.environment
                    .borrow_mut()
                    .define(name.lexeme.clone(), value);

                return Ok(());
            }
            _ => return Err(Err(EvaluatorError::DifferentStatement {
                stmt: stmt.clone(),
                expected: "var".to_string(),
            })),
        }
    }

    fn visit_while_stmt(&mut self, stmt: &Stmt) -> StmtResult {
        match stmt {
            Stmt::While { condition, body } => {
                let mut condition_evaluation = match self.evaluate(condition) {
                    Ok(v) => v,
                    Err(e) => return Err(Err(e)),
                };

                let mut condition_result = match self.is_truthy(&condition_evaluation) {
                    Ok(v) => v,
                    Err(e) => return Err(Err(e)),
                };

                while condition_result {
                    for stmt in body {
                        match self.execute(stmt) {
                            Ok(_) => {}
                            Err(r) => return Err(Ok(r)?)
                        };
                    }

                    condition_evaluation = match self.evaluate(condition) {
                        Ok(v) => v,
                        Err(e) => return Err(Err(e)),
                    };

                    condition_result = match self.is_truthy(&condition_evaluation) {
                        Ok(v) => v,
                        Err(e) => return Err(Err(e)),
                    };
                }

                return Ok(());
            }
            _ => return Err(Err(EvaluatorError::DifferentStatement {
                stmt: stmt.clone(),
                expected: "while".to_string(),
            })),
        }
    }
}
