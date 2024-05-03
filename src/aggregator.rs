use crate::{constants, tools, ColumnData, NumericType};
use std::{sync::mpsc, thread};

pub fn sum<T: NumericType<T>>(data: &ColumnData<T>) -> T {
    let (tx, rx) = mpsc::channel();
    let ranges = tools::calculate_ranges(data.data().len(), constants::NUMBER_OF_NODES);
    let data_threaded_ref = data.data();
    for range in ranges {
        let data_safe = data_threaded_ref.clone();
        let tx1 = tx.clone();
        thread::spawn(move || {
            let result = data_safe[range.0..range.1]
                .iter()
                .fold(T::default(), |acc, e| acc + e);
            tx1.send(result).unwrap();
        });
    }
    drop(tx);
    rx.iter().fold(T::default(), |acc, e| acc + &e)
}

pub fn count<T>(data: &ColumnData<T>) -> usize {
    data.data().iter().count()
}
