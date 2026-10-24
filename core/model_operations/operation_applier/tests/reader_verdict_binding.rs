//! R2 (b7-plan Unit R): the wasm engine's base-reader entry points, run natively.
//! They parse the JSON text and return `result_export::reader_verdict`'s outcome;
//! the desktop's wasm build compiles these same functions (`wasm_api`).
use open_pipe_stress_operation_applier::{
    reader_dispatch_json, reader_row_semantics_json, reader_transport_json, reader_validate_json,
};
use open_pipe_stress_product_physics::result_export::reader_verdict as rv;
use serde_json::{json, Value};

// B4/D1 (b): the composite statement is the source route here; B7b: the current
// pressure-source-1 (0.7.0/v3) statement, and pressure-2 with the
// load/reference-state group as the pressure reader's input.
const SOURCE: &str = include_str!("../../../../fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json");
const REQUEST: &str = include_str!("../../../../fixtures/product_preview/pressure_source/physics_source/n05.request.json");
const LOAD_REFERENCE: &str = include_str!("../../../../fixtures/product_preview/pressure/load_reference/connected-sparse_interactive.raw.json");
const LOAD_REFERENCE_REQUEST: &str = include_str!("../../../../fixtures/product_preview/pressure/load_reference/connected.request.json");

fn outcome(text: String) -> Value {
    serde_json::from_str(&text).unwrap()
}

#[test]
fn the_entry_points_return_the_verdicts() {
    let source: Value = serde_json::from_str(SOURCE).unwrap();
    let request: Value = serde_json::from_str(REQUEST).unwrap();
    let invocation = json!({"request": request, "solver_mode": "sparse_interactive"});
    let (model_text, invocation_text) = (request["model"].to_string(), invocation.to_string());
    let bound = outcome(reader_dispatch_json(SOURCE, Some(&model_text), Some(&invocation_text)).unwrap());
    assert_eq!(bound, rv::dispatch(&source, Some(&request["model"]), Some(&invocation)));
    assert_eq!((bound["route"].as_str(), bound["standing"]["eligible"].as_bool()), (Some("pressure_source"), Some(true)));
    // An absent model or invocation, or the text `null`, reads as absent.
    let unbound = outcome(reader_dispatch_json(SOURCE, None, None).unwrap());
    assert_eq!(unbound, rv::dispatch(&source, None, None));
    assert_eq!(outcome(reader_dispatch_json(SOURCE, Some("null"), Some("null")).unwrap()), unbound);
    assert_eq!(unbound["standing"]["findings"], json!([rv::SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED]));
    let validated = outcome(reader_validate_json("pressure-source", SOURCE, None, Some(&invocation_text)).unwrap());
    assert_eq!(validated, json!({"ok": true, "eligible": true, "findings": []}));
    assert_eq!(outcome(reader_validate_json("source-recovery", SOURCE, None, None).unwrap()), json!({"ok": true}));
    assert_eq!(outcome(reader_transport_json("pressure-source", SOURCE).unwrap()), json!({"ok": true}));
    let mut bad = source.clone();
    bad["source_block_recovery"]["receipt_sha256"] = json!("0".repeat(64));
    assert_eq!(
        outcome(reader_transport_json("pressure-source", &bad.to_string()).unwrap()),
        rv::transport("pressure-source", &bad)
    );
    assert_eq!(outcome(reader_transport_json("pressure-source", &bad.to_string()).unwrap())["ok"], false);
    // The retired source-blocks readers, and (B7b) the retired exact v2 readers,
    // have no entry.
    for retired in ["source-blocks", "physics-source", "physics", "load-reference", "load-reference-source"] {
        assert_eq!(outcome(reader_transport_json(retired, SOURCE).unwrap())["error"]["code"], "READER_UNKNOWN", "{retired}");
        assert_eq!(outcome(reader_validate_json(retired, SOURCE, None, None).unwrap())["error"]["code"], "READER_UNKNOWN", "{retired}");
    }
    // The model reaches the pressure reader (G2); B7b: on the received pressure-2
    // statement its case-coverage refusal is the model check's own code.
    let lr_model: Value = serde_json::from_str(LOAD_REFERENCE_REQUEST).unwrap();
    let mut fewer = lr_model["model"].clone();
    fewer["load_cases"].as_array_mut().unwrap().remove(0);
    let lr: Value = serde_json::from_str(LOAD_REFERENCE).unwrap();
    let refused = outcome(reader_validate_json("pressure", LOAD_REFERENCE, Some(&fewer.to_string()), None).unwrap());
    assert_eq!(refused, rv::validate("pressure", &lr, Some(&fewer), None));
    assert_eq!(refused["error"]["code"], "SOURCE_PHYSICS_MODEL_CASE_COVERAGE");
    assert_eq!(outcome(reader_validate_json("pressure", LOAD_REFERENCE, None, None).unwrap()), json!({"ok": true}));
    // pressure-2 has no transport reader (refused by name).
    let transport = outcome(reader_transport_json("pressure", LOAD_REFERENCE).unwrap());
    assert_eq!(transport, rv::transport("pressure", &lr));
    assert_eq!(transport["ok"], false);
    assert_eq!(transport["error"]["code"], rv::PRESSURE_TRANSPORT_UNSUPPORTED);
    assert_eq!(
        outcome(reader_transport_json("invented", LOAD_REFERENCE).unwrap())["error"],
        json!({"code": "READER_UNKNOWN", "detail": "READER_UNKNOWN: invented"})
    );
    let row = source["results"][0].to_string();
    assert_eq!(
        outcome(reader_row_semantics_json(&row, Some(SOURCE)).unwrap()),
        rv::row_semantics(&source["results"][0], Some(&source))
    );
    assert_eq!(
        outcome(reader_row_semantics_json(&row, Some("null")).unwrap()),
        rv::row_semantics(&source["results"][0], None)
    );
}

