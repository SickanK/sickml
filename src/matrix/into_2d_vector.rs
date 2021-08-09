use crate::vector::Vector;
use core::fmt;
use num::FromPrimitive;

pub trait Into2dVector<T, const N: usize, const M: usize> {
    fn into_2d_vector(self) -> [Vector<T, M>; N];
}

impl<T, const N: usize, const M: usize> Into2dVector<T, N, M> for [Vector<T, M>; N] {
    fn into_2d_vector(self) -> [Vector<T, M>; N] {
        self
    }
}

impl<T, const N: usize, const M: usize> Into2dVector<T, N, M> for Vec<Vec<T>>
where
    T: fmt::Debug + FromPrimitive + Copy,
{
    fn into_2d_vector(self) -> [Vector<T, M>; N] {
        let mut array_of_vectors: [Vector<T, M>; N] =
            [Vector::new([FromPrimitive::from_u8(0).unwrap(); M]); N];

        for (idx, container) in array_of_vectors.iter_mut().enumerate() {
            *container = Vector::new(self[idx].clone());
        }

        array_of_vectors
    }
}

impl<T, const N: usize, const M: usize> Into2dVector<T, N, M> for Vec<[T; M]>
where
    T: fmt::Debug + FromPrimitive + Copy,
{
    fn into_2d_vector(self) -> [Vector<T, M>; N] {
        let mut array_of_vectors: [Vector<T, M>; N] =
            [Vector::new([FromPrimitive::from_u8(0).unwrap(); M]); N];

        for (idx, container) in array_of_vectors.iter_mut().enumerate() {
            *container = Vector::new(self[idx]);
        }

        array_of_vectors
    }
}

impl<T, const N: usize, const M: usize> Into2dVector<T, N, M> for [[T; M]; N]
where
    T: fmt::Debug + FromPrimitive + Copy,
{
    fn into_2d_vector(self) -> [Vector<T, M>; N]
where {
        let mut array_of_vectors: [Vector<T, M>; N] =
            [Vector::new([FromPrimitive::from_u8(0).unwrap(); M]); N];

        for (idx, container) in array_of_vectors.iter_mut().enumerate() {
            *container = Vector::new(self[idx]);
        }

        array_of_vectors
    }
}

impl<T, const N: usize, const M: usize> Into2dVector<T, N, M> for [Vec<T>; N]
where
    T: fmt::Debug + FromPrimitive + Copy,
{
    fn into_2d_vector(self) -> [Vector<T, M>; N] {
        let mut array_of_vectors: [Vector<T, M>; N] =
            [Vector::new([FromPrimitive::from_u8(0).unwrap(); M]); N];

        for (idx, vec) in array_of_vectors.iter_mut().enumerate() {
            *vec = Vector::new(self[idx].clone());
        }

        array_of_vectors
    }
}

impl<T, const N: usize, const M: usize> Into2dVector<T, N, M> for Vec<Vector<T, M>>
where
    T: fmt::Debug + FromPrimitive + Copy,
{
    fn into_2d_vector(self) -> [Vector<T, M>; N] {
        let mut array_of_vectors: [Vector<T, M>; N] =
            [Vector::new([FromPrimitive::from_u8(0).unwrap(); M]); N];

        for (idx, container) in array_of_vectors.iter_mut().enumerate() {
            *container = self[idx];
        }

        array_of_vectors
    }
}
