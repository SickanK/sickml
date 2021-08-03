use rand::{self, Rng};
use std::vec::Vec;

// implement iterate
// make sure that matrix only include numbers
// make it generic
// create a better structure

pub struct Matrix {
    matrix: Vec<Vec<f64>>,
    columns: usize,
    rows: usize,
}

impl Matrix {
    pub fn new_empty(rows: usize, columns: usize) -> Result<Matrix, String> {
        if rows <= 0 && columns <= 0 {
            return Err(String::from("Matrix must have more than 1 row and column"));
        }

        let mut matrix: Vec<Vec<f64>> = Vec::new();

        for _ in 0..rows {
            let mut row: Vec<f64> = Vec::new();
            for _ in 0..columns {
                row.push(0.0);
            }
            matrix.push(row);
        }

        Ok(Matrix {
            matrix,
            rows,
            columns,
        })
    }

    pub fn new_random(rows: usize, columns: usize) -> Result<Matrix, String> {
        if rows <= 0 && columns <= 0 {
            return Err(String::from("Matrix must have more than 1 row and column"));
        }

        let mut matrix: Vec<Vec<f64>> = Vec::new();
        let mut rng = rand::thread_rng();

        for _ in 0..rows {
            let mut row: Vec<f64> = Vec::new();
            for _ in 0..columns {
                row.push(rng.gen());
            }
            matrix.push(row);
        }

        Ok(Matrix {
            matrix,
            rows,
            columns,
        })
    }

    pub fn from(matrix: Vec<Vec<f64>>) -> Result<Matrix, String> {
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

impl Matrix {
    pub fn dim(&self) -> (usize, usize) {
        (self.rows, self.columns)
    }
}

impl Matrix {
    pub fn mult(m1: Matrix, m2: Matrix) -> Result<Matrix, String> {
        let m1_dim = m1.dim();
        let m2_dim = m2.dim();

        if m1_dim.1 != m2_dim.0 {
            return Err(String::from(
                "Number of columns in the first matrix must match number of rows in the second matrix",
            ));
        }

        let Matrix = Matrix::new_empty(m1_dim.0, m2_dim.1);

        for row in 0..m1_dim.0 {
            for col in 0..m2_dim.1 {
                // assign to matrix
            }
        }

        Ok(m1)
    }

    pub fn hadamard(m1: Matrix, m2: Matrix) -> Result<Matrix, String> {
        let m1_dim = m1.dim();
        let m2_dim = m2.dim();

        if m1_dim != m2_dim {
            return Err(String::from("Both matrices must be identically shaped"));
        }

        Ok(m1)
    }

    pub fn dot(m1: Matrix, m2: Matrix) -> Result<usize, String> {
        let m1_dim = m1.dim();
        let m2_dim = m2.dim();

        if m1_dim != m2_dim {
            return Err(String::from("Both matrices must be identically shaped"));
        }

        Ok(0)
    }
}
