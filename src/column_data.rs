pub struct ColumnData<T> {
    data: Vec<T>,
}

impl<T> ColumnData<T> {
    pub fn new(container: Vec<T>) -> ColumnData<T> {
        ColumnData { data: container }
    }
    pub fn data(&self) -> &Vec<T> {
        &self.data
    }
}
