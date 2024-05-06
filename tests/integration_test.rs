use assert_approx_eq::assert_approx_eq;
use granite_reckoner::Column;

mod common;

#[test]
fn test_column_sum_function_on_f32_works() {
    let container: Vec<f32> = vec![1.0, 4.5, 6.2, 2.4, 8.7, 5.5, 2.3, 4.2, 1.9];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 36.7, 1e-5);
}

#[test]
fn test_column_sum_function_on_f64_works() {
    let container: Vec<f64> = vec![1.0, 4.5, 6.2, 2.4, 8.7, 5.5, 2.3, 4.2, 1.9];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 36.7, 1e-5);
}

#[test]
fn test_column_sum_function_on_u8_works() {
    let container: Vec<u8> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum().expect("there shouldn't be overflow");
    assert_eq!(result, 33);
}

#[test]
fn test_column_sum_function_u8_overflow_works() {
    let container: Vec<u8> = vec![u8::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum();
    assert_eq!(result, None);
}

#[test]
fn test_column_sum_function_on_u16_works() {
    let container: Vec<u16> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum().expect("there shouldn't be overflow");
    assert_eq!(result, 33);
}

#[test]
fn test_column_sum_function_u16_overflow_works() {
    let container: Vec<u16> = vec![u16::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum();
    assert_eq!(result, None);
}

#[test]
fn test_column_sum_function_on_u32_works() {
    let container: Vec<u32> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum().expect("there shouldn't be overflow");
    assert_eq!(result, 33);
}

#[test]
fn test_column_sum_function_u32_overflow_works() {
    let container: Vec<u32> = vec![u32::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum();
    assert_eq!(result, None);
}
#[test]
fn test_column_sum_function_on_u64_works() {
    let container: Vec<u64> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum().expect("there shouldn't be overflow");
    assert_eq!(result, 33);
}

#[test]
fn test_column_sum_function_u64_overflow_works() {
    let container: Vec<u64> = vec![u64::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum();
    assert_eq!(result, None);
}

#[test]
fn test_column_sum_function_on_usize_works() {
    let container: Vec<usize> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum().expect("there shouldn't be overflow");
    assert_eq!(result, 33);
}

#[test]
fn test_column_sum_function_usize_overflow_works() {
    let container: Vec<usize> = vec![usize::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum();
    assert_eq!(result, None);
}

#[test]
fn test_column_sum_function_on_u128_works() {
    let container: Vec<u128> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum().expect("there shouldn't be overflow");
    assert_eq!(result, 33);
}

#[test]
fn test_column_sum_function_u128_overflow_works() {
    let container: Vec<u128> = vec![u128::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum();
    assert_eq!(result, None);
}

#[test]
fn test_column_sum_function_on_i8_works() {
    let container: Vec<i8> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum().expect("there shouldn't be overflow");
    assert_eq!(result, 26);
}

#[test]
fn test_column_sum_function_i8_overflow_works() {
    let container: Vec<i8> = vec![i8::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum();
    assert_eq!(result, None);
}

#[test]
fn test_column_sum_function_on_i16_works() {
    let container: Vec<i16> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum().expect("there shouldn't be overflow");
    assert_eq!(result, 26);
}

#[test]
fn test_column_sum_function_i16_overflow_works() {
    let container: Vec<i16> = vec![i16::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum();
    assert_eq!(result, None);
}

#[test]
fn test_column_sum_function_on_i32_works() {
    let container: Vec<i32> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum().expect("there shouldn't be overflow");
    assert_eq!(result, 26);
}

#[test]
fn test_column_sum_function_i32_overflow_works() {
    let container: Vec<i32> = vec![i32::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum();
    assert_eq!(result, None);
}

#[test]
fn test_column_sum_function_on_i64_works() {
    let container: Vec<i64> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum().expect("there shouldn't be overflow");
    assert_eq!(result, 26);
}

#[test]
fn test_column_sum_function_i64_overflow_works() {
    let container: Vec<i64> = vec![i64::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum();
    assert_eq!(result, None);
}

#[test]
fn test_column_sum_function_on_i128_works() {
    let container: Vec<i128> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum().expect("there shouldn't be overflow");
    assert_eq!(result, 26);
}

#[test]
fn test_column_sum_function_i128_overflow_works() {
    let container: Vec<i128> = vec![i128::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum();
    assert_eq!(result, None);
}

#[test]
fn test_column_sum_function_on_isize_works() {
    let container: Vec<isize> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum().expect("there shouldn't be overflow");
    assert_eq!(result, 26);
}

#[test]
fn test_column_sum_function_isize_overflow_works() {
    let container: Vec<isize> = vec![isize::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum();
    assert_eq!(result, None);
}

#[test]
fn test_column_count_function_on_f32_works() {
    let container: Vec<f32> = vec![1.0, 4.5, 6.2, 2.4, 8.7, 5.5, 2.3, 4.2, 1.9];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.count();
    assert_eq!(result, Some(9));
}

#[test]
fn test_column_count_function_on_f64_works() {
    let container: Vec<f64> = vec![1.0, 4.5, 6.2, 2.4, 8.7, 5.5, 2.3, 4.2, 1.9];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.count();
    assert_eq!(result, Some(9));
}

#[test]
fn test_column_count_function_on_u8_works() {
    let container: Vec<u8> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.count();
    assert_eq!(result, Some(9));
}

#[test]
fn test_column_count_function_on_u16_works() {
    let container: Vec<u16> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.count();
    assert_eq!(result, Some(9));
}

#[test]
fn test_column_count_function_on_u32_works() {
    let container: Vec<u32> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.count();
    assert_eq!(result, Some(9));
}

#[test]
fn test_column_count_function_on_u64_works() {
    let container: Vec<u64> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.count();
    assert_eq!(result, Some(9));
}

#[test]
fn test_column_count_function_on_usize_works() {
    let container: Vec<usize> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.count();
    assert_eq!(result, Some(9));
}

#[test]
fn test_column_count_function_on_u128_works() {
    let container: Vec<u128> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.count();
    assert_eq!(result, Some(9));
}

#[test]
fn test_column_count_function_on_i8_works() {
    let container: Vec<i8> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.count();
    assert_eq!(result, Some(10));
}

#[test]
fn test_column_count_function_on_i16_works() {
    let container: Vec<i16> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.count();
    assert_eq!(result, Some(10));
}

#[test]
fn test_column_count_function_on_i32_works() {
    let container: Vec<i32> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.count();
    assert_eq!(result, Some(10));
}

#[test]
fn test_column_count_function_on_i64_works() {
    let container: Vec<i64> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.count();
    assert_eq!(result, Some(10));
}

#[test]
fn test_column_count_function_on_i128_works() {
    let container: Vec<i128> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.count();
    assert_eq!(result, Some(10));
}

#[test]
fn test_column_count_function_on_isize_works() {
    let container: Vec<isize> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.count();
    assert_eq!(result, Some(10));
}

#[test]
fn test_column_min_function_on_f32_works() {
    let container: Vec<f32> = vec![1.0, 4.5, 6.2, 2.4, 8.7, 5.5, 2.3, 4.2, 1.9];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .min()
        .expect("all the input data should be comparable");
    assert_eq!(result, 1.0);
}

#[test]
fn test_column_min_function_f32_non_comparable_works() {
    let container: Vec<f32> = vec![1.0, 4.5, f32::NAN, 6.2, 2.4, 8.7, 5.5, 2.3, 4.2, 1.9];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.min();
    assert_eq!(result, None);
}

#[test]
fn test_column_min_function_on_f64_works() {
    let container: Vec<f64> = vec![-1.5, 4.5, 6.2, 2.4, 8.7, 5.5, 2.3, 4.2, 1.9];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .min()
        .expect("all the input data should be comparable");
    assert_eq!(result, -1.5);
}

#[test]
fn test_column_min_function_on_f64_non_comparable_works() {
    let container: Vec<f64> = vec![-1.5, 4.5, 2.4, 8.7, f64::NAN, 2.3, f64::NAN, 1.9];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.min();
    assert_eq!(result, None);
}
#[test]
fn test_column_min_function_on_u8_works() {
    let container: Vec<u8> = vec![1, 4, 6, 2, 8, 0, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .min()
        .expect("all the input data should be comparable");
    assert_eq!(result, 0);
}

#[test]
fn test_column_min_function_on_u16_works() {
    let container: Vec<u16> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .min()
        .expect("all the input data should be comparable");
    assert_eq!(result, 1);
}

#[test]
fn test_column_min_function_on_u32_works() {
    let container: Vec<u32> = vec![8, 4, 6, 2, 8, 5, 2, 4, 10];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .min()
        .expect("all the input data should be comparable");
    assert_eq!(result, 2);
}

#[test]
fn test_column_min_function_on_u64_works() {
    let container: Vec<u64> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .min()
        .expect("all the input data should be comparable");
    assert_eq!(result, 1);
}

#[test]
fn test_column_min_function_on_usize_works() {
    let container: Vec<usize> = vec![11, 4, 16, 12, 18, 15, 12, 14, 11];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .min()
        .expect("all the input data should be comparable");
    assert_eq!(result, 4);
}

#[test]
fn test_column_min_function_on_u128_works() {
    let container: Vec<u128> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .min()
        .expect("all the input data should be comparable");
    assert_eq!(result, 1);
}

#[test]
fn test_column_min_function_on_i8_works() {
    let container: Vec<i8> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .min()
        .expect("all the input data should be comparable");
    assert_eq!(result, -4);
}

#[test]
fn test_column_min_function_on_i16_works() {
    let container: Vec<i16> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, -5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .min()
        .expect("all the input data should be comparable");
    assert_eq!(result, -5);
}

#[test]
fn test_column_min_function_on_i32_works() {
    let container: Vec<i32> = vec![1, -4, 6, 2, -8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .min()
        .expect("all the input data should be comparable");
    assert_eq!(result, -8);
}

#[test]
fn test_column_min_function_on_i64_works() {
    let container: Vec<i64> = vec![1, -4, 6, 2, 8, 5, -2, -14, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .min()
        .expect("all the input data should be comparable");
    assert_eq!(result, -14);
}

#[test]
fn test_column_min_function_on_i128_works() {
    let container: Vec<i128> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .min()
        .expect("all the input data should be comparable");
    assert_eq!(result, -4);
}

#[test]
fn test_column_min_function_on_isize_works() {
    let container: Vec<isize> = vec![1, -4, -16, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .min()
        .expect("all the input data should be comparable");
    assert_eq!(result, -16);
}

#[test]
fn test_column_max_function_on_f32_works() {
    let container: Vec<f32> = vec![1.0, 4.5, 6.2, 2.4, 8.7, 5.5, 2.3, 4.2, 1.9];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .max()
        .expect("all the input data should be comparable");
    assert_eq!(result, 8.7);
}

#[test]
fn test_column_max_function_f32_non_comparable_works() {
    let container: Vec<f32> = vec![1.0, 4.5, 6.2, 2.4, 8.7, f32::NAN, 5.5, 2.3, 4.2, 1.9];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.max();
    assert_eq!(result, None);
}

#[test]
fn test_column_max_function_on_f64_works() {
    let container: Vec<f64> = vec![-1.5, 4.5, 6.2, 2.4, 8.7, 15.5, 2.3, 4.2, 1.9];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .max()
        .expect("all the input data should be comparable");
    assert_eq!(result, 15.5);
}

#[test]
fn test_column_max_function_on_f64_non_comparable_works() {
    let container: Vec<f64> = vec![-1.5, f64::NAN, 6.2, 2.4, 8.7, 15.5, 2.3, 4.2, 1.9];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.max();
    assert_eq!(result, None);
}

#[test]
fn test_column_max_function_on_u8_works() {
    let container: Vec<u8> = vec![1, 4, 6, 2, 8, 0, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .max()
        .expect("all the input data should be comparable");
    assert_eq!(result, 8);
}

#[test]
fn test_column_max_function_on_u16_works() {
    let container: Vec<u16> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .max()
        .expect("all the input data should be comparable");
    assert_eq!(result, 8);
}

#[test]
fn test_column_max_function_on_u32_works() {
    let container: Vec<u32> = vec![8, 4, 6, 2, 8, 5, 2, 4, 10];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .max()
        .expect("all the input data should be comparable");
    assert_eq!(result, 10);
}

#[test]
fn test_column_max_function_on_u64_works() {
    let container: Vec<u64> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .max()
        .expect("all the input data should be comparable");
    assert_eq!(result, 8);
}

#[test]
fn test_column_max_function_on_usize_works() {
    let container: Vec<usize> = vec![11, 4, 16, 12, 18, 15, 12, 14, 11];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .max()
        .expect("all the input data should be comparable");
    assert_eq!(result, 18);
}

#[test]
fn test_column_max_function_on_u128_works() {
    let container: Vec<u128> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .max()
        .expect("all the input data should be comparable");
    assert_eq!(result, 8);
}

#[test]
fn test_column_max_function_on_i8_works() {
    let container: Vec<i8> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .max()
        .expect("all the input data should be comparable");
    assert_eq!(result, 8);
}

#[test]
fn test_column_max_function_on_i16_works() {
    let container: Vec<i16> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, -5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .max()
        .expect("all the input data should be comparable");
    assert_eq!(result, 8);
}

#[test]
fn test_column_max_function_on_i32_works() {
    let container: Vec<i32> = vec![1, -4, 6, 2, -8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .max()
        .expect("all the input data should be comparable");
    assert_eq!(result, 6);
}

#[test]
fn test_column_max_function_on_i64_works() {
    let container: Vec<i64> = vec![1, -4, 6, 2, 8, 5, -2, -14, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .max()
        .expect("all the input data should be comparable");
    assert_eq!(result, 8);
}

#[test]
fn test_column_max_function_on_i128_works() {
    let container: Vec<i128> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .max()
        .expect("all the input data should be comparable");
    assert_eq!(result, 8);
}

#[test]
fn test_column_max_function_on_isize_works() {
    let container: Vec<isize> = vec![1, -4, -16, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .max()
        .expect("all the input data should be comparable");
    assert_eq!(result, 8);
}

#[test]
fn load_test_column_sum_on_5_m_row_data_works() {
    let container: Vec<f64> = common::load_data("data_5M.txt");
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 9997165331593.543, 1e5);
}

#[test]
fn load_test_column_sum_on_10_m_row_data_works() {
    let container: Vec<f64> = common::load_data("data_10M.txt");
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 14996497014084.932, 1e5);
}

#[test]
fn load_test_column_sum_on_20_m_row_data_works() {
    let container: Vec<f64> = common::load_data("data_20M.txt");
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum().expect("there shouldn't be overflow");
    assert_approx_eq!(result, -20010771492612.383, 1e5);
}

#[test]
fn load_test_column_min_on_5_m_row_data_works() {
    let container: Vec<f64> = common::load_data("data_5M.txt");
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .min()
        .expect("all the input data should be comparable");
    assert_approx_eq!(result, -999999.5781282932, 1e5);
}

#[test]
fn load_test_column_min_on_10_m_row_data_works() {
    let container: Vec<f64> = common::load_data("data_10M.txt");
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .min()
        .expect("all the input data should be comparable");
    assert_approx_eq!(result, -1999998.7980072543, 1e5);
}

#[test]
fn load_test_column_min_on_20_m_row_data_works() {
    let container: Vec<f64> = common::load_data("data_20M.txt");
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .min()
        .expect("all the input data should be comparable");
    assert_approx_eq!(result, -6999999.502178693, 1e5);
}

#[test]
fn load_test_column_max_on_5_m_row_data_works() {
    let container: Vec<f64> = common::load_data("data_5M.txt");
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .max()
        .expect("all the input data should be comparable");
    assert_approx_eq!(result, 4999998.977779031, 1e5);
}

#[test]
fn load_test_column_max_on_10_m_row_data_works() {
    let container: Vec<f64> = common::load_data("data_10M.txt");
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .max()
        .expect("all the input data should be comparable");
    assert_approx_eq!(result, 4999999.815092964, 1e5);
}

#[test]
fn load_test_column_max_on_20_m_row_data_works() {
    let container: Vec<f64> = common::load_data("data_20M.txt");
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .max()
        .expect("all the input data should be comparable");
    assert_approx_eq!(result, 4999999.481657896, 1e5);
}

#[test]
fn load_test_column_count_on_5_m_row_data_works() {
    let container: Vec<f64> = common::load_data("data_5M.txt");
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.count().expect("count should always work");
    assert_eq!(result, 5000000);
}

#[test]
fn load_test_column_count_on_10_m_row_data_works() {
    let container: Vec<f64> = common::load_data("data_10M.txt");
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.count().expect("count should always work");
    assert_eq!(result, 10000000);
}

#[test]
fn load_test_column_count_on_20_m_row_data_works() {
    let container: Vec<f64> = common::load_data("data_20M.txt");
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.count().expect("count should always work");
    assert_eq!(result, 20000000);
}
