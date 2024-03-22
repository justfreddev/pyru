use paste::paste;

use crate::{expr::Expr, tokens::Token};

macro_rules! stmt_visitor {
    ( $($stmts:literal),+ ; $($idents:ident),+ ) => {
        pub trait Visitor<T> {
            $(
                paste! {
                    fn [<visit_ $stmts _stmt>](&mut self, stmt: &Stmt) -> T;
                }
            )+
        }
    
        impl Stmt {
            pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
                match self {
                    $(
                        Stmt::$idents { .. } => {
                            paste! {
                                visitor.[<visit_ $stmts _stmt>](self)
                            }
                        },
                    )+
                }
            }
        }
    };
}

#[derive(Clone, Debug)]
pub enum Stmt {
    Expression {
        expression: Expr
    },
    Print {
        expression: Expr
    },
    Var {
        name: Token,
        initializer: Option<Expr>
    },
    Block {
        statements: Vec<Stmt>
    }
}

stmt_visitor!("expression", "print", "var", "block"; Expression, Print, Var, Block);