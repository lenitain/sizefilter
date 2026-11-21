//! The [`Size`] type and its checked arithmetic.

use std::fmt;
use std::str::FromStr;

use crate::error::{SizeError, SizeResult};
use crate::units::{EIB, GIB, KIB, MIB, PIB, TIB};

/// A byte count that parses from and formats to a human readable string.
///
/// Sizes are signed: negative values represent deltas. Arithmetic is checked;
/// overflow panics deterministically through the operator traits and is
/// reported through [`Size::checked_add`] and friends.
///
/// `Display` is the human-oriented, rounded form. It is lossy; use
/// [`Size::to_exact_string`] when the output must parse back to the identical
/// byte count. Filters and serde always use the exact form.
///
/// ```
/// use sizefilter::{Size, KIB, MIB};
///
/// let size: Size = "1.5KiB".parse().unwrap();
/// assert_eq!(size.bytes(), 1536);
/// assert_eq!(size.to_string(), "1.5KiB");
/// assert_eq!(size.to_human(), "1.5KiB");
///
/// let odd = Size::from_bytes(1_610_612_737);
/// assert_eq!(odd.to_exact_string(), "1610612737B");
/// assert_eq!(odd.to_exact_string().parse::<Size>().unwrap(), odd);
///
/// assert_eq!(Size::from_bytes(MIB) + Size::from_bytes(KIB), Size::from_bytes(MIB + KIB));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Size(i64);

impl Size {
    /// Zero bytes.
    pub const ZERO: Size = Size(0);

    /// Create a `Size` from a raw byte count.
    #[inline]
    #[must_use]
    pub const fn from_bytes(bytes: i64) -> Self {
        Size(bytes)
    }

    /// Create a `Size` from a quantity of kibibytes (1024 bytes).
    ///
    /// # Panics
    ///
    /// Panics if the multiplication overflows.
    #[inline]
    #[must_use]
    pub const fn from_kib(kib: i64) -> Self {
        match kib.checked_mul(KIB) {
            Some(bytes) => Size(bytes),
            None => panic!("Size::from_kib overflow"),
        }
    }

    /// Create a `Size` from a quantity of mebibytes (1024² bytes).
    ///
    /// # Panics
    ///
    /// Panics if the multiplication overflows.
    #[inline]
    #[must_use]
    pub const fn from_mib(mib: i64) -> Self {
        match mib.checked_mul(MIB) {
            Some(bytes) => Size(bytes),
            None => panic!("Size::from_mib overflow"),
        }
    }

    /// Create a `Size` from a quantity of gibibytes (1024³ bytes).
    ///
    /// # Panics
    ///
    /// Panics if the multiplication overflows.
    #[inline]
    #[must_use]
    pub const fn from_gib(gib: i64) -> Self {
        match gib.checked_mul(GIB) {
            Some(bytes) => Size(bytes),
            None => panic!("Size::from_gib overflow"),
        }
    }

    /// Create a `Size` from a quantity of tebibytes (1024⁴ bytes).
    ///
    /// # Panics
    ///
    /// Panics if the multiplication overflows.
    #[inline]
    #[must_use]
    pub const fn from_tib(tib: i64) -> Self {
        match tib.checked_mul(TIB) {
            Some(bytes) => Size(bytes),
            None => panic!("Size::from_tib overflow"),
        }
    }

    /// Create a `Size` from a quantity of pebibytes (1024⁵ bytes).
    ///
    /// # Panics
    ///
    /// Panics if the multiplication overflows.
    #[inline]
    #[must_use]
    pub const fn from_pib(pib: i64) -> Self {
        match pib.checked_mul(PIB) {
            Some(bytes) => Size(bytes),
            None => panic!("Size::from_pib overflow"),
        }
    }

    /// Create a `Size` from a quantity of exbibytes (1024⁶ bytes).
    ///
    /// # Panics
    ///
    /// Panics if the multiplication overflows.
    #[inline]
    #[must_use]
    pub const fn from_eib(eib: i64) -> Self {
        match eib.checked_mul(EIB) {
            Some(bytes) => Size(bytes),
            None => panic!("Size::from_eib overflow"),
        }
    }

    /// Return the raw byte count.
    #[inline]
    #[must_use]
    pub const fn bytes(self) -> i64 {
        self.0
    }

    /// Format the size for humans, using the same rounded form as
    /// [`format_size`](crate::format_size).
    ///
    /// This is lossy; use [`Size::to_exact_string`] for a lossless form.
    #[inline]
    #[must_use]
    pub fn to_human(self) -> String {
        crate::format::format_size(self.0)
    }

    /// Format the size exactly, using the canonical representation that
    /// [`Size::from_str`] parses back to the identical byte count.
    ///
    /// When no binary unit represents the value with at most two decimal
    /// places, the result falls back to a byte count such as `"1048575B"`.
    #[must_use]
    pub fn to_exact_string(self) -> String {
        let mut out = String::new();
        crate::format::write_exact(&mut out, self.0).expect("writing to a String is infallible");
        out
    }

    /// Checked addition.
    #[inline]
    #[must_use]
    pub const fn checked_add(self, rhs: Size) -> Option<Size> {
        match self.0.checked_add(rhs.0) {
            Some(bytes) => Some(Size(bytes)),
            None => None,
        }
    }

    /// Checked subtraction.
    #[inline]
    #[must_use]
    pub const fn checked_sub(self, rhs: Size) -> Option<Size> {
        match self.0.checked_sub(rhs.0) {
            Some(bytes) => Some(Size(bytes)),
            None => None,
        }
    }

