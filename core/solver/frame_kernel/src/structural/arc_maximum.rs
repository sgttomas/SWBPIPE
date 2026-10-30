//! T4-U4b: the certified along-arc stress maximum of a realized arc
//! (DEL-04-07 Design `u4b-u5b-implementation.md` §2.3 and §2.6; the
//! definitions are those of `validation/references/t4_u4b/README.md` item 4).
//!
//! # The represented problem
//!
//! The operands are binary64 values: the bend radius R, the included angle
//! φ ∈ (0, π], the rows of the end-j tangent frame in global components, the
//! end-j action [N, V_y, V_z, T, M_y, M_z] in that frame (the node-on-element
//! force at j, which is the j-side section action at end_j; N is the
//! pressure-free statics force S on a pressurized arc), the uniform
//! intensities as (global force per unit arc length, scale) pairs, an
//! optional constant pA_i, and the basis A and Z. The represented problem is
//! the exact real function of ψ ∈ [0, φ] (ψ from the authored node i) that
//! these values define through the chain below. With Δ = φ − ψ, c = cos Δ,
//! s = sin Δ, in the coordinates of the end-j frame E (rows x, y, z):
//!
//! - F(ψ) = F_j + RΔ w;
//! - M(ψ) = M_j + d × F_j + R² b × w, with d = (R s, −R(1 − c), 0) and
//!   b = (Δ s − (1 − c), Δ c − s, 0);
//! - the tangent frame at ψ is t = (c, −s, 0), n = (s, c, 0), z;
//!   N = F·t (+ pA_i), V_y = F·n, V_z = F_z, T = M·t, M_y = M·n, M_z = M_z.
//!
//! The exact derivatives in ψ are N′ = −R w·t + V_y, M_y′ = R V_z − T and
//! M_z′ = −R V_y (t4_u4b item 4). The elastic measure is
//! f = |N_eff|/A + hypot(M_y, M_z)/Z; the directional intensified measure is
//! g = hypot(i_ip M_z, i_op M_y)/Z (§2.6). Both are suprema of functions with
//! second derivative ≥ −K (item 4: N″ = −2R w_y − N, M_y″ = −R² w_z − M_y,
//! M_z″ = R² w_y + R N), so on a cell of width h the maximum is at most the
//! larger endpoint value plus K h²/8, with
//! K = (2R|w| + F_max)/A + (R²|w| + M_max + R F_max)/Z, or
//! max(i_ip, i_op)(R²|w| + M_max + R F_max)/Z for g, F_max = |F_j| + |w|Rφ and
//! M_max = |M_j| + RφF_max (both bound the whole arc, because |dF/dψ| = R|w|
//! and |dM/dψ| = R|F|). K is formed upward in binary64, with |F_j|, |M_j|
//! and |w| bounded from the enclosed chained operands.
//!
//! # The operand chain (K2 review, option A)
//!
//! Design §2.3 forms the problem from the published end_j rows taken through
//! the transpose of the end-j frame, and PP's station function evaluates that
//! global action in the coordinates of its rounded arc frame, with an exact
//! rotation giving the tangent frame at ψ. The certificate encloses exactly
//! that chain. The global end-j force and moment are g_F = Eᵀ f_F and
//! g_M = Eᵀ f_M, formed exactly from the rows; they are expressed in the
//! coordinates of the same rounded frame, F_j = E g_F = E Eᵀ f_F and
//! M_j = E Eᵀ f_M, and w = E Σ_k scale_k g_k; the closed form above then runs
//! in those coordinates. Every product is an exact real product enclosed in
//! balls. Rounded direction-cosine rows are not exactly orthogonal, so
//! E Eᵀ ≠ I, and the round trip is enclosed rather than dropped: Codex's
//! rotated small-angle torsion control differs from the direct-local function
//! by 2.8e-9 Pa, more than the gap, and is a test here.
//!
//! Option A (enclose the specified chain) is chosen over option B (keep the
//! direct-local function and widen every evaluation by a frame-defect bound)
//! because it certifies the specified function itself: the gap keeps its
//! meaning, no extra bound is introduced, and it costs 60 ball operations per
//! call. PP's coordinate frame is the macro element's centre frame C rather
//! than E. If C were an exact rotation of E about the bend-plane normal, the
//! two real functions would coincide, because the tangent frame at ψ is that
//! same rotation; C and E are both roundings of one exact geometry, so their
//! difference is part of the station rows' binary64 formation, which the
//! station cover below absorbs upward.
//!
//! # The station cover (§2.3 step 5)
//!
//! The published station values of this arc and basis (max(|σ⁺|, |σ⁻|), or
//! the intensified rows), converted to Pa and rounded upward by the caller,
//! only raise the upper bound; the gap is then checked again. They are not
//! checked against the represented problem: PP's station rows are separately
//! rounded binary64 sums, and no rigorous bound of their formation error is
//! claimed (the K2 review found load-cancellation and short-arc cases where
//! that error is far above any simple allowance). No allowance is therefore
//! needed on the certified route either.
//!
//! The operands are the caller's to get right. In particular, N_w passed as
//! the end-j axial together with pA_i counts the pressure term twice: on the
//! t4_u4b arcs it yields a maximum above the true one, or, where N_w changes
//! sign along the arc, one below the station rows that the cover then
//! refuses; a kernel test shows both. PP's
//! integration test must bind each arc maximum to its own station rows (the
//! station values may not exceed the maximum beyond rounding, and the
//! maximum may not exceed them beyond the along-arc peak), so that such an
//! operand error is caught on the product side.
//!
//! # Arithmetic
//!
//! Every evaluation is an enclosure in `arc_certificate`'s ball arithmetic
//! (`Wide2` midpoints at p = 128, directed binary64 radii). The evaluation
//! points are exact: ψ = (k/2⁶⁰)φ with an integer k, and Δ = ((2⁶⁰ − k)/2⁶⁰)φ
//! is formed exactly (a 61-bit integer times a 53-bit φ fits in 128 bits).
//! Sine and cosine of an argument ball in [0, π] are the Taylor polynomial at
//! the ball's midpoint, with the Lagrange remainder |x|ᵏ/k! enclosed in the
//! same arithmetic, plus the ball's radius (both functions are 1-Lipschitz).
//! No libm transcendental enters a bound. The balls are then turned into
//! directed binary64 intervals: every lower value is rounded downward and
//! every upper value upward.
//!
//! # Method (§2.3 steps 1–7)
//!
//! A uniform grid of 4,096 cells; branch and bound on the cell bound above
//! with `elastic_extrema`'s gap rule (gap ≤ 1e-12 Pa + 1e-12·lower) and
//! limits (131,072 subdivisions, depth 48), refused by name when a limit is
//! reached first; the station cover; the location rule in the design's
//! order; and the binary64 midpoint of [value_lower, upper_bound].
//!
//! The published fraction is k/2⁶⁰ rounded to the nearest binary64. It is
//! exact at the ends and whenever k has at most 53 significant bits, which
//! holds for every grid point and for cells to depth 41 below the grid;
//! deeper bracket midpoints can carry up to 60 significant bits, and the
//! fraction is then within half an ulp (2⁻⁵⁴ relative) of k/2⁶⁰, far inside
//! the 2⁻⁴⁰ location width.

