//! O4 source tests (exploratory)
use super::*;

const N05: &str = include_str!("../../../../fixtures/product_preview/pressure_source/physics_source/n05.request.json");
const FIELDS: &str = include_str!("../../../../fixtures/product_preview/pressure_source/physics_source/fields.request.json");

fn run(request: &Value, mode: PreviewSolverMode) -> Value {
    serde_json::to_value(run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap()).unwrap()
}

#[test]
fn o4_explore() {
    let original: Value = serde_json::from_str(N05).unwrap();
    let mut timo = original.clone();
    timo["model"]["pressure_contract"]["shear_model"] = json!("timoshenko_annulus_energy_v1");
    let mut dual = original.clone();
    dual["model"]["pipe_segments"][0]["section"]["mill_tolerance"] = json!({"value":0.001,"unit":"m"});
    dual["model"]["pipe_segments"][0]["section"]["corrosion_allowance"] = json!({"value":0.0015,"unit":"m"});
    let mut both = dual.clone();
    both["model"]["pressure_contract"]["shear_model"] = json!("timoshenko_annulus_energy_v1");
    let mut two: Value = serde_json::from_str(FIELDS).unwrap();
    let model = &mut two["model"];
    let mut mid = model["nodes"][1].clone();
    mid["id"] = json!("independent-mid");
    mid["position"]["x"] = json!(1.0);
    model["nodes"].as_array_mut().unwrap().insert(1, mid);
    let mut second = model["pipe_segments"][0].clone();
    model["pipe_segments"][0]["to"] = json!("independent-mid");
    second["id"] = json!("independent-member-2");
    second["from"] = json!("independent-mid");
    model["pipe_segments"].as_array_mut().unwrap().push(second);
    let mut mixed = two.clone();
    mixed["model"]["pipe_segments"][0]["section"]["mill_tolerance"] = json!({"value":0.0005,"unit":"m"});
    mixed["model"]["pipe_segments"][0]["section"]["corrosion_allowance"] = json!({"value":0.001,"unit":"m"});
    let mut mixed_timo = mixed.clone();
    mixed_timo["model"]["pressure_contract"]["shear_model"] = json!("timoshenko_annulus_energy_v1");
    for (label, request) in [("two", &two), ("mixed", &mixed), ("mixed_timo", &mixed_timo), ("timo", &timo), ("dual", &dual), ("both", &both)] {
        for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
            let env = run(request, mode);
            let invocation = json!({"request": request, "solver_mode": mode.as_str()});
            let verdict = open_pipe_stress_result_export::reader_verdict::validate("pressure-source", &env, Some(&request["model"]), Some(&invocation));
            eprintln!("{label} {:?}: {} {} {}", mode, env["producer"]["semantic_contract_id"],
                env["diagnostics"].as_array().unwrap().iter().filter(|d| d["code"]=="SOURCE_BLOCK_RECOVERY_UNAVAILABLE" || d["code"]=="SOURCE_ENDPOINT_MAXIMUM_UNAVAILABLE").map(|d| d["message"].to_string()).collect::<Vec<_>>().join(" | "), verdict);
        }
    }
}
