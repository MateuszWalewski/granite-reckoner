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

impl Numeric for i8 {}
impl Numeric for i16 {}
impl Numeric for i32 {}
impl Numeric for i64 {}
impl Numeric for i128 {}
impl Numeric for isize {}
impl Numeric for u8 {}
impl Numeric for u16 {}
impl Numeric for u32 {}
impl Numeric for u64 {}
impl Numeric for u128 {}
impl Numeric for usize {}
impl Numeric for f32 {}
impl Numeric for f64 {}

pub trait CheckedAdd {
    fn checked_add(self, other: Self) -> Option<Self>
    where
        Self: Sized;
}

impl CheckedAdd for i8 {
    fn checked_add(self, other: i8) -> Option<i8> {
        self.checked_add(other)
    }
}

impl CheckedAdd for i16 {
    fn checked_add(self, other: i16) -> Option<i16> {
        self.checked_add(other)
    }
}

impl CheckedAdd for i32 {
    fn checked_add(self, other: i32) -> Option<i32> {
        self.checked_add(other)
    }
}

impl CheckedAdd for i64 {
    fn checked_add(self, other: i64) -> Option<i64> {
        self.checked_add(other)
    }
}

impl CheckedAdd for i128 {
    fn checked_add(self, other: i128) -> Option<i128> {
        self.checked_add(other)
    }
}

impl CheckedAdd for isize {
    fn checked_add(self, other: isize) -> Option<isize> {
        self.checked_add(other)
    }
}

impl CheckedAdd for u8 {
    fn checked_add(self, other: u8) -> Option<u8> {
        self.checked_add(other)
    }
}

impl CheckedAdd for u16 {
    fn checked_add(self, other: u16) -> Option<u16> {
        self.checked_add(other)
    }
}

impl CheckedAdd for u32 {
    fn checked_add(self, other: u32) -> Option<u32> {
        self.checked_add(other)
    }
}

impl CheckedAdd for u64 {
    fn checked_add(self, other: u64) -> Option<u64> {
        self.checked_add(other)
    }
}

impl CheckedAdd for u128 {
    fn checked_add(self, other: u128) -> Option<u128> {
        self.checked_add(other)
    }
}

impl CheckedAdd for usize {
    fn checked_add(self, other: usize) -> Option<usize> {
        self.checked_add(other)
    }
}

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

impl CheckedMul for i8 {
    fn checked_mul(self, other: i8) -> Option<i8> {
        self.checked_mul(other)
    }
}

impl CheckedMul for i16 {
    fn checked_mul(self, other: i16) -> Option<i16> {
        self.checked_mul(other)
    }
}

impl CheckedMul for i32 {
    fn checked_mul(self, other: i32) -> Option<i32> {
        self.checked_mul(other)
    }
}

impl CheckedMul for i64 {
    fn checked_mul(self, other: i64) -> Option<i64> {
        self.checked_mul(other)
    }
}

impl CheckedMul for i128 {
    fn checked_mul(self, other: i128) -> Option<i128> {
        self.checked_mul(other)
    }
}

impl CheckedMul for isize {
    fn checked_mul(self, other: isize) -> Option<isize> {
        self.checked_mul(other)
    }
}

impl CheckedMul for u8 {
    fn checked_mul(self, other: u8) -> Option<u8> {
        self.checked_mul(other)
    }
}

impl CheckedMul for u16 {
    fn checked_mul(self, other: u16) -> Option<u16> {
        self.checked_mul(other)
    }
}

impl CheckedMul for u32 {
    fn checked_mul(self, other: u32) -> Option<u32> {
        self.checked_mul(other)
    }
}

impl CheckedMul for u64 {
    fn checked_mul(self, other: u64) -> Option<u64> {
        self.checked_mul(other)
    }
}

impl CheckedMul for u128 {
    fn checked_mul(self, other: u128) -> Option<u128> {
        self.checked_mul(other)
    }
}

impl CheckedMul for usize {
    fn checked_mul(self, other: usize) -> Option<usize> {
        self.checked_mul(other)
    }
}

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

impl CheckedSub for i8 {
    fn checked_sub(self, other: i8) -> Option<i8> {
        self.checked_sub(other)
    }
}

impl CheckedSub for i16 {
    fn checked_sub(self, other: i16) -> Option<i16> {
        self.checked_sub(other)
    }
}

impl CheckedSub for i32 {
    fn checked_sub(self, other: i32) -> Option<i32> {
        self.checked_sub(other)
    }
}

impl CheckedSub for i64 {
    fn checked_sub(self, other: i64) -> Option<i64> {
        self.checked_sub(other)
    }
}

