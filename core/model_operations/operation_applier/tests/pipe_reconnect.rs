//! W-10 pipe reconnect (`reconnect_pipe_run`, FR-GUI-004): the applier rules
//! of DEL-16-06 Design `pipe-reconnect.md` §1-§2 and §4 "Applier rules". Every
//! model is an inline invented fixture built from `MODEL_SCHEMA_VERSION`; no
//! value is library, catalog or code-rule data.
//!
//! Notation (design): P is the reconnected pipe, F its fixed end node, O the
//! end node it leaves and T the node it joins.
use open_pipe_stress_operation_applier::{
    apply_operation, apply_operation_batch, canonical_json, sha256_hex, validate_operation,
    validate_operation_batch, OperationOutcome,
};
use open_pipe_stress_product_physics::{arc_geometry, MODEL_SCHEMA_VERSION};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

const PROV: &str = "invented W-10 reconnect test input";

pub fn hash(model: &Value) -> Value {
    json!({"algorithm":"sha256","canonicalization":"rfc8785_jcs","payload_scope":"model_payload","value":format!("sha256:{}",sha256_hex(&canonical_json(model)))})
}
#[allow(clippy::too_many_arguments)]
pub fn op(id: &str, kind: &str, change_kind: &str, object: &str, target: &str, path: &str, before: &str, after: &str, unit: &str, dimension: &str, author: &str) -> Value {
    let mut intent = json!({"operation_id":format!("op:{id}"),"operation_kind":kind,"operation_status":"proposed","author_type":author,
      "target":{"object_type":object,"ref":target},
      "change":{"change_id":format!("change:{id}"),"change_kind":change_kind,"field_label":path,"field_path":path,"before":before,"after":after,"unit":unit,"dimension":dimension,"source_note":"invented explicit test entry"},
      "validation":{"schema_validation":"not_run","constraint_validation":"not_run","unit_validation":"not_run","diff_preview_status":"not_generated","application_status":"not_applied"},
      "rationale":"invented W-10 reconnect test operation",
      "audit_boundary":{"mutation_route":"structured_operations_only","direct_model_mutation_allowed":false,"requires_user_acceptance":true,"mutates_accepted_model_state":false},
      "professional_boundary":{"human_review_required":true,"software_makes_compliance_claim":false,"software_makes_certification_claim":false,"software_makes_sealing_claim":false,"software_makes_approval_claim":false,"software_makes_authentication_claim":false}});
    if author == "agent" {
        intent["source"] = json!({"source_ref":"tests/pipe_reconnect.rs","source_channel":"offline_proposal","source_role":"agent"});
    }
    intent
}
fn record<'a>(model: &'a Value, key: &str, id: &str) -> &'a Value {
    model[key].as_array().unwrap().iter().find(|r| r["id"] == id).unwrap_or_else(|| panic!("{id} in {key}"))
}
pub fn reconnect_as(model: &Value, pipe: &str, end: &str, target: &str, y: Option<Value>, author: &str) -> Value {
    let mut payload = json!({"pipe_ref": pipe, "end": end, "target_node_ref": target});
    if let Some(y) = y {
        payload["y_reference"] = y;
    }
    let before = canonical_json(record(model, "pipe_segments", pipe));
    op(&format!("reconnect:{pipe}:{end}:{target}"), "modify", "reconnect_pipe_run", "Element", pipe, "pipe_segments", &before, &payload.to_string(), "none", "dimensionless", author)
}
pub fn reconnect(model: &Value, pipe: &str, end: &str, target: &str, y: Option<Value>) -> Value {
    reconnect_as(model, pipe, end, target, y, "user")
}
fn display_number(value: f64) -> String {
    if value.fract() == 0.0 && value.abs() < 1e15 { format!("{}", value as i64) } else { format!("{value}") }
}
pub fn delete_node(model: &Value, node: &str) -> Value {
    let n = record(model, "nodes", node);
    let p = &n["position"];
    let before = format!("{}; x={}; y={}; z={}", n["label"].as_str().unwrap(), display_number(p["x"].as_f64().unwrap()), display_number(p["y"].as_f64().unwrap()), display_number(p["z"].as_f64().unwrap()));
    op(&format!("delete-node:{node}"), "delete", "delete_node", "Node", node, "nodes", &before, "not_present", "none", "dimensionless", "user")
}
pub fn create_node(id: &str, x: f64, y: f64, z: f64) -> Value {
    let after = json!({"id": id, "label": "Created node", "position": {"x": x, "y": y, "z": z}, "provenance": PROV});
    op(&format!("create-node:{id}"), "create", "create_node", "Node", id, "nodes", "not_present", &after.to_string(), "m", "length", "user")
}
pub fn batch(id: &str, operations: Vec<Value>) -> Value {
    json!({"batch_id": id, "operations": operations})
}

