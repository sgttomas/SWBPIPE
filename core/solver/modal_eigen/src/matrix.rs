//! Dense symmetric matrix input.

use std::error::Error;
use std::fmt;

/// A dense symmetric n×n matrix in row-major storage.
///
/// Only the lower triangle is read: for an index pair the entry used is
/// always `data[max * n + min]`, as `sparse_direct`'s profile storage does, so
/// last-ulp asymmetry left by assembly cannot make the operation order or the
/// result depend on which triangle a routine happens to read.
#[derive(Debug, Clone, PartialEq)]
pub struct SymmetricMatrix {
    n: usize,
    data: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatrixError {
    /// A row of the input does not have `n` entries, or the flat storage is
    /// not `n * n` long.
    NotSquare { rows: usize, row: usize, len: usize },
    /// A lower-triangle entry is NaN or infinite.
    NonFinite { row: usize, col: usize },
}

impl fmt::Display for MatrixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotSquare { rows, row, len } => write!(
                f,
                "matrix is not square: {rows} rows, row {row} has {len} entries"
            ),
            Self::NonFinite { row, col } => {
                write!(f, "matrix entry ({row}, {col}) is not finite")
            }
        }
    }
}

impl Error for MatrixError {}

impl SymmetricMatrix {
    /// Builds the matrix from rows. Every row must have `rows.len()` entries
    /// and every lower-triangle entry must be finite.
    pub fn from_rows(rows: &[Vec<f64>]) -> Result<Self, MatrixError> {
        let n = rows.len();
        let mut data = Vec::with_capacity(n * n);
        for (i, row) in rows.iter().enumerate() {
            if row.len() != n {
                return Err(MatrixError::NotSquare {
                    rows: n,
                    row: i,
                    len: row.len(),
                });
            }
            data.extend_from_slice(row);
        }
        Self::from_row_major(n, data)
    }

    /// Builds the matrix from `n * n` row-major entries.
    pub fn from_row_major(n: usize, data: Vec<f64>) -> Result<Self, MatrixError> {
        if data.len() != n * n {
            return Err(MatrixError::NotSquare {
                rows: n,
                row: 0,
                len: data.len(),
            });
        }
        for i in 0..n {
            for j in 0..=i {
                if !data[i * n + j].is_finite() {
                    return Err(MatrixError::NonFinite { row: i, col: j });
                }
            }
        }
        Ok(Self { n, data })
    }

    /// Builds a diagonal matrix.
    pub fn from_diagonal(diagonal: &[f64]) -> Result<Self, MatrixError> {
        let n = diagonal.len();
        let mut data = vec![0.0; n * n];
        for (i, &value) in diagonal.iter().enumerate() {
            data[i * n + i] = value;
        }
        Self::from_row_major(n, data)
    }

    pub fn dimension(&self) -> usize {
        self.n
    }

    /// The symmetric entry (i, j), read from the lower triangle.
    pub fn get(&self, i: usize, j: usize) -> f64 {
        let (row, col) = if i >= j { (i, j) } else { (j, i) };
        self.data[row * self.n + col]
    }

    /// Row `i` of the lower triangle, columns `0..=i`.
    pub(crate) fn lower_row(&self, i: usize) -> &[f64] {
        &self.data[i * self.n..i * self.n + i + 1]
    }
}
