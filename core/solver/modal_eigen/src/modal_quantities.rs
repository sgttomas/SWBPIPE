//! Shape normalization and participation quantities (design §1.5, with D-v).
//!
//! A mode is published in exactly one of two normalization cases:
//! - (a) unit maximum translation: the largest-magnitude free translational
//!   component is +1;
//! - (b) unit maximum rotation, exactly when the mode has no translation:
//!   t ≤ τ_t·ℓ·θ, with t and θ the largest free translational and rotational
//!   magnitudes and ℓ the model's bounding-box diagonal.
//!
//! Case (b) sets no condition on participation: through the coupling to
//! restrained DOFs in p_k, a shape with no free translation can carry a large
//! effective mass (a simply supported span's bending pair), and it is
//! published as computed. Ties for the largest component go to the lowest
//! global DOF index (6·node + DOF).
//!
//! Units: in case (a) the shape's translations are m/m and its rotations
//! rad/m, m is in kg and Γ is dimensionless; in case (b) the rotations are
//! rad/rad and translations m/rad, m is in kg·m²·rad⁻² and Γ in rad/m. The
//! effective mass is in kg in both cases. The numbers carry no unit; the
//! caller labels them.
//!
//! Participation uses the caller's free-DOF excitation p_k (design §1.5:
//! p_k = (M R_k)_f, which includes consistent-mass coupling to restrained
//! DOFs) and the direction's total mass T_k = R_kᵀ M R_k. This module does
//! not form either. Per direction k:
//! Γ_k = φᵀp_k / m, m_eff,k = (φᵀp_k)² / m and the fraction m_eff,k / T_k,
//! with m = φᵀ M_ff φ. Each is computed from the published vector in
//! double-double with an extended exponent (`dd::WideDd`): every product,
//! sum and quotient, (φᵀp_k)² and m·T_k included, keeps its exponent
//! separately, so no intermediate underflows or overflows, and each
//! quantity is rounded to binary64 once. A published quantity that is not
//! finite refuses the mode (`QuantityOutOfRange`).

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use crate::dd::{DdAccumulator, WideDd};
use crate::matrix::SymmetricMatrix;

/// The six DOFs of a node, in global axes, in global DOF order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dof {
    Ux,
    Uy,
    Uz,
    Rx,
    Ry,
    Rz,
}

impl Dof {
    pub fn index(self) -> usize {
        self as usize
    }

    pub fn is_translation(self) -> bool {
        self.index() < 3
    }
}

/// A free DOF of the reduced system: free index i is `free_dofs[i]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FreeDof {
    pub node: usize,
    pub dof: Dof,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalizationCase {
    UnitMaxTranslation,
    UnitMaxRotation,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ModalQuantityError {
    DimensionMismatch {
        name: &'static str,
        expected: usize,
        actual: usize,
    },
    NonFinite {
        name: &'static str,
        index: usize,
    },
    InvalidParameter {
        name: &'static str,
    },
    /// The same (node, DOF) appears twice in the free-DOF map.
    DuplicateDof {
        index: usize,
    },
    /// The mode shape has no nonzero component.
    ZeroShape {
        mode: usize,
    },
    NonPositiveGeneralizedMass {
        mode: usize,
        value: f64,
    },
    /// A published quantity is not finite: `name`, in direction
    /// `direction` (`None` for the generalized mass).
    QuantityOutOfRange {
        mode: usize,
        name: &'static str,
        direction: Option<usize>,
    },
}

impl fmt::Display for ModalQuantityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DimensionMismatch {
                name,
                expected,
                actual,
            } => write!(f, "`{name}` has {actual} entries, expected {expected}"),
            Self::NonFinite { name, index } => write!(f, "`{name}`[{index}] is not finite"),
            Self::InvalidParameter { name } => write!(f, "invalid parameter `{name}`"),
            Self::DuplicateDof { index } => write!(f, "free DOF {index} is listed twice"),
            Self::ZeroShape { mode } => write!(f, "mode {mode} has a zero shape"),
            Self::NonPositiveGeneralizedMass { mode, value } => {
                write!(f, "mode {mode} has generalized mass {value:e}")
            }
            Self::QuantityOutOfRange {
                mode,
                name,
                direction,
            } => match direction {
                Some(k) => write!(f, "mode {mode}: `{name}` in direction {k} is not finite"),
                None => write!(f, "mode {mode}: `{name}` is not finite"),
            },
        }
    }
}

