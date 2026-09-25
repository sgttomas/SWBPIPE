//! Version-dispatched canonical derivative assembly consumes opaque same-Value actual-solve evidence.
//! Raw MechanicsEnvelope/RunnerResult transport and legacy analysis hashes remain independent.
//! Rows use the complete shared semantic contract; unsupported rows are retained in disclosures.

use open_pipe_stress_product_physics::{
    nonlinear_assembled_loop_context, solver_component_name, solver_component_version,
    Diagnostic as MechanicsDiagnostic, MechanicsEnvelope,
};
use open_pipe_stress_result_export as export;
use serde_json::Value;

use crate::{
    AnalysisStatus, ChecksumRef, Diagnostic,
    Provenance, RedistributionStatus, Reference, RunnerRequest, RunnerResult,
};

/// Blocking runner-diagnostic code for a STRUCTURAL envelope production or
/// validation failure on an otherwise-completed solve (brief §3.5).
pub const RESULT_ENVELOPE_PRODUCTION_FAILED_CODE: &str =
    "HEADLESS_RUNNER_RESULT_ENVELOPE_PRODUCTION_FAILED";

/// Non-blocking per-row vocabulary-boundary disclosure code, carried in the
/// envelope document's own diagnostics (brief §3.2 v3). Never a runner
/// diagnostic; never changes exit codes or serialized CLI output.
pub const VOCABULARY_BOUNDARY_DISCLOSURE_CODE: &str =
    "HEADLESS_RUNNER_ENVELOPE_VOCABULARY_BOUNDARY_ROW";

/// Non-blocking assumption/limitation context codes for the assembled
/// nonlinear active-set loop (brief §3.3).
pub const NONLINEAR_LOOP_ASSUMPTION_CODE: &str = "NONLINEAR_ASSEMBLED_LOOP_ASSUMPTION";
pub const NONLINEAR_LOOP_LIMITATION_CODE: &str = "NONLINEAR_ASSEMBLED_LOOP_LIMITATION";

/// Schema version of the produced result envelope, matching the committed
/// DEL-08-04 fixtures and `schemas/results.schema.yaml` version pattern.
const RESULT_ENVELOPE_SCHEMA_VERSION: &str = "0.1.0";

/// Deterministic detection that the solve exercised (attempted or completed)
/// the nonlinear active-set loop, from envelope content only.
fn nonlinear_exercised(mechanics: &MechanicsEnvelope) -> bool {
    mechanics
        .results
        .iter()
        .any(|row| row.kind.starts_with("nonlinear_support_"))
        || mechanics
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code.starts_with("NONLINEAR_SUPPORT"))
}

fn export_reference(reference: &Reference) -> export::Reference {
    export::Reference::new(reference.ref_type.clone(), reference.ref_id.clone())
}

fn export_redistribution(status: RedistributionStatus) -> export::RedistributionStatus {
    match status {
        RedistributionStatus::PublicPermissive => export::RedistributionStatus::PublicPermissive,
        RedistributionStatus::PrivateOnly => export::RedistributionStatus::PrivateOnly,
        RedistributionStatus::Unknown => export::RedistributionStatus::Unknown,
        RedistributionStatus::ProtectedSuspected => {
            export::RedistributionStatus::ProtectedSuspected
        }
        RedistributionStatus::InventedNonEngineeringExample => {
            export::RedistributionStatus::InventedNonEngineeringExample
        }
        RedistributionStatus::Tbd => export::RedistributionStatus::Tbd,
    }
}

fn export_provenance(provenance: &Provenance) -> export::Provenance {
    export::Provenance {
        source_name: provenance.source_name.clone(),
        source_location: provenance.source_location.clone(),
        source_license: provenance.source_license.clone(),
        contributor: provenance.contributor.clone(),
        contributor_certification: provenance.contributor_certification.clone(),
        redistribution_status: export_redistribution(provenance.redistribution_status),
        review_status: provenance.review_status.clone(),
    }
}

