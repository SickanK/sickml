use core::fmt;
use std::convert::TryInto;

pub trait IntoArray<T, const N: usize> {
    fn into_array(self) -> [T; N];
}

impl<T, const N: usize> IntoArray<T, N> for Vec<T>
where
    T: fmt::Debug,
{
    fn into_array(self) -> [T; N] {
        self.try_into().unwrap_or_else(|v: Vec<T>| {
            panic!("Expected a Vec of length {} but it was {}", N, v.len())
        })
    }
}

impl<T, const N: usize> IntoArray<T, N> for [T; N]
where
    T: fmt::Debug,
{
    fn into_array(self) -> [T; N] {
        self
    }
}
