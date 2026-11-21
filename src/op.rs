//! Comparison operators.

use std::fmt;
use std::str::FromStr;

use crate::error::{SizeError, SizeResult};

/// Comparison operator used by [`SizeFilter`](crate::SizeFilter).
///
/// The enum is `#[non_exhaustive]`: new operators may be added without a
/// breaking change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SizeOp {
    /// Greater than (`>`).
    Gt,
    /// Greater than or equal to (`>=`).
    Ge,
    /// Less than (`<`).
    Lt,
    /// Less than or equal to (`<=`).
    Le,
    /// Equal to (`=`).
    Eq,
}

impl SizeOp {
    /// All variants, in display order.
    pub const ALL: [SizeOp; 5] = [SizeOp::Gt, SizeOp::Ge, SizeOp::Lt, SizeOp::Le, SizeOp::Eq];

    /// Apply this operator to `value` and `threshold` (both in bytes).
    #[inline]
    #[must_use]
    pub fn applies(self, value: i64, threshold: i64) -> bool {
        match self {
            SizeOp::Gt => value > threshold,
            SizeOp::Ge => value >= threshold,
            SizeOp::Lt => value < threshold,
            SizeOp::Le => value <= threshold,
            SizeOp::Eq => value == threshold,
        }
    }

    /// Split a leading operator off `s`.
    ///
    /// Two-character operators are matched before their one-character prefixes.
    pub(crate) fn strip_prefix(s: &str) -> Option<(SizeOp, &str)> {
        const PREFIXES: &[(&str, SizeOp)] = &[
            (">=", SizeOp::Ge),
            ("<=", SizeOp::Le),
            (">", SizeOp::Gt),
            ("<", SizeOp::Lt),
            ("=", SizeOp::Eq),
        ];
        PREFIXES
            .iter()
            .find_map(|&(prefix, op)| s.strip_prefix(prefix).map(|rest| (op, rest)))
    }

    /// Whether `s` starts with a character that belongs to an operator.
    ///
    /// Used to distinguish a missing operator from an unsupported or
    /// malformed one (`!=`, `==`, `!1KiB`).
    pub(crate) fn looks_like_operator(s: &str) -> bool {
        s.starts_with(['<', '>', '=', '!'])
    }
}

impl fmt::Display for SizeOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            SizeOp::Gt => ">",
            SizeOp::Ge => ">=",
            SizeOp::Lt => "<",
            SizeOp::Le => "<=",
            SizeOp::Eq => "=",
        })
    }
}

impl FromStr for SizeOp {
    type Err = SizeError;

    /// Parse a bare operator such as `">="`.
    fn from_str(s: &str) -> SizeResult<Self> {
        let s = s.trim();
        if s.is_empty() {
            return Err(SizeError::EmptyInput);
        }
        match SizeOp::strip_prefix(s) {
            Some((op, "")) => Ok(op),
            Some(_) => Err(SizeError::InvalidOperator),
            None if SizeOp::looks_like_operator(s) => Err(SizeError::InvalidOperator),
            None => Err(SizeError::MissingOperator),
        }
    }
}
