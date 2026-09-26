//! T0R admission through headless custody: actual producer invocations only.
//! No native UI qualification is claimed.
use open_pipe_stress_headless_runner::{
    run_preview_model_value_with_mode, validate_result_with_optional_envelope_payload,
    PrivacyContext, ProfessionalBoundary, Provenance, RedistributionStatus, Reference,
    RunnerOperation, RunnerRequest, TbdDecisions,
};
use open_pipe_stress_product_physics::PreviewSolverMode;
use open_pipe_stress_result_export::{derivative, semantic_contract, source_blocks};
use serde_json::{json, Value};
use std::path::Path;

const MODES: [PreviewSolverMode; 2] = [
    PreviewSolverMode::SparseInteractive,
    PreviewSolverMode::DenseScrutiny,
];

fn request(model: &Value) -> RunnerRequest {
    RunnerRequest {
        request_id: "t0r-preview-physics-admission".into(),
        operation: RunnerOperation::Solve,
        operation_ref: Reference::new("api_operation", "ops.solve.job"),
        project_ref: Reference::new("project", model["project"]["id"].as_str().unwrap()),
        model_ref: Reference::new("model", model["project"]["id"].as_str().unwrap()),
        unit_system_ref: Reference::new("unit_system", "invented-si"),
        load_basis_refs: model["load_cases"]
            .as_array()
            .unwrap()
            .iter()
            .map(|c| Reference::new("load_case", c["id"].as_str().unwrap()))
            .collect(),
        input_manifest_ref: Reference::new("audit_manifest", "t0r-preview-input-manifest"),
        requested_outputs: vec![
            "result_envelope".into(),
            "audit_manifest".into(),
            "diagnostics".into(),
        ],
        privacy: PrivacyContext::local_first_public_metadata(),
        provenance: Provenance {
            source_name: "invented T0R admission fixture".into(),
            source_location: "fixtures/model_operations".into(),
            source_license: "project invented".into(),
            contributor: "OpenPipeStress".into(),
            contributor_certification: "invented non-engineering example".into(),
            redistribution_status: RedistributionStatus::InventedNonEngineeringExample,
            review_status: "pending".into(),
        },
        professional_boundary: ProfessionalBoundary::project_default(),
        tbd_decisions: TbdDecisions::d33_local_cli_policy(),
    }
}
fn read(path: &str) -> Value {
    serde_json::from_slice(
        &std::fs::read(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../../")
                .join(path),
        )
        .unwrap(),
    )
    .unwrap()
}
fn bases(model: &Value) -> Vec<Value> {
    model["load_cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| json!({"ref_type":"load_case","ref_id":c["id"]}))
        .collect()
}

#[test]
fn fresh_non_exact_solve_is_preview_physics_admitted_and_exportable() {
    let model = read("fixtures/model_operations/precision_connected_ui_model.json");
    let payload = json!({"model":model,"materials":[]});
    for mode in MODES {
        let output =
            run_preview_model_value_with_mode(request(&model), payload.clone(), mode).unwrap();
        let raw = serde_json::to_value(output.mechanics_envelope.as_ref().unwrap()).unwrap();
        assert_eq!(
            raw["producer"]["semantic_contract_id"],
            semantic_contract::PREVIEW_PHYSICS_ID
        );
        assert_ne!(
            raw["producer"]["semantic_contract_id"],
            semantic_contract::PRECISION_ID
        );
        assert_eq!(
            semantic_contract::for_source(&raw).unwrap().0["semantic_contract_id"],
            semantic_contract::PREVIEW_PHYSICS_ID
        );
        assert_eq!(
            semantic_contract::numerical_use_standing(&raw, &bases(&model)),
            "numerically_eligible"
        );
        let doc = output
            .result_envelope_document
            .as_ref()
            .unwrap_or_else(|| panic!("{mode:?}: {:?}", output.canonical_export_unavailability));
        assert_eq!(
            doc["result_envelope"]["contract_evidence"],
            raw["contract_evidence"]
        );
        derivative::validate_document(doc, &raw).unwrap();
        let validation =
            validate_result_with_optional_envelope_payload(&output.runner_result, Some(doc));
        assert!(
            !validation.has_blocking_diagnostics(),
            "{mode:?}: {:?}",
            validation.diagnostics
        );
        // A derivative relabelled with a retired identity is not admitted as fresh.
        let mut relabelled = doc.clone();
        relabelled["result_envelope"]["producer"]["semantic_contract_id"] =
            json!("openpipestress.result_semantics/0.3.0/unknown-1");
        relabelled["result_envelope"]["semantic_contract_ref"]["ref_id"] =
            json!("openpipestress.result_semantics/0.3.0/unknown-1");
        assert!(validate_result_with_optional_envelope_payload(
            &output.runner_result,
            Some(&relabelled)
        )
        .has_blocking_diagnostics());
    }
}

#[test]
fn selected_plus_failed_source_blocks_stays_not_eligible() {
    // Received source-block request plus one element-load case the source gate
    // cannot select: the actual producer records it as failed (receipt partial).
    for stem in ["multicase", "n05"] {
        for mode in MODES {
            let mut input = read(&format!(
                "fixtures/product_preview/source_blocks/ui/{stem}-{}.request.json",
                mode.as_str()
            ));
            let pipe = input["model"]["pipe_segments"][0]["id"].clone();
            input["model"]["load_cases"].as_array_mut().unwrap().push(json!({"id":"case:t0r-failed-companion","kind":"primitive_user_load","label":"element-load companion","provenance":"invented_t0r_selected_failed_control","primitive_loads":[{"category":"distributed_force","dimension":"force_per_length","direction":"global_z","id":"load:t0r-failed-companion","magnitude":{"unit":"N/m","value":-10.0},"provenance":"invented_t0r_selected_failed_control","target":{"type":"element","pipe":pipe}}]}));
            let output =
                run_preview_model_value_with_mode(request(&input["model"]), input.clone(), mode)
                    .unwrap();
            let raw = serde_json::to_value(output.mechanics_envelope.as_ref().unwrap()).unwrap();
            assert_eq!(
                raw["producer"]["semantic_contract_id"],
                source_blocks::CONTRACT_ID,
                "{stem}/{mode:?}"
            );
            let body = &raw["source_block_recovery"]["body"];
            assert_eq!(body["status"], "partial", "{stem}/{mode:?}");
            assert!(body["cases"]
                .as_array()
                .unwrap()
                .iter()
                .any(|c| c["outcome"] != "qualified"));
            assert_eq!(semantic_contract::standing_reason(&raw), None);
            let invocation = json!({"request":input,"solver_mode":mode.as_str()});
            assert_eq!(
                semantic_contract::numerical_use_standing_with_context(
                    &raw,
                    &bases(&input["model"]),
                    Some(&invocation)
                ),
                "needs_recompute",
                "{stem}/{mode:?}"
            );
            assert!(output.result_envelope_document.is_none(), "{stem}/{mode:?}");
        }
    }
}

#[test]
fn all_selected_source_blocks_keeps_its_standing() {
    for stem in ["n05", "n06", "multicase"] {
        for mode in MODES {
            let input = read(&format!(
                "fixtures/product_preview/source_blocks/ui/{stem}-{}.request.json",
                mode.as_str()
            ));
            let output =
                run_preview_model_value_with_mode(request(&input["model"]), input.clone(), mode)
                    .unwrap();
            let raw = serde_json::to_value(output.mechanics_envelope.as_ref().unwrap()).unwrap();
            assert_eq!(
                raw["producer"]["semantic_contract_id"],
                source_blocks::CONTRACT_ID
            );
            assert_eq!(semantic_contract::standing_reason(&raw), None);
            let invocation = json!({"request":input,"solver_mode":mode.as_str()});
            assert_eq!(
                semantic_contract::numerical_use_standing_with_context(
                    &raw,
                    &bases(&input["model"]),
                    Some(&invocation)
                ),
                "numerically_eligible",
                "{stem}/{mode:?}"
            );
            // Its abs-sum summary cannot be bound by a rule (checked at every binding site).
            let headline = raw["summary"]["max_open_formula_stress"]["result_ref"].clone();
            let summary_row = raw["results"]
                .as_array()
                .unwrap()
                .iter()
                .find(|r| r["id"] == headline)
                .unwrap();
            assert_eq!(
                semantic_contract::rule_binding_refusal(&raw, summary_row),
                Some(semantic_contract::RULE_SOURCE_BLOCKS_SUMMARY_NOT_RELIABLE)
            );
        }
    }
}

/// DESIGN 5.7 item 4: headless passes only a caller-supplied aggregate rule
/// status and has no solver-result binding path. A later binding path (T1 WP4,
/// T6) must call `rule_binding_refusal`; this pin fails when one appears.
#[test]
fn no_headless_path_binds_solver_rows_to_rules() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest = std::fs::read_to_string(root.join("Cargo.toml")).unwrap();
    assert!(
        !manifest.contains("rule_check_runner"),
        "headless gained a rule runner dependency"
    );
    let mut sources = vec![root.join("src/lib.rs")];
    for dir in ["src", "src/bin"] {
        for entry in std::fs::read_dir(root.join(dir)).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_some_and(|e| e == "rs") && !sources.contains(&path) {
                sources.push(path);
            }
        }
    }
    assert!(sources.len() >= 6);
    for path in sources {
        let text = std::fs::read_to_string(&path).unwrap();
        for token in [
            "rule_check_runner::",
            "open_pipe_stress_rule_check_runner",
            "SolverResultBinding",
            "solver_result_ref",
            "solver_result_bindings",
            "run_rule_checks",
        ] {
            assert!(!text.contains(token), "{}: {token}", path.display());
        }
    }
}

