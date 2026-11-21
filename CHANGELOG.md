# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2026-11-21

Correctness release. The public API changed; there is no compatibility shim.

### Changed

- Units are IEC binary units: added `KIB`, `MIB`, `GIB`, `TIB`, `PIB`, `EIB`
  and IEC display labels (`KiB`, `MiB`, …). Parsing accepts `KB`/`MB`/… as
  aliases for the binary units.
- `Size`'s `Display` is now the rounded, human-oriented form and is lossy.
  `Size::to_exact_string()` is the canonical form: it selects the largest unit
  that represents the value with at most two decimals and falls back to bytes
  otherwise. The exact output always parses back to the identical byte count.
  `Size::to_human()` exposes the human form as a method.
- `format_size` remains the rounded, one-decimal human formatter. It now
  promotes the unit when rounding reaches 1024.0 (`1_048_575` renders as
  `1.0MiB`, not `1024.0KiB`). Its lossiness is documented.
- Added checked arithmetic: `Size::{checked_add, checked_sub, checked_mul,
  checked_div, checked_rem, checked_neg}`. Operator traits use these and panic
  deterministically on overflow instead of wrapping in release builds.
- Constructors follow the unit names: `from_kib`, `from_mib`, `from_gib`,
  `from_tib`, `from_pib`, `from_eib`.
- `SizeFilter` implements `Hash`; `SizeOp` implements `FromStr`. The derived
  `Ord` on `SizeOp` was removed because declaration order is not meaningful.
- `SizeFilter`'s `Display` and serde use the exact form, so threshold round
  trips are lossless. `Size`'s serde also uses the exact form, independently of
  its human `Display`.
- Added `Size::to_human`, `Size::to_exact_string`, `TryFrom<u64> for Size`, and
  `SizeFilter::matches_u64` for file metadata and other unsigned byte counts.
- Added `SizeError::MissingValue`. Operator-only input such as `">="` now
  returns it instead of `EmptyInput`; `!`-prefixed input such as `"!1GiB"` now
  returns `InvalidOperator` instead of `MissingOperator`.
- `SizeOp` is now `#[non_exhaustive]`, so new operators may be added without a
  breaking change; `SizeOp::ALL` still lists the current five operators.
- Removed the unnecessary `serde/derive` dependency feature; the `serde`
  feature now only enables `serde`.
- Added `rust-version = "1.85"`, docs.rs `all-features`, and kept tests and the
  changelog in the published package.

### Fixed

- `format_size` no longer rounds `i64::MAX` up to the unparseable
  `8.0EiB`; the rounded output stays representable (`7.9EiB`).
- `parse_size("0.1000000000000000000GiB")` no longer panics in debug or returns
  a wrapped negative value in release. Fractional input is limited to 18
  digits and returns `InvalidNumber` beyond that; all arithmetic is checked.
- `parse_size` no longer overflows on large integer parts multiplied by a unit;
  values outside `i64` return `Overflow`.
- `-8EiB` (`i64::MIN`) parses correctly.
- Fractions with leading zeros (`0.05KiB`) are scaled correctly.
- Size filters round trip through `Display`, `FromStr`, and serde without
  changing their threshold (`>=1048575B` stays exact instead of becoming
  `>=1.0MiB`).
- `Size`'s human `Display` no longer changes the meaning of serde or filter
  round trips; the exact form is used where it matters.

### Migration

- `Size::to_string()` is now human-oriented and may be lossy. Use
  `Size::to_exact_string()` or serde when the value must round trip.
- `SizeFilter::to_string()` and serde remain exact; no migration is needed
  there.
- Code that matched `SizeOp` exhaustively must add a wildcard arm because the
  enum is now `#[non_exhaustive]`.
- `parse_size_filter(">=")` now returns `SizeError::MissingValue`; update
  callers that matched `EmptyInput` for operator-only input.

## [0.2.0] - 2026-06-29

### Changed

- **`SizeFilter` fields are now private** ([C-STRUCT-PRIVATE](https://rust-lang.github.io/api-guidelines/interoperability.html#types-are-send-and-sync-where-possible-c-send-sync)):
  - `op` and `bytes` fields are now private
  - Added `op()` getter method (returns `SizeOp`)
  - Added `bytes()` getter method (returns `i64`)
  - `new()` constructor remains unchanged

- **`SizeOp` implements `Hash`, `Ord`, `PartialOrd`** ([C-COMMON-TRAITS](https://rust-lang.github.io/api-guidelines/interoperability.html#commonly-used-types-should-be-the-same-c-common-traits)):
  ```rust
  use std::collections::HashSet;
  use sizefilter::SizeOp;
  
  let ops = HashSet::from([SizeOp::Ge, SizeOp::Lt]);
  assert!(ops.contains(&SizeOp::Ge));
  ```

### Migration Guide

**Struct field access** (breaking):
```rust
// Before (0.1.x)
let f: SizeFilter = ">=1GB".parse().unwrap();
assert_eq!(f.op, SizeOp::Ge);
assert_eq!(f.bytes, 1_073_741_824);

// After (0.2.0)
let f: SizeFilter = ">=1GB".parse().unwrap();
assert_eq!(f.op(), SizeOp::Ge);
assert_eq!(f.bytes(), 1_073_741_824);
```

## [0.1.2] - 2026-06-05

### Fixed

- `parse_size` uses integer arithmetic instead of f64 intermediate values to avoid floating-point precision issues (e.g., 1.5GB parses precisely)

### Added

- Support PB (pebibyte) and EB (exbibyte) unit parsing (long/short suffixes: PB/P, EB/E)
- Unit tests for PB/EB parsing and decimal precision
- Float precision loss demonstration test (`test_float_precision_loss`)

## [0.1.1] - 2026-06-04

- GitHub Actions CI workflow (build + test + fmt + clippy)
- Initial test suite for size parsing, filtering, formatting, and arithmetic.

## [0.1.0] - 2026-05-13

### Added

- Initial release of `sizefilter`.
- `parse_size`: parse human-readable size strings (`"1.5KB"`, `"500MB"`, `"1GB"`) to bytes.
- `format_size`: format raw byte counts to human-readable strings.
- `SizeFilter` / `SizeOp`: typed size filter with operators (`>=`, `>`, `<=`, `<`, `=`).
- `parse_size_filter`: parse combined operator + size strings (`">=500MB"`, `"<1GB"`).
- Size arithmetic support (add/subtract sizes).
