//! K4: the certified inverse-norm bounds of D1 revision 5a.3 (R7 §4.1.6.3
//! item 7, Lemmas D and E; ROOT's A3-0 rulings Q3 to Q5).
//!
//! - **Blocks (7a).** A block is a connected component of the free–free
//!   structural pattern (`structure.pattern`, from which the factor's profile is
//!   built). L and D are block-diagonal over blocks. A block carries data if a
//!   nonzero ledger term acts at one of its DOFs, if one of its DOFs is coupled
//!   by the pattern to a constrained DOF whose prescribed value is nonzero, or if
//!   the state is nonzero at one of its DOFs.
//! - **Uc_c (7b, Lemma D)**, from the verification's own factor K̃ = L·D·Lᵀ in
//!   K4's elimination order, every operation on nonnegative data rounded upward
//!   (`directed.rs`):
//!   a = M(L)⁻¹e (a_i = 1 + Σ_{j<i} |l_ij|·a_j), b_i = a_i/d_i,
//!   c = M(L)⁻ᵀb (c_j = b_j + Σ_{i>j} |l_ij|·c_i); a′_j = 1 + Σ_{i>j} |l_ij|,
//!   b′_i = d_i·a′_i, c′_i = b′_i + Σ_{j<i} |l_ij|·b′_j. Per block, U_c = max c_i
//!   and N_L,c = max c′_i; γ_m = m·2^-P/(1 − m·2^-P) with m = 2n + 2 over all
//!   free DOFs; t_c = U_c·γ_m·N_L,c, and when t_c < 1, Uc_c = U_c/(1 − t_c) with
//!   the denominator rounded downward. Operation order as emu7's `u_pass` and
//!   `nl_pass`; exact-zero multipliers are skipped (adding zero changes
//!   nothing).
//! - **est_c (7c)**: the largest ratio ‖(K̃⁻¹x)_c‖₁/‖x_c‖₁ over the condition
//!   screen's own solves with x_c ≠ 0 (`BlockRatios`, observed by
//!   `RetainedFactor::condition_observed`). Each block sum is one exact sum
//!   rounded once and the ratio one division (ROOT's ruling Q3; emu7 rounds the
//!   exact quotient once). Availability only: est_c only chooses σ_c.
//! - **S_c (7c, Lemma E)**: for a block that carries data, with est_c > 0, whose
//!   Uc_c is missing or exceeds 2·⌈√n_c⌉·est_c (decided exactly):
//!   σ_c = 1/(2·est_c) rounded downward; the shifted factorization replaces each
//!   diagonal entry of the block by d̃_i = fl(K̃_ii − σ_c) (nearest) and runs
//!   `factor()`'s loop, operation for operation, with the pivot test d′_i > 0
//!   (`shifted_factor`, a separate copy: ROOT's ruling Q5). A failing pivot
//!   marks its block failed and is replaced by 1, so later rows continue; by 7a
//!   no other block is affected. For a block whose pivots all pass:
//!   N′_L,c from the shifted factor (upward), δ_c = 2^(1−P)·max|d̃_i| (exact),
//!   σ′_c = σ_c − (γ_m·N′_L,c + δ_c) with the bracket upward and the difference
//!   downward, and S_c = ⌈√n_c⌉/σ′_c upward when σ′_c > 0. A failed block is
//!   refactored at σ_c/2, then σ_c/4 (exact halvings): at most three shifted
//!   factorizations in all. A block whose pivots pass with σ′_c ≤ 0 is not
//!   retried.
//! - **B_c (7d)** = min(Uc_c, S_c) over those that exist, for a block with data.
//! - **Amendment A2 (T3 KF3; ROOT's ruling on I19's plan).** A Uc_c or S_c
//!   whose formation is refused (an exact sum spanning more than
//!   `SPAN_LIMIT_BITS`, or a result outside the exponent range: `Span`,
//!   `Exponent`) is unavailable for its block and treated as R7's "does not
//!   exist" (+∞). The first refusal of a block is recorded (`BoundRefusal`: the
//!   kind, the pass and the elimination row), every later operation on the
//!   block's rows is skipped, and the accumulator is reset in full. Every other
//!   block runs operation for operation as before (its rows read only its own
//!   values: cross-block multipliers are exact zeros, which the passes skip,
//!   7a). Budget stops and every other stop propagate. Whether the attempt
//!   stops is decided in `verify_state` (7d).
use super::adaptive::{AttemptStop, StageGuard};
use super::assemble::Structure;
use super::directed::{add_toward, div_toward, mul_toward, sub_toward, Toward};
use super::factor::{Ordering, RetainedFactor};
use super::ledger::RetainedLedger;
use super::source::PrimitiveSource;
use super::wide::multi::{SupportedWidth, WideContext};
use super::wide::Wide;
use super::wide_sum::ExactWideSum;
use crate::DOF_PER_NODE;
use std::cmp::Ordering as CmpOrdering;

// ------------------------------------------------------------ blocks (7a)

/// The connected components of the free–free structural pattern.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FreeBlocks {
    /// The block of each free position.
    pub(crate) of: Vec<u32>,
    /// The free positions of each block, ascending; blocks are numbered by
    /// their least free position.
    pub(crate) positions: Vec<Vec<usize>>,
    /// The body of each block (blocks refine bodies).
    pub(crate) body: Vec<u32>,
}

impl FreeBlocks {
    pub(crate) fn len(&self) -> usize {
        self.positions.len()
    }
}

pub(crate) fn free_blocks(
    source: &PrimitiveSource,
    structure: &Structure,
    ordering: &Ordering,
) -> FreeBlocks {
    let n = ordering.free.len();
    let mut of = vec![u32::MAX; n];
    let mut positions: Vec<Vec<usize>> = Vec::new();
    for start in 0..n {
        if of[start] != u32::MAX {
            continue;
        }
        let id = positions.len() as u32;
        of[start] = id;
        let mut stack = vec![start];
        let mut component = Vec::new();
        while let Some(a) = stack.pop() {
            component.push(a);
            for index in structure.pattern.row_range(ordering.free[a]) {
                let b = ordering.position[structure.pattern.column(index)];
                if b != usize::MAX && of[b] == u32::MAX {
                    of[b] = id;
                    stack.push(b);
                }
            }
        }
        component.sort_unstable();
        positions.push(component);
    }
    let body = positions
        .iter()
        .map(|p| source.body_of_node((ordering.free[p[0]] / DOF_PER_NODE) as u32))
        .collect();
    FreeBlocks {
        of,
        positions,
        body,
    }
}

