//! The modal resource guard's combined preflight P0 (DEL-04-09 Design
//! §5.3.8, "Where each check sits"), and the budget form it is judged
//! against. **No weight, allotment or ceiling is adopted here** (Agent 0,
//! 2026-11-04): [`ModalResourceBudget`] has no `Default` and no constructor
//! with values; only tests build one, and it is labelled test-local.
//!
//! **What P0 is.** In `run_modal_dense`, after `prepare_modal` and before
//! the state case's solve and any n²-sized allocation, P0 bounds the count
//! c̄_t of every cost term T1 to T10 of §5.3.8's table from request, model
//! and parameter inputs only. Each unknown enters at a bound derived below
//! from source, or at an allotment the budget states and a later stage
//! enforces. Then, in checked u128 arithmetic:
//! - W = Σ_t w_t·c̄_t, with w_t integer picoseconds per counted unit;
//! - the stage allotments A_s = Σ_{t∈s} w_t·c̄_t, for the stage s at which
//!   t's count becomes exact (S1 in the kernel; S2, S2b and the
//!   confirmation counter are not built, so their allotments are recorded
//!   and enforced by nothing yet);
//! - Σ_s A_s ≤ W_budget, inclusive.
//!
//! W is a calibrated weighted-work budget in nominal time units, not a
//! runtime guarantee.
//!
//! **Refusals**, each blocking and in the approved MODAL namespace:
//! - `MODAL_RESOURCE_PREFLIGHT_EXCEEDED`: W exceeds W_budget (the dominant
//!   term, the largest w_t·c̄_t, is named); or a term is inadmissible by
//!   arithmetic, under the approved weight-or-refuse rule: its count is
//!   nonzero and its weight is undetermined (`None`), or its count has no
//!   bound at P0 at all;
//! - `MODAL_RESOURCE_ARITHMETIC_OVERFLOW`: a count, a product or a sum
//!   overflows u128. Nothing wraps or saturates.
//!
//! **The count domain.** Every count below is an upper bound over the
//! inputs P0 sees, never a fit to a population; a population run can
//! support a bound (`tests_resource`) but does not establish it. The
//! symbols are §5.3.8's:
//! - N, the model's nodes, and n_g = 6N its global DOFs.
//! - n̄_f, the pencil order. `reduce_assembled_sparse_system`
//!   (frame_kernel `structural/sparse.rs:851`) takes the free DOFs as
//!   exactly the complement, in 0..n_g, of the restrained set it is given,
//!   and refuses an out-of-range or repeated DOF. That set is the prepared
//!   linear one, or with nonlinear supports a superset of it
//!   (`state::linearize_modal_supports` only adds rigid states). So
//!   n_f ≤ n_g − |distinct linear restrained DOFs in 0..n_g|. With a
//!   restraint basis the partition is on coordinates, so n̄_f = n_g, the
//!   coordinate count.
//! - n̄_c = n̄_f: the computed pairs are at most the condensed order n_m ≤ n_f.
//! - n̄_r = min(n̄_f, ⌊R_max/6N⌋) when the budget states R_max (each reported
//!   mode publishes 6N shape rows), and n̄_f otherwise. n_r ≤ n_c ≤ n_f
//!   by source; **R_max is an allotment that S2 enforces, and S2 is not
//!   built**, so a budget whose R_max binds is not enforced yet. It is never
//!   assumed that n_r ≤ `mode_count`: cluster completion can exceed it.
//! - p̄ = min(n̄_f, A_band): the attempted band pairs are among the condensed
//!   μ (p ≤ n_m ≤ n_f), and S1-band refuses more than A_band before any
//!   band work.
//! - R̄ = A_QL: S1-QL refuses before a sweep that would pass it.
//! - s = s_max (`sturm_max_candidates`): `sturm::run_candidates` takes at
//!   most `max_candidates` shifts in either placement, and one placement
//!   runs per request.
//! - L = L_A, the budget's limb allotment. No data-independent bound on an
//!   `ExactSum`'s limb span is established (§5.3.8), so L is held by an
//!   allotment that S2b enforces; S2b is not built.
//! - The re-formation's element sizes, from `reform_element_stiffness` and
//!   `reform_element_mass` (modal_eigen `reformation/mod.rs:263`, `:357`):
//!   straight, arc, connector and tie-rod stiffness 12 × 12; axis spring
//!   1 × 1; rank-one (skewed) spring 3 × 3; straight mass 12 × 12; an arc's
//!   mass two 6 × 6 lumps; a component body 6 × 6. The operands
//!   (`integrity::modal_reformation_operands`) are at most: every built
//!   pipe as a straight (bend pipes and replaced spans are skipped there,
//!   so this over-counts), every realized arc, one body per mass-bearing
//!   pipe segment (`mass::assemble_mass` keys bodies by pipe), every
//!   connector and its tie rods, every linear spring and every skewed
//!   spring. Hence, with d_e the element's DOF count (an upper bound on its
//!   free DOF count):
//!   - D_K = Σ_K d_e² ≤ 144·(pipes + arcs + connectors + tie rods)
//!     + springs + 9·skewed springs;
//!   - D_M = Σ_M d_e² ≤ 144·pipes + 72·arcs + 36·pipe segments;
//!   - E ≤ pipes + arcs + pipe segments + connectors + tie rods + springs
//!     + skewed springs.
//! - μ, the coordinate multiplier: 1 without a restraint basis. With one,
//!   `ReformedPencil::product_into` enters each element entry (a, b) of a
//!   mixed element as up to 3 × 3 coordinate pairs, two dd words each, and
//!   with a factor two exact products each: at most 36 exact products per
//!   entry, so μ = 36.
//!
//! **The terms** (counted unit; the weight is per that unit; stage):
//! - T1, only with nonlinear supports (the state case's same-run solve,
//!   `state::solve_modal_state_case`):
//!   - S̄ = (I_as + 1)·(2 + d̄) + 1 structural solves. I_as is the product's
//!     active-set iteration cap (`DEC_046_PRODUCT_PREVIEW_ACTIVE_SET_MAX_
//!     ITERATIONS`, lib.rs:197). Per iteration, dense mode solves once
//!     without derived friction and 2 + d with it (one base, d unit, one
//!     final; nonlinear_integration `lib.rs:1639`–`:1795`), and the
//!     single all-active recovery trial of iteration 1 (`:833`) adds at
//!     most one more such step; the final +1 is the confirmation's gate
//!     factorization. d̄ is the friction supports (the sliding candidates
//!     are friction supports only, `:2131`).
//!   - `T1.solve_factor` = S̄·⌈n̄_f³/3⌉ and `T1.solve_passes` = S̄·n_g²: each
//!     solve priced as T2 on its free set (P0).
//!   - `T1.coupling` = (I_as + 1)·(⌈2d̄³/3⌉ + 2d̄² + d̄·n_g): the d × d
//!     elimination `solve_dense`, the influence and coupling matrices and
//!     the unit-force vectors, per iteration with friction (P0).
//!   - `T1.confirmation`: the static confirmation's formation evaluations
//!     E_conf. **Not bounded from source**; §5.3.8 holds it by the
//!     allotment A_conf and a counter in NI `confirmation.rs`, a held seam.
//!     Until that counter exists its count has no bound, so P0 refuses
//!     every request that runs a state solve.
//! - T2, the static screen of the modal pencil: `T2.factor` = ⌈n̄_f³/3⌉
//!   dense Cholesky flops, `T2.passes` = n_g² entry passes (P0).
//! - T3, the binary64 kernel: `T3.kernel` = ⌈19·n̄_f³/3⌉ + 6·n̄_f·R̄ flops,
//!   w_b64's unit (S1).
//! - T4, band recovery: `T4.scaling` = 2·n̄_f² `formed_scaling` entries when
//!   p̄ > 0; `T4.visits` = 4·p̄·n̄_f² (≤ 2n² in `recover_band_mode` and
//!   ≤ 2n² in `formed_pair_errors` per pair); `T4.insertions` =
//!   2·p̄·(nnz(K_ff) + nnz(M_ff)) at nnz ≤ n̄_f² each (S1). The element bound
//!   D is not used for nnz: the formed pattern is not proved to lie in the
//!   re-formed one (the formation difference keeps an off-pattern map).
//! - `T5.census`: n̄_c computed pairs (census, clusters, reported set; S2).
//! - `T6.visits`: n̄_r·(n̄_f² + 3·n̄_f): `mass_bilinear` on dense M_ff plus
//!   three exact dots per mode (S2).
//! - T7, re-formation and formation difference: `T7.elements` = E operands
//!   (w_reform's unit); `T7.limbs` = μ·(D_K + D_M)·L, one `terms()` read of
//!   ≤ L limbs per re-formed position (S2).
//! - T8, residuals of every computed pair: `T8.insertions` =
//!   n̄_c·μ·(D_K + 2·D_M) (K̂x, λM̂x and the exact M̂x; for straight elements
//!   D_K = D_M and this is §5.3.8's 3·n_c·Σ_e d_e²); `T8.generalized_mass` =
//!   n̄_c·n̄_f·L; `T8.reads` = n̄_c·n̄_f·L (`to_wide` of ≤ L limbs per row);
//!   `T8.limbs` = n̄_c·L·(μ·(D_K + 2·D_M) + 3·n̄_f) limb moves (S2b).
//! - T9, Sturm evidence at the dense profile: with P̄ = P̄_u = n̄_f(n̄_f + 1)/2
//!   (lower-triangle positions) and row widths w_i ≤ i:
//!   `T9.construction` = P̄_u·⌈log₂ P̄_u⌉ + n̄_f + P̄_u; `T9.factorization` =
//!   s·(⌈n̄_f(n̄_f + 1)(2n̄_f + 1)/12⌉ + n̄_f(n̄_f + 1)/2) multiply-adds
//!   (½Σw_i² + Σw_i); `T9.passes` = s·(P̄_u + 24·P̄ + n̄_f) (formed list,
//!   profile, observations, ≤ 11 inverse-norm solves of 2P each, vectors);
//!   `T9.neighbours` = 2·P̄_u (S2).
//! - T10, orthogonality of the published set: `T10.products` = n̄_r·μ·D_M;
//!   `T10.bilinear` = n̄_r(n̄_r + 1)/2·n̄_f·L (S2b).
//!
//! **Not in P0 here:** T11, the projection. It runs in the document path
//! after `run_modal_dense`, and its count needs B_doc, which stays
//! proposed; it is an open item. **Memory** is an open item too: the
//! option-(c) ledger's rows still carry unpriced O(·) and approximate
//! constants and the K-D5 buffers, so no memory check is made here.
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
    T1SolveFactor,
    T1SolvePasses,
    T1Coupling,
    T1Confirmation,
    T2Factor,
    T2Passes,
    T3Kernel,
    T4Scaling,
    T4Visits,
    T4Insertions,
    T5Census,
    T6Visits,
    T7Elements,
    T7Limbs,
    T8Insertions,
    T8GeneralizedMass,
    T8Reads,
    T8Limbs,
    T9Construction,
    T9Factorization,
    T9Passes,
    T9Neighbours,
    T10Products,
    T10Bilinear,
}

