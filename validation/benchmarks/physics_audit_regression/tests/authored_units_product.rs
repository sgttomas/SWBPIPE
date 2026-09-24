//! Exercises the structured operation boundary and the same product adapter used by native solve.
//! Inputs are original analytical test data; local provenance is not publication clearance.
use open_pipe_stress_operation_applier::apply_operation;
use open_pipe_stress_product_physics::{
    run_linear_static_preview_with_mode, LinearStaticPreviewRequest, PreviewSolverMode,
};
use serde_json::{json, Value};

fn model(temperature: &str, engineering_units: bool) -> Value {
    let (e, g, unit) = if engineering_units {
        (200.0, 77.0, "GPa")
    } else {
        (200e9, 77e9, "Pa")
    };
    json!({"schema_version":"0.1.0","document_kind":"openpipestress.product_preview.model",
      "project":{"id":"project:authored-unit-contract","units":{"length":"m","force":"N","angle":"rad","pressure":"Pa","temperature":temperature,"stress":"MPa"}},
      "analysis_status":{"mechanics":"ready_for_preview_diagnostics","rule_check":"not_performed_user_rule_inputs_missing","professional_acceptance":"not_provided"},
      "nodes":[{"id":"node:root","position":{"x":0,"y":0,"z":0},"provenance":"user_entered_local_preview"},{"id":"node:tip","position":{"x":6,"y":0,"z":0},"provenance":"user_entered_local_preview"}],
      "pipe_segments":[{"id":"pipe:straight","from":"node:root","to":"node:tip","material":"material:input","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2191,"unit":"m"},"wall_thickness":{"value":0.00818,"unit":"m"}},"provenance":"user_entered_local_preview"}],
      "materials":[{"id":"material:input","elastic_modulus":{"value":e,"unit":unit},"shear_modulus":{"value":g,"unit":unit},"thermal_expansion_coefficient":{"value":1.2e-5,"unit":"1/degC"},"provenance":"user_entered_local_preview"}],
      "supports":[{"id":"support:anchor","node":"node:root","family":"anchor","restraints":["UX","UY","UZ","RX","RY","RZ"],"provenance":"user_entered_local_preview"}],
      "load_cases":[{"id":"load:thermal","label":"Thermal expansion","kind":"primitive_user_load","primitive_loads":[],"provenance":"user_entered_local_preview"}],"combinations":[]})
}
fn intent(unit: &str, value: f64) -> Value {
    let payload = json!({"id":"load:thermal-interval","category":"thermal","target":{"type":"element","pipe":"pipe:straight"},"direction":"global_x","magnitude":{"value":value,"unit":unit},"dimension":"temperature_interval","provenance":"user_entered_local_preview"});
    json!({"operation_id":"op:thermal","operation_kind":"create","operation_status":"proposed","author_type":"user",
      "target":{"object_type":"Load","ref":"load:thermal"},
      "change":{"change_id":"change:thermal","change_kind":"create_primitive_load","field_label":"primitive_loads","field_path":"primitive_loads","before":"not_present","after":serde_json::to_string(&payload).unwrap(),"unit":unit,"dimension":"temperature_interval","source_note":"user entered thermal interval"},
      "validation":{"schema_validation":"not_run","constraint_validation":"not_run","unit_validation":"not_run","diff_preview_status":"not_generated","application_status":"not_applied"},
      "audit_boundary":{"mutation_route":"structured_operations_only","direct_model_mutation_allowed":false,"requires_user_acceptance":true,"mutates_accepted_model_state":false},
      "professional_boundary":{"human_review_required":true,"software_makes_compliance_claim":false,"software_makes_certification_claim":false,"software_makes_sealing_claim":false,"software_makes_approval_claim":false,"software_makes_authentication_claim":false},"rationale":"Original analytical contract regression"})
}
fn check_thermal(project_unit: &str, load_unit: &str, delta: f64, engineering_units: bool) {
    let source = model(project_unit, engineering_units);
    let before = source.clone();
    let applied = apply_operation(&source, &intent(load_unit, delta), None);
    assert_eq!(source, before, "operation must retain the source model");
    assert_eq!(
        applied.validation.application_status, "applied_to_session_model",
        "{:?}",
        applied.diagnostics
    );
    let authored = applied.applied_model.expect("authored model");
    assert_eq!(
        authored["project"]["units"]["temperature"], project_unit,
        "legacy spelling must not silently mutate"
    );
    assert_eq!(
        authored["load_cases"][0]["primitive_loads"][0]["magnitude"]["unit"],
        load_unit
    );
    assert_eq!(
        authored["load_cases"][0]["primitive_loads"][0]["provenance"],
        "user_entered_local_preview"
    );
    // Model JSON roundtrip preserves authored quantities before the native-compatible DTO.
    // This is not a witness of native project-file persistence.
    let reopened: Value = serde_json::from_str(&serde_json::to_string(&authored).unwrap()).unwrap();
    assert_eq!(reopened, authored);
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        let out = run_linear_static_preview_with_mode(
            LinearStaticPreviewRequest {
                model: serde_json::from_value(reopened.clone()).unwrap(),
                materials: vec![],
            },
            mode,
        );
        assert_eq!(
            out.status.mechanics, "MECHANICS_SOLVED",
            "{:?}",
            out.diagnostics
        );
        let rows: Vec<_> = out
            .results
            .iter()
            .filter(|r| r.entity_ref == "node:tip" && r.kind == "global_nodal_displacement_x")
            .collect();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].unit, "mm");
        // Free axial thermal strain: alpha * 100 K * 6 m = 7.2 mm, independent of E and section.
        assert!((rows[0].value - 7.2).abs() < 1e-6, "{}", rows[0].value);
        let root = out
            .results
            .iter()
            .find(|r| r.entity_ref == "support:anchor" && r.kind == "reaction_resultant")
            .unwrap();
        assert_eq!(root.unit, "N");
        assert!(
            root.value.abs() < 1e-5,
            "unrestrained thermal growth must not create anchor load: {}",
            root.value
        );
    }
}
#[test]
fn authored_canonical_temperature_reaches_product_solver() {
    check_thermal("degC", "degC", 100.0, false);
}
#[test]
fn authored_legacy_temperature_reaches_product_solver_without_rewriting_storage() {
    check_thermal("C", "C", 100.0, false);
}
#[test]
fn authored_fahrenheit_interval_and_gpa_reach_product_solver() {
    check_thermal("degC", "degF", 180.0, true);
}
#[test]
fn unknown_matching_project_unit_is_blocked_by_operation() {
    let source = model("unsupported_temperature", false);
    let out = apply_operation(&source, &intent("unsupported_temperature", 100.0), None);
    assert!(
        out.applied_model.is_none(),
        "matching invalid project spelling cannot bypass catalog validation"
    );
    assert_eq!(out.validation.application_status, "blocked");
}

#[test]
fn incompatible_length_unit_cannot_create_thermal_load() {
    let source = model("degC", false);
    let out = apply_operation(&source, &intent("m", 100.0), None);
    assert!(
        out.applied_model.is_none(),
        "a length cannot be accepted as a temperature interval"
    );
    assert_eq!(out.validation.application_status, "blocked");
}
