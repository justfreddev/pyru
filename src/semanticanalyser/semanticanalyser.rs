use std::collections::HashMap;

use crate::{
    error::SemanticAnalyserError,
    expr::{self, Expr},
    stmt::{self, Stmt},
};

/// Represents a symbol in the symbol table.
#[derive(Debug)]
enum Symbol {
    Ident { initialised: bool },
}

/// Represents the type of a function.
#[derive(Clone, PartialEq)]
enum FunctionType {
    Function,
    None,
}

/// The `SemanticAnalyser` struct is responsible for performing semantic analysis on the AST.
/// It checks for semantic errors such as variable declarations, function declarations, and
/// ensures that the program is semantically correct.
/// A struct representing a semantic analyser.
///
/// # Attributes
///
/// `ast` - A vector of statements representing the abstract syntax tree (AST).
/// `symbol_tables` - A vector of hash maps, each representing a symbol table for different scopes.
/// `curr` - An index representing the current position in the AST.
/// `func_type` - An enum representing the type of the current function being analysed.
pub struct SemanticAnalyser {
    ast: Vec<Stmt>,
    symbol_tables: Vec<HashMap<String, Symbol>>, // Stack
    curr: usize,
    func_type: FunctionType,
}

impl SemanticAnalyser {
    /// Creates a new `SemanticAnalyser` instance with the given AST.
    ///
    /// # Parameters
    /// - `ast`: A vector of `Stmt` objects representing the AST.
    ///
    /// # Returns
    /// A new `SemanticAnalyser` instance.
    pub fn new(ast: Vec<Stmt>) -> Self {
        Self {
            ast,
            symbol_tables: vec![HashMap::<String, Symbol>::new()],
            curr: 0,
            func_type: FunctionType::None,
        }
    }

    /// Runs the semantic analysis on the AST.
    ///
    /// # Returns
    /// A `Result` containing `()` if successful, or a `SemanticAnalyserError` if a semantic error is encountered.
    pub fn run(&mut self) -> Result<(), SemanticAnalyserError> {
        for stmt in self.ast.clone() {
            stmt.accept_stmt(self)?;
        }

        return Ok(());
    }

    /// Begins a new scope by pushing a new symbol table onto the stack.
    fn begin_scope(&mut self) {
        let st: HashMap<String, Symbol> = HashMap::new();
        self.curr += 1;
        self.symbol_tables.push(st)
    }

    /// Ends the current scope by popping the symbol table from the stack.
    fn end_scope(&mut self) {
        self.curr -= 1;
        self.symbol_tables.pop();
    }

    /// Checks if a variable is declared in any of the symbol tables.
    fn check_declared(&mut self, name: &String) -> bool {
        if self.curr == 0 {
            if self.symbol_tables[0].contains_key(name) {
                return true;
            }
        } else {
            for i in (0..=self.curr).rev() {
                if self.symbol_tables[i].contains_key(name) {
                    return true;
                }
            }
        }

        return false;
    }

    /// Checks if a variable is defined in the current scope.
    fn check_defined(&mut self, ident_name: &String) -> bool {
        if let Some(sym) = self.symbol_tables[self.curr].get(ident_name) {
            match sym {
                Symbol::Ident { initialised } => {
                    if *initialised {
                        return true;
                    }
                    return false;
                }
            }
        }

        return false;
    }

    /// Checks and resolves a function declaration.
    fn pass_function(&mut self, stmt: &Stmt, declaration: FunctionType) -> Result<(), SemanticAnalyserError> {
        match stmt {
            Stmt::Function { name, params, body } => {
                let sym = Symbol::Ident { initialised: true };
                
                if self.symbol_tables[self.curr].contains_key(&name.lexeme) {
                    return Err(SemanticAnalyserError::VariableAlreadyAssignedInScope {
                        name: name.lexeme.clone(),
                    });
                }
                self.symbol_tables[self.curr].insert(name.lexeme.clone(), sym);

                self.begin_scope();

                let is_closure = self.func_type.clone() == FunctionType::Function;
                self.func_type = declaration;

                for param in params {
                    let sym = Symbol::Ident {
                        initialised: true,
                    };

                    if self.symbol_tables[self.curr].contains_key(&param.lexeme) {
                        return Err(SemanticAnalyserError::VariableAlreadyAssignedInScope {
                            name: param.lexeme.clone(),
                        });
                    }
                    self.symbol_tables[self.curr].insert(param.lexeme.clone(), sym);
                }

                for statement in body {
                    statement.accept_stmt(self)?;
                }

                self.end_scope();

                if !is_closure {
                    self.func_type = FunctionType::None;
                }

                return Ok(());
            },
            _ => {
                return Err(SemanticAnalyserError::DifferentStatement {
                    stmt: stmt.clone(),
                    expected: "function".to_string(),
                })
            }
        }
    }
}

