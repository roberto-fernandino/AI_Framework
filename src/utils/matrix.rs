use rand::thread_rng;
use rand::Rng;
use std::fmt::Debug;
use std::vec;

#[derive(Debug, Clone)]
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
                matrix.data[row][column] = rng.gen::<f64>() * 2.0e6 - 1.0e6;
            }
        }
        return matrix;
    }

    pub fn from(data: Vec<Vec<f64>>) -> Matrix {
        let rows = data.len();
        let cols = data[0].len();
        let matrix = Matrix {
            rows: rows,
            cols: cols,
            data: data,
        };
        matrix
    }

    pub fn map<F>(&self, func: F) -> Matrix
    where
        F: Fn(f64) -> f64,
    {
        let mut result = Matrix::zeros(self.rows, self.cols);
        for row in 0..self.rows {
            for column in 0..self.cols {
                result.data[row][column] = func(self.data[row][column]);
            }
        }
        result
    }

    pub fn random_less_one_to_one(rows: usize, cols: usize) -> Matrix {
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
            panic!(
                "Matrices must be the same size to add {}x{} + {}x{}",
                self.rows, self.cols, other.rows, other.cols
            );
        }
        let mut result = Matrix::zeros(self.rows, self.cols);
        for row in 0..self.rows {
            for column in 0..self.cols {
                result.data[row][column] = self.data[row][column] + other.data[row][column]
            }
        }
        return result;
    }

    pub fn dot_multiply(&self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Matrices must be the same size to dot multiply");
        }
        let mut result = Matrix::zeros(self.rows, self.cols);
        for row in 0..self.rows {
            for column in 0..self.cols {
                result.data[row][column] = self.data[row][column] * other.data[row][column]
            }
        }
        return result;
    }

    pub fn multiply(&self, other: &Matrix) -> Matrix {
        if self.cols != other.rows {
            panic!(
                "Matrice mus be the same size to multiply A_cols_{:?} * B_rows_{:?}",
                self.cols, other.rows
            ); // A¹² * B²³ = C¹³
        }
        let mut result = Matrix::zeros(self.rows, other.cols);
        for row in 0..self.rows {
            // rows of A
            for column in 0..other.cols {
                let mut sum = 0.0;
                // columns of B
                for i in 0..self.cols {
                    // columns of A
                    // A[row][i] * B[i][column] = C[row][column]
                    sum += self.data[row][i] * other.data[i][column];
                }
                result.data[row][column] = sum;
            }
        }
        result
    }

    pub fn transpose(&mut self) -> Matrix {
        let mut res = Matrix::zeros(self.cols, self.rows);
        for row in 0..self.rows {
            for column in 0..self.cols {
                res.data[column][row] = self.data[row][column];
            }
        }
        res
    }

    pub fn represent(&self) -> String {
        // Represent a matrix as a string
        // Nice visual representation
        let mut result = String::new();
        for row in 0..self.rows {
            for column in 0..self.cols {
                if column == 0 {
                    result.push_str("[");
                }
                if column == self.cols - 1 {
                    if row == self.rows - 1 {
                        result.push_str(&format!(
                            "{}] {}x{}",
                            self.data[row][column], self.rows, self.cols
                        ));
                    } else {
                        result.push_str(&format!("{}]", self.data[row][column]));
                    }
                } else {
                    result.push_str(&format!("{} ", self.data[row][column]));
                }
            }
        }
        return result;
    }
}
