//! Core types, constants, and parsing logic.
//!
//! All string constants in errors live in `.rodata` — no heap `String` allocation
//! in error paths. `parse_size` avoids `to_uppercase()` by using byte-level
//! case-insensitive comparison.

use std::error::Error;
use std::fmt;
use std::str::FromStr;

// ── unit constants ───────────────────────────────────────────────────────────

/// Bytes in one kibibyte (2¹⁰).
pub const KB: i64 = 1 << 10;
/// Bytes in one mebibyte (2²⁰).
pub const MB: i64 = 1 << 20;
/// Bytes in one gibibyte (2³⁰).
pub const GB: i64 = 1 << 30;
/// Bytes in one tebibyte (2⁴⁰).
pub const TB: i64 = 1 << 40;
/// Bytes in one pebibyte (2⁵⁰).
pub const PB: i64 = 1 << 50;
/// Bytes in one exbibyte (2⁶⁰).
pub const EB: i64 = 1 << 60;

// ── SizeOp ───────────────────────────────────────────────────────────────────

/// Size comparison operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeOp {
    /// Greater than (`>`)
    Gt,
    /// Greater than or equal to (`>=`)
    Ge,
    /// Less than (`<`)
    Lt,
    /// Less than or equal to (`<=`)
    Le,
    /// Equal to (`=`)
    Eq,
}

impl SizeOp {
    /// All variants, in declaration order.
    pub const ALL: [SizeOp; 5] = [SizeOp::Gt, SizeOp::Ge, SizeOp::Lt, SizeOp::Le, SizeOp::Eq];

    /// Apply this operator to two values.
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

// ── SizeFilter ───────────────────────────────────────────────────────────────

/// A size filter with operator (e.g., `>=1GB`, `<500KB`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SizeFilter {
    pub op: SizeOp,
    pub bytes: i64,
}

impl SizeFilter {
    /// Create a new filter from an operator and byte threshold.
    #[inline]
    #[must_use]
    pub const fn new(op: SizeOp, bytes: i64) -> Self {
        SizeFilter { op, bytes }
    }

    /// Filter: `value > threshold`.
    #[inline]
    #[must_use]
    pub const fn gt(bytes: i64) -> Self {
        SizeFilter {
            op: SizeOp::Gt,
            bytes,
        }
    }

    /// Filter: `value >= threshold`.
    #[inline]
    #[must_use]
    pub const fn ge(bytes: i64) -> Self {
        SizeFilter {
            op: SizeOp::Ge,
            bytes,
        }
    }

    /// Filter: `value < threshold`.
    #[inline]
    #[must_use]
    pub const fn lt(bytes: i64) -> Self {
        SizeFilter {
            op: SizeOp::Lt,
            bytes,
        }
    }

    /// Filter: `value <= threshold`.
    #[inline]
    #[must_use]
    pub const fn le(bytes: i64) -> Self {
        SizeFilter {
            op: SizeOp::Le,
            bytes,
        }
    }

    /// Filter: `value == threshold`.
    #[inline]
    #[must_use]
    pub const fn eq(bytes: i64) -> Self {
        SizeFilter {
            op: SizeOp::Eq,
            bytes,
        }
    }

    /// Check whether `value` (in bytes) passes this filter.
    #[inline]
    #[must_use]
    pub fn matches(self, value: i64) -> bool {
        self.op.applies(value, self.bytes)
    }
}

impl fmt::Display for SizeFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.op, format_size(self.bytes))
    }
}

impl FromStr for SizeFilter {
    type Err = SizeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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

// ── Size newtype ─────────────────────────────────────────────────────────────

/// A byte size that can be parsed from / formatted to a human-readable string.
///
/// # Examples
///
/// ```
/// use sizefilter::{Size, GB};
///
/// let s: Size = "1.5GB".parse().unwrap();
/// assert_eq!(s.bytes(), 1_610_612_736);
/// assert_eq!(s.to_string(), "1.5GB");
///
/// // Arithmetic with constants
/// assert_eq!(s, Size::from_bytes(GB + GB / 2));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Size(i64);

impl Size {
    /// Zero bytes.
    pub const ZERO: Size = Size(0);

    /// Create a `Size` from a raw byte count.
    #[inline]
    #[must_use]
    pub const fn from_bytes(bytes: i64) -> Self {
        Size(bytes)
    }

    /// Create a `Size` from a quantity of kilobytes (binary: 1024).
    #[inline]
    #[must_use]
    pub const fn from_kb(kb: i64) -> Self {
        Size(kb * KB)
    }

    /// Create a `Size` from a quantity of megabytes (binary: 1024²).
    #[inline]
    #[must_use]
    pub const fn from_mb(mb: i64) -> Self {
        Size(mb * MB)
    }

    /// Create a `Size` from a quantity of gigabytes (binary: 1024³).
    #[inline]
    #[must_use]
    pub const fn from_gb(gb: i64) -> Self {
        Size(gb * GB)
    }

