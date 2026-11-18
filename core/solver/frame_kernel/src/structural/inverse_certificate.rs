//! Certificate arithmetic for the integrity check's inverse-norm step
//! (DEL-04-01, exit criterion 18). No production code calls this module: the
//! check still takes `Q = mul_up(1024, est)` as an upper bound on ‖A⁻¹‖∞ without
//! establishing it, and this module repairs nothing until a later change wires
//! it into the two sites that form Q (`formation_check.rs` and
//! `formation_check/skew.rs`).
//!
//! **What it establishes.** For a symmetric binary64 matrix A (the scaled free
//! matrix the check reads) and a quantity Q, either `Verified { bound: B, .. }`
//! with ‖A⁻¹‖∞ ≤ B ≤ Q, or `NotVerified` with typed reasons. Not verified never
//! means the premise ‖A⁻¹‖∞ ≤ Q is refuted. No path returns `Verified` with
//! B > Q, and the only path to `Verified` that does not run the arithmetic
//! below is the empty matrix (n = 0, see Assembly).
//!
//! **Arithmetic model.** IEEE 754 binary64, round to nearest with ties to even,
//! gradual underflow, no fused multiply-add (no `mul_add`, no `sqrt`).
//! u = 2⁻⁵³, η = 2⁻¹⁰⁷⁵. A finite RN product or quotient is
//! (x∘y)(1 + δ) + e with |δ| ≤ u, |e| ≤ η, δe = 0; a finite RN sum or difference
//! is (x ± y)(1 + δ) with no absolute term (a subnormal sum is exact). The
//! factorization below uses its own RN product, quotient and difference, which
//! accept every finite result (subnormals and rounded zeros included) and
//! refuse NaN and ±∞; a divisor must be finite and strictly positive. The
//! directed operations used for every figure take non-negative operands:
//! `add_up(a, b)` = next_up(RN(a + b)), `mul_up` = next_up(RN(ab)),
//! `div_up` = next_up(RN(a/b)), `sub_dn(a, b)` = next_down(RN(a − b)), with an
//! exact zero operand handled exactly. A finite RN result lies within one step
//! of the exact value on the whole grid, so each is ≥ (respectively ≤) the
//! exact result; overflow gives +∞, which is refused.
//!
//! **Hypothesis.** With n the full dimension and m = 2n + 2, checked integer
//! arithmetic establishes m ≤ 2⁵² (so m·u ≤ 1/2, γ_m ≤ 1, η(1 + γ_m) ≤ 2⁻¹⁰⁷⁴),
//! and so that n, m and every k below convert to binary64 exactly; otherwise
//! the input is unusable. γ = `div_up(mul_up(m, u), sub_dn(1, mul_up(m, u)))`
//! ≥ γ_m = mu/(1 − mu).
//!
//! **Blocks.** The connected components of the graph on free positions whose
//! edges are the nonzero off-diagonal stored values (exact zeros of either sign
//! are not edges), found by an explicit-stack search and numbered by least free
//! position. A is exactly block diagonal under the permutation grouping them,
//! so ‖A⁻¹‖∞ = max_c ‖A_c⁻¹‖∞. In any elimination order, every computed
//! cross-block entry of the loop is an exact (signed) zero (by induction: a
//! cross-block a_ij is 0, each product has a cross-block factor, x·(±0) = ±0,
//! 0 − (±0) = ±0, (±0)/d = ±0), so each block's computed factor is the one the
//! loop computes on A_c alone, and the lemmas apply per block.
//!
//! **The loop** (`factor`): the profile LDLᵀ of
//! `ProfileFactor::factor_structural_profile` (structural.rs), operation for
//! operation, on the certificate's own workspace filled from the stored values
//! exactly as `factor_sparse_structural_profile` fills profile rows. Its only
//! differences: the diagonal is first replaced by d̃_i = RN(a_ii − σ_c), with σ_c
//! the shift of the row's block, and the pivot test is d̂_i > 0. In elimination
//! row i: for j in first[i]..i, s = a_ij, then s = RN(s − RN(w_k·l_jk)) for k in
//! max(first[i], first[j])..j, w_j = s, l_ij = RN(w_j/d̂_j); then d̂_i = d̃_i and
//! d̂_i = RN(d̂_i − RN(w_k·l_ik)) for k in first[i]..i. A refused operation marks
//! its block (`ArithmeticNonfinite`) and stands as 0; a pivot that is not > 0
//! marks its block (non-positive pivot) and is replaced by 1, so later rows
//! continue; by the block argument no other block is affected.
//!
//! **Backward error** (per block c, n_c ≤ n). With L̂, D̂ > 0 computed,
//! Ã_c = A_c − σ_c I + E_c the stored shifted block, ΔA_c = L̂D̂L̂ᵀ − Ã_c:
//! |ΔA_c| ≤ γ_m |L̂|D̂|L̂ᵀ| + 2⁻¹⁰⁷⁴(n + ν_c)·𝟙𝟙ᵀ entrywise, so
//! ‖ΔA_c‖₂ ≤ ‖|ΔA_c|‖∞ ≤ γ·N′_c + β_c, where (each formed upward):
//! N′_c = max over the block of c′_i, with a′_j = 1 + Σ_{i>j} |l_ij|,
//! b′_j = d̂_j·a′_j, c′_i = b′_i + Σ_{j<i} |l_ij|·b′_j (that is ‖|L̂|D̂|L̂ᵀ|‖∞);
//! ν_c = max over the block of d̂_i + Σ_{j<i} |l_ij|·d̂_j (that is ‖|L̂|D̂‖∞ on
//! the block, which bounds every absolute term of the block's rows; only rows
//! of block c contribute to them, and n still bounds their count);
//! β_c = `mul_up(mul_up(2⁻¹⁰⁷⁴, n), add_up(n, ν_c))`. Rounded-to-zero products
//! and quotients are covered by β_c; underflowing differences are exact. The
//! shift's own subtraction: ‖E_c‖₂ ≤ δ_c = `mul_up(2u, max_{i∈c} |d̃_i|)`.
//!
//! **Lemma E** (schedule T). k_c = ⌈√n_c⌉ in exact integers. If the loop runs
//! on Ã_c with every value finite and every d̂_i > 0, and
//! σ′_c = `sub_dn(σ_c, add_up(add_up(mul_up(γ, N′_c), δ_c), β_c))` > 0, then
//! for unit v, vᵀA_c v > σ_c − ‖E_c‖₂ − ‖ΔA_c‖₂ ≥ σ′_c, so
//! ‖A_c⁻¹‖₂ ≤ 1/σ′_c and ‖A_c⁻¹‖∞ ≤ √n_c‖A_c⁻¹‖₂ ≤ S_c = `div_up(k_c, σ′_c)`.
//! The block is verified by E when S_c ≤ Q. Shifts: σ₁ =
//! `div_up(mul_up(2, k_c), Q)`; a block with a non-positive pivot at σ₁ (and only
//! such a block) is refactored once at σ₂ = `div_up(mul_up(1.125, k_c), Q)`. A
//! block with positive pivots but σ′ ≤ 0 or S > Q is not retried at σ₂.
//!
//! **Lemma D**, only for blocks E did not verify: the loop at σ = 0 (d̃_i = a_ii
//! exactly, E_c = 0) gives F = L̂D̂L̂ᵀ = A_c + ΔA_c. Upward:
//! a_i = 1 + Σ_{j<i} |l_ij|·a_j, b_i = a_i/d̂_i, c_j = b_j + Σ_{i>j} |l_ij|·c_i
//! (rows descending), U_c = max c ≥ ‖M(L̂)⁻ᵀD̂⁻¹M(L̂)⁻¹‖∞ ≥ ‖F⁻¹‖∞ (M the
//! comparison matrix, |L̂⁻¹| ≤ M(L̂)⁻¹); N_L,c and β_c from the same factor;
//! t_c = `mul_up(U_c, add_up(mul_up(γ, N_L,c), β_c))`. If t_c < 1, then
//! ‖F⁻¹ΔA_c‖∞ ≤ t_c and ‖A_c⁻¹‖∞ ≤ Uc_c = `div_up(U_c, sub_dn(1, t_c))`. The
//! block is verified by D when Uc_c ≤ Q.
//!
//! **Assembly.** B = max_c B_c, the bound of each block's verifying attempt.
//! The matrix is verified only when every block is; n = 0 is verified with
//! B = 0 when 0 ≤ Q. With n > 0, a Q that is not finite and strictly positive is
//! unusable input.
//!
//! **Returned numbers.** `Verified.bound` is B (an upper bound on ‖A⁻¹‖∞, at
//! most Q). A `CandidateBound` is a block bound (S_c or Uc_c) validly formed by
//! its lemma but exceeding Q: it bounds that block's ‖A_c⁻¹‖∞ and is reported
//! for diagnosis only, never as verified. Block sizes are counts of free
//! positions.
//!
//! **Input.** Three borrowed representations, read without copying the matrix
//! (the certificate's only owned matrix storage is its factor workspace), plus
//! a generic lower-triangle reader:
//! - `certify_dense`: the prepared system's dense `matrix` (`PreparedSystem`),
//!   n rows of n entries over free positions, both triangles. Natural
//!   elimination (the dense Cholesky backend, which has no envelope) derives
//!   first[i] = least j ≤ i with a_ij ≠ 0; a supplied order and envelope are the
//!   dense profile backend's.
//! - `certify_compressed`: `SparsePreparedSystem`'s compressed rows
//!   (`row_starts`, `columns`, `values`) over free positions, both triangles
//!   stored, columns ascending, explicit zeros possible.
//! - `certify_compressed_bits`: the same rows with values as `f64::to_bits`,
//!   as `SharedSparseFactor` keeps them.
//! - `certify_lower`: any `LowerTriangle` reader; symmetry holds by
//!   construction because only one triangle is supplied.
//!
//! `order[i]` is the free position eliminated at elimination row i, and
//! `first[i]` is, in elimination coordinates, the least column of row i's
//! profile (first[i] ≤ i), exactly as `ProfileFactor` holds them. The certificate
//! verifies on entry what it relies on and otherwise returns not verified with
//! `InputUnusable`: shapes and ascending distinct columns, finite values,
//! symmetry (each stored entry numerically equal to its transpose, so zeros of
//! either sign are equal, as the kernel's own input validation holds),
//! `order` a permutation, first[i] ≤ i, and every stored nonzero inside the
//! envelope.
#[cfg(test)]
#[path = "inverse_certificate_tests.rs"]
mod tests;

