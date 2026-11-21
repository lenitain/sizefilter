//! Size parser tests.

use sizefilter::prelude::*;

#[test]
fn basic_units() {
    assert_eq!(parse_size("1024").unwrap(), 1024);
    assert_eq!(parse_size("1B").unwrap(), 1);
    assert_eq!(parse_size("1KiB").unwrap(), KIB);
    assert_eq!(parse_size("1MiB").unwrap(), MIB);
    assert_eq!(parse_size("1GiB").unwrap(), GIB);
    assert_eq!(parse_size("1TiB").unwrap(), TIB);
    assert_eq!(parse_size("1PiB").unwrap(), PIB);
    assert_eq!(parse_size("1EiB").unwrap(), EIB);
}

#[test]
fn legacy_unit_spellings() {
    assert_eq!(parse_size("1K").unwrap(), KIB);
    assert_eq!(parse_size("1KB").unwrap(), KIB);
    assert_eq!(parse_size("1M").unwrap(), MIB);
    assert_eq!(parse_size("1MB").unwrap(), MIB);
    assert_eq!(parse_size("1G").unwrap(), GIB);
    assert_eq!(parse_size("1T").unwrap(), TIB);
    assert_eq!(parse_size("1P").unwrap(), PIB);
    assert_eq!(parse_size("1E").unwrap(), EIB);
}

#[test]
fn case_insensitive() {
    for unit in ["kib", "KiB", "KIB", "kIb"] {
        assert_eq!(parse_size(&format!("1{unit}")).unwrap(), KIB);
    }
    for unit in ["mib", "MiB", "MIB", "mIb"] {
        assert_eq!(parse_size(&format!("1{unit}")).unwrap(), MIB);
    }
}

#[test]
fn whitespace() {
    assert_eq!(parse_size("  1KiB  ").unwrap(), KIB);
    assert_eq!(parse_size("\t1024\n").unwrap(), 1024);
    assert_eq!(parse_size("1 KiB").unwrap(), KIB);
    assert_eq!(parse_size("1  MiB").unwrap(), MIB);
}

#[test]
fn decimals() {
    assert_eq!(parse_size("1.5KiB").unwrap(), 1536);
    assert_eq!(parse_size("0.5KiB").unwrap(), 512);
    assert_eq!(parse_size("0.05KiB").unwrap(), 51);
    assert_eq!(parse_size("0.001MiB").unwrap(), 1048);
    assert_eq!(parse_size("0.1KiB").unwrap(), 102);
    assert_eq!(parse_size("1.5GiB").unwrap(), 1_610_612_736);
    assert_eq!(parse_size("2.25MiB").unwrap(), 2_359_296);
    assert_eq!(parse_size("0.125GiB").unwrap(), 134_217_728);
    assert_eq!(parse_size(".5KiB").unwrap(), 512);
}

#[test]
fn decimals_truncate_toward_zero() {
    // 0.1 KiB = 102.4 bytes -> 102
    assert_eq!(parse_size("0.1KiB").unwrap(), 102);
    assert_eq!(parse_size("-0.1KiB").unwrap(), -102);
    // 0.001 KiB = 1.024 bytes -> 1
    assert_eq!(parse_size("0.001KiB").unwrap(), 1);
}

#[test]
fn signed() {
    assert_eq!(parse_size("-1024").unwrap(), -1024);
    assert_eq!(parse_size("-1KiB").unwrap(), -KIB);
    assert_eq!(parse_size("-2.5MiB").unwrap(), -2_621_440);
    assert_eq!(parse_size("-0").unwrap(), 0);
    assert_eq!(parse_size("+1KiB").unwrap(), KIB);
}

#[test]
fn extremes() {
    assert_eq!(parse_size("7EiB").unwrap(), 7 * EIB);
    assert_eq!(parse_size("9223372036854775807").unwrap(), i64::MAX);
    assert_eq!(parse_size("9223372036854775808"), Err(SizeError::Overflow));
    assert_eq!(parse_size("8EiB"), Err(SizeError::Overflow));
    assert_eq!(
        parse_size("99999999999999999999EiB"),
        Err(SizeError::Overflow)
    );
}

#[test]
fn malformed_numbers() {
    assert_eq!(parse_size(""), Err(SizeError::EmptyInput));
    assert_eq!(parse_size("   "), Err(SizeError::EmptyInput));
    assert_eq!(parse_size("abc"), Err(SizeError::InvalidNumber));
    assert_eq!(parse_size("KiB"), Err(SizeError::InvalidNumber));
    assert_eq!(parse_size("1."), Err(SizeError::InvalidNumber));
    assert_eq!(parse_size("1..5KiB"), Err(SizeError::InvalidNumber));
    assert_eq!(parse_size("1.5.5KiB"), Err(SizeError::InvalidNumber));
    assert_eq!(parse_size("1_000"), Err(SizeError::InvalidNumber));
    assert_eq!(parse_size("- 1KiB"), Err(SizeError::InvalidNumber));
    assert_eq!(parse_size("1e6"), Err(SizeError::UnknownUnit));
    assert_eq!(parse_size("1XB"), Err(SizeError::UnknownUnit));
}

#[test]
fn fraction_digit_limit() {
    let eighteen = format!("0.{}KiB", "0".repeat(17) + "1");
    assert!(parse_size(&eighteen).is_ok());

    let nineteen = format!("0.{}KiB", "0".repeat(18) + "1");
    assert_eq!(parse_size(&nineteen), Err(SizeError::InvalidNumber));

    // The input that used to overflow a power of ten.
    assert_eq!(
        parse_size("0.1000000000000000000GiB"),
        Err(SizeError::InvalidNumber)
    );
}

#[test]
fn adversarial_inputs_do_not_panic() {
    let long = "9".repeat(200);
    let inputs = [
        "",
        " ",
        "-",
        "+",
        ".",
        "-.",
        "..",
        "1..",
        "1.2.3.4",
        "e",
        "1e",
        "1e6",
        "1E999999999999999999999",
        "99999999999999999999999999999999999999999999",
        "0.999999999999999999999999999999999999999999KiB",
        "1 КБ",
        "1µB",
        "１KiB",
        long.as_str(),
    ];
    for input in inputs {
        let _ = parse_size(input);
    }
}

#[test]
fn filter_parsing() {
    let f: SizeFilter = ">=1GiB".parse().unwrap();
    assert_eq!(f.op(), SizeOp::Ge);
    assert_eq!(f.bytes(), GIB);

    assert_eq!(parse_size_filter(">= 1GiB").unwrap().bytes(), GIB);
    assert_eq!(parse_size_filter(" <= 500KiB ").unwrap().bytes(), 500 * KIB);
    assert_eq!(parse_size_filter("=0").unwrap().bytes(), 0);
    assert_eq!(parse_size_filter(">-1.5KiB").unwrap().bytes(), -1536);
}

#[test]
fn filter_errors() {
    assert_eq!(parse_size_filter(""), Err(SizeError::EmptyInput));
    assert_eq!(parse_size_filter("   "), Err(SizeError::EmptyInput));
    assert_eq!(parse_size_filter(">="), Err(SizeError::MissingValue));
    assert_eq!(parse_size_filter("1GiB"), Err(SizeError::MissingOperator));
    assert_eq!(parse_size_filter("!1GiB"), Err(SizeError::InvalidOperator));
    assert_eq!(parse_size_filter("==1GiB"), Err(SizeError::InvalidOperator));
    assert_eq!(
        parse_size_filter("> = 1GiB"),
        Err(SizeError::InvalidOperator)
    );
    assert_eq!(parse_size_filter(">=1XB"), Err(SizeError::UnknownUnit));
}
