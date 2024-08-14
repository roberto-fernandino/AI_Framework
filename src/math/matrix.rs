use rand::thread_rng;
use rand::Rng;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn zeros(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows: rows,
            cols: cols,
            data: vec![vec![0.0; cols]; rows],
        }
    }

    pub fn random(rows: usize, cols: usize) -> Matrix {
        let mut rng = thread_rng();
        let mut matrix = Matrix {
            rows: rows,
            cols: cols,
            data: vec![vec![0.0; cols]; rows],
        };
        for row in 0..rows {
            for column in 0..cols {
                matrix.data[row][column] = rng.gen_range(-1.0..1.0);
            }
        }
        return matrix;
    }

    pub fn subtract(&self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Matrices must be the same size to subtract");
        }
        let mut result = Matrix::zeros(self.rows, self.cols);
        for row in 0..self.rows {
            for column in 0..self.cols {
                result.data[row][column] = self.data[row][column] - other.data[row][column]
            }
        }
        return result;
    }

    pub fn add(&self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Matrices must be the same size to add");
        }
        let mut result = Matrix::zeros(self.rows, self.cols);
        for row in 0..self.rows {
            for column in 0..self.cols {
                result.data[row][column] = self.data[row][column] + other.data[row][column]
            }
        }
        return result;
    }
    // A3x2 * B2x3 = A3x3
    pub fn multiply(&self, other: &Matrix) -> Matrix {
        if self.cols != other.rows {
            panic!("Matrice mus be the same size to multiply");
        }
        let mut result = Matrix::zeros(self.rows, other.cols);
        for row in 0..self.rows {
            for column in 0..other.cols {
                for i in 0..self.cols {
                    result.data[row][column] += self.data[row][i] * other.data[i][column];
                }
            }
        }
        return result;
    }
}
