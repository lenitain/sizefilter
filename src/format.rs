//! Size formatting.
//!
//! Two representations exist on purpose:
//!
//! * [`format_size`] is a rounded, human-oriented rendering (one decimal
//!   place). It is lossy, but its output always parses back to a size in the
//!   same unit. [`Size`](crate::Size)'s `Display` uses this form.
//! * [`Size::to_exact_string`](crate::Size::to_exact_string) is the exact,
//!   canonical form: the output parses back to the identical byte count. When
//!   no unit represents the value exactly with at most two decimals, it falls
//!   back to bytes. Filters and serde use this form.

use std::fmt;

use crate::units::UNITS;

/// Format a byte count for humans, using one decimal place.
///
/// Units are binary (`KiB`, `MiB`, …). Rounding is half away from zero and the
/// unit is promoted if rounding reaches 1024.0, so `1_048_575` renders as
/// `1.0MiB`, not `1024.0KiB`.
///
/// This function is explicitly lossy; use
/// [`Size::to_exact_string`](crate::Size::to_exact_string) when the output
/// must round trip exactly. The rounded output is always representable: at the
/// positive `i64` boundary the last tenth is truncated (`i64::MAX` renders as
/// `7.9EiB`), so the result still parses.
///
/// ```
/// use sizefilter::{format_size, KIB, MIB};
///
/// assert_eq!(format_size(0), "0B");
/// assert_eq!(format_size(1023), "1023B");
/// assert_eq!(format_size(KIB), "1.0KiB");
/// assert_eq!(format_size(1536), "1.5KiB");
/// assert_eq!(format_size(MIB - 1), "1.0MiB");
/// ```
#[must_use]
pub fn format_size(bytes: i64) -> String {
    if bytes == 0 {
        return String::from("0B");
    }
    let sign = if bytes < 0 { "-" } else { "" };
    let absolute = u128::from(bytes.unsigned_abs());

    let Some(mut index) = UNITS
        .iter()
        .position(|&(unit, _)| absolute >= u128::from(unit.unsigned_abs()))
    else {
        return format!("{sign}{absolute}B");
    };

    loop {
        let (unit, name) = UNITS[index];
        let unit = u128::from(unit.unsigned_abs());
        // Round half away from zero to one decimal place without floating point.
        let mut tenths = (absolute * 10 + unit / 2) / unit;
        if tenths >= 10_240 && index > 0 {
            index -= 1;
            continue;
        }
        // Rounding must not produce a value the parser rejects. This can only
        // happen for the largest positive sizes, where rounding reaches 2^63.
        if bytes > 0 && tenths * unit / 10 > u128::from(i64::MAX.unsigned_abs()) {
            tenths -= 1;
        }
        return format!("{sign}{}.{}{name}", tenths / 10, tenths % 10);
    }
}

/// Write the exact, canonical representation of `bytes`.
pub(crate) fn write_exact<W: fmt::Write>(f: &mut W, bytes: i64) -> fmt::Result {
    if bytes == 0 {
        return f.write_str("0B");
    }
    let sign = if bytes < 0 { "-" } else { "" };
    let absolute = u128::from(bytes.unsigned_abs());

    for &(unit, name) in UNITS {
        let unit = u128::from(unit.unsigned_abs());
        if absolute < unit {
            continue;
        }
        for decimals in 0..=2u32 {
            let scale = 10u128.pow(decimals);
            let scaled = absolute * scale;
            if scaled % unit != 0 {
                continue;
            }
            let value = scaled / unit;
            if decimals == 0 {
                return write!(f, "{sign}{value}{name}");
            }
            let integer = value / scale;
            let fraction = value % scale;
            return write!(
                f,
                "{sign}{integer}.{fraction:0width$}{name}",
                width = decimals as usize
            );
        }
        // Not representable with two decimals: bytes are always exact.
        return write!(f, "{sign}{absolute}B");
    }
    write!(f, "{sign}{absolute}B")
}