/// u = 2⁻⁵³.
const UNIT_ROUNDOFF: f64 = f64::from_bits(0x3CA0_0000_0000_0000);
/// 2⁻¹⁰⁷⁴, the least positive subnormal (2η).
const LEAST_SUBNORMAL: f64 = f64::from_bits(1);
/// m = 2n + 2 ≤ 2⁵², which gives m·u ≤ 1/2 and exact conversion of n, m, k.
const HYPOTHESIS_LIMIT: u64 = 1 << 52;

// ------------------------------------------------------------------ results

/// The lemma that bounded a block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum BoundMethod {
    LemmaEShiftedFactorization,
    LemmaDComparisonMatrix,
}

/// Lemma E's shift under schedule T.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum ShiftStep {
    Sigma1,
    Sigma2,
}

/// Why a block (or the input) was not verified. None of these refutes the
/// premise ‖A⁻¹‖∞ ≤ Q.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum NotVerifiedReason {
    LemmaENonpositivePivot,
    LemmaESigmaNonpositive,
    LemmaDNonpositivePivot,
    LemmaDContractionNotBelowOne,
    ArithmeticNonfinite,
    BoundExceedsExistingQuantity,
    /// The input could not be used (`InputDefect` says how); no method ran.
    InputUnusable,
}

/// What made the input unusable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InputDefect {
    /// Row lengths, row starts, or value and column counts inconsistent, or a
    /// column out of range, repeated or not ascending.
    Shape,
    EntryNotFinite,
    /// A stored entry without its transpose, or with a different value there
    /// (zeros of either sign are equal).
    NotSymmetric,
    /// `order` is not a permutation of the free positions (or has the wrong
    /// length), or `first` has the wrong length.
    OrderNotPermutation,
    FirstBeyondRow,
    /// A stored nonzero lies left of its elimination row's `first`.
    EnvelopeOmitsNonzero,
    /// Q is not finite and strictly positive (or, for n = 0, below zero or NaN).
    QuantityNotFinitePositive,
    /// 2n + 2 exceeds 2⁵².
    DimensionBeyondHypothesis,
}