/// 7a's data flag of every block, at a state u (all DOFs).
/// `prescribed_nonzero[g]`: whether constrained DOF g's exact prescribed value
/// is nonzero.
pub(crate) fn data_blocks<const L: usize>(
    blocks: &FreeBlocks,
    ordering: &Ordering,
    structure: &Structure,
    ledger: &RetainedLedger,
    prescribed_nonzero: &[bool],
    u: &[Wide<L>],
) -> Vec<bool>
where
    Wide<L>: SupportedWidth,
{
    let mut data = vec![false; blocks.len()];
    for (a, &g) in ordering.free.iter().enumerate() {
        let b = blocks.of[a] as usize;
        if data[b] {
            continue;
        }
        if ledger.has_nonzero_term(g) || !u[g].is_zero() {
            data[b] = true;
            continue;
        }
        for index in structure.pattern.row_range(g) {
            let c = structure.pattern.column(index);
            if ordering.position[c] == usize::MAX && prescribed_nonzero[c] {
                data[b] = true;
                break;
            }
        }
    }
    data
}

/// Borrowed bridge fill of exactly the R7 predicate above. The caller validates
/// dimensions and owns the output; no new report or hidden data allocation.
/// Visits include each free position, ledger comparison and inspected adjacency.
pub(crate) fn fill_data_blocks<const L: usize>(
    blocks: &FreeBlocks,
    ordering: &Ordering,
    structure: &Structure,
    ledger: &RetainedLedger,
    prescribed_nonzero: &[bool],
    u: &[Wide<L>],
    data: &mut [bool],
) -> super::work::WorkTotal
where
    Wide<L>: SupportedWidth,
{
    use super::work::WorkTotal;
    data.fill(false);
    let mut visits = WorkTotal::zero();
    for (a, &g) in ordering.free.iter().enumerate() {
        visits = visits.add(WorkTotal::exact_count(1));
        let b = blocks.of[a] as usize;
        if data[b] {
            continue;
        }
        let (nonzero, spent) = ledger.nonzero_term_spent(g);
        visits = visits.add(spent);
        if nonzero || !u[g].is_zero() {
            data[b] = true;
            continue;
        }
        for index in structure.pattern.row_range(g) {
            visits = visits.add(WorkTotal::exact_count(1));
            let c = structure.pattern.column(index);
            if ordering.position[c] == usize::MAX && prescribed_nonzero[c] {
                data[b] = true;
                break;
            }
        }
    }
    visits
}

// ------------------------------------------------------------ est_c (7c)

/// Per block, the largest ratio ‖(K̃⁻¹x)_c‖₁/‖x_c‖₁ over the condition
/// screen's solves with x_c ≠ 0.
#[derive(Debug, Clone)]
pub(crate) struct BlockRatios<const L: usize>
where
    Wide<L>: SupportedWidth,
{
    positions: Vec<Vec<usize>>,
    est: Vec<Wide<L>>,
}

impl<const L: usize> BlockRatios<L>
where
    Wide<L>: SupportedWidth,
{
    pub(crate) fn new(blocks: &FreeBlocks) -> Self {
        Self {
            positions: blocks.positions.clone(),
            est: vec![Wide::<L>::ZERO; blocks.len()],
        }
    }

    /// After one screen solve y = K̃⁻¹x (both in free-position order).
    pub(crate) fn offer(
        &mut self,
        ctx: &mut WideContext<L>,
        sum: &mut ExactWideSum,
        x: &[Wide<L>],
        y: &[Wide<L>],
    ) -> Result<(), AttemptStop> {
        for (b, positions) in self.positions.iter().enumerate() {
            sum.clear();
            for &a in positions {
                sum.add_wide(&x[a].abs(), false)?;
            }
            let xs = sum.round(ctx)?;
            if xs.is_zero() {
                continue;
            }
            sum.clear();
            for &a in positions {
                sum.add_wide(&y[a].abs(), false)?;
            }
            let ys = sum.round(ctx)?;
            let ratio = ctx.div(&ys, &xs)?;
            if ratio.cmp_value(&self.est[b]) == CmpOrdering::Greater {
                self.est[b] = ratio;
            }
        }
        Ok(())
    }

    pub(crate) fn estimates(self) -> Vec<Wide<L>> {
        self.est
    }
}

// ------------------------------------------------------------ A2: refusals

/// How a bound's formation was refused (amendment A2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefusalKind {
    /// An exact sum spanned more than `SPAN_LIMIT_BITS`.
    Span,
    /// A result outside the `Wide` exponent range.
    Exponent,
}

/// The pass of 7b or 7c in which a block's first refusal occurred.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundPass {
    /// 7b: a = M(L)⁻¹e.
    Forward,
    /// 7b: b_i = a_i/d_i.
    Pivot,
    /// 7b: c = M(L)⁻ᵀb.
    Backward,
    /// 7b (and 7c's N′_L): a′ = |Lᵀ|e.
    NlColumn,
    /// b′_i = d_i·a′_i.
    NlScale,
    /// c′ = |L|b′.
    NlRow,
    /// 7b: t_c and Uc_c from U_c and N_L,c.
    Form,
    /// 7c: whether the shift is needed (the exact comparison).
    Need,
    /// 7c: σ_c = 1/(2·est_c).
    Sigma,
    /// 7c: the shifted factorization.
    ShiftFactor,
    /// 7c: δ_c, σ′_c and S_c.
    ShiftForm,
}

/// A block's first refusal (amendment A2): its kind, its pass and the
/// elimination row (`usize::MAX` for a per-block step).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoundRefusal {
    pub kind: RefusalKind,
    pub pass: BoundPass,
    pub row: usize,
}

impl BoundRefusal {
    /// The stop the refusal would have been before A2 (and is when a block
    /// with data is left with no bound, 7d).
    pub(crate) fn stop(&self) -> AttemptStop {
        match self.kind {
            RefusalKind::Span => AttemptStop::Span,
            RefusalKind::Exponent => AttemptStop::Exponent,
        }
    }
}