use super::arc_certificate::{abs_up, add_up, div_up, mul_up, ArcCertificateFailure, Ball, Balls};
use super::retained::wide::{Wide2, WideArith, WideError};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// Initial uniform cells (§2.3 step 1).
pub const ARC_MAXIMUM_INITIAL_CELLS: usize = 4096;
/// The existing `elastic_extrema` subdivision limit (§2.3 step 4).
pub const ARC_MAXIMUM_SUBDIVISION_LIMIT: usize = 131_072;
/// The existing `elastic_extrema` depth limit (§2.3 step 4).
pub const ARC_MAXIMUM_DEPTH_LIMIT: u32 = 48;

const RELATIVE_GAP: f64 = 1e-12;
const ABSOLUTE_GAP_PA: f64 = 1e-12;
/// Evaluation points are ψ = (k/2⁶⁰)φ.
const FRACTION_BITS: i64 = 60;
const FULL: u64 = 1 << FRACTION_BITS;
/// Width of an initial cell in k (2⁶⁰/4096).
const GRID_STEP: u64 = FULL / ARC_MAXIMUM_INITIAL_CELLS as u64;
/// The location bisection stops at a bracket of at most 2⁻⁴⁰·φ.
const LOCATION_WIDTH: u64 = 1 << 20;
/// 2⁻⁶⁰ (exact).
const TWO_POW_MINUS_60: f64 = f64::from_bits((1023 - 60) << 52);
/// Taylor terms the sine and cosine may use (π⁵⁰/50! < 2⁻¹³⁰; enforced).
const TRIG_MAX_TERMS: usize = 64;
/// 2⁻¹³⁰ (exact): the remainder target, relative to min(1, |x|).
const TRIG_TARGET: f64 = f64::from_bits((1023 - 130) << 52);

/// One uniform intensity: a constant global force per unit arc length and
/// its load-state scale; the represented intensity is scale·global.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ArcUniformIntensity {
    pub global: [f64; 3],
    pub scale: f64,
}

/// The binary64 operands of one arc and basis (module documentation).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ArcStressMaximumProblem<'a> {
    /// R > 0 (m).
    pub bend_radius: f64,
    /// φ ∈ (0, π] (rad), the binary64 included angle the station rows use.
    pub included_angle: f64,
    /// Rows x (tangent toward j), y (toward the centre) and z (bend-plane
    /// normal) of the end-j tangent frame, in global components: the frame
    /// whose transpose carries the end-j rows to global, and whose
    /// coordinates the closed form runs in (module documentation).
    pub end_j_axes: [[f64; 3]; 3],
    /// [N, V_y, V_z, T, M_y, M_z] at end j in the end-j tangent frame (N, N m):
    /// the node-on-element force at j, equal to the j-side section action at
    /// end_j. N is the pressure-free statics force (S on a pressurized arc).
    pub end_j_force: [f64; 6],
    /// The arc's uniform intensities with their load-state scales.
    pub uniform_intensities: &'a [ArcUniformIntensity],
    /// The constant pA_i of a pressurized arc (N); N_eff = N + pA_i.
    pub pressure_area_product: Option<f64>,
    /// Basis area A > 0 (m²).
    pub area: f64,
    /// Basis section modulus Z > 0 (m³).
    pub section_modulus: f64,
}

/// The measure maximized along the arc (Pa).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArcStressMeasure {
    /// |N_eff|/A + hypot(M_y, M_z)/Z (§2.3).
    ElasticNormal,
    /// hypot(i_ip·M_z, i_op·M_y)/Z, with finite factors > 0 (§2.6).
    DirectionalIntensified { in_plane: f64, out_of_plane: f64 },
}

/// How the published location was decided (§2.3 step 6).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArcMaximumLocation {
    /// The end rule at ψ = 0 (f′(0) strictly negative): fraction exactly 0.
    EndI,
    /// The end rule at ψ = φ (f′(φ) strictly positive): fraction exactly 1.
    EndJ,
    /// The midpoint of a sign bracket of f′ refined to at most 2⁻⁴⁰·φ.
    Bracket,
    /// The trial point at which the bisection met an f′ enclosure containing 0.
    UnresolvedSign,
    /// No bracket and no end rule: the governing evaluation point.
    GoverningEvaluation,
}

/// A certified arc maximum (Pa). The true maximum of the represented problem
/// lies in [value_lower, upper_bound]; solution error is separate.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CertifiedArcStressMaximum {
    /// The largest directed lower value of any evaluation.
    pub value_lower: f64,
    /// The governing evaluation's upper value, capped by the upper bound.
    pub value_upper: f64,
    /// The global upper bound, after the station cover.
    pub upper_bound: f64,
    /// next_up(upper_bound − value_lower), or 0 when they are equal.
    pub certified_gap: f64,
    /// value_lower + 0.5·(upper_bound − value_lower), as straights publish.
    pub midpoint: f64,
    /// ψ*/φ of the published location (exactly 0 or 1 at an end).
    pub fraction: f64,
    pub location: ArcMaximumLocation,
    /// Cells bisected by the branch and bound.
    pub subdivisions: usize,
    /// K, the curvature bound used (Pa/rad²).
    pub curvature_bound: f64,
}

