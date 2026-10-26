//! W-11 (DEL-08-01 Design `exact-route-report-sections.md` §2.2): the exact
//! routes' limitation coverage fixture (`fixtures/reports/exact_route_limitations.json`),
//! generated from the public producer the desktop and the runner use
//! (`run_linear_static_preview_value_with_mode`) on invented models, one per
//! shape the producer can emit; the certified-recovery statements entering
//! each published list exactly once; and the guard that the producer emits no
//! `pressure-source-1` list for a model with an objective connector or a
//! realized bend (source recovery refuses both, `src/source_recovery.rs`).
//! The report's statement table
//! (`apps/desktop/src/features/report/exactReportLimitations.ts`) is checked
//! against this fixture. With `OPENPIPESTRESS_WRITE_EXACT_ROUTE_LIMITATIONS=<path>`
//! the fixture test writes the fixture there instead of comparing (the
//! maintained writer). No product source is changed or read here; every input
//! is a committed invented request or a copy of one patched inline. Standard
//! claim fence applies (claims taxonomy per DEC-081).

use open_pipe_stress_product_physics::{
    preview_formulation_basis, run_linear_static_preview_value_with_mode, PreviewSolverMode,
};
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::sync::OnceLock;

#[path = "common/current_model.rs"]
mod current_model;

const FIXTURE: &str = include_str!("../../../fixtures/reports/exact_route_limitations.json");
const WRITE_ENV: &str = "OPENPIPESTRESS_WRITE_EXACT_ROUTE_LIMITATIONS";
const GENERATOR: &str =
    "core/product_physics/tests/exact_route_limitations.rs::exact_route_limitations_fixture_is_current";
const PRESSURE_2: &str = "openpipestress.result_semantics/0.3.0/pressure-2";
const PRESSURE_SOURCE_1: &str = "openpipestress.result_semantics/0.3.0/pressure-source-1";
/// The source method's named declines of a realized bend and of an objective
/// connector (`src/source_recovery.rs`, ahead of its dimension closure; the
/// later component-family refusal stands behind them).
const BEND_REFUSAL: &str = "realized curved bend (curved_bend_macro_element) is not a straight source family";
const CONNECTOR_REFUSAL: &str = "objective connector is not a straight source family";
const PROVENANCE: &str = "invented W-11 exact-route limitation coverage input; not library, component, catalog or code-rule data";

// Invented inputs (committed requests; frozen references are read, never edited).
/// Straight `pressure-2`, without the load/reference-state group.
const STRAIGHT_REQUEST: &str = include_str!("fixtures/exact_pressure_connected_request.json");
/// Straight `pressure-2`, with the load/reference-state group.
const LOAD_STATE_REQUEST: &str =
    include_str!("../../../fixtures/product_preview/pressure/load_reference/pressure.request.json");
/// The T4-I7 L line (a realized bend): 0.3.0 without and 0.4.0 with the group.
const I7_DOCUMENTS: &str = include_str!("../../../validation/references/t4_i7/u2_document_sketches.json");
/// The T4-I12 connector system documents (0.3.0 without and 0.4.0 with the group).
const U3_REFERENCES: &str = include_str!("../../../validation/references/t4_i12/u3_reference_cases.json");
/// The retained-source controls the producer selects (`pressure-source-1`).
const SOURCE_REQUEST: &str =
    include_str!("../../../fixtures/product_preview/pressure_source/physics_source/n05.request.json");
const LOAD_STATE_SOURCE_REQUEST: &str =
    include_str!("../../../fixtures/product_preview/pressure_source/load_reference_source/n05.request.json");
/// A pressure-free (`preview-physics-2`) model.
const PRESSURE_FREE_MODEL: &str = include_str!("fixtures/preview_physics_invented_model.json");

