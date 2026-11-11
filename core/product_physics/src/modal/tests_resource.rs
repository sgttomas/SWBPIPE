//! P0 and S1 in the product (`resource`, `eigen`, `outcome`).
//! Every budget here is **TEST-LOCAL**: no weight, allotment or ceiling is
//! adopted, and none of these values is a proposal. The tests check:
//! - the counts against their formulas evaluated independently;
//! - the bounds against the actual counts on the modal population (support,
//!   not proof);
//! - that R_max and L_A are not charged as bounds;
//! - the unbounded terms' refusals, weight-or-refuse, inclusive boundaries
//!   and overflow;
//! - P0's placement before the state solve;
//! - S1's refusals through the product's own solve;
//! - the changed kernel call against main's call, bit for bit;
//! - the derived limb span L̄ on adversarial exact sums (a witness);
//! - the support terms (`T3.filters`, `T7.assembly`, `T9.construction`):
//!   their closed forms evaluated independently and by hand, their sizes'
//!   bounds against each population model's actual sizes, and the
//!   repository-owned RCM order against `sparse_direct`'s on each
//!   population model's own pattern (old against new; the assembly's and
//!   the formed order's old-against-new oracles are modal_eigen's unit
//!   tests).
use super::eigen::{modal_dense_options as dense_options, solve_modal_pencil, solve_modal_pencil_within};
use super::integrity::modal_reformation_operands;
use super::integrity::{
    MODAL_RESOURCE_ARITHMETIC_OVERFLOW, MODAL_RESOURCE_BAND_ALLOTMENT_EXCEEDED, MODAL_RESOURCE_PREFLIGHT_EXCEEDED,
    MODAL_RESOURCE_QL_ALLOTMENT_EXCEEDED,
};
use super::mass::assemble_mass;
use super::observation_tests::population;
use super::outcome::{run_modal_dense, run_modal_dense_with_budget, ModalOutcome};
use super::phase_measurand_tests::replay_modal_phases;
use super::prepare::prepare_modal;
use super::resource::{
    modal_resource_assembly_bounds, modal_resource_assembly_steps, modal_resource_bound_dimensions, modal_resource_construction_steps,
    modal_resource_filter_steps, modal_resource_judge, modal_resource_kernel_steps, modal_resource_limb_spans, modal_resource_preflight,
    modal_resource_rcm_steps, modal_resource_term_counts, ModalAssemblySizes, ModalResourceBudget, ModalResourceDimensions, ModalResourceOverflow,
    ModalResourceTerm, ModalStateDimensions, MODAL_RESOURCE_LEVEL1_LIMBS, MODAL_RESOURCE_LIMB_BOUND, MODAL_RESOURCE_TERMS,
    MODAL_RESOURCE_TERM_COUNT,
};
use super::tests::{arc_request, codes, modal, pf_request, q, with};
use super::tests_eigen::{sec_a_cantilever, test_local_parameters};
use super::tests_state::kd5;
use super::MODAL_STATE_CASE_UNAVAILABLE;
use crate::Diagnostic;
use open_pipe_stress_modal_eigen::dense::{solve_dense_pencil, BandRecovery, DenseEigenSolution, ScreenedMode};
use open_pipe_stress_modal_eigen::exact::ExactSum;
use open_pipe_stress_modal_eigen::reformation::section::MassBasis;
use open_pipe_stress_modal_eigen::reformation::{reform_element_mass, reform_element_stiffness};
use open_pipe_stress_sparse_direct::SymmetricMatrixEntry;
use serde_json::{json, Value};

const U1: &str = include_str!("../../../../validation/references/kd5_nonlinear/u1_request.json");

/// The terms with no bound at P0 in a request without a state solve.
const UNBOUNDED: [&str; 1] = ["T2.integrity"];
/// And with one.
const UNBOUNDED_WITH_STATE: [&str; 3] = ["T1.integrity", "T1.confirmation", "T2.integrity"];
/// The terms bounded by the repository-owned support operations.
const SUPPORT: [ModalResourceTerm; 3] = [ModalResourceTerm::T3Filters, ModalResourceTerm::T7Assembly, ModalResourceTerm::T9Construction];

/// TEST-LOCAL: every weight 0, the enforced allotments and W_budget at their
/// largest, and L_A and R_max recorded at values P0 must not use.
fn admitting_budget() -> ModalResourceBudget {
    ModalResourceBudget {
        weights: [Some(0); MODAL_RESOURCE_TERM_COUNT],
        work_budget: u128::MAX,
        ql_rotation_allotment: u64::MAX,
        band_pair_allotment: u64::MAX,
        limb_allotment: 202,
        reported_row_allotment: None,
    }
}

/// TEST-LOCAL: every weight 1 ps, the stated enforced allotments.
fn unit_budget(ql: u64, band: u64) -> ModalResourceBudget {
    ModalResourceBudget { weights: [Some(1); MODAL_RESOURCE_TERM_COUNT], ql_rotation_allotment: ql, band_pair_allotment: band, ..admitting_budget() }
}

fn refused(found: Result<ModalOutcome, Vec<Diagnostic>>) -> Vec<Diagnostic> {
    found.err().expect("expected a refusal")
}

fn bounds(request: Value, budget: &ModalResourceBudget) -> ModalResourceDimensions {
    let params = test_local_parameters();
    let parsed = modal(request);
    let prepared = prepare_modal(&parsed).unwrap();
    modal_resource_bound_dimensions(&prepared, &params, budget).unwrap()
}

/// `counts` with every unbounded term given a count, for exercising the
/// arithmetic judgment on its own.
fn as_if_bounded(counts: [Option<u128>; MODAL_RESOURCE_TERM_COUNT], value: u128) -> [Option<u128>; MODAL_RESOURCE_TERM_COUNT] {
    counts.map(|c| Some(c.unwrap_or(value)))
}

// ---------------------------------------------------------------------------
// Counts against their formulas, evaluated independently
// ---------------------------------------------------------------------------

/// T3's kernel steps as one collected polynomial (the per-body audit in
/// `resource::modal_resource_kernel_steps`, summed by power of n):
/// 5·n(n+1)/2 + 2·Σ_i Σ_{j≤i} j + 37·n(n−1)/2 + 3n·n(n−1)/2 + 3·Σ_{t<n} t²
/// + 21n² + 148n + 23 + R(2n + 5).
fn kernel_steps(n: u128, r: u128) -> u128 {
    if n == 0 {
        return 0;
    }
    let (t, pr) = (n * (n - 1) / 2, n * (n + 1) / 2);
    let c6 = (n - 1) * n * (n + 1) / 6;
    let s2 = (n - 1) * n * (2 * n - 1) / 6;
    5 * pr + 2 * c6 + 37 * t + 3 * n * t + 3 * s2 + 21 * n * n + 148 * n + 23 + r * (2 * n + 5)
}

