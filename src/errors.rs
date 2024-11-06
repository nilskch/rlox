use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Debug, Clone, Error, Diagnostic)]
pub enum ScannerError {
    #[error("Illegal token")]
    #[diagnostic(code(scanner::illegal_token_error))]
    IllegalToken {
        #[source_code]
        src: String,
        #[label("this token is illegal")]
        span: SourceSpan,
    },

    #[error("Unterminated string")]
    #[diagnostic(code(scanner::unterminated_string_error))]
    UnterminatedString {
        #[source_code]
        src: String,
        #[label("this string is unterminated")]
        span: SourceSpan,
    },

    #[error("Failed to parse number")]
    #[diagnostic(code(scanner::number_conversion_error))]
    NumberConversion {
        #[source_code]
        src: String,
        #[label("could not parse this into a number")]
        span: SourceSpan,
    },
}

#[derive(Debug, Clone, Error, Diagnostic)]
pub enum ParserError {}
