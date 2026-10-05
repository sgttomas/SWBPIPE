//! K4: the adaptive schedule, the stop rule, the scales, the classification,
//! budgets and the evidence (T3 D1 §4.1.6, §4.1.6.1, §4.1.7, §5 item 1).
//!
//! - **Schedule.** "Candidates at p = 128, 256 and 512, each verified at 2p. The
//!   ceiling is 1024 bits." "The verification solve at 2p repeats formation
//!   from the binary64 operands." "A rejected 128 candidate's 256 verification
//!   becomes the next candidate … at most four solves." Widths (K3 ruling 8):
//!   128 and 256 at L = 4, 512 at L = 8, 1024 at L = 16; the residuals at p + 64
//!   (192, 320, 576) at L = 4, 8, 16; the ceiling's residual at 1024 (ROOT's Q4).
//! - **Solve and refinement** (D1 §4.1.4): the residual against K re-formed at
//!   p + 64 from the same primitives, each r_i one exact expansion of the ledger
//!   terms and the products, gated exactly against 64·γ_p(m_i) with M03's
//!   coalesced denominator; at most three corrections with the p-factor, each
//!   right-hand side rounded once from its exact sum.
//! - **Stop rule**: "|q_p − q_2p| ≤ 2^-64 · max(|q_2p|, S*)" on every published
//!   quantity, compared exactly, with S\* per body and kind formed at 2p (the
//!   coupling of §4.1.6.1 item 6, the products and quotients rounded once at 2p).
//! - **Classification** on the published binary64 values with the pinned
//!   binary64 formulas of §4.1.6.1 (R = 2^-34, t = fl(R·S\*), b = fl↑(2^-64·S\*),
//!   S\* < 2^-988 → `absolute_verified`), rule 2a for restrained and prescribed
//!   DOFs, and the per-member stress scales for F2a's product rows.
//! - **Budgets** (ROOT's K4 ruling Q5, amended): the per-case and
//!   per-invocation limits are required parameters with no default; K4 ships
//!   no numeric limit. Work is counted in limb-multiply equivalents per attempt;
//!   "successful, failed and verification work is all charged".
//! - **Factor reuse** (D1 §4.1.7): cases with the same stiffness identity share
//!   formation, assembly, ordering and the p-factor per precision. Each case's
//!   limit counts the full shared work; the invocation meter counts it once.
use super::assemble::{
    assemble, assemble_bounded, form_directional, form_members, reduced_rhs, BoundedCoefficients,
    DirectionalBlock, MemberOperators, Structure,
};
use super::bound::{block_refusals, free_blocks, BlockRatios, BlockRefusal, FreeBlocks};
use super::directed::binary64_up;
use super::factor::{
    geometry_first, order_free, BodyGeometry, GeometryRefusal, Ordering, RetainedFactor,
};
use super::ledger::{LedgerRefusal, RetainedLedger};
use super::origins::{BatchRecording, OriginSlot, RunPhase, RunTrace, SlotSnapshot};
use super::recover::{
    layout, publish_value, recover, state_encoding, Kind, QuantityId, QuantityMeta, Recovered,
};
use super::source::{put_u32, put_u64, Dof, PrimitiveSource};
use super::verify::{
    build_verify_shared, e_hat, phi_512, verify_state, VerificationReport, VerifyShared,
};
use super::wide::multi::{AttemptWork, Binary64Outcome, SupportedWidth, WideContext};
use super::wide::{Wide, WideError};
use super::wide_sum::{CloneWork, ExactWideSum, SumRefusal, SumWork};
use super::work::{WorkFault, WorkStatus, WorkStream, WorkTotal};
use crate::exact_sum::ExactAccumulator;
use crate::structural::StructuralError;
use std::cmp::Ordering as CmpOrdering;
use std::sync::Arc;

/// Proposed method token (a placeholder for ROOT, D1 §4.1).
pub const METHOD_TOKEN: &str = "contribution_preserving_multiprecision_v1";
/// ROOT's corrected SI publication policy: all R7 gates, then exact H admission.
/// Historical v1 evidence is not certified by this policy retroactively.
pub const POLICY: &str = "M03-INTEGRITY-MP-v2";
/// The label D1 §4.1.3 gives the published condition estimate.
pub const RCOND_LABEL: &str =
    "sensitivity to matrix-entry perturbation, not to authored parameters";
/// The solve precisions: candidates 128, 256, 512; the ceiling 1024.
pub const PRECISIONS: [u32; 4] = [128, 256, 512, 1024];
/// R = 2^-34.
pub const FLOOR_RATIO_BITS: u64 = 0x3DD0_0000_0000_0000;
/// k√2, the nearest double to √2 (≥ √2).
pub const K_SQRT2_BITS: u64 = 0x3FF6_A09E_667F_3BCD;
/// k_{2√2} = 2·k√2.
pub const K_TWO_SQRT2_BITS: u64 = 0x4006_A09E_667F_3BCD;

// ------------------------------------------------------------ budgets

/// The per-case work limit, in limb-multiply equivalents (required; no default).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CaseLimit(u64);

impl CaseLimit {
    pub fn new(limb_multiply_equivalents: u64) -> Self {
        Self(limb_multiply_equivalents)
    }
    pub fn get(self) -> u64 {
        self.0
    }
}

/// The per-invocation work meter (required; no default).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvocationMeter {
    limit: u64,
    charged: WorkTotal,
}

impl InvocationMeter {
    pub fn new(limit: u64) -> Self {
        Self {
            limit,
            charged: WorkTotal::zero(),
        }
    }
    pub fn charged(&self) -> u64 {
        self.charged.legacy_saturated()
    }
    pub fn limit(&self) -> u64 {
        self.limit
    }
    pub fn exhausted(&self) -> bool {
        self.charged.exact().map_or(true, |v| v >= self.limit)
    }
    pub fn checked_charged(&self) -> WorkTotal {
        self.charged
    }
    fn room(&self) -> WorkTotal {
        self.charged.room(self.limit)
    }
    fn charge(&mut self, amount: WorkTotal) {
        self.charged = self.charged.add(amount);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BudgetScope {
    Case,
    Invocation,
}

/// The promised radius that failed after all standing R7 gates passed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicationPredicate {
    AbsoluteBound,
    PublicRelative,
    SharperExact,
    SharperBinary64,
}

/// Malformed certificate data is a terminal error, never an absent zero.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CertificateIssue {
    Shape,
    PairIdentity,
    Precision,
    RowIdentity,
    MissingField,
    NegativeField,
    NonFinite,
    NonCanonicalZero,
    RadiusClassMismatch,
}

/// Why an attempt stopped.
#[derive(Debug, Clone, PartialEq)]
pub enum AttemptStop {
    CountRange(&'static str),
    WorkAccounting(WorkFault),
    Budget(BudgetScope),
    /// An exact sum exceeded the span limit (terminal: ROOT's ruling O4).
    Span,
    /// An exponent outside the `Wide` range (terminal).
    Exponent,
    /// Another `Wide` refusal (terminal; not expected for a valid source).
    Arithmetic(WideError),
    /// The sparse structure could not be built (terminal; internal).
    Structure,
    /// The pivot screen failed (escalates).
    Pivot {
        global_dof: usize,
    },
    /// A free DOF with an exactly zero diagonal (terminal).
    ZeroDiagonal {
        global_dof: usize,
    },
    /// A negative-energy pattern pair witnessed at p (terminal; a refusal).
    NegativeEnergy {
        i: usize,
        j: usize,
    },
    /// rcond ≤ 2^-(p−1) (escalates).
    Condition,
    /// The residual gate failed after bounded refinement (escalates).
    ResidualGate {
        global_dof: usize,
    },
    /// A body's formation scale E rounds to +∞ in binary64 (D1 revision
    /// 5a.3, R7 §4.1.6.2 item 4; ROOT's A3-0 ruling Q8): terminal.
    ResolutionScale {
        body: u32,
        kind: Kind,
    },
    /// The new publication certificate cannot be interpreted (terminal).
    PublicationCertificate {
        index: Option<usize>,
        issue: CertificateIssue,
    },
}

impl From<SumRefusal> for AttemptStop {
    fn from(refusal: SumRefusal) -> Self {
        match refusal {
            SumRefusal::CountRange(field) => Self::CountRange(field),
            SumRefusal::WorkAccounting(fault) => Self::WorkAccounting(fault),
            SumRefusal::Span => Self::Span,
            SumRefusal::Exponent => Self::Exponent,
            SumRefusal::NonFinite => Self::Arithmetic(WideError::NonFinite),
            SumRefusal::Wide(e) => Self::Arithmetic(e),
        }
    }
}

impl From<WideError> for AttemptStop {
    fn from(error: WideError) -> Self {
        match error {
            WideError::CountRange(field) => Self::CountRange(field),
            WideError::WorkAccounting(fault) => Self::WorkAccounting(fault),
            WideError::ExponentRange => Self::Exponent,
            other => Self::Arithmetic(other),
        }
    }
}

impl AttemptStop {
    /// Whether the schedule escalates to the next precision.
    pub(crate) fn escalates(&self) -> bool {
        matches!(
            self,
            Self::Pivot { .. } | Self::Condition | Self::ResidualGate { .. }
        )
    }
}

pub(crate) fn lme<const L: usize>(ctx: &WideContext<L>) -> WorkTotal
where
    Wide<L>: SupportedWidth,
{
    ctx.work().checked_lme(L)
}

/// A budget check point within one stage: `base` is the attempt's work in the
/// contexts the stage does not use; the stage's own context and accumulator
/// are read at each check.
#[derive(Debug, Clone, Copy)]
pub(crate) struct StageGuard {
    base: WorkTotal,
    case_room: WorkTotal,
    invocation_room: WorkTotal,
}

impl StageGuard {
    /// A guard that never stops (tests).
    #[cfg(test)]
    pub(crate) fn unlimited() -> Self {
        Self {
            base: WorkTotal::zero(),
            case_room: WorkTotal::exact_count(u64::MAX),
            invocation_room: WorkTotal::exact_count(u64::MAX),
        }
    }

    /// A guard with a case room (tests; T3 KF3's budget stop inside `uc`).
    #[cfg(test)]
    pub(crate) fn with_case_room(case_room: u64) -> Self {
        Self {
            base: WorkTotal::zero(),
            case_room: WorkTotal::exact_count(case_room),
            invocation_room: WorkTotal::exact_count(u64::MAX),
        }
    }

    pub(crate) fn with_base(self, base: WorkTotal) -> Result<Self, AttemptStop> {
        base.exact()?;
        self.case_room.exact()?;
        self.invocation_room.exact()?;
        Ok(Self { base, ..self })
    }

    pub(crate) fn test(&self, used: WorkTotal) -> Result<(), AttemptStop> {
        let used = used.exact()?;
        let case_room = self.case_room.exact()?;
        let invocation_room = self.invocation_room.exact()?;
        if used > case_room {
            Err(AttemptStop::Budget(BudgetScope::Case))
        } else if used > invocation_room {
            Err(AttemptStop::Budget(BudgetScope::Invocation))
        } else {
            Ok(())
        }
    }

    pub(crate) fn check<const L: usize>(
        &self,
        ctx: &WideContext<L>,
        sum: &ExactWideSum,
    ) -> Result<(), AttemptStop>
    where
        Wide<L>: SupportedWidth,
    {
        self.test(self.base.add(lme(ctx)).add(sum.work().checked_lme()))
    }
}

// ------------------------------------------------------------ §4.1.6.1 (binary64, pinned)

/// The next binary64 above a nonnegative finite x (+∞ above the largest).
pub(crate) fn next_up(x: f64) -> f64 {
    if x == 0.0 {
        return f64::from_bits(1);
    }
    f64::from_bits(x.to_bits() + 1)
}

/// The next binary64 below a positive finite x (0 below the smallest).
pub(crate) fn next_down(x: f64) -> f64 {
    if x <= f64::from_bits(1) {
        return 0.0;
    }
    f64::from_bits(x.to_bits() - 1)
}

/// Item 5: d_a = fl(max_a − min_a), L_b = fl(√(fl(fl(fl(d_x·d_x) + fl(d_y·d_y)) +
/// fl(d_z·d_z)))); 0 for a single node.
pub fn body_extent(coordinates: &[[f64; 3]]) -> f64 {
    if coordinates.is_empty() {
        return 0.0;
    }
    let mut d = [0.0f64; 3];
    for (a, extent) in d.iter_mut().enumerate() {
        let mut low = coordinates[0][a];
        let mut high = coordinates[0][a];
        for p in coordinates {
            low = low.min(p[a]);
            high = high.max(p[a]);
        }
        *extent = high - low;
    }
    ((d[0] * d[0] + d[1] * d[1]) + d[2] * d[2]).sqrt()
}

/// Item 6, in this order: tr = max(S_tr, fl(L_b·S_rot)); ro = max(S_rot,
/// fl(S_tr/L_b)); fo = max(S_fo, fl(S_mo/L_b)); mo = max(S_mo, fl(L_b·S_fo)). A
/// single-node body (L_b = 0) omits the coupled terms. `s` in `Kind` order.
pub fn coupled_scales(s: [f64; 4], extent: f64) -> [f64; 4] {
    if extent == 0.0 {
        return s;
    }
    let [tr, ro, fo, mo] = s;
    [
        tr.max(extent * ro),
        ro.max(tr / extent),
        fo.max(mo / extent),
        mo.max(extent * fo),
    ]
}

/// Item 7: σ_k(m) = fl(fl(fo/A) + fl(k·fl(mo/Z))).
pub fn stress_scale(fo: f64, mo: f64, area: f64, modulus: f64, k: f64) -> f64 {
    fo / area + k * (mo / modulus)
}

/// Item 7: k_i = fl↑(k√2·i): the nearest, then the next up when the nearest is
/// below the exact product (decided with an exact product).
pub fn intensified_k(i: f64) -> f64 {
    let k = f64::from_bits(K_SQRT2_BITS);
    let nearest = k * i;
    let exact_above = (|| -> Option<bool> {
        let mut ctx = WideContext::<4>::new(128).ok()?;
        let mut sum = ExactWideSum::new();
        sum.add_product(
            &mut ctx,
            &Wide::<4>::from_f64(k).ok()?,
            &Wide::<4>::from_f64(i).ok()?,
            false,
        )
        .ok()?;
        sum.add_binary64(nearest, true).ok()?;
        Some(sum.signum().ok()? > 0)
    })()
    .unwrap_or(false);
    if exact_above {
        next_up(nearest)
    } else {
        nearest
    }
}

/// t = fl(R·S*).
pub fn threshold(s_star: f64) -> f64 {
    f64::from_bits(FLOOR_RATIO_BITS) * s_star
}

/// b = fl↑(2^-64·S*), for S* ≥ 0: exact whenever 2^-64·S* is representable,
/// at least 2^-1074 for any 0 < S*, and 0 only at S* = 0.
pub fn absolute_bound(s_star: f64) -> f64 {
    let two64 = 18_446_744_073_709_551_616.0_f64;
    let nearest = s_star / two64;
    if nearest * two64 < s_star {
        next_up(nearest)
    } else {
        nearest
    }
}

/// The verified-accuracy class of a published row of a scaled kind.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RowClass {
    RelativeVerified,
    /// Withheld from reliance, with its absolute bound b (bits).
    AbsoluteVerified {
        bound_bits: u64,
    },
    /// Rule 2a (no threshold).
    InputDerived,
    /// A value the publication could not give (underflow or overflow): no
    /// class; F2a decides its standing (ROOT's K4 ruling O9).
    Unpublishable,
}

/// An `absolute_verified` row's bound. D1 revision 5a.3's amendment A1
/// (ROOT's ruling on RV19-6): where 0 < S\* < 2^-988 a row can reach |q| ≈ S\*,
/// so its bound carries its own publication rounding,
/// b_row = fl↑(fl↑(2^-64·S\*) + fl↑(2^-53·|q|) + 2^-1074), the sum rounded
/// upward once, decided exactly. Elsewhere b = fl↑(2^-64·S\*) is unchanged:
/// an absolute row there has |q| < 2^-34·S\*, whose rounding is below 2^-23·b
/// (R7 §5.2); and b = 0 at S\* = 0.
pub(crate) fn row_bound(value: f64, s_star: f64) -> f64 {
    let small = f64::from_bits(0x0230_0000_0000_0000); // 2^-988
    let b = absolute_bound(s_star);
    if s_star == 0.0 || s_star >= small {
        return b;
    }
    // fl↑(2^-53·|q|): the scaling back is exact, so the comparison decides.
    let two53 = 9_007_199_254_740_992.0_f64;
    let q = value.abs();
    let nearest = q / two53;
    let rounding = if nearest * two53 < q {
        next_up(nearest)
    } else {
        nearest
    };
    // The three terms span at most 2,100 bits: the sum and its upward
    // rounding are exact (no refusal is reachable).
    (|| -> Option<f64> {
        let mut num = ExactWideSum::new();
        num.add_binary64(b, false).ok()?;
        num.add_binary64(rounding, false).ok()?;
        num.add_binary64(f64::from_bits(1), false).ok()?;
        let mut den = ExactWideSum::new();
        den.add_binary64(1.0, false).ok()?;
        let mut ctx = WideContext::<16>::new(1024).ok()?;
        directed_ratio(&mut ctx, &num, &den, Direction::Up).ok()
    })()
    .unwrap_or(f64::INFINITY)
}

/// D1 §4.1.6 item 1 on the published value: `absolute_verified` iff
/// |q| < fl(R·S\*), and always when S\* < 2^-988, with the bound of
/// `row_bound` (amendment A1).
pub fn classify(value: f64, s_star: f64) -> RowClass {
    if relative_class(value, s_star) {
        RowClass::RelativeVerified
    } else {
        RowClass::AbsoluteVerified {
            bound_bits: row_bound(value, s_star).to_bits(),
        }
    }
}

fn relative_class(value: f64, s_star: f64) -> bool {
    // V-K seeded fault VK-F17 (§7.3-17): every scaled row relative_verified.
    #[cfg(any(test, feature = "mutation-controls"))]
    if super::seeded::active(super::seeded::Fault::F17) {
        return true;
    }
    let small = f64::from_bits(0x0230_0000_0000_0000); // 2^-988
    !(s_star < small || value.abs() < threshold(s_star))
}

// ------------------------------------------------------------ directed ratios

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Direction {
    Up,
    Down,
}

/// c·den ≥ num (exact), for c a nonnegative binary64.
fn product_reaches(c: f64, den: &ExactWideSum, num: &ExactWideSum) -> Result<bool, AttemptStop> {
    let mut t = ExactWideSum::new();
    if c != 0.0 {
        let bits = c.to_bits();
        let biased = ((bits >> 52) & 0x7ff) as i64;
        let fraction = bits & ((1u64 << 52) - 1);
        let (significand, lsb) = if biased == 0 {
            (fraction, -1074)
        } else {
            (fraction | (1u64 << 52), biased - 1075)
        };
        t.add_scaled(den, false, significand, lsb)?;
    }
    t.add_scaled(num, true, 1, 0)?;
    Ok(t.signum()? >= 0)
}

/// num/den rounded to binary64 in the given direction (num ≥ 0, den > 0, both
/// exact): the least binary64 ≥ the quotient (Up) or the largest ≤ it (Down).
/// The approximation (both rounded to 1024 bits, divided) is corrected with
/// exact comparisons, so the result is exactly the directed rounding.
pub(crate) fn directed_ratio(
    ctx: &mut WideContext<16>,
    num: &ExactWideSum,
    den: &ExactWideSum,
    direction: Direction,
) -> Result<f64, AttemptStop> {
    num.ensure_valid()?;
    den.ensure_valid()?;
    let mut n = num.clone();
    if n.is_zero()? {
        return Ok(0.0);
    }
    let mut d = den.clone();
    let nv = n.round(ctx)?;
    let dv = d.round(ctx)?;
    let quotient = ctx.div(&nv, &dv)?;
    let mut c = match quotient.to_binary64() {
        Binary64Outcome::Normal(v) => v.abs(),
        Binary64Outcome::Subnormal { value, .. } => value.abs(),
        Binary64Outcome::Underflow { .. } => 0.0,
        Binary64Outcome::Overflow { .. } => f64::MAX,
    };
    match direction {
        Direction::Up => {
            while !product_reaches(c, den, num)? {
                if c == f64::MAX {
                    return Ok(f64::INFINITY);
                }
                c = next_up(c);
            }
            while c > 0.0 && product_reaches(next_down(c), den, num)? {
                c = next_down(c);
            }
        }
        Direction::Down => {
            // Largest c with c·den ≤ num.
            let exceeds = |c: f64| -> Result<bool, AttemptStop> {
                // c·den > num ⟺ not (num ≥ c·den)
                let mut t = ExactWideSum::new();
                t.add_scaled(num, false, 1, 0)?;
                if c != 0.0 {
                    let bits = c.to_bits();
                    let biased = ((bits >> 52) & 0x7ff) as i64;
                    let fraction = bits & ((1u64 << 52) - 1);
                    let (significand, lsb) = if biased == 0 {
                        (fraction, -1074)
                    } else {
                        (fraction | (1u64 << 52), biased - 1075)
                    };
                    t.add_scaled(den, true, significand, lsb)?;
                }
                Ok(t.signum()? < 0)
            };
            while c > 0.0 && exceeds(c)? {
                c = next_down(c);
            }
            while c < f64::MAX && !exceeds(next_up(c))? {
                c = next_up(c);
            }
        }
    }
    Ok(c)
}

/// A quick binary64 approximation of num/den (both at 64 bits), for choosing
/// the rows whose exact directed ratio is computed.
fn approximate_ratio(
    ctx: &mut WideContext<4>,
    num: &ExactWideSum,
    den: &ExactWideSum,
) -> Result<f64, AttemptStop> {
    num.ensure_valid()?;
    den.ensure_valid()?;
    let mut n = num.clone();
    let mut d = den.clone();
    let nv = n.round(ctx)?;
    if nv.is_zero() {
        return Ok(0.0);
    }
    let dv = d.round(ctx)?;
    if dv.is_zero() {
        return Ok(f64::INFINITY);
    }
    Ok(match ctx.div(&nv, &dv)?.to_binary64() {
        Binary64Outcome::Normal(v) => v.abs(),
        Binary64Outcome::Subnormal { value, .. } => value.abs(),
        Binary64Outcome::Underflow { .. } => 0.0,
        Binary64Outcome::Overflow { .. } => f64::INFINITY,
    })
}

/// Width, in units in the last place of the binary64 approximations, of the
/// window around the running extreme within which rows are kept for the exact
/// evaluation (the approximations are within 2 ulps of the exact ratios).
const WINDOW_ULPS: u64 = 1 << 13;

/// KF1: T, the most rows one tracker holds unevaluated. At T it evaluates them
/// (a collapse), so a tracker's memory does not depend on its rows. T = 512
/// (ROOT's ruling "KF1: D received; T reopened and set to 512"): no result
/// depends on T, only the work of rows collapsed and later dropped.
pub(crate) const TRACKER_ROWS: usize = 512;

/// KF1: G, the most unevaluated rows (counted by allocated capacity) the
/// trackers of one call hold together: 8·T, so the eight trackers `rule` keeps
/// for one body never reach it, and many small bodies are bounded as well.
pub(crate) const TRACKER_SET_ROWS: usize = 8 * TRACKER_ROWS;

#[cfg(not(test))]
fn tracker_rows() -> usize {
    TRACKER_ROWS
}

/// T, or this thread's test override (KF1's model-level differential).
#[cfg(test)]
fn tracker_rows() -> usize {
    tracker_hook::get().unwrap_or(TRACKER_ROWS)
}

fn tracker_set_rows() -> usize {
    tracker_rows().saturating_mul(TRACKER_SET_ROWS / TRACKER_ROWS)
}

/// Whether key k lies within the window of the best key b (as bit patterns).
fn in_window(k: u64, b: u64) -> bool {
    k.abs_diff(b) <= WINDOW_ULPS
}

/// A row evaluated at a collapse: its exact directed ratio, or the refusal its
/// exact evaluation met, with the row's place in the stream.
#[derive(Debug, Clone)]
enum Evaluated {
    Ratio(f64),
    Refused { seq: u64, stop: AttemptStop },
}

impl Evaluated {
    /// Whether `self` decides `finish` ahead of `other`: a refusal before any
    /// ratio, the earlier of two refusals, and the more extreme of two ratios.
    fn beats(&self, other: &Self, direction: Direction) -> bool {
        match (self, other) {
            (Self::Refused { seq: a, .. }, Self::Refused { seq: b, .. }) => a < b,
            (Self::Refused { .. }, Self::Ratio(_)) => true,
            (Self::Ratio(_), Self::Refused { .. }) => false,
            (Self::Ratio(a), Self::Ratio(b)) => match direction {
                Direction::Up => a > b,
                Direction::Down => a < b,
            },
        }
    }
}

fn extreme(best: Option<f64>, value: f64, direction: Direction) -> f64 {
    match (best, direction) {
        (None, _) => value,
        (Some(b), Direction::Up) => b.max(value),
        (Some(b), Direction::Down) => b.min(value),
    }
}