pub fn node(id: &str, x: f64, y: f64, z: f64) -> Value {
    json!({"id": id, "label": id.trim_start_matches("node:"), "position": {"x": x, "y": y, "z": z}, "provenance": PROV})
}
pub fn pipe(id: &str, from: &str, to: &str, y: Option<[f64; 3]>) -> Value {
    let mut p = json!({"id": id, "label": id.trim_start_matches("pipe:"), "from": from, "to": to,
        "section": {"outside_diameter": {"value": 0.1, "unit": "m"}, "wall_thickness": {"value": 0.005, "unit": "m"}},
        "material": "material:steel", "provenance": PROV});
    if let Some([x, y, z]) = y {
        p["y_reference"] = json!({"x": x, "y": y, "z": z});
    }
    p
}
const Z: Option<[f64; 3]> = Some([0.0, 0.0, 1.0]);
pub fn anchor(id: &str, node: &str) -> Value {
    json!({"id": id, "node": node, "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV})
}
/// A pressure-free current document around nodes, pipes and supports.
pub fn document(nodes: Vec<Value>, pipes: Vec<Value>, supports: Vec<Value>) -> Value {
    json!({"schema_version": MODEL_SCHEMA_VERSION, "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:w10-reconnect", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": nodes, "pipe_segments": pipes, "supports": supports, "components": [],
        "materials": [{"id": "material:steel", "elastic_modulus": {"value": 200e9, "unit": "Pa"}, "shear_modulus": {"value": 80e9, "unit": "Pa"}, "provenance": PROV}],
        "load_cases": [{"id": "case:w", "primitive_loads": [], "provenance": PROV}], "combinations": []})
}
/// The base: A(0,0,0) -P- B(2,0,0) -Q- D(4,0,0) -R- C(2,2,0). Reconnecting
/// P's `to` end from B to C keeps one connected graph with no junction,
/// orphan, parallel or coincident member: a clean accept with no warning.
fn base() -> Value {
    document(
        vec![node("node:A", 0.0, 0.0, 0.0), node("node:B", 2.0, 0.0, 0.0), node("node:C", 2.0, 2.0, 0.0), node("node:D", 4.0, 0.0, 0.0)],
        vec![pipe("pipe:P", "node:A", "node:B", Z), pipe("pipe:Q", "node:B", "node:D", Z), pipe("pipe:R", "node:D", "node:C", Z)],
        vec![anchor("support:A", "node:A"), anchor("support:D", "node:D")],
    )
}

fn codes(outcome: &OperationOutcome, severity: &str) -> Vec<String> {
    let mut codes: Vec<String> = outcome.diagnostics.iter().filter(|d| d.severity == severity).map(|d| d.code.clone()).collect();
    codes.sort();
    codes
}
/// Validate and apply; the refusal leaves no model, no diff and the input
/// unchanged, in both modes. Returns the apply outcome's blocking diagnostics.
fn refused(model: &Value, intent: &Value, code: &str) -> Vec<open_pipe_stress_operation_applier::OutcomeDiagnostic> {
    let original = model.clone();
    let claim = hash(model);
    let validate = validate_operation(model, intent, Some(&claim));
    let apply = apply_operation(model, intent, Some(&claim));
    for outcome in [&validate, &apply] {
        assert!(outcome.applied_model.is_none(), "{code}: a model was returned");
        assert!(outcome.diff_preview.is_empty(), "{code}: a diff was returned");
        assert!(codes(outcome, "blocking").iter().any(|c| c == code), "{code} not in {:#?}", outcome.diagnostics);
    }
    assert_eq!(apply.validation.application_status, "blocked");
    assert_eq!(model, &original);
    apply.diagnostics.into_iter().filter(|d| d.severity == "blocking").collect()
}
/// Validate then apply; validate returns the same diff and warnings and no
/// model. Returns (applied model, warning codes).
fn accepted(model: &Value, intent: &Value) -> (Value, Vec<String>, OperationOutcome) {
    let claim = hash(model);
    let validate = validate_operation(model, intent, Some(&claim));
    let apply = apply_operation(model, intent, Some(&claim));
    assert!(validate.applied_model.is_none());
    assert_eq!(codes(&validate, "blocking"), Vec::<String>::new(), "{:#?}", validate.diagnostics);
    assert_eq!(validate.diff_preview, apply.diff_preview);
    assert_eq!(validate.diagnostics, apply.diagnostics, "preview shows exactly what apply reports");
    let applied = apply.applied_model.clone().unwrap_or_else(|| panic!("{:#?}", apply.diagnostics));
    assert_eq!(apply.diff_preview.len(), 1);
    let warnings = codes(&apply, "warning");
    (applied, warnings, apply)
}
fn refs_of(diagnostics: &[open_pipe_stress_operation_applier::OutcomeDiagnostic], code: &str) -> Vec<Vec<String>> {
    diagnostics.iter().filter(|d| d.code == code).map(|d| d.affected_refs.clone()).collect()
}

#[test]
fn clean_reconnect_moves_one_end_keeps_identity_and_order() {
    let model = base();
    let (applied, warnings, outcome) = accepted(&model, &reconnect(&model, "pipe:P", "to", "node:C", None));
    assert!(warnings.is_empty(), "{warnings:?}");
    let before = record(&model, "pipe_segments", "pipe:P");
    let after = record(&applied, "pipe_segments", "pipe:P");
    let mut expected = before.clone();
    expected["to"] = json!("node:C");
    assert_eq!(after, &expected, "only the moved end differs");
    let ids = |m: &Value, k: &str| m[k].as_array().unwrap().iter().map(|r| r["id"].clone()).collect::<Vec<_>>();
    for key in ["nodes", "pipe_segments", "supports"] {
        assert_eq!(ids(&model, key), ids(&applied, key), "{key} ids and order unchanged");
    }
    let mut rest = applied.clone();
    rest["pipe_segments"][0] = model["pipe_segments"][0].clone();
    assert_eq!(rest, model, "nothing else changes");
    let row = &outcome.diff_preview[0];
    assert_eq!((row.entity_ref.as_str(), row.object_type.as_str(), row.field_path.as_str(), row.change_kind.as_str()), ("pipe:P", "Element", "pipe_segments", "reconnect_pipe_run"));
    assert_eq!(row.before, canonical_json(before));
    assert_eq!(row.after, canonical_json(after));
    assert_eq!((row.unit.as_str(), row.dimension.as_str()), ("none", "dimensionless"));
    // The `from` end moves the same way and keeps the from→to sense.
    let (applied, warnings, _) = accepted(&model, &reconnect(&model, "pipe:P", "from", "node:C", None));
    assert_eq!(record(&applied, "pipe_segments", "pipe:P")["from"], "node:C");
    assert_eq!(record(&applied, "pipe_segments", "pipe:P")["to"], "node:B");
    // A leaves P; A keeps its anchor, so it is orphaned.
    assert!(warnings.contains(&"OP-RECONNECT-NODE-ORPHANED".to_string()), "{warnings:?}");
}

#[test]
fn agent_and_user_authors_give_the_same_outcome() {
    let model = base();
    let claim = hash(&model);
    let user = apply_operation(&model, &reconnect_as(&model, "pipe:P", "to", "node:C", None, "user"), Some(&claim));
    let agent = apply_operation(&model, &reconnect_as(&model, "pipe:P", "to", "node:C", None, "agent"), Some(&claim));
    assert_eq!(user.applied_model, agent.applied_model);
    assert_eq!(user.diagnostics, agent.diagnostics);
    assert_eq!(user.diff_preview, agent.diff_preview);
    assert!(user.applied_model.is_some());
}

#[test]
fn r1_target_and_payload_refusals() {
    let model = base();
    let good = reconnect(&model, "pipe:P", "to", "node:C", None);
    // P absent.
    let mut absent = good.clone();
    absent["target"]["ref"] = json!("pipe:absent");
    absent["change"]["after"] = json!(json!({"pipe_ref":"pipe:absent","end":"to","target_node_ref":"node:C"}).to_string());
    let refusal = refused(&model, &absent, "OP-TARGET-NOT-FOUND");
    assert_eq!(refusal[0].affected_refs, vec!["pipe:absent"]);
    // pipe_ref differs from target.
    let mut mismatch = good.clone();
    mismatch["change"]["after"] = json!(json!({"pipe_ref":"pipe:Q","end":"to","target_node_ref":"node:C"}).to_string());
    refused(&model, &mismatch, "OP-RECONNECT-PAYLOAD-INVALID");
    for after in [
        json!({"pipe_ref":"pipe:P","end":"middle","target_node_ref":"node:C"}).to_string(),
        json!({"pipe_ref":"pipe:P","end":"to","target_node_ref":"node:C","extra":1}).to_string(),
        json!({"pipe_ref":"pipe:P","end":"to"}).to_string(),
        json!({"pipe_ref":"pipe:P","end":"to","target_node_ref":"node:C","y_reference":{"x":0,"y":1}}).to_string(),
        "not json".to_string(),
    ] {
        let mut bad = good.clone();
        bad["change"]["after"] = json!(after);
        refused(&model, &bad, "OP-RECONNECT-PAYLOAD-INVALID");
    }
    for (path, value) in [("unit", json!("m")), ("dimension", json!("length")), ("field_path", json!("nodes"))] {
        let mut bad = good.clone();
        bad["change"][path] = value;
        refused(&model, &bad, "OP-RECONNECT-PAYLOAD-INVALID");
    }
    let mut kind = good.clone();
    kind["operation_kind"] = json!("connect");
    refused(&model, &kind, "OP-CHANGE-KIND-MISMATCH");
    // Claimed model-hash evidence is required, as for split and transform.
    for outcome in [validate_operation(&model, &good, None), apply_operation(&model, &good, None)] {
        assert!(outcome.applied_model.is_none() && outcome.diff_preview.is_empty());
        assert!(codes(&outcome, "blocking").contains(&"OP-GEOMETRY-MODEL-HASH-REQUIRED".to_string()));
    }
    // The schema's generic `reconnect` tokens stay refused.
    let mut generic = good.clone();
    generic["operation_kind"] = json!("reconnect");
    generic["change"]["change_kind"] = json!("reconnect");
    refused(&model, &generic, "OP-KIND-UNSUPPORTED");
}

#[test]
fn r2_stale_before_value() {
    let model = base();
    let mut stale = reconnect(&model, "pipe:P", "to", "node:C", None);
    stale["change"]["before"] = json!(canonical_json(record(&model, "pipe_segments", "pipe:Q")));
    refused(&model, &stale, "OP-STALE-BEFORE-VALUE");
    let outcome = apply_operation(&model, &stale, Some(&hash(&model)));
    assert_eq!(outcome.validation.before_state_validation, "blocked_stale");
}

#[test]
fn r3_r4_r5_target_node_rules() {
    let model = base();
    let refusal = refused(&model, &reconnect(&model, "pipe:P", "to", "node:absent", None), "OP-RECONNECT-TARGET-NODE-NOT-FOUND");
    assert_eq!(refusal[0].affected_refs, vec!["pipe:P", "node:absent"]);
    refused(&model, &reconnect(&model, "pipe:P", "to", "node:B", None), "OP-RECONNECT-NO-CHANGE");
    refused(&model, &reconnect(&model, "pipe:P", "to", "node:A", None), "OP-RECONNECT-ZERO-LENGTH");
    refused(&model, &reconnect(&model, "pipe:P", "from", "node:B", None), "OP-RECONNECT-ZERO-LENGTH");
}

#[test]
fn r6_degenerate_or_nonfinite_chord() {
    // T at F's position through a different node id.
    let mut model = base();
    model["nodes"].as_array_mut().unwrap().push(node("node:A2", 0.0, 0.0, 0.0));
    refused(&model, &reconnect(&model, "pipe:P", "to", "node:A2", None), "OP-RECONNECT-GEOMETRY-DEGENERATE");
    // Below the kernel's 1e-12 axis tolerance in metres, in millimetres.
    let mut mm = model.clone();
    mm["project"]["units"]["length"] = json!("mm");
    mm["nodes"][4]["position"]["x"] = json!(5e-10);
    refused(&mm, &reconnect(&mm, "pipe:P", "to", "node:A2", None), "OP-RECONNECT-GEOMETRY-DEGENERATE");
    // A chord that overflows binary64.
    let mut huge = base();
    huge["nodes"][0]["position"]["x"] = json!(-1e308);
    huge["nodes"][2]["position"]["x"] = json!(1e308);
    refused(&huge, &reconnect(&huge, "pipe:P", "to", "node:C", None), "OP-RECONNECT-GEOMETRY-DEGENERATE");
}

#[test]
fn r7_kernel_orientation_and_y_reference_rules() {
    // Design §2.3 counterexample: chord (1,1,0) with the kept y_reference
    // (1.00001,1,0). The y-projection check alone passes it.
    let mut model = base();
    model["nodes"].as_array_mut().unwrap().push(node("node:E", 1.0, 1.0, 0.0));
    model["pipe_segments"][0]["y_reference"] = json!({"x": 1.00001, "y": 1.0, "z": 0.0});
    let (x, y) = ([1.0f64, 1.0, 0.0], [1.00001f64, 1.0, 0.0]);
    let xn = (2.0f64).sqrt();
    let x = [x[0] / xn, x[1] / xn, x[2] / xn];
    let p = y[0] * x[0] + y[1] * x[1];
    let perpendicular = ((y[0] - p * x[0]).powi(2) + (y[1] - p * x[1]).powi(2)).sqrt();
    assert!(perpendicular > 1e-12, "the weaker projection check would accept this chord: {perpendicular:e}");
    refused(&model, &reconnect(&model, "pipe:P", "to", "node:E", None), "OP-RECONNECT-ORIENTATION-DEGENERATE");
    // A kept y_reference that becomes parallel to the new chord refuses ...
    let mut up = base();
    up["nodes"].as_array_mut().unwrap().push(node("node:U", 0.0, 0.0, 3.0));
    refused(&up, &reconnect(&up, "pipe:P", "to", "node:U", None), "OP-RECONNECT-ORIENTATION-DEGENERATE");
    // ... and an entered one replaces it in the same change.
    let (applied, warnings, _) = accepted(&up, &reconnect(&up, "pipe:P", "to", "node:U", Some(json!({"x": 0, "y": 1, "z": 0}))));
    assert_eq!(warnings, vec!["OP-RECONNECT-MODEL-SPLIT"], "A-U is a new part");
    assert_eq!(record(&applied, "pipe_segments", "pipe:P")["y_reference"], json!({"x": 0, "y": 1, "z": 0}));
    // An entered y_reference parallel to the chord refuses as well.
    refused(&up, &reconnect(&up, "pipe:P", "to", "node:U", Some(json!({"x": 0, "y": 0, "z": -2}))), "OP-RECONNECT-ORIENTATION-DEGENERATE");
    // Missing y_reference: the chord check only, then the warning; it stays absent.
    let mut missing = base();
    missing["pipe_segments"][0].as_object_mut().unwrap().remove("y_reference");
    let (applied, warnings, _) = accepted(&missing, &reconnect(&missing, "pipe:P", "to", "node:C", None));
    assert_eq!(warnings, vec!["OP-RECONNECT-ORIENTATION-MISSING"]);
    assert!(record(&applied, "pipe_segments", "pipe:P").get("y_reference").is_none());
    // An entered one on a pipe that had none is written and the warning goes.
    let (applied, warnings, _) = accepted(&missing, &reconnect(&missing, "pipe:P", "to", "node:C", Some(json!({"x": 0, "y": 0, "z": 1}))));
    assert!(warnings.is_empty());
    assert_eq!(record(&applied, "pipe_segments", "pipe:P")["y_reference"], json!({"x": 0, "y": 0, "z": 1}));
}

fn with_components(mut model: Value, components: Value) -> Value {
    model["components"] = components;
    model
}

#[test]
fn r9_component_attachments_and_terminal_exemption() {
    let base = base();
    let intent = reconnect(&base, "pipe:P", "to", "node:C", None);
    // A geometry-only bend marker at F (no bend_pipe_ref).
    let model = with_components(base.clone(), json!([{"id":"component:bend-F","kind":"bend","node":"node:A","geometry":{"bend_angle":{"value":90,"unit":"deg"}},"provenance":PROV}]));
    let refusal = refused(&model, &intent, "OP-RECONNECT-COMPONENT-ATTACHED");
    assert_eq!(refusal[0].affected_refs, vec!["pipe:P", "component:bend-F", "node:A"]);
    assert!(refusal[0].message.contains("sits at node node:A, where this reconnect changes the pipe's direction"), "{}", refusal[0].message);
    assert!(refusal[0].remediation.contains("Delete or re-author component:bend-F"));
    // A tee at O.
    let model = with_components(base.clone(), json!([{"id":"component:tee-O","kind":"tee","node":"node:B","provenance":PROV}]));
    let refusal = refused(&model, &intent, "OP-RECONNECT-COMPONENT-ATTACHED");
    assert!(refusal[0].message.contains("component:tee-O (tee) sits at node node:B, where this reconnect removes the pipe"), "{}", refusal[0].message);
    // A valve at T.
    let model = with_components(base.clone(), json!([{"id":"component:valve-T","kind":"valve","node":"node:C","provenance":PROV}]));
    let refusal = refused(&model, &intent, "OP-RECONNECT-COMPONENT-ATTACHED");
    assert!(refusal[0].message.contains("where this reconnect adds the pipe"), "{}", refusal[0].message);
    // A connector naming P by its span.
    let model = with_components(base.clone(), json!([{"id":"component:joint","kind":"expansion_joint","node":"node:Q-mid","objective_connector":{"topology":{"type":"replaces_span","span_ref":"pipe:P"}},"provenance":PROV}]));
    let refusal = refused(&model, &intent, "OP-RECONNECT-COMPONENT-ATTACHED");
    assert!(refusal[0].message.contains("names it in objective_connector.topology.span_ref"), "{}", refusal[0].message);
    assert_eq!(refusal[0].affected_refs, vec!["pipe:P", "component:joint"]);
    // Each named-pipe slot of §2.1.
    for key in ["bend_pipe_ref", "branch_header_pipe_ref", "branch_branch_pipe_ref", "rigid_pipe_ref", "expansion_joint_pipe_ref"] {
        let model = with_components(base.clone(), json!([{"id":"component:named","kind":"rigid","node":"node:far","geometry":{key:"pipe:P"},"provenance":PROV}]));
        let refusal = refused(&model, &intent, "OP-RECONNECT-COMPONENT-ATTACHED");
        assert!(refusal[0].message.contains(&format!("names it in geometry.{key}")), "{}", refusal[0].message);
    }
    // A component naming F in another slot.
    let model = with_components(base.clone(), json!([{"id":"component:other","kind":"rigid","node":"node:far","geometry":{"anchor_node_ref":"node:A"},"provenance":PROV}]));
    let refusal = refused(&model, &intent, "OP-RECONNECT-COMPONENT-ATTACHED");
    assert!(refusal[0].message.contains("names node node:A in geometry.anchor_node_ref"), "{}", refusal[0].message);
    // Each blocker is its own diagnostic, P first.
    let model = with_components(base.clone(), json!([
        {"id":"component:tee-O","kind":"tee","node":"node:B","provenance":PROV},
        {"id":"component:bend-P","kind":"bend","node":"node:far","geometry":{"bend_pipe_ref":"pipe:P"},"provenance":PROV}]));
    let refusal = refused(&model, &intent, "OP-RECONNECT-COMPONENT-ATTACHED");
    assert_eq!(refusal.len(), 2);
    assert!(refusal.iter().all(|d| d.affected_refs[0] == "pipe:P"));
    // A terminal marker at T (and at F, and at O) is allowed and named in the
    // junction and orphan warnings.
    let model = with_components(base.clone(), json!([{"id":"component:end-C","kind":"terminal","node":"node:C","provenance":PROV}]));
    let (_, warnings, _) = accepted(&model, &intent);
    assert!(warnings.is_empty(), "{warnings:?}");
    let mut free = base.clone();
    free["nodes"].as_array_mut().unwrap().push(node("node:X", 6.0, 0.0, 0.0));
    free["pipe_segments"].as_array_mut().unwrap().push(pipe("pipe:S", "node:D", "node:X", Z));
    free["components"] = json!([{"id":"component:end-X","kind":"terminal","node":"node:X","provenance":PROV}]);
    let (_, warnings, outcome) = accepted(&free, &reconnect(&free, "pipe:S", "to", "node:C", None));
    assert!(warnings.contains(&"OP-RECONNECT-NODE-ORPHANED".to_string()), "{warnings:?}");
    let orphan = outcome.diagnostics.iter().find(|d| d.code == "OP-RECONNECT-NODE-ORPHANED").unwrap();
    assert!(orphan.affected_refs.contains(&"component:end-X".to_string()), "{:?}", orphan.affected_refs);
}

fn region(id: &str, members: &[&str], terminals: [&str; 2]) -> Value {
    json!({"id": id, "member_pipe_ids": members, "pressure_basis": "internal_differential_zero_external_v1", "pressure": {"value": 1.0e6, "unit": "Pa"},
        "terminals": [{"node_ref": terminals[0], "closure_transfer": "transfers_to_wall", "provenance": PROV}, {"node_ref": terminals[1], "closure_transfer": "transfers_to_wall", "provenance": PROV}],
        "provenance": PROV})
}

#[test]
fn r10_member_and_r11_internal_branch() {
    let mut model = base();
    model["load_cases"][0]["pressure_regions"] = json!([region("region:P", &["pipe:P"], ["node:A", "node:B"])]);
    let refusal = refused(&model, &reconnect(&model, "pipe:P", "to", "node:C", None), "OP-RECONNECT-PRESSURE-REGION-MEMBER");
    assert_eq!(refusal[0].affected_refs, vec!["pipe:P", "region:P", "case:w"]);
    assert_eq!(refusal[0].message, "Cannot reconnect pipe:P: it is a member of pressure region region:P in load case case:w.");
    assert_eq!(refusal[0].remediation, "Remove pipe:P from region:P, reconnect, then re-enter the region's members and terminals.");
    // D is internal to Q-R (terminals B and C): P cannot join it.
    let mut model = base();
    model["load_cases"][0]["pressure_regions"] = json!([region("region:QR", &["pipe:Q", "pipe:R"], ["node:B", "node:C"])]);
    let refusal = refused(&model, &reconnect(&model, "pipe:P", "to", "node:D", None), "OP-RECONNECT-PRESSURE-REGION-BRANCH");
    assert_eq!(refusal[0].message, "Cannot move the to end of pipe:P to node:D: node:D is an internal node of pressure region region:QR in load case case:w, and a pipe outside the region cannot join it there.");
    assert_eq!(refusal[0].remediation, "Choose another node, or end region:QR at node:D before reconnecting.");
    // A region terminal is not internal: P (not a member) leaves terminal B
    // and joins terminal C; each warns, and the reconnect applies.
    let (_, warnings, outcome) = accepted(&model, &reconnect(&model, "pipe:P", "to", "node:C", None));
    assert_eq!(warnings, vec!["OP-RECONNECT-PRESSURE-TERMINAL-REVIEW", "OP-RECONNECT-PRESSURE-TERMINAL-REVIEW"]);
    let refs = refs_of(&outcome.diagnostics, "OP-RECONNECT-PRESSURE-TERMINAL-REVIEW");
    assert!(refs.contains(&vec!["pipe:P".to_string(), "region:QR".into(), "case:w".into(), "node:B".into()]), "{refs:?}");
    assert!(refs.contains(&vec!["pipe:P".to_string(), "region:QR".into(), "case:w".into(), "node:C".into()]), "{refs:?}");
    // An end of a member connector span counts as a region node.
    let mut joint = base();
    joint["nodes"].as_array_mut().unwrap().push(node("node:J", 4.0, 2.0, 0.0));
    joint["pipe_segments"].as_array_mut().unwrap().push(pipe("pipe:span", "node:C", "node:J", Z));
    joint["components"] = json!([{"id":"component:joint","kind":"expansion_joint","node":"node:far","objective_connector":{"topology":{"type":"replaces_span","span_ref":"pipe:span"}},"provenance":PROV}]);
    let mut connector_region = region("region:J", &["pipe:R"], ["node:D", "node:J"]);
    connector_region["member_connector_ids"] = json!(["component:joint"]);
    joint["load_cases"][0]["pressure_regions"] = json!([connector_region]);
    refused(&joint, &reconnect(&joint, "pipe:P", "to", "node:C", None), "OP-RECONNECT-PRESSURE-REGION-BRANCH");
    // Moving a non-member away from an internal node removes a branch: allowed.
    let mut away = base();
    away["nodes"].as_array_mut().unwrap().push(node("node:E", 0.0, 2.0, 0.0));
    away["pipe_segments"].as_array_mut().unwrap().push(pipe("pipe:X", "node:B", "node:C", Z));
    away["load_cases"][0]["pressure_regions"] = json!([region("region:PQ", &["pipe:P", "pipe:Q"], ["node:A", "node:D"])]);
    let (_, warnings, _) = accepted(&away, &reconnect(&away, "pipe:X", "from", "node:E", None));
    assert!(warnings.is_empty(), "{warnings:?}");
}

fn primitive(id: &str, category: &str, target: Value) -> Value {
    json!({"id": id, "category": category, "target": target, "direction": "global_z", "magnitude": {"value": -100.0, "unit": "N/m"}, "dimension": "force_per_length", "provenance": PROV})
}
/// Every §2.2 allowed slot naming P, with its exact supported shape.
fn slots_model() -> Value {
    let mut model = base();
    model["load_cases"] = json!([{"id": "case:w", "provenance": PROV,
        "primitive_loads": [primitive("load:w", "distributed_force", json!({"type":"element","pipe":"pipe:P"})),
            {"id":"load:t","category":"thermal","target":{"type":"element","pipe":"pipe:P"},"direction":"axial","magnitude":{"value":50.0,"unit":"degC"},"dimension":"temperature_interval","provenance":PROV},
            {"id":"load:n","category":"concentrated_force","target":{"type":"node","node":"node:B"},"direction":"global_z","magnitude":{"value":10.0,"unit":"N"},"dimension":"force","provenance":PROV}],
        "equivalent_static": {"wind": {"pressure":{"value":500.0,"unit":"Pa"},"shape_factor":{"value":0.7,"unit":"1"},"direction":"global_y",
            "exposed_pipe_refs": ["pipe:P"], "exposed_spans": [{"pipe_ref":"pipe:Q","start_fraction":{"value":0.1,"unit":"1"},"end_fraction":{"value":0.5,"unit":"1"}}]}},
        "analysis_state": {"element_states": [{"pipe_ref": "pipe:P"}]}},
        {"id": "case:s", "provenance": PROV, "primitive_loads": [],
         "equivalent_static": {"wind": {"exposed_spans": [{"pipe_ref":"pipe:P","start_fraction":{"value":0.25,"unit":"1"},"end_fraction":{"value":0.75,"unit":"1"}}]}}}]);
    model["reference_configurations"] = json!([{"id":"reference:installed","member_references":[
        {"pipe_ref":"pipe:P","basis":{"kind":"direct_strain_reference"},"fit":{"kind":"natural_length_change","length_change":{"value":-2.0,"unit":"mm"}},"provenance":PROV},
        {"pipe_ref":"pipe:Q","basis":{"kind":"direct_strain_reference"},"fit":{"kind":"none"},"provenance":PROV}]}]);
    model
}

#[test]
fn allowed_pipe_slots_are_kept_byte_identically_with_their_warnings() {
    let model = slots_model();
    let (applied, warnings, outcome) = accepted(&model, &reconnect(&model, "pipe:P", "to", "node:C", None));
    assert_eq!(warnings, vec![
        "OP-RECONNECT-MEMBER-FIT-REVIEW",
        "OP-RECONNECT-MEMBER-LOAD-LENGTH-CHANGED",
        "OP-RECONNECT-WIND-EXPOSURE-REMAPPED",
        "OP-RECONNECT-WIND-EXPOSURE-REMAPPED",
    ]);
    assert_eq!(canonical_json(&applied["load_cases"]), canonical_json(&model["load_cases"]));
    assert_eq!(canonical_json(&applied["reference_configurations"]), canonical_json(&model["reference_configurations"]));
    let load = outcome.diagnostics.iter().find(|d| d.code == "OP-RECONNECT-MEMBER-LOAD-LENGTH-CHANGED").unwrap();
    assert_eq!(load.affected_refs, vec!["pipe:P", "load:w", "case:w"]);
    // A fit of kind `none` gives no review warning.
    let mut none = model.clone();
    none["reference_configurations"][0]["member_references"][0]["fit"] = json!({"kind":"none"});
    let (_, warnings, _) = accepted(&none, &reconnect(&none, "pipe:P", "to", "node:C", None));
    assert!(!warnings.contains(&"OP-RECONNECT-MEMBER-FIT-REVIEW".to_string()));
}

#[test]
fn r13_allowed_slot_with_an_unsupported_shape_refuses() {
    let intent = |m: &Value| reconnect(m, "pipe:P", "to", "node:C", None);
    let mut cases: Vec<(&str, Value)> = Vec::new();
    let mut m = slots_model();
    m["load_cases"][0]["primitive_loads"][0]["extent"] = json!({"start": 0.2});
    cases.push(("primitive extra key", m));
    let mut m = slots_model();
    m["load_cases"][0]["primitive_loads"][0]["target"]["start_fraction"] = json!(0.2);
    cases.push(("target extra key", m));
    let mut m = slots_model();
    m["load_cases"][0]["primitive_loads"][0]["target"]["type"] = json!("support");
    cases.push(("non-element target", m));
    let mut m = slots_model();
    m["load_cases"][0]["primitive_loads"][0]["category"] = json!("concentrated_force");
    cases.push(("unsupported category", m));
    let mut m = slots_model();
    m["load_cases"][1]["equivalent_static"]["wind"]["exposed_spans"][0]["weight"] = json!(1);
    cases.push(("wind span extra key", m));
    let mut m = slots_model();
    m["reference_configurations"][0]["member_references"][0]["scale"] = json!(1);
    cases.push(("member reference extra key", m));
    for (label, model) in cases {
        let refusal = refused(&model, &intent(&model), "OP-RECONNECT-ATTACHMENT-UNSUPPORTED");
        assert!(refusal[0].message.contains("not of the exact supported shape"), "{label}: {}", refusal[0].message);
        assert_eq!(refusal[0].affected_refs[0], "pipe:P", "{label}");
        assert_eq!(refusal[0].remediation, "Remove or re-author that record, then reconnect.");
    }
}

#[test]
fn r12_unknown_slots_refuse_and_stored_diagnostics_are_skipped() {
    let base = base();
    let intent = reconnect(&base, "pipe:P", "to", "node:C", None);
    let mut m = base.clone();
    m["extension"] = json!({"required_input_refs": [{"id": "pipe:P"}]});
    let refusal = refused(&m, &intent, "OP-RECONNECT-ATTACHMENT-UNSUPPORTED");
    assert_eq!(refusal[0].message, "Cannot reconnect pipe:P: pipe:P is named at extension.required_input_refs[0].id, which reconnect does not carry across a geometry change.");
    // S2 (0.7.0 design §6; §2.2 here): the scan does not read the model-data
    // collections. A line keeps P by id across the reconnect; an assumption, a
    // constraint or an equipment interface naming P or O never blocks; tags and
    // notes are annotations, not links.
    let mut m = base.clone();
    m["lines"] = json!([{"id": "line:1", "label": "Invented line", "pipe_refs": ["pipe:P"]}]);
    m["unresolved_assumptions"] = json!([{"assumption_id": "assumption:1", "kind": "assumption", "statement": "Invented.", "status": "unresolved", "affected_refs": ["node:B", "pipe:P"]}]);
    m["constraints"] = json!([{"constraint_id": "constraint:1", "name": "Invented", "constraint_kind": "clearance", "state": "proposed", "target_refs": ["pipe:P"], "statement": "Invented."}]);
    m["equipment_interfaces"] = json!([{"id": "equipment-interface:1", "label": "Invented nozzle", "node_ref": "node:B", "equipment_label": "Invented pump"}]);
    m["pipe_segments"][1]["tags"] = json!(["pipe:P"]);
    m["supports"][0]["notes"] = json!("node:C");
    let (applied, warnings, _) = accepted(&m, &intent);
    assert!(warnings.is_empty(), "{warnings:?}");
    for slot in ["lines", "unresolved_assumptions", "constraints", "equipment_interfaces"] {
        assert_eq!(applied[slot], m[slot], "{slot} is carried unchanged");
    }
    // An unknown slot naming F.
    let mut m = base.clone();
    m["supports"][0]["reaction_node_ref"] = json!("node:A");
    let refusal = refused(&m, &intent, "OP-RECONNECT-ATTACHMENT-UNSUPPORTED");
    assert_eq!(refusal[0].message, "Cannot reconnect pipe:P: node:A is named at supports[0].reaction_node_ref, which reconnect does not carry across a geometry change.");
    assert_eq!(refusal[0].affected_refs, vec!["pipe:P", "node:A", "support:A"]);
    // Another pipe naming P outside its own record.
    let mut m = base.clone();
    m["pipe_segments"][1]["section_ref"] = json!("pipe:P");
    refused(&m, &intent, "OP-RECONNECT-ATTACHMENT-UNSUPPORTED");
    // A stored diagnostics entry naming P and T does not block.
    let mut m = base.clone();
    m["diagnostics"] = json!([{"id": "diagnostic:stored", "code": "X", "affected_refs": ["pipe:P", "node:C"]}]);
    let (applied, warnings, _) = accepted(&m, &intent);
    assert!(warnings.is_empty());
    assert_eq!(applied["diagnostics"], m["diagnostics"]);
    // Labels, names and provenance text are not links.
    let mut m = base.clone();
    m["supports"][0]["label"] = json!("pipe:P");
    m["supports"][0]["provenance"] = json!("node:C");
    accepted(&m, &intent);
}

#[test]
fn a_key_spelled_as_path_syntax_is_one_key_and_refuses() {
    // A literal key "primitive_loads[0].target" naming P, beside a real
    // primitive_loads[0] that targets Q: the scan carries segments, so the hit
    // is an unknown slot (R12), never the allowed load slot of another record.
    let mut model = base();
    model["load_cases"][0]["primitive_loads"] = json!([primitive("load:q", "distributed_force", json!({"type":"element","pipe":"pipe:Q"}))]);
    model["load_cases"][0]["primitive_loads[0].target"] = json!({"pipe": "pipe:P"});
    let intent = reconnect(&model, "pipe:P", "to", "node:C", None);
    let refusal = refused(&model, &intent, "OP-RECONNECT-ATTACHMENT-UNSUPPORTED");
    assert_eq!(refusal.len(), 1);
    assert_eq!(refusal[0].message, "Cannot reconnect pipe:P: pipe:P is named at load_cases[0][\"primitive_loads[0].target\"].pipe, which reconnect does not carry across a geometry change.");
    assert_eq!(refusal[0].affected_refs, vec!["pipe:P", "case:w"]);
    let outcome = validate_operation(&model, &intent, Some(&hash(&model)));
    assert!(!outcome.diagnostics.iter().any(|d| d.code == "OP-RECONNECT-MEMBER-LOAD-LENGTH-CHANGED"), "no warning about Q's load");
}

#[test]
fn an_object_key_spelled_star_is_not_an_index_wildcard() {
    // primitive_loads as an object with a key "*": not the allowed load slot.
    let mut model = base();
    let load = primitive("load:star", "distributed_force", json!({"type":"element","pipe":"pipe:P"}));
    model["load_cases"][0]["primitive_loads"] = json!({"*": load});
    let intent = reconnect(&model, "pipe:P", "to", "node:C", None);
    let refusal = refused(&model, &intent, "OP-RECONNECT-ATTACHMENT-UNSUPPORTED");
    // Refused on both counts: the slot is unknown (R12), and the collection is not an array.
    assert_eq!(refusal.len(), 2, "{refusal:#?}");
    assert!(refusal.iter().any(|d| d.message.contains("is named at load_cases[0].primitive_loads[\"*\"].target.pipe")), "{refusal:#?}");
    assert!(refusal.iter().any(|d| d.message.contains("load_cases[0].primitive_loads is not an array")), "{refusal:#?}");
}

#[test]
fn a_collection_that_is_not_an_array_is_unscannable_and_refuses() {
    // components as an object map naming P: not R9's array records, so the
    // scan reports it and the shape guard refuses (never read as empty).
    let mut model = base();
    model["components"] = json!({"*": {"geometry": {"bend_pipe_ref": "pipe:P"}}});
    let intent = reconnect(&model, "pipe:P", "to", "node:C", None);
    let refusal = refused(&model, &intent, "OP-RECONNECT-ATTACHMENT-UNSUPPORTED");
    assert!(refusal.iter().any(|d| d.message == "Cannot reconnect pipe:P: components is not an array, so reconnect cannot check the records it holds."), "{refusal:#?}");
    // pressure_regions as an object hiding an internal node (R11 would apply):
    // region X (E -> G), Y (G -> H), terminals E and H; P joins G.
    let mut model = base();
    for (id, x) in [("node:E", 10.0), ("node:G", 12.0), ("node:H", 14.0)] {
        model["nodes"].as_array_mut().unwrap().push(node(id, x, 0.0, 0.0));
    }
    model["pipe_segments"].as_array_mut().unwrap().extend([pipe("pipe:X", "node:E", "node:G", Z), pipe("pipe:Y", "node:G", "node:H", Z)]);
    model["load_cases"][0]["pressure_regions"] = json!({"region:XY": region("region:XY", &["pipe:X", "pipe:Y"], ["node:E", "node:H"])});
    let intent = reconnect(&model, "pipe:P", "to", "node:G", None);
    let refusal = refused(&model, &intent, "OP-RECONNECT-ATTACHMENT-UNSUPPORTED");
    assert!(refusal.iter().any(|d| d.message == "Cannot reconnect pipe:P: load_cases[0].pressure_regions is not an array, so reconnect cannot check the records it holds." && d.affected_refs == vec!["pipe:P", "case:w"]), "{refusal:#?}");
    // The same region as an array is R11.
    model["load_cases"][0]["pressure_regions"] = json!([region("region:XY", &["pipe:X", "pipe:Y"], ["node:E", "node:H"])]);
    refused(&model, &reconnect(&model, "pipe:P", "to", "node:G", None), "OP-RECONNECT-PRESSURE-REGION-BRANCH");
}

#[test]
fn allowed_node_slots_are_kept() {
    let mut model = base();
    model["supports"].as_array_mut().unwrap().push(anchor("support:C", "node:C"));
    model["load_cases"][0]["primitive_loads"] = json!([
        {"id":"load:B","category":"concentrated_force","target":{"type":"node","node":"node:B"},"direction":"global_z","magnitude":{"value":10.0,"unit":"N"},"dimension":"force","provenance":PROV},
        {"id":"load:C","category":"concentrated_force","target":{"type":"node","node":"node:C"},"direction":"global_z","magnitude":{"value":10.0,"unit":"N"},"dimension":"force","provenance":PROV}]);
    let (applied, warnings, _) = accepted(&model, &reconnect(&model, "pipe:P", "to", "node:C", None));
    assert!(warnings.is_empty(), "{warnings:?}");
    assert_eq!(applied["supports"], model["supports"]);
    assert_eq!(applied["load_cases"], model["load_cases"]);
}

/// A realized bend arc S1 → S2 (quarter circle, R = 1 m) whose component sits
/// at S1, the arc's `from` node, as insert-at-corner places it.
fn bend_model() -> (Value, [f64; 3], [f64; 3]) {
    let y = [1.0, -1.0, 0.0];
    let frame = arc_geometry([1.0, 1.0, 0.0], 1.0, y).unwrap();
    let mut model = base();
    let nodes = model["nodes"].as_array_mut().unwrap();
    nodes.push(node("node:S1", 10.0, 0.0, 0.0));
    nodes.push(node("node:S2", 11.0, 1.0, 0.0));
    model["pipe_segments"].as_array_mut().unwrap().push(pipe("pipe:S", "node:S1", "node:S2", Some(y)));
    model["components"] = json!([{"id": "component:bend", "kind": "bend", "node": "node:S1",
        "geometry": {"bend_pipe_ref": "pipe:S", "bend_radius": {"value": 1.0, "unit": "m"}},
        "mechanics_interface": {"solver_consumption": "curved_bend_macro_element"}, "provenance": PROV}]);
    (model, frame.tangent_j, frame.tangent_i)
}

#[test]
fn bend_junction_warning_at_an_arc_to_node_at_t_and_at_f() {
    // The arc arrives at S2 travelling along t_j; a straight member leaving
    // S2 along +t_j is tangent to it, one leaving at a right angle is not.
    let (mut model, tangent_j, _) = bend_model();
    let s2 = [11.0, 1.0, 0.0];
    assert_eq!(tangent_j[2], 0.0, "the arc lies in the xy-plane");
    let ahead = [s2[0] + 2.0 * tangent_j[0], s2[1] + 2.0 * tangent_j[1], 0.0];
    let side = [s2[0] - 2.0 * tangent_j[1], s2[1] + 2.0 * tangent_j[0], 0.0];
    let nodes = model["nodes"].as_array_mut().unwrap();
    nodes.push(node("node:ahead", ahead[0], ahead[1], ahead[2]));
    nodes.push(node("node:side", side[0], side[1], side[2]));
    let pipes = model["pipe_segments"].as_array_mut().unwrap();
    pipes.push(pipe("pipe:T1", "node:ahead", "node:D", Z));
    pipes.push(pipe("pipe:T2", "node:side", "node:D", Z));
    let bend = "OP-RECONNECT-BEND-JUNCTION-NOT-TANGENT".to_string();
    // At T: T1 (ahead -> S2) continues the arc: no warning.
    let (_, warnings, _) = accepted(&model, &reconnect(&model, "pipe:T1", "to", "node:S2", None));
    assert!(!warnings.contains(&bend), "tangent at T: {warnings:?}");
    // At T: T2 (side -> S2) meets it at a right angle.
    let (_, warnings, outcome) = accepted(&model, &reconnect(&model, "pipe:T2", "to", "node:S2", None));
    assert!(warnings.contains(&bend), "T: {warnings:?}");
    assert_eq!(refs_of(&outcome.diagnostics, &bend), vec![vec!["pipe:T2".to_string(), "pipe:S".into(), "node:S2".into()]]);
    // At F: pipe F keeps its fixed end at S2 while its other end moves.
    let mut at_f = model.clone();
    at_f["pipe_segments"].as_array_mut().unwrap().push(pipe("pipe:F", "node:S2", "node:B", Z));
    let (_, _, outcome) = accepted(&at_f, &reconnect(&at_f, "pipe:F", "to", "node:side", None));
    assert_eq!(refs_of(&outcome.diagnostics, &bend), vec![vec!["pipe:F".to_string(), "pipe:S".into(), "node:S2".into()]]);
    let (_, warnings, _) = accepted(&at_f, &reconnect(&at_f, "pipe:F", "to", "node:ahead", None));
    assert!(!warnings.contains(&bend), "tangent at F: {warnings:?}");
    // The bend component sits at S1 (the arc's from node): joining S1 is R9.
    refused(&model, &reconnect(&model, "pipe:T2", "to", "node:S1", None), "OP-RECONNECT-COMPONENT-ATTACHED");
}

#[test]
fn topology_warnings_appear_exactly_when_their_condition_holds() {
    let model = base();
    // Parallel: another pipe already joins A and C.
    let mut parallel = model.clone();
    parallel["pipe_segments"].as_array_mut().unwrap().push(pipe("pipe:AC", "node:C", "node:A", Z));
    let (_, warnings, outcome) = accepted(&parallel, &reconnect(&parallel, "pipe:P", "to", "node:C", None));
    assert!(warnings.contains(&"OP-RECONNECT-PARALLEL-PIPE".to_string()), "{warnings:?}");
    assert!(!warnings.contains(&"OP-RECONNECT-COINCIDENT-SPAN".to_string()));
    assert_eq!(refs_of(&outcome.diagnostics, "OP-RECONNECT-PARALLEL-PIPE")[0], vec!["pipe:P", "pipe:AC", "node:A", "node:C"]);
    // Coincident: another pipe over the same positions through a distinct node.
    let mut coincident = model.clone();
    coincident["nodes"].as_array_mut().unwrap().push(node("node:C2", 2.0, 2.0 + 1e-12, 0.0));
    coincident["pipe_segments"].as_array_mut().unwrap().push(pipe("pipe:AC2", "node:A", "node:C2", Z));
    let (_, warnings, _) = accepted(&coincident, &reconnect(&coincident, "pipe:P", "to", "node:C", None));
    assert!(warnings.contains(&"OP-RECONNECT-COINCIDENT-SPAN".to_string()), "{warnings:?}");
    assert!(!warnings.contains(&"OP-RECONNECT-PARALLEL-PIPE".to_string()));
    // Outside the tolerance: no warning.
    let mut apart = model.clone();
    apart["nodes"].as_array_mut().unwrap().push(node("node:C2", 2.0, 2.0 + 1e-6, 0.0));
    apart["pipe_segments"].as_array_mut().unwrap().push(pipe("pipe:AC2", "node:A", "node:C2", Z));
    let (_, warnings, _) = accepted(&apart, &reconnect(&apart, "pipe:P", "to", "node:C", None));
    assert!(!warnings.contains(&"OP-RECONNECT-COINCIDENT-SPAN".to_string()), "{warnings:?}");
    // Orphan: R's to end leaves C (its only pipe) for A; the graph stays one part.
    let (_, warnings, _) = accepted(&model, &reconnect(&model, "pipe:R", "to", "node:A", None));
    assert_eq!(warnings, vec!["OP-RECONNECT-NODE-ORPHANED"]);
    // Split: P leaves B for a new isolated node; A-P-E is a new part.
    let mut split = model.clone();
    split["nodes"].as_array_mut().unwrap().push(node("node:E", 0.0, -3.0, 0.0));
    let (_, warnings, _) = accepted(&split, &reconnect(&split, "pipe:P", "to", "node:E", None));
    assert_eq!(warnings, vec!["OP-RECONNECT-MODEL-SPLIT"]);
    // Junction: D already joins Q and R; P joining it makes three.
    let (_, warnings, outcome) = accepted(&model, &reconnect(&model, "pipe:P", "from", "node:D", None));
    assert!(warnings.contains(&"OP-RECONNECT-JUNCTION-WITHOUT-BRANCH-COMPONENT".to_string()), "{warnings:?}");
    let junction = outcome.diagnostics.iter().find(|d| d.code == "OP-RECONNECT-JUNCTION-WITHOUT-BRANCH-COMPONENT").unwrap();
    assert!(junction.message.starts_with("node:D now joins 3 pipes"), "{}", junction.message);
    // With a terminal marker at the junction it is named.
    let mut marked = model.clone();
    marked["components"] = json!([{"id":"component:end-D","kind":"terminal","node":"node:D","provenance":PROV}]);
    let (_, _, outcome) = accepted(&marked, &reconnect(&marked, "pipe:P", "from", "node:D", None));
    let junction = outcome.diagnostics.iter().find(|d| d.code == "OP-RECONNECT-JUNCTION-WITHOUT-BRANCH-COMPONENT").unwrap();
    assert!(junction.affected_refs.contains(&"component:end-D".to_string()));
    // A cycle is allowed with no diagnostic: with A-B present, P from A to C
    // closes A-B-D-C-A.
    let mut cycle = model.clone();
    cycle["pipe_segments"].as_array_mut().unwrap().push(pipe("pipe:AB", "node:A", "node:B", Z));
    let (_, warnings, _) = accepted(&cycle, &reconnect(&cycle, "pipe:P", "to", "node:C", None));
    assert!(warnings.is_empty(), "a cycle gives no diagnostic: {warnings:?}");
}

// ------------------------------------------------------------------ batches

#[test]
fn batches_create_then_reconnect_and_reconnect_then_delete_are_atomic() {
    let model = base();
    let claim = hash(&model);
    // [create_node, reconnect]: reconnect to a new point; each step validates
    // against a fresh hash of the temporary state.
    let mut after_create = model.clone();
    after_create["nodes"].as_array_mut().unwrap().push(json!({"id":"node:N","label":"Created node","position":{"x":0.0,"y":5.0,"z":0.0},"provenance":PROV}));
    let new_point = batch("batch:new-point", vec![create_node("node:N", 0.0, 5.0, 0.0), reconnect(&after_create, "pipe:P", "to", "node:N", None)]);
    let validated = validate_operation_batch(&model, &new_point, Some(&claim));
    assert_eq!(validated["validation"]["batch_validation_status"], "passed", "{validated:#}");
    let outcome = apply_operation_batch(&model, &new_point, Some(&claim));
    assert_eq!(outcome["validation"]["application_status"], "applied_to_session_model", "{outcome:#}");
    let applied = &outcome["applied_model"];
    assert_eq!(record(applied, "pipe_segments", "pipe:P")["to"], "node:N");
    let step = &outcome["operation_outcomes"][1];
    let warnings: Vec<&str> = step["diagnostics"].as_array().unwrap().iter().map(|d| d["code"].as_str().unwrap()).collect();
    assert_eq!(warnings, vec!["OP-RECONNECT-MODEL-SPLIT"]);
    // [reconnect, delete_node O]: A leaves P with no pipe and no support.
    let mut free = base();
    free["supports"] = json!([anchor("support:D", "node:D")]);
    let mut moved = free.clone();
    moved["pipe_segments"][0]["from"] = json!("node:C");
    let cleanup = batch("batch:cleanup", vec![reconnect(&free, "pipe:P", "from", "node:C", None), delete_node(&moved, "node:A")]);
    let outcome = apply_operation_batch(&free, &cleanup, Some(&hash(&free)));
    assert_eq!(outcome["validation"]["application_status"], "applied_to_session_model", "{outcome:#}");
    assert!(outcome["applied_model"]["nodes"].as_array().unwrap().iter().all(|n| n["id"] != "node:A"));
    // A late refusal rolls the reconnect back: A still carries its support.
    let late = batch("batch:late", vec![reconnect(&model, "pipe:P", "from", "node:C", None), delete_node(&moved, "node:A")]);
    let outcome = apply_operation_batch(&model, &late, Some(&claim));
    assert_eq!(outcome["simulation_disposition"], "rolled_back_no_model_published", "{outcome:#}");
    assert_eq!(outcome["operation_outcomes"][0]["simulation_status"], "validated_on_temporary_state");
    assert_eq!(outcome["operation_outcomes"][1]["simulation_status"], "blocked");
    assert!(outcome["applied_model"].is_null());
}

// ------------------------------------------------------------------ property

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0 >> 33
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}
fn random_model(seed: u64) -> Value {
    let mut rng = Lcg(seed);
    let count = 3 + rng.below(10) as usize;
    let mut positions: Vec<[f64; 3]> = Vec::new();
    for i in 0..count {
        // Occasional coincident positions exercise R6 and the coincident warning.
        if i > 0 && rng.below(7) == 0 {
            let j = rng.below(i as u64) as usize;
            positions.push(positions[j]);
        } else {
            positions.push([rng.below(5) as f64, rng.below(5) as f64, 0.0]);
        }
    }
    let nodes: Vec<Value> = positions.iter().enumerate().map(|(i, p)| node(&format!("node:{i}"), p[0], p[1], p[2])).collect();
    let mut pipes = Vec::new();
    let add = |rng: &mut Lcg, a: usize, b: usize, pipes: &mut Vec<Value>| {
        if positions[a] == positions[b] {
            return;
        }
        let y = if rng.below(4) == 0 { None } else { Z };
        pipes.push(pipe(&format!("pipe:{}", pipes.len()), &format!("node:{a}"), &format!("node:{b}"), y));
    };
    for i in 1..count {
        let j = rng.below(i as u64) as usize;
        add(&mut rng, j, i, &mut pipes);
    }
    for _ in 0..rng.below(4) {
        let (a, b) = (rng.below(count as u64) as usize, rng.below(count as u64) as usize);
        if a != b {
            add(&mut rng, a, b, &mut pipes);
        }
    }
    document(nodes, pipes, vec![])
}
fn position_of(model: &Value, id: &str) -> [f64; 3] {
    let p = &record(model, "nodes", id)["position"];
    [p["x"].as_f64().unwrap(), p["y"].as_f64().unwrap(), p["z"].as_f64().unwrap()]
}
fn parts(model: &Value) -> usize {
    let mut adjacency: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for p in model["pipe_segments"].as_array().unwrap() {
        let (a, b) = (p["from"].as_str().unwrap().to_string(), p["to"].as_str().unwrap().to_string());
        adjacency.entry(a.clone()).or_default().push(b.clone());
        adjacency.entry(b).or_default().push(a);
    }
    let mut seen = BTreeSet::new();
    let mut count = 0;
    for start in adjacency.keys() {
        if seen.insert(start.clone()) {
            count += 1;
            let mut stack = vec![start.clone()];
            while let Some(n) = stack.pop() {
                for m in &adjacency[&n] {
                    if seen.insert(m.clone()) {
                        stack.push(m.clone());
                    }
                }
            }
        }
    }
    count
}
/// The §2.4 warnings this random family can raise, recomputed independently.
fn expected_warnings(before: &Value, after: &Value, pipe_id: &str, old: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let p = record(after, "pipe_segments", pipe_id);
    let (f, t) = (p["from"].as_str().unwrap(), p["to"].as_str().unwrap());
    let (pf, pt) = (position_of(after, f), position_of(after, t));
    let span = ((pt[0] - pf[0]).powi(2) + (pt[1] - pf[1]).powi(2) + (pt[2] - pf[2]).powi(2)).sqrt();
    for q in after["pipe_segments"].as_array().unwrap() {
        if q["id"] == pipe_id {
            continue;
        }
        let (a, b) = (q["from"].as_str().unwrap(), q["to"].as_str().unwrap());
        if (a == f && b == t) || (a == t && b == f) {
            out.insert("OP-RECONNECT-PARALLEL-PIPE".to_string());
            continue;
        }
        let (pa, pb) = (position_of(after, a), position_of(after, b));
        let d = |u: [f64; 3], v: [f64; 3]| ((u[0] - v[0]).powi(2) + (u[1] - v[1]).powi(2) + (u[2] - v[2]).powi(2)).sqrt();
        let tol = 1e-9 * span;
        if (d(pa, pf) <= tol && d(pb, pt) <= tol) || (d(pa, pt) <= tol && d(pb, pf) <= tol) {
            out.insert("OP-RECONNECT-COINCIDENT-SPAN".to_string());
        }
    }
    let degree = |n: &str| after["pipe_segments"].as_array().unwrap().iter().filter(|q| q["from"] == n || q["to"] == n).count();
    if degree(old) == 0 {
        out.insert("OP-RECONNECT-NODE-ORPHANED".to_string());
    }
    if parts(after) > parts(before) {
        out.insert("OP-RECONNECT-MODEL-SPLIT".to_string());
    }
    let moved_to = if p["from"] != record(before, "pipe_segments", pipe_id)["from"] { f } else { t };
    if degree(moved_to) >= 3 {
        out.insert("OP-RECONNECT-JUNCTION-WITHOUT-BRANCH-COMPONENT".to_string());
    }
    if p.get("y_reference").is_none() {
        out.insert("OP-RECONNECT-ORIENTATION-MISSING".to_string());
    }
    out
}

#[test]
fn property_random_graphs_every_pipe_end_and_target() {
    let mut applied_count = 0;
    let mut refused_count = 0;
    let mut inverses = 0;
    for seed in 0..40u64 {
        let model = random_model(seed);
        let claim = hash(&model);
        let pipes: Vec<Value> = model["pipe_segments"].as_array().unwrap().clone();
        let nodes: Vec<String> = model["nodes"].as_array().unwrap().iter().map(|n| n["id"].as_str().unwrap().to_string()).collect();
        let ids = |m: &Value, k: &str| m[k].as_array().unwrap().iter().map(|r| r["id"].clone()).collect::<Vec<_>>();
        for p in &pipes {
            let pipe_id = p["id"].as_str().unwrap();
            for end in ["from", "to"] {
                for (k, target) in nodes.iter().enumerate() {
                    // Alternate between keeping y_reference and entering one.
                    let entered = (seed as usize + k) % 3 == 0;
                    let y = entered.then(|| json!({"x": 0.0, "y": 0.0, "z": 1.0}));
                    let intent = reconnect(&model, pipe_id, end, target, y.clone());
                    let outcome = apply_operation(&model, &intent, Some(&claim));
                    let Some(after) = outcome.applied_model.clone() else {
                        refused_count += 1;
                        assert!(outcome.diff_preview.is_empty());
                        assert!(outcome.diagnostics.iter().any(|d| d.severity == "blocking"));
                        continue;
                    };
                    applied_count += 1;
                    assert_eq!(ids(&model, "nodes"), ids(&after, "nodes"));
                    assert_eq!(ids(&model, "pipe_segments"), ids(&after, "pipe_segments"));
                    for q in after["pipe_segments"].as_array().unwrap() {
                        let (a, b) = (q["from"].as_str().unwrap(), q["to"].as_str().unwrap());
                        assert!(nodes.iter().any(|n| n == a) && nodes.iter().any(|n| n == b), "endpoint exists");
                        assert_ne!(position_of(&after, a), position_of(&after, b), "no zero-length pipe");
                    }
                    let old = p[end].as_str().unwrap();
                    let warnings: BTreeSet<String> = outcome.diagnostics.iter().filter(|d| d.severity == "warning").map(|d| d.code.clone()).collect();
                    assert_eq!(warnings, expected_warnings(&model, &after, pipe_id, old), "seed {seed} {pipe_id} {end} -> {target}");
                    // The inverse reconnect, where representable (§1 Undo).
                    let had_y = p.get("y_reference").cloned();
                    if y.is_some() && had_y.is_none() {
                        continue;
                    }
                    let back = reconnect(&after, pipe_id, end, old, if y.is_some() { had_y } else { None });
                    let restored = apply_operation(&after, &back, Some(&hash(&after)));
                    let restored = restored.applied_model.unwrap_or_else(|| panic!("inverse refused: {:#?}", restored.diagnostics));
                    assert_eq!(sha256_hex(&canonical_json(&restored)), sha256_hex(&canonical_json(&model)));
                    inverses += 1;
                }
            }
        }
    }
    assert!(applied_count > 500 && refused_count > 100 && inverses > 300, "{applied_count} applied, {refused_count} refused, {inverses} inverses");
}

// ------------------------------------------------------------------ physics
//
// §4 "Numerical scrutiny": each physical expectation is stated independently
// of the product (closed-form beam statics, wind and fit equations), every
// solve asserts its status, and both solver modes run. Canonical equality with
// a directly authored model is an identity check only.
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, MechanicsEnvelope, PreviewSolverMode, StructuralStatus};

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::DenseScrutiny, PreviewSolverMode::SparseInteractive];
const E: f64 = 200e9;
const OD: f64 = 0.1;
const WALL: f64 = 0.005;
fn second_moment() -> f64 {
    let id = OD - 2.0 * WALL;
    std::f64::consts::PI * (OD.powi(4) - id.powi(4)) / 64.0
}
fn area() -> f64 {
    let id = OD - 2.0 * WALL;
    std::f64::consts::PI * (OD * OD - id * id) / 4.0
}
fn solve(model: &Value, mode: PreviewSolverMode) -> MechanicsEnvelope {
    run_linear_static_preview_value_with_mode(json!({"model": model, "materials": []}), mode).expect("request parses")
}
fn solved(model: &Value, mode: PreviewSolverMode) -> MechanicsEnvelope {
    let out = solve(model, mode);
    assert_eq!(out.status.mechanics, "MECHANICS_SOLVED", "{mode:?}: {:#?}", out.diagnostics);
    out
}
fn row<'a>(out: &'a MechanicsEnvelope, entity: &str, kind: &str, meta: Option<&str>) -> f64 {
    out.results
        .iter()
        .find(|r| r.entity_ref == entity && r.kind == kind && meta.is_none_or(|m| r.metadata.as_ref().is_some_and(|x| x.component == m || x.location == m)))
        .unwrap_or_else(|| panic!("{entity} {kind} {meta:?}"))
        .value
}
fn close(actual: f64, expected: f64, scale: f64, what: &str) {
    let tolerance = 1e-9 * if expected == 0.0 { scale } else { expected.abs() };
    assert!(actual.is_finite() && (actual - expected).abs() <= tolerance, "{what}: {actual:.17e} vs {expected:.17e}");
}
fn apply_batch(model: &Value, operations: Vec<Value>) -> Value {
    let outcome = apply_operation_batch(model, &batch("batch:physics", operations), Some(&hash(model)));
    assert_eq!(outcome["validation"]["application_status"], "applied_to_session_model", "{outcome:#}");
    outcome["applied_model"].clone()
}
/// [reconnect P.to O -> T, delete_node O]: the reconnect leaves O orphaned
/// and unrestrained (the solve would refuse it as NUMERICAL_INTEGRITY_UNRESOLVED),
/// so the batch removes it.
fn reconnect_and_drop(model: &Value, pipe_id: &str, old: &str, target: &str) -> Value {
    let mut moved = model.clone();
    moved["pipe_segments"].as_array_mut().unwrap().iter_mut().find(|p| p["id"] == pipe_id).unwrap()["to"] = json!(target);
    apply_batch(model, vec![reconnect(model, pipe_id, "to", target, None), delete_node(&moved, old)])
}
/// Cantilever: P anchored at F(0,0,0), first to O(1,0,0); T at (1.5,2,0),
/// so the new length is exactly 2.5 m and P's axis rotates out of x.
fn cantilever(load: Value) -> Value {
    let mut model = document(
        vec![node("node:F", 0.0, 0.0, 0.0), node("node:O", 1.0, 0.0, 0.0), node("node:T", 1.5, 2.0, 0.0)],
        vec![pipe("pipe:P", "node:F", "node:O", Z)],
        vec![anchor("support:F", "node:F")],
    );
    model["load_cases"][0] = load;
    model
}
/// The final geometry authored directly, without any operation.
fn cantilever_direct(load: Value) -> Value {
    let mut model = document(
        vec![node("node:F", 0.0, 0.0, 0.0), node("node:T", 1.5, 2.0, 0.0)],
        vec![pipe("pipe:P", "node:F", "node:T", Z)],
        vec![anchor("support:F", "node:F")],
    );
    model["load_cases"][0] = load;
    model
}
const L_NEW: f64 = 2.5;

