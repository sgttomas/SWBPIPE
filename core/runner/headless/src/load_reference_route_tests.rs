//! T1 WP4: the 0.4.0 (load/reference-state) route through the headless value
//! runner, in both solver modes. Tests only; no product semantics change.
//!
//! - `load-reference-1`: the actual solve of the committed load-reference
//!   requests mints the opaque `QualifiedPreviewEvidence` and a canonical
//!   0.3.0 results document bound to it.
//! - `load-reference-source-1` (joined): the actual solve of the committed joined
//!   witnesses retains a receipt bound to the actual invocation. The joined
//!   route is never numerically eligible in T1 (T1_WAVE1_RULINGS.md section 7),
//!   so the binding builds no canonical document and mints no opaque proof;
//!   a joined binding route belongs to T3 with its eligibility.
//! - Refusals, blocked 0.4.0 envelopes and the SF-1 fallback, through the runner.
//!
//! Actual artifacts are written only when `HEADLESS_LOAD_REFERENCE_OUTPUT_DIR`
//! or `HEADLESS_LOAD_REFERENCE_SOURCE_OUTPUT_DIR` is set; the Python consumer
//! `tests/test_load_reference_headless_artifacts.py` reads them. All inputs
//! are invented, committed fixtures or declared transforms of them.
use crate::result_envelope_binding::{
    build_result_export_document, build_result_export_document_with_evidence,
};
use crate::{
    run_preview_model_value_with_mode, validate_result,
    validate_result_with_optional_envelope_payload, AnalysisStatus, JobStateKind,
    PreviewRunnerOutput, PrivacyContext, ProfessionalBoundary, Provenance, RedistributionStatus,
    Reference, RunnerOperation, RunnerRequest, TbdDecisions,
};
use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, NumericalQualityStatus, PreviewSolverMode,
};
use open_pipe_stress_result_export::derivative::{digest, validate_document};
use open_pipe_stress_result_export::{load_reference_source, semantic_contract, source_blocks};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

const LR_ID: &str = "openpipestress.result_semantics/0.3.0/load-reference-1";
const LR_PROFILE: &str = "resolved_straight_load_state_v1";
const LRS_ID: &str = "openpipestress.result_semantics/0.3.0/load-reference-source-1";
const LRS_PROFILE: &str = "resolved_straight_load_state_source_v1";
const LRS_POLICY: &str = "LOAD-REFERENCE-SOURCE-1";
const PHYSICS_ID: &str = "openpipestress.result_semantics/0.3.0/physics-1";
const PREVIEW_PHYSICS_ID: &str = "openpipestress.result_semantics/0.3.0/preview-physics-1";
const NEEDS_RECOMPUTE_UNAVAILABLE: &str =
    "result-envelope production failed structurally: CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE";
const MODES: [PreviewSolverMode; 2] = [
    PreviewSolverMode::SparseInteractive,
    PreviewSolverMode::DenseScrutiny,
];
const LR_STEMS: [&str; 2] = ["connected", "pressure"];
const LRS_STEMS: [&str; 5] = ["n05", "n06", "fields", "mixed", "eigen_motion"];

fn project() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn read(path: &str) -> Value {
    serde_json::from_slice(&std::fs::read(project().join(path)).unwrap()).unwrap()
}

fn lr_request(stem: &str) -> Value {
    read(&format!(
        "fixtures/product_preview/load_reference/{stem}.request.json"
    ))
}

fn lrs_request(stem: &str) -> Value {
    read(&format!(
        "fixtures/product_preview/load_reference_source/{stem}.request.json"
    ))
}

