//! DEL-08-04 result-envelope producer binding for the governed preview
//! analysis-run path (R14 W1 T1, `CB-2026-07-19-T1-PKG04-PRODUCER-BINDING-001`
//! v3).
//!
//! Bounded-coverage mapping: a `MechanicsEnvelope` result row is exported as a
//! DEL-08-04 `QuantityResult` only when its `(kind, unit)` pair resolves
//! through the explicit [`mapped_family_dimension`] table to an existing
//! `ResultFamily`/`DimensionId` whose semantics match the accepted D-01 unit
//! classification. Every row outside that table is disclosed per-row (result
//! id, kind, unit) in a dedicated NON-BLOCKING vocabulary-boundary diagnostic
//! carried in the envelope document itself — no silent drop, no invented
//! family/dimension identifier, no coercion (in particular, the `N*m`
//! free-DOF work residual is never exported as a moment). A STRUCTURAL
//! production or validation failure returns a blocking runner diagnostic for
//! the caller to append fail-closed; the disclosure path is not a failure and
//! never changes exit codes or serialized CLI output.
//!
//! The DEL-08-04 vocabulary-extension need (stiffness, energy/work,
//! count/state dimensions) is a reported follow-on for HELP_HUMAN, not
//! resolved here.

use open_pipe_stress_product_physics::{
    nonlinear_assembled_loop_context, solver_component_name, solver_component_version,
    Diagnostic as MechanicsDiagnostic, MechanicsEnvelope, ResultItem,
    ResultMetadata as MechanicsResultMetadata,
};
use open_pipe_stress_result_export as export;
use serde_json::Value;

