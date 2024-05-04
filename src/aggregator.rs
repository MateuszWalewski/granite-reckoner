use crate::{constants, tools, ColumnData, NumericType};
use std::{
    sync::{mpsc, Arc},
    thread,
};

pub fn sum<T: NumericType<T>>(data: &ColumnData<T>) -> T {
    let rx = engine(data, sum_func);
    rx.iter().fold(T::default(), |acc, e| acc + &e)
}

pub fn min<T: NumericType<T>>(data: &ColumnData<T>) -> T {
    let rx = engine(data, min_func);
    rx.iter()
        .fold(None, |min, x| match min {
            None => Some(x),
            Some(curr_min) => Some(tools::partial_min(curr_min, x).unwrap()),
        })
        .expect("Should be any value")
}

pub fn max<T: NumericType<T>>(data: &ColumnData<T>) -> T {
    let rx = engine(data, max_func);
    rx.iter()
        .fold(None, |max, x| match max {
            None => Some(x),
            Some(curr_max) => Some(tools::partial_max(curr_max, x).unwrap()),
        })
        .expect("Should be any value")
}

fn engine<T: NumericType<T>, F>(data: &ColumnData<T>, func: F) -> mpsc::Receiver<T>
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
    rx
}

pub fn sum_func<T: NumericType<T>>(data_safe: Arc<Vec<T>>, begin: usize, end: usize) -> T {
    data_safe[begin..end]
        .iter()
        .fold(T::default(), |acc, e| acc + e)
}

pub fn min_func<T: NumericType<T>>(data_safe: Arc<Vec<T>>, begin: usize, end: usize) -> T {
    *data_safe[begin..end]
        .iter()
        .fold(None, |min, x| match min {
            None => Some(x),
            Some(curr_min) => Some(tools::partial_min(curr_min, x).unwrap()),
        })
        .expect("Should be any value")
}

pub fn max_func<T: NumericType<T>>(data_safe: Arc<Vec<T>>, begin: usize, end: usize) -> T {
    *data_safe[begin..end]
        .iter()
        .fold(None, |max, x| match max {
            None => Some(x),
            Some(curr_max) => Some(tools::partial_max(curr_max, x).unwrap()),
        })
        .expect("Should be any value")
}
