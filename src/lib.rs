//! # Granite Reckoner
//!
//! `granite_reckoner` is a simple calculator enabling basic aggregations on Vec<T> in the specified number of threads. Works for all Rust's built-in numeric types.
//!  Returns "None" for overflows and non-comparable data.
//!  As for the primitives, the error propagates accordingly for high multiplicities.

use std::marker::PhantomData;
pub mod constants;

mod aggregator;
mod column_data;
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
    /// Sums x
    pub fn sum(&self) -> Option<T> {
        aggregator::sum(&self.data, constants::NUMBER_OF_NODES)
    }
    /// Sums x^2
    pub fn sum_x2(&self) -> Option<T> {
        aggregator::sum_x2(&self.data, constants::NUMBER_OF_NODES)
    }

    /// Moment-I (pop. Average)
    pub fn moment_i(&self) -> Option<f64> {
        aggregator::moment_i(&self.data, constants::NUMBER_OF_NODES)
    }

    /// Moment-II
    pub fn moment_ii(&self) -> Option<f64> {
        aggregator::moment_ii(&self.data, constants::NUMBER_OF_NODES)
    }

    /// Sample Variance. (Square of the sample standard deviation).
    pub fn variance(&self) -> Option<f64> {
        aggregator::variance(&self.data, constants::NUMBER_OF_NODES)
    }

    /// Sample Standard Deviation.
    pub fn stddev(&self) -> Option<f64> {
        aggregator::stddev(&self.data, constants::NUMBER_OF_NODES)
    }

    pub fn min(&self) -> Option<T> {
        aggregator::min(&self.data, constants::NUMBER_OF_NODES)
    }

    pub fn max(&self) -> Option<T> {
        aggregator::max(&self.data, constants::NUMBER_OF_NODES)
    }

    pub fn count(&self) -> Option<usize> {
        // O(1) complexity
        aggregator::count(&self.data)
    }

    pub fn print(&self) {
        println!("{:?}", &self);
    }

    pub fn sum_t(&self, number_of_threads: usize) -> Option<T> {
        if number_of_threads < 1 || number_of_threads > constants::NUMBER_OF_NODES {
            println!("The number of threads must be in range: (1..=constants::NUMBER_OF_NODES)");
            return None;
        }
        aggregator::sum(&self.data, number_of_threads)
    }

    pub fn sum_x2_t(&self, number_of_threads: usize) -> Option<T> {
        if number_of_threads < 1 || number_of_threads > constants::NUMBER_OF_NODES {
            println!("The number of threads must be in range: (1..=constants::NUMBER_OF_NODES)");
            return None;
        }
        aggregator::sum_x2(&self.data, number_of_threads)
    }

    pub fn moment_i_t(&self, number_of_threads: usize) -> Option<f64> {
        if number_of_threads < 1 || number_of_threads > constants::NUMBER_OF_NODES {
            println!("The number of threads must be in range: (1..=constants::NUMBER_OF_NODES)");
            return None;
        }
        aggregator::moment_i(&self.data, number_of_threads)
    }

    pub fn moment_ii_t(&self, number_of_threads: usize) -> Option<f64> {
        if number_of_threads < 1 || number_of_threads > constants::NUMBER_OF_NODES {
            println!("The number of threads must be in range: (1..=constants::NUMBER_OF_NODES)");
            return None;
        }
        aggregator::moment_ii(&self.data, number_of_threads)
    }

    pub fn variance_t(&self, number_of_threads: usize) -> Option<f64> {
        if number_of_threads < 1 || number_of_threads > constants::NUMBER_OF_NODES {
            println!("The number of threads must be in range: (1..=constants::NUMBER_OF_NODES)");
            return None;
        }
        aggregator::variance(&self.data, number_of_threads)
    }

    pub fn stddev_t(&self, number_of_threads: usize) -> Option<f64> {
        if number_of_threads < 1 || number_of_threads > constants::NUMBER_OF_NODES {
            println!("The number of threads must be in range: (1..=constants::NUMBER_OF_NODES)");
            return None;
        }
        aggregator::stddev(&self.data, number_of_threads)
    }

    pub fn min_t(&self, number_of_threads: usize) -> Option<T> {
        if number_of_threads < 1 || number_of_threads > constants::NUMBER_OF_NODES {
            println!("The number of threads must be in range: (1..=constants::NUMBER_OF_NODES)");
            return None;
        }
        aggregator::min(&self.data, number_of_threads)
    }

    pub fn max_t(&self, number_of_threads: usize) -> Option<T> {
        if number_of_threads < 1 || number_of_threads > constants::NUMBER_OF_NODES {
            println!("The number of threads must be in range: (1..=constants::NUMBER_OF_NODES)");
            return None;
        }
        aggregator::max(&self.data, number_of_threads)
    }

    // just to preserve the interface consistency
    pub fn count_t(&self, number_of_threads: usize) -> Option<usize> {
        if number_of_threads < 1 || number_of_threads > constants::NUMBER_OF_NODES {
            println!("The number of threads must be in range: (1..=constants::NUMBER_OF_NODES)");
            return None;
        }
        // O(1) complexity
        aggregator::count(&self.data)
    }
}
