use crate::{
    errors::ScannerError,
    token::{Range, Token, TokenType},
};

pub struct Scanner<'a> {
    source: &'a str,
    position: usize,
    tokens: Vec<Token<'a>>,
    errors: Vec<ScannerError>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            position: 0,
            tokens: vec![],
            errors: vec![],
        }
    }

    pub fn scan_tokens(&'a mut self) -> Result<Vec<Token>, Vec<ScannerError>> {
        loop {
            let start = self.position;
            let ch = match self.advance() {
                Some(ch) => ch,
                None => break,
            };

            match ch {
                '(' => self.add_token(TokenType::LeftParen, start),
                ')' => self.add_token(TokenType::RightParen, start),
                '{' => self.add_token(TokenType::LeftBrace, start),
                '}' => self.add_token(TokenType::RightBrace, start),
                ',' => self.add_token(TokenType::Comma, start),
                '.' => self.add_token(TokenType::Dot, start),
                '-' => self.add_token(TokenType::Minus, start),
                '+' => self.add_token(TokenType::Plus, start),
                ';' => self.add_token(TokenType::Semicolon, start),
                '*' => self.add_token(TokenType::Star, start),
                '!' => {
                    if self.peek_is('=') {
                        self.advance();
                        self.add_token(TokenType::BangEqual, start);
                    } else {
                        self.add_token(TokenType::Bang, start);
                    }
                }
                '=' => {
                    if self.peek_is('=') {
                        self.advance();
                        self.add_token(TokenType::EqualEqual, start);
                    } else {
                        self.add_token(TokenType::Equal, start);
                    }
                }
                '<' => {
                    if self.peek_is('=') {
                        self.advance();
                        self.add_token(TokenType::LessEqual, start);
                    } else {
                        self.add_token(TokenType::Less, start);
                    }
                }
                '>' => {
                    if self.peek_is('=') {
                        self.advance();
                        self.add_token(TokenType::GreaterEqual, start);
                    } else {
                        self.add_token(TokenType::Greater, start);
                    }
                }
                '/' => {
                    if self.peek_is('/') {
                        while !self.is_at_end() && !self.peek_is('\n') {
                            self.advance();
                        }
                    } else {
                        self.add_token(TokenType::Slash, start);
                    }
                }
                '"' => self.scan_string(start),
                ' ' | '\t' | '\r' | '\n' => {}
                ch => {
                    if ch.is_ascii_digit() {
                        self.scan_number(start)
                    } else if ch.is_ascii_alphabetic() || ch == '_' {
                        self.scan_identifier(start);
                    } else {
                        let err = ScannerError::IllegalTokenError {
                            src: self.source.into(),
                            span: Range::new(start, start).source_span(),
                        };
                        self.errors.push(err);
                    }
                }
            }
        }
        if !self.errors.is_empty() {
            Err(self.errors.clone())
        } else {
            Ok(self.tokens.clone())
        }
    }

    fn scan_identifier(&mut self, start: usize) {
        while self.peek_is_identifier_char() {
            self.advance();
        }
        self.add_token(
            TokenType::get_token_type(&self.source[start..self.position]),
            start,
        );
    }

    fn peek_is_identifier_char(&self) -> bool {
        let ch = self.peek();
        ch.is_ascii_alphanumeric() || ch == '_'
    }

    fn scan_string(&mut self, start: usize) {
        while !self.is_at_end() && !self.peek_is('"') {
            self.advance();
        }

        if self.is_at_end() {
            let err = ScannerError::UnterminatedStringError {
                src: self.source.into(),
                span: Range::new(start, self.position - 1).source_span(),
            };
            self.errors.push(err);
        } else {
            self.advance();
            self.add_token(
                TokenType::String(&self.source[start + 1..self.position - 1]),
                start,
            );
        }
    }

    fn scan_number(&mut self, start: usize) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek_is('.') && self.peek_peek().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        match self.source[start..self.position].parse::<f32>() {
            Ok(number) => self.add_token(TokenType::Number(number), start),
            Err(_) => {
                let err = ScannerError::NumberConversionError {
                    src: self.source.into(),
                    span: Range::new(start, self.position).source_span(),
                };
                self.errors.push(err);
            }
        }
    }

    fn peek(&self) -> char {
        match self.source[self.position..].chars().next() {
            Some(ch) => ch,
            None => '\0',
        }
    }

    fn peek_peek(&self) -> char {
        match self.source[self.position..].chars().nth(1) {
            Some(ch) => ch,
            None => '\0',
        }
    }

    fn peek_is(&self, expected: char) -> bool {
        match self.source[self.position..].chars().next() {
            Some(peek_ch) => peek_ch == expected,
            None => false,
        }
    }

    fn add_token(&mut self, token_type: TokenType<'a>, start: usize) {
        let token = Token::new(
            token_type,
            &self.source[start..self.position],
            Range::new(start, self.position),
        );
        self.tokens.push(token);
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.source[self.position..].chars().next();
        if let Some(ch) = ch {
            self.position += ch.len_utf8();
        }
        ch
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.source.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_single_literal_tokens() {
        let source = r#"( ) { } , . - + * ; ! = < >
            != == <= >=
            // comment that gets ignored
            / "foo bar" "" "foo
bar baz"
123
123.
123.123.12300
some_ident
while
if
foo
        "#;
        let expected_tokens = [
            Token::new(TokenType::LeftParen, "(", Range::new(0, 1)),
            Token::new(TokenType::RightParen, ")", Range::new(2, 3)),
            Token::new(TokenType::LeftBrace, "{", Range::new(4, 5)),
            Token::new(TokenType::RightBrace, "}", Range::new(6, 7)),
            Token::new(TokenType::Comma, ",", Range::new(8, 9)),
            Token::new(TokenType::Dot, ".", Range::new(10, 11)),
            Token::new(TokenType::Minus, "-", Range::new(12, 13)),
            Token::new(TokenType::Plus, "+", Range::new(14, 15)),
            Token::new(TokenType::Star, "*", Range::new(16, 17)),
            Token::new(TokenType::Semicolon, ";", Range::new(18, 19)),
            Token::new(TokenType::Bang, "!", Range::new(20, 21)),
            Token::new(TokenType::Equal, "=", Range::new(22, 23)),
            Token::new(TokenType::Less, "<", Range::new(24, 25)),
            Token::new(TokenType::Greater, ">", Range::new(26, 27)),
            Token::new(TokenType::BangEqual, "!=", Range::new(40, 42)),
            Token::new(TokenType::EqualEqual, "==", Range::new(43, 45)),
            Token::new(TokenType::LessEqual, "<=", Range::new(46, 48)),
            Token::new(TokenType::GreaterEqual, ">=", Range::new(49, 51)),
            Token::new(TokenType::Slash, "/", Range::new(105, 106)),
            Token::new(
                TokenType::String("foo bar"),
                "\"foo bar\"",
                Range::new(107, 116),
            ),
            Token::new(TokenType::String(""), "\"\"", Range::new(117, 119)),
            Token::new(
                TokenType::String("foo\nbar baz"),
                "\"foo\nbar baz\"",
                Range::new(120, 133),
            ),
            Token::new(TokenType::Number(123.0), "123", Range::new(134, 137)),
            Token::new(TokenType::Number(123.0), "123", Range::new(138, 141)),
            Token::new(TokenType::Dot, ".", Range::new(141, 142)),
            Token::new(TokenType::Number(123.123), "123.123", Range::new(143, 150)),
            Token::new(TokenType::Dot, ".", Range::new(150, 151)),
            Token::new(TokenType::Number(12300.0), "12300", Range::new(151, 156)),
            Token::new(
                TokenType::Identifier("some_ident"),
                "some_ident",
                Range::new(157, 167),
            ),
            Token::new(TokenType::While, "while", Range::new(168, 173)),
            Token::new(TokenType::If, "if", Range::new(174, 176)),
            Token::new(TokenType::Identifier("foo"), "foo", Range::new(177, 180)),
        ];

        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().expect("Expected no scanning errors");

        assert_eq!(
            tokens.len(),
            expected_tokens.len(),
            "The number of scanned tokens is wrong"
        );

        for (i, token) in tokens.iter().enumerate() {
            let expected_token = expected_tokens.get(i).unwrap();
            assert_eq!(
                expected_token.token_type, token.token_type,
                "wrong token type"
            );
            assert_eq!(expected_token.literal, token.literal, "wrong token literal");
            assert_eq!(expected_token.range, token.range, "wrong range");
        }
    }
}
