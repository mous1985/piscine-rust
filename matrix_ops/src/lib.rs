
pub trait Scalar:Copy
{
	fn zero() -> Self;
	fn one() -> Self;
}
impl Scalar for u32 {
	fn zero() -> Self{0}
	fn one() -> Self {1}
}
impl Scalar for u64 {   

	fn zero() -> Self {0}
	fn one() -> Self {1}
}
impl Scalar for i32 {
	fn zero() -> Self {0}
	fn one() -> Self {1}
}
impl Scalar for i64 {
	fn zero() -> Self {0}
	fn one() -> Self {1}
}
impl Scalar for f32 {
	fn zero() -> Self {0.0}
	fn one() -> Self {1.0}
}
impl Scalar for f64 {
	fn zero() -> Self {0.0}
	fn one() -> Self {1.0}
}

use std::ops::{ Add, Sub };
#[derive(Debug, Clone)]

pub struct Matrix<T>(pub Vec<Vec<T>>);   
impl<T: Scalar + std::cmp::PartialEq> PartialEq for Matrix<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0   
    }
}
impl<T: Scalar + std::cmp::PartialEq> Eq for Matrix<T> {}
impl<T: Scalar> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![vec![T::zero()]])   
    }
    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero(); col]; row])   
        }
    pub fn identity(n: usize) -> Matrix<T> {
        let mut result = Matrix::zero(n, n); 
        for i in 0..n {
            result.0[i][i] = T::one();   
        }
        result   
    }
}
impl<T: Scalar + std::ops::Add<Output = T>> Add for Matrix<T> {
    type Output = Option<Matrix<T>>;
    fn add(self, other: Matrix<T>) -> Option<Matrix<T>> {
        if self.0.len() != other.0.len() || 
        self.0[0].len() != other.0[0].len() {//M1(i,j) et M2(n,m) =>i=n et j=m pour pouvoir additionner    
            return None; 
        }
        let mut result = Matrix::zero(self.0.len(), self.0[0].len());
        for i in 0..self.0.len() {
            for j in 0..self.0[0].len() {
                result.0[i][j] = self.0[i][j] + other.0[i][j];
            }
        }
        Some(result)
    }
}
impl<T: Scalar + std::ops::Sub<Output = T>> Sub for Matrix<T> {
    type Output = Option<Matrix<T>>;
    fn sub(self, other: Matrix<T>) -> Option<Matrix<T>> {
        if self.0.len() != other.0.len() ||
         self.0[0].len() != other.0[0].len() {
            return None; 
        }
        let mut result = Matrix::zero(self.0.len(), self.0[0].len());
        for i in 0..self.0.len() {
            for j in 0..self.0[0].len() {
                result.0[i][j] = self.0[i][j] - other.0[i][j];
            }
        }
        Some(result)
    }
}