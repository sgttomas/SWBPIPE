//! M-orthogonality of a set of modes (design §3.4, check 3):
//! max over i ≠ j of |φᵢᵀMφⱼ| / √(m_i m_j), with m_i = φᵢᵀMφᵢ, in
//! the bilinear forms summed exactly (`exact::ExactSum`: Mφⱼ as exact terms,
//! then φᵢᵀ(Mφⱼ)), so no contribution is lost before a cancellation, and the
//! generalized masses, their roots and the ratios in double-double with an
//! extended exponent (`dd::WideDd`). The largest ratio is published: an
//! overflow refuses, a nonzero value below the normal range is 0, flagged. A ratio that
//! is not finite is refused. The measure is invariant to the shape
//! normalization. The tolerance it is judged against belongs to the caller.
//!
//! Check 3 evaluates it on the re-formed M̂
//! ([`m_orthogonality_reformed`], M̂φⱼ element by element); the measure on
//! a formed dense M ([`m_orthogonality`]) is kept for formed-pencil tests.

use std::error::Error;
use std::fmt;

use crate::admission::{check_dense, PencilMatrix, PencilRangeUnsupported};
use crate::dd::WideDd;
use crate::exact::{ExactSum, Term};
use crate::matrix::SymmetricMatrix;
use crate::reformation::ReformedPencil;

#[derive(Debug, Clone, PartialEq)]
pub enum OrthogonalityError {
    /// A nonzero entry of M outside the admission range
    /// (`MODAL_PENCIL_RANGE_UNSUPPORTED`).
    PencilRangeUnsupported(PencilRangeUnsupported),
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
    /// The normalized product of the pair (i, j) is not finite.
    RatioOutOfRange {
        pair: (usize, usize),
        value: f64,
    },
}

impl fmt::Display for OrthogonalityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PencilRangeUnsupported(range) => write!(f, "{range}"),
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
            Self::RatioOutOfRange { pair, value } => write!(
                f,
                "the M-orthogonality ratio of modes {} and {} is {value:e}",
                pair.0, pair.1
            ),
        }
    }
}

