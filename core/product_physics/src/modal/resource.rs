//! The modal resource guard's combined preflight P0 (DEL-04-09 Design
//! §5.3.8, "Where each check sits"), and the budget form it is judged
//! against. **No weight, allotment or ceiling is adopted here** (Agent 0,
//! 2026-11-04): [`ModalResourceBudget`] has no `Default` and no constructor
//! with values; only tests build one, and it is labelled test-local.
//!
//! **What P0 is.** In `run_modal_dense`, after `prepare_modal` and before
//! the state case's solve and any n²-sized allocation, P0 forms a count c̄_t
//! for every cost term of §5.3.8's table from request, model and parameter
//! inputs only. **Every bounded c̄_t is an upper bound on the work it
//! charges, in its stated unit, derived from source for every admitted
//! input** (the derivations follow). A term for which no such bound is
//! derived is *unbounded*, and P0 refuses any request in which it occurs.
//! An allotment that a later stage would enforce (R_max at S2, L_A at S2b)
//! is never charged as if it were a bound: those stages are not built.
//! Then, in checked u128 arithmetic:
//! - W = Σ_t w_t·c̄_t, with w_t integer picoseconds per counted unit;
//! - the stage allotments A_s = Σ_{t∈s} w_t·c̄_t, for the stage s at which
//!   t's count becomes exact (S1 in the kernel; S2, S2b and the confirmation
//!   counter are not built, so their allotments are recorded and enforced by
//!   nothing yet);
//! - Σ_s A_s ≤ W_budget, inclusive.
//!
//! W is a calibrated weighted-work budget in nominal time units, not a
//! runtime guarantee. A count is in its own unit, never a calibrated flop
//! proxy; §5.3.8's w_b64 and w_screen units (19n³/3 + 6n·R, n³/3) are
//! leading-order calibration units, not upper bounds, and are not used.
//!
//! **Refusals**, each blocking and in the approved MODAL namespace:
//! - `MODAL_RESOURCE_PREFLIGHT_EXCEEDED`: a term is inadmissible by
//!   arithmetic (unbounded, or a nonzero count with an undetermined weight:
//!   the approved weight-or-refuse rule), naming every such term; or W
//!   exceeds W_budget, naming the dominant term, the largest w_t·c̄_t;
//! - `MODAL_RESOURCE_ARITHMETIC_OVERFLOW`: a count, a product or a sum
//!   overflows u128. Nothing wraps or saturates.
//!
//! **The capability boundary today.** T2's integrity work is unbounded and
//! occurs in every request, so **P0 refuses every budgeted request**, naming
//! the unbounded terms. That is the approved weight-or-refuse boundary,
//! stated plainly: P0 admits nothing until those terms are bounded. A
//! request without a budget is unaffected.
//!
//! **Dimensions** (§5.3.8's symbols; each an upper bound from source):
//! - N, the nodes, and n_g = 6N.
//! - n̄_f, the pencil order. `reduce_assembled_sparse_system` (frame_kernel
//!   `structural/sparse.rs:851`) takes the free DOFs as exactly the
//!   complement in 0..n_g of the restrained set it is given, and refuses an
//!   out-of-range or repeated DOF. That set is the prepared linear one, or
//!   with nonlinear supports a superset of it (`state::linearize_modal_
//!   supports` only adds rigid states). So n_f ≤ n_g − |distinct linear
//!   restrained DOFs in 0..n_g|. With a restraint basis, n̄_f = n_g.
//! - n̄_c = n̄_f: the computed pairs are at most the condensed order.
//! - n̄_r = n̄_f: n_r ≤ n_c ≤ n_f. R_max (⌊R_max/6N⌋) is enforced only at S2,
//!   which is not built, so it is recorded and not used. n_r is never
//!   assumed ≤ `mode_count`: cluster completion can exceed it.
//! - p̄ = min(n̄_f, A_band): the attempted band pairs are among the condensed
//!   μ, and S1-band refuses more than A_band before any band work.
//! - R̄ = A_QL: S1-QL refuses before a sweep that would pass it.
//! - s = s_max: `sturm::run_candidates` takes at most `max_candidates`
//!   shifts in either placement, and one placement runs per request.
//! - L̄ = [`MODAL_RESOURCE_LIMB_BOUND`] limbs, derived below. L_A is enforced
//!   only at S2b, which is not built, so it is recorded and not used.
//! - D_K = Σ_K d_e², D_M = Σ_M d_e², E: from the re-formation's element
//!   sizes (`reform_element_stiffness`/`_mass`, modal_eigen
//!   `reformation/mod.rs:263`, `:357`): 12 × 12 for straights, arcs,
//!   connectors and tie rods; an arc's mass two 6 × 6 lumps; a body 6 × 6;
//!   an axis spring 1 × 1; a skewed spring 3 × 3. Over every built pipe,
//!   arc, connector, tie rod and spring, and one body per mass-bearing pipe
//!   segment (`mass::assemble_mass` keys bodies by pipe):
//!   D_K ≤ 144·(pipes + arcs + connectors + tie rods) + springs + 9·skewed;
//!   D_M ≤ 144·pipes + 72·arcs + 36·segments; E ≤ pipes + arcs + segments +
//!   connectors + tie rods + springs + skewed.
//! - μ, the exact-product calls per element entry: 1 without a restraint
//!   basis (one `add_dd_product`), 36 with one (`product_into` on a mixed
//!   element: ≤ 3 × 3 coordinate pairs × 2 dd words × 2 calls for a factor).
//!   A call is at most 8 parts (`add_product4_within`, four factors), so
//!   Q = 8μ parts per element entry; a re-formed position per element entry
//!   is at most μ.
//!
//! **L̄, the limb span of every `ExactSum`** (modal_eigen `exact.rs`). An
//! add of a part p at scale σ touches the three limbs from ⌊(e(p) + σ −
//! 52)/32⌋ up (reserving a fourth), and `low` only decreases; the length never shrinks; a carry pass
//! pushes a limb only while the top limb is ≥ 2³¹ in magnitude. So for an
//! accumulator whose parts have representation bits in [B_lo, B_hi] and
//! fewer than 2^c adds, the length is at most
//! ⌊(B_hi + c)/32⌋ + 4 − ⌊B_lo/32⌋ + 1.
//! - *Parts.* A product of binary64 factors x_i (e_i ∈ [−1074, 1023]) is
//!   split into exact TwoProduct parts. Each part is a nonzero piece of the
//!   exact product, so its bits lie at or above the product's lowest bit,
//!   ≥ Σ(e_i − 52), and its representation lsb is at most 52 below that;
//!   the product is below 2^(Σe_i + k), so a part's top bit is ≤ Σe_i + k.
//! - *Level 1* (sums of products of binary64 values and dd words): at most
//!   five binary64 factors per product (`product_into` on a mixed element
//!   with a factor: q_a, q_b, the word, λ, x). B_lo = 5·(−1126) − 52 =
//!   −5682; B_hi = 5·1024 = 5120.
//! - *Level 2* (`add_terms_product`: the terms of a level-1 sum times at
//!   most two binary64 factors; `reformation/mod.rs:1233`,
//!   `orthogonality.rs:176`; the formation difference adds level-1 terms
//!   directly, `reformation/mod.rs:1018`): a term is an integer limb below
//!   2³² at 2^(32(low₁ + k)), so B_lo = 32·low₁ − 2·1126 − 52 and B_hi =
//!   32·(top₁ + 1) + 2·1024. No level-2 sum's terms are multiplied again.
//! - *Adds:* every add is a part of a call counted in a term here (≤ 8 parts
//!   a call), and P0 refuses a count ≥ 2¹²⁸, so fewer than 2¹³¹ < 2^c adds,
//!   c = 132.
//! - Hence L̄₁ = 168 + 178 + 1 = 347 and L̄ = L̄₂ = 241 + 250 + 1 = 492 limbs,
//!   for every accumulator; terms() of a sum is at most its length.
//! - *Limb operations:* one part's add moves at most 5L̄ + 16 limbs (a
//!   downward growth: reallocation, resize, shift and fill; or an upward
//!   resize; three chunk updates; the carry pass every 2²⁸ adds, amortized),
//!   and one read (`to_wide_with`, `terms_into`) at most 12L̄ + 32 (copy,
//!   two carry passes, negation, trim, scan and the term list).
//!
//! **The terms** (counted unit; derivation; stage). n = n̄_f throughout.
//! - T1, only with nonlinear supports (the state case's solve):
//!   S̄ = (I_as + 1)·(2 + d̄) + 1 solves. I_as is the product's active-set
//!   cap (`DEC_046_PRODUCT_PREVIEW_ACTIVE_SET_MAX_ITERATIONS`, lib.rs:197);
//!   dense mode solves once per iteration without derived friction and
//!   2 + d with it (nonlinear_integration `lib.rs:1639`–`:1795`); the
//!   all-active recovery trial of iteration 1 adds one more step; the +1 is
//!   the gate factorization. d̄ is the friction supports (`:2131`). Each
//!   solve factors through frame_kernel's `cholesky` (`structural.rs:2063`,
//!   via `solve_structural_dense_binary64`/`factor_structural_cholesky`).
//!   - `T1.factor` = S̄ × T2.factor's count; `T1.entries` = S̄ × T2.entries'
//!     (P0);
//!   - `T1.coupling` = (I_as + 1)·(⌈2d̄³/3⌉ + 2d̄² + d̄·n_g) flops and entry
//!     steps: `solve_dense`, the influence and coupling matrices and the
//!     unit-force vectors (P0);
//!   - `T1.integrity`: **unbounded**, each solve's M03 integrity work, as
//!     T2.integrity;
//!   - `T1.confirmation`: **unbounded**, the static confirmation's
//!     evaluations E_conf, whose A_conf counter is a held NI seam.
//! - T2, the static screen (`stiffness.rs`, the dense structural solve):
//!   - `T2.factor` = (n − 1)n(n + 1)/6 Cholesky inner iterations, exactly
//!     the k-loop of `cholesky` (`structural.rs:2089`–`:2095`), each one
//!     checked product, one checked subtraction and one checked absolute
//!     addition (P0);
//!   - `T2.entries` = 3n² entry steps: n(n + 1)/2 factor entries (each one
//!     read and a checked division, or a square root with its pivot
//!     screen), the n² finiteness scan, the n(n − 1)/2 symmetry scan and the
//!     n² factor allocation (P0);
//!   - `T2.integrity`: **unbounded.** The M03-INTEGRITY work around the
//!     factor: the `SparseAssemblyEvidence` construction, the dense copy and
//!     symmetry view, `prepare_bound`'s radix scaling, exact right-hand
//!     sides (`ExactAccumulator`) and skew audit, `audit_contributions`
//!     (`Expansion` sums and differences, each ≤ 2,099 components since its
//!     parts are nonoverlapping, zero-free binary64), the triangular solves,
//!     residual records, condition estimate and finish. Its pass counts are
//!     not derived in this slice.
//! - `T3.kernel`: kernel steps of `solve_dense_pencil_with_allotments`,
//!   counted loop by loop (see [`modal_resource_kernel_steps`]). A step is
//!   one execution of an innermost loop body or one element of a fill,
//!   copy, swap or scan, at most 24 floating-point operations, comparisons
//!   or moves. R enters as 2R̄(n + 1): each QL iteration has at least one
//!   rotation, and each rotation is a scalar body plus a row update of n.
//!   Band recovery is T4 (S1).
//! - T4, band recovery (`dense.rs:759`–`:880`; S1): `T4.scaling` = 2n² + n
//!   WideDd entry steps when p̄ > 0; `T4.visits` = 4p̄n² exact-product calls
//!   (2n² in `recover_band_mode`, 2n² in `formed_pair_errors`);
//!   `T4.insertions` = 4p̄n² (each call may insert); `T4.rows` = p̄(n + 2)
//!   WideDd row steps; `T4.limbs` = p̄·(16n²·(5L̄+16) + (n + 2)·(12L̄+32))
//!   limb operations.
//! - T5 (S2): `T5.census` = 16·n̄_c steps of linear passes over the computed
//!   pairs (the eigenvalue copy, `cluster_ids`, frequencies,
//!   `reported_set`, estimates, the evidence rows, the cut-off
//!   disclosures); `T5.outcome` = n̄_c·p̄ + 2n̄_c² + n̄_c steps of the
//!   outcome's band flags and cluster ranges (`outcome.rs`).
//! - T6 (S2): `T6.visits` = n̄_r(n² + 3n) exact-product calls (one
//!   `mass_bilinear` and three `wide_dot`s per mode,
//!   `modal_quantities.rs:591`, `:625`); `T6.limbs` =
//!   n̄_r·(4(n² + 3n)·(5L̄+16) + 4·(12L̄+32)).
//! - T7 (S2): `T7.elements` = E element re-formations (fixed-size p128 work
//!   each); `T7.assembly`: **unbounded** (the formation maps are
//!   `BTreeMap`s and the formation difference binary-searches the formed
//!   list: std containers whose step constants are not repository source);
//!   `T7.limbs` = (5L̄+16)·(Q(D_K + D_M) + n(n + 1) + L̄·μ(D_K + D_M)) plus
//!   (12L̄+32)·(2μ(D_K + D_M) + n(n + 1)).
//! - T8, residuals of every computed pair (S2b): `T8.insertions` =
//!   n̄_c·μ(D_K + 2D_M) calls (K̂x, λM̂x, exact M̂x); `T8.generalized_mass` =
//!   n̄_c·n·L̄ calls; `T8.rows` = n̄_c(2n + 1) WideDd row steps; `T8.limbs` =
//!   n̄_c·((5L̄+16)·(Q(D_K + 2D_M) + 4nL̄) + (12L̄+32)·(2n + 1)).
//! - T9, Sturm evidence at the dense profile, P = n(n + 1)/2 (S2):
//!   `T9.construction`: **unbounded** (`ShiftPencil::from_entries` sorts up
//!   to 2P merged records, `sturm.rs:571`, and `reverse_cuthill_mckee` sorts
//!   every adjacency list twice, `sparse_direct/src/lib.rs:549`, `:555`: std
//!   sorts, documented only as O(m log m). The pseudo-peripheral search
//!   alone is bounded: it repeats BFS only while the eccentricity strictly
//!   grows, so at most n + 1 traversals); `T9.factorization` =
//!   s·(n(n − 1)(n − 2)/6 + n²) LDLᵀ steps (`factorize_ldlt`,
//!   `sparse_direct/src/lib.rs:408`: per row of width w ≤ i,
//!   w(w − 1)/2 + 2w + 1); `T9.passes` = s·(29P + 98n + 70) steps (the
//!   formed list, profile build, factor clone and observations, ≤ 11
//!   ordered solves of 2P + 8n, separations, gap positions);
//!   `T9.neighbours` = 2(P + 3n) calls and checks; `T9.limbs` =
//!   2·((5L̄+16)·8P + 2(12L̄+32)).
//! - T10, orthogonality of the published set (S2b): `T10.products` =
//!   n̄_r·μ·D_M calls; `T10.bilinear` = n̄_r(n̄_r + 1)/2·n·L̄ calls;
//!   `T10.limbs` = n̄_r((5L̄+16)·Q·D_M + (12L̄+32)·n) + n̄_r(n̄_r + 1)/2·
//!   ((5L̄+16)·4nL̄ + 12L̄+32).
//!
//! **Not charged here:** T11, the projection (it runs in the document path
//! after `run_modal_dense`, and its count needs B_doc, still proposed), and
//! work outside §5.3.8's table (K's formation, the mass assembly, the
//! pencil's n² fill, the outcome's per-mode O(n_g) publication). **Memory**
//! is an open item: the option-(c) ledger's rows still carry O(·) terms and
//! approximate constants, so no memory check is made.
use super::integrity::{ModalIntegrityParameters, MODAL_RESOURCE_ARITHMETIC_OVERFLOW, MODAL_RESOURCE_PREFLIGHT_EXCEEDED};
use super::prepare::Prepared;
use crate::{diag, Diagnostic, DOF_PER_NODE};
use open_pipe_stress_modal_eigen::dense::DenseAllotments;
use open_pipe_stress_nonlinear_supports::NonlinearSupportBehavior;
use std::collections::BTreeSet;