fn export_checksum(checksum: &ChecksumRef) -> export::ChecksumRef {
    export::ChecksumRef {
        algorithm: checksum.algorithm.clone(),
        canonicalization: checksum.canonicalization.clone(),
        payload_ref: export_reference(&checksum.payload_ref),
        value: checksum.value.clone(),
    }
}

fn export_analysis_status(status: AnalysisStatus) -> export::AnalysisStatus {
    match status {
        AnalysisStatus::ModelIncomplete => export::AnalysisStatus::ModelIncomplete,
        AnalysisStatus::MechanicsSolved => export::AnalysisStatus::MechanicsSolved,
        AnalysisStatus::RuleInputsIncomplete => export::AnalysisStatus::RuleInputsIncomplete,
        AnalysisStatus::UserRuleChecked => export::AnalysisStatus::UserRuleChecked,
        AnalysisStatus::UserRuleFailed => export::AnalysisStatus::UserRuleFailed,
        AnalysisStatus::HumanReviewRequired => export::AnalysisStatus::HumanReviewRequired,
    }
}



/// Producer-origin provenance for disclosure metadata (diagnostic metadata
/// only, following the existing validator invented-provenance precedent).


/// Provenance naming the assembled-loop source crate for the nonlinear
/// assumption/limitation context rows (brief §3.3).
fn nonlinear_context_provenance(component_name: &str) -> export::Provenance {
    export::Provenance {
        source_name: component_name.to_string(),
        source_location: "core/solver/nonlinear_integration".to_string(),
        source_license: "project".to_string(),
        contributor: "OpenPipeStress".to_string(),
        contributor_certification: "assembled-loop context metadata only".to_string(),
        redistribution_status: export::RedistributionStatus::InventedNonEngineeringExample,
        review_status: "accepted".to_string(),
    }
}

/// Deterministic severity mapping for solve diagnostics. Unknown severity
/// strings are conservatively mapped to blocking (never downgraded).
fn export_severity(severity: &str) -> export::DiagnosticSeverity {
    match severity {
        "info" => export::DiagnosticSeverity::Info,
        "warning" => export::DiagnosticSeverity::Warning,
        _ => export::DiagnosticSeverity::Blocking,
    }
}

/// Deterministic diagnostic-class mapping for solve diagnostics, following
/// the existing `core/analysis_runs/records.py::_diagnostic_class` precedent
/// extended with the nonlinear class.
fn export_diagnostic_class(code: &str, severity: &str) -> export::DiagnosticClass {
    if code.contains("RULE") {
        if severity == "blocking" {
            return export::DiagnosticClass::RuleCheckBlocking;
        }
        return export::DiagnosticClass::AssumptionWarning;
    }
    if code.contains("UNIT") {
        return export::DiagnosticClass::UnitWarning;
    }
    if code.contains("PROVENANCE") {
        return export::DiagnosticClass::ProvenanceWarning;
    }
    if severity != "info" && severity != "warning" {
        return export::DiagnosticClass::SolveBlocking;
    }
    if code.starts_with("NONLINEAR") {
        return export::DiagnosticClass::NonlinearWarning;
    }
    export::DiagnosticClass::AssumptionWarning
}

/// Map one solve diagnostic into the envelope, preserving its source string
/// in the diagnostic provenance (brief §3.2: source preserved).
fn export_mechanics_diagnostic(diagnostic: &MechanicsDiagnostic) -> export::Diagnostic {
    let source_name = diagnostic
        .source
        .clone()
        .unwrap_or_else(|| "core/product_physics".to_string());
    export::Diagnostic {
        code: diagnostic.code.clone(),
        class: export_diagnostic_class(&diagnostic.code, &diagnostic.severity),
        severity: export_severity(&diagnostic.severity),
        source: export::Reference::new("solver_component", source_name.clone()),
        affected_object: export::Reference::new("preview_diagnostic", diagnostic.id.clone()),
        message: diagnostic.message.clone(),
        remediation:
            "Review the carried solve diagnostic before relying on this exported result envelope."
                .to_string(),
        provenance: export::Provenance {
            source_name,
            source_location: "core/product_physics".to_string(),
            source_license: "project".to_string(),
            contributor: "OpenPipeStress".to_string(),
            contributor_certification: "solve diagnostic metadata only".to_string(),
            redistribution_status: export::RedistributionStatus::InventedNonEngineeringExample,
            review_status: "accepted".to_string(),
        },
    }
}

