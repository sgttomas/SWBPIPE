//! A variable spring hanger acts through its explicit `hanger.stiffness.dof`
//! (`LinearSupport::spring`), never through rigid restraint rows, so an actually empty
//! restraint list is its authored form: the reference system's `support:RS-VS1` and the
//! invented preview model's `support:SH-140` both store `restraints: []` with stiffness DOF
//! `UZ`. The applier admits that form on create and on a direct restraint replacement, only
//! for family exactly `variable_spring_hanger` with stiffness DOF exactly `UX`, `UY` or
//! `UZ`; every other empty set keeps `OP-RESTRAINT-SET-EMPTY`. Configuration replacement
//! already admitted it (the rich validator iterates the list) and is unchanged. The empty
//! and the matching singleton forms solve identically.
use open_pipe_stress_operation_applier::{apply_operation, canonical_json, sha256_hex, validate_operation, OperationOutcome};
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};

const REFERENCE_SYSTEM: &str = include_str!("../../../../fixtures/reference_system/reference_system.json");
const PREVIEW_MODEL: &str = include_str!("../../../product_physics/tests/fixtures/preview_physics_invented_model.json");

fn q(value: f64, unit: &str) -> Value {
    json!({"value": value, "unit": unit})
}

fn base() -> Value {
    json!({"schema_version":"0.9.0","document_kind":"openpipestress.product_preview.model","project":{"id":"project:vsh","units":{"length":"m","force":"N","angle":"rad","pressure":"Pa","temperature":"K"}},
        "nodes":[{"id":"node:n","label":"Node","position":{"x":0,"y":0,"z":0},"provenance":"user"}],
        "supports":[],"materials":[],"sections":[],"pipe_segments":[],"components":[],"load_cases":[],"combinations":[]})
}

/// A complete variable spring hanger on `dof` with the given restraints and family.
fn hanger(family: &str, dof: Value, restraints: Value) -> Value {
    json!({"id":"support:vs","label":"Variable spring","node":"node:n","family":family,"restraints":restraints,"provenance":"user",
        "hanger":{"hanger_type":"variable_spring_hanger","source_reference":"user","stiffness":{"dof":dof,"value":q(42000.0,"N/m")},
            "installed_load":q(460.0,"N"),"cold_load":q(430.0,"N"),"hot_load":q(390.0,"N"),"travel_range":q(0.045,"m")}})
}

fn with_support(support: &Value) -> Value {
    let mut model = base();
    model["supports"] = json!([support]);
    model
}

fn claim(model: &Value) -> Value {
    json!({"algorithm":"sha256","canonicalization":"rfc8785_jcs","payload_scope":"model_payload","value":format!("sha256:{}",sha256_hex(&canonical_json(model)))})
}

fn op(kind: &str, change_kind: &str, target: &str, path: &str, before: &str, after: &str) -> Value {
    json!({"operation_id":format!("operation:{change_kind}:{path}"),"operation_kind":kind,"operation_status":"proposed","author_type":"user",
        "target":{"object_type":"Support","ref":target},
        "change":{"change_id":format!("change:{change_kind}:{path}"),"change_kind":change_kind,"field_label":"Support","field_path":path,"before":before,"after":after,"unit":"none","dimension":"dimensionless","source_note":"Explicit fixture"},
        "validation":{"schema_validation":"not_run","constraint_validation":"not_run","unit_validation":"not_run","diff_preview_status":"not_generated","application_status":"not_applied"},
        "audit_boundary":{"mutation_route":"structured_operations_only","direct_model_mutation_allowed":false,"requires_user_acceptance":true,"mutates_accepted_model_state":false},
        "professional_boundary":{"human_review_required":true,"software_makes_compliance_claim":false,"software_makes_certification_claim":false,"software_makes_sealing_claim":false,"software_makes_approval_claim":false,"software_makes_authentication_claim":false},
        "rationale":"Explicit fixture"})
}

fn create(record: &Value) -> Value {
    op("create", "create_support", record["id"].as_str().unwrap(), "supports", "not_present", &record.to_string())
}

fn set_restraints(id: &str, before: &str, after: &str) -> Value {
    op("modify", "update_support", id, "restraints", before, after)
}

fn delete(record: &Value) -> Value {
    op("delete", "delete_support", record["id"].as_str().unwrap(), "supports", record["label"].as_str().unwrap(), "not_present")
}

fn blocking(outcome: &OperationOutcome) -> Vec<String> {
    outcome.diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| d.code.clone()).collect()
}

/// Validate, then apply: the same diff preview, no document from validation, and the input
/// never changes.
fn applied(model: &Value, intent: &Value) -> Value {
    let original = model.clone();
    let preview = validate_operation(model, intent, Some(&claim(model)));
    let outcome = apply_operation(model, intent, Some(&claim(model)));
    assert!(preview.applied_model.is_none());
    assert!(blocking(&preview).is_empty(), "preview: {:?}", preview.diagnostics);
    assert_eq!(serde_json::to_value(&preview.diff_preview).unwrap(), serde_json::to_value(&outcome.diff_preview).unwrap());
    assert_eq!(*model, original, "the input is never mutated");
    outcome.applied_model.unwrap_or_else(|| panic!("not applied: {:?}", outcome.diagnostics))
}