/// The stage at which a term's count becomes exact (module docs).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ModalResourceStage {
    /// Exact at P0 from its inputs.
    Preflight,
    /// The kernel's S1 (built): A_QL and A_band.
    Kernel,
    /// T1's confirmation counter (A_conf; not built, a held NI seam).
    Confirmation,
    /// S2, after the reported set (not built).
    ReportedSet,
    /// S2b, after the re-formation (not built).
    Reformed,
}

/// The stages in the order of [`ModalResourcePreflight::stage_allotments`].
pub(crate) const MODAL_RESOURCE_STAGES: [ModalResourceStage; 5] = [
    ModalResourceStage::Preflight,
    ModalResourceStage::Kernel,
    ModalResourceStage::Confirmation,
    ModalResourceStage::ReportedSet,
    ModalResourceStage::Reformed,
];

/// One cost term of P0 (module docs). The discriminant indexes
/// [`ModalResourceBudget::weights`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ModalResourceTerm {
    T1Factor,
    T1Entries,
    T1Coupling,
    T1Integrity,
    T1Confirmation,
    T2Factor,
    T2Entries,
    T2Integrity,
    T3Kernel,
    T4Scaling,
    T4Visits,
    T4Insertions,
    T4Rows,
    T4Limbs,
    T5Census,
    T5Outcome,
    T6Visits,
    T6Limbs,
    T7Elements,
    T7Assembly,
    T7Limbs,
    T8Insertions,
    T8GeneralizedMass,
    T8Rows,
    T8Limbs,
    T9Construction,
    T9Factorization,
    T9Passes,
    T9Neighbours,
    T9Limbs,
    T10Products,
    T10Bilinear,
    T10Limbs,
}

