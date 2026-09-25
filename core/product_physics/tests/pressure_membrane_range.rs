//! Current public applicability control for the original PHYS-R4 source fixture.
//! The unchanged OD/wall/L/E/nu/p and fixed supports are outside the ordinary
//! structural transformation-allowance normal range. The historic public
//! red-02/green-01 proof remains preserved in PHYSICS_MANAGER/MEMBRANE_PUBLICATION.
//! Direct publisher/extrema coverage uses the same frozen independent Fraction
//! reference in the private membrane_publication_range module; it is not a
//! current public-solver success claim for these inputs.

use open_pipe_stress_product_physics::{
    run_linear_static_preview_with_mode, LinearStaticPreviewRequest, PreviewSolverMode,
};
use serde_json::{json, Value};

const CASE: &str = "case:source-section";
const REGION: &str = "region:source-section";
const PIPE: &str = "pipe:source-section";
const A: &str = "node:section-a";
const B: &str = "node:section-b";

fn anchor(id: &str, node: &str) -> Value {
    json!({"id":id,"node":node,"family":"anchor",
        "restraints":["UX","UY","UZ","RX","RY","RZ"],
        "provenance":"independent_section_geometry_control"})
}

fn fixture(inputs: &Value, fixed: bool, pressurized: bool) -> Value {
    let mut supports = vec![anchor("support:section-a", A)];
    if fixed {
        supports.push(anchor("support:section-b", B));
    }
    let mut loads = Vec::new();
    for (id, direction, dimension, unit, magnitude) in [
        (
            "load:tip-y",
            "global_y",
            "force",
            "N",
            inputs["tip_Fy_N"].as_f64().unwrap_or(0.0),
        ),
        (
            "load:tip-torque",
            "rotation_x",
            "moment",
            "N*m",
            inputs["tip_Mx_Nm"].as_f64().unwrap_or(0.0),
        ),
    ] {
        if magnitude != 0.0 {
            loads.push(json!({"id":id,"category":if dimension=="moment" {"concentrated_moment"} else {"concentrated_force"},
                "target":{"type":"node","node":B},"direction":direction,
                "dimension":dimension,"magnitude":{"value":magnitude,"unit":unit},
                "provenance":"independent_section_geometry_reference"}));
        }
    }
    let regions = if pressurized {
        vec![json!({"id":REGION,"member_pipe_ids":[PIPE],
            "pressure_basis":"internal_differential_zero_external_v1",
            "pressure":{"value":inputs["p_Pa"],"unit":"Pa"},
            "terminals":[
                {"node_ref":A,"closure_transfer":"transfers_to_wall","provenance":"explicit_test_closure"},
                {"node_ref":B,"closure_transfer":"transfers_to_wall","provenance":"explicit_test_closure"}],
            "provenance":"independent_geometry_pressure_reference"})]
    } else {
        vec![]
    };
    json!({"model":{
        "schema_version":"0.3.0","document_kind":"openpipestress.product_preview.model",
        "pressure_contract":{"version":"2.0.0","mode":"exact_straight_pressure_v2"},
        "project":{"id":"project:section-oracle","units":{"length":"m","force":"N","angle":"rad",
            "pressure":"Pa","stress":"Pa","temperature":"degC"}},
        "analysis_status":{"mechanics":"ready_for_preview_diagnostics",
            "rule_check":"not_performed_user_rule_inputs_missing","professional_acceptance":"not_provided"},
        "nodes":[{"id":A,"position":{"x":0.0,"y":0.0,"z":0.0},"provenance":"synthetic"},
            {"id":B,"position":{"x":inputs["L_m"],"y":0.0,"z":0.0},"provenance":"synthetic"}],
        "pipe_segments":[{"id":PIPE,"from":A,"to":B,"section":{
            "outside_diameter":{"value":inputs["OD_m"],"unit":"m"},
            "wall_thickness":{"value":inputs["wall_m"],"unit":"m"}},
            "material":"material:section","y_reference":{"x":0.0,"y":1.0,"z":0.0},
            "provenance":"arithmetic_geometry_control_not_manufactured_pipe"}],
        "materials":[{"id":"material:section","constitutive_basis":"homogeneous_isotropic_E_nu_v1",
            "elastic_modulus":{"value":inputs["E_Pa"],"unit":"Pa"},
            "poisson_ratio":{"value":inputs["nu"],"unit":"1"},
            "provenance":"synthetic_isotropic_input"}],
        "supports":supports,"components":[],
        "load_cases":[{"id":CASE,"primitive_loads":loads,"pressure_regions":regions,
            "provenance":"independent_reference"}],"combinations":[]
    },"materials":[]})
}

fn fixed_original_membrane_fixture_is_publicly_out_of_range(mode: PreviewSolverMode) {
    let input = fixture(
        &json!({"OD_m":4e-77,"wall_m":1e-77,"L_m":1.0,
        "E_Pa":1.0,"nu":0.1,"p_Pa":4.7e-170}),
        true,
        true,
    );
    let request: LinearStaticPreviewRequest = serde_json::from_value(input).unwrap();
    let result = run_linear_static_preview_with_mode(request, mode);
    assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE", "{:?}", result.diagnostics);
    assert!(!result.accepted_model_state_mutated);
    assert!(result.results.is_empty(), "a blocked solve must not publish mechanics rows");
    assert!(result.summary.max_open_formula_stress.is_none());
    assert!(result.summary.max_displacement.is_none());
    let matching: Vec<_> = result.diagnostics.iter().filter(|d| {
        d.code == "NUMERICAL_INTEGRITY_UNRESOLVED"
            && d.severity == "blocking"
            && d.affected_refs.iter().any(|r| r == CASE)
    }).collect();
    assert_eq!(matching.len(), 1, "{:?}", result.diagnostics);
    assert!(matching[0].message.contains("Range(\"arithmetic outside normal range\")"),
        "{:?}", matching[0]);
}

#[test]
fn original_subnormal_membrane_fixture_is_publicly_out_of_range_sparse() {
    fixed_original_membrane_fixture_is_publicly_out_of_range(PreviewSolverMode::SparseInteractive);
}

#[test]
fn original_subnormal_membrane_fixture_is_publicly_out_of_range_dense() {
    fixed_original_membrane_fixture_is_publicly_out_of_range(PreviewSolverMode::DenseScrutiny);
}
