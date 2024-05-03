use std::{
    default::Default,
    fmt::{self, Debug, Display, Formatter},
    marker::PhantomData,
    ops::Add,
};

mod aggregator;
mod column_data;
mod constants;
mod tools;

use crate::column_data::ColumnData;

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

pub struct EmptyColumn<T: Numeric> {
    phantom: PhantomData<T>,
}

pub struct Column<T: Numeric> {
    data: ColumnData<T>,
}

impl<T: Numeric> EmptyColumn<T> {
    pub fn add_data(self, container: Vec<T>) -> Column<T> {
        Column {
            data: ColumnData::<T>::new(container),
        }
    }
}

impl<T: Numeric + Display> Debug for Column<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let inner_data = self.data.data();
        for idx in 0..inner_data.len() - 1 {
            write!(f, "{}\n", inner_data[idx])?;
        }
        write!(f, "{}", inner_data[inner_data.len() - 1])?;

        Ok(())
    }
}

impl<
        T: for<'a> Add<&'a T, Output = T>
            + Numeric
            + Debug
            + Display
            + Default
            + std::marker::Send
            + std::marker::Sync
            + 'static,
    > Column<T>
{
    pub fn new() -> EmptyColumn<T> {
        EmptyColumn::<T> {
            phantom: PhantomData,
        }
    }

    pub fn sum(&self) -> T {
        aggregator::sum(&self.data)
    }

    pub fn count(&self) -> usize {
        aggregator::count(&self.data)
    }

    pub fn print(&self) {
        println!("{:?}", &self);
    }
}
