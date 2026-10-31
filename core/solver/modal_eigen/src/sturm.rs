//! Sturm counts (design §3.2 item 7, §3.3 and §3.4 check 4): the number of
//! eigenvalues of K φ = λ M φ below a shift σ, read as the negative-pivot
//! count of an unpivoted LDLᵀ of the scaled shifted matrix, and relied on
//! only under stated premises.
//!
//! For a shift σ:
//! 1. **Formation.** Over the union pattern of K and M, each lower entry is
//!    formed as a = fl(k_ij − fl(σ·m_ij)) and then scaled one factor at a
//!    time, ã_ij = fl(fl(d_i·a)·d_j), with D = diag(|k_ii|^-½) and
//!    d_i = fl(1/fl(√|k_ii|)). The scaling is an exact congruence by the
//!    computed D, so it preserves inertia. Every intermediate of an operation
//!    with nonzero operands must be a normal binary64 number: a result that
//!    rounds to zero or to a subnormal, or overflows, puts the shift out of
//!    range.
//! 2. **Factorization** through `sparse_direct`'s `factorize_ldlt`, in the
//!    reverse Cuthill–McKee order of the union pattern. A pivot at or below
//!    its 1e-12 guard, or a non-finite coefficient, rejects the shift.
//! 3. **Estimated backward error** e_b = γ_n·‖|L̂||D̂||L̂ᵀ|‖_∞ +
//!    c_f·u·‖|K̃| + |σ||M̃|‖_∞, with γ_n = n·u/(1 − n·u), u = 2⁻⁵³ and
//!    c_f = [`FORMATION_ROUNDING_CONSTANT`]. The first term is read through
//!    `ProfileFactorization`'s observation accessor in two profile
//!    traversals with O(n) workspace, v = |L̂|ᵀ·1 and then
//!    w = |L̂|·(|D̂|·v), the unit diagonal of L̂ included.
//! 4. **Premises**, all evaluated for the evidence, decided in this order:
//!    - separation: s_j(σ) = (|λ_j − σ| − e_λ,j·λ_j)·(φ̃_jᵀM̃φ̃_j)/‖φ̃_j‖₂²
//!      ≥ κ_s·e_b for each neighbouring eigenvalue supplied. The quotient's
//!      weighted terms and sums are formed with an extended exponent
//!      (`dd::WideDd`), and the quotient must be positive and normal; a
//!      separation that is not finite refuses the candidate
//!      (`SeparationOutOfRange`), so no NaN reaches the comparison;
//!    - growth: g = max(1, max|L̂_ij|)²·max|d̂_j|/max|ã_jj| ≤ g_max;
//!    - conditioning: e_b·ν ≤ 1/κ_c, with ν the Hager–Higham estimate of
//!      ‖Ã⁻¹‖₁.
//!
//! Placement in a gap tries σ = √λ_lo·√λ_hi first, then the caller's
//! dyadic log-positions in the same gap, at most s_max candidates in all. A
//! shift is never moved out of its gap. The whole-spectrum final shift
//! (design §3.4 check 4, "where shifts go") lies a stated relative offset
//! above the largest eigenvalue and is offered only to a caller that asserts
//! an established finite-mode count (design §3.2 item 5).
//!
//! These are numerical-integrity checks with stated premises, not
//! certified enclosures: the separation premise is first-order, ν can
//! underestimate ‖Ã⁻¹‖₁, and e_b is evaluated in binary64. A count taken
//! under the premises agrees with the reported set under those premises; it
//! does not prove that no eigenvalue is missing.
//!
//! **Memory.** A [`ShiftPencil`] keeps the lower union pattern with both
//! values, 32 B per position (P_u positions), and three n-vectors. Each
//! shift forms a 24 B-per-position entry list, builds the ordered profile
//! (8 B per profile entry, P) and drops the list before `factorize_ldlt`
//! clones the profile. The peak of a count is therefore
//! max(32·P_u + 24·P_u + 8·P, 32·P_u + 16·P) plus O(n) vectors. On the dense
//! path, with a full pattern (P_u = P ≈ n²/2), that is about 32·n² B, beside
//! the caller's K, M and mode shapes.
//!
//! The algorithm parameters (κ_c, κ_s, g_max, s_max and the candidate
//! positions) are caller inputs with no default; their production values are
//! recorded under DEC-026 (design §9, O-6).

use std::error::Error;
use std::fmt;

use open_pipe_stress_sparse_direct::{
    factorize_ldlt, reverse_cuthill_mckee, ProfileFactorization, SparseDirectError,
    SymmetricMatrixEntry, SymmetricProfileMatrix,
};

use crate::admission::{check_entry, PencilMatrix, PencilRangeUnsupported};
use crate::dd::max_propagating_nan;
use crate::exact::ExactSum;
use crate::norm_estimate::inverse_one_norm_estimate;
use crate::matrix::SymmetricMatrix;

/// c_f of the formation term: one rounding each for σ·m_ij, the
/// subtraction and the two scalings, with one to spare for the second-order
/// terms (design §3.4 check 4).
pub const FORMATION_ROUNDING_CONSTANT: f64 = 5.0;

/// The deepest dyadic level a gap log-position may have: t·2^32 must be an
/// integer.
pub const MAX_GAP_POSITION_LEVEL: u32 = 32;

pub use crate::norm_estimate::INVERSE_NORM_MAX_ITERATIONS;

const U: f64 = f64::EPSILON / 2.0;

