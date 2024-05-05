use std::marker::PhantomData;

mod aggregator;
mod column_data;
mod constants;
mod tools;
mod traits;

use crate::column_data::ColumnData;
use crate::traits::NumericType;

pub struct EmptyColumn<T: NumericType<T>> {
    phantom: PhantomData<T>,
}

pub struct Column<T: NumericType<T>> {
    data: ColumnData<T>,
}

impl<T: NumericType<T>> EmptyColumn<T> {
    pub fn add_data(self, container: Vec<T>) -> Column<T> {
        Column {
            data: ColumnData::<T>::new(container),
        }
    }
}

impl<T: NumericType<T>> Column<T> {
    pub fn new() -> EmptyColumn<T> {
        EmptyColumn::<T> {
            phantom: PhantomData,
        }
    }

    pub fn sum(&self) -> Option<T> {
        aggregator::sum(&self.data)
    }

    pub fn min(&self) -> Option<T> {
        aggregator::min(&self.data)
    }

    pub fn max(&self) -> Option<T> {
        aggregator::max(&self.data)
    }

    pub fn count(&self) -> Option<usize> {
        // for returning consistency
        Some(self.data.data().len())
    }

    pub fn print(&self) {
        println!("{:?}", &self);
    }
}