/// Which certified bound a refusal belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CertifiedBound {
    Uc,
    S,
}

/// One block's refusal, as the attempt's evidence records it (amendment A2;
/// ROOT's ruling 3: on `AttemptRecord` only).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockRefusal {
    pub block: u32,
    pub bound: CertifiedBound,
    pub refusal: BoundRefusal,
}

/// A2's classification of an operation's result: `Ok(Some(v))`, `Ok(None)`
/// after recording a refusal in `slot` (the first one kept) and resetting the
/// accumulator in full, or the stop itself when it is not a refusal.
pub(crate) fn refusable<T>(
    result: Result<T, AttemptStop>,
    sum: &mut ExactWideSum,
    slot: &mut Option<BoundRefusal>,
    pass: BoundPass,
    row: usize,
) -> Result<Option<T>, AttemptStop> {
    if let Err(fault) = sum.work().checked_lme().exact() {
        // A pre-existing numerical stop remains the terminal prior; no reset or
        // block-local refusal may hide the unavailable accounting state.
        return match result {
            Err(stop) => Err(stop),
            Ok(_) => Err(fault.into()),
        };
    }
    match result {
        Ok(v) => Ok(Some(v)),
        Err(stop) => {
            let kind = refusal_kind(stop)?;
            sum.reset();
            record(slot, kind, pass, row);
            Ok(None)
        }
    }
}

/// A refusal's kind, or the stop itself when it is not a refusal (A2).
pub(crate) fn refusal_kind(stop: AttemptStop) -> Result<RefusalKind, AttemptStop> {
    match stop {
        AttemptStop::Span => Ok(RefusalKind::Span),
        AttemptStop::Exponent => Ok(RefusalKind::Exponent),
        other => Err(other),
    }
}

/// Keeps a block's first refusal.
fn record(slot: &mut Option<BoundRefusal>, kind: RefusalKind, pass: BoundPass, row: usize) {
    if slot.is_none() {
        *slot = Some(BoundRefusal { kind, pass, row });
    }
}

// ------------------------------------------------------------ Uc_c (7b)

/// A unit lower triangular L and diagonal D on a profile, in elimination
/// order (read-only).
pub(crate) trait ProfileLdl<const L: usize>
where
    Wide<L>: SupportedWidth,
{
    fn rows(&self) -> usize;
    fn first_of(&self, i: usize) -> usize;
    /// l_ij for first_of(i) ≤ j < i; d_i for j = i.
    fn entry(&self, i: usize, j: usize) -> Wide<L>;
}

impl<const L: usize> ProfileLdl<L> for RetainedFactor<L>
where
    Wide<L>: SupportedWidth,
{
    fn rows(&self) -> usize {
        self.first().len()
    }
    fn first_of(&self, i: usize) -> usize {
        self.first()[i]
    }
    fn entry(&self, i: usize, j: usize) -> Wide<L> {
        self.get(i, j)
    }
}

/// c = M(L)⁻ᵀD⁻¹M(L)⁻¹e, every operation upward (emu7's `u_pass`).
///
/// Amendment A2: `refused` holds each block's first refusal. A refusal on a
/// row of block b records (kind, pass, row) in `refused[b]`, and every later
/// operation on b's rows is skipped (their values are not formed and not
/// read); every other block runs operation for operation as without it. A
/// block already refused on entry is skipped throughout.
pub(crate) fn u_pass<const L: usize, F: ProfileLdl<L>>(
    ctx: &mut WideContext<L>,
    sum: &mut ExactWideSum,
    guard: &StageGuard,
    f: &F,
    block_of_row: &[u32],
    refused: &mut [Option<BoundRefusal>],
) -> Result<Vec<Wide<L>>, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    let n = f.rows();
    let one = Wide::<L>::ONE;
    let mut a = vec![one; n];
    for i in 0..n {
        let b = block_of_row[i] as usize;
        if refused[b].is_none() {
            let mut acc = Some(one);
            for j in f.first_of(i)..i {
                let l = f.entry(i, j).abs();
                if l.is_zero() {
                    continue;
                }
                let Some(t) = refusable(
                    mul_toward(ctx, sum, &l, &a[j], Toward::Up),
                    sum,
                    &mut refused[b],
                    BoundPass::Forward,
                    i,
                )?
                else {
                    acc = None;
                    break;
                };
                let Some(v) = refusable(
                    add_toward(ctx, sum, &acc.unwrap_or(one), &t, Toward::Up),
                    sum,
                    &mut refused[b],
                    BoundPass::Forward,
                    i,
                )?
                else {
                    acc = None;
                    break;
                };
                acc = Some(v);
            }
            if let Some(v) = acc {
                a[i] = v;
            }
        }
        guard.check(ctx, sum)?;
    }
    let mut c = Vec::with_capacity(n);
    for (i, ai) in a.iter().enumerate() {
        let b = block_of_row[i] as usize;
        let v = if refused[b].is_none() {
            refusable(
                div_toward(ctx, sum, ai, &f.entry(i, i), Toward::Up),
                sum,
                &mut refused[b],
                BoundPass::Pivot,
                i,
            )?
        } else {
            None
        };
        c.push(v.unwrap_or(Wide::<L>::ZERO));
    }
    for i in (0..n).rev() {
        let b = block_of_row[i] as usize;
        if refused[b].is_none() {
            let ci = c[i];
            for j in f.first_of(i)..i {
                let l = f.entry(i, j).abs();
                if l.is_zero() {
                    continue;
                }
                let Some(t) = refusable(
                    mul_toward(ctx, sum, &l, &ci, Toward::Up),
                    sum,
                    &mut refused[b],
                    BoundPass::Backward,
                    j,
                )?
                else {
                    break;
                };
                let Some(v) = refusable(
                    add_toward(ctx, sum, &c[j], &t, Toward::Up),
                    sum,
                    &mut refused[b],
                    BoundPass::Backward,
                    j,
                )?
                else {
                    break;
                };
                c[j] = v;
            }
        }
        guard.check(ctx, sum)?;
    }
    Ok(c)
}