/// The number of P0 terms.
pub(crate) const MODAL_RESOURCE_TERM_COUNT: usize = 33;

/// Every term, in index order.
pub(crate) const MODAL_RESOURCE_TERMS: [ModalResourceTerm; MODAL_RESOURCE_TERM_COUNT] = {
    use ModalResourceTerm as T;
    [
        T::T1Factor,
        T::T1Entries,
        T::T1Coupling,
        T::T1Integrity,
        T::T1Confirmation,
        T::T2Factor,
        T::T2Entries,
        T::T2Integrity,
        T::T3Kernel,
        T::T4Scaling,
        T::T4Visits,
        T::T4Insertions,
        T::T4Rows,
        T::T4Limbs,
        T::T5Census,
        T::T5Outcome,
        T::T6Visits,
        T::T6Limbs,
        T::T7Elements,
        T::T7Assembly,
        T::T7Limbs,
        T::T8Insertions,
        T::T8GeneralizedMass,
        T::T8Rows,
        T::T8Limbs,
        T::T9Construction,
        T::T9Factorization,
        T::T9Passes,
        T::T9Neighbours,
        T::T9Limbs,
        T::T10Products,
        T::T10Bilinear,
        T::T10Limbs,
    ]
};

impl ModalResourceTerm {
    /// The term's name in refusals and records (module docs).
    pub(crate) fn modal_resource_id(self) -> &'static str {
        use ModalResourceTerm as T;
        match self {
            T::T1Factor => "T1.factor",
            T::T1Entries => "T1.entries",
            T::T1Coupling => "T1.coupling",
            T::T1Integrity => "T1.integrity",
            T::T1Confirmation => "T1.confirmation",
            T::T2Factor => "T2.factor",
            T::T2Entries => "T2.entries",
            T::T2Integrity => "T2.integrity",
            T::T3Kernel => "T3.kernel",
            T::T4Scaling => "T4.scaling",
            T::T4Visits => "T4.visits",
            T::T4Insertions => "T4.insertions",
            T::T4Rows => "T4.rows",
            T::T4Limbs => "T4.limbs",
            T::T5Census => "T5.census",
            T::T5Outcome => "T5.outcome",
            T::T6Visits => "T6.visits",
            T::T6Limbs => "T6.limbs",
            T::T7Elements => "T7.elements",
            T::T7Assembly => "T7.assembly",
            T::T7Limbs => "T7.limbs",
            T::T8Insertions => "T8.insertions",
            T::T8GeneralizedMass => "T8.generalized_mass",
            T::T8Rows => "T8.rows",
            T::T8Limbs => "T8.limbs",
            T::T9Construction => "T9.construction",
            T::T9Factorization => "T9.factorization",
            T::T9Passes => "T9.passes",
            T::T9Neighbours => "T9.neighbours",
            T::T9Limbs => "T9.limbs",
            T::T10Products => "T10.products",
            T::T10Bilinear => "T10.bilinear",
            T::T10Limbs => "T10.limbs",
        }
    }

    /// Why the term has no bound at P0 (module docs); `None` for a bounded
    /// term.
    pub(crate) fn modal_resource_unbounded_reason(self) -> Option<&'static str> {
        use ModalResourceTerm as T;
        match self {
            T::T1Integrity | T::T2Integrity => {
                Some("the static dense path's M03 integrity work (preparation, audits, residual records, condition estimate) is not bounded from source in this slice")
            }
            T::T1Confirmation => Some("the confirmation evaluations have no source bound, and their A_conf counter is a held NI seam"),
            T::T7Assembly => Some("the formation maps are std BTreeMaps and the formation difference binary-searches; their step constants are not repository source"),
            T::T9Construction => Some("the Sturm pencil and the RCM order use std sorts, documented only as O(m log m), with no constant from repository source"),
            _ => None,
        }
    }

    /// The stage at which the term's count becomes exact (module docs).
    pub(crate) fn modal_resource_stage(self) -> ModalResourceStage {
        use ModalResourceTerm as T;
        match self {
            T::T1Factor | T::T1Entries | T::T1Coupling | T::T1Integrity | T::T2Factor | T::T2Entries | T::T2Integrity => {
                ModalResourceStage::Preflight
            }
            T::T1Confirmation => ModalResourceStage::Confirmation,
            T::T3Kernel | T::T4Scaling | T::T4Visits | T::T4Insertions | T::T4Rows | T::T4Limbs => ModalResourceStage::Kernel,
            T::T5Census
            | T::T5Outcome
            | T::T6Visits
            | T::T6Limbs
            | T::T7Elements
            | T::T7Assembly
            | T::T7Limbs
            | T::T9Construction
            | T::T9Factorization
            | T::T9Passes
            | T::T9Neighbours
            | T::T9Limbs => ModalResourceStage::ReportedSet,
            T::T8Insertions | T::T8GeneralizedMass | T::T8Rows | T::T8Limbs | T::T10Products | T::T10Bilinear | T::T10Limbs => {
                ModalResourceStage::Reformed
            }
        }
    }
}

