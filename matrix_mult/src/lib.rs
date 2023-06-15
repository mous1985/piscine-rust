use std::ops::Mul;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T> Matrix<T>
where
    T: Clone + Default + std::ops::Mul<Output = T> + std::ops::AddAssign<T>,
{
    pub fn new(matrix: Vec<Vec<T>>) -> Self {
        Self(matrix)
    }

    pub fn number_of_cols(&self) -> usize {
        self.0.first().map_or(0, |row| row.len())
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0.get(n).map_or(Vec::new(), |row| row.clone())
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        if n >= self.number_of_cols() {
            return Vec::new();
        }
        self.0
            .iter()
            .filter_map(|row| row.get(n).cloned())
            .collect()
    }
}

impl<T> Mul for Matrix<T>
where
    T: Clone + Default + std::ops::Mul<Output = T> + std::ops::AddAssign<T>,
{
    type Output = Option<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.number_of_cols() != rhs.number_of_rows() {
            return None;
        }
        let (p, m) = (self.number_of_rows(), rhs.number_of_cols());
        let mut matrix = vec![vec![T::default(); m]; p];
        for i in 0..p {
            for j in 0..m {
                for k in 0..self.number_of_cols() {
                    matrix[i][j] += self.0[i][k].clone() * rhs.0[k][j].clone();
                }
            }
        }
        Some(Self(matrix))
    }
}