/// Blocks verified by each method.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct BlocksByMethod {
    pub(crate) lemma_e_sigma1: usize,
    pub(crate) lemma_e_sigma2: usize,
    pub(crate) lemma_d: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct VerifiedBound {
    /// B: ‖A⁻¹‖∞ ≤ B ≤ Q.
    pub(crate) bound: f64,
    pub(crate) block_count: usize,
    pub(crate) largest_block: usize,
    pub(crate) by_method: BlocksByMethod,
}

/// The reasons one attempted method (and, for E, shift) failed, with the
/// number of blocks for each.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AttemptReasons {
    pub(crate) method: BoundMethod,
    pub(crate) shift: Option<ShiftStep>,
    pub(crate) reasons: Vec<(NotVerifiedReason, usize)>,
}

/// A block bound validly formed by its lemma that exceeds Q. Diagnostic only.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CandidateBound {
    pub(crate) method: BoundMethod,
    pub(crate) shift: Option<ShiftStep>,
    /// The block's least free position.
    pub(crate) block: usize,
    pub(crate) block_size: usize,
    pub(crate) bound: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct NotVerified {
    pub(crate) block_count: usize,
    pub(crate) largest_block: usize,
    pub(crate) blocks_verified: usize,
    /// `Some` exactly when the reason is `InputUnusable`; then no method ran.
    pub(crate) input: Option<InputDefect>,
    /// Per attempted method and shift, in the order E σ₁, E σ₂, D.
    pub(crate) attempts: Vec<AttemptReasons>,
    pub(crate) candidates: Vec<CandidateBound>,
}

impl NotVerified {
    /// Every reason, `InputUnusable` included.
    pub(crate) fn reasons(&self) -> Vec<NotVerifiedReason> {
        let mut out: Vec<_> = self
            .input
            .map(|_| NotVerifiedReason::InputUnusable)
            .into_iter()
            .collect();
        for attempt in &self.attempts {
            out.extend(attempt.reasons.iter().map(|&(r, _)| r));
        }
        out
    }
}

/// The certificate's verdict: verified with B, or not verified with reasons.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum InverseNormCertificate {
    Verified(VerifiedBound),
    NotVerified(NotVerified),
}

// ---------------------------------------------------------------- the input

/// A symmetric matrix given by its logical lower triangle over free
/// positions. `value(row, column)` with column ≤ row returns the exact stored
/// value, and zero where nothing is stored. Only one triangle is read, so the
/// matrix is symmetric by construction. The certificate reads an entry more
/// than once (validation, blocks, each factorization), and its result is about
/// one matrix only if every read of the same position returns the same value:
/// an implementation must be a fixed stored matrix for the whole call.
pub(crate) trait LowerTriangle {
    fn dimension(&self) -> usize;
    fn value(&self, row: usize, column: usize) -> f64;
}

/// The elimination order and envelope.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Elimination<'a> {
    /// order = identity, first[i] = least j ≤ i with a nonzero a_ij.
    Natural,
    /// `order[i]` the free position of elimination row i; `first[i]` the least
    /// profile column of row i, in elimination coordinates.
    Supplied {
        order: &'a [usize],
        first: &'a [usize],
    },
}

/// A borrowed symmetric matrix read one free row at a time.
trait RowView {
    fn dimension(&self) -> usize;
    /// `visit(column, value)` for each stored entry of free row `row`, both
    /// triangles; a position not visited is zero.
    fn visit_row(&self, row: usize, visit: &mut dyn FnMut(usize, f64));
}

struct DenseView<'a>(&'a [Vec<f64>]);

impl RowView for DenseView<'_> {
    fn dimension(&self) -> usize {
        self.0.len()
    }
    fn visit_row(&self, row: usize, visit: &mut dyn FnMut(usize, f64)) {
        for (column, &value) in self.0[row].iter().enumerate() {
            visit(column, value);
        }
    }
}

trait StoredValue: Copy {
    fn binary64(self) -> f64;
}
impl StoredValue for f64 {
    fn binary64(self) -> f64 {
        self
    }
}
impl StoredValue for u64 {
    fn binary64(self) -> f64 {
        f64::from_bits(self)
    }
}

