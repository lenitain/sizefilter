//! Size filter tests.

use std::collections::HashSet;

use sizefilter::prelude::*;

#[test]
fn from_str_and_display() {
    let f: SizeFilter = ">=1GiB".parse().unwrap();
    assert_eq!(f.op(), SizeOp::Ge);
    assert_eq!(f.bytes(), GIB);
    assert_eq!(f.to_string(), ">=1GiB");

    assert_eq!(SizeFilter::new(SizeOp::Gt, KIB).to_string(), ">1KiB");
    assert_eq!(SizeFilter::new(SizeOp::Le, 0).to_string(), "<=0B");
    assert_eq!(SizeFilter::new(SizeOp::Lt, -1536).to_string(), "<-1.5KiB");
}

#[test]
fn display_round_trips_exactly() {
    let cases = [
        SizeFilter::ge(GIB),
        SizeFilter::gt(MIB),
        SizeFilter::lt(TIB),
        SizeFilter::le(KIB),
        SizeFilter::eq(0),
        SizeFilter::new(SizeOp::Gt, -500),
        // Values with no short exact form must fall back to bytes.
        SizeFilter::ge(1_048_575),
        SizeFilter::ge(1_610_612_737),
        SizeFilter::ge(i64::MAX),
        SizeFilter::le(i64::MIN),
    ];
    for filter in cases {
        let rendered = filter.to_string();
        assert_eq!(
            rendered.parse::<SizeFilter>().unwrap(),
            filter,
            "round trip of {rendered:?}"
        );
    }
}

#[test]
fn odd_thresholds_stay_exact() {
    let filter = SizeFilter::ge(1_048_575);
    assert_eq!(filter.to_string(), ">=1048575B");
    assert!(filter.matches(1_048_575));
    assert!(!filter.matches(1_048_574));
}

#[test]
fn matches() {
    let f = SizeFilter::ge(GIB);
    assert!(f.matches(GIB));
    assert!(f.matches(GIB + 1));
    assert!(!f.matches(GIB - 1));

    assert!(SizeFilter::lt(0).matches(-1));
    assert!(!SizeFilter::lt(0).matches(0));
}

#[test]
fn matches_u64() {
    let at_least_one_gib = SizeFilter::ge(GIB);
    assert!(!at_least_one_gib.matches_u64(0));
    assert!(at_least_one_gib.matches_u64(GIB as u64));
    assert!(!at_least_one_gib.matches_u64(GIB as u64 - 1));
    assert!(at_least_one_gib.matches_u64(u64::MAX));

    assert!(SizeFilter::gt(GIB).matches_u64(u64::MAX));
    assert!(!SizeFilter::lt(GIB).matches_u64(u64::MAX));
    assert!(!SizeFilter::le(GIB).matches_u64(u64::MAX));
    assert!(!SizeFilter::eq(GIB).matches_u64(u64::MAX));
}

#[test]
fn convenience_constructors() {
    assert_eq!(SizeFilter::gt(1), SizeFilter::new(SizeOp::Gt, 1));
    assert_eq!(SizeFilter::ge(2), SizeFilter::new(SizeOp::Ge, 2));
    assert_eq!(SizeFilter::lt(3), SizeFilter::new(SizeOp::Lt, 3));
    assert_eq!(SizeFilter::le(4), SizeFilter::new(SizeOp::Le, 4));
    assert_eq!(SizeFilter::eq(5), SizeFilter::new(SizeOp::Eq, 5));
}

#[test]
fn filter_is_hashable() {
    let filters = HashSet::from([
        SizeFilter::ge(KIB),
        SizeFilter::lt(MIB),
        SizeFilter::ge(KIB),
    ]);
    assert_eq!(filters.len(), 2);
    assert!(filters.contains(&SizeFilter::ge(KIB)));
}

#[test]
fn operator_parsing_and_display() {
    for (text, op) in [
        (">", SizeOp::Gt),
        (">=", SizeOp::Ge),
        ("<", SizeOp::Lt),
        ("<=", SizeOp::Le),
        ("=", SizeOp::Eq),
    ] {
        assert_eq!(text.parse::<SizeOp>().unwrap(), op);
        assert_eq!(op.to_string(), text);
    }
    assert_eq!("".parse::<SizeOp>(), Err(SizeError::EmptyInput));
    assert_eq!(" >= ".parse::<SizeOp>(), Ok(SizeOp::Ge));
    assert_eq!(">=1".parse::<SizeOp>(), Err(SizeError::InvalidOperator));
    assert_eq!("!=".parse::<SizeOp>(), Err(SizeError::InvalidOperator));
    assert_eq!("!".parse::<SizeOp>(), Err(SizeError::InvalidOperator));
    assert_eq!("==".parse::<SizeOp>(), Err(SizeError::InvalidOperator));
    assert_eq!(SizeOp::ALL.len(), 5);
}

#[test]
fn operator_applies() {
    assert!(SizeOp::Gt.applies(2, 1));
    assert!(SizeOp::Ge.applies(1, 1));
    assert!(!SizeOp::Ge.applies(0, 1));
    assert!(SizeOp::Lt.applies(0, 1));
    assert!(SizeOp::Le.applies(1, 1));
    assert!(SizeOp::Eq.applies(1, 1));
}

#[test]
fn canonical_strings_are_stable() {
    assert_eq!(SizeFilter::ge(1_048_575).to_string(), ">=1048575B");
    assert_eq!(SizeFilter::lt(500 * KIB).to_string(), "<500KiB");
    assert_eq!(SizeFilter::eq(0).to_string(), "=0B");
    assert_eq!(format_size(1_048_575), "1.0MiB");
}