/// The exact directed extreme of a stream of ratios num/den (both exact), in
/// memory that does not depend on the stream (KF1).
///
/// A row counts while its 64-bit approximation (its key, compared as a bit
/// pattern) lies within `WINDOW_ULPS` of the running approximate extreme,
/// exactly as K4's tracker kept it; `finish` returns the directed extreme of
/// the exact ratios of the rows that count, or the refusal of the earliest of
/// them whose exact evaluation refuses, bit for bit as K4's did.
///
/// At most T rows are held unevaluated. A collapse evaluates them exactly and
/// keeps (key, outcome) entries, pruned to those that no entry nearer the best
/// key (larger for Up, smaller for Down) with an outcome that decides `finish`
/// at least as early dominates. The window removes rows by key alone, and a
/// dominating entry's key is nearer the best, so it outlasts every row it
/// dominates: the result is the same for any collapse schedule (KF1 plan §2).
pub(crate) struct BoundedExtremeTracker {
    direction: Direction,
    limit: usize,
    best: Option<u64>,
    offered: u64,
    /// Unevaluated rows: (num, den, key, place in the stream).
    lazy: Vec<(ExactWideSum, ExactWideSum, u64, u64)>,
    /// Evaluated entries (key, outcome), nearest the best first.
    table: Vec<(u64, Evaluated)>,
}

impl BoundedExtremeTracker {
    pub(crate) fn new(direction: Direction) -> Self {
        Self::with_limit(direction, tracker_rows())
    }

    pub(crate) fn with_limit(direction: Direction, limit: usize) -> Self {
        Self {
            direction,
            limit: limit.max(1),
            best: None,
            offered: 0,
            lazy: Vec::new(),
            table: Vec::new(),
        }
    }

    pub(crate) fn offer(
        &mut self,
        ctx64: &mut WideContext<4>,
        ctx16: &mut WideContext<16>,
        num: ExactWideSum,
        den: ExactWideSum,
    ) -> Result<(), AttemptStop> {
        let key = approximate_ratio(ctx64, &num, &den)?.to_bits();
        let seq = self.offered;
        self.offered = self
            .offered
            .checked_add(1)
            .ok_or(AttemptStop::CountRange("tracker sequence"))?;
        let better = match (self.best, self.direction) {
            (None, _) => true,
            (Some(b), Direction::Up) => key > b,
            (Some(b), Direction::Down) => key < b,
        };
        if better {
            self.best = Some(key);
            self.lazy.retain(|row| in_window(row.2, key));
            if self.lazy.is_empty() {
                self.lazy = Vec::new();
            }
            self.table.retain(|entry| in_window(entry.0, key));
        }
        if self.best.is_some_and(|b| in_window(key, b)) {
            if self.lazy.len() >= self.limit {
                self.collapse(ctx16)?;
            }
            self.lazy.push((num, den, key, seq));
        }
        Ok(())
    }

    /// Evaluates the unevaluated rows exactly (a refusal is recorded, not
    /// returned) and releases their memory.
    pub(crate) fn collapse(&mut self, ctx16: &mut WideContext<16>) -> Result<(), AttemptStop> {
        let lazy = std::mem::take(&mut self.lazy);
        if lazy.is_empty() {
            return Ok(());
        }
        for (num, den, key, seq) in &lazy {
            let outcome = match directed_ratio(ctx16, num, den, self.direction) {
                Ok(value) => Evaluated::Ratio(value),
                Err(stop @ AttemptStop::WorkAccounting(_)) => return Err(stop),
                Err(stop) => Evaluated::Refused { seq: *seq, stop },
            };
            self.table.push((*key, outcome));
        }
        drop(lazy);
        self.prune();
        Ok(())
    }

    /// Keeps the entries no other entry dominates: sorted nearest the best
    /// first (within a key, the most decisive first), an entry stays only if it
    /// decides `finish` ahead of every entry kept before it.
    fn prune(&mut self) {
        let direction = self.direction;
        self.table.sort_by(|a, b| {
            let by_key = match direction {
                Direction::Up => b.0.cmp(&a.0),
                Direction::Down => a.0.cmp(&b.0),
            };
            by_key.then_with(|| {
                if a.1.beats(&b.1, direction) {
                    CmpOrdering::Less
                } else if b.1.beats(&a.1, direction) {
                    CmpOrdering::Greater
                } else {
                    CmpOrdering::Equal
                }
            })
        });
        let mut kept: Vec<(u64, Evaluated)> = Vec::new();
        for entry in self.table.drain(..) {
            if kept
                .last()
                .is_none_or(|last| entry.1.beats(&last.1, direction))
            {
                kept.push(entry);
            }
        }
        kept.shrink_to_fit();
        self.table = kept;
    }

    pub(crate) fn finish(self, ctx16: &mut WideContext<16>) -> Result<Option<f64>, AttemptStop> {
        let mut best: Option<f64> = None;
        let mut refused: Option<(u64, AttemptStop)> = None;
        for (_, outcome) in self.table {
            match outcome {
                Evaluated::Ratio(value) => best = Some(extreme(best, value, self.direction)),
                Evaluated::Refused { seq, stop } => {
                    if refused.as_ref().is_none_or(|r| seq < r.0) {
                        refused = Some((seq, stop));
                    }
                }
            }
        }
        // A refusal among the evaluated rows came before every unevaluated row.
        if let Some((_, stop)) = refused {
            return Err(stop);
        }
        for (num, den, _, _) in &self.lazy {
            let exact = directed_ratio(ctx16, num, den, self.direction)?;
            best = Some(extreme(best, exact, self.direction));
        }
        Ok(best)
    }

    fn capacity(&self) -> usize {
        self.lazy.capacity()
    }

    /// (Unevaluated rows, their allocated capacity, evaluated entries).
    #[cfg(test)]
    pub(crate) fn held(&self) -> (usize, usize, usize) {
        (self.lazy.len(), self.lazy.capacity(), self.table.len())
    }
}

/// The trackers of one call, by key, with G bounding their unevaluated rows
/// together (counted by allocated capacity). When an offer takes them above
/// G, every tracker holding rows collapses.
pub(crate) struct TrackerSet<K: Ord + Copy> {
    trackers: std::collections::BTreeMap<K, BoundedExtremeTracker>,
    /// The trackers with allocated unevaluated rows.
    holding: std::collections::BTreeSet<K>,
    held: usize,
    limit: usize,
}

impl<K: Ord + Copy> TrackerSet<K> {
    pub(crate) fn new() -> Self {
        Self::with_limit(tracker_set_rows())
    }

    pub(crate) fn with_limit(limit: usize) -> Self {
        Self {
            trackers: std::collections::BTreeMap::new(),
            holding: std::collections::BTreeSet::new(),
            held: 0,
            limit,
        }
    }

    pub(crate) fn offer(
        &mut self,
        key: K,
        direction: Direction,
        ctx64: &mut WideContext<4>,
        ctx16: &mut WideContext<16>,
        num: ExactWideSum,
        den: ExactWideSum,
    ) -> Result<(), AttemptStop> {
        let tracker = self
            .trackers
            .entry(key)
            .or_insert_with(|| BoundedExtremeTracker::new(direction));
        let before = tracker.capacity();
        tracker.offer(ctx64, ctx16, num, den)?;
        let after = tracker.capacity();
        self.held = self
            .held
            .checked_sub(before)
            .and_then(|n| n.checked_add(after))
            .ok_or(AttemptStop::CountRange("tracker capacity"))?;
        if before == 0 && after > 0 {
            self.holding.insert(key);
        } else if before > 0 && after == 0 {
            self.holding.remove(&key);
        }
        if self.held > self.limit {
            for k in std::mem::take(&mut self.holding) {
                if let Some(t) = self.trackers.get_mut(&k) {
                    t.collapse(ctx16)?;
                }
            }
            self.held = 0;
        }
        Ok(())
    }

    /// The trackers in key order.
    pub(crate) fn into_trackers(
        self,
    ) -> std::collections::btree_map::IntoIter<K, BoundedExtremeTracker> {
        self.trackers.into_iter()
    }

    /// (Unevaluated rows' allocated capacity, over every tracker; G.)
    #[cfg(test)]
    pub(crate) fn held(&self) -> (usize, usize) {
        (self.held, self.limit)
    }

    #[cfg(test)]
    pub(crate) fn trackers(&self) -> impl Iterator<Item = (&K, &BoundedExtremeTracker)> {
        self.trackers.iter()
    }
}

// ------------------------------------------------------------ per-case and shared preparation

/// A case's precision-independent data. A combination (ROOT's ruling on
/// I12's F-1) is a case of its own: the combined exact ledger Σ cᵢ·fᵢ and the
/// combined prescribed values Σ cᵢ·vᵢ, on its operands' stiffness source.
#[derive(Debug)]
pub(crate) struct CasePrep {
    /// The stiffness source (for a combination, its first operand's: the
    /// stiffness identity, layout and extents are its operands' own).
    pub(crate) source: PrimitiveSource,
    pub(crate) ledger: RetainedLedger,
    /// Per constrained global DOF (ascending), the exact terms c·v of its
    /// value: [(1, v)] for a case, [(cᵢ, vᵢ)] for a combination. At p the
    /// value is their exact sum rounded once.
    pub(crate) prescribed: Vec<(usize, Vec<(f64, f64)>)>,
    /// The combination's factors (empty for a case).
    pub(crate) factors: Vec<f64>,
    /// The source encoding, or for a combination "K4CMB": its factors' bits
    /// and its operands' source encodings.
    pub(crate) identity: Vec<u8>,
    pub(crate) layout: Vec<QuantityMeta>,
    /// L_b per body (item 5).
    pub(crate) extents: Vec<f64>,
}

pub(crate) enum CombinationPreparationError {
    Ledger(LedgerRefusal),
    CountRange(&'static str),
}
impl From<LedgerRefusal> for CombinationPreparationError {
    fn from(e: LedgerRefusal) -> Self {
        Self::Ledger(e)
    }
}

impl CasePrep {
    pub(crate) fn new(source: PrimitiveSource) -> Result<Self, LedgerRefusal> {
        let ledger = RetainedLedger::from_source(&source)?;
        let prescribed = source
            .constraints()
            .iter()
            .map(|c| (c.dof.global(), vec![(1.0, c.value)]))
            .collect();
        let identity = source.encoding();
        Self::with(source, ledger, prescribed, Vec::new(), identity)
    }

    /// A combination Σ cᵢ·(case i) of case preparations that share one
    /// stiffness identity and layout (checked by the caller).
    pub(crate) fn combination(
        operands: &[(f64, &CasePrep)],
    ) -> Result<Self, CombinationPreparationError> {
        let fail = || CombinationPreparationError::CountRange("combination encoding");
        u32::try_from(operands.len()).map_err(|_| fail())?;
        let mut bytes = 10usize;
        let mut loads = 0usize;
        for (_, prep) in operands {
            u32::try_from(prep.identity.len()).map_err(|_| fail())?;
            bytes = bytes
                .checked_add(12)
                .and_then(|n| n.checked_add(prep.identity.len()))
                .ok_or_else(fail)?;
            loads = loads
                .checked_add(prep.source.loads().len())
                .ok_or_else(fail)?;
        }
        std::alloc::Layout::array::<u8>(bytes).map_err(|_| fail())?;
        std::alloc::Layout::array::<(usize, crate::exact_sum::ExactAccumulator)>(loads)
            .map_err(|_| fail())?;
        let first = operands.first().ok_or_else(fail)?.1;
        let pairs = operands
            .len()
            .checked_mul(first.prescribed.len())
            .ok_or_else(fail)?;
        std::alloc::Layout::array::<(f64, f64)>(pairs).map_err(|_| fail())?;
        let sources: Vec<(f64, &PrimitiveSource)> =
            operands.iter().map(|(c, p)| (*c, &p.source)).collect();
        let ledger = RetainedLedger::combined(&sources)?;
        let first = &operands[0].1.source;
        let prescribed = first
            .constraints()
            .iter()
            .map(|c| {
                let g = c.dof.global();
                let terms = sources
                    .iter()
                    .map(|(factor, s)| (*factor, s.constraint(g).unwrap_or(0.0)))
                    .collect();
                (g, terms)
            })
            .collect();
        let mut identity = b"K4CMB\x01".to_vec();
        put_u32(&mut identity, operands.len() as u32);
        for (factor, prep) in operands {
            put_u64(&mut identity, factor.to_bits());
            put_u32(&mut identity, prep.identity.len() as u32);
            identity.extend_from_slice(&prep.identity);
        }
        let factors = operands.iter().map(|o| o.0).collect();
        Self::with(first.clone(), ledger, prescribed, factors, identity)
            .map_err(CombinationPreparationError::from)
    }

    fn with(
        source: PrimitiveSource,
        ledger: RetainedLedger,
        prescribed: Vec<(usize, Vec<(f64, f64)>)>,
        factors: Vec<f64>,
        identity: Vec<u8>,
    ) -> Result<Self, LedgerRefusal> {
        let layout = layout(&source);
        let extents = (0..source.body_count())
            .map(|b| {
                let coordinates: Vec<[f64; 3]> = source
                    .body_nodes(b)
                    .iter()
                    .map(|&n| source.nodes()[n as usize])
                    .collect();
                body_extent(&coordinates)
            })
            .collect();
        Ok(Self {
            source,
            ledger,
            prescribed,
            factors,
            identity,
            layout,
            extents,
        })
    }

    /// The published prescribed rows (the input-derived displacements), each
    /// from its exact sum of terms c·v rounded once to binary64 (V4's NOTE on
    /// 5a.3: a combination's prescribed value rounded at p first would be
    /// rounded twice). A case's single binary64 value publishes unchanged.
    fn publish_prescribed(&self, values: &mut [Binary64Outcome]) {
        for (meta, value) in self.layout.iter().zip(values.iter_mut()) {
            let (true, QuantityId::Displacement(dof)) = (meta.input_derived, meta.id) else {
                continue;
            };
            let Ok(k) = self.prescribed.binary_search_by_key(&dof.global(), |t| t.0) else {
                continue;
            };
            *value = exact_publication(&self.prescribed[k].1);
        }
    }

    /// The prescribed values at the context's precision, written into u: each
    /// the exact sum of its terms c·v, rounded once.
    fn prescribed_at<const L: usize>(
        &self,
        ctx: &mut WideContext<L>,
        sum: &mut ExactWideSum,
        u: &mut [Wide<L>],
    ) -> Result<(), AttemptStop>
    where
        Wide<L>: SupportedWidth,
    {
        for (g, terms) in &self.prescribed {
            sum.clear();
            for &(factor, value) in terms {
                if factor != 0.0 && value != 0.0 {
                    let (c, v) = (Wide::<L>::from_f64(factor)?, Wide::<L>::from_f64(value)?);
                    sum.add_product(ctx, &c, &v, false)?;
                }
            }
            u[*g] = sum.round(ctx)?;
        }
        Ok(())
    }
}

/// Σ c·v over binary64 pairs, rounded once to binary64 with its outcome: the
/// exact accumulator's single rounding; an exact zero is +0.0; a nonzero sum
/// that rounds to zero is `Underflow`, one beyond the range `Overflow`.
fn exact_publication(terms: &[(f64, f64)]) -> Binary64Outcome {
    let mut accumulator = ExactAccumulator::new();
    for &(c, v) in terms {
        if accumulator.add_product(c, v).is_err() {
            // Not reached for finite binary64 pairs (their products lie in the
            // accumulator's range); the p-rounded state is not substituted.
            return Binary64Outcome::Overflow {
                negative: (c < 0.0) != (v < 0.0),
            };
        }
    }
    let negative = accumulator.signum() < 0;
    match accumulator.round() {
        Ok(x) if x == 0.0 => {
            if accumulator.signum() == 0 {
                Binary64Outcome::Normal(0.0)
            } else {
                Binary64Outcome::Underflow { negative }
            }
        }
        // The rounded value is exact in `Wide`: K3's conversion only labels it
        // (normal, or subnormal with its relative precision).
        Ok(x) => Wide::<4>::from_f64(x)
            .map(|w| w.to_binary64())
            .unwrap_or(Binary64Outcome::Overflow { negative }),
        Err(_) => Binary64Outcome::Overflow { negative },
    }
}

/// A stiffness identity's precision-independent data (shared by its cases).
#[derive(Debug)]
pub(crate) struct GroupPrep {
    pub(crate) structure: Structure,
    pub(crate) ordering: Ordering,
    pub(crate) geometry: Vec<BodyGeometry>,
    /// The free–free blocks (D1 revision 5a.3, R7 §4.1.6.3 item 7a).
    pub(crate) blocks: FreeBlocks,
}

/// Work of the shared stages at one precision.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct StageWork {
    status: WorkStatus,
    pub formation: u64,
    pub assembly: u64,
    pub residual_formation: u64,
    pub factor: u64,
    pub condition: u64,
    pub rhs: u64,
    pub solve: u64,
    pub refinement: u64,
    pub recovery: u64,
    pub stop_rule: u64,
    /// D1 revision 5a.3: the gate's fallback (Ā^q and the bounded rows).
    pub bounded_gate: u64,
    /// The verification pass: E; the estimate (r, δ̂, Ŵ); the charge (r₂,
    /// the norms, ‖ā_q S‖₁, t, C and W⁺); the bounds (data flags, B_c, θ, g);
    /// the shifted factorizations.
    pub scale: u64,
    pub estimate: u64,
    pub charge: u64,
    pub bound: u64,
    pub shift: u64,
    /// The verification's shared stages: Ā at P, K_e at q_W, the Uc passes.
    pub bounded_formation: u64,
    pub wide_formation: u64,
    pub uc: u64,
}

/// A stage of `StageWork` (T3 KF3: the stage in progress when a build stops).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Stage {
    StopRule,
    Formation,
    Assembly,
    ResidualFormation,
    Factor,
    Condition,
    Rhs,
    Solve,
    Refinement,
    Recovery,
    BoundedGate,
    Scale,
    Estimate,
    Charge,
    Bound,
    Shift,
    BoundedFormation,
    WideFormation,
    Uc,
}

impl StageWork {
    pub fn checked_total(&self) -> WorkTotal {
        [
            self.formation,
            self.assembly,
            self.residual_formation,
            self.factor,
            self.condition,
            self.rhs,
            self.solve,
            self.refinement,
            self.recovery,
            self.stop_rule,
            self.bounded_gate,
            self.scale,
            self.estimate,
            self.charge,
            self.bound,
            self.shift,
            self.bounded_formation,
            self.wide_formation,
            self.uc,
        ]
        .into_iter()
        .fold(WorkTotal::zero().join_status(self.status), |t, n| {
            t.add(WorkTotal::exact_count(n))
        })
    }
    #[cfg(test)]
    pub(crate) fn total(&self) -> u64 {
        self.checked_total().legacy_saturated()
    }
    fn slot(&mut self, stage: Stage) -> &mut u64 {
        match stage {
            Stage::Formation => &mut self.formation,
            Stage::Assembly => &mut self.assembly,
            Stage::ResidualFormation => &mut self.residual_formation,
            Stage::Factor => &mut self.factor,
            Stage::Condition => &mut self.condition,
            Stage::Rhs => &mut self.rhs,
            Stage::Solve => &mut self.solve,
            Stage::Refinement => &mut self.refinement,
            Stage::Recovery => &mut self.recovery,
            Stage::StopRule => &mut self.stop_rule,
            Stage::BoundedGate => &mut self.bounded_gate,
            Stage::Scale => &mut self.scale,
            Stage::Estimate => &mut self.estimate,
            Stage::Charge => &mut self.charge,
            Stage::Bound => &mut self.bound,
            Stage::Shift => &mut self.shift,
            Stage::BoundedFormation => &mut self.bounded_formation,
            Stage::WideFormation => &mut self.wide_formation,
            Stage::Uc => &mut self.uc,
        }
    }
    pub(crate) fn set(&mut self, stage: Stage, work: WorkTotal) -> Result<(), WorkFault> {
        self.status = self.status.join(work.status());
        *self.slot(stage) = work.legacy_saturated();
        self.status = self.status.join(self.checked_total().status());
        self.checked_total().exact().map(|_| ())
    }
    pub(crate) fn add_to(&mut self, stage: Stage, work: WorkTotal) -> Result<(), WorkFault> {
        let old = WorkTotal::exact_count(*self.slot(stage));
        self.set(stage, old.add(work))
    }
    pub(crate) fn close_stopped<T, E>(
        &mut self,
        result: &Result<T, E>,
        current: Stage,
        total: WorkTotal,
    ) {
        self.status = self
            .status
            .join(total.status())
            .join(self.checked_total().status());
        if result.is_err() {
            let rest = total.remainder(self.checked_total());
            self.status = self.status.join(rest.status());
            if rest.status().is_exact() {
                let _ = self.add_to(current, rest);
            }
        }
    }
    pub fn merge(&mut self, other: &Self) -> Result<(), WorkFault> {
        self.status = self.status.join(other.status);
        let _ = self.add_to(Stage::Formation, WorkTotal::exact_count(other.formation));
        let _ = self.add_to(Stage::Assembly, WorkTotal::exact_count(other.assembly));
        let _ = self.add_to(
            Stage::ResidualFormation,
            WorkTotal::exact_count(other.residual_formation),
        );
        let _ = self.add_to(Stage::Factor, WorkTotal::exact_count(other.factor));
        let _ = self.add_to(Stage::Condition, WorkTotal::exact_count(other.condition));
        let _ = self.add_to(Stage::Rhs, WorkTotal::exact_count(other.rhs));
        let _ = self.add_to(Stage::Solve, WorkTotal::exact_count(other.solve));
        let _ = self.add_to(Stage::Refinement, WorkTotal::exact_count(other.refinement));
        let _ = self.add_to(Stage::Recovery, WorkTotal::exact_count(other.recovery));
        let _ = self.add_to(Stage::StopRule, WorkTotal::exact_count(other.stop_rule));
        let _ = self.add_to(
            Stage::BoundedGate,
            WorkTotal::exact_count(other.bounded_gate),
        );
        let _ = self.add_to(Stage::Scale, WorkTotal::exact_count(other.scale));
        let _ = self.add_to(Stage::Estimate, WorkTotal::exact_count(other.estimate));
        let _ = self.add_to(Stage::Charge, WorkTotal::exact_count(other.charge));
        let _ = self.add_to(Stage::Bound, WorkTotal::exact_count(other.bound));
        let _ = self.add_to(Stage::Shift, WorkTotal::exact_count(other.shift));
        let _ = self.add_to(
            Stage::BoundedFormation,
            WorkTotal::exact_count(other.bounded_formation),
        );
        let _ = self.add_to(
            Stage::WideFormation,
            WorkTotal::exact_count(other.wide_formation),
        );
        let _ = self.add_to(Stage::Uc, WorkTotal::exact_count(other.uc));
        self.checked_total().exact().map(|_| ())
    }
    pub(crate) fn add(&mut self, other: &Self) -> Result<(), WorkFault> {
        self.merge(other)
    }
}
impl std::fmt::Debug for StageWork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("StageWork");
        d.field("formation", &self.formation);
        d.field("assembly", &self.assembly);
        d.field("residual_formation", &self.residual_formation);
        d.field("factor", &self.factor);
        d.field("condition", &self.condition);
        d.field("rhs", &self.rhs);
        d.field("solve", &self.solve);
        d.field("refinement", &self.refinement);
        d.field("recovery", &self.recovery);
        d.field("stop_rule", &self.stop_rule);
        d.field("bounded_gate", &self.bounded_gate);
        d.field("scale", &self.scale);
        d.field("estimate", &self.estimate);
        d.field("charge", &self.charge);
        d.field("bound", &self.bound);
        d.field("shift", &self.shift);
        d.field("bounded_formation", &self.bounded_formation);
        d.field("wide_formation", &self.wide_formation);
        d.field("uc", &self.uc);
        if !self.status.is_exact() {
            d.field("work_status", &self.status);
        }
        d.finish()
    }
}

impl From<WorkFault> for AttemptStop {
    fn from(fault: WorkFault) -> Self {
        Self::WorkAccounting(fault)
    }
}

pub(crate) fn finish_work<T>(
    result: &mut Result<T, AttemptStop>,
    total: WorkTotal,
    stages: &StageWork,
) -> WorkTotal {
    let mut total = total.join_status(stages.checked_total().status());
    if let Err(AttemptStop::WorkAccounting(fault)) = result {
        total = total.join_status(WorkStatus::from_fault(*fault));
    }
    if result.is_ok() {
        if let Err(fault) = total.exact() {
            *result = Err(fault.into());
        }
    }
    total
}

