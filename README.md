# sizefilter

Human-readable size parsing, formatting, filtering, and checked arithmetic.

[![Crates.io](https://img.shields.io/crates/v/sizefilter.svg)](https://crates.io/crates/sizefilter)
[![Docs.rs](https://docs.rs/sizefilter/badge.svg)](https://docs.rs/sizefilter)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![CI](https://github.com/lenitain/sizefilter/actions/workflows/ci.yml/badge.svg)](https://github.com/lenitain/sizefilter/actions/workflows/ci.yml)

## Overview

`sizefilter` turns strings such as `1.5GiB`, `>=500MiB`, and `1024` into byte
counts and typed filters.

- Units are binary: 1 KiB = 1024 B. `KB`/`MB`/… are accepted as legacy aliases
  for the binary units, matching many filesystem tools. They are **not** SI
  kilobytes and megabytes.
- Parsing is checked. Malformed input returns an error; it never panics, wraps,
  or silently loses the high bits.
- `Size`'s `Display` is the rounded, human-oriented form. It is lossy; use
  `Size::to_exact_string()` for the canonical form that parses back to the same
  byte count. Filters and serde always use the exact form.
- Filter thresholds keep the exact representation, so config and serde round
  trips never change a threshold.
- Arithmetic is checked (`checked_add`, …); operators panic deterministically
  instead of wrapping in release builds.

Requires Rust 1.85 or newer.

## Scope

The crate models filesystem byte sizes and **one comparison expression** such
as `>=1GiB`. A range is composed by the caller from two filters.

It is not a general unit converter, query language, or I/O layer:

- no decimal/SI mode and no automatic unit guessing;
- no ranges, `AND`/`OR`, or `!=` grammar; combine two filters instead;
- no locale-aware formatting or adaptive precision;
- no configuration loading, environment access, or CLI parsing.

## Semantics

- **Units:** `B`, `KiB`, `MiB`, `GiB`, `TiB`, `PiB`, `EiB`. `K`/`KB`/`KiB`
  all mean 1024 bytes; `KB` is a legacy filesystem alias.
- **Fractions:** decimal input is truncated toward zero at byte granularity;
  `0.1KiB` is 102 bytes. At most 18 fractional digits are accepted.
- **Sign:** `Size` is an `i64`; negative values represent deltas. File metadata
  is `u64`, so use `Size::try_from(u64)` and `SizeFilter::matches_u64`.
- **Display:** `Size` displays its human form. `Size::to_exact_string()` and
  `Size::to_human()` make the choice explicit. `SizeFilter` and serde use the
  exact form.
- **Filters:** one operator plus one threshold. Supported operators are `>`,
  `>=`, `<`, `<=`, and `=`.

## Usage

```toml
[dependencies]
sizefilter = "0.3"
```

### Parsing

```rust
use sizefilter::{parse_size, GIB, KIB};

assert_eq!(parse_size("1KiB").unwrap(), KIB);
assert_eq!(parse_size("1KB").unwrap(), KIB);     // legacy binary alias
assert_eq!(parse_size("1.5GiB").unwrap(), 1_610_612_736);
assert_eq!(parse_size("1 GB").unwrap(), GIB);
assert_eq!(parse_size("0.1KiB").unwrap(), 102);  // truncated to whole bytes
```

### The `Size` type

```rust
use sizefilter::{Size, KIB, MIB};

let size: Size = "1.5KiB".parse().unwrap();
assert_eq!(size.bytes(), 1536);
assert_eq!(size.to_string(), "1.5KiB"); // human form (coincidentally exact)

// `Display` is lossy; the exact form round trips.
let odd = Size::from_bytes(1_610_612_737);
assert_eq!(odd.to_string(), "1.5GiB");
assert_eq!(odd.to_exact_string(), "1610612737B");
assert_eq!(odd.to_exact_string().parse::<Size>().unwrap(), odd);

assert_eq!(Size::from_bytes(MIB) + Size::from_bytes(KIB), Size::from_bytes(MIB + KIB));
assert_eq!(Size::from_bytes(i64::MAX).checked_add(Size::from_bytes(1)), None);
```

### Formatting

```rust
use sizefilter::{format_size, Size, MIB};

assert_eq!(format_size(1536), "1.5KiB");
assert_eq!(format_size(MIB - 1), "1.0MiB"); // rounding promotes the unit
assert_eq!(Size::from_bytes(1536).to_human(), "1.5KiB");
```

### Filters

```rust
use sizefilter::{SizeFilter, SizeOp, GIB, MIB};

let filter: SizeFilter = ">=500MiB".parse().unwrap();
assert_eq!(filter.op(), SizeOp::Ge);
assert!(filter.matches(1_000_000_000));
assert!(!filter.matches(100_000_000));

// Odd thresholds stay exact in both directions.
let odd: SizeFilter = ">=1048575B".parse().unwrap();
assert_eq!(odd.to_string(), ">=1048575B");
assert!(odd.matches(1_048_575));
assert!(!odd.matches(1_048_574));

// A range is two filters.
let at_least_mib = SizeFilter::ge(MIB);
let below_gib = SizeFilter::lt(GIB);
assert!(at_least_mib.matches(2 * MIB) && below_gib.matches(2 * MIB));
assert!(!at_least_mib.matches(MIB / 2));
```

### Unsigned file sizes

```rust
use sizefilter::{Size, SizeError, SizeFilter, MIB};

assert_eq!(Size::try_from(1024u64).unwrap().bytes(), 1024);
assert_eq!(Size::try_from(u64::MAX), Err(SizeError::Overflow));

// Values above i64::MAX are greater than every representable threshold.
assert!(SizeFilter::ge(MIB).matches_u64(u64::MAX));
```

### Serde

```toml
[dependencies]
sizefilter = { version = "0.3", features = ["serde"] }
```

```rust
use sizefilter::{Size, SizeFilter};

let json = serde_json::to_string(&SizeFilter::ge(1_048_575)).unwrap();
assert_eq!(json, "\">=1048575B\"");
assert_eq!(serde_json::from_str::<SizeFilter>(&json).unwrap(), SizeFilter::ge(1_048_575));

// `Size` serde uses the exact form even when `Display` is human.
let size = Size::from_bytes(1_048_575);
assert_eq!(serde_json::to_string(&size).unwrap(), "\"1048575B\"");
```

## Errors

```rust
use sizefilter::{parse_size, parse_size_filter, SizeError};

assert_eq!(parse_size(""), Err(SizeError::EmptyInput));
assert_eq!(parse_size("1XB"), Err(SizeError::UnknownUnit));
assert_eq!(parse_size("8EiB"), Err(SizeError::Overflow)); // > i64::MAX
assert_eq!(parse_size_filter(">="), Err(SizeError::MissingValue));
assert_eq!(parse_size_filter("!1GiB"), Err(SizeError::InvalidOperator));
```

## License

MIT
