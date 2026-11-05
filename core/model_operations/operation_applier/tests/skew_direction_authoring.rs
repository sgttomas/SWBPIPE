//! R-6/R-7 S-K1 authoring (DEL-04-03 Work `skew-restraints.md` §11 contract and §11.1
//! AP-1 conditions (a) and (b); §15 step 10). Invented fixtures only.
use open_pipe_stress_operation_applier::{apply_operation, canonical_json, OperationOutcome};
use serde_json::{json, Value};

fn model() -> Value {
    json!({"schema_version":"0.9.0","document_kind":"openpipestress.product_preview.model","project":{"id":"project:test","units":{"length":"m","force":"N","angle":"rad","pressure":"Pa","temperature":"K"}},"nodes":[{"id":"node:n","label":"Node","position":{"x":0,"y":0,"z":0},"provenance":"user"}],"supports":[],"materials":[],"sections":[],"pipe_segments":[],"components":[],"load_cases":[],"combinations":[]})
}
fn projection(s: &Value) -> Value {
    let keys = ["family", "restraints", "stiffness", "hanger", "nonlinear", "provenance"];
    Value::Object(keys.into_iter().filter_map(|k| s.get(k).map(|v| (k.into(), v.clone()))).collect())
}
fn operation(payload: &Value, before: &str, create: bool) -> Value {
    json!({"operation_id":"operation:skew","operation_kind":if create{"create"}else{"modify"},"operation_status":"proposed","author_type":"user","target":{"object_type":"Support","ref":"support:s"},"change":{"change_id":"change:skew","change_kind":if create{"create_support"}else{"update_support"},"field_label":"Support","field_path":if create{"supports"}else{"configuration"},"before":before,"after":payload.to_string(),"unit":"none","dimension":"dimensionless","source_note":"Explicit fixture"},"validation":{"schema_validation":"not_run","constraint_validation":"not_run","unit_validation":"not_run","diff_preview_status":"not_generated","application_status":"not_applied"},"audit_boundary":{"mutation_route":"structured_operations_only","direct_model_mutation_allowed":false,"requires_user_acceptance":true,"mutates_accepted_model_state":false},"professional_boundary":{"human_review_required":true,"software_makes_compliance_claim":false,"software_makes_certification_claim":false,"software_makes_sealing_claim":false,"software_makes_approval_claim":false,"software_makes_authentication_claim":false},"rationale":"Explicit fixture"})
}
fn stop(direction: Value) -> Value {
    json!({"id":"support:s","label":"Stop","node":"node:n","family":"line_stop","restraints":["UY"],"direction":direction,"provenance":"user"})
}
fn create(model: &Value, support: &Value) -> OperationOutcome {
    apply_operation(model, &operation(support, "not_present", true), None)
}
fn codes(outcome: &OperationOutcome) -> Vec<String> {
    outcome.diagnostics.iter().map(|d| d.code.clone()).collect()
}
fn written(outcome: &OperationOutcome) -> Value {
    let applied = outcome.applied_model.as_ref().unwrap_or_else(|| panic!("{:?}", codes(outcome)));
    applied["supports"][0].clone()
}

#[test]
fn direction_is_written_exactly_as_entered() {
    // Bits of the JSON numbers: shortest round-trip text, never normalized.
    for text in [r#"{"x":0.1,"y":0.2,"z":0}"#, r#"{"x":3,"y":-4,"z":5e-300}"#, r#"{"x":0,"y":2,"z":0}"#] {
        let direction: Value = serde_json::from_str(text).unwrap();
        let out = create(&model(), &stop(direction.clone()));
        let stored = written(&out)["direction"].clone();
        for axis in ["x", "y", "z"] {
            assert_eq!(stored[axis].as_f64().unwrap().to_bits(), direction[axis].as_f64().unwrap().to_bits(), "{text}");
        }
        assert_eq!(canonical_json(&stored), canonical_json(&direction));
    }
    // A skewed spring on one translational DOF is written too.
    let mut spring = stop(json!({"x":1,"y":1,"z":0}));
    spring["family"] = json!("spring");
    spring["stiffness"] = json!({"dof":"UY","value":{"value":1000.0,"unit":"N/m"}});
    assert_eq!(written(&create(&model(), &spring))["direction"], json!({"x":1,"y":1,"z":0}));
}