struct CompressedView<'a, V> {
    row_starts: &'a [usize],
    columns: &'a [usize],
    values: &'a [V],
}

impl<V: StoredValue> RowView for CompressedView<'_, V> {
    fn dimension(&self) -> usize {
        self.row_starts.len() - 1
    }
    fn visit_row(&self, row: usize, visit: &mut dyn FnMut(usize, f64)) {
        for index in self.row_starts[row]..self.row_starts[row + 1] {
            visit(self.columns[index], self.values[index].binary64());
        }
    }
}

struct LowerView<'a>(&'a dyn LowerTriangle);

impl RowView for LowerView<'_> {
    fn dimension(&self) -> usize {
        self.0.dimension()
    }
    fn visit_row(&self, row: usize, visit: &mut dyn FnMut(usize, f64)) {
        for column in 0..self.0.dimension() {
            let value = if column <= row {
                self.0.value(row, column)
            } else {
                self.0.value(column, row)
            };
            visit(column, value);
        }
    }
}

fn validate_dense(matrix: &[Vec<f64>]) -> Result<(), InputDefect> {
    let n = matrix.len();
    if matrix.iter().any(|row| row.len() != n) {
        return Err(InputDefect::Shape);
    }
    if matrix.iter().flatten().any(|v| !v.is_finite()) {
        return Err(InputDefect::EntryNotFinite);
    }
    for i in 0..n {
        for j in 0..i {
            if matrix[i][j] != matrix[j][i] {
                return Err(InputDefect::NotSymmetric);
            }
        }
    }
    Ok(())
}

fn validate_compressed<V: StoredValue>(view: &CompressedView<'_, V>) -> Result<(), InputDefect> {
    let starts = view.row_starts;
    let Some(n) = starts.len().checked_sub(1) else {
        return Err(InputDefect::Shape);
    };
    if starts[0] != 0
        || starts.windows(2).any(|w| w[0] > w[1])
        || starts[n] != view.columns.len()
        || view.values.len() != view.columns.len()
    {
        return Err(InputDefect::Shape);
    }
    for r in 0..n {
        let row = &view.columns[starts[r]..starts[r + 1]];
        if row.iter().any(|&c| c >= n) || row.windows(2).any(|w| w[0] >= w[1]) {
            return Err(InputDefect::Shape);
        }
    }
    if view.values.iter().any(|v| !v.binary64().is_finite()) {
        return Err(InputDefect::EntryNotFinite);
    }
    for r in 0..n {
        for index in starts[r]..starts[r + 1] {
            let c = view.columns[index];
            let transpose = &view.columns[starts[c]..starts[c + 1]];
            match transpose.binary_search(&r) {
                Ok(t) if view.values[starts[c] + t].binary64() == view.values[index].binary64() => {
                }
                _ => return Err(InputDefect::NotSymmetric),
            }
        }
    }
    Ok(())
}

fn validate_lower(matrix: &dyn LowerTriangle) -> Result<(), InputDefect> {
    let n = matrix.dimension();
    for r in 0..n {
        for c in 0..=r {
            if !matrix.value(r, c).is_finite() {
                return Err(InputDefect::EntryNotFinite);
            }
        }
    }
    Ok(())
}

/// The elimination order, its inverse and the envelope, checked against the
/// stored nonzeros.
struct Envelope {
    order: Vec<usize>,
    position: Vec<usize>,
    first: Vec<usize>,
}

fn envelope(view: &dyn RowView, elimination: Elimination<'_>) -> Result<Envelope, InputDefect> {
    let n = view.dimension();
    let (order, first) = match elimination {
        Elimination::Natural => {
            let mut first: Vec<usize> = (0..n).collect();
            for (r, slot) in first.iter_mut().enumerate() {
                view.visit_row(r, &mut |c, v| {
                    if c <= r && v != 0.0 && c < *slot {
                        *slot = c;
                    }
                });
            }
            ((0..n).collect(), first)
        }
        Elimination::Supplied { order, first } => {
            if order.len() != n || first.len() != n {
                return Err(InputDefect::OrderNotPermutation);
            }
            (order.to_vec(), first.to_vec())
        }
    };
    let mut position = vec![usize::MAX; n];
    for (i, &p) in order.iter().enumerate() {
        if p >= n || position[p] != usize::MAX {
            return Err(InputDefect::OrderNotPermutation);
        }
        position[p] = i;
    }
    if first.iter().enumerate().any(|(i, &f)| f > i) {
        return Err(InputDefect::FirstBeyondRow);
    }
    // Each stored pair is visited from both of its rows (symmetry is checked
    // first), so testing the entries of row order[i] left of first[i] covers
    // every pair.
    for i in 0..n {
        let mut omitted = false;
        view.visit_row(order[i], &mut |c, v| {
            if position[c] < first[i] && v != 0.0 {
                omitted = true;
            }
        });
        if omitted {
            return Err(InputDefect::EnvelopeOmitsNonzero);
        }
    }
    Ok(Envelope {
        order,
        position,
        first,
    })
}

// ------------------------------------------------------- directed arithmetic

/// ≥ a + b, for a, b ≥ 0.
fn add_up(a: f64, b: f64) -> f64 {
    if a == 0.0 {
        b
    } else if b == 0.0 {
        a
    } else {
        (a + b).next_up()
    }
}

/// ≥ ab, for a, b ≥ 0.
fn mul_up(a: f64, b: f64) -> f64 {
    if a == 0.0 || b == 0.0 {
        0.0
    } else {
        (a * b).next_up()
    }
}

