use std::collections::HashMap;
use crate::{matrix::Matrix, err::MatrixError};

pub struct Evaluator {
    vars: HashMap<String, Matrix>,
}

impl Evaluator {
    pub fn new() -> Self {
        Self { vars: HashMap::new() }
    }

    pub fn eval_line(&mut self, input: &str) -> Result<Option<String>, MatrixError> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() { return Ok(None); }

        // determinant
        if parts[0] == "det" && parts.len() == 2 {
            let m = self.get(parts[1])?;
            return Ok(Some(format!("det({}) = {:.6}", parts[1], m.det())));
        }

        // inverse
        if parts[0] == "inv" && parts.len() == 2 {
            let m = self.get(parts[1])?.inv()?;
            return Ok(Some(format!("{}", m)));
        }

        // transpose
        if parts.len() == 1 && parts[0].ends_with("'") {
            let name = &parts[0][..parts[0].len()-1];
            let m = self.get(name)?.transpose();
            return Ok(Some(format!("{}", m)));
        }

        // trace
        if parts[0] == "tr" && parts.len() == 2 {
            let m = self.get(parts[1])?;
            return Ok(Some(format!("tr({}) = {:.6}", parts[1], m.trace()?)));
        }

        // eigenvalues
        if parts[0] == "eig" && parts.len() == 2 {
            let m = self.get(parts[1])?;
            let eigenvals = m.eigenvalues()?;
            let vals_str = eigenvals.iter()
                .map(|v| format!("{:.6}", v))
                .collect::<Vec<_>>()
                .join(", ");
            return Ok(Some(format!("eigenvalues of {}: [{}]", parts[1], vals_str)));
        }

        // eigenvectors
        if parts[0] == "eigvec" && parts.len() == 2 {
            let m = self.get(parts[1])?;
            let eigvecs = m.eigenvectors()?;
            return Ok(Some(format!("eigenvectors of {}:\n{}", parts[1], eigvecs)));
        }

        // matrix multiplication: c = a * b
        if parts.len() == 3 && parts[1] == "=" {
            let name = parts[0];
            let rhs = parts[2];
            let val = self.get(rhs)?.clone();
            self.vars.insert(name.to_string(), val);
            return Ok(None);
        }

        if parts.len() == 5 && parts[1] == "=" && parts[3] == "*" {
            let name = parts[0];
            let a = self.get(parts[2])?;
            let b = self.get(parts[4])?;
            let result = a.mul(b)?;
            self.vars.insert(name.to_string(), result.clone());
            return Ok(Some(format!("{}", result)));
        }

        // define matrix: a 1 2 3 4 5 6 7 8 9
        let first = parts[0];
        let nums: Result<Vec<f64>, _> = parts[1..].iter().map(|s| s.parse::<f64>()).collect();
        if let Ok(v) = nums {
            let m = Matrix::from_flat(v)?;
            self.vars.insert(first.to_string(), m.clone());
            return Ok(Some(format!("{}", m)));
        }

        Err(MatrixError::ParseError)
    }

    fn get(&self, name: &str) -> Result<&Matrix, MatrixError> {
        self.vars.get(name).ok_or(MatrixError::UndefinedVariable)
    }
}

