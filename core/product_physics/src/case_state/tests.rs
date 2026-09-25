//! Implementer smoke tests for the connected 0.4.0 route. The independent
//! acceptance suite is `tests/load_reference_state_runtime.rs` (separate
//! author). Expected values come from the maintained reference fixture.
use crate::{run_linear_static_preview_value_with_mode, MechanicsEnvelope, PreviewSolverMode};
use serde_json::{json, Value};

const REFERENCES: &str =
    include_str!("../../tests/fixtures/load_reference_states/reference_cases.json");

fn reference(case: &str, variant: &str, path: &[&str]) -> f64 {
    let fixture: Value = serde_json::from_str(REFERENCES).unwrap();
    let mut value = &fixture["cases"][case]["variants"][variant]["expected"];
    for key in path {
        value = &value[*key];
    }
    value["value"].as_f64().unwrap()
}

fn close(actual: f64, expected: f64, scale: f64) {
    let allowance = if expected == 0.0 {
        scale * 1e-9
    } else {
        expected.abs() * 1e-9
    };
    assert!(
        (actual - expected).abs() <= allowance,
        "{actual:.17e} versus {expected:.17e}"
    );
}

fn node(id: &str, x: f64) -> Value {
    json!({"id": id, "position": {"x": x, "y": 0, "z": 0}, "provenance": "invented"})
}

fn pipe(id: &str, from: &str, to: &str) -> Value {
    json!({"id": id, "from": from, "to": to,
        "section": {"outside_diameter": {"value": 0.2, "unit": "m"}, "wall_thickness": {"value": 0.01, "unit": "m"}},
        "material": "material:shared", "y_reference": {"x": 0, "y": 1, "z": 0}, "provenance": "invented"})
}

fn anchor(id: &str, node: &str) -> Value {
    json!({"id": id, "node": node, "family": "anchor",
        "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": "invented"})
}

fn material() -> Value {
    json!({"id": "material:shared", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
        "elastic_modulus": {"value": 200, "unit": "GPa"}, "poisson_ratio": {"value": 0.3, "unit": "1"},
        "provenance": "invented explicit test data",
        "temperature_points": [
            {"id": "point:stiff", "temperature": {"value": 20, "unit": "degC"},
             "elastic_modulus": {"value": 200, "unit": "GPa"}, "poisson_ratio": {"value": 0.3, "unit": "1"}},
            {"id": "point:soft", "temperature": {"value": 120, "unit": "degC"},
             "elastic_modulus": {"value": 100, "unit": "GPa"}, "poisson_ratio": {"value": 0.25, "unit": "1"}}]})
}

fn element(pipe: &str, point: &str) -> Value {
    json!({"pipe_ref": pipe,
        "material_selection": {"kind": "exact_point", "material_ref": "material:shared", "point_ref": point},
        "thermal_state": {"kind": "unchanged_reference", "provenance": "invented"}})
}

fn model(
    nodes: Vec<Value>,
    pipes: Vec<Value>,
    supports: Vec<Value>,
    elements: Vec<Value>,
    support_states: Vec<Value>,
    primitives: Vec<Value>,
    sources: Vec<Value>,
) -> Value {
    let members = pipes
        .iter()
        .map(|p| {
            json!({"pipe_ref": p["id"], "basis": {"kind": "direct_strain_reference"},
            "fit": {"kind": "none"}, "provenance": "invented"})
        })
        .collect::<Vec<_>>();
    json!({"model": {
        "schema_version": "0.4.0", "document_kind": "openpipestress.product_preview.model",
        "pressure_contract": {"version": "2.0.0", "mode": "exact_straight_pressure_v2"},
        "project": {"id": "project:load-state-smoke", "units": {"length": "m"}},
        "analysis_status": {"mechanics": "ready", "rule_check": "not_performed", "professional_acceptance": "not_provided"},
        "nodes": nodes, "pipe_segments": pipes, "supports": supports, "components": [],
        "materials": [material()],
        "reference_configurations": [{"id": "reference:installed", "geometry_ref": {"kind": "authored_model_geometry"},
            "member_references": members, "provenance": "invented"}],
        "load_cases": [{"id": "case:smoke", "primitive_loads": primitives, "pressure_regions": [], "provenance": "invented",
            "analysis_state": {"contract": "openpipestress.load_reference_state/1.0.0",
                "reference_configuration_ref": "reference:installed", "element_states": elements,
                "support_states": support_states, "load_sources": sources,
                "history": {"kind": "independent_equilibrium"}, "provenance": "invented"}}],
        "combinations": []}, "materials": []})
}

fn run(request: &Value, mode: PreviewSolverMode) -> MechanicsEnvelope {
    run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap()
}