impl Error for ModalQuantityError {}

/// The model data every mode is normalized against.
#[derive(Debug, Clone)]
pub struct ModalBasis<'a> {
    mass: &'a SymmetricMatrix,
    free_dofs: &'a [FreeDof],
    global_index: Vec<usize>,
    excitation: [&'a [f64]; 3],
    total_mass: [f64; 3],
    bounding_box_diagonal: f64,
    tau_t: f64,
}

impl<'a> ModalBasis<'a> {
    /// `mass` is M_ff; `excitation[k]` is p_k over the free DOFs and
    /// `total_mass[k]` is T_k, for k = X, Y, Z. T_k = 0 is admitted only with
    /// p_k = 0 (no mass in that direction): Γ_k and m_eff,k are then 0 and
    /// the fraction is undefined. `tau_t` is the design's τ_t, an
    /// algorithm parameter supplied by the caller.
    pub fn new(
        mass: &'a SymmetricMatrix,
        free_dofs: &'a [FreeDof],
        excitation: [&'a [f64]; 3],
        total_mass: [f64; 3],
        bounding_box_diagonal: f64,
        tau_t: f64,
    ) -> Result<Self, ModalQuantityError> {
        let n = mass.dimension();
        if free_dofs.len() != n {
            return Err(ModalQuantityError::DimensionMismatch {
                name: "free_dofs",
                expected: n,
                actual: free_dofs.len(),
            });
        }
        let mut seen = BTreeSet::new();
        let mut global_index = Vec::with_capacity(n);
        for (i, free) in free_dofs.iter().enumerate() {
            let index = free
                .node
                .checked_mul(6)
                .and_then(|base| base.checked_add(free.dof.index()))
                .ok_or(ModalQuantityError::InvalidParameter { name: "free_dofs" })?;
            if !seen.insert(index) {
                return Err(ModalQuantityError::DuplicateDof { index: i });
            }
            global_index.push(index);
        }
        for k in 0..3 {
            let p = excitation[k];
            if p.len() != n {
                return Err(ModalQuantityError::DimensionMismatch {
                    name: "excitation",
                    expected: n,
                    actual: p.len(),
                });
            }
            if let Some(index) = p.iter().position(|v| !v.is_finite()) {
                return Err(ModalQuantityError::NonFinite {
                    name: "excitation",
                    index,
                });
            }
            let t = total_mass[k];
            if !(t.is_finite() && t >= 0.0) || (t == 0.0 && p.iter().any(|&v| v != 0.0)) {
                return Err(ModalQuantityError::InvalidParameter { name: "total_mass" });
            }
        }
        if !(bounding_box_diagonal.is_finite() && bounding_box_diagonal > 0.0) {
            return Err(ModalQuantityError::InvalidParameter {
                name: "bounding_box_diagonal",
            });
        }
        if !(tau_t.is_finite() && tau_t >= 0.0) {
            return Err(ModalQuantityError::InvalidParameter { name: "tau_t" });
        }
        Ok(Self {
            mass,
            free_dofs,
            global_index,
            excitation,
            total_mass,
            bounding_box_diagonal,
            tau_t,
        })
    }

    pub fn dimension(&self) -> usize {
        self.mass.dimension()
    }
}

/// A normalized mode and its participation quantities, all from the
/// published `shape`.
#[derive(Debug, Clone, PartialEq)]
pub struct NormalizedMode {
    pub case: NormalizationCase,
    /// The published shape over the free DOFs.
    pub shape: Vec<f64>,
    /// Free index of the normalizing component, which is exactly +1.
    pub reference_dof: usize,
    /// Largest free translational and rotational magnitudes of the input
    /// vector, before normalization.
    pub max_translation: f64,
    pub max_rotation: f64,
    /// m = φᵀ M_ff φ: kg in case (a), kg·m²·rad⁻² in case (b).
    pub generalized_mass: f64,
    /// φᵀ p_k, per direction.
    pub excitation_product: [f64; 3],
    /// Γ_k = φᵀp_k / m: dimensionless in case (a), rad/m in case (b).
    pub participation: [f64; 3],
    /// m_eff,k = (φᵀp_k)² / m, kg; 0 where T_k = 0 (p_k is then 0).
    pub effective_mass: [f64; 3],
    /// m_eff,k / T_k; `None` (undefined) where T_k = 0.
    pub effective_mass_fraction: [Option<f64>; 3],
}

/// xᵀ M y in [`WideDd`]: each row sum Σ_j M_ij y_j, then multiplied by
/// x_i, rows in ascending order.
fn mass_bilinear(mass: &SymmetricMatrix, x: &[f64], y: &[f64]) -> WideDd {
    let n = mass.dimension();
    let mut total = WideDd::ZERO;
    for i in 0..n {
        let mut row = WideDd::ZERO;
        for (j, &yj) in y.iter().enumerate() {
            row = row.add(WideDd::from_f64(mass.get(i, j)).mul_f64(yj));
        }
        total = total.add(row.mul_f64(x[i]));
    }
    total
}

/// Σ a_i b_i in [`WideDd`], ascending.
fn wide_dot(a: &[f64], b: &[f64]) -> WideDd {
    a.iter().zip(b).fold(WideDd::ZERO, |sum, (&x, &y)| {
        sum.add(WideDd::from_f64(x).mul_f64(y))
    })
}

/// Largest magnitude among the free DOFs selected by `select`, with ties to
/// the lowest global DOF index; `None` if every selected component is zero
/// or none is selected.
fn largest_component(
    basis: &ModalBasis<'_>,
    phi: &[f64],
    select: impl Fn(Dof) -> bool,
) -> (f64, Option<usize>) {
    let mut best = 0.0;
    let mut best_index: Option<usize> = None;
    for (i, free) in basis.free_dofs.iter().enumerate() {
        if !select(free.dof) {
            continue;
        }
        let magnitude = phi[i].abs();
        let better = match best_index {
            None => magnitude > 0.0,
            Some(b) => {
                magnitude > best
                    || (magnitude == best && basis.global_index[i] < basis.global_index[b])
            }
        };
        if better {
            best = magnitude;
            best_index = Some(i);
        }
    }
    (best, best_index)
}

/// Normalizes the computed vector `phi` of mode `mode` (the index used in
/// errors) and forms its participation quantities.
pub fn normalize_mode(
    mode: usize,
    phi: &[f64],
    basis: &ModalBasis<'_>,
) -> Result<NormalizedMode, ModalQuantityError> {
    let n = basis.dimension();
    if phi.len() != n {
        return Err(ModalQuantityError::DimensionMismatch {
            name: "phi",
            expected: n,
            actual: phi.len(),
        });
    }
    if let Some(index) = phi.iter().position(|v| !v.is_finite()) {
        return Err(ModalQuantityError::NonFinite { name: "phi", index });
    }
    let (t, translation_ref) = largest_component(basis, phi, Dof::is_translation);
    let (theta, rotation_ref) = largest_component(basis, phi, |dof| !dof.is_translation());
    let no_translation = t <= basis.tau_t * basis.bounding_box_diagonal * theta;
    let (case, reference) = if no_translation {
        (NormalizationCase::UnitMaxRotation, rotation_ref)
    } else {
        (NormalizationCase::UnitMaxTranslation, translation_ref)
    };
    let reference = reference.ok_or(ModalQuantityError::ZeroShape { mode })?;
    let pivot = phi[reference];
    let shape: Vec<f64> = phi.iter().map(|&v| v / pivot).collect();

    let m = mass_bilinear(basis.mass, &shape, &shape);
    let generalized_mass = m.to_f64();
    // Finiteness first: an overflowing m is out of range, not non-positive.
    if !generalized_mass.is_finite() {
        return Err(ModalQuantityError::QuantityOutOfRange {
            mode,
            name: "generalized_mass",
            direction: None,
        });
    }
    if !(generalized_mass > 0.0) {
        return Err(ModalQuantityError::NonPositiveGeneralizedMass {
            mode,
            value: generalized_mass,
        });
    }
    let mut excitation_product = [0.0; 3];
    let mut participation = [0.0; 3];
    let mut effective_mass = [0.0; 3];
    let mut effective_mass_fraction = [None; 3];
    for k in 0..3 {
        let finite = |name: &'static str, value: f64| {
            if value.is_finite() {
                Ok(value)
            } else {
                Err(ModalQuantityError::QuantityOutOfRange {
                    mode,
                    name,
                    direction: Some(k),
                })
            }
        };
        let q = wide_dot(&shape, basis.excitation[k]);
        let q_squared = q.mul(q);
        excitation_product[k] = finite("excitation_product", q.to_f64())?;
        participation[k] = finite("participation", q.div(m).to_f64())?;
        effective_mass[k] = finite("effective_mass", q_squared.div(m).to_f64())?;
        if basis.total_mass[k] > 0.0 {
            let fraction = q_squared.div(m.mul_f64(basis.total_mass[k]));
            effective_mass_fraction[k] =
                Some(finite("effective_mass_fraction", fraction.to_f64())?);
        }
    }
    Ok(NormalizedMode {
        case,
        shape,
        reference_dof: reference,
        max_translation: t,
        max_rotation: theta,
        generalized_mass,
        excitation_product,
        participation,
        effective_mass,
        effective_mass_fraction,
    })
}

