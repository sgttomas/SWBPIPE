//! R-6/R-7 skewed translational restraints: the authored direction, its exact
//! canonical scaling, the node-local division-free constraint basis and the
//! algebra every consumer shares (DEL-04-03 Work `skew-restraints.md` §3 to §9,
//! slice S-K1).
//!
//! - `SkewDirection` reads an authored binary64 vector d with two or more
//!   nonzero components. Its canonical form is d̃ = 2^s·d with max|d̃_i| in
//!   [1, 2). It is admitted iff every nonzero d̃_i is normal; otherwise
//!   `RangeUnsupported`, with no rounding and no tolerance (§4.2). A
//!   power-of-two rescaling of d gives the same d̃ and pivot, bit for bit.
//! - `RestraintBasis::build` forms, per node with one skewed stop, the
//!   restricted rescale w = 2^ŝ·d̃ on the free axes F and the node matrix Q:
//!   columns w_m e_j − w_j e_m (j ∈ F, j ≠ m), e_m for the pivot coordinate
//!   and e_a for each axis-restrained a (§5.1). Every entry of Q is an
//!   authored component times a power of two, 1.0 or +0.0: nothing is divided.
//! - `congruence_terms` is the one enumeration of (Qᵀ X Q)_rc, used by K′, M′,
//!   the allowance map and K-D5. Each coordinate entry has at most 9 terms
//!   (two Q columns of at most 3 entries each); S-K1's columns have at most
//!   2, so at most 4.
//! - `transform_stiffness`, `reduce`, `lift`, `lift_exact`, `lift_member_bound`,
//!   `attribute`, `spring_reaction` and `ground_rows` are §6, §7, §9 and §10.4.
//!   Each touched value is one exact sum (`ExactAccumulator`) rounded once.
//!   Triple products enter the sum only through an error-free split whose
//!   rounded product is at least `SPLIT_EXACT_MIN` (P1, §10.2).
//!
//! A model without a skewed record never builds a basis: consumers hold
//! `Option<RestraintBasis>` = `None`, and an exactly axis-aligned direction is
//! `DirectionError::AxisAligned` (`judge` returns `Ok(None)`), so today's path
//! runs byte for byte (§4.2 item 2, §6.5). Everything this module allocates is
//! therefore inside the skew branch.
use crate::exact_sum::{ExactAccumulator, SumError};
use crate::load_ledger::{round_upward, AssembledForce, ForceTermKind, LedgerError, LoadLedger};
use crate::structural::{
    reduce_assembled_sparse_system, MemberBoundInputs, SparseReducedSystem, SparseStiffness,
    StiffnessContribution,
};
use crate::{FrameKernelError, DOF_PER_NODE};
use open_pipe_stress_correct_norm::norm3;

/// P1's split rule (DN-2): `connector.rs`'s one definition, 2^-969.
use crate::connector::SPLIT_EXACT_MIN;

const AXES: [&str; 3] = ["x", "y", "z"];

// ------------------------------------------------------------------ exponents

/// The e with |x| ∈ [2^(e−1), 2^e) for finite nonzero x (C `frexp`),
/// subnormals included.
fn frexp_exponent(x: f64) -> i32 {
    let bits = x.to_bits() & !(1u64 << 63);
    let biased = (bits >> 52) as i32;
    if biased != 0 {
        biased - 1022
    } else {
        let fraction = bits & ((1u64 << 52) - 1);
        debug_assert!(fraction != 0);
        (63 - fraction.leading_zeros() as i32) - 1073
    }
}

/// 2^k for k in [-1022, 1023].
fn pow2(k: i32) -> f64 {
    debug_assert!((-1022..=1023).contains(&k));
    f64::from_bits(((k + 1023) as u64) << 52)
}

/// x·2^k in steps of at most 2^±1000. Exact whenever the result is finite and
/// normal (every intermediate then lies between x and the result, so no step
/// leaves the normal range), or when scaling up.
fn scale_pow2(mut x: f64, mut k: i32) -> f64 {
    while k != 0 && x != 0.0 {
        let step = k.clamp(-1000, 1000);
        x *= pow2(step);
        k -= step;
    }
    x
}

// ------------------------------------------------------------------ exact helpers

/// Adds the exact product a·b·c. A factor of magnitude 1 is applied by sign;
/// otherwise one pair is split error-free (TwoProduct), trying (a·b), (b·c)
/// and (a·c) in that order, and admitted only when its rounded product is
/// finite and at least `SPLIT_EXACT_MIN` (P1). Otherwise `false` (the caller
/// names the split-range failure); nothing is rounded.
fn add_triple(sum: &mut ExactAccumulator, a: f64, b: f64, c: f64) -> Result<bool, SumError> {
    if a == 0.0 || b == 0.0 || c == 0.0 {
        return Ok(true);
    }
    for (unit, x, y) in [(a, b, c), (b, a, c), (c, a, b)] {
        if unit.abs() == 1.0 {
            sum.add_product(if unit < 0.0 { -x } else { x }, y)?;
            return Ok(true);
        }
    }
    for (x, y, z) in [(a, b, c), (b, c, a), (a, c, b)] {
        let p = x * y;
        if p.is_finite() && p.abs() >= SPLIT_EXACT_MIN {
            let e = x.mul_add(y, -p);
            sum.add_product(p, z)?;
            sum.add_product(e, z)?;
            return Ok(true);
        }
    }
    Ok(false)
}

/// The exact Σ x_i² of a canonical vector, rounded once (in [1, 12)).
fn norm_squared(v: [f64; 3]) -> f64 {
    let mut sum = ExactAccumulator::new();
    for x in v {
        sum.add_product(x, x).expect("finite canonical components");
    }
    sum.round().expect("a canonical norm square lies in [1, 12)")
}

/// The least E with 2^E ≥ Σ_t m_t·2^(k_t) for the positive dyadic terms
/// (m_t, k_t), exactly; `None` when there are no terms.
fn ceil_log2_dyadic(terms: &[(u64, i64)]) -> Option<i64> {
    let top = terms
        .iter()
        .filter(|(m, _)| *m != 0)
        .map(|&(m, k)| k + i64::from(64 - m.leading_zeros()))
        .max()?;
    // A frame 2^(top - 120): each term is below 2^top, so the in-frame sum of
    // at most 9 terms fits in u128; a term entirely below the frame is a
    // sticky bit that can only break an exact power of two.
    let base = top - 120;
    let mut sum: u128 = 0;
    let mut sticky = false;
    for &(m, k) in terms.iter().filter(|(m, _)| *m != 0) {
        let shift = k - base;
        if shift >= 0 {
            sum += u128::from(m) << shift;
        } else if shift > -64 {
            let s = (-shift) as u32;
            sum += u128::from(m >> s);
            sticky |= m & ((1u64 << s) - 1) != 0;
        } else {
            sticky = true;
        }
    }
    let bits = i64::from(128 - sum.leading_zeros());
    let exact_power = sum.is_power_of_two() && !sticky;
    Some(base + if exact_power { bits - 1 } else { bits })
}

