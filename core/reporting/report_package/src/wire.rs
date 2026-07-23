use crate as package;
use open_pipe_stress_audit_manifest as audit;
use open_pipe_stress_report_renderer::RenderableReportInput;
use open_pipe_stress_result_export as result_export;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct ReportPackageRequest {
    pub package_id: String,
    pub export_profile_id: String,
    pub source_model_ref: ReferenceDto,
    pub source_basis_refs: Vec<ReferenceDto>,
    pub report: Value,
    pub audit_manifest: AuditManifestDto,
    pub result_envelopes: Vec<ResultEnvelopeDto>,
    pub state_comparison_handoff_records: Vec<Value>,
    pub rule_check_aggregate: Option<String>,
    pub solve_rule_check_status: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReferenceDto {
    pub ref_type: String,
    pub ref_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HashRecordDto {
    pub algorithm: String,
    pub canonicalization: String,
    pub payload_kind: String,
    pub payload_ref: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct SolverVersionDto {
    pub solver_name: String,
    pub solver_version: String,
    pub solver_build_ref: String,
}

#[derive(Debug, Deserialize)]
pub struct ProfessionalBoundaryDto {
    pub human_review_required: bool,
    pub software_makes_compliance_claim: bool,
    pub software_makes_certification_claim: bool,
    pub software_makes_sealing_claim: bool,
    pub software_makes_approval_claim: bool,
    #[serde(default)]
    pub software_makes_authentication_claim: bool,
}

#[derive(Debug, Deserialize)]
pub struct RulePackAuditRefDto {
    pub rule_pack_id: String,
    pub rule_pack_version: String,
    pub source_notice: String,
    pub redistribution_status: String,
    pub checksum: Option<HashRecordDto>,
    pub private_payload_redacted: bool,
}

#[derive(Debug, Deserialize)]
pub struct AssetManifestEntryDto {
    pub asset_id: String,
    pub payload_kind: String,
    pub privacy_class: String,
    pub hash: Option<HashRecordDto>,
    pub provenance: String,
}

#[derive(Debug, Deserialize)]
pub struct AuditManifestDto {
    pub manifest_id: String,
    pub model_hash: Option<HashRecordDto>,
    pub input_manifest_hash: Option<HashRecordDto>,
    pub solver_version: SolverVersionDto,
    pub unit_system_ref: String,
    pub rule_pack_refs: Vec<RulePackAuditRefDto>,
    pub assets: Vec<AssetManifestEntryDto>,
    pub professional_boundary: ProfessionalBoundaryDto,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProvenanceDto {
    pub source_name: String,
    pub source_location: String,
    pub source_license: String,
    pub contributor: String,
    pub contributor_certification: String,
    pub redistribution_status: String,
    pub review_status: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChecksumDto {
    pub algorithm: String,
    pub canonicalization: String,
    pub payload_ref: ReferenceDto,
    pub value: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DiagnosticDto {
    pub code: String,
    pub class: String,
    pub severity: String,
    pub source: ReferenceDto,
    pub affected_object: ReferenceDto,
    pub message: String,
    pub remediation: String,
    pub provenance: ProvenanceDto,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResultTraceLinkDto {
    pub trace_id: String,
    pub trace_type: String,
    pub source_ref: ReferenceDto,
    pub target_ref: ReferenceDto,
    pub provenance: ProvenanceDto,
    pub diagnostics: Vec<DiagnosticDto>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResultMetadataDto {
    pub component: String,
    pub coordinate_system: String,
    pub location: String,
    pub basis: String,
    pub sign_convention: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct QuantityResultDto {
    pub result_id: String,
    pub family: String,
    pub object_ref: ReferenceDto,
    pub basis_ref: ReferenceDto,
    pub station_ref: Option<ReferenceDto>,
    pub magnitude: f64,
    pub unit: String,
    pub dimension: String,
    pub metadata: Option<ResultMetadataDto>,
    pub diagnostics: Vec<DiagnosticDto>,
    pub trace_chain: Vec<ResultTraceLinkDto>,
    pub provenance: ProvenanceDto,
}

#[derive(Debug, Deserialize)]
pub struct ResultSetDto {
    pub set_id: String,
    pub set_type: String,
    pub basis_ref: ReferenceDto,
    pub values: Vec<QuantityResultDto>,
}

#[derive(Debug, Deserialize)]
pub struct ReproducibilityDto {
    pub model_hash: ChecksumDto,
    pub run_hashes: Vec<ChecksumDto>,
    pub audit_manifest_ref: ReferenceDto,
    pub deterministic_ordering: bool,
}

#[derive(Debug, Deserialize)]
pub struct RulePackExportRefDto {
    pub rule_pack_id: String,
    pub version: String,
    pub checksum: ChecksumDto,
    pub source_notice: String,
    pub redistribution_status: String,
    pub completeness_status: String,
    pub private_payload_redacted: bool,
}

#[derive(Debug, Deserialize)]
pub struct ResultEnvelopeDto {
    pub envelope_id: String,
    pub schema_version: String,
    pub model_ref: ReferenceDto,
    pub run_ref: ReferenceDto,
    pub solver_name: String,
    pub solver_version: String,
    pub solver_build_ref: String,
    pub unit_system_ref: ReferenceDto,
    pub load_basis_refs: Vec<ReferenceDto>,
    pub result_sets: Vec<ResultSetDto>,
    pub diagnostics: Vec<DiagnosticDto>,
    pub provenance: ProvenanceDto,
    pub reproducibility: ReproducibilityDto,
    pub analysis_status: Vec<String>,
    pub rule_pack_refs: Vec<RulePackExportRefDto>,
    pub professional_boundary: ProfessionalBoundaryDto,
}

fn reject_empty(label: &str, value: &str) -> Result<(), String> {
    if value.trim().is_empty() || value.trim().eq_ignore_ascii_case("TBD") {
        Err(format!(
            "REPORT-PACKAGE-WIRE-INCOMPLETE: {label} is missing or TBD"
        ))
    } else {
        Ok(())
    }
}

fn require_sha256_hex(label: &str, value: &str) -> Result<(), String> {
    if value.len() == 64
        && value
            .as_bytes()
            .iter()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
    {
        Ok(())
    } else {
        Err(format!(
            "REPORT-PACKAGE-SHA256-INVALID: {label} must be bare lowercase 64-hex"
        ))
    }
}

fn reference(value: ReferenceDto) -> Result<result_export::Reference, String> {
    reject_empty("reference.ref_type", &value.ref_type)?;
    reject_empty("reference.ref_id", &value.ref_id)?;
    Ok(result_export::Reference::new(value.ref_type, value.ref_id))
}

fn package_reference(value: ReferenceDto) -> Result<package::PackageRef, String> {
    reject_empty("package reference ref_type", &value.ref_type)?;
    reject_empty("package reference ref_id", &value.ref_id)?;
    Ok(package::PackageRef::new(value.ref_type, value.ref_id))
}

fn audit_payload_kind(value: &str) -> Result<audit::PayloadKind, String> {
    match value {
        "model_json" => Ok(audit::PayloadKind::ModelJson),
        "input_manifest_json" => Ok(audit::PayloadKind::InputManifestJson),
        "rule_pack_reference" => Ok(audit::PayloadKind::RulePackReference),
        "binary_asset" => Ok(audit::PayloadKind::BinaryAsset),
        "external_artifact" => Ok(audit::PayloadKind::ExternalArtifact),
        _ => Err(format!(
            "REPORT-PACKAGE-WIRE-ENUM: unsupported payload_kind {value}"
        )),
    }
}

fn audit_canonicalization(value: &str) -> Result<audit::Canonicalization, String> {
    match value {
        "project_local_deterministic_json" => {
            Ok(audit::Canonicalization::ProjectLocalDeterministicJson)
        }
        "none" => Ok(audit::Canonicalization::None),
        _ => Err(format!(
            "REPORT-PACKAGE-WIRE-ENUM: unsupported canonicalization {value}"
        )),
    }
}

fn audit_hash(value: HashRecordDto) -> Result<audit::HashRecord, String> {
    if value.algorithm != "sha256" {
        return Err(format!(
            "REPORT-PACKAGE-WIRE-ENUM: unsupported hash algorithm {}",
            value.algorithm
        ));
    }
    reject_empty("hash.payload_ref", &value.payload_ref)?;
    require_sha256_hex("audit hash.value", &value.value)?;
    Ok(audit::HashRecord {
        algorithm: audit::HashAlgorithm::Sha256,
        canonicalization: audit_canonicalization(&value.canonicalization)?,
        payload_kind: audit_payload_kind(&value.payload_kind)?,
        payload_ref: value.payload_ref,
        value: value.value,
    })
}

fn audit_redistribution(value: &str) -> Result<audit::RedistributionStatus, String> {
    match value {
        "public_permissive" => Ok(audit::RedistributionStatus::PublicPermissive),
        "private_only" => Ok(audit::RedistributionStatus::PrivateOnly),
        "unknown" => Ok(audit::RedistributionStatus::Unknown),
        "protected_suspected" => Ok(audit::RedistributionStatus::ProtectedSuspected),
        "invented_non_engineering_example" => {
            Ok(audit::RedistributionStatus::InventedNonEngineeringExample)
        }
        "TBD" => Ok(audit::RedistributionStatus::Tbd),
        _ => Err(format!(
            "REPORT-PACKAGE-WIRE-ENUM: unsupported redistribution_status {value}"
        )),
    }
}

fn audit_privacy(value: &str) -> Result<audit::PrivacyClass, String> {
    match value {
        "public_metadata" => Ok(audit::PrivacyClass::PublicMetadata),
        "private_project_data" => Ok(audit::PrivacyClass::PrivateProjectData),
        "private_rule_pack_data" => Ok(audit::PrivacyClass::PrivateRulePackData),
        "redacted" => Ok(audit::PrivacyClass::Redacted),
        "TBD" => Ok(audit::PrivacyClass::Tbd),
        _ => Err(format!(
            "REPORT-PACKAGE-WIRE-ENUM: unsupported privacy_class {value}"
        )),
    }
}

fn audit_manifest(
    value: AuditManifestDto,
    linked_solver: &LinkedSolverIdentity<'_>,
) -> Result<audit::AuditManifest, String> {
    reject_empty("audit_manifest.manifest_id", &value.manifest_id)?;
    if value.solver_version.solver_name != linked_solver.solver_name
        || value.solver_version.solver_version != linked_solver.solver_version
        || value.solver_version.solver_build_ref != linked_solver.solver_build_ref
    {
        return Err("REPORT-PACKAGE-SOLVER-IDENTITY-MISMATCH: request identity does not match the linked product-physics component".to_string());
    }
    Ok(audit::AuditManifest {
        manifest_id: value.manifest_id,
        model_hash: value.model_hash.map(audit_hash).transpose()?,
        input_manifest_hash: value.input_manifest_hash.map(audit_hash).transpose()?,
        solver_version: audit::SolverVersionStamp {
            solver_name: linked_solver.solver_name.to_string(),
            solver_version: linked_solver.solver_version.to_string(),
            solver_build_ref: linked_solver.solver_build_ref.to_string(),
        },
        unit_system_ref: value.unit_system_ref,
        rule_pack_refs: value
            .rule_pack_refs
            .into_iter()
            .map(|item| {
                Ok(audit::RulePackAuditRef {
                    rule_pack_id: item.rule_pack_id,
                    rule_pack_version: item.rule_pack_version,
                    source_notice: item.source_notice,
                    redistribution_status: audit_redistribution(&item.redistribution_status)?,
                    checksum: item.checksum.map(audit_hash).transpose()?,
                    private_payload_redacted: item.private_payload_redacted,
                })
            })
            .collect::<Result<Vec<_>, String>>()?,
        assets: value
            .assets
            .into_iter()
            .map(|item| {
                Ok(audit::AssetManifestEntry {
                    asset_id: item.asset_id,
                    payload_kind: audit_payload_kind(&item.payload_kind)?,
                    privacy_class: audit_privacy(&item.privacy_class)?,
                    hash: item.hash.map(audit_hash).transpose()?,
                    provenance: item.provenance,
                })
            })
            .collect::<Result<Vec<_>, String>>()?,
        professional_boundary: audit::ProfessionalBoundary {
            software_makes_compliance_claim: value
                .professional_boundary
                .software_makes_compliance_claim,
            software_makes_certification_claim: value
                .professional_boundary
                .software_makes_certification_claim,
            software_makes_sealing_claim: value.professional_boundary.software_makes_sealing_claim,
            software_makes_approval_claim: value
                .professional_boundary
                .software_makes_approval_claim,
            human_review_required: value.professional_boundary.human_review_required,
        },
    })
}

fn result_redistribution(value: &str) -> Result<result_export::RedistributionStatus, String> {
    match value {
        "public_permissive" => Ok(result_export::RedistributionStatus::PublicPermissive),
        "private_only" => Ok(result_export::RedistributionStatus::PrivateOnly),
        "unknown" => Ok(result_export::RedistributionStatus::Unknown),
        "protected_suspected" => Ok(result_export::RedistributionStatus::ProtectedSuspected),
        "invented_non_engineering_example" => {
            Ok(result_export::RedistributionStatus::InventedNonEngineeringExample)
        }
        "TBD" => Ok(result_export::RedistributionStatus::Tbd),
        _ => Err(format!(
            "REPORT-PACKAGE-WIRE-ENUM: unsupported result redistribution status {value}"
        )),
    }
}

fn provenance(value: ProvenanceDto) -> Result<result_export::Provenance, String> {
    Ok(result_export::Provenance {
        source_name: value.source_name,
        source_location: value.source_location,
        source_license: value.source_license,
        contributor: value.contributor,
        contributor_certification: value.contributor_certification,
        redistribution_status: result_redistribution(&value.redistribution_status)?,
        review_status: value.review_status,
    })
}

fn checksum(value: ChecksumDto) -> Result<result_export::ChecksumRef, String> {
    reject_empty("checksum.value", &value.value)?;
    if value.algorithm == "sha256" {
        require_sha256_hex("checksum.value", &value.value)?;
    }
    Ok(result_export::ChecksumRef {
        algorithm: value.algorithm,
        canonicalization: value.canonicalization,
        payload_ref: reference(value.payload_ref)?,
        value: value.value,
    })
}

fn diagnostic_class(value: &str) -> Result<result_export::DiagnosticClass, String> {
    match value {
        "solve_blocking" => Ok(result_export::DiagnosticClass::SolveBlocking),
        "rule_check_blocking" => Ok(result_export::DiagnosticClass::RuleCheckBlocking),
        "provenance_warning" => Ok(result_export::DiagnosticClass::ProvenanceWarning),
        "assumption_warning" => Ok(result_export::DiagnosticClass::AssumptionWarning),
        "nonlinear_warning" => Ok(result_export::DiagnosticClass::NonlinearWarning),
        "ip_boundary_warning" => Ok(result_export::DiagnosticClass::IpBoundaryWarning),
        "unit_warning" => Ok(result_export::DiagnosticClass::UnitWarning),
        "export_blocking" => Ok(result_export::DiagnosticClass::ExportBlocking),
        _ => Err(format!(
            "REPORT-PACKAGE-WIRE-ENUM: unsupported diagnostic class {value}"
        )),
    }
}

fn diagnostic_severity(value: &str) -> Result<result_export::DiagnosticSeverity, String> {
    match value {
        "info" => Ok(result_export::DiagnosticSeverity::Info),
        "warning" => Ok(result_export::DiagnosticSeverity::Warning),
        "blocking" => Ok(result_export::DiagnosticSeverity::Blocking),
        _ => Err(format!(
            "REPORT-PACKAGE-WIRE-ENUM: unsupported diagnostic severity {value}"
        )),
    }
}

fn diagnostic(value: DiagnosticDto) -> Result<result_export::Diagnostic, String> {
    Ok(result_export::Diagnostic {
        code: value.code,
        class: diagnostic_class(&value.class)?,
        severity: diagnostic_severity(&value.severity)?,
        source: reference(value.source)?,
        affected_object: reference(value.affected_object)?,
        message: value.message,
        remediation: value.remediation,
        provenance: provenance(value.provenance)?,
    })
}

fn family(value: &str) -> Result<result_export::ResultFamily, String> {
    match value {
        "displacement" => Ok(result_export::ResultFamily::Displacement),
        "rotation" => Ok(result_export::ResultFamily::Rotation),
        "force" => Ok(result_export::ResultFamily::Force),
        "moment" => Ok(result_export::ResultFamily::Moment),
        "reaction" => Ok(result_export::ResultFamily::Reaction),
        "stress" => Ok(result_export::ResultFamily::Stress),
        "section_property" => Ok(result_export::ResultFamily::SectionProperty),
        "ratio" => Ok(result_export::ResultFamily::Ratio),
        "rule_check" => Ok(result_export::ResultFamily::RuleCheck),
        _ => Err(format!(
            "REPORT-PACKAGE-WIRE-ENUM: unsupported result family {value}"
        )),
    }
}

fn dimension(value: &str) -> Result<result_export::DimensionId, String> {
    match value {
        "dimensionless" => Ok(result_export::DimensionId::Dimensionless),
        "length" => Ok(result_export::DimensionId::Length),
        "angle" => Ok(result_export::DimensionId::Angle),
        "force" => Ok(result_export::DimensionId::Force),
        "moment" => Ok(result_export::DimensionId::Moment),
        "stress" => Ok(result_export::DimensionId::Stress),
        "area" => Ok(result_export::DimensionId::Area),
        "section_modulus" => Ok(result_export::DimensionId::SectionModulus),
        "second_moment_area" => Ok(result_export::DimensionId::SecondMomentArea),
        "ratio" => Ok(result_export::DimensionId::Ratio),
        "time" => Ok(result_export::DimensionId::Time),
        "temperature" => Ok(result_export::DimensionId::Temperature),
        "pressure" => Ok(result_export::DimensionId::Pressure),
        "linear_stiffness" => Ok(result_export::DimensionId::LinearStiffness),
        "rotational_stiffness" => Ok(result_export::DimensionId::RotationalStiffness),
        "TBD" => Ok(result_export::DimensionId::Tbd),
        _ => Err(format!(
            "REPORT-PACKAGE-WIRE-ENUM: unsupported dimension {value}"
        )),
    }
}

fn analysis_status(value: &str) -> Result<result_export::AnalysisStatus, String> {
    match value {
        "MODEL_INCOMPLETE" => Ok(result_export::AnalysisStatus::ModelIncomplete),
        "MECHANICS_SOLVED" => Ok(result_export::AnalysisStatus::MechanicsSolved),
        "RULE_INPUTS_INCOMPLETE" => Ok(result_export::AnalysisStatus::RuleInputsIncomplete),
        "USER_RULE_CHECKED" => Ok(result_export::AnalysisStatus::UserRuleChecked),
        "USER_RULE_FAILED" => Ok(result_export::AnalysisStatus::UserRuleFailed),
        "HUMAN_REVIEW_REQUIRED" => Ok(result_export::AnalysisStatus::HumanReviewRequired),
        _ => Err(format!(
            "REPORT-PACKAGE-WIRE-ENUM: unsupported analysis status {value}"
        )),
    }
}

fn boundary(value: ProfessionalBoundaryDto) -> result_export::ProfessionalBoundary {
    result_export::ProfessionalBoundary {
        human_review_required: value.human_review_required,
        software_makes_compliance_claim: value.software_makes_compliance_claim,
        software_makes_certification_claim: value.software_makes_certification_claim,
        software_makes_sealing_claim: value.software_makes_sealing_claim,
        software_makes_approval_claim: value.software_makes_approval_claim,
        software_makes_authentication_claim: value.software_makes_authentication_claim,
    }
}

fn result_envelope(
    value: ResultEnvelopeDto,
    linked_solver: &LinkedSolverIdentity<'_>,
) -> Result<result_export::ResultEnvelope, String> {
    if value.solver_name != linked_solver.solver_name
        || value.solver_version != linked_solver.solver_version
        || value.solver_build_ref != linked_solver.solver_build_ref
    {
        return Err("REPORT-PACKAGE-SOLVER-IDENTITY-MISMATCH: result envelope identity does not match the linked product-physics component".to_string());
    }
    Ok(result_export::ResultEnvelope {
        envelope_id: value.envelope_id,
        schema_version: value.schema_version,
        model_ref: reference(value.model_ref)?,
        run_ref: reference(value.run_ref)?,
        solver_name: linked_solver.solver_name.to_string(),
        solver_version: linked_solver.solver_version.to_string(),
        solver_build_ref: linked_solver.solver_build_ref.to_string(),
        unit_system_ref: reference(value.unit_system_ref)?,
        load_basis_refs: value
            .load_basis_refs
            .into_iter()
            .map(reference)
            .collect::<Result<Vec<_>, _>>()?,
        result_sets: value
            .result_sets
            .into_iter()
            .map(|set| {
                Ok(result_export::ResultSet {
                    set_id: set.set_id,
                    set_type: set.set_type,
                    basis_ref: reference(set.basis_ref)?,
                    values: set
                        .values
                        .into_iter()
                        .map(|item| {
                            if !item.magnitude.is_finite() {
                                return Err(format!(
                                    "REPORT-PACKAGE-NON-FINITE-RESULT: {}",
                                    item.result_id
                                ));
                            }
                            Ok(result_export::QuantityResult {
                                result_id: item.result_id,
                                family: family(&item.family)?,
                                object_ref: reference(item.object_ref)?,
                                basis_ref: reference(item.basis_ref)?,
                                station_ref: item.station_ref.map(reference).transpose()?,
                                magnitude: item.magnitude,
                                unit: item.unit,
                                dimension: dimension(&item.dimension)?,
                                metadata: item.metadata.map(|metadata| {
                                    result_export::ResultMetadata {
                                        component: metadata.component,
                                        coordinate_system: metadata.coordinate_system,
                                        location: metadata.location,
                                        basis: metadata.basis,
                                        sign_convention: metadata.sign_convention,
                                    }
                                }),
                                diagnostics: item
                                    .diagnostics
                                    .into_iter()
                                    .map(diagnostic)
                                    .collect::<Result<Vec<_>, _>>()?,
                                trace_chain: item
                                    .trace_chain
                                    .into_iter()
                                    .map(|trace| {
                                        Ok(result_export::ResultTraceLink {
                                            trace_id: trace.trace_id,
                                            trace_type: trace.trace_type,
                                            source_ref: reference(trace.source_ref)?,
                                            target_ref: reference(trace.target_ref)?,
                                            provenance: provenance(trace.provenance)?,
                                            diagnostics: trace
                                                .diagnostics
                                                .into_iter()
                                                .map(diagnostic)
                                                .collect::<Result<Vec<_>, _>>()?,
                                        })
                                    })
                                    .collect::<Result<Vec<_>, String>>()?,
                                provenance: provenance(item.provenance)?,
                            })
                        })
                        .collect::<Result<Vec<_>, String>>()?,
                })
            })
            .collect::<Result<Vec<_>, String>>()?,
        diagnostics: value
            .diagnostics
            .into_iter()
            .map(diagnostic)
            .collect::<Result<Vec<_>, _>>()?,
        provenance: provenance(value.provenance)?,
        reproducibility: result_export::ReproducibilityRefs {
            model_hash: checksum(value.reproducibility.model_hash)?,
            run_hashes: value
                .reproducibility
                .run_hashes
                .into_iter()
                .map(checksum)
                .collect::<Result<Vec<_>, _>>()?,
            audit_manifest_ref: reference(value.reproducibility.audit_manifest_ref)?,
            deterministic_ordering: value.reproducibility.deterministic_ordering,
        },
        analysis_status: value
            .analysis_status
            .iter()
            .map(|item| analysis_status(item))
            .collect::<Result<Vec<_>, _>>()?,
        rule_pack_refs: value
            .rule_pack_refs
            .into_iter()
            .map(|item| {
                Ok(result_export::RulePackExportRef {
                    rule_pack_id: item.rule_pack_id,
                    version: item.version,
                    checksum: checksum(item.checksum)?,
                    source_notice: item.source_notice,
                    redistribution_status: result_redistribution(&item.redistribution_status)?,
                    completeness_status: item.completeness_status,
                    private_payload_redacted: item.private_payload_redacted,
                })
            })
            .collect::<Result<Vec<_>, String>>()?,
        professional_boundary: boundary(value.professional_boundary),
    })
}

pub struct LinkedSolverIdentity<'a> {
    pub solver_name: &'a str,
    pub solver_version: &'a str,
    pub solver_build_ref: &'a str,
}

pub fn assemble_wire_request(
    request: ReportPackageRequest,
    linked_solver: &LinkedSolverIdentity<'_>,
) -> Result<package::ReportPackageContainerOutcome, String> {
    if request.rule_check_aggregate.is_some()
        || request.solve_rule_check_status != "RULE_INPUTS_INCOMPLETE"
    {
        return Err("REPORT-PACKAGE-RULE-PACK-BINDING-UNAVAILABLE: active rule-pack metadata is unavailable at this boundary".to_string());
    }
    let report: RenderableReportInput = serde_json::from_value(request.report)
        .map_err(|error| format!("REPORT-PACKAGE-REPORT-DTO-INVALID: {error}"))?;
    let audit_manifest = audit_manifest(request.audit_manifest, linked_solver)?;
    let result_envelopes = request
        .result_envelopes
        .into_iter()
        .map(|value| result_envelope(value, linked_solver))
        .collect::<Result<Vec<_>, _>>()?;
    let source_model_ref = package_reference(request.source_model_ref)?;
    let source_basis_refs = request
        .source_basis_refs
        .into_iter()
        .map(package_reference)
        .collect::<Result<Vec<_>, _>>()?;
    let input = package::ReportPackageContainerInput {
        package_id: request.package_id,
        export_profile_id: request.export_profile_id,
        source_model_ref,
        source_basis_refs,
        report: &report,
        audit_manifest: &audit_manifest,
        result_envelopes: &result_envelopes,
        state_comparison_handoff_records: &request.state_comparison_handoff_records,
    };
    Ok(package::assemble_report_package_container(&input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_wire_values_require_bare_lowercase_64_hex() {
        let valid = "a".repeat(64);
        assert!(require_sha256_hex("test", &valid).is_ok());
        for invalid in [
            "a".repeat(63),
            "A".repeat(64),
            format!("sha256:{valid}"),
            "g".repeat(64),
        ] {
            assert!(require_sha256_hex("test", &invalid)
                .expect_err("malformed SHA-256 must block")
                .contains("REPORT-PACKAGE-SHA256-INVALID"));
        }
    }

    #[test]
    fn stiffness_dimensions_cross_the_native_report_package_boundary() {
        assert_eq!(
            dimension("linear_stiffness").expect("linear stiffness accepted"),
            result_export::DimensionId::LinearStiffness
        );
        assert_eq!(
            dimension("rotational_stiffness").expect("rotational stiffness accepted"),
            result_export::DimensionId::RotationalStiffness
        );
    }

    #[test]
    fn audit_and_result_sha256_dtos_reject_malformed_values() {
        let audit_error = audit_hash(HashRecordDto {
            algorithm: "sha256".to_string(),
            canonicalization: "project_local_deterministic_json".to_string(),
            payload_kind: "model_json".to_string(),
            payload_ref: "model:test".to_string(),
            value: "ABC".to_string(),
        })
        .expect_err("audit SHA-256 must be exact");
        assert!(audit_error.contains("REPORT-PACKAGE-SHA256-INVALID"));

        let checksum_error = checksum(ChecksumDto {
            algorithm: "sha256".to_string(),
            canonicalization: "rfc8785_jcs".to_string(),
            payload_ref: ReferenceDto {
                ref_type: "result".to_string(),
                ref_id: "result:test".to_string(),
            },
            value: "sha256:abc".to_string(),
        })
        .expect_err("result SHA-256 must be exact");
        assert!(checksum_error.contains("REPORT-PACKAGE-SHA256-INVALID"));
    }

    #[test]
    fn linked_solver_identity_is_enforced_for_audit_and_result_wire_values() {
        let source: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../validation/witness/inputs/del1005_export_results_success_input.json"
        ))
        .expect("invented export-results witness must parse");
        let request: ReportPackageRequest =
            serde_json::from_value(source["export_results"].clone())
                .expect("invented report-package request must deserialize");
        let error = assemble_wire_request(
            request,
            &LinkedSolverIdentity {
                solver_name: "wrong-solver",
                solver_version: "0.1.0",
                solver_build_ref: "wrong-solver@0.1.0",
            },
        )
        .expect_err("audit identity mismatch must block");
        assert!(error.contains("REPORT-PACKAGE-SOLVER-IDENTITY-MISMATCH"));

        let mut source: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../validation/witness/inputs/del1005_export_results_success_input.json"
        ))
        .expect("invented export-results witness must parse");
        let audit_solver = &mut source["export_results"]["audit_manifest"]["solver_version"];
        audit_solver["solver_name"] = serde_json::json!("wrong-solver");
        audit_solver["solver_build_ref"] = serde_json::json!("wrong-solver@0.1.0");
        let request: ReportPackageRequest =
            serde_json::from_value(source["export_results"].clone())
                .expect("invented report-package request must deserialize");
        let error = assemble_wire_request(
            request,
            &LinkedSolverIdentity {
                solver_name: "wrong-solver",
                solver_version: "0.1.0",
                solver_build_ref: "wrong-solver@0.1.0",
            },
        )
        .expect_err("result identity mismatch must block");
        assert!(error.contains("REPORT-PACKAGE-SOLVER-IDENTITY-MISMATCH"));
    }
}
