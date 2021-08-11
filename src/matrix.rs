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
        T: FromPrimitive + Debug + Copy,
        Standard: Distribution<T>,
    {
        let mut random_matrix_data: [Vector<T, N>; M] =
            [Vector::new([FromPrimitive::from_u8(0).unwrap(); N]); M];

        for idx in 0..M {
            random_matrix_data[idx] = Vector::new_random()
        }

        Matrix {
            inner: random_matrix_data,
        }
    }

    pub fn new_random_boxed() -> Matrix<T, M, N>
    where
        T: FromPrimitive + Debug + Copy,
        Standard: Distribution<T>,
    {
        let mut random_matrix_data: [Vector<T, N>; M] =
            [Vector::new([FromPrimitive::from_u8(0).unwrap(); N]); M];

        for idx in 0..M {
            random_matrix_data[idx] = Vector::new_random_boxed()
        }

        Matrix {
            inner: random_matrix_data,
        }
    }
}
