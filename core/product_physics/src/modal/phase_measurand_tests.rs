//! DEL-04-09 O-4 phase measurand (Design §5.3.8) and the phase replay it
//! times. Test code only: nothing here is reachable from a product entry,
//! and no product source carries instrumentation.
//!
//! **The replay.** [`replay_modal_phases`] calls the product's own phase
//! functions (`prepare_modal`, `solve_modal_state_case`, `assemble_mass`,
//! `check_weight_basis`, `form_modal_pencil`, `solve_modal_pencil`,
//! `complete_reported_set`, `normalize_modal_shape`,
//! `modal_reformation_operands`, `reform_modal_pencil`, `modal_residual`,
//! `modal_sturm_count`, `modal_orthogonality`) in `run_modal_dense`'s order,
//! with a monotonic clock read between them. It stops before the outcome's
//! row assembly (`run_modal_dense`'s tail), which builds no matrix.
//! `the_phase_replay_is_the_product_path` holds the replay to
//! `run_modal_dense` on every value both compute, so a change to the
//! product's sequence that the replay does not follow fails that test.
//!
//! **The measurand** (`o4_phase_measurand_sec_a_cantilever`) is `#[ignore]`d
//! and kept out of the normal test path: it runs only inside a separately
//! authorized measurement window (§5.3.8's stage plan). It pins nothing,
//! applies no budget and has no production parameter set: it uses the
//! labelled **TEST-LOCAL, EXPERIMENTAL** set of the fixtures.
//!
//! **Placement and retained pricing.** The file name ends in `_tests.rs`, so
//! the retained-pricing scanners skip it (`price.py`'s template inventory,
//! `callgraph_g5.py`, `loopscan.py`'s test-file rule and the line map); its
//! only declaration is a `#[cfg(test)]` module item in `modal.rs`, which the
//! scanners blank. No format site, call or loop enters a priced inventory.
use super::document::{modal_doc_solved, modal_result_document};
use super::eigen::{
    complete_reported_set, modal_basis, modal_extraction_bound, modal_sturm_count, normalize_modal_shape, solve_modal_pencil,
    CompletedSet, FiniteCensus, SturmRecord,
};
use super::integrity::{modal_orthogonality, modal_reformation_operands, modal_residual, reform_modal_pencil, ModalIntegrityParameters};
use super::mass::assemble_mass;
use super::outcome::{run_modal_dense, ModalOutcome, MODAL_DENSE_PATH_MODE};
use super::prepare::prepare_modal;
use super::request::ModalPreviewRequest;
use super::state::solve_modal_state_case;
use super::stiffness::{form_modal_pencil, ModalPencil};
use super::tests::{arc_request, ex_request_with_density, modal, pf_request, q, with};
use super::tests_eigen::{sec_a_cantilever, test_local_parameters};
use super::tests_state::kd5;
use super::weight_basis::check_weight_basis;
use crate::Diagnostic;
use open_pipe_stress_canonical_json::canonical_json_checked_v1_text;
use open_pipe_stress_modal_eigen::clusters::cluster_ids;
use open_pipe_stress_modal_eigen::dense::{dense_path_flop_estimate, dense_screen_product_estimate, DenseEigenSolution};
use open_pipe_stress_modal_eigen::modal_quantities::{frequency_from_eigenvalue, NormalizedMode};
use open_pipe_stress_modal_eigen::orthogonality::MOrthogonality;
use open_pipe_stress_modal_eigen::reformation::{cluster_error_estimates, FormationDifference, ReformedPencil, ResidualEstimate};
use open_pipe_stress_modal_eigen::sturm::ShiftPencil;
use serde_json::{json, Value};
use std::time::{Duration, Instant};

/// The label of the measurand's parameter set: the fixtures' test-local set.
const MEASURAND_LABEL: &str = "TEST_LOCAL_I3A_EXPERIMENTAL";

/// A monotonic clock read at each phase boundary.
struct PhaseClock {
    last: Instant,
    phases: Vec<(&'static str, Duration)>,
}

impl PhaseClock {
    fn start() -> Self {
        Self { last: Instant::now(), phases: Vec::new() }
    }