fn row(envelope: &MechanicsEnvelope, kind: &str, entity: &str, component: &str) -> f64 {
    envelope
        .results
        .iter()
        .find(|row| {
            row.kind == kind
                && row.entity_ref == entity
                && row
                    .metadata
                    .as_ref()
                    .is_some_and(|m| m.component == component)
        })
        .unwrap_or_else(|| panic!("missing {kind} {entity} {component}"))
        .value
}

fn blocking_codes(envelope: &MechanicsEnvelope) -> Vec<String> {
    envelope
        .diagnostics
        .iter()
        .filter(|d| d.severity == "blocking")
        .map(|d| d.code.clone())
        .collect()
}

fn two_bar() -> Value {
    model(
        vec![
            node("node:a", 0.0),
            node("node:b", 1.0),
            node("node:c", 2.0),
        ],
        vec![
            pipe("pipe:1", "node:a", "node:b"),
            pipe("pipe:2", "node:b", "node:c"),
        ],
        vec![
            anchor("support:root", "node:a"),
            anchor("support:far", "node:c"),
        ],
        vec![
            element("pipe:1", "point:stiff"),
            element("pipe:2", "point:soft"),
        ],
        vec![
            json!({"support_ref": "support:root", "participation": {"kind": "active_model_device"},
                "boundary_motion": [{"dof": "UX", "value": {"value": 0.1, "unit": "mm"}, "meaning": "absolute_reference_displacement"}]}),
            json!({"support_ref": "support:far", "participation": {"kind": "active_model_device"}}),
        ],
        vec![],
        vec![],
    )
}

#[test]
fn prescribed_root_translation_couples_through_partitioned_solve_and_unreduced_reactions() {
    let request = two_bar();
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        let envelope = run(&request, mode);
        assert!(
            blocking_codes(&envelope).is_empty(),
            "{:?}",
            envelope.diagnostics
        );
        assert_eq!(
            envelope.producer.semantic_contract_id,
            crate::LOAD_REFERENCE_SEMANTIC_CONTRACT_ID
        );
        assert_eq!(
            envelope.formulation_basis.profile_id,
            crate::LOAD_STATE_PROFILE_ID
        );
        assert!(envelope.source_block_recovery.is_none());
        let case = "prescribed_translation_two_bar";
        let middle_mm = row(
            &envelope,
            "global_nodal_displacement_x",
            "node:b",
            "nodal_displacement_x",
        );
        close(
            middle_mm / 1000.0,
            reference(case, "annular_companion", &["middle_UX"]),
            1e-4,
        );
        let root = row(
            &envelope,
            "support_reaction_component_v2",
            "support:root",
            "Fx",
        );
        let far = row(
            &envelope,
            "support_reaction_component_v2",
            "support:far",
            "Fx",
        );
        let expected_root = reference(case, "annular_companion", &["root_Fx"]);
        close(root, expected_root, expected_root.abs());
        close(
            far,
            reference(case, "annular_companion", &["far_Fx"]),
            expected_root.abs(),
        );
        // Reduced-only reaction K_cf u_f - f_c omits K_cc g_c: -2x the correct root value here.
        assert!((root - (-2.0 * expected_root)).abs() > expected_root.abs());
        let evidence = &envelope.contract_evidence.as_ref().unwrap()["load_reference_states"];
        assert_eq!(evidence[0]["load_case_id"], "case:smoke");
        assert_eq!(
            evidence[0]["support_components"][0]["prescribed_value"],
            1e-4
        );
        assert_eq!(evidence[0]["members"][1]["selected_E_pa"], 100e9);
        assert!(envelope
            .diagnostics
            .iter()
            .any(|d| d.code == super::SOURCE_RECOVERY_NOT_JOINED));
    }
}

#[test]
fn all_prescribed_single_bar_has_no_free_axial_motion_but_nonzero_reactions() {
    let request = model(
        vec![node("node:a", 0.0), node("node:b", 1.0)],
        vec![pipe("pipe:1", "node:a", "node:b")],
        vec![
            anchor("support:root", "node:a"),
            anchor("support:far", "node:b"),
        ],
        vec![element("pipe:1", "point:stiff")],
        vec![
            json!({"support_ref": "support:root", "participation": {"kind": "active_model_device"},
                "boundary_motion": [{"dof": "UX", "value": {"value": 0.1, "unit": "mm"}, "meaning": "absolute_reference_displacement"}]}),
            json!({"support_ref": "support:far", "participation": {"kind": "active_model_device"}}),
        ],
        vec![],
        vec![],
    );
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        let envelope = run(&request, mode);
        let codes = blocking_codes(&envelope);
        assert!(codes.is_empty(), "{:?}", envelope.diagnostics);
        let case = "prescribed_translation_all_fixed";
        let expected = reference(case, "annular_companion", &["root_Fx"]);
        close(
            row(
                &envelope,
                "support_reaction_component_v2",
                "support:root",
                "Fx",
            ),
            expected,
            expected.abs(),
        );
        close(
            row(
                &envelope,
                "support_reaction_component_v2",
                "support:far",
                "Fx",
            ),
            reference(case, "annular_companion", &["far_Fx"]),
            expected.abs(),
        );
    }
}

