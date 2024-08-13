mod math;
use crate::math::matrix::Matrix;

fn main() {
    let mut matrix = Matrix::random(3, 3);
    let matrix_a: Matrix = Matrix {
        rows: 3,
        cols: 3,
        data: vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ],
    };
    println!("matrix: {:?}", matrix);
    println!("matrix_a: {:?}", matrix_a);
    matrix = matrix_a.subtract(&matrix);
    println!("matrix_a_after_subtract: {:?}", matrix);
}