/// Running sums of the published effective-mass fractions, in mode order,
/// accumulated in double-double and rounded once each.
pub fn cumulative_effective_mass_fractions(modes: &[NormalizedMode]) -> Vec<[Option<f64>; 3]> {
    let mut sums = [DdAccumulator::new(); 3];
    let mut out = Vec::with_capacity(modes.len());
    for mode in modes {
        let mut row = [None; 3];
        for k in 0..3 {
            if let Some(fraction) = mode.effective_mass_fraction[k] {
                sums[k].add_f64(fraction);
                row[k] = Some(sums[k].value().to_f64());
            }
        }
        out.push(row);
    }
    out
}

/// f (Hz), ω (rad/s) and T (s) of an eigenvalue λ = ω² (rad²/s²).
pub fn frequency_from_eigenvalue(lambda: f64) -> (f64, f64, f64) {
    let omega = lambda.sqrt();
    let f = omega / (2.0 * std::f64::consts::PI);
    (f, omega, 1.0 / f)
}

#[cfg(test)]
mod tests {
    use super::*;

    const U: f64 = f64::EPSILON / 2.0;
    // Test-local τ_t; the production value is the design's (M1b).
    const TAU_T: f64 = 1e-6;

    fn free(node: usize, dof: Dof) -> FreeDof {
        FreeDof { node, dof }
    }