    fn lap(&mut self, name: &'static str) {
        let now = Instant::now();
        self.phases.push((name, now - self.last));
        self.last = now;
    }
}

/// Everything the replay computed, owned, with its phase times.
pub(super) struct ModalReplay {
    pub(super) phases: Vec<(&'static str, Duration)>,
    pub(super) global_dimension: usize,
    pub(super) element_operands: usize,
    pub(super) pencil: ModalPencil,
    pub(super) solution: DenseEigenSolution,
    pub(super) census: FiniteCensus,
    pub(super) cluster_ids: Vec<usize>,
    pub(super) completed: CompletedSet,
    pub(super) bounding_box_diagonal: f64,
    pub(super) normalized: Vec<NormalizedMode>,
    pub(super) reformed: ReformedPencil,
    pub(super) formation: FormationDifference,
    pub(super) residuals: Vec<ResidualEstimate>,
    pub(super) cluster_estimates: Vec<f64>,
    pub(super) sturm: SturmRecord,
    pub(super) orthogonality: MOrthogonality,
}

impl ModalReplay {
    pub(super) fn reported(&self) -> usize {
        self.completed.set.count
    }

    /// Computed mode i's vector over the free DOFs as checks 2 and 4 see it:
    /// the published shape of a reported mode, the solver's vector otherwise.
    pub(super) fn vector(&self, i: usize) -> &[f64] {
        if i < self.reported() {
            &self.normalized[i].shape
        } else {
            self.solution.vector(i)
        }
    }

