pub mod m_matrix;
pub mod matrix;
pub mod vector;
use sick_ml::vector_mut_iterator;
use vector::Vector;

/*
 *
 * Basic matrix multiplication (release):
 * 10x10: 5.4700 µs
 * 100x100: 1.3726 ms
 * 500x500: 212.74 ms
 * 1000x1000: 5.7036 s
 * 5000x5000: 1550 s
 *
 */

fn main() {
    vector_mut_iterator();
    println!("Hello world!");
}
