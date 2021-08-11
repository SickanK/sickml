use super::Vector;

use std::{iter::FromIterator, mem::MaybeUninit};
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

            if let Some(stack_data) = &self.data.stack_data {
                return Some(stack_data[current]);
            }

            if let Some(heap_data) = &self.data.heap_data {
                return Some(heap_data[current]);
            }

            unreachable!()
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
            if let Some(stack_data) = &self.data.stack_data {
                return Some(&stack_data[current]);
            }

            if let Some(heap_data) = &self.data.heap_data {
                return Some(&heap_data[current]);
            }

            unreachable!()
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

            if let Some(stack_data) = &mut self.data.stack_data {
                let ptr = stack_data.as_mut_ptr();
                return Some(unsafe { &mut *ptr.add(current) });
            }

            if let Some(heap_data) = &mut self.data.heap_data {
                let ptr = heap_data.as_mut_ptr();
                return Some(unsafe { &mut *ptr.add(current) });
            }

            unreachable!()
        }
    }
}

impl<T, const N: usize> FromIterator<T> for Vector<T, N> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Vector<T, N> {
        let limit = 1500;

        if N < limit {
            let mut uninit_collector: [MaybeUninit<T>; N] =
                unsafe { MaybeUninit::uninit().assume_init() };

            let mut idx = 0;
            for item in iter {
                uninit_collector[idx] = MaybeUninit::new(item);
                idx += 1;
            }

            Vector {
                stack_data: Some(unsafe { uninit_collector.as_ptr().cast::<[T; N]>().read() }),
                heap_data: None,
                stack_limit: limit,
            }
        } else {
            let mut collector: Vec<T> = Vec::new();

            for item in iter {
                collector.push(item);
            }

            Vector {
                stack_data: None,
                heap_data: Some(collector),
                stack_limit: limit,
            }
        }
    }
}
