//! I2b-1 unit tests (DEL-04-09): the dense modal product path from a modal
//! request to `ModalOutcome`, against the frozen `f1_modal` references (class
//! a: `mass_discrete/eigen_references.json`; class b: the matched FE and
//! continuum of `continuum/fe_convergence.json`), on EX n05, an arc model and
//! a component body, with every refusal, demotion and flag the slice adds,
//! and the test-local O-5 and O-6 observation table. All models are invented.
use super::eigen::{modal_basis, normalize_modal_shape, FiniteCensus, ReportedEndpoint};
use super::integrity::{
    ModalIntegrityParameters, MODAL_FINITE_COUNT_UNRESOLVED, MODAL_INTEGRITY_PARAMETER_INVALID, MODAL_STURM_SHIFT_ILL_CONDITIONED,
    MODAL_STURM_SHIFT_UNSEPARATED,
};
use super::outcome::{run_modal_dense, ModalOutcome, ModalStanding};
use super::stiffness::{DenseGuardRecord, ModalPencil};
use super::tests::{
    arc_request, codes, ex_request_with_density, modal, number, pf_request, q, reference_request, with, PROV, TEST_LOCAL_IDENTITY_BOUND,
};
use super::{MODAL_STATE_CASE_UNAVAILABLE, MODAL_TIMOSHENKO_MASS_UNSUPPORTED};
use crate::Diagnostic;
use open_pipe_stress_modal_eigen::clusters::StoppingBound;
use open_pipe_stress_modal_eigen::modal_quantities::{cumulative_effective_mass_fractions, Dof, FreeDof, NormalizationCase};
use open_pipe_stress_modal_eigen::SymmetricMatrix;
use serde_json::{json, Value};

const EIGEN_REFERENCES: &str = include_str!("../../../../validation/references/f1_modal/mass_discrete/eigen_references.json");
const FE_CONVERGENCE: &str = include_str!("../../../../validation/references/f1_modal/continuum/fe_convergence.json");

/// **TEST-LOCAL, NON-PRODUCTION parameters.** No modal threshold is pinned
/// (DEC-026): these values exist only for these tests'
/// assertions, and no production path reads them. Where I1 used a test-local
/// value it is taken from I1 (`modal_eigen/tests/sturm_counts.rs`,
/// `reformation.rs`): τ_μ = τ_m = 10⁻¹², c_μ = 1, κ_μ = 10, c_f = 64,
/// τ_e = 10⁻⁶ (Agent 0's test-local candidate), 30 QL iterations, δ_c = 10⁻⁸,
/// τ_o = 10⁻⁷, τ_t = 10⁻⁶, κ_c = 10³, κ_s = 10², g_max = 10¹⁵, s_max = 7
/// with I1's positions and offsets; the identity bound is I2a's. τ_r has no
/// I1 value: 10⁻¹⁰ here is test-local only, above the design's "τ_r lies
/// above 10⁻¹²".
pub(super) fn test_local_parameters() -> ModalIntegrityParameters {
    ModalIntegrityParameters {
        infinite_mu_relative_threshold: 1e-12,
        mass_pivot_relative_threshold: 1e-12,
        resolution_floor_constant: 1.0,
        resolution_floor_multiple: 10.0,
        formed_backward_error_multiple: 64.0,
        screen_error_estimate_threshold: 1e-6,
        max_ql_iterations_per_eigenvalue: 30,
        residual_threshold: 1e-10,
        orthogonality_threshold: 1e-7,
        cluster_tolerance: 1e-8,
        no_translation_threshold: 1e-6,
        sturm_conditioning_multiple: 1e3,
        sturm_separation_multiple: 1e2,
        sturm_growth_limit: 1e15,
        sturm_max_candidates: 7,
        sturm_gap_log_positions: vec![0.25, 0.75, 0.125, 0.375, 0.625, 0.875],
        sturm_whole_spectrum_offsets: vec![1.0, 0.5, 0.25],
        identity_tolerance: TEST_LOCAL_IDENTITY_BOUND,
    }
}

/// **TEST-LOCAL, NON-PRODUCTION** class (a) comparison bound: the design's
/// seed of 10⁻⁹ relative in frequency (DEC-026, not pinned). The
/// frozen references state no product tolerance of their own.
const TEST_LOCAL_CLASS_A_FREQUENCY: f64 = 1e-9;

fn modal_run(request: Value) -> Result<ModalOutcome, Vec<Diagnostic>> {
    modal_run_with(request, &test_local_parameters())
}

fn modal_run_with(request: Value, params: &ModalIntegrityParameters) -> Result<ModalOutcome, Vec<Diagnostic>> {
    run_modal_dense(&modal(request), params)
}

fn modal_solved(request: Value) -> ModalOutcome {
    modal_run(request).unwrap_or_else(|found| panic!("refused: {found:#?}"))
}

fn refusal_codes(request: Value, params: &ModalIntegrityParameters) -> Vec<String> {
    match modal_run_with(request, params) {
        Ok(outcome) => panic!("expected a refusal, got {} modes", outcome.modes.len()),
        Err(found) => codes(&found).into_iter().map(str::to_string).collect(),
    }
}