/// The level-1 limb span bound L̄₁ (module docs): ⌊(5120 + 132)/32⌋ + 4
/// − ⌊−5682/32⌋ + 1.
pub(crate) const MODAL_RESOURCE_LEVEL1_LIMBS: u128 = 347;
/// L̄, every `ExactSum`'s limb span bound (module docs): the level-2 span,
/// ⌊(B_hi + 132)/32⌋ + 4 − ⌊B_lo/32⌋ + 1 with B_lo = 32·(−178) − 2304 and
/// B_hi = 32·169 + 2048.
pub(crate) const MODAL_RESOURCE_LIMB_BOUND: u128 = 492;

/// The derivation of [`MODAL_RESOURCE_LEVEL1_LIMBS`] and
/// [`MODAL_RESOURCE_LIMB_BOUND`] (module docs), as arithmetic.
pub(crate) fn modal_resource_limb_spans() -> (i64, i64) {
    const BITS: i64 = 32;
    const CARRY: i64 = 132;
    let factor_low = -1074 - 52;
    let factor_high = 1024;
    let span = |low: i64, high: i64| (high + CARRY).div_euclid(BITS) + 4 - low.div_euclid(BITS) + 1;
    let (low1, high1) = (5 * factor_low - 52, 5 * factor_high);
    let level1 = span(low1, high1);
    let (low_limb1, top_limb1) = (low1.div_euclid(BITS), (high1 + CARRY).div_euclid(BITS) + 4);
    let (low2, high2) = (BITS * low_limb1 + 2 * factor_low - 52, BITS * (top_limb1 + 1) + 2 * factor_high);
    (level1, span(low2, high2))
}