/// The number of P0 terms.
pub(crate) const MODAL_RESOURCE_TERM_COUNT: usize = 24;

/// Every term, in index order.
pub(crate) const MODAL_RESOURCE_TERMS: [ModalResourceTerm; MODAL_RESOURCE_TERM_COUNT] = [
    ModalResourceTerm::T1SolveFactor,
    ModalResourceTerm::T1SolvePasses,
    ModalResourceTerm::T1Coupling,
    ModalResourceTerm::T1Confirmation,
    ModalResourceTerm::T2Factor,
    ModalResourceTerm::T2Passes,
    ModalResourceTerm::T3Kernel,
    ModalResourceTerm::T4Scaling,
    ModalResourceTerm::T4Visits,
    ModalResourceTerm::T4Insertions,
    ModalResourceTerm::T5Census,
    ModalResourceTerm::T6Visits,
    ModalResourceTerm::T7Elements,
    ModalResourceTerm::T7Limbs,
    ModalResourceTerm::T8Insertions,
    ModalResourceTerm::T8GeneralizedMass,
    ModalResourceTerm::T8Reads,
    ModalResourceTerm::T8Limbs,
    ModalResourceTerm::T9Construction,
    ModalResourceTerm::T9Factorization,
    ModalResourceTerm::T9Passes,
    ModalResourceTerm::T9Neighbours,
    ModalResourceTerm::T10Products,
    ModalResourceTerm::T10Bilinear,
];

