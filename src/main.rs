use granite_reckoner::Column;

fn main() {
    let container = vec![1.0, 4.5, 16.2, -2.4, 8.7, 5.5, 2.3, 4.2, 1.9];
    let column = Column::new();
    let column = column.add_data(container);

    column.print();
    let result = column.sum();
    println!("result = {}", result);
    let result = column.count();
    println!("result = {}", result);
    let result = column.min();
    println!("result = {}", result);
    let result = column.max();
    println!("result = {}", result);
}