/// The committed exact corpora (§1.3), each with its published limitation
/// lists found wherever an object carries a producer and a formulation basis.
const EXACT_CORPORA: [(&str, &str); 15] = [
    ("pressure_v3_straight_reader_corpus", include_str!("../../../fixtures/results/pressure_v3_straight_reader_corpus.json")),
    ("pressure_v3_arc_reader_corpus", include_str!("../../../fixtures/results/pressure_v3_arc_reader_corpus.json")),
    ("pressure_v3_connector_reader_corpus", include_str!("../../../fixtures/results/pressure_v3_connector_reader_corpus.json")),
    ("pressure_v3_joint_reader_control", include_str!("../../../fixtures/results/pressure_v3_joint_reader_control.json")),
    ("pressure_source_load_state_eigen_motion_dense", include_str!("../../../fixtures/results/pressure_source_load_state_eigen_motion_dense.document.json")),
    ("pressure_source_load_state_eigen_motion_sparse", include_str!("../../../fixtures/results/pressure_source_load_state_eigen_motion_sparse.document.json")),
    ("pressure_source_load_state_fields_dense", include_str!("../../../fixtures/results/pressure_source_load_state_fields_dense.document.json")),
    ("pressure_source_load_state_fields_sparse", include_str!("../../../fixtures/results/pressure_source_load_state_fields_sparse.document.json")),
    ("pressure_source_load_state_mixed_dense", include_str!("../../../fixtures/results/pressure_source_load_state_mixed_dense.document.json")),
    ("pressure_source_load_state_mixed_sparse", include_str!("../../../fixtures/results/pressure_source_load_state_mixed_sparse.document.json")),
    ("pressure_source_load_state_n05_dense", include_str!("../../../fixtures/results/pressure_source_load_state_n05_dense.document.json")),
    ("pressure_source_load_state_n05_sparse", include_str!("../../../fixtures/results/pressure_source_load_state_n05_sparse.document.json")),
    ("pressure_source_load_state_n06_dense", include_str!("../../../fixtures/results/pressure_source_load_state_n06_dense.document.json")),
    ("pressure_source_load_state_n06_sparse", include_str!("../../../fixtures/results/pressure_source_load_state_n06_sparse.document.json")),
    ("physics_source_n05_sparse_raw", include_str!("../../../fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json")),
];
/// A preview-physics-2 envelope: the recovery statements end its list too.
const PREVIEW_ENVELOPE: &str = include_str!("../../../fixtures/results/preview_physics_connected_sparse.json");

#[derive(Serialize)]
struct CoverageFixture {
    schema_version: &'static str,
    generator: &'static str,
    lists: Vec<CoverageList>,
    strings: Vec<String>,
}

#[derive(Serialize, Clone, PartialEq, Eq, Debug)]
struct CoverageList {
    semantic_contract_id: String,
    load_reference_states: bool,
    objective_connectors: bool,
    realized_bends: bool,
    limitations: Vec<String>,
}