#[test]
fn text_that_is_not_json_is_an_input_error() {
    let invalid = |error: String, label: &str| {
        assert!(error.starts_with(&format!("WASM-ENGINE-INPUT-JSON-INVALID: {label} is not valid JSON")), "{error}")
    };
    invalid(reader_dispatch_json("{", None, None).unwrap_err(), "source");
    invalid(reader_dispatch_json("{}", Some("[1,"), None).unwrap_err(), "model");
    invalid(reader_dispatch_json("{}", None, Some("[1,")).unwrap_err(), "invocation");
    invalid(reader_validate_json("pressure", "nul", None, None).unwrap_err(), "source");
    invalid(reader_transport_json("pressure", "nul").unwrap_err(), "source");
    invalid(reader_row_semantics_json("{", None).unwrap_err(), "row");
}

/// E17 through the wasm engine's entry point, run natively: the governed G7
/// magnitude controls on an invented support-only preview statement (the same
/// shape as the desktop's `previewMagnitudeGuard.test.ts`), direct and combined.
/// Representation controls only: no solve, native witness or producer claim.
#[test]
fn e17_magnitude_controls_through_the_reader_binding() {
    let table: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/results/semantic_contract_v0_3_preview_physics_2.json"
    ))
    .unwrap();
    let controls: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/results/preview_magnitude_controls.json"
    ))
    .unwrap();
    let (force, moment) = ("support_reaction_force_magnitude_v2", "support_reaction_moment_magnitude_v2");
    let (cid, support) = ("invented:e17:case", "invented:e17:support");
    let identity = [cid, support].map(|p| format!("{}:{p}", p.len())).join(":");
    let basis = json!({"ref_type": "load_case", "ref_id": cid});
    let sign = "support-on-pipe; positive global force and right-hand couple about attached node; force and moment norms remain separate";
    let statement = || {
        let components = ["Fx", "Fy", "Fz", "Mx", "My", "Mz", "force_magnitude", "moment_magnitude"];
        let results: Vec<Value> = components.iter().enumerate().map(|(i, c)| json!({
            "id": format!("result:support-action:{identity}:{c}"), "entity_ref": support, "basis_ref": basis, "value": 0.0,
            "kind": if i < 6 { "support_reaction_component_v2" } else if i == 6 { force } else { moment },
            "unit": if i < 3 || i == 6 { "N" } else { "N*m" },
            "metadata": {"component": c, "coordinate_system": "global", "location": "node",
                "basis": "recovered_from_assembled_support_law", "sign_convention": sign}})).collect();
        json!({"schema_version": "0.2.0", "document_kind": "openpipestress.product_preview.mechanics_result",
            "run_id": "invented:e17:run", "model_ref": "invented:e17:model",
            "producer": {"component_name": "open_pipe_stress_product_physics", "component_version": "0.2.0",
                "semantic_contract_id": table["semantic_contract_id"]},
            "formulation_basis": {"profile_id": table["formulation_profile_id"], "limitations": table["supported_profile_limitations"]},
            "numerical_quality": {"value_representation": "finite_binary64", "publication_quantization": "none",
                "integrity_policy": "M03-INTEGRITY-v1", "status": "checks_passed",
                "cases": [{"basis_ref": basis, "structural_status": "passive_model_basis", "solve_quality": "checks_passed",
                    "model_matrix_fidelity": "represented_equations_retained", "accuracy_evidence": "not_claimed", "evidence_refs": []}]},
            "status": {"mechanics": "MECHANICS_SOLVED", "rule_check": "RULE_INPUTS_INCOMPLETE", "professional_acceptance": "NOT_PROVIDED"},
            "results": results,
            "summary": {"node_count": 1, "segment_count": 0, "support_count": 1, "load_case_count": 1,
                "component_stress_modifier_count": 0, "max_displacement": null, "max_open_formula_stress": null},
            "diagnostics": [],
            "contract_evidence": {"preview_cases": [{"load_case_id": cid, "pipe_stress_extrema": [],
                "stress_maximum_coverage": {"complete": true, "unavailable_pipe_ids": [], "outside_domain_pipe_ids": []},
                "support_attribution": {"attributed_support_ids": [support], "withheld": []}, "intensified_measures": []}],
                "combination_gates": []},
            "professional_boundary": {"human_review_required": true}, "accepted_model_state_mutated": false})
    };
    let read = |s: &Value| outcome(reader_validate_json("preview-physics", &s.to_string(), None, None).unwrap());
    assert_eq!(read(&statement()), json!({"ok": true}), "the untouched statement reaches G7 and passes");
    let bits = |v: &Value| f64::from_bits(u64::from_str_radix(v.as_str().unwrap(), 16).unwrap());
    let mut checked = 0;
    for control in controls["controls"].as_array().unwrap() {
        let values: Vec<f64> = (0..3).map(|i| bits(&control["component_bits"][i])).collect();
        let magnitude = bits(&control["magnitude_bits"]);
        let admitted = control["admitted"].as_bool().unwrap();
        for (target, names) in [(force, ["Fx", "Fy", "Fz"]), (moment, ["Mx", "My", "Mz"])] {
            // Direct: the support's own rows.
            let mut s = statement();
            for row in s["results"].as_array_mut().unwrap() {
                if let Some(axis) = names.iter().position(|n| row["metadata"]["component"] == *n) {
                    row["value"] = json!(values[axis]);
                }
                if row["kind"] == target {
                    row["value"] = json!(magnitude);
                }
            }
            let got = read(&s);
            assert_eq!(got["ok"].as_bool(), Some(admitted), "{} direct {target}: {got}", control["id"]);
            if !admitted {
                assert!(got.to_string().contains("SOURCE_PREVIEW_PHYSICS_SUPPORT_MAGNITUDE"), "{} {target}: {got}", control["id"]);
            }
            checked += 1;
        }
        for target in [force, moment, "displacement_magnitude"] {
            // Combined: an explicit combination's components and magnitude.
            let mut s = statement();
            let combination = "invented:e17:combination";
            s["contract_evidence"]["combination_gates"] = json!([{"combination_id": combination, "withheld": false, "reason": null}]);
            let (entity, cbasis) = ("invented:e17:combined", json!({"ref_type": "combination", "ref_id": combination}));
            let unit = if target == force { "N" } else if target == moment { "N*m" } else { "mm" };
            let names = if target == force { ["Fx", "Fy", "Fz"] } else if target == moment { ["Mx", "My", "Mz"] }
                else { ["nodal_displacement_x", "nodal_displacement_y", "nodal_displacement_z"] };
            let meta = |c: &str| json!({"component": c, "coordinate_system": "global", "location": "node",
                "basis": "explicit_user_linear_combination",
                "sign_convention": "positive value follows explicit user linear combination of matching source result sign conventions"});
            let rows = s["results"].as_array_mut().unwrap();
            for (axis, c) in names.iter().enumerate() {
                let kind = if target == "displacement_magnitude" {
                    ["global_nodal_displacement_x", "global_nodal_displacement_y", "global_nodal_displacement_z"][axis]
                } else { "support_reaction_component_v2" };
                rows.push(json!({"id": format!("result:e17:{target}:{c}"), "entity_ref": entity, "basis_ref": cbasis,
                    "value": values[axis], "kind": kind, "unit": unit, "metadata": meta(c), "source_result_refs": []}));
            }
            let mut row = json!({"id": format!("result:e17:{target}:magnitude"), "kind": target, "entity_ref": entity,
                "basis_ref": cbasis, "value": magnitude, "unit": unit, "source_result_refs": []});
            if target != "displacement_magnitude" {
                row["metadata"] = meta(if target == force { "force_magnitude" } else { "moment_magnitude" });
            }
            rows.push(row);
            let got = read(&s);
            assert_eq!(got["ok"].as_bool(), Some(admitted), "{} combined {target}: {got}", control["id"]);
            if !admitted {
                assert!(got.to_string().contains("SOURCE_PREVIEW_PHYSICS_COMBINATION_MAGNITUDE"), "{} {target}: {got}", control["id"]);
            }
            checked += 1;
        }
    }
    assert_eq!(checked, 7 * 5);
}