#[test]
fn declared_sources_apply_once_and_unreferenced_or_duplicate_sources_do_not() {
    let force = |id: &str, value: f64| {
        json!({"id": id, "category": "concentrated_force", "target": {"type": "node", "node": "node:b"},
            "direction": "global_x", "magnitude": {"value": value, "unit": "N"}, "dimension": "force", "provenance": "invented"})
    };
    let source = |id: &str| json!({"source_ref": id, "factor": 1.0});
    let build = |sources: Vec<Value>| {
        model(
            vec![node("node:a", 0.0), node("node:b", 1.0)],
            vec![pipe("pipe:1", "node:a", "node:b")],
            vec![anchor("support:root", "node:a")],
            vec![element("pipe:1", "point:stiff")],
            vec![
                json!({"support_ref": "support:root", "participation": {"kind": "active_model_device"}}),
            ],
            vec![
                force("load:preload", 100.0),
                force("load:weight", -1000.0),
                force("load:independent", 200.0),
                force("load:unused", 999.0),
            ],
            sources,
        )
    };
    let request = build(vec![
        source("load:preload"),
        source("load:weight"),
        source("load:independent"),
    ]);
    let envelope = run(&request, PreviewSolverMode::DenseScrutiny);
    assert!(
        blocking_codes(&envelope).is_empty(),
        "{:?}",
        envelope.diagnostics
    );
    let case = "persistent_source_once";
    let expected = reference(case, "annular_companion", &["root_Fx"]);
    close(
        row(
            &envelope,
            "support_reaction_component_v2",
            "support:root",
            "Fx",
        ),
        expected,
        expected.abs(),
    );
    let evidence = &envelope.contract_evidence.as_ref().unwrap()["load_reference_states"][0];
    assert_eq!(evidence["excluded_sources"][0]["source_id"], "load:unused");
    let duplicate = build(vec![
        source("load:preload"),
        source("load:preload"),
        source("load:weight"),
    ]);
    let envelope = run(&duplicate, PreviewSolverMode::DenseScrutiny);
    assert!(blocking_codes(&envelope).contains(&"LOAD_STATE_SOURCE_DUPLICATE".to_string()));
    assert!(envelope.results.is_empty());
}

#[test]
fn older_documents_carrying_the_new_namespace_block_instead_of_ignoring_it() {
    let mut request = two_bar();
    request["model"]["schema_version"] = json!("0.3.0");
    let envelope = run(&request, PreviewSolverMode::DenseScrutiny);
    assert!(blocking_codes(&envelope).contains(&"LOAD_STATE_CONTRACT_VERSION_MISMATCH".to_string()));
    let mut unknown = two_bar();
    unknown["model"]["load_cases"][0]["analysis_state"]["unexpected"] = json!(true);
    assert!(
        run_linear_static_preview_value_with_mode(unknown, PreviewSolverMode::DenseScrutiny)
            .is_err()
    );
}

fn single_member(material: Value, element: Value, basis: Value) -> Value {
    let mut request = model(
        vec![node("node:a", 0.0), node("node:b", 1.0)],
        vec![pipe("pipe:1", "node:a", "node:b")],
        vec![anchor("support:root", "node:a")],
        vec![element],
        vec![
            json!({"support_ref": "support:root", "participation": {"kind": "active_model_device"}}),
        ],
        vec![],
        vec![],
    );
    request["model"]["materials"] = json!([material]);
    request["model"]["reference_configurations"][0]["member_references"][0]["basis"] = basis;
    request
}

fn point(id: &str, value: f64, unit: &str, e_gpa: f64) -> Value {
    json!({"id": id, "temperature": {"value": value, "unit": unit},
        "elastic_modulus": {"value": e_gpa, "unit": "GPa"}, "poisson_ratio": {"value": 0.3, "unit": "1"}})
}

fn identity_material(points: Vec<Value>, laws: Value) -> Value {
    json!({"id": "material:shared", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
        "elastic_modulus": {"value": 200, "unit": "GPa"}, "poisson_ratio": {"value": 0.3, "unit": "1"},
        "provenance": "invented explicit test data", "temperature_points": points, "expansion_laws": laws})
}

fn resolved_member(request: &Value) -> Value {
    let envelope = run(request, PreviewSolverMode::DenseScrutiny);
    assert!(
        blocking_codes(&envelope).is_empty(),
        "{:?}",
        envelope.diagnostics
    );
    envelope.contract_evidence.unwrap()["load_reference_states"][0]["members"][0].clone()
}

