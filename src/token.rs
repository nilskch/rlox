#[derive(Debug, Clone, PartialEq)]
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
    Identifier,
    String(&'a str),
    Number(f32),

    // Keywords
    And,   // and
    Class, //class
    Else,  // else
    False, //false
    Fun,   // fun
    For,   // for
    If,    // if
    Nil,
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

impl Range {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}
