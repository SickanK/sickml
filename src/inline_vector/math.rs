use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign},
};

use crate::{math_vector::MathVector, matrix::Matrix};
use num::{integer::Roots, FromPrimitive, ToPrimitive};

use super::InlineVector;

impl<T, const N: usize> MathVector<T, N> for InlineVector<T, N>
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
        let mut scaled_array: [T; N] = [T::default(); N];

        let scalar: T = FromPrimitive::from_isize(scalar).expect("Expected isize");
        for (idx, num) in self.data.iter().enumerate() {
            scaled_array[idx] = *num * scalar;
        }

        InlineVector { data: scaled_array }
    }

    fn scalar_mut(&mut self, scalar: isize) {
        let scalar: T = FromPrimitive::from_isize(scalar).expect("Expected isize");
        for num in self.data.iter_mut() {
            *num = *num * scalar;
        }
    }

    fn dot(&self, rhs: impl MathVector<T, N> + Index<usize, Output = T>) -> isize {
        let mut acc: T = T::default();

        for idx in 0..N {
            acc += self.data[idx] * rhs[idx];
        }

        ToPrimitive::to_isize(&acc).expect("Type of T is not supported")
    }

    fn add_vector(&self, rhs: impl MathVector<T, N> + Index<usize, Output = T>) -> Self {
        let mut added_array: [T; N] = [T::default(); N];

        for (idx, num) in added_array.iter_mut().enumerate() {
            *num = self.data[idx] + rhs[idx];
        }

        InlineVector { data: added_array }
    }

    fn add_vector_mut(&mut self, rhs: impl MathVector<T, N> + Index<usize, Output = T>) {
        for (idx, num) in self.iter_mut().enumerate() {
            *num += rhs[idx];
        }
    }

    fn sub_vector(&self, rhs: impl MathVector<T, N> + Index<usize, Output = T>) -> Self {
        let mut subtracted_array: [T; N] = [T::default(); N];

        for (idx, num) in subtracted_array.iter_mut().enumerate() {
            *num = self.data[idx] - rhs[idx];
        }

        InlineVector {
            data: subtracted_array,
        }
    }

    fn sub_vector_mut(&mut self, rhs: impl MathVector<T, N> + Index<usize, Output = T>) {
        for (idx, num) in self.iter_mut().enumerate() {
            *num -= rhs[idx];
        }
    }

    fn entrywise(&self, rhs: impl MathVector<T, N> + Index<usize, Output = T>) -> Self {
        let mut multiplied_array: [T; N] = [T::default(); N];

        for (idx, num) in multiplied_array.iter_mut().enumerate() {
            *num = self.data[idx] * rhs[idx];
        }

        InlineVector {
            data: multiplied_array,
        }
    }

    fn entrywise_mut(&mut self, rhs: impl MathVector<T, N> + Index<usize, Output = T>) {
        for (idx, num) in self.iter_mut().enumerate() {
            *num *= rhs[idx];
        }
    }

    fn cross(&self, rhs: impl MathVector<T, N> + Index<usize, Output = T>) -> Self {
        if N != 3 {
            panic!("The cross product requires that the length of both vectors must be 3");
        }

        let mut crossed_array: [T; N] = [T::default(); N];

        crossed_array[0] = self.data[1] * rhs[2] - self.data[2] * rhs[1];
        crossed_array[1] = self.data[2] * rhs[0] - self.data[0] * rhs[2];
        crossed_array[2] = self.data[0] * rhs[1] - self.data[1] * rhs[0];

        InlineVector {
            data: crossed_array,
        }
    }

    fn cross_mut(&mut self, rhs: impl MathVector<T, N> + Index<usize, Output = T>) {
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
        rhs: impl MathVector<T, N> + Index<usize, Output = T>,
    ) -> Matrix<T, M, N> {
        let mut tensor_product: Matrix<T, M, N> = Matrix::new([[T::default(); N]; M]);

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
