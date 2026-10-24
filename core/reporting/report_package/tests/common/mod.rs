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
    }
}

pub fn invented_audit_manifest() -> audit::AuditManifest {
    audit::AuditManifest {
        manifest_id: "invented-manifest-001".to_string(),
        model_hash: Some(audit::hash_canonical_json(
            audit::PayloadKind::ModelJson,
            "invented-model",
            &audit::CanonicalJson::object([(
                "model_id",
                audit::CanonicalJson::String("invented-model".to_string()),
            )]),
        )),
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

pub fn export_checksum(id: &str) -> export::ChecksumRef {
    export::ChecksumRef {
        algorithm: "sha256".to_string(),
        canonicalization: "JCS".to_string(),
        payload_ref: export::Reference::new("payload", id),
        value: format!("{id}-invented-hash"),
    }
}

pub fn export_provenance() -> export::Provenance {
    export::Provenance {
        source_name: "invented report package fixture".to_string(),
        source_location: "core/reporting/report_package/tests/common/mod.rs".to_string(),
        source_license: "project invented".to_string(),
        contributor: "OpenPipeStress".to_string(),
        contributor_certification: "invented non-engineering example".to_string(),
        redistribution_status: export::RedistributionStatus::InventedNonEngineeringExample,
        review_status: "accepted".to_string(),
    }
}

pub fn invented_result_envelope(magnitude: f64) -> export::ResultEnvelope {
    export::ResultEnvelope {
        envelope_id: "invented-result-envelope-001".to_string(),
        schema_version: "0.1.0".to_string(),
        model_ref: export::Reference::new("model", "invented-model"),
        run_ref: export::Reference::new("analysis_run", "invented-run"),
        solver_name: "open_pipe_stress_core".to_string(),
        solver_version: "0.1.0".to_string(),
        solver_build_ref: "invented-build".to_string(),
        unit_system_ref: export::Reference::new("unit_system", "invented-si"),
        load_basis_refs: vec![export::Reference::new("load_case", "LC1")],
        result_sets: vec![export::ResultSet {
            set_id: "invented-set-1".to_string(),
            set_type: export::ResultSetType::Mechanics.as_str().to_string(),
            basis_ref: export::Reference::new("load_case", "LC1"),
            values: vec![export::QuantityResult {
                result_id: "invented-stress-1".to_string(),
                family: export::ResultFamily::Stress,
                object_ref: export::Reference::new("node", "N1"),
                basis_ref: export::Reference::new("load_case", "LC1"),
                station_ref: None,
                magnitude,
                unit: "Pa".to_string(),
                dimension: export::DimensionId::Stress,
                metadata: None,
                diagnostics: Vec::new(),
                trace_chain: Vec::new(),
                provenance: export_provenance(),
            }],
        }],
        diagnostics: Vec::new(),
        provenance: export_provenance(),
        reproducibility: export::ReproducibilityRefs {
            model_hash: export_checksum("model"),
            run_hashes: vec![export_checksum("run")],
            audit_manifest_ref: export::Reference::new("audit_manifest", "invented-manifest-001"),
            deterministic_ordering: true,
        },
        analysis_status: vec![
            export::AnalysisStatus::MechanicsSolved,
            export::AnalysisStatus::HumanReviewRequired,
        ],
        rule_pack_refs: vec![],
        professional_boundary: export::ProfessionalBoundary::project_default(),
    }
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
    pub result_envelopes: Vec<export::ResultEnvelope>,
    pub sections_records: Vec<Value>,
}

impl Fixture {
    pub fn new() -> Self {
        Self {
            report: fixture_report_input(),
            audit_manifest: invented_audit_manifest(),
            result_envelopes: vec![invented_result_envelope(12.5)],
            sections_records: vec![invented_sections_record()],
        }
    }

    pub fn input(&self) -> ReportPackageContainerInput<'_> {
        ReportPackageContainerInput {
            package_id: "invented-package-001".to_string(),
            export_profile_id: "invented-profile-native-full-report".to_string(),
            source_model_ref: PackageRef::new("model", "invented-model"),
            source_basis_refs: vec![PackageRef::new("analysis_run", "invented-run")],
            report: &self.report,
            audit_manifest: &self.audit_manifest,
            result_envelopes: &self.result_envelopes,
            state_comparison_handoff_records: &self.sections_records,
        }
    }
}
