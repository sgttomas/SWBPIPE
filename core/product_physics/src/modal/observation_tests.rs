//! DEL-04-09 §5.3.7 evidence for the held and unselected parameters (A2:
//! τ_r, τ_o, δ_c; A3: τ_μ, c_μ, κ_μ, c_f), observed through the product
//! path on the existing correctness population. **TEST-LOCAL observations
//! under the labelled test-local set; nothing is pinned, no parameter is
//! proposed here, and no observation is a bound outside this population.**
//! The κ_c and κ_s observations are kernel-level (`modal_eigen`'s
//! `tests/sturm_counts.rs`), where the formed shifted matrices are available.
//!
//! Every model runs through [`replay_modal_phases`], which
//! `the_phase_replay_is_the_product_path` holds to `run_modal_dense`; the
//! replay exposes the re-formed pencil and every computed pair, reported or
//! not.
//!
//! The tripwires asserted below are regression checks on this population
//! (each a power of ten above the observed extreme, recorded in §5.3.7):
//! they flag a change that moves the observations, and they are not
//! admission thresholds or proposed pins.
use super::integrity::ModalIntegrityParameters;
use super::phase_measurand_tests::{replay_modal_phases, ModalReplay};
use super::tests::{arc_request, ex_request_with_density, modal, number, pf_request, q, reference_request, with, PROV};
use super::tests_eigen::{sec_a_cantilever, test_local_parameters};
use super::tests_state::kd5;
use open_pipe_stress_modal_eigen::dd::{Dd, DdAccumulator};
use open_pipe_stress_modal_eigen::orthogonality::{m_orthogonality, m_orthogonality_reformed};
use serde_json::{json, Value};

const EIGEN_REFERENCES: &str = include_str!("../../../../validation/references/f1_modal/mass_discrete/eigen_references.json");
const U1: &str = include_str!("../../../../validation/references/kd5_nonlinear/u1_request.json");
const F1: &str = include_str!("../../../../validation/references/kd5_nonlinear/f1_request.json");
const U: f64 = f64::EPSILON / 2.0;

/// TEST-LOCAL regression tripwires (module docs; §5.3.7's observation
/// table records the observed extremes they sit above).
const TRIPWIRE_ETA_OVER_NU: f64 = 1e6;
const TRIPWIRE_ORTHOGONALITY_CONSTANT: f64 = 10.0;
const TRIPWIRE_EXACT_PAIR_SPLIT_OVER_ESTIMATE: f64 = 10.0;

/// One model of the population: its name, request and, for a class (a)
/// case, its reference.
struct Model {
    name: String,
    request: Value,
    reference: Option<Value>,
}

/// A frozen class (a) eigen case as a product request, as
/// `tests_eigen::eigen_request` forms it (springs as spring supports).
fn eigen_request(case: &Value, mode_count: usize) -> Value {
    let mut request = reference_request(case, "PF").expect("eigen cases hold straights, arcs and components only");
    for (i, spring) in case["model"]["springs"].as_array().unwrap().iter().enumerate() {
        let dof = spring["dof"].as_str().unwrap().to_uppercase();
        let unit = if dof.starts_with('U') { "N/m" } else { "N*m/rad" };
        request["model"]["supports"].as_array_mut().unwrap().push(json!({"id": format!("spring:{i}"), "node": spring["node"], "family": "spring",
            "restraints": [dof], "stiffness": {"dof": dof, "value": q(number(&spring["stiffness"]), unit)}, "provenance": PROV}));
    }
    request["mode_count"] = json!(mode_count);
    request
}

