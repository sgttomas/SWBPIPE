//! Public-route regression for conflicting stiffness and explicit family tokens.
use open_pipe_stress_operation_applier::{
    apply_operation, apply_operation_batch, canonical_json, sha256_hex, validate_operation,
    validate_operation_batch, OperationOutcome,
};
use open_pipe_stress_units::{canonical_unit, convert_for_dimension, unit_by_symbol, Dimension};
use serde_json::{json, Value};

fn model() -> Value {
    json!({"schema_version":"0.1.0","document_kind":"openpipestress.product_preview.model","project":{"id":"project:test","units":{"length":"m","force":"N","angle":"rad","pressure":"Pa","temperature":"K"}},"nodes":[{"id":"node:n","label":"Node","position":{"x":0,"y":0,"z":0},"provenance":"user"}],"supports":[],"materials":[],"sections":[],"pipe_segments":[],"components":[],"load_cases":[],"combinations":[]})
}
fn q(value: f64, unit: &str) -> Value {
    json!({"value":value,"unit":unit})
}
fn stiffness(dof: &str, value: f64, unit: &str) -> Value {
    json!({"dof":dof,"value":q(value,unit)})
}
fn support() -> Value {
    json!({"id":"support:s","label":"Support","node":"node:n","restraints":["UY"],"provenance":"user"})
}
fn hanger() -> Value {
    json!({"hanger_type":"variable_spring_hanger","source_reference":"user","installed_load":q(10.,"N"),"cold_load":q(11.,"N"),"hot_load":q(12.,"N"),"movement_limit":q(1.,"in")})
}
fn configured() -> Value {
    let mut s = support();
    s["family"] = json!("variable_spring_hanger");
    s["hanger"] = hanger();
    s["stiffness"] = stiffness("UY", 20., "N/m");
    s
}
fn projection(s: &Value) -> Value {
    let keys = [
        "family",
        "restraints",
        "stiffness",
        "hanger",
        "nonlinear",
        "provenance",
    ];
    Value::Object(
        keys.into_iter()
            .filter_map(|k| s.get(k).map(|v| (k.into(), v.clone())))
            .collect(),
    )
}
fn operation(payload: &Value, before: &str, create: bool, id: &str) -> Value {
    json!({"operation_id":format!("operation:{id}"),"operation_kind":if create{"create"}else{"modify"},"operation_status":"proposed","author_type":"user","target":{"object_type":"Support","ref":"support:s"},"change":{"change_id":format!("change:{id}"),"change_kind":if create{"create_support"}else{"update_support"},"field_label":"Support","field_path":if create{"supports"}else{"configuration"},"before":before,"after":payload.to_string(),"unit":"none","dimension":"dimensionless","source_note":"Explicit fixture"},"validation":{"schema_validation":"not_run","constraint_validation":"not_run","unit_validation":"not_run","diff_preview_status":"not_generated","application_status":"not_applied"},"audit_boundary":{"mutation_route":"structured_operations_only","direct_model_mutation_allowed":false,"requires_user_acceptance":true,"mutates_accepted_model_state":false},"professional_boundary":{"human_review_required":true,"software_makes_compliance_claim":false,"software_makes_certification_claim":false,"software_makes_sealing_claim":false,"software_makes_approval_claim":false,"software_makes_authentication_claim":false},"rationale":"Explicit fixture"})
}
fn claim(m: &Value) -> Value {
    json!({"algorithm":"sha256","canonicalization":"rfc8785_jcs","payload_scope":"model_payload","value":format!("sha256:{}",sha256_hex(&canonical_json(m)))})
}
fn routes(after: &Value) -> [(Value, Value); 2] {
    let empty = model();
    let create = operation(after, "not_present", true, "create");
    let mut current = model();
    current["supports"] = json!([support()]);
    let update = operation(
        &projection(after),
        &canonical_json(&projection(&support())),
        false,
        "update",
    );
    [(empty, create), (current, update)]
}
fn assert_blocked(out: OperationOutcome, needle: &str) {
    assert!(out.applied_model.is_none());
    assert!(out.diff_preview.is_empty());
    assert!(
        out.diagnostics
            .iter()
            .any(|d| d.severity == "blocking" && d.message.contains(needle)),
        "Unexpected diagnostics: {:?}",
        out.diagnostics
    );
}
fn rejected(after: &Value, needle: &str) {
    for (m, op) in routes(after) {
        let original = m.clone();
        let hash = claim(&m);
        assert_blocked(validate_operation(&m, &op, Some(&hash)), needle);
        assert_blocked(apply_operation(&m, &op, Some(&hash)), needle);
        assert_eq!(m, original);
    }
}
fn accepted(after: &Value) {
    for (m, op) in routes(after) {
        let original = m.clone();
        let hash = claim(&m);
        let validated = validate_operation(&m, &op, Some(&hash));
        let out = apply_operation(&m, &op, Some(&hash));
        assert!(out.applied_model.is_some(), "{:?}", out.diagnostics);
        assert_eq!(
            serde_json::to_value(&validated.diff_preview).unwrap(),
            serde_json::to_value(&out.diff_preview).unwrap()
        );
        assert!(validated.applied_model.is_none());
        assert_eq!(
            canonical_json(&out.applied_model.unwrap()["supports"][0]),
            canonical_json(after)
        );
        assert_eq!(m, original);
    }
}
#[test]
fn public_create_and_replace_reject_conflicting_stiffness_dof_or_exact_normalized_value() {
    let mut s = configured();
    s["hanger"]["stiffness"] = stiffness("UY", 100., "N/m");
    rejected(&s, "same DOF and exactly equal normalized value");
    s["hanger"]["stiffness"] = stiffness("UX", 20., "N/m");
    rejected(&s, "same DOF and exactly equal normalized value");
    s["hanger"]["stiffness"] = stiffness("UY", f64::from_bits(20_f64.to_bits() + 1), "N/m");
    rejected(&s, "same DOF and exactly equal normalized value");
    s["stiffness"] = stiffness("RX", 20., "N*m/rad");
    s["hanger"]["stiffness"] = stiffness("RY", 20., "N*m/rad");
    rejected(&s, "same DOF and exactly equal normalized value");
}
#[test]
fn equal_duplicates_and_single_source_us_stiffness_preserve_entered_json() {
    for (dof, alias, unit) in [("UY", "uy", "N/m"), ("RX", "Rx", "N*m/rad")] {
        let mut s = configured();
        s["stiffness"] = stiffness(dof, 20., unit);
        s["hanger"]["stiffness"] = stiffness(alias, 20., unit);
        accepted(&s);
    }
    for unit in ["lbf/in", "lbf/ft"] {
        for nested in [false, true] {
            let mut s = configured();
            s.as_object_mut().unwrap().remove("stiffness");
            if nested {
                s["hanger"]["stiffness"] = stiffness("Uy", 20., unit);
            } else {
                s["stiffness"] = stiffness("Uy", 20., unit);
            }
            accepted(&s);
        }
        let mut s = configured();
        let normalized = convert_for_dimension(
            20.,
            Dimension::LinearStiffness,
            unit_by_symbol(unit, Dimension::LinearStiffness).unwrap(),
            canonical_unit(Dimension::LinearStiffness).unwrap(),
        )
        .unwrap();
        s["stiffness"] = stiffness("UY", normalized, "N/m");
        s["hanger"]["stiffness"] = stiffness("uy", 20., unit);
        accepted(&s);
    }
}
#[test]
fn explicit_family_tokens_are_exact_while_absence_and_null_are_preserved() {
    for family in [
        "anchor",
        "guide",
        "line_stop",
        "vertical_support",
        "spring",
        "variable_spring_hanger",
        "spring_hanger",
        "constant_effort_support",
        "nonlinear",
    ] {
        let mut s = match family {
            "variable_spring_hanger" | "spring_hanger" => {
                let mut s = configured();
                s["hanger"]["hanger_type"] = json!(family);
                s
            }
            "constant_effort_support" => {
                let mut s = support();
                s["hanger"] = json!({"hanger_type":family,"source_reference":"user","constant_load":q(10.,"N"),"travel_range":q(2.,"in")});
                s
            }
            "spring" => {
                let mut s = support();
                s["stiffness"] = stiffness("UY", 20., "N/m");
                s
            }
            _ => support(),
        };
        s["family"] = json!(family);
        accepted(&s);
    }
    let s = support();
    accepted(&s);
    let mut s = support();
    s["family"] = Value::Null;
    accepted(&s);
    let mut s = configured();
    s["family"] = Value::Null;
    accepted(&s);
}
#[test]
fn invalid_families_reject_without_valid_payload_precedence_bypass() {
    for bad in [
        json!(""),
        json!(" spring"),
        json!("spring "),
        json!("Spring"),
        json!("LineStop"),
        json!("unknown"),
        json!(42),
        json!(false),
        json!({}),
    ] {
        for mut s in [
            support(),
            configured(),
            {
                let mut s = support();
                s["stiffness"] = stiffness("UY", 20., "N/m");
                s
            },
            {
                let mut s = support();
                s["nonlinear"] = json!({"behavior":"gap","dof":"Uy","initial_state":"INACTIVE","closes_when":"positive","gap":q(0.1,"in")});
                s
            },
        ] {
            // Each complete payload is valid without the invalid family token.
            accepted(&s);
            s["family"] = bad.clone();
            rejected(&s, "family");
        }
    }
}
#[test]
fn late_invalid_configuration_rolls_back_earlier_valid_support_creation() {
    for conflict in [true, false] {
        let m = model();
        let first = configured();
        let mut invalid = first.clone();
        if conflict {
            invalid["hanger"]["stiffness"] = stiffness("UY", 100., "N/m");
        } else {
            invalid["family"] = json!("Spring");
        }
        let batch = json!({"batch_id":"batch:conflict","operations":[operation(&first,"not_present",true,"first"),operation(&projection(&invalid),&canonical_json(&projection(&first)),false,"second")]});
        for out in [
            validate_operation_batch(&m, &batch, Some(&claim(&m))),
            apply_operation_batch(&m, &batch, Some(&claim(&m))),
        ] {
            assert_eq!(
                out["simulation_disposition"],
                "rolled_back_no_model_published"
            );
            assert!(out["applied_model"].is_null());
            assert!(out["acceptance"].is_null());
            assert_eq!(
                out["operation_outcomes"].as_array().unwrap().len(),
                2,
                "{out}"
            );
            assert_eq!(
                out["operation_outcomes"][0]["simulation_status"],
                "validated_on_temporary_state"
            );
        }
        assert!(m["supports"].as_array().unwrap().is_empty());
    }
}