/// c′ = |L|D|Lᵀ|e, every operation upward (emu7's `nl_pass`); refusals as in
/// `u_pass` (amendment A2).
pub(crate) fn nl_pass<const L: usize, F: ProfileLdl<L>>(
    ctx: &mut WideContext<L>,
    sum: &mut ExactWideSum,
    guard: &StageGuard,
    f: &F,
    block_of_row: &[u32],
    refused: &mut [Option<BoundRefusal>],
) -> Result<Vec<Wide<L>>, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    let n = f.rows();
    let mut at = vec![Wide::<L>::ONE; n];
    for i in 0..n {
        let b = block_of_row[i] as usize;
        if refused[b].is_some() {
            continue;
        }
        for j in f.first_of(i)..i {
            let l = f.entry(i, j).abs();
            if l.is_zero() {
                continue;
            }
            let Some(v) = refusable(
                add_toward(ctx, sum, &at[j], &l, Toward::Up),
                sum,
                &mut refused[b],
                BoundPass::NlColumn,
                j,
            )?
            else {
                break;
            };
            at[j] = v;
        }
    }
    let mut bt = Vec::with_capacity(n);
    for (i, a) in at.iter().enumerate() {
        let b = block_of_row[i] as usize;
        let v = if refused[b].is_none() {
            refusable(
                mul_toward(ctx, sum, &f.entry(i, i), a, Toward::Up),
                sum,
                &mut refused[b],
                BoundPass::NlScale,
                i,
            )?
        } else {
            None
        };
        bt.push(v.unwrap_or(Wide::<L>::ZERO));
    }
    let mut ct = bt.clone();
    for i in 0..n {
        let b = block_of_row[i] as usize;
        if refused[b].is_none() {
            for j in f.first_of(i)..i {
                let l = f.entry(i, j).abs();
                if l.is_zero() {
                    continue;
                }
                let Some(t) = refusable(
                    mul_toward(ctx, sum, &l, &bt[j], Toward::Up),
                    sum,
                    &mut refused[b],
                    BoundPass::NlRow,
                    i,
                )?
                else {
                    break;
                };
                let Some(v) = refusable(
                    add_toward(ctx, sum, &ct[i], &t, Toward::Up),
                    sum,
                    &mut refused[b],
                    BoundPass::NlRow,
                    i,
                )?
                else {
                    break;
                };
                ct[i] = v;
            }
        }
        guard.check(ctx, sum)?;
    }
    Ok(ct)
}

/// γ_m = m/(2^P − m), m = 2n + 2 (n all free DOFs), rounded upward.
pub(crate) fn gamma_m<const L: usize>(
    ctx: &mut WideContext<L>,
    sum: &mut ExactWideSum,
    free_count: usize,
) -> Result<Wide<L>, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    let m = Wide::<L>::from_f64((2 * free_count + 2) as f64)?;
    sum.clear();
    sum.add_wide_scaled(&Wide::<L>::ONE, false, 1, i64::from(ctx.precision()))?;
    sum.add_wide(&m, true)?;
    // 2^P − m has at most P bits: the rounding is exact.
    let den = sum.round(ctx)?;
    div_toward(ctx, sum, &m, &den, Toward::Up)
}

/// One block's comparison-matrix bound (7b).
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BlockBound<const L: usize>
where
    Wide<L>: SupportedWidth,
{
    pub(crate) u: Wide<L>,
    pub(crate) n_l: Wide<L>,
    pub(crate) t: Wide<L>,
    /// U_c/(1 − t_c) when t_c < 1.
    pub(crate) uc: Option<Wide<L>>,
    /// Amendment A2: the block's first refusal. When present, Uc_c is
    /// unavailable (`uc` is `None`), and `u`, `n_l` and `t` were not formed
    /// (recorded as zero, never read for a bound).
    pub(crate) refused: Option<BoundRefusal>,
}

impl<const L: usize> BlockBound<L>
where
    Wide<L>: SupportedWidth,
{
    fn refused(refusal: BoundRefusal) -> Self {
        Self {
            u: Wide::<L>::ZERO,
            n_l: Wide::<L>::ZERO,
            t: Wide::<L>::ZERO,
            uc: None,
            refused: Some(refusal),
        }
    }
}

/// Per block: max over the rows `rows_of_block` of `values` (elimination order).
fn block_max<const L: usize>(
    values: &[Wide<L>],
    block_of_row: &[u32],
    blocks: usize,
) -> Vec<Wide<L>>
where
    Wide<L>: SupportedWidth,
{
    let mut out = vec![Wide::<L>::ZERO; blocks];
    for (i, v) in values.iter().enumerate() {
        let b = block_of_row[i] as usize;
        if v.cmp_value(&out[b]) == CmpOrdering::Greater {
            out[b] = *v;
        }
    }
    out
}

/// The block of each elimination row of a factor over `ordering`.
pub(crate) fn block_of_rows(ordering: &Ordering, blocks: &FreeBlocks) -> Vec<u32> {
    ordering.order.iter().map(|&a| blocks.of[a]).collect()
}

/// U_c, N_L,c, t_c and Uc_c from per-row c and c′.
fn bounds_from<const L: usize>(
    ctx: &mut WideContext<L>,
    sum: &mut ExactWideSum,
    gamma: &Wide<L>,
    u: &Wide<L>,
    n_l: &Wide<L>,
) -> Result<BlockBound<L>, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    let one = Wide::<L>::ONE;
    let ug = mul_toward(ctx, sum, u, gamma, Toward::Up)?;
    let t = mul_toward(ctx, sum, &ug, n_l, Toward::Up)?;
    let uc = if t.cmp_value(&one) == CmpOrdering::Less {
        let den = sub_toward(ctx, sum, &one, &t, Toward::Down)?;
        Some(div_toward(ctx, sum, u, &den, Toward::Up)?)
    } else {
        None
    };
    Ok(BlockBound {
        u: *u,
        n_l: *n_l,
        t,
        uc,
        refused: None,
    })
}

