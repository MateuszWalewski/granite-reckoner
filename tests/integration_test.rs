use assert_approx_eq::assert_approx_eq;
use granite_reckoner::Column;

#[test]
fn test_column_sum_function_on_f32_works() {
    let container: Vec<f32> = vec![1.0, 4.5, 6.2, 2.4, 8.7, 5.5, 2.3, 4.2, 1.9];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum();
    assert_approx_eq!(result, 36.7, 1e-5);
}

#[test]
fn test_column_sum_function_on_f64_works() {
    let container: Vec<f64> = vec![1.0, 4.5, 6.2, 2.4, 8.7, 5.5, 2.3, 4.2, 1.9];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum();
    assert_approx_eq!(result, 36.7, 1e-5);
}

#[test]
fn test_column_sum_function_on_u8_works() {
    let container: Vec<u8> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum();
    assert_eq!(result, 33);
}

#[test]
fn test_column_sum_function_on_u16_works() {
    let container: Vec<u16> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum();
    assert_eq!(result, 33);
}

#[test]
fn test_column_sum_function_on_u32_works() {
    let container: Vec<u32> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum();
    assert_eq!(result, 33);
}

#[test]
fn test_column_sum_function_on_u64_works() {
    let container: Vec<u64> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum();
    assert_eq!(result, 33);
}

#[test]
fn test_column_sum_function_on_usize_works() {
    let container: Vec<usize> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum();
    assert_eq!(result, 33);
}

#[test]
fn test_column_sum_function_on_u128_works() {
    let container: Vec<u128> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum();
    assert_eq!(result, 33);
}

#[test]
fn test_column_sum_function_on_i8_works() {
    let container: Vec<i8> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum();
    assert_eq!(result, 26);
}

#[test]
fn test_column_sum_function_on_i16_works() {
    let container: Vec<i16> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum();
    assert_eq!(result, 26);
}

#[test]
fn test_column_sum_function_on_i32_works() {
    let container: Vec<i32> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum();
    assert_eq!(result, 26);
}

#[test]
fn test_column_sum_function_on_i64_works() {
    let container: Vec<i64> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum();
    assert_eq!(result, 26);
}

#[test]
fn test_column_sum_function_on_i128_works() {
    let container: Vec<i128> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum();
    assert_eq!(result, 26);
}

#[test]
fn test_column_sum_function_on_isize_works() {
    let container: Vec<isize> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum();
    assert_eq!(result, 26);
}

#[test]
fn test_column_count_function_on_f32_works() {
    let container: Vec<f32> = vec![1.0, 4.5, 6.2, 2.4, 8.7, 5.5, 2.3, 4.2, 1.9];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.count();
    assert_eq!(result, 9);
}

#[test]
fn test_column_count_function_on_f64_works() {
    let container: Vec<f64> = vec![1.0, 4.5, 6.2, 2.4, 8.7, 5.5, 2.3, 4.2, 1.9];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.count();
    assert_eq!(result, 9);
}

#[test]
fn test_column_count_function_on_u8_works() {
    let container: Vec<u8> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.count();
    assert_eq!(result, 9);
}

#[test]
fn test_column_count_function_on_u16_works() {
    let container: Vec<u16> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.count();
    assert_eq!(result, 9);
}

#[test]
fn test_column_count_function_on_u32_works() {
    let container: Vec<u32> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.count();
    assert_eq!(result, 9);
}

#[test]
fn test_column_count_function_on_u64_works() {
    let container: Vec<u64> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.count();
    assert_eq!(result, 9);
}

#[test]
fn test_column_count_function_on_usize_works() {
    let container: Vec<usize> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.count();
    assert_eq!(result, 9);
}

#[test]
fn test_column_count_function_on_u128_works() {
    let container: Vec<u128> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.count();
    assert_eq!(result, 9);
}

#[test]
fn test_column_count_function_on_i8_works() {
    let container: Vec<i8> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.count();
    assert_eq!(result, 10);
}

#[test]
fn test_column_count_function_on_i16_works() {
    let container: Vec<i16> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.count();
    assert_eq!(result, 10);
}

#[test]
fn test_column_count_function_on_i32_works() {
    let container: Vec<i32> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.count();
    assert_eq!(result, 10);
}

#[test]
fn test_column_count_function_on_i64_works() {
    let container: Vec<i64> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.count();
    assert_eq!(result, 10);
}

#[test]
fn test_column_count_function_on_i128_works() {
    let container: Vec<i128> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.count();
    assert_eq!(result, 10);
}

#[test]
fn test_column_count_function_on_isize_works() {
    let container: Vec<isize> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.count();
    assert_eq!(result, 10);
}
