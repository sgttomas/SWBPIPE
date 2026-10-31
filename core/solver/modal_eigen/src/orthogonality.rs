//! M-orthogonality of a set of modes (design §3.4, check 3):
//! max over i ≠ j of |φᵢᵀMφⱼ| / √(m_i m_j), with m_i = φᵢᵀMφᵢ, in
//! double-double. The measure is invariant to the shape normalization. The
//! tolerance it is judged against belongs to the caller.
//!
//! Check 3 evaluates it on the re-formed M̂
//! ([`m_orthogonality_reformed`], M̂φⱼ element by element); the measure on
//! a formed dense M ([`m_orthogonality`]) is kept for formed-pencil tests.

use std::error::Error;
use std::fmt;

use crate::dd::{Dd, DdAccumulator};
use crate::matrix::SymmetricMatrix;
use crate::reformation::ReformedPencil;

#[derive(Debug, Clone, PartialEq)]
pub enum OrthogonalityError {
    DimensionMismatch {
        mode: usize,
        expected: usize,
        actual: usize,
    },
    NonFinite {
        mode: usize,
        index: usize,
    },
    NonPositiveGeneralizedMass {
        mode: usize,
        value: f64,
    },
}

impl fmt::Display for OrthogonalityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DimensionMismatch {
                mode,
                expected,
                actual,
            } => write!(f, "mode {mode} has {actual} entries, expected {expected}"),
            Self::NonFinite { mode, index } => {
                write!(f, "mode {mode} entry {index} is not finite")
            }
            Self::NonPositiveGeneralizedMass { mode, value } => {
                write!(f, "mode {mode} has generalized mass {value:e}")
            }
        }
    }
}

impl Error for OrthogonalityError {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MOrthogonality {
    /// The largest normalized off-diagonal product; 0 with fewer than two
    /// modes.
    pub max_ratio: f64,
    /// The pair (i, j), i < j, attaining it; the first in (i, j) order on a
    /// tie. `None` with fewer than two modes.
    pub pair: Option<(usize, usize)>,
}

/// M φⱼ in double-double for every mode (rows ascending, exact products),
/// then each φᵢᵀ(Mφⱼ), so the cost is p·n² + p²·n.
pub fn m_orthogonality(
    mass: &SymmetricMatrix,
    vectors: &[&[f64]],
) -> Result<MOrthogonality, OrthogonalityError> {
    let n = mass.dimension();
    check_vectors(n, vectors)?;
    let products: Vec<Vec<Dd>> = vectors
        .iter()
        .map(|v| {
            (0..n)
                .map(|i| {
                    let mut row = DdAccumulator::new();
                    for (j, &vj) in v.iter().enumerate() {
                        row.add_product(mass.get(i, j), vj);
                    }
                    row.value()
                })
                .collect()
        })
        .collect();
    from_products(vectors, &products)
}

/// Check 3 on the re-formed M̂: M̂φⱼ element by element in double-double
/// (`ReformedPencil::mass_product`), then each φᵢᵀ(M̂φⱼ).
pub fn m_orthogonality_reformed(
    pencil: &ReformedPencil,
    vectors: &[&[f64]],
) -> Result<MOrthogonality, OrthogonalityError> {
    check_vectors(pencil.dimension(), vectors)?;
    let products: Vec<Vec<Dd>> = vectors.iter().map(|v| pencil.mass_product(v)).collect();
    from_products(vectors, &products)
}

fn check_vectors(n: usize, vectors: &[&[f64]]) -> Result<(), OrthogonalityError> {
    for (mode, v) in vectors.iter().enumerate() {
        if v.len() != n {
            return Err(OrthogonalityError::DimensionMismatch {
                mode,
                expected: n,
                actual: v.len(),
            });
        }
        if let Some(index) = v.iter().position(|x| !x.is_finite()) {
            return Err(OrthogonalityError::NonFinite { mode, index });
        }
    }
    Ok(())
}

fn from_products(
    vectors: &[&[f64]],
    products: &[Vec<Dd>],
) -> Result<MOrthogonality, OrthogonalityError> {
    let bilinear = |i: usize, j: usize| -> Dd {
        let mut acc = DdAccumulator::new();
        for (x, mv) in vectors[i].iter().zip(&products[j]) {
            acc.add_dd(mv.mul_f64(*x));
        }
        acc.value()
    };
    let mut generalized = Vec::with_capacity(vectors.len());
    for mode in 0..vectors.len() {
        let m = bilinear(mode, mode);
        if !(m.to_f64() > 0.0) {
            return Err(OrthogonalityError::NonPositiveGeneralizedMass {
                mode,
                value: m.to_f64(),
            });
        }
        generalized.push(m);
    }
    let mut max_ratio = 0.0;
    let mut pair = None;
    for i in 0..vectors.len() {
        for j in i + 1..vectors.len() {
            let scale = generalized[i].mul(generalized[j]).sqrt();
            let ratio = bilinear(i, j).abs().div(scale).to_f64();
            if pair.is_none() || ratio > max_ratio {
                max_ratio = ratio;
                pair = Some((i, j));
            }
        }
    }
    Ok(MOrthogonality { max_ratio, pair })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_cases() {
        let mass = SymmetricMatrix::from_diagonal(&[2.0, 3.0]).unwrap();
        let e1 = [1.0, 0.0];
        let e2 = [0.0, 5.0];
        let result = m_orthogonality(&mass, &[&e1, &e2]).unwrap();
        assert_eq!(
            result,
            MOrthogonality {
                max_ratio: 0.0,
                pair: Some((0, 1))
            }
        );
        // With M = I, φ₁ = (1, 0) and φ₂ = (1, 1): 1/√2, which the
        // double-double quotient rounds once to the binary64 nearest 1/√2.
        let identity = SymmetricMatrix::from_diagonal(&[1.0, 1.0]).unwrap();
        let a = [1.0, 0.0];
        let b = [1.0, 1.0];
        let c = [0.0, 1.0];
        let result = m_orthogonality(&identity, &[&a, &b, &c]).unwrap();
        assert_eq!(result.max_ratio, std::f64::consts::FRAC_1_SQRT_2);
        assert_eq!(result.pair, Some((0, 1)));
        // Invariant to scaling either vector.
        let b_scaled = [-4.0, -4.0];
        let scaled = m_orthogonality(&identity, &[&a, &b_scaled]).unwrap();
        assert_eq!(scaled.max_ratio, std::f64::consts::FRAC_1_SQRT_2);
        assert_eq!(
            m_orthogonality(&identity, &[&a]).unwrap(),
            MOrthogonality {
                max_ratio: 0.0,
                pair: None
            }
        );
    }

    #[test]
    fn refusals() {
        let singular = SymmetricMatrix::from_diagonal(&[1.0, 0.0]).unwrap();
        let null = [0.0, 1.0];
        assert_eq!(
            m_orthogonality(&singular, &[&null]),
            Err(OrthogonalityError::NonPositiveGeneralizedMass {
                mode: 0,
                value: 0.0
            })
        );
        let short = [1.0];
        assert!(matches!(
            m_orthogonality(&singular, &[&short]),
            Err(OrthogonalityError::DimensionMismatch { mode: 0, .. })
        ));
    }
}
