pub mod m_matrix;
pub mod vector;

use m_matrix::Matrix;
use vector::Vector;

pub fn old_matrix_mult(size: usize) {
    let multi_m1: Matrix<f64> = Matrix::new_random(size, size).unwrap();
    let multi_m2: Matrix<f64> = Matrix::new_random(size, size).unwrap();

    let _: Matrix<f64> = multi_m1 * multi_m2;
}

pub fn vector_iterator() {
    let test_vec: Vector<u8, 10000> = Vector::new([0u8; 10000]);
    let mut sum: usize = 0;
    for s in test_vec.into_iter() {
        sum = sum + s as usize;
    }
}

pub fn vector_mut_iterator() {
    let mut test_vec: Vector<u8, 10000> = Vector::new([1u8; 10000]);
    for s in test_vec.iter_mut() {
        *s = *s + *s;
    }
}
