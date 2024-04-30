use multithreaded_calcs::Column;

fn main() {
    let container = vec![1.0,4.5,6.2,2.4,8.7,5.5,2.3,4.2,1.9];
    let column = Column::new(container); 

    let result = column.sum();

    println!("result = {}", result);

}
