use sizefilter::prelude::*;

// -- consts --

#[test]
fn values() {
    assert_eq!(KB, 1024);
    assert_eq!(MB, 1_048_576);
    assert_eq!(GB, 1_073_741_824);
    assert_eq!(TB, 1_099_511_627_776);
    assert_eq!(PB, 1_125_899_906_842_624);
    assert_eq!(EB, 1_152_921_504_606_846_976);
}

// -- SizeOp --

#[test]
fn all_variants() {
    assert_eq!(SizeOp::ALL.len(), 5);
}

#[test]
fn applies() {
    assert!(SizeOp::Gt.applies(10, 5));
    assert!(!SizeOp::Gt.applies(5, 5));
    assert!(SizeOp::Ge.applies(5, 5));
    assert!(SizeOp::Lt.applies(3, 5));
    assert!(SizeOp::Le.applies(5, 5));
    assert!(SizeOp::Eq.applies(5, 5));
    assert!(!SizeOp::Eq.applies(4, 5));
}

#[test]
fn op_display() {
    assert_eq!(SizeOp::Gt.to_string(), ">");
    assert_eq!(SizeOp::Ge.to_string(), ">=");
    assert_eq!(SizeOp::Lt.to_string(), "<");
    assert_eq!(SizeOp::Le.to_string(), "<=");
    assert_eq!(SizeOp::Eq.to_string(), "=");
}

// -- Size newtype --

#[test]
fn from_str() {
    let s: Size = "1GB".parse().unwrap();
    assert_eq!(s.bytes(), GB);
}

#[test]
fn size_display() {
    assert_eq!(Size::from_bytes(MB).to_string(), "1.0MB");
}

#[test]
fn const_ctors() {
    assert_eq!(Size::from_kb(1), Size::from_bytes(KB));
    assert_eq!(Size::from_mb(1), Size::from_bytes(MB));
    assert_eq!(Size::from_gb(1), Size::from_bytes(GB));
    assert_eq!(Size::from_tb(1), Size::from_bytes(TB));
}

#[test]
fn from_i64() {
    let s: Size = 2048.into();
    assert_eq!(s.bytes(), 2048);
}

#[test]
fn into_i64() {
    let s = Size::from_kb(2);
    let v: i64 = s.into();
    assert_eq!(v, 2048);
}

#[test]
fn default() {
    assert_eq!(Size::default(), Size::ZERO);
}

#[test]
fn ord() {
    assert!(Size::from_kb(1) < Size::from_mb(1));
}

#[test]
fn parse_error() {
    assert_eq!("".parse::<Size>(), Err(SizeError::EmptyInput));
    assert_eq!("abc".parse::<Size>(), Err(SizeError::InvalidNumber));
}

// -- arithmetic --

#[test]
fn add() {
    assert_eq!(
        Size::from_mb(1) + Size::from_kb(512),
        Size::from_bytes(MB + 512 * KB)
    );
    assert_eq!(Size::from_mb(1) + 512, Size::from_bytes(MB + 512));
    assert_eq!(1024i64 + Size::from_kb(1), Size::from_bytes(2048));
}

#[test]
fn sub() {
    assert_eq!(
        Size::from_mb(2) - Size::from_kb(512),
        Size::from_bytes(2 * MB - 512 * KB)
    );
    assert_eq!(Size::from_mb(1) - 1024, Size::from_bytes(MB - 1024));
}

#[test]
fn mul() {
    assert_eq!(Size::from_mb(2) * 3, Size::from_bytes(6 * MB));
    assert_eq!(3 * Size::from_mb(2), Size::from_bytes(6 * MB));
}

#[test]
fn div() {
    assert_eq!(Size::from_mb(4) / 2, Size::from_bytes(2 * MB));
}

#[test]
fn neg() {
    assert_eq!(-Size::from_kb(1), Size::from_bytes(-KB));
}

#[test]
fn add_assign() {
    let mut s = Size::from_mb(1);
    s += Size::from_kb(512);
    assert_eq!(s, Size::from_bytes(MB + 512 * KB));

    let mut s = Size::from_mb(1);
    s += 1024i64;
    assert_eq!(s, Size::from_bytes(MB + 1024));
}

#[test]
fn sub_assign() {
    let mut s = Size::from_mb(2);
    s -= Size::from_kb(512);
    assert_eq!(s, Size::from_bytes(2 * MB - 512 * KB));
}

#[test]
fn mul_assign() {
    let mut s = Size::from_mb(2);
    s *= 3;
    assert_eq!(s, Size::from_bytes(6 * MB));
}

#[test]
fn div_assign() {
    let mut s = Size::from_mb(4);
    s /= 2;
    assert_eq!(s, Size::from_bytes(2 * MB));
}

#[test]
fn rem() {
    assert_eq!(Size::from_bytes(7) % 4, Size::from_bytes(3));
    assert_eq!(Size::from_kb(1) % 1000, Size::from_bytes(24));
}

#[test]
fn rem_assign() {
    let mut s = Size::from_bytes(7);
    s %= 4;
    assert_eq!(s, Size::from_bytes(3));
}

// -- panics --

#[test]
#[should_panic(expected = "attempt to divide by zero")]
fn div_by_zero() {
    let _ = Size::from_kb(1) / 0;
}

#[test]
#[should_panic(expected = "attempt to calculate the remainder")]
fn rem_by_zero() {
    let _ = Size::from_kb(1) % 0;
}

// -- error types --

#[test]
fn error_display() {
    assert_eq!(SizeError::EmptyInput.to_string(), "empty input");
    assert_eq!(
        SizeError::InvalidNumber.to_string(),
        "failed to parse number"
    );
    assert_eq!(SizeError::UnknownUnit.to_string(), "unknown size unit");
    assert!(
        SizeError::MissingOperator
            .to_string()
            .contains("size filter must start")
    );
}

#[test]
fn error_is_proper_error() {
    use std::error::Error;
    assert!(SizeError::EmptyInput.source().is_none());
}

// -- send/sync --

#[test]
fn traits() {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}
    assert_send::<Size>();
    assert_sync::<Size>();
    assert_send::<SizeFilter>();
    assert_sync::<SizeFilter>();
}