/// ≥ a/b, for a ≥ 0, b > 0.
fn div_up(a: f64, b: f64) -> f64 {
    if a == 0.0 {
        0.0
    } else {
        (a / b).next_up()
    }
}

/// ≤ a − b.
fn sub_dn(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        a
    } else {
        (a - b).next_down()
    }
}

/// ⌈√x⌉ in exact integers.
fn ceil_sqrt(x: usize) -> usize {
    let r = x.isqrt();
    if r * r < x {
        r + 1
    } else {
        r
    }
}

// ------------------------------------------------------------------- blocks

struct Blocks {
    /// The block of each free position.
    of: Vec<usize>,
    /// The free positions of each block, ascending; blocks numbered by their
    /// least free position.
    members: Vec<Vec<usize>>,
}

fn blocks(view: &dyn RowView) -> Blocks {
    let n = view.dimension();
    let mut of = vec![usize::MAX; n];
    let mut members = Vec::new();
    for start in 0..n {
        if of[start] != usize::MAX {
            continue;
        }
        let id = members.len();
        of[start] = id;
        let mut stack = vec![start];
        let mut component = Vec::new();
        while let Some(a) = stack.pop() {
            component.push(a);
            view.visit_row(a, &mut |c, v| {
                if c != a && v != 0.0 && of[c] == usize::MAX {
                    of[c] = id;
                    stack.push(c);
                }
            });
        }
        component.sort_unstable();
        members.push(component);
    }
    Blocks { of, members }
}

// ------------------------------------------------------------- the loop

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FactorFailure {
    Pivot,
    Nonfinite,
}

/// The loop's result: profile rows in elimination coordinates (l_ij left of
/// the diagonal, d̂_i on it), the shifted diagonals d̃_i, and each block's first
/// failure.
struct Factored {
    first: Vec<usize>,
    rows: Vec<Vec<f64>>,
    shifted: Vec<f64>,
    failure: Vec<Option<FactorFailure>>,
}

impl Factored {
    fn get(&self, i: usize, j: usize) -> f64 {
        self.rows[i][j - self.first[i]]
    }
}

/// The profile LDLᵀ loop with a per-block shift (`shifts[block]`).
fn factor(view: &dyn RowView, env: &Envelope, blocks: &Blocks, shifts: &[f64]) -> Factored {
    let n = env.order.len();
    let first = &env.first;
    // Fill the profile rows as `factor_sparse_structural_profile` does.
    let mut rows: Vec<Vec<f64>> = (0..n).map(|i| vec![0.0; i - first[i] + 1]).collect();
    for i in 0..n {
        let row = &mut rows[i];
        view.visit_row(env.order[i], &mut |c, v| {
            let k = env.position[c];
            if k >= first[i] && k <= i {
                row[k - first[i]] = v;
            }
        });
    }
    let mut failure = vec![None; blocks.members.len()];
    let mut note = |b: usize, kind: FactorFailure| {
        if failure[b].is_none() {
            failure[b] = Some(kind);
        }
    };
    let mut work = vec![0.0; n];
    let mut shifted = vec![0.0; n];
    for i in 0..n {
        let b = blocks.of[env.order[i]];
        let fi = first[i];
        let finite = |x: f64, note: &mut dyn FnMut(usize, FactorFailure)| {
            if x.is_finite() {
                x
            } else {
                note(b, FactorFailure::Nonfinite);
                0.0
            }
        };
        for j in fi..i {
            let mut sum = rows[i][j - fi];
            for k in fi.max(first[j])..j {
                let product = finite(work[k] * rows[j][k - first[j]], &mut note);
                sum = finite(sum - product, &mut note);
            }
            work[j] = sum;
            let divisor = rows[j][j - first[j]];
            // Pivots are positive or replaced by 1, so this always holds.
            let quotient = if divisor > 0.0 && divisor.is_finite() {
                finite(sum / divisor, &mut note)
            } else {
                note(b, FactorFailure::Nonfinite);
                0.0
            };
            rows[i][j - fi] = quotient;
        }
        let tilde = finite(rows[i][i - fi] - shifts[b], &mut note);
        shifted[i] = tilde;
        let mut pivot = tilde;
        for k in fi..i {
            let product = finite(work[k] * rows[i][k - fi], &mut note);
            pivot = finite(pivot - product, &mut note);
        }
        if !(pivot > 0.0) {
            note(b, FactorFailure::Pivot);
            pivot = 1.0;
        }
        rows[i][i - fi] = pivot;
    }
    Factored {
        first: first.clone(),
        rows,
        shifted,
        failure,
    }
}

/// The upward passes on block `b`'s rows (operands non-negative; exact-zero
/// multipliers skipped, since adding zero changes nothing).
struct Passes {
    /// N′ (or N_L at σ = 0): max of ‖|L̂|D̂|L̂ᵀ|‖∞'s row sums.
    n_prime: f64,
    /// ν: max of |L̂|D̂'s row sums.
    nu: f64,
    /// U: max of M(L̂)⁻ᵀD̂⁻¹M(L̂)⁻¹e.
    u: f64,
    /// max |d̃_i| over the block.
    shifted_max: f64,
}

