use anyhow::{Result, anyhow};
use std::ops::{Add, AddAssign, Deref, Mul};

pub struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Self { data: data.into() }
    }
}

pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Add<Output = T> + AddAssign + Mul<Output = T> + Copy + Default,
{
    if a.len() != b.len() {
        // a.len => a.data.len() (Deref trait)
        return Err(anyhow!("dot product error"));
    }

    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }

    Ok(sum)
}
//     pub fn len(&self) -> usize {
//         self.data.len()
//     }

//     pub fn iter(&self) -> impl Iterator<Item = &T> {
//         self.data.iter()
//     }
// }

// impl<T> Index<usize> for Vector<T> {
//     type Output = T;
//     fn index(&self, index: usize) -> &Self::Output {
//         &self.data[index]
//     }
// }

// 转换成vec去运算
impl<T> Deref for Vector<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
