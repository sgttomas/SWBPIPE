use open_pipe_stress_headless_runner::{
    result_envelope_binding::build_result_export_document, run_preview_in_memory, PrivacyContext,
    ProfessionalBoundary, Provenance, RedistributionStatus, Reference, RunnerOperation,
    RunnerRequest, TbdDecisions,
};
use open_pipe_stress_product_physics::LinearStaticPreviewRequest;
use serde_json::{json, Value};
use std::{collections::BTreeSet, env, fs, path::Path};

const CURVED_SIGN: &str = "local x is endpoint arc tangent toward j; local z is bend-plane normal; local y is z cross x toward arc center; resultants come from section equilibrium over assembled end actions";

fn runner_request(case_id: &str) -> RunnerRequest {
    RunnerRequest {
        request_id: format!("p5-{case_id}"),
        operation: RunnerOperation::Solve,
        operation_ref: Reference::new("api_operation", "solve"),
        project_ref: Reference::new("project", "invented-project"),
        model_ref: Reference::new("model", format!("invented-{case_id}")),
        unit_system_ref: Reference::new("unit_system", "invented-si"),
        load_basis_refs: vec![Reference::new("load_case", "load:L-100")],
        input_manifest_ref: Reference::new("audit_manifest", format!("manifest-{case_id}")),
        requested_outputs: vec![
            "result_envelope".into(),
            "audit_manifest".into(),
            "diagnostics".into(),
        ],
        privacy: PrivacyContext::local_first_public_metadata(),
        provenance: Provenance {
            source_name: "P5 invented complete-envelope schema witness".into(),
            source_location: "instances/P5/schema_witness".into(),
            source_license: "project invented".into(),
            contributor: "OpenPipeStress".into(),
            contributor_certification: "invented non-engineering example".into(),
            redistribution_status: RedistributionStatus::InventedNonEngineeringExample,
            review_status: "accepted".into(),
        },
        professional_boundary: ProfessionalBoundary::project_default(),
        tbd_decisions: TbdDecisions::d33_local_cli_policy(),
    }
}

fn straight_model(mut model: Value) -> Value {
    model["supports"]
        .as_array_mut()
        .expect("supports")
        .retain(|support| support.get("nonlinear").is_none());
    model
}

fn curved_model(mut model: Value, pressure_only: bool) -> Value {
    let nodes = model["nodes"].as_array_mut().expect("nodes");
    nodes.truncate(2);
    nodes[0]["id"] = json!("node:N-100");
    nodes[0]["position"] = json!({"x": 0.0, "y": 0.0, "z": 0.0});
    nodes[1]["id"] = json!("node:N-110");
    nodes[1]["position"] = json!({"x": 2.0, "y": 0.0, "z": 0.0});

    let pipes = model["pipe_segments"].as_array_mut().expect("pipe segments");
    pipes.truncate(1);
    pipes[0]["id"] = json!("pipe:P-100");
    pipes[0]["from"] = json!("node:N-100");
    pipes[0]["to"] = json!("node:N-110");
    pipes[0]["y_reference"] = json!({"x": 0.0, "y": 1.0, "z": 0.0});

    let supports = model["supports"].as_array_mut().expect("supports");
    supports.truncate(1);
    supports[0]["id"] = json!("support:S-100");
    supports[0]["node"] = json!("node:N-100");
    supports[0]["family"] = json!("anchor");
    supports[0]["restraints"] = json!(["UX", "UY", "UZ", "RX", "RY", "RZ"]);
    supports[0].as_object_mut().expect("support object").remove("nonlinear");

    let components = model["components"].as_array_mut().expect("components");
    components.truncate(1);
    components[0]["id"] = json!("component:C-110");
    components[0]["node"] = json!("node:N-110");
    components[0]["geometry"]["bend_pipe_ref"] = json!("pipe:P-100");
    components[0]["geometry"]["bend_radius"] = json!({"value": 2.0_f64.sqrt(), "unit": "m"});
    components[0]["geometry"]["bend_angle"] = json!({"value": std::f64::consts::FRAC_PI_2, "unit": "rad"});
    components[0]["modifiers"]["sif_user_value"] = json!({"value": 1.15, "unit": "none"});
    components[0]["modifiers"]["flexibility_factor_user_value"] = json!({"value": 2.0, "unit": "none"});
    components[0]["mechanics_interface"]["solver_consumption"] = json!("curved_bend_macro_element");

    let cases = model["load_cases"].as_array_mut().expect("load cases");
    cases.truncate(1);
    cases[0]["id"] = json!("load:L-100");
    cases[0]["primitive_loads"] = if pressure_only {
        json!([{
            "id": "load:L-100-P",
            "category": "pressure",
            "target": {"type": "element", "pipe": "pipe:P-100"},
            "direction": "global_x",
            "magnitude": {"value": 2_000_000.0, "unit": "Pa"},
            "dimension": "pressure",
            "provenance": "invented_example_user_input"
        }])
    } else {
        json!([
            {
                "id": "load:L-100-Y",
                "category": "occasional",
                "target": {"type": "node", "node": "node:N-110"},
                "direction": "global_y",
                "magnitude": {"value": 1000.0, "unit": "N"},
                "dimension": "force",
                "provenance": "invented_example_user_input"
            },
            {
                "id": "load:L-100-W",
                "category": "weight",
                "target": {"type": "element", "pipe": "pipe:P-100"},
                "direction": "global_z",
                "magnitude": {"value": -190.0, "unit": "N/m"},
                "dimension": "force_per_length",
                "provenance": "invented_example_user_input"
            }
        ])
    };
    model["combinations"] = json!([]);
    model
}

