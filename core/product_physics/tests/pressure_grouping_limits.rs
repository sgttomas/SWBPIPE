//! Manager public control for explicit unresolved source-factor cancellation.
//! This tests the declared admission guard; it does not add a physical-response oracle.

use open_pipe_stress_product_physics::{
    run_linear_static_preview_with_mode, LinearStaticPreviewRequest, MechanicsEnvelope,
    PreviewSolverMode,
};
use serde_json::{json, Value};

const CASE: &str = "case:pressure-oracle";
const REGION: &str = "region:pressure-oracle";
const PIPE: &str = "pipe:A-B";
const ROOT: &str = "node:A";
const TIP: &str = "node:B";
const ROOT_SUPPORT: &str = "support:A";
const TIP_SUPPORT: &str = "support:B";
const LENGTH: f64 = 6.0;
const MODES: [PreviewSolverMode; 2] = [
    PreviewSolverMode::SparseInteractive,
    PreviewSolverMode::DenseScrutiny,
];

fn node(id: &str, position: [f64; 3]) -> Value {
    json!({"id":id,"position":{"x":position[0],"y":position[1],"z":position[2]},
        "provenance":"independent_pressure_runtime_oracle"})
}

fn anchor(id: &str, node_id: &str) -> Value {
    json!({"id":id,"node":node_id,"family":"anchor",
        "restraints":["UX","UY","UZ","RX","RY","RZ"],
        "provenance":"independent_pressure_runtime_oracle"})
}

fn terminal(node_id: &str, transfer: bool) -> Value {
    json!({"node_ref":node_id,"closure_transfer":if transfer {"transfers_to_wall"}
        else {"separately_supported_or_compensated"},
        "provenance":"explicit_independent_pressure_boundary"})
}

fn material(id: &str, e: f64, nu: f64) -> Value {
    json!({"id":id,"constitutive_basis":"homogeneous_isotropic_E_nu_v1",
        "elastic_modulus":{"value":e,"unit":"Pa"},
        "poisson_ratio":{"value":nu,"unit":"1"},
        "thermal_expansion_coefficient":{"value":0.000012,"unit":"1/degC"},
        "temperature_points":[],"provenance":"synthetic_isotropic_reference_no_catalog"})
}

fn pipe(id: &str, from: &str, to: &str, outside: f64, wall: f64, material_id: &str) -> Value {
    json!({"id":id,"from":from,"to":to,
        "section":{"outside_diameter":{"value":outside,"unit":"m"},
            "wall_thickness":{"value":wall,"unit":"m"}},
        "material":material_id,"y_reference":{"x":0.0,"y":0.0,"z":1.0},
        "provenance":"synthetic_straight_annulus"})
}

fn model(fixed: bool, transfer: bool, thermal: f64) -> Value {
    let mut supports = vec![anchor(ROOT_SUPPORT, ROOT)];
    if fixed {
        supports.push(anchor(TIP_SUPPORT, TIP));
    }
    let loads = if thermal == 0.0 {
        vec![]
    } else {
        vec![json!({"id":"load:thermal","category":"thermal",
            "target":{"type":"element","pipe":PIPE},"direction":"global_x",
            "magnitude":{"value":thermal/0.000012,"unit":"degC"},
            "dimension":"temperature_interval","provenance":"independent_thermal_strain"})]
    };
    json!({"model":{
        "schema_version":"0.3.0","document_kind":"openpipestress.product_preview.model",
        "pressure_contract":{"version":"2.0.0","mode":"exact_straight_pressure_v2"},
        "project":{"id":"project:pressure-oracle","units":{"length":"m","force":"N",
            "angle":"rad","pressure":"Pa","stress":"Pa","temperature":"degC"}},
        "analysis_status":{"mechanics":"ready_for_preview_diagnostics",
            "rule_check":"not_performed_user_rule_inputs_missing","professional_acceptance":"not_provided"},
        "nodes":[node(ROOT,[0.0,0.0,0.0]),node(TIP,[LENGTH,0.0,0.0])],
        "pipe_segments":[pipe(PIPE,ROOT,TIP,0.12,0.01,"material:one")],
        "supports":supports,"components":[],
        "materials":[material("material:one",200e9,0.3)],
        "load_cases":[{"id":CASE,"primitive_loads":loads,"pressure_regions":[{
            "id":REGION,"member_pipe_ids":[PIPE],
            "pressure_basis":"internal_differential_zero_external_v1",
            "pressure":{"value":2e6,"unit":"Pa"},
            "terminals":[terminal(ROOT,transfer),terminal(TIP,transfer)],
            "provenance":"independent_pressure_region"}],
            "provenance":"synthetic_load_case"}],"combinations":[]
    },"materials":[]})
}

fn solve(input: Value, mode: PreviewSolverMode) -> MechanicsEnvelope {
    let request: LinearStaticPreviewRequest = serde_json::from_value(input)
        .expect("the independently authored public DTO must deserialize");
    run_linear_static_preview_with_mode(request, mode)
}

#[test]
fn strongly_cancelling_distinct_pressure_factors_are_explicitly_unqualified() {
    let mut input = model(false, true, 0.0);
    let second_pipe = pipe("pipe:parallel", ROOT, TIP, 0.16, 0.02, "material:one");
    input["model"]["pipe_segments"]
        .as_array_mut()
        .unwrap()
        .push(second_pipe);
    let mut second = input["model"]["load_cases"][0]["pressure_regions"][0].clone();
    second["id"] = json!("region:other-source");
    second["member_pipe_ids"] = json!(["pipe:parallel"]);
    second["pressure"]["value"] = json!(-2e6 * 25.0 / 36.0 * (1.0 + 1e-13));
    input["model"]["load_cases"][0]["pressure_regions"]
        .as_array_mut()
        .unwrap()
        .push(second);
    for mode in MODES {
        let output = solve(input.clone(), mode);
        assert_eq!(output.status.mechanics, "MODEL_INCOMPLETE");
        assert!(output.results.is_empty());
        assert!(
            output
                .diagnostics
                .iter()
                .any(|d| d.code == "PRESSURE_ASSEMBLY_CANCELLATION_UNRESOLVED"),
            "{:?}",
            output.diagnostics
        );
    }
}
