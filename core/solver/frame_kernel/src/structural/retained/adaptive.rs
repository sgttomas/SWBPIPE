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
    assemble, form_directional, form_members, reduced_rhs, DirectionalBlock, MemberOperators,
    Structure,
};
use super::factor::{
    geometry_first, order_free, BodyGeometry, GeometryRefusal, Ordering, RetainedFactor,
};
use super::ledger::{LedgerRefusal, RetainedLedger};
use super::recover::{
    layout, publish_value, recover, state_encoding, Kind, QuantityId, QuantityMeta, Recovered,
};
use super::source::{put_u32, put_u64, Dof, PrimitiveSource};
use super::wide::multi::{AttemptWork, Binary64Outcome, SupportedWidth, WideContext};
use super::wide::{Wide, WideError};
use super::wide_sum::{ExactWideSum, SumRefusal, SumWork};
use crate::exact_sum::ExactAccumulator;
use crate::structural::StructuralError;
use std::cmp::Ordering as CmpOrdering;
use std::sync::Arc;

/// Proposed method token (a placeholder for ROOT, D1 §4.1).
pub(crate) const METHOD_TOKEN: &str = "contribution_preserving_multiprecision_v1";
/// Proposed policy (a placeholder for ROOT).
pub(crate) const POLICY: &str = "M03-INTEGRITY-MP-v1";
/// The label D1 §4.1.3 gives the published condition estimate.
pub(crate) const RCOND_LABEL: &str =
    "sensitivity to matrix-entry perturbation, not to authored parameters";
/// The solve precisions: candidates 128, 256, 512; the ceiling 1024.
pub(crate) const PRECISIONS: [u32; 4] = [128, 256, 512, 1024];
/// R = 2^-34.
pub(crate) const FLOOR_RATIO_BITS: u64 = 0x3DD0_0000_0000_0000;
/// k√2, the nearest double to √2 (≥ √2).
pub(crate) const K_SQRT2_BITS: u64 = 0x3FF6_A09E_667F_3BCD;
/// k_{2√2} = 2·k√2.
#[allow(dead_code)] // F2a API (per-member stress classification)
pub(crate) const K_TWO_SQRT2_BITS: u64 = 0x4006_A09E_667F_3BCD;

// ------------------------------------------------------------ budgets

/// The per-case work limit, in limb-multiply equivalents (required; no default).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CaseLimit(u64);

impl CaseLimit {
    #[allow(dead_code)] // F2a API (W1's caller is wired at F2a; ROOT's K4 ruling Q1)
    pub(crate) fn new(limb_multiply_equivalents: u64) -> Self {
        Self(limb_multiply_equivalents)
    }
    pub(crate) fn get(self) -> u64 {
        self.0
    }
}

/// The per-invocation work meter (required; no default).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct InvocationMeter {
    limit: u64,
    charged: u64,
}

