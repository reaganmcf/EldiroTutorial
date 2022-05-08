use lexer::TokenKind;
use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum ParseError {
    #[error("Expected an expression")]
    #[diagnostic(
        code(nak::cant_parse_primary_expr),
        help("Change this into an expression")
    )]
    ExpectedExpression(#[source_code] NamedSource, #[label] SourceSpan),

    #[error("Expected token {1}")]
    #[diagnostic(code(nak::expected_token))]
    ExpectedToken(
        #[source_code] NamedSource,
        String,
        TokenKind,
        #[label("Consider adding '{2}' here")] SourceSpan,
    ),

    #[error("Unexpected EOF")]
    #[diagnostic(
        code(nak::unexpected_eof),
        help("Expected more tokens, but none were found")
    )]
    UnexpectedEof(#[source_code] NamedSource, #[label] SourceSpan),

    #[error("Invalid assignment target")]
    #[diagnostic(
        code(nak::invalid_assign_target),
        help("You can only assign to variables")
    )]
    InvalidAssignmentTarget(#[source_code] NamedSource, #[label] SourceSpan)
}
