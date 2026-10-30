//! Shared invented fixtures for the report-package container tests. All data
//! invented; no protected standards content.

#![allow(dead_code)]

use open_pipe_stress_audit_manifest as audit;
use open_pipe_stress_report_package::{PackageRef, ReportPackageContainerInput};
use open_pipe_stress_report_renderer::{RenderableReportInput, ResultRow};
use open_pipe_stress_report_sections as sections;
use open_pipe_stress_result_export as export;
use serde_json::{json, Value};

pub const FIXTURE: &str =
    include_str!("../../../../../fixtures/reports/invented/calculation_report_fixture.json");

// ---------------------------------------------------------------------------
// Invented fixtures.
// ---------------------------------------------------------------------------

pub fn invented_section_provenance() -> sections::Provenance {
    sections::Provenance {
        source_name: "Invented OpenPipeStress report package test".to_string(),
        source_location: "core/reporting/report_package/tests/common/mod.rs".to_string(),
        source_license: "project_fixture".to_string(),
        contributor: "OpenPipeStress".to_string(),
        contributor_certification: "Invented non-engineering data only.".to_string(),
        redistribution_status: sections::RedistributionStatus::InventedNonEngineeringExample,
        review_status: sections::ReviewStatus::Accepted,
        privacy_classification: sections::PrivacyClassification::InventedPublicExample,
    }
}

pub fn invented_report_sections() -> sections::ReportSections {
    sections::ReportSections {
        report_section_id: "invented-report-sections-001".to_string(),
        model_ref: sections::Reference::new("model", "invented-model"),
        run_ref: sections::Reference::new("analysis_run", "invented-run"),
        diagnostics: vec![],
        analysis_status_disclosures: vec![
            sections::AnalysisStatusDisclosure {
                status: sections::AnalysisStatus::MechanicsSolved,
                source: sections::Reference::new("solver", "invented-solver"),
                affected_object: sections::Reference::new("model", "invented-model"),
                explanation: "Invented linear-static preview mechanics solved.".to_string(),
                human_review_required: true,
                human_acceptance_ref: None,
            },
            sections::AnalysisStatusDisclosure {
                status: sections::AnalysisStatus::HumanReviewRequired,
                source: sections::Reference::new("report_package", "invented-package"),
                affected_object: sections::Reference::new("report", "invented-report-001"),
                explanation: "Human professional review is required before any reliance."
                    .to_string(),
                human_review_required: true,
                human_acceptance_ref: None,
            },
        ],
        provenance_notes: vec![invented_section_provenance()],
        user_supplied_values: vec![],
        assumptions: vec![],
        limitations: vec![sections::Limitation {
            limitation_id: "invented-limitation-001".to_string(),
            source: sections::Reference::new("report_package", "invented-package"),
            affected_scope: sections::Reference::new("model", "invented-model"),
            statement: "Invented preview data only; not engineering output.".to_string(),
            effect: sections::ReportEffect {
                mechanics_solve_qualified: true,
                user_rule_check_qualified: false,
                report_completeness: sections::ReportCompleteness::Qualified,
                human_review_required: true,
            },
            provenance: invented_section_provenance(),
        }],
        unresolved_tbds: vec![],
        professional_boundary: sections::ProfessionalBoundary::project_default(),
    }
}

pub fn fixture_report_input() -> RenderableReportInput {
    let fixture: Value = serde_json::from_str(FIXTURE).expect("fixture parses");
    let calculation_report = serde_json::from_value(fixture["calculation_report"].clone())
        .expect("calculation_report deserializes via the serde feature");
    RenderableReportInput {
        report_title: "Invented Calculation Report".to_string(),
        calculation_report,
        report_sections: invented_report_sections(),
        result_rows: vec![ResultRow {
            row_id: "invented-row-001".to_string(),
            label: "Max bending stress (invented)".to_string(),
            case_ref: "load_case:invented-weight".to_string(),
            quantity_display: "12.5 MPa".to_string(),
            source_ref: "result_envelope:invented-result".to_string(),
        }],
        report_context: None,
        user_rule_checks: None,
        exact_route: None,
        library_references: None,
    }
}

/// An invented audit manifest whose id and model-hash value are the basis
/// document's, as the writer and the reader require of an unblocked package.
pub fn invented_audit_manifest() -> audit::AuditManifest {
    let document = invented_result_document();
    let reproducibility = &document["result_envelope"]["reproducibility"];
    audit::AuditManifest {
        manifest_id: reproducibility["audit_manifest_ref"]["ref_id"].as_str().unwrap().to_string(),
        model_hash: Some(audit::HashRecord {
            algorithm: audit::HashAlgorithm::Sha256,
            canonicalization: audit::Canonicalization::Rfc8785Jcs,
            payload_kind: audit::PayloadKind::ModelJson,
            payload_ref: "invented-model".to_string(),
            value: reproducibility["model_hash"]["value"].as_str().unwrap().to_string(),
        }),
        input_manifest_hash: Some(audit::hash_canonical_json(
            audit::PayloadKind::InputManifestJson,
            "invented-input-manifest",
            &audit::CanonicalJson::object([(
                "input_manifest_id",
                audit::CanonicalJson::String("invented-input-manifest".to_string()),
            )]),
        )),
        solver_version: audit::SolverVersionStamp {
            solver_name: "open_pipe_stress_core".to_string(),
            solver_version: "0.1.0".to_string(),
            solver_build_ref: "invented-build".to_string(),
        },
        unit_system_ref: "invented-si".to_string(),
        rule_pack_refs: vec![],
        assets: vec![],
        professional_boundary: audit::ProfessionalBoundary::project_default(),
    }
}