/// The best evaluation when a limit stopped the bound: diagnostics only.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ArcMaximumDiagnostic {
    pub value_lower: f64,
    pub upper_bound: f64,
    pub fraction: f64,
    pub subdivisions: usize,
}

/// Why no certified arc maximum is returned. `reason()` names it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArcMaximumFailure {
    /// A non-finite operand, or one outside its stated range.
    InvalidOperand { name: &'static str },
    /// A ball operation failed its precondition.
    Enclosure(ArcCertificateFailure),
    /// A sine or cosine argument ball left [0, π].
    TrigArgumentOutsideDomain,
    /// The Taylor series did not reach its remainder target.
    TrigTermLimit,
    /// K is not finite.
    CurvatureBoundNotFinite,
    /// 131,072 subdivisions were reached before the gap closed.
    SubdivisionLimit(ArcMaximumDiagnostic),
    /// A cell at depth 48 had to be bisected before the gap closed.
    DepthLimit(ArcMaximumDiagnostic),
    /// The station cover raised the upper bound beyond the gap rule.
    StationCoverNotClosed(ArcMaximumDiagnostic),
}

impl ArcMaximumFailure {
    /// The named reason carried by the unavailable path.
    pub fn reason(&self) -> &'static str {
        match self {
            Self::InvalidOperand { .. } => "arc_maximum_invalid_operand",
            Self::Enclosure(_) => "arc_maximum_enclosure_failed",
            Self::TrigArgumentOutsideDomain => "arc_maximum_trig_argument_outside_domain",
            Self::TrigTermLimit => "arc_maximum_trig_term_limit",
            Self::CurvatureBoundNotFinite => "arc_maximum_curvature_bound_not_finite",
            Self::SubdivisionLimit(_) => "arc_maximum_subdivision_limit",
            Self::DepthLimit(_) => "arc_maximum_depth_limit",
            Self::StationCoverNotClosed(_) => "arc_maximum_station_cover_not_closed",
        }
    }
}

impl From<ArcCertificateFailure> for ArcMaximumFailure {
    fn from(error: ArcCertificateFailure) -> Self {
        Self::Enclosure(error)
    }
}

impl From<WideError> for ArcMaximumFailure {
    fn from(error: WideError) -> Self {
        Self::Enclosure(ArcCertificateFailure::Wide(error))
    }
}

type Failure = ArcMaximumFailure;

/// Certify the along-arc maximum of `measure` for one arc and basis (§2.3).
///
/// `station_cover_pa` holds the published station values of this arc and
/// basis (max(|σ⁺|, |σ⁻|), or the intensified rows), converted to Pa and
/// rounded upward by the caller. They only raise the upper bound, and the gap
/// is then checked again (step 5; module documentation).
pub fn certify_arc_stress_maximum(
    problem: &ArcStressMaximumProblem<'_>,
    measure: ArcStressMeasure,
    station_cover_pa: &[f64],
) -> Result<CertifiedArcStressMaximum, ArcMaximumFailure> {
    certify_with_limits(
        problem,
        measure,
        station_cover_pa,
        ARC_MAXIMUM_SUBDIVISION_LIMIT,
        ARC_MAXIMUM_DEPTH_LIMIT,
    )
}

pub(super) fn certify_with_limits(
    problem: &ArcStressMaximumProblem<'_>,
    measure: ArcStressMeasure,
    station_cover_pa: &[f64],
    split_limit: usize,
    depth_limit: u32,
) -> Result<CertifiedArcStressMaximum, Failure> {
    for &value in station_cover_pa {
        if !value.is_finite() {
            return Err(Failure::InvalidOperand {
                name: "station_cover_pa",
            });
        }
    }
    let mut evaluator = Evaluator::new(problem, measure)?;
    let curvature = evaluator.curvature_bound()?;
    let phi = problem.included_angle;

    // Steps 1–3: the grid, its endpoint values and cell bounds.
    let mut best = Best::default();
    let mut values = Vec::with_capacity(ARC_MAXIMUM_INITIAL_CELLS + 1);
    for j in 0..=ARC_MAXIMUM_INITIAL_CELLS as u64 {
        let k = j * GRID_STEP;
        let value = evaluator.value(k)?;
        best.consider(k, value);
        values.push(value);
    }
    let mut heap = BinaryHeap::with_capacity(ARC_MAXIMUM_INITIAL_CELLS);
    for j in 0..ARC_MAXIMUM_INITIAL_CELLS {
        let (lo, hi) = (j as u64 * GRID_STEP, (j as u64 + 1) * GRID_STEP);
        let (left, right) = (values[j], values[j + 1]);
        let upper = cell_upper(left, right, hi - lo, phi, curvature)?;
        heap.push(Cell {
            lo,
            hi,
            depth: 0,
            upper,
            left,
            right,
        });
    }

    // Step 4: bisect until the gap closes or a limit is reached.
    let mut subdivisions = 0usize;
    let upper = loop {
        while heap.peek().is_some_and(|c: &Cell| c.upper < best.lower) {
            heap.pop();
        }
        let upper = heap.peek().map_or(best.lower, |c| c.upper.max(best.lower));
        if gap(best.lower, upper) <= tolerance(best.lower) {
            break upper;
        }
        let diagnostic = ArcMaximumDiagnostic {
            value_lower: best.lower,
            upper_bound: upper,
            fraction: fraction_of(best.k),
            subdivisions,
        };
        if subdivisions >= split_limit {
            return Err(Failure::SubdivisionLimit(diagnostic));
        }
        let cell = heap
            .pop()
            .expect("an open cell exists while the gap is open");
        if cell.depth >= depth_limit || cell.hi - cell.lo < 2 {
            return Err(Failure::DepthLimit(diagnostic));
        }
        let mid = cell.lo + (cell.hi - cell.lo) / 2;
        let value = evaluator.value(mid)?;
        best.consider(mid, value);
        for (lo, hi, left, right) in [
            (cell.lo, mid, cell.left, value),
            (mid, cell.hi, value, cell.right),
        ] {
            // A child lies in its parent: both bounds hold, keep the smaller.
            let upper = cell_upper(left, right, hi - lo, phi, curvature)?.min(cell.upper);
            if upper >= best.lower {
                heap.push(Cell {
                    lo,
                    hi,
                    depth: cell.depth + 1,
                    upper,
                    left,
                    right,
                });
            }
        }
        subdivisions += 1;
    };

    // Step 5: the station cover.
    let mut upper_bound = upper;
    for &value in station_cover_pa {
        upper_bound = upper_bound.max(value);
    }
    if upper_bound > upper && gap(best.lower, upper_bound) > tolerance(best.lower) {
        return Err(Failure::StationCoverNotClosed(ArcMaximumDiagnostic {
            value_lower: best.lower,
            upper_bound,
            fraction: fraction_of(best.k),
            subdivisions,
        }));
    }

    // Step 6: the location. The value enclosure above does not depend on it.
    let cells = heap.into_vec();
    let (k, location) = locate(&mut evaluator, &best, cells)?;

    // Step 7: the publication.
    let value_lower = best.lower;
    Ok(CertifiedArcStressMaximum {
        value_lower,
        value_upper: best.upper.min(upper_bound),
        upper_bound,
        certified_gap: gap(value_lower, upper_bound),
        midpoint: value_lower + 0.5 * (upper_bound - value_lower),
        fraction: fraction_of(k),
        location,
        subdivisions,
        curvature_bound: curvature,
    })
}

