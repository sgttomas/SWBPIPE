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
pub(crate) fn u_pass<const L: usize, F: ProfileLdl<L>>(
    ctx: &mut WideContext<L>,
    sum: &mut ExactWideSum,
    guard: &StageGuard,
    f: &F,
) -> Result<Vec<Wide<L>>, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    let n = f.rows();
    let one = Wide::<L>::ONE;
    let mut a = vec![one; n];
    for i in 0..n {
        let mut acc = one;
        for j in f.first_of(i)..i {
            let l = f.entry(i, j).abs();
            if l.is_zero() {
                continue;
            }
            let t = mul_toward(ctx, sum, &l, &a[j], Toward::Up)?;
            acc = add_toward(ctx, sum, &acc, &t, Toward::Up)?;
        }
        a[i] = acc;
        guard.check(ctx, sum)?;
    }
    let mut c = Vec::with_capacity(n);
    for (i, ai) in a.iter().enumerate() {
        c.push(div_toward(ctx, sum, ai, &f.entry(i, i), Toward::Up)?);
    }
    for i in (0..n).rev() {
        let ci = c[i];
        for j in f.first_of(i)..i {
            let l = f.entry(i, j).abs();
            if l.is_zero() {
                continue;
            }
            let t = mul_toward(ctx, sum, &l, &ci, Toward::Up)?;
            c[j] = add_toward(ctx, sum, &c[j], &t, Toward::Up)?;
        }
        guard.check(ctx, sum)?;
    }
    Ok(c)
}

/// c′ = |L|D|Lᵀ|e, every operation upward (emu7's `nl_pass`).
pub(crate) fn nl_pass<const L: usize, F: ProfileLdl<L>>(
    ctx: &mut WideContext<L>,
    sum: &mut ExactWideSum,
    guard: &StageGuard,
    f: &F,
) -> Result<Vec<Wide<L>>, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    let n = f.rows();
    let mut at = vec![Wide::<L>::ONE; n];
    for i in 0..n {
        for j in f.first_of(i)..i {
            let l = f.entry(i, j).abs();
            if l.is_zero() {
                continue;
            }
            at[j] = add_toward(ctx, sum, &at[j], &l, Toward::Up)?;
        }
    }
    let mut bt = Vec::with_capacity(n);
    for (i, a) in at.iter().enumerate() {
        bt.push(mul_toward(ctx, sum, &f.entry(i, i), a, Toward::Up)?);
    }
    let mut ct = bt.clone();
    for i in 0..n {
        for j in f.first_of(i)..i {
            let l = f.entry(i, j).abs();
            if l.is_zero() {
                continue;
            }
            let t = mul_toward(ctx, sum, &l, &bt[j], Toward::Up)?;
            ct[i] = add_toward(ctx, sum, &ct[i], &t, Toward::Up)?;
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
    })
}