impl ModalResourceTerm {
    /// The term's name in refusals and records (module docs).
    pub(crate) fn modal_resource_id(self) -> &'static str {
        match self {
            Self::T1SolveFactor => "T1.solve_factor",
            Self::T1SolvePasses => "T1.solve_passes",
            Self::T1Coupling => "T1.coupling",
            Self::T1Confirmation => "T1.confirmation",
            Self::T2Factor => "T2.factor",
            Self::T2Passes => "T2.passes",
            Self::T3Kernel => "T3.kernel",
            Self::T4Scaling => "T4.scaling",
            Self::T4Visits => "T4.visits",
            Self::T4Insertions => "T4.insertions",
            Self::T5Census => "T5.census",
            Self::T6Visits => "T6.visits",
            Self::T7Elements => "T7.elements",
            Self::T7Limbs => "T7.limbs",
            Self::T8Insertions => "T8.insertions",
            Self::T8GeneralizedMass => "T8.generalized_mass",
            Self::T8Reads => "T8.reads",
            Self::T8Limbs => "T8.limbs",
            Self::T9Construction => "T9.construction",
            Self::T9Factorization => "T9.factorization",
            Self::T9Passes => "T9.passes",
            Self::T9Neighbours => "T9.neighbours",
            Self::T10Products => "T10.products",
            Self::T10Bilinear => "T10.bilinear",
        }
    }

    /// The stage at which the term's count becomes exact (module docs).
    pub(crate) fn modal_resource_stage(self) -> ModalResourceStage {
        match self {
            Self::T1SolveFactor | Self::T1SolvePasses | Self::T1Coupling | Self::T2Factor | Self::T2Passes => ModalResourceStage::Preflight,
            Self::T1Confirmation => ModalResourceStage::Confirmation,
            Self::T3Kernel | Self::T4Scaling | Self::T4Visits | Self::T4Insertions => ModalResourceStage::Kernel,
            Self::T5Census
            | Self::T6Visits
            | Self::T7Elements
            | Self::T7Limbs
            | Self::T9Construction
            | Self::T9Factorization
            | Self::T9Passes
            | Self::T9Neighbours => ModalResourceStage::ReportedSet,
            Self::T8Insertions | Self::T8GeneralizedMass | Self::T8Reads | Self::T8Limbs | Self::T10Products | Self::T10Bilinear => {
                ModalResourceStage::Reformed
            }
        }
    }
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
    /// A_QL: the most implicit QL rotations (S1-QL).
    pub(crate) ql_rotation_allotment: u64,
    /// A_band: the most attempted band-recovery pairs (S1-band).
    pub(crate) band_pair_allotment: u64,
    /// L_A: the limb allotment every L-dependent term is charged at (S2b
    /// enforces it; not built).
    pub(crate) limb_allotment: u64,
    /// R_max: the projection's row allotment, bounding n_r by ⌊R_max/6N⌋
    /// (S2 enforces it; not built). `None`: n_r at its source bound n_f.
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
    /// μ, the exact products per element entry (1, or 36 with a restraint
    /// basis).
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

