use miette::{SourceOffset, SourceSpan};

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

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub(crate) token_type: TokenType<'a>,
    pub(crate) literal: &'a str,
    pub(crate) range: Range,
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
    pub(crate) start: usize,
    pub(crate) end: usize,
}

impl Range {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn source_span(&self) -> SourceSpan {
        SourceSpan::new(SourceOffset::from(self.start), self.end - self.start)
    }
}