    #[test]
    fn fixed_axial_element_uses_the_support_coupling() {
        // One two-node axial element, node 0 fixed, consistent mass
        // (m/6)[[2, 1], [1, 2]] with m = 6: M_ff = 2, M_fc = 1, so
        // p = M_ff·1 + M_fc·1 = 3 and T = 1ᵀM1 = 6. φ = (1): case (a),
        // m = 2, Γ = 3/2, m_eff = 9/2 and the fraction 3/4, each exact in
        // binary64, so equality is the expectation. With M_ff·r alone Γ would
        // be 1 and the static check below would give m/(3k).
        let mass = SymmetricMatrix::from_diagonal(&[2.0]).unwrap();
        let dofs = [free(1, Dof::Ux)];
        let p = [3.0];
        let zero = [0.0];
        let basis = ModalBasis::new(
            &mass,
            &dofs,
            [&p, &zero, &zero],
            [6.0, 6.0, 6.0],
            2.0,
            TAU_T,
        )
        .unwrap();
        let mode = normalize_mode(0, &[0.25], &basis).unwrap();
        assert_eq!(mode.case, NormalizationCase::UnitMaxTranslation);
        assert_eq!(mode.shape, vec![1.0]);
        assert_eq!(mode.generalized_mass, 2.0);
        assert_eq!(mode.participation, [1.5, 0.0, 0.0]);
        assert_eq!(mode.effective_mass, [4.5, 0.0, 0.0]);
        assert_eq!(
            mode.effective_mass_fraction,
            [Some(0.75), Some(0.0), Some(0.0)]
        );
        // Static response to a unit support acceleration: u/A = Γφ/ω² with
        // ω² = k/M_ff, which must equal the exact m/(2k) for the element.
        let k = 5.0;
        let omega2 = k / mode.generalized_mass;
        assert_eq!(mode.participation[0] / omega2, 6.0 / (2.0 * k));
    }