/// `T7.assembly` collected (module docs of `resource`), written out again:
/// (6θ + 24ν + 7)(D_K + D_M) + (55 + L̄)(min(νD_K, P) + min(νD_M, P)) +
/// 73P + 70n + 81 (2·38 + 5: per matrix two assemblies' 2·11, the formed
/// order's 9 and the difference's 7; once the scaling's 5).
fn assembly_steps(n: u128, dk: u128, dm: u128, nu: u128, theta: u128, l: u128) -> u128 {
    let p = n * (n + 1) / 2;
    (6 * theta + 24 * nu + 7) * (dk + dm) + (55 + l) * ((nu * dk).min(p) + (nu * dm).min(p)) + 73 * p + 70 * n + 81
}

/// `T9.construction` collected, written out again: with P = n(n + 1)/2,
/// L = 2P and q = P − n, 13L + 9n + 5P + 24 + 15n + 8q + 14 + (n + 3)(5n +
/// 2q) + 3n(n + 1) = 39P + 16n + 38 + (n + 3)(3n + 2P) + 3n(n + 1).
fn construction_steps(n: u128) -> u128 {
    let p = n * (n + 1) / 2;
    39 * p + 16 * n + 38 + (n + 3) * (3 * n + 2 * p) + 3 * n * (n + 1)
}

/// §5.3.8's terms as `resource` derives them, written out again in plain
/// arithmetic; the values here are small, so nothing overflows.
fn formulas(d: &ModalResourceDimensions) -> [Option<u128>; MODAL_RESOURCE_TERM_COUNT] {
    use ModalResourceTerm as T;
    let (n, g, mu, l) = (d.free, d.global, d.coordinate_multiplier, d.limbs);
    let (c, r, s, b) = (d.computed, d.reported, d.sturm_candidates, d.band_pairs);
    let (dk, dm) = (d.stiffness_squares, d.mass_squares);
    let (part, read) = (5 * l + 16, 12 * l + 32);
    let p = n * (n + 1) / 2;
    let screen = if n == 0 { 0 } else { (n - 1) * n * (n + 1) / 6 };
    let forms = r * (r + 1) / 2;
    let mut x = [Some(0u128); MODAL_RESOURCE_TERM_COUNT];
    if let Some(st) = d.state {
        x[T::T1Factor as usize] = Some(st.solves * screen);
        x[T::T1Entries as usize] = Some(st.solves * 3 * n * n);
        let f = st.friction;
        x[T::T1Coupling as usize] = Some(st.iterations * ((2 * f * f * f).div_ceil(3) + 2 * f * f + f * g));
        x[T::T1Integrity as usize] = None;
        x[T::T1Confirmation as usize] = None;
    }
    x[T::T2Factor as usize] = Some(screen);
    x[T::T2Entries as usize] = Some(3 * n * n);
    x[T::T2Integrity as usize] = None;
    x[T::T3Kernel as usize] = Some(kernel_steps(n, d.ql_rotations));
    x[T::T3Filters as usize] = Some(if n == 0 { 0 } else { 2 * n + 4 });
    x[T::T4Scaling as usize] = Some(if b > 0 { 2 * n * n + n } else { 0 });
    x[T::T4Visits as usize] = Some(4 * b * n * n);
    x[T::T4Insertions as usize] = Some(4 * b * n * n);
    x[T::T4Rows as usize] = Some(b * (n + 2));
    x[T::T4Limbs as usize] = Some(b * (16 * n * n * part + (n + 2) * read));
    x[T::T5Census as usize] = Some(16 * c);
    x[T::T5Outcome as usize] = Some(c * b + 2 * c * c + c);
    x[T::T6Visits as usize] = Some(r * (n * n + 3 * n));
    x[T::T6Limbs as usize] = Some(r * (4 * (n * n + 3 * n) * part + 4 * read));
    x[T::T7Elements as usize] = Some(d.element_operands);
    x[T::T7Assembly as usize] = Some(assembly_steps(n, dk, dm, d.assembly_visits_per_square, d.assembly_steps_per_square, l));
    let entries = dk + dm;
    x[T::T7Limbs as usize] = Some(part * (16 * mu * entries + n * (n + 1) + l * mu * entries) + read * (3 * mu * entries + n * (n + 1)));
    x[T::T8Insertions as usize] = Some(c * mu * (dk + 2 * dm));
    x[T::T8GeneralizedMass as usize] = Some(c * n * l);
    x[T::T8Rows as usize] = Some(c * (2 * n + 1));
    x[T::T8Limbs as usize] = Some(c * (part * (8 * mu * (dk + 2 * dm) + 4 * n * l) + read * (2 * n + 1)));
    x[T::T9Construction as usize] = Some(construction_steps(n));
    let ldlt = if n < 2 { 0 } else { n * (n - 1) * (n - 2) / 6 };
    x[T::T9Factorization as usize] = Some(s * (ldlt + n * n));
    x[T::T9Passes as usize] = Some(s * (29 * p + 98 * n + 70));
    x[T::T9Neighbours as usize] = Some(2 * (p + 3 * n));
    x[T::T9Limbs as usize] = Some(2 * (part * 8 * p + 2 * read));
    x[T::T10Products as usize] = Some(r * mu * dm);
    x[T::T10Bilinear as usize] = Some(forms * n * l);
    x[T::T10Limbs as usize] = Some(r * (part * 8 * mu * dm + read * n) + forms * (part * 4 * n * l + read));
    x
}

