//! Typed authoring operations for the model 0.4.0 load/reference-state records
//! (T1 WP3): `Model/reference_configurations`, `Material/expansion_laws` and
//! `Load/analysis_state`, carried through the common Review/Apply, inverse
//! (Undo/Redo) and atomic-batch seam.
//!
//! Base models are the maintained product witnesses
//! `fixtures/product_preview/load_reference_source/eigen_motion.request.json`
//! and `fixtures/product_preview/load_reference/connected.request.json`. Every
//! edit below is an invented test input. Closed-form expectations are computed
//! from the witness constants (node positions, entered motions, entered strains,
//! entered coefficients), never from a producer output.
use open_pipe_stress_operation_applier::{
    apply_operation, apply_operation_batch, canonical_json, sha256_hex, validate_operation,
    validate_operation_batch,
};
use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, PreviewSolverMode,
};
use serde_json::{json, Value};

const EIGEN_MOTION: &str = include_str!(
    "../../../../fixtures/product_preview/load_reference_source/eigen_motion.request.json"
);
const CONNECTED: &str =
    include_str!("../../../../fixtures/product_preview/load_reference/connected.request.json");

fn request(text: &str) -> Value {
    serde_json::from_str(text).unwrap()
}
fn eigen_motion() -> Value {
    request(EIGEN_MOTION)["model"].clone()
}
fn connected() -> Value {
    request(CONNECTED)["model"].clone()
}
fn hash(model: &Value) -> Value {
    json!({"algorithm":"sha256","canonicalization":"rfc8785_jcs","payload_scope":"model_payload",
        "value":format!("sha256:{}",sha256_hex(&canonical_json(model)))})
}
/// Display of a key as the operation seam shows it: canonical JSON, or
/// `not_present` for an absent key.
fn display(value: Option<&Value>) -> String {
    value.map(canonical_json).unwrap_or("not_present".into())
}
fn intent(object: &str, id: &str, path: &str, before: &str, after: &str) -> Value {
    json!({"operation_id":format!("op:load-state:{id}:{path}"),"operation_kind":"modify","operation_status":"proposed","author_type":"user",
      "target":{"object_type":object,"ref":id},
      "change":{"change_id":format!("change:load-state:{id}:{path}"),"change_kind":if object=="Load" {"update_load"} else {"set_field"},
        "field_label":path,"field_path":path,"before":before,"after":after,"unit":"none","dimension":"dimensionless","source_note":"Invented load/reference-state authoring input"},
      "validation":{"schema_validation":"not_run","constraint_validation":"not_run","unit_validation":"not_run","diff_preview_status":"not_generated","application_status":"not_applied"},
      "rationale":"Invented explicit user authoring, reviewed through the operation pipeline",
      "audit_boundary":{"mutation_route":"structured_operations_only","direct_model_mutation_allowed":false,"requires_user_acceptance":true,"mutates_accepted_model_state":false},
      "professional_boundary":{"human_review_required":true,"software_makes_compliance_claim":false,"software_makes_certification_claim":false,"software_makes_sealing_claim":false,"software_makes_approval_claim":false,"software_makes_authentication_claim":false}})
}
/// Review then apply; the review returns no model and exactly one diff row
/// showing the before/after displays.
fn apply(model: &Value, intent: &Value) -> Value {
    let claim = hash(model);
    let review = validate_operation(model, intent, Some(&claim));
    assert!(review.applied_model.is_none());
    assert_eq!(review.diff_preview.len(), 1, "{:?}", review.diagnostics);
    assert_eq!(review.diff_preview[0].before, intent["change"]["before"]);
    assert_eq!(review.diff_preview[0].after, intent["change"]["after"]);
    let outcome = apply_operation(model, intent, Some(&claim));
    let applied = outcome
        .applied_model
        .unwrap_or_else(|| panic!("{:?}", outcome.diagnostics));
    assert_eq!(
        outcome.applied_model_backend_hash.as_deref(),
        hash(&applied)["value"].as_str()
    );
    applied
}
/// A refusal is a blocking finding with no applied model and no mutation.
fn refused_with(model: &Value, intent: &Value, claim: Option<&Value>, code: &str) {
    let original = model.clone();
    for outcome in [
        validate_operation(model, intent, claim),
        apply_operation(model, intent, claim),
    ] {
        assert!(outcome.applied_model.is_none());
        assert!(outcome.diff_preview.is_empty());
        assert!(
            outcome
                .diagnostics
                .iter()
                .any(|d| d.code == code && d.severity == "blocking"),
            "expected {code}, got {:?}",
            outcome.diagnostics
        );
    }
    assert_eq!(model, &original);
}
fn refused(model: &Value, intent: &Value, code: &str) {
    refused_with(model, intent, Some(&hash(model)), code)
}
/// Exact bytes, in both the canonical and the plain serialization.
fn assert_same_bytes(left: &Value, right: &Value) {
    assert_eq!(canonical_json(left), canonical_json(right));
    assert_eq!(
        serde_json::to_string(left).unwrap(),
        serde_json::to_string(right).unwrap()
    );
}
/// Forward, inverse (Undo) and forward again (Redo) through the seam. `owner`
/// is the JSON pointer of the record that owns `path`. Returns the applied
/// model. The inverse restores the exact prior bytes of the whole model,
/// including the absence of the key where it was absent.
fn round_trip(
    base: &Value,
    object: &str,
    id: &str,
    owner: &str,
    path: &str,
    after: &Value,
) -> Value {
    let prior = base.pointer(owner).unwrap().get(path).cloned();
    let before = display(prior.as_ref());
    let forward = intent(
        object,
        id,
        path,
        &before,
        &serde_json::to_string(after).unwrap(),
    );
    let applied = apply(base, &forward);
    // Exactly the one key changed.
    let mut expected = base.clone();
    expected.pointer_mut(owner).unwrap()[path] = after.clone();
    assert_same_bytes(&applied, &expected);
    // Undo: the inverse operation restores the exact prior bytes.
    let undo_after = match &prior {
        Some(prior) => serde_json::to_string(prior).unwrap(),
        None => "not_present".to_string(),
    };
    let undo = intent(object, id, path, &canonical_json(after), &undo_after);
    let undone = apply(&applied, &undo);
    assert_same_bytes(&undone, base);
    assert_eq!(
        undone.pointer(owner).unwrap().get(path).is_none(),
        prior.is_none()
    );
    // Redo: the forward operation again, on the restored model.
    let redone = apply(&undone, &forward);
    assert_same_bytes(&redone, &applied);
    applied
}
fn without(mut model: Value, pointer: &str, key: &str) -> Value {
    model
        .pointer_mut(pointer)
        .unwrap()
        .as_object_mut()
        .unwrap()
        .remove(key)
        .unwrap();
    model
}
/// The model with every case's `analysis_state` removed.
fn without_states(model: &Value) -> Value {
    let mut model = model.clone();
    for case in model["load_cases"].as_array_mut().unwrap() {
        case.as_object_mut().unwrap().remove("analysis_state");
    }
    model
}
fn with_model(request_text: &str, model: &Value) -> Value {
    let mut request = request(request_text);
    request["model"] = model.clone();
    request
}
/// Solve the applied model through the product in both solver modes and
/// return the named displacement row value in mm.
fn displacement_mm(request: &Value, case: &str, node: &str, kind: &str) -> f64 {
    let mut values = Vec::new();
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        let envelope = run_linear_static_preview_value_with_mode(request.clone(), mode)
            .unwrap_or_else(|e| panic!("typed boundary refused the applied model: {e}"));
        let envelope = serde_json::to_value(&envelope).unwrap();
        assert_eq!(
            envelope["status"]["mechanics"], "MECHANICS_SOLVED",
            "{:#}",
            envelope["diagnostics"]
        );
        let row = envelope["results"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| {
                r["entity_ref"] == node && r["kind"] == kind && r["basis_ref"]["ref_id"] == case
            })
            .unwrap_or_else(|| panic!("no {kind} row for {node} in {case}"));
        assert_eq!(row["unit"], "mm");
        values.push(row["value"].as_f64().unwrap());
    }
    values[0]
}
fn close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= 1e-9 * expected.abs().max(1.0),
        "{actual} vs {expected}"
    );
}
fn length_mm(quantity: &Value) -> f64 {
    let value = quantity["value"].as_f64().unwrap();
    match quantity["unit"].as_str().unwrap() {
        "mm" => value,
        "m" => value * 1000.0,
        unit => panic!("unexpected invented length unit {unit}"),
    }
}
fn motion<'a>(state: &'a Value, support: &str, dof: &str) -> &'a Value {
    state["support_states"]
        .as_array()
        .unwrap()
        .iter()
        .find(|s| s["support_ref"] == support)
        .unwrap()["boundary_motion"]
        .as_array()
        .unwrap()
        .iter()
        .find(|m| m["dof"] == dof)
        .map(|m| &m["value"])
        .unwrap()
}
fn node_x_m(model: &Value, node: &str) -> f64 {
    model["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|n| n["id"] == node)
        .unwrap()["position"]["x"]
        .as_f64()
        .unwrap()
}

/// `connected` closed form, both members straight along X, equal E·A within a
/// case (both select the same point), ends at root (u_r) and far (u_f = 0):
/// N1 = N2 with N_i = EA (Δu_i / L_i − ε*_i) gives
/// u_m = (u_r + u_f + ε*_1 L_1 − ε*_2 L_2) / 2, where
/// ε*_i = λ_fit,i · λ_th − 1, λ_fit = 1 + ΔL / L (natural length change) and,
/// for an `engineering_secant` constant law whose datum equals the installation
/// temperature, λ_th = 1 + α (T − T_install).
fn connected_middle_ux_mm(model: &Value, case: &str) -> f64 {
    let l1 = node_x_m(model, "node:middle") - node_x_m(model, "node:root");
    let l2 = node_x_m(model, "node:far") - node_x_m(model, "node:middle");
    let state = &model["load_cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["id"] == case)
        .unwrap()["analysis_state"];
    let u_r = length_mm(motion(state, "support:root", "UX")) / 1000.0;
    let members = model["reference_configurations"][0]["member_references"]
        .as_array()
        .unwrap();
    let material = &model["materials"][0];
    let law = &material["expansion_laws"][0];
    assert_eq!(law["definition"], "engineering_secant");
    assert_eq!(law["data"]["kind"], "constant");
    assert_eq!(law["data"]["coefficient"]["unit"], "1/K");
    let alpha = law["data"]["coefficient"]["value"].as_f64().unwrap();
    let eps = |index: usize, length: f64| {
        let member = &members[index];
        let element = &state["element_states"][index];
        assert_eq!(member["pipe_ref"], element["pipe_ref"]);
        let install = &member["basis"]["installation_temperature"];
        let operating = &element["operating_temperature"];
        assert_eq!(install["unit"], "degC");
        assert_eq!(operating["unit"], "degC");
        assert_eq!(law["datum_temperature"], *install);
        let delta_t = operating["value"].as_f64().unwrap() - install["value"].as_f64().unwrap();
        let lambda_th = 1.0 + alpha * delta_t;
        let lambda_fit = match member["fit"]["kind"].as_str().unwrap() {
            "none" => 1.0,
            "natural_length_change" => {
                1.0 + length_mm(&member["fit"]["length_change"]) / 1000.0 / length
            }
            kind => panic!("unexpected invented fit {kind}"),
        };
        lambda_fit * lambda_th - 1.0
    };
    1000.0 * (u_r + 0.0 + eps(0, l1) * l1 - eps(1, l2) * l2) / 2.0
}

#[test]
fn reference_configurations_apply_undo_redo_and_resolve_as_closed_form_predicts() {
    let base = connected();
    let project = base["project"]["id"].as_str().unwrap().to_string();
    // Invented edit: the second member's natural length change -1 mm -> -3 mm.
    let mut after = base["reference_configurations"].clone();
    after[0]["member_references"][1]["fit"]["length_change"] = json!({"value": -3, "unit": "mm"});
    let applied = round_trip(
        &base,
        "Model",
        &project,
        "",
        "reference_configurations",
        &after,
    );
    // The witness itself satisfies the closed form, and the edit moves it.
    let before_cold = connected_middle_ux_mm(&base, "case:cold");
    close(
        displacement_mm(
            &with_model(CONNECTED, &base),
            "case:cold",
            "node:middle",
            "global_nodal_displacement_x",
        ),
        before_cold,
    );
    for case in ["case:cold", "case:hot"] {
        let predicted = connected_middle_ux_mm(&applied, case);
        close(
            displacement_mm(
                &with_model(CONNECTED, &applied),
                case,
                "node:middle",
                "global_nodal_displacement_x",
            ),
            predicted,
        );
    }
    // Cold case, no thermal stretch: u_m = (0.5 mm + 3 mm) / 2.
    close(connected_middle_ux_mm(&applied, "case:cold"), 1.75);
    assert!((connected_middle_ux_mm(&applied, "case:cold") - before_cold).abs() > 0.5);

    // Absent key: authoring it and its inverse restore absence exactly. No
    // case names a configuration yet (addendum 1: the inverse removal is
    // refused while a case still names one; see the inbound-reference test).
    let absent = without(without_states(&base), "", "reference_configurations");
    round_trip(
        &absent,
        "Model",
        &project,
        "",
        "reference_configurations",
        &after,
    );
}

#[test]
fn expansion_laws_apply_undo_redo_and_resolve_as_closed_form_predicts() {
    let base = connected();
    let material = base["materials"][0]["id"].as_str().unwrap().to_string();
    // Invented edit: the constant secant coefficient 1.2e-5 -> 2.0e-5 1/K.
    let mut after = base["materials"][0]["expansion_laws"].clone();
    after[0]["data"]["coefficient"]["value"] = json!(2.0e-5);
    let applied = round_trip(
        &base,
        "Material",
        &material,
        "/materials/0",
        "expansion_laws",
        &after,
    );
    let before_hot = connected_middle_ux_mm(&base, "case:hot");
    close(
        displacement_mm(
            &with_model(CONNECTED, &base),
            "case:hot",
            "node:middle",
            "global_nodal_displacement_x",
        ),
        before_hot,
    );
    let predicted = connected_middle_ux_mm(&applied, "case:hot");
    close(
        displacement_mm(
            &with_model(CONNECTED, &applied),
            "case:hot",
            "node:middle",
            "global_nodal_displacement_x",
        ),
        predicted,
    );
    // Hot case: u_m = (0.5 mm + λ_th · 1 mm) / 2 with λ_th = 1 + 2e-5 · 130.
    close(predicted, (0.5 + (1.0 + 2.0e-5 * 130.0)) / 2.0);
    assert!((predicted - before_hot).abs() > 1e-4);

    // No element names a law yet (addendum 1, as above).
    let absent = without(without_states(&base), "/materials/0", "expansion_laws");
    round_trip(
        &absent,
        "Material",
        &material,
        "/materials/0",
        "expansion_laws",
        &after,
    );
}

#[test]
fn analysis_state_apply_undo_redo_and_boundary_motion_moves_the_tip_rigidly() {
    let base = eigen_motion();
    // Invented edits: anchor UY 1 mm -> 2.5 mm, anchor RZ 1e-4 -> 3e-4 rad,
    // tip line-stop UX 5e-5 m -> 1.2e-4 m.
    let mut after = base["load_cases"][0]["analysis_state"].clone();
    after["support_states"][0]["boundary_motion"][0]["value"] = json!({"value": 2.5, "unit": "mm"});
    after["support_states"][0]["boundary_motion"][1]["value"] =
        json!({"value": 3e-4, "unit": "rad"});
    after["support_states"][2]["boundary_motion"][0]["value"] =
        json!({"value": 1.2e-4, "unit": "m"});
    let applied = round_trip(
        &base,
        "Load",
        "case:join",
        "/load_cases/0",
        "analysis_state",
        &after,
    );
    // Rigid kinematics of the witness: the tip is transversely free, so
    // u_y(tip) = u_y(root) + θ_z(root) · L; the tip's axial value is the
    // entered line-stop motion.
    let predict = |model: &Value| {
        let state = &model["load_cases"][0]["analysis_state"];
        let span_mm = 1000.0 * (node_x_m(model, "tip") - node_x_m(model, "root"));
        let rz = motion(state, "anchor", "RZ");
        assert_eq!(rz["unit"], "rad");
        (
            length_mm(motion(state, "stop", "UX")),
            length_mm(motion(state, "anchor", "UY")) + rz["value"].as_f64().unwrap() * span_mm,
        )
    };
    for (model, expected_uy) in [(&base, 1.2), (&applied, 3.1)] {
        let (ux, uy) = predict(model);
        close(uy, expected_uy);
        let request = with_model(EIGEN_MOTION, model);
        close(
            displacement_mm(&request, "case:join", "tip", "global_nodal_displacement_x"),
            ux,
        );
        close(
            displacement_mm(&request, "case:join", "tip", "global_nodal_displacement_y"),
            uy,
        );
    }

    let absent = without(base.clone(), "/load_cases/0", "analysis_state");
    round_trip(
        &absent,
        "Load",
        "case:join",
        "/load_cases/0",
        "analysis_state",
        &after,
    );
}

/// Places a payload in the model as a document key, for the parity check that
/// the product's typed boundary refuses it too.
fn product_refuses(request_text: &str, model: &Value) -> bool {
    run_linear_static_preview_value_with_mode(
        with_model(request_text, model),
        PreviewSolverMode::SparseInteractive,
    )
    .is_err()
}

#[test]
fn typed_refusals_match_the_product_typed_boundary() {
    let base = connected();
    let project = base["project"]["id"].as_str().unwrap().to_string();
    let configurations = base["reference_configurations"].clone();
    let laws = base["materials"][0]["expansion_laws"].clone();
    let state = base["load_cases"][0]["analysis_state"].clone();
    let before_configurations = canonical_json(&configurations);
    let before_laws = canonical_json(&laws);
    let before_state = canonical_json(&state);

    let mut cases: Vec<(&str, &str, &str, String, Value)> = Vec::new();
    // Unknown fields, at the top of a record and nested inside a quantity.
    let mut v = configurations.clone();
    v[0]["colour"] = json!("invented");
    cases.push((
        "Model",
        &project,
        "reference_configurations",
        before_configurations.clone(),
        v,
    ));
    let mut v = configurations.clone();
    v[0]["member_references"][0]["basis"]["installation_temperature"]["basis"] = json!("invented");
    cases.push((
        "Model",
        &project,
        "reference_configurations",
        before_configurations.clone(),
        v,
    ));
    let mut v = configurations.clone();
    v[0]["geometry_ref"]["projection_sha256"] = json!("invented");
    cases.push((
        "Model",
        &project,
        "reference_configurations",
        before_configurations.clone(),
        v,
    ));
    let mut v = laws.clone();
    v[0]["note"] = json!("invented");
    cases.push((
        "Material",
        "material:shared",
        "expansion_laws",
        before_laws.clone(),
        v,
    ));
    let mut v = state.clone();
    v["support_states"][1]["participation"]["extra"] = json!(true);
    cases.push((
        "Load",
        "case:cold",
        "analysis_state",
        before_state.clone(),
        v,
    ));
    // Unknown discriminants.
    let mut v = configurations.clone();
    v[0]["member_references"][0]["fit"] = json!({"kind": "invented_fit"});
    cases.push((
        "Model",
        &project,
        "reference_configurations",
        before_configurations.clone(),
        v,
    ));
    let mut v = laws.clone();
    v[0]["definition"] = json!("invented_definition");
    cases.push((
        "Material",
        "material:shared",
        "expansion_laws",
        before_laws.clone(),
        v,
    ));
    let mut v = state.clone();
    v["history"] = json!({"kind": "continuation"});
    cases.push((
        "Load",
        "case:cold",
        "analysis_state",
        before_state.clone(),
        v,
    ));
    // Missing required field.
    let mut v = state.clone();
    v.as_object_mut().unwrap().remove("provenance");
    cases.push((
        "Load",
        "case:cold",
        "analysis_state",
        before_state.clone(),
        v,
    ));
    // Wrong JSON shape for the whole key.
    cases.push((
        "Model",
        &project,
        "reference_configurations",
        before_configurations.clone(),
        json!({}),
    ));

    for (object, id, path, before, payload) in cases {
        let op = intent(
            object,
            id,
            path,
            &before,
            &serde_json::to_string(&payload).unwrap(),
        );
        refused(&base, &op, "OP-LOAD-STATE-PAYLOAD-INVALID");
        // Parity: the product's typed boundary refuses the same document.
        let mut document = base.clone();
        match object {
            "Model" => document[path] = payload,
            "Material" => document["materials"][0][path] = payload,
            _ => document["load_cases"][0][path] = payload,
        }
        assert!(product_refuses(CONNECTED, &document), "{object} {path}");
    }
    // The accepted witness payloads pass the product boundary unchanged.
    assert!(!product_refuses(CONNECTED, &base));

    // A duplicated key in the authored text is refused at the typed parse,
    // never resolved as last-wins.
    let text = canonical_json(&laws).replacen("\"id\":", "\"id\":\"law:first\",\"id\":", 1);
    let op = intent(
        "Material",
        "material:shared",
        "expansion_laws",
        &before_laws,
        &text,
    );
    refused(&base, &op, "OP-LOAD-STATE-PAYLOAD-INVALID");
    // Text that is not JSON.
    let op = intent("Load", "case:cold", "analysis_state", &before_state, "{");
    refused(&base, &op, "OP-LOAD-STATE-PAYLOAD-INVALID");
}

#[test]
fn explicit_null_wrong_version_missing_hash_and_envelope_are_refused() {
    let base = connected();
    let project = base["project"]["id"].as_str().unwrap().to_string();
    let ops = [
        (
            "Model",
            project.as_str(),
            "reference_configurations",
            canonical_json(&base["reference_configurations"]),
            serde_json::to_string(&base["reference_configurations"]).unwrap(),
        ),
        (
            "Material",
            "material:shared",
            "expansion_laws",
            canonical_json(&base["materials"][0]["expansion_laws"]),
            serde_json::to_string(&base["materials"][0]["expansion_laws"]).unwrap(),
        ),
        (
            "Load",
            "case:cold",
            "analysis_state",
            canonical_json(&base["load_cases"][0]["analysis_state"]),
            serde_json::to_string(&base["load_cases"][0]["analysis_state"]).unwrap(),
        ),
    ];
    for (object, id, path, before, after) in &ops {
        // The unchanged payload is admitted (a no-op review), so every refusal
        // below is caused by the one named defect.
        let op = intent(object, id, path, before, after);
        let applied = apply(&base, &op);
        assert_same_bytes(&applied, &base);

        // Explicit null is authored presence, never absence.
        refused(
            &base,
            &intent(object, id, path, before, "null"),
            "OP-LOAD-STATE-EXPLICIT-NULL",
        );
        refused(
            &base,
            &intent(object, id, path, before, " null "),
            "OP-LOAD-STATE-EXPLICIT-NULL",
        );

        // The model must already be 0.4.0; nothing sets or changes the version.
        for version in [json!("0.3.0"), json!("0.4.1"), Value::Null] {
            let mut older = base.clone();
            older["schema_version"] = version;
            refused(&older, &op, "OP-LOAD-STATE-SCHEMA-VERSION-INVALID");
        }
        let unversioned = without(base.clone(), "", "schema_version");
        refused(&unversioned, &op, "OP-LOAD-STATE-SCHEMA-VERSION-INVALID");

        // The current complete model hash is required.
        refused_with(&base, &op, None, "OP-LOAD-STATE-MODEL-HASH-REQUIRED");
        refused_with(
            &base,
            &op,
            Some(&Value::Null),
            "OP-LOAD-STATE-MODEL-HASH-REQUIRED",
        );
        let mut stale = base.clone();
        stale["project"]["name"] = json!("invented stale basis");
        refused_with(
            &base,
            &op,
            Some(&hash(&stale)),
            "OP-CLAIMED-MODEL-HASH-MISMATCH",
        );

        // Stale before-value.
        refused(
            &base,
            &intent(object, id, path, "not_present", after),
            "OP-BEFORE-VALUE-MISMATCH",
        );

        // Structured envelope and declared change kind.
        let mut envelope = op.clone();
        envelope["change"]["unit"] = json!("mm");
        refused(&base, &envelope, "OP-LOAD-STATE-ENVELOPE-INVALID");
        let mut kind = op.clone();
        kind["change"]["change_kind"] = json!(if *object == "Load" {
            "set_field"
        } else {
            "update_load"
        });
        refused(&base, &kind, "OP-RICH-KIND-INVALID");

        // Unknown target.
        let mut target = op.clone();
        target["target"]["ref"] = json!("invented:missing");
        refused(&base, &target, "OP-LOAD-STATE-TARGET-INVALID");

        // An ambiguous (duplicated) target entity is refused, never guessed.
        if *object != "Model" {
            let collection = if *object == "Material" {
                "materials"
            } else {
                "load_cases"
            };
            let mut ambiguous = base.clone();
            let duplicate = ambiguous[collection][0].clone();
            ambiguous[collection]
                .as_array_mut()
                .unwrap()
                .push(duplicate);
            refused(&ambiguous, &op, "OP-LOAD-STATE-TARGET-INVALID");
        }
    }
}

#[test]
fn duplicate_ids_and_dangling_references_are_refused() {
    let base = connected();
    let project = base["project"]["id"].as_str().unwrap().to_string();
    let configurations = base["reference_configurations"].clone();
    let laws = base["materials"][0]["expansion_laws"].clone();
    let state = base["load_cases"][0]["analysis_state"].clone();
    let refuse = |object: &str, id: &str, path: &str, before: &Value, after: &Value, code: &str| {
        let op = intent(
            object,
            id,
            path,
            &canonical_json(before),
            &serde_json::to_string(after).unwrap(),
        );
        refused(&base, &op, code);
    };

    // Duplicate IDs.
    let mut v = configurations.clone();
    v.as_array_mut().unwrap().push(configurations[0].clone());
    refuse(
        "Model",
        &project,
        "reference_configurations",
        &configurations,
        &v,
        "OP-LOAD-STATE-DUPLICATE-ID",
    );
    let mut v = laws.clone();
    let mut second = laws[0].clone();
    second["data"]["coefficient"]["value"] = json!(3.0e-5);
    v.as_array_mut().unwrap().push(second);
    refuse(
        "Material",
        "material:shared",
        "expansion_laws",
        &laws,
        &v,
        "OP-LOAD-STATE-DUPLICATE-ID",
    );

    // Dangling references.
    let mut v = configurations.clone();
    v[0]["member_references"][1]["pipe_ref"] = json!("pipe:invented-missing");
    refuse(
        "Model",
        &project,
        "reference_configurations",
        &configurations,
        &v,
        "OP-LOAD-STATE-REFERENCE-UNRESOLVED",
    );
    let dangling: [(&str, Value); 8] = [
        (
            "/reference_configuration_ref",
            json!("reference:invented-missing"),
        ),
        ("/element_states/0/pipe_ref", json!("pipe:invented-missing")),
        (
            "/element_states/0/material_selection/material_ref",
            json!("material:invented-missing"),
        ),
        (
            "/element_states/0/material_selection/point_ref",
            json!("point:invented-missing"),
        ),
        (
            "/element_states/0/thermal_state/expansion_law_ref",
            json!("law:invented-missing"),
        ),
        (
            "/support_states/1/support_ref",
            json!("support:invented-missing"),
        ),
        ("/load_sources/0/source_ref", json!("load:invented-missing")),
        // A primitive stored in another case does not resolve for this case.
        (
            "/load_sources/0/source_ref",
            json!("load:transverse:case:hot"),
        ),
    ];
    for (pointer, value) in dangling {
        let mut v = state.clone();
        *v.pointer_mut(pointer).unwrap() = value;
        refuse(
            "Load",
            "case:cold",
            "analysis_state",
            &state,
            &v,
            "OP-LOAD-STATE-REFERENCE-UNRESOLVED",
        );
    }
    // A reference configuration must exist in the model before a case can
    // name it.
    let unconfigured = without(base.clone(), "", "reference_configurations");
    let op = intent(
        "Load",
        "case:cold",
        "analysis_state",
        &canonical_json(&state),
        &serde_json::to_string(&state).unwrap(),
    );
    refused(&unconfigured, &op, "OP-LOAD-STATE-REFERENCE-UNRESOLVED");
    // A law on the element's material, not merely somewhere in the model.
    let mut other = base.clone();
    let mut second_material = other["materials"][0].clone();
    second_material["id"] = json!("material:invented-second");
    second_material["expansion_laws"][0]["id"] = json!("law:invented-second");
    other["materials"]
        .as_array_mut()
        .unwrap()
        .push(second_material);
    let mut v = state.clone();
    v["element_states"][0]["thermal_state"]["expansion_law_ref"] = json!("law:invented-second");
    let op = intent(
        "Load",
        "case:cold",
        "analysis_state",
        &canonical_json(&state),
        &serde_json::to_string(&v).unwrap(),
    );
    refused(&other, &op, "OP-LOAD-STATE-REFERENCE-UNRESOLVED");
}

#[test]
fn load_state_operations_apply_as_one_atomic_batch_or_not_at_all() {
    let target = connected();
    let project = target["project"]["id"].as_str().unwrap().to_string();
    // Start from the witness with all three records absent, then author them.
    let mut base = without(target.clone(), "", "reference_configurations");
    base = without(base, "/materials/0", "expansion_laws");
    for index in 0..2 {
        base = without(base, &format!("/load_cases/{index}"), "analysis_state");
    }
    let mut operations = vec![
        intent(
            "Model",
            &project,
            "reference_configurations",
            "not_present",
            &serde_json::to_string(&target["reference_configurations"]).unwrap(),
        ),
        intent(
            "Material",
            "material:shared",
            "expansion_laws",
            "not_present",
            &serde_json::to_string(&target["materials"][0]["expansion_laws"]).unwrap(),
        ),
    ];
    for case in target["load_cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        operations.push(intent(
            "Load",
            id,
            "analysis_state",
            "not_present",
            &serde_json::to_string(&case["analysis_state"]).unwrap(),
        ));
    }
    let batch = json!({"batch_id":"batch:load-state","operations":operations});
    let claim = hash(&base);
    let reviewed = validate_operation_batch(&base, &batch, Some(&claim));
    assert_eq!(
        reviewed["validation"]["batch_validation_status"], "passed",
        "{reviewed:#}"
    );
    assert!(reviewed.get("applied_model").is_none_or(Value::is_null));
    let applied = apply_operation_batch(&base, &batch, Some(&claim));
    assert_eq!(
        applied["validation"]["application_status"], "applied_to_session_model",
        "{applied:#}"
    );
    assert_eq!(
        canonical_json(&applied["applied_model"]),
        canonical_json(&target)
    );
    // Order matters: a case cannot name a configuration authored after it.
    let mut reversed = batch.clone();
    reversed["operations"].as_array_mut().unwrap().reverse();
    let refused = apply_operation_batch(&base, &reversed, Some(&claim));
    assert!(
        refused.get("applied_model").is_none_or(Value::is_null),
        "{refused:#}"
    );
    assert_ne!(
        refused["validation"]["application_status"],
        "applied_to_session_model"
    );
    assert!(refused
        .to_string()
        .contains("OP-LOAD-STATE-REFERENCE-UNRESOLVED"));
}

