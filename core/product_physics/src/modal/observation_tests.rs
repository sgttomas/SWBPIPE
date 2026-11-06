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
//! (each at the next power of ten or more above the observed extreme,
//! recorded in §5.3.7):
//! they flag a change that moves the observations, and they are not
//! admission thresholds or proposed pins.
//!
//! **The selected instance** (the last section). These tests are the regression
//! evidence for `ModalIntegrityParameters::selected_a1_a2_a3_criteria` (§5.3.7,
//! "A3, as decided"): the decided values bit for bit, validation, no `Default`,
//! and a source scan showing that no non-test source calls the constructor.
//! The window true positive is a directly built pencil with an exact spectrum,
//! run through the product's solve, census and endpoint functions. The true
//! zeros, the unresolved endpoint and the lower-path premises run on the
//! product path. The n = 1,920 floor is checked by arithmetic only. The
//! population is compared under both sets. The tests above stay on the
//! test-local set.
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
    println!("| model | n | n_m | μ_max | min retained μ/μ_max | hard/μ_max | broad/μ_max | screened | band kept | rejected (≤0, floor/band) | largest rejected μ/μ_max | census | band max η̃/(n·u) | band max e_λ |");
    println!("|---|---|---|---|---|---|---|---|---|---|---|---|---|---|");
    let mut smallest = (f64::INFINITY, String::new());
    for (model, replay) in replayed(&params) {
        let s = &replay.solution;
        let n = s.dimension;
        let min_ratio = s.mu.iter().copied().fold(f64::INFINITY, f64::min) / s.mu_max;
        let kept: Vec<_> = s.screened.iter().filter(|m| m.retained).collect();
        let band_eta = kept.iter().filter_map(|m| m.recovery.as_ref()).map(|r| r.backward_error / (n as f64 * U)).fold(None, |a: Option<f64>, x| Some(a.map_or(x, |a| a.max(x))));
        let band_e = kept.iter().filter_map(|m| m.recovery.as_ref()).map(|r| r.error_estimate).fold(None, |a: Option<f64>, x| Some(a.map_or(x, |a| a.max(x))));
        let rejected = s.screened.iter().filter(|m| !m.retained).map(|m| m.mu / s.mu_max).fold(None, |a: Option<f64>, x| Some(a.map_or(x, |a| a.max(x))));
        let census = match &replay.census {
            super::eigen::FiniteCensus::Established { finite_mode_count, .. } => format!("established {finite_mode_count}"),
            super::eigen::FiniteCensus::Unresolved { .. } => "unresolved".to_string(),
        };
        println!(
            "| {} | {n} | {} | {:.3e} | {min_ratio:.2e} | {:.2e} | {:.2e} | {} | {} | {}, {} | {} | {census} | {} | {} |",
            model.name,
            s.condensed_dimension,
            s.mu_max,
            s.hard_classification_threshold / s.mu_max,
            s.classification_threshold / s.mu_max,
            s.screened.len(),
            kept.len(),
            s.nonpositive_mu_count,
            s.below_threshold_mu_count,
            rejected.map_or("-".to_string(), |x| format!("{x:.2e}")),
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

// ---------------------------------------------------------------------------
// The selected A1 + A2 + A3 instance (Agent 0, 2026-11-05): regression
// evidence. The instance is called from tests only; the existing tests above
// and elsewhere stay on the test-local set.
// ---------------------------------------------------------------------------

use super::eigen::{complete_reported_set, modal_dense_options, solve_modal_pencil, FiniteCensus, ReportedEndpoint};
use super::integrity::{MODAL_FINITE_COUNT_UNRESOLVED, MODAL_STURM_SHIFT_UNSEPARATED};
use super::outcome::{run_modal_dense, ModalOutcome, ModalStanding};
use super::stiffness::{DenseGuardRecord, ModalPencil};
use super::tests::codes;
use open_pipe_stress_modal_eigen::clusters::{cluster_ids, ExtractionBound};
use open_pipe_stress_modal_eigen::dd::max_propagating_nan;
use open_pipe_stress_modal_eigen::dense::DenseEigenSolution;
use open_pipe_stress_modal_eigen::modal_quantities::{frequency_from_eigenvalue, Dof, FreeDof};
use open_pipe_stress_modal_eigen::SymmetricMatrix;

fn instance() -> ModalIntegrityParameters {
    ModalIntegrityParameters::selected_a1_a2_a3_criteria()
}

/// 2^e for a normal-range exponent, built from its bit pattern (an exact
/// construction independent of the source's decimal literals).
fn pow2(e: i32) -> f64 {
    assert!((-1022..=1023).contains(&e));
    f64::from_bits(((1023 + e) as u64) << 52)
}

/// Item 5 (validation) and the decided values, bit for bit.
#[test]
fn the_selected_instance_carries_the_decided_values_and_passes_validation() {
    let p = instance();
    let bits = |x: f64| x.to_bits();
    // A1.
    assert_eq!(bits(p.no_translation_threshold), bits(1e-6));
    assert_eq!(bits(p.identity_tolerance.relative), bits(pow2(-44)));
    assert_eq!(bits(p.identity_tolerance.absolute), bits(0.0));
    assert_eq!(bits(p.sturm_growth_limit), bits(pow2(52)));
    assert_eq!(bits(p.screen_error_estimate_threshold), bits(1e-6));
    assert_eq!(bits(p.mass_pivot_relative_threshold), bits(pow2(-36)));
    assert_eq!(p.sturm_max_candidates, 7);
    assert_eq!(p.sturm_gap_log_positions.iter().map(|&x| bits(x)).collect::<Vec<_>>(), [0.25, 0.75, 0.125, 0.375, 0.625, 0.875].map(bits));
    assert_eq!(p.sturm_whole_spectrum_offsets.iter().map(|&x| bits(x)).collect::<Vec<_>>(), [1.0, 0.5, 0.25].map(bits));
    assert_eq!(p.max_ql_iterations_per_eigenvalue, 30);
    // A2.
    assert_eq!(bits(p.sturm_conditioning_multiple), bits(pow2(10)));
    assert_eq!(bits(p.sturm_separation_multiple), bits(pow2(7)));
    assert_eq!(bits(p.orthogonality_threshold), bits(pow2(-23)));
    assert_eq!(bits(p.cluster_tolerance), bits(pow2(-20)));
    assert_eq!(bits(p.residual_threshold), bits(pow2(-30)));
    // A3.
    assert_eq!(bits(p.infinite_mu_relative_threshold), bits(pow2(-53)));
    assert_eq!(bits(p.infinite_mu_relative_threshold), bits(U));
    assert_eq!(bits(p.resolution_floor_constant), bits(1.0));
    assert_eq!(bits(p.resolution_floor_multiple), bits(10.0));
    assert_eq!(bits(p.formed_backward_error_multiple), bits(pow2(6)));
    // Every field named above: a field added later must be decided here.
    let ModalIntegrityParameters {
        infinite_mu_relative_threshold: _,
        mass_pivot_relative_threshold: _,
        resolution_floor_constant: _,
        resolution_floor_multiple: _,
        formed_backward_error_multiple: _,
        screen_error_estimate_threshold: _,
        max_ql_iterations_per_eigenvalue: _,
        residual_threshold: _,
        orthogonality_threshold: _,
        cluster_tolerance: _,
        no_translation_threshold: _,
        sturm_conditioning_multiple: _,
        sturm_separation_multiple: _,
        sturm_growth_limit: _,
        sturm_max_candidates: _,
        sturm_gap_log_positions: _,
        sturm_whole_spectrum_offsets: _,
        identity_tolerance: _,
    } = p.clone();
    // The existing validator admits it, and it differs from the test-local set.
    assert!(p.modal_parameter_refusals().is_empty(), "{:#?}", p.modal_parameter_refusals());
    assert_ne!(p, test_local_parameters());
    // Not `Default`, checked at compile time: the inherent constant exists
    // only when the bound holds, otherwise the blanket trait's applies.
    trait NoDefault {
        const IMPLEMENTS_DEFAULT: bool = false;
    }
    impl<T> NoDefault for T {}
    struct Probe<T>(std::marker::PhantomData<T>);
    #[allow(dead_code)]
    impl<T: Default> Probe<T> {
        const IMPLEMENTS_DEFAULT: bool = true;
    }
    const { assert!(!<Probe<ModalIntegrityParameters>>::IMPLEMENTS_DEFAULT) };
    const { assert!(<Probe<f64>>::IMPLEMENTS_DEFAULT, "the probe detects a Default type") };
}

/// Item 5 (containment). `integrity` is a private module of the private
/// `modal` module (`mod modal;` in lib.rs, `mod integrity;` in modal.rs), so
/// the `pub(crate)` constructor is nameable only inside `modal`. Every
/// non-test source of `modal` is read at compile time here: the name occurs
/// once outside comments, at its definition, and nowhere else.
#[test]
fn no_non_test_modal_source_calls_the_selected_instance() {
    const NAME: &str = "selected_a1_a2_a3_criteria";
    let sources: [(&str, &str); 13] = [
        ("modal.rs", include_str!("../modal.rs")),
        ("context.rs", include_str!("context.rs")),
        ("document.rs", include_str!("document.rs")),
        ("eigen.rs", include_str!("eigen.rs")),
        ("integrity.rs", include_str!("integrity.rs")),
        ("mass.rs", include_str!("mass.rs")),
        ("outcome.rs", include_str!("outcome.rs")),
        ("prepare.rs", include_str!("prepare.rs")),
        ("request.rs", include_str!("request.rs")),
        ("state.rs", include_str!("state.rs")),
        ("stiffness.rs", include_str!("stiffness.rs")),
        ("skew_pencil.rs", include_str!("skew_pencil.rs")),
        ("weight_basis.rs", include_str!("weight_basis.rs")),
    ];
    let lib = include_str!("../lib.rs");
    assert!(lib.lines().any(|l| l.trim() == "mod modal;"), "modal stays a private module");
    assert!(!lib.contains("modal::integrity"), "lib.rs re-exports nothing from modal::integrity");
    assert!(sources[0].1.lines().any(|l| l.trim() == "mod integrity;"), "integrity stays a private module");
    for (name, text) in sources {
        let code: Vec<&str> = text.lines().map(|l| l.split("//").next().unwrap()).collect();
        let uses = code.iter().filter(|l| l.contains(NAME)).count();
        if name == "integrity.rs" {
            assert_eq!(uses, 1, "integrity.rs holds only the definition");
            assert!(code.iter().any(|l| l.contains(&format!("pub(crate) fn {NAME}() -> Self"))));
        } else {
            assert_eq!(uses, 0, "{name} names the selected instance");
        }
    }
    // The module tree is complete for the current sources. Every `mod NAME`
    // item anywhere in a listed source (after braced items, after inner
    // attributes, one or several per line, any visibility) is found by
    // keyword, with comments removed; it is test-only when one of its own
    // outer attributes is `#[cfg(test)]` (an inner `#![…]` is not the item's).
    // modal.rs's non-test file modules must be exactly the listed sources, and
    // no listed source may declare another non-test module, so a future
    // module fails here and must be added to the scan.
    let mut non_test: Vec<String> = Vec::new();
    for (index, (name, text)) in sources.iter().enumerate() {
        for (test_only, module, inline) in module_items(text) {
            if test_only {
                continue;
            }
            assert!(!inline, "{name} declares a non-test inline module `{module}`, which this scan does not cover");
            assert!(index == 0, "{name} declares a nested non-test module `{module}`, which this scan does not read");
            non_test.push(module);
        }
    }
    let mut listed: Vec<String> = sources[1..].iter().map(|(n, _)| n.trim_end_matches(".rs").to_string()).collect();
    listed.sort();
    non_test.sort();
    assert_eq!(non_test, listed, "modal.rs's non-test modules are exactly the scanned sources");
    // The scan itself, on whole sources: the escapes the review found (a
    // module after a braced item, a module after an inner attribute), S-K1's
    // one-line pair, an attributed test module on the same line, visibility,
    // and an inline test module.
    let found = |text: &str| module_items(text).into_iter().map(|(t, m, i)| (t, m, i)).collect::<Vec<_>>();
    assert_eq!(found("fn f() { let x = 1; }\nmod escaped;"), vec![(false, "escaped".to_string(), false)]);
    assert_eq!(found("#![allow(dead_code)] // note\n\nmod context;"), vec![(false, "context".to_string(), false)]);
    assert_eq!(found("mod stiffness; mod skew_pencil;"), vec![(false, "stiffness".to_string(), false), (false, "skew_pencil".to_string(), false)]);
    assert_eq!(found("mod a; #[cfg(test)] mod tests_skew;"), vec![(false, "a".to_string(), false), (true, "tests_skew".to_string(), false)]);
    assert_eq!(found("pub(crate) mod x;\npub mod y;"), vec![(false, "x".to_string(), false), (false, "y".to_string(), false)]);
    assert_eq!(found("#[cfg(test)]\n#[allow(unused)]\nmod t { fn g() {} }"), vec![(true, "t".to_string(), true)]);
    assert_eq!(found("/* mod hidden; */ let modal = 1; // mod gone;"), vec![]);
}

/// Every `mod NAME` item of a Rust source, in order: (test-only, NAME,
/// inline). Comments are removed first; `mod` is found as a keyword (not
/// inside an identifier such as `modal`); the item's own outer attributes are
/// read backwards over whitespace and an optional visibility.
fn module_items(text: &str) -> Vec<(bool, String, bool)> {
    let bytes = text.as_bytes();
    let mut code = String::with_capacity(text.len());
    let mut i = 0;
    while i < bytes.len() {
        if text[i..].starts_with("//") {
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
        } else if text[i..].starts_with("/*") {
            let end = text[i + 2..].find("*/").map_or(bytes.len(), |e| i + 2 + e + 2);
            code.push(' ');
            i = end;
        } else {
            let c = text[i..].chars().next().unwrap();
            code.push(c);
            i += c.len_utf8();
        }
    }
    let ident = |c: char| c.is_alphanumeric() || c == '_';
    let mut out = Vec::new();
    let mut from = 0;
    while let Some(at) = code[from..].find("mod").map(|p| from + p) {
        from = at + 3;
        let before_ok = code[..at].chars().next_back().is_none_or(|c| !ident(c));
        let after = &code[at + 3..];
        if !before_ok || !after.starts_with(char::is_whitespace) {
            continue;
        }
        let rest = after.trim_start();
        let name: String = rest.chars().take_while(|&c| ident(c)).collect();
        if name.is_empty() {
            continue;
        }
        let tail = rest[name.len()..].trim_start();
        let inline = match tail.chars().next() {
            Some(';') => false,
            Some('{') => true,
            _ => continue,
        };
        // Backwards: optional visibility, then the item's outer attributes.
        let mut head = code[..at].trim_end();
        if let Some(stripped) = head.strip_suffix("pub") {
            head = stripped.trim_end();
        } else if head.ends_with(')') {
            if let Some(open) = head.rfind("pub(") {
                head = head[..open].trim_end();
            }
        }
        let mut test_only = false;
        while head.ends_with(']') {
            let Some(open) = head.rfind('[') else { break };
            let attribute = &head[open + 1..head.len() - 1];
            let prefix = head[..open].trim_end();
            if prefix.ends_with("#!") || !prefix.ends_with('#') {
                break;
            }
            test_only |= attribute.trim() == "cfg(test)";
            head = prefix[..prefix.len() - 1].trim_end();
        }
        out.push((test_only, name, inline));
    }
    out
}

/// The kernel's hard cut h = max(τ_μ·μ_max, κ_μ·δ_q), δ_q = c_μ·n·u·μ_max,
/// in its own binary64 left-to-right order (`dense.rs`), from the options the
/// product maps from the parameters.
fn kernel_hard_cut(params: &ModalIntegrityParameters, n: usize, mu_max: f64) -> (f64, f64) {
    let options = modal_dense_options(params);
    let ql_resolution_floor = options.resolution_floor_constant * n as f64 * (f64::EPSILON / 2.0) * mu_max;
    let tau_term = options.infinite_mu_relative_threshold * mu_max;
    (tau_term, max_propagating_nan(tau_term, options.resolution_floor_multiple * ql_resolution_floor))
}

/// Representative and edge μ_max > 0: every binade from the smallest
/// subnormal to the largest finite value at several mantissas, the first
/// 4,096 subnormal multiples, and the binade edges.
fn mu_max_sweep(mantissas: &[f64]) -> Vec<f64> {
    let mut values = Vec::new();
    for e in -1074..=1023 {
        for &m in mantissas {
            // 2^e as 2^max(e, −1022)·2^min(e + 1022, 0): powi alone overflows
            // its intermediate below −1022.
            let v = m * 2f64.powi(e.max(-1022)) * 2f64.powi((e + 1022).min(0));
            if v > 0.0 && v.is_finite() {
                values.push(v);
            }
        }
    }
    values.extend((1..=4096u64).map(f64::from_bits));
    values.extend([f64::MIN_POSITIVE, f64::from_bits(f64::MIN_POSITIVE.to_bits() - 1), f64::from_bits(f64::MIN_POSITIVE.to_bits() + 1), f64::MAX, 1.0]);
    values
}

/// Item 3. The derived floor at n = 1,920 is the same under the instance and
/// the test-local set, and under the instance τ_μ·μ_max never exceeds κ_μ·δ_q
/// for n ≥ 1 at μ_max > 0, in the kernel's arithmetic. Arithmetic only: no
/// 1,920-order pencil is formed or solved. The formula is first held to the
/// kernel's own recorded h on solved models.
#[test]
fn the_derived_hard_floor_is_unchanged_and_tau_mu_is_never_the_active_floor() {
    let local = test_local_parameters();
    let selected = instance();
    // The replicated evaluation is the kernel's, bit for bit, under both sets.
    for request in [with(sec_a_cantilever(1), json!({"mode_count": 6})), with(ex_request_with_density(), json!({"mode_count": 3}))] {
        for params in [&local, &selected] {
            let replay = replay_modal_phases(&modal(request.clone()), params).unwrap();
            let s = &replay.solution;
            assert_eq!(kernel_hard_cut(params, s.dimension, s.mu_max).1.to_bits(), s.hard_classification_threshold.to_bits());
        }
    }
    // n = 1,920: identical h. It holds over the whole normal range and into the
    // subnormals; the only differences are recorded below.
    let sweep = mu_max_sweep(&[1.0, 1.1, 1.25, 1.5, 1.75, 1.9, 1.999]);
    let mut differing = Vec::new();
    for &mu in &sweep {
        let (a, b) = (kernel_hard_cut(&local, 1920, mu).1, kernel_hard_cut(&selected, 1920, mu).1);
        if a.to_bits() != b.to_bits() {
            differing.push((mu, a, b));
        }
    }
    // Deep in the subnormals, (1,920·u)·μ_max rounds to zero or one ulp before
    // the ×10 while 10⁻¹²·μ_max rounds up to the first subnormals, so the
    // test-local h is larger by at most two subnormal ulps. Those μ_max lie
    // below 2⁻¹⁰³². An exact-spectrum scale argument, not a certified
    // exclusion for every computed result: for SPD K and a nonzero PSD M with
    // a positive diagonal entry m_jj, both with magnitudes in the admitted
    // [2⁻³⁰⁰, 2³⁰⁰], the exact μ_max ≥ m_jj/k_jj ≥ 2⁻⁶⁰⁰ (Rayleigh). The
    // admission check (`modal_eigen::admission`) bounds magnitudes only; it
    // does not itself establish those premises or bound the computed μ_max.
    for &(mu, a, b) in &differing {
        assert!(mu < f64::MIN_POSITIVE / 1024.0, "μ_max = {mu:e} (not below 2⁻¹⁰³²): {a:e} against {b:e}");
        assert!(a > b && a - b <= 2.0 * f64::from_bits(1), "μ_max = {mu:e}: {a:e} against {b:e}");
    }
    let normal: Vec<_> = sweep.iter().filter(|&&mu| mu >= f64::MIN_POSITIVE).collect();
    assert!(normal.iter().all(|&&mu| kernel_hard_cut(&local, 1920, mu).1.to_bits() == kernel_hard_cut(&selected, 1920, mu).1.to_bits()));
    println!(
        "n = 1,920: h identical on {} of {} μ_max values ({} normal); differing only at μ_max in [{:e}, {:e}] (at most 2 subnormal ulps)",
        sweep.len() - differing.len(),
        sweep.len(),
        normal.len(),
        differing.first().map_or(f64::NAN, |d| d.0),
        differing.iter().map(|d| d.0).fold(0.0, f64::max)
    );
    // Under the instance, τ_μ is never the active floor: n = 1..=8,192 on a
    // coarser sweep, and larger orders on the full sweep.
    let coarse = mu_max_sweep(&[1.0, 1.5]);
    let check = |n: usize, mu: f64| {
        let (tau_term, h) = kernel_hard_cut(&selected, n, mu);
        let options = modal_dense_options(&selected);
        let derived = options.resolution_floor_multiple * (options.resolution_floor_constant * n as f64 * (f64::EPSILON / 2.0) * mu);
        assert!(tau_term <= derived, "n = {n}, μ_max = {mu:e}: τ_μ·μ_max = {tau_term:e} > κ_μ·δ_q = {derived:e}");
        assert_eq!(h.to_bits(), derived.to_bits(), "n = {n}, μ_max = {mu:e}");
    };
    for n in 1..=8192 {
        for &mu in &coarse {
            check(n, mu);
        }
    }
    for n in [9_000, 10_000, 1 << 20, 1 << 30, 1 << 40, 1 << 52] {
        for &mu in &sweep {
            check(n, mu);
        }
    }
    // At n = 1,920 the test-local set's τ_μ term is not above its derived term
    // on normal μ_max, so its h is that derived term there.
    let local_options = modal_dense_options(&local);
    for &&mu in &normal {
        let (tau_term, h) = kernel_hard_cut(&local, 1920, mu);
        let derived = local_options.resolution_floor_multiple * (local_options.resolution_floor_constant * 1920.0 * (f64::EPSILON / 2.0) * mu);
        assert!(tau_term <= derived, "μ_max = {mu:e}: {tau_term:e} > {derived:e}");
        assert_eq!(h.to_bits(), derived.to_bits(), "μ_max = {mu:e}");
    }
}

/// K = Q·Λ·Qᵀ on four free DOFs of one node, with Q = H₄/2 (Sylvester's
/// Hadamard matrix, exactly orthogonal) and M = I. For dyadic Λ every entry
/// of K is formed exactly in binary64 (each partial sum needs at most 45
/// bits), so the pencil's spectrum is exactly Λ; the test checks K·q = λ·q
/// bit for bit. Built directly: no product model with an independent
/// reference reaches a top mode inside the window (module docs, item 1).
fn hadamard_pencil(lambda: [f64; 4]) -> ModalPencil {
    const H: [[f64; 4]; 4] = [[1.0, 1.0, 1.0, 1.0], [1.0, -1.0, 1.0, -1.0], [1.0, 1.0, -1.0, -1.0], [1.0, -1.0, -1.0, 1.0]];
    let mut k = vec![0.0; 16];
    for i in 0..4 {
        for j in 0..4 {
            k[i * 4 + j] = (0..4).fold(0.0, |s, l| s + H[i][l] * H[j][l] * (lambda[l] / 4.0));
        }
    }
    for (l, &value) in lambda.iter().enumerate() {
        for i in 0..4 {
            let kq = (0..4).fold(0.0, |s, j| s + k[i * 4 + j] * (H[j][l] / 2.0));
            assert_eq!(kq.to_bits(), (value * (H[i][l] / 2.0)).to_bits(), "K·q_{l} row {i}");
        }
    }
    let dofs = [Dof::Ux, Dof::Uy, Dof::Uz, Dof::Rx];
    ModalPencil {
        global_dimension: 6,
        free_dofs: vec![0, 1, 2, 3],
        dof_map: vec![Some(0), Some(1), Some(2), Some(3), None, None],
        free_labels: dofs.iter().map(|&dof| FreeDof { node: 0, dof }).collect(),
        labels: vec!["n:UX".into(), "n:UY".into(), "n:UZ".into(), "n:RX".into()],
        stiffness: SymmetricMatrix::from_row_major(4, k).unwrap(),
        mass: SymmetricMatrix::from_diagonal(&[1.0; 4]).unwrap(),
        stiffness_entries: Vec::new(),
        mass_entries: Vec::new(),
        stiffness_asymmetric_entries: 0,
        excitation: [vec![0.0; 4], vec![0.0; 4], vec![0.0; 4]],
        total_mass: [1.0; 3],
        free_dof_mass: [0.0; 3],
        guard: DenseGuardRecord { global_dimension: 6, free_dimension: 4, static_estimate_bytes: 0, ceiling_bytes: 0, eigen_core_bytes: 0, operation_estimate: 0 },
        // An axis-coordinate pencil (S-K1's restraint basis absent).
        skew: None,
    }
}

/// The product's census and whole-spectrum endpoint of a solved pencil.
fn census_and_whole_endpoint(solution: &DenseEigenSolution, params: &ModalIntegrityParameters) -> (FiniteCensus, Result<ReportedEndpoint, Vec<String>>) {
    let census = FiniteCensus::of_solution(solution);
    let ids = cluster_ids(&solution.eigenvalues, params.cluster_tolerance).unwrap();
    let frequencies: Vec<f64> = solution.eigenvalues.iter().map(|&l| frequency_from_eigenvalue(l).0).collect();
    let endpoint = complete_reported_set(&frequencies, &ids, ExtractionBound::ModeCount(1000), &census)
        .map(|c| c.endpoint)
        .map_err(|found| codes(&found).into_iter().map(str::to_string).collect());
    (census, endpoint)
}

/// Item 1. A genuine finite top mode with μ/μ_max = 2⁻⁴³ ≈ 1.14·10⁻¹³: 8.8×
/// below the old τ_μ = 10⁻¹², and 26× above the derived floor
/// κ_μ·c_μ·n·u = 4.4·10⁻¹⁵ at n = 4. The spectrum is exact by construction
/// (`hadamard_pencil`). Path: the product's own `solve_modal_pencil` (the
/// kernel's dense solve on `modal_dense_options` of the parameters),
/// `FiniteCensus::of_solution` and `complete_reported_set`, on a pencil built
/// directly. No product model carries an independently known top mode in
/// this window: EX n05's top mode sits at 3.86·10⁻¹², above it.
#[test]
fn a_top_mode_below_the_old_tau_mu_and_above_the_derived_floor_is_retained() {
    let lambda = [1.0, 2.0, 3.0, pow2(43)];
    let pencil = hadamard_pencil(lambda);
    let selected = instance();
    let local = test_local_parameters();
    let kept = solve_modal_pencil(&pencil, &selected).unwrap();
    let cut = solve_modal_pencil(&pencil, &local).unwrap();
    // The QL values do not depend on the parameters.
    assert_eq!(kept.mu_max.to_bits(), cut.mu_max.to_bits());
    let n = 4.0;
    let floor = 10.0 * (n * U);
    let mu_top = 1.0 / pow2(43);
    // Under the instance: four modes, the top one retained without recovery.
    assert_eq!(kept.eigenvalues.len(), 4, "{kept:#?}");
    assert!(kept.screened.is_empty(), "{:#?}", kept.screened);
    let computed_top = kept.mu.iter().copied().fold(f64::INFINITY, f64::min);
    let ratio = computed_top / kept.mu_max;
    println!(
        "item 1: μ_top/μ_max = {ratio:.4e} (exact {:.4e}); h/μ_max = {:.3e} (instance) against {:.3e} (test-local); top λ {:.6e} against exact {:.6e}",
        mu_top,
        kept.hard_classification_threshold / kept.mu_max,
        cut.hard_classification_threshold / cut.mu_max,
        kept.eigenvalues[3],
        lambda[3]
    );
    assert!(ratio < 1e-12 / 4.0 && ratio > 4.0 * floor, "the witness lies inside the window with margin: {ratio:e}");
    // The computed top μ is the exact one to within the derived floor (QL's
    // backward-stable scale), relative to μ_max = 1.
    assert!((computed_top - mu_top).abs() <= floor * kept.mu_max, "{computed_top:e} against {mu_top:e}");
    // Nothing is claimed for the three low modes: they carry the reduction's
    // conditioning error (observed: λ = 1 at 1.2·10⁻⁴ relative), which moves
    // μ_max and so the ratio by that amount, far inside the window's margin.
    let (census, endpoint) = census_and_whole_endpoint(&kept, &selected);
    assert_eq!(census, FiniteCensus::Established { finite_mode_count: 4, structurally_massless: 0 });
    assert_eq!(endpoint, Ok(ReportedEndpoint::WholeFiniteSpectrum { established_finite_mode_count: 4 }));
    // Under the test-local set: the same QL μ is hard-rejected by τ_μ alone.
    assert_eq!(cut.eigenvalues.len(), 3);
    assert_eq!(cut.screened.len(), 1);
    assert!(cut.screened[0].hard_rejected && !cut.screened[0].retained && cut.screened[0].recovery.is_none());
    assert_eq!(cut.screened[0].mu.to_bits(), computed_top.to_bits());
    assert_eq!(cut.hard_classification_threshold.to_bits(), (1e-12 * cut.mu_max).to_bits(), "τ_μ is the active cut");
    let (census, endpoint) = census_and_whole_endpoint(&cut, &local);
    assert_eq!(census, FiniteCensus::Unresolved { structurally_massless: 0, nonpositive_mu: 0, rejected_mu: 1, mass_block_established: true });
    assert_eq!(endpoint, Err(vec![MODAL_FINITE_COUNT_UNRESOLVED.to_string()]));
}

/// The population's models whose finite census is unresolved by design: the
/// offset point lumps and the component body (non-coordinate nullspaces).
const TRUE_ZERO_MODELS: [&str; 4] = ["E-POINTMASS-AXIAL", "E-POINTMASS-LATERAL", "E-POINTMASS-GENERAL", "component PF"];

fn true_zero_models() -> Vec<Model> {
    let models: Vec<Model> = population().into_iter().filter(|m| TRUE_ZERO_MODELS.contains(&m.name.as_str())).collect();
    assert_eq!(models.len(), TRUE_ZERO_MODELS.len());
    models
}

/// Item 2 (product path, the phase replay). The rounding-level μ of the
/// non-coordinate nullspaces stay hard-rejected under the instance, below its
/// lower h with margin; the census stays unresolved; the computed set is the
/// test-local set's bit for bit (nothing truncated, nothing added), and every
/// free DOF is accounted for.
#[test]
fn true_zeros_stay_hard_rejected_and_the_census_unresolved_under_the_instance() {
    let selected = instance();
    let local = test_local_parameters();
    for model in true_zero_models() {
        let name = &model.name;
        let new = replay_modal_phases(&modal(model.request.clone()), &selected).unwrap_or_else(|f| panic!("{name}: {f:#?}"));
        let old = replay_modal_phases(&modal(model.request.clone()), &local).unwrap();
        let (s, t) = (&new.solution, &old.solution);
        assert!(matches!(new.census, FiniteCensus::Unresolved { .. }), "{name}: {:?}", new.census);
        assert_eq!(new.census, old.census, "{name}");
        assert!(s.hard_classification_threshold < t.hard_classification_threshold, "{name}: the instance's h is lower");
        assert!(!s.screened.is_empty() || s.nonpositive_mu_count > 0, "{name}: a true zero is present");
        for m in &s.screened {
            assert!(m.hard_rejected && !m.retained && m.recovery.is_none(), "{name}: {m:?}");
            assert!(m.mu <= s.hard_classification_threshold / 100.0, "{name}: μ = {:e} within 100× of h = {:e}", m.mu, s.hard_classification_threshold);
        }
        assert_eq!(s.screened, t.screened, "{name}");
        assert_eq!((s.nonpositive_mu_count, s.below_threshold_mu_count), (t.nonpositive_mu_count, t.below_threshold_mu_count), "{name}");
        assert_eq!(s.eigenvalues.iter().map(|x| x.to_bits()).collect::<Vec<_>>(), t.eigenvalues.iter().map(|x| x.to_bits()).collect::<Vec<_>>(), "{name}");
        assert_eq!(s.mode_count() + s.nonpositive_mu_count + s.below_threshold_mu_count + s.structurally_massless_count, s.dimension, "{name}");
        let largest = s.screened.iter().map(|m| m.mu / s.mu_max).fold(0.0, f64::max);
        println!(
            "item 2: {name}: n = {}, ≤0 {}, rejected {}, largest rejected μ/μ_max {largest:.2e}, h/μ_max {:.2e} (instance) / {:.2e} (test-local), census {:?}",
            s.dimension,
            s.nonpositive_mu_count,
            s.below_threshold_mu_count,
            s.hard_classification_threshold / s.mu_max,
            t.hard_classification_threshold / t.mu_max,
            new.census
        );
    }
}

/// Item 4 (product path, `run_modal_dense`). Under the instance a whole-
/// spectrum bound on an unresolved census refuses
/// `MODAL_FINITE_COUNT_UNRESOLVED`, and a lower set is served only below a
/// completed computed cluster with the gap-shift Sturm count equal to the
/// reported count, and says so. No existing witness reaches
/// `MODAL_STURM_COUNT_MISMATCH` (no test in the crate does), so none is shown.
#[test]
fn the_unresolved_whole_endpoint_refuses_and_lower_sets_keep_their_premises_under_the_instance() {
    let selected = instance();
    for model in true_zero_models() {
        let name = &model.name;
        let mut bare = model.request.clone();
        bare.as_object_mut().unwrap().remove("mode_count");
        for bound in [json!({"mode_count": 1000}), json!({"frequency_cutoff": q(1e9, "Hz")})] {
            let found = run_modal_dense(&modal(with(bare.clone(), bound.clone())), &selected).err().unwrap_or_else(|| panic!("{name} {bound}: served"));
            assert_eq!(codes(&found), vec![MODAL_FINITE_COUNT_UNRESOLVED], "{name} {bound}: {found:#?}");
        }
        let outcome = run_modal_dense(&modal(model.request.clone()), &selected).unwrap_or_else(|f| panic!("{name}: {f:#?}"));
        assert!(matches!(outcome.integrity.census, FiniteCensus::Unresolved { .. }), "{name}");
        assert_eq!(outcome.completed.endpoint, ReportedEndpoint::BelowComputedCluster { next_mode: outcome.modes.len() }, "{name}");
        assert_eq!((outcome.integrity.sturm.count, outcome.integrity.sturm.reported), (outcome.modes.len(), outcome.modes.len()), "{name}");
        assert!(outcome.integrity.sturm.evidence.accepted().is_some(), "{name}");
        assert!(outcome.disclosures.iter().any(|d| d.contains("finite-mode count is not established")), "{name}: {:#?}", outcome.disclosures);
        println!("item 4: {name}: whole spectrum refused; {} modes served below a completed cluster, Sturm count {}", outcome.modes.len(), outcome.integrity.sturm.count);
    }
    // GENERAL's own whole-spectrum bound (21 finite modes computed).
    let general = true_zero_models().into_iter().find(|m| m.name == "E-POINTMASS-GENERAL").unwrap();
    let Err(found) = run_modal_dense(&modal(with(general.request, json!({"mode_count": 21}))), &selected) else { panic!("served") };
    assert_eq!(codes(&found), vec![MODAL_FINITE_COUNT_UNRESOLVED]);
}

/// One run's verdict, as compared between the two parameter sets.
#[derive(Debug, PartialEq)]
enum Verdict {
    Refused(Vec<String>),
    Served {
        census: String,
        reported: (usize, usize, usize),
        computed: usize,
        clusters: Vec<usize>,
        standing: String,
        mode_standings: Vec<String>,
        band_retained: Vec<bool>,
    },
}

fn standing_text(standing: &ModalStanding) -> String {
    match standing {
        ModalStanding::ChecksPassed => "ChecksPassed".to_string(),
        ModalStanding::Sensitive(reasons) => format!("Sensitive({})", reasons.len()),
    }
}

fn verdict(outcome: Result<ModalOutcome, Vec<crate::Diagnostic>>) -> Verdict {
    match outcome {
        Err(found) => Verdict::Refused(codes(&found).into_iter().map(str::to_string).collect()),
        Ok(o) => Verdict::Served {
            census: match o.integrity.census {
                FiniteCensus::Established { finite_mode_count, .. } => format!("established {finite_mode_count}"),
                FiniteCensus::Unresolved { .. } => "unresolved".to_string(),
            },
            reported: (o.reported.bounded_count, o.reported.extension_count, o.reported.count),
            computed: o.integrity.computed.len(),
            clusters: o.integrity.computed.iter().map(|e| e.cluster).collect(),
            standing: standing_text(&o.standing),
            mode_standings: o.modes.iter().map(|m| standing_text(&m.standing)).collect(),
            band_retained: o.integrity.screened.iter().map(|s| s.retained).collect(),
        },
    }
}

/// The modal correctness population (the 35 models above, plus EX n05 at a
/// bound of 4, the test-local set's `MODAL_STURM_SHIFT_UNSEPARATED` witness)
/// run through `run_modal_dense` under the instance and under the test-local
/// set, compared on a verdict projection: refusal codes, the census kind (an
/// established count, or unresolved without its details), the reported
/// counts, the computed count and clusters, the overall and per-mode standing
/// (Sensitive by its reason count only) and the band retentions. Every
/// difference in that projection is printed, and the observed differences are
/// asserted as found (an observation, not a pin). Differences outside the
/// projection are not compared. The existing tests stay on the test-local set.
#[test]
fn the_selected_instance_against_the_test_local_set_on_the_population() {
    let selected = instance();
    let local = test_local_parameters();
    let mut models = population();
    models.push(Model { name: "EX n05 (4)".to_string(), request: with(ex_request_with_density(), json!({"mode_count": 4})), reference: None });
    let mut differences = Vec::new();
    println!("| model | test-local | instance |");
    println!("|---|---|---|");
    for model in &models {
        let old = verdict(run_modal_dense(&modal(model.request.clone()), &local));
        let new = verdict(run_modal_dense(&modal(model.request.clone()), &selected));
        if old != new {
            println!("| {} | {old:?} | {new:?} |", model.name);
            differences.push((model.name.clone(), old, new));
        }
    }
    println!("{} models; {} differ: {:?}", models.len(), differences.len(), differences.iter().map(|d| &d.0).collect::<Vec<_>>());
    // Observed: only δ_c (10⁻⁸ → 2⁻²⁰) moves anything. EX n05's second bending
    // pair (computed modes 4 and 5, split 1.38·10⁻⁸) becomes one cluster;
    // nothing else changes at a bound of 3. At a bound of 4 the test-local set
    // refuses MODAL_STURM_SHIFT_UNSEPARATED inside that pair, and the instance
    // extends the set to the pair's top and serves five modes, counted and
    // ChecksPassed. No standing, census, band or other refusal changes on the
    // rest of the population (τ_r = 2⁻³⁰ demotes no mode here).
    assert_eq!(differences.iter().map(|d| d.0.as_str()).collect::<Vec<_>>(), ["EX n05", "EX n05 (4)"], "the population's differences moved");
    let served = |v: &Verdict| match v {
        Verdict::Served { census, reported, computed, clusters, standing, mode_standings, band_retained } => {
            (census.clone(), *reported, *computed, clusters.clone(), standing.clone(), mode_standings.clone(), band_retained.clone())
        }
        Verdict::Refused(found) => panic!("refused: {found:?}"),
    };
    let (old, new) = (served(&differences[0].1), served(&differences[0].2));
    assert_eq!(old.3, [0, 1, 1, 2, 3, 4, 5]);
    assert_eq!(new.3, [0, 1, 1, 2, 2, 3, 4]);
    assert_eq!((old.0, old.1, old.2, old.4, old.5, old.6), (new.0, new.1, new.2, new.4, new.5, new.6), "EX n05: only the clusters differ");
    assert_eq!(differences[1].1, Verdict::Refused(vec![MODAL_STURM_SHIFT_UNSEPARATED.to_string()]));
    let extended = served(&differences[1].2);
    assert_eq!((extended.0.as_str(), extended.1, extended.3), ("established 7", (4, 1, 5), new.3));
    assert_eq!(extended.4, "ChecksPassed");
    assert!(extended.5.iter().all(|s| s == "ChecksPassed"));
}