#[derive(Debug, Clone, PartialEq)]
pub enum SturmError {
    DimensionMismatch {
        stiffness: usize,
        mass: usize,
    },
    EntryOutOfBounds {
        row: usize,
        col: usize,
    },
    /// A position given twice in one matrix's entry list. Entries are
    /// assembled values; summing them here would add a rounding the
    /// formation bound does not cover.
    DuplicateEntry {
        row: usize,
        col: usize,
    },
    NonFiniteEntry {
        row: usize,
        col: usize,
    },
    /// A nonzero entry outside the admission range
    /// (`MODAL_PENCIL_RANGE_UNSUPPORTED`).
    PencilRangeUnsupported(PencilRangeUnsupported),
    /// k_ii ≤ 0: D = diag(|k_ii|^-½) is not formed for a stiffness that is
    /// not positive definite.
    NonPositiveStiffnessDiagonal {
        index: usize,
    },
    /// d_i, or a row sum of |K̃| or |M̃|, is not a normal finite number.
    ScalingOutOfRange {
        index: usize,
    },
    InvalidParameter {
        name: &'static str,
    },
    /// A neighbouring eigenvalue that is not positive and finite, an error
    /// estimate that is not finite and non-negative, or a vector of the
    /// wrong length, with non-finite entries or all zero.
    InvalidNeighbour {
        index: usize,
    },
    /// The neighbour's mass quotient φᵀMφ / Σ_i k_ii·φ_i², formed with an
    /// extended exponent, is not a positive normal binary64 number. A
    /// vector with φᵀMφ ≤ 0 is not a mode of a finite positive eigenvalue,
    /// and a separation formed from it is meaningless.
    NeighbourMassQuotient {
        index: usize,
        value: f64,
    },
    /// The gap's ends are not positive, finite and strictly ascending.
    InvalidGap,
    InvalidShift,
}

impl fmt::Display for SturmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DimensionMismatch { stiffness, mass } => write!(
                f,
                "stiffness is {stiffness}x{stiffness} but mass is {mass}x{mass}"
            ),
            Self::EntryOutOfBounds { row, col } => {
                write!(f, "entry ({row}, {col}) is outside the pencil")
            }
            Self::DuplicateEntry { row, col } => write!(f, "entry ({row}, {col}) is given twice"),
            Self::NonFiniteEntry { row, col } => write!(f, "entry ({row}, {col}) is not finite"),
            Self::PencilRangeUnsupported(range) => write!(f, "{range}"),
            Self::NonPositiveStiffnessDiagonal { index } => {
                write!(f, "stiffness diagonal {index} is not positive")
            }
            Self::ScalingOutOfRange { index } => {
                write!(f, "the diagonal scaling of row {index} is out of range")
            }
            Self::InvalidParameter { name } => write!(f, "invalid Sturm parameter `{name}`"),
            Self::InvalidNeighbour { index } => {
                write!(f, "invalid neighbouring eigenvalue {index}")
            }
            Self::NeighbourMassQuotient { index, value } => write!(
                f,
                "the mass quotient of neighbouring eigenvalue {index} is {value:e}, not positive and normal"
            ),
            Self::InvalidGap => write!(f, "the gap's ends are not positive and ascending"),
            Self::InvalidShift => write!(f, "the shift is not a positive finite number"),
        }
    }
}

impl Error for SturmError {}

/// The algorithm parameters of check 4. There is no default.
#[derive(Debug, Clone, PartialEq)]
pub struct SturmParameters {
    /// κ_c: the count is relied on only when e_b·ν ≤ 1/κ_c. Finite, ≥ 1.
    pub conditioning_multiple: f64,
    /// κ_s: the count is relied on only when s_j(σ) ≥ κ_s·e_b for every
    /// neighbour. Finite, ≥ 1.
    pub separation_multiple: f64,
    /// g_max: a factorization with g > g_max is rejected. Finite, ≥ 1.
    pub growth_limit: f64,
    /// s_max: the most candidate shifts tried for one placement. ≥ 1.
    pub max_candidates: usize,
    /// The positions tried in a gap after its geometric mean, as fractions
    /// t of the gap in log scale (σ = λ_lo^(1−t)·λ_hi^t, formed by repeated
    /// geometric means, never by a power). Each t lies in (0, 1) and is a
    /// dyadic rational of level at most [`MAX_GAP_POSITION_LEVEL`].
    pub gap_log_positions: Vec<f64>,
    /// The relative offsets r tried, in order, for the whole-spectrum shift
    /// σ = fl(λ_top·fl(1 + r)). Each is finite and positive.
    pub whole_spectrum_offsets: Vec<f64>,
}

impl SturmParameters {
    fn validate(&self) -> Result<(), SturmError> {
        let at_least_one = |value: f64| value.is_finite() && value >= 1.0;
        if !at_least_one(self.conditioning_multiple) {
            return Err(SturmError::InvalidParameter {
                name: "conditioning_multiple",
            });
        }
        if !at_least_one(self.separation_multiple) {
            return Err(SturmError::InvalidParameter {
                name: "separation_multiple",
            });
        }
        if !at_least_one(self.growth_limit) {
            return Err(SturmError::InvalidParameter {
                name: "growth_limit",
            });
        }
        if self.max_candidates == 0 {
            return Err(SturmError::InvalidParameter {
                name: "max_candidates",
            });
        }
        let scale = (1u64 << MAX_GAP_POSITION_LEVEL) as f64;
        for &t in &self.gap_log_positions {
            if !(t > 0.0 && t < 1.0 && (t * scale).fract() == 0.0) {
                return Err(SturmError::InvalidParameter {
                    name: "gap_log_positions",
                });
            }
        }
        for &r in &self.whole_spectrum_offsets {
            if !(r.is_finite() && r > 0.0) {
                return Err(SturmError::InvalidParameter {
                    name: "whole_spectrum_offsets",
                });
            }
        }
        Ok(())
    }
}

/// A computed eigenvalue next to a shift: the nearest member of a
/// neighbouring cluster.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Neighbour<'a> {
    /// λ_j, positive and finite.
    pub eigenvalue: f64,
    /// e_λ,j, the relative error estimate of check 2 (finite, ≥ 0).
    pub relative_error_estimate: f64,
    /// φ_j in the pencil's numbering (unscaled).
    pub vector: &'a [f64],
}

/// The lower end of a gap.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GapLower<'a> {
    /// The completed last cluster below the shift.
    Mode(Neighbour<'a>),
    /// A bound that is not an eigenvalue: λ_c = ω_c² when the reported set
    /// is empty (a cut-off below the lowest frequency). No separation is
    /// required from it.
    Bound(f64),
}