// ---------------------------------------------------------------------------
// Addendum 1: inbound-reference refusals.
// ---------------------------------------------------------------------------

/// A structured delete intent in the shape the delete operations accept.
#[allow(clippy::too_many_arguments)]
fn delete_intent(
    object: &str,
    id: &str,
    kind: &str,
    path: &str,
    before: &str,
    after: &str,
    unit: &str,
    dimension: &str,
) -> Value {
    let mut op = intent(object, id, path, before, after);
    op["operation_kind"] = json!("delete");
    op["operation_id"] = json!(format!("op:load-state-delete:{kind}:{id}:{path}"));
    op["change"]["change_id"] = json!(format!("change:load-state-delete:{kind}:{id}:{path}"));
    op["change"]["change_kind"] = json!(kind);
    op["change"]["unit"] = json!(unit);
    op["change"]["dimension"] = json!(dimension);
    op
}
/// The refusal's affected refs, for asserting the referring case and path.
fn affected(model: &Value, op: &Value, code: &str) -> Vec<String> {
    refused(model, op, code);
    let outcome = apply_operation(model, op, Some(&hash(model)));
    let diagnostic = outcome.diagnostics.iter().find(|d| d.code == code).unwrap();
    let mut refs = diagnostic.affected_refs.clone();
    refs.push(diagnostic.message.clone());
    refs
}
fn names(refs: &[String], path: &str) -> bool {
    refs.iter().any(|r| r.contains(path))
}
fn set_intent(
    model: &Value,
    object: &str,
    id: &str,
    owner: &str,
    path: &str,
    after: &str,
) -> Value {
    let before = display(model.pointer(owner).unwrap().get(path));
    intent(object, id, path, &before, after)
}

