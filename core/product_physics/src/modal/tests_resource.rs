//! Design §5.3.8 P0 and S1 in the product (`resource`, `eigen`, `outcome`).
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
//! - the derived limb span L̄ on adversarial exact sums (a witness).
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
    modal_resource_bound_dimensions, modal_resource_judge, modal_resource_kernel_steps, modal_resource_limb_spans, modal_resource_preflight,
    modal_resource_term_counts, ModalResourceBudget, ModalResourceDimensions, ModalResourceTerm, ModalStateDimensions,
    MODAL_RESOURCE_LEVEL1_LIMBS, MODAL_RESOURCE_LIMB_BOUND, MODAL_RESOURCE_TERMS, MODAL_RESOURCE_TERM_COUNT,
};
use super::tests::{arc_request, codes, modal, pf_request, q, with};
use super::tests_eigen::{sec_a_cantilever, test_local_parameters};
use super::tests_state::kd5;
use super::MODAL_STATE_CASE_UNAVAILABLE;
use crate::Diagnostic;
use open_pipe_stress_modal_eigen::dense::{solve_dense_pencil, DenseEigenSolution};
use open_pipe_stress_modal_eigen::exact::ExactSum;
use open_pipe_stress_modal_eigen::reformation::section::MassBasis;
use open_pipe_stress_modal_eigen::reformation::{reform_element_mass, reform_element_stiffness};
use serde_json::{json, Value};

const U1: &str = include_str!("../../../../validation/references/kd5_nonlinear/u1_request.json");

/// The terms with no bound at P0 in a request without a state solve.
const UNBOUNDED: [&str; 3] = ["T2.integrity", "T7.assembly", "T9.construction"];
/// And with one.
const UNBOUNDED_WITH_STATE: [&str; 5] = ["T1.integrity", "T1.confirmation", "T2.integrity", "T7.assembly", "T9.construction"];

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

/// T3's kernel steps, the same loops summed in another grouping (module docs
/// of `resource`, `dense.rs:358`).
fn kernel_steps(n: u128, r: u128) -> u128 {
    if n == 0 {
        return 0;
    }
    let (t, pr) = (n * (n - 1) / 2, n * (n + 1) / 2);
    let c6 = (n - 1) * n * (n + 1) / 6;
    let s2 = (n - 1) * n * (2 * n - 1) / 6;
    let g = (n as f64 + 1.0).log2().ceil() as u128 + 1;
    let squares = 2 * n * n + 2 * n * n + n * n + n * n + n * n + n * n + 3 * n * n + n * n + n * n + n * n + n * n + n * n;
    let linear = n + 2 * n + 2 * n + 2 * n + n + n + n + n + 4 * n + 2 * n + n + 7 * n + 110 * n;
    2 * pr + pr + pr + pr + 2 * n * g + 2 * c6 + 2 * n * (t + n) + 2 * t + 10 * t + t + t + t + 3 * s2 + 11 * n * (n - 1) + n * (t + 3 * n) + 2 * r * (n + 1)
        + squares
        + linear
        + 8
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
    x[T::T7Assembly as usize] = None;
    let entries = dk + dm;
    x[T::T7Limbs as usize] = Some(part * (8 * mu * entries + n * (n + 1) + l * mu * entries) + read * (2 * mu * entries + n * (n + 1)));
    x[T::T8Insertions as usize] = Some(c * mu * (dk + 2 * dm));
    x[T::T8GeneralizedMass as usize] = Some(c * n * l);
    x[T::T8Rows as usize] = Some(c * (2 * n + 1));
    x[T::T8Limbs as usize] = Some(c * (part * (8 * mu * (dk + 2 * dm) + 4 * n * l) + read * (2 * n + 1)));
    x[T::T9Construction as usize] = None;
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
    for term in ["T2.integrity", "T7.assembly", "T9.construction"] {
        let t = MODAL_RESOURCE_TERMS.into_iter().find(|t| t.modal_resource_id() == term).unwrap();
        assert_eq!(counts[t as usize], None, "{term} is unbounded");
    }
    // T3 at n = 1 and 2, R = 0, from the parts.
    assert_eq!(modal_resource_kernel_steps(1, 0).unwrap(), kernel_steps(1, 0));
    assert_eq!(modal_resource_kernel_steps(2, 3).unwrap(), kernel_steps(2, 3));
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

/// Every float of a solution as bits, with its counts: a bit-for-bit key.
fn solution_bits(s: &DenseEigenSolution) -> (Vec<u64>, Vec<u64>, Vec<u64>, Vec<u64>, String) {
    let bits = |v: &[f64]| v.iter().map(|x| x.to_bits()).collect::<Vec<_>>();
    let scalars = [
        s.mu_max,
        s.condensed_inverse_norm_estimate,
        s.condensed_mass_norm,
        s.resolution_floor,
        s.classification_threshold,
        s.ql_resolution_floor,
        s.hard_classification_threshold,
        s.formed_backward_error_threshold,
        s.min_cholesky_pivot_ratio,
    ];
    let screened: Vec<f64> = s.screened.iter().flat_map(|m| {
        let r = m.recovery.as_ref();
        [m.mu, m.eigenvalue, r.map_or(0.0, |r| r.mu), r.map_or(0.0, |r| r.backward_error), r.map_or(0.0, |r| r.error_estimate)]
    }).collect();
    let counts = format!(
        "{} {} {} {} {} {} {} {} {} {:?} {}",
        s.dimension, s.condensed_dimension, s.nonpositive_mu_count, s.below_threshold_mu_count, s.ql_iterations, s.ql_rotations,
        s.screened_pair_count, s.attempted_band_pair_count, s.structurally_massless_count, s.established_finite_mode_count, s.finite_count_unresolved
    );
    (bits(&s.eigenvalues), bits(&s.mu), bits(&s.vectors), bits(&[&scalars[..], &screened[..]].concat()), counts)
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
