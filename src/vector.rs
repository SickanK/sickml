pub mod into_vector_data;
pub mod iterator;
pub mod math;
use std::ops::{Index, IndexMut};

use into_vector_data::IntoVectorData;

use num::FromPrimitive;
use rand::Rng;
use rand::{distributions::Standard, prelude::Distribution};

#[derive(Debug)]
pub struct Vector<T, const N: usize> {
    stack_data: Option<[T; N]>,
    heap_data: Option<Vec<T>>,
    stack_limit: usize,
}

impl<T, const N: usize> Vector<T, N>
where
    T: FromPrimitive,
{
    pub fn new(data: impl IntoVectorData<T, N>) -> Self {
        let limit = 1500;
        if N < limit {
            Vector {
                stack_data: None,
                heap_data: Some(data.into_vec()),
                stack_limit: limit,
            }
        } else {
            Vector {
                stack_data: Some(data.into_array()),
                heap_data: None,
                stack_limit: limit,
            }
        }
    }

    pub fn new_limit(data: impl IntoVectorData<T, N>, limit: usize) -> Self {
        if N < limit {
            Vector {
                stack_data: None,
                heap_data: Some(data.into_vec()),
                stack_limit: limit,
            }
        } else {
            Vector {
                stack_data: Some(data.into_array()),
                heap_data: None,
                stack_limit: limit,
            }
        }
    }

    pub fn new_random() -> Vector<T, N>
    where
        Standard: Distribution<T>,
        T: FromPrimitive + Copy,
    {
        let mut rng = rand::thread_rng();
        let limit = 1500;
        if N < limit {
            let mut stack_data: [T; N] = [FromPrimitive::from_u8(0).unwrap(); N];

            for num in &mut stack_data {
                let random_num: T = rng.gen::<T>();
                *num = random_num;
            }

            Vector {
                stack_data: Some(stack_data),
                heap_data: None,
                stack_limit: limit,
            }
        } else {
            let mut heap_data: Vec<T> = Vec::with_capacity(N);

            for _ in 0..N {
                let random_num: T = rng.gen::<T>();
                heap_data.push(random_num)
            }

            Vector {
                stack_data: None,
                heap_data: Some(heap_data),
                stack_limit: limit,
            }
        }
    }

    pub fn new_random_limit(limit: usize) -> Vector<T, N>
    where
        Standard: Distribution<T>,
        T: FromPrimitive + Copy,
    {
        let mut rng = rand::thread_rng();
        if N < limit {
            let mut stack_data: [T; N] = [FromPrimitive::from_u8(0).unwrap(); N];

            for num in &mut stack_data {
                let random_num: T = rng.gen::<T>();
                *num = random_num;
            }

            Vector {
                stack_data: Some(stack_data),
                heap_data: None,
                stack_limit: limit,
            }
        } else {
            let mut heap_data: Vec<T> = Vec::with_capacity(N);

            for _ in 0..N {
                let random_num: T = rng.gen::<T>();
                heap_data.push(random_num)
            }

            Vector {
                stack_data: None,
                heap_data: Some(heap_data),
                stack_limit: limit,
            }
        }
    }
}

impl<T, const N: usize> Index<usize> for Vector<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if let Some(stack_data) = &self.stack_data {
            return &stack_data[index];
        }

        if let Some(heap_data) = &self.heap_data {
            return &heap_data[index];
        }

        unreachable!()
    }
}

impl<T, const N: usize> IndexMut<usize> for Vector<T, N>
where
    T: FromPrimitive,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if let Some(stack_data) = &mut self.stack_data {
            return &mut stack_data[index];
        }

        if let Some(heap_data) = &mut self.heap_data {
            return &mut heap_data[index];
        }

        unreachable!()
    }
}
