use std::collections::HashMap;

use crate::{
    error::SemanticAnalyserError,
    expr::{self, Expr},
    stmt::{self, Stmt},
};

#[derive(Debug)]
enum Symbol {
    Ident { initialised: bool },
}

#[derive(PartialEq)]
enum FunctionType {
    Function,
    None,
}

pub struct SemanticAnalyser {
    ast: Vec<Stmt>,
    symbol_tables: Vec<HashMap<String, Symbol>>,
    curr: usize,
    func_type: FunctionType,
}

impl SemanticAnalyser {
    pub fn new(ast: Vec<Stmt>) -> Self {
        let mut st = Vec::new();
        st.push(HashMap::<String, Symbol>::new());
        Self {
            ast,
            symbol_tables: st,
            curr: 0,
            func_type: FunctionType::None,
        }
    }

    pub fn run(&mut self) -> Result<(), SemanticAnalyserError> {
        for stmt in self.ast.clone() {
            stmt.accept_stmt(self)?;
        }
        Ok(())
    }

    fn begin_scope(&mut self) {
        let st: HashMap<String, Symbol> = HashMap::new();
        self.curr += 1;
        self.symbol_tables.push(st)
    }

    fn end_scope(&mut self) {
        self.curr -= 1;
        self.symbol_tables.pop();
    }

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
        false
    }

    fn check_defined(&mut self, ident_name: &String) -> bool {
        if let Some(sym) = self.symbol_tables[self.curr].get(ident_name) {
            match sym {
                Symbol::Ident {
                    initialised,
                } => {
                    if *initialised {
                        return true;
                    }
                    return false;
                }
            }
        }
        false
    }
}

impl expr::ExprVisitor<Result<(), SemanticAnalyserError>> for SemanticAnalyser {
    fn visit_alteration_expr(&mut self, expr: &Expr) -> Result<(), SemanticAnalyserError> {
        match expr {
            Expr::Alteration { name, .. } => {
                if self.check_declared(&name.lexeme) {
                    return Ok(());
                }
                Err(SemanticAnalyserError::VariableNotFound {
                    name: name.lexeme.clone(),
                })
            }
            _ => Err(SemanticAnalyserError::DifferentExpression {
                expr: expr.clone(),
                expected: "alteration".to_string(),
            }),
        }
    }

    fn visit_assign_expr(&mut self, expr: &Expr) -> Result<(), SemanticAnalyserError> {
        // Assigning a variable
        match expr {
            Expr::Assign { name, value } => {
                value.accept_expr(self)?;

                if self.check_declared(&name.lexeme) {
                    return Ok(());
                }

                Err(SemanticAnalyserError::VariableNotFound {
                    name: name.lexeme.clone(),
                })
            }
            _ => Err(SemanticAnalyserError::DifferentExpression {
                expr: expr.clone(),
                expected: "assign".to_string(),
            }),
        }
    }

    fn visit_binary_expr(&mut self, expr: &Expr) -> Result<(), SemanticAnalyserError> {
        match expr {
            Expr::Binary {
                left,
                operator: _,
                right,
            } => {
                left.accept_expr(self)?;
                right.accept_expr(self)?;
                Ok(())
            }
            _ => Err(SemanticAnalyserError::DifferentExpression {
                expr: expr.clone(),
                expected: "binary".to_string(),
            }),
        }
    }

    fn visit_call_expr(&mut self, expr: &Expr) -> Result<(), SemanticAnalyserError> {
        match expr {
            Expr::Call {
                callee,
                paren: _,
                arguments,
            } => {
                callee.accept_expr(self)?;

                for argument in arguments {
                    argument.accept_expr(self)?;
                }

                Ok(())
            }
            _ => Err(SemanticAnalyserError::DifferentExpression {
                expr: expr.clone(),
                expected: "call".to_string(),
            }),
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) -> Result<(), SemanticAnalyserError> {
        match expr {
            Expr::Grouping { expression } => {
                expression.accept_expr(self)?;
                Ok(())
            }
            _ => Err(SemanticAnalyserError::DifferentExpression {
                expr: expr.clone(),
                expected: "grouping".to_string(),
            }),
        }
    }

    fn visit_literal_expr(&mut self, expr: &Expr) -> Result<(), SemanticAnalyserError> {
        match expr {
            Expr::Literal { .. } => Ok(()),
            _ => Err(SemanticAnalyserError::DifferentExpression {
                expr: expr.clone(),
                expected: "literal".to_string(),
            }),
        }
    }

    fn visit_logical_expr(&mut self, expr: &Expr) -> Result<(), SemanticAnalyserError> {
        match expr {
            Expr::Logical {
                left,
                operator: _,
                right,
            } => {
                left.accept_expr(self)?;
                right.accept_expr(self)?;

                Ok(())
            }
            _ => Err(SemanticAnalyserError::DifferentExpression {
                expr: expr.clone(),
                expected: "logical".to_string(),
            }),
        }
    }

    fn visit_unary_expr(&mut self, expr: &Expr) -> Result<(), SemanticAnalyserError> {
        match expr {
            Expr::Unary { operator: _, right } => {
                right.accept_expr(self)?;
                Ok(())
            }
            _ => Err(SemanticAnalyserError::DifferentExpression {
                expr: expr.clone(),
                expected: "unary".to_string(),
            }),
        }
    }

    fn visit_var_expr(&mut self, expr: &Expr) -> Result<(), SemanticAnalyserError> {
        // The one to get the value from a variable
        match expr {
            Expr::Var { name } => {
                if self.check_declared(&name.lexeme) {
                    return Ok(());
                }
                Err(SemanticAnalyserError::VariableNotFound {
                    name: name.lexeme.clone(),
                })
            }
            _ => Err(SemanticAnalyserError::DifferentExpression {
                expr: expr.clone(),
                expected: "var".to_string(),
            }),
        }
    }
}

impl stmt::StmtVisitor<Result<(), SemanticAnalyserError>> for SemanticAnalyser {
    fn visit_block_stmt(&mut self, stmt: &Stmt) -> Result<(), SemanticAnalyserError> {
        match stmt {
            Stmt::Block { statements } => {
                self.begin_scope();
                for statement in statements {
                    statement.accept_stmt(self)?;
                }
                self.end_scope();
                Ok(())
            }
            _ => {
                return Err(SemanticAnalyserError::DifferentStatement {
                    stmt: stmt.clone(),
                    expected: "block".to_string(),
                })
            }
        }
    }

