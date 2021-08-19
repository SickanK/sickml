use std::convert::TryInto;

use crate::vector::heap_vector::HeapVector;

pub trait IntoArray<T, const N: usize> {
    fn into_array(self) -> [T; N];
}

impl<T, const N: usize> IntoArray<T, N> for [T; N]
where
    T: Copy,
{
    fn into_array(self) -> [T; N] {
        self
    }
}

impl<T, const N: usize> IntoArray<T, N> for Vec<T> {
    fn into_array(self) -> [T; N] {
        self.try_into().unwrap_or_else(|v: Vec<T>| {
            panic!("Expected a Vec of length {} but it was {}", N, v.len())
        })
    }
}

impl<T, const N: usize> IntoArray<T, N> for HeapVector<T, N>
where
    T: Clone,
{
    fn into_array(self) -> [T; N] {
        self.to_array()
    }
}