// ---------------------------------------------------------------------------
// The branch and bound
// ---------------------------------------------------------------------------

/// A directed binary64 interval [lo, hi].
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Interval {
    pub(super) lo: f64,
    pub(super) hi: f64,
}

/// The governing evaluation: the largest directed lower value, ties to the
/// smallest ψ.
#[derive(Debug, Clone, Copy)]
struct Best {
    k: u64,
    lower: f64,
    upper: f64,
}

impl Default for Best {
    fn default() -> Self {
        Self {
            k: 0,
            lower: f64::NEG_INFINITY,
            upper: f64::NEG_INFINITY,
        }
    }
}

impl Best {
    fn consider(&mut self, k: u64, value: Interval) {
        if value.lo > self.lower || (value.lo == self.lower && k < self.k) {
            *self = Self {
                k,
                lower: value.lo,
                upper: value.hi,
            };
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Cell {
    lo: u64,
    hi: u64,
    depth: u32,
    upper: f64,
    left: Interval,
    right: Interval,
}

impl PartialEq for Cell {
    fn eq(&self, rhs: &Self) -> bool {
        self.cmp(rhs) == Ordering::Equal
    }
}
impl Eq for Cell {}
impl PartialOrd for Cell {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}
impl Ord for Cell {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.upper
            .total_cmp(&rhs.upper)
            .then_with(|| rhs.lo.cmp(&self.lo))
            .then_with(|| rhs.depth.cmp(&self.depth))
    }
}

/// max(upper endpoint values) + K h²/8, upward, with h = (width/2⁶⁰)φ.
fn cell_upper(
    left: Interval,
    right: Interval,
    width: u64,
    phi: f64,
    k: f64,
) -> Result<f64, Failure> {
    let h = mul_up(phi, width as f64 * TWO_POW_MINUS_60);
    let bow = mul_up(mul_up(k, mul_up(h, h)), 0.125);
    let upper = add_up(left.hi.max(right.hi), bow);
    if upper.is_finite() {
        Ok(upper)
    } else {
        Err(Failure::CurvatureBoundNotFinite)
    }
}

/// `elastic_extrema`'s certified gap: next_up(upper − lower), 0 when equal.
fn gap(lower: f64, upper: f64) -> f64 {
    if upper == lower {
        0.0
    } else {
        (upper - lower).next_up()
    }
}

/// `elastic_extrema`'s stopping gap: 1e-12 Pa + 1e-12·lower, rounded down.
fn tolerance(lower: f64) -> f64 {
    (ABSOLUTE_GAP_PA + (RELATIVE_GAP * lower).next_down().max(0.0))
        .next_down()
        .max(0.0)
}

/// k/2⁶⁰ rounded to the nearest binary64 (module documentation, "Method").
fn fraction_of(k: u64) -> f64 {
    k as f64 * TWO_POW_MINUS_60
}

// ---------------------------------------------------------------------------
// The location (§2.3 step 6)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Sign {
    Positive,
    Negative,
    Unresolved,
}

fn sign_of(slope: Interval) -> Sign {
    if slope.lo > 0.0 {
        Sign::Positive
    } else if slope.hi < 0.0 {
        Sign::Negative
    } else {
        Sign::Unresolved
    }
}

/// The design's order (§2.3 step 6):
/// (a) the end rule: if the governing evaluation (the largest directed lower
///     value, ties to the smallest ψ) is an endpoint and the derivative
///     enclosure there points out of the member (f′(0) < 0 at ψ = 0,
///     f′(φ) > 0 at ψ = φ), the location is exactly 0 or 1;
/// (b) otherwise the interior candidate rule: candidate cells are the
///     surviving cells (upper ≥ the global lower) with an endpoint whose own
///     lower value is within the gap of the global lower; runs of contiguous
///     candidates are taken in increasing ψ (among candidates the smallest ψ
///     governs); in a run, the bracket's right point is the first trial point
///     with a strictly negative f′ preceded by one with a strictly positive
///     f′, its left point the last such positive point, refined by bisection;
/// (c) otherwise the governing evaluation point.
fn locate(
    evaluator: &mut Evaluator,
    best: &Best,
    cells: Vec<Cell>,
) -> Result<(u64, ArcMaximumLocation), Failure> {
    if best.k == 0 && sign_of(evaluator.slope(0)?) == Sign::Negative {
        return Ok((0, ArcMaximumLocation::EndI));
    }
    if best.k == FULL && sign_of(evaluator.slope(FULL)?) == Sign::Positive {
        return Ok((FULL, ArcMaximumLocation::EndJ));
    }
    let lower = best.lower;
    let tol = tolerance(lower);
    let within = |value: f64| value >= lower || (lower - value).next_up() <= tol;
    let mut candidates: Vec<Cell> = cells
        .into_iter()
        .filter(|c| c.upper >= lower && (within(c.left.lo) || within(c.right.lo)))
        .collect();
    candidates.sort_by_key(|c| c.lo);
    let mut runs: Vec<Vec<Cell>> = Vec::new();
    for cell in candidates {
        match runs.last_mut() {
            Some(run) if run.last().is_some_and(|c| c.hi == cell.lo) => run.push(cell),
            _ => runs.push(vec![cell]),
        }
    }
    for run in &runs {
        let mut points = Vec::with_capacity(run.len() + 1);
        points.push(run[0].lo);
        points.extend(run.iter().map(|c| c.hi));
        let mut positive: Option<u64> = None;
        for &k in &points {
            match sign_of(evaluator.slope(k)?) {
                Sign::Positive => positive = Some(k),
                Sign::Negative => {
                    if let Some(left) = positive {
                        return bisect(evaluator, left, k);
                    }
                }
                Sign::Unresolved => {}
            }
        }
    }
    Ok((best.k, ArcMaximumLocation::GoverningEvaluation))
}

fn bisect(
    evaluator: &mut Evaluator,
    mut left: u64,
    mut right: u64,
) -> Result<(u64, ArcMaximumLocation), Failure> {
    while right - left > LOCATION_WIDTH {
        let mid = left + (right - left) / 2;
        match sign_of(evaluator.slope(mid)?) {
            Sign::Positive => left = mid,
            Sign::Negative => right = mid,
            Sign::Unresolved => return Ok((mid, ArcMaximumLocation::UnresolvedSign)),
        }
    }
    Ok((left + (right - left) / 2, ArcMaximumLocation::Bracket))
}

// ---------------------------------------------------------------------------
// The evaluation
// ---------------------------------------------------------------------------

enum Measure {
    Elastic,
    Intensified {
        in_plane: Ball,
        out_of_plane: Ball,
        larger: f64,
    },
}

/// The section action at one ψ, in the tangent frame at ψ, with w·t.
struct Section {
    n_eff: Ball,
    v_y: Ball,
    v_z: Ball,
    t: Ball,
    m_y: Ball,
    m_z: Ball,
    w_t: Ball,
}

pub(super) struct Evaluator {
    b: Balls,
    exact: WideArith,
    trig: Trig,
    phi: Wide2,
    radius: Ball,
    radius_sq: Ball,
    fj: [Ball; 3],
    mj: [Ball; 3],
    w: [Ball; 3],
    pa: Option<Ball>,
    area: f64,
    z: f64,
    measure: Measure,
    /// Upward |F_j|, |M_j| and |w| of the chained operands, and R, φ, for K.
    norms: [f64; 3],
    r: f64,
    phi_f: f64,
}

fn positive(name: &'static str, value: f64) -> Result<f64, Failure> {
    if value.is_finite() && value > 0.0 {
        Ok(value)
    } else {
        Err(Failure::InvalidOperand { name })
    }
}

fn finite(name: &'static str, value: f64) -> Result<f64, Failure> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(Failure::InvalidOperand { name })
    }
}

