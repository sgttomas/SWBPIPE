//! §4.8's parity checks that are K6's (T3 D1 §4.8 items 1–3; K6 brief Scope
//! 7). No new tolerance: the DEC-053 basis is the harness's existing 1e-9
//! relative criterion, scaled by the dense solution's magnitude.

use open_pipe_stress_frame_kernel::structural::SparseStiffness;

/// Bitwise K: every stored entry of the pattern assembly against the dense
/// assembly's entry (bits), and every unstored dense entry against +0.0.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitwiseK {
    pub stored_entries: usize,
    pub stored_mismatches: usize,
    pub unstored_nonzero_bits: usize,
}

impl BitwiseK {
    pub fn equal(&self) -> bool {
        self.stored_mismatches == 0 && self.unstored_nonzero_bits == 0
    }
}

/// Compares the pattern assembly with a dense assembly of the same elements.
pub fn bitwise_k(sparse: &SparseStiffness, dense: &[Vec<f64>]) -> BitwiseK {
    let n = sparse.dimension();
    let mut result = BitwiseK {
        stored_entries: 0,
        stored_mismatches: usize::from(dense.len() != n),
        unstored_nonzero_bits: 0,
    };
    for (i, row) in dense.iter().enumerate().take(n) {
        if row.len() != n {
            result.stored_mismatches += 1;
            continue;
        }
        let mut stored = sparse.row(i).peekable();
        for (j, value) in row.iter().enumerate() {
            match stored.peek() {
                Some(&(col, s)) if col == j => {
                    result.stored_entries += 1;
                    if s.to_bits() != value.to_bits() {
                        result.stored_mismatches += 1;
                    }
                    stored.next();
                }
                _ => {
                    if value.to_bits() != 0 {
                        result.unstored_nonzero_bits += 1;
                    }
                }
            }
        }
    }
    result
}

/// The DEC-053 parity basis (`H/README.md:54`): max |u_s − u_d| over the
/// solution, and the dense solution's magnitude max |u_d|.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dec053Delta {
    pub max_abs_delta: f64,
    pub dense_scale: f64,
}

impl Dec053Delta {
    pub fn relative(&self) -> f64 {
        if self.dense_scale > 0.0 {
            self.max_abs_delta / self.dense_scale
        } else {
            0.0
        }
    }
}

pub fn dec053_delta(sparse: &[f64], dense: &[f64]) -> Dec053Delta {
    let max_abs_delta = sparse
        .iter()
        .zip(dense)
        .map(|(s, d)| (s - d).abs())
        .fold(0.0, f64::max);
    let dense_scale = dense.iter().map(|d| d.abs()).fold(0.0, f64::max);
    Dec053Delta {
        max_abs_delta,
        dense_scale,
    }
}