    /// Checked multiplication by a scalar.
    #[inline]
    #[must_use]
    pub const fn checked_mul(self, rhs: i64) -> Option<Size> {
        match self.0.checked_mul(rhs) {
            Some(bytes) => Some(Size(bytes)),
            None => None,
        }
    }

    /// Checked division by a scalar. Returns `None` for division by zero.
    #[inline]
    #[must_use]
    pub const fn checked_div(self, rhs: i64) -> Option<Size> {
        match self.0.checked_div(rhs) {
            Some(bytes) => Some(Size(bytes)),
            None => None,
        }
    }

    /// Checked remainder by a scalar. Returns `None` for a zero divisor.
    #[inline]
    #[must_use]
    pub const fn checked_rem(self, rhs: i64) -> Option<Size> {
        match self.0.checked_rem(rhs) {
            Some(bytes) => Some(Size(bytes)),
            None => None,
        }
    }

    /// Checked negation.
    #[inline]
    #[must_use]
    pub const fn checked_neg(self) -> Option<Size> {
        match self.0.checked_neg() {
            Some(bytes) => Some(Size(bytes)),
            None => None,
        }
    }
}

impl From<i64> for Size {
    #[inline]
    fn from(bytes: i64) -> Self {
        Size(bytes)
    }
}

impl From<Size> for i64 {
    #[inline]
    fn from(size: Size) -> Self {
        size.0
    }
}

impl TryFrom<u64> for Size {
    type Error = SizeError;

    /// Convert an unsigned byte count, rejecting values above [`i64::MAX`].
    fn try_from(bytes: u64) -> Result<Self, Self::Error> {
        i64::try_from(bytes)
            .map(Size)
            .map_err(|_| SizeError::Overflow)
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_human())
    }
}

impl FromStr for Size {
    type Err = SizeError;

    fn from_str(s: &str) -> SizeResult<Self> {
        crate::parse::parse_size(s).map(Size)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Size {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(&self.to_exact_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Size {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

macro_rules! checked_op {
    ($trait:ident, $method:ident, $checked:ident, $message:literal) => {
        impl std::ops::$trait for Size {
            type Output = Size;

            #[inline]
            fn $method(self, rhs: Size) -> Size {
                self.$checked(rhs)
                    .expect(concat!("Size ", $message, " overflow"))
            }
        }

        impl std::ops::$trait<i64> for Size {
            type Output = Size;

            #[inline]
            fn $method(self, rhs: i64) -> Size {
                self.$checked(Size(rhs))
                    .expect(concat!("Size ", $message, " overflow"))
            }
        }

        impl std::ops::$trait<Size> for i64 {
            type Output = Size;

            #[inline]
            fn $method(self, rhs: Size) -> Size {
                Size(self)
                    .$checked(rhs)
                    .expect(concat!("Size ", $message, " overflow"))
            }
        }

        impl std::ops::$trait<Size> for &Size {
            type Output = Size;

            #[inline]
            fn $method(self, rhs: Size) -> Size {
                (*self)
                    .$checked(rhs)
                    .expect(concat!("Size ", $message, " overflow"))
            }
        }
    };
}

checked_op!(Add, add, checked_add, "addition");
checked_op!(Sub, sub, checked_sub, "subtraction");

macro_rules! checked_assign {
    ($trait:ident, $method:ident, $op:tt) => {
        impl std::ops::$trait for Size {
            #[inline]
            fn $method(&mut self, rhs: Size) {
                *self = *self $op rhs;
            }
        }

        impl std::ops::$trait<i64> for Size {
            #[inline]
            fn $method(&mut self, rhs: i64) {
                *self = *self $op Size(rhs);
            }
        }
    };
}

impl std::ops::Mul<i64> for Size {
    type Output = Size;

    #[inline]
    fn mul(self, rhs: i64) -> Size {
        self.checked_mul(rhs).expect("Size multiplication overflow")
    }
}

impl std::ops::Mul<Size> for i64 {
    type Output = Size;

    #[inline]
    fn mul(self, rhs: Size) -> Size {
        rhs.checked_mul(self).expect("Size multiplication overflow")
    }
}

impl std::ops::Div<i64> for Size {
    type Output = Size;

    #[inline]
    fn div(self, rhs: i64) -> Size {
        self.checked_div(rhs)
            .expect("Size division by zero or overflow")
    }
}

impl std::ops::Rem<i64> for Size {
    type Output = Size;

    #[inline]
    fn rem(self, rhs: i64) -> Size {
        self.checked_rem(rhs)
            .expect("Size remainder by zero or overflow")
    }
}

impl std::ops::Neg for Size {
    type Output = Size;

    #[inline]
    fn neg(self) -> Size {
        self.checked_neg().expect("Size negation overflow")
    }
}

impl std::ops::Neg for &Size {
    type Output = Size;

    #[inline]
    fn neg(self) -> Size {
        (*self).checked_neg().expect("Size negation overflow")
    }
}

checked_assign!(AddAssign, add_assign, +);
checked_assign!(SubAssign, sub_assign, -);

impl std::ops::MulAssign<i64> for Size {
    #[inline]
    fn mul_assign(&mut self, rhs: i64) {
        *self = *self * rhs;
    }
}

impl std::ops::DivAssign<i64> for Size {
    #[inline]
    fn div_assign(&mut self, rhs: i64) {
        *self = *self / rhs;
    }
}

impl std::ops::RemAssign<i64> for Size {
    #[inline]
    fn rem_assign(&mut self, rhs: i64) {
        *self = *self % rhs;
    }
}
