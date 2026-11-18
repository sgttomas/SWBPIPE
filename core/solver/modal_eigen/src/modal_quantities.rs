//! Shape normalization and participation quantities (with D-v).
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
//! with m = φᵀ M_ff φ. Each is computed from the published vector: the
//! bilinear forms m and φᵀp_k are summed exactly (`exact::ExactSum`), so no
//! contribution is lost to alignment or rounding before a cancellation,
//! and the quotients and products of them ((φᵀp_k)², m·T_k) are formed in
//! double-double with an extended exponent (`dd::WideDd`); each quantity is
//! rounded to binary64 once. A published quantity that overflows or is NaN
//! refuses the mode (`QuantityOutOfRange`); one whose exact value is
//! nonzero but below the normal range is published as 0 and flagged,
//! decided on the exact value.
//!
//! **Storage.** Every vector this module returns for a mode (a normalized or
//! lifted shape, its below-normal indices, a row of cumulative fractions)
//! and the normalization's below-normal scratch is requested through a
//! [`ShapeStorage`], once, at its exact element count, with no further
//! growth. [`FallibleStorage`] checks `len × size_of::<T>()` against
//! `isize::MAX` before any request and asks the allocator with
//! `try_reserve_exact`, so an overflow or an allocator refusal is a typed
//! [`ModalQuantityError::Storage`] with no partial result, never an abort or
//! a numerical refusal. A caller may supply its own storage to add its
//! limits' context or, in its tests, to inject a refusal. The functions
//! without `_with` use [`FallibleStorage`]. Storage changes no value.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use crate::admission::{check_dense, PencilMatrix, PencilRangeUnsupported};
use crate::dd::{DdAccumulator, WideDd};
use crate::exact::ExactSum;
use crate::matrix::SymmetricMatrix;
use crate::workspace::{Retained, SumWorkspace};

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
    /// A nonzero entry of M_ff outside the admission range
    /// (`MODAL_PENCIL_RANGE_UNSUPPORTED`).
    PencilRangeUnsupported(PencilRangeUnsupported),
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
    /// A shape component φ_i/φ_ref overflows. (One below the normal range is
    /// published as 0 and flagged.)
    ShapeOutOfRange {
        mode: usize,
        index: usize,
    },
    /// A published quantity overflows or is NaN: `name`, in direction
    /// `direction` (`None` for the generalized mass). One below the normal
    /// range is published as 0 and flagged instead.
    QuantityOutOfRange {
        mode: usize,
        name: &'static str,
        direction: Option<usize>,
    },
    /// A storage request refused (module docs, "Storage"): a resource
    /// refusal, not a numerical one. Nothing was published.
    Storage(ShapeStorageError),
}

/// A storage request refused by a [`ShapeStorage`], before any partial
/// result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapeStorageError {
    /// `elements × element_size` bytes overflows `usize` or exceeds
    /// `isize::MAX`. Nothing was requested.
    CapacityOverflow {
        structure: &'static str,
        elements: usize,
        element_size: usize,
    },
    /// The allocator refused `bytes` (`elements` elements) for `structure`.
    /// Nothing was allocated.
    AllocationFailed {
        structure: &'static str,
        elements: usize,
        bytes: usize,
    },
}

impl ShapeStorageError {
    /// The structure whose request was refused.
    pub fn structure(&self) -> &'static str {
        match self {
            Self::CapacityOverflow { structure, .. } | Self::AllocationFailed { structure, .. } => structure,
        }
    }
}

impl fmt::Display for ShapeStorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CapacityOverflow { structure, elements, element_size } => write!(
                f,
                "the storage of `{structure}`, {elements} elements × {element_size} B, overflows the addressable size (at most isize::MAX = {} B); nothing was requested",
                isize::MAX
            ),
            Self::AllocationFailed { structure, elements, bytes } => {
                write!(f, "the allocator refused {bytes} B ({elements} elements) for `{structure}`")
            }
        }
    }
}

impl Error for ShapeStorageError {}

/// Where the vectors of this module's results are requested (module docs,
/// "Storage").
pub trait ShapeStorage {
    /// Gives the empty `vec` capacity for exactly `len` elements, one
    /// request, or refuses with nothing requested or allocated.
    fn modal_shape_reserve<T>(&mut self, vec: &mut Vec<T>, len: usize, structure: &'static str) -> Result<(), ShapeStorageError>;
}

/// The default [`ShapeStorage`]: [`reserve_exactly`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct FallibleStorage;

impl ShapeStorage for FallibleStorage {
    fn modal_shape_reserve<T>(&mut self, vec: &mut Vec<T>, len: usize, structure: &'static str) -> Result<(), ShapeStorageError> {
        reserve_exactly(vec, len, structure)
    }
}

/// `len × size_of::<T>()` checked against `isize::MAX` (a
/// [`ShapeStorageError::CapacityOverflow`], before any request), then
/// `try_reserve_exact(len)` on the empty `vec` (an allocator refusal is
/// [`ShapeStorageError::AllocationFailed`]). A zero length requests nothing.
pub fn reserve_exactly<T>(vec: &mut Vec<T>, len: usize, structure: &'static str) -> Result<(), ShapeStorageError> {
    let element_size = std::mem::size_of::<T>();
    let bytes = len
        .checked_mul(element_size)
        .filter(|&bytes| bytes <= isize::MAX as usize)
        .ok_or(ShapeStorageError::CapacityOverflow { structure, elements: len, element_size })?;
    vec.try_reserve_exact(len).map_err(|_| ShapeStorageError::AllocationFailed { structure, elements: len, bytes })
}

/// The structures this module requests (module docs, "Storage").
pub const NORMALIZED_SHAPE: &str = "normalized shape";
pub const NORMALIZED_BELOW_NORMAL: &str = "normalized below-normal indices";
pub const NORMALIZATION_SCRATCH: &str = "normalization below-normal scratch";
pub const LIFTED_SHAPE: &str = "lifted shape";
pub const LIFTED_BELOW_NORMAL: &str = "lifted below-normal indices";
pub const CUMULATIVE_FRACTIONS: &str = "cumulative effective-mass fractions";

impl fmt::Display for ModalQuantityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PencilRangeUnsupported(range) => write!(f, "{range}"),
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
            Self::ShapeOutOfRange { mode, index } => write!(
                f,
                "mode {mode}: shape component {index} leaves the binary64 normal range"
            ),
            Self::QuantityOutOfRange {
                mode,
                name,
                direction,
            } => match direction {
                Some(k) => write!(f, "mode {mode}: `{name}` in direction {k} is not finite"),
                None => write!(f, "mode {mode}: `{name}` is not finite"),
            },
            Self::Storage(error) => write!(f, "{error}"),
        }
    }
}

