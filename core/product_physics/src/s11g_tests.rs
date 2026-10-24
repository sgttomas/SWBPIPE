//! S11-G product tests (`S11G_GUARD.md` revision 2.1 section 8 and revision
//! 2.2 with its erratum, with ROOT's I5 rulings): T1-T6, T4b, T6a, T6b, T10,
//! T11-T13, T13b, T15, T16, T18-T22, the ruling tests (the B = T0 = 0
//! boundary, R-b' alone, both guards) and the unit tests that kill M10, M18,
//! X4 and X7. T7, T14 and T17 live with SP and the ledger; T8 and T10b in
//! `tests/s11f_site_test.rs`; T23 in the headless runner's tests.
//!
//! Every verdict pin first asserts, inside the test, that the two paths differ
//! (the ordinary report is `Passed`, and the guard's statistic is on the
//! firing side, computed from the product's own ledger rows or published
//! values), and only then asserts the verdict. Inputs are invented; no
//! material, component, catalogue or code-rule data is used. Each property is
//! stated where it is authored.
use super::*;
use crate::formation_guard::{self, decide_row, row_scales, RowDecision};
use open_pipe_stress_frame_kernel::structural::ArcCertificateFailure;
use open_pipe_stress_frame_kernel::exact_sum::ExactAccumulator;
use open_pipe_stress_frame_kernel::load_ledger::{Formation, FormationRow, LoadLedger};
use serde_json::{json, Value};
use std::collections::{BTreeSet, HashSet};

const INVENTED: &str = "invented_t3_s11g_test_input_not_library_data";
const MODES: [PreviewSolverMode; 2] = [
    PreviewSolverMode::SparseInteractive,
    PreviewSolverMode::DenseScrutiny,
];
const LOAD_ROW: &str = "S11-G formation-noise guard";
const RECOVERY: &str = "S11-G recovery guard (R-b')";

// ------------------------------------------------------------------ entries

#[derive(Clone, Copy, Debug, PartialEq)]
enum Entry {
    /// `run_linear_static_preview_value_with_mode` (the captured entry).
    Captured,
    /// `run_linear_static_preview_with_mode` (the historical typed entry).
    Typed,
}

fn run(
    entry: Entry,
    request: &Value,
    mode: PreviewSolverMode,
) -> Result<MechanicsEnvelope, String> {
    match entry {
        Entry::Captured => run_linear_static_preview_value_with_mode(request.clone(), mode),
        Entry::Typed => {
            let typed: LinearStaticPreviewRequest =
                serde_json::from_value(request.clone()).map_err(|e| format!("DTO: {e}"))?;
            Ok(run_linear_static_preview_with_mode(typed, mode))
        }
    }
}

fn blocking(envelope: &MechanicsEnvelope) -> Vec<String> {
    envelope
        .diagnostics
        .iter()
        .filter(|d| d.severity == "blocking")
        .map(|d| format!("{}: {}", d.code, d.message))
        .collect()
}

fn solved(entry: Entry, request: &Value, mode: PreviewSolverMode) -> MechanicsEnvelope {
    let envelope = run(entry, request, mode).unwrap_or_else(|e| panic!("{entry:?} {mode:?}: {e}"));
    let blocked = blocking(&envelope);
    assert!(
        blocked.is_empty(),
        "{entry:?} {mode:?} blocked: {blocked:?}"
    );
    envelope
}

fn codes(envelope: &MechanicsEnvelope) -> BTreeSet<String> {
    envelope
        .diagnostics
        .iter()
        .map(|d| d.code.clone())
        .collect()
}

fn integrity<'a>(envelope: &'a MechanicsEnvelope, case: &str) -> &'a Diagnostic {
    envelope
        .diagnostics
        .iter()
        .find(|d| d.id == integrity_diagnostic_id(case))
        .unwrap_or_else(|| panic!("integrity diagnostic of {case}"))
}

fn case_quality(envelope: &MechanicsEnvelope, case: &str) -> NumericalQualityStatus {
    envelope
        .numerical_quality
        .cases
        .iter()
        .find(|q| q.basis_ref.ref_id == case)
        .unwrap_or_else(|| panic!("numerical quality of {case}"))
        .solve_quality
}

fn case_rows<'a>(envelope: &'a MechanicsEnvelope, case: &str) -> Vec<&'a ResultItem> {
    envelope
        .results
        .iter()
        .filter(|r| r.basis_ref.as_ref().is_some_and(|b| b.ref_id == case))
        .collect()
}

/// The precondition every verdict pin needs: the ordinary report is Passed,
/// so without S11-G the case would publish `CHECKS_PASSED`.
fn assert_ordinary_passed(envelope: &MechanicsEnvelope, case: &str, label: &str) {
    let message = &integrity(envelope, case).message;
    assert!(
        message.contains("quality: Passed"),
        "{label}: precondition, the ordinary report is not Passed: {message}"
    );
}

/// The demoted verdict, on every view the envelope carries: the integrity
/// code and severity, the reason sentence (exactly once), the truthful
/// StructuralReport text, the case's `solve_quality` and the envelope status.
/// `numerical_quality.status != checks_passed` is what result_export's
/// `numerical_use_standing` maps to `needs_recompute`.
fn assert_demoted(envelope: &MechanicsEnvelope, case: &str, phrase: &str, label: &str) {
    let d = integrity(envelope, case);
    assert_eq!(d.code, "NUMERICAL_INTEGRITY_SENSITIVE", "{label}");
    assert_eq!(d.severity, "warning", "{label}");
    assert_eq!(
        d.message.matches(phrase).count(),
        1,
        "{label}: {}",
        d.message
    );
    assert!(
        d.message.contains("quality: Passed"),
        "{label}: report text stays truthful"
    );
    assert_eq!(d.affected_refs, vec![case.to_string()], "{label}");
    assert_eq!(
        case_quality(envelope, case),
        NumericalQualityStatus::Sensitive,
        "{label}"
    );
    assert_ne!(
        envelope.numerical_quality.status,
        NumericalQualityStatus::ChecksPassed,
        "{label}"
    );
    assert!(blocking(envelope).is_empty(), "{label}: never refused");
}

fn assert_not_demoted_by_s11g(envelope: &MechanicsEnvelope, case: &str, label: &str) {
    let d = integrity(envelope, case);
    assert!(!d.message.contains("S11-G"), "{label}: {}", d.message);
}

/// K-D5's closure (DEL-04-01 kd5-member-relative-bound.md §§1, 4.2): ρ is
/// formed from the intended loads re-formed at p, so a formed-term ledger
/// defect that the load-row guard judged on a Passed report is now judged by
/// D5C-1 on the closed residual, at the solve. The verdict pinned here: the
/// report is Sensitive, the formation check demotes by its estimate at `row`,
/// no S11-G sentence is added (the no-op rule), the case quality and the
/// envelope status are not Passed, and nothing is refused.
fn assert_demoted_at_the_solve(envelope: &MechanicsEnvelope, case: &str, row: &str, label: &str) {
    let d = integrity(envelope, case);
    assert_eq!(d.code, "NUMERICAL_INTEGRITY_SENSITIVE", "{label}");
    assert_eq!(d.severity, "warning", "{label}");
    assert!(d.message.contains("quality: Sensitive"), "{label}: {}", d.message);
    assert!(
        d.message.contains(&format!("formation_check: reason=estimate; row={row};")),
        "{label}: {}",
        d.message
    );
    assert!(!d.message.contains("S11-G"), "{label}: {}", d.message);
    assert_eq!(d.affected_refs, vec![case.to_string()], "{label}");
    assert_eq!(
        case_quality(envelope, case),
        NumericalQualityStatus::Sensitive,
        "{label}"
    );
    assert_ne!(
        envelope.numerical_quality.status,
        NumericalQualityStatus::ChecksPassed,
        "{label}"
    );
    assert!(blocking(envelope).is_empty(), "{label}: never refused");
}

// ------------------------------------------------------ the guard's own view

/// The case's ledger rows and their scales, from the product's own producer
/// sequence and boundary (as `solve_load_case` forms them), for a pre-0.4
/// request without exact pressure.
struct GuardView {
    rows: Vec<FormationRow>,
    scales: HashMap<usize, f64>,
    labels: Vec<String>,
    force: AssembledForce,
}

impl GuardView {
    fn dof(&self, label: &str) -> usize {
        self.labels
            .iter()
            .position(|l| l == label)
            .unwrap_or_else(|| panic!("no DOF {label}"))
    }
    fn row(&self, label: &str) -> &FormationRow {
        let dof = self.dof(label);
        self.rows
            .iter()
            .find(|r| r.dof == dof)
            .unwrap_or_else(|| panic!("row {label} is not loaded"))
    }
    fn decision(&self, label: &str) -> RowDecision {
        let row = self.row(label);
        decide_row(row, self.scales[&row.dof])
    }
    fn any_fires(&self) -> bool {
        self.rows
            .iter()
            .any(|row| decide_row(row, self.scales[&row.dof]).fires)
    }
}

struct Prepared {
    model: PreviewModel,
    materials: Vec<MaterialInput>,
    built: BuiltModel,
    diagnostics: Vec<Diagnostic>,
}

fn prepared(request: &Value) -> Prepared {
    let request: LinearStaticPreviewRequest = serde_json::from_value(request.clone()).unwrap();
    let mut model = request.model;
    let mut materials = if request.materials.is_empty() {
        model.materials.clone()
    } else {
        request.materials
    };
    let mut diagnostics = Vec::new();
    resolve_shared_sections(&mut model, &mut diagnostics);
    normalize_model_units(&mut model, &mut materials, &mut diagnostics);
    pressure_material::resolve_base(&model, &mut materials, &mut diagnostics);
    let built = build_model(&model, &materials, &mut diagnostics).expect("model builds");
    Prepared {
        model,
        materials,
        built,
        diagnostics,
    }
}

fn guard_view(request: &Value, case_index: usize) -> GuardView {
    let Prepared {
        model,
        materials,
        built,
        mut diagnostics,
    } = prepared(request);
    let load_case = &model.load_cases[case_index];
    let loads = build_load_case_primitive_loads(&model, load_case, &mut diagnostics);
    let application = prepare_loads(built.nodes.len(), built.pipes.len(), &loads);
    let bends = built
        .curved_bend_elements
        .iter()
        .map(|e| (e.pipe_index, e))
        .collect::<HashMap<_, _>>();
    let pipe_map = model
        .pipe_segments
        .iter()
        .enumerate()
        .map(|(i, p)| (p.id.as_str(), i))
        .collect::<HashMap<_, _>>();
    let material_map = materials
        .iter()
        .map(|m| (m.id.as_str(), m))
        .collect::<HashMap<_, _>>();
    let thermal = build_thermal_element_loads(
        &model,
        load_case,
        &material_map,
        &pipe_map,
        &built,
        &mut diagnostics,
    );
    let ledger = case_force_ledger(
        &model,
        &built,
        &application,
        &bends,
        &thermal,
        None,
        &load_case.id,
        &mut diagnostics,
    );
    assert!(!has_blocking(&diagnostics), "{diagnostics:?}");
    let force = ledger.finish(built.nodes.len() * DOF_PER_NODE).unwrap();
    let boundary = prepare_boundary(built.nodes.len(), &built.supports);
    let bodies = formation_bodies(&built);
    let rows = force.formation_rows();
    let restrained: HashSet<usize> = boundary.restrained_dofs.iter().copied().collect();
    let scales = row_scales(&rows, &bodies, &restrained);
    let labels = (0..built.nodes.len() * DOF_PER_NODE)
        .map(|dof| integrity_dof_label(&model, dof))
        .collect();
    GuardView {
        rows,
        scales,
        labels,
        force,
    }
}

fn twelfth(accumulator: &ExactAccumulator) -> f64 {
    accumulator.round().unwrap() / 12.0
}

// -------------------------------------------------- the recovery guard's view

/// One straight member end as R-b' sees it, recomputed in the test from the
/// published rows: q from the published bending rows, B from SP's
/// `bending_formation_bound` on the published displacements (translations
/// converted from mm), S*_moment from the case's published rows.
#[derive(Debug)]
struct EndView {
    member: String,
    end: &'static str,
    q: f64,
    bound: f64,
    s_star: f64,
    first: bool,
    resolved: bool,
    above_floor: bool,
}

impl EndView {
    fn rb(&self) -> bool {
        self.first && self.resolved
    }
    fn rb_prime(&self) -> bool {
        self.rb() && self.above_floor
    }
}

fn rb_view(request: &Value, envelope: &MechanicsEnvelope, case: &str) -> Vec<EndView> {
    let Prepared {
        model,
        materials,
        built,
        mut diagnostics,
    } = prepared(request);
    // The case's own modulus basis, as the product builds it.
    let load_case = model.load_cases.iter().find(|c| c.id == case).unwrap();
    let built = if load_case.modulus_basis_ref.is_some() {
        let (basis, _) =
            materials_for_modulus_basis(&model, &materials, load_case, &mut diagnostics).unwrap();
        build_model(&model, &basis, &mut diagnostics).unwrap()
    } else {
        built
    };
    let rows = case_rows(envelope, case);
    let mut u = vec![0.0; built.nodes.len() * DOF_PER_NODE];
    let kinds = [
        "global_nodal_displacement_x",
        "global_nodal_displacement_y",
        "global_nodal_displacement_z",
        "global_nodal_rotation_x",
        "global_nodal_rotation_y",
        "global_nodal_rotation_z",
    ];
    for row in &rows {
        if let Some(component) = kinds.iter().position(|k| *k == row.kind) {
            let node = node_index(&model, &row.entity_ref).unwrap();
            let scale = if row.unit == "mm" { 1e-3 } else { 1.0 };
            u[node * DOF_PER_NODE + component] = row.value * scale;
        }
    }
    let bodies = formation_bodies(&built);
    let curved: HashSet<String> = built
        .curved_bend_elements
        .iter()
        .map(|b| b.pipe_id.clone())
        .collect();
    let owned: Vec<ResultItem> = rows.iter().map(|r| (*r).clone()).collect();
    // O-13: every published arc is guarded, so its rows are in S* (2b lifted).
    let scales = formation_guard::action_scales(
        &owned,
        &formation_entity_bodies(&model, &built, &bodies),
        &HashSet::new(),
        &bodies,
    );
    let moment = |member: &str, component: &str, location: &str| {
        rows.iter()
            .find(|r| {
                r.kind.starts_with("element_local_bending_moment")
                    && r.entity_ref == member
                    && r.metadata
                        .as_ref()
                        .is_some_and(|m| m.component == component && m.location == location)
            })
            .map(|r| r.value)
    };
    let mut out = Vec::new();
    for pipe in built
        .pipes
        .iter()
        .filter(|p| !curved.contains(&p.element_id))
    {
        let bounds = pipe.bending_formation_bound(&u).unwrap();
        let s_star = bodies
            .body_of_node(pipe.node_i.index)
            .and_then(|b| scales.get(&b))
            .map_or(0.0, |s| s.moment);
        for (index, (end, location)) in [("i", "end_i"), ("j", "end_j")].into_iter().enumerate() {
            let (Some(y), Some(z)) = (
                moment(&pipe.element_id, "bending_moment_y", location),
                moment(&pipe.element_id, "bending_moment_z", location),
            ) else {
                continue;
            };
            let q = y.hypot(z);
            let bound = bounds[index];
            let mut first = ExactAccumulator::new();
            first.add(bound).unwrap();
            first.add_product(-formation_guard::CRITERION, q).unwrap();
            out.push(EndView {
                member: pipe.element_id.clone(),
                end,
                q,
                bound,
                s_star,
                first: first.signum() > 0,
                resolved: q > 1024.0 * bound,
                above_floor: q >= 2.0_f64.powi(-34) * s_star,
            });
        }
    }
    out
}

// ------------------------------------------------------------- builders

fn preview_model(id: &str) -> Value {
    json!({
        "schema_version": "0.7.0", "document_kind": "openpipestress.product_preview.model",
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
            "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "project": {"id": format!("invented:t3-s11g:{id}"), "units": {"length": "m", "force": "N", "angle": "rad",
            "pressure": "Pa", "temperature": "degC", "stress": "Pa"}},
        "nodes": [], "pipe_segments": [], "materials": [], "supports": [], "components": [],
        "load_cases": [{"id": "case", "label": id, "kind": "primitive_user_load", "primitive_loads": [],
            "provenance": INVENTED}],
        "combinations": []})
}

fn node(id: &str, x: f64, y: f64, z: f64) -> Value {
    json!({"id": id, "position": {"x": x, "y": y, "z": z}, "provenance": INVENTED})
}

/// Pipe with invented section OD 0.2 m, wall 0.01 m.
fn pipe(id: &str, from: &str, to: &str, y_reference: [f64; 3]) -> Value {
    json!({"id": id, "from": from, "to": to, "material": "material",
        "y_reference": {"x": y_reference[0], "y": y_reference[1], "z": y_reference[2]},
        "section": {"outside_diameter": {"value": 0.2, "unit": "m"}, "wall_thickness": {"value": 0.01, "unit": "m"}},
        "provenance": INVENTED})
}

/// Invented steel-like material: E = 200 GPa, G = 80 GPa, alpha = 1.2e-5 /degC.
fn material() -> Value {
    json!({"id": "material", "elastic_modulus": {"value": 200e9, "unit": "Pa"},
        "shear_modulus": {"value": 80e9, "unit": "Pa"},
        "thermal_expansion_coefficient": {"value": 1.2e-5, "unit": "1/degC"}, "provenance": INVENTED})
}

fn support(id: &str, node: &str, restraints: &[&str]) -> Value {
    json!({"id": id, "node": node, "family": "anchor", "restraints": restraints, "provenance": INVENTED})
}