    /// Whether computed mode i is a band-recovered pair (§3.2), by the same
    /// bit comparison `run_modal_dense` uses.
    pub(super) fn band_recovered(&self, i: usize) -> bool {
        let lambda = self.solution.eigenvalues[i];
        self.solution
            .screened
            .iter()
            .filter(|s| s.retained)
            .filter_map(|s| s.recovery.as_ref())
            .any(|r| r.eigenvalue.to_bits() == lambda.to_bits())
    }
}

/// The bounding-box diagonal ℓ of §1.5, as `run_modal_dense` forms it (its
/// helper is private to `outcome`; the equality test holds this copy to it).
fn bounding_box_diagonal(positions: &[[f64; 3]]) -> f64 {
    let mut low = [f64::INFINITY; 3];
    let mut high = [f64::NEG_INFINITY; 3];
    for p in positions {
        for k in 0..3 {
            low[k] = low[k].min(p[k]);
            high[k] = high[k].max(p[k]);
        }
    }
    let d = [high[0] - low[0], high[1] - low[1], high[2] - low[2]];
    open_pipe_stress_correct_norm::norm3(d[0], d[1], d[2])
}

/// `run_modal_dense`'s phases, in its order, each timed (module docs).
pub(super) fn replay_modal_phases(req: &ModalPreviewRequest, params: &ModalIntegrityParameters) -> Result<ModalReplay, Vec<Diagnostic>> {
    let mut clock = PhaseClock::start();
    let refusals = params.modal_parameter_refusals();
    if !refusals.is_empty() {
        return Err(refusals);
    }
    let prepared = prepare_modal(req)?;
    // R-6/R-7 S-K1 (§11.3 C8, B2 (b)): the replay mirrors the axis path only; a model with a
    // restraint basis is refused by name, never replayed through it.
    if prepared.built.skew.is_some() {
        return Err(vec![crate::diag(
            "diagnostic:modal:replay:skew-restraint",
            super::integrity::MODAL_PHASE_REPLAY_SKEW_RESTRAINT_UNSUPPORTED,
            "blocking",
            "the phase replay mirrors the axis-coordinate modal path only; a model with a skewed restraint is not replayed".to_string(),
            vec!["supports".into()],
        )]);
    }
    let nonlinear_state = solve_modal_state_case(&prepared, MODAL_DENSE_PATH_MODE)?;
    clock.lap("prepare");
    let mass = assemble_mass(&prepared, prepared.inputs.weight_to_mass_acceleration)?;
    check_weight_basis(&prepared, &mass, params.identity_tolerance)?;
    clock.lap("mass_and_identity");
    let restrained = nonlinear_state.as_ref().map_or(&prepared.boundary.restrained_dofs[..], |state| &state.restrained_dofs[..]);
    let pencil = form_modal_pencil(&prepared, &mass, restrained, params.sturm_max_candidates)?;
    clock.lap("pencil_and_screen");
    let solution = solve_modal_pencil(&pencil, params)?;
    clock.lap("dense_solve");
    let census = FiniteCensus::of_solution(&solution);
    let ids = cluster_ids(&solution.eigenvalues, params.cluster_tolerance).expect("the replay's models cluster");
    let frequencies: Vec<f64> = solution.eigenvalues.iter().map(|&l| frequency_from_eigenvalue(l).0).collect();
    let bound = modal_extraction_bound(&prepared.inputs).expect("a prepared request names its bound");
    let completed = complete_reported_set(&frequencies, &ids, bound, &census)?;
    clock.lap("clusters_and_reported_set");
    let positions: Vec<[f64; 3]> = prepared.built.nodes.iter().map(|n| n.coordinates).collect();
    let diagonal = bounding_box_diagonal(&positions);
    let basis = modal_basis(&pencil, diagonal, params)?;
    let mut normalized = Vec::with_capacity(completed.set.count);
    for mode in 0..completed.set.count {
        normalized.push(normalize_modal_shape(&basis, mode, solution.vector(mode))?);
    }
    drop(basis);
    clock.lap("normalization");
    let operands = modal_reformation_operands(&prepared, &mass)?;
    let (reformed, formation) = reform_modal_pencil(
        &operands,
        prepared.exact,
        pencil.free_dofs.len(),
        &pencil.dof_map,
        None,
        &pencil.stiffness_entries,
        &pencil.mass_entries,
    )?;
    clock.lap("reformation");
    let reported = completed.set.count;
    let total = solution.eigenvalues.len();
    let vectors: Vec<&[f64]> = (0..total).map(|i| if i < reported { normalized[i].shape.as_slice() } else { solution.vector(i) }).collect();
    let mut residuals = Vec::with_capacity(total);
    for (i, vector) in vectors.iter().enumerate() {
        residuals.push(modal_residual(&reformed, &formation, i, solution.eigenvalues[i], vector).map_err(|d| vec![d])?);
    }
    let estimates: Vec<f64> = residuals.iter().map(|r| r.error_estimate).collect();
    let per_cluster = cluster_error_estimates(&estimates, &ids).expect("the replay's estimates are finite");
    let cluster_estimates: Vec<f64> = ids.iter().map(|&id| per_cluster[id]).collect();
    clock.lap("residuals");
    let cutoff_eigenvalue = prepared.inputs.frequency_cutoff_hz.map(|f| {
        let omega = 2.0 * std::f64::consts::PI * f;
        omega * omega
    });
    let sturm = modal_sturm_count(&pencil, &completed, &solution.eigenvalues, &vectors, &cluster_estimates, cutoff_eigenvalue, params)?;
    clock.lap("sturm_evidence");
    let published: Vec<&[f64]> = normalized.iter().map(|m| m.shape.as_slice()).collect();
    let orthogonality = modal_orthogonality(&reformed, &published).map_err(|d| vec![d])?;
    clock.lap("orthogonality");
    drop(vectors);
    drop(published);
    Ok(ModalReplay {
        phases: clock.phases,
        global_dimension: pencil.global_dimension,
        element_operands: operands.len(),
        pencil,
        solution,
        census,
        cluster_ids: ids,
        completed,
        bounding_box_diagonal: diagonal,
        normalized,
        reformed,
        formation,
        residuals,
        cluster_estimates,
        sturm,
        orthogonality,
    })
}

/// Requires the replay and the product's outcome to agree on every value
/// both compute (bit for bit: both run the same functions on the same
/// inputs in one thread).
fn assert_replay_matches(name: &str, replay: &ModalReplay, outcome: &ModalOutcome) {
    let computed = &outcome.integrity.computed;
    assert_eq!(computed.len(), replay.solution.eigenvalues.len(), "{name}");
    for (i, evidence) in computed.iter().enumerate() {
        assert_eq!(evidence.eigenvalue.to_bits(), replay.solution.eigenvalues[i].to_bits(), "{name} computed mode {i}");
        assert_eq!(evidence.cluster, replay.cluster_ids[i], "{name} computed mode {i}");
        assert_eq!(evidence.residual, replay.residuals[i], "{name} computed mode {i}");
        assert_eq!(evidence.band_recovered, replay.band_recovered(i), "{name} computed mode {i}");
    }
    assert_eq!(outcome.completed, replay.completed, "{name}");
    assert_eq!(outcome.integrity.census, replay.census, "{name}");
    assert_eq!(outcome.integrity.screened, replay.solution.screened, "{name}");
    assert_eq!(outcome.integrity.formation, replay.formation, "{name}");
    assert_eq!(outcome.integrity.sturm, replay.sturm, "{name}");
    assert_eq!(outcome.integrity.orthogonality, replay.orthogonality, "{name}");
    assert_eq!(outcome.guard, replay.pencil.guard, "{name}");
    assert_eq!(outcome.bounding_box_diagonal.to_bits(), replay.bounding_box_diagonal.to_bits(), "{name}");
    assert_eq!(outcome.modes.len(), replay.normalized.len(), "{name}");
    for (mode, normalized) in outcome.modes.iter().zip(&replay.normalized) {
        let free: Vec<f64> = replay.pencil.free_dofs.iter().map(|&dof| mode.shape[dof]).collect();
        assert_eq!(free, normalized.shape, "{name} mode {}", mode.index);
        assert_eq!(mode.cluster_error_estimate.to_bits(), replay.cluster_estimates[mode.index - 1].to_bits(), "{name} mode {}", mode.index);
    }
}

/// The models the equality test runs: every route and feature the replay's
/// phases branch on (exact route, arc, component body, a nonlinear state
/// case, cluster extension, a cut-off below the lowest mode, the whole
/// finite spectrum).
fn replay_models() -> Vec<(&'static str, Value)> {
    const U1: &str = include_str!("../../../../validation/references/kd5_nonlinear/u1_request.json");
    vec![
        ("ex_n05", with(ex_request_with_density(), json!({"mode_count": 3}))),
        ("arc", with(arc_request(false), json!({"mode_count": 6}))),
        ("component_body", with(pf_request(true), json!({"mode_count": 3, "weight_to_mass_acceleration": q(9.8125, "m/s^2")}))),
        ("state_one_way_pressed", with(kd5(U1), json!({"mode_count": 3, "state_case": "load:PRESSED"}))),
        ("cantilever_whole_spectrum", with(sec_a_cantilever(2), json!({"mode_count": 1000}))),
        ("cantilever_cluster_extension", with(sec_a_cantilever(2), json!({"mode_count": 1}))),
        ("cantilever_below_cutoff", with(sec_a_cantilever(2), json!({"frequency_cutoff": q(0.5, "Hz")}))),
    ]
}

#[test]
fn the_phase_replay_is_the_product_path() {
    let params = test_local_parameters();
    for (name, request) in replay_models() {
        let parsed = modal(request);
        let replay = replay_modal_phases(&parsed, &params).unwrap_or_else(|found| panic!("{name}: {found:#?}"));
        let outcome = run_modal_dense(&parsed, &params).unwrap_or_else(|found| panic!("{name}: {found:#?}"));
        assert_replay_matches(name, &replay, &outcome);
        let names: Vec<&str> = replay.phases.iter().map(|(phase, _)| *phase).collect();
        assert_eq!(
            names,
            [
                "prepare",
                "mass_and_identity",
                "pencil_and_screen",
                "dense_solve",
                "clusters_and_reported_set",
                "normalization",
                "reformation",
                "residuals",
                "sturm_evidence",
                "orthogonality"
            ],
            "{name}"
        );
    }
    // The measurand's binding route: the product's own projection of the
    // same request refused before any n² allocation carries the binding of
    // its solved projection, so projecting the outcome with it gives the
    // product entry's document byte for byte.
    let request = with(sec_a_cantilever(2), json!({"mode_count": 4}));
    let outcome = run_modal_dense(&modal(request.clone()), &params).unwrap();
    let binding = refused_binding(&request, &params);
    assert_eq!(modal_doc_solved(&outcome, &binding, MEASURAND_LABEL), modal_result_document(&request, &params, MEASURAND_LABEL));
}

/// The binding of `request`'s document, from the product's own projection
/// refused at the dense guard (the test-only ceiling override lowered to 0
/// on this thread, restored after). The guard refuses after preparation,
/// the mass and the identity, before any n² allocation; only requests with
/// no state case take this route (the measurand's cantilever has none).
fn refused_binding(request: &Value, params: &ModalIntegrityParameters) -> Value {
    assert!(request.get("state_case").is_none());
    crate::DENSE_SCRUTINY_CEILING_OVERRIDE.with(|cell| cell.set(Some(0)));
    let refused = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| modal_result_document(request, params, MEASURAND_LABEL)));
    crate::DENSE_SCRUTINY_CEILING_OVERRIDE.with(|cell| cell.set(None));
    let refused = refused.unwrap_or_else(|panic| std::panic::resume_unwind(panic));
    assert_eq!(refused["status"], "refused");
    assert_eq!(refused["refusals"]["rows"][0]["code"], "SOLVER_SYSTEM_BLOCKED", "{refused:#}");
    refused["binding"].clone()
}