#[test]
fn exact_route_and_range_are_refused_by_name() {
    let mut exact = model();
    exact["pressure_contract"] = json!({"version": "pressure-2"});
    let out = create(&exact, &stop(json!({"x":3,"y":4,"z":0})));
    assert!(out.applied_model.is_none());
    assert!(codes(&out).contains(&"SUPPORT_DIRECTION_SKEW_EXACT_UNSUPPORTED".to_string()), "{:?}", codes(&out));
    // An axis-aligned direction is not a skewed record: no exact-route refusal from it.
    assert!(!codes(&create(&exact, &stop(json!({"x":0,"y":-1,"z":0})))).contains(&"SUPPORT_DIRECTION_SKEW_EXACT_UNSUPPORTED".to_string()));
    // §4.2: (1, 2^-1023, 0) has a subnormal canonical component.
    let out = create(&model(), &stop(json!({"x":1.0,"y":1.1125369292536007e-308,"z":0})));
    assert!(out.applied_model.is_none());
    assert!(codes(&out).contains(&"SUPPORT_DIRECTION_RANGE_UNSUPPORTED".to_string()), "{:?}", codes(&out));
    // The document rules come first (§2.2 item 1): a guide with a direction is the
    // record rule's refusal, even on the exact route.
    let mut guide = stop(json!({"x":3,"y":4,"z":0}));
    guide["family"] = json!("guide");
    let out = create(&exact, &guide);
    assert!(out.applied_model.is_none());
    assert!(codes(&out).contains(&"SUPPORT_DIRECTION_RECORD_UNSUPPORTED".to_string()), "{:?}", codes(&out));
    assert!(!codes(&out).contains(&"SUPPORT_DIRECTION_SKEW_EXACT_UNSUPPORTED".to_string()));
}

/// AP-1 (a) and (b): `update_support configuration` replaces only its CONFIG members, so a
/// kept `direction` survives an unrelated edit; a change that makes it inadmissible is
/// refused by the document rule by name, never written and never dropped.
#[test]
fn update_support_keeps_or_refuses_a_kept_direction() {
    let current = stop(json!({"x":3,"y":4,"z":0}));
    let mut doc = model();
    doc["supports"] = json!([current.clone()]);
    let update = |after: &Value| apply_operation(&doc, &operation(&projection(after), &canonical_json(&projection(&current)), false), None);
    // An unrelated edit (provenance) keeps the direction.
    let mut edited = current.clone();
    edited["provenance"] = json!("user-revised");
    assert_eq!(written(&update(&edited))["direction"], json!({"x":3,"y":4,"z":0}));
    // (b) UX → UY on the single translational token is well-defined.
    let mut moved = current.clone();
    moved["restraints"] = json!(["UX"]);
    assert_eq!(written(&update(&moved))["direction"], json!({"x":3,"y":4,"z":0}));
    // (a) Each inadmissible configuration is refused by name.
    let mut family = current.clone();
    family["family"] = json!("guide");
    let mut multi = current.clone();
    multi["restraints"] = json!(["UX", "UY"]);
    let mut nonlinear = current.clone();
    nonlinear["nonlinear"] = json!({"behavior": "one_way", "dof": "UY", "initial_state": "active", "active_when": "positive_reaction"});
    let mut rotational = current.clone();
    rotational["restraints"] = json!(["RX"]);
    for (after, code) in [
        (family, "SUPPORT_DIRECTION_RECORD_UNSUPPORTED"),
        (multi, "SUPPORT_DIRECTION_RECORD_UNSUPPORTED"),
        (nonlinear, "SUPPORT_DIRECTION_RECORD_UNSUPPORTED"),
        (rotational, "SUPPORT_DIRECTION_ROTATIONAL_UNSUPPORTED"),
    ] {
        let out = update(&after);
        assert!(out.applied_model.is_none(), "{code}: {:?}", codes(&out));
        assert!(codes(&out).iter().any(|c| c == code), "{code}: {:?}", codes(&out));
    }
}