impl CheckedSub for i128 {
    fn checked_sub(self, other: i128) -> Option<i128> {
        self.checked_sub(other)
    }
}

impl CheckedSub for isize {
    fn checked_sub(self, other: isize) -> Option<isize> {
        self.checked_sub(other)
    }
}

impl CheckedSub for u8 {
    fn checked_sub(self, other: u8) -> Option<u8> {
        self.checked_sub(other)
    }
}

impl CheckedSub for u16 {
    fn checked_sub(self, other: u16) -> Option<u16> {
        self.checked_sub(other)
    }
}

impl CheckedSub for u32 {
    fn checked_sub(self, other: u32) -> Option<u32> {
        self.checked_sub(other)
    }
}

impl CheckedSub for u64 {
    fn checked_sub(self, other: u64) -> Option<u64> {
        self.checked_sub(other)
    }
}

impl CheckedSub for u128 {
    fn checked_sub(self, other: u128) -> Option<u128> {
        self.checked_sub(other)
    }
}

impl CheckedSub for usize {
    fn checked_sub(self, other: usize) -> Option<usize> {
        self.checked_sub(other)
    }
}

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

impl CheckedDiv for i8 {
    fn checked_div(self, other: i8) -> Option<i8> {
        self.checked_div(other)
    }
}

impl CheckedDiv for i16 {
    fn checked_div(self, other: i16) -> Option<i16> {
        self.checked_div(other)
    }
}

impl CheckedDiv for i32 {
    fn checked_div(self, other: i32) -> Option<i32> {
        self.checked_div(other)
    }
}

impl CheckedDiv for i64 {
    fn checked_div(self, other: i64) -> Option<i64> {
        self.checked_div(other)
    }
}

impl CheckedDiv for i128 {
    fn checked_div(self, other: i128) -> Option<i128> {
        self.checked_div(other)
    }
}

impl CheckedDiv for isize {
    fn checked_div(self, other: isize) -> Option<isize> {
        self.checked_div(other)
    }
}

impl CheckedDiv for u8 {
    fn checked_div(self, other: u8) -> Option<u8> {
        self.checked_div(other)
    }
}

impl CheckedDiv for u16 {
    fn checked_div(self, other: u16) -> Option<u16> {
        self.checked_div(other)
    }
}

impl CheckedDiv for u32 {
    fn checked_div(self, other: u32) -> Option<u32> {
        self.checked_div(other)
    }
}

impl CheckedDiv for u64 {
    fn checked_div(self, other: u64) -> Option<u64> {
        self.checked_div(other)
    }
}

impl CheckedDiv for u128 {
    fn checked_div(self, other: u128) -> Option<u128> {
        self.checked_div(other)
    }
}

impl CheckedDiv for usize {
    fn checked_div(self, other: usize) -> Option<usize> {
        self.checked_div(other)
    }
}

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

impl MinMax for u8 {
    const MIN: u8 = u8::MIN;
    const MAX: u8 = u8::MAX;
}

impl MinMax for u16 {
    const MIN: u16 = u16::MIN;
    const MAX: u16 = u16::MAX;
}

impl MinMax for u32 {
    const MIN: u32 = u32::MIN;
    const MAX: u32 = u32::MAX;
}

impl MinMax for u64 {
    const MIN: u64 = u64::MIN;
    const MAX: u64 = u64::MAX;
}

impl MinMax for u128 {
    const MIN: u128 = u128::MIN;
    const MAX: u128 = u128::MAX;
}

impl MinMax for usize {
    const MIN: usize = usize::MIN;
    const MAX: usize = usize::MAX;
}

impl MinMax for i8 {
    const MIN: i8 = i8::MIN;
    const MAX: i8 = i8::MAX;
}

impl MinMax for i16 {
    const MIN: i16 = i16::MIN;
    const MAX: i16 = i16::MAX;
}

impl MinMax for i32 {
    const MIN: i32 = i32::MIN;
    const MAX: i32 = i32::MAX;
}

impl MinMax for i64 {
    const MIN: i64 = i64::MIN;
    const MAX: i64 = i64::MAX;
}

impl MinMax for i128 {
    const MIN: i128 = i128::MIN;
    const MAX: i128 = i128::MAX;
}

impl MinMax for isize {
    const MIN: isize = isize::MIN;
    const MAX: isize = isize::MAX;
}

impl MinMax for f32 {
    const MIN: f32 = f32::MIN;
    const MAX: f32 = f32::MAX;
}

impl MinMax for f64 {
    const MIN: f64 = f64::MIN;
    const MAX: f64 = f64::MAX;
}
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
