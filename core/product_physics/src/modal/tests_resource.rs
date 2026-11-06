//! Design §5.3.8 P0 and S1 in the product (`resource`, `eigen`, `outcome`).
//! Every budget here is **TEST-LOCAL**: no weight, allotment or ceiling is
//! adopted, and none of these values is a proposal. The tests check the
//! counts against their formulas evaluated independently, the bounds against
//! the actual counts on the modal population (support, not proof), the
//! weight-or-refuse rule, inclusive boundaries, overflow, P0's placement
//! before the state solve, S1's refusals through the product, and that the
//! entry without a budget is bit for bit the old one.
use super::integrity::{
    ModalIntegrityParameters, MODAL_RESOURCE_ARITHMETIC_OVERFLOW, MODAL_RESOURCE_BAND_ALLOTMENT_EXCEEDED, MODAL_RESOURCE_PREFLIGHT_EXCEEDED,
    MODAL_RESOURCE_QL_ALLOTMENT_EXCEEDED,
};
use super::MODAL_STATE_CASE_UNAVAILABLE;
use super::integrity::modal_reformation_operands;
use super::mass::assemble_mass;
use super::observation_tests::population;
use super::outcome::{run_modal_dense, run_modal_dense_with_budget, ModalOutcome};
use super::phase_measurand_tests::replay_modal_phases;
use super::prepare::prepare_modal;
use super::resource::{
    modal_resource_bound_dimensions, modal_resource_judge, modal_resource_preflight, modal_resource_term_counts, ModalResourceBudget,
    ModalResourceDimensions, ModalResourceTerm, ModalStateDimensions, MODAL_RESOURCE_TERMS, MODAL_RESOURCE_TERM_COUNT,
};
use super::tests::{arc_request, codes, modal, pf_request, q, with};
use super::tests_eigen::{sec_a_cantilever, test_local_parameters};
use super::tests_state::kd5;
use crate::Diagnostic;
use open_pipe_stress_modal_eigen::reformation::section::MassBasis;
use open_pipe_stress_modal_eigen::reformation::{reform_element_mass, reform_element_stiffness};
use serde_json::{json, Value};

const U1: &str = include_str!("../../../../validation/references/kd5_nonlinear/u1_request.json");

/// TEST-LOCAL: a budget that every count is admitted under (weights 0,
/// allotments and W_budget at their largest), and L_A = 202, §5.3.8's
/// working proposal, used here only as a test value.
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

/// TEST-LOCAL: every weight 1 ps, the stated allotments.
fn unit_budget(ql: u64, band: u64) -> ModalResourceBudget {
    ModalResourceBudget { weights: [Some(1); MODAL_RESOURCE_TERM_COUNT], ql_rotation_allotment: ql, band_pair_allotment: band, ..admitting_budget() }
}

fn refused(found: Result<ModalOutcome, Vec<Diagnostic>>) -> Vec<Diagnostic> {
    found.err().expect("expected a refusal")
}

fn debug_of(outcome: &ModalOutcome) -> String {
    format!("{outcome:?}")
}

// ---------------------------------------------------------------------------
// Counts against their formulas, evaluated independently
// ---------------------------------------------------------------------------