const ALL: [&str; 6] = ["UX", "UY", "UZ", "RX", "RY", "RZ"];

fn nodal(id: &str, node: &str, direction: &str, value: f64) -> Value {
    let moment = direction.starts_with('R');
    json!({"id": id, "category": if moment { "concentrated_moment" } else { "concentrated_force" },
        "target": {"type": "node", "node": node}, "direction": direction,
        "magnitude": {"value": value, "unit": if moment { "N*m" } else { "N" }},
        "dimension": if moment { "moment" } else { "force" }, "provenance": INVENTED})
}

fn uniform(id: &str, pipe: &str, direction: &str, value: f64) -> Value {
    json!({"id": id, "category": "distributed_force", "target": {"type": "element", "pipe": pipe},
        "direction": direction, "magnitude": {"value": value, "unit": "N/m"},
        "dimension": "force_per_length", "provenance": INVENTED})
}

fn thermal(id: &str, pipe: &str, delta_t: f64) -> Value {
    json!({"id": id, "category": "thermal", "target": {"type": "element", "pipe": pipe},
        "direction": "global_x", "magnitude": {"value": delta_t, "unit": "degC"},
        "dimension": "temperature_interval", "provenance": INVENTED})
}

fn request_of(model: Value) -> Value {
    json!({"model": model, "materials": []})
}

fn rf_cancel_data() -> Value {
    serde_json::from_str(include_str!("../tests/fixtures/s11f/rf_cancel_cases.json")).unwrap()
}

