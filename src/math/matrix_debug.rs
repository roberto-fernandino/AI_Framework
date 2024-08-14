use super::matrix::Matrix;
use std::{thread, time};

pub fn debug_zeros(rows: usize, cols: usize) -> Matrix {
    println!("Criando matriz de zeros {}x{}", rows, cols);
    let matrix = Matrix::zeros(rows, cols);
    println!("Matriz de zeros criada:\n{:?}", matrix);
    thread::sleep(time::Duration::from_secs(1));
    matrix
}

pub fn debug_random(rows: usize, cols: usize) -> Matrix {
    println!("Criando matriz aleatória {}x{}", rows, cols);
    let matrix = Matrix::random(rows, cols);
    println!("Matriz aleatória criada:\n{:?}", matrix);
    thread::sleep(time::Duration::from_secs(1));
    matrix
}

pub fn debug_subtract(a: &Matrix, b: &Matrix) -> Matrix {
    println!("Subtraindo matrizes:");
    println!("A:\n{:?}", a);
    println!("B:\n{:?}", b);
    let result = a.subtract(b);
    println!("Resultado da subtração:\n{:?}", result);
    thread::sleep(time::Duration::from_secs(1));
    result
}

pub fn debug_add(a: &Matrix, b: &Matrix) -> Matrix {
    println!("Adicionando matrizes:");
    println!("A:\n{:?}", a);
    println!("B:\n{:?}", b);
    let result = a.add(b);
    println!("Resultado da adição:\n{:?}", result);
    thread::sleep(time::Duration::from_secs(1));
    result
}

pub fn debug_multiply(a: &Matrix, b: &Matrix) -> Matrix {
    println!("Multiplicando matrizes:");
    println!("A:\n{:?}", a);
    println!("B:\n{:?}", b);
    let result = a.multiply(b);
    println!("Resultado da multiplicação:\n{:?}", result);
    thread::sleep(time::Duration::from_secs(1));
    result
}

pub fn run_debug_operations() {
    println!("Iniciando operações de debug da matriz...");
    
    let matrix1 = debug_zeros(3, 3);
    let matrix2 = debug_random(3, 3);
    
    debug_subtract(&matrix2, &matrix1);
    debug_add(&matrix1, &matrix2);
    debug_multiply(&matrix1, &matrix2);
    
    println!("Operações de debug da matriz concluídas.");
}
