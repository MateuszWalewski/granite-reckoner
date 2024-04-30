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


pub struct Column<T: Numeric> {
    data: Vec<T>
}

impl<T: Numeric> Column<T> {
    pub fn new(container: Vec<T>) -> Column<T> {
       Column{ 
           data: container,
       }
    }

    pub fn sum(&self) -> &T {
        &self.data[1]
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
