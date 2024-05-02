use crate::ColumnData;
use std::ops::Add;

pub fn sum<T: for<'a> Add<&'a T, Output = T> + Default>(data: &ColumnData<T>) -> T {
    data.data().iter().fold(T::default(), |acc, e| acc + e)
}

pub fn count<T>(data: &ColumnData<T>) -> usize {
    data.data().iter().count()
}
