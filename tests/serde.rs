//! Integration tests for serde feature.

#![cfg(feature = "serde")]

use serde::{Deserialize, Serialize};
use sizefilter::{GB, MB, Size, SizeFilter, SizeOp};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Config {
    max_log_size: SizeFilter,
    min_file_size: Size,
}

#[test]
fn serde_roundtrip_size_filter() {
    let f = SizeFilter::ge(GB);
    let json = serde_json::to_string(&f).unwrap();
    assert_eq!(json, "\">=1.0GB\"");

    let parsed: SizeFilter = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.op(), SizeOp::Ge);
    assert_eq!(parsed.bytes(), GB);
}

#[test]
fn serde_roundtrip_size() {
    let s = Size::from_mb(512);
    let json = serde_json::to_string(&s).unwrap();
    assert_eq!(json, "\"512.0MB\"");

    let parsed: Size = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.bytes(), 512 * MB);
}

#[test]
fn serde_struct_roundtrip() {
    let config = Config {
        max_log_size: SizeFilter::lt(10 * MB),
        min_file_size: Size::from_bytes(1024),
    };

    let json = serde_json::to_string_pretty(&config).unwrap();
    let parsed: Config = serde_json::from_str(&json).unwrap();

    assert_eq!(config, parsed);
}
