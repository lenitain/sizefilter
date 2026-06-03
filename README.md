# sizefilter

[![Crates.io](https://img.shields.io/crates/v/sizefilter.svg)](https://crates.io/crates/sizefilter)
[![Docs.rs](https://docs.rs/sizefilter/badge.svg)](https://docs.rs/sizefilter)

Human-readable size string parsing, formatting, **arithmetic**, and **filtering with comparison operators** — `">=1GB"`, `"<500KB"`, `"=0"`.

## Installation

```bash
cargo add sizefilter
```

## Features

- **Parse**: `"1GB"`, `"500KB"`, `"1024"`, `"1.5MB"`, `"1PB"`, `"1EB"` → `i64` bytes
- **Format**: `1073741824` → `"1.0GB"`, `-1024` → `"-1.0KB"`
- **Filter**: `SizeFilter::ge(1_073_741_824)` or `">=1GB".parse::<SizeFilter>()`
- **Arithmetic**: `+`, `-`, `*`, `/`, `Neg` on [`Size`] — all byte-level
- **Serde** (optional): serialize/deserialize [`Size`] & [`SizeFilter`] as strings
- **`i64`** (not `u64`) — negative sizes supported
- **Zero heap allocation** in parsing and error paths — `SizeError` is a ZST in `.rodata`
- **Binary units** (1 KB = 1024 B), matching filesystem convention
- **`#[must_use]`** annotations throughout — no silently ignored results

## Testing

```bash
cargo test
```

79 tests (66 unit + 13 doc-tests) covering parsing, formatting, filtering,
arithmetic, edge cases (whitespace, case sensitivity, negative values,
sub-byte decimals, extreme ranges, invalid inputs, PB/EB units, float precision),
round-trip consistency, and serde round-trip.

## Quick example

```rust
use sizefilter::prelude::*;

// Parse a human-readable size to bytes
let bytes = parse_size("1.5GB").unwrap();
assert_eq!(bytes, 1_610_612_736);

// Using the Size newtype
let s: Size = "2GB".parse().unwrap();
assert_eq!(s.bytes(), 2_147_483_648);
assert_eq!(s.to_string(), "2.0GB");

// Parse a filter expression
let f: SizeFilter = ">=500MB".parse().unwrap();
assert!(f.matches(bytes));
assert!(!f.matches(100_000));

// Use convenience constructors
let f = SizeFilter::lt(GB);
assert!(f.matches(500 * MB));

// Arithmetic
assert_eq!(Size::from_mb(2) + Size::from_kb(512), Size::from_bytes(2_097_152 + 524_288));

// PB/EB support
assert_eq!(parse_size("1PB").unwrap(), PB);
assert_eq!(parse_size("1EB").unwrap(), EB);
```

## Modules

```
src/
├── lib.rs    — Crate root, prelude module
└── size.rs   — Size, SizeOp, SizeFilter, SizeError, constants, all ops
```

## Prelude

`use sizefilter::prelude::*` brings in the most common types and functions:

```rust
use sizefilter::prelude::*;
// Now available:
//   Size, SizeFilter, SizeOp, SizeError, SizeResult
//   KB, MB, GB, TB, PB, EB
//   parse_size(), format_size(), parse_size_filter()
```

## Error handling

All fallible operations return `Result<T, SizeError>`. The `SizeError` enum is
`#[non_exhaustive]` — new variants may be added in minor releases.

```rust
use sizefilter::{parse_size, SizeError};

match parse_size("1XB") {
    Err(SizeError::UnknownUnit) => println!("unknown unit"),
    Err(SizeError::InvalidNumber) => println!("bad number"),
    Err(SizeError::EmptyInput) => println!("empty"),
    _ => {}
}
```

## License

[MIT License](./LICENSE)
