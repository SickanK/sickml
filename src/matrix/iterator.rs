use std::{iter::FromIterator, mem::MaybeUninit};

use crate::vector::Vector;

use super::Matrix;

impl<'a, T, const N: usize, const M: usize> Matrix<T, N, M> {
    pub fn into_iter(self) -> IntoIter<T, N, M> {
        IntoIter {
            data: self,
            current: 0,
            end: N,
        }
    }

    pub fn iter(&'a self) -> Iter<'a, T, N, M> {
        Iter {
            data: &self.inner,
            current: 0,
            end: N,
        }
    }

    pub fn iter_mut(&'a mut self) -> IterMut<'a, T, N, M> {
        IterMut {
            data: &mut self.inner,
            current: 0,
            end: N,
        }
    }
}

pub struct IntoIter<T, const N: usize, const M: usize> {
    data: Matrix<T, N, M>,
    current: usize,
    end: usize,
}

impl<T, const N: usize, const M: usize> Iterator for IntoIter<T, N, M>
where
    T: Copy,
{
    type Item = Vector<T, M>;

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

impl<T, const N: usize, const M: usize> IntoIterator for Matrix<T, N, M>
where
    T: Copy,
{
    type Item = Vector<T, M>;
    type IntoIter = IntoIter<T, N, M>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            data: self,
            current: 0,
            end: N,
        }
    }
}

pub struct Iter<'a, T, const N: usize, const M: usize> {
    data: &'a [Vector<T, M>; N],
    current: usize,
    end: usize,
}

impl<'a, T, const N: usize, const M: usize> IntoIterator for &'a Matrix<T, N, M>
where
    T: Copy,
{
    type Item = &'a Vector<T, M>;
    type IntoIter = Iter<'a, T, N, M>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            data: &self.inner,
            current: 0,
            end: N,
        }
    }
}

impl<'a, T, const N: usize, const M: usize> Iterator for Iter<'a, T, N, M>
where
    T: Copy,
{
    type Item = &'a Vector<T, M>;

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

pub struct IterMut<'a, T, const N: usize, const M: usize> {
    data: &'a mut [Vector<T, M>; N],
    current: usize,
    end: usize,
}

impl<'a, T, const N: usize, const M: usize> IntoIterator for &'a mut Matrix<T, N, M>
where
    T: Copy,
{
    type Item = &'a mut Vector<T, M>;
    type IntoIter = IterMut<'a, T, N, M>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            data: &mut self.inner,
            current: 0,
            end: N,
        }
    }
}

impl<'a, T, const N: usize, const M: usize> Iterator for IterMut<'a, T, N, M>
where
    T: Copy,
{
    type Item = &'a mut Vector<T, M>;

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

impl<T, const N: usize, const M: usize> FromIterator<Vector<T, M>> for Matrix<T, N, M> {
    fn from_iter<I: IntoIterator<Item = Vector<T, M>>>(iter: I) -> Matrix<T, N, M> {
        let mut uninit_collector: [MaybeUninit<Vector<T, M>>; N] =
            unsafe { MaybeUninit::uninit().assume_init() };

        let mut idx = 0;
        for item in iter {
            *&mut uninit_collector[idx] = MaybeUninit::new(item);
            idx += 1;
        }

        Matrix {
            inner: unsafe { uninit_collector.as_ptr().cast::<[Vector<T, M>; N]>().read() },
            transposed: None,
        }
    }
}