    fn twelve_dof_basis_parts() -> (SymmetricMatrix, Vec<FreeDof>, [Vec<f64>; 3]) {
        let mut dofs = Vec::new();
        let mut diag = Vec::new();
        for node in 0..2 {
            for (d, dof) in [Dof::Ux, Dof::Uy, Dof::Uz, Dof::Rx, Dof::Ry, Dof::Rz]
                .into_iter()
                .enumerate()
            {
                dofs.push(free(node + 1, dof));
                diag.push(if d < 3 { 2.0 + node as f64 } else { 0.5 });
            }
        }
        let mass = SymmetricMatrix::from_diagonal(&diag).unwrap();
        let mut p = [vec![0.0; 12], vec![0.0; 12], vec![0.0; 12]];
        for (i, f) in dofs.iter().enumerate() {
            if f.dof.is_translation() {
                p[f.dof.index()][i] = diag[i];
            }
        }
        (mass, dofs, p)
    }

    #[test]
    fn case_a_ties_go_to_the_lowest_global_index() {
        // Free order deliberately differs from global order.
        let mass = SymmetricMatrix::from_diagonal(&[1.0, 1.0, 1.0, 1.0]).unwrap();
        let dofs = [
            free(1, Dof::Uy),
            free(0, Dof::Uz),
            free(0, Dof::Rx),
            free(1, Dof::Ux),
        ];
        let z = [0.0; 4];
        let basis = ModalBasis::new(&mass, &dofs, [&z, &z, &z], [0.0; 3], 3.0, TAU_T).unwrap();
        // Global indices 7, 2, 3, 6; the tie |−2| = |2| goes to global 2.
        let mode = normalize_mode(0, &[-2.0, 2.0, 0.1, 1.0], &basis).unwrap();
        assert_eq!(mode.case, NormalizationCase::UnitMaxTranslation);
        assert_eq!(mode.reference_dof, 1);
        assert_eq!(mode.shape, vec![-1.0, 1.0, 0.05, 0.5]);
        // The sign is fixed by the reference component, not the input's.
        let flipped = normalize_mode(0, &[2.0, -2.0, -0.1, -1.0], &basis).unwrap();
        assert_eq!(flipped.shape, mode.shape);
        assert_eq!(mode.effective_mass_fraction, [None; 3]);
    }

