use std::ops::{Index, IndexMut};

use rand::{distributions::Standard, prelude::Distribution};

use crate::vector::{
    heap_vector::{into_vec::IntoVec, HeapVector},
    inline_vector::{into_array::IntoArray, InlineVector},
};

pub mod heap_vector;
pub mod inline_vector;
pub mod iterator;
pub mod math;
pub mod math_ops;

#[derive(Debug, Clone)]
pub enum Vector<T, const N: usize> {
    Inline(InlineVector<T, N>),
    Heap(HeapVector<T, N>),
}

impl<T, const N: usize> Vector<T, N>
where
    T: Default,
{
    pub fn new(data: impl IntoArray<T, N> + IntoVec<T, N>) -> Self {
        if N < 5001 {
            Self::Inline(InlineVector::new(data))
        } else {
            Self::Heap(HeapVector::new(data))
        }
    }

    pub fn new_random() -> Self
    where
        T: Copy,
        Standard: Distribution<T>,
    {
        if N < 5001 {
            Self::Inline(InlineVector::new_random())
        } else {
            Self::Heap(HeapVector::new_random())
        }
    }

    pub fn inline(data: impl IntoArray<T, N>) -> Self {
        Self::Inline(InlineVector::new(data))
    }

    pub fn inline_random() -> Self
    where
        T: Copy,
        Standard: Distribution<T>,
    {
        Self::Inline(InlineVector::new_random())
    }

    pub fn heap(data: impl IntoVec<T, N>) -> Self {
        Self::Heap(HeapVector::new(data))
    }

    pub fn heap_random() -> Self
    where
        Standard: Distribution<T>,
    {
        Self::Heap(HeapVector::new_random())
    }
}

impl<T, const N: usize> Default for Vector<T, N>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self::Inline(InlineVector::default())
    }
}

impl<T, const N: usize> Index<usize> for Vector<T, N> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        match self {
            Self::Inline(inline_vector) => &inline_vector[idx],
            Self::Heap(heap_vector) => &heap_vector[idx],
        }
    }
}

impl<T, const N: usize> IndexMut<usize> for Vector<T, N> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match self {
            Self::Inline(inline_vector) => &mut inline_vector[idx],
            Self::Heap(heap_vector) => &mut heap_vector[idx],
        }
    }
}