/// The O-6 observations of one outcome.
#[derive(Debug, Clone)]
struct Observation {
    model: String,
    n_free: usize,
    reported: usize,
    max_frequency_error: Option<f64>,
    max_eta: f64,
    max_error_estimate: f64,
    orthogonality: f64,
    sigma: f64,
    conditioning_product: f64,
    separation_ratio: Option<f64>,
    growth: f64,
    candidates: usize,
    formation_k: f64,
    formation_m: f64,
}

fn modal_observation(model: &str, outcome: &ModalOutcome, max_frequency_error: Option<f64>) -> Observation {
    let accepted = outcome.integrity.sturm.evidence.accepted().expect("a counted outcome has an accepted shift");
    let factored = accepted.factored.as_ref().expect("an accepted shift was factored");
    let fold = |f: fn(&super::outcome::ComputedModeEvidence) -> f64| outcome.integrity.computed.iter().map(f).fold(0.0f64, f64::max);
    Observation {
        model: model.to_string(),
        n_free: outcome.guard.free_dimension,
        reported: outcome.modes.len(),
        max_frequency_error,
        max_eta: fold(|e| e.residual.backward_error),
        max_error_estimate: fold(|e| e.residual.error_estimate),
        orthogonality: outcome.integrity.orthogonality.max_ratio,
        sigma: accepted.sigma,
        conditioning_product: factored.conditioning_product,
        separation_ratio: factored.separation_ratio,
        growth: factored.growth,
        candidates: outcome.integrity.sturm.evidence.candidates.len(),
        formation_k: outcome.integrity.formation.stiffness.norm,
        formation_m: outcome.integrity.formation.mass.norm,
    }
}

fn print_observations(title: &str, rows: &[Observation]) {
    println!("{title} (TEST-LOCAL observations, debug build; not a pin, not qualification)");
    println!("| model | n | reported | max rel f err | max η | max e_λ | M-orth | σ | e_b·ν | sep ratio | g | cand | ‖DΔKD‖ | ‖DΔMD‖ |");
    println!("|---|---|---|---|---|---|---|---|---|---|---|---|---|---|");
    for r in rows {
        let err = r.max_frequency_error.map_or("-".to_string(), |e| format!("{e:.2e}"));
        let sep = r.separation_ratio.map_or("-".to_string(), |s| format!("{s:.2e}"));
        println!(
            "| {} | {} | {} | {err} | {:.2e} | {:.2e} | {:.2e} | {:.4e} | {:.2e} | {sep} | {:.2e} | {} | {:.2e} | {:.2e} |",
            r.model, r.n_free, r.reported, r.max_eta, r.max_error_estimate, r.orthogonality, r.sigma, r.conditioning_product, r.growth, r.candidates, r.formation_k, r.formation_m
        );
    }
}

// ---------------------------------------------------------------------------
// Class (a): the frozen discrete eigen references through the product path
// ---------------------------------------------------------------------------

fn eigen_cases() -> Vec<Value> {
    let references: Value = serde_json::from_str(EIGEN_REFERENCES).unwrap();
    references["cases"].as_array().unwrap().clone()
}

/// A frozen eigen case as a product request (I2a's converter, with the
/// case's springs authored as spring supports), extracting `mode_count`.
fn eigen_request(case: &Value, mode_count: usize) -> Value {
    let mut request = reference_request(case, "PF").expect("eigen cases hold straights, arcs and components only");
    for (i, spring) in case["model"]["springs"].as_array().unwrap().iter().enumerate() {
        let dof = spring["dof"].as_str().unwrap().to_uppercase();
        let unit = if dof.starts_with('U') { "N/m" } else { "N*m/rad" };
        request["model"]["supports"].as_array_mut().unwrap().push(json!({"id": format!("spring:{i}"), "node": spring["node"], "family": "spring",
            "restraints": [dof], "stiffness": {"dof": dof, "value": q(number(&spring["stiffness"]), unit)}, "provenance": PROV}));
    }
    assert!(case["model"]["constant_effort_supports"].as_array().unwrap().is_empty());
    request["mode_count"] = json!(mode_count);
    request
}

fn reference_frequency(mode: &Value) -> f64 {
    mode["frequency_Hz"].as_str().unwrap().parse().unwrap()
}

/// The lowest reference modes that end at a reference cluster boundary
/// strictly below the reported top, for a request that must stop below a
/// completed computed cluster.
fn lower_cluster_count(case: &Value) -> usize {
    let clusters = case["clusters"].as_array().unwrap();
    clusters[..clusters.len().min(3)].iter().map(|c| c["modes"].as_array().unwrap().len()).sum()
}

/// Compares one outcome with its reference case: frequencies, the cluster
/// effective-mass sums (normalization- and basis-invariant) and the Sturm
/// count. Returns the largest relative frequency error. A frequency outside
/// the test-local class (a) seed must still lie within the product's own
/// first-order estimate (e_λ/2 in frequency), and is admitted only for the
/// named modes of [`BEYOND_SEED_EXCEPTIONS`] (O-6 observation: the binary64
/// formation of a rigid-scale pencil, not a defect of the solve); every
/// other mode must meet the seed. Every operand is finite.
/// The only (case, mode) pairs allowed beyond the class (a) seed.
const BEYOND_SEED_EXCEPTIONS: [(&str, usize); 2] = [("E-RIGID-SCALE-1e6", 1), ("E-RIGID-SCALE-1e6", 2)];

