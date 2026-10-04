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
        &built.sections,
        &mut diagnostics,
    );
    let thrust = build_pressure_thrust_loads(&model, load_case, &pipe_map, &built.sections);
    let ledger = case_force_ledger(
        &model,
        &built,
        &application,
        &bends,
        &thrust,
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
    let scales = formation_guard::moment_scales(
        &owned,
        &formation_entity_bodies(&model, &built, &bodies),
        &curved,
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
            .and_then(|b| scales.get(&b).copied())
            .unwrap_or(0.0);
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
        "schema_version": "0.1.0", "document_kind": "openpipestress.product_preview.model",
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

fn pressure(id: &str, pipe: &str, value: f64) -> Value {
    json!({"id": id, "category": "pressure", "target": {"type": "element", "pipe": pipe},
        "direction": "global_x", "magnitude": {"value": value, "unit": "Pa"},
        "dimension": "pressure", "provenance": INVENTED})
}

fn request_of(model: Value) -> Value {
    json!({"model": model, "materials": []})
}

fn rf_cancel_data() -> Value {
    serde_json::from_str(include_str!("../tests/fixtures/s11f/rf_cancel_cases.json")).unwrap()
}

fn rf_request(id: &str) -> Value {
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
fn rb_prime_clauses_at_their_boundaries() {
    use formation_guard::{rb_prime_fires, CRITERION};
    // First clause, B > RD(1e-9) q exactly: equality is silent, one step above fires.
    assert!(!rb_prime_fires(1.0, CRITERION, 1.0));
    assert!(rb_prime_fires(1.0, CRITERION.next_up(), 1.0));
    // A B/q between 1e-9 and 1e-6 fires (X7 moves the threshold to 1e-6).
    assert!(rb_prime_fires(1.0, 2e-9, 1.0));
    assert!(rb_prime_fires(1.0, 5e-7, 1.0));
    // Resolution clause, q > 2^10 B: q at 2^10 B is unresolved and silent (X4 drops it).
    let at_noise = 1.0 / 1024.0;
    assert!(!rb_prime_fires(1.0, at_noise, 1.0));
    assert!(rb_prime_fires(1.0, at_noise.next_down(), 1.0));
    assert!(!rb_prime_fires(1.0, 0.5, 1.0));
    // Floor clause, q >= 2^-34 S*: at the floor it fires, one step below it is silent.
    let floor_scale = 2f64.powi(34);
    assert!(rb_prime_fires(1.0, 1e-6, floor_scale));
    assert!(!rb_prime_fires(1.0, 1e-6, floor_scale.next_up()));
    // A non-finite B fires (fail-closed).
    assert!(rb_prime_fires(1.0, f64::INFINITY, 1.0));
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
    for captured in [false, true] {
        for nonlinear in [false, true] {
            for combinations in [false, true] {
                assert_eq!(
                    source_eligible(captured, nonlinear, combinations),
                    captured && !nonlinear && !combinations
                );
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

/// Revision 2.2 (with erratum E-1): on the captured entry, one
/// `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` (info) from the zero-work formation
/// decline and no other retained-source diagnostic; on the typed entry none.
fn assert_source_block_codes(envelope: &MechanicsEnvelope, entry: Entry, label: &str) {
    let found: Vec<&Diagnostic> = envelope
        .diagnostics
        .iter()
        .filter(|d| d.code.starts_with("SOURCE_BLOCK_") || d.code.starts_with("SOURCE_RECOVERY"))
        .collect();
    match entry {
        Entry::Typed => assert!(found.is_empty(), "{label}: {found:?}"),
        Entry::Captured => {
            assert_eq!(found.len(), 1, "{label}: {found:?}");
            assert_eq!(
                found[0].code, "SOURCE_BLOCK_RECOVERY_UNAVAILABLE",
                "{label}"
            );
            assert_eq!(found[0].severity, "info", "{label}");
            assert!(
                found[0].message.contains("formation guard"),
                "{label}: {}",
                found[0].message
            );
        }
    }
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

/// T1: UDL-W1e8 (captured and typed, both modes). Precondition: the ordinary
/// report is Passed, and S1.RZ's exact net defect is V1's independent
/// Fraction figure 2.3594e-8 (S11G_CHECK check 1), about 48x its threshold,
/// with P = 0 (no floor on a UDL row). Pin: Sensitive on every view; no
/// retained-source diagnostic; the only integrity code is SENSITIVE.
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
            assert_ordinary_passed(&envelope, "case", &label);
            assert_demoted(&envelope, "case", LOAD_ROW, &label);
            assert!(
                integrity(&envelope, "case").message.contains("S1:RZ"),
                "{label}"
            );
            assert_source_block_codes(&envelope, entry, &label);
        }
    }
}

/// T2: UDL-W1e80 (typed, both modes; the captured entry refuses it).
/// Precondition: S1.RZ's defect is V1's 2.6328e64 against a 1e-17 threshold.
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
        assert_ordinary_passed(&envelope, "case", &label);
        assert_demoted(&envelope, "case", LOAD_ROW, &label);
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
/// negated operand), so the statistic is the 0.3 term's alone: CHECKS_PASSED.
/// Kills Sum|eps| (M6) and the a-priori bound (M4).
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
                    let envelope = solved(entry, &request, mode);
                    assert_eq!(
                        integrity(&envelope, "case").code,
                        "NUMERICAL_INTEGRITY_CHECKS_PASSED",
                        "G={g} {order:?} {entry:?} {mode:?}"
                    );
                    assert_not_demoted_by_s11g(&envelope, "case", "T5");
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
/// temperature interval (or internal pressure 2 MPa) on every member.
fn collinear_run(end: [f64; 3], stations: &[f64], pressure_run: bool) -> Value {
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
        .map(|i| if pressure_run {
            pressure(&format!("p:{i}"), &format!("m{i}"), 2e6)
        } else {
            thermal(&format!("t:{i}"), &format!("m{i}"), 75.0)
        })
        .collect::<Vec<_>>());
    request_of(model)
}

/// T6a (SF-4): V1's collinear runs and the pressure run stay silent (the
/// pressure run inside the test-only historical pressure scope). Without
/// the floor the self-equilibrated statistic is at least 1e8 on the runs V1
/// found firing (computed here from the ledger rows); with the floor every
/// row is below 1e-3 of its threshold. Kills dropping the floor (M11).
#[test]
fn t6a_collinear_runs_are_silent_with_the_floor() {
    let runs: [([f64; 3], &[f64], bool, bool); 5] = [
        (
            [12.0, 5.0, 0.0],
            &[0.0, 0.13, 0.4, 0.55, 0.81, 1.0],
            false,
            true,
        ),
        ([10.0, 3.7, 2.2], &[0.0, 0.3, 0.55, 0.7, 1.0], false, true),
        ([6.0, 6.0, 0.0], &[0.0, 0.25, 0.5, 0.75, 1.0], false, false),
        // Axis-aligned: FK's normalization gives bit-identical unit vectors
        // here (V1's emulation did not), so there is no noise to silence.
        (
            [9.0, 0.0, 0.0],
            &[0.0, 0.13, 0.4, 0.55, 0.81, 1.0],
            false,
            false,
        ),
        // V1's pressure-thrust run. A fresh solve refuses the legacy nonzero
        // pressure model on every entry before the ledger
        // (`pressure_runtime.rs`, PRESSURE_MODEL_REAUTHOR_REQUIRED), so the
        // straight thrust family is exercised inside the test-only
        // `historical_pressure_reference::with_scope`, as S11-F's F10 does.
        (
            [12.0, 5.0, 0.0],
            &[0.0, 0.13, 0.4, 0.55, 0.81, 1.0],
            true,
            true,
        ),
    ];
    for (end, stations, pressure_run, fires_without_floor) in runs {
        let request = collinear_run(end, stations, pressure_run);
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
                let envelope = if pressure_run {
                    crate::historical_pressure_reference::with_scope(|| {
                        solved(entry, &request, mode)
                    })
                } else {
                    solved(entry, &request, mode)
                };
                assert_eq!(
                    integrity(&envelope, "case").code,
                    "NUMERICAL_INTEGRITY_CHECKS_PASSED",
                    "T6a {end:?} {entry:?} {mode:?}"
                );
                assert_not_demoted_by_s11g(&envelope, "case", "T6a");
            }
        }
        eprintln!(
            "T6a {end:?} pressure={pressure_run}: unfloored {unfloored:e}, floored {floored:e}"
        );
    }
}

/// V1's DB-1 counterexample: S0 (0,0,0), S2 (5,0,0), S3 (3,-2,0), S4 (3,2,0)
/// anchored; S1 (3,0,0) with free translations and restrained rotations. A
/// (S0-S1, L = 3) and B (S1-S2, L = 2) carry uniform global-y loads q_A and
/// q_B; C (S3-S1) and D (S1-S4) carry the same temperature interval (thermal
/// axial load N each, meeting and cancelling at S1); a nodal 1e-3 N at S1.UY.
fn db1(delta_t: f64) -> Value {
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
/// N ~ 1e4, computed here from the ledger rows). Sensitive, naming S1:UY and
/// loads A and B. Kills the whole-row floor (M17).
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
                assert_ordinary_passed(&envelope, "case", &label);
                assert_demoted(&envelope, "case", LOAD_ROW, &label);
                let message = &integrity(&envelope, "case").message;
                assert!(
                    message.contains("S1:UY")
                        && message.contains("load:A")
                        && message.contains("load:B"),
                    "{label}: {message}"
                );
            }
        }
    }
}