/// §5.3.8's formulas, written out again in plain arithmetic (module docs of
/// `resource`); the values here are small, so nothing overflows.
fn formulas(d: &ModalResourceDimensions) -> [Option<u128>; MODAL_RESOURCE_TERM_COUNT] {
    let ceil = |a: u128, b: u128| a.div_ceil(b);
    let (n, g, mu, l) = (d.free, d.global, d.coordinate_multiplier, d.limbs);
    let positions = n * (n + 1) / 2;
    let log2 = if positions <= 1 { 0 } else { (positions as f64).log2().ceil() as u128 };
    let mut c = [Some(0u128); MODAL_RESOURCE_TERM_COUNT];
    use ModalResourceTerm as T;
    if let Some(s) = d.state {
        c[T::T1SolveFactor as usize] = Some(s.solves * ceil(n * n * n, 3));
        c[T::T1SolvePasses as usize] = Some(s.solves * g * g);
        c[T::T1Coupling as usize] = Some(s.iterations * (ceil(2 * s.friction.pow(3), 3) + 2 * s.friction * s.friction + s.friction * g));
        c[T::T1Confirmation as usize] = None;
    }
    c[T::T2Factor as usize] = Some(ceil(n * n * n, 3));
    c[T::T2Passes as usize] = Some(g * g);
    c[T::T3Kernel as usize] = Some(ceil(19 * n * n * n, 3) + 6 * n * d.ql_rotations);
    c[T::T4Scaling as usize] = Some(if d.band_pairs > 0 { 2 * n * n } else { 0 });
    c[T::T4Visits as usize] = Some(4 * d.band_pairs * n * n);
    c[T::T4Insertions as usize] = Some(2 * d.band_pairs * (n * n + n * n));
    c[T::T5Census as usize] = Some(d.computed);
    c[T::T6Visits as usize] = Some(d.reported * (n * n + 3 * n));
    c[T::T7Elements as usize] = Some(d.element_operands);
    c[T::T7Limbs as usize] = Some(mu * (d.stiffness_squares + d.mass_squares) * l);
    let per_pair = mu * (d.stiffness_squares + 2 * d.mass_squares);
    c[T::T8Insertions as usize] = Some(d.computed * per_pair);
    c[T::T8GeneralizedMass as usize] = Some(d.computed * n * l);
    c[T::T8Reads as usize] = Some(d.computed * n * l);
    c[T::T8Limbs as usize] = Some(d.computed * l * (per_pair + 3 * n));
    c[T::T9Construction as usize] = Some(positions * log2 + n + positions);
    c[T::T9Factorization as usize] = Some(d.sturm_candidates * (ceil(n * (n + 1) * (2 * n + 1), 12) + n * (n + 1) / 2));
    c[T::T9Passes as usize] = Some(d.sturm_candidates * (positions + 24 * positions + n));
    c[T::T9Neighbours as usize] = Some(2 * positions);
    c[T::T10Products as usize] = Some(d.reported * mu * d.mass_squares);
    c[T::T10Bilinear as usize] = Some(d.reported * (d.reported + 1) / 2 * n * l);
    c
}

fn bounds(request: Value, budget: &ModalResourceBudget) -> ModalResourceDimensions {
    let params = test_local_parameters();
    let parsed = modal(request);
    let prepared = prepare_modal(&parsed).unwrap();
    modal_resource_bound_dimensions(&prepared, &params, budget).unwrap()
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
        limbs: 202,
        stiffness_squares: 144,
        mass_squares: 144 + 36,
        element_operands: 2,
        coordinate_multiplier: 1,
        state: None,
    };
    assert_eq!(sec_a, expected);
    let counts = modal_resource_term_counts(&sec_a).unwrap();
    assert_eq!(counts, formulas(&sec_a));
    // A few by hand, from §5.3.8's table at these dimensions.
    use ModalResourceTerm as T;
    let at = |t: ModalResourceTerm| counts[t as usize].unwrap();
    assert_eq!(at(T::T2Factor), 72);
    assert_eq!(at(T::T3Kernel), 1368 + 36_000);
    assert_eq!((at(T::T4Scaling), at(T::T4Visits), at(T::T4Insertions)), (72, 576, 576));
    assert_eq!(at(T::T6Visits), 324);
    assert_eq!(at(T::T8Insertions), 6 * (144 + 360));
    assert_eq!(at(T::T9Construction), 21 * 5 + 6 + 21);
    assert_eq!(at(T::T9Factorization), s_max * (46 + 21));
    assert_eq!(at(T::T10Bilinear), 21 * 6 * 202);

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
        assert_eq!((dims.global, dims.free), (n_g, n_g - restrained.len() as u128), "{name}");
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
    // R_max bounds n_r by ⌊R_max/6N⌋, never by mode_count.
    let rows = ModalResourceBudget { reported_row_allotment: Some(12 * 3 + 11), ..budget.clone() };
    assert_eq!(bounds(with(sec_a_cantilever(1), json!({"mode_count": 1})), &rows).reported, 3);
    assert_eq!(bounds(with(sec_a_cantilever(1), json!({"mode_count": 1})), &budget).reported, 6);
}

// ---------------------------------------------------------------------------
// Bound coverage on the population
// ---------------------------------------------------------------------------