#[test]
fn the_counts_are_the_formulas_on_small_models() {
    let budget = unit_budget(1000, 4);
    let s_max = test_local_parameters().sturm_max_candidates as u128;
    // SEC-A N = 1: two nodes, one anchored; one pipe, one segment.
    let sec_a = bounds(with(sec_a_cantilever(1), json!({"mode_count": 6})), &budget);
    let expected = ModalResourceDimensions {
        global: 12,
        free: 6,
        computed: 6,
        reported: 6,
        band_pairs: 4,
        ql_rotations: 1000,
        sturm_candidates: s_max,
        limbs: MODAL_RESOURCE_LIMB_BOUND,
        stiffness_squares: 144,
        mass_squares: 144 + 36,
        element_operands: 2,
        coordinate_multiplier: 1,
        assembly_visits_per_square: 1,
        assembly_steps_per_square: 3,
        state: None,
    };
    assert_eq!(sec_a, expected);
    let counts = modal_resource_term_counts(&sec_a).unwrap();
    assert_eq!(counts, formulas(&sec_a));
    // By hand at n_f = 6: `cholesky`'s k-loop runs Σ_i Σ_{j≤i} j = 35 times
    // (structural.rs:2089), the entries are 21 + 36 + 15 + 36 = 108.
    use ModalResourceTerm as T;
    let at = |t: ModalResourceTerm| counts[t as usize].unwrap();
    assert_eq!((at(T::T2Factor), at(T::T2Entries)), (35, 108));
    assert_eq!(at(T::T9Factorization), s_max * (20 + 36));
    assert_eq!(at(T::T6Visits), 6 * 54);
    // The support terms by hand at n_f = 6 (P = 21), D_K = 144, D_M = 180,
    // ν = 1, θ = 3, L̄ = 492: T3.filters 2·8 = 16; T7.assembly 49·324 +
    // 547·(21 + 21) + 73·21 + 70·6 + 81 = 15,876 + 22,974 + 1,533 + 420 + 81
    // = 40,884; T9.construction 39·21 + 16·6 + 38 + 9·(18 + 42) + 3·6·7 =
    // 819 + 96 + 38 + 540 + 126 = 1,619. (Before the checked reservations:
    // 14, 40,831 and 1,596; the differences are the reservations' and the
    // visit count's steps, 2, 53 and 23.)
    assert_eq!((at(T::T3Filters), at(T::T7Assembly), at(T::T9Construction)), (16, 40_884, 1_619));
    for term in UNBOUNDED {
        let t = MODAL_RESOURCE_TERMS.into_iter().find(|t| t.modal_resource_id() == term).unwrap();
        assert_eq!(counts[t as usize], None, "{term} is unbounded");
    }
    // T3 at small n, from the itemized parts against the collected polynomial,
    // and at n = 1 by hand: 5 + 0 + 0 + 0 + 0 + 21 + 148 + 23 + 7R.
    for (n, r) in [(1, 0), (1, 4), (2, 3), (6, 1000), (37, 12)] {
        assert_eq!(modal_resource_kernel_steps(n, r).unwrap(), kernel_steps(n, r), "n = {n}, R = {r}");
    }
    assert_eq!(modal_resource_kernel_steps(1, 4).unwrap(), 5 + 21 + 148 + 23 + 4 * 7);
    // n = 2, R = 3 by hand: 5·3 + 2·1 + 37·1 + 3·2 + 3·1 + 21·4 + 148·2 + 23 + 3·9 = 493.
    assert_eq!(modal_resource_kernel_steps(2, 3).unwrap(), 493);
    assert_eq!(modal_resource_kernel_steps(0, 9).unwrap(), 0);

    // The arc and the state case: dimensions read from the prepared model,
    // the counts from the formulas.
    for (name, request) in [
        ("arc PF", with(arc_request(false), json!({"mode_count": 6}))),
        ("U-1 PRESSED", with(kd5(U1), json!({"mode_count": 3, "state_case": "load:PRESSED"}))),
    ] {
        let parsed = modal(request.clone());
        let prepared = prepare_modal(&parsed).unwrap();
        let built = &prepared.built;
        let dims = bounds(request, &budget);
        let restrained: std::collections::BTreeSet<usize> = prepared.boundary.restrained_dofs.iter().copied().collect();
        let n_g = 6 * built.nodes.len() as u128;
        assert_eq!((dims.global, dims.free, dims.reported), (n_g, n_g - restrained.len() as u128, dims.free), "{name}");
        let (pipes, arcs, segments) = (built.pipes.len() as u128, built.curved_bend_elements.len() as u128, prepared.model.pipe_segments.len() as u128);
        let springs = prepared.boundary.springs.len() as u128;
        assert_eq!(dims.stiffness_squares, 144 * (pipes + arcs + built.connectors.len() as u128) + springs, "{name}");
        assert_eq!(dims.mass_squares, 144 * pipes + 72 * arcs + 36 * segments, "{name}");
        assert_eq!(modal_resource_term_counts(&dims).unwrap(), formulas(&dims), "{name}");
        if name == "arc PF" {
            assert!(arcs > 0 && dims.state.is_none(), "{name}");
        } else {
            let friction = built.nonlinear_supports.iter().filter(|s| matches!(s.behavior, open_pipe_stress_nonlinear_supports::NonlinearSupportBehavior::Friction)).count() as u128;
            let iterations = crate::DEC_046_PRODUCT_PREVIEW_ACTIVE_SET_MAX_ITERATIONS as u128 + 1;
            assert_eq!(dims.state, Some(ModalStateDimensions { solves: iterations * (2 + friction) + 1, friction, iterations }), "{name}");
        }
    }
}

#[test]
fn r_max_and_l_a_are_recorded_not_charged() {
    // The regression: R_max = 0 and L_A = 0 used to charge T6, T7, T8 and
    // T10 at zero while the reported modes are normalized and checked.
    let request = with(sec_a_cantilever(2), json!({"mode_count": 1}));
    let zero = ModalResourceBudget { limb_allotment: 0, reported_row_allotment: Some(0), ..admitting_budget() };
    let open = ModalResourceBudget { limb_allotment: u64::MAX, reported_row_allotment: None, ..admitting_budget() };
    let (at_zero, at_open) = (bounds(request.clone(), &zero), bounds(request, &open));
    assert_eq!(at_zero, at_open, "P0 does not read R_max or L_A");
    assert_eq!((at_zero.reported, at_zero.limbs), (at_zero.free, MODAL_RESOURCE_LIMB_BOUND));
    let counts = modal_resource_term_counts(&at_zero).unwrap();
    use ModalResourceTerm as T;
    for term in [T::T6Visits, T::T6Limbs, T::T7Limbs, T::T8GeneralizedMass, T::T8Limbs, T::T10Products, T::T10Bilinear, T::T10Limbs] {
        assert!(counts[term as usize].unwrap() > 0, "{} is charged", term.modal_resource_id());
    }
}

// ---------------------------------------------------------------------------
// The limb span L̄ (a witness, not the proof: the proof is `resource`'s docs)
// ---------------------------------------------------------------------------

#[test]
fn the_limb_span_is_its_derivation_and_holds_on_adversarial_sums() {
    assert_eq!(modal_resource_limb_spans(), (MODAL_RESOURCE_LEVEL1_LIMBS as i64, MODAL_RESOURCE_LIMB_BOUND as i64));
    let tiny = f64::from_bits(1); // 2⁻¹⁰⁷⁴
    let (big, odd) = (f64::MAX, f64::from_bits(0x000F_FFFF_FFFF_FFFF)); // a full subnormal significand
    // Level 1: the extremes of five binary64 factors, as three factors and
    // the other two exponents, together with cancelling and tiny parts.
    let mut first = ExactSum::new();
    first.add_product_scaled(&[big, big, big], 2 * 1023);
    first.add_product_scaled(&[tiny, tiny, tiny], -2 * 1074);
    first.add_product_scaled(&[odd, odd, odd], -2 * 1074);
    first.add_product_scaled(&[-big, big, big], 2 * 1023);
    first.add_product_scaled(&[big, big, big], 2 * 1023);
    assert!(first.limb_storage().0 as u128 <= MODAL_RESOURCE_LEVEL1_LIMBS, "{:?}", first.limb_storage());
    // Level 2: its terms times two extreme factors each way.
    let terms = first.terms();
    assert!(terms.len() as u128 <= MODAL_RESOURCE_LEVEL1_LIMBS);
    let mut second = ExactSum::new();
    second.add_terms_product(&terms, &[big, big]);
    second.add_terms_product(&terms, &[tiny, tiny]);
    second.add_terms_product(&terms, &[odd, -odd]);
    assert!(second.limb_storage().0 as u128 <= MODAL_RESOURCE_LIMB_BOUND, "{:?}", second.limb_storage());
}