fn compare_eigen(case: &Value, outcome: &ModalOutcome, beyond_seed: &mut Vec<String>) -> f64 {
    let id = case["id"].as_str().unwrap();
    let modes = case["modes"].as_array().unwrap();
    let mut largest = 0.0f64;
    for mode in &outcome.modes {
        let expected = reference_frequency(&modes[mode.index - 1]);
        let error = (mode.frequency - expected).abs() / expected;
        assert!(
            mode.frequency.is_finite() && expected.is_finite() && expected > 0.0 && error.is_finite(),
            "{id} mode {}: non-finite comparison ({} Hz against {expected} Hz)",
            mode.index,
            mode.frequency
        );
        if !(error <= TEST_LOCAL_CLASS_A_FREQUENCY) {
            assert!(
                BEYOND_SEED_EXCEPTIONS.contains(&(id, mode.index)),
                "{id} mode {}: {} Hz against {expected} Hz ({error:e}) is beyond the seed and not a named exception",
                mode.index,
                mode.frequency
            );
            assert!(
                mode.cluster_error_estimate.is_finite() &&
                error <= mode.cluster_error_estimate / 2.0,
                "{id} mode {}: {} Hz against {expected} Hz ({error:e}; e_λ {:e}, η {:e})",
                mode.index,
                mode.frequency,
                mode.cluster_error_estimate,
                mode.residual.backward_error
            );
            beyond_seed.push(format!("{id} mode {}: rel f error {error:.2e}, e_λ/2 {:.2e}", mode.index, mode.cluster_error_estimate / 2.0));
        }
        largest = largest.max(error);
    }
    for cluster in case["clusters"].as_array().unwrap() {
        let members: Vec<usize> = cluster["modes"].as_array().unwrap().iter().map(|m| m.as_u64().unwrap() as usize).collect();
        if members.iter().any(|&m| m > outcome.modes.len()) {
            continue;
        }
        for (k, axis) in ["X", "Y", "Z"].iter().enumerate() {
            let expected: f64 = cluster["effective_mass_sum_kg"][axis].as_str().unwrap().parse().unwrap();
            let product: f64 = members.iter().map(|&m| outcome.modes[m - 1].effective_mass[k]).sum();
            let scale = outcome.mass_summary.total_by_direction[k];
            assert!((product - expected).abs() <= TEST_LOCAL_CLASS_A_FREQUENCY * scale, "{id} cluster {:?} m_eff {axis}: {product} against {expected}", members);
        }
    }
    assert_eq!(outcome.integrity.sturm.count, outcome.modes.len(), "{id}");
    largest
}

#[test]
fn class_a_eigen_references_agree_through_the_product_path() {
    let mut rows = Vec::new();
    let mut lower_only = Vec::new();
    let mut beyond_seed = Vec::new();
    for case in eigen_cases() {
        let id = case["id"].as_str().unwrap().to_string();
        let reported = case["reported_mode_count"].as_u64().unwrap() as usize;
        let outcome = match modal_run(eigen_request(&case, reported)) {
            Ok(outcome) => {
                assert_eq!(outcome.modes.len(), reported, "{id}");
                outcome
            }
            Err(found) => {
                // Only an unresolved census may refuse, and only the
                // whole-spectrum endpoint; a lower bound is then served.
                assert_eq!(codes(&found), vec![MODAL_FINITE_COUNT_UNRESOLVED], "{id}: {found:#?}");
                lower_only.push(id.clone());
                let outcome = modal_solved(eigen_request(&case, lower_cluster_count(&case)));
                assert!(matches!(outcome.integrity.census, FiniteCensus::Unresolved { .. }), "{id}");
                assert!(matches!(outcome.completed.endpoint, ReportedEndpoint::BelowComputedCluster { .. }), "{id}");
                outcome
            }
        };
        assert_eq!(outcome.standing, ModalStanding::ChecksPassed, "{id}");
        let error = compare_eigen(&case, &outcome, &mut beyond_seed);
        rows.push(modal_observation(&id, &outcome, Some(error)));
    }
    print_observations("Class (a) eigen references through the product path", &rows);
    println!("served below a completed computed cluster (census unresolved): {lower_only:?}");
    println!("beyond the 1e-9 seed but within the product's own e_λ/2: {beyond_seed:#?}");
    // Exactly the named exceptions, no more and no fewer.
    let named: Vec<String> = BEYOND_SEED_EXCEPTIONS.iter().map(|(case, mode)| format!("{case} mode {mode}")).collect();
    let seen: Vec<String> = beyond_seed.iter().map(|row| row.split(':').next().unwrap().to_string()).collect();
    assert_eq!(seen, named, "{beyond_seed:#?}");
    // GENERAL's offset point lump is the design's non-coordinate witness.
    assert!(lower_only.iter().any(|id| id == "E-POINTMASS-GENERAL"), "{lower_only:?}");
    assert!(rows.len() == eigen_cases().len());
}

// ---------------------------------------------------------------------------
// Class (b): the matched FE refinement and the Rayleigh continuum
// ---------------------------------------------------------------------------

