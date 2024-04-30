#[macro_export]
macro_rules! arithmetic {
    ( $operator:tt ; $num1:expr ; $num2:expr ) => {
        if let Value::Literal(LiteralType::Num(ln)) = $num1 {
            if let Value::Literal(LiteralType::Num(rn)) = $num2 {
                return Ok(Value::Literal(LiteralType::Num(ln $operator rn)));
            }
        }
    };
}

#[macro_export]
macro_rules! comparison {
    ( $operator:tt ; $num1:expr ; $num2:expr ) => {
        if let Value::Literal(LiteralType::Num(ln)) = $num1 {
            if let Value::Literal(LiteralType::Num(rn)) = $num2 {
                return Ok(
                    if ln $operator rn {
                        Value::Literal(LiteralType::True)
                    } else {
                        Value::Literal(LiteralType::False)
                    }
                );
            }
        }
    };
}

#[macro_export]
macro_rules! alteration {
    ( $self:ident ; $operator:tt ; $name:expr ; $value:expr ) => {
        if let Value::Literal(LiteralType::Num(n)) = $value {
            return $self.environment.borrow_mut().assign($name.clone(), Value::Literal(LiteralType::Num(n $operator 1.0)));
        };
        return Err(InterpreterError::ExpectedNumber);
    };
}

#[macro_export]
/// Populates the `kw` hashmap with the passed in keywords
macro_rules! keywords {
    ( $kw:expr ; $($kws:ident),+ ) => {
        $(
            let key = stringify!($kws).to_lowercase();
            $kw.insert(key, TokenType::$kws);
        )+
    };
}

#[macro_export]
macro_rules! stmt_visitor {
    ( $($stmts:ident),+ ) => {
        pub trait StmtVisitor<T> {
            $(
                paste! {
                    fn [<visit_ $stmts:lower _stmt>](&mut self, stmt: &Stmt) -> T;
                }
            )+
        }

        impl Stmt {
            pub fn accept_stmt<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
                match self {
                    $(
                        Stmt::$stmts { .. } => {
                            paste! {
                                visitor.[<visit_ $stmts:lower _stmt>](self)
                            }
                        },
                    )+
                }
            }
        }
    };
}

#[macro_export]
macro_rules! expr_visitor {
    ( $($exprs:ident),+ ) => {
        pub trait ExprVisitor<T> {
            $(
                paste! {
                    fn [<visit_ $exprs:lower _expr>](&mut self, expr: &Expr) -> T;
                }
            )+
        }

        impl Expr {
            pub fn accept_expr<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
                match self {
                    $(
                        Expr::$exprs { .. } => {
                            paste! {
                                visitor.[<visit_ $exprs:lower _expr>](self)
                            }
                        },
                    )+
                }
            }
        }
    };
}
