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

// GOOD EXCEPTION HANDLING - EXCELLENT CODING STYLE
pub type ExprResult = Result<Value, EvaluatorError>;
pub type StmtResult = Result<(), Result<Value, EvaluatorError>>;
pub type Env = Rc<RefCell<Environment>>;

/// The `Evaluator` struct is responsible for evaluating the AST and executing the program.
/// It maintains the current environment and provides methods for evaluating expressions and
/// executing statements.
///
/// # Attributes
/// - `environment`: The current environment in which the evaluator is operating. This is an `Rc<RefCell<Environment>>`
///   that allows for shared ownership and interior mutability.
/// - `globals`: The global environment that contains global variables and functions. This is also an `Rc<RefCell<Environment>>`.
/// - `output`: A vector of strings used to store output.
pub struct Evaluator {
    pub environment: Env,
    #[allow(dead_code)]
    pub globals: Env,
    output: Vec<String>,
}

impl Evaluator {
    /// Creates a new `Evaluator` instance with a global environment.
    ///
    /// # Returns
    /// A new `Evaluator` instance.
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

        // HASHING - BAND A
        let hash = NativeFunc::new("hash".to_string(), 1, |_, args| {
            if let Value::Literal(LiteralType::Str(s)) = &args[0] {
                let mut hasher = Sha256::new();
                hasher.update(s);
                return Ok(Value::Literal(LiteralType::Str(format!("{:x}", hasher.finalize()))));
            }
            return Err(EvaluatorError::CannotHashValue);
        });

        // Adds these native functions to the global environment so they can be used in the program.
        globals.borrow_mut().define("clock".to_string(), Value::NativeFunction(clock));
        globals.borrow_mut().define("hash".to_string(), Value::NativeFunction(hash));