/// ⌈a/b⌉ for b > 0.
fn modal_resource_ceil_div(a: u128, b: u128) -> u128 {
    a.div_ceil(b)
}

/// ⌈log₂ x⌉, 0 for x ≤ 1.
fn modal_resource_ceil_log2(x: u128) -> u128 {
    if x <= 1 {
        0
    } else {
        u128::from(128 - (x - 1).leading_zeros())
    }
}

/// n³ and n², checked.
fn modal_resource_cube(n: u128, quantity: &'static str) -> Result<u128, ModalResourceOverflow> {
    modal_resource_mul(modal_resource_mul(n, n, quantity)?, n, quantity)
}

/// The count c_t of every term (module docs) from `dims`; `None` for a term
/// with no bound (T1's confirmation evaluations, whenever a state solve
/// runs).
pub(crate) fn modal_resource_term_counts(
    dims: &ModalResourceDimensions,
) -> Result<[Option<u128>; MODAL_RESOURCE_TERM_COUNT], ModalResourceOverflow> {
    let mul = modal_resource_mul;
    let add = modal_resource_add;
    let n = dims.free;
    let n2 = mul(n, n, "n_f²")?;
    let n3 = modal_resource_cube(n, "n_f³")?;
    let g2 = mul(dims.global, dims.global, "n_g²")?;
    let screen = modal_resource_ceil_div(n3, 3);
    let mu = dims.coordinate_multiplier;
    let (dk, dm) = (dims.stiffness_squares, dims.mass_squares);
    // K̂x, λM̂x and the exact M̂x, per pair.
    let pair_products = mul(mu, add(dk, mul(2, dm, "D_M")?, "D_K + 2·D_M")?, "μ·(D_K + 2·D_M)")?;
    // Lower-triangle positions of the dense profile.
    let positions = n.checked_add(1).ok_or(ModalResourceOverflow("n_f + 1"))?;
    let positions = mul(n, positions, "n_f(n_f + 1)")? / 2;
    let mut counts = [Some(0u128); MODAL_RESOURCE_TERM_COUNT];
    let mut set = |term: ModalResourceTerm, value: Option<u128>| counts[term as usize] = value;
    use ModalResourceTerm as T;

    if let Some(state) = dims.state {
        set(T::T1SolveFactor, Some(mul(state.solves, screen, "T1.solve_factor")?));
        set(T::T1SolvePasses, Some(mul(state.solves, g2, "T1.solve_passes")?));
        let d = state.friction;
        let coupling = add(
            add(modal_resource_ceil_div(mul(2, modal_resource_cube(d, "d³")?, "2d³")?, 3), mul(2, mul(d, d, "d²")?, "2d²")?, "T1.coupling")?,
            mul(d, dims.global, "d·n_g")?,
            "T1.coupling",
        )?;
        set(T::T1Coupling, Some(mul(state.iterations, coupling, "T1.coupling")?));
        set(T::T1Confirmation, None);
    }
    set(T::T2Factor, Some(screen));
    set(T::T2Passes, Some(g2));
    set(
        T::T3Kernel,
        Some(add(modal_resource_ceil_div(mul(19, n3, "19·n_f³")?, 3), mul(mul(6, n, "6·n_f")?, dims.ql_rotations, "6·n_f·R")?, "T3.kernel")?),
    );
    let p = dims.band_pairs;
    set(T::T4Scaling, Some(if p > 0 { mul(2, n2, "T4.scaling")? } else { 0 }));
    set(T::T4Visits, Some(mul(mul(4, p, "4p")?, n2, "T4.visits")?));
    set(T::T4Insertions, Some(mul(mul(2, p, "2p")?, mul(2, n2, "2·n_f²")?, "T4.insertions")?));
    set(T::T5Census, Some(dims.computed));
    set(T::T6Visits, Some(mul(dims.reported, add(n2, mul(3, n, "3·n_f")?, "n_f² + 3·n_f")?, "T6.visits")?));
    set(T::T7Elements, Some(dims.element_operands));
    set(T::T7Limbs, Some(mul(mul(mu, add(dk, dm, "D_K + D_M")?, "μ·(D_K + D_M)")?, dims.limbs, "T7.limbs")?));
    let row_limbs = mul(mul(dims.computed, n, "n_c·n_f")?, dims.limbs, "n_c·n_f·L")?;
    set(T::T8Insertions, Some(mul(dims.computed, pair_products, "T8.insertions")?));
    set(T::T8GeneralizedMass, Some(row_limbs));
    set(T::T8Reads, Some(row_limbs));
    let moves = add(pair_products, mul(3, n, "3·n_f")?, "μ·(D_K + 2·D_M) + 3·n_f")?;
    set(T::T8Limbs, Some(mul(mul(dims.computed, dims.limbs, "n_c·L")?, moves, "T8.limbs")?));
    let construction = add(add(mul(positions, modal_resource_ceil_log2(positions), "P_u·log P_u")?, n, "T9.construction")?, positions, "T9.construction")?;
    set(T::T9Construction, Some(construction));
    let twice = mul(2, n, "2·n_f")?.checked_add(1).ok_or(ModalResourceOverflow("2·n_f + 1"))?;
    // n_f(n_f + 1) = 2·positions exactly.
    let squares = modal_resource_ceil_div(mul(mul(2, positions, "n_f(n_f + 1)")?, twice, "n_f(n_f + 1)(2n_f + 1)")?, 12);
    let s = dims.sturm_candidates;
    set(T::T9Factorization, Some(mul(s, add(squares, positions, "½Σw² + Σw")?, "T9.factorization")?));
    set(T::T9Passes, Some(mul(s, add(add(positions, mul(24, positions, "24·P")?, "P_u + 24·P")?, n, "T9.passes")?, "T9.passes")?));
    set(T::T9Neighbours, Some(mul(2, positions, "T9.neighbours")?));
    set(T::T10Products, Some(mul(mul(dims.reported, mu, "n_r·μ")?, dm, "T10.products")?));
    let r = dims.reported;
    let pairs = mul(r, r.checked_add(1).ok_or(ModalResourceOverflow("n_r + 1"))?, "n_r(n_r + 1)")? / 2;
    set(T::T10Bilinear, Some(mul(mul(pairs, n, "n_r(n_r + 1)/2·n_f")?, dims.limbs, "T10.bilinear")?));
    Ok(counts)
}

