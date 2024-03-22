use paste::paste;

use crate::{expr::Expr, stmt_visitor, tokens::Token};

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

stmt_visitor!(Expression, Print, Var, Block);