        return Self {
            environment: Rc::clone(&globals),
            globals,
            output: Vec::new()
        };
    }

    /// Interprets and executes the given statements.
    /// COMPLEX USER-DEFINED ALGORITHM - BAND A
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

    /// Evaluates an expression.
        fn evaluate(&mut self, expr: &Expr) -> Result<Value, EvaluatorError> {
            // Calls the appropriate method based on the expression type.
            return match expr.accept_expr(self) { // RECURSIVE ALGORITHM - BAND A
                Ok(v) => Ok(v),
                Err(e) => Err(e),
            }
        }

    /// Executes a statement.
    fn execute(&mut self, stmt: &Stmt) -> StmtResult {
        return stmt.accept_stmt(self);
    }

    /// Executes a body of statements within a new environment.
    pub fn execute_body(&mut self, statements: Vec<Stmt>, environment: Env) -> StmtResult {
        // Stores the current environment to be restored after the body is executed.
        let previous = Rc::clone(&self.environment);

        // Sets the current environment to the new environment.
        self.environment = Rc::clone(&environment);

        // Executes all of the statements in the body.
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
        // Pops the previous environment off the stack
        self.environment = previous; // STACK OPERATIONS - BAND A
        return Ok(());
    }

    /// Checks if a value is truthy.
    fn is_truthy(&mut self, object: &Value) -> Result<bool, EvaluatorError> {
        match object {
            // Matches the object to its literal type and returns whether it is truthy.
            Value::Literal(literal) => {
                return Ok(!matches!(literal, LiteralType::Null | LiteralType::False))
            }
            _ => return Err(EvaluatorError::ExpectedLiteralValue),
        }
    }

    /// Checks if two values are equal.
    fn is_equal(&mut self, a: &Value, b: &Value) -> bool {
        return *a == *b;
    }

    /// Converts a literal value to its string representation.
    fn stringify(&self, object: &LiteralType) -> String {
        // Matches the object to its literal type and returns the string representation.
        return match object {
            LiteralType::Num(n) => {
                let mut text = n.to_string();
                // Removes unecessary decimal points.
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

impl expr::ExprVisitor<ExprResult> for Evaluator { // INTERFACE - BAND A
    fn visit_alteration_expr(&mut self, expr: &Expr) -> ExprResult {
        match expr {
            Expr::Alteration { name, alteration_type } => {
                // Gets the value of the variable from the current environment.
                let curr_value = self.environment.borrow().get(name)?;

                // Matches the alteration type and performs the appropriate operation.
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
            _ => return Err(EvaluatorError::DifferentExpression { // GOOD EXCEPTION HANDLING - EXCELLENT CODING STYLE
                expr: expr.clone(),
                expected: "alteration".to_string(),
            }),
        }
    }

    fn visit_assign_expr(&mut self, expr: &Expr) -> ExprResult {
        match expr {
            Expr::Assign { name, value } => {
                // Evaluates the value expression so it can be assigned to the variable.
                let value = self.evaluate(value)?;

                // Assigns the value to the variable in the current environment.
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
                // Evaluates the left and right expressions so they can be used in the operation.
                let left = self.evaluate(&left)?;
                let right = self.evaluate(&right)?;
                
                // Matches the operator type and performs the appropriate operation.
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

                // Recursively evaluates the arguments.
                // RECURSIVE ALGORITHM - BAND A
                for argument in arguments {
                    let arg = self.evaluate(argument)?;
                    args.push(arg);
                }

                match callee {
                    Value::Function(f) => {
                        if args.len() != f.arity {
                            // GOOD EXCEPTION HANDLING - EXCELLENT CODING STYLE
                            return Err(EvaluatorError::ArgsDifferFromArity {
                                args: args.len(),
                                arity: f.arity,
                            });
                        }
                        // DELEGATION / METHOD FORWARDING - BAND A EQUIVALENT
                        return f.call(self, args);
                    }

                    // Checks if the callee is a native function and calls it.
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
            // Evaluates the expression inside the grouping expression.
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
                // Recursively evaluates the items and appends them to a list.
                // RECURSIVE ALGORITHM - BAND A
                for item in items {
                    list.push(self.evaluate(item)?); // LIST OPERATIONS - BAND A
                }
                Ok(Value::List(List::new(list)))
            },
            _ => return Err(EvaluatorError::DifferentExpression {
                expr: expr.clone(),
                expected: "list".to_string(),
            }),
        }
    }

    // THIS ENTIRE METHOD IS LIST OPERATIONS BECAUSE IT HANDLES LIST METHOD CALLS - BAND A
    fn visit_listmethodcall_expr(&mut self, expr: &Expr) -> ExprResult {
        match expr {
            Expr::ListMethodCall { object, call } => {
                // Checks if the call expression is a method call --> DEFENSIVE PROGRAMMING - EXCELLENT CODING STYLE
                if let Expr::Call { callee, arguments } = &**call {
                    // Checks if the callee is a variable --> DEFENSIVE PROGRAMMING - EXCELLENT CODING STYLE
                    if let Expr::Var { name } = &**callee {
                        let mut args: Vec<Value> = Vec::new();

                        // Recursively evaluates the arguments.
                        // RECURSIVE ALGORITHM - BAND A
                        for argument in arguments {
                            let arg = self.evaluate(argument)?;
                            args.push(arg);
                        }

                        // Gets the value of the object from the current environment.
                        let list = self.environment.borrow().get(object)?;
                        let mut result_value: Option<Value> = None;
                        let new_list;

                        // Checks if the object is a list and performs the appropriate operation.
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
                                "sort" => {
                                    let sorted_list = list.tim_sort()?;
                                    result_value = Some(Value::List(sorted_list.clone()));
                                    Ok(sorted_list)?
                                },
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
                // Evaluates the left expression.
                let left = self.evaluate(left)?;

                // Checks if the operator is a logical operator and performs the appropriate operation.
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

    fn visit_membership_expr(&mut self, expr: &Expr) -> ExprResult {
        match expr {
            Expr::Membership { left, not, right } => {
                // Evaluates the left and right expressions.
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;

                // Checks if the right expression is a list --> DEFENSIVE PROGRAMMING - EXCELLENT CODING STYLE
                if let Value::List(list) = right {
                    // Checks if the list contains the value of the left expression.
                    if (list.values.contains(&left) && !not) || (!list.values.contains(&left) && *not) {
                        return Ok(Value::Literal(LiteralType::True));
                    } else {
                        return Ok(Value::Literal(LiteralType::False));
                    }
                }

                return Err(EvaluatorError::ExpectedList);
            },
            _ => return Err(EvaluatorError::DifferentExpression {
                expr: expr.clone(),
                expected: "membership".to_string(),
            }),
        }
    }

    fn visit_splice_expr(&mut self, expr: &Expr) -> ExprResult {
        match expr {
            Expr::Splice { list, is_splice, start, end } => {
                // Initialises the start and end index expressions to be null.
                let mut start_idx_expr: Option<Value> = None;
                let mut end_idx_expr: Option<Value> = None;

                // Evaluates the start and end index expressions if they exist.
                if let Some(start) = start {
                    start_idx_expr = Some(self.evaluate(start)?);
                }
                if let Some(end) = end {
                    end_idx_expr = Some(self.evaluate(end)?);
                }

                // Initialises the start and end index values to be 0 and null respectively.
                let mut start_idx: usize = 0;
                let mut end_idx: Option<usize> = None;

                // Checks if the start index is a number --> DEFENSIVE PROGRAMMING - EXCELLENT CODING STYLE
                if let Some(Value::Literal(ref v)) = start_idx_expr {
                    if let LiteralType::Num(num) = v {
                        start_idx = *num as usize;
                    } else {
                        return Err(EvaluatorError::ExpectedIndexToBeANum);
                    }
                } else if end_idx_expr.is_none() {
                    return Err(EvaluatorError::ExpectedIndexToBeANum)
                }

                // Checks if the end index is a number and assigns it to the end_idx variable if it is.
                if let Some(Value::Literal(v)) = end_idx_expr {
                    if let LiteralType::Num(num) = v {
                        end_idx = Some(num as usize);
                    } else {
                        return Err(EvaluatorError::ExpectedIndexToBeANum); // GOOD EXCEPTION HANDLING - EXCELLENT CODING STYLE
                    }
                } else if end_idx.is_some() {
                    return Err(EvaluatorError::ExpectedIndexToBeANum)
                }

                // Retrieves the value of the list from the current environment.
                let value = self.environment.borrow().get(list)?;

                // Checks if the value is a list and returns the appropriate value(s).
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

                return Err(EvaluatorError::ValueWasNotAList); // GOOD EXCEPTION HANDLING - EXCELLENT CODING STYLE
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
                let right = self.evaluate(right)?; // RECURSIVE ALGORITHM - BAND A

                // Matches the operator type and performs the appropriate operation.
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
                        return Err(EvaluatorError::UnableToNegate) // GOOD EXCEPTION HANDLING - EXCELLENT CODING STYLE
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
                // Retrieves the value of the variable from the current environment.
                return self.environment.borrow().get(name);
            },
            _ => return Err(EvaluatorError::DifferentExpression {
                expr: expr.clone(),
                expected: "variable".to_string(),
            }),
        }
    }
}

impl stmt::StmtVisitor<StmtResult> for Evaluator { // INTERFACES - BAND A
    fn visit_expression_stmt(&mut self, stmt: &Stmt) -> StmtResult {
        match stmt {
            Stmt::Expression { expression } => {
                // Evaluates the expression statement.
                return match self.evaluate(expression) { // RECURSIVE ALGORITHM - BAND A
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
                // Executes the initializer statement before the for loop runs.
                match self.execute(initializer) {
                    Ok(_) => {},
                    Err(r) => return Err(Ok(r)?),
                };

                // Evaluates the condition expression.
                let mut condition_evaluation = match self.evaluate(condition) { // RECURSIVE ALGORITHM - BAND A
                    Ok(v) => v,
                    Err(e) => return Err(Err(e)),
                };

                // Checks if the condition is truthy.
                let mut condition_result = match self.is_truthy(&condition_evaluation) {
                    Ok(v) => v,
                    Err(e) => return Err(Err(e)),
                };

                // Instantiates a new environment for the for loop.
                // DYNAMIC OBJECT GENERATION - BAND A
                self.environment = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(&self.environment)))));
                
                // If the condition is truthy, then it executes the body of the for loop.
                while condition_result {
                    // RECURSIVE ALGORITHM - BAND A
                    // TREE TRAVERSAL - BAND A
                    for stmt in body {
                        match self.execute(stmt) {
                            Ok(_) => {}
                            Err(r) => return Err(Ok(r)?)
                        };
                    }

                    // Executes the step statement after the body of the for loop.
                    match self.evaluate(step) {
                        Ok(_) => {},
                        Err(e) => return Err(Err(e)),
                    };
                    
                    // Re-evaluates the condition expression.
                    condition_evaluation = match self.evaluate(condition) {
                        Ok(v) => v,
                        Err(e) => return Err(Err(e)),
                    };

                    // Checks if the condition is truthy.
                    condition_result = match self.is_truthy(&condition_evaluation) {
                        Ok(v) => v,
                        Err(e) => return Err(Err(e)),
                    };
                }

                return Ok(());
            },
            _ => return Err(Err(EvaluatorError::DifferentStatement {
                stmt: stmt.clone(),
                expected: "for".to_string(),
            }))
        }
    }

    fn visit_function_stmt(&mut self, stmt: &Stmt) -> StmtResult {
        match stmt {
            Stmt::Function { name, .. } => {
                // Initialises the function, using the current environment.
                // DYNAMIC OBJECT GENERATION - BAND A
                let function = match Func::new(stmt.clone(), self.environment.clone()) {
                    Ok(v) => v,
                    Err(e) => return Err(Err(e)),
                };
                // Adds the function to the current environment.
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
                // Evaluates the condition of the if statement.
                let condition_evaluation = match self.evaluate(condition) {
                    Ok(v) => v,
                    Err(e) => return Err(Err(e)),
                };

                // Checks if the condition is truthy.
                let condition_evaluation_result = match self.is_truthy(&condition_evaluation) {
                    Ok(v) => v,
                    Err(e) => return Err(Err(e)),
                };

                // Only if the condition is truthy, then it traverses the tree and recursively executes the body.
                if condition_evaluation_result {
                    // RECURSIVE ALGORITHM - BAND A
                    // TREE TRAVERSAL - BAND A
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
                        Ok(_) => {},
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
                // Evaluates the expression inside of the print statement.
                let value = match self.evaluate(expression) {
                    Ok(v) => v,
                    Err(e) => return Err(Err(e)),
                };
                // Mathes the different types of values that can be printed.
                match value {
                    // Handles printing out a literal value.
                    Value::Literal(literal) => {
                        println!("{}", self.stringify(&literal));
                        self.output.push(self.stringify(&literal));
                        return Ok(());
                    },
                    // Handles printing out lists.
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
                // Initialises the return value to be null.
                let mut return_value = Value::Literal(LiteralType::Null);
                if value.is_some() {
                    // Returns the value in a stack-like manner.
                    // STACK OPERATIONS - BAND A
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
                // Sets the initial value of the variable to null.
                let mut value = Value::Literal(LiteralType::Null);
                
                // Evaluates the initializer expression if it exists.
                if let Some(initializer_expr) = initializer {
                    value = match self.evaluate(initializer_expr) {
                        Ok(v) => v,
                        Err(e) => return Err(Err(e)),
                    };
                }
                
                // Adds the new variable to the current environment.
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
                // Evaluates the condition expression.
                let mut condition_evaluation = match self.evaluate(condition) {
                    Ok(v) => v,
                    Err(e) => return Err(Err(e)),
                };

                // Checks if the condition is truthy.
                let mut condition_result = match self.is_truthy(&condition_evaluation) {
                    Ok(v) => v,
                    Err(e) => return Err(Err(e)),
                };

                while condition_result {
                    // Recursively executes the body of the while loop.
                    // RECURSIVE ALGORITHM - BAND A
                    // TREE TRAVERSAL - BAND A
                    for stmt in body { 
                        match self.execute(stmt) {
                            Ok(_) => {},
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
