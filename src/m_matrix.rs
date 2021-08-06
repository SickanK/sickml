pub mod math;
pub mod math_operators;
pub mod operators;

use num_traits::{FromPrimitive, Num};

use rand::{self, distributions::Standard, prelude::Distribution, Rng};
use std::{
    fmt,
    fmt::{Debug, Display, Formatter},
    vec::Vec,
};

// implement iterate

#[derive(Debug)]
pub struct Matrix<T: FromPrimitive + Num> {
    matrix: Vec<Vec<T>>,
    columns: usize,
    rows: usize,
}

// Print
impl<T: FromPrimitive + Num> Display for Matrix<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.matrix)
    }
}

impl<T: FromPrimitive + Num> Matrix<T> {
    pub fn new_empty(rows: usize, columns: usize) -> Result<Matrix<T>, String> {
        if rows <= 0 && columns <= 0 {
            return Err(String::from("Matrix must have more than 1 row and column"));
        }

        let mut matrix: Vec<Vec<T>> = Vec::new();

        for _ in 0..rows {
            let mut row: Vec<T> = Vec::new();
            for _ in 0..columns {
                row.push(FromPrimitive::from_u8(0).unwrap());
            }
            matrix.push(row);
        }

        Ok(Matrix {
            matrix,
            rows,
            columns,
        })
    }

    pub fn new_random(rows: usize, columns: usize) -> Result<Matrix<T>, String>
    where
        Standard: Distribution<T>,
    {
        if rows <= 0 && columns <= 0 {
            return Err(String::from("Matrix must have more than 1 row and column"));
        }

        let mut matrix: Vec<Vec<T>> = Vec::new();
        let mut rng = rand::thread_rng();

        for _ in 0..rows {
            let mut row: Vec<T> = Vec::new();
            for _ in 0..columns {
                let random_num: T = rng.gen::<T>();
                row.push(random_num);
            }
            matrix.push(row);
        }

        Ok(Matrix {
            matrix,
            rows,
            columns,
        })
    }

    pub fn from(matrix: Vec<Vec<T>>) -> Result<Matrix<T>, String> {
        let columns = matrix[0].len();
        let rows = matrix.len();

        for w in &matrix {
            if w.len() != columns {
                return Err(String::from("Matrix rows has variable lengths"));
            }
        }

        Ok(Matrix {
            rows,
            matrix,
            columns,
        })
    }
}