/// The resource budget P0 and S1 are judged against (module docs). It has
/// no `Default` and no constructor with values: every field is the
/// caller's, and none is adopted. Only tests build one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ModalResourceBudget {
    /// w_t, integer picoseconds per counted unit, indexed by
    /// [`ModalResourceTerm`]. `None` is an undetermined weight: under the
    /// weight-or-refuse rule a term with a nonzero count and no weight
    /// refuses.
    pub(crate) weights: [Option<u64>; MODAL_RESOURCE_TERM_COUNT],
    /// W_budget, picoseconds: Σ_s A_s must not exceed it.
    pub(crate) work_budget: u128,
    /// A_QL: the most implicit QL rotations (S1-QL, enforced).
    pub(crate) ql_rotation_allotment: u64,
    /// A_band: the most attempted band-recovery pairs (S1-band, enforced).
    pub(crate) band_pair_allotment: u64,
    /// L_A: a recorded allotment for S2b. **Not used by P0**: S2b is not
    /// built, so L is charged at the derived L̄.
    pub(crate) limb_allotment: u64,
    /// R_max: a recorded row allotment for S2. **Not used by P0**: S2 is not
    /// built, so n_r is charged at n̄_f.
    pub(crate) reported_row_allotment: Option<u64>,
}

impl ModalResourceBudget {
    /// The kernel's S1 allotments from this budget: A_QL and A_band.
    pub(crate) fn modal_dense_allotments(&self) -> DenseAllotments {
        DenseAllotments { ql_rotations: Some(self.ql_rotation_allotment), band_pairs: Some(self.band_pair_allotment) }
    }
}

/// The state solve's dimensions of T1 (module docs).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ModalStateDimensions {
    /// S̄, structural solves.
    pub(crate) solves: u128,
    /// d̄, derived-friction candidates per iteration.
    pub(crate) friction: u128,
    /// I_as + 1, the iterations with the recovery trial.
    pub(crate) iterations: u128,
}

/// The quantities the term counts are formed from (module docs). P0 fills
/// them with bounds; a test can fill them with a run's actual values and
/// evaluate the same formulas.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ModalResourceDimensions {
    /// n_g = 6N.
    pub(crate) global: u128,
    /// n_f.
    pub(crate) free: u128,
    /// n_c.
    pub(crate) computed: u128,
    /// n_r.
    pub(crate) reported: u128,
    /// p, attempted band pairs.
    pub(crate) band_pairs: u128,
    /// R, QL rotations.
    pub(crate) ql_rotations: u128,
    /// s, Sturm candidates.
    pub(crate) sturm_candidates: u128,
    /// L, limbs.
    pub(crate) limbs: u128,
    /// D_K = Σ_K d_e².
    pub(crate) stiffness_squares: u128,
    /// D_M = Σ_M d_e².
    pub(crate) mass_squares: u128,
    /// E, re-formation operands.
    pub(crate) element_operands: u128,
    /// μ, exact-product calls per element entry (1, or 36 with a
    /// restraint basis).
    pub(crate) coordinate_multiplier: u128,
    /// T1's dimensions; `None` without nonlinear supports (no state solve).
    pub(crate) state: Option<ModalStateDimensions>,
}

/// μ with a restraint basis (module docs).
const MODAL_RESOURCE_SKEW_PRODUCTS: u128 = 36;

/// A resource overflow: the quantity that overflowed u128.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ModalResourceOverflow(pub(crate) &'static str);

fn modal_resource_mul(a: u128, b: u128, quantity: &'static str) -> Result<u128, ModalResourceOverflow> {
    a.checked_mul(b).ok_or(ModalResourceOverflow(quantity))
}

fn modal_resource_add(a: u128, b: u128, quantity: &'static str) -> Result<u128, ModalResourceOverflow> {
    a.checked_add(b).ok_or(ModalResourceOverflow(quantity))
}

/// The checked product of `factors`.
fn modal_resource_product(factors: &[u128], quantity: &'static str) -> Result<u128, ModalResourceOverflow> {
    factors.iter().try_fold(1u128, |p, &f| modal_resource_mul(p, f, quantity))
}

/// The checked sum of `parts`.
fn modal_resource_total(parts: &[u128], quantity: &'static str) -> Result<u128, ModalResourceOverflow> {
    parts.iter().try_fold(0u128, |s, &p| modal_resource_add(s, p, quantity))
}

/// ⌈log₂ x⌉, 0 for x ≤ 1.
fn modal_resource_ceil_log2(x: u128) -> u128 {
    if x <= 1 {
        0
    } else {
        u128::from(128 - (x - 1).leading_zeros())
    }
}

