// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.
use std::cmp::PartialEq;
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub struct SaturatingU16 {
    value: u16,
}

impl PartialEq for SaturatingU16 {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}

impl Add for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value.saturating_add(rhs.value.clone()),
        }
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        Self {
            value: self.value.saturating_add(rhs.value.clone()),
        }
    }
}

macro_rules! impl_trait {
    (From=>$($from:ty),+) => {
        $(impl From<$from> for SaturatingU16 {
            fn from(value: $from) -> Self {
                Self {
                    value: value.clone() as u16,
                }
            }
        }
    )+
    };
    (Add=>$($from:ty),+) => {
        $(impl Add<$from> for SaturatingU16 {
            type Output = Self;

            fn add(self, rhs: $from) -> Self::Output {
                Self {
                value: self.value + rhs.clone() as u16,
                }
            }
        })+
    }
    }

impl_trait!(From=>u8, &u8, u16, &u16);
impl_trait!(Add=>u8, &u8, u16, &u16);