fn refused(model: &Value, intent: &Value, code: &str) {
    let original = model.clone();
    for outcome in [validate_operation(model, intent, Some(&claim(model))), apply_operation(model, intent, Some(&claim(model)))] {
        assert!(outcome.applied_model.is_none(), "{code}: applied unexpectedly");
        assert!(blocking(&outcome).iter().any(|found| found == code), "{code} not in {:?}", outcome.diagnostics);
    }
    assert_eq!(*model, original);
}

#[test]
fn create_admits_an_actually_empty_list_for_an_explicit_translational_dof() {
    for dof in ["UX", "UY", "UZ"] {
        let record = hanger("variable_spring_hanger", json!(dof), json!([]));
        let created = applied(&base(), &create(&record));
        assert_eq!(canonical_json(&created["supports"][0]), canonical_json(&record), "{dof}: stored as authored");
        // Inverse-operation checks (undo by the inverse delete, redo by the same create) and a
        // canonical-serialization round trip. Desktop history and file save/open are integration
        // evidence (the authoring lane's real-engine coverage), not claimed here.
        let undone = applied(&created, &delete(&record));
        assert_eq!(undone, base());
        let redone = applied(&undone, &create(&record));
        assert_eq!(canonical_json(&redone), canonical_json(&created));
        let reopened: Value = serde_json::from_str(&serde_json::to_string(&created).unwrap()).unwrap();
        assert_eq!(canonical_json(&reopened), canonical_json(&created));
        assert!(!open_pipe_stress_operation_applier::validate_model_document(&reopened).iter().any(|d| d.is_blocking()));
    }
}

#[test]
fn the_reference_systems_rs_vs1_is_authored_as_stored() {
    let model: Value = serde_json::from_str(REFERENCE_SYSTEM).unwrap();
    let supports = model["supports"].as_array().unwrap();
    let index = supports.iter().position(|s| s["id"] == "support:RS-VS1").unwrap();
    let record = supports[index].clone();
    assert_eq!(record["restraints"], json!([]));
    let mut without = model.clone();
    without["supports"].as_array_mut().unwrap().remove(index);
    let created = applied(&without, &create(&record));
    let stored = created["supports"].as_array().unwrap().iter().find(|s| s["id"] == "support:RS-VS1").unwrap();
    assert_eq!(canonical_json(stored), canonical_json(&record), "the fixture's record is authored with canonical JSON equality");
}

#[test]
fn a_direct_restraint_replacement_admits_the_empty_set_and_back() {
    let singleton = hanger("variable_spring_hanger", json!("UZ"), json!(["UZ"]));
    let model = with_support(&singleton);
    let emptied = applied(&model, &set_restraints("support:vs", "UZ", ""));
    assert_eq!(emptied["supports"][0]["restraints"], json!([]));
    assert_eq!(emptied["supports"][0]["hanger"], singleton["hanger"], "hanger data is kept");
    let back = applied(&emptied, &set_restraints("support:vs", "", "UZ"));
    assert_eq!(back, model, "the inverse replacement restores the singleton exactly");
    // A whitespace-only replacement is not an actually empty one.
    refused(&model, &set_restraints("support:vs", "UZ", " "), "OP-RESTRAINT-SET-EMPTY");
}

#[test]
fn configuration_replacement_admits_and_preserves_the_same_record() {
    let singleton = hanger("variable_spring_hanger", json!("UZ"), json!(["UZ"]));
    let model = with_support(&singleton);
    let projection = |s: &Value| {
        let keys = ["family", "restraints", "stiffness", "hanger", "nonlinear", "provenance"];
        Value::Object(keys.into_iter().filter_map(|k| s.get(k).map(|v| (k.into(), v.clone()))).collect())
    };
    let empty = hanger("variable_spring_hanger", json!("UZ"), json!([]));
    let intent = op("modify", "update_support", "support:vs", "configuration", &canonical_json(&projection(&singleton)), &projection(&empty).to_string());
    let replaced = applied(&model, &intent);
    assert_eq!(canonical_json(&replaced["supports"][0]), canonical_json(&empty));
    // An unrelated configuration edit keeps the empty list.
    let mut relabelled = empty.clone();
    relabelled["provenance"] = json!("user, revised");
    let intent = op("modify", "update_support", "support:vs", "configuration", &canonical_json(&projection(&empty)), &projection(&relabelled).to_string());
    assert_eq!(applied(&replaced, &intent)["supports"][0]["restraints"], json!([]));
}

