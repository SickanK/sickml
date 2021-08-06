use num_traits::{FromPrimitive, Num};

use crate::m_matrix::Matrix;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

impl<T: Add<Output = T>> Add for Matrix<T>
where
    T: Copy + Num + FromPrimitive,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::add(&self, &other).unwrap()
    }
}

impl<T> AddAssign for Matrix<T>
where
    T: Copy + Num + FromPrimitive,
{
    fn add_assign(&mut self, other: Self) {
        *self = Self::add(self, &other).unwrap();
    }
}

impl<T> Sub for Matrix<T>
where
    T: Copy + Num + FromPrimitive,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::sub(&self, &other).unwrap()
    }
}

impl<T> SubAssign for Matrix<T>
where
    T: Copy + Num + FromPrimitive,
{
    fn sub_assign(&mut self, other: Self) {
        *self = Self::sub(&self, &other).unwrap();
    }
}

impl<T> Mul for Matrix<T>
where
    T: FromPrimitive + Num + Copy + AddAssign,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::mult(&self, &rhs).unwrap()
    }
}

impl<T> MulAssign for Matrix<T>
where
    T: FromPrimitive + Num + Copy + AddAssign,
{
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self::mult(&self, &rhs).unwrap()
    }
}
