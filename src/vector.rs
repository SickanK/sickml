pub mod into_array;
pub mod iterator;
pub mod math;

use std::ops::{Index, IndexMut};

use into_array::IntoArray;

// new random
// new ones
// new zeroes

#[derive(Debug)]
pub struct Vector<T, const N: usize> {
    inner: [T; N],
}

impl<'a, T, const N: usize> Vector<T, N> {
    pub fn new(inner: impl IntoArray<T, N>) -> Self {
        Vector {
            inner: inner.into_array(),
        }
    }
}

impl<T, const N: usize> Index<usize> for Vector<T, N> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.inner[idx]
    }
}

impl<T, const N: usize> IndexMut<usize> for Vector<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}
