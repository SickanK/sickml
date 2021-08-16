use core::fmt;

use num::FromPrimitive;

use crate::vector::Vector;

use super::Matrix;

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: FromPrimitive + fmt::Debug + Copy,
{
    pub fn transpose(&self) -> Matrix<T, N, M> {
        let mut transposed: [Vector<T, M>; N] = unsafe { std::mem::zeroed() };

        for (idx_row, row) in self.iter().enumerate() {
            for (idx_col, num) in row.iter().enumerate() {
                transposed[idx_col][idx_row] = *num;
            }
        }

        Matrix { inner: transposed }
    }
}