    #[test]
    fn case_b_tie_and_threshold() {
        let (mass, dofs, p) = twelve_dof_basis_parts();
        let basis =
            ModalBasis::new(&mass, &dofs, [&p[0], &p[1], &p[2]], [10.0; 3], 2.0, TAU_T).unwrap();
        // Rotations only, tie between node 2 Rx (free 9, global 15) and node 1
        // Rz (free 5, global 11): global 11 wins.
        let mut phi = vec![0.0; 12];
        phi[9] = 0.5;
        phi[5] = -0.5;
        phi[4] = 0.25;
        let mode = normalize_mode(3, &phi, &basis).unwrap();
        assert_eq!(mode.case, NormalizationCase::UnitMaxRotation);
        assert_eq!(mode.reference_dof, 5);
        assert_eq!(mode.shape[5], 1.0);
        assert_eq!(mode.shape[9], -1.0);
        assert_eq!(mode.effective_mass, [0.0; 3]);
        // A translation just above τ_t·ℓ·θ makes it case (a).
        phi[0] = 2.0 * TAU_T * 2.0 * 0.5;
        assert_eq!(
            normalize_mode(3, &phi, &basis).unwrap().case,
            NormalizationCase::UnitMaxTranslation
        );
        // At exactly the bound (inclusive) it is case (b).
        phi[0] = TAU_T * 2.0 * 0.5;
        assert_eq!(
            normalize_mode(3, &phi, &basis).unwrap().case,
            NormalizationCase::UnitMaxRotation
        );
        assert_eq!(
            normalize_mode(3, &[0.0; 12], &basis),
            Err(ModalQuantityError::ZeroShape { mode: 3 })
        );
    }

    #[test]
    fn case_b_publishes_large_participation_through_support_coupling() {
        // One Euler–Bernoulli element, simply supported: both v restrained,
        // both θz free. Consistent mass (mL/420)[[156, 22L, 54, −13L],
        // [22L, 4L², 13L, −3L²], [54, 13L, 156, −22L], [−13L, −3L², −22L, 4L²]]
        // over (v₁, θ₁, v₂, θ₂), with L = 1 and mL/420 = 1 so every entry
        // is an integer. M_ff = [[4, −3], [−3, 4]]; p_y = M_ff·0 + M_fc·1 =
        // (22 + 13, −13 − 22) = (35, −35); T_y = 420. The symmetric bending
        // shape θ = (1, −1) has no free translation (case (b)), m = 14,
        // φᵀp_y = 70, Γ = 5, m_eff = 350 and the fraction 5/6, all exact
        // except the fraction, which is rounded once. With M_ff·r alone the
        // participation would be 0.
        let mass = SymmetricMatrix::from_rows(&[vec![4.0, -3.0], vec![-3.0, 4.0]]).unwrap();
        let dofs = [free(0, Dof::Rz), free(1, Dof::Rz)];
        let py = [35.0, -35.0];
        let z = [0.0, 0.0];
        let basis =
            ModalBasis::new(&mass, &dofs, [&z, &py, &z], [420.0, 420.0, 0.0], 1.0, TAU_T).unwrap();
        // The input sign is arbitrary; the tie |θ₁| = |θ₂| goes to node 0.
        let mode = normalize_mode(0, &[-0.5, 0.5], &basis).unwrap();
        assert_eq!(mode.case, NormalizationCase::UnitMaxRotation);
        assert_eq!(mode.shape, vec![1.0, -1.0]);
        assert_eq!(mode.generalized_mass, 14.0);
        assert_eq!(mode.excitation_product, [0.0, 70.0, 0.0]);
        assert_eq!(mode.participation, [0.0, 5.0, 0.0]);
        assert_eq!(mode.effective_mass, [0.0, 350.0, 0.0]);
        assert_eq!(
            mode.effective_mass_fraction,
            [Some(0.0), Some(5.0 / 6.0), None]
        );
    }

    #[test]
    fn gamma_squared_times_m_is_the_effective_mass() {
        let (mass, dofs, p) = twelve_dof_basis_parts();
        let basis =
            ModalBasis::new(&mass, &dofs, [&p[0], &p[1], &p[2]], [10.0; 3], 2.0, TAU_T).unwrap();
        let phi: Vec<f64> = (0..12)
            .map(|i| ((i * 7 + 3) % 11) as f64 / 7.0 - 0.6)
            .collect();
        let mode = normalize_mode(0, &phi, &basis).unwrap();
        for k in 0..3 {
            // Γ, m and m_eff are each the double-double value rounded once
            // (relative error u plus ~2^-104). Γ²m in binary64 adds the
            // error of Γ twice and two roundings: within 4u of m_eff.
            let g = mode.participation[k];
            let lhs = g * g * mode.generalized_mass;
            let m_eff = mode.effective_mass[k];
            assert!(
                (lhs - m_eff).abs() <= 4.0 * U * m_eff,
                "k={k}: {lhs} vs {m_eff}"
            );
        }
        // Rescaling by a power of two is exact, and so is the normalizing
        // division of the rescaled input, so the published shape and every
        // quantity are bit-identical. (Another factor rounds each component,
        // and the published shapes then differ componentwise.)
        let scaled: Vec<f64> = phi.iter().map(|v| -4.0 * v).collect();
        // Only the input's own magnitudes scale with it.
        let expected = NormalizedMode {
            max_translation: 4.0 * mode.max_translation,
            max_rotation: 4.0 * mode.max_rotation,
            ..mode.clone()
        };
        assert_eq!(normalize_mode(0, &scaled, &basis).unwrap(), expected);
    }