    fn visit_expression_stmt(&mut self, stmt: &Stmt) -> Result<(), SemanticAnalyserError> {
        match stmt {
            Stmt::Expression { expression } => {
                expression.accept_expr(self)?;
                Ok(())
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
                increment,
                body,
            } => {
                if let Some(initialiser) = initializer {
                    initialiser.accept_stmt(self)?;
                };

                condition.accept_expr(self)?;

                if let Some(incr) = increment {
                    incr.accept_expr(self)?;
                };

                body.accept_stmt(self)?;

                Ok(())
            }
            _ => Err(SemanticAnalyserError::DifferentStatement {
                stmt: stmt.clone(),
                expected: "for".to_string(),
            }),
        }
    }

    fn visit_function_stmt(&mut self, stmt: &Stmt) -> Result<(), SemanticAnalyserError> {
        // MAY BE RECURSIVE AND BREAK
        match stmt {
            Stmt::Function { name, params, body } => {
                // Define the function on the symbol table
                let sym = Symbol::Ident {
                    initialised: true,
                };
                if self.symbol_tables[self.curr].contains_key(&name.lexeme) {
                    return Err(SemanticAnalyserError::VariableAlreadyAssignedInScope {
                        name: name.lexeme.clone(),
                    });
                }
                self.symbol_tables[self.curr].insert(name.lexeme.clone(), sym);

                self.begin_scope(); // Begin the function scope for parameters and body

                self.func_type = FunctionType::Function;
                // For param in params
                for param in params {
                    let sym = Symbol::Ident {
                        initialised: true, // They're parameters so they should be
                    };
                    // Check if the parameter is already assigned in scope (if multiple parameters are same name)
                    if self.symbol_tables[self.curr].contains_key(&param.lexeme) {
                        return Err(SemanticAnalyserError::VariableAlreadyAssignedInScope {
                            name: param.lexeme.clone(),
                        });
                    }
                    self.symbol_tables[self.curr].insert(param.lexeme.clone(), sym);
                }

                // Analyse the body
                for statement in body {
                    statement.accept_stmt(self)?;
                }

                self.end_scope(); // End the function scope for parameters and body
                self.func_type = FunctionType::None;

                Ok(())
            }
            _ => {
                return Err(SemanticAnalyserError::DifferentStatement {
                    stmt: stmt.clone(),
                    expected: "function".to_string(),
                })
            }
        }
    }

    fn visit_if_stmt(&mut self, stmt: &Stmt) -> Result<(), SemanticAnalyserError> {
        match stmt {
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                condition.accept_expr(self)?;

                then_branch.accept_stmt(self)?;

                if let Some(e_branch) = else_branch {
                    e_branch.accept_stmt(self)?;
                };

                Ok(())
            }
            _ => Err(SemanticAnalyserError::DifferentStatement {
                stmt: stmt.clone(),
                expected: "if".to_string(),
            }),
        }
    }

    fn visit_print_stmt(&mut self, stmt: &Stmt) -> Result<(), SemanticAnalyserError> {
        match stmt {
            Stmt::Print { expression } => {
                expression.accept_expr(self)?;
                Ok(())
            }
            _ => Err(SemanticAnalyserError::DifferentStatement {
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

                Ok(())
            }
            _ => Err(SemanticAnalyserError::DifferentStatement {
                stmt: stmt.clone(),
                expected: "return".to_string(),
            }),
        }
    }

    fn visit_var_stmt(&mut self, stmt: &Stmt) -> Result<(), SemanticAnalyserError> {
        // The one for declaring a variable
        match stmt {
            Stmt::Var { name, initializer } => {
                // Check if the variable name has already been defined in scope
                if self.check_defined(&name.lexeme) {
                    return Err(SemanticAnalyserError::VariableAlreadyAssignedInScope {
                        name: name.lexeme.clone(),
                    });
                }

                // Pass through the initialiser if it exists
                if let Some(x) = initializer {
                    x.accept_expr(self)?;
                }

                // Declare the variable and say it's been initialised
                let sym = Symbol::Ident {
                    initialised: initializer.is_some(),
                };
                self.symbol_tables[self.curr].insert(name.lexeme.clone(), sym);

                Ok(())
            }
            _ => {
                return Err(SemanticAnalyserError::DifferentStatement {
                    stmt: stmt.clone(),
                    expected: "var".to_string(),
                })
            }
        }
    }

    fn visit_while_stmt(&mut self, stmt: &Stmt) -> Result<(), SemanticAnalyserError> {
        match stmt {
            Stmt::While { condition, body } => {
                condition.accept_expr(self)?;

                body.accept_stmt(self)?;

                Ok(())
            }
            _ => {
                return Err(SemanticAnalyserError::DifferentStatement {
                    stmt: stmt.clone(),
                    expected: "while".to_string(),
                })
            }
        }
    }
}