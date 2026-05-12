//! `sizefilter` — human-readable size string parsing, formatting, and filtering.
//!
//! Handles strings like `">=1GB"`, `"<500KB"`, `"1024"` — parsing them into
//! byte counts, formatting bytes back to human-readable, and applying
//! comparison filters.
//!
//! Unlike existing crates (`human-size`, `bytesize`, `parse-size`), this crate
//! focuses on **filter operators** — the `SizeOp`/`SizeFilter` types that let
//! you write `>=1GB` or `<500KB` as a single filter expression.
//!
//! # Quick Start
//!
//! ```
//! use sizefilter::prelude::*;
//!
//! // Parse a raw size
//! assert_eq!(parse_size("1GB").unwrap(), GB);
//!
//! // Using the Size newtype
//! let s: Size = "1.5GB".parse().unwrap();
//! assert_eq!(s.bytes(), GB + GB / 2);
//! assert_eq!(s.to_string(), "1.5GB");
//!
//! // Parse a filter expression
//! let f: SizeFilter = ">=1GB".parse().unwrap();
//! assert_eq!(f.op, SizeOp::Ge);
//! assert_eq!(f.bytes, GB);
//! assert!(f.matches(GB + 1));
//! ```
//!
//! # Parsing sizes
//!
//! `parse_size` supports various unit suffixes:
//!
//! ```
//! use sizefilter::parse_size;
//!
//! assert_eq!(parse_size("1024").unwrap(), 1024);       // raw bytes
//! assert_eq!(parse_size("1B").unwrap(), 1);            // byte
//! assert_eq!(parse_size("1KB").unwrap(), 1024);        // kilobyte
//! assert_eq!(parse_size("1MB").unwrap(), 1_048_576);   // megabyte
//! assert_eq!(parse_size("1GB").unwrap(), 1_073_741_824); // gigabyte
//! assert_eq!(parse_size("1TB").unwrap(), 1_099_511_627_776); // terabyte
//! ```
//!
//! Short suffixes (`K`, `M`, `G`, `T`) also work:
//!
//! ```
//! use sizefilter::parse_size;
//!
//! assert_eq!(parse_size("1K").unwrap(), 1024);
//! assert_eq!(parse_size("1M").unwrap(), 1_048_576);
//! assert_eq!(parse_size("1G").unwrap(), 1_073_741_824);
//! assert_eq!(parse_size("1T").unwrap(), 1_099_511_627_776);
//! ```
//!
//! Negative sizes and decimals are supported:
//!
//! ```
//! use sizefilter::parse_size;
//!
//! assert_eq!(parse_size("-1024").unwrap(), -1024);
//! assert_eq!(parse_size("-1KB").unwrap(), -1024);
//! assert_eq!(parse_size("0.5KB").unwrap(), 512);
//! assert_eq!(parse_size("1.5GB").unwrap(), 1_610_612_736);
//! ```
//!
//! # Formatting sizes
//!
//! `format_size` converts bytes back to human-readable:
//!
//! ```
//! use sizefilter::format_size;
//!
//! assert_eq!(format_size(0), "0B");
//! assert_eq!(format_size(500), "500B");
//! assert_eq!(format_size(1024), "1.0KB");
//! assert_eq!(format_size(1_048_576), "1.0MB");
//! assert_eq!(format_size(1_073_741_824), "1.0GB");
//! assert_eq!(format_size(-1024), "-1.0KB");
//! ```
//!
//! # Size newtype
//!
//! [`Size`] wraps `i64` with `FromStr` / `Display`:
//!
//! ```
//! use sizefilter::Size;
//!
//! let s: Size = "2GB".parse().unwrap();
//! assert_eq!(s.bytes(), 2_147_483_648);
//! assert_eq!(s.to_string(), "2.0GB");
//!
//! // Construct from constants
//! use sizefilter::{GB, MB};
//! let s = Size::from_mb(512);
//! assert_eq!(s, Size::from_bytes(512 * MB));
//!
//! // Implements From<i64> and Into<i64>
//! let bytes: i64 = s.into();
//! assert_eq!(bytes, 512 * MB);
//! ```
//!
//! # Arithmetic
//!
//! [`Size`] supports `+`, `-`, `*`, `/`, `Neg`, and their `Assign` variants.
//! All operations work at the byte level, so units stay consistent:
//!
//! ```
//! use sizefilter::{Size, MB, KB};
//!
//! let a = Size::from_mb(2);
//! let b = Size::from_kb(512);
//!
//! assert_eq!((a + b).bytes(), 2_097_152i64 + 512 * KB);
//! assert_eq!((a - b).bytes(), 2_097_152i64 - 512 * KB);
//! assert_eq!((a * 3).bytes(), 6 * MB);
//! assert_eq!((a / 2).bytes(), MB);
//!
//! // Mixed: i64 + Size
//! assert_eq!((1024i64 + Size::from_kb(1)).bytes(), 2048);
//! ```
//!
//! # Size filters with operators
//!
//! [`SizeFilter`] implements `FromStr` for ergonomic parsing:
//!
//! ```
//! use sizefilter::SizeFilter;
//!
//! let f: SizeFilter = ">=1GB".parse().unwrap();
//! assert_eq!(f.to_string(), ">=1.0GB");
//! assert!(f.matches(2_000_000_000));
//! assert!(!f.matches(500_000_000));
//! ```
//!
//! Or use convenience constructors without importing [`SizeOp`]:
//!
//! ```
//! use sizefilter::SizeFilter;
//!
//! let f = SizeFilter::ge(1_073_741_824);  // >=1GB
//! assert!(f.matches(2_000_000_000));
//!
//! let f = SizeFilter::lt(524_288_000);    // <500MB
//! assert!(!f.matches(1_000_000_000));
//! ```
//!
//! Or use the free function:
//!
//! ```
//! use sizefilter::{parse_size_filter, SizeOp};
//!
//! let f = parse_size_filter("<500KB").unwrap();
//! assert_eq!(f.op, SizeOp::Lt);
//! ```
//!
//! # Serde support
//!
//! With the `serde` feature enabled, both [`Size`] and [`SizeFilter`]
//! can be serialized and deserialized as human-readable strings:
//!
//! ```toml
//! [dependencies]
//! sizefilter = { version = "0.1", features = ["serde"] }
//! ```
//!
//! ```ignore
//! # #[cfg(feature = "serde")] {
//! use sizefilter::{Size, SizeFilter};
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize, Deserialize)]
//! struct Config {
//!     max_log_size: SizeFilter,
//!     min_file_size: Size,
//! }
//! # }
//! ```

#![forbid(unsafe_code)]

mod size;

pub use size::*;

/// Easy import of the crate's most common items.
///
/// ```
/// use sizefilter::prelude::*;
///
/// let s: Size = "1GB".parse().unwrap();
/// let f: SizeFilter = ">=500MB".parse().unwrap();
/// ```
pub mod prelude {
    pub use super::{EB, GB, KB, MB, PB, TB};
    pub use super::{Size, SizeError, SizeFilter, SizeOp, SizeResult};
    pub use super::{format_size, parse_size, parse_size_filter};
}