/// Formation, assembly, the p-factor and its screens at one precision, shared
/// by the cases of a stiffness identity.
#[derive(Debug)]
pub(crate) struct Shared<const L: usize, const R: usize>
where
    Wide<L>: SupportedWidth,
    Wide<R>: SupportedWidth,
{
    pub(crate) p: u32,
    /// The residual precision: p + 64, or p at the ceiling (ROOT's Q4).
    pub(crate) q: u32,
    pub(crate) members: Vec<MemberOperators<L>>,
    pub(crate) directional: Vec<DirectionalBlock<L>>,
    pub(crate) k: Vec<Wide<L>>,
    pub(crate) k_q: Vec<Wide<R>>,
    /// The bounded-operator coefficients and the directional blocks at q,
    /// kept for the gate's fallback (R7 §4.1.4 step 3: Ā^q).
    pub(crate) bounded_q: Vec<BoundedCoefficients<R>>,
    pub(crate) directional_q: Vec<DirectionalBlock<R>>,
    pub(crate) factor: RetainedFactor<L>,
    /// est_c per block from the condition screen's solves (R7 §4.1.6.3 item
    /// 7c; formed at p ≥ 256, where the state can serve as a verification).
    pub(crate) est_blocks: Vec<Wide<L>>,
    pub(crate) rcond: f64,
    pub(crate) pivot_margin_min: f64,
    pub(crate) work: AttemptWork,
    pub(crate) sum_work: SumWork,
    pub(crate) stages: StageWork,
    pub(crate) total: WorkTotal,
}

/// A case's solve at one precision.
#[derive(Debug)]
pub(crate) struct Solved<const L: usize>
where
    Wide<L>: SupportedWidth,
{
    pub(crate) p: u32,
    /// u at every DOF.
    pub(crate) u: Vec<Wide<L>>,
    pub(crate) recovered: Recovered<L>,
    pub(crate) corrections: u8,
    pub(crate) residual_worst: f64,
    /// Which test passed the gate (R7 §4.1.4 step 3).
    pub(crate) gate: GateTest,
}

/// The test that passed the residual gate (D1 revision 5a.3, R7 §4.1.4 step 3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateTest {
    /// Every row passed with the coalesced denominator d^c.
    Coalesced,
    /// Refinement ended without the coalesced pass; the best evaluated state
    /// (`state`, 0 = the first solve, of `evaluated`) passed with d^b.
    Bounded { state: u8, evaluated: u8 },
}

/// A failed build or solve, with the work it spent.
#[derive(Debug)]
struct Spent<T> {
    result: Result<T, AttemptStop>,
    work: AttemptWork,
    sum_work: SumWork,
    stages: StageWork,
    total: WorkTotal,
}

fn build_shared<const L: usize, const R: usize>(
    p: u32,
    q: u32,
    source: &PrimitiveSource,
    group: &GroupPrep,
    guard: StageGuard,
) -> Spent<Shared<L, R>>
where
    Wide<L>: SupportedWidth,
    Wide<R>: SupportedWidth,
{
    let mut ctx = WideContext::<L>::new(p).expect("supported precision");
    let mut ctx_q = WideContext::<R>::new(q).expect("supported precision");
    let mut ctx16 = WideContext::<16>::new(1024).expect("supported precision");
    let mut ctx64 = WideContext::<4>::new(64).expect("supported precision");
    let mut sum = ExactWideSum::new();
    let mut stages = StageWork::default();
    let stream = WorkStream::new();
    // T3 KF3: the stage in progress, for a stopped build's unstaged work.
    let mut current = Stage::Formation;
    let mut run = || -> Result<Shared<L, R>, AttemptStop> {
        let t0 = stream.snapshot(
            lme(&ctx) + lme(&ctx_q) + lme(&ctx16) + lme(&ctx64) + sum.work().checked_lme(),
        )?;
        let g = guard.with_base(WorkTotal::zero())?;
        let members = form_members(&mut ctx, &mut sum, &g, source)?;
        let directional = form_directional(&mut ctx, &mut sum, source)?;
        let t1 = stream.snapshot(
            lme(&ctx) + lme(&ctx_q) + lme(&ctx16) + lme(&ctx64) + sum.work().checked_lme(),
        )?;
        stages.set(Stage::Formation, t1.delta_since(t0))?;
        current = Stage::Assembly;
        let k = assemble(
            &mut ctx,
            &mut sum,
            &g,
            source,
            &group.structure,
            &members,
            &directional,
        )?;
        let t2 = stream.snapshot(
            lme(&ctx) + lme(&ctx_q) + lme(&ctx16) + lme(&ctx64) + sum.work().checked_lme(),
        )?;
        stages.set(Stage::Assembly, t2.delta_since(t1))?;
        g.check(&ctx, &sum)?;
        current = Stage::ResidualFormation;
        // The residual system re-formed at q (the ceiling: K itself, widened),
        // with the coefficients the gate's fallback forms Ā^q from.
        type AtQ<const R: usize> = (
            Vec<Wide<R>>,
            Vec<BoundedCoefficients<R>>,
            Vec<DirectionalBlock<R>>,
        );
        let (k_q, bounded_q, directional_q): AtQ<R> = if q == p {
            (
                k.iter().map(|w| w.widen::<R>()).collect(),
                members.iter().map(|m| m.bounded().widen::<R>()).collect(),
                directional.iter().map(|d| d.widen::<R>()).collect(),
            )
        } else {
            let gq = guard.with_base(lme(&ctx))?;
            let members_q = form_members(&mut ctx_q, &mut sum, &gq, source)?;
            let directional_q = form_directional(&mut ctx_q, &mut sum, source)?;
            let k_q = assemble(
                &mut ctx_q,
                &mut sum,
                &gq,
                source,
                &group.structure,
                &members_q,
                &directional_q,
            )?;
            (
                k_q,
                members_q.iter().map(|m| m.bounded()).collect(),
                directional_q,
            )
        };
        let t3 = stream.snapshot(
            lme(&ctx) + lme(&ctx_q) + lme(&ctx16) + lme(&ctx64) + sum.work().checked_lme(),
        )?;
        stages.set(Stage::ResidualFormation, t3.delta_since(t2))?;
        current = Stage::Factor;
        let gp = guard.with_base(lme(&ctx_q))?;
        let factor = super::factor::factor(
            &mut ctx,
            &mut sum,
            &gp,
            &group.structure,
            &k,
            &group.ordering,
        )?;
        let t4 = stream.snapshot(
            lme(&ctx) + lme(&ctx_q) + lme(&ctx16) + lme(&ctx64) + sum.work().checked_lme(),
        )?;
        stages.set(Stage::Factor, t4.delta_since(t3))?;
        current = Stage::Condition;
        // At p ≥ 256 the screen's solves also give est_c per block (read only).
        let (rcond, est_blocks) = if p >= 256 {
            let mut ratios = BlockRatios::new(&group.blocks);
            let rcond = factor.condition_observed(
                &mut ctx,
                &mut sum,
                &group.structure,
                &k,
                &group.ordering,
                Some(&mut ratios),
            )?;
            (rcond, ratios.estimates())
        } else {
            let rcond =
                factor.condition(&mut ctx, &mut sum, &group.structure, &k, &group.ordering)?;
            (rcond, Vec::new())
        };
        gp.check(&ctx, &sum)?;
        let rcond_value = publish_value(&rcond).value().unwrap_or(0.0);
        // The pivot margin minimum, rounded downward.
        let mut tracker = BoundedExtremeTracker::new(Direction::Down);
        for screen in &factor.screens {
            let mut num = ExactWideSum::new();
            num.add_wide_scaled(&screen.pivot, false, 1, i64::from(p))?;
            num.add_wide_scaled(&screen.pivot, true, screen.operations, 0)?;
            let mut den = ExactWideSum::new();
            den.add_wide_scaled(
                &screen.scale,
                false,
                screen
                    .operations
                    .checked_mul(64)
                    .ok_or(AttemptStop::CountRange("pivot multiplier"))?,
                0,
            )?;
            if den.is_zero()? {
                continue;
            }
            tracker.offer(&mut ctx64, &mut ctx16, num, den)?;
        }
        let margin = tracker.finish(&mut ctx16)?.unwrap_or(f64::INFINITY);
        let t5 = stream.snapshot(
            lme(&ctx) + lme(&ctx_q) + lme(&ctx16) + lme(&ctx64) + sum.work().checked_lme(),
        )?;
        stages.set(Stage::Condition, t5.delta_since(t4))?;
        Ok(Shared {
            p,
            q,
            members,
            directional,
            k,
            k_q,
            bounded_q,
            directional_q,
            factor,
            est_blocks,
            rcond: rcond_value,
            pivot_margin_min: margin,
            work: AttemptWork::default(),
            sum_work: SumWork::default(),
            stages: StageWork::default(),
            total: WorkTotal::zero(),
        })
    };
    let mut result = run();
    let mut work = AttemptWork::default();
    work.record(&ctx);
    work.record(&ctx_q);
    work.record(&ctx16);
    work.record(&ctx64);
    let sum_work = sum.work();
    let total = work.checked_lme() + sum_work.checked_lme();
    stages.close_stopped(&result, current, total);
    let total = finish_work(&mut result, total, &stages);
    let result = result.map(|mut shared| {
        shared.work = work;
        shared.sum_work = sum_work;
        shared.stages = stages.clone();
        shared.total = total;
        shared
    });
    Spent {
        result,
        work,
        sum_work,
        stages,
        total,
    }
}

/// The residual gate of the free rows, exactly: per row, whether
/// |r|·(2^p − m) ≤ 64·m·d, the approximate ratio, and r rounded once to p.
#[allow(clippy::too_many_arguments)]
fn residual_rows<const L: usize, const R: usize>(
    ctx: &mut WideContext<L>,
    ctx_q: &mut WideContext<R>,
    ctx64: &mut WideContext<4>,
    ctx16: &mut WideContext<16>,
    sum: &mut ExactWideSum,
    p: u32,
    structure: &Structure,
    k_q: &[Wide<R>],
    ledger: &RetainedLedger,
    free: &[usize],
    u: &[Wide<L>],
    tracker: &mut BoundedExtremeTracker,
) -> Result<Vec<(bool, f64, Wide<L>)>, AttemptStop>
where
    Wide<L>: SupportedWidth,
    Wide<R>: SupportedWidth,
{
    let mut rows = Vec::with_capacity(free.len());
    for &i in free {
        // r_i = f_i − Σ_j K^q_ij·u_j (exact); d_i = |f_i| + Σ_j |K^q_ij·u_j|.
        let mut r = ExactWideSum::new();
        let mut d = ExactWideSum::new();
        ledger.add_to(i, &mut r, false)?;
        if let Some(net) = ledger.net(i) {
            if !net.is_zero() {
                d.add_integer(false, &net.magnitude, net.exponent)?;
            }
        }
        let mut count = 0u64;
        for index in structure.pattern.row_range(i) {
            let j = structure.pattern.column(index);
            let kij = &k_q[index];
            let uj = u[j].widen::<R>();
            if kij.is_zero() || uj.is_zero() {
                continue;
            }
            count = count
                .checked_add(1)
                .ok_or(AttemptStop::CountRange("residual row"))?;
            r.add_product(ctx_q, kij, &uj, true)?;
            let negative = kij.is_sign_negative() != uj.is_sign_negative();
            d.add_product(ctx_q, kij, &uj, negative)?;
        }
        let m = count
            .checked_mul(2)
            .and_then(|v| v.checked_add(2))
            .ok_or(AttemptStop::CountRange("residual operations"))?;
        let mut absolute = r.clone();
        absolute.make_absolute()?;
        // num = |r|·(2^p − m), den = 64·m·d.
        let mut num = ExactWideSum::new();
        num.add_scaled(&absolute, false, 1, i64::from(p))?;
        num.add_scaled(&absolute, true, m, 0)?;
        let mut den = ExactWideSum::new();
        den.add_scaled(
            &d,
            false,
            m.checked_mul(64)
                .ok_or(AttemptStop::CountRange("residual multiplier"))?,
            0,
        )?;
        // Gate: den − num ≥ 0.
        sum.clear();
        sum.add_scaled(&den, false, 1, 0)?;
        sum.add_scaled(&num, true, 1, 0)?;
        let passes = sum.signum()? >= 0;
        let ratio = approximate_ratio(ctx64, &num, &den)?;
        let rounded = r.round(ctx)?;
        tracker.offer(ctx64, ctx16, num, den)?;
        rows.push((passes, ratio, rounded));
    }
    Ok(rows)
}

/// num/den against another exact ratio (dens positive), decided exactly.
fn cmp_ratio(
    a: &(ExactWideSum, ExactWideSum),
    b: &(ExactWideSum, ExactWideSum),
) -> Result<CmpOrdering, AttemptStop> {
    let mut t = ExactWideSum::new();
    let mut db = b.1.clone();
    t.add_product_of(&a.0, &mut db, false)?;
    let mut da = a.1.clone();
    t.add_product_of(&b.0, &mut da, true)?;
    Ok(t.signum()?.cmp(&0))
}

/// An exact ratio num/den (den > 0).
pub(crate) type GateRatio = (ExactWideSum, ExactWideSum);

/// The gate's bounded row test (R7 §4.1.4 step 3), exactly: with
/// num = |r|·(2^p − m) and den = 64·m·d^b, whether num ≤ den, and the pair.
pub(crate) fn bounded_row(
    r: ExactWideSum,
    d: &ExactWideSum,
    m: u64,
    p: u32,
    sum: &mut ExactWideSum,
) -> Result<(bool, GateRatio), AttemptStop> {
    let mut absolute = r;
    absolute.make_absolute()?;
    let mut num = ExactWideSum::new();
    num.add_scaled(&absolute, false, 1, i64::from(p))?;
    num.add_scaled(&absolute, true, m, 0)?;
    let mut den = ExactWideSum::new();
    den.add_scaled(
        d,
        false,
        m.checked_mul(64)
            .ok_or(AttemptStop::CountRange("residual multiplier"))?,
        0,
    )?;
    sum.clear();
    sum.add_scaled(&den, false, 1, 0)?;
    sum.add_scaled(&num, true, 1, 0)?;
    Ok((sum.signum()? >= 0, (num, den)))
}

/// The best evaluated state (ROOT's A3-0 ruling Q6): per state, None when it
/// is not eligible, Some(None) when every row's residual is zero, or
/// Some(Some(worst bounded ratio)). The smallest worst ratio wins, compared
/// exactly, and the earliest state on a tie.
pub(crate) fn best_gate_state(
    worst: &[Option<Option<GateRatio>>],
) -> Result<Option<usize>, AttemptStop> {
    let mut best: Option<(usize, &Option<GateRatio>)> = None;
    for (k, state) in worst.iter().enumerate() {
        let Some(w) = state else { continue };
        let replace = match &best {
            None => true,
            Some((_, b)) => match (w, b) {
                (None, None) | (Some(_), None) => false,
                (None, Some(_)) => true,
                (Some(w), Some(b)) => cmp_ratio(w, b)? == CmpOrdering::Less,
            },
        };
        if replace {
            best = Some((k, w));
        }
    }
    Ok(best.map(|b| b.0))
}

/// The gate's fallback (D1 revision 5a.3, R7 §4.1.4 step 3; ROOT's A3-0
/// rulings Q6 and Q17): Ā^q formed once at q from the coefficients kept at q;
/// per evaluated state, the exact rows with the bounded denominator
/// d_i^b = |f_i| + Σ_j Ā^q_ij·|u_j| and m_i as the coalesced test's. A state
/// with a nonzero residual on a row with d^b = 0 is not eligible. The best
/// state (`best_gate_state`) passes when every row passes
/// |r_i|(2^p − m_i) ≤ 64·m_i·d_i^b (`bounded_row`).
/// Returns (its index, fl↑ of its worst bounded ratio), or None.
#[allow(clippy::too_many_arguments)]
fn bounded_fallback<const L: usize, const R: usize>(
    ctx_q: &mut WideContext<R>,
    ctx64: &mut WideContext<4>,
    ctx16: &mut WideContext<16>,
    sum: &mut ExactWideSum,
    p: u32,
    shared: &Shared<L, R>,
    prep: &CasePrep,
    group: &GroupPrep,
    u_base: &[Wide<L>],
    evaluated: &[Vec<Wide<L>>],
    guard: &StageGuard,
) -> Result<Option<(usize, f64)>, AttemptStop>
where
    Wide<L>: SupportedWidth,
    Wide<R>: SupportedWidth,
{
    let structure = &group.structure;
    let free = &group.ordering.free;
    let abar_q = assemble_bounded(
        ctx_q,
        sum,
        guard,
        &prep.source,
        structure,
        &shared.bounded_q,
        &shared.directional_q,
    )?;
    let mut worsts: Vec<Option<Option<GateRatio>>> = Vec::with_capacity(evaluated.len());
    let mut states: Vec<Option<(bool, BoundedExtremeTracker)>> =
        Vec::with_capacity(evaluated.len());
    for u_free in evaluated {
        let mut u = u_base.to_vec();
        for (a, &g) in free.iter().enumerate() {
            u[g] = u_free[a];
        }
        let mut eligible = true;
        let mut all_pass = true;
        let mut tracker = BoundedExtremeTracker::new(Direction::Up);
        let mut rows: Vec<(ExactWideSum, ExactWideSum, f64)> = Vec::new();
        for &i in free {
            let mut r = ExactWideSum::new();
            let mut d = ExactWideSum::new();
            prep.ledger.add_to(i, &mut r, false)?;
            if let Some(net) = prep.ledger.net(i) {
                if !net.is_zero() {
                    d.add_integer(false, &net.magnitude, net.exponent)?;
                }
            }
            let mut count = 0u64;
            for index in structure.pattern.row_range(i) {
                let j = structure.pattern.column(index);
                let uj = u[j].widen::<R>();
                if uj.is_zero() {
                    continue;
                }
                let kij = &shared.k_q[index];
                if !kij.is_zero() {
                    count = count
                        .checked_add(1)
                        .ok_or(AttemptStop::CountRange("residual row"))?;
                    r.add_product(ctx_q, kij, &uj, true)?;
                }
                let aij = &abar_q[index];
                if !aij.is_zero() {
                    d.add_product(ctx_q, aij, &uj.abs(), false)?;
                }
            }
            let m = count
                .checked_mul(2)
                .and_then(|v| v.checked_add(2))
                .ok_or(AttemptStop::CountRange("residual operations"))?;
            let (passes, (mut num, mut den)) = bounded_row(r, &d, m, p, sum)?;
            all_pass &= passes;
            if num.is_zero()? {
                continue;
            }
            if den.is_zero()? {
                eligible = false;
                break;
            }
            let approx = approximate_ratio(ctx64, &num, &den)?;
            tracker.offer(ctx64, ctx16, num.clone(), den.clone())?;
            rows.push((num, den, approx));
        }
        if !eligible {
            worsts.push(None);
            states.push(None);
            continue;
        }
        // The exact worst row, among those whose 64-bit approximation lies
        // within a relative 2^-40 of the largest (the approximations are within
        // a few 2^-64 of the exact ratios).
        let top = rows.iter().map(|r| r.2).fold(0.0f64, f64::max);
        let floor = top - top * f64::from_bits(0x3D70_0000_0000_0000); // 2^-40
        let mut worst: Option<GateRatio> = None;
        for (num, den, approx) in rows {
            if approx < floor {
                continue;
            }
            let candidate = (num, den);
            let greater = match &worst {
                None => true,
                Some(w) => cmp_ratio(&candidate, w)? == CmpOrdering::Greater,
            };
            if greater {
                worst = Some(candidate);
            }
        }
        worsts.push(Some(worst));
        states.push(Some((all_pass, tracker)));
    }
    let Some(k) = best_gate_state(&worsts)? else {
        return Ok(None);
    };
    match states.swap_remove(k) {
        Some((true, tracker)) => Ok(Some((k, tracker.finish(ctx16)?.unwrap_or(0.0)))),
        _ => Ok(None),
    }
}

#[allow(clippy::too_many_arguments)]
fn solve_case_at<const L: usize, const R: usize>(
    shared: &Shared<L, R>,
    prep: &CasePrep,
    group: &GroupPrep,
    guard: StageGuard,
) -> Spent<Solved<L>>
where
    Wide<L>: SupportedWidth,
    Wide<R>: SupportedWidth,
{
    let p = shared.p;
    let mut ctx = WideContext::<L>::new(p).expect("supported precision");
    let mut ctx_q = WideContext::<R>::new(shared.q).expect("supported precision");
    let mut ctx16 = WideContext::<16>::new(1024).expect("supported precision");
    let mut ctx64 = WideContext::<4>::new(64).expect("supported precision");
    let mut sum = ExactWideSum::new();
    let mut stages = StageWork::default();
    let stream = WorkStream::new();
    let source = &prep.source;
    let free = &group.ordering.free;
    // T3 KF3: the stage in progress, for a stopped solve's unstaged work.
    let mut current = Stage::Rhs;
    let mut run = || -> Result<Solved<L>, AttemptStop> {
        let total = |ctx: &WideContext<L>,
                     ctx_q: &WideContext<R>,
                     ctx16: &WideContext<16>,
                     ctx64: &WideContext<4>,
                     sum: &ExactWideSum| {
            lme(ctx) + lme(ctx_q) + lme(ctx16) + lme(ctx64) + sum.work().checked_lme()
        };
        let t0 = stream.snapshot(total(&ctx, &ctx_q, &ctx16, &ctx64, &sum))?;
        // The prescribed values at p (exact for a case's binary64 values; a
        // combination's exact sum, rounded once).
        let mut u = vec![Wide::<L>::ZERO; source.dof_count()];
        prep.prescribed_at(&mut ctx, &mut sum, &mut u)?;
        let rhs = reduced_rhs(
            &mut ctx,
            &mut sum,
            source,
            &group.structure,
            &shared.k,
            &prep.ledger,
            free,
            &u,
        )?;
        let t1 = stream.snapshot(total(&ctx, &ctx_q, &ctx16, &ctx64, &sum))?;
        stages.set(Stage::Rhs, t1.delta_since(t0))?;
        current = Stage::Solve;
        let mut u_free = shared.factor.solve(&mut ctx, &rhs)?;
        guard.test(total(&ctx, &ctx_q, &ctx16, &ctx64, &sum))?;
        let t2 = stream.snapshot(total(&ctx, &ctx_q, &ctx16, &ctx64, &sum))?;
        stages.set(Stage::Solve, t2.delta_since(t1))?;
        current = Stage::Refinement;
        let mut corrections = 0u8;
        let mut prior = f64::INFINITY;
        let residual_worst;
        let mut gate = GateTest::Coalesced;
        let mut evaluated: Vec<Vec<Wide<L>>> = Vec::new();
        let mut fallback_work = WorkTotal::zero();
        loop {
            for (a, &g) in free.iter().enumerate() {
                u[g] = u_free[a];
            }
            evaluated.push(u_free.clone());
            let mut tracker = BoundedExtremeTracker::new(Direction::Up);
            let rows = residual_rows(
                &mut ctx,
                &mut ctx_q,
                &mut ctx64,
                &mut ctx16,
                &mut sum,
                p,
                &group.structure,
                &shared.k_q,
                &prep.ledger,
                free,
                &u,
                &mut tracker,
            )?;
            guard.test(total(&ctx, &ctx_q, &ctx16, &ctx64, &sum))?;
            let mut worst = 0.0f64;
            let mut worst_row = 0;
            for (a, row) in rows.iter().enumerate() {
                if row.1 > worst || (a == 0 && row.1 >= worst) {
                    worst = row.1;
                    worst_row = a;
                }
            }
            if rows.iter().all(|row| row.0) {
                residual_worst = tracker.finish(&mut ctx16)?.unwrap_or(0.0);
                break;
            }
            if corrections == 3 || worst >= prior {
                // D1 revision 5a.3 (R7 §4.1.4 step 3): the bounded test on the
                // best evaluated state (ROOT's A3-0 rulings Q6, Q17).
                let tf = stream.snapshot(total(&ctx, &ctx_q, &ctx16, &ctx64, &sum))?;
                // The refinement up to the fallback, then the fallback's own
                // stage (the same values as at t3).
                stages.set(Stage::Refinement, tf.delta_since(t2))?;
                current = Stage::BoundedGate;
                let chosen = bounded_fallback(
                    &mut ctx_q, &mut ctx64, &mut ctx16, &mut sum, p, shared, prep, group, &u,
                    &evaluated, &guard,
                )?;
                guard.test(total(&ctx, &ctx_q, &ctx16, &ctx64, &sum))?;
                fallback_work = stream
                    .snapshot(total(&ctx, &ctx_q, &ctx16, &ctx64, &sum))?
                    .delta_since(tf);
                stages.set(Stage::BoundedGate, fallback_work)?;
                current = Stage::Refinement;
                match chosen {
                    Some((k, ratio)) => {
                        u_free = evaluated[k].clone();
                        for (a, &g) in free.iter().enumerate() {
                            u[g] = u_free[a];
                        }
                        residual_worst = ratio;
                        gate = GateTest::Bounded {
                            state: k as u8,
                            evaluated: evaluated.len() as u8,
                        };
                        break;
                    }
                    None => {
                        return Err(AttemptStop::ResidualGate {
                            global_dof: free[worst_row],
                        })
                    }
                }
            }
            prior = worst;
            let correction: Vec<Wide<L>> = rows.iter().map(|row| row.2).collect();
            let delta = shared.factor.solve(&mut ctx, &correction)?;
            for (v, d) in u_free.iter_mut().zip(&delta) {
                *v = ctx.add(v, d)?;
            }
            corrections += 1;
        }
        let t3 = stream.snapshot(total(&ctx, &ctx_q, &ctx16, &ctx64, &sum))?;
        stages.set(Stage::Refinement, t3.delta_since(t2) - fallback_work)?;
        stages.set(Stage::BoundedGate, fallback_work)?;
        current = Stage::Recovery;
        // A test-only seed of the final state (R7 §7's SEEDED controls), after
        // the gate and before recovery.
        #[cfg(test)]
        seed::apply(&mut ctx, &mut u)?;
        let base = t3.total() - lme(&ctx) - sum.work().checked_lme();
        let recovered = recover(
            &mut ctx,
            &mut sum,
            &guard.with_base(guard.base + base)?,
            source,
            &prep.layout,
            &group.structure,
            &shared.k,
            &shared.members,
            &shared.directional,
            &prep.ledger,
            &u,
        )?;
        let t4 = stream.snapshot(total(&ctx, &ctx_q, &ctx16, &ctx64, &sum))?;
        stages.set(Stage::Recovery, t4.delta_since(t3))?;
        Ok(Solved {
            p,
            u,
            recovered,
            corrections,
            residual_worst,
            gate,
        })
    };
    let mut result = run();
    let mut work = AttemptWork::default();
    work.record(&ctx);
    work.record(&ctx_q);
    work.record(&ctx16);
    work.record(&ctx64);
    let sum_work = sum.work();
    let total = work.checked_lme() + sum_work.checked_lme();
    stages.close_stopped(&result, current, total);
    let total = finish_work(&mut result, total, &stages);
    Spent {
        result,
        work,
        sum_work,
        stages,
        total,
    }
}