/// T3's kernel steps at order n with R rotations (module docs): every loop
/// of `solve_dense_pencil_with_allotments` (modal_eigen `dense.rs:358`),
/// with n_m ≤ n and n_0 ≤ n, each part monotone in n.
pub(crate) fn modal_resource_kernel_steps(n: u128, rotations: u128) -> Result<u128, ModalResourceOverflow> {
    if n == 0 {
        return Ok(0);
    }
    let q = "T3.kernel";
    let p = |f: &[u128]| modal_resource_product(f, q);
    let n2 = p(&[n, n])?;
    let tri = p(&[n, n - 1])? / 2; // n(n − 1)/2
    let pairs = p(&[n, n + 1])? / 2; // n(n + 1)/2
    let cube6 = p(&[n - 1, n, n + 1])? / 6; // Σ_i Σ_{j≤i} j
    let squares = p(&[n - 1, n, 2 * n - 1])? / 6; // Σ_{t<n} t²
    let search = modal_resource_ceil_log2(n + 1) + 1; // a binary search's comparisons
    modal_resource_total(
        &[
            // check_dense on K and M (admission.rs:84).
            p(&[2, pairs])?,
            // structurally_massless_dofs, in the kernel and the census.
            p(&[2, n2])?,
            // the massive filters (binary searches) and the order.
            p(&[2, n, search])?,
            n,
            // cholesky_ordered: inner p-loop, entry bodies, allocation, finite check.
            cube6,
            pairs,
            p(&[2, n2])?,
            // reduce_mass: fill, two forward-solve passes over n rows, transpose, symmetrize, finite check.
            n2,
            p(&[2, n, tri + n])?,
            p(&[2, tri])?,
            n2,
            // householder_tridiagonalize: Σ_{i<n} (i² + 7i + 1) reduction plus
            // Σ_{t<n} (2t² + 3t + 1) accumulation, the two n-loops; transpose; d, e.
            p(&[3, squares])?,
            p(&[10, tri])?,
            p(&[2, n])?,
            p(&[2, n])?,
            tri,
            p(&[2, n])?,
            // implicit_ql: the e shift, the block scans, the per-l bodies, and
            // per iteration (≤ R) its body and shift loop, per rotation its
            // scalar body and row update.
            n,
            pairs,
            n,
            p(&[2, rotations, n + 1])?,
            // sort_ascending: comparisons, bodies, row swaps.
            tri,
            n,
            n2,
            // the resolution floor: roots, ≤ 11 Hager solves of n(n − 1) + 10n
            // steps (norm_estimate.rs), the mass norm.
            n,
            p(&[11, p(&[n, n - 1])? + 10 * n])?,
            n2,
            // reordering: counts, reverse, row swaps, truncation, row moves.
            p(&[3, n2])?,
            p(&[4, n])?,
            8,
            // back-transform of every row and the finite check.
            p(&[n, tri + 3 * n])?,
            n2,
            // classification, screen bodies, retained rows, the descending sort.
            p(&[2, n])?,
            n2,
            tri,
            n2,
            // established_finite_mode_count: roots, Cholesky, entry bodies, allocation.
            n,
            cube6,
            pairs,
            n2,
            // outputs and their validation, and counts.
            p(&[7, n])?,
            n2,
        ],
        q,
    )
}

/// The count c_t of every term (module docs) from `dims`; `None` for a term
/// with no bound.
pub(crate) fn modal_resource_term_counts(
    dims: &ModalResourceDimensions,
) -> Result<[Option<u128>; MODAL_RESOURCE_TERM_COUNT], ModalResourceOverflow> {
    use ModalResourceTerm as T;
    let p = modal_resource_product;
    let t = modal_resource_total;
    let n = dims.free;
    let n2 = p(&[n, n], "n_f²")?;
    let positions = p(&[n, n + 1], "n_f(n_f + 1)")? / 2;
    let screen = p(&[n.saturating_sub(1), n, n + 1], "T2.factor")? / 6;
    let entries = p(&[3, n2], "T2.entries")?;
    let (l, mu) = (dims.limbs, dims.coordinate_multiplier);
    let part = t(&[p(&[5, l], "5L")?, 16], "limbs per part")?;
    let read = t(&[p(&[12, l], "12L")?, 32], "limbs per read")?;
    let q8 = p(&[8, mu], "Q")?;
    let (dk, dm) = (dims.stiffness_squares, dims.mass_squares);
    let pair_entries = t(&[dk, p(&[2, dm], "2·D_M")?], "D_K + 2·D_M")?;
    let element_entries = t(&[dk, dm], "D_K + D_M")?;
    let (c, r, s, b) = (dims.computed, dims.reported, dims.sturm_candidates, dims.band_pairs);
    let forms = p(&[r, r + 1], "n_r(n_r + 1)")? / 2;
    let mut counts = [Some(0u128); MODAL_RESOURCE_TERM_COUNT];
    let mut set = |term: T, value: u128| counts[term as usize] = Some(value);

    if let Some(state) = dims.state {
        set(T::T1Factor, p(&[state.solves, screen], "T1.factor")?);
        set(T::T1Entries, p(&[state.solves, entries], "T1.entries")?);
        let d = state.friction;
        let coupling = t(&[p(&[2, d, d, d], "2d³")?.div_ceil(3), p(&[2, d, d], "2d²")?, p(&[d, dims.global], "d·n_g")?], "T1.coupling")?;
        set(T::T1Coupling, p(&[state.iterations, coupling], "T1.coupling")?);
    }
    set(T::T2Factor, screen);
    set(T::T2Entries, entries);
    set(T::T3Kernel, modal_resource_kernel_steps(n, dims.ql_rotations)?);
    set(T::T4Scaling, if b > 0 { t(&[p(&[2, n2], "T4.scaling")?, n], "T4.scaling")? } else { 0 });
    set(T::T4Visits, p(&[4, b, n2], "T4.visits")?);
    set(T::T4Insertions, p(&[4, b, n2], "T4.insertions")?);
    set(T::T4Rows, p(&[b, n + 2], "T4.rows")?);
    let band_limbs = t(&[p(&[16, n2, part], "T4.limbs")?, p(&[n + 2, read], "T4.limbs")?], "T4.limbs")?;
    set(T::T4Limbs, p(&[b, band_limbs], "T4.limbs")?);
    set(T::T5Census, p(&[16, c], "T5.census")?);
    set(T::T5Outcome, t(&[p(&[c, b], "n_c·p")?, p(&[2, c, c], "2n_c²")?, c], "T5.outcome")?);
    let mode_calls = t(&[n2, p(&[3, n], "3n")?], "n² + 3n")?;
    set(T::T6Visits, p(&[r, mode_calls], "T6.visits")?);
    let mode_limbs = t(&[p(&[4, mode_calls, part], "T6.limbs")?, p(&[4, read], "T6.limbs")?], "T6.limbs")?;
    set(T::T6Limbs, p(&[r, mode_limbs], "T6.limbs")?);
    set(T::T7Elements, dims.element_operands);
    let formed = p(&[n, n + 1], "n(n + 1)")?;
    let reformed_positions = p(&[mu, element_entries], "μ(D_K + D_M)")?;
    let assembly_parts = t(
        &[p(&[q8, element_entries], "Q(D_K + D_M)")?, formed, p(&[l, reformed_positions], "L·μ(D_K + D_M)")?],
        "T7.limbs",
    )?;
    let assembly_reads = t(&[p(&[2, reformed_positions], "2μ(D_K + D_M)")?, formed], "T7.limbs")?;
    set(T::T7Limbs, t(&[p(&[part, assembly_parts], "T7.limbs")?, p(&[read, assembly_reads], "T7.limbs")?], "T7.limbs")?);
    set(T::T8Insertions, p(&[c, mu, pair_entries], "T8.insertions")?);
    set(T::T8GeneralizedMass, p(&[c, n, l], "T8.generalized_mass")?);
    let pair_rows = t(&[p(&[2, n], "2n")?, 1], "2n + 1")?;
    set(T::T8Rows, p(&[c, pair_rows], "T8.rows")?);
    let pair_parts = t(&[p(&[q8, pair_entries], "Q(D_K + 2D_M)")?, p(&[4, n, l], "4nL")?], "T8.limbs")?;
    let pair_limbs = t(&[p(&[part, pair_parts], "T8.limbs")?, p(&[read, pair_rows], "T8.limbs")?], "T8.limbs")?;
    set(T::T8Limbs, p(&[c, pair_limbs], "T8.limbs")?);
    set(T::T9Factorization, p(&[s, t(&[p(&[n, n.saturating_sub(1), n.saturating_sub(2)], "T9.factorization")? / 6, n2], "T9.factorization")?], "T9.factorization")?);
    set(T::T9Passes, p(&[s, t(&[p(&[29, positions], "29P")?, p(&[98, n], "98n")?, 70], "T9.passes")?], "T9.passes")?);
    set(T::T9Neighbours, p(&[2, t(&[positions, p(&[3, n], "3n")?], "T9.neighbours")?], "T9.neighbours")?);
    set(T::T9Limbs, p(&[2, t(&[p(&[part, 8, positions], "T9.limbs")?, p(&[2, read], "T9.limbs")?], "T9.limbs")?], "T9.limbs")?);
    set(T::T10Products, p(&[r, mu, dm], "T10.products")?);
    set(T::T10Bilinear, p(&[forms, n, l], "T10.bilinear")?);
    let shape_limbs = t(&[p(&[part, q8, dm], "T10.limbs")?, p(&[read, n], "T10.limbs")?], "T10.limbs")?;
    let form_limbs = t(&[p(&[part, 4, n, l], "T10.limbs")?, read], "T10.limbs")?;
    set(T::T10Limbs, t(&[p(&[r, shape_limbs], "T10.limbs")?, p(&[forms, form_limbs], "T10.limbs")?], "T10.limbs")?);

    for term in MODAL_RESOURCE_TERMS {
        let occurs = match term {
            T::T1Integrity | T::T1Confirmation => dims.state.is_some(),
            _ => true,
        };
        if occurs && term.modal_resource_unbounded_reason().is_some() {
            counts[term as usize] = None;
        }
    }
    Ok(counts)
}

