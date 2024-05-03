use granite_reckoner::Column;

fn main() {
    let container = vec![1.0, 4.5, 6.2, 2.4, 8.7, 5.5, 2.3, 4.2, 1.9];
    let column = Column::new();
    let column = column.add_data(container);

    column.print();
    let result = column.sum();
    println!("result = {}", result);
    let result2 = column.count();
    println!("result2 = {}", result2);
}