impl Error for ModalQuantityError {}

/// The model data every mode is normalized against.
///
/// It owns the normalization stage's workspace (one exact accumulator and
/// two scratch vectors, reused for every mode and retained until the basis
/// is dropped); a clone starts with an empty one and `Debug` omits it.
#[derive(Clone)]
pub struct ModalBasis<'a> {
    mass: &'a SymmetricMatrix,
    free_dofs: &'a [FreeDof],
    global_index: Vec<usize>,
    excitation: [&'a [f64]; 3],
    total_mass: [f64; 3],
    bounding_box_diagonal: f64,
    tau_t: f64,
    workspace: Retained<SumWorkspace>,
}

impl fmt::Debug for ModalBasis<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ModalBasis")
            .field("mass", &self.mass)
            .field("free_dofs", &self.free_dofs)
            .field("global_index", &self.global_index)
            .field("excitation", &self.excitation)
            .field("total_mass", &self.total_mass)
            .field("bounding_box_diagonal", &self.bounding_box_diagonal)
            .field("tau_t", &self.tau_t)
            .finish()
    }
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
        check_dense(PencilMatrix::Mass, mass).map_err(ModalQuantityError::PencilRangeUnsupported)?;
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
            workspace: Retained::default(),
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
    /// Which published values are nonzero but below the binary64 normal
    /// range, and so published as 0 (admission range).
    pub below_normal_range: BelowNormalRange,
}

/// The flags of [`NormalizedMode`]'s published values that are nonzero but
/// below the binary64 normal range: each is published as 0.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BelowNormalRange {
    /// Free indices of shape components published as 0.
    pub shape: Vec<usize>,
    pub generalized_mass: bool,
    pub excitation_product: [bool; 3],
    pub participation: [bool; 3],
    pub effective_mass: [bool; 3],
    pub effective_mass_fraction: [bool; 3],
}

/// xᵀ M y, every product x_i·M_ij·y_j summed exactly ([`ExactSum`], the
/// workspace's, reset first) and rounded once.
fn mass_bilinear(
    mass: &SymmetricMatrix,
    x: &[f64],
    y: &[f64],
    sum: &mut ExactSum,
    limbs: &mut Vec<i64>,
) -> WideDd {
    let n = mass.dimension();
    sum.reset();
    for i in 0..n {
        for (j, &yj) in y.iter().enumerate() {
            sum.add_product(&[x[i], mass.get(i, j), yj]);
        }
    }
    sum.to_wide_with(limbs)
}

/// Σ a_i b_i, summed exactly (the workspace's sum, reset first) and rounded
/// once.
fn wide_dot(a: &[f64], b: &[f64], sum: &mut ExactSum, limbs: &mut Vec<i64>) -> WideDd {
    sum.reset();
    for (&x, &y) in a.iter().zip(b) {
        sum.add_product(&[x, y]);
    }
    sum.to_wide_with(limbs)
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
    normalize_mode_with(mode, phi, basis, &mut FallibleStorage)
}

/// [`normalize_mode`] with the caller's [`ShapeStorage`].
pub fn normalize_mode_with<S: ShapeStorage>(
    mode: usize,
    phi: &[f64],
    basis: &ModalBasis<'_>,
    storage: &mut S,
) -> Result<NormalizedMode, ModalQuantityError> {
    // The basis's workspace, reset where used, for every mode.
    basis
        .workspace
        .with(|workspace| normalize_mode_in(mode, phi, basis, workspace, storage))
}

fn normalize_mode_in<S: ShapeStorage>(
    mode: usize,
    phi: &[f64],
    basis: &ModalBasis<'_>,
    workspace: &mut SumWorkspace,
    storage: &mut S,
) -> Result<NormalizedMode, ModalQuantityError> {
    let SumWorkspace { sum, limbs, below } = workspace;
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
    finish_normalization(mode, phi, basis, sum, limbs, below, Chosen { case, reference, pivot, t, theta }, storage)
}

