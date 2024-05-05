use granite_reckoner::Column;

fn main() {
    let container: Vec<i8> = vec![127, 1, 0, 0];
    let column = Column::new();
    let column = column.add_data(container);

    column.print();
    let result = column.sum();
    println!("result = {:?}", result);
    let result = column.count();
    println!("result = {:?}", result);
    let result = column.min();
    println!("result = {:?}", result);
    let result = column.max();
    println!("result = {:?}", result);
}