pub(crate) fn rf_request(id: &str) -> Value {
    rf_cancel_data()["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["id"] == json!(id))
        .unwrap_or_else(|| panic!("{id}"))["request"]
        .clone()
}

fn rb_controls(id: &str) -> Value {
    let data: Value =
        serde_json::from_str(include_str!("../tests/fixtures/s11g/rb_controls.json")).unwrap();
    data["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["id"] == json!(id))
        .unwrap_or_else(|| panic!("{id}"))["request"]
        .clone()
}

// ================================================================ unit level

fn one_row(ledger: LoadLedger) -> FormationRow {
    ledger.finish(1).unwrap().formation_rows().remove(0)
}

/// Ruling 2 (ROOT's erratum to revision 2.1's first clause): the boundary.
/// B = T0 = 0 with A = 0 does not fire (T6's case); B = T0 = 0 with A_net != 0
/// fires; B = T0 = 0 with |A_se| > 12 Tf fires; B > 0 with B = T0 fires.
#[test]
fn ruling2_boundary_of_the_first_clause() {
    // (a) Two same-expression formed terms cancel: A = 0, n = 0, B = 0; S* = 0.
    let (a, b) = (1.3e6_f64, 0.3_f64);
    let mut ledger = LoadLedger::new();
    ledger.push_formed(
        "p",
        0,
        a * b,
        Formation::RoundedProduct { k: 1.0, a, b },
        0.0,
        true,
    );
    ledger.push_formed(
        "m",
        0,
        -(a * b),
        Formation::RoundedProduct { k: -1.0, a, b },
        0.0,
        true,
    );
    let row = one_row(ledger);
    assert!(row.formed && row.net_defect.is_zero() && row.self_equilibrated_defect.is_zero());
    let d = decide_row(&row, 0.0);
    assert_eq!((row.bound, d.t0), (0.0, 0.0), "precondition: B = T0 = 0");
    assert!(!d.fires, "B = T0 = 0 and A = 0 must not fire");
    // (b) A_net != 0 with n = 0 and S* = 0: an Exact term whose formula is 0.
    let mut ledger = LoadLedger::new();
    ledger.push_formed(
        "x",
        0,
        1e-300,
        Formation::Exact {
            scale: 1.0,
            scaled_intended: vec![],
        },
        0.0,
        false,
    );
    let row = one_row(ledger);
    let d = decide_row(&row, 0.0);
    assert!(row.bound == 0.0 && d.t0 == 0.0 && !row.net_defect.is_zero());
    assert!(d.fires, "B = T0 = 0 with A_net != 0 fires");
    // (c) |A_se| > 12 Tf with B = T0 = 0: a self-equilibrated Exact term whose
    // formula is 0 (P = |value|, Tf = RD(1e-9) 2^-10 P).
    let mut ledger = LoadLedger::new();
    ledger.push_formed(
        "s",
        0,
        2.0,
        Formation::Exact {
            scale: 1.0,
            scaled_intended: vec![],
        },
        0.0,
        true,
    );
    let row = one_row(ledger);
    let d = decide_row(&row, 0.0);
    assert!(row.bound == 0.0 && d.t0 == 0.0 && d.tf > 0.0 && row.net_defect.is_zero());
    assert!(d.fires, "B = T0 = 0 with |A_se| > 12 Tf fires");
    // (d) B > 0 with B = T0: an input 1.0 and a Bounded term of bound RD(1e-9).
    let mut ledger = LoadLedger::new();
    ledger.push("n", 0, 1.0);
    ledger.push_formed(
        "q",
        0,
        0.0,
        Formation::Bounded {
            bound: formation_guard::CRITERION,
        },
        0.0,
        false,
    );
    let row = one_row(ledger);
    let d = decide_row(&row, 0.0);
    assert!(
        row.bound > 0.0 && row.bound == d.t0,
        "precondition: B = T0 > 0"
    );
    assert!(d.fires, "B > 0 with B = T0 fires");
}

/// M18 (V1 DN-3): the criterion constant is RD(10^-9). A row whose exact
/// statistic is fl(1e-9) (strictly above 10^-9) fires; with the binary64
/// literal 1e-9 as the threshold constant it would not.
#[test]
fn m18_threshold_constant_is_rounded_down() {
    let literal = 1e-9_f64;
    // Precondition: the literal lies above 10^-9 and RD is one step below it.
    let mut above = ExactAccumulator::new();
    above.add_product(literal, 1e9).unwrap();
    above.add(-1.0).unwrap();
    assert!(above.signum() > 0, "fl(1e-9) > 10^-9");
    let mut below = ExactAccumulator::new();
    below.add_product(formation_guard::CRITERION, 1e9).unwrap();
    below.add(-1.0).unwrap();
    assert!(below.signum() <= 0, "RD(1e-9) <= 10^-9");
    assert_eq!(formation_guard::CRITERION.next_up(), literal);
    let mut ledger = LoadLedger::new();
    ledger.push("n", 0, 1.0);
    ledger.push_formed(
        "x",
        0,
        literal,
        Formation::Exact {
            scale: 1.0,
            scaled_intended: vec![],
        },
        0.0,
        false,
    );
    let row = one_row(ledger);
    assert_eq!(row.intended_net_lower, 1.0);
    let d = decide_row(&row, 0.0);
    assert!(
        d.fires,
        "defect fl(1e-9) of an intended net 1 exceeds 10^-9"
    );
}

/// R-b' at each clause's boundary (unit level; kills X4 and X7, which the
/// product rows cannot separate: every forecast row is either resolved far
/// above noise with B/q > 1e-6, or below the floor). q = 1 and S*_moment = 1
/// put the floor clause on the firing side throughout.
#[test]
fn member_group_clauses_at_their_boundaries() {
    use formation_guard::{member_group_fires as fires, CRITERION};
    assert!(!fires(1.0,1.0,CRITERION,1.0));
    assert!(fires(1.0,1.0,CRITERION.next_up(),1.0));
    // Repair removes the former resolution conjunct, including noise-level q.
    assert!(fires(1.0,1.0,0.5,1.0));
    let scale = 2f64.powi(34);
    assert!(fires(1.0,1.0,1e-6,scale));
    assert!(!fires(1.0,1.0,1e-6,scale.next_up()));
    // Gross error can shrink published q: q_hat is the floor operand.
    assert!(fires(0.0,1.0,1e-6,scale));
    assert!(fires(0.0,0.0,f64::INFINITY,0.0));
    let minimum = 2f64.powi(-988);
    assert!(fires(minimum,minimum,minimum,minimum));
    assert!(!fires(minimum,minimum,minimum,minimum.next_down()));
}

/// RV4-S2 (D21-1; kills RV-M1): the second test decides |A_net| + 12B >
/// 12 T0 exactly, with B inside the sum. A row with an input of 1 N, a
/// `Bounded` term of 0.5 N (bound B) and an `Exact` term whose formula is 0
/// (defect d = its value). With B = T0/4 (so the first clause, B >= T0, is
/// silent) and d = 0.9 T0, d lies in (T0 - B, T0]: the row fires by the
/// second test alone. The same defect with B = 0 is silent.
#[test]
fn d21_1_second_test_adds_the_bound_exactly() {
    let row_with = |bound: f64, defect: f64| {
        let mut ledger = LoadLedger::new();
        ledger.push("n", 0, 1.0);
        ledger.push_formed("b", 0, 0.5, Formation::Bounded { bound }, 0.0, false);
        ledger.push_formed(
            "x",
            0,
            defect,
            Formation::Exact {
                scale: 1.0,
                scaled_intended: vec![],
            },
            0.0,
            false,
        );
        one_row(ledger)
    };
    // T0 depends on the intended net (1.5 N) only, not on d or B.
    let t0 = decide_row(&row_with(0.0, 0.0), 0.0).t0;
    assert!(t0 > 0.0);
    let (bound, defect) = (t0 / 4.0, 0.9 * t0);
    let row = row_with(bound, defect);
    let d = decide_row(&row, 0.0);
    // Preconditions: the same T0; B and d exactly as authored; d in (T0 - B, T0].
    assert_eq!(d.t0, t0);
    assert_eq!(row.bound, bound);
    let mut check = row.net_defect.clone();
    check.add_product(-12.0, defect).unwrap();
    assert!(check.is_zero(), "A_net = 12 d");
    assert!(bound < t0 && defect <= t0 && defect + bound > t0);
    assert!(row.self_equilibrated_defect.is_zero() && row.cannot_bound_sources.is_empty());
    assert!(
        d.fires,
        "|A_net| + 12B > 12 T0 fires by the second test: {d:?}"
    );
    let silent = decide_row(&row_with(0.0, defect), 0.0);
    assert_eq!(silent.t0, t0);
    assert!(!silent.fires, "the same defect with B = 0 is below T0");
}

/// RV4-S3 (kills RV-M2): the exact-pressure operand bound is
/// RU(gamma_20 |t|), plus 2^-1074 (rounded upward) when the operand is
/// subnormal; it is never 0 for a nonzero operand.
#[test]
fn exact_pressure_operand_bound_is_gamma_20() {
    for value in [3.7e5, -2.5e-3, 1.0, -8.25e7] {
        let bound = exact_pressure_operand_bound(value);
        assert_eq!(bound, product_upward(gamma(20), value.abs()), "{value}");
        // gamma_20 > 20u, and RN(20u |t|) stays below gamma_20 |t|.
        assert!(
            bound > 0.0 && bound >= 20.0 * (f64::EPSILON / 2.0) * value.abs(),
            "{value}"
        );
    }
    assert_eq!(exact_pressure_operand_bound(0.0), 0.0);
    let subnormal = f64::from_bits(5);
    let bound = exact_pressure_operand_bound(subnormal);
    assert_eq!(
        bound,
        (product_upward(gamma(20), subnormal) + f64::from_bits(1)).next_up()
    );
    assert!(bound > f64::from_bits(1));
}

/// M10 (SF-1): the families are combined exactly. Two Exact terms carry
/// defects +lo_1 and +lo_2, two RoundedProduct terms -lo_1 and -lo_2 (lo_i
/// the error of fl(a_i b_i)), at a row whose intended net and scale are 0,
/// so the threshold is 0. The exact decision is silent; revision 1's
/// binary64 combination fl(round(A12)/12 + round(A1)) leaves a residual and
/// would fire.
#[test]
fn m10_families_are_combined_exactly() {
    let operands = [(1.7_f64, 2.3_f64), (1.1e-9_f64, 0.3_f64)];
    let mut ledger = LoadLedger::new();
    let mut twelve = ExactAccumulator::new();
    let mut plain = ExactAccumulator::new();
    for (i, &(a, b)) in operands.iter().enumerate() {
        let p = a * b;
        let lo = a.mul_add(b, -p);
        assert_ne!(lo, 0.0);
        // RoundedProduct: value p, defect -lo, intended p + lo.
        ledger.push_formed(
            format!("rp{i}"),
            0,
            p,
            Formation::RoundedProduct { k: 1.0, a, b },
            0.0,
            false,
        );
        // Exact: value -p, intended -p - lo, defect +lo.
        ledger.push_formed(
            format!("ex{i}"),
            0,
            -p,
            Formation::Exact {
                scale: 1.0,
                scaled_intended: vec![-p, -lo],
            },
            0.0,
            false,
        );
        twelve.add_product(12.0, lo).unwrap();
        plain.add(-lo).unwrap();
    }
    let row = one_row(ledger);
    assert!(row.net_defect.is_zero(), "the defects cancel exactly");
    assert!(row.twelve_intended_net.is_zero(), "the intended net is 0");
    // Precondition: the binary64 combination of the per-family parts is not 0.
    let combined = twelve.round().unwrap() / 12.0 + plain.round().unwrap();
    assert_ne!(
        combined, 0.0,
        "precondition: revision 1's combination leaves a residual"
    );
    let d = decide_row(&row, 0.0);
    assert_eq!(d.t0, 0.0);
    assert!(!d.fires, "the exact combination is silent");
}

/// T10 (revision 2.2): the routing predicates. `source_eligible` is main's
/// and does not see the finding (G-1); `needs_source_recovery` is true for a
/// Sensitive report, an erring attempt, or a load-row finding (G-2).
#[test]
fn t10_routing_predicates() {
    let finding = formation_guard::FormationFinding {
        guard: formation_guard::Guard::LoadRow,
        sentence: String::new(),
        fired: vec!["x".into()],
    };
    // B4/D1 (b): only an exact (composite) model selects source blocks.
    for captured in [false, true] {
        for exact in [false, true] {
            for nonlinear in [false, true] {
                for combinations in [false, true] {
                    assert_eq!(
                        source_eligible(captured, exact, nonlinear, combinations),
                        captured && exact && !nonlinear && !combinations
                    );
                }
            }
        }
    }
    for report_sensitive in [false, true] {
        for attempt_err in [false, true] {
            assert_eq!(
                needs_source_recovery(report_sensitive, attempt_err, None),
                report_sensitive || attempt_err
            );
            assert!(needs_source_recovery(
                report_sensitive,
                attempt_err,
                Some(&finding)
            ));
        }
    }
}

/// The no-op rule at unit level (M15's unit kill; T13b is the product one):
/// an integrity record that is not `CHECKS_PASSED` is left byte for byte.
#[test]
fn no_op_rule_leaves_a_non_passed_record_untouched() {
    let finding = formation_guard::FormationFinding {
        guard: formation_guard::Guard::Recovery,
        sentence: "S11-G recovery guard (R-b'): x".into(),
        fired: vec![],
    };
    for code in [
        "NUMERICAL_INTEGRITY_SENSITIVE",
        "NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED",
        "NUMERICAL_INTEGRITY_FAILED",
    ] {
        let original = diag(
            "diagnostic:numerical-integrity:c",
            code,
            "warning",
            "m",
            vec!["c".into()],
        );
        let mut record = original.clone();
        assert!(!formation_guard::demote(&mut record, &finding));
        assert_eq!(
            serde_json::to_string(&record).unwrap(),
            serde_json::to_string(&original).unwrap()
        );
    }
    let mut passed = diag(
        "diagnostic:numerical-integrity:c",
        "NUMERICAL_INTEGRITY_CHECKS_PASSED",
        "info",
        "m.",
        vec!["c".into()],
    );
    assert!(formation_guard::demote(&mut passed, &finding));
    assert_eq!(passed.code, "NUMERICAL_INTEGRITY_SENSITIVE");
    assert_eq!(passed.message, "m. S11-G recovery guard (R-b'): x");
    // Both guards: the load-row sentence stands; R-b' is then a no-op.
    let load_row = formation_guard::FormationFinding {
        guard: formation_guard::Guard::LoadRow,
        sentence: "S11-G formation-noise guard: y".into(),
        fired: vec![],
    };
    let mut diagnostics = vec![diag(
        "diagnostic:numerical-integrity:c",
        "NUMERICAL_INTEGRITY_CHECKS_PASSED",
        "info",
        "m.",
        vec!["c".into()],
    )];
    assert!(formation_guard::demote(&mut diagnostics[0], &load_row));
    let before = serde_json::to_string(&diagnostics).unwrap();
    assert!(!formation_guard::amend_integrity_report(
        &mut diagnostics,
        "diagnostic:numerical-integrity:c",
        &finding
    ));
    assert_eq!(serde_json::to_string(&diagnostics).unwrap(), before);
}

// ============================================================== product level

/// The captured entry refuses |x| >= 2^53 at capture until S-H lands.
fn entries(captured_refused: bool) -> Vec<Entry> {
    if captured_refused {
        vec![Entry::Typed]
    } else {
        vec![Entry::Captured, Entry::Typed]
    }
}

fn rf_captured_refused(id: &str) -> bool {
    rf_cancel_data()["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["id"] == json!(id))
        .unwrap()["captured_refused_at_capture"]
        == json!(true)
}

/// B4/D1 (b): the preview route attempts no retained-source recovery, so
/// neither entry publishes a retained-source diagnostic (revision 2.2's
/// captured-entry formation decline went with it).
fn assert_source_block_codes(envelope: &MechanicsEnvelope, _entry: Entry, label: &str) {
    assert_no_source_blocks(envelope, label);
    let integrity: Vec<_> = codes(envelope)
        .into_iter()
        .filter(|c| c.starts_with("NUMERICAL_INTEGRITY_"))
        .collect();
    assert_eq!(
        integrity,
        vec!["NUMERICAL_INTEGRITY_SENSITIVE".to_string()],
        "{label}"
    );
}

/// B4/D1 (b): no receipt and no retained-source diagnostic.
fn assert_no_source_blocks(envelope: &MechanicsEnvelope, label: &str) {
    let found: Vec<&Diagnostic> = envelope
        .diagnostics
        .iter()
        .filter(|d| d.code.starts_with("SOURCE_BLOCK_") || d.code.starts_with("SOURCE_RECOVERY"))
        .collect();
    assert!(found.is_empty(), "{label}: {found:?}");
    assert!(envelope.source_block_recovery.is_none(), "{label}");
}

/// A case demoted at the solve (`assert_demoted_at_the_solve`) is a
/// Sensitive report. B4/D1 (b): the preview route attempts no retained-source
/// recovery, so neither entry publishes a retained-source diagnostic.
fn assert_source_block_codes_at_the_solve(envelope: &MechanicsEnvelope, entry: Entry, label: &str) {
    assert_source_block_codes(envelope, entry, label);
}

/// T1: UDL-W1e8 (captured and typed, both modes). Precondition: the ordinary
/// report is Passed, and S1.RZ's exact net defect is V1's independent
/// Fraction figure 2.3594e-8 (S11G_CHECK check 1), about 48x its threshold,
/// with P = 0 (no floor on a UDL row). Pin: Sensitive on every view; the
/// only integrity code is SENSITIVE. Under K-D5's closure (DEL-04-01
/// kd5-member-relative-bound.md §§1, 4.2) the same defect is caught at the
/// solve: D5C-1 on the closed residual demotes S1:RZ (trigger about 96, twice
/// the guard's 48), so the report is Sensitive and the load-row guard is a
/// no-op. HELP_HUMAN, 2026-10-12 (W1 routing, agreed on T3's behalf): the
/// solve demotion is the expected outcome; B4/D1 (b) retired the captured
/// entry's retained-source attempt, so no retained-source diagnostic follows.
#[test]
fn t1_udl_w1e8_is_demoted_on_both_entries_and_modes() {
    let request = rf_request("RF-CANCEL-UDL-W1e8");
    let view = guard_view(&request, 0);
    let row = view.row("S1:RZ");
    let defect = twelfth(&row.net_defect).abs();
    assert!((defect / 2.3594e-8 - 1.0).abs() < 1e-3, "defect {defect:e}");
    assert_eq!(row.self_equilibrated_magnitude, 0.0);
    let decision = view.decision("S1:RZ");
    assert!(decision.fires && decision.net_ratio > 40.0, "{decision:?}");
    for entry in [Entry::Captured, Entry::Typed] {
        for mode in MODES {
            let label = format!("T1 {entry:?} {mode:?}");
            let envelope = solved(entry, &request, mode);
            assert_demoted_at_the_solve(&envelope, "case", "S1:RZ", &label);
            assert_source_block_codes_at_the_solve(&envelope, entry, &label);
        }
    }
}

/// T2: UDL-W1e80 (typed, both modes; the captured entry refuses it).
/// Precondition: S1.RZ's defect is V1's 2.6328e64 against a 1e-17 threshold.
/// Caught at the solve by K-D5's D5C-1 on the closed residual (§§1, 4.2);
/// an expected outcome under HELP_HUMAN's 2026-10-12 W1-routing ruling.
#[test]
fn t2_udl_w1e80_is_demoted_on_the_typed_entry() {
    let request = rf_request("RF-CANCEL-UDL-W1e80");
    let view = guard_view(&request, 0);
    let defect = twelfth(&view.row("S1:RZ").net_defect).abs();
    assert!((defect / 2.6328e64 - 1.0).abs() < 1e-3, "defect {defect:e}");
    assert!(view.decision("S1:RZ").fires);
    for mode in MODES {
        assert!(
            run(Entry::Captured, &request, mode).is_err(),
            "captured refuses 1e80"
        );
        let label = format!("T2 typed {mode:?}");
        let envelope = solved(Entry::Typed, &request, mode);
        assert_demoted_at_the_solve(&envelope, "case", "S1:RZ", &label);
        assert_source_block_codes(&envelope, Entry::Typed, &label);
    }
}

/// T3: UDL-W1e5 stays CHECKS_PASSED (V1: defect 1.94e-11, 0.0395 of the
/// threshold). Kills the a-priori bound (M4).
#[test]
fn t3_udl_w1e5_stays_checks_passed() {
    let request = rf_request("RF-CANCEL-UDL-W1e5");
    let view = guard_view(&request, 0);
    let row = view.row("S1:RZ");
    let defect = twelfth(&row.net_defect).abs();
    assert!(
        defect > 0.0 && (defect / 1.94e-11 - 1.0).abs() < 1e-2,
        "defect {defect:e}"
    );
    let decision = view.decision("S1:RZ");
    assert!(!decision.fires && decision.net_ratio < 0.1, "{decision:?}");
    for entry in [Entry::Captured, Entry::Typed] {
        for mode in MODES {
            let envelope = solved(entry, &request, mode);
            assert_eq!(
                integrity(&envelope, "case").code,
                "NUMERICAL_INTEGRITY_CHECKS_PASSED"
            );
            assert_not_demoted_by_s11g(&envelope, "case", "T3");
        }
    }
}

/// T4: nodal cancellation (G, n, -G) carries no formation defect: the binary64
/// fold of the terms differs from the exact net, and no row has a formed
/// term, so the load-row guard cannot fire. (M3, a bound on input terms, is
/// equivalent here because no row is formed; T4b kills it.)
#[test]
fn t4_nodal_cancellation_is_never_demoted_by_the_load_row_guard() {
    for id in ["RF-CANCEL-F-G1e80-GnG", "RF-CANCEL-F-G1e8-GnG"] {
        let request = rf_request(id);
        let view = guard_view(&request, 0);
        let mut folded = vec![0.0; view.force.len()];
        for term in view.force.terms() {
            if let open_pipe_stress_frame_kernel::load_ledger::ForceTermKind::Term(v) = term.kind {
                folded[term.dof] += v;
            }
        }
        assert!(
            (0..view.force.len()).any(|d| folded[d].to_bits() != view.force.values()[d].to_bits()),
            "{id}: precondition, the fold equals the net"
        );
        assert!(!view.force.has_formation_records() && view.rows.iter().all(|r| !r.formed));
        for entry in entries(rf_captured_refused(id)) {
            for mode in MODES {
                let envelope = solved(entry, &request, mode);
                assert!(
                    !integrity(&envelope, "case").message.contains(LOAD_ROW),
                    "{id} {entry:?} {mode:?}"
                );
            }
        }
    }
}

/// T4b (M3's product kill; T4's nodal-only rows have no formed term, so M3 is
/// equivalent there): a 2 m cantilever with an axial w = 3e8 N/m (global x),
/// whose formed fixed-end terms are exact in binary64 (q L (1 - 1/2) and
/// q L / 2 = 3e8 N; the transverse and rotation terms are zero), cancelled at
/// the tip by an authored -3e8 N on UX, with a separate 0.2 N on UX. Every
/// formed defect is exactly zero and inputs carry no bound, so the tip UX
/// row's statistic is zero and the load-row guard stays silent; a bound of
/// u|t| on the inputs (M3) exceeds that row's threshold (about 1e-9 * 0.2)
/// and fires. (A transverse load cannot serve: SP's rotation coefficient
/// fl(fl(-1/3) + 1/4) is never exactly -1/12.)
#[test]
fn t4b_exact_formed_terms_cancelled_by_inputs_stay_silent() {
    let mut model = preview_model("t4b");
    model["nodes"] = json!([node("root", 0.0, 0.0, 0.0), node("tip", 2.0, 0.0, 0.0)]);
    model["pipe_segments"] = json!([pipe("pipe", "root", "tip", [0.0, 1.0, 0.0])]);
    model["materials"] = json!([material()]);
    model["supports"] = json!([support("anchor", "root", &ALL)]);
    model["load_cases"][0]["primitive_loads"] = json!([
        uniform("udl", "pipe", "global_x", 3e8),
        nodal("ux", "tip", "global_x", -3e8),
        nodal("ux-net", "tip", "global_x", 0.2),
    ]);
    let request = request_of(model);
    let view = guard_view(&request, 0);
    let row = view.row("tip:UX");
    let terms: Vec<_> = view
        .force
        .terms()
        .iter()
        .filter(|t| t.dof == row.dof)
        .collect();
    assert!(
        row.formed,
        "precondition: the tip UX row carries a formed term: {terms:?}"
    );
    assert_eq!(
        row.net_defect.signum(),
        0,
        "precondition: its formed term is exact: {row:?} {terms:?}"
    );
    assert_eq!(row.bound, 0.0, "an input term carries no bound");
    assert!(
        row.intended_net_lower > 0.0 && row.intended_net_lower <= 0.2,
        "{row:?}"
    );
    assert!(!view.any_fires());
    for entry in [Entry::Captured, Entry::Typed] {
        for mode in MODES {
            let label = format!("T4b {entry:?} {mode:?}");
            let envelope = solved(entry, &request, mode);
            assert_ordinary_passed(&envelope, "case", &label);
            assert_not_demoted_by_s11g(&envelope, "case", &label);
        }
    }
}

/// P1's probe A: a 2 m cantilever (OD 0.2 m, wall 0.01 m, E = 200 GPa,
/// G = 80 GPa) with three uniform global-y loads in the given order.
fn probe_a(g: f64, order: [f64; 3]) -> Value {
    let mut model = preview_model("probe-a");
    model["nodes"] = json!([node("root", 0.0, 0.0, 0.0), node("tip", 2.0, 0.0, 0.0)]);
    model["pipe_segments"] = json!([pipe("pipe", "root", "tip", [0.0, 1.0, 0.0])]);
    model["materials"] = json!([material()]);
    model["supports"] = json!([support("anchor", "root", &ALL)]);
    model["load_cases"][0]["primitive_loads"] = json!(order
        .iter()
        .enumerate()
        .map(|(i, &f)| uniform(
            &format!("udl:{i}"),
            "pipe",
            "global_y",
            if f == 0.0 { 0.3 } else { f * g }
        ))
        .collect::<Vec<_>>());
    request_of(model)
}

/// T5: probe A, (G, 0.3, -G) and (0.3, G, -G) at G = 1e8 and 1e80. The G
/// terms' own defects are nonzero and cancel exactly (same expression,
/// negated operand), so the load-row statistic is the 0.3 term's alone and
/// S11-G adds nothing. Kills Sum|eps| (M6) and the a-priori bound (M4).
///
/// K-D5 (DEL-04-01 kd5-member-relative-bound.md §§1, 3.1-3.3): at G = 1e8 the
/// certified rows pass: CHECKS_PASSED. At G = 1e80 the intended net 0.3 N/m
/// sits under a cancellation of about 3e80, beyond what p = 128 with the
/// load enclosure radii (2^-120 level, §1's table) and the evaluation radius
/// can certify: every group has B far above 1e-9 q (B ~ 1e48 against q ~ 1),
/// so the unified predicate demotes the case with its sentence. Its circular
/// maximum is named unavailable and its signed rows stay published (§3.2);
/// before the summary fix this case was refused with
/// ELEMENT_FORCE_RECOVERY_FAILED, which kills that regression.
#[test]
fn t5_probe_a_same_expression_defects_cancel() {
    for g in [1e8, 1e80] {
        for order in [[1.0, 0.0, -1.0], [0.0, 1.0, -1.0]] {
            let request = probe_a(g, order);
            let Prepared { built, .. } = prepared(&request);
            let pipe = &built.pipes[0];
            let mut sum = vec![ExactAccumulator::new(); ELEMENT_DOF];
            let mut any_nonzero = false;
            for sign in [1.0, -1.0] {
                let load = SpannedGlobalUniformLoad::full([0.0, sign * g, 0.0]).unwrap();
                let (values, formations) = pipe
                    .equivalent_global_nodal_loads_with_spans_formed(&[load])
                    .unwrap();
                for (slot, formation) in formations.iter().enumerate() {
                    let Formation::Exact {
                        scale,
                        scaled_intended,
                    } = formation
                    else {
                        panic!()
                    };
                    let mut single = ExactAccumulator::new();
                    single.add_product(*scale, values[slot]).unwrap();
                    sum[slot].add_product(*scale, values[slot]).unwrap();
                    for &c in scaled_intended {
                        single.add(-c).unwrap();
                        sum[slot].add(-c).unwrap();
                    }
                    any_nonzero |= !single.is_zero();
                }
            }
            assert!(
                any_nonzero,
                "G={g}: precondition, the G terms carry defects"
            );
            assert!(
                sum.iter().all(ExactAccumulator::is_zero),
                "G={g}: the G defects cancel"
            );
            for entry in entries(g >= 9.0e15) {
                for mode in MODES {
                    let label = format!("G={g} {order:?} {entry:?} {mode:?}");
                    let envelope = solved(entry, &request, mode);
                    let d = integrity(&envelope, "case");
                    assert!(!d.message.contains(LOAD_ROW), "{label}");
                    assert!(d.message.contains("quality: Passed"), "{label}");
                    if g < 9.0e15 {
                        assert_eq!(d.code, "NUMERICAL_INTEGRITY_CHECKS_PASSED", "{label}");
                        assert_not_demoted_by_s11g(&envelope, "case", "T5");
                    } else {
                        assert_demoted(&envelope, "case", RECOVERY, &label);
                        assert!(
                            envelope.diagnostics.iter().any(|d| d.code
                                == "PREVIEW_STRESS_MAXIMUM_UNAVAILABLE"
                                && d.affected_refs == vec!["case".to_string(), "pipe".to_string()]),
                            "{label}: the maximum is named unavailable"
                        );
                        assert!(
                            envelope.results.iter().any(|r| r.kind == "element_local_bending_moment_z"),
                            "{label}: the signed rows stay published"
                        );
                        // Preview-physics renders here and retires the summary
                        // row, so the withheld summary is not named (F3).
                        assert!(
                            !envelope.diagnostics.iter().any(|d| d.id.ends_with(":summary-unavailable")),
                            "{label}: no summary notice on the preview-physics route"
                        );
                    }
                }
            }
        }
    }
}

/// A collinear skew pair n0-n1-n2 along (3, 1.7, 0.4) (n2 = 2 n1, so both
/// held directions are bit-identical), anchored at n0 and n2, with the same
/// temperature interval on each member (or only on the first).
fn skew_pair(both: bool) -> Value {
    let mut model = preview_model("skew-pair");
    model["nodes"] = json!([
        node("n0", 0.0, 0.0, 0.0),
        node("n1", 3.0, 1.7, 0.4),
        node("n2", 6.0, 3.4, 0.8)
    ]);
    model["pipe_segments"] = json!([
        pipe("p1", "n0", "n1", [0.0, 0.0, 1.0]),
        pipe("p2", "n1", "n2", [0.0, 0.0, 1.0])
    ]);
    model["materials"] = json!([material()]);
    model["supports"] = json!([support("a0", "n0", &ALL), support("a2", "n2", &ALL)]);
    let mut loads = vec![thermal("hot:1", "p1", 40.0)];
    if both {
        loads.push(thermal("hot:2", "p2", 40.0));
    }
    model["load_cases"][0]["primitive_loads"] = json!(loads);
    request_of(model)
}

/// T6 (and ruling 2's product-level kill): equal thermal axial loads on a
/// collinear skew pair. Each end term's defect is nonzero (shown on the
/// one-member variant); at n1 they cancel exactly, the intended net and the
/// free-row scale are 0, so B = T0 = 0 there: a first clause without ROOT's
/// B > 0 guard would fire. CHECKS_PASSED.
#[test]
fn t6_collinear_skew_pair_cancels_signed_defects() {
    let single = guard_view(&skew_pair(false), 0);
    assert!(
        ["n1:UX", "n1:UY", "n1:UZ"]
            .iter()
            .any(|l| !single.row(l).self_equilibrated_defect.is_zero()),
        "precondition: an end term carries a defect"
    );
    let request = skew_pair(true);
    let view = guard_view(&request, 0);
    for label in ["n1:UX", "n1:UY", "n1:UZ"] {
        let row = view.row(label);
        assert!(
            row.self_equilibrated_defect.is_zero() && row.net_defect.is_zero(),
            "{label}"
        );
        let d = view.decision(label);
        assert_eq!(
            (row.bound, d.t0),
            (0.0, 0.0),
            "{label}: precondition, B = T0 = 0"
        );
    }
    for entry in [Entry::Captured, Entry::Typed] {
        for mode in MODES {
            let envelope = solved(entry, &request, mode);
            assert_eq!(
                integrity(&envelope, "case").code,
                "NUMERICAL_INTEGRITY_CHECKS_PASSED",
                "T6 {entry:?} {mode:?}"
            );
            assert_not_demoted_by_s11g(&envelope, "case", "T6");
        }
    }
}

/// V1's collinear runs (`probe_thermal_skew`, `probe_sf4`): a straight run
/// from the origin to `end` through the given station fractions (positions
/// rounded to 6 decimals as V1 did), anchored at both ends, with the same
/// temperature interval on every member.
fn collinear_run(end: [f64; 3], stations: &[f64]) -> Value {
    let mut model = preview_model("collinear");
    let round6 = |v: f64| (v * 1e6).round() / 1e6;
    model["nodes"] = json!(stations
        .iter()
        .enumerate()
        .map(|(i, &t)| node(
            &format!("s{i}"),
            round6(t * end[0]),
            round6(t * end[1]),
            round6(t * end[2])
        ))
        .collect::<Vec<_>>());
    let last = stations.len() - 1;
    model["pipe_segments"] = json!((0..last)
        .map(|i| pipe(
            &format!("m{i}"),
            &format!("s{i}"),
            &format!("s{}", i + 1),
            [0.0, 0.0, 1.0]
        ))
        .collect::<Vec<_>>());
    model["materials"] = json!([material()]);
    model["supports"] = json!([
        support("a0", "s0", &ALL),
        support("an", &format!("s{last}"), &ALL)
    ]);
    model["load_cases"][0]["primitive_loads"] = json!((0..last)
        .map(|i| thermal(&format!("t:{i}"), &format!("m{i}"), 75.0))
        .collect::<Vec<_>>());
    request_of(model)
}

/// T6a (SF-4): V1's collinear runs stay silent. (V1's legacy pressure-thrust
/// run was retired with legacy pressure, U3.) Without
/// the floor the self-equilibrated statistic is at least 1e8 on the runs V1
/// found firing (computed here from the ledger rows); with the floor every
/// row is below 1e-3 of its threshold, and S11-G adds no sentence. Kills
/// dropping the floor (M11).
///
/// K-D5 (DEL-04-01 kd5-member-relative-bound.md §§1, 3.2): the two skew runs
/// V1 found firing are now Sensitive at the solve. Fully restrained and
/// uniformly heated, their true rotations are only the residue of the
/// binary64 node coordinates (body rotation scale about 2e-17 to 5e-17 rad).
/// The closure forms the intended thermal loads at p, so D5C-1 now compares
/// the published rotations with the intended system's, and they differ by a
/// large fraction of that scale (trigger 7e8 and 1.1e9 at s4:RZ and s3:RZ).
/// The load-row guard is still silent, so the demotion is K-D5's own. The
/// axis-aligned and 45-degree runs stay CHECKS_PASSED. HELP_HUMAN,
/// 2026-10-12 (T6a): this behaviour is accepted. The unchanged D5C-1 rule's
/// Passed -> Sensitive demotion is fail-safe and moves no realistic, RV29,
/// W1 or RV30 case-mode; no displacement-noise floor is added.
#[test]
fn t6a_collinear_runs_are_silent_with_the_floor() {
    let runs: [([f64; 3], &[f64], bool); 4] = [
        ([12.0, 5.0, 0.0], &[0.0, 0.13, 0.4, 0.55, 0.81, 1.0], true),
        ([10.0, 3.7, 2.2], &[0.0, 0.3, 0.55, 0.7, 1.0], true),
        ([6.0, 6.0, 0.0], &[0.0, 0.25, 0.5, 0.75, 1.0], false),
        // Axis-aligned: FK's normalization gives bit-identical unit vectors
        // here (V1's emulation did not), so there is no noise to silence.
        ([9.0, 0.0, 0.0], &[0.0, 0.13, 0.4, 0.55, 0.81, 1.0], false),
    ];
    let solve_rows: [Option<&str>; 4] = [Some("s4:RZ"), Some("s3:RZ"), None, None];
    for ((end, stations, fires_without_floor), solve_row) in runs.into_iter().zip(solve_rows) {
        let request = collinear_run(end, stations);
        let view = guard_view(&request, 0);
        let mut unfloored = 0.0_f64;
        let mut floored = 0.0_f64;
        for row in &view.rows {
            let d = decide_row(row, view.scales[&row.dof]);
            assert!(!d.fires, "{end:?}: {:?}", d.reason);
            let se = twelfth(&row.self_equilibrated_defect).abs();
            unfloored = unfloored.max(if d.t0 > 0.0 {
                se / d.t0
            } else if se > 0.0 {
                f64::INFINITY
            } else {
                0.0
            });
            floored = floored.max(d.self_equilibrated_ratio.max(d.net_ratio));
        }
        if fires_without_floor {
            assert!(
                unfloored >= 1e8,
                "{end:?}: precondition, unfloored statistic {unfloored:e}"
            );
        }
        assert!(floored < 1e-3, "{end:?}: floored statistic {floored:e}");
        for entry in [Entry::Captured, Entry::Typed] {
            for mode in MODES {
                let label = format!("T6a {end:?} {entry:?} {mode:?}");
                let envelope = solved(entry, &request, mode);
                match solve_row {
                    Some(row) => assert_demoted_at_the_solve(&envelope, "case", row, &label),
                    None => assert_eq!(
                        integrity(&envelope, "case").code,
                        "NUMERICAL_INTEGRITY_CHECKS_PASSED",
                        "{label}"
                    ),
                }
                assert_not_demoted_by_s11g(&envelope, "case", "T6a");
            }
        }
        eprintln!(
            "T6a {end:?}: unfloored {unfloored:e}, floored {floored:e}"
        );
    }
}

/// V1's DB-1 counterexample: S0 (0,0,0), S2 (5,0,0), S3 (3,-2,0), S4 (3,2,0)
/// anchored; S1 (3,0,0) with free translations and restrained rotations. A
/// (S0-S1, L = 3) and B (S1-S2, L = 2) carry uniform global-y loads q_A and
/// q_B; C (S3-S1) and D (S1-S4) carry the same temperature interval (thermal
/// axial load N each, meeting and cancelling at S1); a nodal 1e-3 N at S1.UY.
pub(crate) fn db1(delta_t: f64) -> Value {
    let mut model = preview_model("db1");
    model["nodes"] = json!([
        node("S0", 0.0, 0.0, 0.0),
        node("S1", 3.0, 0.0, 0.0),
        node("S2", 5.0, 0.0, 0.0),
        node("S3", 3.0, -2.0, 0.0),
        node("S4", 3.0, 2.0, 0.0)
    ]);
    model["pipe_segments"] = json!([
        pipe("A", "S0", "S1", [0.0, 1.0, 0.0]),
        pipe("B", "S1", "S2", [0.0, 1.0, 0.0]),
        pipe("C", "S3", "S1", [1.0, 0.0, 0.0]),
        pipe("D", "S1", "S4", [1.0, 0.0, 0.0])
    ]);
    model["materials"] = json!([material()]);
    model["supports"] = json!([
        support("a0", "S0", &ALL),
        support("a2", "S2", &ALL),
        support("a3", "S3", &ALL),
        support("a4", "S4", &ALL),
        support("r1", "S1", &["RX", "RY", "RZ"])
    ]);
    model["load_cases"][0]["primitive_loads"] = json!([
        uniform("load:A", "A", "global_y", 100000000.1),
        uniform("load:B", "B", "global_y", -150000000.15),
        thermal("thermal:C", "C", delta_t),
        thermal("thermal:D", "D", delta_t),
        nodal("nodal:S1", "S1", "global_y", 1e-3)
    ]);
    request_of(model)
}

/// T6b (DB-1): the floor never hides a net formation defect. At S1.UY, A's
/// fixed-end term carries a real defect (V1: -1.49e-8, 14901x the unfloored
/// threshold); two thermal members meeting there raise revision 2's
/// whole-row floor so that it would be silent (0.0076 at N ~ 1e6, 0.76 at
/// N ~ 1e4, computed here from the ledger rows). Sensitive, naming S1:UY.
/// Kills the whole-row floor (M17). Under K-D5's closure (DEL-04-01
/// kd5-member-relative-bound.md §§1, 4.2) the net defect is caught at the
/// solve: D5C-1 on the closed residual demotes S1:UY (trigger about 3e4), and
/// the floor never enters that rule. HELP_HUMAN, 2026-10-12 (W1 routing,
/// agreed on T3's behalf): the solve demotion, and on the captured entry the
/// retained-source attempt that declines the uniform-load producer, are
/// expected outcomes.
#[test]
fn t6b_floor_never_hides_a_net_defect() {
    // E A alpha = 200e9 * pi (0.1^2 - 0.09^2) * 1.2e-5 = 14325.7 N/degC.
    for delta_t in [69.806, 0.69806] {
        let request = db1(delta_t);
        let view = guard_view(&request, 0);
        let row = view.row("S1:UY");
        let net = twelfth(&row.net_defect);
        assert!((net / -1.49e-8 - 1.0).abs() < 1e-2, "A_net/12 = {net:e}");
        let d = view.decision("S1:UY");
        assert!(d.fires && d.net_ratio > 1e4, "{d:?}");
        // Revision 2's whole-row floor: |A_net + A_se| / (12 Tf) < 1.
        let whole = (twelfth(&row.net_defect) + twelfth(&row.self_equilibrated_defect)).abs();
        assert!(
            whole / d.tf < 1.0,
            "precondition: the whole-row floor would be silent ({})",
            whole / d.tf
        );
        for entry in [Entry::Captured, Entry::Typed] {
            for mode in MODES {
                let label = format!("T6b dT={delta_t} {entry:?} {mode:?}");
                let envelope = solved(entry, &request, mode);
                assert_demoted_at_the_solve(&envelope, "case", "S1:UY", &label);
                assert_source_block_codes_at_the_solve(&envelope, entry, &label);
            }
        }
    }
}

/// I4's pinned INPLANE formation rows (S11-F `FORMATION_PINS`): (case, member,
/// location, mode, [My, Mz]). K-D5 (DEL-04-01 kd5-member-relative-bound.md
/// §§3.1, 3.4) publishes them as certified recoveries: each is now exactly the
/// intended +/-1e-8 N*m in both modes (main published 0.99999937e-8 to
/// 1.0000058e-8, mode-dependent).
const INPLANE_PINS: &[(&str, &str, &str, &str, [f64; 2])] = &[
    (
        "RF-CANCEL-F-G1e80-GnG-INPLANE",
        "M1",
        "end_j",
        "dense_scrutiny",
        [0.0, 1e-8],
    ),
    (
        "RF-CANCEL-F-G1e80-GnG-INPLANE",
        "M1",
        "end_j",
        "sparse_interactive",
        [0.0, 1e-8],
    ),
    (
        "RF-CANCEL-F-G1e80-GnG-INPLANE",
        "M2",
        "end_i",
        "dense_scrutiny",
        [0.0, -1e-8],
    ),
    (
        "RF-CANCEL-F-G1e80-GnG-INPLANE",
        "M2",
        "end_i",
        "sparse_interactive",
        [0.0, -1e-8],
    ),
    (
        "RF-CANCEL-M-G1e80-GnG-INPLANE",
        "M2",
        "end_i",
        "dense_scrutiny",
        [0.0, -1e-8],
    ),
    (
        "RF-CANCEL-M-G1e80-GnG-INPLANE",
        "M2",
        "end_i",
        "sparse_interactive",
        [0.0, -1e-8],
    ),
    (
        "RF-CANCEL-M-G1e80-GnG-INPLANE",
        "M2",
        "end_j",
        "dense_scrutiny",
        [0.0, 1e-8],
    ),
    (
        "RF-CANCEL-M-G1e80-GnG-INPLANE",
        "M2",
        "end_j",
        "sparse_interactive",
        [0.0, 1e-8],
    ),
];

fn element_pair(envelope: &MechanicsEnvelope, member: &str, location: &str) -> [f64; 2] {
    let value = |component: &str| {
        case_rows(envelope, "case")
            .into_iter()
            .find(|r| {
                r.kind.starts_with("element_local_")
                    && r.entity_ref == member
                    && r.metadata
                        .as_ref()
                        .is_some_and(|m| m.component == component && m.location == location)
            })
            .unwrap_or_else(|| panic!("{member} {component} {location}"))
            .value
    };
    [value("bending_moment_y"), value("bending_moment_z")]
}

/// T11 (B-1; R-b' was the only INPLANE catch after S11-F), and ruling 1's
/// "R-b' alone" test. F- and M-G1e80-GnG-INPLANE, typed, both modes.
/// Precondition: no formed term (the load-row guard cannot fire), the
/// ordinary report is Passed, and R-b''s retired binary64 statistic still
/// fires on I4's ends. K-D5's repair (DEL-04-01 kd5-member-relative-bound.md
/// §§3.1-3.4, 4.2) publishes I4's rows as certified recoveries, exactly the
/// intended +/-1e-8 N*m, inside R1's exact intervals: the unified predicate
/// passes them, so the case is CHECKS_PASSED with no S11-G sentence (one of
/// §4.2's Sensitive -> Passed moves). The protective intent is kept by the
/// repaired values themselves (pinned bits and R1's intervals).
#[test]
fn t11_inplane_rows_are_demoted_by_rb_prime_alone() {
    for id in [
        "RF-CANCEL-F-G1e80-GnG-INPLANE",
        "RF-CANCEL-M-G1e80-GnG-INPLANE",
    ] {
        let request = rf_request(id);
        let view = guard_view(&request, 0);
        assert!(
            !view.force.has_formation_records(),
            "{id}: every term is an input"
        );
        for mode in MODES {
            let label = format!("T11 {id} {mode:?}");
            let envelope = solved(Entry::Typed, &request, mode);
            assert_ordinary_passed(&envelope, "case", &label);
            let ends = rb_view(&request, &envelope, "case");
            let pins: Vec<_> = INPLANE_PINS
                .iter()
                .filter(|p| p.0 == id && p.3 == mode.as_str())
                .collect();
            assert_eq!(pins.len(), 2);
            for pin in &pins {
                let end = if pin.2 == "end_i" { "i" } else { "j" };
                let e = ends
                    .iter()
                    .find(|e| e.member == pin.1 && e.end == end)
                    .unwrap();
                assert!(
                    e.rb_prime(),
                    "{label}: precondition, R-b' fires on {}.{end}: {e:?}",
                    pin.1
                );
                assert_eq!(
                    element_pair(&envelope, pin.1, pin.2).map(f64::to_bits),
                    pin.4.map(f64::to_bits),
                    "{label}: pinned bits"
                );
            }
            let case = rf_cancel_data()["cases"]
                .as_array()
                .unwrap()
                .iter()
                .find(|c| c["id"] == json!(id))
                .unwrap()
                .clone();
            let (breaches, checked) = rf_breaches(&case, &envelope);
            assert!(!checked.is_empty() && breaches.is_empty(), "{label}: {breaches:?}");
            assert_eq!(
                integrity(&envelope, "case").code,
                "NUMERICAL_INTEGRITY_CHECKS_PASSED",
                "{label}"
            );
            assert_not_demoted_by_s11g(&envelope, "case", &label);
        }
    }
}

/// T12 (B-1): RF-LARGE-CONT-n00100-AX (captured, both modes), a realistic
/// continuous beam: R-b's two clauses hold on some end from the product's
/// published values, so the two paths differ only by R-b''s floor clause;
/// every such end is below 2^-34 S*_moment. CHECKS_PASSED. Kills shipping
/// R-b (M13).
#[test]
fn t12_accurate_small_moment_rows_below_the_floor_stay_passed() {
    let request = rb_controls("RF-LARGE-CONT-n00100-AX");
    for mode in MODES {
        let envelope = solved(Entry::Captured, &request, mode);
        let ends = rb_view(&request, &envelope, "case");
        let rb: Vec<_> = ends.iter().filter(|e| e.rb()).collect();
        assert!(
            !rb.is_empty(),
            "{mode:?}: precondition, R-b fires somewhere"
        );
        assert!(ends.iter().all(|e| !e.rb_prime()), "{mode:?}: {rb:?}");
        assert_eq!(
            integrity(&envelope, "case").code,
            "NUMERICAL_INTEGRITY_CHECKS_PASSED",
            "{mode:?}"
        );
        assert_not_demoted_by_s11g(&envelope, "case", "T12");
        eprintln!(
            "T12 {mode:?}: R-b ends {:?}",
            rb.iter()
                .map(|e| format!(
                    "{}.{} q/S*={:e} B/q={:e}",
                    e.member,
                    e.end,
                    e.q / e.s_star,
                    e.bound / e.q
                ))
                .collect::<Vec<_>>()
        );
    }
}

/// T13 (B-1): the committed request `load_reference_fallback_uz`, already
/// Sensitive, through its producer (captured entry, pretty JSON plus a
/// newline), against the committed raw file in both modes (the no-op rule).
/// R-b's two clauses hold at end i (the design's precondition, asserted);
/// R-b' is silent there anyway, below its floor.
///
/// K-D5 (DEL-04-01 kd5-member-relative-bound.md §§3.2, 3.4, 6) changed three
/// declared things on this already-Sensitive case against main's bytes: the
/// two linear limitation sentences, the current maximum declaration literals,
/// and the fallback maximum's enclosure (its arithmetic transport widens it;
/// the new enclosure contains main's). Every diagnostic, numerical-quality
/// entry and result row stayed byte-identical. The committed raw file, which
/// result_export's suites and the base reader corpus consume, is regenerated
/// from this producer, so the check is now byte identity with it.
/// `T13_FALLBACK_UZ_WRITE=1` regenerates the two committed raw files from this
/// producer (every other assertion still runs first); a rerun without it then
/// checks the new bytes.
///
/// RF §5.5: the case's source attempt used to decline at the 0.4 replay
/// screen for work alone, which metered charging lifts. Its request now also
/// carries a zero full-span uniform member load (`uniform:0`, 0 N/m). That
/// load is a fixture control, not a modelling input: it exists only to force
/// the retained-source method's named decline at source closure ("non-nodal
/// load producer present"), independent of work, so the fixture stays an
/// ordinary-only sensitive pressure-2 raw (role precondition, asserted).
/// Every mechanics row is unchanged by that load. If a later unit lifts the
/// non-nodal decline, the role assertion fails first and names this reason.
#[test]
fn t13_committed_fallback_uz_is_byte_identical() {
    // B7b: the result_export fixture's request is read through the shared
    // re-authoring (a no-op once that fixture is itself 0.7.0).
    let request: Value = crate::b7b_admission_tests::reauthor_current_request(serde_json::from_str(include_str!(
        "../../reporting/result_export/tests/fixtures/load_reference_fallback_uz.request.json"
    ))
    .unwrap());
    for (mode, committed) in [
        (PreviewSolverMode::SparseInteractive, include_str!("../../reporting/result_export/tests/fixtures/load_reference_fallback_uz-sparse_interactive.raw.json")),
        (PreviewSolverMode::DenseScrutiny, include_str!("../../reporting/result_export/tests/fixtures/load_reference_fallback_uz-dense_scrutiny.raw.json")),
    ] {
        let envelope = run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap();
        // The role (RF §5.5): an ordinary pressure-2 publication whose source
        // attempt declines by name, not by work.
        assert_eq!(envelope.producer.semantic_contract_id, "openpipestress.result_semantics/0.3.0/pressure-2", "{mode:?}");
        assert!(
            envelope.diagnostics.iter().any(|d| d.code == "SOURCE_BLOCK_RECOVERY_UNAVAILABLE"
                && d.message.contains("helper_stage: SourceClosure, error: Unsupported(\"non-nodal load producer present\")")),
            "{mode:?}: precondition, the source attempt declines by name at source closure. The request's zero \
             `uniform:0` load is a fixture control that exists only to force this non-nodal decline, so the \
             fixture stays an ordinary-only sensitive pressure-2 raw (RF design §5.5); if the source method now \
             admits it, choose another work-independent decline for this fixture rather than re-pinning it"
        );
        let sensitive: Vec<_> = envelope
            .diagnostics
            .iter()
            .filter(|d| d.code == "NUMERICAL_INTEGRITY_SENSITIVE")
            .collect();
        assert!(!sensitive.is_empty(), "{mode:?}: precondition, the case is already Sensitive");
        assert!(sensitive.iter().all(|d| !d.message.contains("S11-G")));
        // The design's precondition (RV4-N10): R-b's two clauses hold at end
        // i, so without the no-op rule (and R-b''s floor) the paths differ.
        let case_id = request["model"]["load_cases"][0]["id"].as_str().unwrap();
        let ends = rb_view(&request, &envelope, case_id);
        assert!(
            ends.iter().any(|e| e.end == "i" && e.rb()),
            "{mode:?}: precondition, R-b holds at end i: {ends:?}"
        );
        let text = format!("{}\n", serde_json::to_string_pretty(&envelope).unwrap());
        // The current maximum declaration (§3.2) and the §3.4 sentences.
        let current: Value = serde_json::from_str(&text).unwrap();
        for x in current["contract_evidence"]["exact_cases"][0]["pipe_stress_extrema"].as_array().unwrap() {
            assert_eq!(x["coefficient_basis"], json!("j_side_section_equilibrium_binary64_enclosures"));
            assert_eq!(x["enclosure_scope"], json!("supplied_binary64_polynomial_coefficient_enclosures; solution error is separate"));
            assert_eq!(x["value_upper_pa"], x["global_upper_bound_pa"], "{mode:?}");
        }
        let limitations = current["formulation_basis"]["limitations"].as_array().unwrap();
        assert_eq!(
            limitations[limitations.len() - 2..].to_vec(),
            preview_physics::formulation_basis().limitations[preview_physics::LIMITATIONS.len()..]
                .iter()
                .map(|t| json!(t))
                .collect::<Vec<_>>(),
            "{mode:?}"
        );
        if std::env::var("T13_FALLBACK_UZ_WRITE").as_deref() == Ok("1") {
            let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(format!(
                "../reporting/result_export/tests/fixtures/load_reference_fallback_uz-{}.raw.json",
                mode.as_str()
            ));
            std::fs::write(path, &text).unwrap();
            continue;
        }
        assert!(text == committed, "{mode:?}: the producer's bytes differ from the committed raw file");
    }
}

/// N05 (the sensitive torsion cantilever) with a tip-row formation noise: a
/// uniform W on the pipe, cancelled at the tip by authored nodal UY and RZ
/// inputs, plus 0.2 N*m, so the tip RZ row's intended net is about 0.2 and
/// its formed fixed-end term carries a defect far above 1e-9 of it.
fn n05_tip_noise(case_id: &str) -> Value {
    let w = 1e8_f64;
    json!({"id": case_id, "label": case_id, "kind": "primitive_user_load", "provenance": INVENTED,
    "primitive_loads": [
        uniform(&format!("{case_id}:w"), "pipe", "global_y", w),
        nodal(&format!("{case_id}:uy"), "tip", "global_y", -w),
        nodal(&format!("{case_id}:rz"), "tip", "RZ", w / 3.0 + 0.2),
    ]})
}

fn n05_request(cases: Vec<Value>) -> Value {
    let mut model: Value = serde_json::from_str(include_str!(
        "../../../fixtures/product_preview/numerical_sensitive_torsion_model.json"
    ))
    .unwrap();
    model["load_cases"] = json!(cases);
    request_of(model)
}

fn n05_torques() -> Value {
    json!({"id": "case", "label": "torques", "kind": "primitive_user_load", "provenance": INVENTED,
        "primitive_loads": [nodal("tip:0", "tip", "RX", 1e8), nodal("tip:1", "tip", "RX", 0.3), nodal("tip:2", "tip", "RX", -1e8)]})
}

/// T13b (M15's product kill; the no-op rule): a case whose ordinary report is
/// already Sensitive (N05) and whose load-row guard fires keeps its integrity
/// record exactly: no S11-G sentence, today's code.
#[test]
fn t13b_already_sensitive_case_is_left_untouched() {
    let request = n05_request(vec![n05_tip_noise("case")]);
    let view = guard_view(&request, 0);
    assert!(
        view.decision("tip:RZ").fires,
        "precondition: the load-row guard fires"
    );
    for mode in MODES {
        let envelope = solved(Entry::Typed, &request, mode);
        let d = integrity(&envelope, "case");
        assert!(
            d.message.contains("quality: Sensitive"),
            "precondition: already Sensitive"
        );
        assert_eq!(d.code, "NUMERICAL_INTEGRITY_SENSITIVE");
        assert!(!d.message.contains("S11-G"), "{mode:?}: {}", d.message);
    }
}

/// I61 U1 (G-b): T13b's case through the observed route. The load-row finding
/// is captured typed but, the report being already Sensitive, `demote` leaves it
/// undisclosed, so its diagnostic reference stays null; the ordinary bytes are
/// unchanged by the capture.
#[test]
fn u1_already_sensitive_finding_is_captured_undisclosed() {
    let request = n05_request(vec![n05_tip_noise("case")]);
    for mode in MODES {
        let plain = serde_json::to_vec(&run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap()).unwrap();
        let (typed, capture) = source_receipt::CapturedInvocation::parse(request.clone(), mode).unwrap();
        let mut observer = crate::retained_product::ProductCapture::prepared_probe();
        let observed = run_linear_static_preview_observed(typed, mode, Some(&capture), &mut SourceRecoveryBudget::default(), Some(&mut observer));
        assert_eq!(serde_json::to_vec(&observed).unwrap(), plain, "{mode:?}: capture leaves the ordinary bytes unchanged");
        let [seed] = &observer.ordinary[..] else { panic!("one seed") };
        assert!(matches!(&seed.initial, Some(crate::retained_product::InitialSeed::Report { code, .. }) if code == "NUMERICAL_INTEGRITY_SENSITIVE"), "{mode:?}: {:?}", seed.initial);
        let finding = seed.load_row_finding.as_ref().expect("the load-row guard fires");
        assert!(!finding.sentence.is_empty(), "{mode:?}");
        assert_eq!(finding.diagnostic_ref, None, "{mode:?}: already Sensitive, so the finding is not disclosed");
    }
}

/// The invented bend of S11-F's F8 on a 2 m chord (OD 0.168 m, wall 0.007 m).
fn curved_body(model: &mut Value, prefix: &str, origin: f64, loads: Vec<Value>) {
    curved_arc(
        model,
        prefix,
        [origin, 0.0],
        2.0,
        std::f64::consts::SQRT_2,
        std::f64::consts::PI / 2.0,
        2.0,
        loads,
    );
}

/// T4-U1b: `curved_body` generalised (T15b): a chord of length `chord` along
/// global x from (x, y, 0), user radius `radius`, user angle `angle` (the
/// included angle the chord and radius imply), flexibility factor `k`.
#[allow(clippy::too_many_arguments)]
fn curved_arc(
    model: &mut Value,
    prefix: &str,
    origin: [f64; 2],
    chord: f64,
    radius: f64,
    angle: f64,
    k: f64,
    loads: Vec<Value>,
) {
    let n0 = format!("{prefix}0");
    let n1 = format!("{prefix}1");
    let bend = format!("{prefix}bend");
    model["nodes"].as_array_mut().unwrap().extend([
        node(&n0, origin[0], origin[1], 0.0),
        node(&n1, origin[0] + chord, origin[1], 0.0),
    ]);
    let mut segment = pipe(&bend, &n0, &n1, [0.0, 1.0, 0.0]);
    segment["section"] = json!({"outside_diameter": {"value": 0.168, "unit": "m"}, "wall_thickness": {"value": 0.007, "unit": "m"}});
    segment["material"] = model["materials"][0]["id"].clone();
    model["pipe_segments"].as_array_mut().unwrap().push(segment);
    model["supports"].as_array_mut().unwrap().extend([
        support(&format!("{prefix}a0"), &n0, &ALL),
        json!({"id": format!("{prefix}g1"), "node": n1, "family": "guide", "restraints": ["UX", "UY"], "provenance": INVENTED}),
    ]);
    if model.get("components").is_none() {
        model["components"] = json!([]);
    }
    model["components"].as_array_mut().unwrap().push(json!({
        "id": format!("component:{bend}"), "label": "Invented bend", "kind": "bend", "node": n1,
        "geometry": {"bend_pipe_ref": bend, "bend_radius": {"value": radius, "unit": "m"},
            "bend_angle": {"value": angle, "unit": "rad"},
            "bend_plane_orientation": "global_xy_preview",
            "bend_geometry_source_reference": "invented_user_entered_preview_geometry"},
        "modifiers": {"sif_user_value": {"value": 1.15, "unit": "none"},
            "flexibility_factor_user_value": {"value": k, "unit": "none"},
            "source_reference": "invented_user_entered_preview_no_code_table"},
        "mechanics_interface": {"solver_consumption": "curved_bend_macro_element",
            "rule_check_consumption": "user_rule_pack_inputs_only"},
        "completeness": [{"finding_id": format!("finding:{bend}:geometry"), "status": "complete",
            "diagnostic_code": "BEND_GEOMETRY_INCOMPLETE", "missing_field_kinds": []}],
        "provenance": INVENTED}));
    model["load_cases"][0]["primitive_loads"]
        .as_array_mut()
        .unwrap()
        .extend(loads);
}

/// The realized arc of a one-bend request, as the certificate sees it.
fn arc_formation(request: &Value) -> (CurvedFormation, CurvedBendMacroElement) {
    let built = prepared(request).built;
    assert_eq!(built.curved_bend_elements.len(), 1);
    let bend = &built.curved_bend_elements[0];
    (curved_formation_of(bend), bend.macro_element)
}

/// The loaded rows of the arc's load `source`.
fn arc_rows<'a>(view: &'a GuardView, source: &str) -> Vec<&'a FormationRow> {
    view.rows
        .iter()
        .filter(|r| r.formed_sources.iter().any(|s| s == source))
        .collect()
}

/// T15's preconditions (T4-I9 DESIGN_R01 §5): the certificate is Ok; every
/// loaded arc row carries `source` as a formed (not CannotBound) term with a
/// positive bound; no loaded row fires and each exact statistic is below 1;
/// and the paths differ: the same row with the term CannotBound fires with
/// the CannotBound reason. Returns whether some arc row has A_net ≠ 0.
fn assert_certified_rows(request: &Value, intensity: [f64; 3], source: &str, label: &str) -> bool {
    let (formation, _) = arc_formation(request);
    let certified = certify_curved_uniform_load(&formation, intensity);
    assert!(certified.is_ok(), "{label}: certificate {certified:?}");
    let view = guard_view(request, 0);
    let rows = arc_rows(&view, source);
    assert!(!rows.is_empty(), "{label}: no arc row");
    let mut defect = false;
    for row in &rows {
        assert!(row.cannot_bound_sources.is_empty(), "{label}: {row:?}");
        assert!(row.bound > 0.0, "{label}: MU1, bound {}", row.bound);
        defect |= !row.net_defect.is_zero();
        let mut cannot = (*row).clone();
        cannot.cannot_bound_sources.push(source.to_string());
        let fired = decide_row(&cannot, view.scales[&row.dof]);
        assert!(
            fired.fires && fired.reason.as_deref().unwrap().contains("CannotBound"),
            "{label}: paths differ"
        );
    }
    for row in &view.rows {
        let decision = decide_row(row, view.scales[&row.dof]);
        assert!(!decision.fires, "{label}: {:?}", decision.reason);
        assert!(decision.net_ratio < 1.0, "{label}: {}", decision.net_ratio);
    }
    defect
}

/// The Passed pin on every view, with the ordinary report Passed first.
fn assert_checks_passed(envelope: &MechanicsEnvelope, label: &str) {
    assert_ordinary_passed(envelope, "case", label);
    let d = integrity(envelope, "case");
    assert_eq!(d.code, "NUMERICAL_INTEGRITY_CHECKS_PASSED", "{label}: {}", d.message);
    assert!(!d.message.contains(LOAD_ROW), "{label}");
    assert_eq!(
        case_quality(envelope, "case"),
        NumericalQualityStatus::ChecksPassed,
        "{label}"
    );
}

/// T15 (T4-U1b, R-1; DESIGN_R01 §5): a realized arc carrying a uniform load
/// is certified, and the case publishes `CHECKS_PASSED` on both entries and
/// in both modes. Before T4-U1b its consistent vector was CannotBound and the
/// case was demoted (S11-G SF-2).
#[test]
fn t15_curved_uniform_load_is_certified() {
    let mut model = preview_model("curved");
    model["materials"] = json!([material()]);
    curved_body(
        &mut model,
        "c",
        0.0,
        vec![uniform("w:c", "cbend", "global_z", 0.3)],
    );
    let request = request_of(model);
    // MU2: with intended := v every arc A_net would be 0.
    assert!(
        assert_certified_rows(&request, [0.0, 0.0, 0.3], "w:c", "T15"),
        "precondition: some arc row has A_net != 0"
    );
    for entry in [Entry::Captured, Entry::Typed] {
        for mode in MODES {
            let label = format!("T15 {entry:?} {mode:?}");
            assert_checks_passed(&solved(entry, &request, mode), &label);
        }
    }
}

/// T15g (T4-RV15 F-1; DESIGN_R01's MU9 row): a generated (`<case>:generated:`)
/// uniform load on T15's arc carries the γ₅ formation term. Each loaded arc
/// row's bound equals the exact sum of its certified terms'
/// `operand_bound(true)` and exceeds the sum of their `operand_bound(false)`,
/// so PP passes `generated` through to the certificate's operand bound.
#[test]
fn t15g_generated_arc_load_carries_the_gamma5_operand_bound() {
    let source = "case:generated:t15g";
    let intensity = [0.0, 0.0, 0.3];
    let mut model = preview_model("curved-generated");
    model["materials"] = json!([material()]);
    curved_body(
        &mut model,
        "c",
        0.0,
        vec![uniform(source, "cbend", "global_z", 0.3)],
    );
    let request = request_of(model);
    let (formation, _) = arc_formation(&request);
    let certified = certify_curved_uniform_load(&formation, intensity).unwrap();
    let dof_map = element_dof_map(formation.node_i, formation.node_j);
    let view = guard_view(&request, 0);
    let rows = arc_rows(&view, source);
    assert_eq!(rows.len(), ELEMENT_DOF, "every arc DOF carries the load");
    for row in rows {
        assert_eq!(row.formed_sources, [source], "row {} carries only this load", row.dof);
        assert!(row.cannot_bound_sources.is_empty(), "{row:?}");
        // Σ over the row's certified terms, exactly (one term per arc DOF).
        let mut generated = ExactAccumulator::new();
        let mut ordinary = ExactAccumulator::new();
        for (local, &global) in dof_map.iter().enumerate() {
            if global == row.dof {
                generated.add(certified.terms[local].operand_bound(true)).unwrap();
                ordinary.add(certified.terms[local].operand_bound(false)).unwrap();
            }
        }
        let generated = generated.round().unwrap();
        let ordinary = ordinary.round().unwrap();
        assert!(
            generated.is_finite() && generated > ordinary,
            "row {}: {generated} vs {ordinary}",
            row.dof
        );
        assert_eq!(row.bound, generated, "row {}: the generated operand bound", row.dof);
        assert!(row.bound > ordinary, "row {}", row.dof);
    }
}

/// T15b (DESIGN_R01 §5 with T4-RV8 C-1): certified arcs publish Passed at
/// ordinary and UTM origins, from 90° to 5°, at k = 1 and 2, under an
/// out-of-plane and an in-plane load; and a 0.3 m chord at φ = 1e-4 rad.
/// Re-pinned to the unified member predicate's outcome on the certified
/// rows (DEL-04-01 `kd5-member-relative-bound.md`, "Superseded by this unit"
/// item 3: the t15b 1e-4 rad acceptance is superseded; predicted and
/// measured Passed, with the short-bend rule removed).
#[test]
fn t15b_certified_arcs_pass_at_utm_and_small_angles() {
    let mut cases = Vec::new();
    for origin in [[0.0, 0.0], [5e6, 3.5e6], [7.3e6, 0.0]] {
        for degrees in [90.0_f64, 45.0, 15.0, 5.0] {
            let phi = degrees.to_radians();
            cases.push((origin, 2.0, phi, format!("{origin:?} {degrees} deg")));
        }
    }
    cases.push(([0.0, 0.0], 0.3, 1e-4, "0.3 m chord 1e-4 rad".to_string()));
    cases.push(([7.3e6, 0.0], 0.3, 1e-4, "UTM 0.3 m chord 1e-4 rad".to_string()));
    let mut defects = 0;
    let mut checked = 0;
    for (origin, chord, phi, name) in cases {
        let radius = 0.5 * chord / (0.5 * phi).sin();
        let angle = 2.0 * (0.5 * chord / radius).asin();
        for k in [1.0, 2.0] {
            for (direction, intensity) in [("global_z", [0.0, 0.0, 0.3]), ("global_y", [0.0, -0.3, 0.0])] {
                let mut model = preview_model("curved");
                model["materials"] = json!([material()]);
                let value = intensity[1] + intensity[2];
                curved_arc(
                    &mut model,
                    "c",
                    origin,
                    chord,
                    radius,
                    angle,
                    k,
                    vec![uniform("w:c", "cbend", direction, value)],
                );
                let request = request_of(model);
                let label = format!("T15b {name} k={k} {direction}");
                if assert_certified_rows(&request, intensity, "w:c", &label) {
                    defects += 1;
                }
                for entry in [Entry::Captured, Entry::Typed] {
                    for mode in MODES {
                        let label = format!("{label} {entry:?} {mode:?}");
                        assert_checks_passed(&solved(entry, &request, mode), &label);
                    }
                }
                checked += 1;
            }
        }
    }
    assert_eq!(checked, 56);
    assert!(defects > 0, "non-vacuity: some arc row has A_net != 0");
}

/// T15c (DESIGN_R01 §7.2; R-2: a natural refusal, no seam): T15's model at
/// k = 1e36, 1e37 and 1e38, where ρ̂ ≥ 1/2 refuses the certificate. At guard
/// level every loaded arc row is CannotBound and fires with that reason
/// (M16, MU3, MU8). End to end these k never publish: the captured entry
/// refuses the request (k ≥ 2⁵³ is an unsafe integral JSON number) and the
/// typed entry refuses the solve (a nonpositive or cancellation-unresolved
/// structural pivot), so the CannotBound sentence is not asserted in an
/// envelope (R-2's narrowing, T4-I17's return).
#[test]
fn t15c_curved_uniform_load_without_certificate_is_cannot_bound() {
    for k in [1e36, 1e37, 1e38] {
        let mut model = preview_model("curved");
        model["materials"] = json!([material()]);
        let w = vec![uniform("w:c", "cbend", "global_z", 0.3)];
        curved_arc(&mut model, "c", [0.0, 0.0], 2.0, std::f64::consts::SQRT_2, std::f64::consts::PI / 2.0, k, w);
        let request = request_of(model);
        let label = format!("T15c k={k:e}");
        let (formation, _) = arc_formation(&request);
        match certify_curved_uniform_load(&formation, [0.0, 0.0, 0.3]) {
            Err(ArcCertificateFailure::ResidualNotContracting { rho }) => {
                if k == 1e36 {
                    assert!((0.5..1.0).contains(&rho), "{label}: MU4, rho {rho}");
                } else {
                    assert!(rho >= 1.0, "{label}: rho {rho}");
                }
            }
            other => panic!("{label}: {other:?}"),
        }
        let view = guard_view(&request, 0);
        let rows = arc_rows(&view, "w:c");
        assert_eq!(rows.len(), 12, "{label}");
        for row in rows {
            assert_eq!(row.cannot_bound_sources, ["w:c"], "{label}");
            let decision = decide_row(row, view.scales[&row.dof]);
            assert!(decision.fires, "{label}: M16");
            assert!(decision.reason.unwrap().contains("CannotBound"), "{label}");
        }
        for entry in [Entry::Captured, Entry::Typed] {
            for mode in MODES {
                let published_passed = run(entry, &request, mode).is_ok_and(|envelope| {
                    blocking(&envelope).is_empty()
                        && envelope.numerical_quality.status == NumericalQualityStatus::ChecksPassed
                });
                assert!(!published_passed, "{label} {entry:?} {mode:?}");
            }
        }
    }
}

/// T15d (DESIGN_R01 §8, A-5): at k = 1e10 the binary64 inverse of F loses
/// digits like cond(F)·u while the certificate's radius stays small: the
/// row fires by the exact formation defect, not by the radius, and the case
/// is not published as `CHECKS_PASSED`. Kills MU2 (intended := v).
#[test]
fn t15d_large_flexibility_arc_formation_defect_is_caught() {
    let mut model = preview_model("curved");
    model["materials"] = json!([material()]);
    let w = vec![uniform("w:c", "cbend", "global_z", 0.3)];
    curved_arc(&mut model, "c", [0.0, 0.0], 2.0, std::f64::consts::SQRT_2, std::f64::consts::PI / 2.0, 1e10, w);
    let request = request_of(model);
    let (formation, _) = arc_formation(&request);
    assert!(certify_curved_uniform_load(&formation, [0.0, 0.0, 0.3]).is_ok());
    let view = guard_view(&request, 0);
    let mut caught = 0;
    for row in arc_rows(&view, "w:c") {
        assert!(row.cannot_bound_sources.is_empty());
        let decision = decide_row(row, view.scales[&row.dof]);
        if !decision.fires {
            continue;
        }
        // The catch comes from the defect: B < T0/1000 and |A_net|/12 > T0,
        // both decided exactly.
        assert!(row.bound < decision.t0 / 1000.0, "{row:?}");
        let mut excess = row.net_defect.clone();
        let positive = excess.signum() >= 0;
        excess.add_product(if positive { -12.0 } else { 12.0 }, decision.t0).unwrap();
        assert_eq!(excess.signum(), if positive { 1 } else { -1 }, "|A_net|/12 > T0");
        assert!(decision.reason.unwrap().starts_with("net formation defect"));
        caught += 1;
    }
    assert!(caught > 0, "the formation defect is caught");
    for entry in [Entry::Captured, Entry::Typed] {
        for mode in MODES {
            let label = format!("T15d {entry:?} {mode:?}");
            let envelope = solved(entry, &request, mode);
            assert_ne!(
                envelope.numerical_quality.status,
                NumericalQualityStatus::ChecksPassed,
                "{label}"
            );
            let d = integrity(&envelope, "case");
            println!(
                "{label}: {} (ordinary report Passed: {}; S11-G load-row sentence: {})",
                d.code,
                d.message.contains("quality: Passed"),
                d.message.contains(LOAD_ROW)
            );
        }
    }
}

/// V3 (DESIGN_R01 §6.1): perturbation controls on T15's certified arc. The
/// largest arc component pushed as v + 2·T0 with its certified record fires;
/// v + T0/2 stays silent when the base statistic is below 1/2.
#[test]
fn certified_term_perturbation_controls() {
    let mut model = preview_model("curved");
    model["materials"] = json!([material()]);
    curved_body(&mut model, "c", 0.0, vec![uniform("w:c", "cbend", "global_z", 0.3)]);
    let request = request_of(model);
    let (formation, element) = arc_formation(&request);
    let certified = certify_curved_uniform_load(&formation, [0.0, 0.0, 0.3]).unwrap();
    let values = element.consistent_uniform_nodal_loads([0.0, 0.0, 0.3]).unwrap();
    let largest = (0..ELEMENT_DOF)
        .max_by(|&a, &b| values[a].abs().total_cmp(&values[b].abs()))
        .unwrap();
    let view = guard_view(&request, 0);
    let dof = element_dof_map(formation.node_i, formation.node_j)[largest];
    let row = view.rows.iter().find(|r| r.dof == dof).unwrap();
    let base = decide_row(row, view.scales[&dof]);
    let term = &certified.terms[largest];
    let single = |value: f64| {
        let mut ledger = LoadLedger::new();
        ledger.push_formed(
            "w:c",
            0,
            value,
            Formation::Exact {
                scale: 1.0,
                scaled_intended: term.intended.clone(),
            },
            term.operand_bound(false),
            false,
        );
        decide_row(&one_row(ledger), view.scales[&dof])
    };
    // The single-term row is T15's row (it carries only this term).
    assert_eq!(row.formed_sources, ["w:c"]);
    assert_eq!(single(values[largest]).net_ratio, base.net_ratio);
    let fired = single(values[largest] + 2.0 * base.t0);
    assert!(fired.fires && fired.reason.unwrap().starts_with("net formation defect"));
    assert!(base.net_ratio < 0.5, "{}", base.net_ratio);
    assert!(!single(values[largest] + 0.5 * base.t0).fires);
}

/// M16: a CannotBound term is never a zero defect: its row fires with the
/// CannotBound reason.
#[test]
fn m16_cannot_bound_row_fires() {
    let mut ledger = LoadLedger::new();
    ledger.push_formed("w:c", 0, 1.0, Formation::CannotBound, 0.0, false);
    let row = one_row(ledger);
    assert!(row.net_defect.is_zero() && row.bound == 0.0);
    let decision = decide_row(&row, 1.0);
    assert!(decision.fires);
    assert!(decision.reason.unwrap().contains("CannotBound"));
}

/// A straight body built like RF-CANCEL-UDL-W1e8's members and loads,
/// translated to x = `origin`: its S1 RZ row fires the load-row guard (T1).
fn udl_body(model: &mut Value, prefix: &str, origin: f64) {
    let id = |s: &str| format!("{prefix}{s}");
    model["nodes"].as_array_mut().unwrap().extend([
        node(&id("S0"), origin, 0.0, 0.0),
        node(&id("S1"), origin + 2.0, 0.0, 0.0),
        node(&id("S2"), origin + 4.0, 0.0, 0.0),
    ]);
    let material = model["materials"][0]["id"].clone();
    for (name, from, to) in [("A", "S0", "S1"), ("B", "S1", "S2")] {
        let mut segment = pipe(&id(name), &id(from), &id(to), [0.0, 1.0, 0.0]);
        segment["material"] = material.clone();
        model["pipe_segments"].as_array_mut().unwrap().push(segment);
    }
    model["supports"].as_array_mut().unwrap().extend([
        support(&id("a0"), &id("S0"), &ALL),
        json!({"id": id("p1"), "node": id("S1"), "restraints": ["UX", "UY", "UZ"], "provenance": INVENTED}),
        support(&id("a2"), &id("S2"), &ALL),
    ]);
    model["load_cases"][0]["primitive_loads"]
        .as_array_mut()
        .unwrap()
        .extend([
            nodal(&id("load:0"), &id("S1"), "RZ", 0.2),
            uniform(&id("udl:1"), &id("A"), "global_y", 100000000.0),
            uniform(&id("udl:2"), &id("B"), "global_y", 100000000.875),
        ]);
}

/// A body like `udl_body` with a soft 10 m cantilever branch from S1 (OD
/// 2 mm, wall 0.2 mm) carrying a 2e-6 N tip force. The branch tip's rotation
/// (about 1e-3 rad) sets the body's rotation scale, so K-D5's per-DOF rule
/// (DEL-04-01 kd5-member-relative-bound.md §1, D5C-1 on the closed w) passes
/// the S1:RZ formation defect, while the tiny tip force leaves the load-row
/// guard's coupled moment scale (about 0.2 N*m) unchanged, so the guard still
/// fires there: a Passed report with a load-row finding.
fn udl_body_with_soft_branch(model: &mut Value, prefix: &str, origin: f64) {
    udl_body(model, prefix, origin);
    let id = |s: &str| format!("{prefix}{s}");
    model["nodes"].as_array_mut().unwrap().push(node(&id("S3"), origin + 2.0, 10.0, 0.0));
    let mut branch = pipe(&id("C"), &id("S1"), &id("S3"), [1.0, 0.0, 0.0]);
    branch["material"] = model["materials"][0]["id"].clone();
    branch["section"] = json!({"outside_diameter": {"value": 0.002, "unit": "m"}, "wall_thickness": {"value": 0.0002, "unit": "m"}});
    model["pipe_segments"].as_array_mut().unwrap().push(branch);
    model["load_cases"][0]["primitive_loads"]
        .as_array_mut()
        .unwrap()
        .push(nodal(&id("load:tip"), &id("S3"), "global_z", 2e-6));
}

/// Ruling 1: both guards fire. Probe A at G = 1e80 (the unified predicate
/// fires on its certified rows, T5) plus a separate straight body built like
/// RF-CANCEL-UDL-W1e8 with a soft branch (its own RZ row fires the load-row
/// guard on a Passed report, `udl_body_with_soft_branch`). The load-row
/// demotion comes first; the recovery guard's later amendment is a no-op: its
/// sentence does not appear (kd5-member-relative-bound.md §3.3, "the load-row
/// sentence stands first (I5 ruling 1)"). Under K-D5's repair the earlier
/// F-INPLANE trigger is repaired and passes (T11), so probe A replaces it.
/// (T4-U1b, O-8: the earlier trigger, a curved uniform load, is now
/// certified and no longer fires.)
#[test]
fn ruling1_both_guards_fire_and_the_load_row_sentence_stands() {
    let mut request = probe_a(1e80, [1.0, 0.0, -1.0]);
    udl_body_with_soft_branch(&mut request["model"], "u", 100.0);
    let view = guard_view(&request, 0);
    assert!(
        view.decision("uS1:RZ").fires,
        "precondition: the load-row guard fires on uS1:RZ"
    );
    let alone = probe_a(1e80, [1.0, 0.0, -1.0]);
    for mode in MODES {
        let label = format!("both guards {mode:?}");
        // Precondition: alone, probe A is demoted by the recovery guard.
        assert_demoted(&solved(Entry::Typed, &alone, mode), "case", RECOVERY, &label);
        let envelope = solved(Entry::Typed, &request, mode);
        assert_ordinary_passed(&envelope, "case", &label);
        assert_demoted(&envelope, "case", LOAD_ROW, &label);
        assert!(
            integrity(&envelope, "case").message.contains("uS1:RZ"),
            "{label}"
        );
        assert!(
            !integrity(&envelope, "case").message.contains(RECOVERY),
            "{label}: the recovery guard is a no-op"
        );
    }
}

// ---------------------------------------------------------------- T16: gate

fn hex_f64(v: &Value) -> f64 {
    f64::from_bits(u64::from_str_radix(v.as_str().unwrap(), 16).unwrap())
}

fn element_row<'a>(
    envelope: &'a MechanicsEnvelope,
    member: &str,
    component: &str,
    location: &str,
) -> Option<&'a ResultItem> {
    case_rows(envelope, "case").into_iter().find(|r| {
        r.kind.starts_with("element_local_")
            && r.entity_ref == member
            && r.metadata
                .as_ref()
                .is_some_and(|m| m.component == component && m.location == location)
    })
}

