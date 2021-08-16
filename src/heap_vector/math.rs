use num::integer::Roots;
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

use num::{FromPrimitive, ToPrimitive};

use crate::{math_vector::MathVector, matrix::Matrix};

use super::HeapVector;

impl<T, const N: usize> MathVector<T, N> for HeapVector<T, N>
where
    T: Default
        + Copy
        + FromPrimitive
        + ToPrimitive
        + Mul<Output = T>
        + MulAssign
        + Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Debug,
{
    fn scalar(&self, scalar: isize) -> Self {
        let mut scaled_vec: Vec<T> = Vec::with_capacity(N);

        for num in self.data.iter() {
            scaled_vec.push(*num * FromPrimitive::from_isize(scalar).expect("Expected isize"));
        }

        HeapVector { data: scaled_vec }
    }

    fn scalar_mut(&mut self, scalar: isize) {
        for num in self.data.iter_mut() {
            *num *= FromPrimitive::from_isize(scalar).expect("Expected isize")
        }
    }

    fn dot(&self, rhs: impl MathVector<T, N> + std::ops::Index<usize, Output = T>) -> isize {
        let mut acc: T = T::default();

        for idx in 0..N {
            acc += self.data[idx] * rhs[idx];
        }

        ToPrimitive::to_isize(&acc).expect("Type of T is not supported")
    }

    fn add_vector(&self, rhs: impl MathVector<T, N> + std::ops::Index<usize, Output = T>) -> Self {
        let mut added_vec: Vec<T> = Vec::with_capacity(N);

        for idx in 0..N {
            added_vec.push(self.data[idx] + rhs[idx]);
        }

        HeapVector { data: added_vec }
    }

    fn add_vector_mut(&mut self, rhs: impl MathVector<T, N> + std::ops::Index<usize, Output = T>) {
        for (idx, num) in self.data.iter_mut().enumerate() {
            *num += rhs[idx];
        }
    }

    fn sub_vector(&self, rhs: impl MathVector<T, N> + std::ops::Index<usize, Output = T>) -> Self {
        let mut subtracted_vec: Vec<T> = Vec::with_capacity(N);

        for idx in 0..N {
            subtracted_vec.push(self.data[idx] - rhs[idx]);
        }

        HeapVector {
            data: subtracted_vec,
        }
    }

    fn sub_vector_mut(&mut self, rhs: impl MathVector<T, N> + std::ops::Index<usize, Output = T>) {
        for (idx, num) in self.data.iter_mut().enumerate() {
            *num -= rhs[idx];
        }
    }

    fn entrywise(&self, rhs: impl MathVector<T, N> + std::ops::Index<usize, Output = T>) -> Self {
        let mut multiplied_vec: Vec<T> = Vec::with_capacity(N);

        for idx in 0..N {
            multiplied_vec.push(self.data[idx] * rhs[idx]);
        }

        HeapVector {
            data: multiplied_vec,
        }
    }

    fn entrywise_mut(&mut self, rhs: impl MathVector<T, N> + std::ops::Index<usize, Output = T>) {
        for (idx, num) in self.data.iter_mut().enumerate() {
            *num *= rhs[idx];
        }
    }

    fn cross(&self, rhs: impl MathVector<T, N> + std::ops::Index<usize, Output = T>) -> Self {
        if N != 3 {
            panic!("The cross product requires that the length of both vectors must be 3");
        }

        let mut crossed_vec: Vec<T> = Vec::with_capacity(N);

        crossed_vec[0] = self.data[1] * rhs[2] - self.data[2] * rhs[1];
        crossed_vec[1] = self.data[2] * rhs[0] - self.data[0] * rhs[2];
        crossed_vec[2] = self.data[0] * rhs[1] - self.data[1] * rhs[0];

        HeapVector { data: crossed_vec }
    }

    fn cross_mut(&mut self, rhs: impl MathVector<T, N> + std::ops::Index<usize, Output = T>) {
        if N != 3 {
            panic!("The cross product requires that the length of both vectors must be 3");
        }

        let data = self.data.clone();
        self.data[0] = data[1] * rhs[2] - data[2] * rhs[1];
        self.data[1] = data[2] * rhs[0] - data[0] * rhs[2];
        self.data[2] = data[0] * rhs[1] - data[1] * rhs[0];
    }

    fn tensor_prod<const M: usize>(
        &self,
        rhs: impl MathVector<T, N> + std::ops::Index<usize, Output = T>,
    ) -> Matrix<T, M, N> {
        let mut tensor_product: Matrix<T, M, N> = Matrix::default();

        for (row_idx, row) in tensor_product.iter_mut().enumerate() {
            for (col_idx, col) in row.iter_mut().enumerate() {
                *col = self.data[row_idx] * rhs[col_idx];
            }
        }

        tensor_product
    }

    fn magnitude(&self) -> usize {
        let mut acc: T = T::default();

        for num in self.iter() {
            acc += *num * *num;
        }

        let isize_acc = ToPrimitive::to_usize(&acc)
            .expect("Valid integers are required to calculate the magnitude");

        isize_acc.sqrt()
    }

    fn sum(&self) -> isize {
        let mut acc: T = T::default();

        for num in self.iter() {
            acc += *num;
        }

        ToPrimitive::to_isize(&acc).expect("Valid integers are required to calculate the sum")
    }
}
