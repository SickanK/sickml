use crate::math_vector::MathVector;
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

use num::{FromPrimitive, ToPrimitive};

use super::Vector;

impl<T, const N: usize> Add for Vector<T, N>
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
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        match self {
            Self::Inline(inline_vector) => Vector::Inline(inline_vector.add_vector(rhs)),
            Self::Heap(heap_vector) => Vector::Heap(heap_vector.add_vector(rhs)),
        }
    }
}

impl<T, const N: usize> AddAssign for Vector<T, N>
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
    fn add_assign(&mut self, rhs: Self) {
        match self {
            Self::Inline(inline_vector) => inline_vector.add_vector_mut(rhs),
            Self::Heap(heap_vector) => heap_vector.add_vector_mut(rhs),
        }
    }
}

impl<T, const N: usize> Sub for Vector<T, N>
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
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        match self {
            Self::Inline(inline_vector) => Vector::Inline(inline_vector.sub_vector(rhs)),
            Self::Heap(heap_vector) => Vector::Heap(heap_vector.add_vector(rhs)),
        }
    }
}

impl<T, const N: usize> SubAssign for Vector<T, N>
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
    fn sub_assign(&mut self, rhs: Self) {
        match self {
            Self::Inline(inline_vector) => inline_vector.sub_vector_mut(rhs),
            Self::Heap(heap_vector) => heap_vector.add_vector_mut(rhs),
        }
    }
}

impl<T, const N: usize> Mul for Vector<T, N>
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
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        match self {
            Self::Inline(inline_vector) => Vector::Inline(inline_vector.entrywise(rhs)),
            Self::Heap(heap_vector) => Vector::Heap(heap_vector.entrywise(rhs)),
        }
    }
}

impl<T, const N: usize> MulAssign for Vector<T, N>
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
    fn mul_assign(&mut self, rhs: Self) {
        match self {
            Self::Inline(inline_vector) => inline_vector.entrywise_mut(rhs),
            Self::Heap(heap_vector) => heap_vector.entrywise_mut(rhs),
        }
    }
}