#[test]
fn cantilever_tip_deflection_and_reaction_at_the_new_length() {
    let w = 1000.0;
    let load = json!({"id": "case:w", "provenance": PROV, "primitive_loads": [primitive("load:w", "distributed_force", json!({"type":"element","pipe":"pipe:P"}))]});
    let mut load = load;
    load["primitive_loads"][0]["magnitude"]["value"] = json!(w);
    let model = reconnect_and_drop(&cantilever(load.clone()), "pipe:P", "node:O", "node:T");
    let direct = cantilever_direct(load);
    assert_eq!(canonical_json(&model), canonical_json(&direct), "identity check");
    // Euler-Bernoulli cantilever under uniform w (independent closed form).
    let deflection_mm = 1000.0 * w * L_NEW.powi(4) / (8.0 * E * second_moment());
    for mode in MODES {
        let out = solved(&model, mode);
        close(row(&out, "node:T", "global_nodal_displacement_z", None), deflection_mm, 0.0, "tip uz");
        close(row(&out, "support:F", "support_reaction_component_v2", Some("Fz")), -w * L_NEW, 0.0, "anchor Fz");
        close(row(&out, "support:F", "support_reaction_component_v2", Some("Fx")), 0.0, w * L_NEW, "anchor Fx");
        close(row(&out, "support:F", "support_reaction_component_v2", Some("Fy")), 0.0, w * L_NEW, "anchor Fy");
        // Local frame: P's axis now follows (0.6, 0.8, 0); the solve reports
        // local end forces, |shear| = wL at the fixed end and 0 at the tip.
        let shear: Vec<f64> = out.results.iter().filter(|r| r.entity_ref == "pipe:P" && r.kind.starts_with("element_local_shear_force")).map(|r| r.value).collect();
        assert!(!shear.is_empty(), "local results reported");
        let fixed_end = shear_at(&out, "end_i");
        let tip = shear_at(&out, "end_j");
        close(fixed_end, w * L_NEW, 0.0, "|shear| at F");
        close(tip, 0.0, w * L_NEW, "|shear| at T");
        // Bit-for-bit identity with the directly authored model, per mode.
        assert_eq!(serde_json::to_value(&out.results).unwrap(), serde_json::to_value(&solved(&direct, mode).results).unwrap());
    }
}
fn shear_at(out: &MechanicsEnvelope, end: &str) -> f64 {
    let component = |kind: &str| row(out, "pipe:P", kind, Some(end));
    component("element_local_shear_force_y").hypot(component("element_local_shear_force_z"))
}

