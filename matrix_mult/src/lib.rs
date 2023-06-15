use std::ops::{Mul,Add};

#[derive(Debug, Clone)]
struct Matrix<T>(Vec<Vec<T>>);

impl<T> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        if let Some(row) = self.0.first() {
            row.len()
        } else {
            0
        }
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Option<&[T]> {
        self.0.get(n).map(|row| row.as_slice())
    }

    pub fn col(&self, n: usize) -> Option<Vec<&T>> {
        if n >= self.number_of_cols() {
            return None;
        }

        Some(self.0.iter().map(|row| &row[n]).collect())
    }
}

impl<T> Mul for Matrix<T>
where
    T: Mul<Output = T> + Add<Output = T> + Clone + Default,
{
    type Output = Option<Matrix<T>>;

    fn mul(self, rhs: Matrix<T>) -> Option<Matrix<T>> {
        if self.number_of_cols() != rhs.number_of_rows() {
            return None;
        }

        let rows = self.number_of_rows();
        let cols = rhs.number_of_cols();
        let inner_len = self.number_of_cols();

        let mut result = vec![vec![T::default(); cols]; rows];

        for i in 0..rows {
            for j in 0..cols {
                let mut sum = T::default();
                for k in 0..inner_len {
                    sum = sum + self.0[i][k].clone() * rhs.0[k][j].clone();
                }
                result[i][j] = sum;
            }
        }

        Some(Matrix(result))
    }
}