    /// Create a `Size` from a quantity of terabytes (binary: 1024⁴).
    #[inline]
    #[must_use]
    pub const fn from_tb(tb: i64) -> Self {
        Size(tb * TB)
    }

    /// Return the raw byte count.
    #[inline]
    #[must_use]
    pub const fn bytes(self) -> i64 {
        self.0
    }
}

impl From<i64> for Size {
    #[inline]
    fn from(v: i64) -> Self {
        Size(v)
    }
}

impl From<Size> for i64 {
    #[inline]
    fn from(s: Size) -> Self {
        s.0
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format_size(self.0))
    }
}

impl FromStr for Size {
    type Err = SizeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_size(s).map(Size)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Size {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Size {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

// ── arithmetic ──────────────────────────────────────────────────────────────

impl std::ops::Add for Size {
    type Output = Size;
    #[inline]
    fn add(self, rhs: Size) -> Size {
        Size(self.0 + rhs.0)
    }
}

impl std::ops::Add<i64> for Size {
    type Output = Size;
    #[inline]
    fn add(self, rhs: i64) -> Size {
        Size(self.0 + rhs)
    }
}

impl std::ops::Add<Size> for i64 {
    type Output = Size;
    #[inline]
    fn add(self, rhs: Size) -> Size {
        Size(self + rhs.0)
    }
}

impl std::ops::AddAssign for Size {
    #[inline]
    fn add_assign(&mut self, rhs: Size) {
        self.0 += rhs.0;
    }
}

impl std::ops::AddAssign<i64> for Size {
    #[inline]
    fn add_assign(&mut self, rhs: i64) {
        self.0 += rhs;
    }
}

impl std::ops::Sub for Size {
    type Output = Size;
    #[inline]
    fn sub(self, rhs: Size) -> Size {
        Size(self.0 - rhs.0)
    }
}

impl std::ops::Sub<i64> for Size {
    type Output = Size;
    #[inline]
    fn sub(self, rhs: i64) -> Size {
        Size(self.0 - rhs)
    }
}

impl std::ops::Sub<Size> for i64 {
    type Output = Size;
    #[inline]
    fn sub(self, rhs: Size) -> Size {
        Size(self - rhs.0)
    }
}

impl std::ops::SubAssign for Size {
    #[inline]
    fn sub_assign(&mut self, rhs: Size) {
        self.0 -= rhs.0;
    }
}

impl std::ops::SubAssign<i64> for Size {
    #[inline]
    fn sub_assign(&mut self, rhs: i64) {
        self.0 -= rhs;
    }
}

impl std::ops::Mul<i64> for Size {
    type Output = Size;
    #[inline]
    fn mul(self, rhs: i64) -> Size {
        Size(self.0 * rhs)
    }
}

impl std::ops::Mul<Size> for i64 {
    type Output = Size;
    #[inline]
    fn mul(self, rhs: Size) -> Size {
        Size(self * rhs.0)
    }
}

impl std::ops::MulAssign<i64> for Size {
    #[inline]
    fn mul_assign(&mut self, rhs: i64) {
        self.0 *= rhs;
    }
}

impl std::ops::Div<i64> for Size {
    type Output = Size;
    #[inline]
    fn div(self, rhs: i64) -> Size {
        Size(self.0 / rhs)
    }
}

impl std::ops::DivAssign<i64> for Size {
    #[inline]
    fn div_assign(&mut self, rhs: i64) {
        self.0 /= rhs;
    }
}

impl std::ops::Rem<i64> for Size {
    type Output = Size;
    #[inline]
    fn rem(self, rhs: i64) -> Size {
        Size(self.0 % rhs)
    }
}

impl std::ops::RemAssign<i64> for Size {
    #[inline]
    fn rem_assign(&mut self, rhs: i64) {
        self.0 %= rhs;
    }
}

impl std::ops::Neg for Size {
    type Output = Size;
    #[inline]
    fn neg(self) -> Size {
        Size(-self.0)
    }
}

// ── SizeError ────────────────────────────────────────────────────────────────

/// Errors that can occur during size parsing and formatting.
///
/// All variants carry zero heap-allocated data — error strings are
/// `&'static str` literals in `.rodata`.
///
/// This enum is `#[non_exhaustive]` — new variants may be added in
/// minor releases without breaking changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SizeError {
    /// No operator found in filter string.
    MissingOperator,
    /// Unable to parse the numeric part of a size string.
    InvalidNumber,
    /// Unknown or unsupported size unit suffix.
    UnknownUnit,
    /// Empty input string.
    EmptyInput,
}

impl fmt::Display for SizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            SizeError::MissingOperator => {
                "size filter must start with an operator (>=, >, <=, <, =)"
            }
            SizeError::InvalidNumber => "failed to parse number",
            SizeError::UnknownUnit => "unknown size unit",
            SizeError::EmptyInput => "empty input",
        })
    }
}

impl Error for SizeError {}

