use crate::{constants, tools, ColumnData, NumericType};
use std::{
    sync::{mpsc, Arc},
    thread,
};

pub fn sum<T: NumericType<T>>(data: &ColumnData<T>) -> Option<T> {
    let received = engine(data, sum_on_node);
    gather_from_nodes(received, Some(T::default()), |acc, x| acc?.checked_add(x?))
}

pub fn min<T: NumericType<T>>(data: &ColumnData<T>) -> Option<T> {
    let received = engine(data, min_on_node);
    gather_from_nodes(received, Some(T::MAX), |min, x| match min {
        None => None,
        Some(curr_min) => tools::partial_min(curr_min, x?),
    })
}

pub fn max<T: NumericType<T>>(data: &ColumnData<T>) -> Option<T> {
    let received = engine(data, max_on_node);
    gather_from_nodes(received, Some(T::MIN), |max, x| match max {
        None => None,
        Some(curr_max) => tools::partial_max(curr_max, x?),
    })
}

fn gather_from_nodes<T: NumericType<T>, F>(
    receiver: mpsc::Receiver<Option<T>>,
    init: Option<T>,
    func: F,
) -> Option<T>
where
    F: FnOnce(Option<T>, Option<T>) -> Option<T>,
    F: Send + Copy + 'static,
{
    receiver.iter().fold(init, |max, x| func(max, x))
}

fn engine<T: NumericType<T>, F>(data: &ColumnData<T>, func: F) -> mpsc::Receiver<Option<T>>
where
    F: Fn(Arc<Vec<T>>, usize, usize) -> Option<T> + Send + 'static + Copy,
{
    let (tx, rx) = mpsc::channel();
    let ranges = tools::calculate_ranges(data.data().len(), constants::NUMBER_OF_NODES);
    let data_threaded_ref = data.data();
    let mut threads = Vec::<thread::JoinHandle<()>>::with_capacity(constants::NUMBER_OF_NODES);
    for range in ranges {
        let data_safe = data_threaded_ref.clone();
        let tx1 = tx.clone();
        let th = thread::spawn(move || {
            let result = func(data_safe, range.0, range.1);
            tx1.send(result).unwrap();
        });
        threads.push(th);
    }
    for thread in threads {
        thread.join().unwrap()
    }
    drop(tx);
    rx
}

pub fn sum_on_node<T: NumericType<T>>(
    data_safe: Arc<Vec<T>>,
    begin: usize,
    end: usize,
) -> Option<T> {
    data_safe[begin..end]
        .iter()
        .fold(Some(T::default()), |acc, &x| acc?.checked_add(x))
}

pub fn min_on_node<T: NumericType<T>>(
    data_safe: Arc<Vec<T>>,
    begin: usize,
    end: usize,
) -> Option<T> {
    data_safe[begin..end]
        .iter()
        .fold(Some(T::MAX), |min, &x| match min {
            None => None,
            Some(curr_min) => tools::partial_min(curr_min, x),
        })
}

pub fn max_on_node<T: NumericType<T>>(
    data_safe: Arc<Vec<T>>,
    begin: usize,
    end: usize,
) -> Option<T> {
    data_safe[begin..end]
        .iter()
        .fold(Some(T::MIN), |max, &x| match max {
            None => None,
            Some(curr_max) => tools::partial_max(curr_max, x),
        })
}
