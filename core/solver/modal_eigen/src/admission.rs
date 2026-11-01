//! The admission range of design §3.2 item 5 ("Admission range"): the modal
//! kernel admits a pencil only when every nonzero entry of the formed K and
//! M (and of the re-formed K̂ and M̂) has magnitude in [2⁻³⁰⁰, 2³⁰⁰], and
//! refuses any other by name (`MODAL_PENCIL_RANGE_UNSUPPORTED`). Physical
//! piping models in SI units lie far inside the range; it leaves headroom
//! so that products and sums of entries, scaled quantities and
//! double-double intermediates stay in binary64 range for every admitted
//! dimension. Each entry point checks it: `dense::solve_dense_pencil`,
//! `sturm::ShiftPencil`, `orthogonality::m_orthogonality`,
//! `modal_quantities::ModalBasis`, `reformation::ReformedPencil` (the
//! assembled re-formed matrices) and `reformation::formation_difference`
//! (the formed matrices).

use std::fmt;

use crate::matrix::SymmetricMatrix;

/// 2⁻³⁰⁰, the smallest admitted nonzero magnitude.
pub const ADMISSION_MIN: f64 = 4.909093465297727e-91;
/// 2³⁰⁰, the largest admitted magnitude.
pub const ADMISSION_MAX: f64 = 2.037035976334486e90;
/// The refusal's name.
pub const PENCIL_RANGE_UNSUPPORTED: &str = "MODAL_PENCIL_RANGE_UNSUPPORTED";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PencilMatrix {
    Stiffness,
    Mass,
}

/// A nonzero entry outside [2⁻³⁰⁰, 2³⁰⁰] in magnitude, or not finite.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PencilRangeUnsupported {
    pub matrix: PencilMatrix,
    pub row: usize,
    pub col: usize,
    pub value: f64,
}

impl PencilRangeUnsupported {
    pub fn code(&self) -> &'static str {
        PENCIL_RANGE_UNSUPPORTED
    }
}

impl fmt::Display for PencilRangeUnsupported {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self.matrix {
            PencilMatrix::Stiffness => "stiffness",
            PencilMatrix::Mass => "mass",
        };
        write!(
            f,
            "{}: {name} entry ({}, {}) = {:e} is outside the admitted magnitudes [2^-300, 2^300]",
            PENCIL_RANGE_UNSUPPORTED, self.row, self.col, self.value
        )
    }
}

/// Zero, or finite with magnitude in [2⁻³⁰⁰, 2³⁰⁰]. NaN is not admitted.
pub fn admitted(value: f64) -> bool {
    value == 0.0 || (value.abs() >= ADMISSION_MIN && value.abs() <= ADMISSION_MAX)
}

pub fn check_entry(
    matrix: PencilMatrix,
    row: usize,
    col: usize,
    value: f64,
) -> Result<(), PencilRangeUnsupported> {
    if admitted(value) {
        Ok(())
    } else {
        Err(PencilRangeUnsupported {
            matrix,
            row,
            col,
            value,
        })
    }
}

/// Every lower-triangle entry of a dense matrix, rows ascending.
pub fn check_dense(
    matrix: PencilMatrix,
    values: &SymmetricMatrix,
) -> Result<(), PencilRangeUnsupported> {
    let n = values.dimension();
    for row in 0..n {
        for col in 0..=row {
            check_entry(matrix, row, col, values.get(row, col))?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_range_is_inclusive_powers_of_two() {
        assert_eq!(ADMISSION_MIN, 2f64.powi(-300));
        assert_eq!(ADMISSION_MAX, 2f64.powi(300));
        for x in [0.0, ADMISSION_MIN, -ADMISSION_MAX, 1.0] {
            assert!(admitted(x), "{x:e}");
        }
        for x in [
            ADMISSION_MIN * (1.0 - f64::EPSILON),
            ADMISSION_MAX * (1.0 + f64::EPSILON),
            f64::NAN,
            f64::INFINITY,
            f64::from_bits(1),
        ] {
            assert!(!admitted(x), "{x:e}");
        }
    }
}