/// Per-row NON-BLOCKING vocabulary-boundary disclosure (brief §3.2 v3).


fn nonlinear_context_diagnostic(
    code: &str,
    class: export::DiagnosticClass,
    severity: export::DiagnosticSeverity,
    component_name: &str,
    envelope_id: &str,
    message: &str,
) -> export::Diagnostic {
    export::Diagnostic {
        code: code.to_string(),
        class,
        severity,
        source: export::Reference::new("solver_component", component_name.to_string()),
        affected_object: export::Reference::new("result_envelope", envelope_id.to_string()),
        message: message.to_string(),
        remediation: "Review the assembled active-set loop context before relying on the \
                      nonlinear result rows of this envelope."
            .to_string(),
        provenance: nonlinear_context_provenance(component_name),
    }
}

fn production_failed(envelope_id: &str, detail: String) -> Diagnostic {
    Diagnostic::runner_blocking(
        RESULT_ENVELOPE_PRODUCTION_FAILED_CODE,
        Reference::new("result_envelope", envelope_id),
        format!("result-envelope production failed structurally: {detail}"),
    )
}

/// Build the DEL-08-04 result-export envelope document (the exact
/// `result_export_document` wrapper shape, `deliverable_id: "DEL-08-04"`)
/// from a completed preview solve, using only `open_pipe_stress_result_export`
/// public vocabulary (brief §3.1).
///
/// Returns the document on the clean path (both validators free of blocking
/// diagnostics), or one blocking runner diagnostic on a STRUCTURAL production
/// or validation failure for the caller to append fail-closed (brief §3.5).
/// The per-row vocabulary-boundary disclosure is NOT a failure: it rides the
/// document's own diagnostics with non-blocking severity.
/// Typed legacy callers lack the exact authored solve payload. Raw result
/// transport remains available; canonical derivative export is unavailable.
pub fn build_result_export_document(_request: &RunnerRequest, runner_result: &RunnerResult, mechanics: &MechanicsEnvelope) -> Result<Value, Diagnostic> {
    Err(production_failed(&runner_result.result_envelope_ref.envelope_ref.ref_id,
        if mechanics.status.mechanics != "MECHANICS_SOLVED" { "SOURCE_NOT_SOLVED" } else { "EXACT_SOLVED_MODEL_EVIDENCE_UNAVAILABLE" }.to_string()))
}

