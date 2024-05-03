use crate::{constants, tools, ColumnData, NumericType};
use std::{
    sync::{mpsc, Arc},
    thread,
};

pub fn sum<T: NumericType<T>>(data: &ColumnData<T>) -> T {
    engine(data, sum_func)
}

fn engine<T: NumericType<T>, F>(data: &ColumnData<T>, func: F) -> T
where
    F: Fn(Arc<Vec<T>>, usize, usize) -> T + Send + 'static + Copy,
{
    let (tx, rx) = mpsc::channel();
    let ranges = tools::calculate_ranges(data.data().len(), constants::NUMBER_OF_NODES);
    let data_threaded_ref = data.data();
    for range in ranges {
        let data_safe = data_threaded_ref.clone();
        let tx1 = tx.clone();
        thread::spawn(move || {
            let result = func(data_safe, range.0, range.1);
            tx1.send(result).unwrap();
        });
    }
    drop(tx);
    rx.iter().fold(T::default(), |acc, e| acc + &e)
}

pub fn count<T>(data: &ColumnData<T>) -> usize {
    data.data().iter().count()
}
pub fn sum_func<T: NumericType<T>>(data_safe: Arc<Vec<T>>, begin: usize, end: usize) -> T {
    data_safe[begin..end]
        .iter()
        .fold(T::default(), |acc, e| acc + e)
}