#[test]
fn built_topology_names_f_and_t_and_never_o() {
    // O kept with a full anchor so the model solves; P's ends are F and T.
    let w = 1000.0;
    let mut load = json!({"id": "case:w", "provenance": PROV, "primitive_loads": [primitive("load:w", "distributed_force", json!({"type":"element","pipe":"pipe:P"}))]});
    load["primitive_loads"][0]["magnitude"]["value"] = json!(w);
    let mut model = cantilever(load);
    model["supports"].as_array_mut().unwrap().push(anchor("support:O", "node:O"));
    let (applied, warnings, _) = accepted(&model, &reconnect(&model, "pipe:P", "to", "node:T", None));
    assert_eq!(warnings, vec!["OP-RECONNECT-MEMBER-LOAD-LENGTH-CHANGED", "OP-RECONNECT-NODE-ORPHANED"]);
    for mode in MODES {
        let out = solved(&applied, mode);
        // The whole load reaches F; O carries nothing, so no element ends at O.
        close(row(&out, "support:F", "support_reaction_component_v2", Some("Fz")), -w * L_NEW, 0.0, "F carries wL");
        for component in ["Fx", "Fy", "Fz", "Mx", "My", "Mz"] {
            close(row(&out, "support:O", "support_reaction_component_v2", Some(component)), 0.0, w * L_NEW, "O carries nothing");
        }
        // T is P's free end j: the cantilever tip.
        close(row(&out, "node:T", "global_nodal_displacement_z", None), 1000.0 * w * L_NEW.powi(4) / (8.0 * E * second_moment()), 0.0, "tip at T");
        close(shear_at(&out, "end_i"), w * L_NEW, 0.0, "end i is F");
        close(shear_at(&out, "end_j"), 0.0, w * L_NEW, "end j is T");
    }
}

