use std::fmt;

#[derive(Debug)]
pub enum MatrixError {
    NotPerfectSquare,
    UndefinedVariable,
    ParseError,
    ShapeMismatch,
    NotInvertible,
    NotSquare,
}

impl fmt::Display for MatrixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for MatrixError {}

