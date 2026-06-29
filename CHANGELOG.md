# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

## Unreleased

### Changed

- Internal: `if-else` chains replaced with `match` and table-driven lookups in `parse_size_filter`, `unit_multiplier`, and `format_size` for clarity and maintainability
- Internal: prelude re-exports use `crate::` instead of `super::` for better readability

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
