//! Silent-zero guard (DEL-04-07 `Design/pressure-contract.md`, "Load cases that
//! apply nothing"): a load case that applies no action is refused by name and
//! never published as an all-zero result as if its loads had been solved. On
//! main `9ad5d79fa5` the models of the refusal and warning tests solved and
//! published with no diagnostic (all-zero rows marked
//! NUMERICAL_INTEGRITY_CHECKS_PASSED for the refused ones). Every model goes through the desktop's value entry, in both
//! solver modes.
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];

fn node(id: &str, x: f64) -> Value {
    json!({"id": id, "position": {"x": x, "y": 0, "z": 0}, "provenance": "invented"})
}
fn pipe(exact: bool) -> Value {
    let mut section = json!({"outside_diameter": {"value": 0.2, "unit": "m"}, "wall_thickness": {"value": 0.01, "unit": "m"}});
    if exact {
        section["corrosion_allowance"] = json!({"value": 0.0, "unit": "m"});
    }
    json!({"id": "pipe:1", "from": "node:a", "to": "node:b", "section": section,
        "material": "material:shared", "y_reference": {"x": 0, "y": 1, "z": 0}, "provenance": "invented"})
}
fn anchor(id: &str, node: &str) -> Value {
    json!({"id": id, "node": node, "family": "anchor",
        "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": "invented"})
}
fn force(id: &str, value: f64) -> Value {
    json!({"id": id, "category": "concentrated_force", "target": {"type": "node", "node": "node:b"},
        "direction": "global_x", "magnitude": {"value": value, "unit": "N"}, "dimension": "force", "provenance": "invented"})
}
fn material() -> Value {
    json!({"id": "material:shared", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
        "elastic_modulus": {"value": 200, "unit": "GPa"}, "poisson_ratio": {"value": 0.3, "unit": "1"},
        "shear_modulus": {"value": 76.923076923, "unit": "GPa"},
        "provenance": "invented explicit test data",
        "temperature_points": [
            {"id": "point:stiff", "temperature": {"value": 20, "unit": "degC"},
             "elastic_modulus": {"value": 200, "unit": "GPa"}, "poisson_ratio": {"value": 0.3, "unit": "1"}}]})
}
fn exact_contract() -> Value {
    json!({"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"})
}
fn base(supports: Vec<Value>, cases: Vec<Value>, exact: bool, load_state: bool) -> Value {
    let mut model = json!({
        "schema_version": "0.6.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:silent-zero", "units": {"length": "m"}},
        "analysis_status": {"mechanics": "ready", "rule_check": "not_performed", "professional_acceptance": "not_provided"},
        "nodes": [node("node:a", 0.0), node("node:b", 1.0)], "pipe_segments": [pipe(exact)], "supports": supports,
        "components": [], "materials": [material()], "load_cases": cases, "combinations": []});
    if exact {
        model["pressure_contract"] = exact_contract();
    }
    if load_state {
        model["reference_configurations"] = json!([{"id": "reference:installed",
            "geometry_ref": {"kind": "authored_model_geometry"},
            "member_references": [{"pipe_ref": "pipe:1", "basis": {"kind": "direct_strain_reference"},
                "fit": {"kind": "none"}, "provenance": "invented"}],
            "provenance": "invented"}]);
    }
    json!({"model": model, "materials": []})
}
/// A load-state case on the anchored cantilever a-b.
fn state_case(primitives: Vec<Value>, sources: &[&str], support_states: Vec<Value>) -> Value {
    json!({"id": "case:state", "primitive_loads": primitives, "pressure_regions": [], "provenance": "invented",
        "analysis_state": {"contract": "openpipestress.load_reference_state/1.0.0",
            "reference_configuration_ref": "reference:installed",
            "element_states": [{"pipe_ref": "pipe:1",
                "material_selection": {"kind": "exact_point", "material_ref": "material:shared", "point_ref": "point:stiff"},
                "thermal_state": {"kind": "unchanged_reference", "provenance": "invented"}}],
            "support_states": support_states,
            "load_sources": sources.iter().map(|s| json!({"source_ref": s, "factor": 1.0})).collect::<Vec<_>>(),
            "history": {"kind": "independent_equilibrium"}, "provenance": "invented"}})
}
fn active(support: &str) -> Value {
    json!({"support_ref": support, "participation": {"kind": "active_model_device"}})
}

