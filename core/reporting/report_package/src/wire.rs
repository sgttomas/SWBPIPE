//! The report-package wire request, version 2.0.0 (DEL-08-01 Design
//! `report-package-current-results.md` §2.3), shared by the desktop's
//! `save_report_package` and the headless runner's `export-results`.
//!
//! The request carries the DEL-08-04 0.3.0 result document and the source
//! result it derives from as JSON values. [`assemble_wire_request`] converts
//! the audit manifest, runs [`crate::bindings::check_request`] and only then
//! assembles. The typed 1.0.0 result-envelope DTOs and the fresh-result
//! refusal are retired with the 1.0.0 package. Standard claim fence applies
//! (claims taxonomy per DEC-081).

use crate as package;
use crate::admitted::{self, AdmittedSet};
use open_pipe_stress_audit_manifest as audit;
use open_pipe_stress_report_renderer::RenderableReportInput;
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
    /// The DEL-08-04 0.3.0 result-export documents (exactly one).
    pub result_documents: Vec<Value>,
    /// The received result each document derives from, in the same order. A
    /// validation and screening input only; never a package member.
    pub source_results: Vec<Value>,
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

#[derive(Debug, Clone, Deserialize)]
pub struct SolverVersionDto {
    pub solver_name: String,
    pub solver_version: String,
    pub solver_build_ref: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProfessionalBoundaryDto {
    pub human_review_required: bool,
    pub software_makes_compliance_claim: bool,
    pub software_makes_certification_claim: bool,
    pub software_makes_sealing_claim: bool,
    pub software_makes_approval_claim: bool,
    #[serde(default)]
    pub software_makes_authentication_claim: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RulePackAuditRefDto {
    pub rule_pack_id: String,
    pub rule_pack_version: String,
    pub source_notice: String,
    pub redistribution_status: String,
    pub checksum: Option<HashRecordDto>,
    pub private_payload_redacted: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AssetManifestEntryDto {
    pub asset_id: String,
    pub payload_kind: String,
    pub privacy_class: String,
    pub hash: Option<HashRecordDto>,
    pub provenance: String,
}

#[derive(Debug, Clone, Deserialize)]
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
        "rfc8785_jcs" => Ok(audit::Canonicalization::Rfc8785Jcs),
        "rule_pack_payload_jcs" => Ok(audit::Canonicalization::RulePackPayloadJcs),
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

pub struct LinkedSolverIdentity<'a> {
    pub solver_name: &'a str,
    pub solver_version: &'a str,
    pub solver_build_ref: &'a str,
}

pub fn assemble_wire_request(
    request: ReportPackageRequest,
    linked_solver: &LinkedSolverIdentity<'_>,
) -> Result<package::ReportPackageContainerOutcome, String> {
    assemble_wire_request_in(request, linked_solver, admitted::PRODUCTION)
}

/// [`assemble_wire_request`] against `admitted` (W-11, DEL-08-01 Design
/// `exact-route-report-sections.md` §5.5), which it forwards unchanged to every
/// item-0 check on its path.
pub(crate) fn assemble_wire_request_in(
    request: ReportPackageRequest,
    linked_solver: &LinkedSolverIdentity<'_>,
    admitted: AdmittedSet,
) -> Result<package::ReportPackageContainerOutcome, String> {
    // Item 0, the route preflight, comes before any other check, including the
    // audit conversion (DEL-08-01 Design §2.3).
    for document in &request.result_documents {
        crate::document_form::check_route_admitted_in(document, admitted)?;
    }
    let audit_manifest = audit_manifest(request.audit_manifest.clone(), linked_solver)?;
    crate::bindings::check_request_in(&request, &audit_manifest, linked_solver, admitted)?;
    let report: RenderableReportInput = serde_json::from_value(request.report)
        .map_err(|error| format!("REPORT-PACKAGE-REPORT-DTO-INVALID: {error}"))?;
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
        result_documents: &request.result_documents,
        state_comparison_handoff_records: &request.state_comparison_handoff_records,
    };
    Ok(package::assemble_report_package_container_in(&input, admitted))
}

#[cfg(test)]
mod tests {
    use super::*;