#[test]
fn every_other_empty_set_keeps_its_refusal() {
    // Create: a DOF that is missing, null, invalid, rotational or not exactly canonical,
    // a family other than exactly `variable_spring_hanger`, and stiffness only at the top level.
    let mut top_level_only = hanger("variable_spring_hanger", json!("UZ"), json!([]));
    top_level_only["stiffness"] = top_level_only["hanger"]["stiffness"].clone();
    top_level_only["hanger"].as_object_mut().unwrap().remove("stiffness");
    let mut missing = hanger("variable_spring_hanger", json!("UZ"), json!([]));
    missing["hanger"]["stiffness"].as_object_mut().unwrap().remove("dof");
    let cases = [
        hanger("variable_spring_hanger", json!("uz"), json!([])),
        hanger("variable_spring_hanger", json!(" UZ"), json!([])),
        hanger("variable_spring_hanger", json!("RZ"), json!([])),
        hanger("variable_spring_hanger", json!("UQ"), json!([])),
        hanger("variable_spring_hanger", Value::Null, json!([])),
        hanger("spring_hanger", json!("UZ"), json!([])),
        hanger("Variable_Spring_Hanger", json!("UZ"), json!([])),
        missing,
        top_level_only,
        // An empty list made of blank tokens is not an actually empty list.
        hanger("variable_spring_hanger", json!("UZ"), json!([" "])),
    ];
    for record in &cases {
        let outcome = apply_operation(&base(), &create(record), Some(&claim(&base())));
        assert!(outcome.applied_model.is_none(), "applied unexpectedly: {record}");
        assert!(!blocking(&outcome).is_empty(), "{record}");
    }
    // The exactly-canonical cases that the rich validator would otherwise accept are refused
    // by the empty-set code itself.
    for dof in ["uz", " UZ", "RZ"] {
        let record = hanger("variable_spring_hanger", json!(dof), json!([]));
        refused(&base(), &create(&record), "OP-RESTRAINT-SET-EMPTY");
    }
    // Constant effort keeps its create and direct-edit refusals.
    let constant = json!({"id":"support:ce","label":"Constant effort","node":"node:n","family":"constant_effort_support","restraints":[],"provenance":"user",
        "hanger":{"hanger_type":"constant_effort_support","source_reference":"user","constant_load":q(2.0,"kN"),"travel_range":q(0.05,"m")}});
    refused(&base(), &create(&constant), "OP-RESTRAINT-SET-EMPTY");
    let mut constant_uz = constant.clone();
    constant_uz["restraints"] = json!(["UZ"]);
    refused(&with_support(&constant_uz), &set_restraints("support:ce", "UZ", ""), "OP-RESTRAINT-SET-EMPTY");
    // A plain support and a plain spring keep theirs.
    let plain = json!({"id":"support:p","label":"Stop","node":"node:n","restraints":["UY"],"provenance":"user"});
    refused(&with_support(&plain), &set_restraints("support:p", "UY", ""), "OP-RESTRAINT-SET-EMPTY");
    let spring = json!({"id":"support:k","label":"Spring","node":"node:n","family":"spring","restraints":["UZ"],"stiffness":{"dof":"UZ","value":q(1.0e6,"N/m")},"provenance":"user"});
    refused(&with_support(&spring), &set_restraints("support:k", "UZ", ""), "OP-RESTRAINT-SET-EMPTY");
    // A variable spring hanger whose stiffness DOF is not exactly canonical keeps it on edit.
    let lower = hanger("variable_spring_hanger", json!("uz"), json!(["UZ"]));
    refused(&with_support(&lower), &set_restraints("support:vs", "UZ", ""), "OP-RESTRAINT-SET-EMPTY");
}

/// The empty and the matching singleton forms are the same mechanics: the hanger is a spring
/// on its stiffness DOF, with no rigid restraint row.
#[test]
fn empty_and_singleton_restraints_solve_identically() {
    let model: Value = serde_json::from_str(PREVIEW_MODEL).unwrap();
    let index = model["supports"].as_array().unwrap().iter().position(|s| s["id"] == "support:SH-140").unwrap();
    assert_eq!(model["supports"][index]["restraints"], json!([]));
    let mut singleton = model.clone();
    singleton["supports"][index]["restraints"] = json!(["UZ"]);
    for mode in [PreviewSolverMode::DenseScrutiny, PreviewSolverMode::SparseInteractive] {
        let solve = |m: &Value| serde_json::to_value(run_linear_static_preview_value_with_mode(json!({"model": m, "materials": []}), mode).unwrap()).unwrap();
        let (empty, single) = (solve(&model), solve(&singleton));
        let rows = empty["results"].as_array().expect("result rows");
        assert!(rows.iter().any(|row| row["entity_ref"] == "support:SH-140"), "{mode:?}: the hanger publishes rows");
        assert_eq!(empty["results"], single["results"], "{mode:?}: identical result rows");
        assert_eq!(empty["diagnostics"], single["diagnostics"], "{mode:?}: identical findings");
    }
}