/// the normalization of a
/// reduced (skew-coordinate) mode chosen on its lifted shape φ = Q̃φ′. The case
/// (τ_t, M1b), the pivot and the reported magnitudes are read from `lifted`
/// (global DOFs, node-major ux uy uz rx ry rz; ties to the lowest global
/// index), exactly as `normalize_mode` reads a free vector. The lift is linear,
/// so φ′/pivot lifts to φ/pivot: the generalized mass on M′, the excitation
/// products on Q̃ᵀ(M r_k) and the below-normal flags are formed on `phi` (the
/// reduced vector) with that pivot. `reference_dof` is the lifted global DOF.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LiftedNormalization {
    pub case: NormalizationCase,
    pub reference_dof: usize,
    pub pivot: f64,
    pub max_translation: f64,
    pub max_rotation: f64,
}

/// The lifted choice of C6 (see [`LiftedNormalization`]).
pub fn lifted_normalization(
    mode: usize,
    lifted: &[f64],
    bounding_box_diagonal: f64,
    tau_t: f64,
) -> Result<LiftedNormalization, ModalQuantityError> {
    if let Some(index) = lifted.iter().position(|v| !v.is_finite()) {
        return Err(ModalQuantityError::NonFinite { name: "lifted phi", index });
    }
    let largest = |translation: bool| {
        let mut best = (0.0_f64, None::<usize>);
        for (index, &v) in lifted.iter().enumerate() {
            if (index % 6 < 3) == translation && v.abs() > best.0 {
                best = (v.abs(), Some(index));
            }
        }
        best
    };
    let (t, translation_ref) = largest(true);
    let (theta, rotation_ref) = largest(false);
    let (case, reference) = if t <= tau_t * bounding_box_diagonal * theta {
        (NormalizationCase::UnitMaxRotation, rotation_ref)
    } else {
        (NormalizationCase::UnitMaxTranslation, translation_ref)
    };
    let reference_dof = reference.ok_or(ModalQuantityError::ZeroShape { mode })?;
    Ok(LiftedNormalization { case, reference_dof, pivot: lifted[reference_dof], max_translation: t, max_rotation: theta })
}

/// R-6/R-7 D-1 (C6, dynamics' B1): the published shape of a reduced mode, on the
/// raw lifted vector `lifted_normalization` chose on: shape_d = fl(lifted_d/pivot),
/// with `normalize_mode`'s below-normal rule applied per global DOF
/// (|lifted_d| < |pivot|·2⁻¹⁰²², decided in WideDd before rounding: 0, flagged).
/// The reference entry is exactly 1, and every entry of the normalizing class
/// (translations under UnitMaxTranslation, rotations under UnitMaxRotation) has
/// magnitude ≤ 1. An entry of the other class can exceed 1 (its unit differs), as on
/// the axis path.
#[derive(Debug, Clone, PartialEq)]
pub struct LiftedShape {
    /// Over the global DOFs.
    pub shape: Vec<f64>,
    /// Global DOFs published as 0 (nonzero, below the normal range).
    pub below_normal_range: Vec<usize>,
}

/// The published lifted shape (see [`LiftedShape`]).
pub fn publish_lifted(mode: usize, lifted: &[f64], chosen: &LiftedNormalization) -> Result<LiftedShape, ModalQuantityError> {
    publish_lifted_with(mode, lifted, chosen, &mut FallibleStorage)
}

/// [`publish_lifted`] with the caller's [`ShapeStorage`]. The shape is
/// requested at the lifted length; the below-normal indices, counted in the
/// shape's pass, are then requested at their exact count and gathered by a
/// second pass that applies the same exact test to the same entries.
pub fn publish_lifted_with<S: ShapeStorage>(
    mode: usize,
    lifted: &[f64],
    chosen: &LiftedNormalization,
    storage: &mut S,
) -> Result<LiftedShape, ModalQuantityError> {
    let pivot = chosen.pivot;
    if pivot == 0.0 || !pivot.is_finite() {
        return Err(ModalQuantityError::ZeroShape { mode });
    }
    let threshold = WideDd::from_f64(pivot.abs()).mul_f64(f64::MIN_POSITIVE);
    let below_normal = |v: f64| v != 0.0 && !WideDd::from_f64(v.abs()).sub(threshold).is_non_negative();
    let mut shape = Vec::new();
    storage.modal_shape_reserve(&mut shape, lifted.len(), LIFTED_SHAPE).map_err(ModalQuantityError::Storage)?;
    let mut below_count = 0;
    for (index, &v) in lifted.iter().enumerate() {
        if !v.is_finite() {
            return Err(ModalQuantityError::NonFinite { name: "lifted phi", index });
        }
        let s = v / pivot;
        if !s.is_finite() {
            return Err(ModalQuantityError::ShapeOutOfRange { mode, index });
        }
        if below_normal(v) {
            below_count += 1;
            shape.push(0.0);
        } else {
            shape.push(s);
        }
    }
    let mut below_normal_range = Vec::new();
    storage.modal_shape_reserve(&mut below_normal_range, below_count, LIFTED_BELOW_NORMAL).map_err(ModalQuantityError::Storage)?;
    below_normal_range.extend(lifted.iter().enumerate().filter(|&(_, &v)| below_normal(v)).map(|(index, _)| index));
    Ok(LiftedShape { shape, below_normal_range })
}

