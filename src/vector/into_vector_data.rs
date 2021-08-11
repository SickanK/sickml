use std::convert::TryInto;

pub trait IntoVectorData<T, const N: usize> {
    fn into_array(self) -> [T; N];
    fn into_vec(self) -> Vec<T>;
}

impl<T, const N: usize> IntoVectorData<T, N> for Vec<T> {
    fn into_array(self) -> [T; N] {
        self.try_into().unwrap_or_else(|v: Vec<T>| {
            panic!("Expected a Vec of length {} but it was {}", N, v.len())
        })
    }

    fn into_vec(self) -> Vec<T> {
        self
    }
}

impl<T, const N: usize> IntoVectorData<T, N> for [T; N]
where
    T: Copy,
{
    fn into_array(self) -> [T; N] {
        self
    }

    fn into_vec(self) -> Vec<T> {
        let mut converted_vec: Vec<T> = Vec::with_capacity(N);
        for t in self.iter() {
            converted_vec.push(*t);
        }

        converted_vec
    }
}
