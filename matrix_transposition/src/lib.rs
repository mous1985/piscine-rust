
#[derive(Debug, PartialEq, Eq)]
pub struct Matrix((i32, i32), (i32, i32));

pub fn transpose(m: Matrix) -> Matrix {
    let ((a, b), (c, d)) = m;
    Matrix((a, c), (b, d))
}