// ---------------------------------------------------------------------------
// Bound coverage on the population
// ---------------------------------------------------------------------------

/// The actual dimensions of one run, from the product's own phases. L, T1's
/// solves and μ have no counter; they are taken from the bound, so coverage
/// is not claimed for them here.
fn actual_dimensions(request: &Value, bound: &ModalResourceDimensions) -> ModalResourceDimensions {
    let params = test_local_parameters();
    let parsed = modal(request.clone());
    let replay = replay_modal_phases(&parsed, &params).unwrap_or_else(|found| panic!("{found:#?}"));
    let prepared = prepare_modal(&parsed).unwrap();
    let mass = assemble_mass(&prepared, prepared.inputs.weight_to_mass_acceleration).unwrap();
    let operands = modal_reformation_operands(&prepared, &mass).unwrap();
    let basis = if prepared.exact { MassBasis::NominalWallPressureBore } else { MassBasis::EffectiveWall };
    let (mut dk, mut dm) = (0u128, 0u128);
    for named in &operands {
        if let Some(k) = reform_element_stiffness(&named.operand, basis).unwrap() {
            dk += (k.dofs.len() as u128).pow(2);
        }
        for m in reform_element_mass(&named.operand, basis).unwrap() {
            dm += (m.dofs.len() as u128).pow(2);
        }
    }
    let solution = &replay.solution;
    ModalResourceDimensions {
        global: replay.global_dimension as u128,
        free: replay.pencil.free_dofs.len() as u128,
        computed: solution.mode_count() as u128,
        reported: replay.completed.set.count as u128,
        band_pairs: u128::from(solution.attempted_band_pair_count),
        ql_rotations: u128::from(solution.ql_rotations),
        sturm_candidates: replay.sturm.evidence.candidates.len() as u128,
        limbs: bound.limbs,
        stiffness_squares: dk,
        mass_squares: dm,
        element_operands: operands.len() as u128,
        coordinate_multiplier: bound.coordinate_multiplier,
        assembly_visits_per_square: bound.assembly_visits_per_square,
        assembly_steps_per_square: bound.assembly_steps_per_square,
        state: bound.state,
    }
}

#[test]
fn the_bounds_cover_the_actual_counts_on_every_population_model() {
    let budget = admitting_budget();
    let models = population();
    assert_eq!(models.len(), 35);
    let mut band = 0;
    for model in models {
        let name = &model.name;
        let bound = bounds(model.request.clone(), &budget);
        let actual = actual_dimensions(&model.request, &bound);
        for (what, a, b) in [
            ("n_g", actual.global, bound.global),
            ("n_f", actual.free, bound.free),
            ("n_c", actual.computed, bound.computed),
            ("n_r", actual.reported, bound.reported),
            ("p", actual.band_pairs, bound.band_pairs),
            ("R", actual.ql_rotations, bound.ql_rotations),
            ("s", actual.sturm_candidates, bound.sturm_candidates),
            ("D_K", actual.stiffness_squares, bound.stiffness_squares),
            ("D_M", actual.mass_squares, bound.mass_squares),
            ("E", actual.element_operands, bound.element_operands),
        ] {
            assert!(a <= b, "{name}: actual {what} = {a} exceeds its bound {b}");
        }
        assert_eq!(actual.global, bound.global, "{name}: n_g is exact");
        let (counts, covered) = (modal_resource_term_counts(&actual).unwrap(), modal_resource_term_counts(&bound).unwrap());
        for term in MODAL_RESOURCE_TERMS {
            match (counts[term as usize], covered[term as usize]) {
                (Some(a), Some(b)) => assert!(a <= b, "{name}: {} actual {a} exceeds c̄ {b}", term.modal_resource_id()),
                (None, None) => assert!(term.modal_resource_unbounded_reason().is_some()),
                other => panic!("{name}: {}: {other:?}", term.modal_resource_id()),
            }
        }
        band += usize::from(actual.band_pairs > 0);
    }
    // The population attempts band recovery somewhere, so p's bound is used.
    assert!(band > 0, "no population model attempts a band pair");
}

// ---------------------------------------------------------------------------
// The judgment: unbounded terms, weight-or-refuse, boundaries, overflow
// ---------------------------------------------------------------------------

fn sec_a(n: usize) -> Value {
    with(sec_a_cantilever(n), json!({"mode_count": 2}))
}

fn preflight(request: Value, budget: &ModalResourceBudget) -> Result<super::resource::ModalResourcePreflight, Vec<Diagnostic>> {
    let parsed = modal(request);
    let prepared = prepare_modal(&parsed).unwrap();
    modal_resource_preflight(&prepared, &test_local_parameters(), budget)
}

#[test]
fn every_budgeted_request_refuses_naming_its_unbounded_terms() {
    let params = test_local_parameters();
    for model in population() {
        let state = model.request.get("state_case").is_some_and(|c| !c.is_null());
        let found = refused(run_modal_dense_with_budget(&modal(model.request.clone()), &params, Some(&admitting_budget())));
        assert_eq!(codes(&found), [MODAL_RESOURCE_PREFLIGHT_EXCEEDED], "{}", model.name);
        let expected: &[&str] = if state { &UNBOUNDED_WITH_STATE } else { &UNBOUNDED };
        assert_eq!(found[0].affected_refs, expected, "{}", model.name);
        assert!(found[0].message.contains("unbounded:"), "{}", found[0].message);
    }
}

