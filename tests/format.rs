use sizefilter::prelude::*;

#[test]
fn basic() {
    assert_eq!(format_size(0), "0B");
    assert_eq!(format_size(1), "1B");
    assert_eq!(format_size(100), "100B");
    assert_eq!(format_size(500), "500B");
    assert_eq!(format_size(KB), "1.0KB");
    assert_eq!(format_size(1536), "1.5KB");
    assert_eq!(format_size(2048), "2.0KB");
    assert_eq!(format_size(MB), "1.0MB");
    assert_eq!(format_size(GB), "1.0GB");
    assert_eq!(format_size(TB), "1.0TB");
}

#[test]
fn negative() {
    assert_eq!(format_size(-KB), "-1.0KB");
    assert_eq!(format_size(-MB), "-1.0MB");
    assert_eq!(format_size(-500), "-500B");
    assert_eq!(format_size(i64::MIN), "-8.0EB");
}

#[test]
fn edge() {
    assert_eq!(format_size(1023), "1023B");
    assert_eq!(format_size(-0), "0B");
}

#[test]
fn extreme_values() {
    // i64::MAX ≈ 8 EB
    let s = format_size(i64::MAX);
    assert_eq!(s, "8.0EB");

    // Zero
    assert_eq!(format_size(0), "0B");
    assert_eq!(format_size(-0), "0B");

    // PB boundary
    assert_eq!(format_size(PB), "1.0PB");
}

#[test]
fn roundtrip() {
    let cases = [0i64, 1, 500, KB, 2048, MB, GB];
    for &bytes in &cases {
        let formatted = format_size(bytes);
        let parsed = parse_size(&formatted).unwrap();
        assert_eq!(parsed, bytes, "roundtrip failed for {}", bytes);
    }
}
