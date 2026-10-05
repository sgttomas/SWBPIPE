//! I61 U3 grant 1b, R-2 condition (disposable lane archive only, never committed):
//! runner/headless carries a base preview-physics-1 publication with one info
//! RETAINED_PRECISION_UNAVAILABLE notice through its actual Value route. The probe
//! repeats `run_preview_model_value_dispatch` with the producer's envelope plus
//! the notice, and compares every runner outcome with the unchanged run.
use super::*;
use open_pipe_stress_result_export::derivative::validate_document;
use open_pipe_stress_result_export::semantic_contract as sc;

const PLAIN: &str = "Retained-precision recovery is unavailable for this load case. Its published rows keep their ordinary values, standing and diagnostics.";

fn request() -> RunnerRequest {
    RunnerRequest {
        request_id: "request-1".to_string(), operation: RunnerOperation::Solve,
        operation_ref: Reference::new("api_operation", "ops.solve.job"), project_ref: Reference::new("project", "invented-project"),
        model_ref: Reference::new("model", "invented-model"), unit_system_ref: Reference::new("unit_system", "invented-si"),
        load_basis_refs: vec![Reference::new("load_case", "case")], input_manifest_ref: Reference::new("audit_manifest", "manifest-1"),
        requested_outputs: vec!["result_envelope".to_string(), "audit_manifest".to_string(), "diagnostics".to_string()],
        privacy: PrivacyContext::local_first_public_metadata(),
        provenance: Provenance { source_name: "invented headless runner fixture".to_string(), source_location: "validation/benchmarks/invented".to_string(),
            source_license: "project invented".to_string(), contributor: "OpenPipeStress".to_string(),
            contributor_certification: "invented non-engineering example".to_string(),
            redistribution_status: RedistributionStatus::InventedNonEngineeringExample, review_status: "accepted".to_string() },
        professional_boundary: ProfessionalBoundary::project_default(), tbd_decisions: TbdDecisions::d33_local_cli_policy(),
    }
}

/// `run_preview_model_value_dispatch`'s body with an optional appended notice.
fn run(solve_payload: Value, mode: PreviewSolverMode, notice: Option<&str>) -> PreviewRunnerOutput {
    let case = solve_payload["model"]["load_cases"][0]["id"].as_str().unwrap().to_owned();
    let request = request();
    let actual_invocation = serde_json::json!({"request": &solve_payload, "solver_mode": mode.as_str()});
    let retained_request = request.clone();
    let mut output = run_preview_with_producer(request, None, || {
        let mut envelope = run_linear_static_preview_value_with_mode(solve_payload.clone(), mode)?;
        if let Some(message) = notice {
            envelope.diagnostics.push(open_pipe_stress_product_physics::Diagnostic {
                id: format!("diagnostic:retained-precision:{case}:unavailable"), code: "RETAINED_PRECISION_UNAVAILABLE".into(),
                severity: "info".into(), message: message.into(), source: Some("core/product_physics".into()), affected_refs: vec![case.clone()] });
        }
        Ok(envelope)
    }).unwrap();
    let qualified = || -> Result<QualifiedPreviewEvidence, String> {
        use open_pipe_stress_result_export::derivative::{digest, guard_json};
        guard_json(&solve_payload)?;
        let mechanics = output.mechanics_envelope.as_ref().ok_or("SOURCE_UNAVAILABLE")?;
        if mechanics.status.mechanics != "MECHANICS_SOLVED" { return Err("SOURCE_NOT_SOLVED".into()); }
        if solve_payload["model"]["project"]["id"] != mechanics.model_ref { return Err("SOURCE_MODEL_IDENTITY_MISMATCH".into()); }
        Ok(QualifiedPreviewEvidence { solve_payload_digest: digest(&solve_payload)?, invocation_digest: digest(&actual_invocation)?, actual_invocation: actual_invocation.clone(),
            solve_payload: solve_payload.clone(), mechanics_digest: digest(&serde_json::to_value(mechanics).map_err(|e| e.to_string())?)?,
            runner_digest: digest(&serde_json::to_value(&output.runner_result).map_err(|e| e.to_string())?)?,
            request_digest: digest(&serde_json::to_value(&retained_request).map_err(|e| e.to_string())?)?, run_id: output.runner_result.run_id.clone() })
    }();
    match qualified {
        Ok(evidence) => match result_envelope_binding::build_result_export_document_with_evidence(&retained_request, &output.runner_result, output.mechanics_envelope.as_ref().unwrap(), &evidence) {
            Ok(doc) => { output.result_envelope_document = Some(doc); output.canonical_export_unavailability = None; output.qualified_preview_evidence = Some(evidence); }
            Err(d) => output.canonical_export_unavailability = Some(d.message),
        },
        Err(reason) => output.canonical_export_unavailability = Some(reason),
    }
    output
}

