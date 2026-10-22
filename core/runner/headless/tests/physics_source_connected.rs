//! Actual producer invocation through headless custody and canonical projection.
//! No native UI qualification or packet replay as a solve is claimed.
use open_pipe_stress_headless_runner::{
    result_envelope_binding, run_preview_model_value_with_mode, PrivacyContext,
    ProfessionalBoundary, Provenance, RedistributionStatus, Reference, RunnerOperation,
    RunnerRequest, TbdDecisions,
};
use open_pipe_stress_product_physics::PreviewSolverMode;
use open_pipe_stress_result_export::{derivative, semantic_contract};
use serde_json::{json, Value};
use std::path::Path;

fn request(model: &Value) -> RunnerRequest {
    RunnerRequest {
        request_id: "actual-composite-headless-consumer".into(),
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
        input_manifest_ref: Reference::new("audit_manifest", "actual-composite-input-manifest"),
        requested_outputs: vec![
            "result_envelope".into(),
            "audit_manifest".into(),
            "diagnostics".into(),
        ],
        privacy: PrivacyContext::local_first_public_metadata(),
        provenance: Provenance {
            source_name: "invented composite consumer fixture".into(),
            source_location: "fixtures/product_preview/pressure_source/physics_source".into(),
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

#[test]
fn actual_composite_invocations_preserve_canonical_evidence_and_maximum_methods() {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../fixtures/product_preview/pressure_source/physics_source");
    for name in ["n05", "n06", "mixed", "fields"] {
        for mode in [
            PreviewSolverMode::SparseInteractive,
            PreviewSolverMode::DenseScrutiny,
        ] {
            let input: Value = serde_json::from_slice(
                &std::fs::read(fixtures.join(format!("{name}.request.json"))).unwrap(),
            )
            .unwrap();
            let original = input.clone();
            let metadata = request(&input["model"]);
            let output =
                run_preview_model_value_with_mode(metadata.clone(), input.clone(), mode).unwrap();
            let mechanics = output.mechanics_envelope.as_ref().unwrap();
            let raw = serde_json::to_value(mechanics).unwrap();
            let captured: Value = serde_json::from_slice(
                &std::fs::read(fixtures.join(format!("{name}-{}.raw.json", mode.as_str())))
                    .unwrap(),
            )
            .unwrap();
            assert_eq!(raw, captured, "actual producer changed: {name}/{mode:?}");
            let document = output.result_envelope_document.as_ref().unwrap_or_else(|| {
                panic!(
                    "{name}/{mode:?}: {:?}",
                    output.canonical_export_unavailability
                )
            });
            assert_eq!(
                raw["producer"]["semantic_contract_id"],
                semantic_contract::PRESSURE_SOURCE_ID
            );
            assert_eq!(
                document["result_envelope"]["contract_evidence"],
                raw["contract_evidence"]
            );
            assert_eq!(
                document["result_envelope"]["source_block_recovery"],
                raw["source_block_recovery"]
            );
            derivative::validate_document(document, &raw).unwrap();
            let proof = output.qualified_preview_evidence.as_ref().unwrap();
            assert_eq!(
                *document,
                result_envelope_binding::build_result_export_document_with_evidence(
                    &metadata,
                    &output.runner_result,
                    mechanics,
                    proof
                )
                .unwrap()
            );
            assert!(result_envelope_binding::build_result_export_document(
                &metadata,
                &output.runner_result,
                mechanics
            )
            .is_err());
            let requested = input["model"]["load_cases"]
                .as_array()
                .unwrap()
                .iter()
                .map(|c| json!({"ref_type":"load_case","ref_id":c["id"]}))
                .collect::<Vec<_>>();
            assert_eq!(
                semantic_contract::numerical_use_standing(&raw, &requested),
                "needs_recompute"
            );
            let invocation = json!({"request":input,"solver_mode":mode.as_str()});
            assert_eq!(
                semantic_contract::numerical_use_standing_with_context(
                    &raw,
                    &requested,
                    Some(&invocation)
                ),
                "numerically_eligible"
            );
            for key in ["model", "solver_mode"] {
                let mut wrong = invocation.clone();
                if key == "model" {
                    wrong["request"]["model"]["project"]["name"] =
                        json!("same-id edited after solve");
                } else {
                    wrong["solver_mode"] = json!(if mode == PreviewSolverMode::SparseInteractive {
                        "dense_scrutiny"
                    } else {
                        "sparse_interactive"
                    });
                }
                assert_ne!(
                    semantic_contract::numerical_use_standing_with_context(
                        &raw,
                        &requested,
                        Some(&wrong)
                    ),
                    "numerically_eligible"
                );
            }
            for row in raw["results"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|r| r["kind"] == "pipe_elastic_normal_stress_maximum_v2")
            {
                let signature = semantic_contract::signature_in(
                    semantic_contract::pressure_source_contract(),
                    row,
                )
                .unwrap()
                .unwrap();
                assert_eq!(signature["source_basis"], row["metadata"]["basis"]);
                let account = document["result_envelope"]["row_accounting"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|r| r["source_result_id"] == row["id"])
                    .unwrap();
                let quantity = document
                    .pointer(account["target_field_path"].as_str().unwrap())
                    .unwrap();
                assert_eq!(
                    quantity["magnitude"].as_f64().unwrap().to_bits(),
                    row["value"].as_f64().unwrap().to_bits()
                );
                assert_eq!(quantity["metadata"], row["metadata"]);
            }
            assert_eq!(input, original);
            // Epoch 8 (DEL-04-07 Design desktop-exact-outputs, section 6.2): n05's
            // document is the plain pressure-source-1 result-export twin, the
            // desktop's equal-input oracle. The raw above is the committed raw
            // control (equal JSON values and canonical digests; the committed
            // file is compact and producer-ordered). The committed twin is checked
            // byte for byte, and rewritten only with HEADLESS_PLAIN_TWIN_WRITE_FIXTURES=1.
            if name == "n05" {
                assert_eq!(derivative::digest(&raw).unwrap(), derivative::digest(&captured).unwrap());
                derivative::validate_document(document, &captured).unwrap();
                let short = if mode == PreviewSolverMode::SparseInteractive { "sparse" } else { "dense" };
                let text = serde_json::to_string(document).unwrap() + "\n";
                let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(format!(
                    "../../../fixtures/results/pressure_source_plain_n05_{short}.document.json"
                ));
                if std::env::var("HEADLESS_PLAIN_TWIN_WRITE_FIXTURES").as_deref() == Ok("1") {
                    std::fs::write(&path, &text).unwrap();
                }
                assert_eq!(std::fs::read_to_string(&path).unwrap(), text, "{name}/{mode:?}: document twin bytes");
            }
            if let Ok(folder) = std::env::var("HEADLESS_COMPOSITE_OUTPUT_DIR") {
                let folder = Path::new(&folder);
                std::fs::create_dir_all(folder).unwrap();
                let stem = format!("{name}-{}", mode.as_str());
                for (suffix, value) in [
                    ("request", &input),
                    ("invocation", &invocation),
                    ("raw", &raw),
                    ("document", document),
                ] {
                    std::fs::write(
                        folder.join(format!("{stem}.{suffix}.json")),
                        serde_json::to_vec_pretty(value).unwrap(),
                    )
                    .unwrap();
                }
                let manifest = json!({"claim":"actual headless invocation and canonical projection; no native UI claim","mode":mode.as_str(),"request_sha256":derivative::digest(&input).unwrap(),"raw_sha256":derivative::digest(&raw).unwrap(),"document_sha256":derivative::digest(document).unwrap()});
                std::fs::write(
                    folder.join(format!("{stem}.manifest.json")),
                    serde_json::to_vec_pretty(&manifest).unwrap(),
                )
                .unwrap();
            }
        }
    }
}