use crate::{
    validate_result_with_optional_envelope_payload, AnalysisStatus, ChecksumRef, Diagnostic,
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

/// The enumerated deterministic `(kind, unit)` → (`ResultFamily`,
/// `DimensionId`) export table (brief §3.2 v3, §4 bounded-coverage shape).
///
/// Every entry is checkable against the accepted D-01 unit classification
/// (`docs/SPEC.md` §4): displacements/lengths in `mm` → length, rotations in
/// `rad` → angle, forces in `N` → force, moments in `N*m` → moment, stresses
/// in `MPa` → stress. Rows deliberately absent from this table (and therefore
/// disclosed, never coerced) include: user-stiffness review echoes
/// (`N/m`, `N*m/rad` — D-01 classifies these `linear_stiffness` /
/// `rotational_stiffness`, which DEL-08-04 does not carry), nonlinear
/// count/flag/state-code rows, residual-observation rows (including the
/// `N*m` free-DOF work residual, an energy quantity), solver-mode basis
/// rows, hanger travel-range rows, and every kind not listed here.
fn mapped_family_dimension(
    kind: &str,
    unit: &str,
) -> Option<(export::ResultFamily, export::DimensionId)> {
    use export::DimensionId;
    use export::ResultFamily;
    match (kind, unit) {
        // Straight-pipe / global displacement results (floor classes: export).
        ("displacement_magnitude", "mm")
        | ("global_nodal_displacement_x", "mm")
        | ("global_nodal_displacement_y", "mm")
        | ("global_nodal_displacement_z", "mm") => {
            Some((ResultFamily::Displacement, DimensionId::Length))
        }
        ("global_nodal_rotation_x", "rad")
        | ("global_nodal_rotation_y", "rad")
        | ("global_nodal_rotation_z", "rad") => Some((ResultFamily::Rotation, DimensionId::Angle)),
        // Support reactions.
        ("reaction_resultant", "N") => Some((ResultFamily::Reaction, DimensionId::Force)),
        // Straight-pipe element local force results (metadata-gated family).
        ("element_local_axial_force", "N")
        | ("element_local_shear_force_y", "N")
        | ("element_local_shear_force_z", "N") => Some((ResultFamily::Force, DimensionId::Force)),
        // Straight-pipe element local moment results (metadata-gated family).
        ("element_local_bending_moment_y", "N*m")
        | ("element_local_bending_moment_z", "N*m")
        | ("element_local_torsional_moment", "N*m") => {
            Some((ResultFamily::Moment, DimensionId::Moment))
        }
        // Stress results (element-local recovery, open-formula summary,
        // pressure hoop, and the user-multiplier stress review value — all
        // stress magnitudes in MPa).
        ("element_local_axial_normal_stress", "MPa")
        | ("element_local_bending_normal_stress_y", "MPa")
        | ("element_local_bending_normal_stress_z", "MPa")
        | ("element_local_torsional_shear_stress", "MPa")
        | ("open_formula_stress_summary", "MPa")
        | ("pipe_section_pressure_hoop_stress", "MPa")
        | ("component_user_stress_multiplier_review", "MPa") => {
            Some((ResultFamily::Stress, DimensionId::Stress))
        }
        // Force-valued user-input review evidence (metadata-gated family).
        ("expansion_joint_pressure_thrust_load_review", "N")
        | ("spring_hanger_user_input_review", "N")
        | ("constant_effort_user_input_review", "N") => {
            Some((ResultFamily::Force, DimensionId::Force))
        }
        // Nonlinear final-state physical values (displacements/reactions).
        ("nonlinear_support_final_displacement", "mm") => {
            Some((ResultFamily::Displacement, DimensionId::Length))
        }
        ("nonlinear_support_final_reaction", "N")
        | ("nonlinear_support_friction_normal_reaction_input", "N")
        | ("nonlinear_support_friction_normal_reaction_derived", "N") => {
            Some((ResultFamily::Reaction, DimensionId::Force))
        }
        _ => None,
    }
}

/// Whether the exported family requires complete five-field metadata under
/// the DEL-08-04 validator gate.
fn family_requires_metadata(family: export::ResultFamily) -> bool {
    matches!(
        family,
        export::ResultFamily::Force
            | export::ResultFamily::Moment
            | export::ResultFamily::SectionProperty
    )
}

fn metadata_gate_complete(metadata: Option<&MechanicsResultMetadata>) -> bool {
    let Some(metadata) = metadata else {
        return false;
    };
    let non_tbd =
        |value: &str| !value.trim().is_empty() && !value.trim().eq_ignore_ascii_case("TBD");
    non_tbd(&metadata.component)
        && non_tbd(&metadata.coordinate_system)
        && non_tbd(&metadata.location)
        && non_tbd(&metadata.basis)
        && !metadata.sign_convention.trim().is_empty()
}

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

fn export_metadata(metadata: &MechanicsResultMetadata) -> export::ResultMetadata {
    export::ResultMetadata {
        component: metadata.component.clone(),
        coordinate_system: metadata.coordinate_system.clone(),
        location: metadata.location.clone(),
        basis: metadata.basis.clone(),
        sign_convention: metadata.sign_convention.clone(),
    }
}

/// Producer-origin provenance for disclosure metadata (diagnostic metadata
/// only, following the existing validator invented-provenance precedent).
fn producer_provenance() -> export::Provenance {
    export::Provenance {
        source_name: "OpenPipeStress headless result-envelope producer".to_string(),
        source_location: "core/runner/headless/src/result_envelope_binding.rs".to_string(),
        source_license: "project".to_string(),
        contributor: "OpenPipeStress".to_string(),
        contributor_certification: "vocabulary-boundary disclosure metadata only".to_string(),
        redistribution_status: export::RedistributionStatus::InventedNonEngineeringExample,
        review_status: "accepted".to_string(),
    }
}

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
fn disclosure_diagnostic(row: &ResultItem) -> export::Diagnostic {
    export::Diagnostic {
        code: VOCABULARY_BOUNDARY_DISCLOSURE_CODE.to_string(),
        class: export::DiagnosticClass::UnitWarning,
        severity: export::DiagnosticSeverity::Info,
        source: export::Reference::new("result_envelope_producer", "core/runner/headless"),
        affected_object: export::Reference::new("preview_result", row.id.clone()),
        message: format!(
            "result row not exported: id={} kind={} unit={} is outside the enumerated \
             DEL-08-04 export mapping table; disclosed without coercion. The DEL-08-04 \
             vocabulary-extension need is a reported follow-on for HELP_HUMAN.",
            row.id, row.kind, row.unit
        ),
        remediation: "Consume this row from the carried mechanics envelope; do not infer a \
                      result family or dimension for it from this export."
            .to_string(),
        provenance: producer_provenance(),
    }
}

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
pub fn build_result_export_document(
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

    // Bounded-coverage export with per-row disclosure (no silent drop).
    let mut values = Vec::new();
    let mut diagnostics: Vec<export::Diagnostic> = mechanics
        .diagnostics
        .iter()
        .map(export_mechanics_diagnostic)
        .collect();
    for row in &mechanics.results {
        let mapped = mapped_family_dimension(&row.kind, &row.unit).filter(|(family, _)| {
            !family_requires_metadata(*family) || metadata_gate_complete(row.metadata.as_ref())
        });
        let Some((family, dimension)) = mapped else {
            diagnostics.push(disclosure_diagnostic(row));
            continue;
        };
        values.push(export::QuantityResult {
            result_id: row.id.clone(),
            family,
            object_ref: export::Reference::new("preview_entity", row.entity_ref.clone()),
            basis_ref: row
                .basis_ref
                .as_ref()
                .map(|basis| export::Reference::new(basis.ref_type.clone(), basis.ref_id.clone()))
                .unwrap_or_else(|| run_basis.clone()),
            station_ref: None,
            magnitude: row.value,
            unit: row.unit.clone(),
            dimension,
            metadata: row.metadata.as_ref().map(export_metadata),
            diagnostics: Vec::new(),
            trace_chain: Vec::new(),
            provenance: envelope_provenance.clone(),
        });
    }

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

    // Structural validation gate 1: the typed DEL-08-04 validator.
    let typed_validation = export::validate_result_envelope(&envelope);
    if typed_validation.has_blocking_diagnostics() {
        let codes: Vec<&str> = typed_validation
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.severity == export::DiagnosticSeverity::Blocking)
            .map(|diagnostic| diagnostic.code.as_str())
            .collect();
        return Err(production_failed(
            &envelope_id,
            format!("typed envelope validation blocked: {}", codes.join(", ")),
        ));
    }

    // Serialize into the exact schema-first wrapper shape.
    let document = export::result_export_document(&envelope);

    // Structural validation gate 2: the runner payload contract (envelope-id
    // match, HUMAN_REVIEW_REQUIRED, result sets, ordering, checksum).
    let payload_validation =
        validate_result_with_optional_envelope_payload(runner_result, Some(&document));
    if payload_validation.has_blocking_diagnostics() {
        let codes: Vec<&str> = payload_validation
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.severity == crate::DiagnosticSeverity::Blocking)
            .map(|diagnostic| diagnostic.code.as_str())
            .collect();
        return Err(production_failed(
            &envelope_id,
            format!("payload contract validation blocked: {}", codes.join(", ")),
        ));
    }

    Ok(document)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        run_preview_in_memory, validate_result, JobStateKind, PrivacyContext, ProfessionalBoundary,
        RunnerOperation, TbdDecisions,
    };
    use open_pipe_stress_product_physics::LinearStaticPreviewRequest;

    const PREVIEW_MODEL_FIXTURE: &str =
        include_str!("../../../../fixtures/product_preview/invented_preview_model.json");

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

    fn nonlinear_bearing_preview_request() -> LinearStaticPreviewRequest {
        LinearStaticPreviewRequest {
            model: serde_json::from_str(PREVIEW_MODEL_FIXTURE)
                .expect("invented preview model fixture should parse"),
            materials: Vec::new(),
        }
    }

    /// The same invented model with every nonlinear-bearing support removed,
    /// so the solve never enters the active-set loop (linear-only fixture).
    fn linear_only_preview_request() -> LinearStaticPreviewRequest {
        let mut model: Value = serde_json::from_str(PREVIEW_MODEL_FIXTURE)
            .expect("invented preview model fixture should parse");
        let supports = model["supports"]
            .as_array_mut()
            .expect("fixture carries a supports array");
        supports.retain(|support| support.get("nonlinear").is_none());
        LinearStaticPreviewRequest {
            model: serde_json::from_value(model).expect("linear-only model still deserializes"),
            materials: Vec::new(),
        }
    }

    fn envelope_object(document: &Value) -> &serde_json::Map<String, Value> {
        document["result_envelope"]
            .as_object()
            .expect("document carries a result_envelope object")
    }

    fn diagnostics_with_code<'a>(document: &'a Value, code: &str) -> Vec<&'a Value> {
        envelope_object(document)["diagnostics"]
            .as_array()
            .expect("envelope carries diagnostics")
            .iter()
            .filter(|diagnostic| diagnostic["code"].as_str() == Some(code))
            .collect()
    }

    #[test]
    fn mapping_table_is_bounded_and_never_coerces() {
        use export::{DimensionId, ResultFamily};
        assert_eq!(
            mapped_family_dimension("element_local_axial_force", "N"),
            Some((ResultFamily::Force, DimensionId::Force))
        );
        assert_eq!(
            mapped_family_dimension("element_local_bending_moment_z", "N*m"),
            Some((ResultFamily::Moment, DimensionId::Moment))
        );
        assert_eq!(
            mapped_family_dimension("global_nodal_displacement_y", "mm"),
            Some((ResultFamily::Displacement, DimensionId::Length))
        );
        assert_eq!(
            mapped_family_dimension("reaction_resultant", "N"),
            Some((ResultFamily::Reaction, DimensionId::Force))
        );
        // Out-of-vocabulary classes are disclosed, never coerced.
        assert_eq!(
            mapped_family_dimension("component_user_stiffness_macro_element_review", "N/m"),
            None
        );
        assert_eq!(
            mapped_family_dimension("component_user_stiffness_macro_element_review", "N*m/rad"),
            None
        );
        assert_eq!(
            mapped_family_dimension("spring_hanger_user_input_review", "N/m"),
            None
        );
        assert_eq!(
            mapped_family_dimension("nonlinear_support_active_set_iteration_count", "count"),
            None
        );
        assert_eq!(
            mapped_family_dimension("nonlinear_support_active_set_converged_flag", "boolean"),
            None
        );
        assert_eq!(
            mapped_family_dimension("nonlinear_support_active_set_state_code", "state_code"),
            None
        );
        assert_eq!(
            mapped_family_dimension("linear_solver_mode_basis", "mode_code"),
            None
        );
        // The N*m free-DOF work residual is an energy quantity: never a moment.
        assert_eq!(
            mapped_family_dimension("nonlinear_support_free_dof_work_residual", "N*m"),
            None
        );
        assert_eq!(
            mapped_family_dimension("nonlinear_support_observed_max_translation_delta", "mm"),
            None
        );
        // The table is unit-exact: an unexpected unit never inherits a mapping.
        assert_eq!(
            mapped_family_dimension("element_local_axial_force", "kN"),
            None
        );
    }

    #[test]
    fn nonlinear_bearing_solve_attaches_validated_envelope_with_context() {
        let output = run_preview_in_memory(request(), nonlinear_bearing_preview_request());
        let mechanics = output.mechanics_envelope.as_ref().expect("solve envelope");
        assert_eq!(mechanics.status.mechanics, "MECHANICS_SOLVED");
        // Clean success path: document attached, no runner diagnostics, so
        // exit codes and serialized CLI output are unchanged.
        assert!(output.runner_result.diagnostics.is_empty());
        assert_eq!(output.runner_result.job.state, JobStateKind::Completed);
        let document = output
            .result_envelope_document
            .as_ref()
            .expect("completed solve attaches the envelope document");

        // Exact wrapper shape and envelope-id binding.
        assert_eq!(document["deliverable_id"].as_str(), Some("DEL-08-04"));
        let envelope = envelope_object(document);
        assert_eq!(
            envelope["envelope_id"].as_str(),
            Some(
                output
                    .runner_result
                    .result_envelope_ref
                    .envelope_ref
                    .ref_id
                    .as_str()
            )
        );

        // Solver-version block: product-physics identity plus the
        // nonlinear-integration component identity, crate-constant-derived.
        let solver_version = &envelope["solver_version"];
        assert_eq!(
            solver_version["solver_name"].as_str(),
            Some(solver_component_name())
        );
        assert_eq!(
            solver_version["solver_version"].as_str(),
            Some(solver_component_version())
        );
        let context = nonlinear_assembled_loop_context();
        let build_ref = solver_version["solver_build_ref"]
            .as_str()
            .expect("build ref");
        assert!(build_ref.contains(&format!(
            "nonlinear_component={}@{}",
            context.component_name, context.component_version
        )));

        // Assumption/limitation context rows ride as non-blocking
        // diagnostics with provenance naming the source crate.
        let assumptions = diagnostics_with_code(document, NONLINEAR_LOOP_ASSUMPTION_CODE);
        let limitations = diagnostics_with_code(document, NONLINEAR_LOOP_LIMITATION_CODE);
        assert_eq!(assumptions.len(), context.assumptions.len());
        assert_eq!(limitations.len(), context.limitations.len());
        for diagnostic in assumptions.iter().chain(limitations.iter()) {
            assert_ne!(diagnostic["severity"].as_str(), Some("blocking"));
            assert_eq!(
                diagnostic["provenance"]["source_name"].as_str(),
                Some(context.component_name)
            );
        }
        assert_eq!(
            assumptions[0]["message"].as_str(),
            Some(context.assumptions[0].as_str())
        );

        // Bounded coverage with per-row disclosure: every mechanics result
        // row is either exported or disclosed — never silently dropped.
        let values = envelope["result_sets"][0]["values"]
            .as_array()
            .expect("mechanics result set carries values");
        let disclosures = diagnostics_with_code(document, VOCABULARY_BOUNDARY_DISCLOSURE_CODE);
        assert_eq!(values.len() + disclosures.len(), mechanics.results.len());
        assert!(!values.is_empty());
        assert!(!disclosures.is_empty());
        for disclosure in &disclosures {
            assert_eq!(disclosure["severity"].as_str(), Some("info"));
        }

        // Straight-pipe element rows export with their metadata; the
        // stiffness review echo is disclosed by id/kind/unit, not exported.
        assert!(values.iter().any(|value| {
            value["result_id"].as_str().is_some_and(|id| {
                id.contains("element_local_axial_force")
                    || id.contains(":axial-force")
                    || id.contains("axial")
            }) && value["family"].as_str() == Some("force")
                && value["metadata"].is_object()
        }));
        let stiffness_row_id = "result:component-stiffness:component-C-150:axial";
        assert!(!values
            .iter()
            .any(|value| value["result_id"].as_str() == Some(stiffness_row_id)));
        assert!(disclosures.iter().any(|disclosure| {
            disclosure["affected_object"]["ref_id"].as_str() == Some(stiffness_row_id)
                && disclosure["message"]
                    .as_str()
                    .is_some_and(|message| message.contains("N/m"))
        }));
        // Nonlinear evidence scalars are disclosed, not exported.
        assert!(disclosures.iter().any(|disclosure| {
            disclosure["message"].as_str().is_some_and(|message| {
                message.contains("nonlinear_support_active_set_iteration_count")
            })
        }));

        // Both validators are clean on the attached document.
        let payload_validation =
            validate_result_with_optional_envelope_payload(&output.runner_result, Some(document));
        assert!(
            !payload_validation.has_blocking_diagnostics(),
            "{payload_validation:?}"
        );
        let result_validation = validate_result(&output.runner_result);
        assert!(
            !result_validation.has_blocking_diagnostics(),
            "{result_validation:?}"
        );
    }

    #[test]
    fn linear_only_solve_attaches_envelope_without_nonlinear_context() {
        let output = run_preview_in_memory(request(), linear_only_preview_request());
        let mechanics = output.mechanics_envelope.as_ref().expect("solve envelope");
        assert_eq!(mechanics.status.mechanics, "MECHANICS_SOLVED");
        assert!(output.runner_result.diagnostics.is_empty());
        let document = output
            .result_envelope_document
            .as_ref()
            .expect("linear-only solve attaches the envelope document");

        // No false nonlinear context: no assumption/limitation rows and no
        // nonlinear component identity in the build reference.
        assert!(diagnostics_with_code(document, NONLINEAR_LOOP_ASSUMPTION_CODE).is_empty());
        assert!(diagnostics_with_code(document, NONLINEAR_LOOP_LIMITATION_CODE).is_empty());
        let build_ref = envelope_object(document)["solver_version"]["solver_build_ref"]
            .as_str()
            .expect("build ref");
        assert!(!build_ref.contains("nonlinear_component="));

        // Disclosure still applies to out-of-vocabulary linear rows (for
        // example the solver-mode basis row) without any exit-code effect.
        let disclosures = diagnostics_with_code(document, VOCABULARY_BOUNDARY_DISCLOSURE_CODE);
        assert!(disclosures.iter().any(|disclosure| {
            disclosure["message"]
                .as_str()
                .is_some_and(|message| message.contains("linear_solver_mode_basis"))
        }));
        let values = envelope_object(document)["result_sets"][0]["values"]
            .as_array()
            .expect("values");
        assert_eq!(values.len() + disclosures.len(), mechanics.results.len());
    }

    #[test]
    fn preview_runner_output_serialization_excludes_envelope_document() {
        let output = run_preview_in_memory(request(), nonlinear_bearing_preview_request());
        assert!(output.result_envelope_document.is_some());
        let serialized = serde_json::to_value(&output).expect("output serializes");
        let object = serialized
            .as_object()
            .expect("output serializes to an object");
        // Library-only surface: the document never reaches any serialized
        // output (headless_preview_runner witness stdout, CliOutput).
        assert!(!object.contains_key("result_envelope_document"));
        assert!(object.contains_key("runner_result"));
        assert!(object.contains_key("mechanics_envelope"));
    }

    #[test]
    fn structural_failure_appends_blocking_runner_diagnostic() {
        let request = request();
        let output = run_preview_in_memory(request.clone(), nonlinear_bearing_preview_request());
        let mechanics = output.mechanics_envelope.expect("solve envelope");
        let mut doctored = output.runner_result.clone();
        doctored.result_envelope_ref.envelope_ref.ref_id =
            "result-envelope:wrong-reference".to_string();

        let attached = crate::attach_result_envelope_document(&request, &mut doctored, &mechanics);

        assert!(attached.is_none());
        assert!(doctored.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == RESULT_ENVELOPE_PRODUCTION_FAILED_CODE
                && diagnostic.severity == crate::DiagnosticSeverity::Blocking
        }));
    }

    #[test]
    fn producer_rejects_missing_envelope_checksum_structurally() {
        let request = request();
        let output = run_preview_in_memory(request.clone(), nonlinear_bearing_preview_request());
        let mechanics = output.mechanics_envelope.expect("solve envelope");
        let mut doctored = output.runner_result.clone();
        doctored
            .checksums
            .retain(|checksum| checksum.payload_ref.ref_type != "result_envelope");

        let produced = build_result_export_document(&request, &doctored, &mechanics);

        let error = produced.expect_err("missing envelope checksum is structural");
        assert_eq!(error.code, RESULT_ENVELOPE_PRODUCTION_FAILED_CODE);
        assert_eq!(error.severity, crate::DiagnosticSeverity::Blocking);
    }

    #[test]
    fn model_incomplete_solve_attaches_nothing_and_adds_no_diagnostics() {
        let mut model: Value = serde_json::from_str(PREVIEW_MODEL_FIXTURE)
            .expect("invented preview model fixture should parse");
        model["schema_version"] = Value::String(String::new());
        let preview_request = LinearStaticPreviewRequest {
            model: serde_json::from_value(model).expect("model still deserializes"),
            materials: Vec::new(),
        };

        let output = run_preview_in_memory(request(), preview_request);
        let mechanics = output
            .mechanics_envelope
            .as_ref()
            .expect("blocked envelope");
        assert_ne!(mechanics.status.mechanics, "MECHANICS_SOLVED");
        // Not a completed solve product: no envelope document, and the
        // pre-existing not-clean signaling is preserved without any new
        // runner diagnostic (byte-stable behavior for incomplete solves).
        assert!(output.result_envelope_document.is_none());
        assert!(output.runner_result.diagnostics.is_empty());
        let validation = validate_result(&output.runner_result);
        assert!(validation.has_blocking_diagnostics());
    }
}
