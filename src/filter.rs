//! Size filters: an operator plus a byte threshold.

use std::fmt;
use std::str::FromStr;

use crate::error::{SizeError, SizeResult};
use crate::op::SizeOp;
use crate::parse::parse_size;

/// A size filter such as `>=1GiB`, `<500KiB`, or `=0`.
///
/// Thresholds are byte counts. `Display` uses
/// [`Size::to_exact_string`](crate::Size::to_exact_string), so a filter round
/// trips through `to_string`/`parse` and through serde without changing its
/// threshold.
///
/// ```
/// use sizefilter::{SizeFilter, SizeOp, GIB};
///
/// let f: SizeFilter = ">=1GiB".parse().unwrap();
/// assert_eq!(f.op(), SizeOp::Ge);
/// assert_eq!(f.bytes(), GIB);
/// assert!(f.matches(GIB));
///
/// // Odd byte counts stay exact.
/// let odd = SizeFilter::ge(1_048_575);
/// assert_eq!(odd.to_string(), ">=1048575B");
/// assert_eq!(odd.to_string().parse::<SizeFilter>().unwrap(), odd);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SizeFilter {
    op: SizeOp,
    bytes: i64,
}

impl SizeFilter {
    /// Create a filter from an operator and a byte threshold.
    #[inline]
    #[must_use]
    pub const fn new(op: SizeOp, bytes: i64) -> Self {
        SizeFilter { op, bytes }
    }

    /// The comparison operator.
    #[inline]
    #[must_use]
    pub const fn op(self) -> SizeOp {
        self.op
    }

    /// The byte threshold.
    #[inline]
    #[must_use]
    pub const fn bytes(self) -> i64 {
        self.bytes
    }

    /// Filter: `value > bytes`.
    #[inline]
    #[must_use]
    pub const fn gt(bytes: i64) -> Self {
        SizeFilter::new(SizeOp::Gt, bytes)
    }

    /// Filter: `value >= bytes`.
    #[inline]
    #[must_use]
    pub const fn ge(bytes: i64) -> Self {
        SizeFilter::new(SizeOp::Ge, bytes)
    }

    /// Filter: `value < bytes`.
    #[inline]
    #[must_use]
    pub const fn lt(bytes: i64) -> Self {
        SizeFilter::new(SizeOp::Lt, bytes)
    }

    /// Filter: `value <= bytes`.
    #[inline]
    #[must_use]
    pub const fn le(bytes: i64) -> Self {
        SizeFilter::new(SizeOp::Le, bytes)
    }

    /// Filter: `value == bytes`.
    #[inline]
    #[must_use]
    pub const fn eq(bytes: i64) -> Self {
        SizeFilter::new(SizeOp::Eq, bytes)
    }

    /// Check whether a signed byte count passes this filter.
    #[inline]
    #[must_use]
    pub fn matches(self, value: i64) -> bool {
        self.op.applies(value, self.bytes)
    }

    /// Check whether an unsigned byte count passes this filter.
    ///
    /// This is the natural entry point for [`std::fs::Metadata::len`]. Values
    /// above [`i64::MAX`] are greater than every representable threshold, so
    /// only `>` and `>=` can match them.
    #[inline]
    #[must_use]
    pub fn matches_u64(self, value: u64) -> bool {
        match i64::try_from(value) {
            Ok(value) => self.matches(value),
            Err(_) => matches!(self.op, SizeOp::Gt | SizeOp::Ge),
        }
    }
}

impl fmt::Display for SizeFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.op)?;
        crate::format::write_exact(f, self.bytes)
    }
}

impl FromStr for SizeFilter {
    type Err = SizeError;

    fn from_str(s: &str) -> SizeResult<Self> {
        parse_size_filter(s)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for SizeFilter {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SizeFilter {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

/// Parse a size filter expression such as `">=1GiB"` or `"<500KiB"`.
///
/// Whitespace after the operator is allowed.
///
/// ```
/// use sizefilter::{parse_size_filter, SizeOp};
///
/// let f = parse_size_filter(">= 1GiB").unwrap();
/// assert_eq!(f.op(), SizeOp::Ge);
/// assert!(f.matches(2 * 1024 * 1024 * 1024));
/// ```
///
/// # Errors
///
/// Returns [`SizeError::MissingOperator`] when no operator is present,
/// [`SizeError::InvalidOperator`] for malformed or unsupported operators,
/// [`SizeError::MissingValue`] when the operator has no value, and the errors
/// of [`parse_size`] for the value part.
pub fn parse_size_filter(s: &str) -> SizeResult<SizeFilter> {
    let s = s.trim();
    if s.is_empty() {
        return Err(SizeError::EmptyInput);
    }
    let (op, rest) = match SizeOp::strip_prefix(s) {
        Some(parts) => parts,
        None if SizeOp::looks_like_operator(s) => return Err(SizeError::InvalidOperator),
        None => return Err(SizeError::MissingOperator),
    };
    let rest = rest.trim();
    if rest.is_empty() {
        return Err(SizeError::MissingValue);
    }
    if SizeOp::looks_like_operator(rest) {
        return Err(SizeError::InvalidOperator);
    }
    let bytes = parse_size(rest)?;
    Ok(SizeFilter::new(op, bytes))
}