/// The population: the 26 class (a) cases (a case whose census is
/// unresolved by design asks for the modes below its third reference
/// cluster, as `tests_eigen` serves it), SEC-A at N = 1, 2, 4, 8 (whole
/// spectrum), EX n05, the arc, the component body and the two I2b-2 state
/// cases. The dense path computes every finite mode whatever the bound, so
/// the computed population is the same either way.
fn population() -> Vec<Model> {
    let references: Value = serde_json::from_str(EIGEN_REFERENCES).unwrap();
    let mut models = Vec::new();
    for case in references["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let unresolved = matches!(id, "E-POINTMASS-AXIAL" | "E-POINTMASS-LATERAL" | "E-POINTMASS-GENERAL");
        let count = if unresolved {
            case["clusters"].as_array().unwrap()[..3].iter().map(|c| c["modes"].as_array().unwrap().len()).sum()
        } else {
            case["reported_mode_count"].as_u64().unwrap() as usize
        };
        models.push(Model { name: id.to_string(), request: eigen_request(case, count), reference: Some(case.clone()) });
    }
    for n in [1, 2, 4, 8] {
        models.push(Model { name: format!("SEC-A N={n}"), request: with(sec_a_cantilever(n), json!({"mode_count": 6 * n})), reference: None });
    }
    let product = [
        ("EX n05", with(ex_request_with_density(), json!({"mode_count": 3}))),
        ("arc PF", with(arc_request(false), json!({"mode_count": 6}))),
        ("component PF", with(pf_request(true), json!({"mode_count": 3, "weight_to_mass_acceleration": q(9.8125, "m/s^2")}))),
        ("U-1 PRESSED", with(kd5(U1), json!({"mode_count": 3, "state_case": "load:PRESSED"}))),
        ("F-1 SLIDE-POS", with(kd5(F1), json!({"mode_count": 3, "state_case": "load:SLIDE-POS"}))),
    ];
    for (name, request) in product {
        models.push(Model { name: name.to_string(), request, reference: None });
    }
    models
}

fn replayed(params: &ModalIntegrityParameters) -> Vec<(Model, ModalReplay)> {
    population()
        .into_iter()
        .map(|model| {
            let replay = replay_modal_phases(&modal(model.request.clone()), params).unwrap_or_else(|found| panic!("{}: {found:#?}", model.name));
            (model, replay)
        })
        .collect()
}

// ---------------------------------------------------------------------------
// τ_r: η/(n·u) over every computed pair
// ---------------------------------------------------------------------------

/// Item 1 (τ_r). For every computed pair, reported, unreported and
/// band-recovered alike, η on the re-formed pencil (check 2) over n·u with n
/// the free order. ν(K̃̂) (the condensed equilibrated stiffness's inverse-norm
/// estimate) and the largest κ_λ describe each model's conditioning; ν is an
/// estimate, so it describes and bounds nothing.
#[test]
fn residual_backward_error_over_n_u_on_every_computed_pair() {
    let params = test_local_parameters();
    println!("τ_r observations (TEST-LOCAL, product path; every computed pair)");
    println!("| model | n | computed | reported | max η (reported) | max η (unreported) | max η/(n·u) | at mode | band | max κ_λ | ν(K̃̂) |");
    println!("|---|---|---|---|---|---|---|---|---|---|---|");
    let mut largest = (0.0f64, String::new());
    for (model, replay) in replayed(&params) {
        let n = replay.pencil.free_dofs.len();
        let reported = replay.reported();
        let (mut eta_r, mut eta_u, mut worst, mut at, mut kappa) = (0.0f64, 0.0f64, 0.0f64, 0, 0.0f64);
        for (i, r) in replay.residuals.iter().enumerate() {
            assert!(r.backward_error.is_finite() && r.condition.is_finite(), "{} mode {i}", model.name);
            if i < reported {
                eta_r = eta_r.max(r.backward_error);
            } else {
                eta_u = eta_u.max(r.backward_error);
            }
            let ratio = r.backward_error / (n as f64 * U);
            if ratio > worst {
                (worst, at) = (ratio, i);
            }
            kappa = kappa.max(r.condition);
        }
        println!(
            "| {} | {n} | {} | {reported} | {eta_r:.2e} | {eta_u:.2e} | {worst:.2e} | {} | {} | {kappa:.2e} | {:.2e} |",
            model.name,
            replay.residuals.len(),
            at + 1,
            replay.band_recovered(at),
            replay.solution.condensed_inverse_norm_estimate
        );
        if worst > largest.0 {
            largest = (worst, model.name.clone());
        }
    }
    println!("largest η/(n·u): {:.3e} ({})", largest.0, largest.1);
    assert!(largest.0 <= TRIPWIRE_ETA_OVER_NU, "η/(n·u) = {:.3e} on {} moved above the tripwire", largest.0, largest.1);
}

// ---------------------------------------------------------------------------
// τ_o: measured orthogonality, formed and re-formed, every computed pair
// ---------------------------------------------------------------------------