#[test]
fn wind_exposed_span_resultant_follows_the_new_length() {
    let (pressure, shape, a, b) = (500.0, 0.7, 0.25, 0.75);
    let wind = |m: &mut Value| {
        m["load_cases"][0]["equivalent_static"] = json!({"wind": {"pressure": {"value": pressure, "unit": "Pa"}, "shape_factor": {"value": shape, "unit": "1"}, "direction": "global_z",
            "exposed_spans": [{"pipe_ref": "pipe:P", "start_fraction": {"value": a, "unit": "1"}, "end_fraction": {"value": b, "unit": "1"}}]}, "provenance": PROV});
    };
    let load = json!({"id": "case:w", "primitive_loads": [], "provenance": PROV});
    let mut before = cantilever(load.clone());
    wind(&mut before);
    let mut direct = cantilever_direct(load);
    wind(&mut direct);
    let model = reconnect_and_drop(&before, "pipe:P", "node:O", "node:T");
    assert_eq!(canonical_json(&model), canonical_json(&direct), "identity check");
    // Per-length wind force p * Cs * D over (b - a) of the new length.
    let resultant = pressure * shape * OD * (b - a) * L_NEW;
    for mode in MODES {
        let out = solved(&model, mode);
        close(row(&out, "support:F", "support_reaction_component_v2", Some("Fz")), -resultant, 0.0, "wind resultant");
        assert_eq!(serde_json::to_value(&out.results).unwrap(), serde_json::to_value(&solved(&direct, mode).results).unwrap());
    }
}