/// Manager steer (§2.2 item 2, §9): the applier's EX predicate is
/// `condition_set_authoring::has_pressure_contract`, present and not null. Any
/// non-null contract, a retired or invalid one included, refuses a skewed direction;
/// an absent or null contract admits it; an axis-aligned direction is never refused
/// for EX.
#[test]
fn the_pressure_contract_predicate_is_present_and_not_null() {
    let skewed = stop(json!({"x":3,"y":4,"z":0}));
    let mut retired = model();
    retired["pressure_contract"] = json!({"version": "2.0.0", "mode": "retired_pressure_v2"});
    let out = create(&retired, &skewed);
    assert!(out.applied_model.is_none());
    assert!(codes(&out).contains(&"SUPPORT_DIRECTION_SKEW_EXACT_UNSUPPORTED".to_string()), "{:?}", codes(&out));
    let mut null = model();
    null["pressure_contract"] = Value::Null;
    assert_eq!(written(&create(&null, &skewed))["direction"], json!({"x":3,"y":4,"z":0}));
    let axis = create(&retired, &stop(json!({"x":0,"y":1,"z":0})));
    assert!(axis.applied_model.is_some(), "{:?}", codes(&axis));
    assert_eq!(written(&axis)["direction"], json!({"x":0,"y":1,"z":0}));
}

/// Authoring's should-fix (§11.2): the skew gates run only after the document
/// validator's own item-1 rules pass, on the record as the applier writes it (no
/// re-implemented trim, case or null semantics). The validator's closed token set
/// admits `uy`, so that record reaches item 2 exactly as the validator says. An
/// earlier stage's refusal stands alone: `nonlinear: null` is the applier's rich
/// configuration refusal, and a family the record rules refuse is their code; neither
/// gets the skew code (one code per object). A padded token the applier normalizes
/// (" UY") is judged as written ("UY") and is refused for EX by name.
#[test]
fn item_one_decides_before_the_skew_gates() {
    let mut exact = model();
    exact["pressure_contract"] = json!({"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"});
    let skew_code = "SUPPORT_DIRECTION_SKEW_EXACT_UNSUPPORTED".to_string();
    let mut lower = stop(json!({"x":3,"y":4,"z":0}));
    lower["restraints"] = json!(["uy"]);
    let mut null = stop(json!({"x":3,"y":4,"z":0}));
    null["nonlinear"] = Value::Null;
    let mut padded = stop(json!({"x":3,"y":4,"z":0}));
    padded["restraints"] = json!([" UY"]);
    for (what, record) in [("lowercase token", lower), ("padded token", padded)] {
        let out = create(&exact, &record);
        assert!(out.applied_model.is_none() && codes(&out) == vec![skew_code.clone()], "{what}: {:?}", codes(&out));
    }
    let mut cases = vec![("nonlinear null", null)];
    for family in ["anchor", "vertical_support", "guide"] {
        let mut record = stop(json!({"x":3,"y":4,"z":0}));
        record["family"] = json!(family);
        cases.push((family, record));
    }
    for (what, record) in cases {
        let out = create(&exact, &record);
        assert!(out.applied_model.is_none(), "{what}");
        assert!(!codes(&out).contains(&skew_code), "{what}: {:?}", codes(&out));
        assert!(codes(&out).iter().any(|c| c.starts_with("SUPPORT_") || c.starts_with("MODEL") || c.starts_with("OP-")), "{what}: {:?}", codes(&out));
        eprintln!("{what}: {:?}", codes(&out));
    }
}
