use std::fmt::Debug;

use crate::token::Token;

pub enum Expr<'a> {
    // Literal -> need to figure out how to best implement this bad boy in rust
    Binary {
        left: Box<Expr<'a>>,
        right: Box<Expr<'a>>,
        operator: Token<'a>,
    },
    Grouping(Box<Expr<'a>>),
    Unary {
        operator: Token<'a>,
        right: Box<Expr<'a>>,
    },
}

impl<'a> Debug for Expr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary {
                left,
                right,
                operator,
            } => write!(f, "({:?} {:?} {:?})", operator, left, right),
            Expr::Grouping(expr) => write!(f, "(group {:?})", expr),
            Expr::Unary { operator, right } => write!(f, "({:?} {:?})", operator, right),
        }
    }
}