/// `lo <= a^2 + b^2 <= hi`, exactly.
fn hypot_squared_within(a: f64, b: f64, lo: f64, hi: f64) -> bool {
    let mut low = ExactAccumulator::new();
    low.add_product(a, a).unwrap();
    low.add_product(b, b).unwrap();
    low.add(-lo).unwrap();
    let mut high = ExactAccumulator::new();
    high.add(hi).unwrap();
    high.add_product(-a, a).unwrap();
    high.add_product(-b, b).unwrap();
    low.signum() >= 0 && high.signum() >= 0
}

/// S11-F's `check_rf_rows` (the same generated intervals): the breaching keys
/// and the checked keys of one run.
fn rf_breaches(case: &Value, envelope: &MechanicsEnvelope) -> (Vec<String>, BTreeSet<String>) {
    let mut breaches = Vec::new();
    let mut checked = BTreeSet::new();
    for row in case["rows"].as_array().unwrap() {
        let key = row["key"].as_str().unwrap().to_string();
        let check = row["check"].as_str().unwrap();
        if check == "not_published" {
            continue;
        }
        let (lo, hi) = (hex_f64(&row["lo_bits"]), hex_f64(&row["hi_bits"]));
        let inside = |x: f64| lo <= x && x <= hi;
        let ok = match check {
            "value" => {
                let id = row["id"].as_str().unwrap();
                let Some(r) = case_rows(envelope, "case").into_iter().find(|r| r.id == id) else {
                    continue;
                };
                inside(r.value)
            }
            "support" => {
                let Some(r) = case_rows(envelope, "case").into_iter().find(|r| {
                    r.kind == "support_reaction_component_v2"
                        && r.entity_ref == row["support"].as_str().unwrap()
                        && r.metadata
                            .as_ref()
                            .is_some_and(|m| m.component == row["component"].as_str().unwrap())
                }) else {
                    continue;
                };
                inside(r.value)
            }
            "pair" => {
                let member = row["member"].as_str().unwrap();
                let component = row["component"].as_str().unwrap();
                let (Some(i), Some(j)) = (
                    element_row(envelope, member, component, "end_i"),
                    element_row(envelope, member, component, "end_j"),
                ) else {
                    continue;
                };
                inside(-i.value) && inside(j.value)
            }
            "hypot" => {
                let member = row["member"].as_str().unwrap();
                let location = row["location"].as_str().unwrap();
                let (Some(y), Some(z)) = (
                    element_row(envelope, member, "bending_moment_y", location),
                    element_row(envelope, member, "bending_moment_z", location),
                ) else {
                    continue;
                };
                hypot_squared_within(y.value, z.value, lo, hi)
            }
            other => panic!("unknown check {other}"),
        };
        checked.insert(key.clone());
        if !ok {
            breaches.push(key);
        }
    }
    (breaches, checked)
}