#[test]
fn i61_r2_runner_carries_the_unavailable_notice() {
    let milestone: Value = serde_json::from_str(include_str!("../../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json")).unwrap();
    // An exportable preview-physics-1 solve (the maintained runner test's zero-pressure
    // invented model), so the canonical export path itself carries the notice.
    let mut exportable: Value = serde_json::json!({"model": serde_json::from_str::<Value>(include_str!(
        "../../../product_physics/tests/fixtures/preview_physics_invented_model.json")).unwrap(), "materials": []});
    for case in exportable["model"]["load_cases"].as_array_mut().unwrap() {
        for load in case["primitive_loads"].as_array_mut().unwrap() {
            if load["category"] == "pressure" || load["dimension"] == "pressure" { load["magnitude"]["value"] = serde_json::json!(0.0); }
        }
    }
    let mut lines = Vec::new();
    let mut minted = 0;
    for (label, payload) in [("milestone", milestone), ("exportable", exportable)] {
    for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
        // The probe's copy of the dispatch equals the maintained route on the unchanged envelope.
        let maintained = run_preview_model_value_with_mode(request(), payload.clone(), mode).unwrap();
        let base = run(payload.clone(), mode, None);
        assert_eq!(serde_json::to_value(&base).unwrap(), serde_json::to_value(&maintained).unwrap(), "probe = maintained route");
        assert_eq!(base.result_envelope_document, maintained.result_envelope_document);
        let base_raw = serde_json::to_value(base.mechanics_envelope.as_ref().unwrap()).unwrap();
        let receipt = format!("{PLAIN} Reason: receipt_encoding; detail: work_counter_inconsistent.");
        for (kind, message) in [("plain", PLAIN), ("receipt", receipt.as_str())] {
            let noticed = run(payload.clone(), mode, Some(message));
            let raw = serde_json::to_value(noticed.mechanics_envelope.as_ref().unwrap()).unwrap();
            assert_eq!(raw["diagnostics"].as_array().unwrap().len(), base_raw["diagnostics"].as_array().unwrap().len() + 1);
            // The runner's own result and its validation are unchanged apart from the source checksum.
            assert_eq!(noticed.runner_result.analysis_status, base.runner_result.analysis_status);
            assert_eq!(noticed.runner_result.diagnostics, base.runner_result.diagnostics);
            assert_eq!(noticed.runner_result.job, base.runner_result.job);
            let validation = validate_result_with_optional_envelope_payload(&noticed.runner_result, None);
            assert!(!validation.has_blocking_diagnostics(), "{:?}", validation.diagnostics);
            // Canonical export availability is the base's, exactly (the milestone is
            // Sensitive, so neither mints a Current export document).
            assert_eq!(noticed.canonical_export_unavailability, base.canonical_export_unavailability);
            assert_eq!(noticed.result_envelope_document.is_some(), base.result_envelope_document.is_some());
            assert_eq!(noticed.qualified_preview_evidence.is_some(), base.qualified_preview_evidence.is_some());
            if let Some(doc) = noticed.result_envelope_document.as_ref() {
                validate_document(doc, &raw).unwrap();
                let payload_validation = validate_result_with_optional_envelope_payload(&noticed.runner_result, Some(doc));
                let base_payload_validation = validate_result_with_optional_envelope_payload(&base.runner_result, base.result_envelope_document.as_ref());
                assert_eq!(payload_validation.diagnostics, base_payload_validation.diagnostics);
                let carries = doc.to_string().contains("RETAINED_PRECISION_UNAVAILABLE");
                let base_doc = base.result_envelope_document.as_ref().unwrap();
                println!("I61_R2_RUNNER_DOC {label} {} {kind} carries_notice={carries} base_doc_diagnostic_count={} noticed_doc_diagnostic_count={}", mode.as_str(),
                    base_doc.to_string().matches("\"severity\"").count(), doc.to_string().matches("\"severity\"").count());
                minted += 1;
                lines.push(format!("I61_R2_RUNNER {label} {} {kind} export_document=minted validate_document=ok payload_blocking={}", mode.as_str(), payload_validation.has_blocking_diagnostics()));
            }
            assert_eq!(sc::for_source(&raw), sc::for_source(&base_raw));
            assert_eq!(sc::numerical_use_standing(&raw, &[]), sc::numerical_use_standing(&base_raw, &[]));
            lines.push(format!("I61_R2_RUNNER {label} {} {kind} runner_blocking={} export={:?} (base {:?}) qualified_evidence={} (base {}) for_source=ok standing={}",
                mode.as_str(), validation.has_blocking_diagnostics(), noticed.canonical_export_unavailability, base.canonical_export_unavailability,
                noticed.qualified_preview_evidence.is_some(), base.qualified_preview_evidence.is_some(), sc::numerical_use_standing(&raw, &[])));
        }
    }
    }
    for line in &lines { println!("{line}"); }
    assert_eq!(minted, 4, "the exportable solve minted its export with the notice in both modes");
}
