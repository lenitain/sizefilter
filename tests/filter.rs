use sizefilter::prelude::*;

// -- SizeFilter type --

#[test]
fn from_str() {
    let f: SizeFilter = ">=1GB".parse().unwrap();
    assert_eq!(f.op, SizeOp::Ge);
    assert_eq!(f.bytes, GB);
}

#[test]
fn display() {
    let f = SizeFilter::new(SizeOp::Gt, 1024);
    assert_eq!(f.to_string(), ">1.0KB");
}

#[test]
fn matches() {
    let f = SizeFilter::new(SizeOp::Ge, GB);
    assert!(f.matches(GB));
    assert!(f.matches(GB + 1));
    assert!(!f.matches(GB - 1));
}

#[test]
fn convenience_ctors() {
    assert_eq!(SizeFilter::gt(1), SizeFilter::new(SizeOp::Gt, 1));
    assert_eq!(SizeFilter::ge(2), SizeFilter::new(SizeOp::Ge, 2));
    assert_eq!(SizeFilter::lt(3), SizeFilter::new(SizeOp::Lt, 3));
    assert_eq!(SizeFilter::le(4), SizeFilter::new(SizeOp::Le, 4));
    assert_eq!(SizeFilter::eq(5), SizeFilter::new(SizeOp::Eq, 5));
}

#[test]
fn matches_with_negative_threshold() {
    let f = SizeFilter::gt(-1024);
    assert!(f.matches(0));
    assert!(!f.matches(-2048));
    assert!(f.matches(-1023));

    let f = SizeFilter::lt(0);
    assert!(f.matches(-1));
    assert!(!f.matches(0));
    assert!(!f.matches(1));
}

#[test]
fn roundtrip_tostring_parse() {
    let cases = [
        SizeFilter::ge(GB),
        SizeFilter::gt(MB),
        SizeFilter::lt(TB),
        SizeFilter::le(KB),
        SizeFilter::eq(0),
        SizeFilter::new(SizeOp::Gt, -500),
    ];
    for f in cases {
        let s = f.to_string();
        let parsed: SizeFilter = s.parse().unwrap();
        assert_eq!(parsed.op, f.op, "op mismatch for {}", s);
        assert_eq!(parsed.bytes, f.bytes, "bytes mismatch for {}", s);
    }
}

// -- parse_size_filter --

#[test]
fn parse_ge() {
    let f = parse_size_filter(">=1GB").unwrap();
    assert_eq!(f.op, SizeOp::Ge);
    assert_eq!(f.bytes, GB);
}

#[test]
fn parse_gt() {
    let f = parse_size_filter(">500MB").unwrap();
    assert_eq!(f.op, SizeOp::Gt);
    assert_eq!(f.bytes, 500 * MB);
}

#[test]
fn parse_le() {
    let f = parse_size_filter("<=100KB").unwrap();
    assert_eq!(f.op, SizeOp::Le);
    assert_eq!(f.bytes, 100 * KB);
}

#[test]
fn parse_lt() {
    let f = parse_size_filter("<10MB").unwrap();
    assert_eq!(f.op, SizeOp::Lt);
}

#[test]
fn parse_eq() {
    let f = parse_size_filter("=0").unwrap();
    assert_eq!(f.op, SizeOp::Eq);
    assert_eq!(f.bytes, 0);
    let f = parse_size_filter("=1KB").unwrap();
    assert_eq!(f.op, SizeOp::Eq);
    assert_eq!(f.bytes, KB);
}

#[test]
fn no_operator_errors() {
    assert_eq!(parse_size_filter("1GB"), Err(SizeError::MissingOperator));
    assert_eq!(parse_size_filter("500MB"), Err(SizeError::MissingOperator));
    assert_eq!(parse_size_filter("0"), Err(SizeError::MissingOperator));
    assert_eq!(parse_size_filter("hello"), Err(SizeError::MissingOperator));
    assert_eq!(parse_size_filter(""), Err(SizeError::MissingOperator));
}

#[test]
fn whitespace() {
    let f = parse_size_filter("  >=  1MB  ").unwrap();
    assert_eq!(f.op, SizeOp::Ge);
    assert_eq!(f.bytes, MB);
    let f = parse_size_filter("  < 500KB  ").unwrap();
    assert_eq!(f.op, SizeOp::Lt);
    assert_eq!(f.bytes, 500 * KB);
}

#[test]
fn negative() {
    let f = parse_size_filter(">-1KB").unwrap();
    assert_eq!(f.op, SizeOp::Gt);
    assert_eq!(f.bytes, -KB);
    let f = parse_size_filter("<=-1024").unwrap();
    assert_eq!(f.op, SizeOp::Le);
    assert_eq!(f.bytes, -1024);
    let f = parse_size_filter("<=0").unwrap();
    assert_eq!(f.op, SizeOp::Le);
    assert_eq!(f.bytes, 0);
}

#[test]
fn decimal() {
    let f = parse_size_filter(">=1.5KB").unwrap();
    assert_eq!(f.op, SizeOp::Ge);
    assert_eq!(f.bytes, 1536);
    let f = parse_size_filter("<0.5MB").unwrap();
    assert_eq!(f.op, SizeOp::Lt);
    assert_eq!(f.bytes, 524288);
}

#[test]
fn invalid() {
    assert_eq!(parse_size_filter(">=abc"), Err(SizeError::InvalidNumber));
    assert_eq!(parse_size_filter("<= "), Err(SizeError::EmptyInput));
    assert_eq!(parse_size_filter("><1GB"), Err(SizeError::InvalidNumber));
}

#[test]
fn empty_after_operator() {
    assert_eq!(parse_size_filter(">="), Err(SizeError::EmptyInput));
    assert_eq!(parse_size_filter("<"), Err(SizeError::EmptyInput));
    assert_eq!(parse_size_filter(">"), Err(SizeError::EmptyInput));
    assert_eq!(parse_size_filter("<="), Err(SizeError::EmptyInput));
}