/// Consume evidence minted only by the model-aware actual solve route.
pub fn build_result_export_document_with_evidence(request: &RunnerRequest, runner_result: &RunnerResult, mechanics: &MechanicsEnvelope, evidence: &crate::QualifiedPreviewEvidence) -> Result<Value, Diagnostic> {
    use export::derivative::{checksum, digest, reference, derive_document};
    let envelope_id=&runner_result.result_envelope_ref.envelope_ref.ref_id;
    let make=|| -> Result<Value,String> {
        let source=serde_json::to_value(mechanics).map_err(|e|e.to_string())?;
        let request_value=serde_json::to_value(request).map_err(|e|e.to_string())?;
        if digest(&evidence.solve_payload)? != evidence.solve_payload_digest || digest(&source)? != evidence.mechanics_digest || digest(&serde_json::to_value(runner_result).map_err(|e|e.to_string())?)? != evidence.runner_digest || digest(&request_value)? != evidence.request_digest || evidence.run_id != runner_result.run_id {
            return Err("SOLVED_SOURCE_BINDING_MISMATCH".into());
        }
        let model=&evidence.solve_payload["model"];
        if model["project"]["id"] != source["model_ref"] { return Err("SOURCE_MODEL_IDENTITY_MISMATCH".into()); }
        // Authentic computation and numerical qualification are separate facts.
        let cases = model["load_cases"].as_array().ok_or("REQUESTED_NUMERICAL_BASIS_UNAVAILABLE")?;
        let requested: Vec<Value> = cases.iter().map(|case| {
            case["id"].as_str().filter(|id| !id.is_empty())
                .map(|id| reference("load_case", id)).ok_or("REQUESTED_NUMERICAL_BASIS_UNAVAILABLE")
        }).collect::<Result<_, _>>()?;
        if export::semantic_contract::numerical_use_standing(&source, &requested) != "numerically_eligible" {
            return Err("CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE".into());
        }
        let payload_ref=reference("attested_headless_producer",envelope_id);
        let carrier=checksum(&source,"attested_headless_producer_carrier",payload_ref)?;
        let actual_ref=reference("model_payload",mechanics.model_ref.as_str());
        let alias=if request.model_ref.ref_id!=mechanics.model_ref {serde_json::json!(format!("request model_ref {} is a provenance alias; actual model {}",request.model_ref.ref_id,mechanics.model_ref))}else{Value::Null};
        let origin=serde_json::json!({"origin_id":"source-origin:headless-actual-solve","origin_class":"attested_headless_producer","qualification_ref":reference("private_solve_evidence",&format!("{}:{}",evidence.run_id,evidence.solve_payload_digest)),"authentic_producer_available":true,"received_carrier_checksum":carrier,"original_producer_checksum":carrier,"origin_limit":"Private same-Value actual solve evidence; exact solve payload incl material overrides retained; no model reconstruction","actual_model_ref":actual_ref,"mechanics_run_ref":reference("mechanics_run",&mechanics.run_id),"request_model_ref":export_reference_json(&request.model_ref),"request_run_ref":reference("runner_run",&runner_result.run_id),"request_alias_disclosure":alias});
        let base=build_base_result_export_document(request,runner_result,mechanics).map_err(|e|e.message)?;
        derive_document(base,model,&source,origin,Some(&request_value))
    };
    make().map_err(|detail|production_failed(envelope_id,detail))
}
fn export_reference_json(r:&Reference)->Value {serde_json::json!({"ref_type":r.ref_type,"ref_id":r.ref_id})}

