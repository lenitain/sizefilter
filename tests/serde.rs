//! Serde tests. Run with `--features serde`.

#![cfg(feature = "serde")]

use sizefilter::{KIB, MIB, Size, SizeFilter, SizeOp};

#[test]
fn size_round_trips_exactly() {
    for bytes in [0i64, 1024, 1536, 1_048_575, 1_610_612_737, -1536, i64::MAX] {
        let size = Size::from_bytes(bytes);
        let json = serde_json::to_string(&size).unwrap();
        let parsed: Size = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, size, "{json} did not round trip");
    }
    assert_eq!(
        serde_json::to_string(&Size::from_kib(1)).unwrap(),
        "\"1KiB\""
    );

    // Serde uses the exact form even when `Display` is human-oriented.
    let odd = Size::from_bytes(1_048_575);
    assert_eq!(odd.to_string(), "1.0MiB");
    assert_eq!(serde_json::to_string(&odd).unwrap(), "\"1048575B\"");
}

#[test]
fn filter_round_trips_exactly() {
    let filter = SizeFilter::ge(1_048_575);
    let json = serde_json::to_string(&filter).unwrap();
    assert_eq!(json, "\">=1048575B\"");

    let parsed: SizeFilter = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, filter);
    assert_eq!(parsed.op(), SizeOp::Ge);
    assert_eq!(parsed.bytes(), 1_048_575);
}

#[test]
fn filter_with_round_units_uses_short_form() {
    let json = serde_json::to_string(&SizeFilter::lt(500 * KIB)).unwrap();
    assert_eq!(json, "\"<500KiB\"");
    let parsed: SizeFilter = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.bytes(), 500 * KIB);
}

#[test]
fn invalid_json_is_rejected() {
    assert!(serde_json::from_str::<Size>("\"1XB\"").is_err());
    assert!(serde_json::from_str::<SizeFilter>("\"1MiB\"").is_err());
    assert!(serde_json::from_str::<SizeFilter>("\">=99999999999999999999EiB\"").is_err());
}

#[test]
fn sizes_inside_structures() {
    #[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug)]
    struct Config {
        max_log_size: SizeFilter,
        min_file_size: Size,
    }

    let config = Config {
        max_log_size: SizeFilter::le(MIB),
        min_file_size: Size::from_bytes(1_048_575),
    };
    let json = serde_json::to_string(&config).unwrap();
    assert_eq!(
        json,
        r#"{"max_log_size":"<=1MiB","min_file_size":"1048575B"}"#
    );
    assert_eq!(serde_json::from_str::<Config>(&json).unwrap(), config);
}
