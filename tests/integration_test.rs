use multithreaded_calcs::Column;

#[test]
fn test_column_sum_function_works() {
    let container = vec![1.0,4.5,6.2,2.4,8.7,5.5,2.3,4.2,1.9];
    let column = Column::new(container); 
    let result = column.sum();
    assert_eq!(result, &4.5);
}
