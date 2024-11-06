use std::fmt::Debug;

use miette::{SourceOffset, SourceSpan};

#[derive(Clone, PartialEq)]
pub enum TokenType<'a> {
    // Single-character tokens
    LeftParen,  // (
    RightParen, // )
    LeftBrace,  // {
    RightBrace, // }
    Comma,      // ,
    Dot,        // .
    Minus,      // -
    Plus,       // +
    Semicolon,  // ;
    Slash,      // /
    Star,       // *

    // One or two character tokens
    Bang,         // !
    BangEqual,    // !=
    Equal,        // =
    EqualEqual,   // ==
    Greater,      // >
    GreaterEqual, // >=
    Less,         // <
    LessEqual,    // <=

    // Literals
    Identifier(&'a str),
    String(&'a str),
    Number(f32),

    // Keywords
    And,    // and
    Class,  // class
    Else,   // else
    False,  // false
    Fun,    // fun
    For,    // for
    If,     // if
    Nil,    // nil
    Or,     // or
    Print,  // print
    Return, // return
    Super,  // super
    This,   // this
    True,   // true
    Var,    // var
    While,  // while

    Eof,
}

impl<'a> TokenType<'a> {
    pub fn get_token_type(literal: &'a str) -> TokenType<'a> {
        match literal {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "fun" => TokenType::Fun,
            "for" => TokenType::For,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier(literal),
        }
    }
}

impl<'a> Debug for TokenType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Single-character tokens
            TokenType::LeftParen => write!(f, "("),
            TokenType::RightParen => write!(f, ")"),
            TokenType::LeftBrace => write!(f, "{{"),
            TokenType::RightBrace => write!(f, "}}"),
            TokenType::Comma => write!(f, ","),
            TokenType::Dot => write!(f, "."),
            TokenType::Minus => write!(f, "-"),
            TokenType::Plus => write!(f, "+"),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::Slash => write!(f, "/"),
            TokenType::Star => write!(f, "*"),

            // One or two character tokens
            TokenType::Bang => write!(f, "!"),
            TokenType::BangEqual => write!(f, "!="),
            TokenType::Equal => write!(f, "="),
            TokenType::EqualEqual => write!(f, "=="),
            TokenType::Greater => write!(f, ">"),
            TokenType::GreaterEqual => write!(f, ">="),
            TokenType::Less => write!(f, "<"),
            TokenType::LessEqual => write!(f, "<="),

            // Literals
            TokenType::Identifier(ident) => write!(f, "{}", ident),
            TokenType::String(value) => write!(f, "{}", value),
            TokenType::Number(value) => write!(f, "{}", value),

            // Keywords
            TokenType::And => write!(f, "and"),
            TokenType::Class => write!(f, "class"),
            TokenType::Else => write!(f, "else"),
            TokenType::False => write!(f, "false"),
            TokenType::Fun => write!(f, "fun"),
            TokenType::For => write!(f, "for"),
            TokenType::If => write!(f, "if"),
            TokenType::Nil => write!(f, "nil"),
            TokenType::Or => write!(f, "or"),
            TokenType::Print => write!(f, "print"),
            TokenType::Return => write!(f, "return"),
            TokenType::Super => write!(f, "super"),
            TokenType::This => write!(f, "this"),
            TokenType::True => write!(f, "true"),
            TokenType::Var => write!(f, "var"),
            TokenType::While => write!(f, "while"),
            TokenType::Eof => write!(f, "eof"),
        }
    }
}

#[derive(Clone)]
pub struct Token<'a> {
    pub token_type: TokenType<'a>,
    pub literal: &'a str,
    pub range: Range,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType<'a>, literal: &'a str, range: Range) -> Self {
        Self {
            token_type,
            literal,
            range,
        }
    }
}

impl<'a> Debug for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.token_type)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

impl Range {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn source_span(&self) -> SourceSpan {
        SourceSpan::new(SourceOffset::from(self.start), self.end - self.start)
    }
}