fn build_base_result_export_document(
    request: &RunnerRequest,
    runner_result: &RunnerResult,
    mechanics: &MechanicsEnvelope,
) -> Result<Value, Diagnostic> {
    let envelope_id = runner_result
        .result_envelope_ref
        .envelope_ref
        .ref_id
        .clone();
    let run_id = runner_result.run_id.clone();
    let run_basis = export::Reference::new("analysis_run", run_id.clone());
    let envelope_provenance = export_provenance(&runner_result.provenance);

    let values = Vec::new();
    let mut diagnostics: Vec<export::Diagnostic> = mechanics.diagnostics.iter().map(export_mechanics_diagnostic).collect();

    // Nonlinear assembled-loop context binds only when the solve actually
    // exercised nonlinear supports (no false context, brief §3.3).
    let exercised = nonlinear_exercised(mechanics);
    let mut solver_build_ref = format!(
        "headless_preview_producer; solver_component={}@{}",
        solver_component_name(),
        solver_component_version()
    );
    if exercised {
        let context = nonlinear_assembled_loop_context();
        solver_build_ref.push_str(&format!(
            "; nonlinear_component={}@{}",
            context.component_name, context.component_version
        ));
        for assumption in &context.assumptions {
            diagnostics.push(nonlinear_context_diagnostic(
                NONLINEAR_LOOP_ASSUMPTION_CODE,
                export::DiagnosticClass::AssumptionWarning,
                export::DiagnosticSeverity::Info,
                context.component_name,
                &envelope_id,
                assumption,
            ));
        }
        for limitation in &context.limitations {
            diagnostics.push(nonlinear_context_diagnostic(
                NONLINEAR_LOOP_LIMITATION_CODE,
                export::DiagnosticClass::NonlinearWarning,
                export::DiagnosticSeverity::Warning,
                context.component_name,
                &envelope_id,
                limitation,
            ));
        }
    }

    // Reproducibility rides the existing checksum machinery only (no
    // invented facts, brief §3.6): the runner-request checksum and the
    // result-envelope checksum already carried by the runner result.
    let Some(request_checksum) = runner_result
        .checksums
        .iter()
        .find(|checksum| checksum.payload_ref.ref_type == "runner_request")
    else {
        return Err(production_failed(
            &envelope_id,
            "runner result carries no runner_request checksum reference".to_string(),
        ));
    };
    let Some(envelope_checksum) = runner_result.checksums.iter().find(|checksum| {
        checksum.payload_ref.ref_type == "result_envelope"
            && checksum.payload_ref.ref_id == envelope_id
    }) else {
        return Err(production_failed(
            &envelope_id,
            "runner result carries no result-envelope checksum reference".to_string(),
        ));
    };

    let envelope = export::ResultEnvelope {
        envelope_id: envelope_id.clone(),
        schema_version: RESULT_ENVELOPE_SCHEMA_VERSION.to_string(),
        model_ref: export_reference(&request.model_ref),
        run_ref: export::Reference::new("analysis_run", run_id.clone()),
        solver_name: solver_component_name().to_string(),
        solver_version: solver_component_version().to_string(),
        solver_build_ref,
        unit_system_ref: export_reference(&request.unit_system_ref),
        load_basis_refs: request
            .load_basis_refs
            .iter()
            .map(export_reference)
            .collect(),
        result_sets: vec![export::ResultSet {
            set_id: format!("result-set:{run_id}:mechanics"),
            set_type: export::ResultSetType::Mechanics.as_str().to_string(),
            basis_ref: run_basis,
            values,
        }],
        diagnostics,
        provenance: envelope_provenance,
        reproducibility: export::ReproducibilityRefs {
            model_hash: export_checksum(request_checksum),
            run_hashes: vec![export_checksum(envelope_checksum)],
            audit_manifest_ref: export_reference(&runner_result.audit_manifest_ref),
            deterministic_ordering: true,
        },
        analysis_status: runner_result
            .analysis_status
            .iter()
            .map(|status| export_analysis_status(*status))
            .collect(),
        rule_pack_refs: Vec::new(),
        professional_boundary: export::ProfessionalBoundary {
            human_review_required: runner_result.professional_boundary.human_review_required,
            software_makes_compliance_claim: runner_result
                .professional_boundary
                .software_makes_compliance_claim,
            software_makes_certification_claim: runner_result
                .professional_boundary
                .software_makes_certification_claim,
            software_makes_sealing_claim: runner_result
                .professional_boundary
                .software_makes_sealing_claim,
            software_makes_approval_claim: runner_result
                .professional_boundary
                .software_makes_approval_claim,
            software_makes_authentication_claim: runner_result
                .professional_boundary
                .software_makes_authentication_claim,
        },
    };

    Ok(export::result_export_document(&envelope))
}

#[cfg(test)]
mod tests {
 use super::*;
 use crate::{run_preview_model_value,run_preview_in_memory,JobStateKind,PrivacyContext,ProfessionalBoundary,RunnerOperation,TbdDecisions};
 use open_pipe_stress_product_physics::LinearStaticPreviewRequest;
    fn provenance() -> Provenance {
        Provenance {
            source_name: "invented producer-binding fixture".to_string(),
            source_location: "fixtures/product_preview".to_string(),
            source_license: "project invented".to_string(),
            contributor: "OpenPipeStress".to_string(),
            contributor_certification: "invented non-engineering example".to_string(),
            redistribution_status: RedistributionStatus::InventedNonEngineeringExample,
            review_status: "accepted".to_string(),
        }
    }

    fn request() -> RunnerRequest {
        RunnerRequest {
            request_id: "producer-binding-request-1".to_string(),
            operation: RunnerOperation::Solve,
            operation_ref: Reference::new("api_operation", "ops.solve.job"),
            project_ref: Reference::new("project", "invented-project"),
            model_ref: Reference::new("model", "invented-model"),
            unit_system_ref: Reference::new("unit_system", "invented-si"),
            load_basis_refs: vec![Reference::new("load_case", "LC1")],
            input_manifest_ref: Reference::new("audit_manifest", "manifest-1"),
            requested_outputs: vec![
                "result_envelope".to_string(),
                "audit_manifest".to_string(),
                "diagnostics".to_string(),
            ],
            privacy: PrivacyContext::local_first_public_metadata(),
            provenance: provenance(),
            professional_boundary: ProfessionalBoundary::project_default(),
            tbd_decisions: TbdDecisions::d33_local_cli_policy(),
        }
    }