/// |φᵢᵀMφⱼ|/√(m_i m_j) for every pair, on the formed M_ff, with exact
/// products accumulated in double-double. Returns (i, j, deviation).
fn pairwise_formed_deviation(replay: &ModalReplay) -> Vec<(usize, usize, f64)> {
    let mass = &replay.pencil.mass;
    let n = mass.dimension();
    let total = replay.solution.eigenvalues.len();
    let products: Vec<Vec<Dd>> = (0..total)
        .map(|j| {
            let v = replay.vector(j);
            (0..n)
                .map(|row| {
                    let mut acc = DdAccumulator::new();
                    for (col, &x) in v.iter().enumerate() {
                        acc.add_product(mass.get(row, col), x);
                    }
                    acc.value()
                })
                .collect()
        })
        .collect();
    let dot = |i: usize, j: usize| {
        let v = replay.vector(i);
        let mut acc = DdAccumulator::new();
        for (row, &x) in v.iter().enumerate() {
            acc.add_dd(products[j][row].mul_f64(x));
        }
        acc.value()
    };
    let masses: Vec<Dd> = (0..total).map(|i| dot(i, i)).collect();
    let mut out = Vec::new();
    for i in 0..total {
        for j in i + 1..total {
            let deviation = dot(i, j).abs().div(masses[i].mul(masses[j]).sqrt()).to_f64();
            out.push((i, j, deviation));
        }
    }
    out
}

/// Item 4 (τ_o). The measured M-orthogonality deviation of every computed
/// pair on the formed M_ff and on the re-formed M̂ (check 3's measure,
/// extended to the unreported and band-recovered vectors), and the constant
/// c′ = deviation / (n·u·√(λᵢλⱼ)/λ_min) of the reduced-problem form §5.3.7
/// derives, per pair on the formed M. The formation and back-transformation
/// factors (‖R̂⁻¹‖₂²‖M̃‖₂/μ_max) are not computed here.
#[test]
fn measured_orthogonality_formed_and_reformed_over_every_computed_pair() {
    let params = test_local_parameters();
    println!("τ_o observations (TEST-LOCAL, product path)");
    println!("| model | n | computed | published (M̂) | all computed (M̂) | all computed (M) | max c′ | c′ pair | band pairs: max deviation |");
    println!("|---|---|---|---|---|---|---|---|---|");
    let mut largest_constant = (0.0f64, String::new());
    for (model, replay) in replayed(&params) {
        let n = replay.pencil.free_dofs.len();
        let total = replay.solution.eigenvalues.len();
        let vectors: Vec<&[f64]> = (0..total).map(|i| replay.vector(i)).collect();
        let reformed = m_orthogonality_reformed(&replay.reformed, &vectors).expect("finite");
        let formed = m_orthogonality(&replay.pencil.mass, &vectors).expect("finite");
        let lambda = &replay.solution.eigenvalues;
        let lambda_min = lambda[0];
        let mut constant = (0.0f64, (0, 0));
        let mut band = None::<f64>;
        for (i, j, deviation) in pairwise_formed_deviation(&replay) {
            let form = n as f64 * U * (lambda[i].sqrt() * lambda[j].sqrt()) / lambda_min;
            let c = deviation / form;
            if c > constant.0 {
                constant = (c, (i + 1, j + 1));
            }
            if replay.band_recovered(i) || replay.band_recovered(j) {
                band = Some(band.map_or(deviation, |b: f64| b.max(deviation)));
            }
        }
        println!(
            "| {} | {n} | {total} | {:.2e} | {:.2e} | {:.2e} | {:.2e} | {:?} | {} |",
            model.name,
            replay.orthogonality.max_ratio,
            reformed.max_ratio,
            formed.max_ratio,
            constant.0,
            constant.1,
            band.map_or("-".to_string(), |b| format!("{b:.2e}"))
        );
        if constant.0 > largest_constant.0 {
            largest_constant = (constant.0, model.name.clone());
        }
    }
    println!("largest c′: {:.3e} ({})", largest_constant.0, largest_constant.1);
    assert!(largest_constant.0 <= TRIPWIRE_ORTHOGONALITY_CONSTANT, "c′ = {:.3e} on {}", largest_constant.0, largest_constant.1);
}