/// An exact_pressure_v3 document with the load/reference-state group: P
/// between two anchors, fit by a natural length change Δ at the installation
/// temperature (no thermal strain). Mirrors the maintained signed-fit wire.
fn fit_document(nodes: Vec<Value>, pipe_to: &str, supports: Vec<Value>, delta_mm: f64) -> Value {
    let qv = |v: f64, u: &str| json!({"value": v, "unit": u});
    let mut model = document(nodes, vec![pipe("pipe:P", "node:F", pipe_to, Z)], supports.clone());
    model["pressure_contract"] = json!({"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"});
    model["pipe_segments"][0]["section"]["corrosion_allowance"] = qv(0.0, "m");
    model["materials"] = json!([{"id": "material:steel", "constitutive_basis": "homogeneous_isotropic_E_nu_v1", "elastic_modulus": qv(E, "Pa"), "poisson_ratio": qv(0.3, "1"),
        "temperature_points": [{"id": "point:cold", "temperature": qv(20.0, "degC"), "elastic_modulus": qv(E, "Pa"), "poisson_ratio": qv(0.3, "1"), "provenance": PROV}],
        "expansion_laws": [{"id": "law:constant", "definition": "engineering_secant", "datum_temperature": qv(20.0, "degC"), "data": {"kind": "constant", "coefficient": qv(1e-5, "1/K")}, "provenance": PROV}],
        "provenance": PROV}]);
    model["reference_configurations"] = json!([{"id": "reference:installed", "label": "Installed reference", "geometry_ref": {"kind": "authored_model_geometry"},
        "member_references": [{"pipe_ref": "pipe:P", "basis": {"kind": "temperature_reference", "installation_temperature": qv(20.0, "degC")},
            "fit": {"kind": "natural_length_change", "length_change": qv(delta_mm, "mm")}, "provenance": PROV}], "provenance": PROV}]);
    let support_states: Vec<Value> = supports.iter().map(|s| json!({"support_ref": s["id"], "participation": {"kind": "active_model_device"}})).collect();
    model["load_cases"] = json!([{"id": "case:fit", "pressure_regions": [], "primitive_loads": [], "provenance": PROV,
        "analysis_state": {"contract": "openpipestress.load_reference_state/1.0.0", "reference_configuration_ref": "reference:installed",
            "element_states": [{"pipe_ref": "pipe:P", "operating_temperature": qv(20.0, "degC"),
                "material_selection": {"kind": "exact_point", "material_ref": "material:steel", "point_ref": "point:cold"},
                "thermal_state": {"kind": "free_length_state", "expansion_law_ref": "law:constant"}}],
            "support_states": support_states, "load_sources": [], "history": {"kind": "independent_equilibrium"}, "provenance": PROV}}]);
    model
}