/// `normalize_mode` of a reduced vector with its lifted choice (C6). The returned
/// shape, flags and reference are the reduced coordinates' (φ′/pivot), which checks 2
/// and 3 read; the generalized mass, excitation products, participation and effective
/// masses are the reduced-coordinate quantities on M′ and Q̃ᵀ(M r_k), consistent with
/// the lifted shape (`publish_lifted`) to rounding. Publication uses `publish_lifted`.
pub fn normalize_mode_lifted(
    mode: usize,
    phi: &[f64],
    basis: &ModalBasis<'_>,
    chosen: LiftedNormalization,
) -> Result<NormalizedMode, ModalQuantityError> {
    normalize_mode_lifted_with(mode, phi, basis, chosen, &mut FallibleStorage)
}

/// [`normalize_mode_lifted`] with the caller's [`ShapeStorage`].
pub fn normalize_mode_lifted_with<S: ShapeStorage>(
    mode: usize,
    phi: &[f64],
    basis: &ModalBasis<'_>,
    chosen: LiftedNormalization,
    storage: &mut S,
) -> Result<NormalizedMode, ModalQuantityError> {
    basis.workspace.with(|workspace| {
        let SumWorkspace { sum, limbs, below } = workspace;
        let n = basis.dimension();
        if phi.len() != n {
            return Err(ModalQuantityError::DimensionMismatch { name: "phi", expected: n, actual: phi.len() });
        }
        if let Some(index) = phi.iter().position(|v| !v.is_finite()) {
            return Err(ModalQuantityError::NonFinite { name: "phi", index });
        }
        if chosen.pivot == 0.0 || !chosen.pivot.is_finite() {
            return Err(ModalQuantityError::ZeroShape { mode });
        }
        let chosen_at = Chosen {
            case: chosen.case,
            reference: chosen.reference_dof,
            pivot: chosen.pivot,
            t: chosen.max_translation,
            theta: chosen.max_rotation,
        };
        finish_normalization(mode, phi, basis, sum, limbs, below, chosen_at, storage)
    })
}

/// The normalization chosen for one mode.
struct Chosen {
    case: NormalizationCase,
    reference: usize,
    pivot: f64,
    t: f64,
    theta: f64,
}

#[allow(clippy::too_many_arguments)]
fn finish_normalization<S: ShapeStorage>(
    mode: usize,
    phi: &[f64],
    basis: &ModalBasis<'_>,
    sum: &mut crate::exact::ExactSum,
    limbs: &mut Vec<i64>,
    below: &mut Vec<usize>,
    chosen: Chosen,
    storage: &mut S,
) -> Result<NormalizedMode, ModalQuantityError> {
    let n = basis.dimension();
    let Chosen { case, reference, pivot, t, theta } = chosen;
    // Every shape component is published: an overflow refuses, and a
    // nonzero component whose exact quotient is below the normal range is
    // published as 0, flagged. The class is decided exactly, before
    // rounding (|v| < |pivot|·2⁻¹⁰²², compared in WideDd, where both sides
    // are exact): 2⁻¹⁰²² − 2⁻¹⁰⁷⁵ rounds to the smallest normal number. The
    // quantities below are formed from the published shape.
    let threshold = WideDd::from_f64(pivot.abs()).mul_f64(f64::MIN_POSITIVE);
    let mut below_normal_range = BelowNormalRange::default();
    let mut shape = Vec::new();
    storage.modal_shape_reserve(&mut shape, n, NORMALIZED_SHAPE).map_err(ModalQuantityError::Storage)?;
    // The below-normal indices are gathered in the workspace (room for n,
    // requested once) and copied out at exact capacity.
    below.clear();
    if below.capacity() < n {
        storage.modal_shape_reserve(below, n, NORMALIZATION_SCRATCH).map_err(ModalQuantityError::Storage)?;
    }
    for (index, &v) in phi.iter().enumerate() {
        let s = v / pivot;
        if !s.is_finite() {
            return Err(ModalQuantityError::ShapeOutOfRange { mode, index });
        }
        let below_normal = v != 0.0 && !WideDd::from_f64(v.abs()).sub(threshold).is_non_negative();
        if below_normal {
            below.push(index);
            shape.push(0.0);
        } else {
            shape.push(s);
        }
    }
    storage.modal_shape_reserve(&mut below_normal_range.shape, below.len(), NORMALIZED_BELOW_NORMAL).map_err(ModalQuantityError::Storage)?;
    below_normal_range.shape.extend_from_slice(below);

    let m = mass_bilinear(basis.mass, &shape, &shape, sum, limbs);
    // Range first: an m that overflows or is NaN is out of range, not
    // non-positive. Positivity is decided on the exact value.
    let Some((generalized_mass, m_below)) = m.to_published() else {
        return Err(ModalQuantityError::QuantityOutOfRange {
            mode,
            name: "generalized_mass",
            direction: None,
        });
    };
    if !m.is_positive() {
        return Err(ModalQuantityError::NonPositiveGeneralizedMass {
            mode,
            value: generalized_mass,
        });
    }
    below_normal_range.generalized_mass = m_below;
    let mut excitation_product = [0.0; 3];
    let mut participation = [0.0; 3];
    let mut effective_mass = [0.0; 3];
    let mut effective_mass_fraction = [None; 3];
    for k in 0..3 {
        // Published (admission range): an overflow or NaN
        // refuses; a nonzero value below the normal range is 0, flagged.
        let published = |name: &'static str, value: WideDd| {
            value
                .to_published()
                .ok_or(ModalQuantityError::QuantityOutOfRange {
                    mode,
                    name,
                    direction: Some(k),
                })
        };
        let flags = &mut below_normal_range;
        let q = wide_dot(&shape, basis.excitation[k], sum, limbs);
        let q_squared = q.mul(q);
        (excitation_product[k], flags.excitation_product[k]) =
            published("excitation_product", q)?;
        (participation[k], flags.participation[k]) = published("participation", q.div(m))?;
        (effective_mass[k], flags.effective_mass[k]) =
            published("effective_mass", q_squared.div(m))?;
        if basis.total_mass[k] > 0.0 {
            let fraction = q_squared.div(m.mul_f64(basis.total_mass[k]));
            let (value, below) = published("effective_mass_fraction", fraction)?;
            effective_mass_fraction[k] = Some(value);
            flags.effective_mass_fraction[k] = below;
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
        below_normal_range,
    })
}