/// 7b per block, from a factor whose rows' blocks are `block_of_row`.
/// `refused` holds one slot per block (all `None` on entry). The caller owns
/// it, so a refusal recorded before a later stop in the same build survives
/// that stop and reaches the evidence (RV23-1).
pub(crate) fn uc_bounds<const L: usize, F: ProfileLdl<L>>(
    ctx: &mut WideContext<L>,
    sum: &mut ExactWideSum,
    guard: &StageGuard,
    f: &F,
    block_of_row: &[u32],
    refused: &mut [Option<BoundRefusal>],
    gamma: &Wide<L>,
) -> Result<Vec<BlockBound<L>>, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    // Amendment A2: a refusal makes only its own block's Uc_c unavailable.
    let blocks = refused.len();
    let c = u_pass(ctx, sum, guard, f, block_of_row, refused)?;
    let ct = nl_pass(ctx, sum, guard, f, block_of_row, refused)?;
    let u = block_max(&c, block_of_row, blocks);
    let n_l = block_max(&ct, block_of_row, blocks);
    let mut out = Vec::with_capacity(blocks);
    for (b, slot) in refused.iter_mut().enumerate() {
        if let Some(r) = slot {
            out.push(BlockBound::refused(*r));
            continue;
        }
        match refusable(
            bounds_from(ctx, sum, gamma, &u[b], &n_l[b]),
            sum,
            slot,
            BoundPass::Form,
            usize::MAX,
        )? {
            Some(bound) => out.push(bound),
            None => out.push(BlockBound::refused(slot.expect("recorded"))),
        }
    }
    Ok(out)
}

// ------------------------------------------------------------ S_c (7c, Lemma E)

/// K̃ = S·K·S in elimination order on the factor's profile, built as
/// `factor()` builds it (the same scalings, the same profile).
#[derive(Debug, Clone)]
pub(crate) struct ScaledProfile<const L: usize>
where
    Wide<L>: SupportedWidth,
{
    first: Vec<usize>,
    rows: Vec<Vec<Wide<L>>>,
    block_of_row: Vec<u32>,
    blocks: usize,
}

impl<const L: usize> ScaledProfile<L>
where
    Wide<L>: SupportedWidth,
{
    /// A profile given directly (the low-precision stress and V4's F2 family):
    /// `rows[i]` holds K̃_ij for j in first[i]..=i.
    #[allow(dead_code)] // test entry (the stress and the F2 family)
    pub(crate) fn from_rows(
        first: Vec<usize>,
        rows: Vec<Vec<Wide<L>>>,
        block_of_row: Vec<u32>,
    ) -> Self {
        let blocks = block_of_row
            .iter()
            .map(|&b| b as usize + 1)
            .max()
            .unwrap_or(0);
        Self {
            first,
            rows,
            block_of_row,
            blocks,
        }
    }

    #[allow(dead_code)] // test entry
    pub(crate) fn block_of_row(&self) -> &[u32] {
        &self.block_of_row
    }
}

pub(crate) fn scaled_profile<const L: usize>(
    structure: &Structure,
    k: &[Wide<L>],
    ordering: &Ordering,
    blocks: &FreeBlocks,
    scale: &[i64],
) -> Result<ScaledProfile<L>, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    let n = ordering.free.len();
    let mut rows = Vec::with_capacity(n);
    for i in 0..n {
        let a = ordering.order[i];
        let g = ordering.free[a];
        let f = ordering.first[i];
        let mut row = vec![Wide::<L>::ZERO; i - f + 1];
        for index in structure.pattern.row_range(g) {
            let b = ordering.position[structure.pattern.column(index)];
            if b == usize::MAX {
                continue;
            }
            let j = ordering.rank[b];
            if j <= i {
                row[j - f] = k[index].mul_pow2(scale[a] + scale[b])?;
            }
        }
        rows.push(row);
    }
    Ok(ScaledProfile {
        first: ordering.first.clone(),
        rows,
        block_of_row: block_of_rows(ordering, blocks),
        blocks: blocks.len(),
    })
}

/// A factor of the shifted matrix (Lemma E).
#[derive(Debug, Clone)]
pub(crate) struct ShiftedFactor<const L: usize>
where
    Wide<L>: SupportedWidth,
{
    first: Vec<usize>,
    rows: Vec<Vec<Wide<L>>>,
    /// Per block: a pivot that was not positive.
    pub(crate) failed: Vec<bool>,
    /// d̃_i for each row of a shifted block.
    pub(crate) shifted: Vec<Option<Wide<L>>>,
}

impl<const L: usize> ProfileLdl<L> for ShiftedFactor<L>
where
    Wide<L>: SupportedWidth,
{
    fn rows(&self) -> usize {
        self.first.len()
    }
    fn first_of(&self, i: usize) -> usize {
        self.first[i]
    }
    fn entry(&self, i: usize, j: usize) -> Wide<L> {
        if j < self.first[i] {
            Wide::<L>::ZERO
        } else {
            self.rows[i][j - self.first[i]]
        }
    }
}