impl Evaluator {
    pub(super) fn new(
        problem: &ArcStressMaximumProblem<'_>,
        measure: ArcStressMeasure,
    ) -> Result<Self, Failure> {
        let r = positive("bend_radius", problem.bend_radius)?;
        let phi = positive("included_angle", problem.included_angle)?;
        if phi > std::f64::consts::PI {
            return Err(Failure::InvalidOperand {
                name: "included_angle",
            });
        }
        let area = positive("area", problem.area)?;
        let z = positive("section_modulus", problem.section_modulus)?;
        for row in &problem.end_j_axes {
            for &value in row {
                finite("end_j_axes", value)?;
            }
        }
        for &value in &problem.end_j_force {
            finite("end_j_force", value)?;
        }
        for intensity in problem.uniform_intensities {
            finite("uniform_intensity_scale", intensity.scale)?;
            for &value in &intensity.global {
                finite("uniform_intensity", value)?;
            }
        }
        let mut b = Balls::new()?;
        let trig = Trig::new(&mut b)?;
        let measure = match measure {
            ArcStressMeasure::ElasticNormal => Measure::Elastic,
            ArcStressMeasure::DirectionalIntensified {
                in_plane,
                out_of_plane,
            } => {
                let i_ip = positive("in_plane_factor", in_plane)?;
                let i_op = positive("out_of_plane_factor", out_of_plane)?;
                Measure::Intensified {
                    in_plane: b.lift(i_ip)?,
                    out_of_plane: b.lift(i_op)?,
                    larger: i_ip.max(i_op),
                }
            }
        };
        let pa = match problem.pressure_area_product {
            Some(value) => Some(b.lift(finite("pressure_area_product", value)?)?),
            None => None,
        };
        let radius = b.lift(r)?;
        let radius_sq = b.mul(radius, radius)?;
        // The chain (module documentation): F_j = E (Eᵀ f_F), M_j = E (Eᵀ f_M).
        let mut axes = [[Ball::ZERO; 3]; 3];
        for (a, row) in axes.iter_mut().enumerate() {
            for (i, slot) in row.iter_mut().enumerate() {
                *slot = b.lift(problem.end_j_axes[a][i])?;
            }
        }
        let force = problem.end_j_force;
        let mut fj = [Ball::ZERO; 3];
        let mut mj = [Ball::ZERO; 3];
        for (block, out) in [(0usize, &mut fj), (3usize, &mut mj)] {
            let local = [
                b.lift(force[block])?,
                b.lift(force[block + 1])?,
                b.lift(force[block + 2])?,
            ];
            let mut global = [Ball::ZERO; 3];
            for (i, slot) in global.iter_mut().enumerate() {
                let column = [axes[0][i], axes[1][i], axes[2][i]];
                *slot = b.dot(&column, &local)?;
            }
            for (a, slot) in out.iter_mut().enumerate() {
                *slot = b.dot(&axes[a], &global)?;
            }
        }
        // w = Σ_k scale_k·(E_j g_k) in the end-j frame, as exact real values.
        let mut w = [Ball::ZERO; 3];
        for intensity in problem.uniform_intensities {
            let g = [
                b.lift(intensity.global[0])?,
                b.lift(intensity.global[1])?,
                b.lift(intensity.global[2])?,
            ];
            let scale = b.lift(intensity.scale)?;
            for (a, slot) in w.iter_mut().enumerate() {
                let along = b.dot(&axes[a], &g)?;
                let scaled = b.mul(scale, along)?;
                *slot = b.add(*slot, scaled)?;
            }
        }
        let norm_up = |v: &[Ball; 3]| -> Result<f64, Failure> {
            Ok(norm3_up(
                magnitude_interval(bounds(&v[0])?).hi,
                magnitude_interval(bounds(&v[1])?).hi,
                magnitude_interval(bounds(&v[2])?).hi,
            ))
        };
        let force_norm = norm_up(&fj)?;
        let moment_norm = norm_up(&mj)?;
        let w_norm = norm_up(&w)?;
        Ok(Self {
            b,
            exact: WideArith::new(128)?,
            trig,
            phi: Wide2::from_f64(phi)?,
            radius,
            radius_sq,
            fj,
            mj,
            w,
            pa,
            area,
            z,
            measure,
            norms: [force_norm, moment_norm, w_norm],
            r,
            phi_f: phi,
        })
    }

