use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use num::FromPrimitive;

use crate::vector::Vector;

impl<T, const N: usize> Add for Vector<T, N>
where
    T: Copy + Add<Output = T> + FromPrimitive,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        self.sum(other)
    }
}

impl<T, const N: usize> AddAssign for Vector<T, N>
where
    T: Copy + AddAssign,
{
    fn add_assign(&mut self, other: Self) {
        self.sum_mut(other)
    }
}

impl<T, const N: usize> Sub for Vector<T, N>
where
    T: Copy + Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self.subtract(other)
    }
}

impl<T, const N: usize> SubAssign for Vector<T, N>
where
    T: Copy + SubAssign,
{
    fn sub_assign(&mut self, other: Self) {
        self.subtract_mut(other)
    }
}

impl<T, const N: usize> Mul for Vector<T, N>
where
    T: Copy + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        self.entrywise(rhs)
    }
}

impl<T, const N: usize> MulAssign for Vector<T, N>
where
    T: Copy + MulAssign,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.entrywise_mut(rhs)
    }
}