    const COMPONENT_PROVENANCE_PROJECTION: &str = include_str!(
        "../../../../fixtures/reports/invented/component_provenance_cross_layer_projection.json"
    );
    const REPORT_PACKAGE_WITNESS: &str = include_str!(
        "../../../../validation/witness/inputs/del1005_export_results_success_input.json"
    );

    #[derive(Debug, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    struct ComponentProvenanceProjection {
        schema_version: String,
        present_component: PresentComponentProvenanceCase,
        missing_component: MissingComponentProvenanceCase,
    }

    #[derive(Debug, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    struct PresentComponentProvenanceCase {
        value: serde_json::Value,
        provenance_note: serde_json::Value,
    }

    #[derive(Debug, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    struct MissingComponentProvenanceCase {
        value: serde_json::Value,
        provenance_note: serde_json::Value,
        diagnostic: serde_json::Value,
    }

    fn parse_component_provenance_projection(
        source: &str,
    ) -> Result<ComponentProvenanceProjection, String> {
        let projection: ComponentProvenanceProjection = serde_json::from_str(source)
            .map_err(|error| format!("COMPONENT-PROVENANCE-PROJECTION-INVALID: {error}"))?;
        if projection.schema_version != "1.0.0" {
            return Err(format!(
                "COMPONENT-PROVENANCE-PROJECTION-VERSION-UNSUPPORTED: {}",
                projection.schema_version
            ));
        }
        Ok(projection)
    }

    fn request_with_component_provenance_projection() -> serde_json::Value {
        let projection = parse_component_provenance_projection(COMPONENT_PROVENANCE_PROJECTION)
            .expect("shared component-provenance projection contract must validate");
        let mut witness: serde_json::Value = serde_json::from_str(REPORT_PACKAGE_WITNESS)
            .expect("invented report-package witness must parse");
        let sections = witness
            .pointer_mut("/export_results/report/report_sections")
            .expect("report sections must exist in the report-package witness");
        sections["user_supplied_values"] = serde_json::json!([
            projection.present_component.value,
            projection.missing_component.value
        ]);
        sections["provenance_notes"] = serde_json::json!([
            projection.present_component.provenance_note,
            projection.missing_component.provenance_note
        ]);
        sections["diagnostics"] = serde_json::json!([projection.missing_component.diagnostic]);
        witness["export_results"].clone()
    }

    /// The identity the committed witness was generated against: the linked
    /// product-physics component at generation (the witness generator asserts
    /// it equals the linked one).
    fn linked_product_solver() -> LinkedSolverIdentity<'static> {
        let witness: serde_json::Value = serde_json::from_str(REPORT_PACKAGE_WITNESS).unwrap();
        let solver = &witness["export_results"]["audit_manifest"]["solver_version"];
        let leak = |key: &str| -> &'static str { Box::leak(solver[key].as_str().unwrap().to_string().into_boxed_str()) };
        LinkedSolverIdentity {
            solver_name: leak("solver_name"),
            solver_version: leak("solver_version"),
            solver_build_ref: leak("solver_build_ref"),
        }
    }

    #[test]
    fn component_provenance_projection_contract_rejects_root_drift() {
        let unsupported_version = COMPONENT_PROVENANCE_PROJECTION.replacen(
            "\"schema_version\": \"1.0.0\"",
            "\"schema_version\": \"2.0.0\"",
            1,
        );
        let version_error = parse_component_provenance_projection(&unsupported_version)
            .expect_err("unsupported projection schema version must fail closed");
        assert!(version_error.contains("VERSION-UNSUPPORTED"));

        let mut unexpected_root: serde_json::Value =
            serde_json::from_str(COMPONENT_PROVENANCE_PROJECTION)
                .expect("shared component-provenance projection must parse as JSON");
        unexpected_root["unexpected_root_field"] = serde_json::json!(true);
        let unknown_error = parse_component_provenance_projection(
            &serde_json::to_string(&unexpected_root).expect("mutated fixture must serialize"),
        )
        .expect_err("unexpected projection root field must fail closed");
        assert!(unknown_error.contains("unknown field `unexpected_root_field`"));
    }

    #[test]
    fn component_provenance_projection_crosses_wire_and_rendered_package_boundary() {
        let source = request_with_component_provenance_projection();
        let request: ReportPackageRequest = serde_json::from_value(source)
            .expect("shared component-provenance projection must deserialize in the wire request");
        let outcome = assemble_wire_request(request, &linked_product_solver())
            .expect("shared component-provenance projection must assemble");
        let html_member = outcome
            .members
            .iter()
            .find(|member| member.role == package::ROLE_REPORT_HTML)
            .expect("assembled package must contain its canonical HTML member");
        let html = std::str::from_utf8(&html_member.bytes).expect("HTML member must be UTF-8");

        assert!(html.contains("component-provenance:component:C-110"));
        assert!(html.contains("component:C-110"));
        assert!(html.contains(
            "component.provenance=invented_example_user_entered_bend_values_no_code_table"
        ));
        assert!(html.contains("COMPONENT_PROVENANCE_MISSING"));
        assert!(html.contains("component-provenance:component:C-140"));
        assert!(html.contains("component provenance missing"));
        assert!(
            !outcome.export_blocked,
            "missing provenance remains a warning"
        );
    }

    #[test]
    fn malformed_component_provenance_fails_closed_at_rust_consumer_validation() {
        let mut source = request_with_component_provenance_projection();
        source["report"]["report_sections"]["user_supplied_values"][0]["provenance"]
            ["privacy_classification"] = serde_json::json!("public_permissive");
        let request: ReportPackageRequest = serde_json::from_value(source)
            .expect("outer wire request deliberately retains the report as untyped JSON");
        let error = assemble_wire_request(request, &linked_product_solver())
            .expect_err("malformed component provenance must fail closed");

        assert!(error.contains("REPORT-PACKAGE-REPORT-DTO-INVALID"));
        assert!(error.contains("public_permissive"), "{error}");
    }

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
    fn audit_sha256_dtos_reject_malformed_values() {
        let audit_error = audit_hash(HashRecordDto {
            algorithm: "sha256".to_string(),
            canonicalization: "project_local_deterministic_json".to_string(),
            payload_kind: "model_json".to_string(),
            payload_ref: "model:test".to_string(),
            value: "ABC".to_string(),
        })
        .expect_err("audit SHA-256 must be exact");
        assert!(audit_error.contains("REPORT-PACKAGE-SHA256-INVALID"));
    }

    /// DEL-08-01 Design §2.4: the two recorded bases decode; other labels
    /// (the report's own `JCS` declaration among them) do not.
    #[test]
    fn recorded_checksum_bases_decode_by_their_tokens() {
        assert_eq!(audit_canonicalization("rfc8785_jcs"), Ok(audit::Canonicalization::Rfc8785Jcs));
        assert_eq!(audit_canonicalization("rule_pack_payload_jcs"), Ok(audit::Canonicalization::RulePackPayloadJcs));
        for label in ["JCS", "jcs_compatible", "openpipestress_jcs_ijson_v1", "not recorded"] {
            assert!(audit_canonicalization(label).unwrap_err().contains("REPORT-PACKAGE-WIRE-ENUM"), "{label}");
        }
    }

    /// The 1.0.0 request shape is no longer read: its typed result envelopes
    /// leave the 2.0.0 request without result documents.
    #[test]
    fn a_1_0_0_request_shape_is_refused_by_the_typed_decode() {
        let mut source: serde_json::Value = serde_json::from_str(REPORT_PACKAGE_WITNESS).unwrap();
        let request = source["export_results"].as_object_mut().unwrap();
        let documents = request.remove("result_documents").unwrap();
        request.remove("source_results");
        request.insert("result_envelopes".to_string(), documents);
        let error = serde_json::from_value::<ReportPackageRequest>(source["export_results"].clone()).unwrap_err();
        assert!(error.to_string().contains("missing field `result_documents`"), "{error}");
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
        audit_solver["solver_version"] = serde_json::json!("0.1.0");
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
        assert!(error.contains("result document"), "{error}");
    }
}
