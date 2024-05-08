use crate::traits::{CheckedDiv, CheckedMul, CheckedSub};
use crate::{tools, ColumnData, NumericType};
use std::{
    sync::{mpsc, Arc},
    thread,
};

pub fn sum<T: NumericType<T>>(data: &ColumnData<T>, number_of_threads: usize) -> Option<T> {
    let received = engine(data, sum_on_node, number_of_threads);
    gather_from_nodes(received, Some(T::default()), |acc, x| acc?.checked_add(x?))
}

pub fn sum_x2<T: NumericType<T>>(data: &ColumnData<T>, number_of_threads: usize) -> Option<T> {
    let received = engine(data, sum_x2_on_node, number_of_threads);
    gather_from_nodes(received, Some(T::default()), |acc, x| acc?.checked_add(x?))
}

pub fn count<T: NumericType<T>>(data: &ColumnData<T>) -> Option<usize> {
    Some(data.data().len())
}

pub fn moment_i<T: NumericType<T>>(data: &ColumnData<T>, number_of_threads: usize) -> Option<f64> {
    let received = engine(data, sum_on_node, number_of_threads);
    let sum = gather_from_nodes(received, Some(T::default()), |acc, x| acc?.checked_add(x?));
    let count_f: f64 = count(data)? as f64;
    let sum_f: f64 = T::to_f64(sum?);
    sum_f.checked_div(count_f)
}

pub fn moment_ii<T: NumericType<T>>(data: &ColumnData<T>, number_of_threads: usize) -> Option<f64> {
    let received = engine(data, sum_x2_on_node, number_of_threads);
    let sum_x2 = gather_from_nodes(received, Some(T::default()), |acc, x| acc?.checked_add(x?));
    let count_f: f64 = count(data)? as f64;
    let sum_x2f: f64 = T::to_f64(sum_x2?);
    sum_x2f.checked_div(count_f)
}

pub fn variance<T: NumericType<T>>(data: &ColumnData<T>, number_of_threads: usize) -> Option<f64> {
    let count_f: f64 = count(data)? as f64;

    let moment_i: f64 = moment_i(data, number_of_threads)?;
    let moment_ii: f64 = moment_ii(data, number_of_threads)?;
    let moment_i2 = moment_i.checked_mul(moment_i)?;

    let first_factor = moment_ii.checked_sub(moment_i2)?;
    let second_factor = count_f.checked_div(count_f.checked_sub(1.0 as f64)?)?;

    first_factor.checked_mul(second_factor)
}

pub fn stddev<T: NumericType<T>>(data: &ColumnData<T>, number_of_threads: usize) -> Option<f64> {
    let variance = variance(data, number_of_threads);
    Some(variance?.sqrt())
}

pub fn min<T: NumericType<T>>(data: &ColumnData<T>, number_of_threads: usize) -> Option<T> {
    let received = engine(data, min_on_node, number_of_threads);
    gather_from_nodes(received, Some(T::MAX), |min, x| match min {
        None => None,
        Some(curr_min) => tools::partial_min(curr_min, x?),
    })
}

pub fn max<T: NumericType<T>>(data: &ColumnData<T>, number_of_threads: usize) -> Option<T> {
    let received = engine(data, max_on_node, number_of_threads);
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

fn engine<T: NumericType<T>, F>(
    data: &ColumnData<T>,
    func: F,
    number_of_threads: usize,
) -> mpsc::Receiver<Option<T>>
where
    F: Fn(Arc<Vec<T>>, usize, usize) -> Option<T> + Send + 'static + Copy,
{
    let (tx, rx) = mpsc::channel();
    let ranges = tools::calculate_ranges(data.data().len(), number_of_threads);
    let data_threaded_ref = data.data();
    let mut threads = Vec::<thread::JoinHandle<()>>::with_capacity(number_of_threads);
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

pub fn sum_x2_on_node<T: NumericType<T>>(
    data_safe: Arc<Vec<T>>,
    begin: usize,
    end: usize,
) -> Option<T> {
    data_safe[begin..end]
        .iter()
        .fold(Some(T::default()), |acc, &x| {
            acc?.checked_add(x.checked_mul(x)?)
        })
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
