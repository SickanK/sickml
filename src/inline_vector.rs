pub mod into_array;
pub mod iterator;

pub mod math;
pub mod math_ops;

use std::ops::{Index, IndexMut};

use self::into_array::IntoArray;
use rand::{distributions::Standard, prelude::Distribution, Rng};

#[derive(Debug, Clone)]
pub struct InlineVector<T, const N: usize> {
    pub data: [T; N],
}

impl<T, const N: usize> InlineVector<T, N> {
    pub fn new(data: impl IntoArray<T, N>) -> Self {
        InlineVector {
            data: data.into_array(),
        }
    }

    pub fn new_random() -> Self
    where
        T: Default + Copy,
        Standard: Distribution<T>,
    {
        let mut rng = rand::thread_rng();
        let mut random_data: [T; N] = [T::default(); N];

        for num in &mut random_data {
            let random_num: T = rng.gen::<T>();
            *num = random_num;
        }

        InlineVector { data: random_data }
    }
}

impl<T, const N: usize> Default for InlineVector<T, N>
where
    T: Default + Copy,
{
    fn default() -> Self {
        InlineVector {
            data: [T::default(); N],
        }
    }
}

impl<T, const N: usize> Index<usize> for InlineVector<T, N> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.data[idx]
    }
}

impl<T, const N: usize> IndexMut<usize> for InlineVector<T, N> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.data[idx]
    }
}