// ------------------------------------------------------------ the stop rule

/// The outcome of one stop-rule comparison.
#[derive(Debug, Clone)]
pub(crate) struct StopDecision {
    /// Ok(accepted) or the stop that ended the comparison (budget, span).
    pub(crate) result: Result<bool, AttemptStop>,
    /// The first quantity (layout index) that disagrees, if any.
    #[allow(dead_code)] // read by the stop rule's tests (`rejection` carries it)
    pub(crate) first_failure: Option<usize>,
    /// D1 revision 5a.3: the test that rejected p (R7's order, ROOT's Q7).
    pub(crate) rejection: Option<Rejection>,
    /// Per (body, kind): the worst normalized disagreement (|Δ| + V)/M,
    /// rounded upward (computed for an accepted comparison only).
    pub(crate) summary: Vec<(u32, Kind, f64)>,
    /// Per (body, kind) of force and moment: the worst Ŵ/V and C/allowance,
    /// rounded upward (5a.3; accepted comparisons only).
    pub(crate) estimate_summary: Vec<(u32, Kind, f64)>,
    pub(crate) charge_summary: Vec<(u32, Kind, f64)>,
    /// Φ = fl↑(2^-438·ê) per body [fo, mo] at p = 512 (item 6a).
    pub(crate) floor: Option<Vec<[f64; 2]>>,
    pub(crate) work: AttemptWork,
    pub(crate) sum_work: SumWork,
    pub(crate) total: WorkTotal,
}

/// The test that rejected a candidate (D1 revision 5a.3, R7 §5.1 (a)–(d),
/// §4.1.6.3 item 13).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Rejection {
    /// (a): the layout index of the first failing quantity.
    StopRule { index: usize },
    /// (b): Ŵ_q > 2^(6−2p)·ê.
    VerificationEstimate { index: usize },
    /// (c): a block with data without a certified bound.
    Uc { block: usize },
    /// (c): θ_c > 1/2.
    Theta { block: usize },
    /// (c): a member in scope with g > 2^(P−16) (its id).
    GValidity { member: u32 },
    /// (d): C_q above its allowance.
    Charge { index: usize },
}

/// S\* per body and kind at the verification precision (item 6's coupling,
/// rounded once at 2p), from the rows that are not input-derived and that the
/// candidate can publish: a row whose candidate value has no binary64 value
/// (`skip`) is left out, as the classification leaves it out of S\*_pub (O9;
/// ROOT's ruling on RV19-1). So both scales are formed from the same rows.
fn scales_at<const M: usize>(
    ctx: &mut WideContext<M>,
    layout: &[QuantityMeta],
    values: &[Wide<M>],
    extents: &[f64],
    skip: &[bool],
) -> Result<Vec<[Wide<M>; 4]>, AttemptStop>
where
    Wide<M>: SupportedWidth,
{
    let zero = Wide::<M>::ZERO;
    let mut s = vec![[zero; 4]; extents.len()];
    for ((meta, v), &skipped) in layout.iter().zip(values).zip(skip) {
        if meta.input_derived || skipped {
            continue;
        }
        let slot = &mut s[meta.body as usize][meta.kind.index()];
        if v.abs().cmp_value(slot) == CmpOrdering::Greater {
            *slot = v.abs();
        }
    }
    let max = |a: Wide<M>, b: Wide<M>| {
        if b.cmp_value(&a) == CmpOrdering::Greater {
            b
        } else {
            a
        }
    };
    let mut out = Vec::with_capacity(extents.len());
    for (body, &extent) in extents.iter().enumerate() {
        let [tr, ro, fo, mo] = s[body];
        if extent == 0.0 {
            out.push([tr, ro, fo, mo]);
            continue;
        }
        let lb = Wide::<M>::from_f64(extent)?;
        out.push([
            max(tr, ctx.mul(&lb, &ro)?),
            max(ro, ctx.div(&tr, &lb)?),
            max(fo, ctx.div(&mo, &lb)?),
            max(mo, ctx.mul(&lb, &fo)?),
        ]);
    }
    Ok(out)
}

/// "|q_p − q_2p| ≤ 2^-64 · max(|q_2p|, S*)" on every published quantity,
/// decided exactly (the summary ratios rounded upward when accepted). Charged
/// to the candidate attempt, with budget checks. Revision 5a.2's rule, with no
/// resolution term: `decide` is revision 5a.3's.
#[allow(dead_code)] // test entry: the rule (a) predicate's tests (SD-G5, O6)
pub(crate) fn stop_rule<const L: usize, const M: usize>(
    layout: &[QuantityMeta],
    extents: &[f64],
    candidate: &[Wide<L>],
    verification: &[Wide<M>],
    verification_precision: u32,
    guard: StageGuard,
) -> StopDecision
where
    Wide<L>: SupportedWidth,
    Wide<M>: SupportedWidth,
{
    rule(
        layout,
        extents,
        candidate,
        verification,
        verification_precision,
        None,
        guard,
    )
}

/// D1 revision 5a.3's acceptance (R7 §5.1): (a) with V_q — ê·2^(8−2p) for
/// force and moment rows, W⁺_q for the other rows that are not input-derived
/// (plus 2^(1−2p)·|q_2p| for a magnitude) — and, at p = 512, S\* floored by
/// Φ = fl↑(2^-438·ê) for force and moment; then (b) the verification estimate
/// Ŵ_q ≤ 2^(6−2p)·ê; (c) `uc`, θ_c ≤ 1/2 and the g check; (d) the charge,
/// C_q ≤ 60·2^-2p·ê at p = 128 and 256 and C_q ≤ 2^-86·M_q at p = 512. The
/// first failure in that order (emu7's; ROOT's A3-0 ruling Q7) rejects p.
pub(crate) fn decide<const L: usize, const M: usize>(
    layout: &[QuantityMeta],
    extents: &[f64],
    candidate: &[Wide<L>],
    verification: &[Wide<M>],
    report: &VerificationReport<M>,
    guard: StageGuard,
) -> StopDecision
where
    Wide<L>: SupportedWidth,
    Wide<M>: SupportedWidth,
{
    rule(
        layout,
        extents,
        candidate,
        verification,
        report.precision,
        Some(report),
        guard,
    )
}

/// The tests of `rule` that keep a summary tracker, in R7's order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum RuleTest {
    /// (a): (|Δ| + V)/M.
    Disagreement,
    /// (b): Ŵ/V.
    Estimate,
    /// (d): C over its allowance.
    Charge,
}

#[allow(clippy::too_many_lines)]
fn rule<const L: usize, const M: usize>(
    layout: &[QuantityMeta],
    extents: &[f64],
    candidate: &[Wide<L>],
    verification: &[Wide<M>],
    verification_precision: u32,
    report: Option<&VerificationReport<M>>,
    guard: StageGuard,
) -> StopDecision
where
    Wide<L>: SupportedWidth,
    Wide<M>: SupportedWidth,
{
    let mut ctx = WideContext::<M>::new(verification_precision).expect("supported precision");
    let mut ctx16 = WideContext::<16>::new(1024).expect("supported precision");
    let mut ctx64 = WideContext::<4>::new(64).expect("supported precision");
    let mut sum = ExactWideSum::new();
    let mut first_failure = None;
    let mut rejection = None;
    let mut summary = Vec::new();
    let mut estimate_summary = Vec::new();
    let mut charge_summary = Vec::new();
    let mut floor = None;
    let big_p = i64::from(verification_precision);
    let mut run = || -> Result<bool, AttemptStop> {
        let spent = |ctx: &WideContext<M>,
                     ctx16: &WideContext<16>,
                     ctx64: &WideContext<4>,
                     sum: &ExactWideSum| {
            lme(ctx) + lme(ctx16) + lme(ctx64) + sum.work().checked_lme()
        };
        // O9 on the stop rule (ROOT's ruling on RV19-1): a row the candidate
        // cannot publish (a nonzero value that underflows binary64, or one
        // that overflows) sets no S\*. Leaving rows out only lowers S\*, so
        // every allowance of (a) and (d) can only shrink: availability, never
        // honesty. The skipped rows themselves still meet (a).
        let skip: Vec<bool> = candidate
            .iter()
            .map(|q| !q.is_zero() && q.to_binary64().value().is_none())
            .collect();
        let mut scales = scales_at(&mut ctx, layout, verification, extents, &skip)?;
        // ê per body (item 6a), and the floor at p = 512 only.
        let hats: Vec<[f64; 2]> = match report {
            Some(r) => r
                .resolution
                .iter()
                .zip(extents)
                .map(|(e, &x)| e_hat(*e, x))
                .collect(),
            None => Vec::new(),
        };
        if report.is_some() && verification_precision == 1024 {
            let mut phis = Vec::with_capacity(hats.len());
            for (b, h) in hats.iter().enumerate() {
                let phi = [phi_512(h[0]), phi_512(h[1])];
                for (slot, value) in [(Kind::Force, phi[0]), (Kind::Moment, phi[1])] {
                    let v = Wide::<M>::from_f64(value)?;
                    let s = &mut scales[b][slot.index()];
                    if v.cmp_value(s) == CmpOrdering::Greater {
                        *s = v;
                    }
                }
                phis.push(phi);
            }
            floor = Some(phis);
        }
        let hat = |body: u32, kind: Kind| -> f64 {
            match kind {
                Kind::Force => hats[body as usize][0],
                _ => hats[body as usize][1],
            }
        };
        let magnitude_of = |index: usize, meta: &QuantityMeta| -> Wide<M> {
            let q2 = &verification[index];
            let s_star = &scales[meta.body as usize][meta.kind.index()];
            if q2.abs().cmp_value(s_star) == CmpOrdering::Greater {
                q2.abs()
            } else {
                *s_star
            }
        };
        // (a). One set holds the trackers of (a), (b) and (d), by (test, body,
        // kind), so G bounds them together (KF1).
        let mut trackers: TrackerSet<(RuleTest, u32, Kind)> = TrackerSet::new();
        for (index, meta) in layout.iter().enumerate() {
            let q2 = &verification[index];
            let magnitude = magnitude_of(index, meta);
            let mut difference = ExactWideSum::new();
            difference.add_wide(&candidate[index], false)?;
            difference.add_wide(q2, true)?;
            difference.make_absolute()?;
            // |Δ| + V_q.
            if let Some(r) = report {
                match meta.kind {
                    Kind::Force | Kind::Moment => {
                        let e = Wide::<M>::from_f64(hat(meta.body, meta.kind))?;
                        difference.add_wide_scaled(&e, false, 1, 8 - big_p)?;
                    }
                    _ if !meta.input_derived => {
                        if let Some(wp) = &r.w_plus[index] {
                            difference.add_wide(wp, false)?;
                        }
                        if matches!(meta.id, QuantityId::DisplacementMagnitude(_)) {
                            difference.add_wide_scaled(&q2.abs(), false, 1, 1 - big_p)?;
                        }
                    }
                    _ => {}
                }
            }
            sum.clear();
            sum.add_wide_scaled(&magnitude, false, 1, -64)?;
            sum.add_scaled(&difference, true, 1, 0)?;
            if sum.signum()? < 0 {
                first_failure = Some(index);
                rejection = Some(Rejection::StopRule { index });
                return Ok(false);
            }
            if !magnitude.is_zero() {
                let mut den = ExactWideSum::new();
                den.add_wide(&magnitude, false)?;
                trackers.offer(
                    (RuleTest::Disagreement, meta.body, meta.kind),
                    Direction::Up,
                    &mut ctx64,
                    &mut ctx16,
                    difference,
                    den,
                )?;
            }
            if index % 64 == 63 {
                guard.test(spent(&ctx, &ctx16, &ctx64, &sum))?;
            }
        }
        let Some(r) = report else {
            for ((_, body, kind), tracker) in trackers.into_trackers() {
                let worst = tracker.finish(&mut ctx16)?.unwrap_or(0.0);
                summary.push((body, kind, worst));
            }
            guard.test(spent(&ctx, &ctx16, &ctx64, &sum))?;
            return Ok(true);
        };
        // (b): Ŵ_q ≤ 2^(6−2p)·ê, with the summary Ŵ/V.
        for (index, meta) in layout.iter().enumerate() {
            let Some(w) = &r.w[index] else { continue };
            let e = Wide::<M>::from_f64(hat(meta.body, meta.kind))?;
            sum.clear();
            sum.add_wide_scaled(&e, false, 1, 6 - big_p)?;
            sum.add_wide(w, true)?;
            if sum.signum()? < 0 {
                rejection = Some(Rejection::VerificationEstimate { index });
                return Ok(false);
            }
            if !e.is_zero() {
                let mut num = ExactWideSum::new();
                num.add_wide(w, false)?;
                let mut den = ExactWideSum::new();
                den.add_wide_scaled(&e, false, 1, 8 - big_p)?;
                trackers.offer(
                    (RuleTest::Estimate, meta.body, meta.kind),
                    Direction::Up,
                    &mut ctx64,
                    &mut ctx16,
                    num,
                    den,
                )?;
            }
        }
        // (c): `uc`, θ, g.
        if let Some(block) = r.uc_missing {
            rejection = Some(Rejection::Uc { block });
            return Ok(false);
        }
        let half = Wide::<M>::ONE.mul_pow2(-1)?;
        for (block, theta) in r.theta.iter().enumerate() {
            if theta.is_some_and(|t| t.cmp_value(&half) == CmpOrdering::Greater) {
                rejection = Some(Rejection::Theta { block });
                return Ok(false);
            }
        }
        if let Some(member) = r.g_violation {
            rejection = Some(Rejection::GValidity { member });
            return Ok(false);
        }
        // (d): the charge.
        for (index, meta) in layout.iter().enumerate() {
            let Some(c) = &r.charge[index] else { continue };
            let mut allowance = ExactWideSum::new();
            if verification_precision < 1024 {
                let e = Wide::<M>::from_f64(hat(meta.body, meta.kind))?;
                allowance.add_wide_scaled(&e, false, 60, -big_p)?;
            } else {
                allowance.add_wide_scaled(&magnitude_of(index, meta), false, 1, -86)?;
            }
            sum.clear();
            sum.add_scaled(&allowance, false, 1, 0)?;
            sum.add_wide(c, true)?;
            if sum.signum()? < 0 {
                rejection = Some(Rejection::Charge { index });
                return Ok(false);
            }
            if !allowance.is_zero()? {
                let mut num = ExactWideSum::new();
                num.add_wide(c, false)?;
                trackers.offer(
                    (RuleTest::Charge, meta.body, meta.kind),
                    Direction::Up,
                    &mut ctx64,
                    &mut ctx16,
                    num,
                    allowance,
                )?;
            }
            if index % 64 == 63 {
                guard.test(spent(&ctx, &ctx16, &ctx64, &sum))?;
            }
        }
        // In (test, body, kind) order: (a)'s, then (b)'s, then (d)'s, each by
        // (body, kind), as K4 finished its three maps.
        for ((test, body, kind), tracker) in trackers.into_trackers() {
            let worst = tracker.finish(&mut ctx16)?.unwrap_or(0.0);
            match test {
                RuleTest::Disagreement => summary.push((body, kind, worst)),
                RuleTest::Estimate => estimate_summary.push((body, kind, worst)),
                RuleTest::Charge => charge_summary.push((body, kind, worst)),
            }
        }
        guard.test(spent(&ctx, &ctx16, &ctx64, &sum))?;
        Ok(true)
    };
    let mut result = run();
    let mut work = AttemptWork::default();
    work.record(&ctx);
    work.record(&ctx16);
    work.record(&ctx64);
    let sum_work = sum.work();
    let total = work.checked_lme() + sum_work.checked_lme();
    let total = finish_work(&mut result, total, &StageWork::default());
    StopDecision {
        result,
        first_failure,
        rejection,
        summary,
        estimate_summary,
        charge_summary,
        floor,
        work,
        sum_work,
        total,
    }
}

// ------------------------------------------------------------ states by precision

/// A case's retained state at one precision (the shared operators it was
/// solved with stay in the group's cache).
#[derive(Debug, Clone)]
pub(crate) enum PrecisionState {
    P128(Arc<Solved<4>>),
    P256(Arc<Solved<4>>),
    P512(Arc<Solved<8>>),
    P1024(Arc<Solved<16>>),
}

impl PrecisionState {
    pub(crate) fn precision(&self) -> u32 {
        match self {
            Self::P128(..) => 128,
            Self::P256(..) => 256,
            Self::P512(..) => 512,
            Self::P1024(..) => 1024,
        }
    }

    /// The published binary64 values (one rounding each).
    pub(crate) fn published(&self) -> Vec<Binary64Outcome> {
        match self {
            Self::P128(s) | Self::P256(s) => s.recovered.values.iter().map(publish_value).collect(),
            Self::P512(s) => s.recovered.values.iter().map(publish_value).collect(),
            Self::P1024(s) => s.recovered.values.iter().map(publish_value).collect(),
        }
    }

    pub(crate) fn encoding(&self) -> Vec<u8> {
        match self {
            Self::P128(s) | Self::P256(s) => state_encoding(s.p, &s.u, &s.recovered.q),
            Self::P512(s) => state_encoding(s.p, &s.u, &s.recovered.q),
            Self::P1024(s) => state_encoding(s.p, &s.u, &s.recovered.q),
        }
    }

    fn corrections(&self) -> u8 {
        match self {
            Self::P128(s) | Self::P256(s) => s.corrections,
            Self::P512(s) => s.corrections,
            Self::P1024(s) => s.corrections,
        }
    }
}

/// A verification state's report (D1 revision 5a.3), per width.
#[derive(Debug, Clone)]
pub(crate) enum VerificationState {
    V256(Arc<VerificationReport<4>>),
    V512(Arc<VerificationReport<8>>),
    V1024(Arc<VerificationReport<16>>),
}

/// The report is valid only for the prep and verification Arc that produced it.
/// These references are transient; no report is retained by RetainedSolve.
struct BoundVerification {
    prep: Arc<CasePrep>,
    state: PrecisionState,
    report: VerificationState,
}

impl PrecisionState {
    fn same_state(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::P128(a), Self::P128(b)) | (Self::P256(a), Self::P256(b)) => Arc::ptr_eq(a, b),
            (Self::P512(a), Self::P512(b)) => Arc::ptr_eq(a, b),
            (Self::P1024(a), Self::P1024(b)) => Arc::ptr_eq(a, b),
            _ => false,
        }
    }
}

/// A verification's report in binary64 (the attempt's evidence).
#[derive(Debug, Clone, PartialEq)]
pub struct VerificationSummary {
    /// [E_fo, E_mo] per body (uncoupled, rounded upward).
    pub resolution: Vec<[f64; 2]>,
    /// Per body: the largest θ_c over its blocks with data, rounded upward (0
    /// without), and B_b rounded upward (None without a block with data).
    pub theta: Vec<f64>,
    pub bound: Vec<Option<f64>>,
    pub data_blocks: usize,
    pub shift_factorizations: u8,
    pub uc_missing: Option<usize>,
    pub g_max: u32,
    pub g_violation: Option<u32>,
}

fn summarize<const L: usize>(r: &VerificationReport<L>) -> Result<VerificationSummary, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    let mut theta = Vec::with_capacity(r.bodies.len());
    let mut bound = Vec::with_capacity(r.bodies.len());
    for body in &r.bodies {
        theta.push(binary64_up(&body.theta)?);
        bound.push(match &body.b {
            Some(b) => Some(binary64_up(b)?),
            None => None,
        });
    }
    Ok(VerificationSummary {
        resolution: r.resolution.clone(),
        theta,
        bound,
        data_blocks: r.blocks.iter().filter(|b| b.data).count(),
        shift_factorizations: r.shift_factorizations,
        uc_missing: r.uc_missing,
        g_max: r.g_max,
        g_violation: r.g_violation,
    })
}

impl VerificationState {
    pub(crate) fn summary(&self) -> Result<VerificationSummary, AttemptStop> {
        match self {
            Self::V256(r) => summarize(r),
            Self::V512(r) => summarize(r),
            Self::V1024(r) => summarize(r),
        }
    }
}

/// D1 revision 5a.3's decision between a candidate and its verification (2p).
fn compare_states(
    layout: &[QuantityMeta],
    extents: &[f64],
    candidate: &PrecisionState,
    verification: &PrecisionState,
    report: &VerificationState,
    guard: StageGuard,
) -> StopDecision {
    match (candidate, verification, report) {
        (PrecisionState::P128(a), PrecisionState::P256(b), VerificationState::V256(r)) => decide(
            layout,
            extents,
            &a.recovered.values,
            &b.recovered.values,
            r,
            guard,
        ),
        (PrecisionState::P256(a), PrecisionState::P512(b), VerificationState::V512(r)) => decide(
            layout,
            extents,
            &a.recovered.values,
            &b.recovered.values,
            r,
            guard,
        ),
        (PrecisionState::P512(a), PrecisionState::P1024(b), VerificationState::V1024(r)) => decide(
            layout,
            extents,
            &a.recovered.values,
            &b.recovered.values,
            r,
            guard,
        ),
        _ => unreachable!("the schedule compares p with 2p"),
    }
}

