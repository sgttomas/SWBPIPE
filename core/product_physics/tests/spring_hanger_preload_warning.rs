//! §8.30 correction 3 (RT-S3): a variable spring hanger's installed, cold and
//! hot loads are authored and parsed, but only its stiffness is consumed. Each
//! non-zero entered load is named by a warning,
//! `SPRING_HANGER_LOAD_NOT_APPLIED_AS_PRELOAD`, per support and field: the
//! load is not applied as preload, and results treat the hanger as a linear
//! spring from zero load. The solve itself is unchanged. Invented inputs only.

use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};

const CODE: &str = "SPRING_HANGER_LOAD_NOT_APPLIED_AS_PRELOAD";
const HANGER: &str = "support:SH";
const PROV: &str = "invented_830_hanger_preload_input_no_library_data";
const EXACT: &str = include_str!("fixtures/shared_terminal_junction.request.json");
const MODES: [PreviewSolverMode; 2] = [
    PreviewSolverMode::SparseInteractive,
    PreviewSolverMode::DenseScrutiny,
];

fn q(value: f64, unit: &str) -> Value {
    json!({"value": value, "unit": unit})
}

fn hanger(node: &str) -> Value {
    json!({"id": HANGER, "node": node, "family": "variable_spring_hanger", "restraints": [],
        "hanger": {"hanger_type": "variable_spring_hanger",
                   "stiffness": {"dof": "UZ", "value": q(42_000.0, "N/m")},
                   "installed_load": q(460.0, "N"), "cold_load": q(430.0, "N"), "hot_load": q(390.0, "N"),
                   "travel_range": q(0.045, "m"),
                   "manufacturer_reference": "invented_user_entered_variable_hanger_reference_no_catalog",
                   "source_reference": "invented_user_entered_spring_hanger_values_no_catalog",
                   "load_side_review_reference": "invented_review_only_hot_cold_load_metadata",
                   "mechanics_consumption": "linear_spring_primitive_user_stiffness"},
        "provenance": PROV})
}

/// A pressure-free two-member cantilever along x, anchored at N0, with the
/// hanger at its tip and a downward tip load.
fn pressure_free() -> Value {
    let nodes: Vec<Value> = (0..=2)
        .map(|i| json!({"id": format!("N{i}"), "position": {"x": i as f64, "y": 0.0, "z": 0.0}, "provenance": PROV}))
        .collect();
    let pipes: Vec<Value> = (1..=2)
        .map(|i| {
            json!({"id": format!("M{i}"), "from": format!("N{}", i - 1), "to": format!("N{i}"), "material": "mat:H",
                   "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0},
                   "section": {"outside_diameter": q(0.2, "m"), "wall_thickness": q(0.01, "m")},
                   "provenance": PROV})
        })
        .collect();
    json!({"model": {
        "schema_version": "0.8.0", "document_kind": "openpipestress.product_preview.model",
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
                            "rule_check": "not_performed_user_rule_inputs_missing",
                            "professional_acceptance": "not_provided"},
        "project": {"id": "invented:830:hanger-preload",
                    "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa",
                              "temperature": "degC", "stress": "Pa"}},
        "nodes": nodes,
        "pipe_segments": pipes,
        "materials": [{"id": "mat:H", "elastic_modulus": q(2.0e11, "Pa"),
                       "shear_modulus": q(8.0e10, "Pa"), "provenance": PROV}],
        "supports": [
            {"id": "anchor:N0", "node": "N0", "family": "anchor",
             "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV},
            hanger("N2")],
        "load_cases": [{"id": "case", "label": "tip", "kind": "primitive_user_load",
                        "primitive_loads": [{"id": "load:tip", "category": "concentrated_force",
                            "target": {"type": "node", "node": "N2"},
                            "direction": "global_z", "magnitude": q(-500.0, "N"),
                            "dimension": "force", "provenance": PROV}],
                        "provenance": PROV}],
        "combinations": []
    }, "materials": []})
}

/// The exact-route tee of correction 1, every terminal transferring, with the
/// hanger at the branch end.
fn exact() -> Value {
    let mut document: Value = serde_json::from_str(EXACT).unwrap();
    for region in document["model"]["load_cases"][0]["pressure_regions"].as_array_mut().unwrap() {
        for terminal in region["terminals"].as_array_mut().unwrap() {
            terminal["closure_transfer"] = json!("transfers_to_wall");
        }
    }
    document["model"]["supports"].as_array_mut().unwrap().push(hanger("node:C"));
    document
}

fn message(name: &str, field: &str) -> String {
    format!("variable spring hanger {HANGER}: its {name} ({field}) is not applied as preload; results treat the hanger as a linear spring from zero load, whose force is its stiffness times the displacement at its DOF, and the entered load remains review evidence only")
}

fn warned(document: &Value, name: &str, field: &str) {
    for mode in MODES {
        let envelope = serde_json::to_value(
            run_linear_static_preview_value_with_mode(document.clone(), mode).unwrap(),
        )
        .unwrap();
        assert_eq!(
            envelope["status"]["mechanics"], "MECHANICS_SOLVED",
            "{mode:?}: {:#?}",
            envelope["diagnostics"]
        );
        let diagnostics = envelope["diagnostics"].as_array().unwrap();
        let named = diagnostics
            .iter()
            .filter(|d| d["code"] == CODE && d["affected_refs"] == json!([HANGER, field]))
            .collect::<Vec<_>>();
        assert_eq!(named.len(), 1, "{mode:?}: {diagnostics:#?}");
        assert_eq!(named[0]["severity"], "warning");
        assert_eq!(
            named[0]["id"],
            format!("diagnostic:spring-hanger:support-SH:{field}")
        );
        assert_eq!(named[0]["message"], message(name, field));
        // One warning per entered load, no more.
        assert_eq!(diagnostics.iter().filter(|d| d["code"] == CODE).count(), 3, "{mode:?}");
    }
}

#[test]
fn an_installed_load_is_named_as_not_applied_as_preload() {
    warned(&pressure_free(), "installed load", "hanger.installed_load");
}

#[test]
fn a_cold_load_is_named_as_not_applied_as_preload() {
    warned(&pressure_free(), "cold load", "hanger.cold_load");
}

#[test]
fn a_hot_load_is_named_as_not_applied_as_preload() {
    warned(&pressure_free(), "hot load", "hanger.hot_load");
}

#[test]
fn the_exact_route_names_each_load_too() {
    let document = exact();
    for (name, field) in [
        ("installed load", "hanger.installed_load"),
        ("cold load", "hanger.cold_load"),
        ("hot load", "hanger.hot_load"),
    ] {
        warned(&document, name, field);
    }
}
