use nalgebra::DMatrix;
use crate::err::MatrixError;

#[derive(Clone)]
pub struct Matrix {
    pub data: DMatrix<f64>,
}

impl Matrix {
    pub fn from_flat(vec: Vec<f64>) -> Result<Self, MatrixError> {
        let n = (vec.len() as f64).sqrt() as usize;
        if n * n != vec.len() {
            return Err(MatrixError::NotPerfectSquare);
        }
        Ok(Self { data: DMatrix::from_row_slice(n, n, &vec) })
    }

    pub fn mul(&self, other: &Matrix) -> Result<Self, MatrixError> {
        if self.data.ncols() != other.data.nrows() {
            return Err(MatrixError::ShapeMismatch);
        }
        Ok(Self { data: &self.data * &other.data })
    }

    pub fn det(&self) -> f64 {
        self.data.determinant()
    }

    pub fn inv(&self) -> Result<Self, MatrixError> {
        self.data.clone().try_inverse()
            .map(|m| Self { data: m })
            .ok_or(MatrixError::NotInvertible)
    }

    pub fn transpose(&self) -> Self {
        Self { data: self.data.transpose() }
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.data.nrows() {
            for j in 0..self.data.ncols() {
                write!(f, "{:.3}\t", self.data[(i, j)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