/// The RF-CANCEL case-modes demoted, as (entry, case, mode): the UDL
/// formation rows' case-modes, which K-D5 demotes at the solve.
const EXPECTED_DEMOTED: &[(&str, &str)] = &[
    ("captured", "RF-CANCEL-UDL-W1e8"),
    ("typed", "RF-CANCEL-UDL-W1e8"),
    ("typed", "RF-CANCEL-UDL-W1e80"),
];

/// T16 (extended from S11-F's F1/F11/F12). Every RF-CANCEL case through both
/// entries and both modes. Under K-D5 (DEL-04-01 kd5-member-relative-bound.md
/// §§1, 3.4, 4.2): the 6 UDL formation rows (S1's rotation, a displacement
/// row published unchanged) still breach and are published in a non-Passed
/// case, demoted at the solve by D5C-1 on the closed residual; the 8 INPLANE
/// formation rows are repaired and now meet R1's predicate in a
/// CHECKS_PASSED case. Every row outside the formation list meets R1's
/// predicate (the S11 list stays empty), S11-G demotes no case-mode, and the
/// set of demoted case-modes is exactly `EXPECTED_DEMOTED` in both modes.
#[test]
fn t16_formation_rows_are_published_non_passed_on_both_entries() {
    let data = rf_cancel_data();
    let formation: BTreeSet<(String, String, String, String)> = data["formation_exceptions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            (0..4)
                .map(|k| r[k].as_str().unwrap().to_string())
                .collect::<Vec<_>>()
        })
        .map(|r| (r[0].clone(), r[1].clone(), r[2].clone(), r[3].clone()))
        .collect();
    assert_eq!(formation.len(), 14);
    let mut demoted = BTreeSet::new();
    let mut codes_by_case_mode = BTreeMap::new();
    let mut breaches = BTreeSet::new();
    let mut checked: BTreeMap<(String, String, String), BTreeSet<String>> = BTreeMap::new();
    for case in data["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        for entry in [Entry::Captured, Entry::Typed] {
            let entry_name = if entry == Entry::Captured {
                "captured"
            } else {
                "typed"
            };
            for mode in MODES {
                if entry == Entry::Captured && case["captured_refused_at_capture"] == json!(true) {
                    assert!(run(entry, &case["request"], mode).is_err());
                    continue;
                }
                let envelope = solved(entry, &case["request"], mode);
                let d = integrity(&envelope, "case");
                let slot = (
                    entry_name.to_string(),
                    id.to_string(),
                    mode.as_str().to_string(),
                );
                codes_by_case_mode.insert(slot.clone(), d.code.clone());
                assert!(!d.message.contains("S11-G"), "{slot:?}: {}", d.message);
                if d.code == "NUMERICAL_INTEGRITY_SENSITIVE" {
                    assert!(d.message.contains("formation_check: reason=estimate; row=S1:RZ;"), "{slot:?}");
                    demoted.insert(slot.clone());
                }
                let (bad, keys) = rf_breaches(case, &envelope);
                for key in bad {
                    breaches.insert((
                        entry_name.to_string(),
                        id.to_string(),
                        key,
                        mode.as_str().to_string(),
                    ));
                }
                checked.entry(slot).or_default().extend(keys);
            }
        }
    }
    // Every formation row is checked: a UDL row still breaches and is
    // published in a demoted case; an INPLANE row is repaired and Passed.
    for (entry, case, key, mode) in &formation {
        let slot = (entry.clone(), case.clone(), mode.clone());
        assert!(
            checked.get(&slot).is_some_and(|k| k.contains(key)),
            "{slot:?} {key} not published"
        );
        let row = (entry.clone(), case.clone(), key.clone(), mode.clone());
        if case.contains("UDL") {
            assert!(breaches.contains(&row), "{row:?}");
            assert!(demoted.contains(&slot), "{slot:?} {key}: not demoted");
        } else {
            assert!(!breaches.contains(&row), "{row:?}: repaired rows meet R1's predicate");
            assert_eq!(
                codes_by_case_mode[&slot], "NUMERICAL_INTEGRITY_CHECKS_PASSED",
                "{slot:?} {key}"
            );
        }
    }
    // No breach outside the formation list: the S11 list stays empty.
    let outside: Vec<_> = breaches
        .iter()
        .filter(|b| !formation.contains(*b))
        .collect();
    assert!(
        outside.is_empty(),
        "breaches outside the lists: {outside:?}"
    );
    for triple in data["exceptions"].as_array().unwrap() {
        for mode in MODES {
            let slot = (
                triple[0].as_str().unwrap().to_string(),
                triple[1].as_str().unwrap().to_string(),
                mode.as_str().to_string(),
            );
            assert!(
                checked
                    .get(&slot)
                    .is_some_and(|k| k.contains(triple[2].as_str().unwrap())),
                "{slot:?}"
            );
        }
    }
    let expected: BTreeSet<_> = EXPECTED_DEMOTED
        .iter()
        .flat_map(|(entry, case)| {
            MODES
                .iter()
                .map(move |m| (entry.to_string(), case.to_string(), m.as_str().to_string()))
        })
        .collect();
    eprintln!("T16 demoted case-modes: {demoted:?}");
    assert_eq!(demoted, expected, "the set of demoted case-modes");
}