fn envelope(wire: &Value, mode: PreviewSolverMode) -> Value {
    serde_json::to_value(run_linear_static_preview_value_with_mode(wire.clone(), mode).expect("captured entry")).unwrap()
}
fn diagnostics(v: &Value) -> Vec<Value> {
    v["diagnostics"].as_array().cloned().unwrap_or_default()
}
fn with_code(v: &Value, code: &str) -> Vec<Value> {
    diagnostics(v).into_iter().filter(|d| d["code"] == code).collect()
}
fn blocking(v: &Value) -> Vec<String> {
    diagnostics(v)
        .into_iter()
        .filter(|d| d["severity"] == "blocking")
        .map(|d| d["code"].as_str().unwrap().to_string())
        .collect()
}
fn results(v: &Value) -> usize {
    v["results"].as_array().map_or(0, Vec::len)
}
fn reaction_fx(v: &Value, support: &str) -> f64 {
    v["results"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["kind"] == "support_reaction_component_v2" && r["entity_ref"] == support && r["metadata"]["component"] == "Fx")
        .unwrap_or_else(|| panic!("no Fx reaction at {support}"))["value"]
        .as_f64()
        .unwrap()
}

/// The reported defect: the case stores a force but lists no load source, so
/// nothing is applied. It is refused by name, with no published rows.
#[test]
fn a_load_state_case_whose_primitives_are_all_unlisted_is_refused() {
    let wire = base(
        vec![anchor("support:root", "node:a")],
        vec![state_case(vec![force("load:pull", 1000.0)], &[], vec![active("support:root")])],
        true,
        true,
    );
    for mode in MODES {
        let v = envelope(&wire, mode);
        assert_eq!(blocking(&v), ["LOAD_STATE_CASE_APPLIES_NO_ACTION"], "{:?}", diagnostics(&v));
        let found = with_code(&v, "LOAD_STATE_CASE_APPLIES_NO_ACTION");
        assert_eq!(found[0]["severity"], "blocking");
        assert_eq!(found[0]["affected_refs"], json!(["case:state"]));
        let message = found[0]["message"].as_str().unwrap();
        assert!(message.contains("none of its 1 stored primitive load (load:pull) is listed in analysis_state.load_sources"), "{message}");
        assert_eq!(results(&v), 0, "no all-zero result is published");
    }
}

/// Partial exclusion stays legal and solves, and is now visible.
#[test]
fn an_unlisted_primitive_beside_a_listed_one_warns_and_is_not_applied() {
    let wire = base(
        vec![anchor("support:root", "node:a")],
        vec![state_case(
            vec![force("load:pull", 1000.0), force("load:unused", 999.0)],
            &["load:pull"],
            vec![active("support:root")],
        )],
        true,
        true,
    );
    for mode in MODES {
        let v = envelope(&wire, mode);
        assert!(blocking(&v).is_empty(), "{:?}", diagnostics(&v));
        let found = with_code(&v, "LOAD_STATE_SOURCE_EXCLUDED");
        assert_eq!(found.len(), 1, "{:?}", diagnostics(&v));
        assert_eq!(found[0]["severity"], "warning");
        assert_eq!(found[0]["affected_refs"], json!(["case:state", "load:unused"]));
        assert!(found[0]["message"].as_str().unwrap().ends_with("is not applied: load:unused"));
        // Only the listed 1000 N is applied: the support resists it.
        assert_eq!(reaction_fx(&v, "support:root"), -1000.0);
    }
}

/// A case whose only action is a boundary motion applies an action.
#[test]
fn a_load_state_case_with_only_boundary_motion_solves() {
    let moved = json!({"support_ref": "support:far", "participation": {"kind": "active_model_device"},
        "boundary_motion": [{"dof": "UX", "value": {"value": 0.001, "unit": "m"}, "meaning": "absolute_reference_displacement"}]});
    let wire = base(
        vec![anchor("support:root", "node:a"), anchor("support:far", "node:b")],
        vec![state_case(vec![], &[], vec![active("support:root"), moved])],
        true,
        true,
    );
    for mode in MODES {
        let v = envelope(&wire, mode);
        assert!(blocking(&v).is_empty(), "{:?}", diagnostics(&v));
        assert!(with_code(&v, "LOAD_STATE_CASE_APPLIES_NO_ACTION").is_empty());
        assert!(reaction_fx(&v, "support:root") != 0.0);
    }
}

/// The same pattern on the pressure-free route: one empty case among loaded
/// ones publishes an all-zero result. It may be deliberate (a cold operand of
/// a range), so it is reported by name, not refused.
#[test]
fn an_empty_case_beside_a_loaded_case_is_reported_on_the_preview_route() {
    let wire = base(
        vec![anchor("support:root", "node:a")],
        vec![
            json!({"id": "case:loaded", "primitive_loads": [force("load:pull", 1000.0)], "provenance": "invented"}),
            json!({"id": "case:empty", "primitive_loads": [], "provenance": "invented"}),
        ],
        false,
        false,
    );
    for mode in MODES {
        let v = envelope(&wire, mode);
        assert!(blocking(&v).is_empty(), "{:?}", diagnostics(&v));
        let found = with_code(&v, "LOAD_CASE_APPLIES_NO_ACTION");
        assert_eq!(found.len(), 1, "{:?}", diagnostics(&v));
        assert_eq!(found[0]["severity"], "warning");
        assert_eq!(found[0]["affected_refs"], json!(["case:empty"]));
        assert!(with_code(&v, "LOAD_INPUT_MISSING").is_empty());
    }
}