/// (significand, exponent of its unit bit) of a finite nonzero |x|.
fn dyadic(x: f64) -> (u64, i64) {
    let bits = x.abs().to_bits();
    let biased = (bits >> 52) as i64;
    let fraction = bits & ((1u64 << 52) - 1);
    if biased == 0 {
        (fraction, -1074)
    } else {
        (fraction | (1u64 << 52), biased - 1075)
    }
}

/// The error-free product x·y = h + l, when its rounded product is finite and
/// zero or at least `SPLIT_EXACT_MIN` (P1); `None` otherwise.
fn two_product(x: f64, y: f64) -> Option<(f64, f64)> {
    if x == 0.0 || y == 0.0 {
        return Some((0.0, 0.0));
    }
    let p = x * y;
    if !p.is_finite() || p.abs() < SPLIT_EXACT_MIN {
        return None;
    }
    Some((p, x.mul_add(y, -p)))
}

/// Four binary64 values whose exact sum is a·b·c (zeros where nothing is
/// left), from one error-free pair split and a TwoProduct of each part with
/// the third factor, trying (a·b), (b·c), (a·c) in that order (P1, §10.2).
/// `None` when no order stays in the split range: the caller names it and
/// nothing is rounded.
pub fn exact_triple_parts(a: f64, b: f64, c: f64) -> Option<[f64; 4]> {
    if a == 0.0 || b == 0.0 || c == 0.0 {
        return Some([0.0; 4]);
    }
    if !(a.is_finite() && b.is_finite() && c.is_finite()) {
        return None;
    }
    for (x, y, z) in [(a, b, c), (b, c, a), (a, c, b)] {
        let Some((h, l)) = two_product(x, y) else { continue };
        let (Some((h1, l1)), Some((h2, l2))) = (two_product(h, z), two_product(l, z)) else {
            continue;
        };
        return Some([h1, l1, h2, l2]);
    }
    None
}

// ------------------------------------------------------------------ direction

/// R-7: an authored direction with two or more nonzero components, its
/// canonical exact power-of-two scaling and its pivot (§4). Never normalized.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SkewDirection {
    authored: [f64; 3],
    scaled: [f64; 3],
    scale_exponent: i32,
    pivot: usize,
}

/// Why an authored vector is not a skew direction (§4.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectionError {
    /// A non-finite component (the validator's `SUPPORT_DIRECTION_INVALID`).
    NonFinite,
    /// Every component ±0 (the validator's `SUPPORT_DIRECTION_ZERO`).
    Zero,
    /// Exactly one nonzero component: today's lowered axis path.
    AxisAligned { axis: usize },
    /// Some nonzero component of 2^s·d is subnormal: `components[i]` names
    /// each such component (`SUPPORT_DIRECTION_RANGE_UNSUPPORTED`).
    RangeUnsupported {
        components: [bool; 3],
        scale_exponent: i32,
    },
}

impl SkewDirection {
    /// §4.2: classify and canonicalize, exactly.
    pub fn new(authored: [f64; 3]) -> Result<Self, DirectionError> {
        if authored.iter().any(|x| !x.is_finite()) {
            return Err(DirectionError::NonFinite);
        }
        let nonzero = authored.map(|x| x != 0.0);
        match nonzero.iter().filter(|&&n| n).count() {
            0 => return Err(DirectionError::Zero),
            1 => {
                let axis = (0..3).find(|&i| nonzero[i]).expect("one nonzero component");
                return Err(DirectionError::AxisAligned { axis });
            }
            _ => {}
        }
        let e = (0..3)
            .filter(|&i| nonzero[i])
            .map(|i| frexp_exponent(authored[i]))
            .max()
            .expect("two or more nonzero components");
        let scale_exponent = 1 - e;
        let mut components = [false; 3];
        for i in (0..3).filter(|&i| nonzero[i]) {
            // |2^s·d_i| ∈ [2^(e_i + s − 1), 2^(e_i + s)): normal iff e_i + s − 1 ≥ −1022.
            if frexp_exponent(authored[i]) + scale_exponent - 1 < -1022 {
                components[i] = true;
            }
        }
        if components.iter().any(|&c| c) {
            return Err(DirectionError::RangeUnsupported {
                components,
                scale_exponent,
            });
        }
        let scaled = authored.map(|x| scale_pow2(x, scale_exponent));
        let mut pivot = 0;
        for i in 1..3 {
            if scaled[i].abs() > scaled[pivot].abs() {
                pivot = i;
            }
        }
        Ok(Self {
            authored,
            scaled,
            scale_exponent,
            pivot,
        })
    }

    pub fn authored(&self) -> [f64; 3] {
        self.authored
    }
    /// d̃ = 2^s·d, max|d̃_i| ∈ [1, 2).
    pub fn scaled(&self) -> [f64; 3] {
        self.scaled
    }
    /// s.
    pub fn scale_exponent(&self) -> i32 {
        self.scale_exponent
    }
    /// argmax|d̃_i|, ties to the lowest index.
    pub fn pivot(&self) -> usize {
        self.pivot
    }
    /// n = d̃ᵀd̃, the exact sum rounded once (in [1, 12)).
    pub fn scaled_norm_squared(&self) -> f64 {
        norm_squared(self.scaled)
    }
    /// ν = RN(|d̃|) (`correct_norm::norm3`), in [1, 2√3).
    pub fn scaled_norm(&self) -> f64 {
        norm3(self.scaled[0], self.scaled[1], self.scaled[2])
    }
}

// ------------------------------------------------------------------ codes and judge

/// The §2.2 codes this module decides or carries. Their strings have one source,
/// PP `document_members` (`skew_code`, M-1); the kernel names no code string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkewCode {
    DirectionInvalid,
    DirectionZero,
    /// §2.2 item 2 (S-K5 removes it).
    ExactUnsupported,
    /// §2.2 item 3 (S-K4 removes it).
    NonlinearModelUnsupported,
    /// §2.2 item 4 (permanent).
    RangeUnsupported,
    /// §2.2 item 5, `imposed_motion::validate` (S-K3 removes it). The kernel's
    /// records carry no ground motion, so the product decides it.
    ImposedDisplacementSkewSupport,
    /// §2.2 item 6, boundary preparation (the product's pattern check;
    /// `transform_stiffness` refuses the same condition as `Shape`).
    NodeUnconnected,
    /// §2.2 item 7 (S-K2 removes it).
    StopsPerNode,
    /// §2.2 item 8 (permanent).
    ConstraintRedundant,
    /// §2.2 item 9 (S-K3 removes it).
    NodePrescribedMotion,
    /// §2.2 item 10 (S-K2 removes it).
    ForceScaledUnsupported,
    /// §2.2 item 11 (permanent).
    ReactionRange,
}