#[test]
fn an_undetermined_weight_with_a_nonzero_count_refuses_naming_the_term() {
    let dims = bounds(sec_a(2), &admitting_budget());
    let counts = as_if_bounded(modal_resource_term_counts(&dims).unwrap(), 1);
    let mut budget = unit_budget(1000, 4);
    budget.weights[ModalResourceTerm::T6Visits as usize] = None;
    let found = modal_resource_judge(dims, &counts, &budget).unwrap_err();
    assert_eq!(codes(&found), [MODAL_RESOURCE_PREFLIGHT_EXCEEDED]);
    assert_eq!(found[0].affected_refs, ["T6.visits"]);
    assert!(found[0].message.contains("T6.visits (count") && found[0].message.contains("weight undetermined"), "{}", found[0].message);
    // A zero count needs no weight: with A_band = 0, p̄ = 0 and every T4
    // count is 0.
    let zero_dims = bounds(sec_a(2), &unit_budget(1000, 0));
    let zero_counts = as_if_bounded(modal_resource_term_counts(&zero_dims).unwrap(), 1);
    let mut zero_band = unit_budget(1000, 0);
    for term in [ModalResourceTerm::T4Scaling, ModalResourceTerm::T4Visits, ModalResourceTerm::T4Insertions, ModalResourceTerm::T4Rows, ModalResourceTerm::T4Limbs] {
        zero_band.weights[term as usize] = None;
    }
    let admitted = modal_resource_judge(zero_dims, &zero_counts, &zero_band).unwrap();
    assert_eq!(admitted.counts[ModalResourceTerm::T4Visits as usize], 0);
    // An unbounded term refuses whatever its weight.
    let found = preflight(with(kd5(U1), json!({"mode_count": 3, "state_case": "load:PRESSED"})), &admitting_budget()).unwrap_err();
    assert_eq!(found[0].affected_refs, UNBOUNDED_WITH_STATE);
}

#[test]
fn the_budget_boundary_is_inclusive_and_names_the_dominant_term() {
    let dims = bounds(sec_a(2), &unit_budget(1000, 4));
    let counts = as_if_bounded(modal_resource_term_counts(&dims).unwrap(), 1);
    let budget = unit_budget(1000, 4);
    let total = modal_resource_judge(dims, &counts, &budget).unwrap().total;
    let at = ModalResourceBudget { work_budget: total, ..budget.clone() };
    let admitted = modal_resource_judge(dims, &counts, &at).unwrap();
    assert_eq!(admitted.total, total);
    assert_eq!(admitted.stage_allotments.iter().sum::<u128>(), total);
    let above = ModalResourceBudget { work_budget: total + 1, ..budget.clone() };
    assert_eq!(modal_resource_judge(dims, &counts, &above).unwrap().total, total);
    // The support terms enter with their own counts, not the stand-in.
    let real = modal_resource_term_counts(&dims).unwrap();
    for term in SUPPORT {
        assert!(real[term as usize].is_some_and(|c| c > 1) && counts[term as usize] == real[term as usize], "{}", term.modal_resource_id());
    }
    let below = ModalResourceBudget { work_budget: total - 1, ..budget };
    let found = modal_resource_judge(dims, &counts, &below).unwrap_err();
    assert_eq!(codes(&found), [MODAL_RESOURCE_PREFLIGHT_EXCEEDED]);
    let dominant = MODAL_RESOURCE_TERMS.iter().copied().max_by_key(|&t| (admitted.charges[t as usize], std::cmp::Reverse(t as usize))).unwrap();
    assert_eq!(found[0].affected_refs, [dominant.modal_resource_id()]);
    assert!(found[0].message.contains(&format!("W = {total} ps")), "{}", found[0].message);
}

#[test]
fn every_overflow_refuses_by_name() {
    let dims = bounds(sec_a(1), &admitting_budget());
    // A count: n_f³ past u128.
    let mut huge = dims;
    huge.free = 1 << 43;
    assert!(modal_resource_term_counts(&huge).is_err());
    // A charge: w = u64::MAX on a count past 2⁶⁴.
    let mut counts = [Some(0u128); MODAL_RESOURCE_TERM_COUNT];
    counts[ModalResourceTerm::T3Kernel as usize] = Some(1 << 70);
    let mut budget = unit_budget(0, 0);
    budget.weights[ModalResourceTerm::T3Kernel as usize] = Some(u64::MAX);
    let found = modal_resource_judge(dims, &counts, &budget).unwrap_err();
    assert_eq!(codes(&found), [MODAL_RESOURCE_ARITHMETIC_OVERFLOW]);
    assert_eq!(found[0].affected_refs, ["model", "T3.kernel"]);
    // A sum: two charges each just over u128::MAX / 2.
    let mut counts = [Some(0u128); MODAL_RESOURCE_TERM_COUNT];
    counts[ModalResourceTerm::T2Factor as usize] = Some(u128::MAX / 2 + 1);
    counts[ModalResourceTerm::T5Census as usize] = Some(u128::MAX / 2 + 1);
    let found = modal_resource_judge(dims, &counts, &unit_budget(0, 0)).unwrap_err();
    assert_eq!(codes(&found), [MODAL_RESOURCE_ARITHMETIC_OVERFLOW]);
    // The support counts: each overflow names its term; the last values
    // that fit do not overflow.
    assert_eq!(modal_resource_filter_steps(u128::MAX), Err(ModalResourceOverflow("T3.filters")));
    assert_eq!(modal_resource_filter_steps(u128::MAX / 2 - 2), Ok(u128::MAX - 1));
    assert_eq!(modal_resource_filter_steps(u128::MAX / 2 - 1), Err(ModalResourceOverflow("T3.filters")));
    assert_eq!(modal_resource_rcm_steps(1 << 64, 0), Err(ModalResourceOverflow("T9.construction")));
    assert_eq!(modal_resource_construction_steps(1 << 64, 0, 1 << 64), Err(ModalResourceOverflow("T9.construction")));
    let mut wide = bounds(sec_a(1), &admitting_budget());
    wide.stiffness_squares = u128::MAX / 3;
    assert_eq!(modal_resource_assembly_bounds(&wide).and_then(|s| modal_resource_assembly_steps(&s)), Err(ModalResourceOverflow("T7.assembly")));
    let sizes = ModalAssemblySizes { order: 0, squares: [0; 2], traversal: [0; 2], visits: [0; 2], positions: [0; 2], formed: [0; 2], limbs: 0 };
    assert_eq!(modal_resource_assembly_steps(&sizes), Ok(2 * 38 + 5));
    assert_eq!(modal_resource_assembly_steps(&ModalAssemblySizes { traversal: [u128::MAX / 6 + 1, 0], ..sizes }), Err(ModalResourceOverflow("T7.assembly")));
}

// ---------------------------------------------------------------------------
// Placement
// ---------------------------------------------------------------------------

