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
    let container: Vec<i8> = vec![1, -4, 6, 1, 1, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum().expect("there shouldn't be overflow");
    assert_eq!(result, 18);
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
fn test_column_sum_x2_function_on_f32_works() {
    let container: Vec<f32> = vec![1.0, 4.5, 6.2, 2.4, 8.7, 5.5, 2.3, 4.2, 1.9];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum_x2().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 197.93, 1.0e-4);
}

#[test]
fn test_column_sum_x2_function_on_f64_works() {
    let container: Vec<f64> = vec![1.0, 4.5, 6.2, 2.4, 8.7, 5.5, 2.3, 4.2, 1.9];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum_x2().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 197.93, 1.0e-4);
}

#[test]
fn test_column_sum_x2_function_on_u8_works() {
    let container: Vec<u8> = vec![1, 4, 2, 2, 0, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum_x2().expect("there shouldn't be overflow");
    assert_eq!(result, 71);
}

#[test]
fn test_column_sum_x2_function_u8_overflow_works() {
    let container: Vec<u8> = vec![u8::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum_x2();
    assert_eq!(result, None);
}

#[test]
fn test_column_sum_x2_function_on_u16_works() {
    let container: Vec<u16> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum_x2().expect("there shouldn't be overflow");
    assert_eq!(result, 167);
}

#[test]
fn test_column_sum_x2_function_u16_overflow_works() {
    let container: Vec<u16> = vec![u16::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum_x2();
    assert_eq!(result, None);
}

#[test]
fn test_column_sum_x2_function_on_u32_works() {
    let container: Vec<u32> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum_x2().expect("there shouldn't be overflow");
    assert_eq!(result, 167);
}

#[test]
fn test_column_sum_x2_function_u32_overflow_works() {
    let container: Vec<u32> = vec![u32::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum_x2();
    assert_eq!(result, None);
}
#[test]
fn test_column_sum_x2_function_on_u64_works() {
    let container: Vec<u64> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum_x2().expect("there shouldn't be overflow");
    assert_eq!(result, 167);
}

#[test]
fn test_column_sum_x2_function_u64_overflow_works() {
    let container: Vec<u64> = vec![u64::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum_x2();
    assert_eq!(result, None);
}

#[test]
fn test_column_sum_x2_function_on_usize_works() {
    let container: Vec<usize> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum_x2().expect("there shouldn't be overflow");
    assert_eq!(result, 167);
}

#[test]
fn test_column_sum_x2_function_usize_overflow_works() {
    let container: Vec<usize> = vec![usize::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum_x2();
    assert_eq!(result, None);
}

#[test]
fn test_column_sum_x2_function_on_u128_works() {
    let container: Vec<u128> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum_x2().expect("there shouldn't be overflow");
    assert_eq!(result, 167);
}

#[test]
fn test_column_sum_x2_function_u128_overflow_works() {
    let container: Vec<u128> = vec![u128::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum_x2();
    assert_eq!(result, None);
}

#[test]
fn test_column_sum_x2_function_on_i8_works() {
    let container: Vec<i8> = vec![1, -4, 2, 2, 2, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum_x2().expect("there shouldn't be overflow");
    assert_eq!(result, 100);
}

#[test]
fn test_column_sum_x2_function_i8_overflow_works() {
    let container: Vec<i8> = vec![i8::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum_x2();
    assert_eq!(result, None);
}

#[test]
fn test_column_sum_x2_function_on_i16_works() {
    let container: Vec<i16> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum_x2().expect("there shouldn't be overflow");
    assert_eq!(result, 192);
}

#[test]
fn test_column_sum_x2_function_i16_overflow_works() {
    let container: Vec<i16> = vec![i16::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum_x2();
    assert_eq!(result, None);
}

#[test]
fn test_column_sum_x2_function_on_i32_works() {
    let container: Vec<i32> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum_x2().expect("there shouldn't be overflow");
    assert_eq!(result, 192);
}

#[test]
fn test_column_sum_x2_function_i32_overflow_works() {
    let container: Vec<i32> = vec![i32::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum_x2();
    assert_eq!(result, None);
}

#[test]
fn test_column_sum_x2_function_on_i64_works() {
    let container: Vec<i64> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum_x2().expect("there shouldn't be overflow");
    assert_eq!(result, 192);
}

#[test]
fn test_column_sum_x2_function_i64_overflow_works() {
    let container: Vec<i64> = vec![i64::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum_x2();
    assert_eq!(result, None);
}

#[test]
fn test_column_sum_x2_function_on_i128_works() {
    let container: Vec<i128> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum_x2().expect("there shouldn't be overflow");
    assert_eq!(result, 192);
}

#[test]
fn test_column_sum_x2_function_i128_overflow_works() {
    let container: Vec<i128> = vec![i128::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum_x2();
    assert_eq!(result, None);
}

#[test]
fn test_column_sum_x2_function_on_isize_works() {
    let container: Vec<isize> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum_x2().expect("there shouldn't be overflow");
    assert_eq!(result, 192);
}

#[test]
fn test_column_sum_x2_function_isize_overflow_works() {
    let container: Vec<isize> = vec![isize::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum_x2();
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
fn test_column_variance_function_on_f32_works() {
    let container: Vec<f32> = vec![1.0, 4.5, 6.2, 2.4, 8.7, 5.5, 2.3, 4.2, 1.9];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.variance().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 6.034444444444446, 1e-5);
}

#[test]
fn test_column_variance_function_on_f64_works() {
    let container: Vec<f64> = vec![1.0, 4.5, 6.2, 2.4, 8.7, 5.5, 2.3, 4.2, 1.9];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.variance().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 6.034444444444446, 1e-5);
}

#[test]
fn test_column_variance_function_on_u8_works() {
    let container: Vec<u8> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.variance().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 5.749999999999999, 1e-6);
}

#[test]
fn test_column_variance_function_u8_overflow_works() {
    let container: Vec<u8> = vec![u8::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.variance();
    assert_eq!(result, None);
}

#[test]
fn test_column_variance_function_on_u16_works() {
    let container: Vec<u16> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.variance().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 5.749999999999999, 1e-6);
}

#[test]
fn test_column_variance_function_u16_overflow_works() {
    let container: Vec<u16> = vec![u16::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.variance();
    assert_eq!(result, None);
}

#[test]
fn test_column_variance_function_on_u32_works() {
    let container: Vec<u32> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.variance().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 5.749999999999999, 1e-6);
}

#[test]
fn test_column_variance_function_u32_overflow_works() {
    let container: Vec<u32> = vec![u32::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.variance();
    assert_eq!(result, None);
}
#[test]
fn test_column_variance_function_on_u64_works() {
    let container: Vec<u64> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.variance().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 5.749999999999999, 1e-6);
}

#[test]
fn test_column_variance_function_u64_overflow_works() {
    let container: Vec<u64> = vec![u64::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.variance();
    assert_eq!(result, None);
}

#[test]
fn test_column_variance_function_on_usize_works() {
    let container: Vec<usize> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.variance().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 5.749999999999999, 1e-6);
}

#[test]
fn test_column_variance_function_usize_overflow_works() {
    let container: Vec<usize> = vec![usize::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.variance();
    assert_eq!(result, None);
}

#[test]
fn test_column_variance_function_on_u128_works() {
    let container: Vec<u128> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.variance().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 5.749999999999999, 1e-6);
}

#[test]
fn test_column_variance_function_u128_overflow_works() {
    let container: Vec<u128> = vec![u128::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.variance();
    assert_eq!(result, None);
}

#[test]
fn test_column_variance_function_on_i8_works() {
    let container: Vec<i8> = vec![1, -4, 6, 1, 1, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.variance().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 10.4, 1e-10);
}

#[test]
fn test_column_variance_function_i8_overflow_works() {
    let container: Vec<i8> = vec![i8::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.variance();
    assert_eq!(result, None);
}

#[test]
fn test_column_variance_function_on_i16_works() {
    let container: Vec<i16> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.variance().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 13.822222222222223, 1e-7);
}

#[test]
fn test_column_variance_function_i16_overflow_works() {
    let container: Vec<i16> = vec![i16::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.variance();
    assert_eq!(result, None);
}

#[test]
fn test_column_variance_function_on_i32_works() {
    let container: Vec<i32> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.variance().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 13.822222222222223, 1e-7);
}

#[test]
fn test_column_variance_function_i32_overflow_works() {
    let container: Vec<i32> = vec![i32::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.variance();
    assert_eq!(result, None);
}

#[test]
fn test_column_variance_function_on_i64_works() {
    let container: Vec<i64> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.variance().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 13.822222222222223, 1e-7);
}

#[test]
fn test_column_variance_function_i64_overflow_works() {
    let container: Vec<i64> = vec![i64::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.variance();
    assert_eq!(result, None);
}

#[test]
fn test_column_variance_function_on_i128_works() {
    let container: Vec<i128> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.variance().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 13.822222222222223, 1e-7);
}

#[test]
fn test_column_variance_function_i128_overflow_works() {
    let container: Vec<i128> = vec![i128::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.variance();
    assert_eq!(result, None);
}

#[test]
fn test_column_variance_function_on_isize_works() {
    let container: Vec<isize> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.variance().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 13.822222222222223, 1e-7);
}

#[test]
fn test_column_variance_function_isize_overflow_works() {
    let container: Vec<isize> = vec![isize::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.variance();
    assert_eq!(result, None);
}

#[test]
fn test_column_stddev_function_on_f32_works() {
    let container: Vec<f32> = vec![1.0, 4.5, 6.2, 2.4, 8.7, 5.5, 2.3, 4.2, 1.9];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.stddev().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 2.4565106237190277, 1e-5);
}

#[test]
fn test_column_stddev_function_on_f64_works() {
    let container: Vec<f64> = vec![1.0, 4.5, 6.2, 2.4, 8.7, 5.5, 2.3, 4.2, 1.9];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.stddev().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 2.4565106237190277, 1e-5);
}

#[test]
fn test_column_stddev_function_on_u8_works() {
    let container: Vec<u8> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.stddev().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 2.3979157616563596, 1e-6);
}

#[test]
fn test_column_stddev_function_u8_overflow_works() {
    let container: Vec<u8> = vec![u8::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.stddev();
    assert_eq!(result, None);
}

#[test]
fn test_column_stddev_function_on_u16_works() {
    let container: Vec<u16> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.stddev().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 2.3979157616563596, 1e-6);
}

#[test]
fn test_column_stddev_function_u16_overflow_works() {
    let container: Vec<u16> = vec![u16::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.stddev();
    assert_eq!(result, None);
}

#[test]
fn test_column_stddev_function_on_u32_works() {
    let container: Vec<u32> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.stddev().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 2.3979157616563596, 1e-6);
}

#[test]
fn test_column_stddev_function_u32_overflow_works() {
    let container: Vec<u32> = vec![u32::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.stddev();
    assert_eq!(result, None);
}
#[test]
fn test_column_stddev_function_on_u64_works() {
    let container: Vec<u64> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.stddev().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 2.3979157616563596, 1e-6);
}

#[test]
fn test_column_stddev_function_u64_overflow_works() {
    let container: Vec<u64> = vec![u64::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.stddev();
    assert_eq!(result, None);
}

#[test]
fn test_column_stddev_function_on_usize_works() {
    let container: Vec<usize> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.stddev().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 2.3979157616563596, 1e-6);
}

#[test]
fn test_column_stddev_function_usize_overflow_works() {
    let container: Vec<usize> = vec![usize::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.stddev();
    assert_eq!(result, None);
}

#[test]
fn test_column_stddev_function_on_u128_works() {
    let container: Vec<u128> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.stddev().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 2.3979157616563596, 1e-6);
}

#[test]
fn test_column_stddev_function_u128_overflow_works() {
    let container: Vec<u128> = vec![u128::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.stddev();
    assert_eq!(result, None);
}

#[test]
fn test_column_stddev_function_on_i8_works() {
    let container: Vec<i8> = vec![1, -4, 6, 1, 1, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.stddev().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 3.22490309931942, 1e-10);
}

#[test]
fn test_column_stddev_function_i8_overflow_works() {
    let container: Vec<i8> = vec![i8::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.stddev();
    assert_eq!(result, None);
}

#[test]
fn test_column_stddev_function_on_i16_works() {
    let container: Vec<i16> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.stddev().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 3.7178249316263163, 1e-7);
}

#[test]
fn test_column_stddev_function_i16_overflow_works() {
    let container: Vec<i16> = vec![i16::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.stddev();
    assert_eq!(result, None);
}

#[test]
fn test_column_stddev_function_on_i32_works() {
    let container: Vec<i32> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.stddev().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 3.7178249316263163, 1e-7);
}

#[test]
fn test_column_stddev_function_i32_overflow_works() {
    let container: Vec<i32> = vec![i32::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.stddev();
    assert_eq!(result, None);
}

#[test]
fn test_column_stddev_function_on_i64_works() {
    let container: Vec<i64> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.stddev().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 3.7178249316263163, 1e-7);
}

#[test]
fn test_column_stddev_function_i64_overflow_works() {
    let container: Vec<i64> = vec![i64::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.stddev();
    assert_eq!(result, None);
}

#[test]
fn test_column_stddev_function_on_i128_works() {
    let container: Vec<i128> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.stddev().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 3.7178249316263163, 1e-7);
}

#[test]
fn test_column_stddev_function_i128_overflow_works() {
    let container: Vec<i128> = vec![i128::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.stddev();
    assert_eq!(result, None);
}

#[test]
fn test_column_stddev_function_on_isize_works() {
    let container: Vec<isize> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.stddev().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 3.7178249316263163, 1e-7);
}

#[test]
fn test_column_stddev_function_isize_overflow_works() {
    let container: Vec<isize> = vec![isize::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.stddev();
    assert_eq!(result, None);
}

#[test]
fn test_column_moment_i_function_on_f32_works() {
    let container: Vec<f32> = vec![1.0, 4.5, 6.2, 2.4, 8.7, 5.5, 2.3, 4.2, 1.9];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_i().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 4.0777777777777775, 1e-6);
}

#[test]
fn test_column_moment_i_function_on_f64_works() {
    let container: Vec<f64> = vec![1.0, 4.5, 6.2, 2.4, 8.7, 5.5, 2.3, 4.2, 1.9];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_i().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 4.0777777777777775, 1e-6);
}

#[test]
fn test_column_moment_i_function_on_u8_works() {
    let container: Vec<u8> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_i().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 3.6666666666666665, 1e-9);
}

#[test]
fn test_column_moment_i_function_u8_overflow_works() {
    let container: Vec<u8> = vec![u8::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_i();
    assert_eq!(result, None);
}

#[test]
fn test_column_moment_i_function_on_u16_works() {
    let container: Vec<u16> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_i().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 3.6666666666666665, 1e-9);
}

#[test]
fn test_column_moment_i_function_u16_overflow_works() {
    let container: Vec<u16> = vec![u16::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_i();
    assert_eq!(result, None);
}

#[test]
fn test_column_moment_i_function_on_u32_works() {
    let container: Vec<u32> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_i().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 3.6666666666666665, 1e-9);
}

#[test]
fn test_column_moment_i_function_u32_overflow_works() {
    let container: Vec<u32> = vec![u32::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_i();
    assert_eq!(result, None);
}
#[test]
fn test_column_moment_i_function_on_u64_works() {
    let container: Vec<u64> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_i().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 3.6666666666666665, 1e-9);
}

#[test]
fn test_column_moment_i_function_u64_overflow_works() {
    let container: Vec<u64> = vec![u64::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_i();
    assert_eq!(result, None);
}

#[test]
fn test_column_moment_i_function_on_usize_works() {
    let container: Vec<usize> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_i().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 3.6666666666666665, 1e-9);
}

#[test]
fn test_column_moment_i_function_usize_overflow_works() {
    let container: Vec<usize> = vec![usize::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_i();
    assert_eq!(result, None);
}

#[test]
fn test_column_moment_i_function_on_u128_works() {
    let container: Vec<u128> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_i().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 3.6666666666666665, 1e-9);
}

#[test]
fn test_column_moment_i_function_u128_overflow_works() {
    let container: Vec<u128> = vec![u128::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_i();
    assert_eq!(result, None);
}

#[test]
fn test_column_moment_i_function_on_i8_works() {
    let container: Vec<i8> = vec![1, -4, 6, 1, 1, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_i().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 1.8, 1e-10);
}

#[test]
fn test_column_moment_i_function_i8_overflow_works() {
    let container: Vec<i8> = vec![i8::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_i();
    assert_eq!(result, None);
}

#[test]
fn test_column_moment_i_function_on_i16_works() {
    let container: Vec<i16> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_i().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 2.6, 1e-7);
}

#[test]
fn test_column_moment_i_function_i16_overflow_works() {
    let container: Vec<i16> = vec![i16::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_i();
    assert_eq!(result, None);
}

#[test]
fn test_column_moment_i_function_on_i32_works() {
    let container: Vec<i32> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_i().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 2.6, 1e-7);
}

#[test]
fn test_column_moment_i_function_i32_overflow_works() {
    let container: Vec<i32> = vec![i32::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_i();
    assert_eq!(result, None);
}

#[test]
fn test_column_moment_i_function_on_i64_works() {
    let container: Vec<i64> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_i().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 2.6, 1e-7);
}

#[test]
fn test_column_moment_i_function_i64_overflow_works() {
    let container: Vec<i64> = vec![i64::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_i();
    assert_eq!(result, None);
}

#[test]
fn test_column_moment_i_function_on_i128_works() {
    let container: Vec<i128> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_i().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 2.6, 1e-7);
}

#[test]
fn test_column_moment_i_function_i128_overflow_works() {
    let container: Vec<i128> = vec![i128::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_i();
    assert_eq!(result, None);
}

#[test]
fn test_column_moment_i_function_on_isize_works() {
    let container: Vec<isize> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_i().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 2.6, 1e-7);
}

#[test]
fn test_column_moment_i_function_isize_overflow_works() {
    let container: Vec<isize> = vec![isize::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_i();
    assert_eq!(result, None);
}

#[test]
fn test_column_moment_ii_function_on_f32_works() {
    let container: Vec<f32> = vec![1.0, 4.5, 6.2, 2.4, 8.7, 5.5, 2.3, 4.2, 1.9];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_ii().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 21.992222222222225, 1e-5);
}

#[test]
fn test_column_moment_ii_function_on_f64_works() {
    let container: Vec<f64> = vec![1.0, 4.5, 6.2, 2.4, 8.7, 5.5, 2.3, 4.2, 1.9];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_ii().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 21.992222222222225, 1e-5);
}

#[test]
fn test_column_moment_ii_function_on_u8_works() {
    let container: Vec<u8> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_ii().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 18.555555555555557, 1e-9);
}

#[test]
fn test_column_moment_ii_function_u8_overflow_works() {
    let container: Vec<u8> = vec![u8::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_ii();
    assert_eq!(result, None);
}

#[test]
fn test_column_moment_ii_function_on_u16_works() {
    let container: Vec<u16> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_ii().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 18.555555555555557, 1e-9);
}

#[test]
fn test_column_moment_ii_function_u16_overflow_works() {
    let container: Vec<u16> = vec![u16::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_ii();
    assert_eq!(result, None);
}

#[test]
fn test_column_moment_ii_function_on_u32_works() {
    let container: Vec<u32> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_ii().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 18.555555555555557, 1e-9);
}

#[test]
fn test_column_moment_ii_function_u32_overflow_works() {
    let container: Vec<u32> = vec![u32::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_ii();
    assert_eq!(result, None);
}
#[test]
fn test_column_moment_ii_function_on_u64_works() {
    let container: Vec<u64> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_ii().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 18.555555555555557, 1e-9);
}

#[test]
fn test_column_moment_ii_function_u64_overflow_works() {
    let container: Vec<u64> = vec![u64::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_ii();
    assert_eq!(result, None);
}

#[test]
fn test_column_moment_ii_function_on_usize_works() {
    let container: Vec<usize> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_ii().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 18.555555555555557, 1e-9);
}

#[test]
fn test_column_moment_ii_function_usize_overflow_works() {
    let container: Vec<usize> = vec![usize::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_ii();
    assert_eq!(result, None);
}

#[test]
fn test_column_moment_ii_function_on_u128_works() {
    let container: Vec<u128> = vec![1, 4, 6, 2, 8, 5, 2, 4, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_ii().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 18.555555555555557, 1e-9);
}

#[test]
fn test_column_moment_ii_function_u128_overflow_works() {
    let container: Vec<u128> = vec![u128::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_ii();
    assert_eq!(result, None);
}

#[test]
fn test_column_moment_ii_function_on_i8_works() {
    let container: Vec<i8> = vec![1, -4, 6, 1, 1, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_ii().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 12.6, 1e-10);
}

#[test]
fn test_column_moment_ii_function_i8_overflow_works() {
    let container: Vec<i8> = vec![i8::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_ii();
    assert_eq!(result, None);
}

#[test]
fn test_column_moment_ii_function_on_i16_works() {
    let container: Vec<i16> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_ii().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 19.2, 1e-7);
}

#[test]
fn test_column_moment_ii_function_i16_overflow_works() {
    let container: Vec<i16> = vec![i16::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_ii();
    assert_eq!(result, None);
}

#[test]
fn test_column_moment_ii_function_on_i32_works() {
    let container: Vec<i32> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_ii().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 19.2, 1e-7);
}

#[test]
fn test_column_moment_ii_function_i32_overflow_works() {
    let container: Vec<i32> = vec![i32::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_ii();
    assert_eq!(result, None);
}

#[test]
fn test_column_moment_ii_function_on_i64_works() {
    let container: Vec<i64> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_ii().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 19.2, 1e-7);
}

#[test]
fn test_column_moment_ii_function_i64_overflow_works() {
    let container: Vec<i64> = vec![i64::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_ii();
    assert_eq!(result, None);
}

#[test]
fn test_column_moment_ii_function_on_i128_works() {
    let container: Vec<i128> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_ii().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 19.2, 1e-7);
}

#[test]
fn test_column_moment_ii_function_i128_overflow_works() {
    let container: Vec<i128> = vec![i128::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_ii();
    assert_eq!(result, None);
}

#[test]
fn test_column_moment_ii_function_on_isize_works() {
    let container: Vec<isize> = vec![1, -4, 6, 2, 8, 5, -2, 4, 1, 5];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_ii().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 19.2, 1e-7);
}

#[test]
fn test_column_moment_ii_function_isize_overflow_works() {
    let container: Vec<isize> = vec![isize::MAX, 1];
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.moment_ii();
    assert_eq!(result, None);
}

#[test]
fn load_test_column_sum_on_1_m_row_data_works() {
    let container: Vec<f64> = common::load_data("data_1M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum().expect("there shouldn't be overflow");
    assert_approx_eq!(result, -916668927.4241314, 1.0e-3);
}

#[test]
fn load_test_column_sum_on_3_m_row_data_works() {
    let container: Vec<f64> = common::load_data("data_3M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 2415672220.437594, 1.0e-3);
}

#[test]
fn load_test_column_sum_x2_on_1_m_row_data_works() {
    let container: Vec<f64> = common::load_data("data_1M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum_x2().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 3.329285644276438e+17, 1.0e3);
}

#[test]
fn load_test_column_sum_x2_on_3_m_row_data_works() {
    let container: Vec<f64> = common::load_data("data_3M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.sum_x2().expect("there shouldn't be overflow");
    assert_approx_eq!(result, 8.996801005832153e+18, 1.0e6);
}

#[test]
fn load_test_column_variance_on_1_m_row_data_works() {
    let container: Vec<f64> = common::load_data("data_1M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .variance()
        .expect("all the input data should be comparable");
    assert_approx_eq!(result, 332928057073.7754, 1.0e-1);
}

#[test]
fn load_test_column_variance_on_3_m_row_data_works() {
    let container: Vec<f64> = common::load_data("data_3M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .variance()
        .expect("all the input data should be comparable");
    assert_approx_eq!(result, 2998934019869.4873, 1.0e-1);
}

#[test]
fn load_test_column_stddev_on_1_m_row_data_works() {
    let container: Vec<f64> = common::load_data("data_1M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .stddev()
        .expect("all the input data should be comparable");
    assert_approx_eq!(result, 576999.1829056462, 1.0e-1);
}

#[test]
fn load_test_column_stddev_on_3_m_row_data_works() {
    let container: Vec<f64> = common::load_data("data_3M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .stddev()
        .expect("all the input data should be comparable");
    assert_approx_eq!(result, 1731743.058270926, 1.0e-1);
}

#[test]
fn load_test_column_moment_i_on_1_m_row_data_works() {
    let container: Vec<f64> = common::load_data("data_1M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .moment_i()
        .expect("all the input data should be comparable");
    assert_approx_eq!(result, -916.6689274241082, 1.0e-3);
}

#[test]
fn load_test_column_moment_i_on_3_m_row_data_works() {
    let container: Vec<f64> = common::load_data("data_3M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .moment_i()
        .expect("all the input data should be comparable");
    assert_approx_eq!(result, 805.2240734792023, 1.0e-3);
}

#[test]
fn load_test_column_moment_ii_on_1_m_row_data_works() {
    let container: Vec<f64> = common::load_data("data_1M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .moment_ii()
        .expect("all the input data should be comparable");
    assert_approx_eq!(result, 332928564427.6435, 1.0e-1);
}

#[test]
fn load_test_column_moment_ii_on_3_m_row_data_works() {
    let container: Vec<f64> = common::load_data("data_3M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .moment_ii()
        .expect("all the input data should be comparable");
    assert_approx_eq!(result, 2998933668610.7383, 1.0e-1);
}

#[test]
fn load_test_column_min_on_1_m_row_data_works() {
    let container: Vec<f64> = common::load_data("data_1M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .min()
        .expect("all the input data should be comparable");
    assert_approx_eq!(result, -999996.8386372058, 1.0e-10);
}

#[test]
fn load_test_column_min_on_3_m_row_data_works() {
    let container: Vec<f64> = common::load_data("data_3M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .min()
        .expect("all the input data should be comparable");
    assert_approx_eq!(result, -2999998.2130748853, 1.0e-10);
}

#[test]
fn load_test_column_max_on_1_m_row_data_works() {
    let container: Vec<f64> = common::load_data("data_1M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .max()
        .expect("all the input data should be comparable");
    assert_approx_eq!(result, 999998.7204128127, 1.0e-10);
}

#[test]
fn load_test_column_max_on_3_m_row_data_works() {
    let container: Vec<f64> = common::load_data("data_3M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let result = column
        .max()
        .expect("all the input data should be comparable");
    assert_approx_eq!(result, 2999996.6583054466, 1.0e-10);
}

#[test]
fn load_test_column_count_on_1_m_row_data_works() {
    let container: Vec<f64> = common::load_data("data_1M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.count().expect("count should always work");
    assert_eq!(result, 1000000);
}

#[test]
fn load_test_column_count_on_3_m_row_data_works() {
    let container: Vec<f64> = common::load_data("data_3M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let result = column.count().expect("count should always work");
    assert_eq!(result, 3000000);
}
