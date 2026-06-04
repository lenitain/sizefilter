# sizefilter

Human-readable size string parsing, formatting, arithmetic, and filtering with comparison operators.

[![Crates.io](https://img.shields.io/crates/v/sizefilter.svg)](https://crates.io/crates/sizefilter)
[![Docs.rs](https://docs.rs/sizefilter/badge.svg)](https://docs.rs/sizefilter)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![CI](https://github.com/lenitain/sizefilter/actions/workflows/ci.yml/badge.svg)](https://github.com/lenitain/sizefilter/actions/workflows/ci.yml)

## Overview

**sizefilter** provides human-readable size string parsing, formatting, arithmetic, and filtering with comparison operators. It supports binary units (1 KB = 1024 B) matching filesystem conventions, with zero heap allocation in parsing and error paths. The library offers comprehensive support for sizes from bytes to exabytes with intuitive string representations.

### Why sizefilter?

Unlike other size parsing libraries that only handle basic conversions, **sizefilter** provides a complete solution for working with human-readable sizes in Rust. It supports comparison operators (`>=1GB`, `<500KB`, `=0`), arithmetic operations, and seamless integration with serde for configuration files. The library's use of `i64` (not `u64`) allows for negative sizes, and its zero-allocation design makes it suitable for performance-critical applications. For tools that need to parse, compare, or manipulate file sizes from user input, sizefilter offers the most ergonomic and complete solution.

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
sizefilter = "0.1.2"
```

### Quick start

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
```
