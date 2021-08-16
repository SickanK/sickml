use crate::math_vector::MathVector;
use crate::vector::Vector;
use num::{FromPrimitive, ToPrimitive};
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

use super::Matrix;

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
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
    pub fn mult<const P: usize>(self, matrix2: Matrix<T, N, P>) -> Matrix<T, M, P>
    where
        T: FromPrimitive + ToPrimitive + Debug + Copy + Mul<Output = T> + AddAssign,
    {
        let mut multiplied_matrix_data: [Vector<T, P>; M] = unsafe { std::mem::zeroed() };

        if P * M < 300 * 800 {
            for row in 0..M {
                for col in 0..P {
                    let mut acc: T = T::default();

                    for index in 0..P {
                        acc += self.inner[row][index] * matrix2.inner[index][col]
                    }
                    multiplied_matrix_data[row][col] = acc;
                }
            }
        } else {
            for (idx_row, row) in self.into_iter().enumerate() {
                for (idx_col, col) in matrix2.transpose().into_iter().enumerate() {
                    multiplied_matrix_data[idx_row][idx_col] =
                        FromPrimitive::from_isize(row.dot(col)).expect("Expected valid isize");
                }
            }
        }

        Matrix {
            inner: multiplied_matrix_data,
        }
    }
}