/// The facts skew admission reads for one record (§2.2 items 2 to 4).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SkewRecordFacts {
    pub authored: [f64; 3],
    /// The document has a `pressure_contract` (the EX route).
    pub pressure_contract: bool,
    /// The model holds any nonlinear support.
    pub nonlinear_model: bool,
}

/// One record's refusal: its code, and for a range refusal the components.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SkewRefusal {
    pub code: SkewCode,
    pub direction: Option<DirectionError>,
}

/// Skew admission for one record, in gate order (§2.2): the validator's
/// invalid and zero codes (which the document rules give first), then for a
/// skewed vector the first that applies of 2 (EX), 3 (nonlinear model) and
/// 4 (range). An exactly axis-aligned vector is `Ok(None)`: no basis, today's
/// path.
pub fn judge(facts: &SkewRecordFacts) -> Result<Option<SkewDirection>, SkewRefusal> {
    let classified = SkewDirection::new(facts.authored);
    let refuse = |code: SkewCode, direction: Option<DirectionError>| -> Result<Option<SkewDirection>, SkewRefusal> {
        Err(SkewRefusal { code, direction })
    };
    match classified {
        Err(DirectionError::NonFinite) => {
            return refuse(SkewCode::DirectionInvalid, Some(DirectionError::NonFinite))
        }
        Err(DirectionError::Zero) => return refuse(SkewCode::DirectionZero, Some(DirectionError::Zero)),
        Err(DirectionError::AxisAligned { .. }) => return Ok(None),
        _ => {}
    }
    if facts.pressure_contract {
        return refuse(SkewCode::ExactUnsupported, None);
    }
    if facts.nonlinear_model {
        return refuse(SkewCode::NonlinearModelUnsupported, None);
    }
    match classified {
        Ok(direction) => Ok(Some(direction)),
        Err(error) => refuse(SkewCode::RangeUnsupported, Some(error)),
    }
}

// ------------------------------------------------------------------ records and basis

/// A skewed rigid `line_stop` (R-6): an exact constraint d̃ᵀu_node = 0.
#[derive(Debug, Clone, PartialEq)]
pub struct SkewStop {
    pub support_id: String,
    pub node: usize,
    pub direction: SkewDirection,
}

/// A skewed `spring` (R-7): stiffness k along d, block k·d̃d̃ᵀ/(d̃ᵀd̃).
#[derive(Debug, Clone, PartialEq)]
pub struct SkewSpring {
    pub support_id: String,
    pub node: usize,
    pub direction: SkewDirection,
    pub stiffness: f64,
}

/// One constrained node (§5.1): its stop, free axes F, pivot m on F, the
/// restricted rescale w = 2^ŝ·d̃ (max_{i∈F}|w_i| ∈ [1, 2)) and Q, row-major
/// (`q[i][c]`: physical translation i, coordinate slot c). Every entry of Q is
/// a component of w (exact), 1.0 or +0.0.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NodeBasis {
    pub node: usize,
    pub stop: usize,
    pub free_axes: [bool; 3],
    pub pivot: usize,
    pub restricted_scale_exponent: i32,
    pub w: [f64; 3],
    pub q: [[f64; 3]; 3],
}

impl NodeBasis {
    /// §5.1's exact rank: |A| + [d̃_F ≠ 0] (always 1 for d̃_F here).
    pub fn rank(&self) -> usize {
        self.free_axes.iter().filter(|&&f| !f).count() + 1
    }
    /// The free coordinate slots: F ∖ {m}, ascending.
    pub fn free_slots(&self) -> impl Iterator<Item = usize> + '_ {
        (0..3).filter(move |&c| self.free_axes[c] && c != self.pivot)
    }
}

/// A boundary-preparation finding (§2.2 items 7 and 8), per node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BasisFinding {
    StopsPerNode { node: usize, supports: Vec<String> },
    Redundant { node: usize, supports: Vec<String> },
    /// Inputs outside `build`'s contract (named), never a model condition.
    InvalidInput(&'static str),
}

impl BasisFinding {
    pub fn code(&self) -> Option<SkewCode> {
        match self {
            Self::StopsPerNode { .. } => Some(SkewCode::StopsPerNode),
            Self::Redundant { .. } => Some(SkewCode::ConstraintRedundant),
            Self::InvalidInput(_) => None,
        }
    }
}

