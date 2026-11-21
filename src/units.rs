//! Binary byte units and unit lookup.
//!
//! The crate uses IEC binary units: 1 KiB = 1024 B, 1 MiB = 1024 KiB, and so
//! on. Parsing also accepts the common `KB`/`MB`/… spellings as aliases for the
//! binary units, because filesystem tools conventionally use them that way.

/// Bytes in one kibibyte (2¹⁰).
pub const KIB: i64 = 1 << 10;
/// Bytes in one mebibyte (2²⁰).
pub const MIB: i64 = 1 << 20;
/// Bytes in one gibibyte (2³⁰).
pub const GIB: i64 = 1 << 30;
/// Bytes in one tebibyte (2⁴⁰).
pub const TIB: i64 = 1 << 40;
/// Bytes in one pebibyte (2⁵⁰).
pub const PIB: i64 = 1 << 50;
/// Bytes in one exbibyte (2⁶⁰).
pub const EIB: i64 = 1 << 60;

/// Units from largest to smallest, used for formatting.
pub(crate) const UNITS: &[(i64, &str)] = &[
    (EIB, "EiB"),
    (PIB, "PiB"),
    (TIB, "TiB"),
    (GIB, "GiB"),
    (MIB, "MiB"),
    (KIB, "KiB"),
];

/// Accepted unit spellings, mapped to their byte multiplier.
///
/// Matching is ASCII case-insensitive. The empty suffix means bytes.
pub(crate) fn unit_multiplier(unit: &str) -> Option<i64> {
    const PARSE_UNITS: &[(&str, i64)] = &[
        ("b", 1),
        ("k", KIB),
        ("kb", KIB),
        ("kib", KIB),
        ("m", MIB),
        ("mb", MIB),
        ("mib", MIB),
        ("g", GIB),
        ("gb", GIB),
        ("gib", GIB),
        ("t", TIB),
        ("tb", TIB),
        ("tib", TIB),
        ("p", PIB),
        ("pb", PIB),
        ("pib", PIB),
        ("e", EIB),
        ("eb", EIB),
        ("eib", EIB),
    ];
    if unit.is_empty() {
        return Some(1);
    }
    PARSE_UNITS
        .iter()
        .find(|(name, _)| unit.eq_ignore_ascii_case(name))
        .map(|&(_, multiplier)| multiplier)
}
