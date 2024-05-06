//! # Granite Reckoner
//!
//! `granite_reckoner` is a simple calculator enabling basic aggregations on Vec<T> in the specified number of threads. Works for all Rust's built-in numeric types.
//!  Returns "None" for overflows and non-comparable data. 

use std::marker::PhantomData;

mod aggregator;
mod column_data;
mod constants;
mod tools;
mod traits;

use crate::column_data::ColumnData;
use crate::traits::NumericType;

/// Data wrapper without any data yet.
pub struct EmptyColumn<T: NumericType<T>> {
    phantom: PhantomData<T>,
}

/// Data wrapper populated with values.
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
        aggregator::sum(&self.data, constants::NUMBER_OF_NODES)
    }

    pub fn sum_t(&self, number_of_threads: usize) -> Option<T> {
        if number_of_threads < 1 || number_of_threads > constants::NUMBER_OF_NODES {
            println!("The number of threads must be in range: (1..constants::NUMBER_OF_NODES)");
            return None;
        }
        aggregator::sum(&self.data, number_of_threads)
    }

    pub fn min(&self) -> Option<T> {
        aggregator::min(&self.data, constants::NUMBER_OF_NODES)
    }

    pub fn min_t(&self, number_of_threads: usize) -> Option<T> {
        if number_of_threads > constants::NUMBER_OF_NODES {
            println!("The number of threads must be in range: (1..constants::NUMBER_OF_NODES)");
            return None;
        }
        aggregator::min(&self.data, number_of_threads)
    }

    pub fn max(&self) -> Option<T> {
        aggregator::max(&self.data, constants::NUMBER_OF_NODES)
    }

    pub fn max_t(&self, number_of_threads: usize) -> Option<T> {
        if number_of_threads > constants::NUMBER_OF_NODES {
            println!("The number of threads must be in range: (1..constants::NUMBER_OF_NODES)");
            return None;
        }
        aggregator::max(&self.data, number_of_threads)
    }

    pub fn count(&self) -> Option<usize> {
        // for the interface consistency
        Some(self.data.data().len())
    }

    pub fn print(&self) {
        println!("{:?}", &self);
    }
}
