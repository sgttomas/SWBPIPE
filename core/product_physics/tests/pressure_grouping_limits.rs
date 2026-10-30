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
            "wall_thickness":{"value":wall,"unit":"m"},
            "corrosion_allowance":{"value":0.0,"unit":"m"}},
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
        "schema_version":"0.8.0","document_kind":"openpipestress.product_preview.model",
        "pressure_contract":{"version":"3.0.0","mode":"exact_pressure_v3","shear_model":"euler_bernoulli"},
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

/// The two-pipe geometry this control used before §8.30 correction 1: two
/// parallel pipes between A and B, each its own region, region 2 separately
/// supported at both terminals. Its terminals share A and B, so it is a
/// shared-terminal pressure junction with a dropped cap and is refused by name
/// before assembly (the expected code moved from
/// PRESSURE_ASSEMBLY_CANCELLATION_UNRESOLVED; Agent 0, 2026-10-28).
fn two_parallel_regions() -> Value {
    let mut input = model(false, true, 0.0);
    let second_pipe = pipe("pipe:parallel", ROOT, TIP, 0.16, 0.02, "material:one");
    input["model"]["pipe_segments"]
        .as_array_mut()
        .unwrap()
        .push(second_pipe);
    let mut second = input["model"]["load_cases"][0]["pressure_regions"][0].clone();
    second["id"] = json!("region:other-source");
    second["member_pipe_ids"] = json!(["pipe:parallel"]);
    second["terminals"] = json!([terminal(ROOT, false), terminal(TIP, false)]);
    let nu = 0.3;
    second["pressure"]["value"] =
        json!(2e6 * 25.0 / 36.0 * ((1.0 - 2.0 * nu) / (2.0 * nu)) * (1.0 + 1e-13));
    input["model"]["load_cases"][0]["pressure_regions"]
        .as_array_mut()
        .unwrap()
        .push(second);
    input
}

#[test]
fn strongly_cancelling_distinct_pressure_factors_are_explicitly_unqualified() {
    // One region over two collinear pipes A-M-B, transferring at both
    // terminals, so no terminal is shared. Pressure groups are keyed by
    // pressure and bore. The second pipe's outside diameter is one ulp above
    // the first's: its bore differs within the region's equal-bore guard (64
    // epsilon of the radius), so at M the two members' Poisson groups,
    // -2*nu*p*Ai1 and +2*nu*p*Ai2, are distinct and strongly cancel. With
    // nu = 0.5*(1 - 1e-13) each terminal's cap and Poisson terms share a group
    // whose load (1 - 2*nu)*p*Ai is about 1e-13*p*Ai, so the whole
    // translational RHS is tiny and the guard's ratio is about 1e13.
    let mut input = model(false, true, 0.0);
    let middle = "node:M";
    let document = &mut input["model"];
    document["nodes"].as_array_mut().unwrap().push(node(middle, [LENGTH / 2.0, 0.0, 0.0]));
    document["pipe_segments"] = json!([
        pipe(PIPE, ROOT, middle, 0.12, 0.01, "material:one"),
        pipe("pipe:M-B", middle, TIP, f64::from_bits(0.12_f64.to_bits() + 1), 0.01, "material:one"),
    ]);
    document["materials"] = json!([material("material:one", 200e9, 0.5 * (1.0 - 1e-13))]);
    document["load_cases"][0]["pressure_regions"][0]["member_pipe_ids"] = json!([PIPE, "pipe:M-B"]);
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

#[test]
fn the_former_two_parallel_region_control_is_a_refused_shared_terminal_junction() {
    for mode in MODES {
        let output = solve(two_parallel_regions(), mode);
        assert_eq!(output.status.mechanics, "MODEL_INCOMPLETE");
        assert!(output.results.is_empty());
        let codes = output.diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| (d.code.as_str(), d.affected_refs.clone())).collect::<Vec<_>>();
        assert_eq!(
            codes,
            [
                ("EXACT_PRESSURE_SHARED_TERMINAL_JUNCTION_UNSUPPORTED", vec![CASE.to_string(), ROOT.to_string(), "region:other-source".to_string()]),
                ("EXACT_PRESSURE_SHARED_TERMINAL_JUNCTION_UNSUPPORTED", vec![CASE.to_string(), TIP.to_string(), "region:other-source".to_string()]),
            ],
            "{mode:?}"
        );
    }
}