// ------------------------------------------------------------ attempts and evidence

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttemptRole {
    Candidate,
    Verification,
    VerificationThenCandidate,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttemptReason {
    Stop(AttemptStop),
    StopRule {
        quantity: QuantityId,
        body: u32,
        kind: Kind,
    },
    VerificationFailed,
    /// Exact H did not fit this row's published promise (escalates).
    PublicationEnclosure {
        quantity: QuantityId,
        body: u32,
        kind: Kind,
        predicate: PublicationPredicate,
    },
    /// D1 revision 5a.3 (R7 §4.1.6.3 item 13): `verification_estimate`.
    VerificationEstimate {
        quantity: QuantityId,
        body: u32,
        kind: Kind,
    },
    /// `uc`: a block with data of this body has neither Uc_c nor S_c.
    Uc {
        body: u32,
    },
    /// `theta`: θ_c > 1/2 on a block with data of this body.
    Theta {
        body: u32,
    },
    /// `g_validity`: a member in scope with g > 2^(P−16).
    GValidity {
        member: u32,
    },
    /// `charge`: C_q above its allowance.
    Charge {
        quantity: QuantityId,
        body: u32,
        kind: Kind,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttemptOutcome {
    Accepted,
    Verified,
    Rejected(AttemptReason),
    Failed(AttemptReason),
    /// Solved as a verification that was not needed as a candidate.
    Solved,
}

/// Deterministic storage counts (not measurements).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct StorageCounts {
    pub pattern_entries: usize,
    pub profile_entries: usize,
    pub limbs_per_entry: usize,
}

/// One solve's record (D1 §5 item 1: "the attempts list (p, outcome, reason,
/// work)").
#[derive(Clone, PartialEq)]
pub struct AttemptRecord {
    work_status: WorkStatus,
    pub precision: u32,
    pub role: AttemptRole,
    pub outcome: AttemptOutcome,
    /// p + 64, or p at the ceiling (ROOT's Q4).
    pub residual_basis: u32,
    pub corrections: u8,
    /// fl↓ of the minimum d_i/(64·γ_p(m_i)·c_i).
    pub pivot_margin_min: Option<f64>,
    /// rcond at p (nearest; model information).
    pub rcond: Option<f64>,
    /// fl↑ of the worst |r_i|/(64·γ_p(m_i)·d_i).
    pub residual_worst: Option<f64>,
    /// This case's own contexts (each recorded once).
    pub work: AttemptWork,
    pub k4_work: SumWork,
    pub stages: StageWork,
    /// The shared stages' work at this precision (formation, assembly, residual
    /// formation, factor, condition), counted in full against the case limit.
    pub shared_work: u64,
    pub shared_stages: StageWork,
    /// Whether this attempt built the shared stages (so charged them to the
    /// invocation).
    pub shared_built_here: bool,
    /// The stop-rule work charged to this attempt as a candidate (a part of
    /// `work` and `k4_work`, which hold every context and sum it charged).
    pub stop_rule_work: u64,
    pub storage: StorageCounts,
    /// D1 revision 5a.3: the test that passed the gate.
    pub gate: Option<GateTest>,
    /// The verification pass on this state (a part of `work`, `k4_work` and
    /// `stages`), and its report.
    pub verification_work: u64,
    pub verification: Option<VerificationSummary>,
    /// The verification's shared stages (Ā at P, K_e at q_W, the Uc passes),
    /// counted in full against the case, and against the invocation when
    /// built here.
    pub verification_shared_work: u64,
    pub verification_shared_built_here: bool,
    /// T3 KF3 (D1 revision 5a.3 amendment A2; ROOT's ruling 3 on I19's plan):
    /// on a verification attempt, every block's Uc_c refusal (from the shared
    /// build) and every S_c refusal of this pass, on every path, a build that
    /// stops after a refusal included (RV23-1).
    pub bound_refusals: Vec<BlockRefusal>,
}

impl AttemptRecord {
    pub fn work_status(&self) -> WorkStatus {
        self.work_status
            .join(self.work.checked_lme().status())
            .join(self.k4_work.checked_lme().status())
            .join(self.stages.checked_total().status())
            .join(self.shared_stages.checked_total().status())
    }
    fn latch(&mut self, work: WorkTotal) {
        self.work_status = self.work_status.join(work.status());
    }
    pub fn checked_own_work(&self) -> WorkTotal {
        self.work
            .checked_lme()
            .add(self.k4_work.checked_lme())
            .join_status(self.work_status())
    }
    pub fn checked_shared_work(&self) -> WorkTotal {
        WorkTotal::exact_count(self.shared_work).join_status(self.work_status())
    }
    pub fn checked_verification_shared_work(&self) -> WorkTotal {
        WorkTotal::exact_count(self.verification_shared_work).join_status(self.work_status())
    }
    pub fn checked_verification_work(&self) -> WorkTotal {
        WorkTotal::exact_count(self.verification_work).join_status(self.work_status())
    }
    pub fn checked_stop_rule_work(&self) -> WorkTotal {
        WorkTotal::exact_count(self.stop_rule_work).join_status(self.work_status())
    }
    pub fn checked_case_charge(&self) -> WorkTotal {
        self.checked_own_work()
            .add(self.checked_shared_work())
            .add(self.checked_verification_shared_work())
    }
    pub fn checked_invocation_increment(&self) -> WorkTotal {
        self.checked_own_work()
            .add(
                self.checked_shared_work()
                    .mul(u64::from(self.shared_built_here)),
            )
            .add(
                self.checked_verification_shared_work()
                    .mul(u64::from(self.verification_shared_built_here)),
            )
    }
}
impl std::fmt::Debug for AttemptRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("AttemptRecord");
        d.field("precision", &self.precision);
        d.field("role", &self.role);
        d.field("outcome", &self.outcome);
        d.field("residual_basis", &self.residual_basis);
        d.field("corrections", &self.corrections);
        d.field("pivot_margin_min", &self.pivot_margin_min);
        d.field("rcond", &self.rcond);
        d.field("residual_worst", &self.residual_worst);
        d.field("work", &self.work);
        d.field("k4_work", &self.k4_work);
        d.field("stages", &self.stages);
        d.field("shared_work", &self.shared_work);
        d.field("shared_stages", &self.shared_stages);
        d.field("shared_built_here", &self.shared_built_here);
        d.field("stop_rule_work", &self.stop_rule_work);
        d.field("storage", &self.storage);
        d.field("gate", &self.gate);
        d.field("verification_work", &self.verification_work);
        d.field("verification", &self.verification);
        d.field("verification_shared_work", &self.verification_shared_work);
        d.field(
            "verification_shared_built_here",
            &self.verification_shared_built_here,
        );
        d.field("bound_refusals", &self.bound_refusals);
        if !self.work_status().is_exact() {
            d.field("work_status", &self.work_status());
        }
        d.finish()
    }
}

/// A refusal: no rows, no escalation.
#[derive(Debug, Clone, PartialEq)]
pub enum Refusal {
    MechanismWitnessed {
        body: u32,
        rigid_parameters: [f64; 6],
    },
    GeometryUnavailable {
        body: u32,
        error: StructuralError,
    },
    NegativeEnergy {
        i: usize,
        j: usize,
    },
    LedgerUnavailable(LedgerRefusal),
    Structure,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnresolvedReason {
    CountRange(&'static str),
    WorkAccounting {
        fault: WorkFault,
        prior: Option<AttemptStop>,
    },
    /// "At the ceiling … the case is unresolved."
    Ceiling,
    Budget(BudgetScope),
    ExactSumSpan,
    ExponentRange,
    ZeroDiagonal {
        global_dof: usize,
    },
    Arithmetic(WideError),
    /// D1 revision 5a.3 (ROOT's A3-0 ruling Q8): a body's E rounds to +∞;
    /// F2a maps it to `receipt_encoding`.
    ResolutionScaleUnencodable {
        body: u32,
        kind: Kind,
    },
    /// A selected pair's B_b not below 2^1024 (R7 §4.1.6.3 item 14: derived
    /// impossible at a selected p; kept as a guard).
    CertifiedBoundUnencodable {
        body: u32,
    },
    PublicationCertificate {
        index: Option<usize>,
        issue: CertificateIssue,
    },
}

/// A published row and its class.
#[derive(Debug, Clone, PartialEq)]
pub struct PublishedRow {
    pub id: QuantityId,
    pub kind: Kind,
    pub body: u32,
    pub value: Binary64Outcome,
    pub class: RowClass,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Publication {
    pub rows: Vec<PublishedRow>,
    /// S\* per (body, kind) from the published values, as bits.
    pub body_scales: Vec<(u32, Kind, u64)>,
}

/// The classification of published rows (items 1, 2a, 4–6; O9).
pub fn classify_rows(
    layout: &[QuantityMeta],
    values: &[Binary64Outcome],
    extents: &[f64],
) -> Publication {
    classify_rows_floored(layout, values, extents, None)
}

/// `classify_rows` with item 6a (D1 revision 5a.3): when the selected
/// precision is 512, `floor` holds Φ_fo and Φ_mo per body, applied after item
/// 6's coupling and before items 7 and 8.
pub fn classify_rows_floored(
    layout: &[QuantityMeta],
    values: &[Binary64Outcome],
    extents: &[f64],
    floor: Option<&[[f64; 2]]>,
) -> Publication {
    publication_with(layout, values, extents, floor, |x, s| Ok(classify(x, s)))
        .expect("the public classification closure is infallible")
}

/// One publication algorithm for both the public pure helper and the metered
/// production gate. Shape and finite metadata are checked by the latter's caller.
fn publication_with(
    layout: &[QuantityMeta],
    values: &[Binary64Outcome],
    extents: &[f64],
    floor: Option<&[[f64; 2]]>,
    mut classify_row: impl FnMut(f64, f64) -> Result<RowClass, AttemptStop>,
) -> Result<Publication, AttemptStop> {
    let mut s = vec![[0.0f64; 4]; extents.len()];
    for (meta, v) in layout.iter().zip(values) {
        if meta.input_derived {
            continue;
        }
        if let Some(x) = v.value() {
            let slot = &mut s[meta.body as usize][meta.kind.index()];
            *slot = slot.max(x.abs());
        }
    }
    let scales: Vec<[f64; 4]> = s
        .iter()
        .zip(extents)
        .enumerate()
        .map(|(b, (&s, &e))| {
            let mut c = coupled_scales(s, e);
            if let Some(phi) = floor {
                c[Kind::Force.index()] = c[Kind::Force.index()].max(phi[b][0]);
                c[Kind::Moment.index()] = c[Kind::Moment.index()].max(phi[b][1]);
            }
            c
        })
        .collect();
    let rows = layout
        .iter()
        .zip(values)
        .map(|(meta, v)| {
            let class = if meta.input_derived {
                RowClass::InputDerived
            } else if let Some(x) = v.value() {
                classify_row(x, scales[meta.body as usize][meta.kind.index()])?
            } else {
                RowClass::Unpublishable
            };
            Ok(PublishedRow {
                id: meta.id,
                kind: meta.kind,
                body: meta.body,
                value: *v,
                class,
            })
        })
        .collect::<Result<Vec<_>, AttemptStop>>()?;
    let body_scales = scales
        .iter()
        .enumerate()
        .flat_map(|(b, s)| {
            Kind::ALL
                .iter()
                .map(move |&k| (b as u32, k, s[k.index()].to_bits()))
        })
        .collect();
    Ok(Publication { rows, body_scales })
}

// ------------------------------------------------------------ SI publication certificate

const ABSENT_RADIUS_BITS: u64 = 0x7ff0_0000_0000_0000;

fn certificate_error(index: Option<usize>, issue: CertificateIssue) -> AttemptStop {
    AttemptStop::PublicationCertificate { index, issue }
}

fn finite_nonnegative(value: f64) -> Result<(), CertificateIssue> {
    if !value.is_finite() {
        Err(CertificateIssue::NonFinite)
    } else if value.to_bits() == 1u64 << 63 {
        Err(CertificateIssue::NonCanonicalZero)
    } else if value < 0.0 {
        Err(CertificateIssue::NegativeField)
    } else {
        Ok(())
    }
}

/// The candidate's new work, with no shared/cache charge. Every local sum is
/// collected even when an operation refuses. The legacy ExactAccumulator used
/// for exact prescriptions remains outside this inherited LME instrumentation.
struct CertificateMeter {
    ctx: WideContext<16>,
    sums: SumWork,
    guard: StageGuard,
}

impl CertificateMeter {
    fn new(guard: StageGuard) -> Self {
        Self {
            ctx: WideContext::new(1024).expect("supported precision"),
            sums: SumWork::default(),
            guard,
        }
    }

    fn total(&self) -> WorkTotal {
        lme(&self.ctx).add(self.sums.checked_lme())
    }

    fn checked<T>(&self, result: Result<T, AttemptStop>) -> Result<T, AttemptStop> {
        // Keep an arithmetic refusal's existing precedence; charge its work in
        // the caller before returning it. Case wins over Invocation in test().
        match result {
            Ok(value) => {
                self.guard.test(self.total())?;
                Ok(value)
            }
            Err(stop) => Err(stop),
        }
    }

    fn collect<T>(
        &mut self,
        sum: &ExactWideSum,
        result: Result<T, AttemptStop>,
    ) -> Result<T, AttemptStop> {
        self.sums.merge(&sum.work());
        self.checked(result)
    }

    fn collect_clone<T>(
        &mut self,
        operation: (Result<T, SumRefusal>, SumWork),
    ) -> Result<T, AttemptStop> {
        let (result, delta) = operation;
        self.sums.merge(&delta);
        self.checked(result.map_err(AttemptStop::from))
    }

    fn reaches(
        &mut self,
        c: f64,
        den: &ExactWideSum,
        num: &ExactWideSum,
    ) -> Result<bool, AttemptStop> {
        let mut scratch = ExactWideSum::new();
        let result = (|| {
            if c != 0.0 {
                let bits = c.to_bits();
                let biased = ((bits >> 52) & 0x7ff) as i64;
                let fraction = bits & ((1u64 << 52) - 1);
                let (significand, lsb) = if biased == 0 {
                    (fraction, -1074)
                } else {
                    (fraction | (1u64 << 52), biased - 1075)
                };
                scratch.add_scaled(den, false, significand, lsb)?;
            }
            scratch.add_scaled(num, true, 1, 0)?;
            Ok(scratch.signum()? >= 0)
        })();
        self.collect(&scratch, result)
    }

    /// Accounting-aware upward H/1 conversion. Same exact correction as
    /// directed_ratio(Up); the old R7 helper and its work contract are unchanged.
    fn round_up(&mut self, num: &ExactWideSum) -> Result<f64, AttemptStop> {
        let mut n = CloneWork::new(num);
        let sign = self.collect_clone(n.signum())?;
        if sign < 0 {
            return Err(certificate_error(None, CertificateIssue::NegativeField));
        }
        if sign == 0 {
            return Ok(0.0);
        }
        let operation = n.round(&mut self.ctx);
        let nv = self.collect_clone(operation)?;
        let mut den = ExactWideSum::new();
        let formed = den.add_binary64(1.0, false).map_err(AttemptStop::from);
        self.collect(&den, formed)?;
        let mut d = CloneWork::new(&den);
        let operation = d.round(&mut self.ctx);
        let dv = self.collect_clone(operation)?;
        let quotient = self.ctx.div(&nv, &dv).map_err(AttemptStop::from);
        let quotient = self.checked(quotient)?;
        let mut c = match quotient.to_binary64() {
            Binary64Outcome::Normal(v) => v.abs(),
            Binary64Outcome::Subnormal { value, .. } => value.abs(),
            Binary64Outcome::Underflow { .. } => 0.0,
            Binary64Outcome::Overflow { .. } => f64::MAX,
        };
        while !self.reaches(c, &den, num)? {
            if c == f64::MAX {
                return Ok(f64::INFINITY);
            }
            c = next_up(c);
        }
        while c > 0.0 && self.reaches(next_down(c), &den, num)? {
            c = next_down(c);
        }
        self.checked(Ok(c))
    }

    fn row_bound(&mut self, value: f64, scale: f64) -> Result<f64, AttemptStop> {
        finite_nonnegative(scale).map_err(|e| certificate_error(None, e))?;
        if !value.is_finite() {
            return Err(certificate_error(None, CertificateIssue::NonFinite));
        }
        let b = absolute_bound(scale);
        if scale == 0.0 || scale >= f64::from_bits(0x0230_0000_0000_0000) {
            return self.checked(Ok(b));
        }
        let q = value.abs();
        let two53 = 9_007_199_254_740_992.0;
        let nearest = q / two53;
        let rounding = if nearest * two53 < q {
            next_up(nearest)
        } else {
            nearest
        };
        let mut num = ExactWideSum::new();
        let formed = (|| {
            num.add_binary64(b, false)?;
            num.add_binary64(rounding, false)?;
            num.add_binary64(f64::from_bits(1), false)?;
            Ok(())
        })();
        self.collect(&num, formed)?;
        self.round_up(&num)
    }
}

fn required_error<const L: usize>(
    rows: &[Option<Wide<L>>],
    index: usize,
) -> Result<&Wide<L>, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    let value = rows
        .get(index)
        .and_then(Option::as_ref)
        .ok_or_else(|| certificate_error(Some(index), CertificateIssue::MissingField))?;
    if value.is_sign_negative() {
        return Err(certificate_error(
            Some(index),
            CertificateIssue::NegativeField,
        ));
    }
    Ok(value)
}

/// Full R7 verification-error theorem, never its disputed scale-transfer corollary.
fn publication_h<const L: usize>(
    index: usize,
    meta: &QuantityMeta,
    x: f64,
    v: &Wide<L>,
    report: &VerificationReport<L>,
    meter: &mut CertificateMeter,
) -> Result<ExactWideSum, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    let p = i64::from(report.precision);
    let mut h = ExactWideSum::new();
    let formed = (|| {
        h.add_binary64(x, false)?;
        h.add_wide(v, true)?;
        h.make_absolute()?;
        match meta.kind {
            Kind::Force | Kind::Moment => {
                h.add_wide_scaled(required_error(&report.e_rows, index)?, false, 69, -p)?;
                let w = required_error(&report.w, index)?;
                h.add_wide(w, false)?;
                h.add_wide_scaled(w, false, 1, -p)?;
                h.add_wide(required_error(&report.charge, index)?, false)?;
            }
            Kind::Translation | Kind::Rotation => {
                h.add_wide(required_error(&report.w_plus, index)?, false)?;
                if matches!(meta.id, QuantityId::DisplacementMagnitude(_)) {
                    h.add_wide_scaled(&v.abs(), false, 1, 1 - p)?;
                }
            }
        }
        Ok(())
    })();
    meter.collect(&h, formed)?;
    Ok(h)
}

/// The protected binary64 allowance, with both branches rounded separately.
fn sharper_binary64(x: f64, scale: f64) -> Result<f64, CertificateIssue> {
    sharper_binary64_spent(x, scale).result
}

struct SharperBinary64Spent {
    result: Result<f64, CertificateIssue>,
    f64_operations: u8,
}

// The count belongs to this single producing execution, including later
// validation failure. Input refusals execute none of the five operations.
fn sharper_binary64_spent(x: f64, scale: f64) -> SharperBinary64Spent {
    let mut f64_operations = 0;
    let result = (|| {
        finite_nonnegative(scale)?;
        if !x.is_finite() {
            return Err(CertificateIssue::NonFinite);
        }
        let a0 = f64::from_bits(0x3bf0_0000_0000_0000) * x.abs().max(scale);
        let a1 = a0 * f64::from_bits(0x3ff0_0000_8000_0000);
        let u0 = f64::from_bits(0x3ca0_0000_0000_0000) * x.abs();
        let u1 = u0 + f64::from_bits(1);
        let a2 = a1 + u1;
        f64_operations = 5; // all five nonfallible operations actually completed
        for value in [a0, a1, u0, u1, a2] {
            finite_nonnegative(value)?;
        }
        Ok(a2)
    })();
    SharperBinary64Spent {
        result,
        f64_operations,
    }
}

fn publication_predicate(
    h: &ExactWideSum,
    row: &PublishedRow,
    scale: f64,
    meter: &mut CertificateMeter,
) -> Result<Option<PublicationPredicate>, AttemptStop> {
    let x = row
        .value
        .value()
        .ok_or_else(|| certificate_error(None, CertificateIssue::RadiusClassMismatch))?;
    if !x.is_finite() {
        return Err(certificate_error(None, CertificateIssue::NonFinite));
    }
    if x.to_bits() == 1u64 << 63 {
        return Err(certificate_error(None, CertificateIssue::NonCanonicalZero));
    }
    finite_nonnegative(scale).map_err(|e| certificate_error(None, e))?;
    let mut compare =
        |predicate, build: &mut dyn FnMut(&mut ExactWideSum) -> Result<(), AttemptStop>| {
            let mut difference = ExactWideSum::new();
            let result = (|| {
                build(&mut difference)?;
                Ok(difference.signum()? >= 0)
            })();
            let passed = meter.collect(&difference, result)?;
            Ok::<_, AttemptStop>((!passed).then_some(predicate))
        };
    match row.class {
        RowClass::AbsoluteVerified { bound_bits } => {
            let b = f64::from_bits(bound_bits);
            finite_nonnegative(b).map_err(|e| certificate_error(None, e))?;
            compare(PublicationPredicate::AbsoluteBound, &mut |s| {
                s.add_binary64(b, false)?;
                s.add_scaled(h, true, 1, 0)?;
                Ok(())
            })
        }
        RowClass::RelativeVerified => {
            // Validate the entire finite allowance before its use, even if the
            // earlier public predicate would reject this row.
            let a64 = sharper_binary64(x, scale).map_err(|e| certificate_error(None, e))?;
            if let Some(failed) = compare(PublicationPredicate::PublicRelative, &mut |s| {
                s.add_binary64(x.abs(), false)?;
                s.add_scaled(h, true, 1_000_000_000, 0)?;
                Ok(())
            })? {
                return Ok(Some(failed));
            }
            let m = Wide::<4>::from_f64(x.abs().max(scale))?;
            let abs_x = Wide::<4>::from_f64(x.abs())?;
            if let Some(failed) = compare(PublicationPredicate::SharperExact, &mut |s| {
                s.add_wide_scaled(&m, false, 1, -64)?;
                s.add_wide_scaled(&m, false, 1, -85)?;
                s.add_wide_scaled(&abs_x, false, 1, -53)?;
                s.add_binary64(f64::from_bits(1), false)?;
                s.add_scaled(h, true, 1, 0)?;
                Ok(())
            })? {
                return Ok(Some(failed));
            }
            compare(PublicationPredicate::SharperBinary64, &mut |s| {
                s.add_binary64(a64, false)?;
                s.add_scaled(h, true, 1, 0)?;
                Ok(())
            })
        }
        _ => Err(certificate_error(
            None,
            CertificateIssue::RadiusClassMismatch,
        )),
    }
}

struct CertifiedPublication {
    publication: Publication,
    radii: Box<[u64]>,
    prep: Arc<CasePrep>,
    precision: u32,
}

enum PublicationDecision {
    Accepted(CertifiedPublication),
    Rejected {
        index: usize,
        predicate: PublicationPredicate,
    },
}

struct PublicationSpent {
    result: Result<PublicationDecision, AttemptStop>,
    work: AttemptWork,
    sums: SumWork,
    total: WorkTotal,
}

fn report_shape<const L: usize>(
    prep: &CasePrep,
    candidate_len: usize,
    verification_len: usize,
    candidate_p: u32,
    verification_p: u32,
    report: &VerificationReport<L>,
) -> Result<(), AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    let q = prep.layout.len();
    let b = prep.source.body_count() as usize;
    if verification_p != 2 * candidate_p || report.precision != verification_p {
        return Err(certificate_error(None, CertificateIssue::Precision));
    }
    if candidate_len != q
        || verification_len != q
        || prep.extents.len() != b
        || report.resolution.len() != b
        || report.bodies.len() != b
        || [&report.e_rows, &report.w, &report.charge, &report.w_plus]
            .iter()
            .any(|rows| rows.len() != q)
    {
        return Err(certificate_error(None, CertificateIssue::Shape));
    }
    // A transient canonical layout is released before the publication draft.
    if prep.layout != layout(&prep.source) {
        return Err(certificate_error(None, CertificateIssue::RowIdentity));
    }
    for &extent in &prep.extents {
        finite_nonnegative(extent).map_err(|e| certificate_error(None, e))?;
    }
    for value in report.resolution.iter().flatten() {
        finite_nonnegative(*value).map_err(|e| certificate_error(None, e))?;
    }
    for (index, meta) in prep.layout.iter().enumerate() {
        if meta.body as usize >= b {
            return Err(certificate_error(
                Some(index),
                CertificateIssue::RowIdentity,
            ));
        }
        if meta.input_derived {
            let QuantityId::Displacement(dof) = meta.id else {
                return Err(certificate_error(
                    Some(index),
                    CertificateIssue::RowIdentity,
                ));
            };
            if prep
                .prescribed
                .binary_search_by_key(&dof.global(), |t| t.0)
                .is_err()
            {
                return Err(certificate_error(
                    Some(index),
                    CertificateIssue::MissingField,
                ));
            }
        }
    }
    Ok(())
}

fn validate_publication_pair(
    prep: &Arc<CasePrep>,
    candidate: &PrecisionState,
    verification: &PrecisionState,
    bound: &BoundVerification,
) -> Result<(), AttemptStop> {
    if !Arc::ptr_eq(prep, &bound.prep) || !verification.same_state(&bound.state) {
        return Err(certificate_error(None, CertificateIssue::PairIdentity));
    }
    match (candidate, verification, &bound.report) {
        (PrecisionState::P128(a), PrecisionState::P256(b), VerificationState::V256(r))
            if a.p == 128 && b.p == 256 =>
        {
            report_shape(
                prep,
                a.recovered.values.len(),
                b.recovered.values.len(),
                a.p,
                b.p,
                r,
            )
        }
        (PrecisionState::P256(a), PrecisionState::P512(b), VerificationState::V512(r))
            if a.p == 256 && b.p == 512 =>
        {
            report_shape(
                prep,
                a.recovered.values.len(),
                b.recovered.values.len(),
                a.p,
                b.p,
                r,
            )
        }
        (PrecisionState::P512(a), PrecisionState::P1024(b), VerificationState::V1024(r))
            if a.p == 512 && b.p == 1024 =>
        {
            report_shape(
                prep,
                a.recovered.values.len(),
                b.recovered.values.len(),
                a.p,
                b.p,
                r,
            )
        }
        _ => Err(certificate_error(None, CertificateIssue::Precision)),
    }
}