#[test]
fn replacing_or_removing_owners_cannot_orphan_case_references() {
    let base = connected();
    let project = base["project"]["id"].as_str().unwrap().to_string();
    let configurations = base["reference_configurations"].clone();
    let laws = base["materials"][0]["expansion_laws"].clone();
    let code = "OP-LOAD-STATE-INBOUND-REFERENCE";

    // (a) Renaming or removing the configuration both cases name.
    let mut renamed = configurations.clone();
    renamed[0]["id"] = json!("reference:invented-renamed");
    for after in [
        serde_json::to_string(&renamed).unwrap(),
        "not_present".into(),
    ] {
        let op = set_intent(
            &base,
            "Model",
            &project,
            "",
            "reference_configurations",
            &after,
        );
        let refs = affected(&base, &op, code);
        for case in ["case:cold", "case:hot"] {
            assert!(
                names(
                    &refs,
                    &format!("{case}.analysis_state.reference_configuration_ref")
                ),
                "{refs:?}"
            );
        }
    }
    // The earlier absent-key scenario: once a configuration makes the cases'
    // references resolve, its inverse removal is refused.
    let absent = without(base.clone(), "", "reference_configurations");
    let forward = set_intent(
        &absent,
        "Model",
        &project,
        "",
        "reference_configurations",
        &serde_json::to_string(&configurations).unwrap(),
    );
    let authored = apply(&absent, &forward);
    let undo = set_intent(
        &authored,
        "Model",
        &project,
        "",
        "reference_configurations",
        "not_present",
    );
    refused(&authored, &undo, code);
    // Keeping the named configuration and adding another is admitted.
    let mut extended = configurations.clone();
    let mut extra = configurations[0].clone();
    extra["id"] = json!("reference:invented-extra");
    extended.as_array_mut().unwrap().push(extra);
    apply(
        &base,
        &set_intent(
            &base,
            "Model",
            &project,
            "",
            "reference_configurations",
            &serde_json::to_string(&extended).unwrap(),
        ),
    );
    // A reference that is already unresolved is not orphaned by this edit.
    let mut stale = base.clone();
    stale["load_cases"][0]["analysis_state"]["reference_configuration_ref"] =
        json!("reference:invented-gone");
    apply(
        &stale,
        &set_intent(
            &stale,
            "Model",
            &project,
            "",
            "reference_configurations",
            &serde_json::to_string(&extended).unwrap(),
        ),
    );

    // (b) Renaming or removing the law both cases' elements name.
    let mut renamed = laws.clone();
    renamed[0]["id"] = json!("law:invented-renamed");
    for after in [
        serde_json::to_string(&renamed).unwrap(),
        "not_present".into(),
    ] {
        let op = set_intent(
            &base,
            "Material",
            "material:shared",
            "/materials/0",
            "expansion_laws",
            &after,
        );
        let refs = affected(&base, &op, code);
        for case in ["case:cold", "case:hot"] {
            for index in 0..2 {
                assert!(names(&refs, &format!("{case}.analysis_state.element_states.{index}.thermal_state.expansion_law_ref")), "{refs:?}");
            }
        }
    }
    // Only elements that select this material are counted: a second material
    // that happens to own a law with the same ID can drop it.
    let mut second = base.clone();
    let mut material = base["materials"][0].clone();
    material["id"] = json!("material:invented-second");
    second["materials"].as_array_mut().unwrap().push(material);
    apply(
        &second,
        &set_intent(
            &second,
            "Material",
            "material:invented-second",
            "/materials/1",
            "expansion_laws",
            "not_present",
        ),
    );
}

