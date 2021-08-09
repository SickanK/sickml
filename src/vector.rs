pub mod into_array;
pub mod iterator;
pub mod math;
pub mod math_ops;

use into_array::IntoArray;
use num::FromPrimitive;
use rand::{distributions::Standard, prelude::Distribution, Rng};
use std::ops::{Index, IndexMut};

// new ones
// new zeroes

#[derive(Copy, Clone, Debug)]
pub struct Vector<T, const N: usize> {
    inner: [T; N],
}

impl<'a, T, const N: usize> Vector<T, N> {
    pub fn new(inner: impl IntoArray<T, N>) -> Self {
        Vector {
            inner: inner.into_array(),
        }
    }

    pub fn new_random() -> Vector<T, N>
    where
        Standard: Distribution<T>,
        T: FromPrimitive + Copy,
    {
        let mut inner: [T; N] = [FromPrimitive::from_u8(0).unwrap(); N];
        let mut rng = rand::thread_rng();

        for num in &mut inner {
            let random_num: T = rng.gen::<T>();
            *num = random_num;
        }

        Vector { inner }
    }

    pub fn new_zeroes() -> Vector<T, N>
    where
        T: FromPrimitive + Copy,
    {
        Vector {
            inner: [FromPrimitive::from_u8(0).unwrap(); N],
        }
    }

    pub fn new_ones() -> Vector<T, N>
    where
        T: FromPrimitive + Copy,
    {
        Vector {
            inner: [FromPrimitive::from_u8(1).unwrap(); N],
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