    /// Upward F_max = |F_j| + |w|Rφ and M_max = |M_j| + RφF_max.
    fn maxima(&self) -> (f64, f64) {
        let [force_norm, moment_norm, w_norm] = self.norms;
        let r_phi = mul_up(self.r, self.phi_f);
        let f_max = add_up(force_norm, mul_up(w_norm, r_phi));
        (f_max, add_up(moment_norm, mul_up(r_phi, f_max)))
    }

    /// K, formed upward (module documentation).
    pub(super) fn curvature_bound(&self) -> Result<f64, Failure> {
        let [_, _, w_norm] = self.norms;
        let r = self.r;
        let (f_max, m_max) = self.maxima();
        let moment = add_up(
            add_up(mul_up(mul_up(r, r), w_norm), m_max),
            mul_up(r, f_max),
        );
        let k = match self.measure {
            Measure::Elastic => {
                let axial = add_up(mul_up(2.0, mul_up(r, w_norm)), f_max);
                add_up(div_up(axial, self.area), div_up(moment, self.z))
            }
            Measure::Intensified { larger, .. } => mul_up(larger, div_up(moment, self.z)),
        };
        if k.is_finite() {
            Ok(k)
        } else {
            Err(Failure::CurvatureBoundNotFinite)
        }
    }

    /// Δ = ((2⁶⁰ − k)/2⁶⁰)φ, exactly.
    fn delta(&mut self, k: u64) -> Result<Ball, Failure> {
        debug_assert!(k <= FULL);
        let q = FULL - k;
        let high = Wide2::from_f64((q >> 30) as f64)?.mul_pow2(30)?;
        let low = Wide2::from_f64((q & ((1 << 30) - 1)) as f64)?;
        // At most 61 bits, then at most 114 bits: both exact at p = 128.
        let q = self.exact.add(&high, &low)?;
        let product = self.exact.mul(&q, &self.phi)?;
        Ok(Ball::exact(product.mul_pow2(-FRACTION_BITS)?))
    }

    fn section(&mut self, k: u64) -> Result<Section, Failure> {
        let delta = self.delta(k)?;
        let (s, c) = self.trig.sin_cos(&mut self.b, delta)?;
        let b = &mut self.b;
        let (fj, mj, w) = (self.fj, self.mj, self.w);
        let one_c = b.sub(Ball::ONE, c)?;
        let r_delta = b.mul(self.radius, delta)?;
        let mut f = [Ball::ZERO; 3];
        for i in 0..3 {
            let load = b.mul(r_delta, w[i])?;
            f[i] = b.add(fj[i], load)?;
        }
        let d_x = b.mul(self.radius, s)?;
        let d_y = b.mul(self.radius, one_c)?.neg();
        let ds = b.mul(delta, s)?;
        let b_x = b.sub(ds, one_c)?;
        let dc = b.mul(delta, c)?;
        let b_y = b.sub(dc, s)?;
        // d × F_j and b × w with d_z = b_z = 0.
        let df = [b.mul(d_y, fj[2])?, b.mul(d_x, fj[2])?.neg(), {
            let p = b.mul(d_x, fj[1])?;
            let q = b.mul(d_y, fj[0])?;
            b.sub(p, q)?
        }];
        let bw = [b.mul(b_y, w[2])?, b.mul(b_x, w[2])?.neg(), {
            let p = b.mul(b_x, w[1])?;
            let q = b.mul(b_y, w[0])?;
            b.sub(p, q)?
        }];
        let mut m = [Ball::ZERO; 3];
        for i in 0..3 {
            let load = b.mul(self.radius_sq, bw[i])?;
            let sum = b.add(mj[i], df[i])?;
            m[i] = b.add(sum, load)?;
        }
        let rotate = |b: &mut Balls, x: Ball, y: Ball| -> Result<(Ball, Ball), Failure> {
            // (x c − y s, x s + y c): components along t and n.
            let xc = b.mul(x, c)?;
            let ys = b.mul(y, s)?;
            let xs = b.mul(x, s)?;
            let yc = b.mul(y, c)?;
            Ok((b.sub(xc, ys)?, b.add(xs, yc)?))
        };
        let (n, v_y) = rotate(b, f[0], f[1])?;
        let (t, m_y) = rotate(b, m[0], m[1])?;
        let (w_t, _) = rotate(b, w[0], w[1])?;
        let n_eff = match self.pa {
            Some(pa) => b.add(n, pa)?,
            None => n,
        };
        Ok(Section {
            n_eff,
            v_y,
            v_z: f[2],
            t,
            m_y,
            m_z: m[2],
            w_t,
        })
    }

