//! Formatting tests.

use sizefilter::prelude::*;

#[test]
fn human_basic() {
    assert_eq!(format_size(0), "0B");
    assert_eq!(format_size(1), "1B");
    assert_eq!(format_size(500), "500B");
    assert_eq!(format_size(1023), "1023B");
    assert_eq!(format_size(KIB), "1.0KiB");
    assert_eq!(format_size(1536), "1.5KiB");
    assert_eq!(format_size(2048), "2.0KiB");
    assert_eq!(format_size(MIB), "1.0MiB");
    assert_eq!(format_size(GIB), "1.0GiB");
    assert_eq!(format_size(TIB), "1.0TiB");
    assert_eq!(format_size(PIB), "1.0PiB");
    assert_eq!(format_size(EIB), "1.0EiB");
}

#[test]
fn human_rounds_and_promotes() {
    assert_eq!(format_size(MIB - 1), "1.0MiB");
    assert_eq!(format_size(GIB - 1), "1.0GiB");
    assert_eq!(format_size(1_048_575), "1.0MiB");
    assert_eq!(format_size(1_000_000), "976.6KiB");
    assert_eq!(format_size(1024 * 1023 + 512), "1023.5KiB");
    // Rounds up to exactly 1024.0KiB, so it must promote instead.
    assert_eq!(format_size(1024 * 1023 + 1023), "1.0MiB");
}

#[test]
fn human_negative() {
    assert_eq!(format_size(-KIB), "-1.0KiB");
    assert_eq!(format_size(-1536), "-1.5KiB");
    assert_eq!(format_size(-500), "-500B");
    assert_eq!(format_size(i64::MIN), "-8.0EiB");
    assert_eq!(format_size(i64::MAX), "7.9EiB");
}

#[test]
fn display_is_human_oriented() {
    let size = Size::from_bytes(1_000_000_000);
    assert_eq!(size.to_string(), "953.7MiB");
    assert_eq!(size.to_human(), "953.7MiB");
    assert_eq!(format_size(1_000_000_000), "953.7MiB");

    let odd = Size::from_bytes(1_048_575);
    assert_eq!(odd.to_string(), "1.0MiB");
    assert_eq!(odd.to_human(), "1.0MiB");
}

#[test]
fn exact_string_round_trips() {
    let cases = [
        0i64,
        1,
        1023,
        1024,
        1536,
        2048,
        1_048_064,
        1_048_575,
        1_048_576,
        1_610_612_736,
        1_610_612_737,
        1_000_000,
        -1,
        -1536,
        i64::MIN,
        i64::MAX,
    ];
    for bytes in cases {
        let size = Size::from_bytes(bytes);
        let rendered = size.to_exact_string();
        assert_eq!(
            rendered.parse::<Size>().unwrap(),
            size,
            "{bytes} rendered as {rendered:?} did not round trip"
        );
    }
}

#[test]
fn exact_string_prefers_short_units() {
    assert_eq!(Size::from_bytes(0).to_exact_string(), "0B");
    assert_eq!(Size::from_bytes(1024).to_exact_string(), "1KiB");
    assert_eq!(Size::from_bytes(1536).to_exact_string(), "1.5KiB");
    assert_eq!(Size::from_bytes(1_310_720).to_exact_string(), "1.25MiB");
    assert_eq!(Size::from_bytes(1_048_064).to_exact_string(), "1023.5KiB");
    // No unit represents this with at most two decimals: bytes are exact.
    assert_eq!(Size::from_bytes(1_048_575).to_exact_string(), "1048575B");
    assert_eq!(
        Size::from_bytes(1_610_612_737).to_exact_string(),
        "1610612737B"
    );
    assert_eq!(Size::from_bytes(-1536).to_exact_string(), "-1.5KiB");
}

#[test]
fn exact_string_is_parseable_for_every_unit() {
    for unit in [KIB, MIB, GIB, TIB, PIB, EIB] {
        for multiplier in [-7i64, -1, 0, 1, 2, 7] {
            let bytes = unit.checked_mul(multiplier).unwrap();
            let rendered = Size::from_bytes(bytes).to_exact_string();
            assert_eq!(rendered.parse::<Size>().unwrap().bytes(), bytes);
        }
    }
}

#[test]
fn human_output_is_always_parseable() {
    let cases = [
        0i64,
        1,
        1023,
        1024,
        1_048_575,
        1_048_576,
        i64::MIN,
        i64::MIN + 1,
        i64::MAX - 1,
        i64::MAX,
        EIB * 7,
        7 * EIB + 1,
    ];
    for bytes in cases {
        let rendered = format_size(bytes);
        assert!(
            parse_size(&rendered).is_ok(),
            "format_size({bytes}) = {rendered:?} does not parse"
        );
    }
}

#[test]
fn exact_string_round_trips_over_a_range() {
    for bytes in -5_000i64..=5_000 {
        let size = Size::from_bytes(bytes);
        assert_eq!(size.to_exact_string().parse::<Size>().unwrap(), size);
    }
    for unit in [KIB, MIB, GIB, TIB, PIB, EIB] {
        for factor in -3i64..=3 {
            let bytes = unit * factor;
            let size = Size::from_bytes(bytes);
            assert_eq!(size.to_exact_string().parse::<Size>().unwrap(), size);
        }
    }
}