fn validate_curved_metadata(mechanics: &open_pipe_stress_product_physics::MechanicsEnvelope) {
    let expected_locations = BTreeSet::from(["end_i", "end_j", "quarter_1", "midspan", "quarter_3"]);
    let mut stress_locations = BTreeSet::new();
    for row in mechanics.results.iter().filter(|row| row.entity_ref == "pipe:P-100") {
        let Some(metadata) = row.metadata.as_ref() else { continue };
        let is_station_force_or_moment = matches!(metadata.location.as_str(), "quarter_1" | "midspan" | "quarter_3")
            && (row.kind.starts_with("element_local_") && (row.kind.contains("force") || row.kind.contains("moment")));
        let is_mechanical_stress = row.id.starts_with("result:stress:pipe-P-100:")
            && row.kind.starts_with("element_local_");
        if is_mechanical_stress {
            stress_locations.insert(metadata.location.as_str());
        }
        if is_station_force_or_moment || is_mechanical_stress {
            assert_eq!(metadata.coordinate_system, "element_local", "{}", row.id);
            assert_eq!(metadata.basis, "recovered_from_local_element_stiffness", "{}", row.id);
            assert_eq!(metadata.sign_convention, CURVED_SIGN, "{}", row.id);
        }
    }
    assert_eq!(stress_locations, expected_locations);
}

