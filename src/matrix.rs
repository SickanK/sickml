pub mod into_2d_vector;
pub mod iterator;
pub mod math;
pub mod transpose;

use std::fmt::Debug;

use crate::vector::Vector;
use into_2d_vector::Into2dVector;
use num::FromPrimitive;
use rand::{distributions::Standard, prelude::Distribution};

#[derive(Debug)]
pub struct Matrix<T, const M: usize, const N: usize> {
    inner: [Vector<T, N>; M],
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn new(inner: impl Into2dVector<T, M, N>) -> Matrix<T, M, N> {
        Matrix {
            inner: inner.into_2d_vector(),
        }
    }

    pub fn new_random() -> Matrix<T, M, N>
    where
        T: Default + FromPrimitive + Debug + Copy,
        Standard: Distribution<T>,
    {
        let mut random_matrix_data: Vec<Vector<T, N>> = Vec::with_capacity(M);

        for _ in 0..M {
            random_matrix_data.push(Vector::new_random())
        }

        Matrix {
            inner: random_matrix_data.into_2d_vector(),
        }
    }
}

impl<T, const M: usize, const N: usize> Default for Matrix<T, M, N>
where
    T: Default + Clone + FromPrimitive + Copy + Debug,
{
    fn default() -> Matrix<T, M, N> {
        Matrix {
            inner: vec![Vector::default(); M].into_2d_vector(),
        }
    }
}