/// A gap between computed clusters.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Gap<'a> {
    pub lower: GapLower<'a>,
    /// The nearest member of the next computed cluster above.
    pub upper: Neighbour<'a>,
}

/// An operation of the formation whose result left the normal range.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormationOperation {
    /// fl(σ·m_ij)
    ShiftTimesMass,
    /// fl(k_ij − σ·m_ij)
    Difference,
    /// fl(d_i·a)
    FirstScaling,
    /// fl(fl(d_i·a)·d_j)
    SecondScaling,
    /// The shift itself (whole-spectrum offsets), or the formation term's
    /// row sums.
    Norm,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RangeViolation {
    pub row: usize,
    pub col: usize,
    pub operation: FormationOperation,
    pub value: f64,
}

/// The decision on one candidate shift.
#[derive(Debug, Clone, PartialEq)]
pub enum ShiftDecision {
    /// Every premise holds; the count is taken.
    Accepted,
    /// The candidate does not lie strictly inside its gap (a gap too narrow
    /// for distinct binary64 positions). Not factored.
    OutsideGap,
    /// The formation met a subnormal, a zero from nonzero operands, or an
    /// overflow. Not factored.
    OutOfRange(RangeViolation),
    /// The separation s_j(σ) of neighbour `neighbour` (its index in the
    /// placement's neighbour list) is not finite, so it cannot be compared
    /// with κ_s·e_b. Not factored.
    SeparationOutOfRange { neighbour: usize, value: f64 },
    /// `factorize_ldlt` refused: a pivot at or below its guard, or a
    /// non-finite coefficient, at a shift not already shown unseparated by
    /// the formation term alone.
    FactorizationRejected(SparseDirectError),
    /// s_j(σ) < κ_s·e_b for some neighbour.
    Unseparated,
    /// g > g_max.
    GrowthLimit,
    /// e_b·ν > 1/κ_c.
    IllConditioned,
}

impl ShiftDecision {
    /// How far the candidate got through the checks, for naming a set-level
    /// refusal (see [`SturmRefusal`]).
    fn stage(&self) -> u8 {
        match self {
            Self::OutOfRange(_) | Self::SeparationOutOfRange { .. } => 0,
            Self::FactorizationRejected(_) => 1,
            Self::OutsideGap | Self::Unseparated => 2,
            Self::GrowthLimit => 3,
            Self::IllConditioned => 4,
            Self::Accepted => 5,
        }
    }
}

/// The observations on a factored candidate. Every premise is evaluated, so
/// the evidence is complete whichever premise decides.
#[derive(Debug, Clone, PartialEq)]
pub struct FactoredShift {
    /// The negative-pivot count, the Sturm count when accepted.
    pub negative_pivot_count: usize,
    /// γ_n·‖|L̂||D̂||L̂ᵀ|‖_∞.
    pub factor_term: f64,
    /// e_b = factor term + formation term.
    pub backward_error: f64,
    /// ν, the Hager–Higham estimate of ‖Ã⁻¹‖₁ (+∞ when a solve overflowed).
    pub inverse_norm_estimate: f64,
    /// Solves spent on ν.
    pub inverse_norm_solves: usize,
    /// e_b·ν.
    pub conditioning_product: f64,
    /// min_j s_j(σ)/e_b over the neighbours; `None` when no neighbour
    /// constrains the shift.
    pub separation_ratio: Option<f64>,
    /// g.
    pub growth: f64,
    pub max_abs_lower_factor: f64,
    pub max_abs_pivot: f64,
    pub max_abs_scaled_diagonal: f64,
    pub separated: bool,
    pub within_growth_limit: bool,
    pub conditioned: bool,
}

/// One candidate shift and what was found.
#[derive(Debug, Clone, PartialEq)]
pub struct ShiftEvaluation {
    pub sigma: f64,
    pub decision: ShiftDecision,
    /// c_f·u·‖|K̃| + |σ||M̃|‖_∞, once the shifted matrix is formed.
    pub formation_term: Option<f64>,
    /// min_j s_j(σ) / (formation term): an upper bound on the separation
    /// ratio, known before factoring (e_b ≥ the formation term). When the
    /// factorization is rejected and this is below κ_s, the candidate is
    /// `Unseparated`, whatever the factor term would have added. `None`
    /// before formation or with no neighbour.
    pub formation_separation_ratio: Option<f64>,
    pub factored: Option<FactoredShift>,
}

/// Where the candidates came from.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Placement {
    /// One shift chosen by the caller.
    Given,
    /// A gap between computed clusters: its ends.
    Gap { lower: f64, upper: f64 },
    /// Above the whole finite spectrum, under the caller's established
    /// finite-mode count.
    AboveSpectrum {
        top_eigenvalue: f64,
        established_finite_mode_count: usize,
    },
}

/// The evidence of a placement, whatever its outcome.
#[derive(Debug, Clone, PartialEq)]
pub struct SturmEvidence {
    pub placement: Placement,
    pub conditioning_multiple: f64,
    /// 1/κ_c, the limit the conditioning product is compared with.
    pub conditioning_limit: f64,
    pub separation_multiple: f64,
    pub growth_limit: f64,
    /// Every candidate tried, in order; the last is the accepted one when
    /// the outcome is `Counted`.
    pub candidates: Vec<ShiftEvaluation>,
}

impl SturmEvidence {
    /// The accepted candidate, if any.
    pub fn accepted(&self) -> Option<&ShiftEvaluation> {
        self.candidates
            .last()
            .filter(|candidate| candidate.decision == ShiftDecision::Accepted)
    }

    /// The largest growth factor over the factored candidates (NaN if any is
    /// NaN).
    pub fn largest_growth(&self) -> Option<f64> {
        self.candidates
            .iter()
            .filter_map(|candidate| candidate.factored.as_ref().map(|f| f.growth))
            .fold(None, |acc: Option<f64>, g| {
                Some(acc.map_or(g, |a| max_propagating_nan(a, g)))
            })
    }
}

