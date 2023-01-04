use nalgebra::{Complex};
use num_traits::Zero;
use num_traits::One;


pub(crate) struct Matrix {
    pub rows: Vec<Vec<Complex<f32>>>
}

impl Matrix {
    pub fn new_zero(size: usize) -> Self {
        Self {
            rows: vec![vec![Complex::zero() ; size] ; size]
        }
    }

    pub fn new_identity(size: usize) -> Self{
        let mut rows:Vec<Vec<Complex<f32>>> = vec![];
        for i in 0..size{
            let mut row: Vec<Complex<f32>> = vec![Complex::zero() ; size];
            row[i] = Complex::one();
            rows.push(row);
        }
        Self {rows}
    }

    pub fn new(data: Vec<Vec<Complex<f32>>>) -> Self {
        Self {
            rows: data
        }
    }

    // Multiply by a scalar in place
    pub fn scalar_multiplication(&mut self, c: Complex<f32>){
        for i in 0..self.rows.len(){
            for j in 0..self.rows[i].len(){
                self.rows[i][j] *= c;
            }
        }
    }

    // Return copy of matrix after applying a scalar multiple
    pub fn scalar_multiplication_copy(&mut self, matrix: &Matrix, c: Complex<f32>) -> Matrix {
        let mut ret: Matrix = Matrix::new_zero(matrix.rows.len());
        for i in 0..matrix.rows.len(){
            for j in 0..matrix.rows[i].len(){
                ret.rows[i][j] = c*matrix.rows[i][j];
            }
        }
        ret
    }

    pub fn matrix_multiplication(&mut self, m2: Matrix) -> Matrix {
        let mut ret: Matrix = Matrix::new_zero(self.rows[0].len());
        for i in 0..m2.rows[0].len(){
            for j in 0..self.rows.len(){
                for k in 0..m2.rows.len(){
                    ret.rows[i][j] += self.rows[i][k] * m2.rows[k][j];
                }
            }
        }
        ret
    }
    
    pub fn matrix_addition(m1: Matrix, m2: Matrix) -> Result<Self, i32> {
        if m1.rows.len() != m2.rows.len(){
            return Err(-1);
        }
        let mut rows: Vec<Vec<Complex<f32>>> = vec![];
        for i in 0..m1.rows.len() {
            let mut row: Vec<Complex<f32>> = vec![Complex::zero() ; m1.rows.len()];
            for j in 0..m1.rows.len() {
                row[j] = m1.rows[i][j] + m2.rows[i][j];
            }
            rows.push(row);
        }
        Ok(Self {
            rows
        })
    }

