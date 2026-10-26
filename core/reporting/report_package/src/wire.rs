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
use serde::{Deserialize, Serialize};
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

/// A struct in positional (JSON array) form in the report (Codex review
/// round 3 of #1648).
pub const REQUEST_POSITIONAL: &str = "REPORT-PACKAGE-REQUEST-POSITIONAL";

/// The request's members in declaration order (its positional form).
pub const REQUEST_MEMBERS: &[&str] = &["package_id", "export_profile_id", "source_model_ref", "source_basis_refs", "report", "audit_manifest", "result_documents", "source_results", "state_comparison_handoff_records", "rule_check_aggregate", "solve_rule_check_status"];

/// The path of the first struct the request writes in positional (JSON
/// array) form, at any depth, or `None` (W-11, Codex review round 3 of #1648;
/// `open_pipe_stress_report_renderer::object_form`). The shared decode reads
/// a positional struct by field order, while the native save's policy checks
/// read members by key, so the save refuses a positional struct before any of
/// them. A positional request is `the request`. The typed members
/// (`source_model_ref`, `source_basis_refs`, `audit_manifest`) and the report
/// (as the renderer's input) are each decoded by reference and compared with
/// their JSON; the untyped members (result documents, source results,
/// handoff records) have no struct decoder and are read by key everywhere,
/// so they need no comparison. A member that does not decode yields nothing
/// here: the decode refuses it later.
pub fn request_positional_struct_path(request: &Value) -> Option<String> {
    use open_pipe_stress_report_renderer::object_form::{positional_struct_path, report_positional_struct_path};
    fn typed<T: Serialize + for<'de> Deserialize<'de>>(value: Option<&Value>, member: &str) -> Option<String> {
        let value = value?;
        let decoded = T::deserialize(value).ok()?;
        positional_struct_path(&decoded, value).map(|path| under(member, &path))
    }
    fn under(member: &str, path: &str) -> String {
        if path == "the input" { member.to_string() } else if path.starts_with('[') { format!("{member}{path}") } else { format!("{member}.{path}") }
    }
    let fields = match request {
        Value::Array(_) => return Some("the request".to_string()),
        Value::Object(fields) => fields,
        _ => return None,
    };
    typed::<ReferenceDto>(fields.get("source_model_ref"), "source_model_ref")
        .or_else(|| typed::<Vec<ReferenceDto>>(fields.get("source_basis_refs"), "source_basis_refs"))
        .or_else(|| typed::<AuditManifestDto>(fields.get("audit_manifest"), "audit_manifest"))
        .or_else(|| fields.get("report").and_then(report_positional_struct_path).map(|path| under("report", &path)))
}

