use std::fmt::Debug;

use crate::token::Token;

pub enum Expr<'a> {
    Literal(Literal<'a>),
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
            Expr::Literal(literal) => write!(f, "{:?}", literal),
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

pub enum Literal<'a> {
    Number(f32),
    String(&'a str),
    Bool(bool),
    Nil,
}

impl<'a> Debug for Literal<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Number(value) => write!(f, "{}", value),
            Literal::String(value) => write!(f, "{}", value),
            Literal::Bool(value) => match value {
                true => write!(f, "true"),
                false => write!(f, "false"),
            },
            Literal::Nil => write!(f, "nil"),
        }
    }
}