    /// The bending pair whose hypot the measure uses: (M_y, M_z), or
    /// (i_op M_y, i_ip M_z).
    fn bending_pair(&mut self, section: &Section) -> Result<(Ball, Ball, Ball, Ball), Failure> {
        let b = &mut self.b;
        let m_y_slope = {
            let rv = b.mul(self.radius, section.v_z)?;
            b.sub(rv, section.t)?
        };
        let m_z_slope = b.mul(self.radius, section.v_y)?.neg();
        match self.measure {
            Measure::Elastic => Ok((section.m_y, section.m_z, m_y_slope, m_z_slope)),
            Measure::Intensified {
                in_plane,
                out_of_plane,
                ..
            } => Ok((
                b.mul(out_of_plane, section.m_y)?,
                b.mul(in_plane, section.m_z)?,
                b.mul(out_of_plane, m_y_slope)?,
                b.mul(in_plane, m_z_slope)?,
            )),
        }
    }

    /// The enclosure of the measure at ψ = (k/2⁶⁰)φ.
    pub(super) fn value(&mut self, k: u64) -> Result<Interval, Failure> {
        let section = self.section(k)?;
        let (a, c, _, _) = self.bending_pair(&section)?;
        let a = magnitude_interval(bounds(&a)?);
        let c = magnitude_interval(bounds(&c)?);
        let bending_lo = quot_dn(norm2_directed(a.lo, c.lo, false), self.z);
        let bending_hi = quot_up(norm2_directed(a.hi, c.hi, true), self.z);
        let (lo, hi) = match self.measure {
            Measure::Elastic => {
                let n = magnitude_interval(bounds(&section.n_eff)?);
                (
                    sum_dn(quot_dn(n.lo, self.area), bending_lo),
                    sum_up(quot_up(n.hi, self.area), bending_hi),
                )
            }
            Measure::Intensified { .. } => (bending_lo, bending_hi),
        };
        if !(lo.is_finite() && hi.is_finite() && lo <= hi) {
            return Err(Failure::Enclosure(ArcCertificateFailure::RadiusNotFinite {
                site: "arc maximum value",
            }));
        }
        Ok(Interval {
            lo: lo.max(0.0),
            hi,
        })
    }

    /// The enclosure of the measure's derivative in ψ at ψ = (k/2⁶⁰)φ. At a
    /// kink (|N_eff| or the hypot enclosing 0) the term is enclosed by
    /// ±|its derivative|, which contains every one-sided derivative.
    pub(super) fn slope(&mut self, k: u64) -> Result<Interval, Failure> {
        let section = self.section(k)?;
        let (a, c, a1, c1) = self.bending_pair(&section)?;
        let bending = self.hypot_slope(a, c, a1, c1)?;
        let bending = Interval {
            lo: quot_dn(bending.lo, self.z),
            hi: quot_up(bending.hi, self.z),
        };
        let slope = match self.measure {
            Measure::Elastic => {
                let b = &mut self.b;
                let rw = b.mul(self.radius, section.w_t)?;
                let n1 = b.sub(section.v_y, rw)?;
                let n = bounds(&section.n_eff)?;
                let n1 = bounds(&n1)?;
                let axial = if n.lo > 0.0 {
                    n1
                } else if n.hi < 0.0 {
                    Interval {
                        lo: -n1.hi,
                        hi: -n1.lo,
                    }
                } else {
                    let m = magnitude_interval(n1).hi;
                    Interval { lo: -m, hi: m }
                };
                Interval {
                    lo: sum_dn(quot_dn(axial.lo, self.area), bending.lo),
                    hi: sum_up(quot_up(axial.hi, self.area), bending.hi),
                }
            }
            Measure::Intensified { .. } => bending,
        };
        if !(slope.lo.is_finite() && slope.hi.is_finite() && slope.lo <= slope.hi) {
            return Err(Failure::Enclosure(ArcCertificateFailure::RadiusNotFinite {
                site: "arc maximum slope",
            }));
        }
        Ok(slope)
    }

    /// d/dψ hypot(a, c) = (a a′ + c c′)/hypot(a, c), or ±hypot(a′, c′) when
    /// the hypot's enclosure reaches 0 or a ball precondition fails.
    fn hypot_slope(&mut self, a: Ball, c: Ball, a1: Ball, c1: Ball) -> Result<Interval, Failure> {
        let fallback = {
            let a1 = magnitude_interval(bounds(&a1)?);
            let c1 = magnitude_interval(bounds(&c1)?);
            let m = norm2_directed(a1.hi, c1.hi, true);
            Interval { lo: -m, hi: m }
        };
        let am = magnitude_interval(bounds(&a)?);
        let cm = magnitude_interval(bounds(&c)?);
        if norm2_directed(am.lo, cm.lo, false) <= 0.0 {
            return Ok(fallback);
        }
        let b = &mut self.b;
        let aa = b.mul(a, a1)?;
        let cc = b.mul(c, c1)?;
        let dot = b.add(aa, cc)?;
        let a2 = b.mul(a, a)?;
        let c2 = b.mul(c, c)?;
        let sq = b.add(a2, c2)?;
        let h = match b.sqrt(sq, "arc maximum hypot") {
            Ok(h) => h,
            Err(_) => return Ok(fallback),
        };
        match b.div(dot, h, "arc maximum hypot") {
            Ok(q) => bounds(&q),
            Err(_) => Ok(fallback),
        }
    }
}

// ---------------------------------------------------------------------------
// Sine and cosine (§2.3 "Arithmetic")
// ---------------------------------------------------------------------------

/// Enclosed sine and cosine for an argument ball within [0, π].
pub(super) struct Trig {
    /// Balls enclosing 1/k, k = 1…TRIG_MAX_TERMS.
    inverse: Vec<Ball>,
}

impl Trig {
    pub(super) fn new(b: &mut Balls) -> Result<Self, ArcCertificateFailure> {
        let mut inverse = Vec::with_capacity(TRIG_MAX_TERMS);
        for k in 1..=TRIG_MAX_TERMS {
            let d = b.lift(k as f64)?;
            inverse.push(b.div(Ball::ONE, d, "1/k")?);
        }
        Ok(Self { inverse })
    }