#[test]
fn fit_axial_force_follows_the_new_length() {
    let delta_mm = -2.0;
    let supports = vec![anchor("support:F", "node:F"), anchor("support:T", "node:T")];
    let before = fit_document(
        vec![node("node:F", 0.0, 0.0, 0.0), node("node:O", 1.0, 0.0, 0.0), node("node:T", 1.5, 2.0, 0.0)],
        "node:O", supports.clone(), delta_mm);
    let direct = fit_document(vec![node("node:F", 0.0, 0.0, 0.0), node("node:T", 1.5, 2.0, 0.0)], "node:T", supports, delta_mm);
    let mut moved = before.clone();
    moved["pipe_segments"][0]["to"] = json!("node:T");
    let outcome = apply_operation_batch(&before, &batch("batch:fit", vec![reconnect(&before, "pipe:P", "to", "node:T", None), delete_node(&moved, "node:O")]), Some(&hash(&before)));
    assert_eq!(outcome["validation"]["application_status"], "applied_to_session_model", "{outcome:#}");
    let warnings: Vec<&str> = outcome["operation_outcomes"][0]["diagnostics"].as_array().unwrap().iter().map(|d| d["code"].as_str().unwrap()).collect();
    assert_eq!(warnings, vec!["OP-RECONNECT-MEMBER-FIT-REVIEW", "OP-RECONNECT-NODE-ORPHANED"]);
    let model = outcome["applied_model"].clone();
    assert_eq!(canonical_json(&model), canonical_json(&direct), "identity check");
    // Fixed ends: N = -E A Δ / L (Δ < 0 shortens the natural length: tension).
    let expected = E * area() * (delta_mm / 1000.0).abs() / L_NEW;
    for mode in MODES {
        let out = solved(&model, mode);
        for end in ["end_i", "end_j"] {
            close(row(&out, "pipe:P", "element_local_axial_force", Some(end)).abs(), expected, 0.0, "|N|");
        }
        assert_eq!(serde_json::to_value(&out.results).unwrap(), serde_json::to_value(&solved(&direct, mode).results).unwrap());
    }
}

/// Pinned behaviour: the solve refuses the orphaned, unrestrained node as
/// NUMERICAL_INTEGRITY_UNRESOLVED (a zero original diagonal at O's first
/// DOF, "no physical nullity inferred"), not as a witnessed mechanism, and
/// publishes no result.
#[test]
fn orphaning_an_unsupported_node_refuses_the_solve_without_a_result() {
    // P: F -> O, Q: T -> U anchored at T; P's to end moves from O to T and
    // O, unsupported, is left with no pipe.
    let mut model = document(
        vec![node("node:F", 0.0, 0.0, 0.0), node("node:O", 1.0, 0.0, 0.0), node("node:T", 0.0, 2.0, 0.0), node("node:U", 1.0, 2.0, 0.0)],
        vec![pipe("pipe:P", "node:F", "node:O", Z), pipe("pipe:Q", "node:T", "node:U", Z)],
        vec![anchor("support:F", "node:F"), anchor("support:T", "node:T")],
    );
    model["load_cases"][0]["primitive_loads"] = json!([primitive("load:w", "distributed_force", json!({"type":"element","pipe":"pipe:P"}))]);
    let (applied, warnings, outcome) = accepted(&model, &reconnect(&model, "pipe:P", "to", "node:T", None));
    assert!(warnings.contains(&"OP-RECONNECT-NODE-ORPHANED".to_string()));
    // The warning states the as-built solve behaviour.
    let orphan = outcome.diagnostics.iter().find(|d| d.code == "OP-RECONNECT-NODE-ORPHANED").unwrap();
    assert_eq!(orphan.message, "Node node:O has no pipe after this reconnect; it and its attachments (listed: none) are kept. If it has no restraint either, the solve refuses the case as NUMERICAL_INTEGRITY_UNRESOLVED and publishes no result.");
    assert!(!orphan.message.contains("mechanism"));
    for mode in MODES {
        solved(&model, mode);
        let out = solve(&applied, mode);
        assert_ne!(out.status.mechanics, "MECHANICS_SOLVED", "{mode:?}");
        assert!(out.results.is_empty(), "{mode:?}: no result is published");
        let refusal = out.diagnostics.iter().find(|d| d.code == "NUMERICAL_INTEGRITY_UNRESOLVED" && d.severity == "blocking").unwrap_or_else(|| panic!("{mode:?}: {:#?}", out.diagnostics));
        assert!(refusal.message.contains("zero original diagonal") && refusal.message.contains("global_dof: Some(6)"), "{}", refusal.message);
        assert!(out.numerical_quality.cases.iter().all(|c| c.structural_status != StructuralStatus::PassiveModelBasis), "{mode:?}");
    }
}

