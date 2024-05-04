use std::{
    default::Default,
    fmt::{Debug, Display},
    marker::{Send, Sync},
    ops::Add,
};

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
            + PartialEq,
    > NumericType<T> for T
{
}
