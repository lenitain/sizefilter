//! `sizefilter` — human readable size parsing, formatting, filtering, and
//! checked arithmetic.
//!
//! Sizes are byte counts (`i64`, negative values allowed for deltas). Units are
//! binary: 1 KiB = 1024 B. Parsing accepts `KB`/`MB`/… as legacy aliases for the
//! binary units, because many filesystem tools conventionally use them that
//! way. There is no decimal/SI mode.
//!
//! # Design
//!
//! * Parsing is checked: no input can panic or silently wrap.
//! * [`Size`]'s `Display` is the rounded, human-oriented form. It is lossy;
//!   [`Size::to_exact_string`] is the canonical form that parses back to the
//!   identical byte count.
//! * [`format_size`] is that same human formatter, exposed as a function.
//! * [`SizeFilter`] stores a byte threshold and uses the exact representation,
//!   so config round trips never change a threshold.
//!
//! # Scope
//!
//! The crate models filesystem byte sizes and one comparison expression such
//! as `>=1GiB`. Ranges are composed by the caller from two filters. It is not a
//! general unit converter, query language, or I/O layer.
//!
//! # Quick start
//!
//! ```
//! use sizefilter::prelude::*;
//!
//! assert_eq!(parse_size("1.5GiB").unwrap(), 1_610_612_736);
//!
//! let size: Size = "1.5GiB".parse().unwrap();
//! assert_eq!(size.bytes(), 1_610_612_736);
//! assert_eq!(size.to_string(), "1.5GiB");
//! assert_eq!(size.to_exact_string(), "1.5GiB");
//!
//! let filter: SizeFilter = ">=500MiB".parse().unwrap();
//! assert!(filter.matches(1_000_000_000));
//! assert!(!filter.matches(100_000_000));
//! assert!(filter.matches_u64(u64::MAX));
//! ```
//!
//! # Units
//!
//! ```
//! use sizefilter::{parse_size, GIB, KIB, MIB};
//!
//! assert_eq!(parse_size("1KiB").unwrap(), KIB);
//! assert_eq!(parse_size("1KB").unwrap(), KIB);   // legacy binary alias
//! assert_eq!(parse_size("1M").unwrap(), MIB);
//! assert_eq!(parse_size("1.5GiB").unwrap(), 3 * GIB / 2);
//! ```
//!
//! `KB` is not the SI kilobyte. Use `KiB` when the distinction matters.
//!
//! Fractions are truncated toward zero at byte granularity:
//!
//! ```
//! use sizefilter::parse_size;
//!
//! assert_eq!(parse_size("0.1KiB").unwrap(), 102);
//! assert_eq!(parse_size("-1.5KiB").unwrap(), -1536);
//! ```
//!
//! # Formatting
//!
//! ```
//! use sizefilter::{format_size, Size, MIB};
//!
//! assert_eq!(format_size(1536), "1.5KiB");
//! assert_eq!(format_size(MIB - 1), "1.0MiB"); // rounding promotes the unit
//!
//! // `Display` is human-oriented and lossy.
//! let odd = Size::from_bytes(1_048_575);
//! assert_eq!(odd.to_string(), "1.0MiB");
//!
//! // The exact form round trips, even when no unit represents it neatly.
//! assert_eq!(odd.to_exact_string(), "1048575B");
//! assert_eq!(odd.to_exact_string().parse::<Size>().unwrap(), odd);
//! ```
//!
//! # Unsigned sizes
//!
//! File metadata exposes `u64`. Convert with `TryFrom` or use the
//! [`SizeFilter::matches_u64`] entry point:
//!
//! ```
//! use sizefilter::{Size, SizeError, SizeFilter, MIB};
//!
//! assert_eq!(Size::try_from(1024u64).unwrap().bytes(), 1024);
//! assert_eq!(Size::try_from(u64::MAX), Err(SizeError::Overflow));
//! assert!(SizeFilter::ge(MIB).matches_u64(u64::MAX));
//! ```
//!
//! # Checked arithmetic
//!
//! ```
//! use sizefilter::{Size, MIB};
//!
//! let a = Size::from_bytes(MIB);
//! assert_eq!(a.checked_add(a), Some(Size::from_bytes(2 * MIB)));
//! assert_eq!(Size::from_bytes(i64::MAX).checked_add(a), None);
//! ```
//!
//! # Serde
//!
//! With the `serde` feature, [`Size`] and [`SizeFilter`] serialize to their
//! canonical string and deserialize through their parsers. Round trips are
//! exact.
//!
//! ```toml
//! [dependencies]
//! sizefilter = { version = "0.3", features = ["serde"] }
//! ```

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod error;
mod filter;
mod format;
mod op;
mod parse;
mod size;
mod units;

pub use error::{SizeError, SizeResult};
pub use filter::{SizeFilter, parse_size_filter};
pub use format::format_size;
pub use op::SizeOp;
pub use parse::parse_size;
pub use size::Size;
pub use units::{EIB, GIB, KIB, MIB, PIB, TIB};

/// Easy import of the crate's most common items.
pub mod prelude {
    pub use crate::{
        EIB, GIB, KIB, MIB, PIB, Size, SizeError, SizeFilter, SizeOp, SizeResult, TIB, format_size,
        parse_size, parse_size_filter,
    };
}