/// The actual dimensions of one run, from the product's own phases. L, T1's
/// solves and μ are not observed (no counter exists); they are taken from
/// the bound, so coverage is not claimed for them.
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
                (None, None) => assert_eq!(term, ModalResourceTerm::T1Confirmation),
                other => panic!("{name}: {}: {other:?}", term.modal_resource_id()),
            }
        }
        band += usize::from(actual.band_pairs > 0);
    }
    // The population attempts band recovery somewhere, so p's bound is used.
    assert!(band > 0, "no population model attempts a band pair");
}

// ---------------------------------------------------------------------------
// The judgment: weight-or-refuse, boundaries, overflow
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
fn an_undetermined_weight_with_a_nonzero_count_refuses_naming_the_term() {
    let mut budget = unit_budget(1000, 4);
    budget.weights[ModalResourceTerm::T6Visits as usize] = None;
    let found = preflight(sec_a(2), &budget).unwrap_err();
    assert_eq!(codes(&found), [MODAL_RESOURCE_PREFLIGHT_EXCEEDED]);
    assert_eq!(found[0].affected_refs, ["T6.visits"]);
    assert!(found[0].message.contains("T6.visits (count") && found[0].message.contains("weight undetermined"), "{}", found[0].message);
    // A zero count needs no weight: with A_band = 0, p̄ = 0 and every T4
    // count is 0.
    let mut zero_band = unit_budget(1000, 0);
    for term in [ModalResourceTerm::T4Scaling, ModalResourceTerm::T4Visits, ModalResourceTerm::T4Insertions] {
        zero_band.weights[term as usize] = None;
    }
    let admitted = preflight(sec_a(2), &zero_band).unwrap();
    assert_eq!(admitted.counts[ModalResourceTerm::T4Visits as usize], 0);
    // A state solve's confirmation evaluations have no bound (their counter
    // is a held NI seam): refused whatever the weights.
    let found = preflight(with(kd5(U1), json!({"mode_count": 3, "state_case": "load:PRESSED"})), &admitting_budget()).unwrap_err();
    assert_eq!(codes(&found), [MODAL_RESOURCE_PREFLIGHT_EXCEEDED]);
    assert_eq!(found[0].affected_refs, ["T1.confirmation"]);
}

#[test]
fn the_budget_and_the_allotments_are_inclusive() {
    let budget = unit_budget(1000, 4);
    let total = preflight(sec_a(2), &budget).unwrap().total;
    let at = ModalResourceBudget { work_budget: total, ..budget.clone() };
    let admitted = preflight(sec_a(2), &at).unwrap();
    assert_eq!(admitted.total, total);
    assert_eq!(admitted.stage_allotments.iter().sum::<u128>(), total);
    let below = ModalResourceBudget { work_budget: total - 1, ..budget.clone() };
    let found = preflight(sec_a(2), &below).unwrap_err();
    assert_eq!(codes(&found), [MODAL_RESOURCE_PREFLIGHT_EXCEEDED]);
    // The dominant term is the largest charge (unit weights: the largest count).
    let dominant = MODAL_RESOURCE_TERMS.iter().copied().max_by_key(|&t| (admitted.charges[t as usize], std::cmp::Reverse(t as usize))).unwrap();
    assert_eq!(found[0].affected_refs, [dominant.modal_resource_id()]);
    assert!(found[0].message.contains(&format!("W = {total} ps")), "{}", found[0].message);

    // S1 through the product: the actual R and p pass, one less refuses.
    let params = test_local_parameters();
    let (request, solution) = population()
        .into_iter()
        .filter(|m| !m.request.get("state_case").is_some_and(|c| !c.is_null()))
        .map(|m| {
            let replay = replay_modal_phases(&modal(m.request.clone()), &params).unwrap();
            (m.request, replay.solution)
        })
        .find(|(_, s)| s.attempted_band_pair_count > 0 && s.ql_rotations > 0)
        .expect("a population model without a state case attempts a band pair");
    let (rotations, pairs) = (solution.ql_rotations, solution.attempted_band_pair_count);
    let tight = |ql: u64, band: u64| ModalResourceBudget { ql_rotation_allotment: ql, band_pair_allotment: band, ..admitting_budget() };
    let plain = run_modal_dense(&modal(request.clone()), &params).unwrap();
    let exact = run_modal_dense_with_budget(&modal(request.clone()), &params, Some(&tight(rotations, pairs))).unwrap();
    assert_eq!(debug_of(&exact), debug_of(&plain), "allotments equal to the actual counts change nothing");
    let found = refused(run_modal_dense_with_budget(&modal(request.clone()), &params, Some(&tight(rotations - 1, pairs))));
    assert_eq!(codes(&found), [MODAL_RESOURCE_QL_ALLOTMENT_EXCEEDED]);
    let found = refused(run_modal_dense_with_budget(&modal(request), &params, Some(&tight(rotations, pairs - 1))));
    assert_eq!(codes(&found), [MODAL_RESOURCE_BAND_ALLOTMENT_EXCEEDED]);
    assert_eq!(found[0].id, "diagnostic:modal:resource:kernel");
}