/// Decode a request, refusing an invalid one as `<kind> at <member>` only:
/// the error kind and the top-level member, never the offending value or key
/// (W-11 slice (c), Codex review round 3 of #1639). Each member is moved out
/// and decoded on its own, as the whole decode decodes it, so the request is
/// never copied; an unknown top-level member is ignored, as there.
pub fn decode_report_package_request(value: Value) -> Result<ReportPackageRequest, String> {
    use serde::de::DeserializeOwned;
    let kind = |error: serde_json::Error| open_pipe_stress_report_renderer::serde_error_kind(&error.to_string());
    // A positional request (a JSON array) is read by field order, as serde
    // reads it; every field is positional there, the optional one included.
    let mut fields = match value {
        Value::Object(fields) => fields,
        Value::Array(items) => open_pipe_stress_report_renderer::positional_members(items, REQUEST_MEMBERS, REQUEST_MEMBERS.len())
            .map_err(|kind| format!("{kind} at the request"))?,
        _ => return Err("invalid type at the request".to_string()),
    };
    let mut take = |key: &str| fields.remove(key);
    fn decoded<T: DeserializeOwned>(value: Option<Value>, key: &str, kind: &dyn Fn(serde_json::Error) -> &'static str, optional: bool) -> Result<T, String> {
        match value {
            None if !optional => Err(format!("missing field at {key}")),
            None => serde_json::from_value(Value::Null).map_err(|error| format!("{} at {key}", kind(error))),
            Some(value) => serde_json::from_value(value).map_err(|error| format!("{} at {key}", kind(error))),
        }
    }
    // Every member, exhaustively (a new member is a compile error here).
    Ok(ReportPackageRequest {
        package_id: decoded(take("package_id"), "package_id", &kind, false)?,
        export_profile_id: decoded(take("export_profile_id"), "export_profile_id", &kind, false)?,
        source_model_ref: decoded(take("source_model_ref"), "source_model_ref", &kind, false)?,
        source_basis_refs: decoded(take("source_basis_refs"), "source_basis_refs", &kind, false)?,
        report: decoded(take("report"), "report", &kind, false)?,
        audit_manifest: decoded(take("audit_manifest"), "audit_manifest", &kind, false)?,
        result_documents: decoded(take("result_documents"), "result_documents", &kind, false)?,
        source_results: decoded(take("source_results"), "source_results", &kind, false)?,
        state_comparison_handoff_records: decoded(take("state_comparison_handoff_records"), "state_comparison_handoff_records", &kind, false)?,
        rule_check_aggregate: decoded(take("rule_check_aggregate"), "rule_check_aggregate", &kind, true)?,
        solve_rule_check_status: decoded(take("solve_rule_check_status"), "solve_rule_check_status", &kind, false)?,
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceDto {
    pub ref_type: String,
    pub ref_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashRecordDto {
    pub algorithm: String,
    pub canonicalization: String,
    pub payload_kind: String,
    pub payload_ref: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolverVersionDto {
    pub solver_name: String,
    pub solver_version: String,
    pub solver_build_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfessionalBoundaryDto {
    pub human_review_required: bool,
    pub software_makes_compliance_claim: bool,
    pub software_makes_certification_claim: bool,
    pub software_makes_sealing_claim: bool,
    pub software_makes_approval_claim: bool,
    #[serde(default)]
    pub software_makes_authentication_claim: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulePackAuditRefDto {
    pub rule_pack_id: String,
    pub rule_pack_version: String,
    pub source_notice: String,
    pub redistribution_status: String,
    pub checksum: Option<HashRecordDto>,
    pub private_payload_redacted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetManifestEntryDto {
    pub asset_id: String,
    pub payload_kind: String,
    pub privacy_class: String,
    pub hash: Option<HashRecordDto>,
    pub provenance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

pub(crate) fn audit_manifest(
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
    // The report is untyped here and decoded only by the renderer, which reads
    // a positional struct by field order while the bindings read keys: a
    // positional struct anywhere in it refuses (Codex review round 3 of #1648).
    if let Some(path) = open_pipe_stress_report_renderer::object_form::report_positional_struct_path(&request.report) {
        return Err(format!("{REQUEST_POSITIONAL}: the report writes a struct in positional (array) form at report.{path}"));
    }
    let audit_manifest = audit_manifest(request.audit_manifest.clone(), linked_solver)?;
    crate::bindings::check_request_in(&request, &audit_manifest, linked_solver, admitted)?;
    // The refusal names the JSON path and the error kind, never the offending
    // value or key (W-11 slice (c), Codex review round 3 of #1639).
    let report: RenderableReportInput = open_pipe_stress_report_renderer::decode_renderable_report_input_as(
        request.report,
        "REPORT-PACKAGE-REPORT-DTO-INVALID",
        "report",
    )?;
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

    /// Owner ruling on #1639 ("C1 b, C2 a", owner, 2026-10-26): the package
    /// renders its members as the rendered report does. A local-private
    /// package with an unresolved FR-REP-003 record assembles with the
    /// non-blocking warning in its HTML member; a quarantined record blocks it;
    /// any other export context is refused.
    #[test]
    fn the_package_renders_unresolved_records_as_the_report_does() {
        let with_record = |redistribution: &str, review: &str| {
            let mut source = request_with_component_provenance_projection();
            let record = serde_json::json!({
                "source_name": "Invented material datasheet", "source_location": "invented-datasheet-shelf", "source_license": "invented-license",
                "contributor": "invented contributor", "contributor_certification": "invented certification",
                "redistribution_status": redistribution, "review_status": review, "privacy_classification": "private_project_data"
            });
            let sections = &mut source["report"]["report_sections"];
            sections["provenance_notes"].as_array_mut().unwrap().push(record.clone());
            sections["user_supplied_values"].as_array_mut().unwrap().push(serde_json::json!({
                "value_id": "material-provenance:material:invented-steel", "value_category": "report_metadata",
                "source": {"ref_type": "material", "ref_id": "material:invented-steel"}, "provenance": record,
                "privacy_classification": "private_project_data", "required_for": ["reporting", "human_review"],
                "review_status": review, "missing_data_finding": false
            }));
            source
        };
        let assemble = |source: serde_json::Value| {
            let request: ReportPackageRequest = serde_json::from_value(source).unwrap();
            assemble_wire_request(request, &linked_product_solver())
        };
        assert_eq!(with_record("private_only", "pending")["report"]["report_context"]["export_context"], "local_private");
        let outcome = assemble(with_record("unknown", "pending")).expect("an unresolved record assembles locally");
        assert!(!outcome.export_blocked, "{:?}", outcome.blocking_reasons);
        let html = std::str::from_utf8(&outcome.members.iter().find(|member| member.role == package::ROLE_REPORT_HTML).unwrap().bytes).unwrap();
        assert!(html.contains("<td>REPORT_SECTION_PROVENANCE_UNRESOLVED</td><td>warning</td><td>user_supplied_value material-provenance:material:invented-steel: redistribution status unknown; "));
        let blocked = assemble(with_record("private_only", "quarantined")).expect("assembles, blocked");
        assert!(blocked.export_blocked);
        assert!(blocked.blocking_reasons.iter().any(|reason| reason.contains("REPORT_PROVENANCE_DISPOSITION_BLOCKED")), "{:?}", blocked.blocking_reasons);
        let mut public = with_record("unknown", "pending");
        public["report"]["report_context"]["export_context"] = serde_json::json!("public_report");
        match assemble(public) {
            Err(error) => assert!(error.contains("REPORT-PACKAGE-REPORT-CONTEXT-NOT-LOCAL-PRIVATE") || error.contains("local_private"), "{error}"),
            Ok(outcome) => {
                assert!(outcome.export_blocked);
                assert!(outcome.blocking_reasons.iter().any(|reason| reason.contains("REPORT_SECTION_PROVENANCE_UNRESOLVED")), "{:?}", outcome.blocking_reasons);
            }
        }
    }

    /// Codex review round 3 of #1639: a package refused for its provenance
    /// records carries no authored id (value ids, an assumption id, library
    /// owners) in its blocking reasons, its ZIP manifest or any member.
    #[test]
    fn a_refused_package_carries_no_authored_id() {
        let texts = ["/Users/invented-user/private-datasheets/steel-a.pdf", "C:/x.pdf", "Invented Private Client Project"];
        let mut source = request_with_component_provenance_projection();
        let record = serde_json::json!({
            "source_name": "Invented material datasheet", "source_location": "invented-datasheet-shelf", "source_license": "invented-license",
            "contributor": "invented contributor", "contributor_certification": "invented certification",
            "redistribution_status": "private_only", "review_status": "quarantined", "privacy_classification": "private_project_data"
        });
        let sections = &mut source["report"]["report_sections"];
        for (kind, id) in [("material", texts[0]), ("section", texts[1]), ("component", texts[2]), ("support", texts[0])] {
            sections["user_supplied_values"].as_array_mut().unwrap().push(serde_json::json!({
                "value_id": format!("{kind}-provenance:{kind}:{id}"), "value_category": "report_metadata",
                "source": {"ref_type": kind, "ref_id": format!("{kind}:{id}")}, "provenance": record.clone(),
                "privacy_classification": "private_project_data", "required_for": ["reporting"],
                "review_status": "quarantined", "missing_data_finding": false
            }));
        }
        let mut provenance = record.clone();
        provenance["review_status"] = serde_json::json!("accepted");
        provenance["redistribution_status"] = serde_json::json!("invented_non_engineering_example");
        provenance["privacy_classification"] = serde_json::json!("invented_public_example");
        sections["assumptions"] = serde_json::json!([{
            "assumption_id": format!("assumption:{} {}", texts[2], texts[1]), "owner": "invented owner",
            "source": {"ref_type": "model", "ref_id": "invented"}, "affected_scope": {"ref_type": "model", "ref_id": "invented"},
            "statement": "Invented assumption.", "review_status": "quarantined", "expiration_or_re_review_trigger": null,
            "effect": {"mechanics_solve_qualified": true, "user_rule_check_qualified": false, "report_completeness": "qualified", "human_review_required": true},
            "provenance": provenance
        }]);
        source["report"]["library_references"] = serde_json::json!([{
            "reference_id": format!("library-ref:{}", texts[1]), "library_kind": "material", "library_id": texts[2],
            "library_version": "not recorded", "record_id": texts[0], "entity_refs": [format!("material:{}", texts[0])], "provenance": record
        }]);
        let request: ReportPackageRequest = serde_json::from_value(source).unwrap();
        let outcome = assemble_wire_request(request, &linked_product_solver()).expect("assembles, blocked");
        assert!(outcome.export_blocked);
        let manifest = &outcome.members[0];
        assert_eq!(manifest.role, package::ROLE_PACKAGE_MANIFEST);
        let reasons = outcome.blocking_reasons.join("\n");
        for private in texts {
            assert!(!reasons.contains(private), "{private} in {reasons}");
            for member in &outcome.members {
                assert!(!member.bytes.windows(private.len()).any(|window| window == private.as_bytes()), "{private} in {}", member.file_name);
            }
            assert!(!outcome.container_bytes.windows(private.len()).any(|window| window == private.as_bytes()), "{private} in the container");
        }
        assert!(outcome.blocking_reasons.iter().any(|reason| reason.contains("(assumptions[0] review_status quarantined;")));
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

        // The refusal names the path and the kind, and never echoes the value:
        // an echoed value can be a private path (Codex review round 3 of #1639).
        assert_eq!(
            error,
            "REPORT-PACKAGE-REPORT-DTO-INVALID: unknown variant at report.report_sections.user_supplied_values[0].provenance.privacy_classification"
        );
        assert!(!error.contains("public_permissive"), "{error}");
    }

    /// Codex review round 3 of #1639: a typed path given as an invalid value
    /// in the report, in a typed request member or in the request's shape is
    /// refused by path and kind only, never echoed.
    #[test]
    fn request_and_report_decode_refusals_never_echo_a_value() {
        let typed = "/Users/invented-user/private-datasheets/steel-a.pdf";
        for (member, path) in [
            ("review_status", "report.report_sections.provenance_notes[0].review_status"),
            ("redistribution_status", "report.report_sections.provenance_notes[0].redistribution_status"),
        ] {
            let mut source = request_with_component_provenance_projection();
            source["report"]["report_sections"]["provenance_notes"][0][member] = serde_json::json!(typed);
            let request = decode_report_package_request(source).unwrap();
            let error = assemble_wire_request(request, &linked_product_solver()).unwrap_err();
            assert_eq!(error, format!("REPORT-PACKAGE-REPORT-DTO-INVALID: unknown variant at {path}"));
        }
        let mut source = request_with_component_provenance_projection();
        source["source_model_ref"] = serde_json::json!(typed);
        let error = decode_report_package_request(source).unwrap_err();
        assert_eq!(error, "invalid type at source_model_ref");
        let mut source = request_with_component_provenance_projection();
        source.as_object_mut().unwrap().remove("result_documents");
        assert_eq!(decode_report_package_request(source).unwrap_err(), "missing field at result_documents");
        assert_eq!(decode_report_package_request(serde_json::json!(typed)).unwrap_err(), "invalid type at the request");
        // The positional (array) form decodes as the whole decode reads it.
        let source = request_with_component_provenance_projection();
        let positional = serde_json::Value::Array(REQUEST_MEMBERS.iter().map(|key| source[*key].clone()).collect());
        let whole: ReportPackageRequest = serde_json::from_value(positional.clone()).unwrap();
        assert_eq!(format!("{:?}", decode_report_package_request(positional.clone()).unwrap()), format!("{whole:?}"));
        let mut short_form = positional;
        short_form.as_array_mut().unwrap().pop();
        assert_eq!(decode_report_package_request(short_form).unwrap_err(), "invalid length at the request");
        // A valid request decodes as the whole decode does.
        let source = request_with_component_provenance_projection();
        let whole: ReportPackageRequest = serde_json::from_value(source.clone()).unwrap();
        assert_eq!(format!("{:?}", decode_report_package_request(source).unwrap()), format!("{whole:?}"));
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