/// 7b per block, from a factor whose rows' blocks are `block_of_row`.
pub(crate) fn uc_bounds<const L: usize, F: ProfileLdl<L>>(
    ctx: &mut WideContext<L>,
    sum: &mut ExactWideSum,
    guard: &StageGuard,
    f: &F,
    block_of_row: &[u32],
    blocks: usize,
    gamma: &Wide<L>,
) -> Result<Vec<BlockBound<L>>, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    let c = u_pass(ctx, sum, guard, f)?;
    let ct = nl_pass(ctx, sum, guard, f)?;
    let u = block_max(&c, block_of_row, blocks);
    let n_l = block_max(&ct, block_of_row, blocks);
    let mut out = Vec::with_capacity(blocks);
    for b in 0..blocks {
        out.push(bounds_from(ctx, sum, gamma, &u[b], &n_l[b])?);
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
pub(crate) fn shifted_factor<const L: usize>(
    ctx: &mut WideContext<L>,
    sum: &ExactWideSum,
    guard: &StageGuard,
    profile: &ScaledProfile<L>,
    sigma: &[Option<Wide<L>>],
) -> Result<ShiftedFactor<L>, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    let n = profile.first.len();
    let first = profile.first.clone();
    let mut rows = profile.rows.clone();
    let mut shifted = vec![None; n];
    for i in 0..n {
        if let Some(s) = &sigma[profile.block_of_row[i] as usize] {
            let at = i - first[i];
            let d = ctx.sub(&rows[i][at], s)?;
            rows[i][at] = d;
            shifted[i] = Some(d);
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
        if pivot.is_zero() || pivot.is_sign_negative() {
            // The block has failed at this σ_c; its later rows are not used.
            failed[profile.block_of_row[i] as usize] = true;
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
}

/// ⌈√n⌉, exactly.
pub(crate) fn ceil_sqrt(n: usize) -> u64 {
    let n = n as u64;
    let mut r = (n as f64).sqrt() as u64;
    while r * r > n {
        r -= 1;
    }
    while r * r < n {
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
    let above = sum.signum() > 0;
    sum.clear();
    Ok(above)
}

/// 7c's schedule for the blocks in `start` (block, σ_c, n_c): at most three
/// shifted factorizations in all, each over the whole matrix with the blocks
/// still failing shifted (their σ halved exactly after a failure). Returns each
/// block's result and the number of factorizations.
pub(crate) fn shift_schedule<const L: usize>(
    ctx: &mut WideContext<L>,
    sum: &mut ExactWideSum,
    guard: &StageGuard,
    profile: &ScaledProfile<L>,
    gamma: &Wide<L>,
    start: &[(usize, Wide<L>, usize)],
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
                },
            )
        })
        .collect();
    let mut current: Vec<(usize, Wide<L>, usize)> = start.to_vec();
    let mut factorizations = 0u8;
    while !current.is_empty() && factorizations < 3 {
        let mut sigma = vec![None; profile.blocks];
        for (b, s, _) in &current {
            sigma[*b] = Some(*s);
        }
        let f = shifted_factor(ctx, sum, guard, profile, &sigma)?;
        factorizations += 1;
        let ct = nl_pass(ctx, sum, guard, &f)?;
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
            if f.failed[b] {
                next.push((b, s.mul_pow2(-1)?, n_c));
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
            let delta = top.mul_pow2(1 - i64::from(ctx.precision()))?;
            let gn = mul_toward(ctx, sum, gamma, &n_l[b], Toward::Up)?;
            let e = add_toward(ctx, sum, &gn, &delta, Toward::Up)?;
            let sp = sub_toward(ctx, sum, &s, &e, Toward::Down)?;
            r.n_l = Some(n_l[b]);
            r.delta = Some(delta);
            r.sigma_prime = Some(sp);
            if !sp.is_zero() && !sp.is_sign_negative() {
                let root = Wide::<L>::from_f64(ceil_sqrt(n_c) as f64)?;
                r.s = Some(div_toward(ctx, sum, &root, &sp, Toward::Up)?);
            }
        }
        current = next;
    }
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
    /// B_c, for a block with data (None: neither bound exists, or no data).
    pub(crate) b: Option<Wide<L>>,
}

/// 7b to 7d for every block: the shift runs only for the blocks with data that
/// need it (`shift_needed`), and B_c is formed for the blocks with data.
/// Returns the certificates and the number of shifted factorizations.
#[allow(dead_code)] // test entry: 7b–7d composed as `verify_state` does
#[allow(clippy::too_many_arguments)]
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
) -> Result<(Vec<BlockCertificate<L>>, u8), AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    let mut start = Vec::new();
    for b in 0..blocks.len() {
        let n_c = blocks.positions[b].len();
        if data[b] && shift_needed(sum, &bounds[b].uc, &est[b], n_c)? {
            start.push((b, sigma_from_estimate(ctx, sum, &est[b])?, n_c));
        }
    }
    let (shifts, factorizations) = if start.is_empty() {
        (Vec::new(), 0)
    } else {
        let profile = scaled_profile(structure, k, ordering, blocks, factor.scale())?;
        shift_schedule(ctx, sum, guard, &profile, gamma, &start)?
    };
    let mut out = Vec::with_capacity(blocks.len());
    for b in 0..blocks.len() {
        let shift = shifts.iter().find(|s| s.0 == b).map(|s| s.1.clone());
        let s = shift.as_ref().and_then(|r| r.s);
        out.push(BlockCertificate {
            bound: bounds[b].clone(),
            est: est[b],
            data: data[b],
            b: if data[b] {
                certified(&bounds[b].uc, &s)
            } else {
                None
            },
            shift,
        });
    }
    Ok((out, factorizations))
}

#[cfg(test)]
#[path = "../../../tests/retained_k4/bound_tests.rs"]
mod tests;