/// `factor()`'s loop, operation for operation, on K̃ with d̃_i = fl(K̃_ii − σ_c)
/// in every block with a σ_c, and the pivot test d′_i > 0 (module
/// documentation). With no block shifted it forms `factor()`'s L and D bit for
/// bit wherever `factor()`'s pivot screen passes (the loop-parity test).
///
/// `refused` holds one slot per block (all `None` on entry): each block's
/// first refusal of this factorization (A2; only an exponent refusal can occur
/// in the context's operations). A refused block's later rows are not formed.
/// The caller owns the slots, so a refusal survives a later stop (RV23-1).
pub(crate) fn shifted_factor<const L: usize>(
    ctx: &mut WideContext<L>,
    sum: &ExactWideSum,
    guard: &StageGuard,
    profile: &ScaledProfile<L>,
    sigma: &[Option<Wide<L>>],
    refused: &mut [Option<BoundRefusal>],
) -> Result<ShiftedFactor<L>, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    let n = profile.first.len();
    let first = profile.first.clone();
    let mut rows = profile.rows.clone();
    let mut shifted = vec![None; n];
    for i in 0..n {
        let b = profile.block_of_row[i] as usize;
        if let Some(s) = &sigma[b] {
            if refused[b].is_some() {
                continue;
            }
            let at = i - first[i];
            match ctx.sub(&rows[i][at], s) {
                Ok(d) => {
                    rows[i][at] = d;
                    shifted[i] = Some(d);
                }
                Err(e) => record(
                    &mut refused[b],
                    refusal_kind(e.into())?,
                    BoundPass::ShiftFactor,
                    i,
                ),
            }
        }
    }
    let zero = Wide::<L>::ZERO;
    let get = |rows: &Vec<Vec<Wide<L>>>, i: usize, j: usize| -> Wide<L> {
        if j < first[i] {
            zero
        } else {
            rows[i][j - first[i]]
        }
    };
    let mut failed = vec![false; profile.blocks];
    let mut work = vec![zero; n];
    for i in 0..n {
        let b = profile.block_of_row[i] as usize;
        if refused[b].is_some() {
            guard.check(ctx, sum)?;
            continue;
        }
        // The row's operations; an exponent refusal ends the row (A2).
        let mut row = || -> Result<Wide<L>, AttemptStop> {
            for j in first[i]..i {
                let mut s = get(&rows, i, j);
                for kk in first[i].max(first[j])..j {
                    let t = ctx.mul(&work[kk], &get(&rows, j, kk))?;
                    s = ctx.sub(&s, &t)?;
                }
                work[j] = s;
                let d = get(&rows, j, j);
                rows[i][j - first[i]] = ctx.div(&s, &d)?;
            }
            let mut pivot = get(&rows, i, i);
            for kk in first[i]..i {
                let term = ctx.mul(&work[kk], &get(&rows, i, kk))?;
                pivot = ctx.sub(&pivot, &term)?;
            }
            Ok(pivot)
        };
        let mut pivot = match row() {
            Ok(p) => p,
            Err(e) => {
                record(&mut refused[b], refusal_kind(e)?, BoundPass::ShiftFactor, i);
                Wide::<L>::ONE
            }
        };
        if pivot.is_zero() || pivot.is_sign_negative() {
            // The block has failed at this σ_c; its later rows are not used.
            failed[b] = true;
            pivot = Wide::<L>::ONE;
        }
        rows[i][i - first[i]] = pivot;
        guard.check(ctx, sum)?;
    }
    Ok(ShiftedFactor {
        first,
        rows,
        failed,
        shifted,
    })
}

/// One block's shift-bound outcome.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ShiftResult<const L: usize>
where
    Wide<L>: SupportedWidth,
{
    /// The last σ_c tried.
    pub(crate) sigma: Wide<L>,
    /// The shifted factorizations this block took part in.
    pub(crate) tries: u8,
    pub(crate) n_l: Option<Wide<L>>,
    pub(crate) delta: Option<Wide<L>>,
    pub(crate) sigma_prime: Option<Wide<L>>,
    pub(crate) s: Option<Wide<L>>,
    /// Amendment A2: the block's first refusal in 7c; S_c is then unavailable
    /// and the block is not retried.
    pub(crate) refused: Option<BoundRefusal>,
}

/// ⌈√n⌉, exactly.
pub(crate) fn ceil_sqrt(n: usize) -> u64 {
    let n = n as u64;
    let mut r = (n as f64).sqrt() as u64;
    while u128::from(r) * u128::from(r) > u128::from(n) {
        r -= 1;
    }
    while u128::from(r) * u128::from(r) < u128::from(n) {
        r += 1;
    }
    r
}

/// σ_c = 1/(2·est_c), rounded downward (est_c > 0).
pub(crate) fn sigma_from_estimate<const L: usize>(
    ctx: &mut WideContext<L>,
    sum: &mut ExactWideSum,
    est: &Wide<L>,
) -> Result<Wide<L>, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    div_toward(ctx, sum, &Wide::<L>::ONE, &est.mul_pow2(1)?, Toward::Down)
}

/// Whether 7c's shift runs for a block with data: est_c > 0, and Uc_c missing
/// or above 2·⌈√n_c⌉·est_c (exactly).
pub(crate) fn shift_needed<const L: usize>(
    sum: &mut ExactWideSum,
    uc: &Option<Wide<L>>,
    est: &Wide<L>,
    n_c: usize,
) -> Result<bool, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    // V-K seeded fault VK-R28 (R7-M28): no shift (Uc alone).
    #[cfg(any(test, feature = "mutation-controls"))]
    if super::seeded::active(super::seeded::Fault::R28) {
        return Ok(false);
    }
    if est.is_zero() {
        return Ok(false);
    }
    let Some(uc) = uc else {
        return Ok(true);
    };
    sum.clear();
    sum.add_wide(uc, false)?;
    sum.add_wide_scaled(est, true, 2 * ceil_sqrt(n_c), 0)?;
    let above = sum.signum()? > 0;
    sum.clear();
    Ok(above)
}