/// A published value of an attribution (§7.2) that left binary64.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublishedValue {
    StopComponent { axis: usize },
    AlongDirection,
    AxisComponent { dof: usize },
    SpringComponent { axis: usize },
    SpringAlongDirection,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BasisError {
    /// A triple product outside P1's split range, at coordinate row `dof`.
    SplitRange { dof: usize },
    /// A value whose single rounding is outside binary64, at `dof`.
    NonFinite { dof: usize },
    Shape(&'static str),
    Sum(SumError),
    /// A nonzero value on an axis restraint at a stop's node (§2.2 item 9).
    PrescribedMotion { dof: usize },
    /// §2.2 item 11: a published value outside binary64.
    ReactionRange { node: usize, value: PublishedValue },
    Reduction(FrameKernelError),
    Ledger(LedgerError),
}

impl From<SumError> for BasisError {
    fn from(error: SumError) -> Self {
        Self::Sum(error)
    }
}

/// (Qᵀ X Q)_rc's terms: physical (i, j) with weights (q_ir, q_jc). At most 9.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CongruenceTerm {
    pub i: usize,
    pub j: usize,
    pub left: f64,
    pub right: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CongruenceTerms {
    terms: [CongruenceTerm; 9],
    len: usize,
}

impl CongruenceTerms {
    pub fn as_slice(&self) -> &[CongruenceTerm] {
        &self.terms[..self.len]
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

/// One Q column's nonzero entries (physical DOF, weight): at most 3.
#[derive(Clone, Copy)]
struct Column {
    entries: [(usize, f64); 3],
    len: usize,
}

impl Column {
    fn as_slice(&self) -> &[(usize, f64)] {
        &self.entries[..self.len]
    }
}

/// One Q row's nonzero entries (coordinate, weight): at most 3.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QRow {
    entries: [(usize, f64); 3],
    len: usize,
}

impl QRow {
    pub fn as_slice(&self) -> &[(usize, f64)] {
        &self.entries[..self.len]
    }
}

/// a + b rounded upward, for finite nonnegative operands: the nearest sum,
/// moved up one step when TwoSum's exact error is positive.
fn add_up(a: f64, b: f64) -> f64 {
    let s = a + b;
    if !s.is_finite() {
        return s;
    }
    let bb = s - a;
    let error = (a - (s - bb)) + (b - bb);
    if error > 0.0 {
        s.next_up()
    } else {
        s
    }
}

/// a·b rounded upward (finite nonnegative operands).
fn mul_up(a: f64, b: f64) -> f64 {
    crate::load_ledger::product_upward(a, b)
}

/// ½ulp(x) rounded upward (the binade's half spacing; 2^-1074 at zero).
fn half_ulp(x: f64) -> f64 {
    let biased = (x.abs().to_bits() >> 52) as i32;
    if biased <= 53 {
        f64::from_bits(1)
    } else {
        f64::from_bits(((biased - 53) as u64) << 52)
    }
}

/// K-D5's view of a skew case (§10): the basis, and the **physical** ledger
/// and stiffness contributions the intended source binds, while the gate's
/// system carries the coordinate ones (K′, f′ and `transform_contributions`).
/// `FormationSource.restraint` points at it.
#[derive(Debug)]
pub struct SkewFormation<'a> {
    pub basis: &'a RestraintBasis,
    pub physical_force: &'a AssembledForce,
    pub physical_contributions: &'a [StiffnessContribution],
}

impl PartialEq for SkewFormation<'_> {
    /// The same basis value and the same physical inputs (by identity: an
    /// `AssembledForce` has no value equality).
    fn eq(&self, other: &Self) -> bool {
        self.basis == other.basis
            && std::ptr::eq(self.physical_force, other.physical_force)
            && self.physical_contributions == other.physical_contributions
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StopReaction {
    /// Support-on-pipe global force (N).
    pub global: [f64; 3],
    /// Positive along the authored +d (N).
    pub along: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodeAttribution {
    pub stop: StopReaction,
    /// Each axis restraint a ∈ A at the node: (global DOF, r_a − μ·w_a).
    pub axis: Vec<(usize, f64)>,
}

/// §7.2 intermediates, for the references' `attribution_range` bits.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AttributionTrace {
    pub k: i32,
    pub n_prime: f64,
    pub mu_prime: f64,
    pub mu: f64,
    pub n_w: f64,
}

/// A ground row for rigid-body screening (§9).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GroundRow {
    Axis(usize),
    Direction { node: usize, d: [f64; 3] },
}

/// §10.4: K-D5's coordinate bound inputs lifted to physical DOFs. `bound`'s
/// vectors are physical, and `bound.lift_radius` encloses the lift's rounding.
#[derive(Debug, Clone, PartialEq)]
pub struct LiftedMemberBound {
    /// u_p = fl(Q v).
    pub u: Vec<f64>,
    pub bound: MemberBoundInputs,
}

/// The coordinate system of one case: f′ = Qᵀf as a coordinate ledger and the
/// partition with its reduced right-hand side.
#[derive(Debug)]
pub struct CoordinateReduction {
    pub force: AssembledForce,
    pub reduced: SparseReducedSystem,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RestraintBasis {
    node_count: usize,
    axis_restrained: Vec<usize>,
    restrained_coordinates: Vec<usize>,
    nodes: Vec<NodeBasis>,
    stops: Vec<SkewStop>,
    springs: Vec<SkewSpring>,
}

impl RestraintBasis {
    /// `axis_restrained` is today's `BoundaryPreparation::restrained_dofs`
    /// (strictly ascending). Per node in ascending order, the first that
    /// applies of §2.2 items 7 and 8; every node is reported.
    pub fn build(
        node_count: usize,
        axis_restrained: &[usize],
        stops: Vec<SkewStop>,
        springs: Vec<SkewSpring>,
    ) -> Result<Self, Vec<BasisFinding>> {
        let dimension = node_count * DOF_PER_NODE;
        if axis_restrained.windows(2).any(|p| p[0] >= p[1])
            || axis_restrained.last().is_some_and(|&d| d >= dimension)
        {
            return Err(vec![BasisFinding::InvalidInput("axis restrained DOFs")]);
        }
        if stops.iter().any(|s| s.node >= node_count) || springs.iter().any(|s| s.node >= node_count) {
            return Err(vec![BasisFinding::InvalidInput("skew record node")]);
        }
        if springs
            .iter()
            .any(|s| !s.stiffness.is_finite() || s.stiffness < 0.0)
        {
            return Err(vec![BasisFinding::InvalidInput("skew spring stiffness")]);
        }
        let mut order: Vec<usize> = (0..stops.len()).collect();
        order.sort_by_key(|&k| (stops[k].node, k));
        let mut findings = Vec::new();
        let mut nodes = Vec::new();
        let mut start = 0;
        while start < order.len() {
            let node = stops[order[start]].node;
            let mut end = start;
            while end < order.len() && stops[order[end]].node == node {
                end += 1;
            }
            let at_node = &order[start..end];
            start = end;
            if at_node.len() >= 2 {
                findings.push(BasisFinding::StopsPerNode {
                    node,
                    supports: at_node.iter().map(|&k| stops[k].support_id.clone()).collect(),
                });
                continue;
            }
            let stop = at_node[0];
            let scaled = stops[stop].direction.scaled();
            let free_axes = [0, 1, 2]
                .map(|t| axis_restrained.binary_search(&(node * DOF_PER_NODE + t)).is_err());
            let mut pivot = None::<usize>;
            for i in (0..3).filter(|&i| free_axes[i] && scaled[i] != 0.0) {
                if pivot.map_or(true, |m| scaled[i].abs() > scaled[m].abs()) {
                    pivot = Some(i);
                }
            }
            let Some(pivot) = pivot else {
                findings.push(BasisFinding::Redundant {
                    node,
                    supports: vec![stops[stop].support_id.clone()],
                });
                continue;
            };
            // §5.1: ŝ ≥ 0 puts |w_m| in [1, 2); exact (scaling up), ŝ ≤ 1022.
            let restricted_scale_exponent = 1 - frexp_exponent(scaled[pivot]);
            let w = scaled.map(|x| scale_pow2(x, restricted_scale_exponent));
            let mut q = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
            for j in (0..3).filter(|&j| free_axes[j] && j != pivot) {
                q[j][j] = w[pivot];
                q[pivot][j] = if w[j] == 0.0 { 0.0 } else { -w[j] };
            }
            nodes.push(NodeBasis {
                node,
                stop,
                free_axes,
                pivot,
                restricted_scale_exponent,
                w,
                q,
            });
        }
        if !findings.is_empty() {
            return Err(findings);
        }
        let restrained_coordinates = if nodes.is_empty() {
            axis_restrained.to_vec()
        } else {
            let mut all: Vec<usize> = axis_restrained
                .iter()
                .copied()
                .chain(nodes.iter().map(|n| n.node * DOF_PER_NODE + n.pivot))
                .collect();
            all.sort_unstable();
            all
        };
        Ok(Self {
            node_count,
            axis_restrained: axis_restrained.to_vec(),
            restrained_coordinates,
            nodes,
            stops,
            springs,
        })
    }

    /// True when some node carries a stop (`!nodes.is_empty()`).
    pub fn has_constraint_transform(&self) -> bool {
        !self.nodes.is_empty()
    }
    pub fn node_count(&self) -> usize {
        self.node_count
    }
    pub fn dimension(&self) -> usize {
        self.node_count * DOF_PER_NODE
    }
    /// Physical, as today.
    pub fn axis_restrained(&self) -> &[usize] {
        &self.axis_restrained
    }
    /// Axis ∪ pivots, ascending: the coordinate partition.
    pub fn restrained_coordinates(&self) -> &[usize] {
        &self.restrained_coordinates
    }
    pub fn nodes(&self) -> &[NodeBasis] {
        &self.nodes
    }
    pub fn stops(&self) -> &[SkewStop] {
        &self.stops
    }
    pub fn springs(&self) -> &[SkewSpring] {
        &self.springs
    }
    pub fn node_basis(&self, node: usize) -> Option<&NodeBasis> {
        self.nodes
            .binary_search_by_key(&node, |n| n.node)
            .ok()
            .map(|k| &self.nodes[k])
    }

    /// The constrained node and slot of a global translation DOF, if any.
    fn translation(&self, dof: usize) -> Option<(&NodeBasis, usize)> {
        let slot = dof % DOF_PER_NODE;
        if slot >= 3 || self.nodes.is_empty() {
            return None;
        }
        self.node_basis(dof / DOF_PER_NODE).map(|n| (n, slot))
    }

    /// Q's row `dof`: the coordinates c with q_dof,c ≠ 0 and their weights (at
    /// most 3; the single (dof, 1) away from constrained nodes). A physical
    /// DOF's value is Σ_c q_dof,c·v_c, and a physical row enters coordinate
    /// row c with weight q_dof,c.
    pub fn q_row(&self, dof: usize) -> QRow {
        let mut row = QRow { entries: [(0, 0.0); 3], len: 0 };
        match self.translation(dof) {
            None => {
                row.entries[0] = (dof, 1.0);
                row.len = 1;
            }
            Some((node, i)) => {
                for c in 0..3 {
                    let q = node.q[i][c];
                    if q != 0.0 {
                        row.entries[row.len] = (node.node * DOF_PER_NODE + c, q);
                        row.len += 1;
                    }
                }
            }
        }
        row
    }

    /// Q's column `coordinate`: the physical DOFs i with q_i,coordinate ≠ 0 and
    /// their weights (at most 3; the single (coordinate, 1) away from
    /// constrained nodes). The coordinate entry of Qᵀx is Σ_i q_i,c·x_i.
    pub fn q_column(&self, coordinate: usize) -> QRow {
        let column = self.column(coordinate);
        QRow { entries: column.entries, len: column.len }
    }

    /// Whether `dof` is a translation at a node with a stop (a touched DOF).
    pub fn is_constrained_translation(&self, dof: usize) -> bool {
        self.translation(dof).is_some()
    }

    /// The coordinate contributions of physical `contributions` (both
    /// triangles, in order) and their element grouping (exclusive ends): a
    /// contribution away from constrained nodes is kept as it is; one
    /// touching a constrained node becomes, for each (r, c) of its
    /// congruence terms in `q_row` order, exactly four binary64 parts whose
    /// exact sum is q_ir·v·q_jc (`exact_triple_parts`). So the exact sum of the
    /// coordinate contributions at (r, c) is (Qᵀ(Σ_e contrib_e)Q)_rc, and the
    /// gate's contribution audits judge K′ against it (§10.3). A product
    /// outside the split range is `SplitRange` at its coordinate row.
    pub fn transform_contributions(
        &self,
        contributions: &[StiffnessContribution],
        groups: Option<&[usize]>,
    ) -> Result<(Vec<StiffnessContribution>, Option<Vec<usize>>), BasisError> {
        let mut out = Vec::with_capacity(contributions.len());
        let mut ends = groups.map(|g| Vec::with_capacity(g.len()));
        let mut next_group = 0;
        for (index, e) in contributions.iter().enumerate() {
            if e.row >= self.dimension() || e.col >= self.dimension() {
                return Err(BasisError::Shape("contribution outside the model"));
            }
            if !self.is_constrained_translation(e.row) && !self.is_constrained_translation(e.col) {
                out.push(*e);
            } else {
                for &(r, q_ir) in self.q_row(e.row).as_slice() {
                    for &(c, q_jc) in self.q_row(e.col).as_slice() {
                        let parts = exact_triple_parts(q_ir, e.value, q_jc).ok_or(BasisError::SplitRange { dof: r })?;
                        out.extend(parts.iter().map(|&value| StiffnessContribution { row: r, col: c, value }));
                    }
                }
            }
            if let (Some(groups), Some(ends)) = (groups, ends.as_mut()) {
                if groups.get(next_group) == Some(&(index + 1)) {
                    ends.push(out.len());
                    next_group += 1;
                }
            }
        }
        Ok((out, ends))
    }

    /// §10.3's per-entry allowances on K′'s pattern, from the physical ones
    /// on K's (same pattern): untouched entries keep theirs; a touched entry
    /// (r, c) gets Σ|q_ir|·ar_ij·|q_jc| + ½ulp(K′_rc), upward, and the sum of
    /// its terms' operation counts plus one.
    pub fn transform_roundoff(
        &self,
        k: &SparseStiffness,
        k_prime: &SparseStiffness,
        roundoff: &[f64],
        counts: &[usize],
    ) -> Result<(Vec<f64>, Vec<usize>), BasisError> {
        let pattern = k.pattern();
        if k_prime.pattern() != pattern || roundoff.len() != pattern.entry_count() || counts.len() != roundoff.len() {
            return Err(BasisError::Shape("roundoff evidence"));
        }
        let mut out = roundoff.to_vec();
        let mut out_counts = counts.to_vec();
        for node in &self.nodes {
            for slot in 0..3 {
                let row = node.node * DOF_PER_NODE + slot;
                for index in pattern.row_range(row) {
                    let col = pattern.column(index);
                    let mut bound = 0.0;
                    let mut count = 1usize;
                    for term in self.congruence_terms(row, col).as_slice() {
                        let Some(at) = pattern.find(term.i, term.j) else { continue };
                        bound = add_up(bound, mul_up(mul_up(term.left.abs(), roundoff[at]), term.right.abs()));
                        count = count.saturating_add(counts[at]);
                    }
                    bound = add_up(bound, half_ulp(k_prime.values()[index]));
                    for at in [index, pattern.transpose(index)] {
                        out[at] = bound;
                        out_counts[at] = count;
                    }
                }
            }
        }
        Ok((out, out_counts))
    }

    /// Q's column `coordinate`: its nonzero entries.
    fn column(&self, coordinate: usize) -> Column {
        let mut column = Column {
            entries: [(0, 0.0); 3],
            len: 0,
        };
        match self.translation(coordinate) {
            None => {
                column.entries[0] = (coordinate, 1.0);
                column.len = 1;
            }
            Some((node, c)) => {
                for i in 0..3 {
                    let q = node.q[i][c];
                    if q != 0.0 {
                        column.entries[column.len] = (node.node * DOF_PER_NODE + i, q);
                        column.len += 1;
                    }
                }
            }
        }
        column
    }

    /// The binary64 3×3 block of one skewed spring (§6.1):
    /// fl(k·fl(fl(d̃_i·d̃_j)/n)), n = d̃ᵀd̃ rounded once; symmetric bit for bit.
    pub fn spring_block(&self, spring: usize) -> [[f64; 3]; 3] {
        let spring = &self.springs[spring];
        let d = spring.direction.scaled();
        let n = spring.direction.scaled_norm_squared();
        let mut block = [[0.0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                block[i][j] = spring.stiffness * ((d[i] * d[j]) / n);
            }
        }
        block
    }

    /// (Qᵀ X Q)_{rc} = Σ q_ir · X_ij · q_jc: the physical (i, j) pairs and the
    /// exact weights (q_ir, q_jc) of one coordinate entry. Identity rows give
    /// the single pair (r, c) with weights (1, 1). The one enumeration that
    /// K′, M′, the allowance map and K-D5 use.
    pub fn congruence_terms(&self, row: usize, col: usize) -> CongruenceTerms {
        let mut out = CongruenceTerms {
            terms: [CongruenceTerm {
                i: 0,
                j: 0,
                left: 0.0,
                right: 0.0,
            }; 9],
            len: 0,
        };
        let left = self.column(row);
        let right = self.column(col);
        for &(i, q_ir) in left.as_slice() {
            for &(j, q_jc) in right.as_slice() {
                out.terms[out.len] = CongruenceTerm {
                    i,
                    j,
                    left: q_ir,
                    right: q_jc,
                };
                out.len += 1;
            }
        }
        out
    }

    /// The node-block premise of K′'s pattern (§6.2, §2.2 item 6): at each
    /// constrained node the three translation rows share one column set that
    /// holds the node's own 3×3 translation block.
    fn check_node_blocks(&self, k: &SparseStiffness) -> Result<(), BasisError> {
        let pattern = k.pattern();
        for node in &self.nodes {
            let base = node.node * DOF_PER_NODE;
            let first = pattern.row(base);
            if (0..3).any(|t| first.binary_search(&(base + t)).is_err())
                || pattern.row(base + 1) != first
                || pattern.row(base + 2) != first
            {
                return Err(BasisError::Shape("skew node block outside the stiffness pattern"));
            }
        }
        Ok(())
    }

    /// K′ = QᵀKQ on K's pattern (§6.2): entries away from constrained nodes
    /// are K's values; each touched entry is one exact sum Σ q_ir·K_ij·q_jc
    /// rounded once (symmetric bit for bit, never −0.0).
    pub fn transform_stiffness(&self, k: &SparseStiffness) -> Result<SparseStiffness, BasisError> {
        if k.dimension() != self.dimension() {
            return Err(BasisError::Shape("stiffness dimension"));
        }
        self.check_node_blocks(k)?;
        let pattern = k.pattern();
        let mut values = k.values().to_vec();
        for node in &self.nodes {
            for slot in 0..3 {
                let row = node.node * DOF_PER_NODE + slot;
                for index in pattern.row_range(row) {
                    let col = pattern.column(index);
                    let value = self.transformed_entry(k, row, col)?;
                    values[index] = value;
                    values[pattern.transpose(index)] = value;
                }
            }
        }
        SparseStiffness::from_pattern_values(pattern.clone(), values)
            .map_err(|_| BasisError::Shape("transformed stiffness pattern"))
    }

    fn transformed_entry(&self, k: &SparseStiffness, row: usize, col: usize) -> Result<f64, BasisError> {
        let mut sum = ExactAccumulator::new();
        for term in self.congruence_terms(row, col).as_slice() {
            let k_ij = k.get(term.i, term.j);
            if !k_ij.is_finite() {
                return Err(BasisError::NonFinite { dof: row });
            }
            if !add_triple(&mut sum, term.left, k_ij, term.right)? {
                return Err(BasisError::SplitRange { dof: row });
            }
        }
        sum.round().map_err(|_| BasisError::NonFinite { dof: row })
    }

    /// f′ = Qᵀf as a coordinate ledger (§6.3): on each touched coordinate row
    /// r, each physical term on DOF i with q_ir ≠ 0 becomes q_ir·v (a
    /// product) or, for a product a·b, the split parts of q_ir·a times b. Rows
    /// away from constrained nodes keep their terms. Each row's net is one
    /// exact sum rounded once. Formation records and load primitives are not
    /// carried: K-D5 binds the physical ledger (§6.3, §10).
    pub fn transform_force(&self, force: &AssembledForce) -> Result<AssembledForce, BasisError> {
        let n = self.dimension();
        if force.len() != n {
            return Err(BasisError::Shape("force length"));
        }
        let mut ledger = LoadLedger::new();
        for r in 0..n {
            for &(i, q_ir) in self.column(r).as_slice() {
                // A unit weight (identity rows, and ±1 entries of Q) moves the
                // term exactly, by sign; any other weight is a product.
                let unit = q_ir.abs() == 1.0;
                let signed = |x: f64| if q_ir < 0.0 { -x } else { x };
                for term in force.terms_for(i) {
                    let source = term.source.clone();
                    match term.kind {
                        ForceTermKind::Term(v) if unit => ledger.push(source, r, signed(v)),
                        ForceTermKind::Product(a, b) if unit => ledger.push_product(source, r, signed(a), b),
                        ForceTermKind::Term(v) => ledger.push_product(source, r, q_ir, v),
                        ForceTermKind::Product(a, b) if a == 0.0 || b == 0.0 => {
                            ledger.push_product(source, r, a, b)
                        }
                        ForceTermKind::Product(a, b) => {
                            // q_ir·a·b: split q_ir·a (else q_ir·b) error-free (P1).
                            let (x, y) = [(a, b), (b, a)]
                                .into_iter()
                                .find(|&(x, _)| {
                                    let p = q_ir * x;
                                    p.is_finite() && p.abs() >= SPLIT_EXACT_MIN
                                })
                                .ok_or(BasisError::SplitRange { dof: r })?;
                            let p = q_ir * x;
                            let e = q_ir.mul_add(x, -p);
                            ledger.push_product(source.clone(), r, p, y);
                            if e != 0.0 {
                                ledger.push_product(source, r, e, y);
                            }
                        }
                    }
                }
            }
        }
        ledger.finish(n).map_err(BasisError::Ledger)
    }

    /// The axis coordinates' values (aligned with `axis_restrained`) as the
    /// coordinate partition's, with 0 at each pivot. A nonzero value on an
    /// axis translation at a stop's node is §2.2 item 9.
    fn coordinate_values(&self, values: &[f64]) -> Result<Vec<f64>, BasisError> {
        if values.len() != self.axis_restrained.len() {
            return Err(BasisError::Shape("prescribed values length"));
        }
        let mut out = Vec::with_capacity(self.restrained_coordinates.len());
        for &dof in &self.restrained_coordinates {
            match self.axis_restrained.binary_search(&dof) {
                Ok(k) => {
                    if values[k] != 0.0 && self.translation(dof).is_some() {
                        return Err(BasisError::PrescribedMotion { dof });
                    }
                    out.push(values[k]);
                }
                Err(_) => out.push(0.0),
            }
        }
        Ok(out)
    }

    /// §2.2 item 9 before reduction: every axis translation DOF at a stop's
    /// node carrying a nonzero prescribed value, ascending.
    pub fn judge_prescribed(&self, prescribed: &[(usize, f64)]) -> Vec<usize> {
        let mut out: Vec<usize> = prescribed
            .iter()
            .filter(|&&(dof, g)| g != 0.0 && self.translation(dof).is_some())
            .map(|&(dof, _)| dof)
            .collect();
        out.sort_unstable();
        out.dedup();
        out
    }

    /// The coordinate partition and reduced right-hand side
    /// f′_r − Σ_c K′_rc g_c (§6.3), each free row one exact sum rounded once
    /// (`reduce_assembled_sparse_system` on the coordinate ledger). `values`
    /// are aligned with `axis_restrained` (`None`: all zero).
    pub fn reduce(
        &self,
        k_prime: &SparseStiffness,
        force: &AssembledForce,
        values: Option<&[f64]>,
    ) -> Result<CoordinateReduction, BasisError> {
        let force = self.transform_force(force)?;
        let values = values.map(|v| self.coordinate_values(v)).transpose()?;
        let reduced = reduce_assembled_sparse_system(
            k_prime,
            &force,
            &self.restrained_coordinates,
            values.as_deref(),
        )
        .map_err(BasisError::Reduction)?;
        Ok(CoordinateReduction { force, reduced })
    }

    /// The restrained coordinates at constrained nodes (pivot and axis slots)
    /// must hold exactly zero in a coordinate vector.
    fn check_coordinates(&self, v: &[f64]) -> Result<(), BasisError> {
        if v.len() != self.dimension() {
            return Err(BasisError::Shape("coordinate vector length"));
        }
        for node in &self.nodes {
            for c in (0..3).filter(|&c| !node.free_axes[c] || c == node.pivot) {
                if v[node.node * DOF_PER_NODE + c] != 0.0 {
                    return Err(BasisError::Shape("nonzero restrained coordinate at a stop's node"));
                }
            }
        }
        Ok(())
    }

    /// u = Q v, each physical entry one exact sum rounded once; the pivot
    /// and axis coordinates at a stop's node must be 0.
    pub fn lift(&self, v: &[f64]) -> Result<Vec<f64>, BasisError> {
        self.check_coordinates(v)?;
        let mut u = v.to_vec();
        for node in &self.nodes {
            for i in 0..3 {
                let dof = node.node * DOF_PER_NODE + i;
                let mut sum = ExactAccumulator::new();
                self.lift_exact(v, dof, &mut sum)?;
                u[dof] = sum.round().map_err(|_| BasisError::NonFinite { dof })?;
            }
        }
        Ok(u)
    }

    /// Adds (Q v)_dof exactly to `acc` (for certificates and the exact
    /// constraint check).
    pub fn lift_exact(&self, v: &[f64], dof: usize, acc: &mut ExactAccumulator) -> Result<(), BasisError> {
        if v.len() != self.dimension() || dof >= v.len() {
            return Err(BasisError::Shape("coordinate vector length"));
        }
        match self.translation(dof) {
            None => acc.add(v[dof])?,
            Some((node, i)) => {
                for c in 0..3 {
                    acc.add_product(node.q[i][c], v[node.node * DOF_PER_NODE + c])?;
                }
            }
        }
        Ok(())
    }

    /// The exact w·(Q v) at a stop's node (= 2^ŝ·d̃ᵀ(Q v); zero exactly iff the
    /// unrounded lift satisfies the authored constraint).
    pub fn constraint_residual(&self, node: usize, v: &[f64]) -> Result<ExactAccumulator, BasisError> {
        let basis = *self.node_basis(node).ok_or(BasisError::Shape("no stop at node"))?;
        if v.len() != self.dimension() {
            return Err(BasisError::Shape("coordinate vector length"));
        }
        let base = basis.node * DOF_PER_NODE;
        let mut sum = ExactAccumulator::new();
        for i in 0..3 {
            for c in 0..3 {
                if !add_triple(&mut sum, basis.w[i], basis.q[i][c], v[base + c])? {
                    return Err(BasisError::SplitRange { dof: base + i });
                }
            }
        }
        Ok(sum)
    }

    /// K-D5's coordinate bound inputs as physical ones (§10.4):
    /// - u_p = fl(Q v); w_p = fl(exact(Q(v + w)) − u_p);
    ///   w₂,p = fl(exact(Q(v + w + w₂)) − u_p − w_p);
    /// - lift radius |exact(Q(v + w + w₂)) − u_p − w_p − w₂,p|, rounded up;
    /// - E_i the least integer with 2^E_i ≥ Σ_j |q_ij|·2^(exp_j) over free
    ///   coordinates j (exact), or `i32::MIN` when no free coordinate maps to
    ///   DOF i (its lift values and radius are then exactly 0).
    ///
    /// DOFs away from constrained nodes keep their coordinate values.
    pub fn lift_member_bound(&self, v: &[f64], bound: &MemberBoundInputs) -> Result<LiftedMemberBound, BasisError> {
        let n = self.dimension();
        if v.len() != n || bound.w.len() != n || bound.w2.len() != n || bound.exponents.len() != n {
            return Err(BasisError::Shape("member bound length"));
        }
        let u = self.lift(v)?;
        let mut w = bound.w.clone();
        let mut w2 = bound.w2.clone();
        let mut exponents = bound.exponents.clone();
        let mut lift_radius = vec![0.0; n];
        for node in &self.nodes {
            let base = node.node * DOF_PER_NODE;
            for i in 0..3 {
                let dof = base + i;
                let lifted = |sum: &mut ExactAccumulator, sign: f64, second: bool| -> Result<(), SumError> {
                    for c in 0..3 {
                        let q = sign * node.q[i][c];
                        sum.add_product(q, v[base + c])?;
                        sum.add_product(q, bound.w[base + c])?;
                        if second {
                            sum.add_product(q, bound.w2[base + c])?;
                        }
                    }
                    Ok(())
                };
                let mut first = ExactAccumulator::new();
                lifted(&mut first, 1.0, false)?;
                first.add(-u[dof])?;
                let w_p = first.round().map_err(|_| BasisError::NonFinite { dof })?;
                let mut second = ExactAccumulator::new();
                lifted(&mut second, 1.0, true)?;
                second.add(-u[dof])?;
                second.add(-w_p)?;
                let w2_p = second.round().map_err(|_| BasisError::NonFinite { dof })?;
                second.add(-w2_p)?;
                let residual = if second.signum() < 0 {
                    let mut negated = ExactAccumulator::new();
                    lifted(&mut negated, -1.0, true)?;
                    negated.add(u[dof])?;
                    negated.add(w_p)?;
                    negated.add(w2_p)?;
                    negated
                } else {
                    second
                };
                let radius = round_upward(&residual)?;
                let terms: Vec<(u64, i64)> = (0..3)
                    .filter(|&c| node.q[i][c] != 0.0 && bound.exponents[base + c] != i32::MIN)
                    .map(|c| {
                        let (m, k) = dyadic(node.q[i][c]);
                        (m, k + i64::from(bound.exponents[base + c]))
                    })
                    .collect();
                let exponent = match ceil_log2_dyadic(&terms) {
                    None => i32::MIN,
                    Some(e) => i32::try_from(e)
                        .ok()
                        .filter(|&e| e != i32::MIN)
                        .ok_or(BasisError::Shape("lifted remainder exponent range"))?,
                };
                if exponent == i32::MIN && (u[dof] != 0.0 || w_p != 0.0 || w2_p != 0.0 || radius != 0.0) {
                    return Err(BasisError::Shape("prescribed DOF with a nonzero lift"));
                }
                w[dof] = w_p;
                w2[dof] = w2_p;
                exponents[dof] = exponent;
                lift_radius[dof] = radius;
            }
        }
        Ok(LiftedMemberBound {
            u,
            bound: MemberBoundInputs {
                w,
                w2,
                exponents,
                lambda_sigma: bound.lambda_sigma,
                lift_radius,
            },
        })
    }

    /// Binary64 attribution at one constrained node from its physical rows
    /// r = (K u − f) (§7.2): the unique multiplier, range-safe.
    pub fn attribute(&self, node: usize, r: [f64; 3]) -> Result<NodeAttribution, BasisError> {
        self.attribute_traced(node, r).map(|(a, _)| a)
    }

    /// `attribute` with its §7.2 intermediates.
    pub fn attribute_traced(&self, node: usize, r: [f64; 3]) -> Result<(NodeAttribution, AttributionTrace), BasisError> {
        let basis = *self.node_basis(node).ok_or(BasisError::Shape("no stop at node"))?;
        if r.iter().any(|x| !x.is_finite()) {
            return Err(BasisError::Shape("non-finite reaction row"));
        }
        let free = |i: usize| basis.free_axes[i];
        let range = |value| BasisError::ReactionRange { node, value };
        let w = basis.w;
        // n_w ∈ [1, 12): exact, rounded once; never underflows.
        let mut n_w_sum = ExactAccumulator::new();
        let mut numerator = ExactAccumulator::new();
        for i in (0..3).filter(|&i| free(i)) {
            n_w_sum.add_product(w[i], w[i])?;
            numerator.add_product(w[i], r[i])?;
        }
        let n_w = n_w_sum.round()?;
        // N_w stays exact; k = 4 only when it does not round finitely.
        let k = match numerator.round() {
            Ok(_) => 0,
            Err(SumError::NonRepresentable) => 4,
            Err(error) => return Err(error.into()),
        };
        let n_prime = numerator.round_scaled(-k)?;
        let mu_prime = n_prime / n_w;
        let mu = mu_prime * pow2(k);
        if !mu.is_finite() {
            return Err(range(PublishedValue::StopComponent { axis: basis.pivot }));
        }
        let mut global = [0.0; 3];
        for i in 0..3 {
            global[i] = mu * w[i];
            if !global[i].is_finite() {
                return Err(range(PublishedValue::StopComponent { axis: i }));
            }
        }
        let nu = self.stops[basis.stop].direction.scaled_norm();
        let mut along = ExactAccumulator::new();
        along.add_product(mu, nu)?;
        let along = along
            .round_scaled(basis.restricted_scale_exponent)
            .map_err(|_| range(PublishedValue::AlongDirection))?;
        let mut axis = Vec::new();
        for a in (0..3).filter(|&a| !free(a)) {
            let dof = node * DOF_PER_NODE + a;
            let mut sum = ExactAccumulator::new();
            sum.add(r[a])?;
            sum.add_product(-mu, w[a])?;
            axis.push((
                dof,
                sum.round().map_err(|_| range(PublishedValue::AxisComponent { dof }))?,
            ));
        }
        Ok((
            NodeAttribution {
                stop: StopReaction { global, along },
                axis,
            },
            AttributionTrace {
                k,
                n_prime,
                mu_prime,
                mu,
                n_w,
            },
        ))
    }

    /// One skewed spring's reaction (§7.2): t = d̃ᵀu (exact, rounded once),
    /// κ = fl(k·t); components fl(−fl(κ/n)·d̃_i), along −fl(κ/ν). Both branch
    /// from κ; no chained division.
    pub fn spring_reaction(&self, spring: usize, u: &[f64]) -> Result<StopReaction, BasisError> {
        let record = self.springs.get(spring).ok_or(BasisError::Shape("spring index"))?;
        if u.len() != self.dimension() {
            return Err(BasisError::Shape("displacement length"));
        }
        let d = record.direction.scaled();
        let base = record.node * DOF_PER_NODE;
        let mut dot = ExactAccumulator::new();
        for i in 0..3 {
            dot.add_product(d[i], u[base + i])?;
        }
        let node = record.node;
        let t = dot
            .round()
            .map_err(|_| BasisError::ReactionRange { node, value: PublishedValue::SpringAlongDirection })?;
        let kappa = record.stiffness * t;
        if !kappa.is_finite() {
            return Err(BasisError::ReactionRange { node, value: PublishedValue::SpringAlongDirection });
        }
        let per_n = kappa / record.direction.scaled_norm_squared();
        let mut global = [0.0; 3];
        for i in 0..3 {
            global[i] = -(per_n * d[i]);
            if !global[i].is_finite() {
                return Err(BasisError::ReactionRange { node, value: PublishedValue::SpringComponent { axis: i } });
            }
        }
        let along = -(kappa / record.direction.scaled_norm());
        Ok(StopReaction { global, along })
    }

    /// Ground rows for rigid-body screening (§9): axis DOFs as today, then
    /// one direction row per stop and per positive skewed spring (d̃).
    pub fn ground_rows(&self) -> Vec<GroundRow> {
        self.axis_restrained
            .iter()
            .map(|&dof| GroundRow::Axis(dof))
            .chain(self.stops.iter().map(|s| GroundRow::Direction {
                node: s.node,
                d: s.direction.scaled(),
            }))
            .chain(
                self.springs
                    .iter()
                    .filter(|s| s.stiffness > 0.0)
                    .map(|s| GroundRow::Direction {
                        node: s.node,
                        d: s.direction.scaled(),
                    }),
            )
            .collect()
    }
}

/// The axis letter of a translation slot, for messages.
pub fn axis_name(axis: usize) -> &'static str {
    AXES[axis]
}

#[cfg(test)]
#[path = "restraint_basis_tests.rs"]
mod tests;