fn passes(f: &Factored, env: &Envelope, blocks: &Blocks, b: usize) -> Passes {
    let n = env.order.len();
    let mine: Vec<usize> = (0..n).filter(|&i| blocks.of[env.order[i]] == b).collect();
    let lower = |i: usize| {
        (f.first[i]..i).filter_map(move |j| {
            let l = f.get(i, j);
            debug_assert!(
                l == 0.0 || blocks.of[env.order[j]] == b,
                "cross-block entry is not an exact zero"
            );
            (l != 0.0).then_some((j, l.abs()))
        })
    };
    let d = |i: usize| f.get(i, i);
    let mut a_col = vec![0.0; n];
    for &i in &mine {
        a_col[i] = 1.0;
    }
    for &i in &mine {
        for (j, l) in lower(i) {
            a_col[j] = add_up(a_col[j], l);
        }
    }
    let mut b_col = vec![0.0; n];
    for &i in &mine {
        b_col[i] = mul_up(d(i), a_col[i]);
    }
    let mut n_prime = 0.0_f64;
    let mut nu = 0.0_f64;
    for &i in &mine {
        let mut row = b_col[i];
        let mut row_nu = d(i);
        for (j, l) in lower(i) {
            row = add_up(row, mul_up(l, b_col[j]));
            row_nu = add_up(row_nu, mul_up(l, d(j)));
        }
        n_prime = n_prime.max(row);
        nu = nu.max(row_nu);
    }
    let mut a = vec![0.0; n];
    for &i in &mine {
        let mut acc = 1.0;
        for (j, l) in lower(i) {
            acc = add_up(acc, mul_up(l, a[j]));
        }
        a[i] = acc;
    }
    let mut c = vec![0.0; n];
    for &i in &mine {
        c[i] = div_up(a[i], d(i));
    }
    for &i in mine.iter().rev() {
        for (j, l) in lower(i) {
            c[j] = add_up(c[j], mul_up(l, c[i]));
        }
    }
    let u = mine.iter().fold(0.0_f64, |m, &i| m.max(c[i]));
    let shifted_max = mine.iter().fold(0.0_f64, |m, &i| m.max(f.shifted[i].abs()));
    Passes {
        n_prime,
        nu,
        u,
        shifted_max,
    }
}

// ------------------------------------------------------------- the procedure

#[derive(Debug, Clone, Copy, PartialEq)]
enum AttemptOutcome {
    Verified(f64),
    /// The reason, and the candidate bound when one was validly formed.
    NotVerified(NotVerifiedReason, Option<f64>),
}