/// The connected witness with invented labels, which the delete operations
/// require for their before-value display.
fn labelled() -> Value {
    let mut model = connected();
    for pipe in model["pipe_segments"].as_array_mut().unwrap() {
        pipe["label"] = json!(format!("Invented {}", pipe["id"].as_str().unwrap()));
    }
    for support in model["supports"].as_array_mut().unwrap() {
        support["label"] = json!(format!("Invented {}", support["id"].as_str().unwrap()));
    }
    let mut material = model["materials"][0].clone();
    material["id"] = json!("material:invented-second");
    model["materials"].as_array_mut().unwrap().push(material);
    model
}
fn delete_pipe(model: &Value, id: &str) -> Value {
    let pipe = model["pipe_segments"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["id"] == id)
        .unwrap();
    let before = format!(
        "{}; {}->{}; material={}",
        pipe["label"].as_str().unwrap(),
        pipe["from"].as_str().unwrap(),
        pipe["to"].as_str().unwrap(),
        pipe["material"].as_str().unwrap()
    );
    delete_intent(
        "Element",
        id,
        "delete_pipe_run",
        "pipe_segments",
        &before,
        "not_present",
        "none",
        "dimensionless",
    )
}
fn delete_support(model: &Value, id: &str) -> Value {
    let support = model["supports"]
        .as_array()
        .unwrap()
        .iter()
        .find(|s| s["id"] == id)
        .unwrap();
    delete_intent(
        "Support",
        id,
        "delete_support",
        "supports",
        support["label"].as_str().unwrap(),
        "not_present",
        "none",
        "dimensionless",
    )
}
fn delete_material(model: &Value, id: &str) -> Value {
    let material = model["materials"]
        .as_array()
        .unwrap()
        .iter()
        .find(|m| m["id"] == id)
        .unwrap();
    delete_intent(
        "Material",
        id,
        "delete_material",
        "materials",
        &canonical_json(material),
        "deleted",
        "none",
        "dimensionless",
    )
}
fn delete_primitive(model: &Value, case: &str, index: usize) -> Value {
    let load = &model["load_cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["id"] == case)
        .unwrap()["primitive_loads"][index];
    let value = load["magnitude"]["value"].as_f64().unwrap();
    assert_eq!(value.fract(), 0.0);
    let before = format!(
        "{}; {}; {}:{}; {}; {} N; force",
        load["id"].as_str().unwrap(),
        load["category"].as_str().unwrap(),
        load["target"]["type"].as_str().unwrap(),
        load["target"]["node"].as_str().unwrap(),
        load["direction"].as_str().unwrap(),
        value as i64
    );
    assert_eq!(load["magnitude"]["unit"], "N");
    delete_intent(
        "Load",
        case,
        "delete_primitive_load",
        &format!("primitive_loads.{index}"),
        &before,
        "not_present",
        "N",
        "force",
    )
}

#[test]
fn delete_operations_refuse_targets_named_by_load_state_records() {
    let model = labelled();
    // Pipe: named by reference-configuration members and by element states.
    let refs = affected(
        &model,
        &delete_pipe(&model, "pipe:first"),
        "OP-PIPE-DELETE-REFERENCED",
    );
    assert!(
        names(
            &refs,
            "reference_configurations.reference:installed.member_references.0.pipe_ref"
        ),
        "{refs:?}"
    );
    for case in ["case:cold", "case:hot"] {
        assert!(
            names(
                &refs,
                &format!("{case}.analysis_state.element_states.0.pipe_ref")
            ),
            "{refs:?}"
        );
    }
    // Either record alone is enough.
    let only_members = without_states(&model);
    let refs = affected(
        &only_members,
        &delete_pipe(&only_members, "pipe:second"),
        "OP-PIPE-DELETE-REFERENCED",
    );
    assert!(
        names(
            &refs,
            "reference_configurations.reference:installed.member_references.1.pipe_ref"
        ),
        "{refs:?}"
    );
    let only_states = without(model.clone(), "", "reference_configurations");
    let refs = affected(
        &only_states,
        &delete_pipe(&only_states, "pipe:second"),
        "OP-PIPE-DELETE-REFERENCED",
    );
    assert!(
        names(&refs, "case:hot.analysis_state.element_states.1.pipe_ref"),
        "{refs:?}"
    );

    // Support: named by support states.
    let refs = affected(
        &model,
        &delete_support(&model, "support:far"),
        "OP-SUPPORT-DELETE-REFERENCED",
    );
    assert!(
        names(
            &refs,
            "case:cold.analysis_state.support_states.1.support_ref"
        ),
        "{refs:?}"
    );

    // Material: an element's material_ref is caught by the existing generic
    // reference scan (no code change was needed for delete_material).
    let mut selected = model.clone();
    selected["load_cases"][0]["analysis_state"]["element_states"][0]["material_selection"]
        ["material_ref"] = json!("material:invented-second");
    let refs = affected(
        &selected,
        &delete_material(&selected, "material:invented-second"),
        "OP-ENTITY-DELETE-REFERENCED",
    );
    assert!(
        names(
            &refs,
            "load_cases[0].analysis_state.element_states[0].material_selection.material_ref"
        ),
        "{refs:?}"
    );

    // Primitive load: named by its own case's load sources.
    for (case, index) in [("case:cold", 0), ("case:hot", 0)] {
        let refs = affected(
            &model,
            &delete_primitive(&model, case, index),
            "OP-LOAD-STATE-INBOUND-REFERENCE",
        );
        assert!(
            names(
                &refs,
                &format!("{case}.analysis_state.load_sources.0.source_ref")
            ),
            "{refs:?}"
        );
    }
    // The stored primitive that no load source names can be deleted.
    let applied = apply(&model, &delete_primitive(&model, "case:cold", 1));
    assert_eq!(
        applied["load_cases"][0]["primitive_loads"]
            .as_array()
            .unwrap()
            .len(),
        1
    );

    // Controls: without the load/reference-state records every one of these
    // deletes is admitted.
    let mut bare = without(without_states(&model), "", "reference_configurations");
    for op in [
        delete_pipe(&bare, "pipe:first"),
        delete_support(&bare, "support:far"),
        delete_material(&bare, "material:invented-second"),
        delete_primitive(&bare, "case:cold", 0),
        delete_primitive(&bare, "case:hot", 0),
    ] {
        bare = apply(&bare, &op);
    }
}

#[test]
fn atomic_batch_removes_the_states_before_their_configuration_and_passes() {
    let model = labelled();
    let project = model["project"]["id"].as_str().unwrap().to_string();
    let display_of = |pointer: &str, key: &str| display(model.pointer(pointer).unwrap().get(key));
    let mut operations = vec![
        intent(
            "Load",
            "case:cold",
            "analysis_state",
            &display_of("/load_cases/0", "analysis_state"),
            "not_present",
        ),
        intent(
            "Load",
            "case:hot",
            "analysis_state",
            &display_of("/load_cases/1", "analysis_state"),
            "not_present",
        ),
        intent(
            "Material",
            "material:shared",
            "expansion_laws",
            &display_of("/materials/0", "expansion_laws"),
            "not_present",
        ),
        intent(
            "Model",
            &project,
            "reference_configurations",
            &display_of("", "reference_configurations"),
            "not_present",
        ),
    ];
    // Once no load source names it, the case's primitive can go too.
    operations.push(delete_primitive(&model, "case:cold", 0));
    let batch = json!({"batch_id":"batch:load-state-removal","operations":operations});
    let claim = hash(&model);
    let reviewed = validate_operation_batch(&model, &batch, Some(&claim));
    assert_eq!(
        reviewed["validation"]["batch_validation_status"], "passed",
        "{reviewed:#}"
    );
    let applied = apply_operation_batch(&model, &batch, Some(&claim));
    assert_eq!(
        applied["validation"]["application_status"], "applied_to_session_model",
        "{applied:#}"
    );
    let mut expected = without(without_states(&model), "", "reference_configurations");
    expected = without(expected, "/materials/0", "expansion_laws");
    expected["load_cases"][0]["primitive_loads"]
        .as_array_mut()
        .unwrap()
        .remove(0);
    assert_same_bytes(&applied["applied_model"], &expected);

    // The configuration first, while the states still name it: refused whole.
    let mut early = batch.clone();
    early["operations"].as_array_mut().unwrap().swap(0, 3);
    let refused = apply_operation_batch(&model, &early, Some(&claim));
    assert!(
        refused.get("applied_model").is_none_or(Value::is_null),
        "{refused:#}"
    );
    assert!(refused
        .to_string()
        .contains("OP-LOAD-STATE-INBOUND-REFERENCE"));
    // The primitive first, while its case still names it: refused whole.
    let mut early = batch.clone();
    early["operations"].as_array_mut().unwrap().rotate_right(1);
    let refused = apply_operation_batch(&model, &early, Some(&claim));
    assert!(
        refused.get("applied_model").is_none_or(Value::is_null),
        "{refused:#}"
    );
    assert!(refused
        .to_string()
        .contains("OP-LOAD-STATE-INBOUND-REFERENCE"));
}
