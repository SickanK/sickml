use core::fmt;
use std::mem::MaybeUninit;

use num::FromPrimitive;

use crate::vector::Vector;

use super::Matrix;

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: FromPrimitive + fmt::Debug + Copy,
{
    pub fn transpose(&self) -> Matrix<T, N, M> {
        let mut transposed: [Vector<MaybeUninit<T>, M>; N] =
            [Vector::new([unsafe { MaybeUninit::uninit().assume_init() }; M]); N];

        for (idx_row, row) in self.iter().enumerate() {
            for (idx_col, num) in row.iter().enumerate() {
                transposed[idx_col][idx_row] = MaybeUninit::new(*num);
            }
        }

        let transposed_cast = unsafe { transposed.as_ptr().cast::<[Vector<T, M>; N]>().read() };

        Matrix {
            inner: transposed_cast,
        }
    }
}
