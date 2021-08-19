use crate::vector::{heap_vector::HeapVector, inline_vector::InlineVector};

use super::Vector;

use std::iter::FromIterator;
impl<'a, T, const N: usize> Vector<T, N> {
    pub fn into_iter(self) -> IntoIter<T, N> {
        IntoIter {
            data: self,
            current: 0,
            end: N,
        }
    }

    pub fn iter(&'a self) -> Iter<'a, T, N> {
        Iter {
            data: self,
            current: 0,
            end: N,
        }
    }

    pub fn iter_mut(&'a mut self) -> IterMut<'a, T, N> {
        IterMut {
            data: self,
            current: 0,
            end: N,
        }
    }
}

pub struct IntoIter<T, const N: usize> {
    data: Vector<T, N>,
    current: usize,
    end: usize,
}

impl<T, const N: usize> Iterator for IntoIter<T, N>
where
    T: Copy,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            return None;
        } else {
            let current = self.current;
            self.current += 1;

            match &self.data {
                Vector::Inline(inline_vector) => Some(inline_vector[current]),
                Vector::Heap(heap_vector) => Some(heap_vector[current]),
            }
        }
    }
}

impl<T, const N: usize> IntoIterator for Vector<T, N>
where
    T: Copy,
{
    type Item = T;
    type IntoIter = IntoIter<Self::Item, N>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            data: self,
            current: 0,
            end: N,
        }
    }
}

pub struct Iter<'a, T, const N: usize> {
    data: &'a Vector<T, N>,
    current: usize,
    end: usize,
}

impl<'a, T, const N: usize> IntoIterator for &'a Vector<T, N>
where
    T: Copy,
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T, N>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            data: self,
            current: 0,
            end: N,
        }
    }
}

impl<'a, T, const N: usize> Iterator for Iter<'a, T, N>
where
    T: Copy,
{
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            return None;
        } else {
            let current = self.current;
            self.current += 1;

            match self.data {
                Vector::Inline(inline_vector) => Some(&inline_vector[current]),
                Vector::Heap(heap_vector) => Some(&heap_vector[current]),
            }
        }
    }
}

pub struct IterMut<'a, T, const N: usize> {
    data: &'a mut Vector<T, N>,
    current: usize,
    end: usize,
}

impl<'a, T, const N: usize> IntoIterator for &'a mut Vector<T, N>
where
    T: Copy,
{
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T, N>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            data: self,
            current: 0,
            end: N,
        }
    }
}

impl<'a, T, const N: usize> Iterator for IterMut<'a, T, N>
where
    T: Copy,
{
    type Item = &'a mut T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            return None;
        } else {
            let current = self.current;
            self.current += 1;

            match self.data {
                Vector::Inline(inline_vector) => {
                    let ptr = inline_vector.data.as_mut_ptr();
                    return Some(unsafe { &mut *ptr.add(current) });
                }
                Vector::Heap(heap_vector) => {
                    let ptr = heap_vector.data.as_mut_ptr();
                    return Some(unsafe { &mut *ptr.add(current) });
                }
            }
        }
    }
}

impl<T, const N: usize> FromIterator<T> for Vector<T, N>
where
    T: Default + Copy,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Vector<T, N> {
        let limit = 5001;

        if N < limit {
            let mut collector: [T; N] = [T::default(); N];

            let mut idx = 0;
            for item in iter {
                collector[idx] = item;
                idx += 1;
            }

            Vector::Inline(InlineVector::new(collector))
        } else {
            let mut collector: Vec<T> = Vec::with_capacity(N);

            for item in iter {
                collector.push(item);
            }

            Vector::Heap(HeapVector::new(collector))
        }
    }
}