#[test]
fn every_overflow_refuses_by_name() {
    // A charge: w = u64::MAX on T3 at R = A_QL = u64::MAX.
    let mut budget = admitting_budget();
    budget.weights[ModalResourceTerm::T3Kernel as usize] = Some(u64::MAX);
    let found = preflight(sec_a(2), &budget).unwrap_err();
    assert_eq!(codes(&found), [MODAL_RESOURCE_ARITHMETIC_OVERFLOW]);
    assert_eq!(found[0].affected_refs, ["model", "T3.kernel"]);
    // A count: n_f³ past u128.
    let mut dims = bounds(sec_a(1), &admitting_budget());
    dims.free = 1 << 43;
    assert!(modal_resource_term_counts(&dims).is_err());
    // A sum: two charges each just under u128::MAX / 2 + 1.
    let mut counts = [Some(0u128); MODAL_RESOURCE_TERM_COUNT];
    counts[ModalResourceTerm::T2Factor as usize] = Some(u128::MAX / 2 + 1);
    counts[ModalResourceTerm::T5Census as usize] = Some(u128::MAX / 2 + 1);
    let found = modal_resource_judge(dims, &counts, &unit_budget(0, 0)).unwrap_err();
    assert_eq!(codes(&found), [MODAL_RESOURCE_ARITHMETIC_OVERFLOW]);
    // And the product carries P0's overflow refusal.
    let found = refused(run_modal_dense_with_budget(&modal(sec_a(2)), &test_local_parameters(), Some(&budget)));
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
    // override): without a budget the pencil's guard refuses; a refusing
    // budget refuses at P0 instead, before it.
    let request = with(pf_request(false), json!({"mode_count": 1}));
    let mut tight = unit_budget(1000, 4);
    tight.work_budget = 0;
    crate::DENSE_SCRUTINY_CEILING_OVERRIDE.with(|ceiling| ceiling.set(Some(0)));
    let guarded = run_modal_dense(&modal(request.clone()), &params);
    let preflighted = run_modal_dense_with_budget(&modal(request), &params, Some(&tight));
    crate::DENSE_SCRUTINY_CEILING_OVERRIDE.with(|ceiling| ceiling.set(None));
    assert_eq!(codes(&refused(guarded)), ["SOLVER_SYSTEM_BLOCKED"]);
    assert_eq!(codes(&refused(preflighted)), [MODAL_RESOURCE_PREFLIGHT_EXCEEDED]);
}

// ---------------------------------------------------------------------------
// No budget: bit for bit the old entry; the budget has no default
// ---------------------------------------------------------------------------

#[test]
fn without_a_budget_the_entry_is_unchanged_on_the_population() {
    let params = test_local_parameters();
    let mut models: Vec<(String, Value)> = population().into_iter().map(|m| (m.name, m.request)).collect();
    models.push(("SEC-A N=2, a cut-off below mode 1".into(), with(sec_a_cantilever(2), json!({"frequency_cutoff": q(0.5, "Hz")}))));
    for (name, request) in models {
        let parsed = modal(request.clone());
        let plain = run_modal_dense(&parsed, &params).unwrap_or_else(|found| panic!("{name}: {found:#?}"));
        let none = run_modal_dense_with_budget(&parsed, &params, None).unwrap();
        assert_eq!(debug_of(&none), debug_of(&plain), "{name}");
        // An admitting budget changes no value either, where it admits (a
        // state solve is refused by its unbounded confirmation term).
        if plain.nonlinear_state.is_none() {
            let budgeted = run_modal_dense_with_budget(&parsed, &params, Some(&admitting_budget())).unwrap();
            assert_eq!(debug_of(&budgeted), debug_of(&plain), "{name}");
        }
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
    let _: fn() -> ModalIntegrityParameters = test_local_parameters;
}