// ============================================ revision 2.2 routing (T18-T22)

/// Adds an invented temperature point (E, G in Pa) to the request's first
/// material and selects it for the named load case (a per-case modulus
/// basis: its own stiffness and verdict in the same invocation).
fn with_basis(mut request: Value, case: &str, point: &str, e: f64, g: f64) -> Value {
    let material = &mut request["model"]["materials"][0];
    if material.get("temperature_points").is_none() {
        material["temperature_points"] = json!([]);
    }
    material["temperature_points"]
        .as_array_mut()
        .unwrap()
        .push(json!({
        "id": point, "temperature": {"value": 20.0, "unit": "degC"},
        "elastic_modulus": {"value": e, "unit": "Pa"}, "shear_modulus": {"value": g, "unit": "Pa"},
        "provenance": INVENTED}));
    for load_case in request["model"]["load_cases"].as_array_mut().unwrap() {
        if load_case["id"] == json!(case) {
            load_case["modulus_basis_ref"] = json!(point);
        }
    }
    request
}

/// T18 (path 2, revision 2.2). I5's path-2 model on the captured entry, both
/// modes: case A carries N05's cancelling torques; case B is already Sensitive
/// (N05's condition) and its load-row guard fires. Precondition: case B's guard
/// fires and its report is Sensitive (2.1's gate refused this invocation, M19).
/// Pin: the invocation is not refused; case B keeps today's integrity record (the
/// no-op rule, no S11-G sentence). B4/D1 (b): the preview route selects no source
/// blocks, so no receipt or retained-source diagnostic is published.
#[test]
fn t18_path2_invocation_is_not_refused() {
    let request = n05_request(vec![n05_torques(), n05_tip_noise("case-b")]);
    assert!(
        guard_view(&request, 1).decision("tip:RZ").fires,
        "precondition: case B's guard fires"
    );
    for mode in MODES {
        let label = format!("T18 {mode:?}");
        let envelope = solved(Entry::Captured, &request, mode);
        let b = integrity(&envelope, "case-b");
        assert!(
            b.message.contains("quality: Sensitive"),
            "{label}: precondition, case B is already Sensitive"
        );
        assert_eq!(b.code, "NUMERICAL_INTEGRITY_SENSITIVE", "{label}");
        assert!(!b.message.contains("S11-G"), "{label}");
        // B4/D1 (b): the preview route publishes no receipt and no retained-source diagnostic.
        assert_source_block_codes(&envelope, Entry::Captured, &label);
    }
}