impl expr::ExprVisitor<Result<(), SemanticAnalyserError>> for SemanticAnalyser {
    fn visit_alteration_expr(&mut self, expr: &Expr) -> Result<(), SemanticAnalyserError> {
        match expr {
            Expr::Alteration { name, .. } => {
                if self.check_declared(&name.lexeme) {
                    return Ok(());
                }
                return Err(SemanticAnalyserError::VariableNotFound {
                    name: name.lexeme.clone(),
                });
            }
            _ => return Err(SemanticAnalyserError::DifferentExpression {
                expr: expr.clone(),
                expected: "alteration".to_string(),
            }),
        }
    }

    fn visit_assign_expr(&mut self, expr: &Expr) -> Result<(), SemanticAnalyserError> {
        match expr {
            Expr::Assign { name, value } => {
                value.accept_expr(self)?;

                if self.check_declared(&name.lexeme) {
                    return Ok(());
                }

                return Err(SemanticAnalyserError::VariableNotFound {
                    name: name.lexeme.clone(),
                });
            }
            _ => return Err(SemanticAnalyserError::DifferentExpression {
                expr: expr.clone(),
                expected: "assign".to_string(),
            }),
        }
    }

    fn visit_binary_expr(&mut self, expr: &Expr) -> Result<(), SemanticAnalyserError> {
        match expr {
            Expr::Binary { left, operator: _, right } => {
                left.accept_expr(self)?;
                right.accept_expr(self)?;
                return Ok(());
            }
            _ => return Err(SemanticAnalyserError::DifferentExpression {
                expr: expr.clone(),
                expected: "binary".to_string(),
            }),
        }
    }

    fn visit_call_expr(&mut self, expr: &Expr) -> Result<(), SemanticAnalyserError> {
        match expr {
            Expr::Call { callee, arguments } => {
                callee.accept_expr(self)?;

                for argument in arguments {
                    argument.accept_expr(self)?;
                }

                return Ok(());
            }
            _ => return Err(SemanticAnalyserError::DifferentExpression {
                expr: expr.clone(),
                expected: "call".to_string(),
            }),
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) -> Result<(), SemanticAnalyserError> {
        match expr {
            Expr::Grouping { expression } => {
                expression.accept_expr(self)?;
                return Ok(());
            },
            _ => return Err(SemanticAnalyserError::DifferentExpression {
                expr: expr.clone(),
                expected: "grouping".to_string(),
            }),
        }
    }

    fn visit_list_expr(&mut self, expr: &Expr) -> Result<(), SemanticAnalyserError> {
        match expr {
            Expr::List { items } => {
                for item in items {
                    item.accept_expr(self)?;
                }

                return Ok(());
            },
            _ => return Err(SemanticAnalyserError::DifferentExpression {
                expr: expr.clone(),
                expected: "list".to_string(),
            }),
        }
    }

    fn visit_listmethodcall_expr(&mut self, expr: &Expr) -> Result<(), SemanticAnalyserError> {
        match expr {
            Expr::ListMethodCall { object, call } => {
                Expr::Var { name: object.clone() }.accept_expr(self)?;
                call.accept_expr(self)?;
                return Ok(());
            },
            _ => return Err(SemanticAnalyserError::DifferentExpression {
                expr: expr.clone(),
                expected: "listmethodcall".to_string(),
            }),
        }
    }

    fn visit_literal_expr(&mut self, expr: &Expr) -> Result<(), SemanticAnalyserError> {
        match expr {
            Expr::Literal { .. } => return Ok(()),
            _ => return Err(SemanticAnalyserError::DifferentExpression {
                expr: expr.clone(),
                expected: "literal".to_string(),
            }),
        }
    }

    fn visit_logical_expr(&mut self, expr: &Expr) -> Result<(), SemanticAnalyserError> {
        match expr {
            Expr::Logical { left, operator: _, right } => {
                left.accept_expr(self)?;
                right.accept_expr(self)?;

                return Ok(());
            }
            _ => return Err(SemanticAnalyserError::DifferentExpression {
                expr: expr.clone(),
                expected: "logical".to_string(),
            }),
        }
    }

    fn visit_membership_expr(&mut self,expr: &Expr) -> Result<(), SemanticAnalyserError> {
        match expr {
            Expr::Membership { left, not: _, right } => {
                left.accept_expr(self)?;
                right.accept_expr(self)?;

                return Ok(());
            },
            _ => return Err(SemanticAnalyserError::DifferentExpression {
                expr: expr.clone(),
                expected: "membership".to_string(),
            }),
        }
    }

    fn visit_splice_expr(&mut self, expr: &Expr) -> Result<(), SemanticAnalyserError> {
        match expr {
            Expr::Splice { list, is_splice: _, start, end } => {
                let var = Expr::Var { name: list.clone() };
                var.accept_expr(self)?;
                if let Some(start) = start {
                    start.accept_expr(self)?;
                }
                if let Some(end) = end {
                    end.accept_expr(self)?;
                }

                return Ok(());
            },
            _ => return Err(SemanticAnalyserError::DifferentExpression {
                expr: expr.clone(),
                expected: "splice".to_string(),
            }),
        }
    }

    fn visit_unary_expr(&mut self, expr: &Expr) -> Result<(), SemanticAnalyserError> {
        match expr {
            Expr::Unary { operator: _, right } => {
                right.accept_expr(self)?;
                return Ok(());
            }
            _ => return Err(SemanticAnalyserError::DifferentExpression {
                expr: expr.clone(),
                expected: "unary".to_string(),
            }),
        }
    }

    fn visit_var_expr(&mut self, expr: &Expr) -> Result<(), SemanticAnalyserError> {
        match expr {
            Expr::Var { name } => {
                if self.check_declared(&name.lexeme) {
                    return Ok(());
                }

                let keywords = vec!["hash", "clock", "push", "pop", "remove",
                "insertAt", "index", "len", "sort"];

                if keywords.contains(&name.lexeme.as_str()) {
                    return Ok(());
                }

                return Err(SemanticAnalyserError::VariableNotFound {
                    name: name.lexeme.clone(),
                });
            },
            _ => return Err(SemanticAnalyserError::DifferentExpression {
                expr: expr.clone(),
                expected: "var".to_string(),
            }),
        }
    }
}

impl stmt::StmtVisitor<Result<(), SemanticAnalyserError>> for SemanticAnalyser {
    fn visit_expression_stmt(&mut self, stmt: &Stmt) -> Result<(), SemanticAnalyserError> {
        match stmt {
            Stmt::Expression { expression } => {
                expression.accept_expr(self)?;
                return Ok(());
            }
            _ => {
                return Err(SemanticAnalyserError::DifferentStatement {
                    stmt: stmt.clone(),
                    expected: "expression".to_string(),
                })
            }
        }
    }