/// T0R A1: the unchanged invented demo keeps its realized user-stiffness joint
/// and is refused before solving, as a sanitized preview-physics-1 blocked envelope.
#[test]
fn realized_joint_is_refused_as_a_sanitized_blocked_envelope() {
    let mut model = read("fixtures/product_preview/invented_preview_model.json");
    // Pressure is incidental; zero it in memory so only the joint refusal applies.
    for case in model["load_cases"].as_array_mut().unwrap() {
        for load in case["primitive_loads"].as_array_mut().unwrap() {
            if load["category"] == "pressure" {
                load["magnitude"]["value"] = json!(0.0);
            }
        }
    }
    let payload = json!({"model":model,"materials":[]});
    for mode in MODES {
        let output =
            run_preview_model_value_with_mode(request(&model), payload.clone(), mode).unwrap();
        let raw = serde_json::to_value(output.mechanics_envelope.as_ref().unwrap()).unwrap();
        assert_eq!(raw["status"]["mechanics"], "MODEL_INCOMPLETE");
        assert_eq!(
            raw["producer"]["semantic_contract_id"],
            semantic_contract::PREVIEW_PHYSICS_ID
        );
        assert_eq!(
            raw["contract_evidence"],
            json!({"preview_cases":[],"combination_gates":[]})
        );
        assert!(raw["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|d| d["code"] == "JOINT_ELEMENT_EQUILIBRIUM_UNQUALIFIED"
                && d["severity"] == "blocking"
                && d["affected_refs"][0] == "component:C-150"));
        let text = serde_json::to_string(&raw["diagnostics"]).unwrap();
        assert!(
            !text.contains("\"result:"),
            "{mode:?}: blocked envelope names a row"
        );
        semantic_contract::for_source(&raw).unwrap();
        assert!(output.result_envelope_document.is_none());
        assert_eq!(
            output.canonical_export_unavailability.as_deref(),
            Some("SOURCE_NOT_SOLVED")
        );
    }
}