/// `Result` type alias for `sizefilter` operations.
pub type SizeResult<T> = Result<T, SizeError>;

// ── parsing ──────────────────────────────────────────────────────────────────

/// Parse a size filter string like `">=1GB"`, `"<500KB"`, `"=0"`.
///
/// Operator is required — returns error if missing.
///
/// # Errors
///
/// Returns [`SizeError::MissingOperator`] if no operator is found,
/// or [`SizeError`] variants from size parsing.
pub fn parse_size_filter(s: &str) -> SizeResult<SizeFilter> {
    let s = s.trim();
    let (op, rest) = if let Some(r) = s.strip_prefix(">=") {
        (SizeOp::Ge, r)
    } else if let Some(r) = s.strip_prefix("<=") {
        (SizeOp::Le, r)
    } else if let Some(r) = s.strip_prefix('>') {
        (SizeOp::Gt, r)
    } else if let Some(r) = s.strip_prefix('<') {
        (SizeOp::Lt, r)
    } else if let Some(r) = s.strip_prefix('=') {
        (SizeOp::Eq, r)
    } else {
        return Err(SizeError::MissingOperator);
    };
    let bytes = parse_size(rest)?;
    Ok(SizeFilter { op, bytes })
}

/// Parse human-readable size string to bytes.
///
/// Supports: `"1GB"`, `"500KB"`, `"1024"`, `"1B"`, `"1K"`, `"1M"`, `"1G"`, `"1T"`.
/// Uses binary units (1KB = 1024 bytes).
///
/// No heap allocation during parsing — unit comparison is done via
/// byte-level `eq_ignore_ascii_case`.
///
/// # Errors
///
/// Returns [`SizeError::InvalidNumber`] if the numeric part cannot be parsed,
/// or [`SizeError::UnknownUnit`] if the unit suffix is not recognized.
pub fn parse_size(size_str: &str) -> SizeResult<i64> {
    let size_str = size_str.trim();
    if size_str.is_empty() {
        return Err(SizeError::EmptyInput);
    }

    // Find split between number and alphabetic unit.  Negative sign "-" at
    // position 0 is part of the number, so skip it when searching.
    let search_start = usize::from(size_str.starts_with('-'));
    let alpha_pos = size_str[search_start..].find(|c: char| c.is_ascii_alphabetic());

    let (num_part, unit) = match alpha_pos {
        Some(pos) => size_str.split_at(search_start + pos),
        None => (size_str, ""),
    };

    let num: f64 = num_part
        .trim()
        .parse()
        .map_err(|_| SizeError::InvalidNumber)?;

    let multiplier = unit_multiplier(unit.trim()).ok_or(SizeError::UnknownUnit)?;

    Ok((num * multiplier as f64) as i64)
}

/// Map a unit string to its byte multiplier, or `None` if unknown.
///
/// Comparison is ASCII case-insensitive — no allocation.
fn unit_multiplier(unit: &str) -> Option<i64> {
    if unit.is_empty() || unit.eq_ignore_ascii_case("B") {
        Some(1)
    } else if unit.eq_ignore_ascii_case("K") || unit.eq_ignore_ascii_case("KB") {
        Some(KB)
    } else if unit.eq_ignore_ascii_case("M") || unit.eq_ignore_ascii_case("MB") {
        Some(MB)
    } else if unit.eq_ignore_ascii_case("G") || unit.eq_ignore_ascii_case("GB") {
        Some(GB)
    } else if unit.eq_ignore_ascii_case("T") || unit.eq_ignore_ascii_case("TB") {
        Some(TB)
    } else {
        None
    }
}

// ── formatting ───────────────────────────────────────────────────────────────

/// Format size (in bytes) to human-readable string.
///
/// Uses binary units: `B`, `KB`, `MB`, `GB`, `TB`, `PB`, `EB`.
/// The returned `String` is the output — unavoidable allocation.
#[must_use]
pub fn format_size(size: i64) -> String {
    let abs = size.unsigned_abs();
    let prefix = if size < 0 { "-" } else { "" };

    if abs >= 1 << 60 {
        format!("{}{:.1}EB", prefix, (abs as f64) / ((1u64 << 60) as f64))
    } else if abs >= 1 << 50 {
        format!("{}{:.1}PB", prefix, (abs as f64) / ((1u64 << 50) as f64))
    } else if abs >= 1 << 40 {
        format!("{}{:.1}TB", prefix, (abs as f64) / ((1u64 << 40) as f64))
    } else if abs >= 1 << 30 {
        format!("{}{:.1}GB", prefix, (abs as f64) / ((1u64 << 30) as f64))
    } else if abs >= 1 << 20 {
        format!("{}{:.1}MB", prefix, (abs as f64) / ((1u64 << 20) as f64))
    } else if abs >= 1 << 10 {
        format!("{}{:.1}KB", prefix, (abs as f64) / ((1u64 << 10) as f64))
    } else {
        format!("{}{}B", prefix, abs)
    }
}
