use sizefilter::prelude::*;

#[test]
fn basic() {
    assert_eq!(parse_size("1024").unwrap(), 1024);
    assert_eq!(parse_size("1KB").unwrap(), KB);
    assert_eq!(parse_size("1MB").unwrap(), MB);
    assert_eq!(parse_size("1GB").unwrap(), GB);
    assert_eq!(parse_size("1TB").unwrap(), TB);
    assert_eq!(parse_size("1PB").unwrap(), PB);
    assert_eq!(parse_size("1EB").unwrap(), EB);
}

#[test]
fn zero_and_small() {
    assert_eq!(parse_size("0").unwrap(), 0);
    assert_eq!(parse_size("0B").unwrap(), 0);
    assert_eq!(parse_size("-0").unwrap(), 0);
    assert_eq!(parse_size("1B").unwrap(), 1);
    assert_eq!(parse_size("512").unwrap(), 512);
}

#[test]
fn short_unit() {
    assert_eq!(parse_size("1K").unwrap(), KB);
    assert_eq!(parse_size("1M").unwrap(), MB);
    assert_eq!(parse_size("1G").unwrap(), GB);
    assert_eq!(parse_size("1T").unwrap(), TB);
    assert_eq!(parse_size("1P").unwrap(), PB);
    assert_eq!(parse_size("1E").unwrap(), EB);
}

#[test]
fn case_insensitive() {
    assert_eq!(parse_size("1KB").unwrap(), KB);
    assert_eq!(parse_size("1Kb").unwrap(), KB);
    assert_eq!(parse_size("1kb").unwrap(), KB);
    assert_eq!(parse_size("1mb").unwrap(), MB);
    assert_eq!(parse_size("1gb").unwrap(), GB);
    assert_eq!(parse_size("1tb").unwrap(), TB);
    assert_eq!(parse_size("1pb").unwrap(), PB);
    assert_eq!(parse_size("1eb").unwrap(), EB);
}

#[test]
fn whitespace_padding() {
    assert_eq!(parse_size("  1KB  ").unwrap(), KB);
    assert_eq!(parse_size("\t1024\n").unwrap(), 1024);
}

#[test]
fn whitespace_between() {
    assert_eq!(parse_size("1 KB").unwrap(), KB);
    assert_eq!(parse_size("1  MB").unwrap(), MB);
    assert_eq!(parse_size("1 kb").unwrap(), KB);
}

#[test]
fn decimal() {
    assert_eq!(parse_size("1.5KB").unwrap(), 1536);
    assert_eq!(parse_size("0.5KB").unwrap(), 512);
    assert_eq!(parse_size("0.001MB").unwrap(), 1048);
    assert_eq!(parse_size("0.1GB").unwrap(), 107374182);
    // 精确小数测试：整数运算避免浮点误差
    assert_eq!(parse_size("1.5GB").unwrap(), 1_610_612_736);
    assert_eq!(parse_size("2.25MB").unwrap(), 2_359_296);
    assert_eq!(parse_size("0.125GB").unwrap(), 134_217_728);
}

#[test]
fn negative() {
    assert_eq!(parse_size("-1024").unwrap(), -1024);
    assert_eq!(parse_size("-1KB").unwrap(), -KB);
    assert_eq!(parse_size("-2GB").unwrap(), -2 * GB);
    assert_eq!(parse_size("-2.5MB").unwrap(), -2_621_440);
    assert_eq!(parse_size("-0.5KB").unwrap(), -512);
}

#[test]
fn extreme() {
    assert!(parse_size("9999GB").is_ok());
    assert_eq!(parse_size("1024TB").unwrap(), 1024 * TB);
    assert_eq!(parse_size("0.000000001KB").unwrap(), 0);
    assert_eq!(parse_size("1PB").unwrap(), 1_125_899_906_842_624);
    assert_eq!(parse_size("1EB").unwrap(), 1_152_921_504_606_846_976);
}

#[test]
fn invalid() {
    assert_eq!(parse_size(""), Err(SizeError::EmptyInput));
    assert_eq!(parse_size("  "), Err(SizeError::EmptyInput));
    assert_eq!(parse_size("abc"), Err(SizeError::InvalidNumber));
    assert_eq!(parse_size("1.5.3KB"), Err(SizeError::InvalidNumber));
    assert_eq!(parse_size("1XB"), Err(SizeError::UnknownUnit));
    assert!(parse_size("KB").is_err());
    assert!(parse_size("MB").is_err());
}