fn certify_rows<const L: usize>(
    prep: &Arc<CasePrep>,
    precision: u32,
    publication: Publication,
    verification: &[Wide<L>],
    report: &VerificationReport<L>,
    meter: &mut CertificateMeter,
) -> Result<PublicationDecision, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    let q = prep.layout.len();
    if publication.rows.len() != q || verification.len() != q {
        return Err(certificate_error(None, CertificateIssue::Shape));
    }
    let mut radii = vec![ABSENT_RADIUS_BITS; q].into_boxed_slice();
    for (index, ((row, meta), v)) in publication
        .rows
        .iter()
        .zip(&prep.layout)
        .zip(verification)
        .enumerate()
    {
        if (row.id, row.body, row.kind) != (meta.id, meta.body, meta.kind) {
            return Err(certificate_error(
                Some(index),
                CertificateIssue::RowIdentity,
            ));
        }
        if meta.input_derived {
            if row.class != RowClass::InputDerived {
                return Err(certificate_error(
                    Some(index),
                    CertificateIssue::RadiusClassMismatch,
                ));
            }
            continue;
        }
        let Some(x) = row.value.value() else {
            if row.class != RowClass::Unpublishable {
                return Err(certificate_error(
                    Some(index),
                    CertificateIssue::RadiusClassMismatch,
                ));
            }
            continue;
        };
        if !x.is_finite() {
            return Err(certificate_error(Some(index), CertificateIssue::NonFinite));
        }
        if x.to_bits() == 1u64 << 63 {
            return Err(certificate_error(
                Some(index),
                CertificateIssue::NonCanonicalZero,
            ));
        }
        let scale = publication
            .body_scales
            .get(meta.body as usize * 4 + meta.kind.index())
            .filter(|&&(body, kind, _)| body == meta.body && kind == meta.kind)
            .map(|&(_, _, bits)| f64::from_bits(bits))
            .ok_or_else(|| certificate_error(Some(index), CertificateIssue::Shape))?;
        let h = publication_h(index, meta, x, v, report, meter)?;
        if let Some(predicate) = publication_predicate(&h, row, scale, meter)? {
            return Ok(PublicationDecision::Rejected { index, predicate });
        }
        let radius = meter.round_up(&h)?;
        finite_nonnegative(radius).map_err(|e| certificate_error(Some(index), e))?;
        let ceiling = match row.class {
            RowClass::AbsoluteVerified { bound_bits } => f64::from_bits(bound_bits),
            RowClass::RelativeVerified => {
                sharper_binary64(x, scale).map_err(|e| certificate_error(Some(index), e))?
            }
            _ => {
                return Err(certificate_error(
                    Some(index),
                    CertificateIssue::RadiusClassMismatch,
                ))
            }
        };
        if radius > ceiling {
            return Err(certificate_error(
                Some(index),
                CertificateIssue::RadiusClassMismatch,
            ));
        }
        radii[index] = radius.to_bits();
    }
    meter.checked(Ok(PublicationDecision::Accepted(CertifiedPublication {
        publication,
        radii,
        prep: prep.clone(),
        precision,
    })))
}

fn certify_publication(
    prep: &Arc<CasePrep>,
    candidate: &PrecisionState,
    verification: &PrecisionState,
    report: &BoundVerification,
    floor: Option<&[[f64; 2]]>,
    guard: StageGuard,
) -> PublicationSpent {
    let mut meter = CertificateMeter::new(guard);
    let mut result = (|| {
        validate_publication_pair(prep, candidate, verification, report)?;
        if (candidate.precision() == 512) != floor.is_some()
            || floor.is_some_and(|f| f.len() != prep.extents.len())
        {
            return Err(certificate_error(None, CertificateIssue::Shape));
        }
        for value in floor.into_iter().flatten().flatten() {
            finite_nonnegative(*value).map_err(|e| certificate_error(None, e))?;
        }
        let mut values = candidate.published();
        // ExactAccumulator is the inherited, explicitly unmetered legacy
        // boundary. This happens once for each candidate reaching the new gate,
        // including rejected candidates; finalization does not repeat it.
        prep.publish_prescribed(&mut values);
        let publication =
            publication_with(&prep.layout, &values, &prep.extents, floor, |x, scale| {
                finite_nonnegative(scale).map_err(|e| certificate_error(None, e))?;
                if relative_class(x, scale) {
                    Ok(RowClass::RelativeVerified)
                } else {
                    Ok(RowClass::AbsoluteVerified {
                        bound_bits: meter.row_bound(x, scale)?.to_bits(),
                    })
                }
            })?;
        for &(_, _, bits) in &publication.body_scales {
            finite_nonnegative(f64::from_bits(bits)).map_err(|e| certificate_error(None, e))?;
        }
        drop(values);
        match (verification, &report.report) {
            (PrecisionState::P256(v), VerificationState::V256(r)) => certify_rows(
                prep,
                candidate.precision(),
                publication,
                &v.recovered.values,
                r,
                &mut meter,
            ),
            (PrecisionState::P512(v), VerificationState::V512(r)) => certify_rows(
                prep,
                candidate.precision(),
                publication,
                &v.recovered.values,
                r,
                &mut meter,
            ),
            (PrecisionState::P1024(v), VerificationState::V1024(r)) => certify_rows(
                prep,
                candidate.precision(),
                publication,
                &v.recovered.values,
                r,
                &mut meter,
            ),
            _ => Err(certificate_error(None, CertificateIssue::Precision)),
        }
    })();
    let mut work = AttemptWork::default();
    work.record(&meter.ctx);
    let total = finish_work(&mut result, meter.total(), &StageWork::default());
    PublicationSpent {
        total,
        sums: meter.sums,
        work,
        result,
    }
}

/// The evidence F2a's receipt needs (D1 §5 item 1), as kernel types.
#[derive(Debug, Clone, PartialEq)]
pub struct RetainedEvidence {
    pub method: &'static str,
    pub policy: &'static str,
    pub attempts: Vec<AttemptRecord>,
    pub selected_precision: u32,
    pub verification_precision: u32,
    pub stop_rule: Vec<(u32, Kind, f64)>,
    pub floor_ratio_bits: u64,
    pub body_scales: Vec<(u32, Kind, u64)>,
    pub input_derived_dofs: Vec<Dof>,
    pub absolute_verified: Vec<(QuantityId, u64)>,
    pub not_covered: Vec<QuantityId>,
    pub unpublishable: Vec<(QuantityId, Binary64Outcome)>,
    pub pivot_margin_min: f64,
    pub rcond: f64,
    pub rcond_label: &'static str,
    pub residual_worst: f64,
    pub corrections: u8,
    pub geometry: Vec<BodyGeometry>,
    pub source_encoding: Vec<u8>,
    pub ledger_encoding: Vec<u8>,
    pub retained_state_encoding: Vec<u8>,
    /// D1 revision 5a.3 (R7 §5.8): per body, E_fo and E_mo bits (uncoupled,
    /// rounded upward, finite).
    pub resolution_scale: Vec<(u32, u64, u64)>,
    /// Per body and kind of force and moment: the worst Ŵ_q/V_q and C_q over
    /// its allowance, rounded upward.
    pub verification_estimate: Vec<(u32, Kind, f64)>,
    pub verification_charge: Vec<(u32, Kind, f64)>,
    /// Per body: the largest θ_c over its blocks with data (0 without).
    pub theta: Vec<(u32, f64)>,
    /// Per body with a block with data: B_b's bits, rounded upward (ROOT's
    /// A3-0 ruling Q9: no entry otherwise).
    pub certified_bound: Vec<(u32, u64)>,
    /// Φ_fo and Φ_mo bits per body when the selected precision is 512.
    pub floor: Option<Vec<(u32, u64, u64)>>,
}

/// A selected case: bound to its source and precision (D1 §4.1.1).
#[derive(Debug, Clone)]
pub struct RetainedSolve {
    pub(crate) prep: Arc<CasePrep>,
    pub(crate) group: Arc<GroupPrep>,
    /// The group's shared stages as this solve left them (a combination of
    /// this solve reuses them: ROOT's ruling on I12's F-1).
    pub(crate) cache: GroupCache,
    pub(crate) states: Vec<PrecisionState>,
    selected: u32,
    evidence: RetainedEvidence,
    publication: Publication,
    /// Private RU64(H), in the row kind's SI unit, parallel to publication.
    /// +infinity is a tagged absence only for InputDerived/Unpublishable.
    publication_radius_bits: Box<[u64]>,
}

impl RetainedSolve {
    /// "RetainedSolve::publish() rounds each quantity once."
    pub fn publish(&self) -> &Publication {
        &self.publication
    }
    pub fn evidence(&self) -> &RetainedEvidence {
        &self.evidence
    }
    pub fn source(&self) -> &PrimitiveSource {
        &self.prep.source
    }
    pub fn selected_precision(&self) -> u32 {
        self.selected
    }
    #[allow(dead_code)] // F2a API
    pub(crate) fn state(&self, precision: u32) -> Option<&PrecisionState> {
        self.states.iter().find(|s| s.precision() == precision)
    }

    /// Private producer integration only; no report borrow or public radius API.
    /// Kind names its SI coordinate (m, rad, N, N·m). Absence is never zero.
    #[allow(dead_code)] // future bounded F2a integration; tested here
    fn publication_radius(
        &self,
        index: usize,
        expected: QuantityMeta,
        source_identity: &[u8],
        precision: u32,
    ) -> Result<Option<SiRadius>, CertificateIssue> {
        self.validate_publication_owner(source_identity, precision)?;
        self.publication_radius_checked(index, expected)
    }

    fn validate_publication_owner(
        &self,
        source_identity: &[u8],
        precision: u32,
    ) -> Result<(), CertificateIssue> {
        self.validate_publication_owner_spent(source_identity, precision, &mut WorkTotal::zero())
    }

    fn validate_publication_owner_spent(
        &self,
        source_identity: &[u8],
        precision: u32,
        compared_bytes: &mut WorkTotal,
    ) -> Result<(), CertificateIssue> {
        if precision != self.selected
            || self.evidence.selected_precision != precision
            || self.evidence.policy != POLICY
        {
            return Err(CertificateIssue::Precision);
        }
        if !bridge_bytes_equal(&self.prep.identity, source_identity, compared_bytes)
            || !bridge_bytes_equal(
                &self.evidence.source_encoding,
                source_identity,
                compared_bytes,
            )
        {
            return Err(CertificateIssue::PairIdentity);
        }
        if self.publication_radius_bits.len() != self.publication.rows.len()
            || self.publication.rows.len() != self.prep.layout.len()
        {
            return Err(CertificateIssue::Shape);
        }
        Ok(())
    }

    // The checked case borrow establishes the common checks once. Every row
    // still passes the original metadata, class, absence and ceiling checks.
    fn publication_radius_checked(
        &self,
        index: usize,
        expected: QuantityMeta,
    ) -> Result<Option<SiRadius>, CertificateIssue> {
        self.publication_radius_checked_spent(index, expected)
            .result
    }

    fn publication_radius_checked_spent(
        &self,
        index: usize,
        expected: QuantityMeta,
    ) -> PublicationRadiusSpent {
        let mut f64_operations = 0;
        let result = (|| {
            let row = self
                .publication
                .rows
                .get(index)
                .ok_or(CertificateIssue::Shape)?;
            let meta = self.prep.layout.get(index).ok_or(CertificateIssue::Shape)?;
            if *meta != expected || (row.id, row.body, row.kind) != (meta.id, meta.body, meta.kind)
            {
                return Err(CertificateIssue::RowIdentity);
            }
            if meta.input_derived != matches!(row.class, RowClass::InputDerived)
                || (!meta.input_derived
                    && (row.value.value().is_none()
                        != matches!(row.class, RowClass::Unpublishable)))
            {
                return Err(CertificateIssue::RadiusClassMismatch);
            }
            let bits = self.publication_radius_bits[index];
            match row.class {
                RowClass::InputDerived | RowClass::Unpublishable => {
                    if bits == ABSENT_RADIUS_BITS {
                        Ok(None)
                    } else {
                        Err(CertificateIssue::RadiusClassMismatch)
                    }
                }
                RowClass::AbsoluteVerified { .. } | RowClass::RelativeVerified => {
                    if bits == ABSENT_RADIUS_BITS {
                        return Err(CertificateIssue::RadiusClassMismatch);
                    }
                    finite_nonnegative(f64::from_bits(bits))?;
                    let ceiling = match row.class {
                        RowClass::AbsoluteVerified { bound_bits } => f64::from_bits(bound_bits),
                        RowClass::RelativeVerified => {
                            let &(body, kind, scale_bits) = self
                                .publication
                                .body_scales
                                .get(row.body as usize * 4 + row.kind.index())
                                .ok_or(CertificateIssue::Shape)?;
                            if (body, kind) != (row.body, row.kind) {
                                return Err(CertificateIssue::RowIdentity);
                            }
                            let spent = sharper_binary64_spent(
                                row.value
                                    .value()
                                    .ok_or(CertificateIssue::RadiusClassMismatch)?,
                                f64::from_bits(scale_bits),
                            );
                            f64_operations = spent.f64_operations;
                            spent.result?
                        }
                        _ => unreachable!("eligible class matched above"),
                    };
                    finite_nonnegative(ceiling)?;
                    if f64::from_bits(bits) > ceiling {
                        return Err(CertificateIssue::RadiusClassMismatch);
                    }
                    Ok(Some(SiRadius {
                        id: row.id,
                        body: row.body,
                        kind: row.kind,
                        bits,
                    }))
                }
            }
        })();
        PublicationRadiusSpent {
            result,
            f64_operations,
        }
    }
}

struct PublicationRadiusSpent {
    result: Result<Option<SiRadius>, CertificateIssue>,
    f64_operations: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SiRadius {
    id: QuantityId,
    body: u32,
    kind: Kind,
    bits: u64,
}

/// A case's outcome.
#[derive(Debug, Clone)]
pub enum CaseOutcome {
    Selected(Box<RetainedSolve>),
    Refused {
        refusal: Refusal,
        geometry: Vec<BodyGeometry>,
    },
    Unresolved {
        reason: UnresolvedReason,
        attempts: Vec<AttemptRecord>,
        geometry: Vec<BodyGeometry>,
    },
}

// ------------------------------------------------------------ the group cache and the schedule

/// A cached shared build: the build, or its non-budget failure with the work
/// it spent (counted again against every case that meets it, so each case's
/// budget decisions equal a separate solve's).
type Slot<S> = Option<Result<Arc<S>, (AttemptStop, WorkTotal, StageWork)>>;

/// A cached verification shared build, as `Slot`; a failure also keeps the
/// Uc_c refusals recorded before it (RV23-1).
type VerifySlot<S> = Option<Result<Arc<S>, (AttemptStop, WorkTotal, StageWork, Vec<BlockRefusal>)>>;

/// The shared stages of one stiffness identity, per precision.
#[derive(Debug, Default, Clone)]
pub(crate) struct GroupCache {
    s128: Slot<Shared<4, 4>>,
    s256: Slot<Shared<4, 8>>,
    s512: Slot<Shared<8, 16>>,
    s1024: Slot<Shared<16, 16>>,
    // D1 revision 5a.3: the verification's shared data per precision (q_W =
    // 448, 832 and 1024).
    v256: VerifySlot<VerifyShared<4, 8>>,
    v512: VerifySlot<VerifyShared<8, 16>>,
    v1024: VerifySlot<VerifyShared<16, 16>>,
}

/// The work already counted against a case.
struct CaseBudget {
    limit: u64,
    used: WorkTotal,
    invocation_increment: WorkTotal,
}

impl CaseBudget {
    fn charge(&mut self, case: WorkTotal, invocation: WorkTotal, meter: &mut InvocationMeter) {
        self.used = self.used.add(case);
        self.invocation_increment = self.invocation_increment.add(invocation);
        meter.charge(invocation);
        let status = self
            .used
            .status()
            .join(self.invocation_increment.status())
            .join(meter.checked_charged().status());
        self.used = self.used.join_status(status);
        self.invocation_increment = self.invocation_increment.join_status(status);
        meter.charge(WorkTotal::zero().join_status(status));
    }
    fn retain<T>(
        &mut self,
        result: Result<T, AttemptStop>,
        record: &mut AttemptRecord,
        meter: &mut InvocationMeter,
    ) -> Result<T, AttemptStop> {
        record.latch(self.used);
        record.latch(meter.checked_charged());
        if let Err(AttemptStop::WorkAccounting(fault)) = &result {
            record.work_status = record.work_status.join(WorkStatus::from_fault(*fault));
        }
        let status = record
            .checked_case_charge()
            .status()
            .join(record.checked_invocation_increment().status());
        record.work_status = record.work_status.join(status);
        self.charge(
            WorkTotal::zero().join_status(status),
            WorkTotal::zero().join_status(status),
            meter,
        );
        match result {
            Ok(v) => {
                record.checked_case_charge().exact()?;
                Ok(v)
            }
            Err(e) => Err(e),
        }
    }
}

/// The shared stages at p: reused from the cache, or built (and cached) here.
/// Returns the build or its stop, its total work, whether it was built here,
/// and its stage work.
fn obtain<const L: usize, const R: usize>(
    slot: &mut Slot<Shared<L, R>>,
    p: u32,
    q: u32,
    source: &PrimitiveSource,
    group: &GroupPrep,
    guard: StageGuard,
) -> (
    Result<Arc<Shared<L, R>>, AttemptStop>,
    WorkTotal,
    bool,
    StageWork,
)
where
    Wide<L>: SupportedWidth,
    Wide<R>: SupportedWidth,
{
    if let Some(cached) = slot {
        return match cached {
            Ok(shared) => (
                Ok(shared.clone()),
                shared.total,
                false,
                shared.stages.clone(),
            ),
            Err((stop, total, stages)) => (Err(stop.clone()), *total, false, stages.clone()),
        };
    }
    let spent = build_shared::<L, R>(p, q, source, group, guard);
    match spent.result {
        Ok(shared) => {
            let shared = Arc::new(shared);
            *slot = Some(Ok(shared.clone()));
            (Ok(shared), spent.total, true, spent.stages)
        }
        Err(stop) => {
            if !matches!(stop, AttemptStop::Budget(_)) {
                *slot = Some(Err((stop.clone(), spent.total, spent.stages.clone())));
            }
            (Err(stop), spent.total, true, spent.stages)
        }
    }
}

/// One solve at precision p: the shared stages (built or reused) and the case's
/// own stages. Returns the state or the stop, and the attempt record (its role
/// and outcome are set by the schedule).
fn solve_precision(
    p: u32,
    prep: &Arc<CasePrep>,
    group: &Arc<GroupPrep>,
    cache: &mut GroupCache,
    budget: &mut CaseBudget,
    meter: &mut InvocationMeter,
    mut trace: Option<&mut RunTrace<'_>>,
    physical_record: usize,
) -> (Result<PrecisionState, AttemptStop>, AttemptRecord) {
    let case_room = budget.used.room(budget.limit);
    let invocation_room = meter.room();
    let guard = StageGuard {
        base: WorkTotal::zero(),
        case_room,
        invocation_room,
    };
    let storage = StorageCounts {
        pattern_entries: group.structure.entry_count(),
        profile_entries: group.ordering.profile_entries,
        limbs_per_entry: match p {
            128 | 256 => 4,
            512 => 8,
            _ => 16,
        },
    };
    let mut record = AttemptRecord {
        work_status: WorkStatus::default(),
        precision: p,
        role: AttemptRole::Candidate,
        outcome: AttemptOutcome::Solved,
        residual_basis: if p == 1024 { 1024 } else { p + 64 },
        corrections: 0,
        pivot_margin_min: None,
        rcond: None,
        residual_worst: None,
        work: AttemptWork::default(),
        k4_work: SumWork::default(),
        stages: StageWork::default(),
        shared_work: 0,
        shared_stages: StageWork::default(),
        shared_built_here: false,
        stop_rule_work: 0,
        storage,
        gate: None,
        verification_work: 0,
        verification: None,
        verification_shared_work: 0,
        verification_shared_built_here: false,
        bound_refusals: Vec::new(),
    };
    macro_rules! run {
        ($slot:expr, $L:literal, $R:literal, $q:expr, $variant:ident) => {{
            let (shared, shared_total, built, shared_stages) =
                obtain::<$L, $R>($slot, p, $q, &prep.source, group, guard);
            if let Some(trace) = trace.as_deref_mut() {
                trace.requested(
                    OriginSlot::solve(p),
                    physical_record,
                    built,
                    shared.as_ref().err(),
                    shared_total,
                    &shared_stages,
                );
            }
            record.latch(shared_total);
            record.shared_work = shared_total.legacy_saturated();
            record.shared_built_here = built;
            record.shared_stages = shared_stages;
            // The shared work counts in full against this case; against the
            // invocation only when built here.
            let invocation_spent = if built {
                shared_total
            } else {
                shared_total.mul(0)
            };
            budget.charge(shared_total, invocation_spent, meter);
            let shared = budget.retain(shared, &mut record, meter);
            match shared {
                Err(stop) => {
                    let result = budget.retain(Err(stop), &mut record, meter);
                    (result, record)
                }
                Ok(_)
                    if shared_total
                        .exact()
                        .is_ok_and(|v| case_room.exact().is_ok_and(|room| v > room)) =>
                {
                    (Err(AttemptStop::Budget(BudgetScope::Case)), record)
                }
                Ok(shared) => {
                    record.pivot_margin_min = Some(shared.pivot_margin_min);
                    record.rcond = Some(shared.rcond);
                    let own_guard = StageGuard {
                        base: WorkTotal::zero(),
                        case_room: case_room - shared_total,
                        invocation_room: invocation_spent
                            .room(invocation_room.legacy_saturated())
                            .join_status(invocation_room.status()),
                    };
                    let spent = solve_case_at::<$L, $R>(&shared, prep, group, own_guard);
                    record.work = spent.work;
                    record.k4_work = spent.sum_work;
                    record.stages = spent.stages;
                    record.latch(spent.total);
                    budget.charge(spent.total, spent.total, meter);
                    match spent.result {
                        Ok(solved) => {
                            record.corrections = solved.corrections;
                            record.residual_worst = Some(solved.residual_worst);
                            record.gate = Some(solved.gate);
                            {
                                let result = budget.retain(
                                    Ok(PrecisionState::$variant(Arc::new(solved))),
                                    &mut record,
                                    meter,
                                );
                                (result, record)
                            }
                        }
                        Err(stop) => {
                            let result = budget.retain(Err(stop), &mut record, meter);
                            (result, record)
                        }
                    }
                }
            }
        }};
    }
    match p {
        128 => run!(&mut cache.s128, 4, 4, 192, P128),
        256 => run!(&mut cache.s256, 4, 8, 320, P256),
        512 => run!(&mut cache.s512, 8, 16, 576, P512),
        _ => run!(&mut cache.s1024, 16, 16, 1024, P1024),
    }
}

/// An attempt's reason for a rejection by `decide`.
fn rejection_reason(
    rejection: Option<Rejection>,
    layout: &[QuantityMeta],
    group: &GroupPrep,
) -> AttemptReason {
    let row = |index: usize| layout[index];
    match rejection {
        Some(Rejection::StopRule { index }) => AttemptReason::StopRule {
            quantity: row(index).id,
            body: row(index).body,
            kind: row(index).kind,
        },
        Some(Rejection::VerificationEstimate { index }) => AttemptReason::VerificationEstimate {
            quantity: row(index).id,
            body: row(index).body,
            kind: row(index).kind,
        },
        Some(Rejection::Uc { block }) => AttemptReason::Uc {
            body: group.blocks.body[block],
        },
        Some(Rejection::Theta { block }) => AttemptReason::Theta {
            body: group.blocks.body[block],
        },
        Some(Rejection::GValidity { member }) => AttemptReason::GValidity { member },
        Some(Rejection::Charge { index }) => AttemptReason::Charge {
            quantity: row(index).id,
            body: row(index).body,
            kind: row(index).kind,
        },
        None => AttemptReason::VerificationFailed,
    }
}

/// The verification's shared data at a precision: reused from the cache, or
/// built (and cached) here; as `obtain`. The last item is a failed build's
/// Uc_c refusals (RV23-1; empty on success, whose are in `uc`).
#[allow(clippy::type_complexity)]
fn obtain_verify<const L: usize, const R: usize, const W: usize>(
    slot: &mut VerifySlot<VerifyShared<L, W>>,
    shared: &Shared<L, R>,
    source: &PrimitiveSource,
    group: &GroupPrep,
    guard: StageGuard,
) -> (
    Result<Arc<VerifyShared<L, W>>, AttemptStop>,
    WorkTotal,
    bool,
    StageWork,
    Vec<BlockRefusal>,
)
where
    Wide<L>: SupportedWidth,
    Wide<R>: SupportedWidth,
    Wide<W>: SupportedWidth,
{
    if let Some(cached) = slot {
        return match cached {
            Ok(v) => (Ok(v.clone()), v.total, false, v.stages.clone(), Vec::new()),
            Err((stop, total, stages, refusals)) => (
                Err(stop.clone()),
                *total,
                false,
                stages.clone(),
                refusals.clone(),
            ),
        };
    }
    let spent = build_verify_shared::<L, R, W>(shared, source, group, guard);
    match spent.result {
        Ok(v) => {
            let v = Arc::new(v);
            *slot = Some(Ok(v.clone()));
            (Ok(v), spent.total, true, spent.stages, Vec::new())
        }
        Err(stop) => {
            if !matches!(stop, AttemptStop::Budget(_)) {
                *slot = Some(Err((
                    stop.clone(),
                    spent.total,
                    spent.stages.clone(),
                    spent.refusals.clone(),
                )));
            }
            (Err(stop), spent.total, true, spent.stages, spent.refusals)
        }
    }
}

/// D1 revision 5a.3's verification pass on a solved verification state: the
/// shared verification data (built or reused) and the case's own pass, both
/// charged to the verification attempt, the case budget and the meter (the
/// shared data once against the invocation).
fn verify_precision(
    state: &PrecisionState,
    prep: &Arc<CasePrep>,
    group: &Arc<GroupPrep>,
    cache: &mut GroupCache,
    budget: &mut CaseBudget,
    meter: &mut InvocationMeter,
    record: &mut AttemptRecord,
    mut trace: Option<&mut RunTrace<'_>>,
    physical_record: usize,
) -> Result<BoundVerification, AttemptStop> {
    let case_room = budget.used.room(budget.limit);
    let invocation_room = meter.room();
    let guard = StageGuard {
        base: WorkTotal::zero(),
        case_room,
        invocation_room,
    };
    macro_rules! run {
        ($slot:expr, $vslot:expr, $L:literal, $R:literal, $W:literal, $solved:expr, $variant:ident) => {{
            let shared = match $slot {
                Some(Ok(shared)) => shared.clone(),
                _ => unreachable!("a solved state's shared stages are cached"),
            };
            let (vs, vs_total, built, vs_stages, vs_refusals) =
                obtain_verify::<$L, $R, $W>($vslot, &shared, &prep.source, group, guard);
            if let Some(trace) = trace.as_deref_mut() {
                trace.requested(
                    OriginSlot::verify(state.precision()),
                    physical_record,
                    built,
                    vs.as_ref().err(),
                    vs_total,
                    &vs_stages,
                );
            }
            record.latch(vs_total);
            record.verification_shared_work = vs_total.legacy_saturated();
            record.verification_shared_built_here = built;
            let stage_merge = record.shared_stages.add(&vs_stages);
            let invocation_spent = if built { vs_total } else { vs_total.mul(0) };
            budget.charge(vs_total, invocation_spent, meter);
            // RV23-1: a stopped shared build's Uc_c refusals reach the record
            // before its stop propagates.
            if vs.is_err() {
                record.bound_refusals = vs_refusals;
            }
            let vs = budget.retain(vs, record, meter)?;
            stage_merge?;
            // Amendment A2: the shared build's Uc_c refusals, per block.
            record.bound_refusals = block_refusals(&vs.uc, &[]);
            if vs_total.exact()? > case_room.exact()? {
                return Err(AttemptStop::Budget(BudgetScope::Case));
            }
            let own_guard = StageGuard {
                base: WorkTotal::zero(),
                case_room: case_room - vs_total,
                invocation_room: invocation_spent
                    .room(invocation_room.legacy_saturated())
                    .join_status(invocation_room.status()),
            };
            let spent = verify_state::<$L, $R, $W>(&shared, &vs, prep, group, $solved, own_guard);
            record.work.merge(&spent.work);
            record.k4_work.merge(&spent.sum_work);
            let stage_merge = record.stages.add(&spent.stages);
            record.latch(spent.total);
            record.verification_work = spent.total.legacy_saturated();
            record.bound_refusals.extend(spent.refusals.iter().copied());
            budget.charge(spent.total, spent.total, meter);
            let result = budget.retain(spent.result, record, meter)?;
            stage_merge?;
            let report_state = VerificationState::$variant(Arc::new(result));
            record.verification = Some(report_state.summary()?);
            Ok(BoundVerification {
                prep: prep.clone(),
                state: (*state).clone(),
                report: report_state,
            })
        }};
    }
    match state {
        PrecisionState::P256(s) => run!(&cache.s256, &mut cache.v256, 4, 8, 8, s, V256),
        PrecisionState::P512(s) => run!(&cache.s512, &mut cache.v512, 8, 16, 16, s, V512),
        PrecisionState::P1024(s) => run!(&cache.s1024, &mut cache.v1024, 16, 16, 16, s, V1024),
        PrecisionState::P128(_) => unreachable!("128 is never a verification"),
    }
}

fn terminal(stop: &AttemptStop) -> Result<UnresolvedReason, Refusal> {
    match stop {
        AttemptStop::CountRange(field) => Ok(UnresolvedReason::CountRange(field)),
        AttemptStop::WorkAccounting(fault) => Ok(UnresolvedReason::WorkAccounting {
            fault: *fault,
            prior: None,
        }),
        AttemptStop::Budget(scope) => Ok(UnresolvedReason::Budget(*scope)),
        AttemptStop::Span => Ok(UnresolvedReason::ExactSumSpan),
        AttemptStop::Exponent => Ok(UnresolvedReason::ExponentRange),
        AttemptStop::ZeroDiagonal { global_dof } => Ok(UnresolvedReason::ZeroDiagonal {
            global_dof: *global_dof,
        }),
        AttemptStop::Arithmetic(e) => Ok(UnresolvedReason::Arithmetic(*e)),
        AttemptStop::ResolutionScale { body, kind } => {
            Ok(UnresolvedReason::ResolutionScaleUnencodable {
                body: *body,
                kind: *kind,
            })
        }
        AttemptStop::PublicationCertificate { index, issue } => {
            Ok(UnresolvedReason::PublicationCertificate {
                index: *index,
                issue: *issue,
            })
        }
        AttemptStop::NegativeEnergy { i, j } => Err(Refusal::NegativeEnergy { i: *i, j: *j }),
        AttemptStop::Structure => Err(Refusal::Structure),
        other => unreachable!("escalating stop {other:?} is not terminal"),
    }
}

/// The actual work of one core run, retained before compatibility projection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RunWork {
    case: WorkTotal,
    invocation_before: WorkTotal,
    invocation_increment: WorkTotal,
    invocation_after: WorkTotal,
}
impl RunWork {
    pub fn case(&self) -> WorkTotal {
        self.case
    }
    pub fn invocation_before(&self) -> WorkTotal {
        self.invocation_before
    }
    pub fn invocation_increment(&self) -> WorkTotal {
        self.invocation_increment
    }
    pub fn invocation_after(&self) -> WorkTotal {
        self.invocation_after
    }
}
#[derive(Debug, Clone)]
pub enum ExecutionOutcome {
    Selected(Box<RetainedSolve>),
    Refused {
        refusal: Refusal,
        attempts: Vec<AttemptRecord>,
        geometry: Vec<BodyGeometry>,
    },
    Unresolved {
        reason: UnresolvedReason,
        attempts: Vec<AttemptRecord>,
        geometry: Vec<BodyGeometry>,
    },
}
impl ExecutionOutcome {
    pub(crate) fn into_legacy(self) -> CaseOutcome {
        match self {
            Self::Selected(s) => CaseOutcome::Selected(s),
            Self::Refused {
                refusal, geometry, ..
            } => CaseOutcome::Refused { refusal, geometry },
            Self::Unresolved {
                reason,
                attempts,
                geometry,
            } => CaseOutcome::Unresolved {
                reason,
                attempts,
                geometry,
            },
        }
    }
}
pub(crate) struct CoreRun {
    pub(crate) outcome: ExecutionOutcome,
    pub(crate) work: RunWork,
}
impl CoreRun {
    fn idle(outcome: ExecutionOutcome, before: WorkTotal) -> Self {
        let zero = WorkTotal::zero().join_status(before.status());
        Self {
            outcome,
            work: RunWork {
                case: zero,
                invocation_before: before,
                invocation_increment: zero,
                invocation_after: before,
            },
        }
    }
    pub(crate) fn into_legacy(self) -> CaseOutcome {
        self.outcome.into_legacy()
    }
}
pub(crate) fn run_core(
    prep: Arc<CasePrep>,
    group: Arc<GroupPrep>,
    cache: &mut GroupCache,
    case_limit: CaseLimit,
    meter: &mut InvocationMeter,
) -> CoreRun {
    run_core_with_origins(prep, group, cache, case_limit, meter, None)
}