/// Runner metadata for an actual 0.4.0 invocation of the given payload.
fn metadata(payload: &Value) -> RunnerRequest {
    let model = &payload["model"];
    let id = model["project"]["id"].as_str().unwrap();
    RunnerRequest {
        request_id: "t1-wp4-load-reference-headless".into(),
        operation: RunnerOperation::Solve,
        operation_ref: Reference::new("api_operation", "ops.solve.job"),
        project_ref: Reference::new("project", id),
        model_ref: Reference::new("model", id),
        unit_system_ref: Reference::new("unit_system", "invented-si"),
        load_basis_refs: model["load_cases"]
            .as_array()
            .unwrap()
            .iter()
            .map(|c| Reference::new("load_case", c["id"].as_str().unwrap()))
            .collect(),
        input_manifest_ref: Reference::new("audit_manifest", "t1-wp4-input-manifest"),
        requested_outputs: vec![
            "result_envelope".into(),
            "audit_manifest".into(),
            "diagnostics".into(),
        ],
        privacy: PrivacyContext::local_first_public_metadata(),
        provenance: Provenance {
            source_name: "invented T1 WP4 load/reference-state fixture".into(),
            source_location: "fixtures/product_preview/load_reference".into(),
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

fn bases(payload: &Value) -> Vec<Value> {
    payload["model"]["load_cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| json!({"ref_type":"load_case","ref_id":c["id"]}))
        .collect()
}

fn invocation(payload: &Value, mode: PreviewSolverMode) -> Value {
    json!({"request": payload, "solver_mode": mode.as_str()})
}

fn other(mode: PreviewSolverMode) -> PreviewSolverMode {
    match mode {
        PreviewSolverMode::SparseInteractive => PreviewSolverMode::DenseScrutiny,
        PreviewSolverMode::DenseScrutiny => PreviewSolverMode::SparseInteractive,
    }
}

fn solve(payload: &Value, mode: PreviewSolverMode) -> PreviewRunnerOutput {
    run_preview_model_value_with_mode(metadata(payload), payload.clone(), mode)
        .unwrap_or_else(|e| panic!("{mode:?}: {e}"))
}

fn raw_of(output: &PreviewRunnerOutput) -> Value {
    serde_json::to_value(output.mechanics_envelope.as_ref().unwrap()).unwrap()
}

fn blocking_codes(raw: &Value) -> Vec<String> {
    raw["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|d| d["severity"] == "blocking")
        .map(|d| d["code"].as_str().unwrap().to_string())
        .collect()
}

/// Library-only fields never reach a serialized runner surface.
fn assert_wire_excludes_library_evidence(output: &PreviewRunnerOutput) {
    let wire = serde_json::to_value(output).unwrap();
    for key in [
        "result_envelope_document",
        "qualified_preview_evidence",
        "canonical_export_unavailability",
        "actual_invocation",
    ] {
        assert!(wire.get(key).is_none(), "{key} leaked onto the wire");
    }
}

fn write_artifacts(var: &str, name: &str, files: &[(&str, &Value)]) {
    let Ok(dir) = std::env::var(var) else { return };
    let dir = Path::new(&dir);
    std::fs::create_dir_all(dir).unwrap();
    let mut manifest = serde_json::Map::new();
    manifest.insert(
        "source".into(),
        json!("actual headless Value invocation; no native UI claim"),
    );
    for (suffix, value) in files {
        std::fs::write(
            dir.join(format!("{name}.{suffix}.json")),
            serde_json::to_vec_pretty(value).unwrap(),
        )
        .unwrap();
        manifest.insert(format!("{suffix}_digest"), json!(digest(value).unwrap()));
    }
    std::fs::write(
        dir.join(format!("{name}.manifest.json")),
        serde_json::to_vec_pretty(&Value::Object(manifest)).unwrap(),
    )
    .unwrap();
}

// ---------------------------------------------------------------- (1) load-reference-1

#[test]
fn load_reference_one_actual_solve_mints_bound_evidence_and_canonical_document_both_modes() {
    for stem in LR_STEMS {
        let payload = lr_request(stem);
        for mode in MODES {
            let ctx = format!("{stem}/{}", mode.as_str());
            let request = metadata(&payload);
            let output = solve(&payload, mode);
            let mechanics = output.mechanics_envelope.as_ref().unwrap();
            let raw = raw_of(&output);
            // Identity routing: the exact 0.4.0 route, never the preview default.
            assert_eq!(raw["producer"]["semantic_contract_id"], LR_ID, "{ctx}");
            assert_eq!(raw["formulation_basis"]["profile_id"], LR_PROFILE, "{ctx}");
            assert!(raw.get("source_block_recovery").is_none(), "{ctx}");
            assert_eq!(mechanics.status.mechanics, "MECHANICS_SOLVED", "{ctx}");
            assert_eq!(
                mechanics.numerical_quality.status,
                NumericalQualityStatus::ChecksPassed,
                "{ctx}"
            );
            // The runner does not alter the frozen producer envelope.
            let frozen = read(&format!(
                "fixtures/product_preview/load_reference/{stem}-{}.raw.json",
                mode.as_str()
            ));
            assert_eq!(raw, frozen, "{ctx}: actual producer changed");
            assert_eq!(
                output.runner_result.job.state,
                JobStateKind::Completed,
                "{ctx}"
            );
            assert!(output.runner_result.diagnostics.is_empty(), "{ctx}");
            assert!(output
                .runner_result
                .analysis_status
                .contains(&AnalysisStatus::MechanicsSolved));
            assert!(
                !validate_result(&output.runner_result).has_blocking_diagnostics(),
                "{ctx}"
            );
            // Standing: numerically eligible, with and without the invocation.
            let actual = invocation(&payload, mode);
            assert_eq!(
                semantic_contract::numerical_use_standing(&raw, &bases(&payload)),
                "numerically_eligible",
                "{ctx}"
            );
            assert_eq!(
                semantic_contract::numerical_use_standing_with_context(
                    &raw,
                    &bases(&payload),
                    Some(&actual)
                ),
                "numerically_eligible",
                "{ctx}"
            );
            // QualifiedPreviewEvidence as the binding defines it: the exact
            // retained request Value and the selected mode, digest-bound.
            assert!(output.canonical_export_unavailability.is_none(), "{ctx}");
            let proof = output
                .qualified_preview_evidence
                .as_ref()
                .expect("opaque proof");
            assert_eq!(proof.solve_payload, payload, "{ctx}");
            assert_eq!(proof.actual_invocation, actual, "{ctx}");
            assert_eq!(proof.solve_payload_digest, digest(&payload).unwrap());
            assert_eq!(proof.invocation_digest, digest(&actual).unwrap());
            assert_eq!(proof.mechanics_digest, digest(&raw).unwrap());
            assert_eq!(
                proof.runner_digest,
                digest(&serde_json::to_value(&output.runner_result).unwrap()).unwrap()
            );
            assert_eq!(
                proof.request_digest,
                digest(&serde_json::to_value(&request).unwrap()).unwrap()
            );
            assert_eq!(proof.run_id, output.runner_result.run_id);
            // The canonical document.
            let doc = output
                .result_envelope_document
                .as_ref()
                .expect("canonical document");
            let e = &doc["result_envelope"];
            assert_eq!(doc["schema_version"], "0.3.0", "{ctx}");
            assert_eq!(doc["deliverable_id"], "DEL-08-04", "{ctx}");
            assert_eq!(
                e["semantic_contract_ref"],
                json!({"ref_type":"semantic_contract","ref_id":LR_ID}),
                "{ctx}"
            );
            for key in [
                "producer",
                "numerical_quality",
                "formulation_basis",
                "contract_evidence",
            ] {
                assert_eq!(e[key], raw[key], "{ctx} {key}");
            }
            assert!(e.get("source_block_recovery").is_none(), "{ctx}");
            assert_eq!(
                e["row_accounting"].as_array().unwrap().len(),
                mechanics.results.len(),
                "{ctx}"
            );
            let origin = &e["reproducibility"]["source_origin_bindings"][0];
            assert_eq!(
                origin["received_carrier_checksum"]["value"],
                digest(&raw).unwrap(),
                "{ctx}"
            );
            assert_eq!(
                origin["qualification_ref"]["ref_id"],
                format!("{}:{}", proof.run_id, proof.invocation_digest),
                "{ctx}"
            );
            validate_document(doc, &raw).unwrap();
            let reloaded: Value =
                serde_json::from_slice(&serde_json::to_vec(&raw).unwrap()).unwrap();
            validate_document(doc, &reloaded).unwrap();
            assert!(
                !validate_result_with_optional_envelope_payload(&output.runner_result, Some(doc))
                    .has_blocking_diagnostics(),
                "{ctx}"
            );
            assert_eq!(
                *doc,
                build_result_export_document_with_evidence(
                    &request,
                    &output.runner_result,
                    mechanics,
                    proof
                )
                .unwrap(),
                "{ctx}"
            );
            // Without the opaque proof, typed callers never get the document.
            assert!(
                build_result_export_document(&request, &output.runner_result, mechanics).is_err(),
                "{ctx}"
            );
            // A document relabelled to another identity is refused by the runner.
            for foreign in [LRS_ID, PREVIEW_PHYSICS_ID] {
                let mut relabelled = doc.clone();
                relabelled["result_envelope"]["producer"]["semantic_contract_id"] = json!(foreign);
                assert!(
                    validate_result_with_optional_envelope_payload(
                        &output.runner_result,
                        Some(&relabelled)
                    )
                    .has_blocking_diagnostics(),
                    "{ctx} {foreign}"
                );
            }
            // The proof binds the actual invocation: mode, payload, source and runner.
            let mut mode_tamper = proof.clone();
            mode_tamper.actual_invocation["solver_mode"] = json!(other(mode).as_str());
            assert!(
                build_result_export_document_with_evidence(
                    &request,
                    &output.runner_result,
                    mechanics,
                    &mode_tamper
                )
                .is_err(),
                "{ctx}"
            );
            let mut payload_tamper = proof.clone();
            payload_tamper.solve_payload["model"]["project"]["description"] =
                json!("same-ID substituted after execution");
            assert!(
                build_result_export_document_with_evidence(
                    &request,
                    &output.runner_result,
                    mechanics,
                    &payload_tamper
                )
                .is_err(),
                "{ctx}"
            );
            // A re-digested invocation whose request is not the retained payload.
            let mut split = proof.clone();
            split.actual_invocation["request"]["model"]["project"]["description"] =
                json!("invocation differs from the retained payload");
            split.invocation_digest = digest(&split.actual_invocation).unwrap();
            assert!(
                build_result_export_document_with_evidence(
                    &request,
                    &output.runner_result,
                    mechanics,
                    &split
                )
                .is_err(),
                "{ctx}"
            );
            // A re-digested invocation naming a mode outside the closed set.
            let mut unknown_mode = proof.clone();
            unknown_mode.actual_invocation["solver_mode"] = json!("dense");
            unknown_mode.invocation_digest = digest(&unknown_mode.actual_invocation).unwrap();
            assert!(
                build_result_export_document_with_evidence(
                    &request,
                    &output.runner_result,
                    mechanics,
                    &unknown_mode
                )
                .is_err(),
                "{ctx}"
            );
            // Characterization, not extension: load-reference-1 carries no receipt,
            // so no received byte binds the invocation. The evidence's only custody
            // is its opacity (crate-private fields, no constructor or Deserialize).
            // A crate-internal forgery that re-digests a relabelled mode is not
            // detected by the binding, unlike the source-block receipt route; the
            // document it yields differs only in the qualification reference.
            // Observed behaviour, not a requirement: physics-1 and preview-physics-1
            // share it (the binding is identity-agnostic). ROOT routed it to T6
            // (reader and proof hardening); a T6 binding flips this assertion.
            mode_tamper.invocation_digest = digest(&mode_tamper.actual_invocation).unwrap();
            let relabelled = build_result_export_document_with_evidence(
                &request,
                &output.runner_result,
                mechanics,
                &mode_tamper,
            )
            .expect("receipt-less identity: digest-consistent private relabel is not detectable");
            assert_ne!(relabelled, *doc, "{ctx}");
            let mut normalized = relabelled.clone();
            normalized["result_envelope"]["reproducibility"]["source_origin_bindings"][0]
                ["qualification_ref"] = origin["qualification_ref"].clone();
            normalized["result_envelope"]["reproducibility"]["derivative_hash"] =
                e["reproducibility"]["derivative_hash"].clone();
            assert_eq!(normalized, *doc, "{ctx}");
            let mut changed = mechanics.clone();
            changed.results[0].value += 1.0;
            assert!(
                build_result_export_document_with_evidence(
                    &request,
                    &output.runner_result,
                    &changed,
                    proof
                )
                .is_err(),
                "{ctx}"
            );
            let mut altered_runner = output.runner_result.clone();
            altered_runner.checksums[0].algorithm = "forged".into();
            assert!(
                build_result_export_document_with_evidence(
                    &request,
                    &altered_runner,
                    mechanics,
                    proof
                )
                .is_err(),
                "{ctx}"
            );
            let mut altered_request = request.clone();
            altered_request.request_id = "substituted-request".into();
            assert!(
                build_result_export_document_with_evidence(
                    &altered_request,
                    &output.runner_result,
                    mechanics,
                    proof
                )
                .is_err(),
                "{ctx}"
            );
            assert_wire_excludes_library_evidence(&output);
            write_artifacts(
                "HEADLESS_LOAD_REFERENCE_OUTPUT_DIR",
                &format!("{stem}-{}", mode.as_str()),
                &[
                    ("request", &payload),
                    ("invocation", &actual),
                    ("raw", &raw),
                    ("document", doc),
                ],
            );
        }
    }
}

// ---------------------------------------------------------------- (2) load-reference-source-1

#[test]
fn joined_actual_solve_retains_invocation_bound_receipt_without_canonical_export_both_modes() {
    for stem in LRS_STEMS {
        let payload = lrs_request(stem);
        for mode in MODES {
            let ctx = format!("{stem}/{}", mode.as_str());
            let request = metadata(&payload);
            let output = solve(&payload, mode);
            let mechanics = output.mechanics_envelope.as_ref().unwrap();
            let raw = raw_of(&output);
            assert_eq!(raw["producer"]["semantic_contract_id"], LRS_ID, "{ctx}");
            assert_eq!(raw["formulation_basis"]["profile_id"], LRS_PROFILE, "{ctx}");
            assert_eq!(mechanics.status.mechanics, "MECHANICS_SOLVED", "{ctx}");
            assert_ne!(
                mechanics.numerical_quality.status,
                NumericalQualityStatus::ChecksPassed,
                "{ctx}"
            );
            let frozen = read(&format!(
                "fixtures/product_preview/load_reference_source/{stem}-{}.raw.json",
                mode.as_str()
            ));
            assert_eq!(raw, frozen, "{ctx}: actual producer changed");
            assert_eq!(
                output.runner_result.job.state,
                JobStateKind::Completed,
                "{ctx}"
            );
            assert!(output.runner_result.diagnostics.is_empty(), "{ctx}");
            assert!(
                !validate_result(&output.runner_result).has_blocking_diagnostics(),
                "{ctx}"
            );
            // The receipt is retained and bound to the actual invocation.
            let actual = invocation(&payload, mode);
            let receipt = &raw["source_block_recovery"];
            assert_eq!(receipt["body"]["policy"], LRS_POLICY, "{ctx}");
            assert_eq!(
                receipt["body"]["invocation"]["value"],
                source_blocks::domain_hash("source_blocks_invocation_v1", &actual).unwrap(),
                "{ctx}"
            );
            assert_ne!(
                receipt["body"]["invocation"]["value"],
                source_blocks::domain_hash(
                    "source_blocks_invocation_v1",
                    &invocation(&payload, other(mode))
                )
                .unwrap(),
                "{ctx}: the receipt names the mode that actually ran"
            );
            // The joined reader admits the received bytes and never grants eligibility.
            assert_eq!(
                load_reference_source::validate_load_reference_source_evidence(&raw),
                Ok(false),
                "{ctx}"
            );
            assert_eq!(
                semantic_contract::numerical_use_standing(&raw, &bases(&payload)),
                "needs_recompute",
                "{ctx}"
            );
            assert_eq!(
                semantic_contract::numerical_use_standing_with_context(
                    &raw,
                    &bases(&payload),
                    Some(&actual)
                ),
                "needs_recompute",
                "{ctx}"
            );
            // T1: no canonical document and no opaque proof for the joined route.
            assert!(output.result_envelope_document.is_none(), "{ctx}");
            assert!(output.qualified_preview_evidence.is_none(), "{ctx}");
            assert_eq!(
                output.canonical_export_unavailability.as_deref(),
                Some(NEEDS_RECOMPUTE_UNAVAILABLE),
                "{ctx}"
            );
            // Reparsed public receipt bytes are not the library's opaque proof.
            let reloaded: Value =
                serde_json::from_slice(&serde_json::to_vec(&raw).unwrap()).unwrap();
            assert_eq!(reloaded, raw, "{ctx}");
            let mut reparsed = mechanics.clone();
            reparsed.source_block_recovery = Some(reloaded["source_block_recovery"].clone());
            // T1_WAVE2_REVIEW F1: a crate-internal, digest-consistent proof over the
            // actual invocation (and over the reparsed receipt) still cannot mint a
            // joined document: the binding refuses on standing, never on a digest.
            for (label, source) in [("actual", mechanics), ("reparsed", &reparsed)] {
                let forged = crate::QualifiedPreviewEvidence {
                    solve_payload: payload.clone(),
                    solve_payload_digest: digest(&payload).unwrap(),
                    actual_invocation: actual.clone(),
                    invocation_digest: digest(&actual).unwrap(),
                    mechanics_digest: digest(&serde_json::to_value(source).unwrap()).unwrap(),
                    runner_digest: digest(&serde_json::to_value(&output.runner_result).unwrap())
                        .unwrap(),
                    request_digest: digest(&serde_json::to_value(&request).unwrap()).unwrap(),
                    run_id: output.runner_result.run_id.clone(),
                };
                let refusal = build_result_export_document_with_evidence(
                    &request,
                    &output.runner_result,
                    source,
                    &forged,
                )
                .expect_err(&format!("{ctx} {label}: joined document minted"));
                assert!(
                    format!("{refusal:?}").contains("CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE"),
                    "{ctx} {label}: {refusal:?}"
                );
            }
            assert_wire_excludes_library_evidence(&output);
            write_artifacts(
                "HEADLESS_LOAD_REFERENCE_SOURCE_OUTPUT_DIR",
                &format!("{stem}-{}", mode.as_str()),
                &[
                    ("request", &payload),
                    ("invocation", &actual),
                    ("raw", &raw),
                ],
            );
        }
    }
}

// ---------------------------------------------------------------- (3) refusals

fn assert_blocked_not_published(output: &PreviewRunnerOutput, ctx: &str) -> Value {
    let raw = raw_of(output);
    assert_eq!(raw["status"]["mechanics"], "MODEL_INCOMPLETE", "{ctx}");
    assert!(raw["results"].as_array().unwrap().is_empty(), "{ctx}");
    assert!(raw.get("source_block_recovery").is_none(), "{ctx}");
    assert!(output.runner_result.result_refs.is_empty(), "{ctx}");
    assert!(
        !output
            .runner_result
            .analysis_status
            .contains(&AnalysisStatus::MechanicsSolved),
        "{ctx}"
    );
    assert!(
        output
            .runner_result
            .analysis_status
            .contains(&AnalysisStatus::ModelIncomplete),
        "{ctx}"
    );
    assert!(
        validate_result(&output.runner_result).has_blocking_diagnostics(),
        "{ctx}: must not validate as a clean result"
    );
    assert!(output.result_envelope_document.is_none(), "{ctx}");
    assert!(output.qualified_preview_evidence.is_none(), "{ctx}");
    assert_eq!(
        output.canonical_export_unavailability.as_deref(),
        Some("SOURCE_NOT_SOLVED"),
        "{ctx}"
    );
    raw
}

#[test]
fn explicit_null_on_a_load_state_key_is_a_targeted_blocked_load_reference_envelope() {
    for stem in LR_STEMS {
        let mut payload = lr_request(stem);
        payload["model"]["materials"][0]["expansion_laws"] = Value::Null;
        for mode in MODES {
            let ctx = format!("explicit null {stem}/{}", mode.as_str());
            let raw = assert_blocked_not_published(&solve(&payload, mode), &ctx);
            assert!(
                blocking_codes(&raw).contains(&"LOAD_STATE_EXPLICIT_NULL_UNSUPPORTED".to_string()),
                "{ctx}: {:?}",
                blocking_codes(&raw)
            );
            assert_eq!(raw["producer"]["semantic_contract_id"], LR_ID, "{ctx}");
            assert_eq!(raw["formulation_basis"]["profile_id"], LR_PROFILE, "{ctx}");
        }
    }
    // A null on a required 0.4.0 key is a typed request-boundary refusal.
    let mut required = lr_request("connected");
    required["model"]["load_cases"][0]["analysis_state"]["history"] = Value::Null;
    for mode in MODES {
        let error = run_preview_model_value_with_mode(metadata(&required), required.clone(), mode)
            .err()
            .unwrap_or_else(|| panic!("required null/{mode:?}: must not produce an envelope"));
        assert!(error.contains("invalid type: null"), "{mode:?}: {error}");
    }
    // A joined witness refuses the same way, on the exact namespace.
    let mut payload = lrs_request("eigen_motion");
    payload["model"]["materials"][0]["expansion_laws"] = Value::Null;
    for mode in MODES {
        let ctx = format!("explicit null joined/{}", mode.as_str());
        let raw = assert_blocked_not_published(&solve(&payload, mode), &ctx);
        assert!(
            blocking_codes(&raw).contains(&"LOAD_STATE_EXPLICIT_NULL_UNSUPPORTED".to_string()),
            "{ctx}: {:?}",
            blocking_codes(&raw)
        );
        assert_eq!(raw["producer"]["semantic_contract_id"], LR_ID, "{ctx}");
    }
}

#[test]
fn unknown_load_state_field_is_refused_at_the_request_boundary_before_any_publication() {
    let mut in_state = lr_request("connected");
    in_state["model"]["load_cases"][0]["analysis_state"]["unreviewed_field"] = json!(true);
    let mut in_configuration = lr_request("connected");
    in_configuration["model"]["reference_configurations"][0]["unreviewed_field"] = json!(true);
    for (label, payload, field) in [
        ("analysis_state", in_state, "unreviewed_field"),
        (
            "reference_configuration",
            in_configuration,
            "unreviewed_field",
        ),
    ] {
        for mode in MODES {
            let error =
                run_preview_model_value_with_mode(metadata(&payload), payload.clone(), mode)
                    .err()
                    .unwrap_or_else(|| {
                        panic!("{label}/{mode:?}: an unknown field must not produce an envelope")
                    });
            assert!(
                error.contains(&format!("unknown field `{field}`")),
                "{label}/{mode:?}: {error}"
            );
        }
    }
}

#[test]
fn pre_0_4_documents_carrying_load_state_keys_block_on_their_own_route() {
    // With the exact pressure contract: the 0.3.0 exact route (physics-1).
    let mut exact = lr_request("connected");
    exact["model"]["schema_version"] = json!("0.3.0");
    // Without it: the ordinary pre-0.4 default route (preview-physics-1).
    let mut ordinary = exact.clone();
    ordinary["model"]
        .as_object_mut()
        .unwrap()
        .remove("pressure_contract");
    for (label, payload, identity) in [
        ("exact contract", exact, PHYSICS_ID),
        ("no contract", ordinary, PREVIEW_PHYSICS_ID),
    ] {
        for mode in MODES {
            let ctx = format!("0.3.0 {label}/{}", mode.as_str());
            let raw = assert_blocked_not_published(&solve(&payload, mode), &ctx);
            assert!(
                blocking_codes(&raw).contains(&"LOAD_STATE_CONTRACT_VERSION_MISMATCH".to_string()),
                "{ctx}: {:?}",
                blocking_codes(&raw)
            );
            assert_eq!(raw["producer"]["semantic_contract_id"], identity, "{ctx}");
            assert_ne!(raw["producer"]["semantic_contract_id"], LR_ID, "{ctx}");
            assert_ne!(raw["producer"]["semantic_contract_id"], LRS_ID, "{ctx}");
        }
    }
}

// ---------------------------------------------------------------- (4) blocked 0.4.0

#[test]
fn blocked_0_4_0_envelopes_keep_the_load_reference_identity_through_the_runner() {
    let mut missing = lr_request("connected");
    missing["model"]
        .as_object_mut()
        .unwrap()
        .remove("pressure_contract");
    let mut legacy = lr_request("connected");
    legacy["model"]["pressure_contract"] = json!({"version": "1.0.0", "mode": "legacy"});
    let mut joined_missing = lrs_request("eigen_motion");
    joined_missing["model"]
        .as_object_mut()
        .unwrap()
        .remove("pressure_contract");
    for (label, payload) in [
        ("missing contract", missing),
        ("legacy contract", legacy),
        ("joined witness, missing contract", joined_missing),
    ] {
        for mode in MODES {
            let ctx = format!("{label}/{}", mode.as_str());
            let raw = assert_blocked_not_published(&solve(&payload, mode), &ctx);
            assert!(!blocking_codes(&raw).is_empty(), "{ctx}");
            assert_eq!(raw["producer"]["semantic_contract_id"], LR_ID, "{ctx}");
            assert_ne!(
                raw["producer"]["semantic_contract_id"], PREVIEW_PHYSICS_ID,
                "{ctx}"
            );
            assert_eq!(raw["formulation_basis"]["profile_id"], LR_PROFILE, "{ctx}");
            assert_eq!(
                raw["contract_evidence"],
                json!({"pressure": [], "connector": [], "exact_cases": [], "load_reference_states": []}),
                "{ctx}"
            );
        }
    }
}

// ---------------------------------------------------------------- (5) SF-1 fallback

/// The CP3 reviewer's probe P9, as in
/// `core/product_physics/src/source_receipt/load_state_fallback_tests.rs`:
/// `extra` declared 1e-6 N tip forces (then a moment) added to the committed
/// joined witness, raising the live join work past the reservation.
fn with_extra_loads(mut request: Value, extra: usize) -> Value {
    let case = &mut request["model"]["load_cases"][0];
    for (i, dir) in ["UZ", "UY", "RY"].iter().take(extra).enumerate() {
        let mut load = case["primitive_loads"][0].clone();
        let id = format!("extra:{i}");
        load["id"] = json!(id);
        load["direction"] = json!(dir);
        if !dir.starts_with('R') {
            load["category"] = json!("concentrated_force");
            load["dimension"] = json!("force");
            load["magnitude"] = json!({"value": 1.0e-6, "unit": "N"});
        }
        case["primitive_loads"].as_array_mut().unwrap().push(load);
        case["analysis_state"]["load_sources"]
            .as_array_mut()
            .unwrap()
            .push(json!({"source_ref": id, "factor": 1.0}));
    }
    request
}

fn sf1_budget_cliff_request(extra: usize) -> Value {
    with_extra_loads(lrs_request("eigen_motion"), extra)
}

#[test]
fn sf1_budget_cliff_publishes_the_ordinary_route_through_the_runner_not_a_blocked_result() {
    for extra in 1..=3 {
        let payload = sf1_budget_cliff_request(extra);
        for mode in MODES {
            let ctx = format!("SF-1 extra={extra}/{}", mode.as_str());
            let output = solve(&payload, mode);
            let raw = raw_of(&output);
            assert_eq!(raw["status"]["mechanics"], "MECHANICS_SOLVED", "{ctx}");
            assert!(
                blocking_codes(&raw).is_empty(),
                "{ctx}: {:?}",
                blocking_codes(&raw)
            );
            assert!(!raw["results"].as_array().unwrap().is_empty(), "{ctx}");
            assert_eq!(raw["producer"]["semantic_contract_id"], LR_ID, "{ctx}");
            assert_eq!(raw["formulation_basis"]["profile_id"], LR_PROFILE, "{ctx}");
            assert!(
                raw.get("source_block_recovery").is_none(),
                "{ctx}: no receipt on the ordinary route"
            );
            let unavailable: Vec<&Value> = raw["diagnostics"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|d| d["code"] == "SOURCE_BLOCK_RECOVERY_UNAVAILABLE")
                .collect();
            assert_eq!(unavailable.len(), 1, "{ctx}");
            let message = unavailable[0]["message"].as_str().unwrap();
            assert!(
                message.contains("captured replay reservation")
                    && message.contains("Exact(Budget)"),
                "{ctx}: {message}"
            );
            for record in raw["contract_evidence"]["load_reference_states"]
                .as_array()
                .unwrap()
            {
                assert_eq!(record["source_recovery"]["status"], "not_joined", "{ctx}");
                assert!(
                    record["solve"]["recovery_method"]
                        .as_str()
                        .unwrap()
                        .starts_with("ordinary_"),
                    "{ctx}"
                );
            }
            // The runner publishes the product's ordinary envelope unchanged.
            let direct = run_linear_static_preview_value_with_mode(payload.clone(), mode).unwrap();
            assert_eq!(raw, serde_json::to_value(&direct).unwrap(), "{ctx}");
            assert_eq!(
                output.runner_result.job.state,
                JobStateKind::Completed,
                "{ctx}"
            );
            assert!(output.runner_result.diagnostics.is_empty(), "{ctx}");
            assert!(
                output
                    .runner_result
                    .analysis_status
                    .contains(&AnalysisStatus::MechanicsSolved),
                "{ctx}"
            );
            assert!(
                !validate_result(&output.runner_result).has_blocking_diagnostics(),
                "{ctx}"
            );
            // Sensitive ordinary evidence is inspectable, never promoted.
            assert_eq!(raw["numerical_quality"]["status"], "sensitive", "{ctx}");
            assert_eq!(
                semantic_contract::numerical_use_standing(&raw, &bases(&payload)),
                "needs_recompute",
                "{ctx}"
            );
            assert!(output.result_envelope_document.is_none(), "{ctx}");
            assert!(output.qualified_preview_evidence.is_none(), "{ctx}");
            assert_eq!(
                output.canonical_export_unavailability.as_deref(),
                Some(NEEDS_RECOMPUTE_UNAVAILABLE),
                "{ctx}"
            );
        }
    }
}
