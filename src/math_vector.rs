use std::ops::Index;

use crate::matrix::Matrix;

pub trait MathVector<T, const N: usize> {
    fn scalar(&self, scalar: isize) -> Self;
    fn scalar_mut(&mut self, scalar: isize);
    fn dot(&self, rhs: impl MathVector<T, N> + Index<usize, Output = T>) -> isize;
    fn add_vector(&self, rhs: impl MathVector<T, N> + Index<usize, Output = T>) -> Self;
    fn add_vector_mut(&mut self, rhs: impl MathVector<T, N> + Index<usize, Output = T>);
    fn sub_vector(&self, rhs: impl MathVector<T, N> + Index<usize, Output = T>) -> Self;
    fn sub_vector_mut(&mut self, rhs: impl MathVector<T, N> + Index<usize, Output = T>);
    fn entrywise(&self, rhs: impl MathVector<T, N> + Index<usize, Output = T>) -> Self;
    fn entrywise_mut(&mut self, rhs: impl MathVector<T, N> + Index<usize, Output = T>);
    fn cross(&self, rhs: impl MathVector<T, N> + Index<usize, Output = T>) -> Self;
    fn cross_mut(&mut self, rhs: impl MathVector<T, N> + Index<usize, Output = T>);

    fn tensor_prod<const M: usize>(
        &self,
        rhs: impl MathVector<T, N> + Index<usize, Output = T>,
    ) -> Matrix<T, M, N>;

    fn magnitude(&self) -> usize;
    fn sum(&self) -> isize;
}