// ---------------------------------------------------------------------------
// δ_c: exact-repeat splits and close-distinct gaps against the references
// ---------------------------------------------------------------------------

fn reference_eigenvalue(value: &Value) -> f64 {
    value.as_str().unwrap().parse().unwrap()
}

/// Item 5 (δ_c), product side. The class (a) references state each exact
/// multiplicity (`clusters[].multiplicity`, from the references' 80-digit
/// generator), independently of the solve. For each, the computed relative
/// split (λ_last − λ_first)/λ_last of its members against the members'
/// largest e_λ; and for each pair of adjacent distinct reference clusters,
/// the reference relative gap and the computed one (the close-distinct
/// controls). The computed modes are matched to the reference by index
/// (the class (a) test establishes that agreement).
#[test]
fn exact_repeat_splits_and_close_distinct_gaps_against_the_references() {
    let params = test_local_parameters();
    let mut splits = Vec::new();
    let mut gaps = Vec::new();
    for (model, replay) in replayed(&params) {
        let Some(case) = &model.reference else { continue };
        let lambda = &replay.solution.eigenvalues;
        let clusters = case["clusters"].as_array().unwrap();
        for cluster in clusters {
            let members: Vec<usize> = cluster["modes"].as_array().unwrap().iter().map(|m| m.as_u64().unwrap() as usize - 1).collect();
            if members.len() < 2 || *members.last().unwrap() >= lambda.len() {
                continue;
            }
            let (first, last) = (members[0], *members.last().unwrap());
            let split = (lambda[last] - lambda[first]) / lambda[last];
            let estimate = members.iter().map(|&m| replay.residuals[m].error_estimate).fold(0.0, f64::max);
            splits.push((model.name.clone(), members.len(), lambda[last], split, estimate));
        }
        for pair in clusters.windows(2) {
            let below = pair[0]["modes"].as_array().unwrap().last().unwrap().as_u64().unwrap() as usize - 1;
            let above = pair[1]["modes"].as_array().unwrap()[0].as_u64().unwrap() as usize - 1;
            if above >= lambda.len() {
                continue;
            }
            let (lo, hi) = (reference_eigenvalue(&pair[0]["eigenvalue_rad2_per_s2"]), reference_eigenvalue(&pair[1]["eigenvalue_rad2_per_s2"]));
            let reference_gap = (hi - lo) / hi;
            let computed_gap = (lambda[above] - lambda[below]) / lambda[above];
            gaps.push((model.name.clone(), above, reference_gap, computed_gap));
        }
    }
    splits.sort_by(|a, b| b.3.total_cmp(&a.3));
    gaps.sort_by(|a, b| a.2.total_cmp(&b.2));
    println!("δ_c observations (TEST-LOCAL): exact reference multiplicities, largest computed splits first");
    println!("| model | multiplicity | λ | computed relative split | members' max e_λ | split / e_λ |");
    println!("|---|---|---|---|---|---|");
    for (name, m, l, split, e) in splits.iter().take(12) {
        println!("| {name} | {m} | {l:.4e} | {split:.2e} | {e:.2e} | {:.2e} |", split / e);
    }
    println!("{} exact multiplicities in all; the largest split is {:.3e}", splits.len(), splits[0].3);
    println!("close-distinct controls: adjacent distinct reference clusters, smallest reference gap first");
    println!("| model | upper mode | reference relative gap | computed relative gap | relative error of the gap |");
    println!("|---|---|---|---|---|");
    for (name, mode, reference, computed) in gaps.iter().take(8) {
        println!("| {name} | {} | {reference:.4e} | {computed:.4e} | {:.2e} |", mode + 1, (computed - reference).abs() / reference);
    }
    // Every computed split lies within the members' own first-order
    // estimate on this population (a regression tripwire, not a theorem).
    let worst = splits.iter().map(|s| s.3 / s.4).fold(0.0, f64::max);
    println!("largest split / e_λ: {worst:.3e}");
    assert!(worst <= TRIPWIRE_EXACT_PAIR_SPLIT_OVER_ESTIMATE, "an exact multiplicity split by {worst:.3e} of its own e_λ");
    // Every close-distinct gap is computed as distinct, and to a small
    // fraction of itself.
    for (name, mode, reference, computed) in &gaps {
        assert!((computed - reference).abs() <= 1e-3 * reference, "{name} mode {}: gap {computed:e} against {reference:e}", mode + 1);
    }
}