/// Running sums of the published effective-mass fractions, in mode order,
/// accumulated in double-double and rounded once each.
pub fn cumulative_effective_mass_fractions(modes: &[NormalizedMode]) -> Vec<[Option<f64>; 3]> {
    let mut out = Vec::with_capacity(modes.len());
    push_cumulative_fractions(modes, &mut out);
    out
}

/// [`cumulative_effective_mass_fractions`] with the caller's
/// [`ShapeStorage`]: the rows are requested first, at the mode count, and
/// formed only once the request is granted.
pub fn cumulative_effective_mass_fractions_with<S: ShapeStorage>(
    modes: &[NormalizedMode],
    storage: &mut S,
) -> Result<Vec<[Option<f64>; 3]>, ShapeStorageError> {
    let mut out = Vec::new();
    storage.modal_shape_reserve(&mut out, modes.len(), CUMULATIVE_FRACTIONS)?;
    push_cumulative_fractions(modes, &mut out);
    Ok(out)
}

fn push_cumulative_fractions(modes: &[NormalizedMode], out: &mut Vec<[Option<f64>; 3]>) {
    let mut sums = [DdAccumulator::new(); 3];
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
}

/// f (Hz), ω (rad/s) and T (s) of an eigenvalue λ = ω² (rad²/s²).
pub fn frequency_from_eigenvalue(lambda: f64) -> (f64, f64, f64) {
    let omega = lambda.sqrt();
    let f = omega / (2.0 * std::f64::consts::PI);
    (f, omega, 1.0 / f)
}

#[cfg(test)]
mod tests {

    /// D-1 B1: the lifted publication. The reference entry is exactly 1, every entry
    /// is at most 1 in magnitude, and the flags are exactly the zeroed below-normal
    /// lifted entries (a constructed below-normal lift).
    #[test]
    fn lifted_publication_is_normalized_and_flags_below_normal_entries() {
        let tiny = f64::MIN_POSITIVE * 0.25; // below |pivot|·2⁻¹⁰²² for pivot 1.5
        let lifted = [0.75, -1.5, 0.0, tiny, -tiny, 1e-3, 0.0, 1.5 * f64::MIN_POSITIVE];
        let chosen = lifted_normalization(0, &lifted, 1.0, 1e-6).unwrap();
        assert_eq!((chosen.reference_dof, chosen.pivot), (1, -1.5));
        let published = publish_lifted(0, &lifted, &chosen).unwrap();
        assert_eq!(published.shape[chosen.reference_dof].to_bits(), 1.0f64.to_bits());
        assert!(published.shape.iter().all(|v| v.abs() <= 1.0));
        assert_eq!(published.below_normal_range, vec![3, 4]);
        for &d in &published.below_normal_range {
            assert_eq!(published.shape[d], 0.0);
            assert_ne!(lifted[d], 0.0);
        }
        // |v| = 1.5·2⁻¹⁰²² = |pivot|·2⁻¹⁰²² is at the threshold: published, not flagged.
        assert_eq!(published.shape[7], -f64::MIN_POSITIVE);
    }

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

    /// A test storage: counts each nonzero request and refuses the `armed`-th
    /// (0-based) as an allocator refusal would, before any request.
    struct CountingStorage {
        armed: Option<usize>,
        seen: Vec<&'static str>,
    }

    impl ShapeStorage for CountingStorage {
        fn modal_shape_reserve<T>(&mut self, vec: &mut Vec<T>, len: usize, structure: &'static str) -> Result<(), ShapeStorageError> {
            if len > 0 {
                let index = self.seen.len();
                self.seen.push(structure);
                if self.armed == Some(index) {
                    return Err(ShapeStorageError::AllocationFailed { structure, elements: len, bytes: len * std::mem::size_of::<T>() });
                }
            }
            reserve_exactly(vec, len, structure)
        }
    }

