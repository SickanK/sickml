pub mod into_2d_vector;
pub mod iterator;
use crate::vector::Vector;
use into_2d_vector::Into2dVector;

#[derive(Debug)]
pub struct Matrix<T, const N: usize, const M: usize> {
    inner: [Vector<T, M>; N],
    transposed: Option<[Vector<T, M>; N]>,
}

impl<T, const N: usize, const M: usize> Matrix<T, N, M> {
    pub fn new(inner: impl Into2dVector<T, N, M>) -> Matrix<T, N, M> {
        Matrix {
            inner: inner.into_2d_vector(),
            transposed: None,
        }
    }
}