// ---------------------------------------------------------------------------
// A3: the μ-domain observations (τ_μ, c_μ, κ_μ, c_f)
// ---------------------------------------------------------------------------

/// A3, for the μ-domain parameters (unselected). Per model: μ_max, the
/// smallest retained μ/μ_max, the hard threshold max(τ_μ, κ_μ·c_μ·n·u) and
/// the broad band edge over μ_max, the screened, band-retained and rejected
/// counts, the census, and the band pairs' formed backward error η̃ over
/// n·u (against c_f) and e_λ (against τ_e). The census limit (GENERAL's
/// non-coordinate nullspace) and the high-mode band (E-RIGID-SCALE-1e6) are
/// kept as found; nothing is truncated or recovered here.
#[test]
fn mu_domain_observation_table() {
    let params = test_local_parameters();
    println!("A3 μ-domain observations (TEST-LOCAL, product path; τ_μ = {:e}, c_μ = {}, κ_μ = {}, c_f = {})",
        params.infinite_mu_relative_threshold, params.resolution_floor_constant, params.resolution_floor_multiple, params.formed_backward_error_multiple);
    println!("| model | n | n_m | μ_max | min retained μ/μ_max | hard/μ_max | broad/μ_max | screened | band kept | rejected (≤0, floor/band) | census | band max η̃/(n·u) | band max e_λ |");
    println!("|---|---|---|---|---|---|---|---|---|---|---|---|---|");
    let mut smallest = (f64::INFINITY, String::new());
    for (model, replay) in replayed(&params) {
        let s = &replay.solution;
        let n = s.dimension;
        let min_ratio = s.mu.iter().copied().fold(f64::INFINITY, f64::min) / s.mu_max;
        let kept: Vec<_> = s.screened.iter().filter(|m| m.retained).collect();
        let band_eta = kept.iter().filter_map(|m| m.recovery.as_ref()).map(|r| r.backward_error / (n as f64 * U)).fold(None, |a: Option<f64>, x| Some(a.map_or(x, |a| a.max(x))));
        let band_e = kept.iter().filter_map(|m| m.recovery.as_ref()).map(|r| r.error_estimate).fold(None, |a: Option<f64>, x| Some(a.map_or(x, |a| a.max(x))));
        let census = match &replay.census {
            super::eigen::FiniteCensus::Established { finite_mode_count, .. } => format!("established {finite_mode_count}"),
            super::eigen::FiniteCensus::Unresolved { .. } => "unresolved".to_string(),
        };
        println!(
            "| {} | {n} | {} | {:.3e} | {min_ratio:.2e} | {:.2e} | {:.2e} | {} | {} | {}, {} | {census} | {} | {} |",
            model.name,
            s.condensed_dimension,
            s.mu_max,
            s.hard_classification_threshold / s.mu_max,
            s.classification_threshold / s.mu_max,
            s.screened.len(),
            kept.len(),
            s.nonpositive_mu_count,
            s.below_threshold_mu_count,
            band_eta.map_or("-".to_string(), |x| format!("{x:.2e}")),
            band_e.map_or("-".to_string(), |x| format!("{x:.2e}")),
        );
        if min_ratio < smallest.0 {
            smallest = (min_ratio, model.name.clone());
        }
        // Every retained μ lies above the hard threshold, by construction.
        assert!(s.mu.iter().all(|&mu| mu > s.hard_classification_threshold), "{}", model.name);
    }
    println!("smallest retained μ/μ_max: {:.3e} ({})", smallest.0, smallest.1);
}

// ---------------------------------------------------------------------------
// The identity tolerance (A1): injected mass errors, measured
// ---------------------------------------------------------------------------