/// A load-state case with no stored primitive and no action is a reference
/// state (the cold state of a cold/hot sequence): its zero result is correct
/// and it is neither refused nor reported, beside a case that applies a load.
#[test]
fn a_load_state_reference_case_beside_a_loaded_case_solves_to_zero() {
    let mut cold = state_case(vec![], &[], vec![active("support:root")]);
    cold["id"] = json!("case:cold");
    let mut hot = state_case(vec![force("load:pull", 1000.0)], &["load:pull"], vec![active("support:root")]);
    hot["id"] = json!("case:hot");
    let wire = base(vec![anchor("support:root", "node:a")], vec![cold, hot], true, true);
    for mode in MODES {
        let v = envelope(&wire, mode);
        assert!(blocking(&v).is_empty(), "{:?}", diagnostics(&v));
        assert!(with_code(&v, "LOAD_STATE_CASE_APPLIES_NO_ACTION").is_empty());
        assert!(with_code(&v, "LOAD_STATE_SOURCE_EXCLUDED").is_empty());
        // Exact-route rows are not case-qualified: one Fx reaction per case.
        let mut fx = v["results"].as_array().unwrap().iter()
            .filter(|r| r["kind"] == "support_reaction_component_v2" && r["entity_ref"] == "support:root"
                && r["metadata"]["component"] == "Fx")
            .map(|r| r["value"].as_f64().unwrap())
            .collect::<Vec<_>>();
        fx.sort_by(f64::total_cmp);
        assert_eq!(fx, [-1000.0, 0.0]);
    }
}

/// A load-state model whose every case is a reference state publishes only
/// zeros; it is refused as an all-empty model is on the other routes.
#[test]
fn a_load_state_model_of_reference_states_only_is_refused() {
    let wire = base(
        vec![anchor("support:root", "node:a")],
        vec![state_case(vec![], &[], vec![active("support:root")])],
        true,
        true,
    );
    for mode in MODES {
        let v = envelope(&wire, mode);
        assert_eq!(blocking(&v), ["LOAD_INPUT_MISSING"], "{:?}", diagnostics(&v));
        assert_eq!(results(&v), 0);
    }
}

/// An objective connector loads every case only through a preload: an
/// untied connector with zero `q_ref` does not exempt an all-empty exact
/// model; the same connector with a nonzero `q_ref` (prestressed) does.
#[test]
fn only_a_preloaded_connector_loads_every_case() {
    let stress_free: Value = serde_json::from_str(include_str!(
        "fixtures/silent_zero/stress_free_connector_empty.request.json"
    ))
    .unwrap();
    let index = stress_free["model"]["components"]
        .as_array()
        .unwrap()
        .iter()
        .position(|c| c.get("objective_connector").is_some())
        .unwrap();
    assert_eq!(stress_free["model"]["components"][index]["objective_connector"]["reference_state"], "stress_free");
    let mut prestressed = stress_free.clone();
    let oc = &mut prestressed["model"]["components"][index]["objective_connector"];
    oc["reference_state"] = json!("prestressed");
    oc["q_ref"]["translation"]["x"] = json!(0.001);
    for mode in MODES {
        let v = envelope(&stress_free, mode);
        assert_eq!(blocking(&v), ["LOAD_INPUT_MISSING"], "{:?}", diagnostics(&v));
        let v = envelope(&prestressed, mode);
        assert!(blocking(&v).is_empty(), "{:?}", diagnostics(&v));
        assert!(with_code(&v, "LOAD_CASE_APPLIES_NO_ACTION").is_empty());
    }
}

/// Control: the loaded case alone solves on the same model.
#[test]
fn the_loaded_case_alone_solves_on_the_preview_route() {
    let wire = base(
        vec![anchor("support:root", "node:a")],
        vec![json!({"id": "case:loaded", "primitive_loads": [force("load:pull", 1000.0)], "provenance": "invented"})],
        false,
        false,
    );
    for mode in MODES {
        let v = envelope(&wire, mode);
        assert!(blocking(&v).is_empty(), "{:?}", diagnostics(&v));
        assert_eq!(reaction_fx(&v, "support:root"), -1000.0);
    }
}