/// (identity, load/reference states, objective connectors, realized bends).
type Shape = (&'static str, bool, bool, bool);

fn parse(text: &str) -> Value {
    serde_json::from_str(text).unwrap()
}

/// The public producer's envelope for `request`, as the desktop and the runner
/// publish it.
fn run(request: Value) -> Value {
    let envelope = run_linear_static_preview_value_with_mode(request, PreviewSolverMode::SparseInteractive)
        .unwrap_or_else(|error| panic!("the producer refused: {error}"));
    serde_json::to_value(envelope).unwrap()
}

/// The shape a published envelope states: its identity, the load/reference-state
/// evidence, its connector records and its realized curved bends.
fn published_shape(envelope: &Value) -> (String, bool, bool, bool) {
    let evidence = &envelope["contract_evidence"];
    (
        envelope["producer"]["semantic_contract_id"].as_str().unwrap().to_string(),
        evidence.get("load_reference_states").is_some(),
        evidence["connector"].as_array().is_some_and(|records| !records.is_empty()),
        envelope["summary"]["curved_bend_macro_element_count"].as_u64().unwrap() > 0,
    )
}

/// The blocking diagnostics, for a failure message.
fn blocking(envelope: &Value) -> String {
    let codes: Vec<String> = envelope["diagnostics"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|d| d["severity"] == "blocking")
        .map(|d| format!("{}: {}", d["code"], d["message"]))
        .collect();
    codes.join("; ")
}

fn limitations(envelope: &Value) -> Vec<String> {
    envelope["formulation_basis"]["limitations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|text| text.as_str().unwrap().to_string())
        .collect()
}

fn i7(id: &str, version: &str) -> Value {
    current_model::reauthor_request(parse(I7_DOCUMENTS)["sketches"][id][version]["document"].clone())
}

fn u3(case: &str, key: &str) -> Value {
    current_model::reauthor_request(parse(U3_REFERENCES)["cases"][case]["inputs"][key].clone())
}

/// Adds `pipe` to the model, with a member reference and an element state
/// copied from the first pipe's wherever the model carries the
/// load/reference-state group. The added member's thermal state is the
/// unchanged reference (a replaced span's eigenstrain has no owner,
/// `JOINT_REPLACED_SPAN_LOAD_UNOWNED`).
fn add_pipe(model: &mut Value, pipe: Value) {
    let id = pipe["id"].clone();
    model["pipe_segments"].as_array_mut().unwrap().push(pipe);
    for configuration in model.get_mut("reference_configurations").and_then(Value::as_array_mut).into_iter().flatten() {
        let references = configuration["member_references"].as_array_mut().unwrap();
        let mut reference = references[0].clone();
        reference["pipe_ref"] = id.clone();
        references.push(reference);
    }
    for case in model["load_cases"].as_array_mut().unwrap() {
        if let Some(states) = case.get_mut("analysis_state").and_then(|state| state.get_mut("element_states")).and_then(Value::as_array_mut) {
            let mut state = states[0].clone();
            state["pipe_ref"] = id.clone();
            state["thermal_state"] = json!({"kind": "unchanged_reference", "provenance": PROVENANCE});
            states.push(state);
        }
    }
}

fn add_node(model: &mut Value, id: &str, x: f64, y: f64, z: f64) {
    model["nodes"].as_array_mut().unwrap().push(json!({"id": id, "position": {"x": x, "y": y, "z": z}, "provenance": PROVENANCE}));
}

/// A straight pipe from `from` to `to` with the first pipe's section and material.
fn pipe_like_first(model: &Value, id: &str, from: &str, to: &str, y_reference: Value) -> Value {
    let mut pipe = model["pipe_segments"][0].clone();
    pipe["id"] = json!(id);
    pipe["from"] = json!(from);
    pipe["to"] = json!(to);
    pipe["y_reference"] = y_reference;
    pipe["provenance"] = json!(PROVENANCE);
    if let Some(label) = pipe.get_mut("label") {
        *label = json!(format!("Invented {id}"));
    }
    pipe
}

/// An invented objective connector replacing `span` from `node_i` to
/// `node_j`, whose first connector axis (Q's first column) is the span
/// direction (the form of `pressure_arc_readers.rs`'s connector).
fn connector(span: &str, node_i: &str, node_j: &str, axes: Value) -> Value {
    json!({"id": "component:invented-connector", "label": "Invented expansion joint (objective connector)", "kind": "expansion_joint",
        "node": node_j, "provenance": PROVENANCE,
        "objective_connector": {"version": "1.0.0", "motion_basis": "symmetric_midpoint_small_rotation_v1",
            "end_i": {"node_ref": node_i, "initial_node_axes_global": [[1, 0, 0], [0, 1, 0], [0, 0, 1]], "offset_local": {"x": 0, "y": 0, "z": 0, "unit": "m"}},
            "end_j": {"node_ref": node_j, "initial_node_axes_global": [[1, 0, 0], [0, 1, 0], [0, 0, 1]], "offset_local": {"x": 0, "y": 0, "z": 0, "unit": "m"}},
            "connector_axes_global": axes,
            "installed_reference_temperature": {"value": 20, "unit": "degC"},
            "temperature_applicability": "fixed_installed_parameters_v1", "reference_state": "stress_free",
            "q_ref": {"translation": {"x": 0, "y": 0, "z": 0, "unit": "m"}, "rotation": {"x": 0, "y": 0, "z": 0, "unit": "rad"}},
            "stiffness": {"version": "1.0.0", "representation": "scaled_work_coefficients_v1",
                "translation_scale": {"value": 0.5, "unit": "m"}, "rotation_scale": {"value": 1, "unit": "rad"}, "coefficient_unit": "N*m",
                "upper_triangle": [800000, 0, 0, 0, 0, 0, 225000, 0, 0, 0, 0, 225000, 0, 0, 0, 620000, 0, 0, 480000, 0, 480000],
                "coordinate_order": ["tx", "ty", "tz", "rx", "ry", "rz"],
                "provenance": {"source_reference": PROVENANCE, "measurement_restraints": "invented", "basis_transform_reference": "none"}},
            "topology": {"type": "replaces_span", "span_ref": span},
            "calibration": {"kind": "constant_structural_elasticity_v1", "includes_pressure_dependent_tangent": false, "installed_geometry": "authored model geometry"},
            "hardware": {"kind": "untied"}, "pressure_model": {"kind": "unpressurized"},
            "provenance": {"source_reference": PROVENANCE, "measurement_restraints": "invented", "basis_transform_reference": "none", "validity_statement": "invented analytical control; no applicability claim"}}})
}

/// The L line (`version` 0.3.0, or 0.4.0 with the load/reference-state group)
/// with an added straight tail D→E (+y) replaced by a connector, anchored at
/// A and E, without pressure (a case with a connector and a pressure region is
/// refused, `JOINT_PRESSURE_INTERFACE_UNRESOLVED`).
fn l_line_with_connector(version: &str) -> Value {
    let mut document = i7("U2-L-ANCH-PTW-K2", version);
    let model = &mut document["model"];
    model["project"]["id"] = json!("project:w11-arc-connector");
    add_node(model, "node:E", 3.25, 4.75, 0.0);
    let mut tail = model["pipe_segments"][2].clone();
    tail["id"] = json!("pipe:S3");
    tail["from"] = json!("node:D");
    tail["to"] = json!("node:E");
    tail["provenance"] = json!(PROVENANCE);
    add_pipe(model, tail);
    model["components"].as_array_mut().unwrap().push(connector("pipe:S3", "node:D", "node:E", json!([[0, -1, 0], [1, 0, 0], [0, 0, 1]])));
    rename_support(model, "support:D", "support:E", "node:E");
    for case in model["load_cases"].as_array_mut().unwrap() {
        case["pressure_regions"] = json!([]);
    }
    document
}

fn rename_support(model: &mut Value, from: &str, to: &str, node: &str) {
    for support in model["supports"].as_array_mut().unwrap() {
        if support["id"] == from {
            support["id"] = json!(to);
            support["node"] = json!(node);
        }
    }
    for case in model["load_cases"].as_array_mut().unwrap() {
        for state in case.get_mut("analysis_state").and_then(|state| state.get_mut("support_states")).and_then(Value::as_array_mut).into_iter().flatten() {
            if state["support_ref"] == from {
                state["support_ref"] = json!(to);
            }
        }
    }
}

/// The eight `pressure-2` shapes and the two `pressure-source-1` shapes the
/// producer emits, in fixture order, each with its invented request.
fn producer_inputs() -> Vec<(Shape, Value)> {
    vec![
        ((PRESSURE_2, false, false, false), parse(STRAIGHT_REQUEST)),
        ((PRESSURE_2, false, false, true), i7("U2-L-ANCH-PTW-K2", "0.3.0")),
        ((PRESSURE_2, false, true, false), u3("U3-SYS-DEMO-CONNECTOR-001", "document_v3_0.3.0")),
        ((PRESSURE_2, false, true, true), l_line_with_connector("0.3.0")),
        ((PRESSURE_2, true, false, false), parse(LOAD_STATE_REQUEST)),
        ((PRESSURE_2, true, false, true), i7("U2-L-ANCH-PTW-K2", "0.4.0")),
        ((PRESSURE_2, true, true, false), u3("U3-SYS-DEMO-CONNECTOR-002-LR1", "document_v3_0.4.0")),
        ((PRESSURE_2, true, true, true), l_line_with_connector("0.4.0")),
        ((PRESSURE_SOURCE_1, false, false, false), parse(SOURCE_REQUEST)),
        ((PRESSURE_SOURCE_1, true, false, false), parse(LOAD_STATE_SOURCE_REQUEST)),
    ]
}

/// The producer's list for each emitted shape, checked against the shape the
/// envelope itself states.
fn produced_lists() -> &'static [CoverageList] {
    static LISTS: OnceLock<Vec<CoverageList>> = OnceLock::new();
    LISTS.get_or_init(|| {
        producer_inputs()
            .into_iter()
            .map(|((id, load_state, connectors, bends), request)| {
                let envelope = run(request);
                assert_eq!(
                    envelope["status"]["mechanics"],
                    "MECHANICS_SOLVED",
                    "{id} {load_state} {connectors} {bends}: {}",
                    blocking(&envelope)
                );
                assert_eq!(
                    published_shape(&envelope),
                    (id.to_string(), load_state, connectors, bends),
                    "the envelope states its intended shape"
                );
                assert_eq!(envelope["formulation_basis"]["profile_id"], "exact_pressure_v3");
                CoverageList {
                    semantic_contract_id: id.to_string(),
                    load_reference_states: load_state,
                    objective_connectors: connectors,
                    realized_bends: bends,
                    limitations: limitations(&envelope),
                }
            })
            .collect()
    })
}

fn coverage_fixture() -> CoverageFixture {
    let lists = produced_lists().to_vec();
    // BTreeSet<String> orders by bytes, which for UTF-8 is code-point order.
    let strings: BTreeSet<String> = lists.iter().flat_map(|list| list.limitations.iter().cloned()).collect();
    CoverageFixture { schema_version: "1.0.0", generator: GENERATOR, lists, strings: strings.into_iter().collect() }
}

/// Every published (identity, limitations) pair in `document`.
fn published_lists(document: &Value, out: &mut Vec<(String, Vec<String>)>) {
    match document {
        Value::Object(object) => {
            let id = object.get("producer").and_then(|producer| producer["semantic_contract_id"].as_str());
            let limitations = object.get("formulation_basis").and_then(|basis| basis["limitations"].as_array());
            if let (Some(id), Some(limitations)) = (id, limitations) {
                let texts = limitations.iter().map(|text| text.as_str().unwrap().to_string()).collect();
                out.push((id.to_string(), texts));
            }
            object.values().for_each(|value| published_lists(value, out));
        }
        Value::Array(items) => items.iter().for_each(|value| published_lists(value, out)),
        _ => {}
    }
}

fn corpus_lists() -> Vec<(String, String, Vec<String>)> {
    let mut found = Vec::new();
    for (name, text) in EXACT_CORPORA {
        let mut lists = Vec::new();
        published_lists(&parse(text), &mut lists);
        assert!(!lists.is_empty(), "{name} publishes no limitation list");
        found.extend(lists.into_iter().map(|(id, list)| (name.to_string(), id, list)));
    }
    found
}

#[test]
fn exact_route_limitations_fixture_is_current() {
    let fixture = coverage_fixture();
    // §2.2: the ten shapes the producer emits. The six pressure-source-1 shapes
    // with a connector or a realized bend are never emitted (see the guard).
    assert_eq!(fixture.lists.len(), 10);
    assert_eq!(fixture.strings.len(), 19, "§1.3: 2 + 6 + 2 + 5 + 1 + 1 + 2 distinct strings");
    for list in &fixture.lists {
        assert!((10..=17).contains(&list.limitations.len()), "{} entries", list.limitations.len());
    }
    // Against real envelopes: every published exact list in the committed
    // corpora equals one fixture list of its own identity.
    let mut shapes = BTreeSet::new();
    for (name, id, published) in corpus_lists() {
        let matched = fixture
            .lists
            .iter()
            .find(|list| list.semantic_contract_id == id && list.limitations == published)
            .unwrap_or_else(|| panic!("{name}: a published {id} list equals no fixture list"));
        shapes.insert((id, matched.load_reference_states, matched.objective_connectors, matched.realized_bends));
    }
    let expected: BTreeSet<_> = [
        (PRESSURE_2, false, false, false),
        (PRESSURE_2, true, false, false),
        (PRESSURE_2, false, false, true),
        (PRESSURE_2, true, false, true),
        (PRESSURE_2, false, true, true),
        (PRESSURE_2, false, true, false),
        (PRESSURE_2, true, true, false),
        (PRESSURE_SOURCE_1, true, false, false),
        (PRESSURE_SOURCE_1, false, false, false),
    ]
    .into_iter()
    .map(|(id, l, c, b)| (id.to_string(), l, c, b))
    .collect();
    assert_eq!(shapes, expected, "the committed corpora's published shapes");

    let text = format!("{}\n", serde_json::to_string_pretty(&fixture).unwrap());
    if let Ok(target) = std::env::var(WRITE_ENV) {
        std::fs::write(target, &text).unwrap();
        return;
    }
    assert!(text == FIXTURE, "the committed coverage fixture is stale; regenerate it with {WRITE_ENV}=<path>");
}

/// The route-independent certified-recovery pair: the last two statements of
/// the first produced list.
fn recovery_pair() -> Vec<String> {
    let first = &produced_lists()[0].limitations;
    first[first.len() - 2..].to_vec()
}

fn assert_recovery_once(list: &[String], pair: &[String], what: &str) {
    for statement in pair {
        assert_eq!(list.iter().filter(|text| *text == statement).count(), 1, "{what}: {statement:.40}");
    }
    assert!(list.len() >= 2, "{what}");
    assert_eq!(&list[list.len() - 2..], pair, "{what}: the pair is last");
}

#[test]
fn certified_recovery_statements_enter_once() {
    let pair = recovery_pair();
    assert_eq!(pair.len(), 2);
    assert_ne!(pair[0], pair[1]);
    for list in produced_lists() {
        assert_recovery_once(&list.limitations, &pair, &format!("producer {list:?}"));
    }
    for (name, id, list) in corpus_lists() {
        assert_recovery_once(&list, &pair, &format!("{name} ({id})"));
    }
    let mut preview = Vec::new();
    published_lists(&parse(PREVIEW_ENVELOPE), &mut preview);
    assert_eq!(preview.len(), 1);
    assert_eq!(preview[0].0, "openpipestress.result_semantics/0.3.0/preview-physics-2");
    assert_recovery_once(&preview[0].1, &pair, "preview-physics-2 envelope");
    // The pressure-free route publishes the same pair once and last: the pair
    // is route independent. The public base basis carries neither statement,
    // so the producer appends each exactly once.
    let envelope = run(json!({"model": parse(PRESSURE_FREE_MODEL), "materials": []}));
    assert_eq!(envelope["producer"]["semantic_contract_id"], "openpipestress.result_semantics/0.3.0/preview-physics-2");
    assert_recovery_once(&limitations(&envelope), &pair, "pressure-free producer");
    let base = preview_formulation_basis().limitations;
    assert!(pair.iter().all(|statement| !base.contains(statement)));
}

/// A retained-source control (`n05`, with or without the load/reference-state
/// group) with an objective connector, a realized bend or both added. Each
/// keeps the control's near-singular torsion, so the ordinary check still
/// asks for source recovery.
fn source_control_with(load_state: bool, connectors: bool, bends: bool) -> Value {
    let mut request = parse(if load_state { LOAD_STATE_SOURCE_REQUEST } else { SOURCE_REQUEST });
    let model = &mut request["model"];
    model["project"]["id"] = json!("synthetic:w11-source-family-guard");
    // The straight run ends at the tip (2, 0, 0); a bend turns it to +y.
    let mut end = ("independent-tip", 2.0, 0.0);
    if bends {
        add_node(model, "invented-bend-end", 2.25, 0.25, 0.0);
        add_node(model, "invented-leg-end", 2.25, 2.25, 0.0);
        let bend = pipe_like_first(model, "invented-bend", "independent-tip", "invented-bend-end", json!({"x": 1.0, "y": -1.0, "z": 0.0}));
        add_pipe(model, bend);
        let leg = pipe_like_first(model, "invented-leg", "invented-bend-end", "invented-leg-end", json!({"x": 0.0, "y": 0.0, "z": 1.0}));
        add_pipe(model, leg);
        model["components"].as_array_mut().unwrap().push(json!({"id": "component:invented-bend", "kind": "bend", "node": "independent-tip",
            "geometry": {"bend_pipe_ref": "invented-bend", "bend_radius": {"value": 0.25, "unit": "m"}, "bend_angle": {"value": std::f64::consts::FRAC_PI_2, "unit": "rad"},
                "bend_plane_orientation": "plane of the span chord and the pipe y_reference; the arc bows toward +y_reference"},
            "modifiers": {"flexibility_factor_user_value": {"value": 2.0, "unit": "none"}},
            "mechanics_interface": {"solver_consumption": "curved_bend_macro_element"}, "provenance": PROVENANCE}));
        end = ("invented-leg-end", 2.25, 2.25);
    }
    if connectors {
        let (node, x, y) = end;
        let along_y = bends;
        let (tx, ty) = if along_y { (x, y + 0.5) } else { (x + 0.5, y) };
        add_node(model, "invented-tail-end", tx, ty, 0.0);
        let tail = pipe_like_first(model, "invented-tail", node, "invented-tail-end", json!({"x": 0.0, "y": 0.0, "z": 1.0}));
        add_pipe(model, tail);
        let axes = if along_y { json!([[0, -1, 0], [1, 0, 0], [0, 0, 1]]) } else { json!([[1, 0, 0], [0, 1, 0], [0, 0, 1]]) };
        model["components"].as_array_mut().unwrap().push(connector("invented-tail", node, "invented-tail-end", axes));
    }
    request
}

/// The guard (§2.2): the producer emits no `pressure-source-1` list for a model
/// with an objective connector or a realized bend. For each of the six shapes,
/// the retained-source control with the component added asks for source
/// recovery, the source method refuses it by name, and the case publishes on
/// the ordinary `pressure-2` route with that shape's fixture list. If the
/// producer ever selects such a model, this test fails and the coverage
/// fixture and the statement table must grow.
#[test]
fn pressure_source_1_is_never_emitted_with_a_connector_or_a_realized_bend() {
    let source_only: BTreeSet<&String> = {
        let source: BTreeSet<&String> = produced_lists()
            .iter()
            .filter(|list| list.semantic_contract_id == PRESSURE_SOURCE_1)
            .flat_map(|list| &list.limitations)
            .collect();
        let pressure_2: BTreeSet<&String> = produced_lists()
            .iter()
            .filter(|list| list.semantic_contract_id == PRESSURE_2)
            .flat_map(|list| &list.limitations)
            .collect();
        source.difference(&pressure_2).copied().collect()
    };
    // The pressure-source-1 identity sentence and source paragraph.
    assert_eq!(source_only.len(), 2);
    let mut guarded = 0;
    for load_state in [false, true] {
        for (connectors, bends) in [(true, false), (false, true), (true, true)] {
            let what = format!("load state {load_state}, connector {connectors}, bend {bends}");
            let envelope = run(source_control_with(load_state, connectors, bends));
            let (id, l, c, b) = published_shape(&envelope);
            assert_eq!(id, PRESSURE_2, "{what}: the producer emitted another identity");
            assert_eq!((l, c, b), (load_state, connectors, bends), "{what}");
            let published = limitations(&envelope);
            assert!(published.iter().all(|text| !source_only.contains(text)), "{what}: pressure-source-1 text emitted");
            let fixture_list = produced_lists()
                .iter()
                .find(|list| {
                    list.semantic_contract_id == PRESSURE_2
                        && (list.load_reference_states, list.objective_connectors, list.realized_bends) == (load_state, connectors, bends)
                })
                .unwrap();
            assert_eq!(published, fixture_list.limitations, "{what}: the ordinary route's list");
            // The named refusal: the source method was asked and declined the
            // bend (checked first) or the connector by name.
            let named = if bends { BEND_REFUSAL } else { CONNECTOR_REFUSAL };
            let refusals: Vec<&Value> = envelope["diagnostics"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|d| d["code"] == "SOURCE_BLOCK_RECOVERY_UNAVAILABLE")
                .collect();
            assert!(!refusals.is_empty(), "{what}: source recovery was not attempted");
            for refusal in refusals {
                assert!(refusal["message"].as_str().unwrap().contains(named), "{what}: {}", refusal["message"]);
            }
            // A load-state case states that it publishes the ordinary route.
            if load_state {
                let joined: Vec<&Value> = envelope["diagnostics"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter(|d| d["code"] == "LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED")
                    .collect();
                assert!(!joined.is_empty(), "{what}");
                assert!(joined.iter().all(|d| d["message"].as_str().unwrap().ends_with("retained_source_attempt=unavailable")), "{what}");
            }
            guarded += 1;
        }
    }
    assert_eq!(guarded, 6);
}