    /**
    Tensor product of 2 matrices
    A tensor B where A is self and B is another matrix
    Example on two 2x2:
        [a11B a12B]
        [a21B a22B]
    where anm is a scalar at index(n, m) in matrix A and B is the 2nd matrix
     */
    pub fn tensor_product(&mut self, m2: &Matrix) -> Matrix {
        let num_columns: usize = self.rows[0].len()*m2.rows[0].len();
        let num_rows: usize = self.rows.len()*m2.rows.len();
        let mut new_matrix: Matrix = Matrix::new_zero(num_rows);

        for i in 0..self.rows.len(){
            for j in 0..self.rows[i].len(){
                // Current scalar multiple of m2 where it is being scaled by a factor of Aij
                let current: Matrix = self.scalar_multiplication_copy(&m2, self.rows[i][j]);
                for k in 0..current.rows.len(){
                    for l in 0..current.rows[k].len(){
                        new_matrix.rows[i*current.rows.len() + k][j*current.rows[k].len() + l] = current.rows[k][l];
                    }
                }
            }
        }
        new_matrix
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init(){
        let matrix: Matrix = Matrix::new_identity(2);

        for row in matrix.rows{
            for num in row{
                print!("Real: {}\n", num);
            }
        }
    }

    #[test]
    fn test_scalar_multiplication(){
        let data: Vec<Vec<Complex<f32>>> = vec![vec![Complex::one();2];2];
        let mut matrix:Matrix = Matrix::new(data);

        for i in 0..matrix.rows.len(){
            for j in 0..matrix.rows[i].len(){
                assert_eq!(Complex::one(), matrix.rows[i][j]);
                print!("Real: {}\n", matrix.rows[i][j].re);
                print!("Imaginary: {}\n", matrix.rows[i][j].im);
            }
        }

        matrix.scalar_multiplication(Complex::one()*3.);

        for i in 0..matrix.rows.len(){
            for j in 0..matrix.rows[i].len(){
                assert_eq!(Complex::one()*3., matrix.rows[i][j]);
                print!("Real: {}\n", matrix.rows[i][j].re);
                print!("Imaginary: {}\n", matrix.rows[i][j].im);
            }
        }
    }

    #[test]
    fn test_matrix_multiplication(){
        let data: Vec<Vec<Complex<f32>>> = vec![vec![Complex::one();2];2];
        let mut matrix:Matrix = Matrix::new(data);

        for i in 0..matrix.rows.len(){
            for j in 0..matrix.rows[i].len(){
                assert_eq!(Complex::one(), matrix.rows[i][j]);
                print!("Real: {}\n", matrix.rows[i][j].re);
                print!("Imaginary: {}\n", matrix.rows[i][j].im);
            }
        }

        let data2: Vec<Vec<Complex<f32>>> = vec![vec![Complex::one()*2.;2];2];
        let matrix2: Matrix = Matrix::new(data2);

        let new_matrix: Matrix = matrix.matrix_multiplication(matrix2);

        for i in 0..new_matrix.rows.len(){
            for j in 0..new_matrix.rows[i].len(){
                assert_eq!(Complex::one()*4., new_matrix.rows[i][j]);
                print!("Real: {}\n", new_matrix.rows[i][j].re);
                print!("Imaginary: {}\n", new_matrix.rows[i][j].im);
            }
        }
    }

    #[test]
    fn test_matrix_addition(){
        let mut matrix1: Matrix = Matrix::new_zero(2);
        matrix1.rows[0][0] = Complex::one();
        let mut matrix2: Matrix = Matrix::new_zero(2);
        matrix2.rows[1][1] = Complex::one();
        let matrix = Matrix::matrix_addition(matrix1, matrix2).unwrap();

        for i in 0..matrix.rows.len(){
            for j in 0..matrix.rows[i].len(){
                print!("Value: {}\n", matrix.rows[i][j]);
            }
        }
    }

    #[test]
    fn test_matrix_tensor(){
        let data: Vec<Vec<Complex<f32>>> = vec![vec![Complex::one(), Complex::one() * 2.], vec![Complex::one() * 2., Complex::one()]];
        let mut matrix:Matrix = Matrix::new(data);

        for i in 0..matrix.rows.len(){
            for j in 0..matrix.rows[i].len(){
                print!("Real: {}\n", matrix.rows[i][j].re);
                print!("Imaginary: {}\n", matrix.rows[i][j].im);
            }
        }
        print!("\n\n");
        let matrix2: Matrix = Matrix::new_identity(2);

        let mut new_matrix: Matrix = matrix.tensor_product(&matrix2);

        for i in 0..new_matrix.rows.len(){
            for j in 0..new_matrix.rows[i].len(){
                print!("Real: {}\n", new_matrix.rows[i][j]);
            }
        }

        let matrix3: Matrix = Matrix::new_identity(2);

        let new_matrix2: Matrix = new_matrix.tensor_product(&matrix3);
        print!("\n\n");
        for i in 0..new_matrix2.rows.len(){
            for j in 0..new_matrix2.rows[i].len(){
                print!("Real: {}\n", new_matrix2.rows[i][j]);
            }
        }
    }
}
