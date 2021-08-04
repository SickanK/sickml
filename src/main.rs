pub mod matrix;
use matrix::Matrix;
use std::time::Instant;

/*
 *
 * Basic matrix multiplication (release):
 * 10x10: 4.594µs
 * 100x100: 1.294321ms
 * 500x500: 228.77975ms
 * 1000x1000: 6.823345366s
 * 5000x5000: 1550.443132781s
 *
 */

fn main() {
    /*
     * Matrix multiplication test
     *
        const SIZE: usize = 5000;

        let multi_m1: Matrix<f64> = Matrix::new_random(SIZE, SIZE).unwrap();
        let multi_m2: Matrix<f64> = Matrix::new_random(SIZE, SIZE).unwrap();

        let now = Instant::now();
        let _: Matrix<f64> = multi_m1 * multi_m2;
        let new_now = Instant::now();

        println!("{:?}", new_now.duration_since(now));
    */
}
