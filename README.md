# Granite Reckoner

A simple calculator enabling basic aggregations on Vec<T> in the specified number of threads. 
Works for all Rust's built-in numeric types. Applies ```checked_*``` for sum and ```partial_cmp``` for min/max.

## Usage

```rust
use granite_reckoner::Column;

fn main() {
    let container: Vec<i8> = vec![123, 1, 1, 2];
    let column = Column::new();
    let column = column.add_data(container);
    column.print();
    // Deafult number of threads from constants::NUMBER_OF_NODES
    let _result = column.sum();
    let _result = column.sum_x2();
    let _result = column.count();
    let _result = column.min();
    let _result = column.max();
    // Specified number of threads within range (1..=constants::NUMBER_OF_NODES)
    let _result = column.sum_t(2);
    let _result = column.sum_x2_t(4);
    let _result = column.min_t(4);
    let _result = column.max_t(6);
}
```