#[test]
fn temperature_identity_is_exact_across_authored_units() {
    // Reviewer SF3 regressions: -50 degC endpoint of a 223.15 K table.
    let material = identity_material(
        vec![
            point("point:cold", 223.15, "K", 200.0),
            point("point:warm", 300.0, "K", 180.0),
        ],
        json!([]),
    );
    let element = json!({"pipe_ref": "pipe:1",
        "material_selection": {"kind": "temperature_interpolation", "material_ref": "material:shared",
            "temperature": {"value": -50.0, "unit": "degC"}, "interpolation": "piecewise_linear", "extrapolation": "forbidden"},
        "thermal_state": {"kind": "unchanged_reference", "provenance": "invented"}});
    let member = resolved_member(&single_member(
        material,
        element,
        json!({"kind": "direct_strain_reference"}),
    ));
    assert_eq!(
        member["consumed_material_points"].as_array().unwrap().len(),
        1
    );
    assert_eq!(
        member["consumed_material_points"][0]["point_id"],
        "point:cold"
    );
    assert_eq!(member["interpolation_fraction"], Value::Null);
    // 242 degC exact point with actual temperature 467.6 degF needs no override.
    let material = identity_material(vec![point("point:hot", 242.0, "degC", 150.0)], json!([]));
    let element = json!({"pipe_ref": "pipe:1", "operating_temperature": {"value": 467.6, "unit": "degF"},
        "material_selection": {"kind": "exact_point", "material_ref": "material:shared", "point_ref": "point:hot"},
        "thermal_state": {"kind": "unchanged_reference", "provenance": "invented"}});
    let member = resolved_member(&single_member(
        material,
        element,
        json!({"kind": "direct_strain_reference"}),
    ));
    assert_eq!(
        member["operating_temperature_k"],
        member["material_selection_temperature_k"]
    );
    assert_eq!(member["selected_E_pa"], 150e9);
    // Interior 515.15 K point requested at 467.6 degF is that exact point.
    let material = identity_material(
        vec![
            point("point:a", 400.0, "K", 190.0),
            point("point:b", 515.15, "K", 170.0),
            point("point:c", 600.0, "K", 150.0),
        ],
        json!([]),
    );
    let element = json!({"pipe_ref": "pipe:1",
        "material_selection": {"kind": "temperature_interpolation", "material_ref": "material:shared",
            "temperature": {"value": 467.6, "unit": "degF"}, "interpolation": "piecewise_linear", "extrapolation": "forbidden"},
        "thermal_state": {"kind": "unchanged_reference", "provenance": "invented"}});
    let member = resolved_member(&single_member(
        material,
        element,
        json!({"kind": "direct_strain_reference"}),
    ));
    assert_eq!(
        member["consumed_material_points"].as_array().unwrap().len(),
        1
    );
    assert_eq!(member["consumed_material_points"][0]["point_id"], "point:b");
    assert_eq!(member["selected_E_pa"], 170e9);
}

#[test]
fn thermal_datum_and_table_identities_are_exact_across_units() {
    // Dilation datum authored as 20 degC must hit the 293.15 K zero point and
    // installation 50 degC the 323.15 K point; 150 degC operating is a point.
    let laws = json!([{"id": "law:dilation", "definition": "engineering_dilation",
        "datum_temperature": {"value": 20.0, "unit": "degC"},
        "data": {"kind": "table", "interpolation": "linear_dilation", "points": [
            {"temperature": {"value": 293.15, "unit": "K"}, "dilation": {"value": 0.0, "unit": "1"}},
            {"temperature": {"value": 323.15, "unit": "K"}, "dilation": {"value": 0.00036, "unit": "1"}},
            {"temperature": {"value": 302.0, "unit": "degF"}, "dilation": {"value": 0.00208, "unit": "1"}}]},
        "provenance": "invented"}]);
    let material = identity_material(vec![], laws);
    let element = json!({"pipe_ref": "pipe:1", "operating_temperature": {"value": 150.0, "unit": "degC"},
        "material_selection": {"kind": "explicit_base_properties", "material_ref": "material:shared",
            "applicability_reference": "invented fixed basis"},
        "thermal_state": {"kind": "free_length_state", "expansion_law_ref": "law:dilation"}});
    let member = resolved_member(&single_member(
        material,
        element,
        json!({"kind": "temperature_reference", "installation_temperature": {"value": 50.0, "unit": "degC"}}),
    ));
    // 302 degF = 150 degC exactly; control 4 dilation values give 43/25009.
    close(
        member["thermal_strain"].as_f64().unwrap(),
        reference(
            "thermal_datum_ratio",
            "generic_reviewed",
            &["thermal_strain"],
        ),
        1.0,
    );
    let consumed = member["consumed_law_segments"].as_array().unwrap();
    assert!(consumed
        .iter()
        .all(|segment| segment["use"] == "integration_interval"));
}
