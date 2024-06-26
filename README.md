# Granite Reckoner

A simple aggregator enabling basic statistical operations on Vec<T> in the specified number of threads. 
Applies ```checked_*``` and ```partial_cmp``` to deal with overflows and non-comparable input. 
Works for all Rust's built-in numeric types. 

## Usage

```rust
use granite_reckoner::Column;

fn main() {
    let container: Vec<u32> = vec![123, 1, 1, 2];
    let column = Column::new();
    let column = column.add_data(container);
    column.print();
    // Default number of threads from constants::NUMBER_OF_NODES
    let _result = column.sum();
    let _result = column.sum_x2(); // sums x^2
    let _result = column.moment_i(); // = average
    let _result = column.moment_ii();
    let _result = column.variance(); // sample variance
    let _result = column.stddev(); // sample standard deviation
    let _result = column.count();
    let _result = column.min();
    let _result = column.max();
    // Specified number of threads within range (1..=constants::NUMBER_OF_NODES)
    let _result = column.sum_t(2);
    let _result = column.sum_x2_t(4);
    let _result = column.moment_i_t(2);
    let _result = column.moment_ii_t(6);
    let _result = column.variance_t(4);
    let _result = column.stddev_t(6);
    let _result = column.count_t(6);
    let _result = column.min_t(4);
    let _result = column.max_t(6);
}
```