/// T19's construction: N05's model, unchanged, with two cases on per-case
/// modulus bases. Case A (N05's cancelling torques, base basis) is Sensitive
/// by N05's soft torsion spring and source-selected. Case B is N05's tip
/// formation noise (`n05_tip_noise`) plus a 1e-10 N*m tip torque, on an
/// invented basis with E = 1 Pa and a torsionally soft G = 1e-12 Pa. On it
/// N05's spring no longer makes the model ill conditioned, and the tiny
/// torque twists the pipe by about 3e6 rad. That rotation sets the body's
/// scales, so K-D5's per-DOF rule (DEL-04-01 kd5-member-relative-bound.md
/// §1, D5C-1 on the closed w) passes the tip:RZ formation defect, while the
/// torque leaves the load-row guard's moment scale unchanged, so the guard
/// still fires there (the soft-branch pattern of T22 and F1a; a branch
/// member would couple more than two free DOFs, which the retained exact
/// context refuses for case A).
pub(crate) fn t19_request() -> Value {
    let mut case_b = n05_tip_noise("case-b");
    case_b["primitive_loads"]
        .as_array_mut()
        .unwrap()
        .push(nodal("case-b:rx", "tip", "RX", 1e-10));
    with_basis(
        n05_request(vec![n05_torques(), case_b]),
        "case-b",
        "point:soft",
        1.0,
        1e-12,
    )
}

/// T19 (path 1, load-row variant; revision 2.2 with erratum E-1). Case A carries
/// N05's cancelling torques; case B is Passed in its report and its load-row guard
/// fires (`t19_request`). With G = 0.4 Pa K-D5's closure judges case B's
/// tip formation defect at the solve (DEL-04-01 kd5-member-relative-bound.md
/// §§1, 4.2), so the construction adds the soft twist, whose rotation scale
/// keeps the report Passed while the guard still fires on tip:RZ. Pin: no refusal; case B `SENSITIVE` by the
/// load-row guard. B4/D1 (b): no receipt and no retained-source diagnostic.
#[test]
fn t19_path1_load_row_variant_is_not_refused() {
    let request = t19_request();
    assert!(
        guard_view(&request, 1).decision("tip:RZ").fires,
        "precondition: case B's guard fires"
    );
    // Control (paths differ): without the soft twist (G = 0.4 Pa, no torque)
    // K-D5's closure demotes case B at the solve.
    let control = with_basis(
        n05_request(vec![n05_torques(), n05_tip_noise("case-b")]),
        "case-b",
        "point:soft",
        1.0,
        0.4,
    );
    for mode in MODES {
        let label = format!("T19 {mode:?}");
        assert_demoted_at_the_solve(&solved(Entry::Typed, &control, mode), "case-b", "tip:RZ", &label);
        // B4/D1 (b): on the captured entry the control, demoted at the solve,
        // attempts no retained-source recovery (the preview route selects none).
        let captured = solved(Entry::Captured, &control, mode);
        assert_demoted_at_the_solve(&captured, "case-b", "tip:RZ", &format!("{label} control captured"));
        assert_source_block_codes(&captured, Entry::Captured, &format!("{label} control captured"));
        let envelope = solved(Entry::Captured, &request, mode);
        assert_ordinary_passed(&envelope, "case-b", &label);
        assert_demoted(&envelope, "case-b", LOAD_ROW, &label);
        assert!(integrity(&envelope, "case-b").message.contains("tip:RZ"), "{label}");
        assert_source_block_codes(&envelope, Entry::Captured, &label);
    }
}

/// Case B of RV4's construction C1: nodal inputs only at N05's tip, a
/// transverse force (N) along global y and a moment (N*m) about global z.
fn c1_case_b(force: f64, moment: f64) -> Value {
    json!({"id": "case-b", "label": "case-b", "kind": "primitive_user_load", "provenance": INVENTED,
        "primitive_loads": [nodal("b:f", "tip", "global_y", force), nodal("b:m", "tip", "RZ", moment)]})
}

/// RV4's construction C1: N05's cantilever with T19's per-case modulus bases.
/// Case A (N05's cancelling tip torques, base basis) is Sensitive by N05's
/// torsion spring and source-selected; case B (`c1_case_b`) runs on the
/// invented soft basis E = 1 Pa, G = 0.4 Pa.
fn c1_request(force: f64, moment: f64) -> Value {
    with_basis(
        n05_request(vec![n05_torques(), c1_case_b(force, moment)]),
        "case-b",
        "point:soft",
        1.0,
        0.4,
    )
}

/// T20 (ruling 3; ROOT's ruling on RV4's finding). B4/D1 (b) CLOSED this
/// residual on the pre-0.4 route: with no preview-route source blocks there is
/// no receipt to finalize, so the invocation below publishes (demoted case B).
/// The history follows: CHARACTERIZATION OF A KNOWN RESIDUAL, NOT DESIRED BEHAVIOUR. The residual
/// is path 1's R-b' variant: a case whose report is Passed and which R-b'
/// demotes after routing, beside a source-selected case. Its `ordinary`
/// receipt entry then fails the wire binding, and receipt finalization
/// refuses the invocation. It is REACHABLE (RV4's construction C1),
/// FAIL-CLOSED (`Err("SOURCE_BLOCKS_FINALIZATION_FAILED")`, no envelope and
/// no published value). C1 uses per-case modulus bases; reach with a single
/// modulus basis (via FK's load audit, or via K-D5's formation check) is not
/// refuted. The residual needed a pre-0.4 captured invocation (a 0.4.0
/// captured invocation was republished under CP3 SF-1). Owner: T3's composite
/// SOURCE_BLOCKS_FINALIZATION_FAILED item (the "demoted-ordinary" receipt
/// form), which must close before T3 closes. Since the pre-0.4 failure
/// unification (DEL-04-08 Design `pre04-failure-unification.md`) no captured
/// invocation takes that refusal: a selected source that cannot finalize
/// republishes every case on its ordinary route, on both routes.
///
/// C1 at m = 1e-7 N*m and F = 1 N. The precondition (paths differ): on the
/// typed entry (no receipt) case B's report is Passed, R-b' fires at the tip
/// end, and the published tip moment carries a genuine relative error above
/// the criterion against its exact value |m| (a free tip's end moment
/// equals the applied tip moment, and its y component is zero). A
/// single-case captured invocation of case B is demoted and not refused. The
/// control (m = 0.5, R-b' silent) publishes with a receipt, case A qualified.
///
/// K-D5 (DEL-04-01 kd5-member-relative-bound.md §§3.1-3.4, 4.2): C1 no longer
/// reaches the residual. Case B's tip moment is now its certified recovery,
/// exactly |m|, so the unified predicate passes it while R-b''s retired
/// binary64 statistic would still fire there: case B is CHECKS_PASSED on the
/// typed entry, alone on the captured entry, and in the two-case captured
/// invocation, which now publishes with both receipt entries qualified.
///
/// The residual itself (a Passed report demoted after routing beside a
/// source-selected case) is reached under K-D5 by a probe A-style case B
/// (`residual_case_b`): N05's pipe carries cancelling uniform loads (G, n, -G)
/// along global y with G = 8e15 N/m (below the captured entry's 2^53 limit)
/// and n = 1e-8 N/m, on C1's soft basis. Its report is Passed; the load
/// enclosure radii (2^-120 level, §1) enter every group's remainder R, so
/// against the tiny net B exceeds RD(1e-9)·q (the anchor's force group has
/// B/q of about 6e-9) and the unified predicate fires after routing (§3.3). Typed: demoted with the unified sentence, no receipt;
/// alone on the captured entry: demoted, not refused; beside case A on the
/// captured entry: refused, fail-closed. Control: n = 0.3 N/m lifts the body
/// scale, the predicate is silent and the invocation publishes with both
/// receipt entries qualified.
/// T20's probe A-style case B (see T20): (G, n, -G) uniform loads along
/// global y on N05's pipe, G = 8e15 N/m.
fn residual_case_b(n: f64) -> Value {
    let g = 8e15_f64;
    json!({"id": "case-b", "label": "case-b", "kind": "primitive_user_load", "provenance": INVENTED,
        "primitive_loads": [uniform("b:g", "pipe", "global_y", g), uniform("b:n", "pipe", "global_y", n), uniform("b:-g", "pipe", "global_y", -g)]})
}

fn residual_request(cases: Vec<Value>) -> Value {
    with_basis(n05_request(cases), "case-b", "point:soft", 1.0, 0.4)
}

#[test]
fn t20_characterization_rb_prime_residual_c1() {
    let (force, moment) = (1.0, 1e-7);
    let request = c1_request(force, moment);
    let model: PreviewModel = serde_json::from_value(request["model"].clone()).unwrap();
    assert!(!case_state::is_load_state(&model), "pre-0.4 only");
    for mode in MODES {
        let label = format!("T20 {mode:?}");
        // Typed pin: case B Passed in its report; R-b''s retired statistic
        // fires at the tip, but the repaired row is exact: Passed, no receipt.
        let typed = solved(Entry::Typed, &request, mode);
        assert!(typed.source_block_recovery.is_none(), "{label}");
        assert_ordinary_passed(&typed, "case-b", &label);
        let tip = rb_view(&request, &typed, "case-b")
            .into_iter()
            .find(|e| e.member == "pipe" && e.end == "j")
            .expect("the tip end");
        assert!(tip.rb_prime(), "{label}: R-b''s retired statistic fires at the tip: {tip:?}");
        // The repaired tip moment is exactly the applied |m|.
        assert_eq!(tip.q.to_bits(), moment.to_bits(), "{label}: {tip:?}");
        assert_eq!(integrity(&typed, "case-b").code, "NUMERICAL_INTEGRITY_CHECKS_PASSED", "{label}");
        assert_not_demoted_by_s11g(&typed, "case-b", &label);
        // Case B alone on the captured entry: Passed.
        let alone = with_basis(
            n05_request(vec![c1_case_b(force, moment)]),
            "case-b",
            "point:soft",
            1.0,
            0.4,
        );
        let single = solved(Entry::Captured, &alone, mode);
        assert_ordinary_passed(&single, "case-b", &label);
        assert_eq!(integrity(&single, "case-b").code, "NUMERICAL_INTEGRITY_CHECKS_PASSED", "{label}");
        // C1's two-case captured invocation publishes (B4/D1 (b): without a receipt).
        let both = solved(Entry::Captured, &request, mode);
        assert!(both.source_block_recovery.is_none(), "{label}");
        assert_eq!(integrity(&both, "case-b").code, "NUMERICAL_INTEGRITY_CHECKS_PASSED", "{label}");
        // Control: m = 0.5 (R-b' silent) publishes.
        let control = solved(Entry::Captured, &c1_request(force, 0.5), mode);
        assert!(control.source_block_recovery.is_none(), "{label}");
        assert_not_demoted_by_s11g(&control, "case-b", &label);
        assert_eq!(
            integrity(&control, "case-b").code,
            "NUMERICAL_INTEGRITY_CHECKS_PASSED",
            "{label}"
        );

        // The residual under K-D5: the probe A-style case B.
        let residual = residual_request(vec![n05_torques(), residual_case_b(1e-8)]);
        let typed = solved(Entry::Typed, &residual, mode);
        assert!(typed.source_block_recovery.is_none(), "{label}");
        assert_ordinary_passed(&typed, "case-b", &label);
        assert_demoted(&typed, "case-b", RECOVERY, &label);
        let single = solved(Entry::Captured, &residual_request(vec![residual_case_b(1e-8)]), mode);
        assert_ordinary_passed(&single, "case-b", &label);
        assert_demoted(&single, "case-b", RECOVERY, &label);
        // B4/D1 (b) closes the residual: the pre-0.4 captured invocation selects no
        // source blocks, so no receipt is finalized and the invocation publishes.
        let closed = run(Entry::Captured, &residual, mode)
            .unwrap_or_else(|error| panic!("{label}: the residual is closed: {error}"));
        assert!(closed.source_block_recovery.is_none(), "{label}");
        assert_demoted(&closed, "case-b", RECOVERY, &label);
        let control = solved(
            Entry::Captured,
            &residual_request(vec![n05_torques(), residual_case_b(0.3)]),
            mode,
        );
        assert!(control.source_block_recovery.is_none(), "{label}");
        assert_eq!(
            integrity(&control, "case-b").code,
            "NUMERICAL_INTEGRITY_CHECKS_PASSED",
            "{label}"
        );
    }
}