/// The identity's largest relative discrepancy per subject for a formed
/// mass that may have been corrupted, judged at a permissive TEST-LOCAL
/// relative 0.999 so the values are measured rather than refused; a row
/// that still fails is reported as refused at 0.999 with its message.
fn identity_rows(prepared: &super::prepare::Prepared, mass: &super::mass::ModalMass) -> Vec<String> {
    let permissive = super::weight_basis::IdentityTolerance { relative: 0.999, absolute: 0.0 };
    match super::weight_basis::check_weight_basis(prepared, mass, permissive) {
        Ok(report) => {
            let mut subjects: Vec<String> = report.discrepancies.iter().map(|d| format!("{} {}", d.subject_kind, d.subject)).collect();
            subjects.sort();
            subjects.dedup();
            subjects
                .into_iter()
                .map(|s| {
                    let (rel, kind) = report
                        .discrepancies
                        .iter()
                        .filter(|d| format!("{} {}", d.subject_kind, d.subject) == s)
                        .map(|d| (d.relative, d.kind))
                        .fold((0.0f64, ""), |acc, x| if x.0 > acc.0 { x } else { acc });
                    format!("{s}: {rel:.3e} ({kind})")
                })
                .collect()
        }
        Err(found) => {
            let mut messages: Vec<String> = found.iter().map(|d| format!("refused even at relative 0.999: {}", d.message)).collect();
            messages.dedup();
            messages
        }
    }
}

/// A1 (identity tolerance): what injected mass errors actually measure as,
/// relative, on the test models. Each corruption changes the formed mass
/// only; the self-weight basis it is compared with is the authored model's.
/// These are measurements on these models, not a detection guarantee.
#[test]
fn identity_discrepancies_of_injected_mass_errors() {
    use super::mass::assemble_mass;
    use super::prepare::{prepare_modal, tests_support::with_model};
    println!("Identity tolerance: injected mass errors (TEST-LOCAL; the largest relative discrepancy per compared subject)");
    // Straights: the density scaled in the formed mass (PF pipe-only, two straights).
    for factor in [1e-9, 1e-2] {
        let request = modal(with(pf_request(false), json!({"mode_count": 1})));
        let prepared = prepare_modal(&request).unwrap();
        let mut corrupted = prepared.model.clone();
        corrupted.pipe_segments[0].section.material_density.as_mut().unwrap().value *= 1.0 + factor;
        let scaled = with_model(prepared, corrupted);
        let mass = assemble_mass(&scaled, None).unwrap();
        println!("- PF pipe-only, pipe:p1 material density x(1 + {factor:e}): {:?}", identity_rows(&scaled, &mass));
    }
    // Component bodies: the valve (W = 500 N, and a light variant W = 0.5 N),
    // each error injected into the lowered model so that the formed mass is
    // assembled consistently from it (the body record and the assembled
    // blocks alike), while the inspection reads the authored model.
    type Corruption = fn(&mut crate::PreviewComponent);
    let corruptions: [(&str, Corruption); 4] = [
        ("intact", |_| {}),
        ("body weight x1.01 in the mass", |c| c.geometry.as_mut().unwrap().weight.as_mut().unwrap().value *= 1.01),
        ("centre of gravity (−0.5, 0.25, 0) → (−0.5, 0, 0) in the mass", |c| c.geometry.as_mut().unwrap().center_of_gravity.as_mut().unwrap().y = 0.0),
        ("body on node b instead of c in the mass", |c| c.node = "b".to_string()),
    ];
    for weight in [500.0, 0.5] {
        let mut authored = pf_request(true);
        authored["model"]["components"][0]["geometry"]["weight"] = q(weight, "N");
        let request = modal(with(authored, json!({"mode_count": 1, "weight_to_mass_acceleration": q(9.8125, "m/s^2")})));
        for (what, corrupt) in corruptions {
            let prepared = prepare_modal(&request).unwrap();
            let mut corrupted = prepared.model.clone();
            corrupt(&mut corrupted.components[0]);
            let scaled = with_model(prepared, corrupted);
            match assemble_mass(&scaled, Some(9.8125)) {
                Ok(mass) => println!("- component PF, W = {weight} N, {what}: {:?}", identity_rows(&scaled, &mass)),
                Err(found) => println!("- component PF, W = {weight} N, {what}: mass assembly refused: {:?}", found.iter().map(|d| &d.code).collect::<Vec<_>>()),
            }
        }
        // A body dropped from the mass only (its record removed): refused by
        // the body census before any tolerance applies.
        let prepared = prepare_modal(&request).unwrap();
        let mut dropped = assemble_mass(&prepared, Some(9.8125)).unwrap();
        dropped.bodies.clear();
        println!("- component PF, W = {weight} N, body record dropped from the mass: {:?}", identity_rows(&prepared, &dropped));
    }
}