    /// Every storage request of a normalization (plain and lifted), a lifted
    /// publication and the cumulative rows refuses in turn with the typed
    /// storage error and nothing published; unarmed, each result is the
    /// default storage's, bit for bit.
    #[test]
    fn each_storage_request_refuses_typed_and_unarmed_results_are_unchanged() {
        let (mass, dofs, p) = twelve_dof_basis_parts();
        let basis = ModalBasis::new(&mass, &dofs, [&p[0], &p[1], &p[2]], [10.0; 3], 2.0, TAU_T).unwrap();
        let mut phi = vec![0.0; 12];
        phi[0] = 0.5;
        phi[7] = -1.25;
        phi[9] = f64::MIN_POSITIVE * 1e-3; // a below-normal published component
        let lifted: Vec<f64> = phi.iter().map(|v| v * 3.0).collect();
        let chosen = lifted_normalization(0, &lifted, 2.0, TAU_T).unwrap();
        let bits = |v: &[f64]| v.iter().map(|x| x.to_bits()).collect::<Vec<_>>();
        type Run<'a> = Box<dyn Fn(&mut CountingStorage) -> Result<String, ModalQuantityError> + 'a>;
        let runs: Vec<(&str, Run<'_>, String, Vec<&str>)> = {
            let plain = normalize_mode(0, &phi, &basis).unwrap();
            let reduced = normalize_mode_lifted(0, &phi, &basis, chosen).unwrap();
            let published = publish_lifted(0, &lifted, &chosen).unwrap();
            let cumulative = cumulative_effective_mass_fractions(std::slice::from_ref(&plain));
            assert_eq!(plain.below_normal_range.shape, vec![9]);
            assert_eq!(published.below_normal_range, vec![9]);
            let b = &basis;
            let (phi2, phi3, lifted2, plain2) = (phi.clone(), phi.clone(), lifted.clone(), plain.clone());
            vec![
                ("plain", Box::new(move |s: &mut CountingStorage| normalize_mode_with(0, &phi2, b, s).map(|m| format!("{m:?}{:?}", bits(&m.shape)))) as Run<'_>,
                    format!("{plain:?}{:?}", bits(&plain.shape)), vec![NORMALIZED_SHAPE, NORMALIZED_BELOW_NORMAL]),
                ("reduced", Box::new(move |s: &mut CountingStorage| normalize_mode_lifted_with(0, &phi3, b, chosen, s).map(|m| format!("{m:?}{:?}", bits(&m.shape)))) as Run<'_>,
                    format!("{reduced:?}{:?}", bits(&reduced.shape)), vec![NORMALIZED_SHAPE, NORMALIZED_BELOW_NORMAL]),
                ("lifted", Box::new(move |s: &mut CountingStorage| publish_lifted_with(0, &lifted2, &chosen, s).map(|m| format!("{m:?}{:?}", bits(&m.shape)))) as Run<'_>,
                    format!("{published:?}{:?}", bits(&published.shape)), vec![LIFTED_SHAPE, LIFTED_BELOW_NORMAL]),
                ("cumulative", Box::new(move |s: &mut CountingStorage| {
                    cumulative_effective_mass_fractions_with(std::slice::from_ref(&plain2), s).map(|c| format!("{c:?}")).map_err(ModalQuantityError::Storage)
                }) as Run<'_>, format!("{cumulative:?}"), vec![CUMULATIVE_FRACTIONS]),
            ]
        };
        for (name, run, expected, structures) in &runs {
            let mut unarmed = CountingStorage { armed: None, seen: Vec::new() };
            assert_eq!(&run(&mut unarmed).unwrap(), expected, "{name}: unarmed equals the default storage");
            assert_eq!(&unarmed.seen, structures, "{name}: the requests (the workspace scratch is already retained)");
            for k in 0..structures.len() {
                let mut armed = CountingStorage { armed: Some(k), seen: Vec::new() };
                match run(&mut armed) {
                    Err(ModalQuantityError::Storage(ShapeStorageError::AllocationFailed { structure, .. })) => assert_eq!(structure, structures[k], "{name} {k}"),
                    other => panic!("{name}: request {k} did not refuse typed: {other:?}"),
                }
                assert_eq!(armed.seen.len(), k + 1, "{name}: the refusal at request {k} ends the call");
            }
        }
        // A fresh basis's first normalization also requests its retained scratch.
        let fresh = ModalBasis::new(&mass, &dofs, [&p[0], &p[1], &p[2]], [10.0; 3], 2.0, TAU_T).unwrap();
        let mut first = CountingStorage { armed: Some(1), seen: Vec::new() };
        assert!(matches!(
            normalize_mode_with(0, &phi, &fresh, &mut first),
            Err(ModalQuantityError::Storage(ShapeStorageError::AllocationFailed { structure: NORMALIZATION_SCRATCH, .. }))
        ));
    }

    /// The byte size is checked before any request: an element count whose
    /// bytes pass isize::MAX refuses CapacityOverflow and allocates nothing;
    /// a zero count requests nothing.
    #[test]
    fn storage_overflow_is_refused_before_any_request() {
        let mut v: Vec<f64> = Vec::new();
        let too_many = (isize::MAX as usize) / 8 + 1;
        assert_eq!(
            reserve_exactly(&mut v, too_many, LIFTED_SHAPE),
            Err(ShapeStorageError::CapacityOverflow { structure: LIFTED_SHAPE, elements: too_many, element_size: 8 })
        );
        assert_eq!(reserve_exactly(&mut v, usize::MAX, LIFTED_SHAPE).unwrap_err().structure(), LIFTED_SHAPE);
        assert_eq!(v.capacity(), 0);
        reserve_exactly(&mut v, 0, LIFTED_SHAPE).unwrap();
        assert_eq!(v.capacity(), 0);
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
        // Review round 3: the witness masses lie outside the admission
        // range and are refused at entry.
        for s in [1e200, 1e-200, f64::MAX, f64::MIN_POSITIVE] {
            let mass = SymmetricMatrix::from_diagonal(&[s]).unwrap();
            let p = [s];
            assert!(matches!(
                ModalBasis::new(&mass, &dofs, [&p, &zero, &zero], [s, 0.0, 0.0], 1.0, TAU_T),
                Err(ModalQuantityError::PencilRangeUnsupported(_))
            ));
        }
        let p2 = |e: i32| 2f64.powi(e);
        for s in [p2(290), p2(-290), p2(300), p2(-300)] {
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
        // (p_k is not a pencil entry and is not range-checked.)
        let mass = SymmetricMatrix::from_diagonal(&[p2(-300)]).unwrap();
        let p = [1e300];
        let basis =
            ModalBasis::new(&mass, &dofs, [&p, &zero, &zero], [1e300, 0.0, 0.0], 1.0, TAU_T)
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
    /// T_x = 10²⁰⁰. q/T = 10⁻⁴⁰⁰ underflowed before Γ multiplied it and the
    /// fraction was published as 0. Since review round 3 that M_ff is
    /// outside the admission range and refused at entry. Within the range,
    /// M_ff = 2⁻³⁰⁰, p_x = 2⁻²⁰⁰ and T_x = 2²⁰⁰ publish Γ = 2¹⁰⁰,
    /// m_eff = 2⁻¹⁰⁰ and the fraction 2⁻³⁰⁰, each exact.
    #[test]
    fn a_small_fraction_is_not_lost_to_an_intermediate_underflow() {
        let dofs = [free(1, Dof::Ux)];
        let zero = [0.0];
        let mass = SymmetricMatrix::from_diagonal(&[1e-300]).unwrap();
        let p = [1e-200];
        assert!(matches!(
            ModalBasis::new(&mass, &dofs, [&p, &zero, &zero], [1e200, 0.0, 0.0], 1.0, TAU_T),
            Err(ModalQuantityError::PencilRangeUnsupported(_))
        ));
        let p2 = |e: i32| 2f64.powi(e);
        let mass = SymmetricMatrix::from_diagonal(&[p2(-300)]).unwrap();
        let p = [p2(-200)];
        let basis =
            ModalBasis::new(&mass, &dofs, [&p, &zero, &zero], [p2(200), 0.0, 0.0], 1.0, TAU_T)
                .unwrap();
        let mode = normalize_mode(0, &[1.0], &basis).unwrap();
        assert_eq!(mode.participation[0], p2(100));
        assert_eq!(mode.effective_mass[0], p2(-100));
        assert_eq!(mode.effective_mass_fraction[0], Some(p2(-300)));
    }

    /// Review round 2, item 5: M_ff = diag(10³⁰⁸, 10³⁰⁸) and the shape
    /// (1, 1) give m = 2·10³⁰⁸, which overflowed and was refused as
    /// NonPositiveGeneralizedMass { NaN }. Since review round 3 that M_ff is
    /// outside the admission range and refused at entry; within it m cannot
    /// overflow (m ≤ n²·2³⁰⁰ with a normalized shape). The finiteness check
    /// still precedes positivity.
    #[test]
    fn an_overflowing_generalized_mass_is_out_of_range() {
        let dofs = [free(1, Dof::Ux), free(1, Dof::Uy)];
        let zero = [0.0, 0.0];
        let mass = SymmetricMatrix::from_diagonal(&[1e308, 1e308]).unwrap();
        assert_eq!(
            ModalBasis::new(&mass, &dofs, [&zero, &zero, &zero], [0.0; 3], 1.0, TAU_T)
                .unwrap_err(),
            ModalQuantityError::PencilRangeUnsupported(PencilRangeUnsupported {
                matrix: PencilMatrix::Mass,
                row: 0,
                col: 0,
                value: 1e308,
            })
        );
    }

    /// Review round 3, item B, with the published-value rule (design §3.2,
    /// admission range): with τ_t = 0 a mode with any translation is case
    /// (a). φ = (2⁻⁶⁰⁰ on Ux, 2⁶⁰⁰ on Rz) normalizes Rz to 2¹²⁰⁰, an overflow,
    /// which refuses. φ = (2⁶⁰⁰, 2⁻⁶⁰⁰) normalizes Rz to 2⁻¹²⁰⁰, nonzero but
    /// below the normal range: it was a silent zero, then a refusal, and is
    /// now published as 0 with its flag.
    #[test]
    fn every_shape_component_is_validated() {
        let p2 = |e: i32| 2f64.powi(e);
        let mass = SymmetricMatrix::from_diagonal(&[1.0, 1.0]).unwrap();
        let dofs = [free(1, Dof::Ux), free(1, Dof::Rz)];
        let z = [0.0, 0.0];
        let basis = ModalBasis::new(&mass, &dofs, [&z, &z, &z], [0.0; 3], 1.0, 0.0).unwrap();
        assert_eq!(
            normalize_mode(7, &[p2(-600), p2(600)], &basis),
            Err(ModalQuantityError::ShapeOutOfRange { mode: 7, index: 1 })
        );
        let mode = normalize_mode(7, &[p2(600), p2(-600)], &basis).unwrap();
        assert_eq!(mode.shape, vec![1.0, 0.0]);
        assert_eq!(mode.below_normal_range.shape, vec![1]);
        // An exactly zero component is an unflagged zero.
        let exact = normalize_mode(7, &[p2(600), 0.0], &basis).unwrap();
        assert_eq!(exact.shape, vec![1.0, 0.0]);
        assert_eq!(exact.below_normal_range, BelowNormalRange::default());
    }

    /// The published-value rule: φ = 1, M_ff = 1, p_x = 10⁻¹⁶⁰, T_x = 1. Γ
    /// and φᵀp are 10⁻¹⁶⁰ (normal), while m_eff and the fraction are about
    /// 10⁻³²⁰, nonzero but below the normal range: published as 0 with
    /// their flags, and the mode is not refused.
    #[test]
    fn a_fraction_below_the_normal_range_is_a_flagged_zero() {
        let dofs = [free(1, Dof::Ux)];
        let zero = [0.0];
        let mass = SymmetricMatrix::from_diagonal(&[1.0]).unwrap();
        let p = [1e-160];
        let basis =
            ModalBasis::new(&mass, &dofs, [&p, &zero, &zero], [1.0, 0.0, 0.0], 1.0, TAU_T)
                .unwrap();
        let mode = normalize_mode(0, &[1.0], &basis).unwrap();
        assert_eq!(mode.participation[0], 1e-160);
        assert_eq!(mode.excitation_product[0], 1e-160);
        assert_eq!(mode.effective_mass[0], 0.0);
        assert_eq!(mode.effective_mass_fraction[0], Some(0.0));
        let flags = &mode.below_normal_range;
        assert_eq!(flags.effective_mass, [true, false, false]);
        assert_eq!(flags.effective_mass_fraction, [true, false, false]);
        assert_eq!(flags.participation, [false; 3]);
        assert!(!flags.generalized_mass && flags.shape.is_empty());
    }

    /// Review round 4, item 2: three Ux DOFs, M_ff = diag(1, 2⁻³⁰⁰, 1),
    /// p_x = M·1 and φ = (1, 2⁻⁷⁰¹, −1). The exact φᵀp = 1 + 2⁻¹⁰⁰¹ − 1 =
    /// 2⁻¹⁰⁰¹ and Γ = φᵀp/m = 2⁻¹⁰⁰² are normal, but the extended-exponent
    /// sum dropped 2⁻¹⁰⁰¹ beside 1 and published both as unflagged zeros.
    /// The exact accumulation keeps it.
    #[test]
    fn a_contribution_dropped_by_alignment_is_not_exposed_by_cancellation() {
        let p2 = |e: i32| 2f64.powi(e);
        let mass = SymmetricMatrix::from_diagonal(&[1.0, p2(-300), 1.0]).unwrap();
        let dofs = [free(0, Dof::Ux), free(1, Dof::Ux), free(2, Dof::Ux)];
        let p = [1.0, p2(-300), 1.0];
        let z = [0.0; 3];
        let total = 2.0 + p2(-300);
        let basis =
            ModalBasis::new(&mass, &dofs, [&p, &z, &z], [total, 0.0, 0.0], 1.0, TAU_T).unwrap();
        let mode = normalize_mode(0, &[1.0, p2(-701), -1.0], &basis).unwrap();
        assert_eq!(mode.generalized_mass, 2.0);
        assert_eq!(mode.excitation_product[0], p2(-1001));
        assert_eq!(mode.participation[0], p2(-1002));
        assert_eq!(mode.below_normal_range.excitation_product, [false; 3]);
        assert_eq!(mode.below_normal_range.participation, [false; 3]);
        // m_eff = 2⁻²⁰⁰³ and the fraction are below the range: flagged.
        assert_eq!(mode.effective_mass[0], 0.0);
        assert!(mode.below_normal_range.effective_mass[0]);
        assert!(mode.below_normal_range.effective_mass_fraction[0]);
    }

    /// Review round 4, item 5: φ = (2, 2⁻¹⁰²¹ − 2⁻¹⁰⁷⁴) with M = I. The
    /// second shape component's exact quotient 2⁻¹⁰²² − 2⁻¹⁰⁷⁵ rounds to the
    /// smallest normal number and was published unflagged; classified on
    /// the exact quotient, it is below the range: 0, flagged.
    #[test]
    fn a_shape_quotient_is_classified_before_rounding() {
        let mass = SymmetricMatrix::from_diagonal(&[1.0, 1.0]).unwrap();
        let dofs = [free(1, Dof::Ux), free(1, Dof::Uy)];
        let z = [0.0, 0.0];
        let basis = ModalBasis::new(&mass, &dofs, [&z, &z, &z], [0.0; 3], 1.0, TAU_T).unwrap();
        let tiny = 2f64.powi(-1021) - f64::from_bits(1);
        assert_eq!(tiny / 2.0, f64::MIN_POSITIVE);
        let mode = normalize_mode(0, &[2.0, tiny], &basis).unwrap();
        assert_eq!(mode.shape, vec![1.0, 0.0]);
        assert_eq!(mode.below_normal_range.shape, vec![1]);
        // Exactly 2⁻¹⁰²² is normal and published as it is.
        let edge = normalize_mode(0, &[2.0, 2f64.powi(-1021)], &basis).unwrap();
        assert_eq!(edge.shape, vec![1.0, f64::MIN_POSITIVE]);
        assert!(edge.below_normal_range.shape.is_empty());
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
            below_normal_range: BelowNormalRange::default(),
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
