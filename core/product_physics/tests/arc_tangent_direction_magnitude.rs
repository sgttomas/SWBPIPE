//! DEL-04-07 Work `pressure-contract.md`, "Direction magnitudes of arc
//! tangents": a realized arc's end tangent t = c*d + s*n is formed without
//! renormalizing, so a component can round above 1. This is the RT-U4
//! linear-ex rack's 90-degree bend (#31's generator, `--profile linear-ex
//! --nodes 82`, pipe RK-H0022: chord (0.405, 0.405, 0) m, R = 405 mm, y
//! reference +y) inside one pressurized region between two straights, in both
//! solver modes. Its pressure groups at the bend's end publish the direction
//! magnitude 1 + 2^-52 (bits 3ff0000000000001), which the readers refused
//! (`SOURCE_PHYSICS_RHS_DIRECTION`) while their bound was 1. The readers now
//! admit a group with a term on a realized arc up to 1 + 16u and keep 1 for a
//! group on straight members only.
//!
//! Invented data only.

use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode, MODEL_SCHEMA_VERSION};
use open_pipe_stress_result_export::{reader_verdict, semantic_contract};
use serde_json::{json, Value};

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const PROVENANCE: &str = "invented_arc_tangent_direction_control";
const ONE_PLUS_ULP: u64 = 0x3ff0_0000_0000_0001;

fn q(value: f64, unit: &str) -> Value {
    json!({"value": value, "unit": unit})
}

fn node(id: &str, x: f64, y: f64, z: f64) -> Value {
    json!({"id": id, "position": {"x": x, "y": y, "z": z}, "provenance": PROVENANCE})
}

fn pipe(id: &str, from: &str, to: &str, y: [f64; 3]) -> Value {
    json!({"id": id, "from": from, "to": to, "material": "material:steel",
        "section": {"outside_diameter": q(270.0, "mm"), "wall_thickness": q(9.0, "mm"),
            "mill_tolerance": q(1.0, "mm"), "corrosion_allowance": q(1.5, "mm")},
        "y_reference": {"x": y[0], "y": y[1], "z": y[2]}, "provenance": PROVENANCE})
}

fn document() -> Value {
    json!({"model": {"schema_version": MODEL_SCHEMA_VERSION, "document_kind": "openpipestress.product_preview.model",
        "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"},
        "project": {"id": "project:arc-tangent", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": [node("node:A", 25.905, 3.905, 6.0), node("node:B", 25.905, 4.905, 6.0), node("node:C", 26.31, 5.31, 6.0), node("node:D", 27.31, 5.31, 6.0)],
        "pipe_segments": [pipe("pipe:AB", "node:A", "node:B", [0.0, 0.0, 1.0]), pipe("pipe:BC", "node:B", "node:C", [0.0, 1.0, 0.0]), pipe("pipe:CD", "node:C", "node:D", [0.0, 0.0, 1.0])],
        "materials": [{"id": "material:steel", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
            "elastic_modulus": q(2.0e11, "Pa"), "poisson_ratio": q(0.3, "1"), "provenance": PROVENANCE}],
        "supports": [{"id": "support:A", "node": "node:A", "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROVENANCE}],
        "components": [{"id": "component:bend", "kind": "bend", "node": "node:B",
            "geometry": {"bend_pipe_ref": "pipe:BC", "bend_radius": q(405.0, "mm"), "bend_geometry_source_reference": "invented"},
            "modifiers": {"flexibility_factor_user_value": q(5.0, "none"), "source_reference": "invented"},
            "mechanics_interface": {"solver_consumption": "curved_bend_macro_element"}, "provenance": PROVENANCE}],
        "load_cases": [{"id": "case:pressure", "primitive_loads": [], "provenance": PROVENANCE,
            "pressure_regions": [{"id": "region:line", "member_pipe_ids": ["pipe:AB", "pipe:BC", "pipe:CD"],
                "pressure_basis": "internal_differential_zero_external_v1", "pressure": q(1.6e6, "Pa"),
                "terminals": [{"node_ref": "node:A", "closure_transfer": "transfers_to_wall", "provenance": PROVENANCE},
                    {"node_ref": "node:D", "closure_transfer": "transfers_to_wall", "provenance": PROVENANCE}],
                "provenance": PROVENANCE}]}],
        "combinations": []},
        "materials": []})
}

fn groups(envelope: &Value) -> &Vec<Value> {
    envelope["contract_evidence"]["exact_cases"][0]["pressure_rhs_assembly"]["groups"].as_array().unwrap()
}

#[test]
fn an_arc_tangent_component_above_one_is_published_and_read() {
    let document = document();
    for mode in MODES {
        let envelope = serde_json::to_value(run_linear_static_preview_value_with_mode(document.clone(), mode).unwrap()).unwrap();
        assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}: {:#?}", envelope["diagnostics"]);
        let above = groups(&envelope).iter()
            .filter(|g| g["direction_component_magnitude"].as_f64().unwrap().to_bits() == ONE_PLUS_ULP)
            .collect::<Vec<_>>();
        assert!(!above.is_empty(), "{mode:?}: the rack bend's tangent component rounds to 1 + 2^-52");
        assert!(above.iter().all(|g| g["terms"].as_array().unwrap().iter().any(|t| t["pipe_id"] == "pipe:BC")), "{mode:?}: only arc groups");
        semantic_contract::validate_pressure_evidence(&envelope).unwrap_or_else(|e| panic!("{mode:?}: {e}"));
        assert_eq!(reader_verdict::validate("pressure", &envelope, Some(&document["model"]), None), json!({"ok": true}), "{mode:?}");

        // The bound is not loosened beyond the arc construction, nor on straights.
        let mut beyond = envelope.clone();
        let index = groups(&envelope).iter().position(|g| g["direction_component_magnitude"].as_f64().unwrap().to_bits() == ONE_PLUS_ULP).unwrap();
        beyond["contract_evidence"]["exact_cases"][0]["pressure_rhs_assembly"]["groups"][index]["direction_component_magnitude"] = json!(1.0 + 16.0 * f64::EPSILON);
        assert_eq!(semantic_contract::validate_pressure_evidence(&beyond).unwrap_err(), "SOURCE_PHYSICS_RHS_DIRECTION", "{mode:?}");
        let straight = groups(&envelope).iter().position(|g| g["terms"].as_array().unwrap().iter().all(|t| t["pipe_id"] != "pipe:BC")).unwrap();
        let mut over = envelope.clone();
        over["contract_evidence"]["exact_cases"][0]["pressure_rhs_assembly"]["groups"][straight]["direction_component_magnitude"] = json!(f64::from_bits(ONE_PLUS_ULP));
        assert_eq!(semantic_contract::validate_pressure_evidence(&over).unwrap_err(), "SOURCE_PHYSICS_RHS_DIRECTION", "{mode:?}");
    }
}