/// I4's pinned INPLANE formation rows (S11-F `FORMATION_PINS`): (case, member,
/// location, mode, [My, Mz]).
const INPLANE_PINS: &[(&str, &str, &str, &str, [f64; 2])] = &[
    (
        "RF-CANCEL-F-G1e80-GnG-INPLANE",
        "M1",
        "end_j",
        "dense_scrutiny",
        [0.0, 1.0000036354540498e-08],
    ),
    (
        "RF-CANCEL-F-G1e80-GnG-INPLANE",
        "M1",
        "end_j",
        "sparse_interactive",
        [0.0, 1.0000022143685783e-08],
    ),
    (
        "RF-CANCEL-F-G1e80-GnG-INPLANE",
        "M2",
        "end_i",
        "dense_scrutiny",
        [0.0, -9.999993721976352e-09],
    ),
    (
        "RF-CANCEL-F-G1e80-GnG-INPLANE",
        "M2",
        "end_i",
        "sparse_interactive",
        [0.0, -1.0000043459967856e-08],
    ),
    (
        "RF-CANCEL-M-G1e80-GnG-INPLANE",
        "M2",
        "end_i",
        "dense_scrutiny",
        [0.0, -1.000005767082257e-08],
    ),
    (
        "RF-CANCEL-M-G1e80-GnG-INPLANE",
        "M2",
        "end_i",
        "sparse_interactive",
        [0.0, -1.0000043459967856e-08],
    ),
    (
        "RF-CANCEL-M-G1e80-GnG-INPLANE",
        "M2",
        "end_j",
        "dense_scrutiny",
        [0.0, 9.999993721976352e-09],
    ),
    (
        "RF-CANCEL-M-G1e80-GnG-INPLANE",
        "M2",
        "end_j",
        "sparse_interactive",
        [0.0, 9.999993721976352e-09],
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

/// T11 (B-1; load-bearing: R-b' is the only INPLANE catch after S11-F), and
/// ruling 1's "R-b' alone" test. F- and M-G1e80-GnG-INPLANE, typed, both
/// modes. Precondition: no formed term (the load-row guard cannot fire), the
/// ordinary report is Passed, and R-b' fires on I4's rows from the published
/// values. Pin: Sensitive on every view (code, severity, the one R-b'
/// sentence naming the ends, q, B and S*, the case quality and the envelope
/// status), no load-row sentence, and I4's pinned bits unchanged.
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
            assert_demoted(&envelope, "case", RECOVERY, &label);
            let message = &integrity(&envelope, "case").message;
            assert!(!message.contains(LOAD_ROW), "{label}");
            for pin in &pins {
                let end = if pin.2 == "end_i" { "i" } else { "j" };
                assert!(
                    message.contains(&format!("{}.{end}: q=", pin.1)),
                    "{label}: {message}"
                );
            }
            assert!(message.contains("S*_moment="), "{label}");
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
/// newline): the envelope is byte-identical to the committed raw file in
/// both modes (the no-op rule). R-b's two clauses hold at end i (the design's
/// precondition, asserted); R-b' is silent there anyway, below its floor.
#[test]
fn t13_committed_fallback_uz_is_byte_identical() {
    let request: Value = serde_json::from_str(include_str!(
        "../../reporting/result_export/tests/fixtures/load_reference_fallback_uz.request.json"
    ))
    .unwrap();
    for (mode, committed) in [
        (PreviewSolverMode::SparseInteractive, include_str!("../../reporting/result_export/tests/fixtures/load_reference_fallback_uz-sparse_interactive.raw.json")),
        (PreviewSolverMode::DenseScrutiny, include_str!("../../reporting/result_export/tests/fixtures/load_reference_fallback_uz-dense_scrutiny.raw.json")),
    ] {
        let envelope = run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap();
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
        assert!(text == committed, "{mode:?}: committed bytes changed");
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
    let n0 = format!("{prefix}0");
    let n1 = format!("{prefix}1");
    let bend = format!("{prefix}bend");
    model["nodes"].as_array_mut().unwrap().extend([
        node(&n0, origin, 0.0, 0.0),
        node(&n1, origin + 2.0, 0.0, 0.0),
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
        "geometry": {"bend_pipe_ref": bend, "bend_radius": {"value": std::f64::consts::SQRT_2, "unit": "m"},
            "bend_angle": {"value": std::f64::consts::PI / 2.0, "unit": "rad"},
            "bend_plane_orientation": "global_xy_preview",
            "bend_geometry_source_reference": "invented_user_entered_preview_geometry"},
        "modifiers": {"sif_user_value": {"value": 1.15, "unit": "none"},
            "flexibility_factor_user_value": {"value": 2.0, "unit": "none"},
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

/// T15 (SF-2): a realized curved span carrying a uniform load. Its
/// consistent vector is CannotBound, which demotes: Sensitive with the
/// CannotBound reason. Kills treating CannotBound as a zero defect (M16).
#[test]
fn t15_curved_uniform_load_is_cannot_bound() {
    let mut model = preview_model("curved");
    model["materials"] = json!([material()]);
    curved_body(
        &mut model,
        "c",
        0.0,
        vec![uniform("w:c", "cbend", "global_z", 0.3)],
    );
    let request = request_of(model);
    let view = guard_view(&request, 0);
    assert!(
        view.rows.iter().any(|r| !r.cannot_bound_sources.is_empty()),
        "precondition: CannotBound terms"
    );
    for entry in [Entry::Captured, Entry::Typed] {
        for mode in MODES {
            let label = format!("T15 {entry:?} {mode:?}");
            let envelope = solved(entry, &request, mode);
            assert_ordinary_passed(&envelope, "case", &label);
            assert_demoted(&envelope, "case", LOAD_ROW, &label);
            assert!(
                integrity(&envelope, "case").message.contains("CannotBound"),
                "{label}"
            );
        }
    }
}

/// Ruling 1: both guards fire. The F-INPLANE model (R-b' fires on its
/// members) plus a separate body carrying a curved uniform load (the
/// load-row guard fires with CannotBound). The load-row demotion comes
/// first; R-b''s later amendment is a no-op: its sentence does not appear.
#[test]
fn ruling1_both_guards_fire_and_the_load_row_sentence_stands() {
    let mut request = rf_request("RF-CANCEL-F-G1e80-GnG-INPLANE");
    curved_body(
        &mut request["model"],
        "c",
        100.0,
        vec![uniform("w:c", "cbend", "global_z", 0.3)],
    );
    let view = guard_view(&request, 0);
    assert!(view.any_fires(), "precondition: the load-row guard fires");
    for mode in MODES {
        let label = format!("both guards {mode:?}");
        let envelope = solved(Entry::Typed, &request, mode);
        assert_ordinary_passed(&envelope, "case", &label);
        let ends = rb_view(&request, &envelope, "case");
        assert!(
            ends.iter().any(EndView::rb_prime),
            "{label}: precondition, R-b' fires too"
        );
        assert_demoted(&envelope, "case", LOAD_ROW, &label);
        assert!(
            !integrity(&envelope, "case").message.contains(RECOVERY),
            "{label}: R-b' is a no-op"
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

/// The RF-CANCEL case-modes S11-G demotes, as (entry, case, mode). The 14
/// formation rows sit in the first 7 (entry, case) pairs' case-modes.
const EXPECTED_DEMOTED: &[(&str, &str)] = &[
    ("captured", "RF-CANCEL-UDL-W1e8"),
    ("typed", "RF-CANCEL-UDL-W1e8"),
    ("typed", "RF-CANCEL-UDL-W1e80"),
    ("typed", "RF-CANCEL-F-G1e80-GnG-INPLANE"),
    ("typed", "RF-CANCEL-M-G1e80-GnG-INPLANE"),
];

/// T16 (extended from S11-F's F1/F11/F12). Every RF-CANCEL case through both
/// entries and both modes: all 14 formation rows are published in a
/// non-Passed case (6 UDL rows by the load-row guard, 8 INPLANE rows by
/// R-b'), every row outside the formation list meets R1's predicate (so the
/// S11 list stays empty and there is no breach outside the lists), and the
/// set of case-modes S11-G demotes is exactly `EXPECTED_DEMOTED` in both
/// modes.
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
                if d.message.contains("S11-G") {
                    assert_eq!(d.code, "NUMERICAL_INTEGRITY_SENSITIVE");
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
    // Every formation row is checked and published in a non-Passed case.
    for (entry, case, key, mode) in &formation {
        let slot = (entry.clone(), case.clone(), mode.clone());
        assert!(
            checked.get(&slot).is_some_and(|k| k.contains(key)),
            "{slot:?} {key} not published"
        );
        assert_ne!(
            codes_by_case_mode[&slot], "NUMERICAL_INTEGRITY_CHECKS_PASSED",
            "{slot:?} {key} is published Passed"
        );
        assert!(
            demoted.contains(&slot),
            "{slot:?} {key}: not demoted by S11-G"
        );
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
    assert_eq!(demoted, expected, "the set of case-modes S11-G demotes");
}

// ============================================ revision 2.2 routing (T18-T22)

fn receipt_case<'a>(envelope: &'a MechanicsEnvelope, case: &str) -> &'a Value {
    envelope.source_block_recovery.as_ref().expect("a receipt")["body"]["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["basis_ref"]["ref_id"] == json!(case))
        .unwrap_or_else(|| panic!("receipt case {case}"))
}

fn diagnostics_of<'a>(envelope: &'a MechanicsEnvelope, case: &str) -> Vec<&'a Diagnostic> {
    envelope
        .diagnostics
        .iter()
        .filter(|d| d.affected_refs.iter().any(|r| r == case))
        .collect()
}

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
/// modes: case A (N05's cancelling torques) is source-selected; case B is
/// already Sensitive (N05's condition) and its load-row guard fires.
/// Precondition: case B's guard fires and its report is Sensitive (2.1's gate
/// refused this invocation, M19). Pin: an envelope with a receipt, case A
/// qualified, case B unsupported with ordinary outcome `sensitive`; no
/// blocking diagnostic; case B keeps today's integrity record (the no-op
/// rule, no S11-G sentence) and main's real refused attempt (its failure is
/// retained scope's, not the formation decline). Byte equality of case B with
/// the unguarded base is run evidence (the fixture-diff harness).
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
        assert_eq!(
            receipt_case(&envelope, "case")["outcome"],
            json!("qualified"),
            "{label}"
        );
        let entry = receipt_case(&envelope, "case-b");
        assert_eq!(entry["outcome"], json!("unsupported"), "{label}: {entry}");
        assert_eq!(
            entry["ordinary_attempt"]["outcome"],
            json!("sensitive"),
            "{label}"
        );
        let unavailable: Vec<_> = diagnostics_of(&envelope, "case-b")
            .into_iter()
            .filter(|d| d.code == "SOURCE_BLOCK_RECOVERY_UNAVAILABLE")
            .collect();
        assert_eq!(unavailable.len(), 1, "{label}");
        assert!(
            unavailable[0].message.contains("non-nodal"),
            "{label}: {}",
            unavailable[0].message
        );
        assert!(
            !unavailable[0].message.contains("formation guard"),
            "{label}"
        );
    }
}

/// T19 (path 1, load-row variant; revision 2.2). Per-case modulus bases:
/// case A (N05's cancelling torques, base basis) is Sensitive by N05's soft
/// torsion spring and source-selected; case B (the tip formation noise) runs
/// on an invented soft basis (E = 1 Pa, G = 0.4 Pa) on which the same model
/// is well conditioned, so its report is Passed, and its load-row guard
/// fires. Pin: no refusal; case B `SENSITIVE`; its receipt entry is
/// `unsupported` with ordinary outcome `sensitive`, stage
/// `source_validation`, code `unsupported_family` and zero work (erratum
/// E-1); case A qualified.
#[test]
fn t19_path1_load_row_variant_is_not_refused() {
    let request = with_basis(
        n05_request(vec![n05_torques(), n05_tip_noise("case-b")]),
        "case-b",
        "point:soft",
        1.0,
        0.4,
    );
    assert!(
        guard_view(&request, 1).decision("tip:RZ").fires,
        "precondition: case B's guard fires"
    );
    for mode in MODES {
        let label = format!("T19 {mode:?}");
        let envelope = solved(Entry::Captured, &request, mode);
        assert_ordinary_passed(&envelope, "case-b", &label);
        assert!(
            envelope
                .diagnostics
                .iter()
                .any(|d| d.code == "SOURCE_BLOCK_RECOVERY_SELECTED"
                    && d.affected_refs == vec!["case".to_string()]),
            "{label}: precondition, case A is selected: {:?}",
            diagnostics_of(&envelope, "case")
                .iter()
                .map(|d| format!("{}: {}", d.code, d.message))
                .collect::<Vec<_>>()
        );
        assert_demoted(&envelope, "case-b", LOAD_ROW, &label);
        assert_eq!(
            receipt_case(&envelope, "case")["outcome"],
            json!("qualified"),
            "{label}"
        );
        let entry = receipt_case(&envelope, "case-b");
        assert_eq!(entry["outcome"], json!("unsupported"), "{label}: {entry}");
        assert_eq!(
            entry["ordinary_attempt"]["outcome"],
            json!("sensitive"),
            "{label}"
        );
        assert_eq!(
            entry["failure"]["stage"],
            json!("source_validation"),
            "{label}"
        );
        assert_eq!(
            entry["failure"]["code"],
            json!("unsupported_family"),
            "{label}"
        );
        assert_eq!(entry["work"]["limit"], json!(0), "{label}");
        assert_eq!(entry["work"]["charged"], json!(0), "{label}");
        assert_eq!(
            entry["work"]["rejected_reservation"]["amount"],
            json!(0),
            "{label}"
        );
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

/// T20 (ruling 3, which stands; ROOT's ruling on RV4's finding):
/// CHARACTERIZATION OF A KNOWN RESIDUAL, NOT DESIRED BEHAVIOUR. The residual
/// is path 1's R-b' variant: a case whose report is Passed and which R-b'
/// demotes after routing, beside a source-selected case. Its `ordinary`
/// receipt entry then fails the wire binding, and receipt finalization
/// refuses the invocation. It is REACHABLE (RV4's construction C1),
/// FAIL-CLOSED (`Err("SOURCE_BLOCKS_FINALIZATION_FAILED")`, no envelope and
/// no published value). C1 uses per-case modulus bases; reach with a single
/// modulus basis (via FK's load audit, or via K-D5's formation check) is not
/// refuted. The residual needs a pre-0.4 captured invocation (a 0.4.0
/// captured invocation is republished under CP3 SF-1). Owner: T3's composite
/// SOURCE_BLOCKS_FINALIZATION_FAILED item (the "demoted-ordinary" receipt
/// form), which must close before T3 closes.
///
/// C1 at m = 1e-7 N*m and F = 1 N. The precondition (paths differ): on the
/// typed entry (no receipt) case B's report is Passed, R-b' fires at the tip
/// end, and the published tip moment carries a genuine relative error above
/// the criterion against its exact value |m| (a free tip's end moment
/// equals the applied tip moment, and its y component is zero). A
/// single-case captured invocation of case B is demoted and not refused. The
/// control (m = 0.5, R-b' silent) publishes with a receipt, case A qualified.
#[test]
fn t20_characterization_rb_prime_residual_c1() {
    let (force, moment) = (1.0, 1e-7);
    let request = c1_request(force, moment);
    let model: PreviewModel = serde_json::from_value(request["model"].clone()).unwrap();
    assert!(!case_state::is_load_state(&model), "pre-0.4 only");
    for mode in MODES {
        let label = format!("T20 {mode:?}");
        // Precondition and typed pin: case B Passed in its report, R-b' fires
        // on a genuinely inaccurate row, published Sensitive, no receipt.
        let typed = solved(Entry::Typed, &request, mode);
        assert!(typed.source_block_recovery.is_none(), "{label}");
        assert_ordinary_passed(&typed, "case-b", &label);
        let tip = rb_view(&request, &typed, "case-b")
            .into_iter()
            .find(|e| e.member == "pipe" && e.end == "j")
            .expect("the tip end");
        assert!(tip.rb_prime(), "{label}: R-b' fires at the tip: {tip:?}");
        // q and |m| lie within a factor of two, so q - m is exact (Sterbenz).
        let relative = (tip.q - moment).abs() / moment;
        assert!(
            relative > formation_guard::CRITERION,
            "{label}: the tip row's relative error {relative:e} is genuine"
        );
        assert_demoted(&typed, "case-b", RECOVERY, &label);
        // Case B alone on the captured entry: demoted, not refused.
        let alone = with_basis(
            n05_request(vec![c1_case_b(force, moment)]),
            "case-b",
            "point:soft",
            1.0,
            0.4,
        );
        let single = solved(Entry::Captured, &alone, mode);
        assert_ordinary_passed(&single, "case-b", &label);
        assert_demoted(&single, "case-b", RECOVERY, &label);
        // The residual: the two-case captured invocation is refused, fail-closed.
        match run(Entry::Captured, &request, mode) {
            Err(error) => assert_eq!(error, "SOURCE_BLOCKS_FINALIZATION_FAILED", "{label}"),
            Ok(_) => {
                panic!("{label}: the residual is expected to refuse (update T20 when it closes)")
            }
        }
        // Control: m = 0.5 (R-b' silent) publishes with a receipt.
        let control = solved(Entry::Captured, &c1_request(force, 0.5), mode);
        assert_eq!(
            receipt_case(&control, "case")["outcome"],
            json!("qualified"),
            "{label}"
        );
        assert_eq!(
            receipt_case(&control, "case-b")["outcome"],
            json!("qualified"),
            "{label}"
        );
        assert_not_demoted_by_s11g(&control, "case-b", &label);
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
        "../../../fixtures/product_preview/load_reference/pressure.request.json"
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
        &built.sections,
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
        "../../../fixtures/product_preview/source_blocks/ui/n06-sparse_interactive.request.json"
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
        // Pin: main's real attempt, retained scope's refusal, charged.
        let unavailable: Vec<_> = diagnostics_of(&envelope, "case-b")
            .into_iter()
            .filter(|d| d.code == "SOURCE_BLOCK_RECOVERY_UNAVAILABLE")
            .collect();
        assert_eq!(unavailable.len(), 1, "{label}");
        let message = &unavailable[0].message;
        assert!(
            message.contains("non-nodal") && !message.contains("formation guard"),
            "{label}: main's real attempt, not the zero-work decline: {message}"
        );
        assert!(
            !message.contains("charged: 0,"),
            "{label}: main's real attempt is charged: {message}"
        );
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
    let mut stiffness = assemble_global_stiffness_with_user_elements(
        built.nodes.len(),
        &built.frame_elements,
        &built.user_stiffness_elements,
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
        pressure_thrust_loads: &[],
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
/// (UDL-W1e8, captured) the invocation budget equals the unguarded route's:
/// the ordinary route never attempts a Passed case, so nothing is charged,
/// no attempt is counted and nothing fails. Kills restoring the charged
/// attempt (M23).
#[test]
fn t22_invocation_budget_equals_main_on_a_passed_guard_fired_case() {
    let request = rf_request("RF-CANCEL-UDL-W1e8");
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