 const PREVIEW_MODEL_FIXTURE:&str=include_str!("../../../../fixtures/product_preview/invented_preview_model.json");
 fn cases()->Value{serde_json::from_str(include_str!("../../../../fixtures/results/invented/result_export_v0_2.json")).unwrap()}
 // These tests exercise source/proof/derivative binding. Pressure is not an
 // oracle for those assertions. Keep the historical fixtures unchanged and
 // declare a zero-pressure current companion before invoking the public solver.
 fn unpressurized_binding_model(mut model: Value) -> Value {
   for case in model["load_cases"].as_array_mut().unwrap() {
     for load in case["primitive_loads"].as_array_mut().unwrap() {
       if load["category"] == "pressure" || load["dimension"] == "pressure" {
         load["magnitude"]["value"] = serde_json::json!(0.0);
       }
     }
   }
   model
 }
 #[test] fn qualified_actual_solved_documents_match_explicit_library_and_bind_model_identity(){
   for case in cases()["producer_cases"].as_array().unwrap(){
     let model = unpressurized_binding_model(case["model"].clone());
     let output=run_preview_model_value(request(),serde_json::json!({"model":model,"materials":[]})).unwrap();
     let mechanics=output.mechanics_envelope.as_ref().unwrap();
     assert_eq!(mechanics.status.mechanics,case["expected_status"].as_str().unwrap(),"{}",case["case_id"]);
     if mechanics.status.mechanics!="MECHANICS_SOLVED"{
       assert!(output.result_envelope_document.is_none());assert!(output.qualified_preview_evidence.is_none());
       assert!(build_result_export_document(&request(),&output.runner_result,mechanics).is_err());
       if case["case_id"]=="gap-chain-five-active"{assert!(mechanics.diagnostics.iter().any(|d|d.code=="NONLINEAR_SUPPORT_NONCONVERGENCE"));assert!(mechanics.results.is_empty());}
       continue;
     }
     if matches!(case["case_id"].as_str(), Some("gap-chain-two-active-control" | "gap-chain-five-inactive-control")) {
       assert_eq!(mechanics.numerical_quality.status, open_pipe_stress_product_physics::NumericalQualityStatus::Unresolved,
         "the solved gap controls must exercise unqualified export refusal");
     }
     if mechanics.numerical_quality.status != open_pipe_stress_product_physics::NumericalQualityStatus::ChecksPassed {
       assert!(!mechanics.results.is_empty(), "unqualified computation stays inspectable");
       assert!(output.result_envelope_document.is_none());
       assert!(output.qualified_preview_evidence.is_none());
       assert!(output.canonical_export_unavailability.as_deref().unwrap().contains("CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE"));
       continue;
     }
     let doc=output.result_envelope_document.as_ref().unwrap_or_else(||panic!("{} {:?}",case["case_id"],output.canonical_export_unavailability));
     assert_eq!(doc["schema_version"], "0.3.0");
     let raw = serde_json::to_value(mechanics).unwrap();
     if let Ok(dir)=std::env::var("HEADLESS_PRECISION_OUTPUT_DIR") {
         let dir=std::path::Path::new(&dir);std::fs::create_dir_all(dir).unwrap();
         let name=case["case_id"].as_str().unwrap();
         std::fs::write(dir.join(format!("{name}.raw.json")),serde_json::to_vec_pretty(&raw).unwrap()).unwrap();
         std::fs::write(dir.join(format!("{name}.document.json")),serde_json::to_vec_pretty(doc).unwrap()).unwrap();
     }
     let read: Value = serde_json::from_slice(&serde_json::to_vec(&raw).unwrap()).unwrap();
     assert_eq!(read, raw);
     for key in ["producer", "numerical_quality", "formulation_basis"] { assert_eq!(doc["result_envelope"][key], raw[key]); }
     for (original, parsed) in raw["results"].as_array().unwrap().iter().zip(read["results"].as_array().unwrap()) {
         assert_eq!(original["value"].as_f64().unwrap().to_bits(), parsed["value"].as_f64().unwrap().to_bits());
     }
     export::derivative::validate_document(doc, &read).unwrap();
     let proof=output.qualified_preview_evidence.as_ref().unwrap();
     let explicit=build_result_export_document_with_evidence(&request(),&output.runner_result,mechanics,proof).unwrap();assert_eq!(doc,&explicit);
     assert_eq!(doc["result_envelope"]["model_ref"]["ref_id"],case["model"]["project"]["id"]);
     assert!(doc["result_envelope"]["reproducibility"]["source_origin_bindings"][0]["request_alias_disclosure"].is_string());
     assert_eq!(doc["result_envelope"]["row_accounting"].as_array().unwrap().len(),mechanics.results.len());
     export::derivative::validate_document(doc,&serde_json::to_value(mechanics).unwrap()).unwrap();
     let mut altered_runner=output.runner_result.clone();altered_runner.checksums[0].algorithm="forged".into();assert!(build_result_export_document_with_evidence(&request(),&altered_runner,mechanics,proof).is_err());
     let mut changed=mechanics.clone();changed.results[0].value+=1.0;assert!(build_result_export_document_with_evidence(&request(),&output.runner_result,&changed,proof).is_err());
     let mut fake=proof.clone();fake.solve_payload["model"]["unknown_authored_metadata"]=serde_json::json!("same-id-substitution");assert!(build_result_export_document_with_evidence(&request(),&output.runner_result,mechanics,&fake).is_err());
     let serialized=serde_json::to_value(&output).unwrap();assert!(serialized.get("result_envelope_document").is_none());assert!(serialized.get("qualified_preview_evidence").is_none());assert!(serialized.get("canonical_export_unavailability").is_none());
   }
 }
 #[test]
 fn connected_physics_actual_solve_preserves_case_evidence_and_qualified_export() {
   let payload: Value = serde_json::from_str(include_str!("../../../product_physics/tests/fixtures/exact_pressure_connected_request.json")).unwrap();
   for (name, mode) in [("sparse", open_pipe_stress_product_physics::PreviewSolverMode::SparseInteractive), ("dense", open_pipe_stress_product_physics::PreviewSolverMode::DenseScrutiny)] {
     let output = crate::run_preview_model_value_with_mode(request(), payload.clone(), mode).unwrap();
     let mechanics = output.mechanics_envelope.as_ref().unwrap();
     assert_eq!(mechanics.status.mechanics, "MECHANICS_SOLVED");
     assert_eq!(mechanics.numerical_quality.status, open_pipe_stress_product_physics::NumericalQualityStatus::ChecksPassed);
     let raw = serde_json::to_value(mechanics).unwrap();
     assert_eq!(raw["producer"]["semantic_contract_id"], export::semantic_contract::PHYSICS_ID);
     export::semantic_contract::validate_physics_evidence(&raw).unwrap();
     let doc = output.result_envelope_document.as_ref().unwrap_or_else(|| panic!("{name}: {:?}", output.canonical_export_unavailability));
     export::derivative::validate_document(doc, &raw).unwrap();
     for key in ["producer", "numerical_quality", "formulation_basis", "contract_evidence"] {
       assert_eq!(doc["result_envelope"][key], raw[key]);
     }
     assert_eq!(doc["result_envelope"]["row_accounting"].as_array().unwrap().len(), mechanics.results.len());
     assert_eq!(raw["contract_evidence"]["exact_cases"].as_array().unwrap().len(), 2);
     assert_eq!(raw["contract_evidence"]["pressure"].as_array().unwrap().len(), 1);
     let validation = crate::validate_result_with_optional_envelope_payload(&output.runner_result, Some(doc));
     assert!(!validation.has_blocking_diagnostics(), "{name}: {:?}", validation.diagnostics);
     let proof = output.qualified_preview_evidence.as_ref().unwrap();
     let mut corrupted = mechanics.clone();
     corrupted.contract_evidence.as_mut().unwrap()["pressure"][0]["result_ids"] = serde_json::json!([]);
     assert!(build_result_export_document_with_evidence(&request(), &output.runner_result, &corrupted, proof).is_err());
     let mut unsupported = doc.clone();
     unsupported["result_envelope"]["producer"]["semantic_contract_id"] = serde_json::json!("openpipestress.result_semantics/0.3.0/source-blocks-1");
     assert!(crate::validate_result_with_optional_envelope_payload(&output.runner_result, Some(&unsupported)).has_blocking_diagnostics());
     if let Ok(dir) = std::env::var("HEADLESS_PHYSICS_OUTPUT_DIR") {
       let dir = std::path::Path::new(&dir); std::fs::create_dir_all(dir).unwrap();
       std::fs::write(dir.join(format!("{name}.raw.json")), serde_json::to_vec_pretty(&raw).unwrap()).unwrap();
       std::fs::write(dir.join(format!("{name}.document.json")), serde_json::to_vec_pretty(doc).unwrap()).unwrap();
     }
   }
 }
 #[test]
 fn sensitive_actual_source_cannot_mint_qualified_canonical_export() {
   let model: Value = serde_json::from_str(include_str!("../../../../fixtures/product_preview/numerical_sensitive_torsion_model.json")).unwrap();
   for mode in [open_pipe_stress_product_physics::PreviewSolverMode::DenseScrutiny, open_pipe_stress_product_physics::PreviewSolverMode::SparseInteractive] {
     let output = crate::run_preview_model_value_with_mode(request(), serde_json::json!({"model": model, "materials": []}), mode).unwrap();
     let raw = output.mechanics_envelope.as_ref().unwrap();
     assert_eq!(raw.status.mechanics, "MECHANICS_SOLVED");
     assert_eq!(raw.numerical_quality.status, open_pipe_stress_product_physics::NumericalQualityStatus::Sensitive);
     assert!(!raw.results.is_empty());
     assert!(output.result_envelope_document.is_none());
     assert!(output.qualified_preview_evidence.is_none());
     assert!(output.canonical_export_unavailability.as_deref().unwrap().contains("CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE"));
   }
 }
 #[test]
 fn public_nonzero_legacy_pressure_refuses_before_opaque_proof_or_export() {
   for name in ["straight-full", "curved-pressure-full"] {
     let fixtures = cases();
     let case = fixtures["producer_cases"].as_array().unwrap().iter().find(|case| case["case_id"] == name).unwrap();
     let original = case["model"].clone();
     let output = run_preview_model_value(request(), serde_json::json!({"model": original, "materials": []})).unwrap();
     let mechanics = output.mechanics_envelope.as_ref().unwrap();
     assert_eq!(mechanics.status.mechanics, "MODEL_INCOMPLETE");
     assert!(mechanics.diagnostics.iter().any(|d| d.code == "PRESSURE_MODEL_REAUTHOR_REQUIRED"), "{name}");
     assert!(mechanics.results.is_empty());
     assert!(output.result_envelope_document.is_none());
     assert!(output.qualified_preview_evidence.is_none());
     assert_eq!(output.canonical_export_unavailability.as_deref(), Some("SOURCE_NOT_SOLVED"));
     assert_eq!(case["model"], original);
   }
 }
 #[test]fn typed_legacy_caller_has_explicit_canonical_unavailability_without_raw_changes(){
   let model=unpressurized_binding_model(serde_json::from_str(PREVIEW_MODEL_FIXTURE).unwrap());let typed:LinearStaticPreviewRequest=serde_json::from_value(serde_json::json!({"model":model,"materials":[]})).unwrap();let output=run_preview_in_memory(request(),typed);assert_eq!(output.runner_result.job.state,JobStateKind::Completed);assert!(output.runner_result.diagnostics.is_empty());assert!(output.result_envelope_document.is_none());assert_eq!(output.canonical_export_unavailability.as_deref(),Some("EXACT_SOLVED_MODEL_EVIDENCE_UNAVAILABLE"));
 }
}