/// The committed witness success input (`validation/witness/inputs/
/// del1005_export_results_success_input.json`, generated by
/// `core/runner/headless/tests/del1005_package_witness.rs`).
pub const BASIS: &str =
    include_str!("../../../../../validation/witness/inputs/del1005_export_results_success_input.json");

pub fn basis() -> Value {
    serde_json::from_str(BASIS).expect("the witness basis parses")
}

/// The basis's DEL-08-04 0.3.0 result document, with its row arrays trimmed
/// to their first entries and its derivative hash recomputed. It keeps the
/// complete 0.3.0 form, the transported header and a valid self-hash, which
/// is all the lib-level assembler and the reader check (row accounting needs
/// the source and is the producer's check). The trimming keeps the exhaustive
/// byte-mutation reader tests proportionate.
pub fn invented_result_document() -> Value {
    let mut document = basis()["export_results"]["result_documents"][0].clone();
    let envelope = &mut document["result_envelope"];
    for (pointer, keep) in [
        ("/result_sets/0/values", 2),
        ("/unit_preservation_witnesses", 2),
        ("/row_accounting", 3),
        ("/source_annotations", 3),
        ("/diagnostics", 2),
    ] {
        if let Some(items) = envelope.pointer_mut(pointer).and_then(Value::as_array_mut) {
            items.truncate(keep);
        }
    }
    let document = rehash(document);
    open_pipe_stress_report_package::document_form::check_result_member_form(&document)
        .expect("the trimmed invented document keeps the 0.3.0 form");
    document
}

/// Recomputes a document's derivative hash after an edit, so only the checks
/// the edit targets can refuse it.
pub fn rehash(mut document: Value) -> Value {
    let envelope_id = document["result_envelope"]["envelope_id"].as_str().unwrap().to_string();
    document["result_envelope"]["reproducibility"].as_object_mut().unwrap().remove("derivative_hash");
    let hash = export::derivative::checksum(
        &document,
        "derivative_document_excludes_own_hash",
        export::derivative::reference("derivative_document", &envelope_id),
    )
    .unwrap();
    document["result_envelope"]["reproducibility"]["derivative_hash"] = hash;
    document
}

/// A self-hash-consistent variant of the basis document: another envelope id
/// and/or another first quantity magnitude (the lib-level assembler checks no
/// source accounting).
pub fn invented_result_document_variant(envelope_id: Option<&str>, magnitude: Option<f64>) -> Value {
    let mut document = invented_result_document();
    if let Some(envelope_id) = envelope_id {
        // Relabel the envelope coherently: every checksum subject that names
        // the envelope (the derivative and the attested carrier) follows it.
        let old = document["result_envelope"]["envelope_id"].clone();
        fn relabel(value: &mut Value, old: &Value, new: &str) {
            match value {
                Value::Object(members) => {
                    if let Some(subject) = members.get_mut("payload_ref") {
                        let names_envelope = matches!(
                            subject["ref_type"].as_str(),
                            Some("derivative_document" | "attested_headless_producer")
                        );
                        if names_envelope && subject["ref_id"] == *old {
                            subject["ref_id"] = json!(new);
                        }
                    }
                    members.values_mut().for_each(|member| relabel(member, old, new));
                }
                Value::Array(items) => items.iter_mut().for_each(|item| relabel(item, old, new)),
                _ => {}
            }
        }
        relabel(&mut document, &old, envelope_id);
        document["result_envelope"]["envelope_id"] = json!(envelope_id);
    }
    if let Some(magnitude) = magnitude {
        document["result_envelope"]["result_sets"][0]["values"][0]["magnitude"] = json!(magnitude);
    }
    rehash(document)
}

pub fn invented_sections_record() -> Value {
    json!({
        "schema_version": "0.1.0",
        "deliverable_id": "DEL-08-06",
        "package_id": "PKG-08",
        "scope_item": "SOW-024",
        "section_set_id": "invented-section-set-001",
        "section_contract_status": "backend_report_section_records_only",
        "sections": {
            "state_run_sections": [],
            "comparison_sections": [],
            "handoff_sections": []
        },
        "diagnostics": [],
        "professional_boundary": {
            "human_review_required": true,
            "software_makes_compliance_claim": false,
            "software_makes_certification_claim": false,
            "software_makes_sealing_claim": false,
            "software_makes_approval_claim": false,
            "software_makes_authentication_claim": false,
            "software_creates_professional_reliance_record": false,
            "software_creates_external_validation_record": false
        }
    })
}

pub struct Fixture {
    pub report: RenderableReportInput,
    pub audit_manifest: audit::AuditManifest,
    pub result_documents: Vec<Value>,
    pub sections_records: Vec<Value>,
}

impl Fixture {
    pub fn new() -> Self {
        Self {
            report: fixture_report_input(),
            audit_manifest: invented_audit_manifest(),
            result_documents: vec![invented_result_document()],
            sections_records: vec![invented_sections_record()],
        }
    }

    pub fn input(&self) -> ReportPackageContainerInput<'_> {
        self.input_with_profile("invented-profile-native-full-report")
    }

    /// [`Fixture::input`] under another export profile.
    pub fn input_with_profile(&self, export_profile_id: &str) -> ReportPackageContainerInput<'_> {
        ReportPackageContainerInput {
            package_id: "invented-package-001".to_string(),
            export_profile_id: export_profile_id.to_string(),
            source_model_ref: PackageRef::new("model", "invented-model"),
            source_basis_refs: vec![PackageRef::new("analysis_run", "invented-run")],
            report: &self.report,
            audit_manifest: &self.audit_manifest,
            result_documents: &self.result_documents,
            state_comparison_handoff_records: &self.sections_records,
        }
    }
}