#[test]
fn p0_refuses_before_the_state_solve_and_before_the_pencil_guard() {
    // A gap model: without a budget its state solve runs (its static
    // diagnostics are carried) and is refused after it. With a budget, P0
    // refuses first, and nothing of the solve appears.
    let mut u1 = kd5(U1);
    let support = u1["model"]["supports"].as_array_mut().unwrap().iter_mut().find(|s| s["id"] == "support:NL-U").unwrap();
    support["nonlinear"] = json!({"behavior": "gap", "dof": "UZ", "initial_state": "inactive", "closes_when": "negative_displacement", "gap": q(1e-5, "m")});
    let request = with(u1, json!({"mode_count": 3, "state_case": "load:PRESSED"}));
    let params = test_local_parameters();
    let solved = refused(run_modal_dense(&modal(request.clone()), &params));
    assert_eq!(solved.last().unwrap().code, MODAL_STATE_CASE_UNAVAILABLE);
    assert!(codes(&solved).contains(&"NONLINEAR_SUPPORT_LOOP_CONVERGED"), "the witness: the solve ran without a budget");
    let found = refused(run_modal_dense_with_budget(&modal(request), &params, Some(&admitting_budget())));
    assert_eq!(codes(&found), [MODAL_RESOURCE_PREFLIGHT_EXCEEDED], "{found:#?}");

    // A linear model with the dense guard's ceiling at 0 (test-only
    // override): without a budget the pencil's guard refuses; with one, P0
    // refuses before it.
    let request = with(pf_request(false), json!({"mode_count": 1}));
    crate::DENSE_SCRUTINY_CEILING_OVERRIDE.with(|ceiling| ceiling.set(Some(0)));
    let guarded = run_modal_dense(&modal(request.clone()), &params);
    let preflighted = run_modal_dense_with_budget(&modal(request), &params, Some(&admitting_budget()));
    crate::DENSE_SCRUTINY_CEILING_OVERRIDE.with(|ceiling| ceiling.set(None));
    assert_eq!(codes(&refused(guarded)), ["SOLVER_SYSTEM_BLOCKED"]);
    assert_eq!(codes(&refused(preflighted)), [MODAL_RESOURCE_PREFLIGHT_EXCEEDED]);
}

// ---------------------------------------------------------------------------
// S1 through the product's solve
// ---------------------------------------------------------------------------

/// Every field of a solution and of each screened mode, `Option` presence
/// kept, every float as its bits: a bit-for-bit key. The destructuring has
/// no `..`, so a new field fails to compile here until it is keyed.
fn solution_bits(solution: &DenseEigenSolution) -> Vec<String> {
    let DenseEigenSolution {
        dimension,
        condensed_dimension,
        eigenvalues,
        mu,
        vectors,
        mu_max,
        condensed_inverse_norm_estimate,
        condensed_mass_norm,
        resolution_floor,
        classification_threshold,
        ql_resolution_floor,
        hard_classification_threshold,
        formed_backward_error_threshold,
        screened,
        nonpositive_mu_count,
        below_threshold_mu_count,
        ql_iterations,
        ql_rotations,
        screened_pair_count,
        hard_rejected_pair_count,
        attempted_band_pair_count,
        min_cholesky_pivot_ratio,
        structurally_massless_count,
        established_finite_mode_count,
        finite_count_unresolved,
    } = solution;
    let bits = |name: &str, values: &[f64]| format!("{name}:{:x?}", values.iter().map(|x| x.to_bits()).collect::<Vec<_>>());
    let mut key = vec![
        format!(
            "counts:{dimension} {condensed_dimension} {nonpositive_mu_count} {below_threshold_mu_count} {ql_iterations} {ql_rotations} {screened_pair_count} {hard_rejected_pair_count} {attempted_band_pair_count} {structurally_massless_count} {established_finite_mode_count:?} {finite_count_unresolved}"
        ),
        bits("eigenvalues", eigenvalues),
        bits("mu", mu),
        bits("vectors", vectors),
        bits(
            "scalars",
            &[
                *mu_max,
                *condensed_inverse_norm_estimate,
                *condensed_mass_norm,
                *resolution_floor,
                *classification_threshold,
                *ql_resolution_floor,
                *hard_classification_threshold,
                *formed_backward_error_threshold,
                *min_cholesky_pivot_ratio,
            ],
        ),
    ];
    for (index, mode) in screened.iter().enumerate() {
        let ScreenedMode { mu, eigenvalue, hard_rejected, recovery, retained } = mode;
        key.push(format!("screened {index}: hard_rejected {hard_rejected}, retained {retained}"));
        key.push(bits(&format!("screened {index}"), &[*mu, *eigenvalue]));
        key.push(match recovery {
            None => format!("screened {index} recovery: none"),
            Some(BandRecovery { mu, eigenvalue, backward_error, error_estimate }) => {
                bits(&format!("screened {index} recovery: some"), &[*mu, *eigenvalue, *backward_error, *error_estimate])
            }
        });
    }
    key
}

#[test]
fn s1_refuses_by_its_codes_through_the_products_solve() {
    // P0 refuses every budgeted request today, so S1 is exercised through
    // the product's own solve (`eigen::solve_modal_pencil_within`) on the
    // product's own pencil, with the budget's allotments.
    let params = test_local_parameters();
    let (pencil, solution) = population()
        .into_iter()
        .filter(|m| !m.request.get("state_case").is_some_and(|c| !c.is_null()))
        .map(|m| {
            let replay = replay_modal_phases(&modal(m.request), &params).unwrap();
            (replay.pencil, replay.solution)
        })
        .find(|(_, s)| s.attempted_band_pair_count > 0 && s.ql_rotations > 0)
        .expect("a population model without a state case attempts a band pair");
    let (rotations, pairs) = (solution.ql_rotations, solution.attempted_band_pair_count);
    let tight = |ql: u64, band: u64| ModalResourceBudget { ql_rotation_allotment: ql, band_pair_allotment: band, ..admitting_budget() };
    let exact = solve_modal_pencil_within(&pencil, &params, &tight(rotations, pairs).modal_dense_allotments()).unwrap();
    assert_eq!(solution_bits(&exact), solution_bits(&solution), "allotments equal to the actual counts change nothing");
    let found = solve_modal_pencil_within(&pencil, &params, &tight(rotations - 1, pairs).modal_dense_allotments()).unwrap_err();
    assert_eq!(codes(&found), [MODAL_RESOURCE_QL_ALLOTMENT_EXCEEDED]);
    let found = solve_modal_pencil_within(&pencil, &params, &tight(rotations, pairs - 1).modal_dense_allotments()).unwrap_err();
    assert_eq!(codes(&found), [MODAL_RESOURCE_BAND_ALLOTMENT_EXCEEDED]);
    assert_eq!(found[0].id, "diagnostic:modal:resource:kernel");
}

// ---------------------------------------------------------------------------
// Without a budget
// ---------------------------------------------------------------------------