/// On the exact route an explicit `pressure_regions: []` means unpressurized
/// and applies nothing.
#[test]
fn an_exact_case_with_only_an_empty_pressure_region_list_applies_nothing() {
    let wire = base(
        vec![anchor("support:root", "node:a")],
        vec![json!({"id": "case:unpressurized", "primitive_loads": [], "pressure_regions": [], "provenance": "invented"})],
        true,
        false,
    );
    for mode in MODES {
        let v = envelope(&wire, mode);
        assert_eq!(blocking(&v), ["LOAD_INPUT_MISSING"], "{:?}", diagnostics(&v));
        assert_eq!(results(&v), 0);
    }
}

/// A load-state case whose stored primitive is unlisted but whose connector is
/// prestressed applies an action (the preload): it solves, with the excluded
/// primitive reported (independent review's model, from the T4-I12 demo).
#[test]
fn a_load_state_case_with_a_prestressed_connector_applies_its_preload() {
    let wire: Value = serde_json::from_str(include_str!(
        "fixtures/silent_zero/load_state_prestressed_connector_unlisted.request.json"
    ))
    .unwrap();
    for mode in MODES {
        let v = envelope(&wire, mode);
        assert!(blocking(&v).is_empty(), "{:?}", diagnostics(&v));
        assert_eq!(with_code(&v, "LOAD_STATE_SOURCE_EXCLUDED").len(), 1, "{:?}", diagnostics(&v));
        let moved = v["results"].as_array().unwrap().iter().any(|r| {
            r["kind"] == "global_nodal_displacement_x" && r["value"].as_f64().is_some_and(|x| x != 0.0)
        });
        assert!(moved, "the preload moves the structure");
    }
}

/// A connector whose K is positive semidefinite (rx stiffness zero) and whose
/// q_ref lies in K's null space (a 0.01 rad rx reference) applies no load: it
/// is not a preload (independent review's case). `node:b` gains an RX restraint
/// so the system stays nonsingular.
fn null_space_q_ref(mut wire: Value) -> Value {
    let index = wire["model"]["components"].as_array().unwrap().iter()
        .position(|c| c.get("objective_connector").is_some()).unwrap();
    let oc = &mut wire["model"]["components"][index]["objective_connector"];
    oc["stiffness"]["upper_triangle"][15] = json!(0);
    oc["reference_state"] = json!("stress_free");
    oc["q_ref"]["translation"]["x"] = json!(0);
    oc["q_ref"]["rotation"]["x"] = json!(0.01);
    wire["model"]["supports"].as_array_mut().unwrap()
        .push(json!({"id": "support:b-rx", "node": "node:b", "family": "anchor", "restraints": ["RX"], "provenance": "invented"}));
    if let Some(cases) = wire["model"]["load_cases"].as_array_mut() {
        for case in cases {
            if let Some(states) = case
                .get_mut("analysis_state")
                .and_then(|state| state.get_mut("support_states"))
                .and_then(Value::as_array_mut)
            {
                states.push(json!({"support_ref": "support:b-rx", "participation": {"kind": "active_model_device"}}));
            }
        }
    }
    wire
}

#[test]
fn a_null_space_reference_is_not_a_preload() {
    let exact: Value = serde_json::from_str(include_str!("fixtures/silent_zero/stress_free_connector_empty.request.json")).unwrap();
    let exact = null_space_q_ref(exact);
    let unlisted = null_space_q_ref(serde_json::from_str(include_str!(
        "fixtures/silent_zero/load_state_prestressed_connector_unlisted.request.json")).unwrap());
    let mut reference_only = unlisted.clone();
    reference_only["model"]["load_cases"][0]["primitive_loads"] = json!([]);
    // Control: the same exact model with a load solves.
    let mut loaded = exact.clone();
    loaded["model"]["load_cases"][0]["primitive_loads"] = json!([{"id": "load:pull", "category": "concentrated_force",
        "target": {"type": "node", "node": "node:b"}, "direction": "global_x", "magnitude": {"value": 1000.0, "unit": "N"},
        "dimension": "force", "provenance": "invented"}]);
    for mode in MODES {
        let v = envelope(&exact, mode);
        assert_eq!(blocking(&v), ["LOAD_INPUT_MISSING"], "{:?}", diagnostics(&v));
        let v = envelope(&unlisted, mode);
        assert_eq!(blocking(&v), ["LOAD_STATE_CASE_APPLIES_NO_ACTION"], "{:?}", diagnostics(&v));
        let v = envelope(&reference_only, mode);
        assert_eq!(blocking(&v), ["LOAD_INPUT_MISSING"], "{:?}", diagnostics(&v));
        let v = envelope(&loaded, mode);
        assert!(blocking(&v).is_empty(), "{:?}", diagnostics(&v));
    }
}
