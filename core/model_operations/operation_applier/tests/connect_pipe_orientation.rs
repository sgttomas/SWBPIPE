//! O4 (DEL-07-02 `run-sheet-entry.md` §8): `connect_pipe_run` applies the
//! kernel orientation predicate that `reconnect_pipe_run` applies as R6/R7
//! (DEL-16-06 Design `pipe-reconnect.md` §2.3), so an orientation the solve's
//! frame constructor refuses is refused when the pipe is authored.
//!
//! One table of chords and `y_reference` vectors is run through connect,
//! reconnect, the kernel constructor itself and the product solve; all four
//! must agree. Every model is an inline invented fixture built from
//! `MODEL_SCHEMA_VERSION`; no value is library, catalog or code-rule data.
use open_pipe_stress_frame_kernel::{FrameElement, FrameNode, FrameSection};
use open_pipe_stress_operation_applier::{
    apply_operation, apply_operation_batch, canonical_json, sha256_hex, validate_operation, OperationOutcome,
};
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode, MODEL_SCHEMA_VERSION};
use serde_json::{json, Value};

const PROV: &str = "invented O4 connect-orientation test input";
const CONNECT_ORIENTATION: &str = "OP-CONNECT-PIPE-ORIENTATION-DEGENERATE";
const CONNECT_GEOMETRY: &str = "OP-CONNECT-PIPE-GEOMETRY-DEGENERATE";
const RECONNECT_ORIENTATION: &str = "OP-RECONNECT-ORIENTATION-DEGENERATE";
const RECONNECT_GEOMETRY: &str = "OP-RECONNECT-GEOMETRY-DEGENERATE";

