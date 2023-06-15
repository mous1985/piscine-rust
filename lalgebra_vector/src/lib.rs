#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

use std::ops::Add;

pub trait Scalar: Add<Output = Self> + Clone {}
impl<T> Scalar for T where T: Add<Output = Self> + Clone {}

impl<T: Scalar> Add for Vector<T> {
    type Output = Option<Vector<T>>;

    fn add(self, other: Vector<T>) -> Option<Vector<T>> {
        if self.0.len() != other.0.len() {
            None
        } else {
            let sum: Vec<T> = self
                .0
                .iter()
                .zip(other.0.iter())
                .map(|(&ref a, &ref b)| a.clone() + b.clone())
                .collect();
            Some(Vector(sum))
        }
    }
}

impl<T: Scalar + std::ops::Mul + std::iter::Sum<<T as std::ops::Mul>::Output>> Vector<T> {
    pub fn new() -> Self {
        Vector(Vec::new())
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            None
        } else {
            let dot_product: T = self
                .0
                .iter()
                .zip(other.0.iter())
                .map(|(&ref a, &ref b)| a.clone() * b.clone())
                .sum();
            Some(dot_product)
        }
    }
}