pub(crate) fn run_core_with_origins(
    prep: Arc<CasePrep>,
    group: Arc<GroupPrep>,
    cache: &mut GroupCache,
    case_limit: CaseLimit,
    meter: &mut InvocationMeter,
    trace: Option<&mut RunTrace<'_>>,
) -> CoreRun {
    let invocation_before = meter.checked_charged();
    let mut budget = CaseBudget {
        limit: case_limit.get(),
        used: WorkTotal::zero(),
        invocation_increment: WorkTotal::zero(),
    };
    let outcome = if let Some(fault) = invocation_before.status().fault() {
        budget.used = budget.used.join_status(invocation_before.status());
        budget.invocation_increment = budget
            .invocation_increment
            .join_status(invocation_before.status());
        ExecutionOutcome::Unresolved {
            reason: UnresolvedReason::WorkAccounting { fault, prior: None },
            attempts: Vec::new(),
            geometry: group.geometry.clone(),
        }
    } else {
        run_schedule_inner(prep, group, cache, &mut budget, meter, trace)
    };
    CoreRun {
        outcome,
        work: RunWork {
            case: budget.used,
            invocation_before,
            invocation_increment: budget.invocation_increment,
            invocation_after: meter.checked_charged(),
        },
    }
}
/// The schedule of one case (module documentation).
fn run_schedule_inner(
    prep: Arc<CasePrep>,
    group: Arc<GroupPrep>,
    cache: &mut GroupCache,
    budget: &mut CaseBudget,
    meter: &mut InvocationMeter,
    mut trace: Option<&mut RunTrace<'_>>,
) -> ExecutionOutcome {
    let geometry = group.geometry.clone();

    let mut attempts: Vec<AttemptRecord> = Vec::new();
    let mut states: Vec<PrecisionState> = Vec::new();
    // A solved verification that becomes the next candidate.
    let mut pending: Option<(PrecisionState, usize)> = None;
    let mut c = 0;
    while c < 3 {
        let p = PRECISIONS[c];
        let (candidate, candidate_index) = match pending.take() {
            Some(state) => state,
            None => {
                let (result, record) = solve_precision(
                    p,
                    &prep,
                    &group,
                    cache,
                    budget,
                    meter,
                    trace.as_deref_mut(),
                    attempts.len(),
                );
                attempts.push(record);
                let index = attempts.len() - 1;
                match result {
                    Ok(state) => {
                        states.push(state.clone());
                        (state, index)
                    }
                    Err(stop) => {
                        attempts[index].outcome =
                            AttemptOutcome::Failed(AttemptReason::Stop(stop.clone()));
                        if attempts.iter().any(|a| !a.work_status().is_exact()) {
                            return finish_terminal(&stop, attempts, geometry);
                        }
                        if stop.escalates() {
                            c += 1;
                            continue;
                        }
                        return finish_terminal(&stop, attempts, geometry);
                    }
                }
            }
        };
        let verification_p = PRECISIONS[c + 1];
        let (result, mut record) = solve_precision(
            verification_p,
            &prep,
            &group,
            cache,
            budget,
            meter,
            trace.as_deref_mut(),
            attempts.len(),
        );
        record.role = AttemptRole::Verification;
        attempts.push(record);
        let v_index = attempts.len() - 1;
        let verification = match result {
            Ok(state) => {
                states.push(state.clone());
                state
            }
            Err(stop) => {
                attempts[candidate_index].outcome =
                    AttemptOutcome::Rejected(AttemptReason::VerificationFailed);
                attempts[v_index].outcome =
                    AttemptOutcome::Failed(AttemptReason::Stop(stop.clone()));
                if attempts.iter().any(|a| !a.work_status().is_exact()) {
                    return finish_terminal(&stop, attempts, geometry);
                }
                if stop.escalates() {
                    // The failed verification cannot be the next candidate.
                    c += 2;
                    continue;
                }
                return finish_terminal(&stop, attempts, geometry);
            }
        };
        // D1 revision 5a.3: the verification pass (charged to the verification).
        let report = match verify_precision(
            &verification,
            &prep,
            &group,
            cache,
            budget,
            meter,
            &mut attempts[v_index],
            trace.as_deref_mut(),
            v_index,
        ) {
            Ok(report) => report,
            Err(stop) => {
                attempts[candidate_index].outcome =
                    AttemptOutcome::Rejected(AttemptReason::VerificationFailed);
                attempts[v_index].outcome =
                    AttemptOutcome::Failed(AttemptReason::Stop(stop.clone()));
                return finish_terminal(&stop, attempts, geometry);
            }
        };
        let guard = StageGuard {
            base: WorkTotal::zero(),
            case_room: budget.used.room(budget.limit),
            invocation_room: meter.room(),
        };
        if let Err(stop) = validate_publication_pair(&prep, &candidate, &verification, &report) {
            attempts[candidate_index].outcome =
                AttemptOutcome::Failed(AttemptReason::Stop(stop.clone()));
            return finish_terminal(&stop, attempts, geometry);
        }
        let decision = compare_states(
            &prep.layout,
            &prep.extents,
            &candidate,
            &verification,
            &report.report,
            guard,
        );
        // V-K seeded fault VK-F06 (§7.3-6): the candidate accepted whatever the
        // verification's verdict (accepted on the pivot screen alone).
        #[cfg(any(test, feature = "mutation-controls"))]
        let decision = if super::seeded::active(super::seeded::Fault::F06)
            && matches!(decision.result, Ok(false))
        {
            StopDecision {
                result: Ok(true),
                ..decision
            }
        } else {
            decision
        };
        {
            let record = &mut attempts[candidate_index];
            record.latch(decision.total);
            let stop_work = record.checked_stop_rule_work().add(decision.total);
            record.latch(stop_work);
            record.stop_rule_work = stop_work.legacy_saturated();
            let _ = record.stages.add_to(Stage::StopRule, decision.total);
            record.work.merge(&decision.work);
            record.k4_work.merge(&decision.sum_work);
        }
        budget.charge(decision.total, decision.total, meter);
        let decision_result = budget.retain(
            decision.result.clone(),
            &mut attempts[candidate_index],
            meter,
        );
        match decision_result {
            Err(stop) => {
                attempts[candidate_index].outcome =
                    AttemptOutcome::Failed(AttemptReason::Stop(stop.clone()));
                return finish_terminal(&stop, attempts, geometry);
            }
            Ok(true) => {
                let publication_guard = StageGuard {
                    base: WorkTotal::zero(),
                    case_room: budget.used.room(budget.limit),
                    invocation_room: meter.room(),
                };
                let certificate = certify_publication(
                    &prep,
                    &candidate,
                    &verification,
                    &report,
                    decision.floor.as_deref(),
                    publication_guard,
                );
                let record = &mut attempts[candidate_index];
                record.latch(certificate.total);
                let stop_work = record.checked_stop_rule_work().add(certificate.total);
                record.latch(stop_work);
                record.stop_rule_work = stop_work.legacy_saturated();
                let _ = record.stages.add_to(Stage::StopRule, certificate.total);
                record.work.merge(&certificate.work);
                record.k4_work.merge(&certificate.sums);
                budget.charge(certificate.total, certificate.total, meter);
                let certificate_result = budget.retain(certificate.result, record, meter);
                match certificate_result {
                    Err(stop) => {
                        attempts[candidate_index].outcome =
                            AttemptOutcome::Failed(AttemptReason::Stop(stop.clone()));
                        return finish_terminal(&stop, attempts, geometry);
                    }
                    Ok(PublicationDecision::Accepted(publication)) => {
                        attempts[candidate_index].outcome = AttemptOutcome::Accepted;
                        attempts[v_index].outcome = AttemptOutcome::Verified;
                        return finish_selected(
                            prep,
                            group,
                            cache.clone(),
                            states,
                            &candidate,
                            verification_p,
                            attempts,
                            &decision,
                            &report.report,
                            publication,
                            geometry,
                        );
                    }
                    Ok(PublicationDecision::Rejected { index, predicate }) => {
                        let row = prep.layout[index];
                        attempts[candidate_index].outcome =
                            AttemptOutcome::Rejected(AttemptReason::PublicationEnclosure {
                                quantity: row.id,
                                body: row.body,
                                kind: row.kind,
                                predicate,
                            });
                        #[cfg(any(test, feature = "mutation-controls"))]
                        if super::seeded::active(super::seeded::Fault::F05) {
                            return ExecutionOutcome::Unresolved {
                                reason: UnresolvedReason::Ceiling,
                                attempts,
                                geometry,
                            };
                        }
                        if c + 1 < 3 {
                            attempts[v_index].role = AttemptRole::VerificationThenCandidate;
                            pending = Some((verification, v_index));
                        }
                        c += 1;
                    }
                }
            }
            Ok(false) => {
                attempts[candidate_index].outcome = AttemptOutcome::Rejected(rejection_reason(
                    decision.rejection,
                    &prep.layout,
                    &group,
                ));
                // V-K seeded fault VK-F05 (§7.3-5): no escalation after a
                // rejected candidate.
                #[cfg(any(test, feature = "mutation-controls"))]
                if super::seeded::active(super::seeded::Fault::F05) {
                    return ExecutionOutcome::Unresolved {
                        reason: UnresolvedReason::Ceiling,
                        attempts,
                        geometry,
                    };
                }
                if c + 1 < 3 {
                    attempts[v_index].role = AttemptRole::VerificationThenCandidate;
                    pending = Some((verification, v_index));
                }
                c += 1;
            }
        }
    }
    ExecutionOutcome::Unresolved {
        reason: UnresolvedReason::Ceiling,
        attempts,
        geometry,
    }
}

fn finish_terminal(
    stop: &AttemptStop,
    attempts: Vec<AttemptRecord>,
    geometry: Vec<BodyGeometry>,
) -> ExecutionOutcome {
    let status = attempts.iter().fold(WorkStatus::default(), |status, a| {
        status.join(a.checked_case_charge().status())
    });
    if let Some(fault) = status.fault() {
        return ExecutionOutcome::Unresolved {
            reason: UnresolvedReason::WorkAccounting {
                fault,
                prior: if matches!(stop, AttemptStop::WorkAccounting(_)) {
                    None
                } else {
                    Some(stop.clone())
                },
            },
            attempts,
            geometry,
        };
    }
    match terminal(stop) {
        Ok(reason) => ExecutionOutcome::Unresolved {
            reason,
            attempts,
            geometry,
        },
        Err(refusal) => ExecutionOutcome::Refused {
            refusal,
            attempts,
            geometry,
        },
    }
}

#[allow(clippy::too_many_arguments)]
fn finish_selected(
    prep: Arc<CasePrep>,
    group: Arc<GroupPrep>,
    cache: GroupCache,
    states: Vec<PrecisionState>,
    selected: &PrecisionState,
    verification_precision: u32,
    mut attempts: Vec<AttemptRecord>,
    decision: &StopDecision,
    report: &VerificationState,
    certified: CertifiedPublication,
    geometry: Vec<BodyGeometry>,
) -> ExecutionOutcome {
    if !Arc::ptr_eq(&prep, &certified.prep) || selected.precision() != certified.precision {
        let stop = certificate_error(None, CertificateIssue::PairIdentity);
        if let Some(record) = attempts
            .iter_mut()
            .find(|a| a.precision == selected.precision() && a.outcome == AttemptOutcome::Accepted)
        {
            record.outcome = AttemptOutcome::Failed(AttemptReason::Stop(stop.clone()));
        }
        return finish_terminal(&stop, attempts, geometry);
    }
    // The certified bytes are moved, never re-rounded/reclassified here.
    let publication = certified.publication;
    let publication_radius_bits = certified.radii;
    let verification = match report.summary() {
        Ok(v) => v,
        Err(stop) => return finish_terminal(&stop, attempts, geometry),
    };
    for (body, bound) in verification.bound.iter().enumerate() {
        if bound.is_some_and(|b| !b.is_finite()) {
            return ExecutionOutcome::Unresolved {
                reason: UnresolvedReason::CertifiedBoundUnencodable { body: body as u32 },
                attempts,
                geometry,
            };
        }
    }
    let selected_record = attempts
        .iter()
        .find(|a| a.precision == selected.precision() && a.outcome == AttemptOutcome::Accepted)
        .cloned();
    let evidence = RetainedEvidence {
        method: METHOD_TOKEN,
        policy: POLICY,
        selected_precision: selected.precision(),
        verification_precision,
        stop_rule: decision.summary.clone(),
        floor_ratio_bits: FLOOR_RATIO_BITS,
        body_scales: publication.body_scales.clone(),
        input_derived_dofs: prep.source.constraints().iter().map(|c| c.dof).collect(),
        absolute_verified: publication
            .rows
            .iter()
            .filter_map(|r| match r.class {
                RowClass::AbsoluteVerified { bound_bits } => Some((r.id, bound_bits)),
                _ => None,
            })
            .collect(),
        not_covered: Vec::new(),
        unpublishable: publication
            .rows
            .iter()
            .filter(|r| r.class == RowClass::Unpublishable)
            .map(|r| (r.id, r.value))
            .collect(),
        pivot_margin_min: selected_record
            .as_ref()
            .and_then(|r| r.pivot_margin_min)
            .unwrap_or(0.0),
        rcond: selected_record
            .as_ref()
            .and_then(|r| r.rcond)
            .unwrap_or(0.0),
        rcond_label: RCOND_LABEL,
        residual_worst: selected_record
            .as_ref()
            .and_then(|r| r.residual_worst)
            .unwrap_or(0.0),
        corrections: selected.corrections(),
        geometry,
        source_encoding: prep.identity.clone(),
        ledger_encoding: prep.ledger.encoding(),
        retained_state_encoding: selected.encoding(),
        attempts,
        resolution_scale: verification
            .resolution
            .iter()
            .enumerate()
            .map(|(b, e)| (b as u32, e[0].to_bits(), e[1].to_bits()))
            .collect(),
        verification_estimate: decision.estimate_summary.clone(),
        verification_charge: decision.charge_summary.clone(),
        theta: verification
            .theta
            .iter()
            .enumerate()
            .map(|(b, t)| (b as u32, *t))
            .collect(),
        certified_bound: verification
            .bound
            .iter()
            .enumerate()
            .filter_map(|(b, v)| v.map(|v| (b as u32, v.to_bits())))
            .collect(),
        floor: decision.floor.as_ref().map(|f| {
            f.iter()
                .enumerate()
                .map(|(b, x)| (b as u32, x[0].to_bits(), x[1].to_bits()))
                .collect()
        }),
    };
    ExecutionOutcome::Selected(Box::new(RetainedSolve {
        prep,
        group,
        cache,
        states,
        selected: selected.precision(),
        evidence,
        publication,
        publication_radius_bits,
    }))
}

/// Geometry first, then the group's structure and ordering.
fn prepare_group(source: &PrimitiveSource) -> Result<GroupPrep, (Refusal, Vec<BodyGeometry>)> {
    let geometry = match geometry_first(source) {
        Ok(g) => g,
        Err(GeometryRefusal::MechanismWitnessed {
            body,
            rigid_parameters,
        }) => {
            return Err((
                Refusal::MechanismWitnessed {
                    body,
                    rigid_parameters,
                },
                Vec::new(),
            ))
        }
        Err(GeometryRefusal::Unavailable { body, error }) => {
            return Err((Refusal::GeometryUnavailable { body, error }, Vec::new()))
        }
    };
    let structure = Structure::new(source).map_err(|_| (Refusal::Structure, geometry.clone()))?;
    let ordering = order_free(source, &structure);
    let blocks = free_blocks(source, &structure, &ordering);
    Ok(GroupPrep {
        structure,
        ordering,
        geometry,
        blocks,
    })
}

/// The kernel entry: every case of an invocation (F2a API). Cases with the
/// same stiffness identity share formation and the p-factor per precision.
pub fn solve_cases(
    sources: &[PrimitiveSource],
    case_limit: CaseLimit,
    meter: &mut InvocationMeter,
) -> Vec<CaseOutcome> {
    // Project before appending each result: legacy refuses retain no extra
    // terminal attempts and allocate no origin inventory.
    solve_cases_projected(
        sources,
        case_limit,
        meter,
        None,
        Vec::with_capacity(sources.len()),
        |run, _| run.into_legacy(),
    )
}

