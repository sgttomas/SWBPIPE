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

fn run_case(case_id: &str, model: Value, output_path: &Path, curved: bool) {
    let request = runner_request(case_id);
    let preview = LinearStaticPreviewRequest {
        model: serde_json::from_value(model).expect("preview model must deserialize"),
        materials: vec![],
    };
    let output = run_preview_in_memory(request.clone(), preview);
    let mechanics = output.mechanics_envelope.as_ref().expect("mechanics envelope");
    assert_eq!(mechanics.status.mechanics, "MECHANICS_SOLVED", "{case_id}: {:?}", mechanics.diagnostics);
    assert!(output.runner_result.diagnostics.iter().all(|item| item.severity != open_pipe_stress_headless_runner::DiagnosticSeverity::Blocking));
    if curved {
        validate_curved_metadata(mechanics);
    }
    let rebuilt = build_result_export_document(&request, &output.runner_result, mechanics)
        .expect("complete result export document");
    assert_eq!(output.result_envelope_document.as_ref(), Some(&rebuilt));
    fs::write(output_path, format!("{}\n", serde_json::to_string_pretty(&rebuilt).unwrap())).unwrap();
}

fn main() {
    let args: Vec<_> = env::args_os().collect();
    assert_eq!(args.len(), 3, "usage: witness <invented-model.json> <output-dir>");
    let model: Value = serde_json::from_slice(&fs::read(&args[1]).unwrap()).unwrap();
    let output_dir = Path::new(&args[2]);
    fs::create_dir_all(output_dir).unwrap();
    run_case("straight-full", straight_model(model.clone()), &output_dir.join("straight_full_document.json"), false);
    run_case("curved-tip-weight-full", curved_model(model.clone(), false), &output_dir.join("curved_tip_weight_full_document.json"), true);
    run_case("curved-pressure-full", curved_model(model, true), &output_dir.join("curved_pressure_full_document.json"), true);
}