/// The bounds P0 uses (module docs), from the prepared request, the
/// parameters and the budget only.
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
    let reported = match budget.reported_row_allotment {
        Some(rows) if global > 0 => free.min(u128::from(rows) / global),
        _ => free,
    };
    let pipes = wide(built.pipes.len());
    let arcs = wide(built.curved_bend_elements.len());
    let connectors = wide(built.connectors.len());
    let tie_rods = wide(built.connectors.iter().map(|c| c.tie_rods().len()).sum::<usize>());
    let springs = wide(prepared.boundary.springs.len());
    let skewed = wide(built.skew.as_ref().map_or(0, |basis| basis.springs().len()));
    let segments = wide(prepared.model.pipe_segments.len());
    let twelve = modal_resource_add(modal_resource_add(pipes, arcs, "elements")?, modal_resource_add(connectors, tie_rods, "elements")?, "elements")?;
    let stiffness_squares = modal_resource_add(
        modal_resource_add(modal_resource_mul(144, twelve, "D_K")?, springs, "D_K")?,
        modal_resource_mul(9, skewed, "D_K")?,
        "D_K",
    )?;
    let mass_squares = modal_resource_add(
        modal_resource_add(modal_resource_mul(144, pipes, "D_M")?, modal_resource_mul(72, arcs, "D_M")?, "D_M")?,
        modal_resource_mul(36, segments, "D_M")?,
        "D_M",
    )?;
    let element_operands = [segments, springs, skewed].into_iter().try_fold(twelve, |sum, n| modal_resource_add(sum, n, "E"))?;
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
        reported,
        band_pairs: free.min(u128::from(budget.band_pair_allotment)),
        ql_rotations: u128::from(budget.ql_rotation_allotment),
        sturm_candidates: wide(params.sturm_max_candidates),
        limbs: u128::from(budget.limb_allotment),
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
            (None, _) => inadmissible.push((id, "no bound at P0: its counter is not built".to_string())),
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
    let total = stage_allotments.iter().try_fold(0u128, |sum, &a| modal_resource_add(sum, a, "W")).map_err(modal_resource_overflow_refusal)?;
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