/// One attempt on one block, with the figures it formed, by name, in order.
#[derive(Debug, Clone)]
struct AttemptRecord {
    method: BoundMethod,
    shift: Option<ShiftStep>,
    outcome: AttemptOutcome,
    figures: Vec<(&'static str, f64)>,
}

#[derive(Debug, Clone)]
struct BlockRecord {
    least: usize,
    size: usize,
    k: usize,
    attempts: Vec<AttemptRecord>,
}

impl BlockRecord {
    /// The verifying attempt and its bound.
    fn verified(&self) -> Option<(&AttemptRecord, f64)> {
        self.attempts.iter().find_map(|a| match a.outcome {
            AttemptOutcome::Verified(bound) => Some((a, bound)),
            AttemptOutcome::NotVerified(..) => None,
        })
    }
}

/// The verdict with every block's attempts (the verdict is formed from them).
struct Run {
    result: InverseNormCertificate,
    blocks: Vec<BlockRecord>,
}

fn unusable(defect: InputDefect) -> Run {
    Run {
        result: InverseNormCertificate::NotVerified(NotVerified {
            block_count: 0,
            largest_block: 0,
            blocks_verified: 0,
            input: Some(defect),
            attempts: Vec::new(),
            candidates: Vec::new(),
        }),
        blocks: Vec::new(),
    }
}

fn run(
    view: &dyn RowView,
    valid: Result<(), InputDefect>,
    elimination: Elimination<'_>,
    q: f64,
) -> Run {
    if let Err(defect) = valid {
        return unusable(defect);
    }
    let env = match envelope(view, elimination) {
        Ok(env) => env,
        Err(defect) => return unusable(defect),
    };
    let n = env.order.len();
    if n == 0 {
        return if 0.0 <= q {
            Run {
                result: InverseNormCertificate::Verified(VerifiedBound {
                    bound: 0.0,
                    block_count: 0,
                    largest_block: 0,
                    by_method: BlocksByMethod::default(),
                }),
                blocks: Vec::new(),
            }
        } else {
            unusable(InputDefect::QuantityNotFinitePositive)
        };
    }
    if !(q.is_finite() && q > 0.0) {
        return unusable(InputDefect::QuantityNotFinitePositive);
    }
    let m = match u64::try_from(n)
        .ok()
        .and_then(|n| n.checked_mul(2))
        .and_then(|x| x.checked_add(2))
    {
        Some(m) if m <= HYPOTHESIS_LIMIT => m,
        _ => return unusable(InputDefect::DimensionBeyondHypothesis),
    };
    // Exact: m ≤ 2⁵², n < m, and every k ≤ n.
    let m_f = m as f64;
    let n_f = n as f64;
    let mu = mul_up(m_f, UNIT_ROUNDOFF);
    let gamma_den = sub_dn(1.0, mu);
    let gamma = div_up(mu, gamma_den);
    if !(gamma_den > 0.0 && gamma.is_finite()) {
        return unusable(InputDefect::DimensionBeyondHypothesis);
    }
    let blocks = blocks(view);
    let mut records: Vec<BlockRecord> = blocks
        .members
        .iter()
        .map(|mem| BlockRecord {
            least: mem[0],
            size: mem.len(),
            k: ceil_sqrt(mem.len()),
            attempts: Vec::new(),
        })
        .collect();
    let beta = |nu: f64| mul_up(mul_up(LEAST_SUBNORMAL, n_f), add_up(n_f, nu));
    let count = records.len();

    // Lemma E under schedule T.
    let lemma_e = |shift: ShiftStep,
                   sigmas: &[f64],
                   which: &[usize],
                   records: &mut Vec<BlockRecord>| {
        let f = factor(view, &env, &blocks, sigmas);
        for &b in which {
            let sigma = sigmas[b];
            let mut figures = vec![("sigma", sigma)];
            let outcome = match f.failure[b] {
                Some(FactorFailure::Pivot) => {
                    AttemptOutcome::NotVerified(NotVerifiedReason::LemmaENonpositivePivot, None)
                }
                Some(FactorFailure::Nonfinite) => {
                    AttemptOutcome::NotVerified(NotVerifiedReason::ArithmeticNonfinite, None)
                }
                None => {
                    let p = passes(&f, &env, &blocks, b);
                    let delta = mul_up(2.0 * UNIT_ROUNDOFF, p.shifted_max);
                    let beta_c = beta(p.nu);
                    let loss = add_up(add_up(mul_up(gamma, p.n_prime), delta), beta_c);
                    figures.extend([
                        ("gamma", gamma),
                        ("n_prime", p.n_prime),
                        ("nu", p.nu),
                        ("delta", delta),
                        ("beta", beta_c),
                        ("loss", loss),
                    ]);
                    if ![p.n_prime, p.nu, delta, beta_c, loss]
                        .iter()
                        .all(|x| x.is_finite())
                    {
                        AttemptOutcome::NotVerified(NotVerifiedReason::ArithmeticNonfinite, None)
                    } else {
                        let sigma_prime = sub_dn(sigma, loss);
                        figures.push(("sigma_prime", sigma_prime));
                        if !(sigma_prime > 0.0) {
                            AttemptOutcome::NotVerified(
                                NotVerifiedReason::LemmaESigmaNonpositive,
                                None,
                            )
                        } else {
                            let s = div_up(records[b].k as f64, sigma_prime);
                            figures.push(("s", s));
                            if !s.is_finite() {
                                AttemptOutcome::NotVerified(
                                    NotVerifiedReason::ArithmeticNonfinite,
                                    None,
                                )
                            } else if s <= q {
                                AttemptOutcome::Verified(s)
                            } else {
                                AttemptOutcome::NotVerified(
                                    NotVerifiedReason::BoundExceedsExistingQuantity,
                                    Some(s),
                                )
                            }
                        }
                    }
                }
            };
            records[b].attempts.push(AttemptRecord {
                method: BoundMethod::LemmaEShiftedFactorization,
                shift: Some(shift),
                outcome,
                figures,
            });
        }
    };
    let sigma1: Vec<f64> = records
        .iter()
        .map(|r| div_up(mul_up(2.0, r.k as f64), q))
        .collect();
    let all: Vec<usize> = (0..count).collect();
    lemma_e(ShiftStep::Sigma1, &sigma1, &all, &mut records);
    let retry: Vec<usize> = (0..count)
        .filter(|&b| {
            records[b].attempts[0].outcome
                == AttemptOutcome::NotVerified(NotVerifiedReason::LemmaENonpositivePivot, None)
        })
        .collect();
    if !retry.is_empty() {
        let sigma2: Vec<f64> = (0..count)
            .map(|b| {
                if retry.contains(&b) {
                    div_up(mul_up(1.125, records[b].k as f64), q)
                } else {
                    sigma1[b]
                }
            })
            .collect();
        lemma_e(ShiftStep::Sigma2, &sigma2, &retry, &mut records);
    }

    // Lemma D, for the blocks E did not verify.
    let need_d: Vec<usize> = (0..count)
        .filter(|&b| records[b].verified().is_none())
        .collect();
    if !need_d.is_empty() {
        let f = factor(view, &env, &blocks, &vec![0.0; count]);
        for &b in &need_d {
            let mut figures = vec![("sigma", 0.0)];
            let outcome = match f.failure[b] {
                Some(FactorFailure::Pivot) => {
                    AttemptOutcome::NotVerified(NotVerifiedReason::LemmaDNonpositivePivot, None)
                }
                Some(FactorFailure::Nonfinite) => {
                    AttemptOutcome::NotVerified(NotVerifiedReason::ArithmeticNonfinite, None)
                }
                None => {
                    let p = passes(&f, &env, &blocks, b);
                    let beta_c = beta(p.nu);
                    let t = mul_up(p.u, add_up(mul_up(gamma, p.n_prime), beta_c));
                    figures.extend([
                        ("gamma", gamma),
                        ("u", p.u),
                        ("n_l", p.n_prime),
                        ("nu", p.nu),
                        ("beta", beta_c),
                        ("t", t),
                    ]);
                    if ![p.u, p.n_prime, p.nu, beta_c, t]
                        .iter()
                        .all(|x| x.is_finite())
                    {
                        AttemptOutcome::NotVerified(NotVerifiedReason::ArithmeticNonfinite, None)
                    } else if !(t < 1.0) {
                        AttemptOutcome::NotVerified(
                            NotVerifiedReason::LemmaDContractionNotBelowOne,
                            None,
                        )
                    } else {
                        let den = sub_dn(1.0, t);
                        figures.push(("den", den));
                        if !(den > 0.0) {
                            AttemptOutcome::NotVerified(
                                NotVerifiedReason::LemmaDContractionNotBelowOne,
                                None,
                            )
                        } else {
                            let uc = div_up(p.u, den);
                            figures.push(("uc", uc));
                            if !uc.is_finite() {
                                AttemptOutcome::NotVerified(
                                    NotVerifiedReason::ArithmeticNonfinite,
                                    None,
                                )
                            } else if uc <= q {
                                AttemptOutcome::Verified(uc)
                            } else {
                                AttemptOutcome::NotVerified(
                                    NotVerifiedReason::BoundExceedsExistingQuantity,
                                    Some(uc),
                                )
                            }
                        }
                    }
                }
            };
            records[b].attempts.push(AttemptRecord {
                method: BoundMethod::LemmaDComparisonMatrix,
                shift: None,
                outcome,
                figures,
            });
        }
    }
    Run {
        result: assemble(&records, q),
        blocks: records,
    }
}

fn assemble(records: &[BlockRecord], q: f64) -> InverseNormCertificate {
    let block_count = records.len();
    let largest_block = records.iter().map(|r| r.size).max().unwrap_or(0);
    let mut by_method = BlocksByMethod::default();
    let mut bound = 0.0_f64;
    let mut verified = 0;
    for record in records {
        if let Some((attempt, b)) = record.verified() {
            verified += 1;
            bound = bound.max(b);
            match (attempt.method, attempt.shift) {
                (BoundMethod::LemmaEShiftedFactorization, Some(ShiftStep::Sigma2)) => {
                    by_method.lemma_e_sigma2 += 1
                }
                (BoundMethod::LemmaEShiftedFactorization, _) => by_method.lemma_e_sigma1 += 1,
                (BoundMethod::LemmaDComparisonMatrix, _) => by_method.lemma_d += 1,
            }
        }
    }
    if verified == block_count && bound <= q {
        return InverseNormCertificate::Verified(VerifiedBound {
            bound,
            block_count,
            largest_block,
            by_method,
        });
    }
    let kinds = [
        (
            BoundMethod::LemmaEShiftedFactorization,
            Some(ShiftStep::Sigma1),
        ),
        (
            BoundMethod::LemmaEShiftedFactorization,
            Some(ShiftStep::Sigma2),
        ),
        (BoundMethod::LemmaDComparisonMatrix, None),
    ];
    let mut attempts = Vec::new();
    let mut candidates = Vec::new();
    for (method, shift) in kinds {
        let mut reasons: Vec<(NotVerifiedReason, usize)> = Vec::new();
        let mut attempted = false;
        for record in records {
            for attempt in record
                .attempts
                .iter()
                .filter(|a| a.method == method && a.shift == shift)
            {
                attempted = true;
                if let AttemptOutcome::NotVerified(reason, candidate) = attempt.outcome {
                    match reasons.iter_mut().find(|(r, _)| *r == reason) {
                        Some((_, n)) => *n += 1,
                        None => reasons.push((reason, 1)),
                    }
                    if let Some(c) = candidate {
                        candidates.push(CandidateBound {
                            method,
                            shift,
                            block: record.least,
                            block_size: record.size,
                            bound: c,
                        });
                    }
                }
            }
        }
        if attempted {
            reasons.sort();
            attempts.push(AttemptReasons {
                method,
                shift,
                reasons,
            });
        }
    }
    InverseNormCertificate::NotVerified(NotVerified {
        block_count,
        largest_block,
        blocks_verified: verified,
        input: None,
        attempts,
        candidates,
    })
}

// ------------------------------------------------------------- entry points

fn dense_run(matrix: &[Vec<f64>], elimination: Elimination<'_>, q: f64) -> Run {
    run(&DenseView(matrix), validate_dense(matrix), elimination, q)
}

fn compressed_run<V: StoredValue>(
    row_starts: &[usize],
    columns: &[usize],
    values: &[V],
    order: &[usize],
    first: &[usize],
    q: f64,
) -> Run {
    let view = CompressedView {
        row_starts,
        columns,
        values,
    };
    match validate_compressed(&view) {
        Ok(()) => run(&view, Ok(()), Elimination::Supplied { order, first }, q),
        Err(defect) => unusable(defect),
    }
}

fn lower_run(matrix: &dyn LowerTriangle, elimination: Elimination<'_>, q: f64) -> Run {
    run(&LowerView(matrix), validate_lower(matrix), elimination, q)
}

/// The dense prepared matrix (`PreparedSystem::matrix`): n rows of n binary64
/// entries over free positions, both triangles. `Elimination::Natural` for the
/// dense Cholesky backend, or the dense profile backend's `order` and `first`.
pub(crate) fn certify_dense(
    matrix: &[Vec<f64>],
    elimination: Elimination<'_>,
    q: f64,
) -> InverseNormCertificate {
    dense_run(matrix, elimination, q).result
}

/// The sparse prepared matrix (`SparsePreparedSystem`'s `row_starts`,
/// `columns`, `values`): compressed rows over free positions, both triangles
/// stored, columns ascending, explicit zeros possible; with the profile
/// factor's `order` and `first`.
pub(crate) fn certify_compressed(
    row_starts: &[usize],
    columns: &[usize],
    values: &[f64],
    order: &[usize],
    first: &[usize],
    q: f64,
) -> InverseNormCertificate {
    compressed_run(row_starts, columns, values, order, first, q).result
}

/// The kept factor's matrix (`SharedSparseFactor`'s `row_starts`, `columns`,
/// and `values` as `f64::to_bits`), with the factor's `order` and `first`.
pub(crate) fn certify_compressed_bits(
    row_starts: &[usize],
    columns: &[usize],
    values: &[u64],
    order: &[usize],
    first: &[usize],
    q: f64,
) -> InverseNormCertificate {
    compressed_run(row_starts, columns, values, order, first, q).result
}

/// Any lower-triangle reader (symmetric by construction).
pub(crate) fn certify_lower(
    matrix: &dyn LowerTriangle,
    elimination: Elimination<'_>,
    q: f64,
) -> InverseNormCertificate {
    lower_run(matrix, elimination, q).result
}