/// Why no count was taken. The set-level reason is that of the candidate
/// that got furthest through the checks (formation, factorization,
/// separation, growth, conditioning):
/// - `OutOfRange` when every candidate left the normal range in formation,
///   or had a separation that is not finite;
/// - `GrowthLimit` when the furthest candidate was refused by the
///   factorization's pivot guard (design §3.4 check 4 groups the guard with
///   the growth limit) or was separated but exceeded g_max;
/// - `Unseparated` when no candidate satisfied the separation premise;
/// - `IllConditioned` when a separated candidate within the growth limit
///   failed the conditioning premise.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SturmRefusal {
    Unseparated,
    IllConditioned,
    GrowthLimit,
    OutOfRange,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SturmOutcome {
    Counted {
        count: usize,
        evidence: SturmEvidence,
    },
    Refused {
        reason: SturmRefusal,
        evidence: SturmEvidence,
    },
}

impl SturmOutcome {
    pub fn evidence(&self) -> &SturmEvidence {
        match self {
            Self::Counted { evidence, .. } | Self::Refused { evidence, .. } => evidence,
        }
    }

    pub fn count(&self) -> Option<usize> {
        match self {
            Self::Counted { count, .. } => Some(*count),
            Self::Refused { .. } => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct PencilEntry {
    row: usize,
    col: usize,
    stiffness: f64,
    mass: f64,
}

/// K and M over their union pattern, with the diagonal scaling, the scaled
/// row sums of |K̃| and |M̃| and the reverse Cuthill–McKee order of the
/// pattern. Built once and shifted many times.
#[derive(Debug, Clone, PartialEq)]
pub struct ShiftPencil {
    dimension: usize,
    /// Lower-triangle union entries, (row ≥ col), ascending by (row, col);
    /// every diagonal is present.
    entries: Vec<PencilEntry>,
    scaling: Vec<f64>,
    stiffness_row_sums: Vec<f64>,
    mass_row_sums: Vec<f64>,
    order: Vec<usize>,
}

impl ShiftPencil {
    /// From dense K and M: the lower-triangle entries where either is
    /// nonzero, and every diagonal.
    pub fn from_dense(
        stiffness: &SymmetricMatrix,
        mass: &SymmetricMatrix,
    ) -> Result<Self, SturmError> {
        let n = stiffness.dimension();
        if mass.dimension() != n {
            return Err(SturmError::DimensionMismatch {
                stiffness: n,
                mass: mass.dimension(),
            });
        }
        let mut k_entries = Vec::new();
        let mut m_entries = Vec::new();
        for row in 0..n {
            for col in 0..=row {
                let k = stiffness.get(row, col);
                if k != 0.0 {
                    k_entries.push(SymmetricMatrixEntry { row, col, value: k });
                }
                let m = mass.get(row, col);
                if m != 0.0 {
                    m_entries.push(SymmetricMatrixEntry { row, col, value: m });
                }
            }
        }
        Self::from_entries(n, &k_entries, &m_entries)
    }

    /// From assembled entries of K and M, one triangle each: a position
    /// (in either orientation) may appear at most once per matrix.
    pub fn from_entries(
        dimension: usize,
        stiffness: &[SymmetricMatrixEntry],
        mass: &[SymmetricMatrixEntry],
    ) -> Result<Self, SturmError> {
        let mut merged: Vec<(usize, usize, bool, f64)> =
            Vec::with_capacity(stiffness.len() + mass.len() + dimension);
        for (is_mass, list) in [(false, stiffness), (true, mass)] {
            for entry in list {
                if entry.row >= dimension || entry.col >= dimension {
                    return Err(SturmError::EntryOutOfBounds {
                        row: entry.row,
                        col: entry.col,
                    });
                }
                if !entry.value.is_finite() {
                    return Err(SturmError::NonFiniteEntry {
                        row: entry.row,
                        col: entry.col,
                    });
                }
                let (row, col) = if entry.row >= entry.col {
                    (entry.row, entry.col)
                } else {
                    (entry.col, entry.row)
                };
                let matrix = if is_mass {
                    PencilMatrix::Mass
                } else {
                    PencilMatrix::Stiffness
                };
                check_entry(matrix, row, col, entry.value)
                    .map_err(SturmError::PencilRangeUnsupported)?;
                merged.push((row, col, is_mass, entry.value));
            }
        }
        merged.sort_by_key(|a| (a.0, a.1, a.2));
        for pair in merged.windows(2) {
            if (pair[0].0, pair[0].1, pair[0].2) == (pair[1].0, pair[1].1, pair[1].2) {
                return Err(SturmError::DuplicateEntry {
                    row: pair[0].0,
                    col: pair[0].1,
                });
            }
        }

        let mut entries: Vec<PencilEntry> = Vec::with_capacity(merged.len() + dimension);
        let mut next_diagonal = 0;
        let push = |entries: &mut Vec<PencilEntry>, row: usize, col: usize| {
            if entries
                .last()
                .is_some_and(|last| (last.row, last.col) == (row, col))
            {
                return;
            }
            entries.push(PencilEntry {
                row,
                col,
                stiffness: 0.0,
                mass: 0.0,
            });
        };
        for &(row, col, is_mass, value) in &merged {
            // Every diagonal is present, in its place in (row, col) order.
            while next_diagonal < row {
                push(&mut entries, next_diagonal, next_diagonal);
                next_diagonal += 1;
            }
            push(&mut entries, row, col);
            let last = entries.last_mut().expect("just pushed");
            if is_mass {
                last.mass = value;
            } else {
                last.stiffness = value;
            }
            if row == col {
                next_diagonal = row + 1;
            }
        }
        while next_diagonal < dimension {
            push(&mut entries, next_diagonal, next_diagonal);
            next_diagonal += 1;
        }
        // Zero values were not given as entries, so drop positions where
        // both are zero (the diagonal is always kept).
        entries.retain(|e| e.row == e.col || e.stiffness != 0.0 || e.mass != 0.0);

        let mut scaling = vec![0.0; dimension];
        for e in entries.iter().filter(|e| e.row == e.col) {
            if e.stiffness <= 0.0 {
                return Err(SturmError::NonPositiveStiffnessDiagonal { index: e.row });
            }
            let d = 1.0 / e.stiffness.sqrt();
            if !d.is_normal() {
                return Err(SturmError::ScalingOutOfRange { index: e.row });
            }
            scaling[e.row] = d;
        }

        let mut stiffness_row_sums = vec![0.0; dimension];
        let mut mass_row_sums = vec![0.0; dimension];
        for e in &entries {
            let k = scale(e.stiffness.abs(), scaling[e.row], scaling[e.col]);
            let m = scale(e.mass.abs(), scaling[e.row], scaling[e.col]);
            stiffness_row_sums[e.row] += k;
            mass_row_sums[e.row] += m;
            if e.row != e.col {
                stiffness_row_sums[e.col] += k;
                mass_row_sums[e.col] += m;
            }
        }
        for i in 0..dimension {
            if !(stiffness_row_sums[i].is_finite() && mass_row_sums[i].is_finite()) {
                return Err(SturmError::ScalingOutOfRange { index: i });
            }
        }

        let mut adjacency = vec![Vec::new(); dimension];
        for e in entries.iter().filter(|e| e.row != e.col) {
            adjacency[e.row].push(e.col);
            adjacency[e.col].push(e.row);
        }
        let order = reverse_cuthill_mckee(&adjacency)
            .expect("the adjacency indices are within the dimension");

        Ok(Self {
            dimension,
            entries,
            scaling,
            stiffness_row_sums,
            mass_row_sums,
            order,
        })
    }

    pub fn dimension(&self) -> usize {
        self.dimension
    }

    /// d_i = fl(1/fl(√k_ii)).
    pub fn scaling(&self) -> &[f64] {
        &self.scaling
    }

    /// The reverse Cuthill–McKee order of the union pattern: `order[k]` is
    /// the pencil index of ordered position k.
    pub fn order(&self) -> &[usize] {
        &self.order
    }

    /// The number of union-pattern positions in the lower triangle,
    /// diagonal included.
    pub fn pattern_entry_count(&self) -> usize {
        self.entries.len()
    }

    /// φ̃ᵀM̃φ̃ / ‖φ̃‖₂² = φᵀMφ / Σ_i k_ii·φ_i², rounded once. φ̃ = D⁻¹φ is
    /// taken with the exact D = diag(k_ii^-½). Each weighted term w·φ_i·φ_j
    /// enters both sums exactly ([`ExactSum`]), so a square below or above
    /// binary64 still meets its weight (review round 2) and no contribution
    /// is lost before a cancellation (review round 4); the sums are rounded
    /// once and divided in `WideDd`. The vector is nonzero and finite.
    fn mass_quotient(&self, vector: &[f64]) -> f64 {
        let mut quadratic = ExactSum::new();
        let mut norm = ExactSum::new();
        for e in &self.entries {
            let (x, y) = (vector[e.row], vector[e.col]);
            if e.row == e.col {
                norm.add_product(&[e.stiffness, x, x]);
                quadratic.add_product(&[e.mass, x, x]);
            } else {
                // 2·m_ij·x·y: the factor 2 as an exponent.
                quadratic.add_product_scaled(&[e.mass, x, y], 1);
            }
        }
        quadratic.to_wide().div(norm.to_wide()).to_f64()
    }

    /// The scaled shifted lower entries, in pattern order, and max|ã_jj|.
    fn form(&self, sigma: f64) -> Result<(Vec<SymmetricMatrixEntry>, f64), RangeViolation> {
        let mut formed = Vec::with_capacity(self.entries.len());
        let mut max_diagonal: f64 = 0.0;
        for e in &self.entries {
            let violation = |operation, value| RangeViolation {
                row: e.row,
                col: e.col,
                operation,
                value,
            };
            let product = sigma * e.mass;
            if e.mass != 0.0 && !product.is_normal() {
                return Err(violation(FormationOperation::ShiftTimesMass, product));
            }
            let a = e.stiffness - product;
            if a != 0.0 && !a.is_normal() {
                return Err(violation(FormationOperation::Difference, a));
            }
            let first = self.scaling[e.row] * a;
            if a != 0.0 && !first.is_normal() {
                return Err(violation(FormationOperation::FirstScaling, first));
            }
            let value = first * self.scaling[e.col];
            if a != 0.0 && !value.is_normal() {
                return Err(violation(FormationOperation::SecondScaling, value));
            }
            if e.row == e.col {
                max_diagonal = max_diagonal.max(value.abs());
            }
            formed.push(SymmetricMatrixEntry {
                row: e.row,
                col: e.col,
                value,
            });
        }
        Ok((formed, max_diagonal))
    }

    /// c_f·u·max_i (Σ_j |k̃_ij| + σ·Σ_j |m̃_ij|).
    fn formation_term(&self, sigma: f64) -> f64 {
        let mut largest: f64 = 0.0;
        for i in 0..self.dimension {
            largest = max_propagating_nan(
                largest,
                self.stiffness_row_sums[i] + sigma * self.mass_row_sums[i],
            );
        }
        FORMATION_ROUNDING_CONSTANT * U * largest
    }
}

fn scale(value: f64, d_i: f64, d_j: f64) -> f64 {
    (d_i * value) * d_j
}

struct PreparedNeighbour {
    eigenvalue: f64,
    relative_error_estimate: f64,
    mass_quotient: f64,
}

fn prepare_neighbours(
    pencil: &ShiftPencil,
    neighbours: &[Neighbour<'_>],
) -> Result<Vec<PreparedNeighbour>, SturmError> {
    neighbours
        .iter()
        .enumerate()
        .map(|(index, nb)| {
            let valid = nb.eigenvalue.is_finite()
                && nb.eigenvalue > 0.0
                && nb.relative_error_estimate.is_finite()
                && nb.relative_error_estimate >= 0.0
                && nb.vector.len() == pencil.dimension
                && nb.vector.iter().all(|x| x.is_finite())
                && nb.vector.iter().any(|&x| x != 0.0);
            if !valid {
                return Err(SturmError::InvalidNeighbour { index });
            }
            let mass_quotient = pencil.mass_quotient(nb.vector);
            if !(mass_quotient.is_normal() && mass_quotient > 0.0) {
                return Err(SturmError::NeighbourMassQuotient {
                    index,
                    value: mass_quotient,
                });
            }
            Ok(PreparedNeighbour {
                eigenvalue: nb.eigenvalue,
                relative_error_estimate: nb.relative_error_estimate,
                mass_quotient,
            })
        })
        .collect()
}

/// Forms, factors and judges one shift.
fn evaluate(
    pencil: &ShiftPencil,
    sigma: f64,
    neighbours: &[PreparedNeighbour],
    parameters: &SturmParameters,
) -> ShiftEvaluation {
    let rejected = |decision, formation_term, formation_separation_ratio| ShiftEvaluation {
        sigma,
        decision,
        formation_term,
        formation_separation_ratio,
        factored: None,
    };
    let (formed, max_abs_scaled_diagonal) = match pencil.form(sigma) {
        Ok(formed) => formed,
        Err(violation) => return rejected(ShiftDecision::OutOfRange(violation), None, None),
    };
    let formation_term = pencil.formation_term(sigma);
    if !formation_term.is_finite() {
        let violation = RangeViolation {
            row: 0,
            col: 0,
            operation: FormationOperation::Norm,
            value: formation_term,
        };
        return rejected(ShiftDecision::OutOfRange(violation), None, None);
    }
    // Every separation is finite before it enters the minimum, so none is
    // dropped by a NaN.
    let mut smallest_separation: Option<f64> = None;
    for (index, nb) in neighbours.iter().enumerate() {
        let s = ((nb.eigenvalue - sigma).abs() - nb.relative_error_estimate * nb.eigenvalue)
            * nb.mass_quotient;
        if !s.is_finite() {
            let decision = ShiftDecision::SeparationOutOfRange {
                neighbour: index,
                value: s,
            };
            return rejected(decision, Some(formation_term), None);
        }
        smallest_separation = Some(smallest_separation.map_or(s, |a: f64| a.min(s)));
    }
    // e_b ≥ the formation term, so this bounds the separation ratio from
    // above before any factorization.
    let formation_separation_ratio = smallest_separation.map(|s| s / formation_term);
    let profile =
        SymmetricProfileMatrix::from_entries_with_order(pencil.dimension, &formed, &pencil.order)
            .expect("the formed entries and the order are valid by construction");
    drop(formed);
    let factorization = match factorize_ldlt(&profile) {
        Ok(factorization) => factorization,
        Err(error) => {
            // Unseparated whatever the factor term would have added.
            let decision = if formation_separation_ratio
                .is_some_and(|ratio| ratio < parameters.separation_multiple)
            {
                ShiftDecision::Unseparated
            } else {
                ShiftDecision::FactorizationRejected(error)
            };
            return rejected(decision, Some(formation_term), formation_separation_ratio);
        }
    };
    drop(profile);

    let n = pencil.dimension;
    let (factor_norm, max_abs_lower_factor, max_abs_pivot) = factor_observations(&factorization, n);
    let nu_f = n as f64 * U;
    let gamma_n = nu_f / (1.0 - nu_f);
    let factor_term = gamma_n * factor_norm;
    let backward_error = factor_term + formation_term;
    let (inverse_norm_estimate, inverse_norm_solves) =
        inverse_one_norm_estimate(n, |x| factorization.solve_ordered(x).ok());
    let conditioning_product = backward_error * inverse_norm_estimate;
    let conditioning_limit = 1.0 / parameters.conditioning_multiple;

    let separated =
        smallest_separation.is_none_or(|s| s >= parameters.separation_multiple * backward_error);
    let separation_ratio = smallest_separation.map(|s| s / backward_error);

    let growth = if max_abs_scaled_diagonal > 0.0 {
        let l = max_abs_lower_factor.max(1.0);
        l * l * max_abs_pivot / max_abs_scaled_diagonal
    } else {
        f64::INFINITY
    };
    let within_growth_limit = growth <= parameters.growth_limit;
    let conditioned = conditioning_product <= conditioning_limit;

    let decision = if !separated {
        ShiftDecision::Unseparated
    } else if !within_growth_limit {
        ShiftDecision::GrowthLimit
    } else if !conditioned {
        ShiftDecision::IllConditioned
    } else {
        ShiftDecision::Accepted
    };
    ShiftEvaluation {
        sigma,
        decision,
        formation_term: Some(formation_term),
        formation_separation_ratio,
        factored: Some(FactoredShift {
            negative_pivot_count: factorization.report().nonpositive_pivot_count,
            factor_term,
            backward_error,
            inverse_norm_estimate,
            inverse_norm_solves,
            conditioning_product,
            separation_ratio,
            growth,
            max_abs_lower_factor,
            max_abs_pivot,
            max_abs_scaled_diagonal,
            separated,
            within_growth_limit,
            conditioned,
        }),
    }
}

/// ‖|L̂||D̂||L̂ᵀ|‖_∞, max|L̂_ij| and max|d̂_j|, in two traversals of the
/// factor's profile with two n-vectors: v = |L̂|ᵀ·1 (unit diagonal
/// included), then w = |L̂|·(|D̂|·v).
fn factor_observations(factorization: &ProfileFactorization, n: usize) -> (f64, f64, f64) {
    let mut v = vec![1.0; n];
    let mut max_abs_lower: f64 = 0.0;
    for (_, col, l) in factorization.lower_factor_entries() {
        v[col] += l.abs();
        max_abs_lower = max_abs_lower.max(l.abs());
    }
    let mut max_abs_pivot: f64 = 0.0;
    for (value, d) in v.iter_mut().zip(factorization.pivots()) {
        *value *= d.abs();
        max_abs_pivot = max_abs_pivot.max(d.abs());
    }
    // v now holds t = |D̂|·v; w starts as the unit-diagonal term.
    let mut w = v.clone();
    for (row, col, l) in factorization.lower_factor_entries() {
        w[row] += l.abs() * v[col];
    }
    // A NaN (0·∞ after an overflowing v) propagates into e_b, whose
    // comparisons then refuse the shift.
    let norm = w.iter().fold(0.0f64, |acc, &x| max_propagating_nan(acc, x));
    (norm, max_abs_lower, max_abs_pivot)
}

/// σ at log-fraction t of the gap (lo, hi): repeated geometric means
/// √a·√b, halving the log-interval toward t until its midpoint is t.
fn log_position(lo: f64, hi: f64, t: f64) -> f64 {
    let (mut a, mut b) = (lo, hi);
    let (mut ta, mut tb) = (0.0, 1.0);
    loop {
        let mid = a.sqrt() * b.sqrt();
        let tm = 0.5 * (ta + tb);
        if t == tm {
            return mid;
        }
        if t < tm {
            b = mid;
            tb = tm;
        } else {
            a = mid;
            ta = tm;
        }
    }
}

fn evidence(placement: Placement, parameters: &SturmParameters) -> SturmEvidence {
    SturmEvidence {
        placement,
        conditioning_multiple: parameters.conditioning_multiple,
        conditioning_limit: 1.0 / parameters.conditioning_multiple,
        separation_multiple: parameters.separation_multiple,
        growth_limit: parameters.growth_limit,
        candidates: Vec::new(),
    }
}

/// Tries the candidates in order, stopping at the first accepted. A
/// candidate given with a decision is recorded without being formed.
fn run_candidates(
    pencil: &ShiftPencil,
    candidates: impl Iterator<Item = (f64, Option<ShiftDecision>)>,
    neighbours: &[PreparedNeighbour],
    parameters: &SturmParameters,
    mut evidence: SturmEvidence,
) -> SturmOutcome {
    for (sigma, early) in candidates.take(parameters.max_candidates) {
        let evaluation = match early {
            None => evaluate(pencil, sigma, neighbours, parameters),
            Some(decision) => ShiftEvaluation {
                sigma,
                decision,
                formation_term: None,
                formation_separation_ratio: None,
                factored: None,
            },
        };
        let accepted = evaluation.decision == ShiftDecision::Accepted;
        let count = evaluation.factored.as_ref().map(|f| f.negative_pivot_count);
        evidence.candidates.push(evaluation);
        if accepted {
            return SturmOutcome::Counted {
                count: count.expect("an accepted shift was factored"),
                evidence,
            };
        }
    }
    let furthest = evidence
        .candidates
        .iter()
        .map(|c| c.decision.stage())
        .max()
        .unwrap_or(0);
    let reason = match furthest {
        0 => SturmRefusal::OutOfRange,
        1 | 3 => SturmRefusal::GrowthLimit,
        2 => SturmRefusal::Unseparated,
        _ => SturmRefusal::IllConditioned,
    };
    SturmOutcome::Refused { reason, evidence }
}

/// The count at one given shift, under the premises, with `neighbours` the
/// nearest member of each neighbouring computed cluster.
pub fn count_at_shift(
    pencil: &ShiftPencil,
    sigma: f64,
    neighbours: &[Neighbour<'_>],
    parameters: &SturmParameters,
) -> Result<SturmOutcome, SturmError> {
    parameters.validate()?;
    if !(sigma.is_finite() && sigma > 0.0) {
        return Err(SturmError::InvalidShift);
    }
    let prepared = prepare_neighbours(pencil, neighbours)?;
    Ok(run_candidates(
        pencil,
        std::iter::once((sigma, None)),
        &prepared,
        parameters,
        evidence(Placement::Given, parameters),
    ))
}

/// The count at a shift placed in a gap between computed clusters: the
/// geometric mean √λ_lo·√λ_hi first, then the caller's log-positions, at
/// most s_max candidates. A candidate that rounds onto or outside an end is
/// not factored (`OutsideGap`).
pub fn count_in_gap(
    pencil: &ShiftPencil,
    gap: &Gap<'_>,
    parameters: &SturmParameters,
) -> Result<SturmOutcome, SturmError> {
    parameters.validate()?;
    let (lo, mut given) = match gap.lower {
        GapLower::Mode(nb) => (nb.eigenvalue, vec![nb]),
        GapLower::Bound(bound) => (bound, Vec::new()),
    };
    let hi = gap.upper.eigenvalue;
    if !(lo.is_finite() && lo > 0.0 && hi.is_finite() && lo < hi) {
        return Err(SturmError::InvalidGap);
    }
    given.push(gap.upper);
    let prepared = prepare_neighbours(pencil, &given)?;
    let positions = std::iter::once(0.5).chain(parameters.gap_log_positions.iter().copied());
    let candidates = positions.map(|t| {
        let sigma = log_position(lo, hi, t);
        let outside = !(sigma > lo && sigma < hi);
        (sigma, outside.then_some(ShiftDecision::OutsideGap))
    });
    Ok(run_candidates(
        pencil,
        candidates,
        &prepared,
        parameters,
        evidence(
            Placement::Gap {
                lower: lo,
                upper: hi,
            },
            parameters,
        ),
    ))
}

/// The whole-spectrum final shift: σ = fl(λ_top·fl(1 + r)) for the
/// caller's offsets r in order, separated on the lower side only. Offered
/// only when the caller asserts an established finite-mode count (design
/// §3.2 item 5); the count taken there is compared with it by the caller.
pub fn count_above_spectrum(
    pencil: &ShiftPencil,
    top: &Neighbour<'_>,
    established_finite_mode_count: usize,
    parameters: &SturmParameters,
) -> Result<SturmOutcome, SturmError> {
    parameters.validate()?;
    let prepared = prepare_neighbours(pencil, std::slice::from_ref(top))?;
    let candidates = parameters.whole_spectrum_offsets.iter().map(|&r| {
        let sigma = top.eigenvalue * (1.0 + r);
        let overflow = !sigma.is_finite();
        let violation = RangeViolation {
            row: 0,
            col: 0,
            operation: FormationOperation::Norm,
            value: sigma,
        };
        (
            sigma,
            overflow.then_some(ShiftDecision::OutOfRange(violation)),
        )
    });
    Ok(run_candidates(
        pencil,
        candidates,
        &prepared,
        parameters,
        evidence(
            Placement::AboveSpectrum {
                top_eigenvalue: top.eigenvalue,
                established_finite_mode_count,
            },
            parameters,
        ),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parameters() -> SturmParameters {
        // Test-local values; the production values are recorded under
        // DEC-026, not here.
        SturmParameters {
            conditioning_multiple: 1e4,
            separation_multiple: 1e2,
            growth_limit: 1e8,
            max_candidates: 7,
            gap_log_positions: vec![0.25, 0.75, 0.125, 0.375, 0.625, 0.875],
            whole_spectrum_offsets: vec![1.0, 0.5, 0.25],
        }
    }

    #[test]
    fn log_positions_are_nested_geometric_means() {
        assert_eq!(log_position(1.0, 16.0, 0.5), 4.0);
        assert_eq!(log_position(1.0, 16.0, 0.25), 2.0);
        assert_eq!(log_position(1.0, 16.0, 0.75), 8.0);
        assert_eq!(log_position(1.0, 256.0, 0.375), 8.0);
        // Formed without overflow at the top of the range.
        let top = log_position(1e300, f64::MAX, 0.5);
        assert!(top.is_finite() && top > 1e300);
    }

    #[test]
    fn parameters_are_validated() {
        let base = parameters();
        let mut bad = base.clone();
        bad.conditioning_multiple = 0.5;
        let pencil = ShiftPencil::from_dense(
            &SymmetricMatrix::from_diagonal(&[1.0]).unwrap(),
            &SymmetricMatrix::from_diagonal(&[1.0]).unwrap(),
        )
        .unwrap();
        assert_eq!(
            count_at_shift(&pencil, 0.5, &[], &bad),
            Err(SturmError::InvalidParameter {
                name: "conditioning_multiple"
            })
        );
        let mut bad = base.clone();
        bad.gap_log_positions = vec![0.3];
        assert_eq!(
            count_at_shift(&pencil, 0.5, &[], &bad),
            Err(SturmError::InvalidParameter {
                name: "gap_log_positions"
            })
        );
        let mut bad = base.clone();
        bad.max_candidates = 0;
        assert!(count_at_shift(&pencil, 0.5, &[], &bad).is_err());
        assert_eq!(
            count_at_shift(&pencil, -1.0, &[], &base),
            Err(SturmError::InvalidShift)
        );
    }

    #[test]
    fn pencil_requires_a_positive_stiffness_diagonal_and_unique_entries() {
        let k = SymmetricMatrix::from_rows(&[vec![1.0, 0.0], vec![0.0, 0.0]]).unwrap();
        let m = SymmetricMatrix::from_diagonal(&[1.0, 1.0]).unwrap();
        assert_eq!(
            ShiftPencil::from_dense(&k, &m),
            Err(SturmError::NonPositiveStiffnessDiagonal { index: 1 })
        );
        let entry = |row, col, value| SymmetricMatrixEntry { row, col, value };
        assert_eq!(
            ShiftPencil::from_entries(
                2,
                &[
                    entry(0, 0, 1.0),
                    entry(1, 1, 1.0),
                    entry(0, 1, 0.5),
                    entry(1, 0, 0.5)
                ],
                &[]
            ),
            Err(SturmError::DuplicateEntry { row: 1, col: 0 })
        );
        // The union pattern holds positions from either matrix.
        let pencil = ShiftPencil::from_entries(
            3,
            &[
                entry(0, 0, 4.0),
                entry(1, 1, 1.0),
                entry(2, 2, 1.0),
                entry(1, 0, 1.0),
            ],
            &[entry(2, 1, 0.5), entry(0, 0, 1.0)],
        )
        .unwrap();
        assert_eq!(pencil.pattern_entry_count(), 5);
        assert_eq!(pencil.scaling(), &[0.5, 1.0, 1.0]);
    }

    #[test]
    fn factor_norm_is_the_row_sum_of_the_absolute_product() {
        // A = [[4, 2, 0], [2, 5, 3], [0, 3, 6]]: L = [[1], [1/2, 1], [0, 3/4, 1]],
        // D = (4, 4, 15/4). |L||D||Lᵀ| = A here (no cancellation), so the
        // ∞-norm is the largest row sum of |A|, 10.
        let profile = SymmetricProfileMatrix::from_dense(&[
            vec![4.0, 2.0, 0.0],
            vec![2.0, 5.0, 3.0],
            vec![0.0, 3.0, 6.0],
        ])
        .unwrap();
        let factorization = factorize_ldlt(&profile).unwrap();
        let (norm, max_l, max_d) = factor_observations(&factorization, 3);
        assert_eq!(norm, 10.0);
        assert_eq!(max_l, 0.75);
        assert_eq!(max_d, 4.0);
    }

    #[test]
    fn inverse_norm_estimate_is_exact_on_a_diagonal() {
        let profile =
            SymmetricProfileMatrix::from_dense(&[vec![2.0, 0.0], vec![0.0, -0.25]]).unwrap();
        let factorization = factorize_ldlt(&profile).unwrap();
        let (nu, solves) = inverse_one_norm_estimate(2, |x| factorization.solve_ordered(x).ok());
        assert_eq!(nu, 4.0);
        assert!(solves >= 3);
    }
}