pub(crate) fn solve_cases_projected<T>(
    sources: &[PrimitiveSource],
    case_limit: CaseLimit,
    meter: &mut InvocationMeter,
    mut recording: Option<BatchRecording<'_>>,
    mut out: Vec<T>,
    mut project: impl FnMut(CoreRun, Option<usize>) -> T,
) -> Vec<T> {
    let mut groups: Vec<(
        Vec<u8>,
        Result<Arc<GroupPrep>, (Refusal, Vec<BodyGeometry>)>,
        GroupCache,
    )> = Vec::new();
    for (position, source) in sources.iter().enumerate() {
        let before = meter.checked_charged();
        let early = if let Some(fault) = before.status().fault() {
            Some(UnresolvedReason::WorkAccounting { fault, prior: None })
        } else if meter.exhausted() {
            Some(UnresolvedReason::Budget(BudgetScope::Invocation))
        } else {
            None
        };
        if let Some(reason) = early {
            let run = CoreRun::idle(
                ExecutionOutcome::Unresolved {
                    reason,
                    attempts: Vec::new(),
                    geometry: Vec::new(),
                },
                before,
            );
            let id = recording
                .as_mut()
                .map(|r| r.finish(position, None, RunPhase::InvocationEntry, &run, None));
            out.push(project(run, id));
            continue;
        }
        let identity = source.stiffness_encoding();
        let index = match groups.iter().position(|g| g.0 == identity) {
            Some(k) => k,
            None => {
                let prepared = prepare_group(source).map(Arc::new);
                if let Some(recording) = recording.as_mut() {
                    recording.group(position, prepared.as_ref().err().map(|e| &e.0));
                }
                groups.push((identity, prepared, GroupCache::default()));
                groups.len() - 1
            }
        };
        let (_, group, cache) = &mut groups[index];
        let group = match group {
            Ok(g) => g.clone(),
            Err((refusal, geometry)) => {
                let run = CoreRun::idle(
                    ExecutionOutcome::Refused {
                        refusal: refusal.clone(),
                        attempts: Vec::new(),
                        geometry: geometry.clone(),
                    },
                    before,
                );
                let id = recording.as_mut().map(|r| {
                    r.finish(
                        position,
                        Some(index),
                        RunPhase::GroupPreparation,
                        &run,
                        None,
                    )
                });
                out.push(project(run, id));
                continue;
            }
        };
        let prep = match CasePrep::new(source.clone()) {
            Ok(p) => Arc::new(p),
            Err(e) => {
                let run = CoreRun::idle(
                    ExecutionOutcome::Refused {
                        refusal: Refusal::LedgerUnavailable(e),
                        attempts: Vec::new(),
                        geometry: group.geometry.clone(),
                    },
                    before,
                );
                let id = recording.as_mut().map(|r| {
                    r.finish(
                        position,
                        Some(index),
                        RunPhase::SourcePreparation,
                        &run,
                        None,
                    )
                });
                out.push(project(run, id));
                continue;
            }
        };
        let mut trace = recording.as_mut().map(|r| r.trace(index));
        let run = run_core_with_origins(prep, group, cache, case_limit, meter, trace.as_mut());
        let capture = trace.map(RunTrace::finish);
        let id = recording
            .as_mut()
            .map(|r| r.finish(position, Some(index), RunPhase::Schedule, &run, capture));
        out.push(project(run, id));
    }
    out
}

/// One case (a group of one; F2a API).
pub fn solve_case(
    source: PrimitiveSource,
    case_limit: CaseLimit,
    meter: &mut InvocationMeter,
) -> CaseOutcome {
    solve_cases(std::slice::from_ref(&source), case_limit, meter)
        .pop()
        .expect("one outcome per case")
}

// ------------------------------------------------------------ support for combinations

impl GroupCache {
    pub(crate) fn matches_origins(&self, slots: &SlotSnapshot) -> bool {
        let occupied = [
            self.s128.is_some(),
            self.s256.is_some(),
            self.s512.is_some(),
            self.s1024.is_some(),
            self.v256.is_some(),
            self.v512.is_some(),
            self.v1024.is_some(),
        ];
        occupied
            .iter()
            .zip(slots)
            .all(|(present, origin)| *present == origin.is_some())
    }
    /// The first cached build (or failure) per precision among the caches of
    /// solves that share one stiffness identity.
    pub(crate) fn merged<'a>(caches: impl IntoIterator<Item = &'a GroupCache>) -> Self {
        let mut out = GroupCache::default();
        for cache in caches {
            if out.s128.is_none() {
                out.s128 = cache.s128.clone();
            }
            if out.s256.is_none() {
                out.s256 = cache.s256.clone();
            }
            if out.s512.is_none() {
                out.s512 = cache.s512.clone();
            }
            if out.s1024.is_none() {
                out.s1024 = cache.s1024.clone();
            }
            if out.v256.is_none() {
                out.v256 = cache.v256.clone();
            }
            if out.v512.is_none() {
                out.v512 = cache.v512.clone();
            }
            if out.v1024.is_none() {
                out.v1024 = cache.v1024.clone();
            }
        }
        out
    }
}

/// A test-only seed of the final state (R7 §7's SEEDED-COMMON and
/// SEEDED-SOFT): added after the gate and before recovery, at every precision.
#[cfg(test)]
pub(crate) mod seed {
    use super::super::wide::multi::{SupportedWidth, WideContext};
    use super::super::wide::Wide;
    use super::AttemptStop;
    use std::cell::RefCell;

    thread_local! {
        static SEED: RefCell<Vec<(usize, f64)>> = const { RefCell::new(Vec::new()) };
    }

    /// Sets this thread's seeds (global DOF, value); empty clears them.
    pub(crate) fn set(values: Vec<(usize, f64)>) {
        SEED.with(|s| *s.borrow_mut() = values);
    }

    pub(crate) fn apply<const L: usize>(
        ctx: &mut WideContext<L>,
        u: &mut [Wide<L>],
    ) -> Result<(), AttemptStop>
    where
        Wide<L>: SupportedWidth,
    {
        SEED.with(|s| {
            for &(g, v) in s.borrow().iter() {
                u[g] = ctx.add(&u[g], &Wide::<L>::from_f64(v)?)?;
            }
            Ok(())
        })
    }
}

/// KF1's test hook: this thread's T for the trackers built on it (None: T =
/// `TRACKER_ROWS`); G follows as 8·T. Test builds only.
#[cfg(test)]
pub(crate) mod tracker_hook {
    use std::cell::Cell;

    thread_local! {
        static ROWS: Cell<Option<usize>> = const { Cell::new(None) };
    }

    pub(crate) fn set(rows: Option<usize>) {
        ROWS.with(|r| r.set(rows));
    }

    pub(crate) fn get() -> Option<usize> {
        ROWS.with(|r| r.get())
    }
}

#[cfg(test)]
#[path = "../../../tests/retained_k4/adaptive_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "../../../tests/retained_k4/kf1_tracker_tests.rs"]
mod kf1_tracker_tests;

#[cfg(test)]
#[path = "../../../tests/retained_k4/references_tests.rs"]
mod references_tests;

#[cfg(test)]
#[path = "../../../tests/retained_k4/classification_tests.rs"]
mod classification_tests;
#[cfg(test)]
#[path = "../../../tests/retained_k4/kf3_tests.rs"]
mod kf3_tests;
#[cfg(test)]
#[path = "../../../tests/retained_k4/method_tests.rs"]
mod method_tests;
#[cfg(test)]
#[path = "../../../tests/retained_k4/scale_tests.rs"]
mod scale_tests;

#[cfg(test)]
#[path = "../../../tests/retained_k4/publication_tests.rs"]
mod publication_tests;

// A byte is booked immediately before its actual comparison. Length or earlier
// precision refusal therefore does not fabricate a full-byte spent prefix.
fn bridge_bytes_equal(a: &[u8], b: &[u8], spent: &mut WorkTotal) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for (&x, &y) in a.iter().zip(b) {
        *spent = spent.add(WorkTotal::exact_count(1));
        if x != y {
            return false;
        }
    }
    true
}

// I42's native borrow is not a PP invocation/material/final-row binding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SourceBridgeViewIssue {
    Certificate(CertificateIssue),
    ForeignOwner,
    UnsupportedCombination,
    VerificationCache,
    Ordering,
    BodyBound,
    CountRange,
    Work(WorkFault),
}
impl From<CertificateIssue> for SourceBridgeViewIssue {
    fn from(e: CertificateIssue) -> Self {
        Self::Certificate(e)
    }
}
#[derive(Debug, Default)]
pub(crate) struct SourceBridgeViewWork {
    /// Named loop events and explicit source/ledger byte or key comparisons.
    /// Constraint/pattern library lookups and body-bound/theta scans remain
    /// unqualified auxiliary costs; this is not a complete facade visit ledger.
    pub(crate) visits: WorkTotal,
    pub(crate) f64_operations: WorkTotal,
}
impl SourceBridgeViewWork {
    fn visit(&mut self, count: usize) -> Result<(), SourceBridgeViewIssue> {
        let n = u64::try_from(count).map_err(|_| SourceBridgeViewIssue::CountRange)?;
        self.visits = self.visits.add(WorkTotal::exact_count(n));
        match self.visits.status().fault() {
            Some(f) => Err(SourceBridgeViewIssue::Work(f)),
            None => Ok(()),
        }
    }
}
#[derive(Debug)]
pub(crate) struct SourceBridgeViewSpent<'a> {
    pub(crate) result: Result<SourceBridgeView<'a>, SourceBridgeViewIssue>,
    pub(crate) work: SourceBridgeViewWork,
}
/// Rows, private radii, scales and maps always remain attached to this owner.
/// The only new persistent owner is the checked data flag buffer.
#[derive(Debug)]
pub(crate) struct SourceBridgeView<'a> {
    owner: &'a RetainedSolve,
    scales: &'a [i64],
    data: Vec<bool>,
}
impl<'a> SourceBridgeView<'a> {
    pub(crate) fn owner(&self) -> &'a RetainedSolve {
        self.owner
    }
    pub(crate) fn source(&self) -> &'a PrimitiveSource {
        self.owner.source()
    }
    pub(crate) fn group(&self) -> &'a GroupPrep {
        &self.owner.group
    }
    pub(crate) fn scales(&self) -> &'a [i64] {
        self.scales
    }
    pub(crate) fn data(&self) -> &[bool] {
        &self.data
    }
    pub(crate) fn data_capacity(&self) -> usize {
        self.data.capacity()
    }
    pub(crate) fn row(&self, index: usize) -> (&'a PublishedRow, Option<f64>) {
        let row = &self.owner.publication.rows[index];
        let bits = self.owner.publication_radius_bits[index];
        (
            row,
            (bits != ABSENT_RADIUS_BITS).then(|| f64::from_bits(bits)),
        )
    }
    pub(crate) fn bound(&self, body: u32) -> Option<f64> {
        self.owner
            .evidence
            .certified_bound
            .iter()
            .find(|v| v.0 == body)
            .map(|v| f64::from_bits(v.1))
    }
}
impl RetainedSolve {
    pub(crate) fn source_bridge_view<'a>(
        &'a self,
        source: &'a PrimitiveSource,
        identity: &[u8],
        precision: u32,
    ) -> SourceBridgeViewSpent<'a> {
        let mut work = SourceBridgeViewWork::default();
        let result = self.build_source_bridge_view(source, identity, precision, &mut work);
        SourceBridgeViewSpent { result, work }
    }
    fn build_source_bridge_view<'a>(
        &'a self,
        source: &'a PrimitiveSource,
        identity: &[u8],
        precision: u32,
        work: &mut SourceBridgeViewWork,
    ) -> Result<SourceBridgeView<'a>, SourceBridgeViewIssue> {
        if !std::ptr::eq(source, self.source()) {
            return Err(SourceBridgeViewIssue::ForeignOwner);
        }
        self.validate_publication_owner_spent(identity, precision, &mut work.visits)?;
        work.visit(0)?;
        if !self.prep.factors.is_empty() {
            return Err(SourceBridgeViewIssue::UnsupportedCombination);
        }
        let p = precision
            .checked_mul(2)
            .ok_or(SourceBridgeViewIssue::CountRange)?;
        if self.evidence.method != METHOD_TOKEN
            || self.evidence.verification_precision != p
            || !matches!(precision, 128 | 256 | 512)
            || self.state(precision).is_none()
        {
            return Err(SourceBridgeViewIssue::VerificationCache);
        }
        let (ledger_matches, spent) = self
            .prep
            .ledger
            .encoding_matches(&self.evidence.ledger_encoding);
        work.visits = work.visits.add(spent);
        work.visit(0)?;
        if !ledger_matches {
            return Err(CertificateIssue::PairIdentity.into());
        }
        let n = source.dof_count();
        let ordering = &self.group.ordering;
        let blocks = &self.group.blocks;
        let nf = ordering.free.len();
        if self.group.structure.pattern.dimension() != n
            || ordering.position.len() != n
            || ordering.order.len() != nf
            || ordering.rank.len() != nf
            || ordering.first.len() != nf
            || blocks.of.len() != nf
            || blocks.body.len() != blocks.len()
        {
            return Err(SourceBridgeViewIssue::Ordering);
        }
        for (g, &a) in ordering.position.iter().enumerate() {
            work.visit(1)?;
            if source.constraint(g).is_some() != (a == usize::MAX)
                || (a != usize::MAX && ordering.free.get(a) != Some(&g))
            {
                return Err(SourceBridgeViewIssue::Ordering);
            }
        }
        for (a, &g) in ordering.free.iter().enumerate() {
            work.visit(1)?;
            if g >= n
                || ordering.position[g] != a
                || (a > 0 && ordering.free[a - 1] >= g)
                || ordering.rank[a] >= nf
                || ordering.order[ordering.rank[a]] != a
                || blocks.of[a] as usize >= blocks.len()
                || blocks.body[blocks.of[a] as usize] != source.body_of_node((g / 6) as u32)
            {
                return Err(SourceBridgeViewIssue::Ordering);
            }
            for index in self.group.structure.pattern.row_range(g) {
                work.visit(1)?;
                let c = self.group.structure.pattern.column(index);
                if c >= n {
                    return Err(SourceBridgeViewIssue::Ordering);
                }
                let b = ordering.position[c];
                if b != usize::MAX && blocks.of[a] != blocks.of[b] {
                    return Err(SourceBridgeViewIssue::Ordering);
                }
            }
        }
        let mut positions = 0usize;
        for (b, list) in blocks.positions.iter().enumerate() {
            if list.is_empty() {
                return Err(SourceBridgeViewIssue::Ordering);
            }
            for (i, &a) in list.iter().enumerate() {
                work.visit(1)?;
                positions = positions
                    .checked_add(1)
                    .ok_or(SourceBridgeViewIssue::CountRange)?;
                if a >= nf || blocks.of[a] as usize != b || (i > 0 && list[i - 1] >= a) {
                    return Err(SourceBridgeViewIssue::Ordering);
                }
            }
        }
        if positions != nf {
            return Err(SourceBridgeViewIssue::Ordering);
        }
        // Check all potential member entries, including numerical zeros.
        for m in source.members() {
            for a in 0..12 {
                let g = if a < 6 { m.node_i } else { m.node_j } as usize * 6 + a % 6;
                for b in 0..12 {
                    work.visit(1)?;
                    let h = if b < 6 { m.node_i } else { m.node_j } as usize * 6 + b % 6;
                    if self.group.structure.pattern.find(g, h).is_none() {
                        return Err(SourceBridgeViewIssue::Ordering);
                    }
                }
            }
        }
        for (index, meta) in self.prep.layout.iter().copied().enumerate() {
            work.visit(1)?;
            let spent = self.publication_radius_checked_spent(index, meta);
            work.f64_operations = work
                .f64_operations
                .add(WorkTotal::exact_count(u64::from(spent.f64_operations)));
            // Collect before either exit. Original numeric cause takes precedence
            // over simultaneous accounting loss; both remain in the spent return.
            spent.result?;
            if let Some(fault) = work.f64_operations.status().fault() {
                return Err(SourceBridgeViewIssue::Work(fault));
            }
            if self.publication.rows[index]
                .value
                .value()
                .is_some_and(|x| !x.is_finite())
            {
                return Err(CertificateIssue::NonFinite.into());
            }
        }
        std::alloc::Layout::array::<bool>(n).map_err(|_| SourceBridgeViewIssue::CountRange)?;
        std::alloc::Layout::array::<bool>(blocks.len())
            .map_err(|_| SourceBridgeViewIssue::CountRange)?;
        let mut prescribed = vec![false; n];
        if self.prep.prescribed.len() != source.constraints().len() {
            return Err(CertificateIssue::PairIdentity.into());
        }
        for ((g, terms), c) in self.prep.prescribed.iter().zip(source.constraints()) {
            work.visit(1)?;
            if *g != c.dof.global()
                || terms.len() != 1
                || terms[0].0.to_bits() != 1f64.to_bits()
                || terms[0].1.to_bits() != c.value.to_bits()
            {
                return Err(CertificateIssue::PairIdentity.into());
            }
            prescribed[*g] = c.value != 0.0;
        }
        let mut data = vec![false; blocks.len()];
        macro_rules! checked_slot {
            ($shared:expr, $verify:expr, $state:expr) => {{
                let shared = $shared
                    .as_ref()
                    .and_then(|s| s.as_ref().ok())
                    .ok_or(SourceBridgeViewIssue::VerificationCache)?;
                let verify = $verify
                    .as_ref()
                    .and_then(|s| s.as_ref().ok())
                    .ok_or(SourceBridgeViewIssue::VerificationCache)?;
                let state = $state;
                if shared.p != p
                    || state.p != p
                    || state.u.len() != n
                    || shared.factor.scale().len() != nf
                    || shared.factor.first() != ordering.first
                    || verify.uc.len() != blocks.len()
                    || verify.abar.len() != self.group.structure.pattern.entry_count()
                {
                    return Err(SourceBridgeViewIssue::VerificationCache);
                }
                work.visits = work.visits.add(super::bound::fill_data_blocks(
                    blocks,
                    ordering,
                    &self.group.structure,
                    &self.prep.ledger,
                    &prescribed,
                    &state.u,
                    &mut data,
                ));
                shared.factor.scale()
            }};
        }
        let scales = match (p, self.state(p)) {
            (256, Some(PrecisionState::P256(s))) => {
                checked_slot!(self.cache.s256, self.cache.v256, s)
            }
            (512, Some(PrecisionState::P512(s))) => {
                checked_slot!(self.cache.s512, self.cache.v512, s)
            }
            (1024, Some(PrecisionState::P1024(s))) => {
                checked_slot!(self.cache.s1024, self.cache.v1024, s)
            }
            _ => return Err(SourceBridgeViewIssue::VerificationCache),
        };
        for (i, &(body, bits)) in self.evidence.certified_bound.iter().enumerate() {
            work.visit(1)?;
            let bound = f64::from_bits(bits);
            if body >= source.body_count()
                || !bound.is_finite()
                || bound <= 0.0
                || (i > 0 && self.evidence.certified_bound[i - 1].0 >= body)
            {
                return Err(SourceBridgeViewIssue::BodyBound);
            }
        }
        for (b, &has_data) in data.iter().enumerate() {
            work.visit(1)?;
            if has_data {
                let body = blocks.body[b];
                if !self.evidence.certified_bound.iter().any(|v| v.0 == body)
                    || !self
                        .evidence
                        .theta
                        .iter()
                        .any(|v| v.0 == body && v.1.is_finite() && v.1 >= 0.0 && v.1 <= 0.5)
                {
                    return Err(SourceBridgeViewIssue::BodyBound);
                }
            }
        }
        work.visit(0)?;
        if let Some(f) = work.f64_operations.status().fault() {
            return Err(SourceBridgeViewIssue::Work(f));
        }
        Ok(SourceBridgeView {
            owner: self,
            scales,
            data,
        })
    }
}

#[cfg(test)]
#[path = "../../../tests/retained_k4/source_bridge_tests.rs"]
mod source_bridge_tests;

/// Private trial correction only. The source residual, not factor accuracy,
/// establishes its usefulness. All arithmetic owners are collected on failure.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum SourceCorrectionError {
    Arithmetic(AttemptStop),
    Storage,
    CountRange,
    Identity,
}
impl From<AttemptStop> for SourceCorrectionError {
    fn from(e: AttemptStop) -> Self {
        Self::Arithmetic(e)
    }
}
impl From<WideError> for SourceCorrectionError {
    fn from(e: WideError) -> Self {
        Self::Arithmetic(e.into())
    }
}
#[derive(Debug, Default)]
pub(crate) struct SourceCorrectionWork {
    pub(crate) cast: AttemptWork,
    pub(crate) factor: AttemptWork,
    /// Entered cast elements and exact widening copy limbs. Core round/factor
    /// internal visits and allocation remain unqualified auxiliary work, not
    /// invented spent counts reconstructed from their safety upper bounds.
    pub(crate) visits: WorkTotal,
    pub(crate) calls: WorkTotal,
    pub(crate) rhs_capacity: usize,
    pub(crate) output_capacity: usize,
    pub(crate) converted_capacity: usize,
    #[cfg(all(test))]
    pub(crate) actual_rhs: Vec<Wide<16>>,
}
impl SourceCorrectionWork {
    pub(crate) fn status(&self) -> WorkStatus {
        self.cast
            .checked_lme()
            .status()
            .join(self.factor.checked_lme().status())
            .join(self.visits.status())
            .join(self.calls.status())
    }
    fn visit(&mut self, n: usize) -> Result<(), SourceCorrectionError> {
        let n = u64::try_from(n).map_err(|_| SourceCorrectionError::CountRange)?;
        self.visits = self.visits.add(WorkTotal::exact_count(n));
        self.status().fault().map_or(Ok(()), |f| {
            Err(SourceCorrectionError::Arithmetic(
                AttemptStop::WorkAccounting(f),
            ))
        })
    }
}
#[derive(Debug)]
pub(crate) struct SourceCorrectionSpent {
    result: Result<Vec<Wide<16>>, SourceCorrectionError>,
    pub(crate) work: SourceCorrectionWork,
}
impl SourceCorrectionSpent {
    pub(crate) fn into_parts(
        self,
    ) -> (
        Result<Vec<Wide<16>>, SourceCorrectionError>,
        SourceCorrectionWork,
    ) {
        let result = match self.result {
            Err(e) => Err(e),
            Ok(v) => match self.work.status().fault() {
                Some(f) => Err(SourceCorrectionError::Arithmetic(
                    AttemptStop::WorkAccounting(f),
                )),
                None => Ok(v),
            },
        };
        (result, self.work)
    }
}
fn source_correction_at<const L: usize>(
    factor: &RetainedFactor<L>,
    p: u32,
    midpoint: Vec<Wide<16>>,
    work: &mut SourceCorrectionWork,
) -> Result<Vec<Wide<16>>, SourceCorrectionError>
where
    Wide<L>: SupportedWidth,
{
    let n = midpoint.len();
    // These preflights do not qualify factor.rs's existing collect/vec allocations.
    std::alloc::Layout::array::<Wide<L>>(n).map_err(|_| SourceCorrectionError::CountRange)?;
    std::alloc::Layout::array::<Wide<16>>(n).map_err(|_| SourceCorrectionError::CountRange)?;
    let mut rhs = Vec::new();
    rhs.try_reserve_exact(n)
        .map_err(|_| SourceCorrectionError::Storage)?;
    work.rhs_capacity = rhs.capacity();
    let mut cast = WideContext::<L>::new(p)?;
    let cast_result = (|| -> Result<(), SourceCorrectionError> {
        for value in &midpoint {
            work.visit(1)?;
            rhs.push(cast.round(value)?); // explicit RN-even to matching P
        }
        Ok(())
    })();
    work.cast.record(&cast);
    drop(cast);
    drop(midpoint);
    cast_result?;
    work.visit(0)?;
    #[cfg(test)]
    {
        work.actual_rhs = rhs.iter().map(Wide::<L>::widen::<16>).collect();
    }
    let mut context = WideContext::<L>::new(p)?;
    let result = source_factor_once(factor, &rhs, &mut context, work);
    drop(context);
    drop(rhs); // RHS no longer overlaps conversion of the retained P output.
    let output = result?;
    let mut converted = Vec::new();
    converted
        .try_reserve_exact(output.len())
        .map_err(|_| SourceCorrectionError::Storage)?;
    work.converted_capacity = converted.capacity();
    for value in &output {
        work.visit(L)?; // actual copied source limbs; widening is exact
        converted.push(value.widen::<16>());
    }
    Ok(converted)
}
fn source_factor_once<const L: usize>(
    factor: &RetainedFactor<L>,
    rhs: &[Wide<L>],
    context: &mut WideContext<L>,
    work: &mut SourceCorrectionWork,
) -> Result<Vec<Wide<L>>, SourceCorrectionError>
where
    Wide<L>: SupportedWidth,
{
    work.visit(0)?;
    work.calls = work.calls.add(WorkTotal::exact_count(1));
    work.visit(0)?;
    let result = factor.solve_scaled(context, rhs);
    work.factor.record(context); // same producing context, before any failure exit
    let output = result?;
    work.output_capacity = output.capacity();
    work.visit(0)?;
    Ok(output)
}
impl SourceBridgeView<'_> {
    pub(crate) fn source_correction(&self, midpoint: Vec<Wide<16>>) -> SourceCorrectionSpent {
        let mut work = SourceCorrectionWork::default();
        let result = (|| {
            let nf = self.group().ordering.free.len();
            if midpoint.len() != nf {
                return Err(SourceCorrectionError::Identity);
            }
            let p = self.owner.evidence.verification_precision;
            macro_rules! apply {
                ($slot:expr) => {{
                    let shared = $slot
                        .as_ref()
                        .and_then(|v| v.as_ref().ok())
                        .ok_or(SourceCorrectionError::Identity)?;
                    if shared.p != p
                        || shared.factor.first() != self.group().ordering.first
                        || !std::ptr::eq(shared.factor.scale(), self.scales)
                    {
                        return Err(SourceCorrectionError::Identity);
                    }
                    source_correction_at(&shared.factor, p, midpoint, &mut work)
                }};
            }
            match p {
                256 => apply!(self.owner.cache.s256),
                512 => apply!(self.owner.cache.s512),
                1024 => apply!(self.owner.cache.s1024),
                _ => Err(SourceCorrectionError::Identity),
            }
        })();
        let result = match result {
            Err(e) => Err(e),
            Ok(v) => work.visit(0).map(|()| v),
        };
        SourceCorrectionSpent { result, work }
    }
}

#[cfg(test)]
#[path = "../../../tests/retained_k4/source_residual_tests.rs"]
mod source_residual_tests;
