use crate::vector::Vector;
use core::fmt;
use num::FromPrimitive;
use std::mem::{replace, swap, take};

pub trait Into2dVector<T, const M: usize, const N: usize> {
    fn into_2d_vector(self) -> [Vector<T, N>; M];
}
impl<T, const M: usize, const N: usize> Into2dVector<T, M, N> for [Vector<T, N>; M] {
    fn into_2d_vector(self) -> [Vector<T, N>; M] {
        self
    }
}

impl<T, const M: usize, const N: usize> Into2dVector<T, M, N> for Vec<Vec<T>>
where
    T: Default + fmt::Debug + FromPrimitive + Copy,
{
    fn into_2d_vector(self) -> [Vector<T, N>; M] {
        let mut array_of_vectors: [Vector<T, N>; M] = unsafe { std::mem::zeroed() };

        for (idx, container) in array_of_vectors.iter_mut().enumerate() {
            *container = Vector::new(self[idx].clone());
        }

        array_of_vectors
    }
}

impl<T, const M: usize, const N: usize> Into2dVector<T, M, N> for Vec<[T; N]>
where
    T: Default + fmt::Debug + FromPrimitive + Copy,
{
    fn into_2d_vector(self) -> [Vector<T, N>; M] {
        let mut array_of_vectors: [Vector<T, N>; M] = unsafe { std::mem::zeroed() };

        for (idx, container) in array_of_vectors.iter_mut().enumerate() {
            *container = Vector::new(self[idx]);
        }

        array_of_vectors
    }
}

impl<T, const M: usize, const N: usize> Into2dVector<T, M, N> for [[T; N]; M]
where
    T: Default + fmt::Debug + FromPrimitive + Copy,
{
    fn into_2d_vector(self) -> [Vector<T, N>; M]
where {
        let mut array_of_vectors: [Vector<T, N>; M] = unsafe { std::mem::zeroed() };

        for (idx, container) in array_of_vectors.iter_mut().enumerate() {
            *container = Vector::new(self[idx]);
        }

        array_of_vectors
    }
}

impl<T, const M: usize, const N: usize> Into2dVector<T, M, N> for [Vec<T>; M]
where
    T: Default + fmt::Debug + FromPrimitive + Copy,
{
    fn into_2d_vector(self) -> [Vector<T, N>; M] {
        let mut array_of_vectors: [Vector<T, N>; M] = unsafe { std::mem::zeroed() };

        for (idx, vec) in array_of_vectors.iter_mut().enumerate() {
            *vec = Vector::new(self[idx].clone());
        }

        array_of_vectors
    }
}

impl<T, const M: usize, const N: usize> Into2dVector<T, M, N> for Vec<Vector<T, N>>
where
    T: Default + fmt::Debug + FromPrimitive + Copy,
{
    fn into_2d_vector(self) -> [Vector<T, N>; M] {
        let mut array_of_vectors: [Vector<T, N>; M] = unsafe { std::mem::zeroed() };

        for (idx, container) in array_of_vectors.iter_mut().enumerate() {
            *container = self[idx].clone();
        }

        array_of_vectors
    }
}