/// The bounds P0 uses (module docs), from the prepared request, the
/// parameters and the budget's enforced allotments only (A_QL, A_band).
pub(crate) fn modal_resource_bound_dimensions(
    prepared: &Prepared,
    params: &ModalIntegrityParameters,
    budget: &ModalResourceBudget,
) -> Result<ModalResourceDimensions, ModalResourceOverflow> {
    let built = &prepared.built;
    let wide = |value: usize| value as u128;
    let nodes = wide(built.nodes.len());
    let global = modal_resource_mul(nodes, wide(DOF_PER_NODE), "n_g")?;
    let free = if built.skew.is_some() {
        global
    } else {
        let restrained: BTreeSet<usize> = prepared.boundary.restrained_dofs.iter().copied().filter(|&dof| wide(dof) < global).collect();
        global - wide(restrained.len())
    };
    let pipes = wide(built.pipes.len());
    let arcs = wide(built.curved_bend_elements.len());
    let connectors = wide(built.connectors.len());
    let tie_rods = built.connectors.iter().try_fold(0u128, |sum, c| modal_resource_add(sum, wide(c.tie_rods().len()), "tie_rods"))?;
    let springs = wide(prepared.boundary.springs.len());
    let skewed = wide(built.skew.as_ref().map_or(0, |basis| basis.springs().len()));
    let segments = wide(prepared.model.pipe_segments.len());
    let twelve = modal_resource_total(&[pipes, arcs, connectors, tie_rods], "elements")?;
    let stiffness_squares = modal_resource_total(&[modal_resource_mul(144, twelve, "D_K")?, springs, modal_resource_mul(9, skewed, "D_K")?], "D_K")?;
    let mass_squares = modal_resource_total(
        &[modal_resource_mul(144, pipes, "D_M")?, modal_resource_mul(72, arcs, "D_M")?, modal_resource_mul(36, segments, "D_M")?],
        "D_M",
    )?;
    let element_operands = modal_resource_total(&[twelve, segments, springs, skewed], "E")?;
    let state = if built.nonlinear_supports.is_empty() {
        None
    } else {
        let friction = wide(built.nonlinear_supports.iter().filter(|s| matches!(s.behavior, NonlinearSupportBehavior::Friction)).count());
        let iterations = modal_resource_add(wide(crate::DEC_046_PRODUCT_PREVIEW_ACTIVE_SET_MAX_ITERATIONS), 1, "I_as + 1")?;
        let per = modal_resource_add(friction, 2, "2 + d")?;
        let solves = modal_resource_add(modal_resource_mul(iterations, per, "S_T1")?, 1, "S_T1")?;
        Some(ModalStateDimensions { solves, friction, iterations })
    };
    Ok(ModalResourceDimensions {
        global,
        free,
        computed: free,
        reported: free,
        band_pairs: free.min(u128::from(budget.band_pair_allotment)),
        ql_rotations: u128::from(budget.ql_rotation_allotment),
        sturm_candidates: wide(params.sturm_max_candidates),
        limbs: MODAL_RESOURCE_LIMB_BOUND,
        stiffness_squares,
        mass_squares,
        element_operands,
        coordinate_multiplier: if built.skew.is_some() { MODAL_RESOURCE_SKEW_PRODUCTS } else { 1 },
        state,
    })
}

