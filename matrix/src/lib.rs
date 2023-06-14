
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