/// 7c's schedule for the blocks in `start` (block, σ_c, n_c): at most three
/// shifted factorizations in all, each over the whole matrix with the blocks
/// still failing shifted (their σ halved exactly after a failure). Returns each
/// block's result and the number of factorizations.
///
/// RV23-1: when the schedule stops (a budget stop, say) after a refusal, each
/// started block's refusal so far is recorded in `s_refused` before the stop
/// propagates: those kept in the results of earlier factorizations, and those
/// of the factorization in progress for the blocks it shifts. They are the
/// refusals the results would have carried had it run on. On success
/// `s_refused` is left to the caller, which records the results' refusals.
pub(crate) fn shift_schedule<const L: usize>(
    ctx: &mut WideContext<L>,
    sum: &mut ExactWideSum,
    guard: &StageGuard,
    profile: &ScaledProfile<L>,
    gamma: &Wide<L>,
    start: &[(usize, Wide<L>, usize)],
    s_refused: &mut [Option<BoundRefusal>],
) -> Result<(Vec<(usize, ShiftResult<L>)>, u8), AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    let mut results: Vec<(usize, ShiftResult<L>)> = start
        .iter()
        .map(|&(b, sigma, _)| {
            (
                b,
                ShiftResult {
                    sigma,
                    tries: 0,
                    n_l: None,
                    delta: None,
                    sigma_prime: None,
                    s: None,
                    refused: None,
                },
            )
        })
        .collect();
    // RV23-1: the factorization in progress's shifted blocks and refusals,
    // held here so that a stop after a refusal keeps it.
    let mut in_flight: Vec<usize> = Vec::new();
    let mut refused: Vec<Option<BoundRefusal>> = vec![None; profile.blocks];
    // The factorizations (a closure, so the stop comes back here).
    let mut steps = || -> Result<u8, AttemptStop> {
        let mut current: Vec<(usize, Wide<L>, usize)> = start.to_vec();
        let mut factorizations = 0u8;
        while !current.is_empty() && factorizations < 3 {
            let mut sigma = vec![None; profile.blocks];
            for (b, s, _) in &current {
                sigma[*b] = Some(*s);
            }
            in_flight.clear();
            in_flight.extend(current.iter().map(|c| c.0));
            refused.fill(None);
            let f = shifted_factor(ctx, sum, guard, profile, &sigma, &mut refused)?;
            factorizations += 1;
            // A2: N′_L per block, a refusal of the factorization carried over.
            let ct = nl_pass(ctx, sum, guard, &f, &profile.block_of_row, &mut refused)?;
            let n_l = block_max(&ct, &profile.block_of_row, profile.blocks);
            let mut next = Vec::new();
            for (b, s, n_c) in current {
                let r = &mut results
                    .iter_mut()
                    .find(|r| r.0 == b)
                    .expect("a started block")
                    .1;
                r.tries += 1;
                r.sigma = s;
                if let Some(refusal) = refused[b] {
                    // S_c is unavailable; a refusal is not retried (A2).
                    r.refused = Some(refusal);
                    continue;
                }
                if f.failed[b] {
                    match s.mul_pow2(-1) {
                        Ok(half) => next.push((b, half, n_c)),
                        Err(e) => record(
                            &mut r.refused,
                            refusal_kind(e.into())?,
                            BoundPass::ShiftForm,
                            usize::MAX,
                        ),
                    }
                    continue;
                }
                // δ_c = 2^(1−P)·max_{i∈c}|d̃_i|, exact.
                let mut top = Wide::<L>::ZERO;
                for (i, d) in f.shifted.iter().enumerate() {
                    if profile.block_of_row[i] as usize == b {
                        if let Some(d) = d {
                            if d.abs().cmp_value(&top) == CmpOrdering::Greater {
                                top = d.abs();
                            }
                        }
                    }
                }
                let form = |ctx: &mut WideContext<L>,
                            sum: &mut ExactWideSum|
                 -> Result<(Wide<L>, Wide<L>, Option<Wide<L>>), AttemptStop> {
                    let delta = top.mul_pow2(1 - i64::from(ctx.precision()))?;
                    let gn = mul_toward(ctx, sum, gamma, &n_l[b], Toward::Up)?;
                    let e = add_toward(ctx, sum, &gn, &delta, Toward::Up)?;
                    let sp = sub_toward(ctx, sum, &s, &e, Toward::Down)?;
                    let sv = if !sp.is_zero() && !sp.is_sign_negative() {
                        let root = Wide::<L>::from_f64(ceil_sqrt(n_c) as f64)?;
                        Some(div_toward(ctx, sum, &root, &sp, Toward::Up)?)
                    } else {
                        None
                    };
                    Ok((delta, sp, sv))
                };
                let formed = form(ctx, sum);
                if let Some((delta, sp, sv)) = refusable(
                    formed,
                    sum,
                    &mut r.refused,
                    BoundPass::ShiftForm,
                    usize::MAX,
                )? {
                    r.n_l = Some(n_l[b]);
                    r.delta = Some(delta);
                    r.sigma_prime = Some(sp);
                    r.s = sv;
                }
            }
            current = next;
        }
        Ok(factorizations)
    };
    let steps = steps();
    if steps.is_err() {
        for (b, r) in &results {
            if let Some(x) = r.refused {
                record(&mut s_refused[*b], x.kind, x.pass, x.row);
            }
        }
        for &b in &in_flight {
            if let Some(x) = refused[b] {
                record(&mut s_refused[b], x.kind, x.pass, x.row);
            }
        }
    }
    let factorizations = steps?;
    Ok((results, factorizations))
}

/// B_c = min(Uc_c, S_c) over those that exist (7d).
pub(crate) fn certified<const L: usize>(
    uc: &Option<Wide<L>>,
    s: &Option<Wide<L>>,
) -> Option<Wide<L>>
where
    Wide<L>: SupportedWidth,
{
    match (uc, s) {
        (Some(a), Some(b)) => Some(if b.cmp_value(a) == CmpOrdering::Less {
            *b
        } else {
            *a
        }),
        (Some(a), None) => Some(*a),
        (None, Some(b)) => Some(*b),
        (None, None) => None,
    }
}

/// One block's certificate at a verification (7b to 7d).
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BlockCertificate<const L: usize>
where
    Wide<L>: SupportedWidth,
{
    pub(crate) bound: BlockBound<L>,
    pub(crate) est: Wide<L>,
    pub(crate) data: bool,
    pub(crate) shift: Option<ShiftResult<L>>,
    /// Amendment A2: S_c's first refusal (before the shift, or in it).
    pub(crate) s_refused: Option<BoundRefusal>,
    /// B_c, for a block with data (None: neither bound exists, or no data).
    pub(crate) b: Option<Wide<L>>,
}

/// 7c's start, for the blocks with data (`shift_needed`): (block, σ_c, n_c).
/// A refusal of the exact comparison or of σ_c makes that block's S_c
/// unavailable (A2), recorded in `s_refused`.
pub(crate) fn shift_start<const L: usize>(
    ctx: &mut WideContext<L>,
    sum: &mut ExactWideSum,
    blocks: &FreeBlocks,
    bounds: &[BlockBound<L>],
    est: &[Wide<L>],
    data: &[bool],
    s_refused: &mut [Option<BoundRefusal>],
) -> Result<Vec<(usize, Wide<L>, usize)>, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    let mut start = Vec::new();
    for b in 0..blocks.len() {
        if !data[b] {
            continue;
        }
        let n_c = blocks.positions[b].len();
        let needed = refusable(
            shift_needed(sum, &bounds[b].uc, &est[b], n_c),
            sum,
            &mut s_refused[b],
            BoundPass::Need,
            usize::MAX,
        )?;
        if needed == Some(true) {
            if let Some(sigma) = refusable(
                sigma_from_estimate(ctx, sum, &est[b]),
                sum,
                &mut s_refused[b],
                BoundPass::Sigma,
                usize::MAX,
            )? {
                start.push((b, sigma, n_c));
            }
        }
    }
    Ok(start)
}

