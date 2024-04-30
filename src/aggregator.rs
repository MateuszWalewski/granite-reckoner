use crate::ColumnData;

pub fn run<T: for<'a> std::ops::Add<&'a T, Output = T> + Default>(
    data: &ColumnData<T>,
    operation: &str,
) -> T {
    match operation {
        "Sum" => sum(data),
        _ => sum(data),
    }
}

fn sum<T: for<'a> std::ops::Add<&'a T, Output = T> + Default>(data: &ColumnData<T>) -> T {
    data.data().iter().fold(T::default(), |acc, e| acc + e)
}
