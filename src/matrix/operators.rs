use crate::matrix::Matrix;
use num_traits::{FromPrimitive, Num};
use std::ops::{Index, IndexMut};

impl<T: FromPrimitive + Num> Matrix<T> {
    pub fn dim(&self) -> (usize, usize) {
        (self.rows, self.columns)
    }

    pub fn get(&self, row: usize, column: usize) -> Result<T, String>
    where
        T: Copy,
    {
        let dim = self.dim();

        if row >= dim.0 || column >= dim.1 {
            return Err(String::from("Index out of bounds"));
        }

        Ok(self.matrix[row][column])
    }

    pub fn update(&mut self, row: usize, column: usize, value: T) -> Result<(), String> {
        let dim = self.dim();

        if row >= dim.0 || column >= dim.1 {
            return Err(String::from("Index out of bounds"));
        }

        self.matrix[row][column] = value;

        Ok(())
    }
}

impl<T: FromPrimitive + Num> Index<usize> for Matrix<T> {
    type Output = Vec<T>;

    fn index(&self, row: usize) -> &Self::Output {
        &self.matrix[row]
    }
}

impl<T: FromPrimitive + Num> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.matrix[row]
    }
}

impl<T: FromPrimitive + Num> IntoIterator for Matrix<T> {
    type Item = Vec<T>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.matrix.into_iter()
    }
}