/// 7c's shifted factorizations for `start`, on the verification's own K̃. A
/// refusal of the scaled profile (derived unreachable: `factor()` formed the
/// same scalings) makes S_c unavailable for every started block (A2).
#[allow(clippy::too_many_arguments)]
pub(crate) fn shift_run<const L: usize>(
    ctx: &mut WideContext<L>,
    sum: &mut ExactWideSum,
    guard: &StageGuard,
    structure: &Structure,
    k: &[Wide<L>],
    ordering: &Ordering,
    scale: &[i64],
    blocks: &FreeBlocks,
    gamma: &Wide<L>,
    start: &[(usize, Wide<L>, usize)],
    s_refused: &mut [Option<BoundRefusal>],
) -> Result<(Vec<(usize, ShiftResult<L>)>, u8), AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    if start.is_empty() {
        return Ok((Vec::new(), 0));
    }
    let profile = match scaled_profile(structure, k, ordering, blocks, scale) {
        Ok(profile) => profile,
        Err(stop) => {
            let kind = refusal_kind(stop)?;
            for &(b, _, _) in start {
                record(&mut s_refused[b], kind, BoundPass::ShiftFactor, usize::MAX);
            }
            return Ok((Vec::new(), 0));
        }
    };
    let (shifts, count) = shift_schedule(ctx, sum, guard, &profile, gamma, start, s_refused)?;
    for (b, r) in &shifts {
        if let Some(refusal) = r.refused {
            record(&mut s_refused[*b], refusal.kind, refusal.pass, refusal.row);
        }
    }
    Ok((shifts, count))
}

/// 7d with amendment A2, for every block: the certificates, the first block
/// with data and no bound (`uc`), and the first such block with a refused
/// bound, which stops the attempt with that refusal (it takes precedence over
/// `uc`: ROOT's rulings 1 and 2 on I19's plan; Uc_c's refusal before S_c's).
#[allow(clippy::type_complexity)]
pub(crate) fn certificates<const L: usize>(
    blocks: &FreeBlocks,
    bounds: &[BlockBound<L>],
    est: &[Wide<L>],
    data: &[bool],
    shifts: &[(usize, ShiftResult<L>)],
    s_refused: &[Option<BoundRefusal>],
) -> (
    Vec<BlockCertificate<L>>,
    Option<usize>,
    Option<(usize, BoundRefusal)>,
)
where
    Wide<L>: SupportedWidth,
{
    let mut out = Vec::with_capacity(blocks.len());
    let (mut uc_missing, mut stop) = (None, None);
    for b in 0..blocks.len() {
        let shift = shifts.iter().find(|s| s.0 == b).map(|s| s.1.clone());
        let s = shift.as_ref().and_then(|r| r.s);
        let bound = if data[b] {
            certified(&bounds[b].uc, &s)
        } else {
            None
        };
        if data[b] && bound.is_none() {
            match bounds[b].refused.or(s_refused[b]) {
                Some(refusal) if stop.is_none() => stop = Some((b, refusal)),
                Some(_) => {}
                None if uc_missing.is_none() => uc_missing = Some(b),
                None => {}
            }
        }
        out.push(BlockCertificate {
            bound: bounds[b].clone(),
            est: est[b],
            data: data[b],
            shift,
            s_refused: s_refused[b],
            b: bound,
        });
    }
    (out, uc_missing, stop)
}

/// The Uc_c refusals held in a stopped shared build's slots (RV23-1), in
/// block order, as `block_refusals` lists a completed build's from `uc`.
pub(crate) fn uc_refusals(slots: &[Option<BoundRefusal>]) -> Vec<BlockRefusal> {
    slots
        .iter()
        .enumerate()
        .filter_map(|(b, r)| {
            r.map(|refusal| BlockRefusal {
                block: b as u32,
                bound: CertifiedBound::Uc,
                refusal,
            })
        })
        .collect()
}

/// The evidence of a verification's refusals, per block (A2; ROOT's ruling 3:
/// `AttemptRecord.bound_refusals`): every block's Uc_c refusal, then every
/// block's S_c refusal, each in block order.
pub(crate) fn block_refusals<const L: usize>(
    bounds: &[BlockBound<L>],
    s_refused: &[Option<BoundRefusal>],
) -> Vec<BlockRefusal>
where
    Wide<L>: SupportedWidth,
{
    let uc = bounds.iter().enumerate().filter_map(|(b, bb)| {
        bb.refused.map(|refusal| BlockRefusal {
            block: b as u32,
            bound: CertifiedBound::Uc,
            refusal,
        })
    });
    let s = s_refused.iter().enumerate().filter_map(|(b, r)| {
        r.map(|refusal| BlockRefusal {
            block: b as u32,
            bound: CertifiedBound::S,
            refusal,
        })
    });
    uc.chain(s).collect()
}

/// 7b to 7d for every block: the shift runs only for the blocks with data that
/// need it (`shift_needed`), and B_c is formed for the blocks with data (with
/// amendment A2, as `verify_state` composes them). Returns the certificates,
/// the number of shifted factorizations, and the refusal that stops the
/// attempt, if any.
#[allow(dead_code)] // test entry: 7b–7d composed as `verify_state` does
#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
pub(crate) fn certify<const L: usize>(
    ctx: &mut WideContext<L>,
    sum: &mut ExactWideSum,
    guard: &StageGuard,
    structure: &Structure,
    k: &[Wide<L>],
    ordering: &Ordering,
    factor: &RetainedFactor<L>,
    blocks: &FreeBlocks,
    bounds: &[BlockBound<L>],
    est: &[Wide<L>],
    data: &[bool],
    gamma: &Wide<L>,
) -> Result<(Vec<BlockCertificate<L>>, u8, Option<(usize, BoundRefusal)>), AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    let mut s_refused = vec![None; blocks.len()];
    let start = shift_start(ctx, sum, blocks, bounds, est, data, &mut s_refused)?;
    let (shifts, factorizations) = shift_run(
        ctx,
        sum,
        guard,
        structure,
        k,
        ordering,
        factor.scale(),
        blocks,
        gamma,
        &start,
        &mut s_refused,
    )?;
    let (out, _, stop) = certificates(blocks, bounds, est, data, &shifts, &s_refused);
    Ok((out, factorizations, stop))
}

#[cfg(test)]
#[path = "../../../tests/retained_k4/bound_tests.rs"]
mod tests;