fn required_env(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| {
        panic!(
            "{name} is required: MODAL_MEASURAND_NF (free DOFs, a positive multiple of 6), MODAL_MEASURAND_MODES \
             (the request's mode_count) and MODAL_MEASURAND_PASS (phases, entry or both); there is no default point"
        )
    })
}

fn seconds(d: Duration) -> f64 {
    d.as_secs_f64()
}

/// **O-4 phase measurand (ignored).** One deterministic, source-bound model:
/// the SEC-A straight cantilever of `tests_eigen::sec_a_cantilever` with
/// n_f/6 elements (n_f free DOFs, 6(n_f/6 + 1) global), `mode_count` from
/// the environment, under the TEST-LOCAL set. Two passes, selectable so a
/// window can run one per process for its own peak RSS:
/// - `phases`: [`replay_modal_phases`], each phase timed;
/// - `entry`: `run_modal_dense` timed whole, then the document projection
///   (`modal_doc_solved`) and its canonical text timed separately.
///
/// It prints `O4 key=value` lines. Work counts are counts or stated
/// formulas, never timings: the binary64 flop estimate, the band recovery's
/// exact-product insertions and the re-formation's exact work are reported
/// separately and are not interchangeable. Peak RSS is not sampled in
/// process: the caller reads it per process from `/usr/bin/time -l`, and no
/// per-phase RSS is reported.
#[test]
#[ignore = "O-4 phase measurand: runs only inside a separately authorized measurement window (Design §5.3.8)"]
fn o4_phase_measurand_sec_a_cantilever() {
    let n_f: usize = required_env("MODAL_MEASURAND_NF").parse().expect("MODAL_MEASURAND_NF is an integer");
    let modes: usize = required_env("MODAL_MEASURAND_MODES").parse().expect("MODAL_MEASURAND_MODES is an integer");
    let pass = required_env("MODAL_MEASURAND_PASS");
    assert!(n_f >= 6 && n_f % 6 == 0, "MODAL_MEASURAND_NF = {n_f} is not a positive multiple of 6");
    assert!(modes >= 1, "MODAL_MEASURAND_MODES must be at least 1");
    assert!(matches!(pass.as_str(), "phases" | "entry" | "both"), "MODAL_MEASURAND_PASS = {pass}");
    let elements = n_f / 6;
    let request = with(sec_a_cantilever(elements), json!({"mode_count": modes}));
    let params = test_local_parameters();

    println!("O4 model=SEC-A cantilever L=6m elements={elements} n_f={n_f} n_global={} mode_count={modes} pass={pass}", 6 * (elements + 1));
    println!(
        "O4 build debug_assertions={} target={}-{} pointer_width={} parameters={MEASURAND_LABEL}",
        cfg!(debug_assertions),
        std::env::consts::ARCH,
        std::env::consts::OS,
        usize::BITS
    );

    let mut replay_bits: Option<Vec<u64>> = None;
    if pass == "phases" || pass == "both" {
        let started = Instant::now();
        let parsed = modal(request.clone());
        let parse = started.elapsed();
        let replay = replay_modal_phases(&parsed, &params).unwrap_or_else(|found| panic!("refused: {found:#?}"));
        println!("O4 phase name=parse seconds={:.6}", seconds(parse));
        for (name, duration) in &replay.phases {
            println!("O4 phase name={name} seconds={:.6}", seconds(*duration));
        }
        let total: Duration = replay.phases.iter().map(|(_, d)| *d).sum();
        println!("O4 phases_total seconds={:.6}", seconds(total));
        let n = replay.pencil.free_dofs.len();
        let candidates = &replay.sturm.evidence.candidates;
        let factored = candidates.iter().filter(|c| c.factored.is_some()).count();
        let solves: usize = candidates.iter().filter_map(|c| c.factored.as_ref()).map(|f| f.inverse_norm_solves).sum();
        let screened = replay.solution.screened.len();
        let band_retained = replay.solution.screened.iter().filter(|s| s.retained).count();
        let clusters = replay.cluster_ids.last().map_or(0, |&c| c + 1);
        // The Sturm pencil's union pattern, re-formed here after the timed
        // phases (`modal_sturm_count` forms and drops its own).
        let pattern = ShiftPencil::from_entries(n, &replay.pencil.stiffness_entries, &replay.pencil.mass_entries)
            .expect("the replay's pencil forms")
            .pattern_entry_count();
        println!(
            "O4 counts n_free={n} computed={} reported={} clusters={clusters} census={:?} ql_iterations={} structurally_massless={}",
            replay.solution.eigenvalues.len(),
            replay.reported(),
            replay.census,
            replay.solution.ql_iterations,
            replay.solution.structurally_massless_count
        );
        println!(
            "O4 sturm placement={:?} candidates={} factored={factored} inverse_norm_solves={solves} pattern_entries={pattern}",
            replay.sturm.evidence.placement,
            candidates.len()
        );
        println!(
            "O4 reformation element_operands={} stiffness_entries={} mass_entries={} residual_evaluations={}",
            replay.element_operands,
            replay.pencil.stiffness_entries.len(),
            replay.pencil.mass_entries.len(),
            replay.residuals.len()
        );
        println!(
            "O4 work binary64_flop_estimate={} (9n^3 + s*n^3/3 at s={factored} factored shifts; a stated formula, not measured) \
             band_screened={screened} band_retained={band_retained} band_exact_insertions_estimate={} (4pn^2; exact products, not binary64 flops)",
            dense_path_flop_estimate(n as u64, factored as u64),
            dense_screen_product_estimate(n as u64, screened as u64)
        );
        println!(
            "O4 stated_bytes static_guard={} eigen_core={} (formulas the product records; not measurements, not a guard)",
            replay.pencil.guard.static_estimate_bytes, replay.pencil.guard.eigen_core_bytes
        );
        replay_bits = Some(replay.solution.eigenvalues.iter().map(|l| l.to_bits()).collect());
    }

    if pass == "entry" || pass == "both" {
        let started = Instant::now();
        let parsed = modal(request.clone());
        let parse = started.elapsed();
        let started = Instant::now();
        let outcome = run_modal_dense(&parsed, &params).unwrap_or_else(|found| panic!("refused: {found:#?}"));
        let run = started.elapsed();
        if let Some(bits) = &replay_bits {
            let entry: Vec<u64> = outcome.integrity.computed.iter().map(|e| e.eigenvalue.to_bits()).collect();
            assert_eq!(&entry, bits, "the entry pass and the replay computed different eigenvalues");
        }
        let started = Instant::now();
        let binding = refused_binding(&request, &params);
        let binding_time = started.elapsed();
        let started = Instant::now();
        let document = modal_doc_solved(&outcome, &binding, MEASURAND_LABEL);
        let projection = started.elapsed();
        let started = Instant::now();
        let compact = serde_json::to_string(&document).expect("the document serializes");
        let canonical = canonical_json_checked_v1_text(&compact).expect("the document is in the canonical profile");
        let serialization = started.elapsed();
        println!("O4 entry parse_seconds={:.6} run_modal_dense_seconds={:.6}", seconds(parse), seconds(run));
        println!(
            "O4 entry binding_seconds={:.6} (the refused projection at the dense guard; not a product phase) projection_seconds={:.6} canonical_seconds={:.6}",
            seconds(binding_time),
            seconds(projection),
            seconds(serialization)
        );
        let mut rows_total = 0usize;
        if let Some(tables) = document["tables"].as_object() {
            for (name, table) in tables {
                let rows = table["rows"].as_array().map_or(0, Vec::len);
                rows_total += rows;
                println!("O4 table name={name} rows={rows}");
            }
        }
        println!("O4 output rows={rows_total} canonical_bytes={} compact_bytes={}", canonical.len(), compact.len());
    }
}
