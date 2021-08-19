use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

use num::{FromPrimitive, ToPrimitive};

use crate::math_vector::MathVector;

use super::InlineVector;

impl<T, const N: usize> Add for InlineVector<T, N>
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
        self.add_vector(rhs)
    }
}

impl<T, const N: usize> AddAssign for InlineVector<T, N>
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
        self.add_vector_mut(rhs)
    }
}

impl<T, const N: usize> Sub for InlineVector<T, N>
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
        self.sub_vector(rhs)
    }
}

impl<T, const N: usize> SubAssign for InlineVector<T, N>
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
        self.sub_vector_mut(rhs)
    }
}

impl<T, const N: usize> Mul for InlineVector<T, N>
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
        self.entrywise(rhs)
    }
}

impl<T, const N: usize> MulAssign for InlineVector<T, N>
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
        self.entrywise_mut(rhs)
    }
}
