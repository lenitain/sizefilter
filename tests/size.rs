//! `Size` arithmetic, conversions, and overflow behaviour.

use sizefilter::prelude::*;

#[test]
fn add_and_sub() {
    assert_eq!(
        Size::from_kib(2) + Size::from_kib(512),
        Size::from_bytes(2 * KIB + 512 * KIB)
    );
    assert_eq!(Size::from_bytes(10) + 5, Size::from_bytes(15));
    assert_eq!(5 + Size::from_bytes(10), Size::from_bytes(15));
    assert_eq!(Size::from_bytes(10) - 5, Size::from_bytes(5));
    assert_eq!(5 - Size::from_bytes(10), Size::from_bytes(-5));
}

#[test]
fn assign_operators() {
    let mut size = Size::from_bytes(10);
    size += Size::from_bytes(5);
    assert_eq!(size, Size::from_bytes(15));
    size -= 3;
    assert_eq!(size, Size::from_bytes(12));
    size *= 2;
    assert_eq!(size, Size::from_bytes(24));
    size /= 3;
    assert_eq!(size, Size::from_bytes(8));
    size %= 5;
    assert_eq!(size, Size::from_bytes(3));
}

#[test]
fn mul_div_rem_neg() {
    assert_eq!(Size::from_bytes(5) * 3, Size::from_bytes(15));
    assert_eq!(3 * Size::from_bytes(5), Size::from_bytes(15));
    assert_eq!(Size::from_bytes(15) / 3, Size::from_bytes(5));
    assert_eq!(Size::from_bytes(16) % 5, Size::from_bytes(1));
    assert_eq!(-Size::from_bytes(5), Size::from_bytes(-5));
    assert_eq!(-&Size::from_bytes(5), Size::from_bytes(-5));
}

#[test]
fn checked_operations() {
    assert_eq!(
        Size::from_bytes(2).checked_add(Size::from_bytes(3)),
        Some(Size::from_bytes(5))
    );
    assert_eq!(
        Size::from_bytes(i64::MAX).checked_add(Size::from_bytes(1)),
        None
    );
    assert_eq!(
        Size::from_bytes(i64::MIN).checked_sub(Size::from_bytes(1)),
        None
    );
    assert_eq!(
        Size::from_bytes(2).checked_mul(3),
        Some(Size::from_bytes(6))
    );
    assert_eq!(Size::from_bytes(i64::MAX).checked_mul(2), None);
    assert_eq!(
        Size::from_bytes(6).checked_div(3),
        Some(Size::from_bytes(2))
    );
    assert_eq!(Size::from_bytes(6).checked_div(0), None);
    assert_eq!(
        Size::from_bytes(6).checked_div(-1),
        Some(Size::from_bytes(-6))
    );
    assert_eq!(Size::from_bytes(7).checked_rem(0), None);
    assert_eq!(Size::from_bytes(i64::MIN).checked_neg(), None);
}

#[test]
#[should_panic(expected = "Size addition overflow")]
fn add_overflow_panics() {
    let _ = Size::from_bytes(i64::MAX) + Size::from_bytes(1);
}

#[test]
#[should_panic(expected = "Size subtraction overflow")]
fn sub_overflow_panics() {
    let _ = Size::from_bytes(i64::MIN) - Size::from_bytes(1);
}

#[test]
#[should_panic(expected = "Size multiplication overflow")]
fn mul_overflow_panics() {
    let _ = Size::from_bytes(i64::MAX) * 2;
}

#[test]
#[should_panic(expected = "Size division by zero")]
fn div_by_zero_panics() {
    let _ = Size::from_bytes(1) / 0;
}

#[test]
#[should_panic(expected = "Size negation overflow")]
fn neg_overflow_panics() {
    let _ = -Size::from_bytes(i64::MIN);
}

#[test]
fn conversions_and_traits() {
    assert_eq!(i64::from(Size::from_bytes(42)), 42);
    assert_eq!(Size::from(42i64), Size::from_bytes(42));
    assert_eq!(Size::try_from(42u64).unwrap(), Size::from_bytes(42));
    assert_eq!(Size::try_from(u64::MAX), Err(SizeError::Overflow));
    assert_eq!(Size::default(), Size::ZERO);
    assert_eq!(Size::from_kib(1).bytes(), KIB);
    assert_eq!(Size::from_mib(1).bytes(), MIB);
    assert_eq!(Size::from_gib(1).bytes(), GIB);
    assert_eq!(Size::from_tib(1).bytes(), TIB);
    assert_eq!(Size::from_pib(1).bytes(), PIB);
    assert_eq!(Size::from_eib(1).bytes(), EIB);
    assert!(Size::from_bytes(1) < Size::from_bytes(2));
}

#[test]
fn display_is_human() {
    assert_eq!(Size::from_bytes(1_000_000_000).to_string(), "953.7MiB");
    assert_eq!(Size::from_bytes(1_000_000_000).to_human(), "953.7MiB");
    assert_eq!(
        Size::from_bytes(1_000_000_000).to_exact_string(),
        "1000000000B"
    );
}

#[test]
fn zero_and_negative_zero() {
    assert_eq!(Size::ZERO.bytes(), 0);
    assert_eq!(Size::from_bytes(-0).bytes(), 0);
    assert_eq!(Size::from_bytes(0).to_string(), "0B");
}
