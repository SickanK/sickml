use std::{
    mem::MaybeUninit,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

use num::{FromPrimitive, Num, ToPrimitive};

use super::Vector;

impl<T, const N: usize> Vector<T, N> {
    pub fn scalar(&self, scalar: isize) -> Self
    where
        T: FromPrimitive + Num + Copy,
    {
        if let Some(stack_data) = &self.stack_data {
            let mut product_array: [T; N] = [FromPrimitive::from_u8(0).unwrap(); N];

            let mut idx: usize = 0;
            for n in stack_data {
                product_array[idx] = *n * FromPrimitive::from_isize(scalar).unwrap();
                idx += 1;
            }

            return Vector {
                stack_data: Some(product_array),
                heap_data: None,
                stack_limit: self.stack_limit,
            };
        }

        if let Some(heap_data) = &self.heap_data {
            let mut product_vec: Vec<T> = Vec::new();

            for n in heap_data {
                product_vec.push(*n * FromPrimitive::from_isize(scalar).unwrap());
            }

            return Vector {
                stack_data: None,
                heap_data: Some(product_vec),
                stack_limit: self.stack_limit,
            };
        }

        unreachable!()
    }

    pub fn scalar_mut(&mut self, scalar: isize)
    where
        T: FromPrimitive + MulAssign,
    {
        if let Some(stack_data) = &mut self.stack_data {
            for num in stack_data.iter_mut() {
                *num *= FromPrimitive::from_isize(scalar).unwrap()
            }
        }

        if let Some(heap_data) = &mut self.heap_data {
            for num in heap_data.iter_mut() {
                *num *= FromPrimitive::from_isize(scalar).unwrap()
            }
        }

        unreachable!()
    }

    pub fn dot(&self, vector2: Vector<T, N>) -> T
    where
        T: Mul<Output = T> + AddAssign<T> + Copy + ToPrimitive + FromPrimitive,
    {
        let mut acc = FromPrimitive::from_u8(0).unwrap();

        if let (Some(stack_data), Some(stack_data2)) = (&self.stack_data, &vector2.stack_data) {
            for index in 0..N {
                acc += stack_data[index] * stack_data2[index];
            }
        }

        if let (Some(heap_data), Some(heap_data2)) = (&self.heap_data, &vector2.heap_data) {
            for index in 0..N {
                acc += heap_data[index] * heap_data2[index];
            }
        }

        acc
    }

    pub(super) fn sum(&self, vector2: Vector<T, N>) -> Vector<T, N>
    where
        T: Add<Output = T> + Copy,
    {
        if let (Some(stack_data), Some(stack_data2)) = (&self.stack_data, &vector2.stack_data) {
            let mut added_array: [MaybeUninit<T>; N] =
                unsafe { MaybeUninit::uninit().assume_init() };

            let mut idx = 0;
            for _ in added_array {
                added_array[idx] = MaybeUninit::new(stack_data[idx] + stack_data2[idx]);
                idx += 1;
            }

            return Vector {
                stack_data: Some(unsafe { added_array.as_ptr().cast::<[T; N]>().read() }),
                heap_data: None,
                stack_limit: self.stack_limit,
            };
        }

        if let (Some(heap_data), Some(heap_data2)) = (&self.heap_data, &vector2.heap_data) {
            let mut added_vec: Vec<T> = Vec::new();

            for idx in 0..N {
                added_vec.push(heap_data[idx] + heap_data2[idx]);
            }

            return Vector {
                stack_data: None,
                heap_data: Some(added_vec),
                stack_limit: self.stack_limit,
            };
        }

        unreachable!()
    }

    pub(super) fn sum_mut(&mut self, vector2: Vector<T, N>)
    where
        T: AddAssign + Copy,
    {
        if let (Some(stack_data), Some(stack_data2)) = (&mut self.stack_data, &vector2.stack_data) {
            let mut idx = 0;
            for num in stack_data.iter_mut() {
                *num += stack_data2[idx];
                idx += 1;
            }
        }

        if let (Some(heap_data), Some(heap_data2)) = (&mut self.heap_data, &vector2.heap_data) {
            let mut idx = 0;
            for num in heap_data.iter_mut() {
                *num += heap_data2[idx];
                idx += 1;
            }
        }
    }

    pub(super) fn subtract(&self, vector2: Vector<T, N>) -> Self
    where
        T: Sub<Output = T> + Copy,
    {
        if let (Some(stack_data), Some(stack_data2)) = (&self.stack_data, &vector2.stack_data) {
            let mut added_array: [MaybeUninit<T>; N] =
                unsafe { MaybeUninit::uninit().assume_init() };

            let mut idx = 0;
            for _ in added_array {
                added_array[idx] = MaybeUninit::new(stack_data[idx] - stack_data2[idx]);
                idx += 1;
            }

            return Vector {
                stack_data: Some(unsafe { added_array.as_ptr().cast::<[T; N]>().read() }),
                heap_data: None,
                stack_limit: self.stack_limit,
            };
        }

        if let (Some(heap_data), Some(heap_data2)) = (&self.heap_data, &vector2.heap_data) {
            let mut added_vec: Vec<T> = Vec::new();

            for idx in 0..N {
                added_vec.push(heap_data[idx] - heap_data2[idx]);
            }

            return Vector {
                stack_data: None,
                heap_data: Some(added_vec),
                stack_limit: self.stack_limit,
            };
        }

        unreachable!()
    }

    pub(super) fn subtract_mut(&mut self, vector2: Vector<T, N>)
    where
        T: SubAssign + Copy,
    {
        if let (Some(stack_data), Some(stack_data2)) = (&mut self.stack_data, vector2.stack_data) {
            let mut idx = 0;
            for num in stack_data.iter_mut() {
                *num -= stack_data2[idx];
                idx += 1;
            }
        }

        if let (Some(heap_data), Some(heap_data2)) = (&mut self.heap_data, &vector2.heap_data) {
            let mut idx = 0;
            for num in heap_data.iter_mut() {
                *num -= heap_data2[idx];
                idx += 1;
            }
        }
    }

    pub fn entrywise(&self, vector2: Vector<T, N>) -> Self
    where
        T: Mul<Output = T> + Copy,
    {
        if let (Some(stack_data), Some(stack_data2)) = (&self.stack_data, &vector2.stack_data) {
            let mut added_array: [MaybeUninit<T>; N] =
                unsafe { MaybeUninit::uninit().assume_init() };

            let mut idx = 0;
            for _ in added_array {
                added_array[idx] = MaybeUninit::new(stack_data[idx] * stack_data2[idx]);
                idx += 1;
            }

            return Vector {
                stack_data: Some(unsafe { added_array.as_ptr().cast::<[T; N]>().read() }),
                heap_data: None,
                stack_limit: self.stack_limit,
            };
        }

        if let (Some(heap_data), Some(heap_data2)) = (&self.heap_data, &vector2.heap_data) {
            let mut added_vec: Vec<T> = Vec::new();

            for idx in 0..N {
                added_vec.push(heap_data[idx] * heap_data2[idx]);
            }

            return Vector {
                stack_data: None,
                heap_data: Some(added_vec),
                stack_limit: self.stack_limit,
            };
        }

        unreachable!()
    }

    pub fn entrywise_mut(&mut self, vector2: Vector<T, N>)
    where
        T: MulAssign + Copy,
    {
        if let (Some(stack_data), Some(stack_data2)) = (&mut self.stack_data, &vector2.stack_data) {
            let mut idx = 0;
            for num in stack_data.iter_mut() {
                *num *= stack_data2[idx];
                idx += 1;
            }
        }

        if let (Some(heap_data), Some(heap_data2)) = (&mut self.heap_data, &vector2.heap_data) {
            let mut idx = 0;
            for num in heap_data.iter_mut() {
                *num *= heap_data2[idx];
                idx += 1;
            }
        }
    }
}

impl<T> Vector<T, 3> {
    pub fn cross(&self, vector2: Vector<T, 3>) -> Vector<T, 3>
    where
        T: FromPrimitive + Mul<Output = T> + Sub<Output = T> + Copy,
    {
        if let (Some(stack_data), Some(stack_data2)) = (&self.stack_data, &vector2.stack_data) {
            let mut added_array: [T; 3] = [FromPrimitive::from_u8(0).unwrap(); 3];

            added_array[0] = stack_data[1] * stack_data2[2] - stack_data[2] * stack_data2[1];
            added_array[1] = stack_data[2] * stack_data2[0] - stack_data[0] * stack_data2[2];
            added_array[2] = stack_data[0] * stack_data2[1] - stack_data[1] * stack_data2[0];

            return Vector {
                stack_data: Some(added_array),
                heap_data: None,
                stack_limit: self.stack_limit,
            };
        }

        // Too small for a Vec
        unreachable!()
    }

    pub fn cross_mut(&mut self, vector2: Vector<T, 3>)
    where
        T: FromPrimitive + Mul<Output = T> + Sub<Output = T> + Copy,
    {
        if let (Some(stack_data), Some(stack_data2)) = (&mut self.stack_data, vector2.stack_data) {
            let v1 = stack_data.clone();
            stack_data[0] = v1[1] * stack_data2[2] - v1[2] * stack_data2[1];
            stack_data[1] = v1[2] * stack_data2[0] - v1[0] * stack_data2[2];
            stack_data[2] = v1[0] * stack_data2[1] - v1[1] * stack_data2[0];
        }
    }
}