/// RV4-S3, product level (with the unit test above, kills RV-M2): the
/// committed exact-pressure request's cases, through the product's own
/// exact-pressure builder and operand producer. Every loaded operand row
/// carries B >= 20u * sum|t| > 0 (gamma_20 > 20u).
#[test]
fn exact_pressure_operand_rows_carry_their_bound() {
    let request: Value = serde_json::from_str(include_str!(
        "../../../fixtures/product_preview/pressure/load_reference/pressure.request.json"
    ))
    .unwrap();
    let Prepared {
        model,
        materials,
        built,
        mut diagnostics,
    } = prepared(&request);
    let mut checked = 0;
    for case in &model.load_cases {
        let exact = pressure_runtime::build_pressure_case_with_members(
            &model,
            &built,
            &materials,
            case,
            None,
            &mut diagnostics,
        )
        .unwrap_or_else(|| panic!("{}: an exact-pressure case: {diagnostics:?}", case.id));
        let mut ledger = LoadLedger::new();
        push_exact_pressure_operands(&mut ledger, &exact);
        let force = ledger.finish(built.nodes.len() * DOF_PER_NODE).unwrap();
        for row in force.formation_rows() {
            // B - 20u * sum|t| > 0, exactly (20u = 20 * 2^-53 is exact).
            let mut margin = ExactAccumulator::new();
            margin.add(row.bound).unwrap();
            let mut nonzero = false;
            for term in force.terms().iter().filter(|t| t.dof == row.dof) {
                if let open_pipe_stress_frame_kernel::load_ledger::ForceTermKind::Term(v) =
                    term.kind
                {
                    nonzero |= v != 0.0;
                    margin
                        .add_product(-20.0 * (f64::EPSILON / 2.0), v.abs())
                        .unwrap();
                }
            }
            if !nonzero {
                continue;
            }
            assert!(
                row.bound > 0.0 && margin.signum() > 0,
                "{}: dof {} bound {}",
                case.id,
                row.dof,
                row.bound
            );
            checked += 1;
        }
    }
    assert!(checked > 0, "precondition: nonzero operand rows");
}

/// RV4-S4 (kills RV-M3): a realized curved span (the invented bend, on a
/// chord (1.2, 1.6, 0) m) carrying a thermal load. Each curved-thermal term
/// is K_rc * fl(eps * chord_c), and its record must name that product's
/// operands. The test forms every bend row's self-equilibrated defect
/// independently, 12 * sum_c -K_rc * lo(eps, chord_c) with lo the exact
/// rounding error of fl(eps * chord_c), from the bend's own stiffness and
/// chord and the case's thermal strain, and requires the ledger's A_se to
/// equal it exactly. The two nonzero chord components round differently, so
/// an operand taken from another axis changes A_se. The self-equilibrated
/// floor holds: the case publishes CHECKS_PASSED on both entries and modes.
#[test]
fn curved_thermal_records_name_the_pushed_products() {
    let mut model = preview_model("curved-thermal");
    model["materials"] = json!([material()]);
    curved_body(&mut model, "c", 0.0, vec![thermal("t:c", "cbend", 150.0)]);
    for node in model["nodes"].as_array_mut().unwrap() {
        if node["id"] == json!("c1") {
            node["position"] = json!({"x": 1.2, "y": 1.6, "z": 0.0});
        }
    }
    let request = request_of(model);
    let view = guard_view(&request, 0);
    let Prepared {
        model,
        materials,
        built,
        mut diagnostics,
    } = prepared(&request);
    let pipe_map = model
        .pipe_segments
        .iter()
        .enumerate()
        .map(|(i, p)| (p.id.as_str(), i))
        .collect::<HashMap<_, _>>();
    let material_map = materials
        .iter()
        .map(|m| (m.id.as_str(), m))
        .collect::<HashMap<_, _>>();
    let thermal = build_thermal_element_loads(
        &model,
        &model.load_cases[0],
        &material_map,
        &pipe_map,
        &built,
        &mut diagnostics,
    );
    assert_eq!(thermal.len(), 1);
    let eps = thermal[0].thermal_strain;
    assert_eq!(
        built.curved_bend_elements.len(),
        1,
        "precondition: a realized curved span"
    );
    let bend = &built.curved_bend_elements[0];
    let lo = |c: f64| eps.mul_add(c, -(eps * c));
    // Precondition: the chord's nonzero components round differently.
    assert_eq!(bend.chord[2], 0.0);
    let (lo_x, lo_y) = (lo(bend.chord[0]), lo(bend.chord[1]));
    assert!(
        lo_x != 0.0 && lo_y != 0.0 && lo_x != lo_y,
        "{lo_x:e} {lo_y:e}"
    );
    let dof_map = element_dof_map(bend.node_i, bend.node_j);
    let mut nonzero = 0;
    for row in &view.rows {
        let Some(local_row) = dof_map.iter().position(|&d| d == row.dof) else {
            continue;
        };
        // A_se - 12 * sum_c -K_rc * lo_c, with 12 * (-K lo) = 3 * (-4K lo) exactly.
        let mut difference = row.self_equilibrated_defect.clone();
        for axis in 0..3 {
            let c = bend.chord[axis];
            if eps * c == 0.0 {
                continue;
            }
            let k = bend.global_stiffness[local_row][DOF_PER_NODE + axis];
            for _ in 0..3 {
                difference.add_product(4.0 * k, lo(c)).unwrap();
            }
        }
        assert!(
            difference.is_zero(),
            "row {}: A_se differs from the pushed products",
            row.dof
        );
        nonzero += usize::from(!row.self_equilibrated_defect.is_zero());
    }
    assert!(nonzero > 0, "precondition: A_se != 0 at a bend row");
    for entry in [Entry::Captured, Entry::Typed] {
        for mode in MODES {
            let label = format!("curved thermal {entry:?} {mode:?}");
            let envelope = solved(entry, &request, mode);
            assert_ordinary_passed(&envelope, "case", &label);
            assert_eq!(
                integrity(&envelope, "case").code,
                "NUMERICAL_INTEGRITY_CHECKS_PASSED",
                "{label}"
            );
        }
    }
}

/// RV4-N2 (behavioural kill of RV-M6; T10b pins the source): E-1's
/// zero-work decline applies only where the ordinary route would not attempt.
/// N06's model (its ordinary attempt is rejected at assembly), with one case
/// carrying N05-style tip formation noise: the case's ordinary attempt errs
/// and its load-row guard fires, so it keeps main's real retained-source
/// attempt (retained scope's non-nodal refusal, with its work charged) and
/// main's blocking integrity failure, not the zero-work formation decline.
/// The precondition comes from the typed entry, which has no retained-source
/// route; the pin is on the captured envelope.
#[test]
fn e1_ordinary_err_guard_fired_case_keeps_mains_attempt() {
    let mut request: Value = serde_json::from_str(include_str!(
        "../../../fixtures/product_preview/w1_controls/ui/n06-sparse_interactive.request.json"
    ))
    .unwrap();
    let w = 1e8_f64;
    request["model"]["load_cases"] = json!([{"id": "case-b", "label": "case-b",
    "kind": "primitive_user_load", "provenance": INVENTED,
    "primitive_loads": [
        uniform("case-b:w", "independent-member", "global_y", w),
        nodal("case-b:uy", "independent-tip", "global_y", -w),
        nodal("case-b:rz", "independent-tip", "RZ", w / 3.0 + 0.2),
    ]}]);
    assert!(
        guard_view(&request, 0).decision("independent-tip:RZ").fires,
        "precondition: the load-row guard fires"
    );
    for mode in MODES {
        let label = format!("E-1 Err {mode:?}");
        // Precondition, from the typed entry: the ordinary attempt errs.
        let typed = run(Entry::Typed, &request, mode).unwrap();
        assert_eq!(
            integrity(&typed, "case-b").code,
            "NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED",
            "{label}: precondition"
        );
        let envelope =
            run(Entry::Captured, &request, mode).unwrap_or_else(|e| panic!("{label}: {e}"));
        // B4/D1 (b): the preview route makes no retained-source attempt.
        assert_no_source_blocks(&envelope, &label);
        // Pin: main's blocking integrity failure, not an S11-G record.
        let failure = envelope
            .diagnostics
            .iter()
            .find(|d| d.id == integrity_diagnostic_id("case-b"))
            .unwrap_or_else(|| panic!("{label}: the integrity record"));
        assert_eq!(
            failure.code, "NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED",
            "{label}"
        );
        assert!(!failure.message.contains("S11-G"), "{label}");
    }
}

/// RV4-N1 (kills RV-M10): restrained rows take S* over all the body's loaded
/// rows (section 3.4), free rows over its free rows. A body anchored at
/// `root` and `mid` with a free `tip`: w = 1e8 N/m on root-mid, whose formed
/// fixed-end moment at root is cancelled by an authored nodal input to a net
/// of about 0.2 N*m (its defect about 1.4e-8 N*m), and small tip inputs
/// (0.1 N, 0.01 N*m). Precondition (paths differ): the root RZ row fires
/// under the body's free-row moment scale (read at the tip RZ row) and is
/// silent under its own all-rows scale. Pin: CHECKS_PASSED.
#[test]
fn restrained_rows_take_the_all_rows_scale() {
    let mut model = preview_model("restrained-scale");
    model["nodes"] = json!([
        node("root", 0.0, 0.0, 0.0),
        node("mid", 2.0, 0.0, 0.0),
        node("tip", 4.0, 0.0, 0.0)
    ]);
    model["pipe_segments"] = json!([
        pipe("m1", "root", "mid", [0.0, 1.0, 0.0]),
        pipe("m2", "mid", "tip", [0.0, 1.0, 0.0])
    ]);
    model["materials"] = json!([material()]);
    model["supports"] = json!([
        support("a-root", "root", &ALL),
        support("a-mid", "mid", &ALL)
    ]);
    let w = 1e8_f64;
    model["load_cases"][0]["primitive_loads"] = json!([
        uniform("udl", "m1", "global_y", w),
        nodal("root-rz", "root", "RZ", -(w / 3.0) + 0.2),
        nodal("tip-uy", "tip", "global_y", 0.1),
        nodal("tip-rz", "tip", "RZ", 0.01),
    ]);
    let request = request_of(model);
    let view = guard_view(&request, 0);
    let root = view.row("root:RZ");
    assert!(
        root.formed && !root.net_defect.is_zero(),
        "precondition: a formed defect"
    );
    let own = decide_row(root, view.scales[&root.dof]);
    let free = decide_row(root, view.scales[&view.dof("tip:RZ")]);
    assert!(
        !own.fires,
        "root RZ is silent under the all-rows scale: {own:?}"
    );
    assert!(
        free.fires,
        "precondition: it fires under the free-row scale: {free:?}"
    );
    assert!(!view.any_fires());
    for entry in [Entry::Captured, Entry::Typed] {
        for mode in MODES {
            let label = format!("restrained scale {entry:?} {mode:?}");
            let envelope = solved(entry, &request, mode);
            assert_ordinary_passed(&envelope, "case", &label);
            assert_not_demoted_by_s11g(&envelope, "case", &label);
        }
    }
}

/// T21 (G-3, unit): a recovery input admissible to retained scope (N05's
/// nodal torques, S11-F's F4) is selected with no finding; with a synthetic
/// load-row finding the selection is declined into a failure at the source
/// closure stage (`source_validation`), unsupported (`unsupported_family`),
/// with its executed work kept charged.
#[test]
fn t21_selection_is_declined_for_a_formation_finding() {
    use open_pipe_stress_frame_kernel::structural::exact_boundary::Limits;
    let request = n05_request(vec![n05_torques()]);
    let Prepared { model, built, .. } = prepared(&request);
    let boundary = prepare_boundary(built.nodes.len(), &built.supports);
    let mut stiffness = assemble_global_stiffness_with_connectors(
        built.nodes.len(),
        &built.frame_elements,
        &built.connectors,
    )
    .unwrap();
    for spring in &boundary.springs {
        let dof = spring.node_dof.global_index();
        stiffness[dof][dof] += spring.stiffness.value;
    }
    let mut diagnostics = Vec::new();
    let primitives =
        build_load_case_primitive_loads(&model, &model.load_cases[0], &mut diagnostics);
    let loads = prepare_loads(built.nodes.len(), built.pipes.len(), &primitives);
    let force = nodal_and_eigen_case_force(&loads, &[], &built).unwrap();
    let prescribed: Vec<_> = boundary.restrained_dofs.iter().map(|&d| (d, 0.0)).collect();
    let free: Vec<usize> = (0..force.len())
        .filter(|d| !prescribed.iter().any(|(p, _)| p == d))
        .collect();
    let input = || source_recovery::Input {
        model: &model,
        built: &built,
        stiffness: &stiffness,
        force: &force,
        free: &free,
        prescribed: &prescribed,
        spring_entries: &boundary.springs,
        load_case: &model.load_cases[0],
        load_application: &loads,
        thermal_loads: &[],
        load_state: None,
    };
    let finding = formation_guard::FormationFinding {
        guard: formation_guard::Guard::LoadRow,
        sentence: String::new(),
        fired: vec!["synthetic".into()],
    };
    // Precondition: without a finding the attempt is selected.
    let selected =
        source_recovery::solve(input(), Limits::default()).expect("admissible and selected");
    assert!(decline_for_formation(selected, None).is_ok());
    let selected = source_recovery::solve(input(), Limits::default()).unwrap();
    let work = selected.summary().work;
    assert!(work.charged > 0);
    let failure = match decline_for_formation(selected, Some(&finding)) {
        Err(failure) => failure,
        Ok(_) => panic!("a guard-fired case must not be selected"),
    };
    assert!(matches!(failure.helper_stage, open_pipe_stress_frame_kernel::structural::exact_boundary::functionals::AttemptStage::SourceClosure));
    match &failure.error {
        source_recovery::RecoveryError::Unsupported(reason) => {
            assert!(
                !reason.contains("coincident") && !reason.contains("transform"),
                "{reason}"
            );
        }
        other => panic!("{other:?}"),
    }
    assert_eq!(
        (failure.work.charged, failure.work.limit),
        (work.charged, work.limit),
        "the executed work stays charged"
    );
}

/// T22 (erratum E-1, ROOT's D22-1 condition): on a Passed, guard-fired case
/// (captured) the invocation budget equals the unguarded route's: the
/// ordinary route never attempts a Passed case, so nothing is charged, no
/// attempt is counted and nothing fails. Kills restoring the charged attempt
/// (M23). UDL-W1e8 itself is now Sensitive at the solve (K-D5's closure,
/// DEL-04-01 kd5-member-relative-bound.md §§1, 4.2; T1), so the Passed
/// guard-fired case is the same body with a soft branch
/// (`udl_body_with_soft_branch`), whose RZ row still fires the load-row guard.
#[test]
fn t22_invocation_budget_equals_main_on_a_passed_guard_fired_case() {
    let mut model = preview_model("t22");
    model["nodes"] = json!([]);
    model["pipe_segments"] = json!([]);
    model["materials"] = json!([material()]);
    model["supports"] = json!([]);
    model["load_cases"][0]["primitive_loads"] = json!([]);
    udl_body_with_soft_branch(&mut model, "", 0.0);
    let request = request_of(model);
    assert!(
        guard_view(&request, 0).decision("S1:RZ").fires,
        "precondition: the load-row guard fires on S1:RZ"
    );
    for mode in MODES {
        let (typed, capture) =
            source_receipt::CapturedInvocation::parse(request.clone(), mode).unwrap();
        let mut budget = SourceRecoveryBudget::default();
        let envelope = run_linear_static_preview_captured(typed, mode, Some(&capture), &mut budget);
        assert_ordinary_passed(&envelope, "case", "T22");
        assert_demoted(&envelope, "case", LOAD_ROW, "T22");
        assert_eq!(
            (
                budget.charged,
                budget.failed_charged,
                budget.attempts,
                budget.rejected
            ),
            (0, 0, 0, 0),
            "{mode:?}: the invocation ledger moved"
        );
    }
}

// HELP_HUMAN's ruling 5 (T4): short stiff bends.
mod short_bend;

// O-13's arc tests under K-D5's unified predicate (the short-bend rule removed).
mod arc_recovery;
