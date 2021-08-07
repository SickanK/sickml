use std::{
    mem::MaybeUninit,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

use num::{FromPrimitive, Num, ToPrimitive};

use crate::vector::Vector;

impl<T, const N: usize> Vector<T, N> {
    pub fn scalar(&self, scalar: isize) -> Self
    where
        T: FromPrimitive + Num + Copy,
    {
        let mut product_array: [T; N] = [FromPrimitive::from_u8(0).unwrap(); N];

        let mut idx: usize = 0;
        for n in &self.inner {
            product_array[idx] = *n * FromPrimitive::from_isize(scalar).unwrap();
            idx += 1;
        }

        Vector {
            inner: product_array,
        }
    }

    pub fn scalar_mut(&mut self, scalar: isize)
    where
        T: FromPrimitive + MulAssign,
    {
        for num in &mut self.inner {
            *num *= FromPrimitive::from_isize(scalar).unwrap()
        }
    }

    pub fn dot(&self, vector2: Vector<T, N>) -> T
    where
        T: Mul<Output = T> + AddAssign<T> + Copy + ToPrimitive + FromPrimitive,
    {
        let mut acc = FromPrimitive::from_u8(0).unwrap();

        for index in 0..self.inner.len() {
            acc += self.inner[index] * vector2.inner[index];
        }

        acc
    }

    pub(super) fn sum(&self, vector2: Vector<T, N>) -> Self
    where
        T: Add<Output = T> + Copy,
    {
        let mut added_array: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        let mut idx = 0;
        for _ in added_array {
            *&mut added_array[idx] = MaybeUninit::new(self.inner[idx] + vector2.inner[idx]);
            idx += 1;
        }

        Vector {
            inner: unsafe { added_array.as_ptr().cast::<[T; N]>().read() },
        }
    }

    pub(super) fn sum_mut(&mut self, vector2: Vector<T, N>)
    where
        T: AddAssign + Copy,
    {
        let mut idx = 0;
        for num in &mut self.inner {
            *num += vector2[idx];
            idx += 1;
        }
    }

    pub(super) fn subtract(&self, vector2: Vector<T, N>) -> Self
    where
        T: Sub<Output = T> + Copy,
    {
        let mut subtracted_array: [MaybeUninit<T>; N] =
            unsafe { MaybeUninit::uninit().assume_init() };

        let mut idx = 0;
        for _ in subtracted_array {
            *&mut subtracted_array[idx] = MaybeUninit::new(self.inner[idx] - vector2.inner[idx]);
            idx += 1;
        }

        Vector {
            inner: unsafe { subtracted_array.as_ptr().cast::<[T; N]>().read() },
        }
    }

    pub(super) fn subtract_mut(&mut self, vector2: Vector<T, N>)
    where
        T: SubAssign + Copy,
    {
        let mut idx = 0;
        for num in &mut self.inner {
            *num -= vector2[idx];
            idx += 1;
        }
    }

    pub fn entrywise(&self, vector2: Vector<T, N>) -> Self
    where
        T: Mul<Output = T> + Copy,
    {
        let mut multiplied_array: [MaybeUninit<T>; N] =
            unsafe { MaybeUninit::uninit().assume_init() };

        let mut idx = 0;
        for _ in multiplied_array {
            *&mut multiplied_array[idx] = MaybeUninit::new(self.inner[idx] * vector2.inner[idx]);
            idx += 1;
        }

        Vector {
            inner: unsafe { multiplied_array.as_ptr().cast::<[T; N]>().read() },
        }
    }

    pub fn entrywise_mut(&mut self, vector2: Vector<T, N>)
    where
        T: MulAssign + Copy,
    {
        let mut idx = 0;
        for num in &mut self.inner {
            *num *= vector2[idx];
            idx += 1;
        }
    }
}

impl<T> Vector<T, 3> {
    pub fn cross(&self, vector2: Vector<T, 3>) -> Vector<T, 3>
    where
        T: FromPrimitive + Mul<Output = T> + Sub<Output = T> + Copy,
    {
        let mut added_array: [T; 3] = [FromPrimitive::from_u8(0).unwrap(); 3];

        added_array[0] = self.inner[1] * vector2.inner[2] - self.inner[2] * vector2.inner[1];
        added_array[1] = self.inner[2] * vector2.inner[0] - self.inner[0] * vector2.inner[2];
        added_array[2] = self.inner[0] * vector2.inner[1] - self.inner[1] * vector2.inner[0];

        Vector { inner: added_array }
    }

    pub fn cross_mut(&mut self, vector2: Vector<T, 3>)
    where
        T: FromPrimitive + Mul<Output = T> + Sub<Output = T> + Copy,
    {
        let v1 = self.inner;
        self.inner[0] = v1[1] * vector2.inner[2] - v1[2] * vector2.inner[1];
        self.inner[1] = v1[2] * vector2.inner[0] - v1[0] * vector2.inner[2];
        self.inner[2] = v1[0] * vector2.inner[1] - v1[1] * vector2.inner[0];
    }
}