/// Old against new at the one changed numerical call: main called
/// `solve_dense_pencil(K, M, options)` where the product now calls
/// `solve_dense_pencil_with_allotments(.., default)` through
/// `solve_modal_pencil`. Both are evaluated here on each model's own pencil
/// and compared float bit by float bit. The other old-against-new evidence
/// is the committed fixtures' bytes and the producer edges
/// (`tests_document`), which main's code generated.
#[test]
fn the_changed_kernel_call_is_bit_identical_to_mains_call_on_the_population() {
    let params = test_local_parameters();
    for model in population() {
        let replay = replay_modal_phases(&modal(model.request), &params).unwrap();
        let main = solve_dense_pencil(&replay.pencil.stiffness, &replay.pencil.mass, &dense_options(&params)).unwrap();
        let now = solve_modal_pencil(&replay.pencil, &params).unwrap();
        assert_eq!(solution_bits(&now), solution_bits(&main), "{}", model.name);
    }
}

/// Wrapper equivalence only: `run_modal_dense` calls
/// `run_modal_dense_with_budget(.., None)`, so this checks the delegation
/// (by `Debug` text), not old against new.
#[test]
fn run_modal_dense_and_its_none_budget_wrapper_agree() {
    let params = test_local_parameters();
    for model in population() {
        let parsed = modal(model.request);
        let plain = run_modal_dense(&parsed, &params).unwrap_or_else(|found| panic!("{}: {found:#?}", model.name));
        let none = run_modal_dense_with_budget(&parsed, &params, None).unwrap();
        assert_eq!(format!("{none:?}"), format!("{plain:?}"), "{}", model.name);
    }
}

#[test]
fn the_codes_are_the_approved_names_and_the_budget_has_no_default() {
    assert_eq!(MODAL_RESOURCE_PREFLIGHT_EXCEEDED, "MODAL_RESOURCE_PREFLIGHT_EXCEEDED");
    assert_eq!(MODAL_RESOURCE_ARITHMETIC_OVERFLOW, "MODAL_RESOURCE_ARITHMETIC_OVERFLOW");
    assert_eq!(MODAL_RESOURCE_QL_ALLOTMENT_EXCEEDED, "MODAL_RESOURCE_QL_ALLOTMENT_EXCEEDED");
    assert_eq!(MODAL_RESOURCE_BAND_ALLOTMENT_EXCEEDED, "MODAL_RESOURCE_BAND_ALLOTMENT_EXCEEDED");
    assert_eq!(open_pipe_stress_modal_eigen::dense::RESOURCE_ALLOCATION_REFUSED, "MODAL_RESOURCE_ALLOCATION_REFUSED");
    let source = include_str!("resource.rs");
    let definition = &source[..source.find("pub(crate) struct ModalResourceBudget").unwrap()];
    let derive = definition.lines().rev().find(|l| l.starts_with("#[derive")).unwrap();
    assert!(!derive.contains("Default"), "{derive}");
    assert!(!source.contains("impl Default for ModalResourceBudget"));
    // No non-test modal source builds a budget: only this test module does.
    for (file, text) in [
        ("outcome.rs", include_str!("outcome.rs")),
        ("eigen.rs", include_str!("eigen.rs")),
        ("document.rs", include_str!("document.rs")),
        ("integrity.rs", include_str!("integrity.rs")),
        ("state.rs", include_str!("state.rs")),
        ("../modal.rs", include_str!("../modal.rs")),
        ("../lib.rs", include_str!("../lib.rs")),
    ] {
        assert!(!text.contains("ModalResourceBudget {"), "{file} builds a budget");
    }
    assert_eq!(source.matches("ModalResourceBudget {").count(), 2, "only the definition and its impl");
    // The integrity parameters carry no budget.
    let parameters = include_str!("integrity.rs");
    let fields = &parameters[parameters.find("pub(crate) struct ModalIntegrityParameters").unwrap()..];
    let fields = &fields[..fields.find("\n}\n").unwrap()];
    assert!(!fields.contains("Budget") && !fields.contains("allotment"));
}

// ---------------------------------------------------------------------------
// The support terms on the population (old against new for the order)
// ---------------------------------------------------------------------------

/// The reverse Cuthill–McKee order of a formed pencil's union pattern, as
/// `ShiftPencil` formerly formed it: the lower positions where K or M has a
/// nonzero entry, and every diagonal, as both-orientation adjacency lists
/// for `sparse_direct::reverse_cuthill_mckee`. Also the union's size before
/// and after the zero positions are dropped.
fn former_order(n: usize, k: &[SymmetricMatrixEntry], m: &[SymmetricMatrixEntry]) -> (Vec<usize>, usize, usize) {
    let mut all = std::collections::BTreeSet::new();
    let mut kept = std::collections::BTreeSet::new();
    for i in 0..n {
        all.insert((i, i));
        kept.insert((i, i));
    }
    for e in k.iter().chain(m) {
        let key = (e.row.max(e.col), e.row.min(e.col));
        all.insert(key);
        if e.value != 0.0 {
            kept.insert(key);
        }
    }
    // A position is kept when either matrix is nonzero there.
    let mut adjacency = vec![Vec::new(); n];
    for &(r, c) in kept.iter().filter(|(r, c)| r != c) {
        adjacency[r].push(c);
        adjacency[c].push(r);
    }
    (open_pipe_stress_sparse_direct::reverse_cuthill_mckee(&adjacency).unwrap(), all.len(), kept.len())
}

#[test]
fn the_support_terms_cover_the_actual_sizes_and_keep_the_order_on_every_population_model() {
    let params = test_local_parameters();
    let budget = admitting_budget();
    for model in population() {
        let name = &model.name;
        let bound = bounds(model.request.clone(), &budget);
        let replay = replay_modal_phases(&modal(model.request.clone()), &params).unwrap_or_else(|found| panic!("{name}: {found:#?}"));
        let actual = actual_dimensions(&model.request, &bound);
        let n = replay.pencil.free_dofs.len();
        let (k, m) = (&replay.pencil.stiffness_entries, &replay.pencil.mass_entries);
        let wide = |x: usize| x as u128;
        let p = wide(n * (n + 1) / 2);

        // T3.filters: n + 1 per complement, at the actual n.
        assert!(modal_resource_filter_steps(wide(n)).unwrap() <= modal_resource_filter_steps(bound.free).unwrap(), "{name}");

        // T7.assembly: each size within its bound, and the count at the
        // actual sizes within P0's.
        let visits = replay.reformed.assembly_visits().map(wide);
        let positions = [replay.reformed.assembled_stiffness().len(), replay.reformed.assembled_mass().len()].map(wide);
        let formed = [k.len(), m.len()].map(wide);
        let squares = [actual.stiffness_squares, actual.mass_squares];
        for x in 0..2 {
            assert!(visits[x] <= bound.assembly_visits_per_square * squares[x], "{name}: v_{x}");
            assert!(positions[x] <= visits[x].min(p), "{name}: u_{x}");
            assert!(formed[x] <= p, "{name}: F_{x}");
        }
        let at_actual = ModalAssemblySizes {
            order: wide(n),
            squares,
            traversal: squares.map(|d| bound.assembly_steps_per_square * d),
            visits,
            positions,
            formed,
            limbs: bound.limbs,
        };
        let charged = modal_resource_term_counts(&bound).unwrap();
        assert!(modal_resource_assembly_steps(&at_actual).unwrap() <= charged[ModalResourceTerm::T7Assembly as usize].unwrap(), "{name}");

        // T9.construction: L ≤ 2P, the union within P; the order is
        // `sparse_direct`'s on the same pattern.
        let shift = open_pipe_stress_modal_eigen::sturm::ShiftPencil::from_entries(n, k, m).unwrap();
        let (order, before, after) = former_order(n, k, m);
        assert_eq!(shift.order(), order.as_slice(), "{name}: the order is sparse_direct's");
        assert_eq!(shift.pattern_entry_count(), after, "{name}");
        assert!(wide(k.len() + m.len()) <= 2 * p && wide(before) <= p, "{name}");
        let construction = modal_resource_construction_steps(wide(n), wide(k.len() + m.len()), wide(before)).unwrap();
        assert!(construction <= charged[ModalResourceTerm::T9Construction as usize].unwrap(), "{name}");
    }
}

