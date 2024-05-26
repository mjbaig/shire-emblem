use core::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct FalseMatrix {
    data: Vec<i32>,
    pub rows: usize,
    pub cols: usize,
}

impl FalseMatrix {
    pub fn new(data: Vec<i32>, rows: usize, cols: usize) -> Self {
        Self { data, rows, cols }
    }
}

impl Index<(usize, usize)> for FalseMatrix {
    type Output = i32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        let index: usize = row * self.cols + col;
        if index > self.data.len() {
            panic!("{} is out of bounds", index);
        }
        &self.data[row * self.cols + col]
    }
}

impl IndexMut<(usize, usize)> for FalseMatrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, col) = index;
        let index: usize = row + col * self.cols;
        if index > self.data.len() {
            panic!("{} is out of bounds", index);
        }
        &mut self.data[row * self.cols + col]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_matrix_from_vec() {
        let input_data = vec![1, 2, 3, 4];

        let under_test = FalseMatrix::new(input_data, 2, 2);

        assert_eq!(under_test[(0, 0)], 1);
        assert_eq!(under_test[(0, 1)], 2);
        assert_eq!(under_test[(1, 0)], 3);
        assert_eq!(under_test[(1, 1)], 4);
    }
}
