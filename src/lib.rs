use std::fmt;
use std::marker::PhantomData;

pub trait Numeric {}

impl Numeric for i8 {}
impl Numeric for i16 {}
impl Numeric for i32 {}
impl Numeric for i64 {}
impl Numeric for u8 {}
impl Numeric for u16 {}
impl Numeric for u32 {}
impl Numeric for u64 {}
impl Numeric for f32 {}
impl Numeric for f64 {}

pub struct EmptyColumn<T: Numeric> {
    phantom: PhantomData<T>,
}

pub struct Column<T: Numeric> {
    data: Vec<T>,
}

impl<T: Numeric> EmptyColumn<T> {
    pub fn add_data(self, container: Vec<T>) -> Column<T> {
        Column { data: container }
    }
}

impl<T: Numeric + std::fmt::Display> fmt::Debug for Column<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for idx in 0..self.data.len() - 2 {
            write!(f, "{}\n", self.data[idx])?;
        }
        write!(f, "{}", self.data[self.data.len() - 1])?;

        Ok(())
    }
}

impl<T: Numeric + std::fmt::Debug + std::fmt::Display> Column<T> {
    pub fn new() -> EmptyColumn<T> {
        EmptyColumn::<T> {
            phantom: PhantomData,
        }
    }

    pub fn aggregate(&self) -> &T {
        &self.data[1]
    }

    pub fn print(&self) {
        println!("{:?}", &self);
    }
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn it_works() {
//        let result = add(2, 2);
//        assert_eq!(result, 4);
//    }
//}