fn hash(model: &Value) -> Value {
    json!({"algorithm":"sha256","canonicalization":"rfc8785_jcs","payload_scope":"model_payload","value":format!("sha256:{}",sha256_hex(&canonical_json(model)))})
}
#[allow(clippy::too_many_arguments)]
fn op(id: &str, kind: &str, change_kind: &str, object: &str, target: &str, path: &str, before: &str, after: &str, unit: &str, dimension: &str) -> Value {
    json!({"operation_id":format!("op:{id}"),"operation_kind":kind,"operation_status":"proposed","author_type":"user",
      "target":{"object_type":object,"ref":target},
      "change":{"change_id":format!("change:{id}"),"change_kind":change_kind,"field_label":path,"field_path":path,"before":before,"after":after,"unit":unit,"dimension":dimension,"source_note":"invented explicit test entry"},
      "validation":{"schema_validation":"not_run","constraint_validation":"not_run","unit_validation":"not_run","diff_preview_status":"not_generated","application_status":"not_applied"},
      "rationale":"invented O4 connect-orientation test operation",
      "audit_boundary":{"mutation_route":"structured_operations_only","direct_model_mutation_allowed":false,"requires_user_acceptance":true,"mutates_accepted_model_state":false},
      "professional_boundary":{"human_review_required":true,"software_makes_compliance_claim":false,"software_makes_certification_claim":false,"software_makes_sealing_claim":false,"software_makes_approval_claim":false,"software_makes_authentication_claim":false}})
}
fn node(id: &str, at: [f64; 3]) -> Value {
    json!({"id": id, "label": id.trim_start_matches("node:"), "position": {"x": at[0], "y": at[1], "z": at[2]}, "provenance": PROV})
}
fn section() -> Value {
    json!({"outside_diameter": {"value": 0.1, "unit": "m"}, "wall_thickness": {"value": 0.005, "unit": "m"}})
}
fn pipe(id: &str, from: &str, to: &str, y: [f64; 3]) -> Value {
    json!({"id": id, "label": id.trim_start_matches("pipe:"), "from": from, "to": to, "section": section(),
        "material": "material:steel", "y_reference": {"x": y[0], "y": y[1], "z": y[2]}, "provenance": PROV})
}
fn anchor(id: &str, node: &str) -> Value {
    json!({"id": id, "node": node, "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV})
}
fn document(unit: &str, nodes: Vec<Value>, pipes: Vec<Value>, supports: Vec<Value>) -> Value {
    json!({"schema_version": MODEL_SCHEMA_VERSION, "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:o4-connect", "units": {"length": unit, "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": nodes, "pipe_segments": pipes, "supports": supports, "components": [],
        "materials": [{"id": "material:steel", "elastic_modulus": {"value": 200e9, "unit": "Pa"}, "shear_modulus": {"value": 80e9, "unit": "Pa"}, "provenance": PROV}],
        "load_cases": [{"id": "case:w", "primitive_loads": [], "provenance": PROV}], "combinations": []})
}
/// The connect payload the desktop emits: every field explicit.
fn connect(id: &str, from: &str, to: &str, y: [f64; 3]) -> Value {
    let payload = json!({"id": id, "label": "Connected", "from": from, "to": to, "section": section(), "material": "material:steel",
        "y_reference": {"x": y[0], "y": y[1], "z": y[2]}, "provenance": PROV});
    op(&format!("connect:{id}"), "connect", "connect_pipe_run", "Element", id, "pipe_segments", "not_present", &payload.to_string(), "m", "length")
}
fn reconnect(model: &Value, pipe: &str, end: &str, target: &str, y: [f64; 3]) -> Value {
    let record = model["pipe_segments"].as_array().unwrap().iter().find(|p| p["id"] == pipe).unwrap();
    let payload = json!({"pipe_ref": pipe, "end": end, "target_node_ref": target, "y_reference": {"x": y[0], "y": y[1], "z": y[2]}});
    op(&format!("reconnect:{pipe}"), "modify", "reconnect_pipe_run", "Element", pipe, "pipe_segments", &canonical_json(record), &payload.to_string(), "none", "dimensionless")
}
fn create_node(id: &str, at: [f64; 3]) -> Value {
    let after = json!({"id": id, "label": "Created node", "position": {"x": at[0], "y": at[1], "z": at[2]}, "provenance": PROV});
    op(&format!("create-node:{id}"), "create", "create_node", "Node", id, "nodes", "not_present", &after.to_string(), "m", "length")
}
fn blocking(outcome: &OperationOutcome) -> Vec<String> {
    outcome.diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| d.code.clone()).collect()
}

/// Validate and apply; a refusal returns no model and no hash, and leaves the
/// input unchanged. Returns the apply outcome.
fn run(model: &Value, intent: &Value) -> OperationOutcome {
    let original = model.clone();
    let claim = hash(model);
    let validate = validate_operation(model, intent, Some(&claim));
    let apply = apply_operation(model, intent, Some(&claim));
    assert_eq!(blocking(&validate), blocking(&apply), "preview shows what apply reports");
    assert_eq!(model, &original);
    if !blocking(&apply).is_empty() {
        assert!(apply.applied_model.is_none() && apply.applied_model_backend_hash.is_none());
        assert_eq!(apply.validation.application_status, "blocked");
    }
    apply
}

/// The connect verdict: `Ok` when applied, else the blocking codes.
fn connect_verdict(model: &Value, from: &str, to: &str, y: [f64; 3]) -> Result<(), Vec<String>> {
    let outcome = run(model, &connect("pipe:NEW", from, to, y));
    if outcome.applied_model.is_some() {
        Ok(())
    } else {
        Err(blocking(&outcome))
    }
}

/// The product solve's verdict on a document that already holds the pipe:
/// `Err` when the solve refuses that pipe's element by name.
/// One explicit nodal force at B is the solve's only load.
fn solve_verdict(model: &Value, pipe_id: &str) -> Result<(), String> {
    let mut loaded = model.clone();
    loaded["load_cases"][0]["primitive_loads"] = json!([{"id": "load:w-b", "category": "concentrated_force", "target": {"type": "node", "node": "node:B"},
        "direction": "global_y", "magnitude": {"value": 100.0, "unit": "N"}, "dimension": "force", "provenance": PROV}]);
    let mut verdicts = Vec::new();
    for mode in [PreviewSolverMode::DenseScrutiny, PreviewSolverMode::SparseInteractive] {
        let out = run_linear_static_preview_value_with_mode(json!({"model": &loaded, "materials": []}), mode).expect("request parses");
        let refused = out.diagnostics.iter().find(|d| d.code == "PIPE_ELEMENT_INPUT_INVALID" && d.affected_refs.iter().any(|r| r == pipe_id));
        verdicts.push(match refused {
            Some(d) => {
                assert_ne!(out.status.mechanics, "MECHANICS_SOLVED", "{mode:?}");
                Err(d.message.clone())
            }
            None => {
                assert_eq!(out.status.mechanics, "MECHANICS_SOLVED", "{mode:?}: {:#?}", out.diagnostics);
                Ok(())
            }
        });
    }
    assert_eq!(verdicts[0].is_ok(), verdicts[1].is_ok(), "both solver modes agree");
    verdicts.remove(0)
}

struct Row {
    name: &'static str,
    unit: &'static str,
    /// Chord ends A and B.
    a: [f64; 3],
    b: [f64; 3],
    y: [f64; 3],
    /// `None`: accepted; `Some(code)`: connect's refusal code.
    refused: Option<&'static str>,
}

const O: [f64; 3] = [0.0, 0.0, 0.0];

/// The shared table. Positions are in `unit`; `y_reference` is a direction.
fn table() -> Vec<Row> {
    vec![
        Row { name: "axis chord, perpendicular y", unit: "m", a: O, b: [2.0, 0.0, 0.0], y: [0.0, 0.0, 1.0], refused: None },
        Row { name: "axis chord, y exactly parallel", unit: "m", a: O, b: [2.0, 0.0, 0.0], y: [3.0, 0.0, 0.0], refused: Some(CONNECT_ORIENTATION) },
        Row { name: "axis chord, y exactly antiparallel", unit: "m", a: O, b: [2.0, 0.0, 0.0], y: [-1.0, 0.0, 0.0], refused: Some(CONNECT_ORIENTATION) },
        // Near-parallel: the perpendicular part of y is 5e-13 <= 1e-12.
        Row { name: "axis chord, y near-parallel", unit: "m", a: O, b: [2.0, 0.0, 0.0], y: [1.0, 0.0, 5e-13], refused: Some(CONNECT_ORIENTATION) },
        // Just inside the tolerance: perpendicular part 2e-12 > 1e-12; on an
        // axis-aligned chord the kernel's axes are then exactly orthonormal.
        Row { name: "axis chord, y just inside tolerance", unit: "m", a: O, b: [2.0, 0.0, 0.0], y: [1.0, 0.0, 2e-12], refused: None },
        // A perpendicular y whose whole length is below the kernel tolerance.
        Row { name: "axis chord, y below tolerance in length", unit: "m", a: O, b: [2.0, 0.0, 0.0], y: [0.0, 0.0, 1e-13], refused: Some(CONNECT_ORIENTATION) },
        // Design §2.3 counterexample: the projection is nonzero (about 7.07e-6)
        // but the kernel's |x.y| after normalization exceeds 1e-12.
        Row { name: "oblique chord, y near-parallel (§2.3)", unit: "m", a: O, b: [1.0, 1.0, 0.0], y: [1.00001, 1.0, 0.0], refused: Some(CONNECT_ORIENTATION) },
        Row { name: "oblique chord, y well off the chord", unit: "m", a: O, b: [1.0, 1.0, 0.0], y: [1.0, 1.0, 1.0], refused: None },
        Row { name: "oblique chord, y perpendicular", unit: "m", a: O, b: [1.0, 2.0, 3.0], y: [3.0, 0.0, -1.0], refused: None },
        Row { name: "mm project, axis chord, y near-parallel", unit: "mm", a: O, b: [2000.0, 0.0, 0.0], y: [1.0, 0.0, 5e-13], refused: Some(CONNECT_ORIENTATION) },
        Row { name: "mm project, axis chord, y just inside tolerance", unit: "mm", a: O, b: [2000.0, 0.0, 0.0], y: [1.0, 0.0, 2e-12], refused: None },
        Row { name: "mm project, oblique chord, y perpendicular", unit: "mm", a: O, b: [300.0, 400.0, 0.0], y: [-4.0, 3.0, 0.0], refused: None },
        // Distinct nodes at one position: there is no chord to orient.
        Row { name: "coincident endpoints", unit: "m", a: O, b: [0.0, 0.0, 0.0], y: [0.0, 0.0, 1.0], refused: Some(CONNECT_GEOMETRY) },
        // 5e-10 mm = 5e-13 m, at or below the kernel's 1e-12 m axis tolerance.
        Row { name: "mm project, chord below the axis tolerance", unit: "mm", a: O, b: [5e-10, 0.0, 0.0], y: [0.0, 0.0, 1.0], refused: Some(CONNECT_GEOMETRY) },
        // Review counterexample 1: near-parallel on an oblique chord that the
        // solve admits, because it normalizes the chord before the constructor.
        Row { name: "oblique chord, y near-parallel, solve admits (review 1)", unit: "m", a: O, b: [1.0, 1.0, 0.0], y: [1.000022, 1.0, 0.0], refused: None },
        // Review counterexample 2 (reconnect): a 1 mm chord away from the origin.
        Row { name: "mm project, offset oblique chord, solve admits (review 2)", unit: "mm", a: [10.0, 10.0, 0.0], b: [11.0, 11.0, 0.0], y: [1.000006, 1.0, 0.0], refused: None },
    ]
}

fn metres(unit: &str) -> f64 {
    match unit {
        "m" => 1.0,
        "mm" => 1e-3,
        other => panic!("table unit {other}"),
    }
}

/// The base for connect and reconnect: A at the origin, B at the row's end,
/// C off both, and pipe Q = A -> C, so reconnecting Q's `to` end to B makes the
/// same chord A -> B that connect authors.
fn base(row: &Row) -> Value {
    let c = [row.a[0], row.a[1], row.a[2] - 1.0 / metres(row.unit)];
    document(
        row.unit,
        vec![node("node:A", row.a), node("node:B", row.b), node("node:C", c)],
        vec![pipe("pipe:Q", "node:A", "node:C", [1.0, 0.0, 0.0])],
        vec![anchor("support:A", "node:A")],
    )
}

fn frame_element(a: [f64; 3], b: [f64; 3], y: [f64; 3]) -> Result<FrameElement, open_pipe_stress_frame_kernel::FrameKernelError> {
    let section = FrameSection::new(200e9, 80e9, 1e-3, 1e-6, 1e-6, 2e-6).expect("valid section");
    FrameElement::new(FrameNode::new(0, a)?, FrameNode::new(1, b)?, section, y)
}

#[test]
fn connect_refuses_parallel_and_near_parallel_orientations_by_name() {
    for row in table() {
        let model = base(&row);
        let verdict = connect_verdict(&model, "node:A", "node:B", row.y);
        match row.refused {
            None => assert_eq!(verdict, Ok(()), "{}", row.name),
            Some(code) => assert_eq!(verdict, Err(vec![code.to_string()]), "{}", row.name),
        }
    }
}

#[test]
fn the_refusal_names_the_pipe_its_ends_and_the_orientation() {
    let row = &table()[1];
    let outcome = run(&base(row), &connect("pipe:NEW", "node:A", "node:B", row.y));
    let diagnostic = outcome.diagnostics.iter().find(|d| d.code == CONNECT_ORIENTATION).expect("refused");
    assert_eq!(diagnostic.affected_refs, vec!["pipe:NEW", "node:A", "node:B"]);
    assert!(diagnostic.message.contains("pipe:NEW") && diagnostic.message.contains("(3, 0, 0)"), "{}", diagnostic.message);
    assert!(diagnostic.message.contains("node:A -> node:B"), "{}", diagnostic.message);
    assert!(diagnostic.message.contains("y reference parallel to local x axis"), "{}", diagnostic.message);
    assert!(outcome.diff_preview.is_empty());
    // The oblique near-parallel case is the kernel's orthogonality refusal.
    let row = &table()[6];
    let outcome = run(&base(row), &connect("pipe:NEW", "node:A", "node:B", row.y));
    let diagnostic = outcome.diagnostics.iter().find(|d| d.code == CONNECT_ORIENTATION).expect("refused");
    assert!(diagnostic.message.contains("InvalidOrientation"), "{}", diagnostic.message);
}

#[test]
fn connect_reconnect_kernel_and_solve_agree_on_the_table() {
    for row in table() {
        let model = base(&row);
        let connect = connect_verdict(&model, "node:A", "node:B", row.y);
        let reconnect = {
            let outcome = run(&model, &reconnect(&model, "pipe:Q", "to", "node:B", row.y));
            if outcome.applied_model.is_some() { Ok(()) } else { Err(blocking(&outcome)) }
        };
        // Connect's codes are reconnect's R6/R7 under the connect prefix.
        let mapped = connect.clone().map_err(|codes| {
            codes
                .into_iter()
                .map(|code| match code.as_str() {
                    CONNECT_ORIENTATION => RECONNECT_ORIENTATION.to_string(),
                    CONNECT_GEOMETRY => RECONNECT_GEOMETRY.to_string(),
                    other => other.to_string(),
                })
                .collect::<Vec<_>>()
        });
        assert_eq!(mapped, reconnect, "{}: connect and reconnect", row.name);

        // In a metre project the solve's element is `FrameElement::new` on the
        // stored coordinates (no conversion), the call it makes per straight pipe.
        if row.unit == "m" {
            let element = frame_element(row.a, row.b, row.y);
            assert_eq!(connect.is_ok(), element.is_ok(), "{}: connect and FrameElement::new ({element:?})", row.name);
        }

        // The solve on the same pipe authored directly, with no operation.
        let direct = document(
            row.unit,
            vec![node("node:A", row.a), node("node:B", row.b)],
            vec![pipe("pipe:P", "node:A", "node:B", row.y)],
            vec![anchor("support:A", "node:A")],
        );
        let solve = solve_verdict(&direct, "pipe:P");
        assert_eq!(connect.is_ok(), solve.is_ok(), "{}: connect and the solve ({solve:?})", row.name);
    }
}

#[test]
fn an_accepted_connect_solves_and_records_the_entered_orientation() {
    let row = &table()[4];
    let model = base(row);
    let outcome = run(&model, &connect("pipe:NEW", "node:A", "node:B", row.y));
    let applied = outcome.applied_model.expect("just inside the tolerance is accepted");
    let created = applied["pipe_segments"].as_array().unwrap().iter().find(|p| p["id"] == "pipe:NEW").unwrap();
    assert_eq!(created["y_reference"], json!({"x": 1.0, "y": 0.0, "z": 2e-12}));
    assert_eq!(solve_verdict(&applied, "pipe:NEW"), Ok(()));
}

#[test]
fn batches_refuse_the_connect_step_atomically_and_order_matters() {
    let model = document("m", vec![node("node:A", [0.0, 0.0, 0.0])], vec![], vec![anchor("support:A", "node:A")]);
    let claim = hash(&model);
    // [create_node, connect] with the orientation along the new chord.
    let refused = apply_operation_batch(
        &model,
        &json!({"batch_id": "batch:o4-refused", "operations": [create_node("node:B", [2.0, 0.0, 0.0]), connect("pipe:NEW", "node:A", "node:B", [1.0, 0.0, 0.0])]}),
        Some(&claim),
    );
    assert!(refused["applied_model"].is_null(), "{refused:#}");
    assert_ne!(refused["validation"]["application_status"], "applied_to_session_model");
    let codes: Vec<&str> = refused["operation_outcomes"][1]["diagnostics"].as_array().unwrap().iter().filter(|d| d["severity"] == "blocking").map(|d| d["code"].as_str().unwrap()).collect();
    assert_eq!(codes, vec![CONNECT_ORIENTATION]);
    // The same batch with a transverse orientation applies.
    let accepted = apply_operation_batch(
        &model,
        &json!({"batch_id": "batch:o4-accepted", "operations": [create_node("node:B", [2.0, 0.0, 0.0]), connect("pipe:NEW", "node:A", "node:B", [0.0, 0.0, 1.0])]}),
        Some(&claim),
    );
    assert_eq!(accepted["validation"]["application_status"], "applied_to_session_model", "{accepted:#}");
    // Each step is checked on the model as it stands: a batch that connects
    // along the chord and only then moves B off it is refused at the connect
    // step; moving B first, then connecting, applies.
    let with_b = document("m", vec![node("node:A", [0.0, 0.0, 0.0]), node("node:B", [2.0, 0.0, 0.0])], vec![], vec![anchor("support:A", "node:A")]);
    let claim = hash(&with_b);
    let move_b = op("move-b", "modify", "set_field", "Node", "node:B", "position.y", "0", "1", "m", "length");
    let parallel_then_move = apply_operation_batch(
        &with_b,
        &json!({"batch_id": "batch:o4-parallel-then-move", "operations": [connect("pipe:NEW", "node:A", "node:B", [1.0, 0.0, 0.0]), move_b.clone()]}),
        Some(&claim),
    );
    assert!(parallel_then_move["applied_model"].is_null(), "{parallel_then_move:#}");
    let move_then_connect = apply_operation_batch(
        &with_b,
        &json!({"batch_id": "batch:o4-move-then-connect", "operations": [move_b, connect("pipe:NEW", "node:A", "node:B", [1.0, 0.0, 0.0])]}),
        Some(&claim),
    );
    assert_eq!(move_then_connect["validation"]["application_status"], "applied_to_session_model", "{move_then_connect:#}");
}

#[test]
fn an_endpoint_without_an_explicit_position_is_refused() {
    let row = &table()[0];
    let mut model = base(row);
    model["nodes"][1]["position"] = json!({"x": 2.0, "y": 0.0});
    let outcome = run(&model, &connect("pipe:NEW", "node:A", "node:B", row.y));
    assert!(blocking(&outcome).contains(&CONNECT_GEOMETRY.to_string()), "{:#?}", outcome.diagnostics);
}

/// The verdicts of connect A -> B, reconnect Q.to C -> B, and the solve of
/// the pipe A -> B authored directly, for one row.
fn verdicts(row: &Row) -> (bool, bool, bool) {
    let model = base(row);
    let connect = connect_verdict(&model, "node:A", "node:B", row.y).is_ok();
    let reconnect = run(&model, &reconnect(&model, "pipe:Q", "to", "node:B", row.y)).applied_model.is_some();
    let direct = document(
        row.unit,
        vec![node("node:A", row.a), node("node:B", row.b)],
        vec![pipe("pipe:P", "node:A", "node:B", row.y)],
        vec![anchor("support:A", "node:A")],
    );
    (connect, reconnect, solve_verdict(&direct, "pipe:P").is_ok())
}

#[test]
fn review_counterexample_1_connect_admits_the_oblique_orientation_the_solve_admits() {
    // Metre endpoints (0,0,0) -> (1,1,0), y (1.000022, 1, 0). The raw chord
    // handed to the constructor gives |x.y| about 2.02e-11 and was refused;
    // the solve normalizes the chord first and admits it.
    let row = Row { name: "review 1", unit: "m", a: O, b: [1.0, 1.0, 0.0], y: [1.000022, 1.0, 0.0], refused: None };
    assert!(frame_element(row.a, row.b, row.y).is_ok(), "the solve's own element admits it");
    assert_eq!(verdicts(&row), (true, true, true), "connect, reconnect, solve");
}

#[test]
fn review_counterexample_2_reconnect_in_mm_admits_the_orientation_the_solve_admits() {
    // mm endpoints (10,10,0) -> (11,11,0), y (1.000006, 1, 0): the chord is
    // about 1.414e-3 m. Convert-then-subtract with the raw chord refused it
    // (|x.y| about 1.48e-10); the solve converts, subtracts, normalizes and
    // admits it.
    let row = Row { name: "review 2", unit: "mm", a: [10.0, 10.0, 0.0], b: [11.0, 11.0, 0.0], y: [1.000006, 1.0, 0.0], refused: None };
    let model = base(&row);
    let outcome = run(&model, &reconnect(&model, "pipe:Q", "to", "node:B", row.y));
    assert!(outcome.applied_model.is_some(), "{:#?}", outcome.diagnostics);
    assert_eq!(verdicts(&row), (true, true, true), "connect, reconnect, solve");
}

/// A sweep of near-parallel `y_reference` vectors on oblique and offset
/// chords, in metres and millimetres: connect and reconnect must give the
/// solve's verdict on every case. Both verdicts must occur.
#[test]
fn near_parallel_sweep_on_oblique_chords_matches_the_solve() {
    let chords: [(&'static str, [f64; 3], [f64; 3]); 7] = [
        ("m", O, [1.0, 1.0, 0.0]),
        ("m", O, [1.0, 2.0, 3.0]),
        ("m", [0.3, -1.7, 2.2], [1.9, 0.4, -0.6]),
        ("m", [5.0, 5.0, 5.0], [5.001, 4.9993, 5.0004]),
        ("mm", [10.0, 10.0, 0.0], [11.0, 11.0, 0.0]),
        ("mm", [-250.0, 40.0, 7.0], [1250.0, -960.0, 307.0]),
        ("mm", [1.0e6, 2.0e6, -3.0e5], [1.0e6 + 700.0, 2.0e6 + 300.0, -3.0e5 - 1100.0]),
    ];
    let perturbations = [0.0, 1e-14, 1e-13, 3e-13, 1e-12, 3e-12, 1e-11, 3e-11, 1e-10, 1e-9, 1e-8, 1e-7, 1e-6, 3e-6, 6e-6, 1e-5, 2.2e-5, 5e-5, 1e-4, 3e-4, 1e-3, 1e-2];
    let (mut cases, mut admitted) = (0, 0);
    for (unit, a, b) in chords {
        let d = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
        for e in perturbations {
            for y in [[d[0] * (1.0 + e), d[1], d[2]], [d[0] + e, d[1] - e, d[2] + 2.0 * e]] {
                let row = Row { name: "sweep", unit, a, b, y, refused: None };
                let (connect, reconnect, solve) = verdicts(&row);
                assert_eq!((connect, reconnect), (solve, solve), "{unit} {a:?} -> {b:?}, y {y:?}: connect, reconnect vs solve {solve}");
                if unit == "m" {
                    assert_eq!(solve, frame_element(a, b, y).is_ok(), "{a:?} -> {b:?}, y {y:?}: solve vs FrameElement::new");
                }
                cases += 1;
                admitted += usize::from(solve);
            }
        }
    }
    println!("O4 sweep: {cases} cases, {admitted} admitted, {} refused; connect = reconnect = solve on all", cases - admitted);
    assert!(admitted > 0 && admitted < cases, "the sweep must cross the kernel tolerance: {admitted}/{cases}");
}
