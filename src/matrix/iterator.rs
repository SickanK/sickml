use std::{iter::FromIterator, mem::MaybeUninit};

use crate::vector::Vector;

use super::Matrix;

impl<'a, T, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn into_iter(self) -> IntoIter<T, M, N> {
        IntoIter {
            data: self,
            current: 0,
            end: M,
        }
    }

    pub fn iter(&'a self) -> Iter<T, M, N> {
        Iter {
            data: &self.inner,
            current: 0,
            end: M,
        }
    }

    pub fn iter_mut(&'a mut self) -> IterMut<T, M, N> {
        IterMut {
            data: &mut self.inner,
            current: 0,
            end: M,
        }
    }
}

pub struct IntoIter<T, const M: usize, const N: usize> {
    data: Matrix<T, M, N>,
    current: usize,
    end: usize,
}

impl<T, const M: usize, const N: usize> Iterator for IntoIter<T, M, N>
where
    T: Copy,
{
    type Item = Vector<T, N>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            return None;
        } else {
            let current = self.current;
            self.current += 1;
            return Some(self.data.inner[current]);
        }
    }
}

impl<'a, T, const M: usize, const N: usize> IntoIterator for Matrix<T, M, N>
where
    T: Copy,
{
    type Item = Vector<T, N>;
    type IntoIter = IntoIter<T, M, N>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            data: self,
            current: 0,
            end: M,
        }
    }
}

pub struct Iter<'a, T, const M: usize, const N: usize> {
    data: &'a [Vector<T, N>; M],
    current: usize,
    end: usize,
}

impl<'a, T, const M: usize, const N: usize> IntoIterator for &'a Matrix<T, M, N>
where
    T: Copy,
{
    type Item = &'a Vector<T, N>;
    type IntoIter = Iter<'a, T, M, N>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            data: &self.inner,
            current: 0,
            end: M,
        }
    }
}

impl<'a, T, const M: usize, const N: usize> Iterator for Iter<'a, T, M, N>
where
    T: Copy,
{
    type Item = &'a Vector<T, N>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            return None;
        } else {
            let current = self.current;
            self.current += 1;
            return Some(&self.data[current]);
        }
    }
}

pub struct IterMut<'a, T, const M: usize, const N: usize> {
    data: &'a mut [Vector<T, N>; M],
    current: usize,
    end: usize,
}

impl<'a, T, const M: usize, const N: usize> IntoIterator for &'a mut Matrix<T, M, N>
where
    T: Copy,
{
    type Item = &'a mut Vector<T, N>;
    type IntoIter = IterMut<'a, T, M, N>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            data: &mut self.inner,
            current: 0,
            end: M,
        }
    }
}

impl<'a, T, const M: usize, const N: usize> Iterator for IterMut<'a, T, M, N>
where
    T: Copy,
{
    type Item = &'a mut Vector<T, N>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            return None;
        } else {
            let current = self.current;
            self.current += 1;
            let ptr = self.data.as_mut_ptr();
            return Some(unsafe { &mut *ptr.add(current) });
        }
    }
}

impl<T, const M: usize, const N: usize> FromIterator<Vector<T, N>> for Matrix<T, M, N> {
    fn from_iter<I: IntoIterator<Item = Vector<T, N>>>(iter: I) -> Matrix<T, M, N> {
        let mut uninit_collector: [MaybeUninit<Vector<T, N>>; M] =
            unsafe { MaybeUninit::uninit().assume_init() };

        let mut idx = 0;
        for item in iter {
            *&mut uninit_collector[idx] = MaybeUninit::new(item);
            idx += 1;
        }

        Matrix {
            inner: unsafe { uninit_collector.as_ptr().cast::<[Vector<T, N>; M]>().read() },
        }
    }
}
