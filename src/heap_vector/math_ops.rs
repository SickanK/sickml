use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use num::ToPrimitive;

use crate::math_vector::MathVector;

use super::HeapVector;

impl<T, const N: usize> Add for HeapVector<T, N>
where
    T: Default + ToPrimitive,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self.add_vector(rhs)
    }
}

impl<T, const N: usize> AddAssign for HeapVector<T, N>
where
    T: Default + ToPrimitive,
{
    fn add_assign(&mut self, rhs: Self) {
        self.add_vector_mut(rhs)
    }
}

impl<T, const N: usize> Sub for HeapVector<T, N>
where
    T: Default + ToPrimitive,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self.sub_vector(rhs)
    }
}

impl<T, const N: usize> SubAssign for HeapVector<T, N>
where
    T: Default + ToPrimitive,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.sub_vector_mut(rhs)
    }
}

impl<T, const N: usize> Mul for HeapVector<T, N>
where
    T: Default + ToPrimitive,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        self.entrywise(rhs)
    }
}

impl<T, const N: usize> MulAssign for HeapVector<T, N>
where
    T: Default + ToPrimitive,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.entrywise_mut(rhs)
    }
}
