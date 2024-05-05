use std::{
    default::Default,
    fmt::{Debug, Display},
    marker::{Send, Sync},
    ops::Add,
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

pub trait NumericType<T>:
    Numeric
    + Display
    + for<'a> Add<&'a T, Output = T>
    + Debug
    + Default
    + Send
    + Sync
    + 'static
    + Copy
    + PartialOrd
    + PartialEq
    + CheckedAdd
{
}
impl<
        T: Numeric
            + Display
            + for<'a> Add<&'a T, Output = T>
            + Debug
            + Default
            + Send
            + Sync
            + 'static
            + Copy
            + PartialOrd
            + PartialEq
            + CheckedAdd,
    > NumericType<T> for T
{
}