/// exact_pressure_v3: a region over P1 (A -> B) and P2 (B -> C) with
/// terminals A and C; the non-member P3 runs from D to the internal node B.
fn region_document() -> Value {
    let mut model = document(
        vec![node("node:A", 0.0, 0.0, 0.0), node("node:B", 1.0, 0.0, 0.0), node("node:C", 2.0, 0.0, 0.0), node("node:D", 1.0, 1.0, 0.0)],
        vec![pipe("pipe:P1", "node:A", "node:B", Z), pipe("pipe:P2", "node:B", "node:C", Z), pipe("pipe:P3", "node:D", "node:B", Some([1.0, 0.0, 0.0]))],
        vec![anchor("support:A", "node:A"), anchor("support:C", "node:C"), anchor("support:D", "node:D")],
    );
    model["pressure_contract"] = json!({"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"});
    for p in model["pipe_segments"].as_array_mut().unwrap() {
        p["section"]["corrosion_allowance"] = json!({"value": 0.0, "unit": "m"});
    }
    model["materials"] = json!([{"id": "material:steel", "constitutive_basis": "homogeneous_isotropic_E_nu_v1", "elastic_modulus": {"value": E, "unit": "Pa"}, "poisson_ratio": {"value": 0.3, "unit": "1"}, "provenance": PROV}]);
    model["load_cases"][0]["pressure_regions"] = json!([region("region:AC", &["pipe:P1", "pipe:P2"], ["node:A", "node:C"])]);
    model
}

#[test]
fn exact_pressure_internal_branch_is_cleared_by_reconnecting_away() {
    let model = region_document();
    for mode in MODES {
        let out = solve(&model, mode);
        assert_ne!(out.status.mechanics, "MECHANICS_SOLVED");
        assert!(out.diagnostics.iter().any(|d| d.code == "PRESSURE_REGION_INTERNAL_BRANCH"), "{mode:?}: {:#?}", out.diagnostics);
    }
    // P3 moves away from B to a new node E: [create_node, reconnect]. No
    // region refusal, and it solves.
    let mut with_e = model.clone();
    with_e["nodes"].as_array_mut().unwrap().push(json!({"id":"node:E","label":"Created node","position":{"x":1.0,"y":1.0,"z":1.0},"provenance":PROV}));
    let outcome = apply_operation_batch(&model, &batch("batch:away", vec![create_node("node:E", 1.0, 1.0, 1.0), reconnect(&with_e, "pipe:P3", "to", "node:E", None)]), Some(&hash(&model)));
    assert_eq!(outcome["validation"]["application_status"], "applied_to_session_model", "{outcome:#}");
    let warnings: Vec<&str> = outcome["operation_outcomes"][1]["diagnostics"].as_array().unwrap().iter().map(|d| d["code"].as_str().unwrap()).collect();
    assert_eq!(warnings, vec!["OP-RECONNECT-MODEL-SPLIT"]);
    let away = outcome["applied_model"].clone();
    for mode in MODES {
        solved(&away, mode);
    }
    // P3 joins the region's terminal C instead: a review warning, and it solves.
    let (onto, warnings, _) = accepted(&model, &reconnect(&model, "pipe:P3", "to", "node:C", None));
    assert_eq!(warnings, vec!["OP-RECONNECT-PRESSURE-TERMINAL-REVIEW"]);
    for mode in MODES {
        solved(&onto, mode);
    }
    // Back onto the internal node B is R11.
    let mut moved = away.clone();
    moved["load_cases"][0]["pressure_regions"] = model["load_cases"][0]["pressure_regions"].clone();
    refused(&moved, &reconnect(&moved, "pipe:P3", "to", "node:B", None), "OP-RECONNECT-PRESSURE-REGION-BRANCH");
}

#[test]
fn kernel_refuses_the_near_parallel_case_and_it_never_reaches_the_solve() {
    let mut model = document(
        vec![node("node:F", 0.0, 0.0, 0.0), node("node:O", 1.0, 0.0, 0.0), node("node:T", 1.0, 1.0, 0.0)],
        vec![pipe("pipe:P", "node:F", "node:O", Some([1.00001, 1.0, 0.0]))],
        vec![anchor("support:F", "node:F")],
    );
    model["load_cases"][0]["primitive_loads"] = json!([]);
    let refusal = refused(&model, &reconnect(&model, "pipe:P", "to", "node:T", None), "OP-RECONNECT-ORIENTATION-DEGENERATE");
    assert!(refusal[0].message.contains("InvalidOrientation"), "{}", refusal[0].message);
    // The same geometry authored directly is refused by the solve by name.
    let mut direct = model.clone();
    direct["pipe_segments"][0]["to"] = json!("node:T");
    for mode in MODES {
        let out = solve(&direct, mode);
        assert_ne!(out.status.mechanics, "MECHANICS_SOLVED", "{mode:?}");
    }
}

// ------------------------------------------------------------------ runtime equivalence
//
// CLM-003 witness in the W-10 PR: the same inline model and reconnect batch as
// `apps/desktop/src/services/pipeReconnectEquivalence.test.ts`, built from each
// language's MODEL_SCHEMA_VERSION; both state the same codes, diff rows,
// warnings and applied document.
pub fn equivalence_model() -> Value {
    let mut model = document(
        vec![node("node:A", 0.0, 0.0, 0.0), node("node:B", 2.0, 0.0, 0.0), node("node:C", 2.0, 2.0, 0.0)],
        vec![pipe("pipe:P", "node:A", "node:B", Z)],
        vec![anchor("support:A", "node:A")],
    );
    model["project"]["id"] = json!("project:w10-equivalence");
    model
}

#[test]
fn runtime_equivalence_witness_native() {
    let model = equivalence_model();
    let claim = hash(&model);
    let accept = batch("batch:w10-equivalence-accept", vec![reconnect(&model, "pipe:P", "to", "node:C", None)]);
    let outcome = apply_operation_batch(&model, &accept, Some(&claim));
    assert_eq!(outcome["validation"]["application_status"], "applied_to_session_model", "{outcome:#}");
    let step = &outcome["operation_outcomes"][0];
    assert_eq!(step["diff_preview"], json!([{
        "entity_ref": "pipe:P", "object_type": "Element", "field_path": "pipe_segments",
        "before": "{\"from\":\"node:A\",\"id\":\"pipe:P\",\"label\":\"P\",\"material\":\"material:steel\",\"provenance\":\"invented W-10 reconnect test input\",\"section\":{\"outside_diameter\":{\"unit\":\"m\",\"value\":0.1},\"wall_thickness\":{\"unit\":\"m\",\"value\":0.005}},\"to\":\"node:B\",\"y_reference\":{\"x\":0,\"y\":0,\"z\":1}}",
        "after": "{\"from\":\"node:A\",\"id\":\"pipe:P\",\"label\":\"P\",\"material\":\"material:steel\",\"provenance\":\"invented W-10 reconnect test input\",\"section\":{\"outside_diameter\":{\"unit\":\"m\",\"value\":0.1},\"wall_thickness\":{\"unit\":\"m\",\"value\":0.005}},\"to\":\"node:C\",\"y_reference\":{\"x\":0,\"y\":0,\"z\":1}}",
        "unit": "none", "dimension": "dimensionless", "change_kind": "reconnect_pipe_run"}]));
    let warnings: Vec<(&str, &str, Vec<&str>)> = step["diagnostics"].as_array().unwrap().iter()
        .map(|d| (d["code"].as_str().unwrap(), d["severity"].as_str().unwrap(), d["affected_refs"].as_array().unwrap().iter().map(|r| r.as_str().unwrap()).collect()))
        .collect();
    assert_eq!(warnings, vec![("OP-RECONNECT-NODE-ORPHANED", "warning", vec!["pipe:P", "node:B"])]);
    let mut expected = model.clone();
    expected["pipe_segments"][0]["to"] = json!("node:C");
    assert_eq!(outcome["applied_model"], expected);
    println!("W10-EQUIVALENCE applied sha256:{}", sha256_hex(&canonical_json(&outcome["applied_model"])));
    // One R9 refusal: a tee at B.
    let mut tee = model.clone();
    tee["components"] = json!([{"id": "component:tee", "kind": "tee", "node": "node:B", "provenance": PROV}]);
    let refused_outcome = apply_operation_batch(&tee, &batch("batch:w10-equivalence-refuse", vec![reconnect(&tee, "pipe:P", "to", "node:C", None)]), Some(&hash(&tee)));
    let step = &refused_outcome["operation_outcomes"][0];
    assert!(refused_outcome["applied_model"].is_null());
    assert_eq!(step["diff_preview"], json!([]));
    let blockers: Vec<(&str, &str, Vec<&str>)> = step["diagnostics"].as_array().unwrap().iter()
        .map(|d| (d["code"].as_str().unwrap(), d["message"].as_str().unwrap(), d["affected_refs"].as_array().unwrap().iter().map(|r| r.as_str().unwrap()).collect()))
        .collect();
    assert_eq!(blockers, vec![(
        "OP-RECONNECT-COMPONENT-ATTACHED",
        "Cannot reconnect pipe:P: component component:tee (tee) sits at node node:B, where this reconnect removes the pipe.",
        vec!["pipe:P", "component:tee", "node:B"],
    )]);
}

/// Model document 0.9.0 (DEL-02-01 Design `model-document-0.9.0.md` §1.4): a reconnected
/// pipe keeps its condition_set_ref; the applied document still validates.
#[test]
fn a_reconnected_pipe_keeps_its_condition_set_ref() {
    let mut model = base();
    model["condition_sets"] = json!([{"id": "cs:line", "provenance": PROV}]);
    model["pipe_segments"][0]["condition_set_ref"] = json!("cs:line");
    let (applied, warnings, _) = accepted(&model, &reconnect(&model, "pipe:P", "to", "node:C", None));
    assert!(warnings.is_empty(), "{warnings:?}");
    let pipe = record(&applied, "pipe_segments", "pipe:P");
    assert_eq!(pipe["to"], "node:C");
    assert_eq!(pipe["condition_set_ref"], "cs:line");
    assert_eq!(applied["condition_sets"], model["condition_sets"]);
}
