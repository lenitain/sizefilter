//! Error type shared by every parser in this crate.

use std::error::Error;
use std::fmt;

/// Errors produced while parsing size expressions and filters.
///
/// Variants carry no heap-allocated data. The enum is `#[non_exhaustive]`; new
/// variants may be added without a breaking change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SizeError {
    /// The input is empty or contains only whitespace.
    EmptyInput,
    /// The filter expression does not begin with a comparison operator.
    MissingOperator,
    /// The expression begins with a malformed comparison operator.
    InvalidOperator,
    /// The expression ends after its comparison operator.
    MissingValue,
    /// The numeric part is malformed (empty, multiple dots, junk characters).
    InvalidNumber,
    /// The unit suffix is not recognised.
    UnknownUnit,
    /// The value does not fit in an `i64` byte count.
    Overflow,
}

impl fmt::Display for SizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            SizeError::EmptyInput => "empty input",
            SizeError::MissingOperator => "size filter must start with one of >, >=, <, <=, =",
            SizeError::InvalidOperator => "invalid comparison operator",
            SizeError::MissingValue => "missing value after comparison operator",
            SizeError::InvalidNumber => "invalid number",
            SizeError::UnknownUnit => "unknown size unit",
            SizeError::Overflow => "value out of range",
        })
    }
}

impl Error for SizeError {}

/// `Result` alias for size parsing operations.
pub type SizeResult<T> = Result<T, SizeError>;