    /// Independent review of PR #33, finding 4: one free Ux with φ = 1 and
    /// M_ff = p_x = T_x = s. Then Γ = 1, m_eff = s and the fraction 1, but
    /// (φᵀp)² overflowed for s = 10²⁰⁰ (m_eff NaN) and underflowed for
    /// s = 10⁻²⁰⁰ (m_eff and the fraction 0).
    #[test]
    fn participation_quantities_are_range_safe() {
        let dofs = [free(1, Dof::Ux)];
        let zero = [0.0];
        for s in [1e200, 1e-200, f64::MAX, f64::MIN_POSITIVE] {
            let mass = SymmetricMatrix::from_diagonal(&[s]).unwrap();
            let p = [s];
            let basis =
                ModalBasis::new(&mass, &dofs, [&p, &zero, &zero], [s, 0.0, 0.0], 1.0, TAU_T)
                    .unwrap();
            let mode = normalize_mode(0, &[1.0], &basis).unwrap();
            assert_eq!(mode.generalized_mass, s);
            assert_eq!(mode.participation[0], 1.0, "s = {s:e}");
            assert_eq!(mode.effective_mass[0], s, "s = {s:e}");
            assert_eq!(mode.effective_mass_fraction[0], Some(1.0), "s = {s:e}");
        }
        // A participation factor that truly overflows is refused, not
        // published as infinite.
        let mass = SymmetricMatrix::from_diagonal(&[1e-300]).unwrap();
        let p = [1e10];
        let basis =
            ModalBasis::new(&mass, &dofs, [&p, &zero, &zero], [1e10, 0.0, 0.0], 1.0, TAU_T)
                .unwrap();
        assert_eq!(
            normalize_mode(2, &[1.0], &basis),
            Err(ModalQuantityError::QuantityOutOfRange {
                mode: 2,
                name: "participation",
                direction: Some(0),
            })
        );
    }

    /// Review round 2, item 3: φ = 1, M_ff = 10⁻³⁰⁰, p_x = 10⁻²⁰⁰,
    /// T_x = 10²⁰⁰. Γ = 10¹⁰⁰, m_eff = 10⁻¹⁰⁰ and the fraction 10⁻³⁰⁰, but
    /// q/T = 10⁻⁴⁰⁰ underflowed before Γ multiplied it and the fraction was
    /// published as 0.
    #[test]
    fn a_small_fraction_is_not_lost_to_an_intermediate_underflow() {
        let dofs = [free(1, Dof::Ux)];
        let zero = [0.0];
        let mass = SymmetricMatrix::from_diagonal(&[1e-300]).unwrap();
        let p = [1e-200];
        let basis =
            ModalBasis::new(&mass, &dofs, [&p, &zero, &zero], [1e200, 0.0, 0.0], 1.0, TAU_T)
                .unwrap();
        let mode = normalize_mode(0, &[1.0], &basis).unwrap();
        // Each is the exact quotient of binary64 operands rounded once,
        // within 2u of the decimal value's binary64 rounding.
        let close = |actual: f64, expected: f64| {
            assert!(
                (actual - expected).abs() <= 2.0 * f64::EPSILON * expected,
                "{actual:e} vs {expected:e}"
            );
        };
        close(mode.participation[0], 1e100);
        close(mode.effective_mass[0], 1e-100);
        close(mode.effective_mass_fraction[0].unwrap(), 1e-300);
    }

