//! Size parsing.

use crate::error::{SizeError, SizeResult};
use crate::units::unit_multiplier;

/// Maximum number of fractional decimal digits accepted.
///
/// Eighteen digits keep every intermediate value inside `i128` while covering
/// any precision a byte count can meaningfully carry.
const MAX_FRACTION_DIGITS: usize = 18;

/// Parse a human readable size into bytes.
///
/// Units are binary and case-insensitive: `B`, `K`/`KB`/`KiB`, `M`/`MB`/`MiB`,
/// `G`/`GB`/`GiB`, `T`/`TB`/`TiB`, `P`/`PB`/`PiB`, `E`/`EB`/`EiB`. An omitted
/// unit means bytes. Surrounding whitespace and whitespace between the number
/// and the unit are ignored.
///
/// Decimal values are truncated toward zero at byte granularity:
/// `0.1KiB` is 102 bytes. At most 18 fractional digits are accepted.
///
/// ```
/// use sizefilter::{parse_size, GIB, KIB};
///
/// assert_eq!(parse_size("1024").unwrap(), 1024);
/// assert_eq!(parse_size("1KiB").unwrap(), KIB);
/// assert_eq!(parse_size("1.5KiB").unwrap(), 1536);
/// assert_eq!(parse_size("1.5GiB").unwrap(), 1_610_612_736);
/// assert_eq!(parse_size("1 GB").unwrap(), GIB);
/// assert_eq!(parse_size("-1.5KiB").unwrap(), -1536);
/// assert_eq!(parse_size("0.1KiB").unwrap(), 102);
/// ```
///
/// # Errors
///
/// * [`SizeError::EmptyInput`] for blank input.
/// * [`SizeError::InvalidNumber`] for a malformed numeric part.
/// * [`SizeError::UnknownUnit`] for an unrecognised suffix.
/// * [`SizeError::Overflow`] when the result does not fit in `i64`.
pub fn parse_size(s: &str) -> SizeResult<i64> {
    let s = s.trim();
    if s.is_empty() {
        return Err(SizeError::EmptyInput);
    }

    // The sign belongs to the number, so skip it while looking for the unit.
    let start = usize::from(matches!(s.as_bytes().first(), Some(b'+' | b'-')));
    let (number, unit) = match s[start..].find(|c: char| c.is_ascii_alphabetic()) {
        Some(position) => s.split_at(start + position),
        None => (s, ""),
    };
    let number = parse_number(number.trim())?;
    let multiplier = i128::from(unit_multiplier(unit.trim()).ok_or(SizeError::UnknownUnit)?);
    let whole = number
        .integer
        .checked_mul(multiplier)
        .ok_or(SizeError::Overflow)?;
    let fractional = number
        .fraction
        .checked_mul(multiplier)
        .ok_or(SizeError::Overflow)?
        / number.scale;
    let magnitude = whole.checked_add(fractional).ok_or(SizeError::Overflow)?;
    let bytes = if number.negative {
        i64::try_from(-magnitude).map_err(|_| SizeError::Overflow)?
    } else {
        i64::try_from(magnitude).map_err(|_| SizeError::Overflow)?
    };

    Ok(bytes)
}

struct Number {
    negative: bool,
    integer: i128,
    fraction: i128,
    scale: i128,
}

fn parse_number(text: &str) -> SizeResult<Number> {
    let (negative, body) = match text.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, text.strip_prefix('+').unwrap_or(text)),
    };

    let (integer, fraction) = match body.split_once('.') {
        Some((integer, fraction)) => {
            if fraction.is_empty() {
                return Err(SizeError::InvalidNumber);
            }
            (integer, fraction)
        }
        None => (body, ""),
    };

    if integer.is_empty() && fraction.is_empty() {
        return Err(SizeError::InvalidNumber);
    }
    if fraction.len() > MAX_FRACTION_DIGITS {
        return Err(SizeError::InvalidNumber);
    }
    if !integer.bytes().all(|b| b.is_ascii_digit()) || !fraction.bytes().all(|b| b.is_ascii_digit())
    {
        return Err(SizeError::InvalidNumber);
    }

    let integer = if integer.is_empty() {
        0
    } else {
        integer.parse().map_err(|_| SizeError::InvalidNumber)?
    };
    let digits = u32::try_from(fraction.len()).map_err(|_| SizeError::Overflow)?;
    let fraction = if fraction.is_empty() {
        0
    } else {
        fraction.parse().map_err(|_| SizeError::InvalidNumber)?
    };
    let scale = 10i128.checked_pow(digits).ok_or(SizeError::Overflow)?;

    Ok(Number {
        negative,
        integer,
        fraction,
        scale,
    })
}
