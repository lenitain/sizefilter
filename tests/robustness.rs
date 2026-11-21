//! Regression tests for robustness: garbage input must not panic, and the
//! exact string forms must round trip.

use sizefilter::prelude::*;

fn next(state: &mut u64) -> u64 {
    *state = state
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407);
    *state
}

#[test]
fn parsing_garbage_never_panics() {
    let alphabet: Vec<char> =
        "0123456789.-+eEikmgtpKMGTPibkKBMGTPE/ +-:=<>,;[]{}()abcxyz\u{00b5}\u{041a}\u{ff11}"
            .chars()
            .collect();
    let mut state = 0x5eed_1234_9876_abcd;
    let mut buffer = String::new();
    for _ in 0..20_000 {
        buffer.clear();
        let len = (next(&mut state) % 48) as usize;
        for _ in 0..len {
            buffer.push(alphabet[(next(&mut state) as usize) % alphabet.len()]);
        }
        let _ = parse_size(&buffer);
        let _ = parse_size_filter(&buffer);
    }
}

#[test]
fn sizes_round_trip() {
    let mut state = 0xdead_beef_cafe_babe;
    for _ in 0..20_000 {
        let bytes = next(&mut state) as i64;
        let size = Size::from_bytes(bytes);
        assert_eq!(size.to_exact_string().parse::<Size>().unwrap(), size);
    }
}

#[test]
fn filters_round_trip() {
    let mut state = 0x0bad_f00d_1234_5678;
    for _ in 0..20_000 {
        let bytes = next(&mut state) as i64;
        let filter = SizeFilter::ge(bytes);
        assert_eq!(filter.to_string().parse::<SizeFilter>().unwrap(), filter);
    }
}