    /// Review round 2, item 5: M_ff = diag(10³⁰⁸, 10³⁰⁸) and the shape
    /// (1, 1) give m = 2·10³⁰⁸, which overflows: refused as out of range,
    /// not as NonPositiveGeneralizedMass { NaN }.
    #[test]
    fn an_overflowing_generalized_mass_is_out_of_range() {
        let dofs = [free(1, Dof::Ux), free(1, Dof::Uy)];
        let zero = [0.0, 0.0];
        let mass = SymmetricMatrix::from_diagonal(&[1e308, 1e308]).unwrap();
        let basis =
            ModalBasis::new(&mass, &dofs, [&zero, &zero, &zero], [0.0; 3], 1.0, TAU_T).unwrap();
        assert_eq!(
            normalize_mode(4, &[1.0, 1.0], &basis),
            Err(ModalQuantityError::QuantityOutOfRange {
                mode: 4,
                name: "generalized_mass",
                direction: None,
            })
        );
    }

    #[test]
    fn cumulative_fractions_sum_the_published_values() {
        let make = |fx: f64| NormalizedMode {
            case: NormalizationCase::UnitMaxTranslation,
            shape: vec![1.0],
            reference_dof: 0,
            max_translation: 1.0,
            max_rotation: 0.0,
            generalized_mass: 1.0,
            excitation_product: [0.0; 3],
            participation: [0.0; 3],
            effective_mass: [0.0; 3],
            effective_mass_fraction: [Some(fx), Some(0.0), None],
        };
        let modes = [make(0.5), make(0.25), make(0.125)];
        let cumulative = cumulative_effective_mass_fractions(&modes);
        assert_eq!(cumulative[0], [Some(0.5), Some(0.0), None]);
        assert_eq!(cumulative[2], [Some(0.875), Some(0.0), None]);
    }

    #[test]
    fn basis_inputs_are_checked() {
        let mass = SymmetricMatrix::from_diagonal(&[1.0, 1.0]).unwrap();
        let dofs = [free(1, Dof::Ux), free(1, Dof::Ux)];
        let z = [0.0, 0.0];
        assert_eq!(
            ModalBasis::new(&mass, &dofs, [&z, &z, &z], [1.0; 3], 1.0, TAU_T).unwrap_err(),
            ModalQuantityError::DuplicateDof { index: 1 }
        );
        let dofs = [free(1, Dof::Ux), free(1, Dof::Uy)];
        let p = [1.0, 0.0];
        assert_eq!(
            ModalBasis::new(&mass, &dofs, [&p, &z, &z], [0.0, 1.0, 1.0], 1.0, TAU_T).unwrap_err(),
            ModalQuantityError::InvalidParameter { name: "total_mass" }
        );
        assert_eq!(
            ModalBasis::new(&mass, &dofs, [&z, &z, &z], [1.0; 3], 0.0, TAU_T).unwrap_err(),
            ModalQuantityError::InvalidParameter {
                name: "bounding_box_diagonal"
            }
        );
        let short = [0.0];
        assert!(matches!(
            ModalBasis::new(&mass, &dofs, [&short, &z, &z], [1.0; 3], 1.0, TAU_T),
            Err(ModalQuantityError::DimensionMismatch {
                name: "excitation",
                ..
            })
        ));
    }

    #[test]
    fn frequency_triplet() {
        let lambda = (2.0 * std::f64::consts::PI * 5.0).powi(2);
        let (f, omega, period) = frequency_from_eigenvalue(lambda);
        // λ carries three roundings (π, ×10, squaring), halved by the square
        // root; each later step adds one rounding.
        assert!((omega - 10.0 * std::f64::consts::PI).abs() <= 4.0 * U * omega);
        assert!((f - 5.0).abs() <= 6.0 * U * 5.0);
        assert!((period - 0.2).abs() <= 8.0 * U * 0.2);
    }
}