impl Error for OrthogonalityError {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MOrthogonality {
    /// The largest normalized off-diagonal product; 0 with fewer than two
    /// modes.
    pub max_ratio: f64,
    /// The pair (i, j), i < j, attaining it (compared exactly, before
    /// rounding); the first in (i, j) order on a tie. `None` with fewer than
    /// two modes.
    pub pair: Option<(usize, usize)>,
    /// The largest ratio is nonzero but below the binary64 normal range and
    /// is published as 0 (design §3.2, admission range).
    pub max_ratio_below_normal_range: bool,
}

/// M φⱼ in double-double for every mode (rows ascending, exact products),
/// then each φᵢᵀ(Mφⱼ), so the cost is p·n² + p²·n.
pub fn m_orthogonality(
    mass: &SymmetricMatrix,
    vectors: &[&[f64]],
) -> Result<MOrthogonality, OrthogonalityError> {
    let n = mass.dimension();
    check_dense(PencilMatrix::Mass, mass).map_err(OrthogonalityError::PencilRangeUnsupported)?;
    check_vectors(n, vectors)?;
    let products: Vec<Vec<Vec<Term>>> = vectors
        .iter()
        .map(|v| {
            (0..n)
                .map(|i| {
                    let mut row = ExactSum::new();
                    for (j, &vj) in v.iter().enumerate() {
                        row.add_product(&[mass.get(i, j), vj]);
                    }
                    row.terms()
                })
                .collect()
        })
        .collect();
    from_products(vectors, &products)
}

/// Check 3 on the re-formed M̂: M̂φⱼ element by element, exactly
/// (`ReformedPencil::mass_product_exact`), then each φᵢᵀ(M̂φⱼ).
pub fn m_orthogonality_reformed(
    pencil: &ReformedPencil,
    vectors: &[&[f64]],
) -> Result<MOrthogonality, OrthogonalityError> {
    check_vectors(pencil.dimension(), vectors)?;
    let products: Vec<Vec<Vec<Term>>> = vectors
        .iter()
        .map(|v| pencil.mass_product_exact(v).iter().map(ExactSum::terms).collect())
        .collect();
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

/// The bilinear forms, generalized masses and ratios in `WideDd`, so a
/// generalized mass below or above the binary64 range is still exact to
/// double-double and only the ratio is rounded; it must be finite, and
/// normal or exactly zero.
fn from_products(
    vectors: &[&[f64]],
    products: &[Vec<Vec<Term>>],
) -> Result<MOrthogonality, OrthogonalityError> {
    // φᵢᵀ(Mφⱼ) summed exactly from the exact terms of Mφⱼ, then rounded
    // once: no contribution is lost before the cancellation that a pair of
    // orthogonal modes produces.
    let bilinear = |i: usize, j: usize| -> WideDd {
        let mut sum = ExactSum::new();
        for (&x, terms) in vectors[i].iter().zip(&products[j]) {
            sum.add_terms_product(terms, &[x]);
        }
        sum.to_wide()
    };
    let mut roots = Vec::with_capacity(vectors.len());
    for mode in 0..vectors.len() {
        let m = bilinear(mode, mode);
        if !m.is_positive() {
            return Err(OrthogonalityError::NonPositiveGeneralizedMass {
                mode,
                value: m.to_f64(),
            });
        }
        roots.push(m.sqrt());
    }
    // The largest ratio is chosen on the exact values, then published: an
    // overflow or NaN refuses, a value below the normal range is 0, flagged.
    let mut best = WideDd::ZERO;
    let mut pair = None;
    for i in 0..vectors.len() {
        for j in i + 1..vectors.len() {
            let exact = bilinear(i, j).abs().div(roots[i].mul(roots[j]));
            if !exact.is_finite() {
                return Err(OrthogonalityError::RatioOutOfRange {
                    pair: (i, j),
                    value: exact.to_f64(),
                });
            }
            if pair.is_none() || exact.sub(best).is_positive() {
                best = exact;
                pair = Some((i, j));
            }
        }
    }
    let Some((max_ratio, max_ratio_below_normal_range)) = best.to_published() else {
        return Err(OrthogonalityError::RatioOutOfRange {
            pair: pair.expect("a ratio was formed"),
            value: best.to_f64(),
        });
    };
    Ok(MOrthogonality {
        max_ratio,
        pair,
        max_ratio_below_normal_range,
    })
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
                pair: Some((0, 1)),
                max_ratio_below_normal_range: false,
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
                pair: None,
                max_ratio_below_normal_range: false,
            }
        );
    }

    /// Independent review of PR #33, finding 5: scalar M = 10^±200 and two
    /// copies of the vector (1). The ratio is exactly 1, but m₁·m₂
    /// overflowed or underflowed before its square root (NaN).
    #[test]
    fn the_denominator_does_not_leave_the_range() {
        let one = [1.0];
        // Since review round 3 these masses are outside the admission range
        // and refused at entry.
        for s in [1e200, 1e-200, f64::MAX, f64::MIN_POSITIVE] {
            let mass = SymmetricMatrix::from_diagonal(&[s]).unwrap();
            assert!(matches!(
                m_orthogonality(&mass, &[&one, &one]),
                Err(OrthogonalityError::PencilRangeUnsupported(_))
            ));
        }
        let p2 = |e: i32| 2f64.powi(e);
        for s in [p2(300), p2(-300)] {
            let mass = SymmetricMatrix::from_diagonal(&[s]).unwrap();
            let result = m_orthogonality(&mass, &[&one, &one]).unwrap();
            assert_eq!(result.max_ratio, 1.0, "M = {s:e}");
            assert_eq!(result.pair, Some((0, 1)));
        }
        // An indefinite M with |φ₁ᵀMφ₂| far above √(m₁m₂): within the
        // admission range the ratio, 2⁶⁰⁰, is finite and reported.
        let indefinite =
            SymmetricMatrix::from_rows(&[vec![p2(-300), p2(300)], vec![p2(300), p2(-300)]])
                .unwrap();
        let (e1, e2) = ([1.0, 0.0], [0.0, 1.0]);
        assert_eq!(
            m_orthogonality(&indefinite, &[&e1, &e2]).unwrap().max_ratio,
            p2(600)
        );
    }

    /// Review round 3: the orthogonality witness M = diag(2⁻¹⁰²², 2⁻¹⁰²², 0)
    /// is outside the admission range. Within it, M = diag(2⁻²⁹⁰, 2⁻²⁹⁰, 0)
    /// with φ₁ = (1, 0, 0) and φ₂ = 2⁻⁵⁰³·(4, 3, 2⁵⁰³): m₂ = 25·2⁻¹²⁹⁶ lies
    /// below binary64 (it underflowed in double-double), and the true ratio
    /// is 4·2⁻⁷⁹³ / (5·2⁻⁷⁹³) = 0.8.
    #[test]
    fn an_in_range_ratio_with_a_generalized_mass_below_binary64_is_exact() {
        let p2 = |e: i32| 2f64.powi(e);
        let mass = SymmetricMatrix::from_diagonal(&[p2(-290), p2(-290), 0.0]).unwrap();
        let a = [1.0, 0.0, 0.0];
        let b = [4.0 * p2(-503), 3.0 * p2(-503), 1.0];
        let result = m_orthogonality(&mass, &[&a, &b]).unwrap();
        assert_eq!(result.max_ratio, 0.8);
        assert_eq!(result.pair, Some((0, 1)));
    }

    /// The published-value rule: M = I₂, φ₁ = (1, 0), φ₂ = (2⁻¹⁰⁶⁰, 1). The
    /// largest ratio, 2⁻¹⁰⁶⁰, is nonzero but below the normal range: it was
    /// refused as out of range and is now published as 0 with its flag.
    #[test]
    fn a_ratio_below_the_normal_range_is_a_flagged_zero() {
        let identity = SymmetricMatrix::from_diagonal(&[1.0, 1.0]).unwrap();
        let a = [1.0, 0.0];
        let b = [f64::from_bits(1u64 << 14), 1.0];
        let result = m_orthogonality(&identity, &[&a, &b]).unwrap();
        assert_eq!(result.max_ratio, 0.0);
        assert!(result.max_ratio_below_normal_range);
        assert_eq!(result.pair, Some((0, 1)));
        // An exactly orthogonal pair is an unflagged zero.
        let c = [0.0, 1.0];
        let exact = m_orthogonality(&identity, &[&a, &c]).unwrap();
        assert!(!exact.max_ratio_below_normal_range);
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
