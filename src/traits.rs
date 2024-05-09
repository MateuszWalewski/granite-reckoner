use std::{
    default::Default,
    fmt::{Debug, Display},
    marker::{Send, Sync},
    ops::{Add, Div, Mul, Sub},
};
// Supported types:
// i8
// i16
// i32
// i64
// i128
// isize
// u8
// u16
// u32
// u64
// u128
// usize
// f32
// f64

pub trait Numeric {}

macro_rules! impl_numeric {
    ($($t:ty),*) => {
        $(impl Numeric for $t {})*
        };

    }

impl_numeric!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

pub trait CheckedAdd {
    fn checked_add(self, other: Self) -> Option<Self>
    where
        Self: Sized;
}

macro_rules! impl_checked_add {
    ($($t:ty),*) => {
        $(impl CheckedAdd for $t {
            fn checked_add(self, other: $t) -> Option<$t> {
                self.checked_add(other)
            }
        })*
        };

    }

impl_checked_add!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl CheckedAdd for f32 {
    fn checked_add(self, other: f32) -> Option<f32> {
        Some(self.add(other))
    }
}

impl CheckedAdd for f64 {
    fn checked_add(self, other: f64) -> Option<f64> {
        Some(self.add(other))
    }
}

pub trait CheckedMul {
    fn checked_mul(self, other: Self) -> Option<Self>
    where
        Self: Sized;
}

macro_rules! impl_checked_mul {
    ($($t:ty),*) => {
        $(impl CheckedMul for $t {
            fn checked_mul(self, other: $t) -> Option<$t> {
                self.checked_mul(other)
            }
        })*
        };

    }

impl_checked_mul!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl CheckedMul for f32 {
    fn checked_mul(self, other: f32) -> Option<f32> {
        Some(self.mul(other))
    }
}

impl CheckedMul for f64 {
    fn checked_mul(self, other: f64) -> Option<f64> {
        Some(self.mul(other))
    }
}

pub trait CheckedSub {
    fn checked_sub(self, other: Self) -> Option<Self>
    where
        Self: Sized;
}

macro_rules! impl_checked_sub {
    ($($t:ty),*) => {
        $(impl CheckedSub for $t {
            fn checked_sub(self, other: $t) -> Option<$t> {
                self.checked_sub(other)
            }
        })*
        };

    }

impl_checked_sub!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl CheckedSub for f32 {
    fn checked_sub(self, other: f32) -> Option<f32> {
        Some(self.sub(other))
    }
}

impl CheckedSub for f64 {
    fn checked_sub(self, other: f64) -> Option<f64> {
        Some(self.sub(other))
    }
}

pub trait CheckedDiv {
    fn checked_div(self, other: Self) -> Option<Self>
    where
        Self: Sized;
}

macro_rules! impl_checked_div {
    ($($t:ty),*) => {
        $(impl CheckedDiv for $t {
            fn checked_div(self, other: $t) -> Option<$t> {
                self.checked_div(other)
            }
        })*
        };

    }

impl_checked_div!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl CheckedDiv for f32 {
    fn checked_div(self, other: f32) -> Option<f32> {
        Some(self.div(other))
    }
}

impl CheckedDiv for f64 {
    fn checked_div(self, other: f64) -> Option<f64> {
        Some(self.div(other))
    }
}

pub trait MinMax {
    const MIN: Self;
    const MAX: Self;
}

macro_rules! impl_min_max {
    ($($t:ty),*) => {
        $(impl MinMax for $t {
            const MIN: $t = <$t>::MIN;
            const MAX: $t = <$t>::MAX;
        })*
        };

    }

impl_min_max!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

pub trait ConvertFromUsize {
    fn from_usize(val: usize) -> Self;
}

macro_rules! impl_convert_from_usize {
    ($($t:ty),*) => {
        $(impl ConvertFromUsize for $t {
            fn from_usize(val: usize) -> Self {
                val as $t
            }
        })*
        };

    }

impl_convert_from_usize!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

pub trait Tof64 {
    fn to_f64(val: Self) -> f64;
}

macro_rules! impl_convert_to_f64 {
    ($($t:ty),*) => {
        $(impl Tof64 for $t {
            fn to_f64(val: Self) -> f64 {
                val as f64
            }
        })*
        };

    }

impl_convert_to_f64!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

pub trait NumericType<T>:
    Numeric
    + Display
    + Debug
    + Default
    + Send
    + Sync
    + 'static
    + Copy
    + PartialOrd
    + PartialEq
    + CheckedAdd
    + CheckedMul
    + CheckedSub
    + CheckedDiv
    + MinMax
    + ConvertFromUsize
    + Tof64
{
}
impl<
        T: Numeric
            + Display
            + Debug
            + Default
            + Send
            + Sync
            + 'static
            + Copy
            + PartialOrd
            + PartialEq
            + CheckedAdd
            + CheckedMul
            + CheckedSub
            + CheckedDiv
            + MinMax
            + ConvertFromUsize
            + Tof64,
    > NumericType<T> for T
{
}
