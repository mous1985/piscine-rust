
#[derive(Debug, PartialEq, Eq)]
pub struct Matrix((i32, i32), (i32, i32));

pub fn transpose(m: Matrix) -> Matrix {
    let ((a, b), (c, d)) = m;
    Matrix((a, c), (b, d))
}

fn main() {
    let matrix = Matrix((1, 3), (4, 5));
    println!("Original matrix {:?}", matrix);
    println!("Transpose matrix {:?}", transpose(matrix));
}
