use crate::vector::Vector;
use num::{FromPrimitive, ToPrimitive};
use std::{
    fmt::Debug,
    ops::{AddAssign, Mul},
};

use super::Matrix;

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn mult<const P: usize>(&self, matrix2: &Matrix<T, N, P>) -> Matrix<T, M, P>
    where
        T: FromPrimitive + ToPrimitive + Debug + Copy + Mul<Output = T> + AddAssign,
    {
        let mut multiplied_matrix_data: [Vector<T, P>; M] =
            [Vector::new([FromPrimitive::from_u8(0).unwrap(); P]); M];

        if P * M < 300 * 800 {
            for row in 0..M {
                for col in 0..P {
                    let mut acc: T = FromPrimitive::from_u8(0).unwrap();

                    for index in 0..P {
                        acc += self.inner[row][index] * matrix2.inner[index][col]
                    }
                    multiplied_matrix_data[row][col] = acc;
                }
            }
        } else {
            let matrix2_transposed = matrix2.transpose();
            for (idx_row, row) in self.iter().enumerate() {
                for (idx_col, col) in matrix2_transposed.iter().enumerate() {
                    multiplied_matrix_data[idx_row][idx_col] = row.dot(*col);
                }
            }
        }

        Matrix {
            inner: multiplied_matrix_data,
        }
    }
}
