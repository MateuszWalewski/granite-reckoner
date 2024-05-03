use std::{
    cmp,
    fmt::{self, Debug, Formatter},
};

use crate::{Column, NumericType};

pub fn calculate_ranges(data_size: usize, number_of_nodes: usize) -> Vec<(usize, usize)> {
    let mut ranges: Vec<(usize, usize)> = vec![];
    let length: usize = data_size / number_of_nodes;
    let mut remain: usize = data_size % number_of_nodes;

    let mut begin: usize = 0;
    let mut end: usize = 0;

    for _ in 0..cmp::min(data_size, number_of_nodes) {
        end += if remain > 0 {
            length + (remain > 0) as usize
        } else {
            length
        };
        if remain > 0 {
            remain -= 1
        };
        ranges.push((begin, end));
        begin = end;
    }

    ranges
}

impl<T: NumericType<T>> Debug for Column<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let inner_data = self.data.data();
        for idx in 0..inner_data.len() - 1 {
            write!(f, "{}\n", inner_data[idx])?;
        }
        write!(f, "{}", inner_data[inner_data.len() - 1])?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ranges_1_works() {
        let ranges = calculate_ranges(10, 4);
        assert_eq!(ranges[0], (0, 3));
        assert_eq!(ranges[1], (3, 6));
        assert_eq!(ranges[2], (6, 8));
        assert_eq!(ranges[3], (8, 10));
    }
    #[test]
    fn test_ranges_2_works() {
        let ranges = calculate_ranges(30, 5);
        assert_eq!(ranges[0], (0, 6));
        assert_eq!(ranges[1], (6, 12));
        assert_eq!(ranges[2], (12, 18));
        assert_eq!(ranges[3], (18, 24));
        assert_eq!(ranges[4], (24, 30));
    }
    #[test]
    fn test_ranges_3_works() {
        let ranges = calculate_ranges(11, 4);
        assert_eq!(ranges[0], (0, 3));
        assert_eq!(ranges[1], (3, 6));
        assert_eq!(ranges[2], (6, 9));
        assert_eq!(ranges[3], (9, 11));
    }
    #[test]
    fn test_ranges_4_works() {
        let ranges = calculate_ranges(3, 4);
        assert_eq!(ranges[0], (0, 1));
        assert_eq!(ranges[1], (1, 2));
        assert_eq!(ranges[2], (2, 3));
    }
    #[test]
    fn test_ranges_5_works() {
        let ranges = calculate_ranges(5, 3);
        assert_eq!(ranges[0], (0, 2));
        assert_eq!(ranges[1], (2, 4));
        assert_eq!(ranges[2], (4, 5));
    }
}