/// Hand count of the complete T7 consumers at n=1 with D_K=D_M=1,
/// no mixed coordinates and Lbar=492. Each matrix is assembled for its
/// constructor range check AND its formation difference. The two phases
/// together read each re-formed sum three times, then each formed sum once.
#[test]
fn t7_limb_charge_includes_both_assembly_passes_and_all_three_reads() {
    let mut dims = bounds(sec_a(1), &unit_budget(0, 0));
    dims.free = 1;
    dims.stiffness_squares = 1;
    dims.mass_squares = 1;
    dims.coordinate_multiplier = 1;
    // Add: 2 assemblies * 8 parts * 2 element squares + 2 formed values
    // + 492 difference terms * 2 positions = 1018 parts at 2476 moves.
    // Read: 3 * 2 re-formed positions + 2 formed positions = 8 at 5936.
    let count = modal_resource_term_counts(&dims).unwrap()[ModalResourceTerm::T7Limbs as usize].unwrap();
    assert_eq!(count, 2_568_056);
    let old_single_assembly_charge = 2_516_568u128;
    assert_eq!(count - old_single_assembly_charge, 51_488);
}

// ---------------------------------------------------------------------------
// The support reservations' typed refusals through the product's routes
// ---------------------------------------------------------------------------

/// The support refusals reach the product typed and are named by the
/// kernel's `MODAL_RESOURCE_*` names through `modal_support_resource_refusal`
/// (`diagnostic:modal:resource:support`), never as a numerical, re-formation
/// or eigensolve refusal: on the actual re-formation and formation-difference
/// route (`reform_modal_pencil`) with a dimension whose structures overflow
/// (no oversized request is made), on the Sturm construction's actual error,
/// and for an allocator refusal (the kernel's own failpoint tests inject it
/// through the same public errors; a product build has no failpoint).
#[test]
fn the_support_refusals_are_named_resource_refusals_on_each_route() {
    use super::eigen::modal_sturm_pencil_refusal;
    use super::integrity::{modal_refusal_code, reform_modal_pencil, NamedOperand};
    use super::resource::modal_support_resource_refusal;
    use open_pipe_stress_modal_eigen::dense::{DenseError, RESOURCE_ALLOCATION_REFUSED};
    use open_pipe_stress_modal_eigen::reformation::{ElementOperand, ReformationFailure, SpringOperand};
    use open_pipe_stress_modal_eigen::sturm::{ShiftPencil, SturmError};
    use open_pipe_stress_modal_eigen::SupportResourceError;
    let huge = 1usize << 60;
    let is_support = |found: &[Diagnostic], code: &str, stage: &str| {
        assert_eq!(codes(found), [code]);
        assert_eq!(found[0].id, "diagnostic:modal:resource:support");
        assert!(found[0].message.contains(stage) && found[0].message.contains("a resource refusal, not a numerical one"), "{}", found[0].message);
    };

    // The re-formed pencil's assembly: the counting starts overflow.
    let spring = NamedOperand { subject: "spring:a".into(), operand: ElementOperand::Spring(SpringOperand::Axis { dof: 0, stiffness: 2.0 }) };
    let found = reform_modal_pencil(&[spring], false, huge, &[Some(0)], None, &[], &[]).unwrap_err();
    is_support(&found, MODAL_RESOURCE_ARITHMETIC_OVERFLOW, "the re-formed pencil's assembly");
    // The formation difference: an empty pencil is formed (nothing to
    // request), and the difference's scaling overflows.
    let found = reform_modal_pencil(&[], false, huge, &[], None, &[], &[]).unwrap_err();
    is_support(&found, MODAL_RESOURCE_ARITHMETIC_OVERFLOW, "the formation difference");

    // The Sturm construction: its actual overflow, an allocator refusal,
    // and a numerical input refusal kept as before.
    let overflow = ShiftPencil::from_entries(huge, &[], &[]).unwrap_err();
    assert!(matches!(overflow, SturmError::SupportResource(SupportResourceError::CapacityOverflow { .. })));
    is_support(&modal_sturm_pencil_refusal(overflow), MODAL_RESOURCE_ARITHMETIC_OVERFLOW, "the Sturm pencil's construction");
    let refused = SupportResourceError::AllocationFailed { structure: "Sturm scaling", bytes: 48 };
    is_support(&modal_sturm_pencil_refusal(SturmError::SupportResource(refused)), RESOURCE_ALLOCATION_REFUSED, "the Sturm pencil's construction");
    let numerical = modal_sturm_pencil_refusal(SturmError::DuplicateEntry { row: 1, col: 0 });
    assert_eq!(codes(&numerical), ["MODAL_EIGENSOLVE_UNAVAILABLE"]);
    assert!(numerical[0].message.contains("the Sturm pencil cannot be formed"));

    // The dense solve's typed error and the re-formation's code map.
    let dense = DenseError::SupportResource(refused);
    assert_eq!(dense.resource_code(), Some(RESOURCE_ALLOCATION_REFUSED));
    is_support(&modal_support_resource_refusal("the dense eigensolve", &refused), RESOURCE_ALLOCATION_REFUSED, "the dense eigensolve");
    assert_eq!(modal_refusal_code(&ReformationFailure::SupportResource(refused)), RESOURCE_ALLOCATION_REFUSED);
    let overflowed = SupportResourceError::CapacityOverflow { structure: "assembly keys" };
    assert_eq!(modal_refusal_code(&ReformationFailure::SupportResource(overflowed)), MODAL_RESOURCE_ARITHMETIC_OVERFLOW);
}