/// What P0 admitted: the bounds, the counts, each term's charge, the stage
/// allotments A_s and W.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ModalResourcePreflight {
    pub(crate) dimensions: ModalResourceDimensions,
    pub(crate) counts: [u128; MODAL_RESOURCE_TERM_COUNT],
    /// w_t·c̄_t, ps (0 for a zero count with no weight).
    pub(crate) charges: [u128; MODAL_RESOURCE_TERM_COUNT],
    /// A_s, ps, in [`MODAL_RESOURCE_STAGES`] order.
    pub(crate) stage_allotments: [u128; 5],
    /// W = Σ_s A_s, ps.
    pub(crate) total: u128,
}

fn modal_resource_overflow_refusal(overflow: ModalResourceOverflow) -> Vec<Diagnostic> {
    vec![diag(
        "diagnostic:modal:resource:overflow",
        MODAL_RESOURCE_ARITHMETIC_OVERFLOW,
        "blocking",
        format!(
            "the modal resource preflight's checked arithmetic overflowed at `{}`; no count, charge or sum is wrapped or saturated, so the request is refused before any state solve or n² allocation",
            overflow.0
        ),
        vec!["model".into(), overflow.0.to_string()],
    )]
}

fn modal_preflight_refusal(message: String, terms: Vec<String>) -> Vec<Diagnostic> {
    vec![diag("diagnostic:modal:resource:preflight", MODAL_RESOURCE_PREFLIGHT_EXCEEDED, "blocking", message, terms)]
}

/// P0 (module docs): the bounds, the counts and the budget check, or the
/// refusal by name.
pub(crate) fn modal_resource_preflight(
    prepared: &Prepared,
    params: &ModalIntegrityParameters,
    budget: &ModalResourceBudget,
) -> Result<ModalResourcePreflight, Vec<Diagnostic>> {
    let dimensions = modal_resource_bound_dimensions(prepared, params, budget).map_err(modal_resource_overflow_refusal)?;
    let bounded = modal_resource_term_counts(&dimensions).map_err(modal_resource_overflow_refusal)?;
    modal_resource_judge(dimensions, &bounded, budget)
}

/// The budget check on P0's counts (module docs): weight-or-refuse, the
/// checked charges and stage allotments, and Σ_s A_s ≤ W_budget.
pub(crate) fn modal_resource_judge(
    dimensions: ModalResourceDimensions,
    bounded: &[Option<u128>; MODAL_RESOURCE_TERM_COUNT],
    budget: &ModalResourceBudget,
) -> Result<ModalResourcePreflight, Vec<Diagnostic>> {
    // A term that arithmetic cannot admit: no bound, or a nonzero count with
    // an undetermined weight. The list is bounded by the fixed term count.
    let mut inadmissible = Vec::new();
    for term in MODAL_RESOURCE_TERMS {
        let id = term.modal_resource_id();
        match (bounded[term as usize], budget.weights[term as usize]) {
            (None, _) => inadmissible.push((id, format!("unbounded: {}", term.modal_resource_unbounded_reason().unwrap_or("no bound at P0")))),
            (Some(count), None) if count > 0 => inadmissible.push((id, format!("count {count}, weight undetermined"))),
            _ => {}
        }
    }
    if !inadmissible.is_empty() {
        let terms = inadmissible.iter().map(|(id, _)| id.to_string()).collect();
        let stated: Vec<String> = inadmissible.iter().map(|(id, why)| format!("{id} ({why})")).collect();
        return Err(modal_preflight_refusal(
            format!(
                "the modal resource preflight cannot admit {} by arithmetic: {}. A term whose weight is undetermined, or whose count has no bound, refuses rather than being dropped (weight-or-refuse), before any state solve or n² allocation",
                if inadmissible.len() == 1 { "a term" } else { "these terms" },
                stated.join("; ")
            ),
            terms,
        ));
    }
    let mut counts = [0u128; MODAL_RESOURCE_TERM_COUNT];
    let mut charges = [0u128; MODAL_RESOURCE_TERM_COUNT];
    let mut stage_allotments = [0u128; 5];
    for term in MODAL_RESOURCE_TERMS {
        let index = term as usize;
        let count = bounded[index].unwrap_or_default();
        let charge = modal_resource_mul(u128::from(budget.weights[index].unwrap_or_default()), count, term.modal_resource_id())
            .map_err(modal_resource_overflow_refusal)?;
        let stage = MODAL_RESOURCE_STAGES.iter().position(|&s| s == term.modal_resource_stage()).unwrap_or_default();
        stage_allotments[stage] = modal_resource_add(stage_allotments[stage], charge, "A_s").map_err(modal_resource_overflow_refusal)?;
        counts[index] = count;
        charges[index] = charge;
    }
    let total = modal_resource_total(&stage_allotments, "W").map_err(modal_resource_overflow_refusal)?;
    if total > budget.work_budget {
        // The dominant term: the largest charge, the first on a tie.
        let dominant = MODAL_RESOURCE_TERMS.iter().copied().fold(MODAL_RESOURCE_TERMS[0], |best, t| if charges[t as usize] > charges[best as usize] { t } else { best });
        let id = dominant.modal_resource_id();
        return Err(modal_preflight_refusal(
            format!(
                "the modal resource preflight's weighted work W = {total} ps exceeds the budget W_budget = {} ps; the dominant term is {id}, count {} at {} ps per unit = {} ps. Refused before any state solve or n² allocation; W is a weighted-work budget, not a runtime",
                budget.work_budget,
                counts[dominant as usize],
                budget.weights[dominant as usize].unwrap_or_default(),
                charges[dominant as usize]
            ),
            vec![id.to_string()],
        ));
    }
    Ok(ModalResourcePreflight { dimensions, counts, charges, stage_allotments, total })
}