    /// (sin x, cos x) for a ball x with x ⊂ [0, π] (the binary64 π, which is
    /// below the real π). At the midpoint m ≥ 0 the Taylor polynomial of
    /// degree k − 1 is summed, where k ≥ 3 is the first index whose term
    /// tₖ = mᵏ/k! has an upward magnitude bound at most 2⁻¹³⁰·min(1, m)
    /// (at least the smallest normal number); by Lagrange the remainder of
    /// either function is at most tₖ, because every derivative is bounded by
    /// 1. Each tₖ is a ball (tₖ₋₁·m·(1/k)), so its bound encloses the exact
    /// term. The ball's radius is added last (both functions are
    /// 1-Lipschitz). No argument reduction is used.
    pub(super) fn sin_cos(&self, b: &mut Balls, x: Ball) -> Result<(Ball, Ball), Failure> {
        let range = bounds(&x)?;
        if !(range.lo >= 0.0 && range.hi <= std::f64::consts::PI) {
            return Err(Failure::TrigArgumentOutsideDomain);
        }
        let midpoint_lo = floor_ceil(&x.m)?.0;
        let threshold = (TRIG_TARGET * midpoint_lo.min(1.0)).max(f64::MIN_POSITIVE);
        let m = Ball::exact(x.m);
        let mut sin = Ball::ZERO;
        let mut cos = Ball::ONE;
        let mut term = Ball::ONE;
        let mut remainder = None;
        for (index, inverse) in self.inverse.iter().enumerate() {
            let k = index + 1;
            let power = b.mul(term, m)?;
            term = b.mul(power, *inverse)?;
            let bound = add_up(abs_up(&term.m)?, term.r);
            if k >= 3 && bound <= threshold {
                remainder = Some(bound);
                break;
            }
            match k % 4 {
                1 => sin = b.add(sin, term)?,
                2 => cos = b.sub(cos, term)?,
                3 => sin = b.sub(sin, term)?,
                _ => cos = b.add(cos, term)?,
            }
        }
        let remainder = remainder.ok_or(Failure::TrigTermLimit)?;
        let extra = add_up(remainder, x.r);
        sin.r = add_up(sin.r, extra);
        cos.r = add_up(cos.r, extra);
        if !(sin.r.is_finite() && cos.r.is_finite()) {
            return Err(Failure::Enclosure(ArcCertificateFailure::RadiusNotFinite {
                site: "sin/cos",
            }));
        }
        Ok((sin, cos))
    }
}

// ---------------------------------------------------------------------------
// Directed binary64 conversions and arithmetic
// ---------------------------------------------------------------------------

/// Binary64 values f ≤ m ≤ c (equal when m is a binary64 value).
pub(super) fn floor_ceil(m: &Wide2) -> Result<(f64, f64), Failure> {
    let split = m.split_binary64()?;
    let terms = split.terms();
    let negative = m.is_sign_negative();
    if terms.is_empty() {
        return Ok(if m.is_zero() {
            (0.0, 0.0)
        } else if negative {
            (-f64::from_bits(1), 0.0)
        } else {
            (0.0, f64::from_bits(1))
        });
    }
    let t0 = terms[0];
    if terms.len() == 1 && !split.truncated_below_min_subnormal() {
        Ok((t0, t0))
    } else if negative {
        Ok((t0.next_down(), t0))
    } else {
        Ok((t0, t0.next_up()))
    }
}

/// The directed interval [m − r, m + r] of a ball.
pub(super) fn bounds(ball: &Ball) -> Result<Interval, Failure> {
    let (lo, hi) = floor_ceil(&ball.m)?;
    let (lo, hi) = if ball.r == 0.0 {
        (lo, hi)
    } else {
        ((lo - ball.r).next_down(), (hi + ball.r).next_up())
    };
    if lo.is_finite() && hi.is_finite() {
        Ok(Interval { lo, hi })
    } else {
        Err(Failure::Enclosure(ArcCertificateFailure::RadiusNotFinite {
            site: "arc maximum bounds",
        }))
    }
}

/// The interval of |x| for x in `x`.
fn magnitude_interval(x: Interval) -> Interval {
    if x.lo >= 0.0 {
        x
    } else if x.hi <= 0.0 {
        Interval {
            lo: -x.hi,
            hi: -x.lo,
        }
    } else {
        Interval {
            lo: 0.0,
            hi: (-x.lo).max(x.hi),
        }
    }
}

fn sum_dn(a: f64, b: f64) -> f64 {
    if a == 0.0 {
        b
    } else if b == 0.0 {
        a
    } else {
        (a + b).next_down()
    }
}

fn sum_up(a: f64, b: f64) -> f64 {
    if a == 0.0 {
        b
    } else if b == 0.0 {
        a
    } else {
        (a + b).next_up()
    }
}

/// ≤ a/d for d > 0.
fn quot_dn(a: f64, d: f64) -> f64 {
    if a == 0.0 {
        0.0
    } else {
        (a / d).next_down()
    }
}

/// ≥ a/d for d > 0.
fn quot_up(a: f64, d: f64) -> f64 {
    if a == 0.0 {
        0.0
    } else {
        (a / d).next_up()
    }
}

fn directed(x: f64, upper: bool) -> f64 {
    if upper {
        x.next_up()
    } else {
        x.next_down().max(0.0)
    }
}

/// A directed bound of √(x² + y²) for x, y ≥ 0 (monotone in both).
fn norm2_directed(x: f64, y: f64, upper: bool) -> f64 {
    let (max, min) = (x.max(y), x.min(y));
    if min == 0.0 {
        return max;
    }
    let ratio = directed(min / max, upper);
    let square = directed(ratio * ratio, upper);
    let sum = directed(1.0 + square, upper);
    let root = directed(sum.sqrt(), upper);
    directed(max * root, upper)
}

/// An upward bound of √(a² + b² + c²) for a, b, c ≥ 0.
fn norm3_up(a: f64, b: f64, c: f64) -> f64 {
    let max = a.max(b).max(c);
    if max == 0.0 {
        return 0.0;
    }
    let mut sum = 1.0_f64;
    let mut skipped_max = false;
    for x in [a, b, c] {
        if x == max && !skipped_max {
            skipped_max = true;
            continue;
        }
        if x != 0.0 {
            let ratio = (x / max).next_up();
            sum = (sum + (ratio * ratio).next_up()).next_up();
        }
    }
    (max * sum.sqrt().next_up()).next_up()
}

#[cfg(test)]
#[path = "arc_maximum_tests.rs"]
mod tests;