impl InvocationMeter {
    #[allow(dead_code)] // F2a API
    pub(crate) fn new(limit: u64) -> Self {
        Self { limit, charged: 0 }
    }
    #[allow(dead_code)] // F2a API (evidence)
    pub(crate) fn charged(&self) -> u64 {
        self.charged
    }
    #[allow(dead_code)] // F2a API (evidence)
    pub(crate) fn limit(&self) -> u64 {
        self.limit
    }
    pub(crate) fn exhausted(&self) -> bool {
        self.charged >= self.limit
    }
    fn room(&self) -> u64 {
        self.limit.saturating_sub(self.charged)
    }
    fn charge(&mut self, amount: u64) {
        self.charged = self.charged.saturating_add(amount);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BudgetScope {
    Case,
    Invocation,
}

/// Why an attempt stopped.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum AttemptStop {
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
}

impl From<SumRefusal> for AttemptStop {
    fn from(refusal: SumRefusal) -> Self {
        match refusal {
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

fn lme<const L: usize>(ctx: &WideContext<L>) -> u64
where
    Wide<L>: SupportedWidth,
{
    ctx.work().limb_multiply_equivalents(L)
}

/// A budget check point within one stage: `base` is the attempt's work in the
/// contexts the stage does not use; the stage's own context and accumulator
/// are read at each check.
#[derive(Debug, Clone, Copy)]
pub(crate) struct StageGuard {
    base: u64,
    case_room: u64,
    invocation_room: u64,
}

impl StageGuard {
    /// A guard that never stops (tests).
    #[cfg(test)]
    pub(crate) fn unlimited() -> Self {
        Self {
            base: 0,
            case_room: u64::MAX,
            invocation_room: u64::MAX,
        }
    }

    fn with_base(self, base: u64) -> Self {
        Self { base, ..self }
    }

    fn test(&self, used: u64) -> Result<(), AttemptStop> {
        if used > self.case_room {
            Err(AttemptStop::Budget(BudgetScope::Case))
        } else if used > self.invocation_room {
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
        self.test(
            self.base
                .saturating_add(lme(ctx))
                .saturating_add(sum.work().limb_multiply_equivalents()),
        )
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
pub(crate) fn body_extent(coordinates: &[[f64; 3]]) -> f64 {
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
pub(crate) fn coupled_scales(s: [f64; 4], extent: f64) -> [f64; 4] {
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
#[allow(dead_code)] // F2a API (per-member stress classification)
pub(crate) fn stress_scale(fo: f64, mo: f64, area: f64, modulus: f64, k: f64) -> f64 {
    fo / area + k * (mo / modulus)
}

/// Item 7: k_i = fl↑(k√2·i): the nearest, then the next up when the nearest is
/// below the exact product (decided with an exact product).
#[allow(dead_code)] // F2a API (per-member stress classification)
pub(crate) fn intensified_k(i: f64) -> f64 {
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
        Some(sum.signum() > 0)
    })()
    .unwrap_or(false);
    if exact_above {
        next_up(nearest)
    } else {
        nearest
    }
}

/// t = fl(R·S*).
pub(crate) fn threshold(s_star: f64) -> f64 {
    f64::from_bits(FLOOR_RATIO_BITS) * s_star
}

/// b = fl↑(2^-64·S*), for S* ≥ 0: exact whenever 2^-64·S* is representable,
/// at least 2^-1074 for any 0 < S*, and 0 only at S* = 0.
pub(crate) fn absolute_bound(s_star: f64) -> f64 {
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
pub(crate) enum RowClass {
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

/// D1 §4.1.6 item 1 on the published value: `absolute_verified` iff
/// |q| < fl(R·S\*), and always when S\* < 2^-988.
pub(crate) fn classify(value: f64, s_star: f64) -> RowClass {
    let small = f64::from_bits(0x0230_0000_0000_0000); // 2^-988
    if s_star < small || value.abs() < threshold(s_star) {
        RowClass::AbsoluteVerified {
            bound_bits: absolute_bound(s_star).to_bits(),
        }
    } else {
        RowClass::RelativeVerified
    }
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
    Ok(t.signum() >= 0)
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
    let mut n = num.clone();
    if n.is_zero() {
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
                Ok(t.signum() < 0)
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

/// The exact directed extreme of a stream of ratios num/den (both exact): the
/// rows whose 64-bit approximation lies within `WINDOW_ULPS` of the running
/// approximate extreme are kept (compared as bit patterns, integers only) and
/// evaluated exactly at the end, so the result is the directed rounding of the
/// true extreme.
pub(crate) struct ExtremeTracker {
    direction: Direction,
    best: Option<u64>,
    kept: Vec<(ExactWideSum, ExactWideSum, u64)>,
}

impl ExtremeTracker {
    pub(crate) fn new(direction: Direction) -> Self {
        Self {
            direction,
            best: None,
            kept: Vec::new(),
        }
    }

    pub(crate) fn offer(
        &mut self,
        ctx64: &mut WideContext<4>,
        num: ExactWideSum,
        den: ExactWideSum,
    ) -> Result<(), AttemptStop> {
        let approx = approximate_ratio(ctx64, &num, &den)?.to_bits();
        let better = match (self.best, self.direction) {
            (None, _) => true,
            (Some(b), Direction::Up) => approx > b,
            (Some(b), Direction::Down) => approx < b,
        };
        if better {
            self.best = Some(approx);
            self.kept.retain(|k| k.2.abs_diff(approx) <= WINDOW_ULPS);
        }
        if self.best.is_some_and(|b| approx.abs_diff(b) <= WINDOW_ULPS) {
            self.kept.push((num, den, approx));
        }
        Ok(())
    }

    pub(crate) fn finish(self, ctx16: &mut WideContext<16>) -> Result<Option<f64>, AttemptStop> {
        let mut best: Option<f64> = None;
        for (num, den, _) in &self.kept {
            let exact = directed_ratio(ctx16, num, den, self.direction)?;
            best = Some(match (best, self.direction) {
                (None, _) => exact,
                (Some(b), Direction::Up) => b.max(exact),
                (Some(b), Direction::Down) => b.min(exact),
            });
        }
        Ok(best)
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
    pub(crate) fn combination(operands: &[(f64, &CasePrep)]) -> Result<Self, LedgerRefusal> {
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
}

/// Work of the shared stages at one precision.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct StageWork {
    pub(crate) formation: u64,
    pub(crate) assembly: u64,
    pub(crate) residual_formation: u64,
    pub(crate) factor: u64,
    pub(crate) condition: u64,
    pub(crate) rhs: u64,
    pub(crate) solve: u64,
    pub(crate) refinement: u64,
    pub(crate) recovery: u64,
    pub(crate) stop_rule: u64,
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
    pub(crate) factor: RetainedFactor<L>,
    pub(crate) rcond: f64,
    pub(crate) pivot_margin_min: f64,
    pub(crate) work: AttemptWork,
    pub(crate) sum_work: SumWork,
    pub(crate) stages: StageWork,
    pub(crate) total: u64,
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
}

/// A failed build or solve, with the work it spent.
#[derive(Debug)]
struct Spent<T> {
    result: Result<T, AttemptStop>,
    work: AttemptWork,
    sum_work: SumWork,
    stages: StageWork,
    total: u64,
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
    let mut run = || -> Result<Shared<L, R>, AttemptStop> {
        let t0 = lme(&ctx) + sum.work().limb_multiply_equivalents();
        let g = guard.with_base(0);
        let members = form_members(&mut ctx, &mut sum, &g, source)?;
        let directional = form_directional(&mut ctx, &mut sum, source)?;
        let t1 = lme(&ctx) + sum.work().limb_multiply_equivalents();
        stages.formation = t1 - t0;
        let k = assemble(
            &mut ctx,
            &mut sum,
            &g,
            source,
            &group.structure,
            &members,
            &directional,
        )?;
        let t2 = lme(&ctx) + sum.work().limb_multiply_equivalents();
        stages.assembly = t2 - t1;
        g.check(&ctx, &sum)?;
        // The residual system re-formed at q (the ceiling: K itself, widened).
        let k_q: Vec<Wide<R>> = if q == p {
            k.iter().map(|w| w.widen::<R>()).collect()
        } else {
            let gq = guard.with_base(lme(&ctx));
            let members_q = form_members(&mut ctx_q, &mut sum, &gq, source)?;
            let directional_q = form_directional(&mut ctx_q, &mut sum, source)?;
            assemble(
                &mut ctx_q,
                &mut sum,
                &gq,
                source,
                &group.structure,
                &members_q,
                &directional_q,
            )?
        };
        let t3 = lme(&ctx) + lme(&ctx_q) + sum.work().limb_multiply_equivalents();
        stages.residual_formation = t3 - t2;
        let gp = guard.with_base(lme(&ctx_q));
        let factor = super::factor::factor(
            &mut ctx,
            &mut sum,
            &gp,
            &group.structure,
            &k,
            &group.ordering,
        )?;
        let t4 = lme(&ctx) + lme(&ctx_q) + sum.work().limb_multiply_equivalents();
        stages.factor = t4 - t3;
        let rcond = factor.condition(&mut ctx, &mut sum, &group.structure, &k, &group.ordering)?;
        gp.check(&ctx, &sum)?;
        let rcond_value = publish_value(&rcond).value().unwrap_or(0.0);
        // The pivot margin minimum, rounded downward.
        let mut tracker = ExtremeTracker::new(Direction::Down);
        for screen in &factor.screens {
            let mut num = ExactWideSum::new();
            num.add_wide_scaled(&screen.pivot, false, 1, i64::from(p))?;
            num.add_wide_scaled(&screen.pivot, true, screen.operations, 0)?;
            let mut den = ExactWideSum::new();
            den.add_wide_scaled(&screen.scale, false, 64 * screen.operations, 0)?;
            if den.is_zero() {
                continue;
            }
            tracker.offer(&mut ctx64, num, den)?;
        }
        let margin = tracker.finish(&mut ctx16)?.unwrap_or(f64::INFINITY);
        let t5 = lme(&ctx)
            + lme(&ctx_q)
            + sum.work().limb_multiply_equivalents()
            + lme(&ctx16)
            + lme(&ctx64);
        stages.condition = t5 - t4;
        Ok(Shared {
            p,
            q,
            members,
            directional,
            k,
            k_q,
            factor,
            rcond: rcond_value,
            pivot_margin_min: margin,
            work: AttemptWork::default(),
            sum_work: SumWork::default(),
            stages: StageWork::default(),
            total: 0,
        })
    };
    let result = run();
    let mut work = AttemptWork::default();
    work.record(&ctx);
    work.record(&ctx_q);
    work.record(&ctx16);
    work.record(&ctx64);
    let sum_work = sum.work();
    let total = work.limb_multiply_equivalents() + sum_work.limb_multiply_equivalents();
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
    sum: &mut ExactWideSum,
    p: u32,
    structure: &Structure,
    k_q: &[Wide<R>],
    ledger: &RetainedLedger,
    free: &[usize],
    u: &[Wide<L>],
    tracker: &mut ExtremeTracker,
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
            count += 1;
            r.add_product(ctx_q, kij, &uj, true)?;
            let negative = kij.is_sign_negative() != uj.is_sign_negative();
            d.add_product(ctx_q, kij, &uj, negative)?;
        }
        let m = 2 * count + 2;
        let mut absolute = r.clone();
        absolute.make_absolute();
        // num = |r|·(2^p − m), den = 64·m·d.
        let mut num = ExactWideSum::new();
        num.add_scaled(&absolute, false, 1, i64::from(p))?;
        num.add_scaled(&absolute, true, m, 0)?;
        let mut den = ExactWideSum::new();
        den.add_scaled(&d, false, 64 * m, 0)?;
        // Gate: den − num ≥ 0.
        sum.clear();
        sum.add_scaled(&den, false, 1, 0)?;
        sum.add_scaled(&num, true, 1, 0)?;
        let passes = sum.signum() >= 0;
        let ratio = approximate_ratio(ctx64, &num, &den)?;
        let rounded = r.round(ctx)?;
        tracker.offer(ctx64, num, den)?;
        rows.push((passes, ratio, rounded));
    }
    Ok(rows)
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
    let source = &prep.source;
    let free = &group.ordering.free;
    let mut run = || -> Result<Solved<L>, AttemptStop> {
        let total = |ctx: &WideContext<L>,
                     ctx_q: &WideContext<R>,
                     ctx16: &WideContext<16>,
                     ctx64: &WideContext<4>,
                     sum: &ExactWideSum| {
            lme(ctx) + lme(ctx_q) + lme(ctx16) + lme(ctx64) + sum.work().limb_multiply_equivalents()
        };
        let t0 = total(&ctx, &ctx_q, &ctx16, &ctx64, &sum);
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
        let t1 = total(&ctx, &ctx_q, &ctx16, &ctx64, &sum);
        stages.rhs = t1 - t0;
        let mut u_free = shared.factor.solve(&mut ctx, &rhs)?;
        guard.test(total(&ctx, &ctx_q, &ctx16, &ctx64, &sum))?;
        let t2 = total(&ctx, &ctx_q, &ctx16, &ctx64, &sum);
        stages.solve = t2 - t1;
        let mut corrections = 0u8;
        let mut prior = f64::INFINITY;
        let residual_worst;
        loop {
            for (a, &g) in free.iter().enumerate() {
                u[g] = u_free[a];
            }
            let mut tracker = ExtremeTracker::new(Direction::Up);
            let rows = residual_rows(
                &mut ctx,
                &mut ctx_q,
                &mut ctx64,
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
                return Err(AttemptStop::ResidualGate {
                    global_dof: free[worst_row],
                });
            }
            prior = worst;
            let correction: Vec<Wide<L>> = rows.iter().map(|row| row.2).collect();
            let delta = shared.factor.solve(&mut ctx, &correction)?;
            for (v, d) in u_free.iter_mut().zip(&delta) {
                *v = ctx.add(v, d)?;
            }
            corrections += 1;
        }
        let t3 = total(&ctx, &ctx_q, &ctx16, &ctx64, &sum);
        stages.refinement = t3 - t2;
        let base = t3 - lme(&ctx) - sum.work().limb_multiply_equivalents();
        let recovered = recover(
            &mut ctx,
            &mut sum,
            &guard.with_base(guard.base + base),
            source,
            &prep.layout,
            &group.structure,
            &shared.k,
            &shared.members,
            &shared.directional,
            &prep.ledger,
            &u,
        )?;
        let t4 = total(&ctx, &ctx_q, &ctx16, &ctx64, &sum);
        stages.recovery = t4 - t3;
        Ok(Solved {
            p,
            u,
            recovered,
            corrections,
            residual_worst,
        })
    };
    let result = run();
    let mut work = AttemptWork::default();
    work.record(&ctx);
    work.record(&ctx_q);
    work.record(&ctx16);
    work.record(&ctx64);
    let sum_work = sum.work();
    let total = work.limb_multiply_equivalents() + sum_work.limb_multiply_equivalents();
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
    pub(crate) first_failure: Option<usize>,
    /// Per (body, kind): the worst normalized disagreement, rounded upward
    /// (computed for an accepted comparison only).
    pub(crate) summary: Vec<(u32, Kind, f64)>,
    pub(crate) work: AttemptWork,
    pub(crate) sum_work: SumWork,
    pub(crate) total: u64,
}

/// S\* per body and kind at the verification precision (item 6's coupling,
/// rounded once at 2p), from the rows that are not input-derived.
fn scales_at<const M: usize>(
    ctx: &mut WideContext<M>,
    layout: &[QuantityMeta],
    values: &[Wide<M>],
    extents: &[f64],
) -> Result<Vec<[Wide<M>; 4]>, AttemptStop>
where
    Wide<M>: SupportedWidth,
{
    let zero = Wide::<M>::ZERO;
    let mut s = vec![[zero; 4]; extents.len()];
    for (meta, v) in layout.iter().zip(values) {
        if meta.input_derived {
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
/// to the candidate attempt, with budget checks.
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
    let mut ctx = WideContext::<M>::new(verification_precision).expect("supported precision");
    let mut ctx16 = WideContext::<16>::new(1024).expect("supported precision");
    let mut ctx64 = WideContext::<4>::new(64).expect("supported precision");
    let mut sum = ExactWideSum::new();
    let mut first_failure = None;
    let mut summary = Vec::new();
    let mut run = || -> Result<bool, AttemptStop> {
        let spent = |ctx: &WideContext<M>,
                     ctx16: &WideContext<16>,
                     ctx64: &WideContext<4>,
                     sum: &ExactWideSum| {
            lme(ctx) + lme(ctx16) + lme(ctx64) + sum.work().limb_multiply_equivalents()
        };
        let scales = scales_at(&mut ctx, layout, verification, extents)?;
        let mut trackers: std::collections::BTreeMap<(u32, Kind), ExtremeTracker> =
            std::collections::BTreeMap::new();
        for (index, meta) in layout.iter().enumerate() {
            let q2 = &verification[index];
            let s_star = &scales[meta.body as usize][meta.kind.index()];
            let magnitude = if q2.abs().cmp_value(s_star) == CmpOrdering::Greater {
                q2.abs()
            } else {
                *s_star
            };
            let mut difference = ExactWideSum::new();
            difference.add_wide(&candidate[index], false)?;
            difference.add_wide(q2, true)?;
            difference.make_absolute();
            sum.clear();
            sum.add_wide_scaled(&magnitude, false, 1, -64)?;
            sum.add_scaled(&difference, true, 1, 0)?;
            if sum.signum() < 0 {
                first_failure = Some(index);
                return Ok(false);
            }
            if !magnitude.is_zero() {
                let mut den = ExactWideSum::new();
                den.add_wide(&magnitude, false)?;
                trackers
                    .entry((meta.body, meta.kind))
                    .or_insert_with(|| ExtremeTracker::new(Direction::Up))
                    .offer(&mut ctx64, difference, den)?;
            }
            if index % 64 == 63 {
                guard.test(spent(&ctx, &ctx16, &ctx64, &sum))?;
            }
        }
        for ((body, kind), tracker) in trackers {
            let worst = tracker.finish(&mut ctx16)?.unwrap_or(0.0);
            summary.push((body, kind, worst));
        }
        guard.test(spent(&ctx, &ctx16, &ctx64, &sum))?;
        Ok(true)
    };
    let result = run();
    let mut work = AttemptWork::default();
    work.record(&ctx);
    work.record(&ctx16);
    work.record(&ctx64);
    let sum_work = sum.work();
    let total = work.limb_multiply_equivalents() + sum_work.limb_multiply_equivalents();
    StopDecision {
        result,
        first_failure,
        summary,
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

/// The stop rule between a candidate and its verification (2p).
fn compare_states(
    layout: &[QuantityMeta],
    extents: &[f64],
    candidate: &PrecisionState,
    verification: &PrecisionState,
    guard: StageGuard,
) -> StopDecision {
    match (candidate, verification) {
        (PrecisionState::P128(a), PrecisionState::P256(b)) => stop_rule(
            layout,
            extents,
            &a.recovered.values,
            &b.recovered.values,
            256,
            guard,
        ),
        (PrecisionState::P256(a), PrecisionState::P512(b)) => stop_rule(
            layout,
            extents,
            &a.recovered.values,
            &b.recovered.values,
            512,
            guard,
        ),
        (PrecisionState::P512(a), PrecisionState::P1024(b)) => stop_rule(
            layout,
            extents,
            &a.recovered.values,
            &b.recovered.values,
            1024,
            guard,
        ),
        _ => unreachable!("the schedule compares p with 2p"),
    }
}

// ------------------------------------------------------------ attempts and evidence

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AttemptRole {
    Candidate,
    Verification,
    VerificationThenCandidate,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum AttemptReason {
    Stop(AttemptStop),
    StopRule {
        quantity: QuantityId,
        body: u32,
        kind: Kind,
    },
    VerificationFailed,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum AttemptOutcome {
    Accepted,
    Verified,
    Rejected(AttemptReason),
    Failed(AttemptReason),
    /// Solved as a verification that was not needed as a candidate.
    Solved,
}

/// Deterministic storage counts (not measurements).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct StorageCounts {
    pub(crate) pattern_entries: usize,
    pub(crate) profile_entries: usize,
    pub(crate) limbs_per_entry: usize,
}

/// One solve's record (D1 §5 item 1: "the attempts list (p, outcome, reason,
/// work)").
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct AttemptRecord {
    pub(crate) precision: u32,
    pub(crate) role: AttemptRole,
    pub(crate) outcome: AttemptOutcome,
    /// p + 64, or p at the ceiling (ROOT's Q4).
    pub(crate) residual_basis: u32,
    pub(crate) corrections: u8,
    /// fl↓ of the minimum d_i/(64·γ_p(m_i)·c_i).
    pub(crate) pivot_margin_min: Option<f64>,
    /// rcond at p (nearest; model information).
    pub(crate) rcond: Option<f64>,
    /// fl↑ of the worst |r_i|/(64·γ_p(m_i)·d_i).
    pub(crate) residual_worst: Option<f64>,
    /// This case's own contexts (each recorded once).
    pub(crate) work: AttemptWork,
    pub(crate) k4_work: SumWork,
    pub(crate) stages: StageWork,
    /// The shared stages' work at this precision (formation, assembly, residual
    /// formation, factor, condition), counted in full against the case limit.
    pub(crate) shared_work: u64,
    pub(crate) shared_stages: StageWork,
    /// Whether this attempt built the shared stages (so charged them to the
    /// invocation).
    pub(crate) shared_built_here: bool,
    /// The stop-rule work charged to this attempt as a candidate (a part of
    /// `work` and `k4_work`, which hold every context and sum it charged).
    pub(crate) stop_rule_work: u64,
    pub(crate) storage: StorageCounts,
}

/// A refusal: no rows, no escalation.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Refusal {
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
pub(crate) enum UnresolvedReason {
    /// "At the ceiling … the case is unresolved."
    Ceiling,
    Budget(BudgetScope),
    ExactSumSpan,
    ExponentRange,
    ZeroDiagonal {
        global_dof: usize,
    },
    Arithmetic(WideError),
}

/// A published row and its class.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PublishedRow {
    pub(crate) id: QuantityId,
    pub(crate) kind: Kind,
    pub(crate) body: u32,
    pub(crate) value: Binary64Outcome,
    pub(crate) class: RowClass,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Publication {
    pub(crate) rows: Vec<PublishedRow>,
    /// S\* per (body, kind) from the published values, as bits.
    pub(crate) body_scales: Vec<(u32, Kind, u64)>,
}

/// The classification of published rows (items 1, 2a, 4–6; O9).
pub(crate) fn classify_rows(
    layout: &[QuantityMeta],
    values: &[Binary64Outcome],
    extents: &[f64],
) -> Publication {
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
        .map(|(&s, &e)| coupled_scales(s, e))
        .collect();
    let rows = layout
        .iter()
        .zip(values)
        .map(|(meta, v)| {
            let class = if meta.input_derived {
                RowClass::InputDerived
            } else if let Some(x) = v.value() {
                classify(x, scales[meta.body as usize][meta.kind.index()])
            } else {
                RowClass::Unpublishable
            };
            PublishedRow {
                id: meta.id,
                kind: meta.kind,
                body: meta.body,
                value: *v,
                class,
            }
        })
        .collect();
    let body_scales = scales
        .iter()
        .enumerate()
        .flat_map(|(b, s)| {
            Kind::ALL
                .iter()
                .map(move |&k| (b as u32, k, s[k.index()].to_bits()))
        })
        .collect();
    Publication { rows, body_scales }
}

/// The evidence F2a's receipt needs (D1 §5 item 1), as kernel types.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RetainedEvidence {
    pub(crate) method: &'static str,
    pub(crate) policy: &'static str,
    pub(crate) attempts: Vec<AttemptRecord>,
    pub(crate) selected_precision: u32,
    pub(crate) verification_precision: u32,
    pub(crate) stop_rule: Vec<(u32, Kind, f64)>,
    pub(crate) floor_ratio_bits: u64,
    pub(crate) body_scales: Vec<(u32, Kind, u64)>,
    pub(crate) input_derived_dofs: Vec<Dof>,
    pub(crate) absolute_verified: Vec<(QuantityId, u64)>,
    pub(crate) not_covered: Vec<QuantityId>,
    pub(crate) unpublishable: Vec<(QuantityId, Binary64Outcome)>,
    pub(crate) pivot_margin_min: f64,
    pub(crate) rcond: f64,
    pub(crate) rcond_label: &'static str,
    pub(crate) residual_worst: f64,
    pub(crate) corrections: u8,
    pub(crate) geometry: Vec<BodyGeometry>,
    pub(crate) source_encoding: Vec<u8>,
    pub(crate) ledger_encoding: Vec<u8>,
    pub(crate) retained_state_encoding: Vec<u8>,
}

/// A selected case: bound to its source and precision (D1 §4.1.1).
#[derive(Debug, Clone)]
pub(crate) struct RetainedSolve {
    pub(crate) prep: Arc<CasePrep>,
    pub(crate) group: Arc<GroupPrep>,
    /// The group's shared stages as this solve left them (a combination of
    /// this solve reuses them: ROOT's ruling on I12's F-1).
    pub(crate) cache: GroupCache,
    pub(crate) states: Vec<PrecisionState>,
    selected: u32,
    evidence: RetainedEvidence,
    publication: Publication,
}

impl RetainedSolve {
    /// "RetainedSolve::publish() rounds each quantity once."
    #[allow(dead_code)] // F2a API
    pub(crate) fn publish(&self) -> &Publication {
        &self.publication
    }
    #[allow(dead_code)] // F2a API
    pub(crate) fn evidence(&self) -> &RetainedEvidence {
        &self.evidence
    }
    #[allow(dead_code)] // F2a API
    pub(crate) fn source(&self) -> &PrimitiveSource {
        &self.prep.source
    }
    #[allow(dead_code)] // F2a API
    pub(crate) fn selected_precision(&self) -> u32 {
        self.selected
    }
    #[allow(dead_code)] // F2a API
    pub(crate) fn state(&self, precision: u32) -> Option<&PrecisionState> {
        self.states.iter().find(|s| s.precision() == precision)
    }
}

/// A case's outcome.
#[allow(dead_code)] // F2a API (F2a reads the outcome's fields)
#[derive(Debug, Clone)]
pub(crate) enum CaseOutcome {
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
type Slot<S> = Option<Result<Arc<S>, (AttemptStop, u64, StageWork)>>;

/// The shared stages of one stiffness identity, per precision.
#[derive(Debug, Default, Clone)]
pub(crate) struct GroupCache {
    s128: Slot<Shared<4, 4>>,
    s256: Slot<Shared<4, 8>>,
    s512: Slot<Shared<8, 16>>,
    s1024: Slot<Shared<16, 16>>,
}

/// The work already counted against a case.
struct CaseBudget {
    limit: u64,
    used: u64,
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
) -> (Result<Arc<Shared<L, R>>, AttemptStop>, u64, bool, StageWork)
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
) -> (Result<PrecisionState, AttemptStop>, AttemptRecord) {
    let case_room = budget.limit.saturating_sub(budget.used);
    let invocation_room = meter.room();
    let guard = StageGuard {
        base: 0,
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
    };
    macro_rules! run {
        ($slot:expr, $L:literal, $R:literal, $q:expr, $variant:ident) => {{
            let (shared, shared_total, built, shared_stages) =
                obtain::<$L, $R>($slot, p, $q, &prep.source, group, guard);
            record.shared_work = shared_total;
            record.shared_built_here = built;
            record.shared_stages = shared_stages;
            // The shared work counts in full against this case; against the
            // invocation only when built here.
            let invocation_spent = if built { shared_total } else { 0 };
            budget.used = budget.used.saturating_add(shared_total);
            meter.charge(invocation_spent);
            match shared {
                Err(stop) => (Err(stop), record),
                Ok(_) if shared_total > case_room => {
                    (Err(AttemptStop::Budget(BudgetScope::Case)), record)
                }
                Ok(shared) => {
                    record.pivot_margin_min = Some(shared.pivot_margin_min);
                    record.rcond = Some(shared.rcond);
                    let own_guard = StageGuard {
                        base: 0,
                        case_room: case_room - shared_total,
                        invocation_room: invocation_room.saturating_sub(invocation_spent),
                    };
                    let spent = solve_case_at::<$L, $R>(&shared, prep, group, own_guard);
                    record.work = spent.work;
                    record.k4_work = spent.sum_work;
                    record.stages = spent.stages;
                    budget.used = budget.used.saturating_add(spent.total);
                    meter.charge(spent.total);
                    match spent.result {
                        Ok(solved) => {
                            record.corrections = solved.corrections;
                            record.residual_worst = Some(solved.residual_worst);
                            (Ok(PrecisionState::$variant(Arc::new(solved))), record)
                        }
                        Err(stop) => (Err(stop), record),
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

fn terminal(stop: &AttemptStop) -> Result<UnresolvedReason, Refusal> {
    match stop {
        AttemptStop::Budget(scope) => Ok(UnresolvedReason::Budget(*scope)),
        AttemptStop::Span => Ok(UnresolvedReason::ExactSumSpan),
        AttemptStop::Exponent => Ok(UnresolvedReason::ExponentRange),
        AttemptStop::ZeroDiagonal { global_dof } => Ok(UnresolvedReason::ZeroDiagonal {
            global_dof: *global_dof,
        }),
        AttemptStop::Arithmetic(e) => Ok(UnresolvedReason::Arithmetic(*e)),
        AttemptStop::NegativeEnergy { i, j } => Err(Refusal::NegativeEnergy { i: *i, j: *j }),
        AttemptStop::Structure => Err(Refusal::Structure),
        other => unreachable!("escalating stop {other:?} is not terminal"),
    }
}

/// The schedule of one case (module documentation).
pub(crate) fn run_schedule(
    prep: Arc<CasePrep>,
    group: Arc<GroupPrep>,
    cache: &mut GroupCache,
    case_limit: CaseLimit,
    meter: &mut InvocationMeter,
) -> CaseOutcome {
    let geometry = group.geometry.clone();
    let mut budget = CaseBudget {
        limit: case_limit.get(),
        used: 0,
    };
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
                let (result, record) = solve_precision(p, &prep, &group, cache, &mut budget, meter);
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
        let (result, mut record) =
            solve_precision(verification_p, &prep, &group, cache, &mut budget, meter);
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
                if stop.escalates() {
                    // The failed verification cannot be the next candidate.
                    c += 2;
                    continue;
                }
                return finish_terminal(&stop, attempts, geometry);
            }
        };
        let guard = StageGuard {
            base: 0,
            case_room: budget.limit.saturating_sub(budget.used),
            invocation_room: meter.room(),
        };
        let decision = compare_states(
            &prep.layout,
            &prep.extents,
            &candidate,
            &verification,
            guard,
        );
        {
            let record = &mut attempts[candidate_index];
            record.stop_rule_work += decision.total;
            record.stages.stop_rule += decision.total;
            record.work.merge(&decision.work);
            record.k4_work.merge(&decision.sum_work);
        }
        budget.used = budget.used.saturating_add(decision.total);
        meter.charge(decision.total);
        match decision.result {
            Err(stop) => {
                attempts[candidate_index].outcome =
                    AttemptOutcome::Failed(AttemptReason::Stop(stop.clone()));
                return finish_terminal(&stop, attempts, geometry);
            }
            Ok(true) => {
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
                    decision.summary,
                    geometry,
                );
            }
            Ok(false) => {
                let meta = decision
                    .first_failure
                    .and_then(|i| prep.layout.get(i))
                    .copied();
                attempts[candidate_index].outcome = AttemptOutcome::Rejected(match meta {
                    Some(meta) => AttemptReason::StopRule {
                        quantity: meta.id,
                        body: meta.body,
                        kind: meta.kind,
                    },
                    None => AttemptReason::VerificationFailed,
                });
                if c + 1 < 3 {
                    attempts[v_index].role = AttemptRole::VerificationThenCandidate;
                    pending = Some((verification, v_index));
                }
                c += 1;
            }
        }
    }
    CaseOutcome::Unresolved {
        reason: UnresolvedReason::Ceiling,
        attempts,
        geometry,
    }
}

fn finish_terminal(
    stop: &AttemptStop,
    attempts: Vec<AttemptRecord>,
    geometry: Vec<BodyGeometry>,
) -> CaseOutcome {
    match terminal(stop) {
        Ok(reason) => CaseOutcome::Unresolved {
            reason,
            attempts,
            geometry,
        },
        Err(refusal) => CaseOutcome::Refused { refusal, geometry },
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
    attempts: Vec<AttemptRecord>,
    summary: Vec<(u32, Kind, f64)>,
    geometry: Vec<BodyGeometry>,
) -> CaseOutcome {
    let mut values = selected.published();
    prep.publish_prescribed(&mut values);
    let publication = classify_rows(&prep.layout, &values, &prep.extents);
    let selected_record = attempts
        .iter()
        .find(|a| a.precision == selected.precision() && a.outcome == AttemptOutcome::Accepted)
        .cloned();
    let evidence = RetainedEvidence {
        method: METHOD_TOKEN,
        policy: POLICY,
        selected_precision: selected.precision(),
        verification_precision,
        stop_rule: summary,
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
    };
    CaseOutcome::Selected(Box::new(RetainedSolve {
        prep,
        group,
        cache,
        states,
        selected: selected.precision(),
        evidence,
        publication,
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
    Ok(GroupPrep {
        structure,
        ordering,
        geometry,
    })
}

/// The kernel entry: every case of an invocation (F2a API). Cases with the
/// same stiffness identity share formation and the p-factor per precision.
#[allow(dead_code)] // F2a API (W1's caller is wired at F2a; ROOT's K4 ruling Q1)
pub(crate) fn solve_cases(
    sources: &[PrimitiveSource],
    case_limit: CaseLimit,
    meter: &mut InvocationMeter,
) -> Vec<CaseOutcome> {
    let mut groups: Vec<(
        Vec<u8>,
        Result<Arc<GroupPrep>, (Refusal, Vec<BodyGeometry>)>,
        GroupCache,
    )> = Vec::new();
    let mut out = Vec::with_capacity(sources.len());
    for source in sources {
        if meter.exhausted() {
            out.push(CaseOutcome::Unresolved {
                reason: UnresolvedReason::Budget(BudgetScope::Invocation),
                attempts: Vec::new(),
                geometry: Vec::new(),
            });
            continue;
        }
        let identity = source.stiffness_encoding();
        let index = match groups.iter().position(|g| g.0 == identity) {
            Some(k) => k,
            None => {
                groups.push((
                    identity,
                    prepare_group(source).map(Arc::new),
                    GroupCache::default(),
                ));
                groups.len() - 1
            }
        };
        let (_, group, cache) = &mut groups[index];
        let group = match group {
            Ok(g) => g.clone(),
            Err((refusal, geometry)) => {
                out.push(CaseOutcome::Refused {
                    refusal: refusal.clone(),
                    geometry: geometry.clone(),
                });
                continue;
            }
        };
        let prep = match CasePrep::new(source.clone()) {
            Ok(p) => Arc::new(p),
            Err(e) => {
                out.push(CaseOutcome::Refused {
                    refusal: Refusal::LedgerUnavailable(e),
                    geometry: group.geometry.clone(),
                });
                continue;
            }
        };
        out.push(run_schedule(prep, group, cache, case_limit, meter));
    }
    out
}

/// One case (a group of one; F2a API).
#[allow(dead_code)] // F2a API (W1's caller is wired at F2a; ROOT's K4 ruling Q1)
pub(crate) fn solve_case(
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
        }
        out
    }
}

#[cfg(test)]
#[path = "../../../tests/retained_k4/adaptive_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "../../../tests/retained_k4/references_tests.rs"]
mod references_tests;

#[cfg(test)]
#[path = "../../../tests/retained_k4/classification_tests.rs"]
mod classification_tests;
