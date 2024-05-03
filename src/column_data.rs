use std::sync::Arc;
pub struct ColumnData<T> {
    data: Arc<Vec<T>>,
}

impl<T> ColumnData<T> {
    pub fn new(container: Vec<T>) -> ColumnData<T> {
        ColumnData {
            data: Arc::new(container),
        }
    }
    pub fn data(&self) -> Arc<Vec<T>> {
        Arc::clone(&self.data)
    }
}