/// SEC-A of `continuum.json` (OD 0.2 m, wall 0.008 m, 7850 kg/m³, contents
/// 1000 kg/m³, E 200 GPa, G 80 GPa, no mill tolerance) as an `n`-element
/// cantilever of length 6 m along X, anchored at x = 0.
pub(super) fn sec_a_cantilever(elements: usize) -> Value {
    let length = 6.0;
    let nodes: Vec<Value> = (0..=elements)
        .map(|i| json!({"id": format!("n{i}"), "position": {"x": length * i as f64 / elements as f64, "y": 0.0, "z": 0.0}, "provenance": PROV}))
        .collect();
    let section = json!({"outside_diameter": q(0.2, "m"), "wall_thickness": q(0.008, "m"), "material_density": q(7850.0, "kg/m^3"),
        "contents_density": q(1000.0, "kg/m^3")});
    let pipes: Vec<Value> = (0..elements)
        .map(|i| json!({"id": format!("p{i}"), "from": format!("n{i}"), "to": format!("n{}", i + 1), "section": section, "material": "m",
            "y_reference": {"x": 0, "y": 1, "z": 0}, "provenance": PROV}))
        .collect();
    json!({"model": {"schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:i2b-continuum", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": nodes, "pipe_segments": pipes,
        "materials": [{"id": "m", "elastic_modulus": q(200e9, "Pa"), "shear_modulus": q(80e9, "Pa"), "provenance": PROV}],
        "supports": [{"id": "s", "node": "n0", "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV}],
        "components": [],
        "load_cases": [{"id": "case:f", "primitive_loads": [{"id": "load:f", "category": "concentrated_force", "target": {"type": "node", "node": format!("n{elements}")},
            "direction": "global_y", "magnitude": q(1000.0, "N"), "dimension": "force", "provenance": PROV}], "provenance": PROV}],
        "combinations": []},
        "materials": []})
}

/// √(EI/(m̄L⁴)) of SEC-A at 6 m, in binary64 from the same inputs.
fn sec_a_beam_scale() -> f64 {
    use std::f64::consts::PI;
    let (od, t, length) = (0.2f64, 0.008f64, 6.0f64);
    let id = od - 2.0 * t;
    let area = PI / 4.0 * (od * od - id * id);
    let inertia = PI / 64.0 * (od.powi(4) - id.powi(4));
    let mbar = 7850.0 * area + 1000.0 * PI / 4.0 * id * id;
    (200e9 * inertia / (mbar * length.powi(4))).sqrt()
}

#[test]
fn class_b_rayleigh_cantilever_converges_through_the_product_path() {
    let references: Value = serde_json::from_str(FE_CONVERGENCE).unwrap();
    let family = references["rayleigh"].as_array().unwrap().iter().find(|f| f["id"] == "FE-RAY-CANTILEVER-SEC-A-L6").unwrap().clone();
    let continuum: Vec<f64> = family["continuum_Omega"].as_array().unwrap().iter().map(|v| v.as_str().unwrap().parse().unwrap()).collect();
    let scale = sec_a_beam_scale();
    let mut rows = Vec::new();
    let mut o5 = Vec::new();
    let mut previous_error = f64::INFINITY;
    for refinement in family["refinement"].as_array().unwrap() {
        let elements = refinement["N"].as_u64().unwrap() as usize;
        if elements > 8 {
            continue;
        }
        let outcome = modal_solved(with(sec_a_cantilever(elements), json!({"mode_count": 6 * elements})));
        assert_eq!(outcome.standing, ModalStanding::ChecksPassed);
        let omegas: Vec<f64> = outcome.modes.iter().map(|m| m.circular_frequency / scale).collect();
        let mut largest = 0.0f64;
        for (n, value) in refinement["Omega^2"].as_array().unwrap().iter().enumerate().take(4) {
            // The matched FE (class a for this element): each bending
            // frequency appears once per plane, a pair in 3D.
            let expected = value.as_str().unwrap().parse::<f64>().unwrap().sqrt();
            let matched: Vec<f64> = omegas.iter().copied().filter(|o| (o - expected).abs() <= TEST_LOCAL_CLASS_A_FREQUENCY * expected).collect();
            assert_eq!(matched.len(), 2, "N = {elements}, bending mode {}: {expected} in {omegas:?}", n + 1);
            largest = largest.max(matched.iter().map(|o| (o - expected).abs() / expected).fold(0.0, f64::max));
            // Class (b): from above, at the reference's own stated error.
            let error = expected / continuum[n] - 1.0;
            let stated: f64 = refinement["rel_freq_error"][n].as_str().unwrap().parse().unwrap();
            assert!(error > 0.0 && (error - stated).abs() <= 1e-5 * stated + 1e-12, "N = {elements} mode {}: {error:e} against {stated:e}", n + 1);
            if n == 0 {
                assert!(error < previous_error, "mode 1 converges under refinement");
                previous_error = error;
            }
            // O-5: f_e / f_n against the frequency error (test-local).
            let f_e = outcome.discretization.iter().map(|m| m.fixed_fixed_frequency).fold(f64::INFINITY, f64::min);
            o5.push((elements, n + 1, f_e / (expected * scale / (2.0 * std::f64::consts::PI)), error));
        }
        rows.push(modal_observation(&format!("SEC-A cantilever N = {elements}"), &outcome, Some(largest)));
    }
    print_observations("Class (b) Rayleigh cantilever, SEC-A, L = 6 m", &rows);
    println!("O-5 observations (TEST-LOCAL; c is not adopted, no warning decision): | N | mode | f_e/f_n | rel f error vs continuum |");
    for (n, mode, ratio, error) in &o5 {
        println!("| {n} | {mode} | {ratio:.3} | {error:.3e} |");
    }
}

// ---------------------------------------------------------------------------
// Product models: EX n05, an arc, a component body
// ---------------------------------------------------------------------------

#[test]
fn ex_n05_an_arc_and_a_component_body_pass_through_the_product_path() {
    let mut rows = Vec::new();
    // n05's second bending pair (λ ≈ 5.42·10⁶) is split at 1.4·10⁻⁸ relative,
    // above the test-local δ_c, so a bound of 4 stops between the pair's
    // members and is refused (`an_unseparated_or_ill_conditioned_final_shift_refuses`);
    // a bound of 3 ends below a well-separated gap.
    let ex = modal_solved(with(ex_request_with_density(), json!({"mode_count": 3})));
    assert!(ex.exact_route && ex.standing == ModalStanding::ChecksPassed, "{:?}", ex.standing);
    assert!(ex.disclosures.iter().any(|d| d.contains("pressure stiffening is not modelled")));
    rows.push(modal_observation("EX n05", &ex, None));

    let arc = modal_solved(with(arc_request(false), json!({"mode_count": 6})));
    assert_eq!(arc.standing, ModalStanding::ChecksPassed);
    assert!(arc.integrity.formation.section.is_some());
    rows.push(modal_observation("arc PF", &arc, None));

    let body = modal_solved(with(pf_request(true), json!({"mode_count": 3, "weight_to_mass_acceleration": q(9.8125, "m/s^2")})));
    assert_eq!(body.standing, ModalStanding::ChecksPassed);
    assert!(body.disclosures.iter().any(|d| d.contains("component bodies are point masses")));
    assert!(body.identity.acceleration == 9.8125);
    // The body replaces its member's distributed mass: no f_e for it.
    assert!(body.discretization.iter().all(|m| m.pipe != "body:v"));
    rows.push(modal_observation("component PF", &body, None));

    // Each reported mode is in one normalization case with its +1 component.
    for outcome in [&ex, &arc, &body] {
        for mode in &outcome.modes {
            assert_eq!(mode.shape[mode.reference_dof], 1.0);
            let translational = mode.reference_dof % 6 < 3;
            assert_eq!(translational, mode.normalization == NormalizationCase::UnitMaxTranslation);
        }
        // T_k is the fractions' denominator; Bessel's inequality holds.
        for k in 0..3 {
            let sum: f64 = outcome.modes.iter().map(|m| m.effective_mass[k]).sum();
            assert!(sum <= outcome.mass_summary.total_by_direction[k] * (1.0 + 1e-12));
        }
    }
    print_observations("Product models", &rows);
}

// ---------------------------------------------------------------------------
// The reported set: cluster completion, census, endpoints
// ---------------------------------------------------------------------------

#[test]
fn a_partial_cluster_at_the_bound_is_extended_and_disclosed() {
    // A circular cantilever's bending modes are exact pairs: a bound of one
    // mode stops inside the first pair.
    let outcome = modal_solved(with(sec_a_cantilever(2), json!({"mode_count": 1})));
    assert_eq!((outcome.reported.bounded_count, outcome.reported.extension_count, outcome.reported.count), (1, 1, 2));
    assert_eq!(outcome.reported.stopped_by, StoppingBound::ModeCount);
    assert_eq!(outcome.modes.len(), 2);
    assert_eq!(outcome.integrity.sturm.count, 2);
    assert!(outcome.disclosures.iter().any(|d| d.contains("1 modes added to complete a cluster")), "{:#?}", outcome.disclosures);
    assert!(matches!(outcome.completed.endpoint, ReportedEndpoint::BelowComputedCluster { next_mode: 2 }));
    // The cut-off is inclusive and extends the same way.
    let f1 = outcome.modes[0].frequency;
    let cut = modal_solved(with(sec_a_cantilever(2), json!({"frequency_cutoff": q(f1, "Hz")})));
    assert_eq!(cut.modes.len(), 2);
    assert_eq!(cut.reported.stopped_by, StoppingBound::FrequencyCutoff);
    // Below the lowest mode: nothing reported, a zero count below mode 1.
    let empty = modal_solved(with(sec_a_cantilever(2), json!({"frequency_cutoff": q(f1 / 2.0, "Hz")})));
    assert!(empty.modes.is_empty() && empty.integrity.sturm.count == 0);
    assert_eq!(empty.completed.endpoint, ReportedEndpoint::BelowLowestMode);
    // Beyond the spectrum on an established census: the whole finite spectrum.
    let whole = modal_solved(with(sec_a_cantilever(2), json!({"mode_count": 1000})));
    assert_eq!(whole.reported.stopped_by, StoppingBound::SpectrumExhausted);
    assert_eq!(whole.completed.endpoint, ReportedEndpoint::WholeFiniteSpectrum { established_finite_mode_count: 12 });
    assert_eq!(whole.integrity.sturm.count, 12);
}

fn general_point_mass_case() -> Value {
    eigen_cases().into_iter().find(|c| c["id"] == "E-POINTMASS-GENERAL").unwrap()
}

#[test]
fn a_non_coordinate_mass_nullspace_refuses_the_whole_spectrum_endpoint() {
    // GENERAL's offset point lump: a non-coordinate mass nullspace. The
    // census stays unresolved; no rank is truncated.
    let case = general_point_mass_case();
    for bound in [json!({"mode_count": 21}), json!({"mode_count": 1000}), json!({"frequency_cutoff": q(1e9, "Hz")})] {
        let mut request = eigen_request(&case, 1);
        request.as_object_mut().unwrap().remove("mode_count");
        let found = refusal_codes(with(request, bound.clone()), &test_local_parameters());
        assert_eq!(found, vec![MODAL_FINITE_COUNT_UNRESOLVED], "{bound}");
    }
    // A bound below a completed computed cluster is served, and says so.
    let outcome = modal_solved(eigen_request(&case, 4));
    assert!(matches!(outcome.integrity.census, FiniteCensus::Unresolved { mass_block_established: false, .. }), "{:?}", outcome.integrity.census);
    assert!(outcome.disclosures.iter().any(|d| d.contains("finite-mode count is not established")));
    assert_eq!(outcome.integrity.sturm.count, 4);
}

#[test]
fn a_coordinate_massless_node_is_condensed_and_counted() {
    // NODE's point lump at its own node: three structurally massless
    // rotations, condensed; the census of 21 is established.
    let case = eigen_cases().into_iter().find(|c| c["id"] == "E-POINTMASS-NODE").unwrap();
    let outcome = modal_solved(eigen_request(&case, 21));
    assert_eq!(outcome.integrity.census, FiniteCensus::Established { finite_mode_count: 21, structurally_massless: 3 });
    assert_eq!(outcome.completed.endpoint, ReportedEndpoint::WholeFiniteSpectrum { established_finite_mode_count: 21 });
}

// ---------------------------------------------------------------------------
// Refusals
// ---------------------------------------------------------------------------

#[test]
fn a_mechanism_is_refused_before_any_eigen_solve() {
    // Two pinned ends restrain six ground DOFs, so the static pre-check
    // passes, but the line can twist about its own axis.
    let mut request = with(pf_request(false), json!({"mode_count": 1}));
    request["model"]["supports"] = json!([
        {"id": "s:a", "node": "a", "family": "anchor", "restraints": ["UX", "UY", "UZ"], "provenance": PROV},
        {"id": "s:c", "node": "c", "family": "anchor", "restraints": ["UX", "UY", "UZ"], "provenance": PROV}]);
    let found = refusal_codes(request, &test_local_parameters());
    assert_eq!(found.len(), 1, "{found:?}");
    assert_eq!(found[0], "NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM", "{found:?}");
}

#[test]
fn the_dense_guard_refuses_before_any_n2_allocation() {
    crate::DENSE_SCRUTINY_CEILING_OVERRIDE.with(|ceiling| ceiling.set(Some(1)));
    let found = modal_run(with(pf_request(false), json!({"mode_count": 1})));
    crate::DENSE_SCRUTINY_CEILING_OVERRIDE.with(|ceiling| ceiling.set(None));
    let found = found.err().expect("the guard refuses");
    assert_eq!(found.len(), 1);
    assert_eq!((found[0].id.as_str(), found[0].code.as_str()), ("diagnostic:physics:dense-scrutiny-resource-guard", "SOLVER_SYSTEM_BLOCKED"));
}

#[test]
fn the_guard_record_states_formulas_not_a_budget() {
    let outcome = modal_solved(with(pf_request(false), json!({"mode_count": 1})));
    let DenseGuardRecord { global_dimension, free_dimension, static_estimate_bytes, eigen_core_bytes, operation_estimate, .. } = outcome.guard;
    assert_eq!((global_dimension, free_dimension), (18, 12));
    assert_eq!(static_estimate_bytes, 96 * 18 * 18);
    assert_eq!(eigen_core_bytes, 32 * 12 * 12);
    assert_eq!(operation_estimate, 9 * 1728 + (7 * 1728u128).div_ceil(3));
}

#[test]
fn an_unseparated_or_ill_conditioned_final_shift_refuses() {
    // A product witness: on EX n05 the bound of 4 stops between the members
    // of a near pair whose error estimates overlap the gap (O-6 observation:
    // relative gap 1.4·10⁻⁸, the upper member's e_λ ≈ 1.4·10⁻⁸). No shift can
    // be placed, so the set is refused rather than counted.
    let found = refusal_codes(with(ex_request_with_density(), json!({"mode_count": 4})), &test_local_parameters());
    assert_eq!(found, vec![MODAL_STURM_SHIFT_UNSEPARATED]);
    let request = || with(sec_a_cantilever(2), json!({"mode_count": 2}));
    let mut unseparated = test_local_parameters();
    unseparated.sturm_separation_multiple = 1e300;
    assert_eq!(refusal_codes(request(), &unseparated), vec![MODAL_STURM_SHIFT_UNSEPARATED]);
    let mut ill = test_local_parameters();
    ill.sturm_conditioning_multiple = 1e300;
    assert_eq!(refusal_codes(request(), &ill), vec![MODAL_STURM_SHIFT_ILL_CONDITIONED]);
}

#[test]
fn every_integrity_parameter_is_required_finite_and_in_domain() {
    let request = || with(pf_request(false), json!({"mode_count": 1}));
    let cases: Vec<(&str, Box<dyn Fn(&mut ModalIntegrityParameters)>)> = vec![
        ("residual_threshold", Box::new(|p| p.residual_threshold = f64::NAN)),
        ("residual_threshold", Box::new(|p| p.residual_threshold = 0.0)),
        ("screen_error_estimate_threshold", Box::new(|p| p.screen_error_estimate_threshold = 1.5)),
        ("cluster_tolerance", Box::new(|p| p.cluster_tolerance = f64::INFINITY)),
        ("orthogonality_threshold", Box::new(|p| p.orthogonality_threshold = -1e-7)),
        ("mass_pivot_relative_threshold", Box::new(|p| p.mass_pivot_relative_threshold = 1.0)),
        ("resolution_floor_constant", Box::new(|p| p.resolution_floor_constant = 0.0)),
        ("sturm_separation_multiple", Box::new(|p| p.sturm_separation_multiple = 0.5)),
        ("sturm_growth_limit", Box::new(|p| p.sturm_growth_limit = f64::NAN)),
        ("sturm_max_candidates", Box::new(|p| p.sturm_max_candidates = 0)),
        ("sturm_gap_log_positions", Box::new(|p| p.sturm_gap_log_positions = vec![0.3])),
        ("sturm_whole_spectrum_offsets", Box::new(|p| p.sturm_whole_spectrum_offsets = Vec::new())),
        ("sturm_whole_spectrum_offsets", Box::new(|p| p.sturm_whole_spectrum_offsets = vec![1.0, f64::NAN])),
        ("identity_tolerance.relative", Box::new(|p| p.identity_tolerance.relative = f64::NAN)),
        ("identity_tolerance.absolute", Box::new(|p| p.identity_tolerance.absolute = -1.0)),
        ("max_ql_iterations_per_eigenvalue", Box::new(|p| p.max_ql_iterations_per_eigenvalue = 0)),
    ];
    for (name, change) in cases {
        let mut params = test_local_parameters();
        change(&mut params);
        let found = modal_run_with(request(), &params).err().unwrap_or_else(|| panic!("{name} must refuse"));
        assert_eq!(found.len(), 1, "{name}: {found:#?}");
        assert_eq!(found[0].code, MODAL_INTEGRITY_PARAMETER_INVALID);
        assert_eq!(found[0].affected_refs, vec![name.to_string()]);
    }
    // A list refusal names the offending entry and its index, never the
    // caller's whole (unbounded) list.
    let mut params = test_local_parameters();
    params.sturm_gap_log_positions = vec![0.25; 1000];
    params.sturm_gap_log_positions[700] = 0.3;
    let found = modal_run_with(request(), &params).err().expect("a non-dyadic position must refuse");
    assert!(found[0].message.contains("3e-1 at index 700"), "{}", found[0].message);
    assert!(found[0].message.len() < 256, "{}", found[0].message);
    let mut params = test_local_parameters();
    params.sturm_whole_spectrum_offsets = vec![1.0, 2.0, f64::NAN];
    let found = modal_run_with(request(), &params).err().expect("a non-finite offset must refuse");
    assert!(found[0].message.contains("NaN at index 2"), "{}", found[0].message);
}

#[test]
fn timoshenko_and_nonlinear_models_keep_i2as_refusals() {
    let params = test_local_parameters();
    // I2b-2: a gap model's state case is never confirmed by the static path,
    // so it is refused at the state case (D2), not by the retired interim code.
    let found = refusal_codes(with(arc_request(true), json!({"mode_count": 1, "state_case": "case:z"})), &params);
    assert_eq!(found.last().unwrap(), MODAL_STATE_CASE_UNAVAILABLE);
    let mut timoshenko = pf_request(false);
    timoshenko["model"]["model_settings"] = json!({"shear_model": "timoshenko_annulus_energy_v1"});
    timoshenko["model"]["materials"][0]["poisson_ratio"] = q(0.25, "1");
    let found = refusal_codes(with(timoshenko, json!({"mode_count": 1})), &params);
    assert_eq!(found.last().unwrap(), MODAL_TIMOSHENKO_MASS_UNSUPPORTED);
    // A sparse solver mode is not a member of the modal request: refused as
    // an undeclared member before anything is formed.
    let refused = super::request::ModalPreviewRequest::parse_modal_request(with(pf_request(false), json!({"mode_count": 1, "solver_mode": "sparse_interactive"})));
    assert!(refused.is_err());
}

// ---------------------------------------------------------------------------
// Standing, undefined fractions and below-normal flags
// ---------------------------------------------------------------------------

#[test]
fn a_residual_or_orthogonality_breach_demotes_to_sensitive_and_still_publishes() {
    let request = || with(sec_a_cantilever(2), json!({"mode_count": 4}));
    let mut tight_residual = test_local_parameters();
    tight_residual.residual_threshold = f64::MIN_POSITIVE;
    let outcome = modal_run_with(request(), &tight_residual).unwrap();
    assert_eq!(outcome.modes.len(), 4);
    assert!(outcome.modes.iter().all(|m| matches!(m.standing, ModalStanding::Sensitive(_)) || m.residual.backward_error == 0.0));
    assert!(matches!(outcome.standing, ModalStanding::Sensitive(_)));
    let mut tight_orthogonality = test_local_parameters();
    tight_orthogonality.orthogonality_threshold = f64::MIN_POSITIVE;
    let outcome = modal_run_with(request(), &tight_orthogonality).unwrap();
    if outcome.integrity.orthogonality.max_ratio > f64::MIN_POSITIVE {
        let ModalStanding::Sensitive(reasons) = &outcome.standing else { panic!("an orthogonality breach demotes the set") };
        assert!(reasons.iter().any(|r| r.contains("M-orthogonality")));
        assert!(outcome.modes.iter().all(|m| m.standing == ModalStanding::ChecksPassed));
    }
}

/// A two-DOF pencil on one node (ux, uy), built directly: the product's
/// normalization and participation glue, where a product model cannot
/// reach the case (a pipe's or body's mass is the same in every translation,
/// so T_k = 0 in one direction means M = 0 in all).
fn two_dof_pencil(total_mass: [f64; 3], excitation: [Vec<f64>; 3]) -> ModalPencil {
    let identity = SymmetricMatrix::from_diagonal(&[1.0, 1.0]).unwrap();
    ModalPencil {
        skew: None,
        global_dimension: 6,
        free_dofs: vec![0, 1],
        dof_map: vec![Some(0), Some(1), None, None, None, None],
        free_labels: vec![FreeDof { node: 0, dof: Dof::Ux }, FreeDof { node: 0, dof: Dof::Uy }],
        labels: vec!["n:UX".into(), "n:UY".into()],
        stiffness: identity.clone(),
        mass: identity,
        stiffness_entries: Vec::new(),
        mass_entries: Vec::new(),
        stiffness_asymmetric_entries: 0,
        excitation,
        total_mass,
        free_dof_mass: [0.0; 3],
        guard: DenseGuardRecord { global_dimension: 6, free_dimension: 2, static_estimate_bytes: 0, ceiling_bytes: 0, eigen_core_bytes: 0, operation_estimate: 0 },
    }
}

#[test]
fn a_fraction_with_no_direction_mass_is_undefined_never_zero() {
    let pencil = two_dof_pencil([1.0, 1.0, 0.0], [vec![1.0, 0.0], vec![0.0, 1.0], vec![0.0, 0.0]]);
    let params = test_local_parameters();
    let basis = modal_basis(&pencil, 1.0, &params).unwrap();
    let modes = vec![normalize_modal_shape(&basis, 0, &[1.0, 0.0]).unwrap(), normalize_modal_shape(&basis, 1, &[0.0, 1.0]).unwrap()];
    assert_eq!(modes[0].effective_mass_fraction, [Some(1.0), Some(0.0), None]);
    assert_eq!(modes[0].effective_mass[2], 0.0);
    let cumulative = cumulative_effective_mass_fractions(&modes);
    assert_eq!(cumulative[1], [Some(1.0), Some(1.0), None]);
}

#[test]
fn a_value_below_the_normal_range_is_published_as_a_flagged_zero() {
    let pencil = two_dof_pencil([1.0, 1.0, 1.0], [vec![1.0, 0.0], vec![0.0, 1.0], vec![0.0, 0.0]]);
    let params = test_local_parameters();
    let basis = modal_basis(&pencil, 1.0, &params).unwrap();
    // φ_uy / φ_ux is nonzero but below 2⁻¹⁰²²: published 0, flagged.
    let mode = normalize_modal_shape(&basis, 0, &[1.0, 1e-310]).unwrap();
    assert_eq!(mode.shape, vec![1.0, 0.0]);
    assert_eq!(mode.below_normal_range.shape, vec![1]);
    // A NaN is refused, never discarded.
    let refused = normalize_modal_shape(&basis, 0, &[1.0, f64::NAN]).unwrap_err();
    assert_eq!(refused[0].code, "MODAL_PENCIL_RANGE_UNSUPPORTED");
}

#[test]
fn every_computed_mode_keeps_its_re_formed_evidence() {
    let outcome = modal_solved(with(sec_a_cantilever(4), json!({"mode_count": 4})));
    // 24 free DOFs, all massive: 24 computed modes, each with its check-2
    // evidence, whether reported or not.
    assert_eq!(outcome.integrity.computed.len(), 24);
    assert_eq!(outcome.integrity.computed.iter().filter(|e| e.reported).count(), 4);
    assert!(outcome.integrity.computed.iter().all(|e| e.residual.backward_error.is_finite()));
    assert_eq!(outcome.clusters.iter().map(|c| c.modes.len()).sum::<usize>(), 24);
    // The reported modes' evidence is the published vectors'.
    for mode in &outcome.modes {
        assert_eq!(outcome.integrity.computed[mode.index - 1].residual, mode.residual);
    }
}


#[test]
fn a_constant_effort_support_adds_no_stiffness_and_no_mass() {
    let plain = modal_solved(with(pf_request(false), json!({"mode_count": 4})));
    let mut request = with(pf_request(false), json!({"mode_count": 4}));
    request["model"]["supports"].as_array_mut().unwrap().push(json!({"id": "support:ce", "node": "b", "family": "constant_effort_support", "restraints": [],
        "hanger": {"hanger_type": "constant_effort_support", "constant_load": q(375.0, "N"), "travel_range": q(0.04, "m"),
            "manufacturer_reference": "invented", "source_reference": "invented", "load_side_review_reference": "review_only_constant_load_not_combined_as_load_effect",
            "mechanics_consumption": "load_side_review_only_no_global_solve_consumption"}, "provenance": PROV}));
    let with_support = modal_solved(request);
    let bits = |o: &ModalOutcome| o.modes.iter().map(|m| m.eigenvalue.to_bits()).collect::<Vec<_>>();
    assert_eq!(bits(&plain), bits(&with_support));
    assert!(with_support.disclosures.iter().any(|d| d.contains("constant-effort supports contribute zero stiffness and zero mass")));
    assert!(!plain.disclosures.iter().any(|d| d.contains("constant-effort")));
}