fn write_json(path: &Path, value: &impl serde::Serialize) {
    fs::write(path, format!("{}\n", serde_json::to_string_pretty(value).unwrap())).unwrap();
}
fn run_case(case_id: &str, model: Value, output_dir: &Path, curved: bool, mode: Option<open_pipe_stress_product_physics::PreviewSolverMode>) {
    use open_pipe_stress_canonical_json::canonical_json;
    use sha2::{Digest, Sha256};
    let request = runner_request(case_id);
    write_json(&output_dir.join(format!("{case_id}.model.json")), &model);
    write_json(&output_dir.join(format!("{case_id}.request.json")), &request);
    let preview = LinearStaticPreviewRequest { model: serde_json::from_value(model.clone()).unwrap(), materials: vec![] };
    let output = run_preview_in_memory(request.clone(), preview);
    let mut runner = output.runner_result.clone();
    let mechanics = match mode {
        Some(mode) => open_pipe_stress_product_physics::run_linear_static_preview_with_mode(LinearStaticPreviewRequest { model: serde_json::from_value(model).unwrap(), materials: vec![] }, mode),
        None => output.mechanics_envelope.clone().expect("mechanics envelope"),
    };
    if mode.is_some() {
        // Explicit lower-level producer: preserve exact current checksum algorithm.
        // This runner context came from the default attached route; no dense attachment claim.
        let raw = canonical_json(&serde_json::to_value(&mechanics).unwrap());
        runner.checksums.iter_mut().find(|c| c.payload_ref.ref_type == "result_envelope").unwrap().value = format!("{:x}", Sha256::digest(raw.as_bytes()));
    }
    // Freeze actual producer rows and metadata BEFORE calling the adapter.
    write_json(&output_dir.join(format!("{case_id}.mechanics.json")), &mechanics);
    write_json(&output_dir.join(format!("{case_id}.runner.json")), &runner);
    if mechanics.status.mechanics == "MECHANICS_SOLVED" {
        if curved { validate_curved_metadata(&mechanics); }
        match build_result_export_document(&request, &runner, &mechanics) {
            Ok(rebuilt) => {
                if mode.is_none() { assert_eq!(output.result_envelope_document.as_ref(), Some(&rebuilt)); }
                write_json(&output_dir.join(format!("{case_id}.document.json")), &rebuilt);
            },
            Err(diag) => write_json(&output_dir.join(format!("{case_id}.adapter_error.json")), &diag),
        }
    } else {
        if mode.is_none() { assert!(output.result_envelope_document.is_none()); }
        write_json(&output_dir.join(format!("{case_id}.canonical_absence.json")), &json!({"canonical_solved_export": false, "producer_mechanics_status": mechanics.status.mechanics, "route": if mode.is_some() {"explicit_lower_level_mode"} else {"attached_default_sparse"}}));
    }
}
fn friction_candidate(mut model: Value, coefficient: f64, normal: f64, load: f64, seed: &str) -> Value {
    model = straight_model(model);
    model["nodes"].as_array_mut().unwrap().truncate(2);
    model["nodes"][0]["position"] = json!({"x":0.0,"y":0.0,"z":0.0});
    model["nodes"][1]["position"] = json!({"x":1.0,"y":0.0,"z":0.0});
    model["pipe_segments"].as_array_mut().unwrap().truncate(1);
    model["pipe_segments"][0]["y_reference"] = json!({"x":0.0,"y":1.0,"z":0.0});
    model["supports"].as_array_mut().unwrap().truncate(1);
    model["supports"][0]["family"] = json!("anchor");
    model["supports"][0]["restraints"] = json!(["UX","UY","UZ","RX","RY","RZ"]);
    model["supports"].as_array_mut().unwrap().push(json!({"id":"support:NL-FRIC-110","node":"node:N-110","restraints":[],"family":"nonlinear","provenance":"invented_example", "nonlinear":{"behavior":"friction","dof":"UX","initial_state":seed,"friction_coefficient":{"value":coefficient,"unit":"none"},"normal_reaction":{"value":normal,"unit":"N"}}}));
    model["components"] = json!([]); model["combinations"] = json!([]);
    model["load_cases"].as_array_mut().unwrap().truncate(1);
    model["load_cases"][0]["primitive_loads"] = json!([{"id":"load:probe","category":"occasional","target":{"type":"node","node":"node:N-110"},"direction":"global_x","magnitude":{"value":load,"unit":"N"},"dimension":"force","provenance":"invented_example"}]); model
}
fn main() {
    let args: Vec<_> = env::args_os().collect(); assert_eq!(args.len(), 3);
    let model: Value = serde_json::from_slice(&fs::read(&args[1]).unwrap()).unwrap();
    let out = Path::new(&args[2]); fs::create_dir_all(out).unwrap();
    run_case("straight-full", straight_model(model.clone()), out, false, None);
    run_case("curved-tip-weight-full", curved_model(model.clone(), false), out, true, None);
    run_case("curved-pressure-full", curved_model(model.clone(), true), out, true, None);
    run_case("straight-dense-full", straight_model(model.clone()), out, false, Some(open_pipe_stress_product_physics::PreviewSolverMode::DenseScrutiny));
    run_case("straight-sparse-full", straight_model(model.clone()), out, false, Some(open_pipe_stress_product_physics::PreviewSolverMode::SparseInteractive));
    let mut blocked=straight_model(model.clone()); blocked["document_kind"]=json!("invalid-invented-probe"); run_case("blocked",blocked,out,false,None);
    // Read-only current-product probes; fixed cap=4 remains unchanged. Selection requires actual NONCONVERGENCE evidence.
    for (i, (mu, normal, load, seed)) in [(0.5,1000.0,100.0,"sticking"),(0.5,1000.0,501.0,"sticking"),(0.5,1000.0,499.0,"sliding"),(0.5,1000.0,-501.0,"sliding"),(1.0,1000.0,1000.0,"sliding"),(0.01,1000.0,10.0,"sliding")].iter().enumerate() {
        run_case(&format!("nonconverged-candidate-{i}"),friction_candidate(model.clone(),*mu,*normal,*load,seed),out,false,None);
    }
}