    fn visit_for_stmt(&mut self, stmt: &Stmt) -> Result<(), SemanticAnalyserError> {
        match stmt {
            Stmt::For {
                initializer,
                condition,
                step,
                body,
            } => {
                initializer.accept_stmt(self)?;

                condition.accept_expr(self)?;

                step.accept_expr(self)?;

                for stmt in body {
                    stmt.accept_stmt(self)?;
                }

                return Ok(());
            }
            _ => return Err(SemanticAnalyserError::DifferentStatement {
                stmt: stmt.clone(),
                expected: "for".to_string(),
            }),
        }
    }

    fn visit_function_stmt(&mut self, stmt: &Stmt) -> Result<(), SemanticAnalyserError> {
        return self.pass_function(stmt, FunctionType::Function);
    }

    fn visit_if_stmt(&mut self, stmt: &Stmt) -> Result<(), SemanticAnalyserError> {
        match stmt {
            Stmt::If { condition, then_branch, else_branch } => {
                condition.accept_expr(self)?;

                for stmt in then_branch {
                    stmt.accept_stmt(self)?;
                }

                if let Some(e_branch) = else_branch {
                    e_branch.accept_stmt(self)?;
                };

                return Ok(());
            }
            _ => return Err(SemanticAnalyserError::DifferentStatement {
                stmt: stmt.clone(),
                expected: "if".to_string(),
            }),
        }
    }

    fn visit_print_stmt(&mut self, stmt: &Stmt) -> Result<(), SemanticAnalyserError> {
        match stmt {
            Stmt::Print { expression } => {
                expression.accept_expr(self)?;
                return Ok(());
            }
            _ => return Err(SemanticAnalyserError::DifferentStatement {
                stmt: stmt.clone(),
                expected: "print".to_string(),
            }),
        }
    }

    fn visit_return_stmt(&mut self, stmt: &Stmt) -> Result<(), SemanticAnalyserError> {
        match stmt {
            Stmt::Return { keyword: _, value } => {
                if self.func_type == FunctionType::None {
                    return Err(SemanticAnalyserError::CannotReturnOutsideFunction);
                }

                if let Some(v) = value {
                    v.accept_expr(self)?;
                };

                return Ok(());
            }
            _ => return Err(SemanticAnalyserError::DifferentStatement {
                stmt: stmt.clone(),
                expected: "return".to_string(),
            }),
        }
    }

    fn visit_var_stmt(&mut self, stmt: &Stmt) -> Result<(), SemanticAnalyserError> {
        match stmt {
            Stmt::Var { name, initializer } => {
                if self.check_defined(&name.lexeme) {
                    return Err(SemanticAnalyserError::VariableAlreadyAssignedInScope {
                        name: name.lexeme.clone(),
                    });
                }

                if let Some(x) = initializer {
                    x.accept_expr(self)?;
                }

                let sym = Symbol::Ident {
                    initialised: initializer.is_some(),
                };
                self.symbol_tables[self.curr].insert(name.lexeme.clone(), sym);

                return Ok(());
            }
            _ => {
                return Err(SemanticAnalyserError::DifferentStatement {
                    stmt: stmt.clone(),
                    expected: "var".to_string(),
                });
            }
        }
    }

    fn visit_while_stmt(&mut self, stmt: &Stmt) -> Result<(), SemanticAnalyserError> {
        match stmt {
            Stmt::While { condition, body } => {
                condition.accept_expr(self)?;
                
                for stmt in body {
                    stmt.accept_stmt(self)?;
                }

                return Ok(());
            }
            _ => {
                return Err(SemanticAnalyserError::DifferentStatement {
                    stmt: stmt.clone(),
                    expected: "while".to_string(),
                });
            }
        }
    }
}
