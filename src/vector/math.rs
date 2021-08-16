use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Index, Mul, MulAssign, Sub, SubAssign},
};

use num::{FromPrimitive, ToPrimitive};

use crate::{math_vector::MathVector, matrix::Matrix};

use super::Vector;

impl<T, const N: usize> MathVector<T, N> for Vector<T, N>
where
    T: Default
        + Copy
        + FromPrimitive
        + ToPrimitive
        + Mul<Output = T>
        + MulAssign
        + Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Debug,
{
    fn scalar(&self, scalar: isize) -> Self {
        match self {
            Self::Inline(inline_vector) => Vector::Inline(inline_vector.scalar(scalar)),
            Self::Heap(heap_vector) => Vector::Heap(heap_vector.scalar(scalar)),
        }
    }

    fn scalar_mut(&mut self, scalar: isize) {
        match self {
            Self::Inline(inline_vector) => inline_vector.scalar_mut(scalar),
            Self::Heap(heap_vector) => heap_vector.scalar_mut(scalar),
        };
    }

    fn dot(&self, rhs: impl MathVector<T, N> + std::ops::Index<usize, Output = T>) -> isize {
        match self {
            Self::Inline(inline_vector) => inline_vector.dot(rhs),
            Self::Heap(heap_vector) => heap_vector.dot(rhs),
        }
    }

    fn add_vector(&self, rhs: impl MathVector<T, N> + std::ops::Index<usize, Output = T>) -> Self {
        match self {
            Self::Inline(inline_vector) => Vector::Inline(inline_vector.add_vector(rhs)),
            Self::Heap(heap_vector) => Vector::Heap(heap_vector.add_vector(rhs)),
        }
    }

    fn add_vector_mut(&mut self, rhs: impl MathVector<T, N> + std::ops::Index<usize, Output = T>) {
        match self {
            Self::Inline(inline_vector) => inline_vector.add_vector_mut(rhs),
            Self::Heap(heap_vector) => heap_vector.add_vector_mut(rhs),
        }
    }

    fn sub_vector(&self, rhs: impl MathVector<T, N> + std::ops::Index<usize, Output = T>) -> Self {
        match self {
            Self::Inline(inline_vector) => Vector::Inline(inline_vector.sub_vector(rhs)),
            Self::Heap(heap_vector) => Vector::Heap(heap_vector.sub_vector(rhs)),
        }
    }

    fn sub_vector_mut(&mut self, rhs: impl MathVector<T, N> + std::ops::Index<usize, Output = T>) {
        match self {
            Self::Inline(inline_vector) => inline_vector.sub_vector_mut(rhs),
            Self::Heap(heap_vector) => heap_vector.sub_vector_mut(rhs),
        }
    }

    fn entrywise(&self, rhs: impl MathVector<T, N> + std::ops::Index<usize, Output = T>) -> Self {
        match self {
            Self::Inline(inline_vector) => Vector::Inline(inline_vector.entrywise(rhs)),
            Self::Heap(heap_vector) => Vector::Heap(heap_vector.entrywise(rhs)),
        }
    }

    fn entrywise_mut(&mut self, rhs: impl MathVector<T, N> + std::ops::Index<usize, Output = T>) {
        match self {
            Self::Inline(inline_vector) => inline_vector.entrywise_mut(rhs),
            Self::Heap(heap_vector) => heap_vector.entrywise_mut(rhs),
        }
    }

    fn cross(&self, rhs: impl MathVector<T, N> + Index<usize, Output = T>) -> Self {
        match self {
            Self::Inline(inline_vector) => Vector::Inline(inline_vector.cross(rhs)),
            Self::Heap(heap_vector) => Vector::Heap(heap_vector.cross(rhs)),
        }
    }

    fn cross_mut(&mut self, rhs: impl MathVector<T, N> + std::ops::Index<usize, Output = T>) {
        match self {
            Self::Inline(inline_vector) => inline_vector.cross_mut(rhs),
            Self::Heap(heap_vector) => heap_vector.cross_mut(rhs),
        }
    }

    fn tensor_prod<const M: usize>(
        &self,
        rhs: impl MathVector<T, N> + Index<usize, Output = T>,
    ) -> Matrix<T, M, N> {
        match self {
            Self::Inline(inline_vector) => inline_vector.tensor_prod(rhs),
            Self::Heap(heap_vector) => heap_vector.tensor_prod(rhs),
        }
    }

    fn magnitude(&self) -> usize {
        match self {
            Self::Inline(inline_vector) => inline_vector.magnitude(),
            Self::Heap(heap_vector) => heap_vector.magnitude(),
        }
    }

    fn sum(&self) -> isize {
        match self {
            Self::Inline(inline_vector) => inline_vector.sum(),
            Self::Heap(heap_vector) => heap_vector.sum(),
        }
    }
}
