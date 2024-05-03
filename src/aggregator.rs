use crate::tools;
use crate::ColumnData;
use std::marker::Send;
use std::ops::Add;
use std::sync::mpsc;
use std::thread;

pub fn sum<
    T: for<'a> Add<&'a T, Output = T>
        + Default
        + std::fmt::Display
        + std::marker::Sync
        + std::marker::Send
        + 'static,
>(
    data: &ColumnData<T>,
) -> T {
    let (tx, rx) = mpsc::channel();
    let ranges = tools::calculate_ranges(data.data().len(), 4);
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
