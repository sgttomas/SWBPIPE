//! Calculation report assembly support.
//!
//! This crate validates bounded in-memory calculation report records, checks
//! template slots, and prepares deterministic section ordering for neutral test
//! output. It does not parse project files, call solver internals, render GUI
//! previews, run CLI/API/adapter workflows, implement protected-content
//! linting, implement redaction/export controls, access host resources, or
//! emit professional or code-compliance claims.

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AnalysisStatus {
    ModelIncomplete,
    MechanicsSolved,
    RuleInputsIncomplete,
    UserRuleChecked,
    UserRuleFailed,
    HumanReviewRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticClass {
    SolveBlocking,
    RuleCheckBlocking,
    ProvenanceWarning,
    AssumptionWarning,
    NonlinearWarning,
    IpBoundaryWarning,
    UnitWarning,
    ReportBlocking,
    TemplateBlocking,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Blocking,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SectionKind {
    ModelInputSummary,
    LoadCases,
    Results,
    WarningsAssumptionsProvenance,
    AuditManifest,
    RulePackReferences,
    Limitations,
    ProfessionalBoundaryNotice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivacyClassification {
    PublicMetadata,
    InventedPublicExample,
    PrivateProjectData,
    PrivateRulePackData,
    PrivateLibraryData,
    ProtectedSuspected,
    Redacted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedistributionStatus {
    PublicPermissive,
    PrivateOnly,
    Unknown,
    ProtectedSuspected,
    InventedNonEngineeringExample,
    Tbd,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reference {
    pub ref_type: String,
    pub ref_id: String,
}

impl Reference {
    pub fn new(ref_type: impl Into<String>, ref_id: impl Into<String>) -> Self {
        Self {
            ref_type: ref_type.into(),
            ref_id: ref_id.into(),
        }
    }

    fn is_complete(&self) -> bool {
        !self.ref_type.trim().is_empty() && !self.ref_id.trim().is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Provenance {
    pub source_name: String,
    pub source_location: String,
    pub source_license: String,
    pub contributor: String,
    pub contributor_certification: String,
    pub redistribution_status: RedistributionStatus,
    pub review_status: String,
    pub privacy_classification: PrivacyClassification,
}

impl Provenance {
    fn is_complete(&self) -> bool {
        !self.source_name.trim().is_empty()
            && !self.source_location.trim().is_empty()
            && !self.source_license.trim().is_empty()
            && !self.contributor.trim().is_empty()
            && !self.contributor_certification.trim().is_empty()
            && !self.review_status.trim().is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChecksumRef {
    pub algorithm: String,
    pub canonicalization: String,
    pub payload_ref: Reference,
    pub value: String,
}

impl ChecksumRef {
    fn is_complete(&self) -> bool {
        !self.algorithm.trim().is_empty()
            && !self.canonicalization.trim().is_empty()
            && self.payload_ref.is_complete()
            && !self.value.trim().is_empty()
            && !self.value.trim().eq_ignore_ascii_case("TBD")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferencedEnvelope {
    pub reference: Reference,
    pub schema_ref: Reference,
    pub checksum: ChecksumRef,
    pub privacy_classification: PrivacyClassification,
    pub provenance: Provenance,
}

impl ReferencedEnvelope {
    fn is_complete(&self) -> bool {
        self.reference.is_complete()
            && self.schema_ref.is_complete()
            && self.checksum.is_complete()
            && self.provenance.is_complete()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelInputSummary {
    pub project_ref: Reference,
    pub model_ref: Reference,
    pub persistence_ref: Reference,
    pub unit_system_ref: Reference,
    pub model_hash: ChecksumRef,
    pub input_manifest_ref: Reference,
    pub provenance: Provenance,
}

impl ModelInputSummary {
    fn is_complete(&self) -> bool {
        self.project_ref.is_complete()
            && self.model_ref.is_complete()
            && self.persistence_ref.is_complete()
            && self.unit_system_ref.is_complete()
            && self.model_hash.is_complete()
            && self.input_manifest_ref.is_complete()
            && self.provenance.is_complete()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadCaseSummary {
    pub load_ref: Reference,
    pub label: String,
    pub basis: String,
    pub source: Reference,
    pub provenance: Provenance,
}

impl LoadCaseSummary {
    fn is_complete(&self) -> bool {
        self.load_ref.is_complete()
            && !self.label.trim().is_empty()
            && !self.basis.trim().is_empty()
            && self.source.is_complete()
            && self.provenance.is_complete()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RulePackRef {
    pub rule_pack_id: String,
    pub version: String,
    pub checksum: ChecksumRef,
    pub source_notice: String,
    pub redistribution_status: RedistributionStatus,
    pub completeness_status: String,
    pub private_payload_redacted: bool,
}

impl RulePackRef {
    fn is_safe_reference(&self) -> bool {
        !self.rule_pack_id.trim().is_empty()
            && !self.version.trim().is_empty()
            && self.checksum.is_complete()
            && !self.source_notice.trim().is_empty()
            && !self.completeness_status.trim().is_empty()
            && self.private_payload_redacted
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProfessionalBoundary {
    pub human_review_required: bool,
    pub software_makes_compliance_claim: bool,
    pub software_makes_certification_claim: bool,
    pub software_makes_sealing_claim: bool,
    pub software_makes_approval_claim: bool,
    pub software_makes_authentication_claim: bool,
}

impl ProfessionalBoundary {
    pub fn project_default() -> Self {
        Self {
            human_review_required: true,
            software_makes_compliance_claim: false,
            software_makes_certification_claim: false,
            software_makes_sealing_claim: false,
            software_makes_approval_claim: false,
            software_makes_authentication_claim: false,
        }
    }

    fn preserves_boundary(&self) -> bool {
        self.human_review_required
            && !self.software_makes_compliance_claim
            && !self.software_makes_certification_claim
            && !self.software_makes_sealing_claim
            && !self.software_makes_approval_claim
            && !self.software_makes_authentication_claim
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub code: String,
    pub class: DiagnosticClass,
    pub severity: DiagnosticSeverity,
    pub source: Reference,
    pub affected_object: Reference,
    pub message: String,
    pub remediation: String,
    pub provenance: Provenance,
}

impl Diagnostic {
    fn report_blocking(
        code: impl Into<String>,
        affected_object: Reference,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            class: DiagnosticClass::ReportBlocking,
            severity: DiagnosticSeverity::Blocking,
            source: Reference::new("report_generator", "DEL-08-01"),
            affected_object,
            message: message.into(),
            remediation:
                "Provide complete report assembly metadata before relying on this report surface."
                    .to_string(),
            provenance: invented_provenance(),
        }
    }

    fn template_blocking(
        code: impl Into<String>,
        affected_object: Reference,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            class: DiagnosticClass::TemplateBlocking,
            severity: DiagnosticSeverity::Blocking,
            source: Reference::new("report_generator", "DEL-08-01"),
            affected_object,
            message: message.into(),
            remediation: "Provide deterministic template slots and matching sections.".to_string(),
            provenance: invented_provenance(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateSlot {
    pub slot_id: String,
    pub required: bool,
    pub section_kind: SectionKind,
    pub source_contract: Reference,
    pub ordering_index: usize,
}

impl TemplateSlot {
    fn is_complete(&self) -> bool {
        !self.slot_id.trim().is_empty() && self.source_contract.is_complete()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedSection {
    pub section_id: String,
    pub slot_id: String,
    pub section_kind: SectionKind,
    pub title: String,
    pub source_refs: Vec<Reference>,
    pub content_status: String,
}

impl RenderedSection {
    fn is_complete(&self) -> bool {
        !self.section_id.trim().is_empty()
            && !self.slot_id.trim().is_empty()
            && !self.title.trim().is_empty()
            && !self.content_status.trim().is_empty()
            && !self.source_refs.is_empty()
            && self.source_refs.iter().all(Reference::is_complete)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnresolvedTbd {
    pub tbd_id: String,
    pub topic: String,
    pub affected_scope: Reference,
    pub description: String,
    pub review_needed: bool,
}

impl UnresolvedTbd {
    fn requires_review(&self) -> bool {
        !self.tbd_id.trim().is_empty()
            && !self.topic.trim().is_empty()
            && self.affected_scope.is_complete()
            && !self.description.trim().is_empty()
            && self.review_needed
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalculationReport {
    pub report_id: String,
    pub model_input_summary: ModelInputSummary,
    pub load_case_summary: Vec<LoadCaseSummary>,
    pub result_export_refs: Vec<ReferencedEnvelope>,
    pub audit_manifest_refs: Vec<ReferencedEnvelope>,
    pub report_section_refs: Vec<ReferencedEnvelope>,
    pub rule_pack_refs: Vec<RulePackRef>,
    pub diagnostics: Vec<Diagnostic>,
    pub template_slots: Vec<TemplateSlot>,
    pub rendered_sections: Vec<RenderedSection>,
    pub analysis_status: Vec<AnalysisStatus>,
    pub professional_boundary: ProfessionalBoundary,
    pub provenance: Provenance,
    pub privacy_classification: PrivacyClassification,
    pub unresolved_runtime_tbds: Vec<UnresolvedTbd>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportValidation {
    pub diagnostics: Vec<Diagnostic>,
}

impl ReportValidation {
    pub fn is_blocked(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == DiagnosticSeverity::Blocking)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NeutralReportSection {
    pub ordering_index: usize,
    pub section_id: String,
    pub title: String,
    pub source_count: usize,
    pub content_status: String,
}

pub fn validate_report(report: &CalculationReport) -> ReportValidation {
    let mut diagnostics = Vec::new();

    if report.report_id.trim().is_empty() {
        diagnostics.push(Diagnostic::report_blocking(
            "REPORT-MISSING-ID",
            Reference::new("report", "UNKNOWN"),
            "calculation report must identify report_id",
        ));
    }
    if !report.model_input_summary.is_complete() {
        diagnostics.push(Diagnostic::report_blocking(
            "REPORT-MODEL-SUMMARY-INCOMPLETE",
            Reference::new("report", &report.report_id),
            "calculation report requires complete persisted model, unit, manifest, and hash references",
        ));
    }
    if report.load_case_summary.is_empty()
        || !report
            .load_case_summary
            .iter()
            .all(LoadCaseSummary::is_complete)
    {
        diagnostics.push(Diagnostic::report_blocking(
            "REPORT-LOADS-INCOMPLETE",
            Reference::new("report", &report.report_id),
            "calculation report requires explicit load case or combination summaries",
        ));
    }
    check_referenced_envelopes(
        "REPORT-MISSING-RESULT-EXPORT",
        "result export envelope references are required",
        &report.report_id,
        &report.result_export_refs,
        &mut diagnostics,
    );
    check_referenced_envelopes(
        "REPORT-MISSING-AUDIT-MANIFEST",
        "audit manifest references are required",
        &report.report_id,
        &report.audit_manifest_refs,
        &mut diagnostics,
    );
    check_referenced_envelopes(
        "REPORT-MISSING-SECTIONS",
        "report section envelope references are required",
        &report.report_id,
        &report.report_section_refs,
        &mut diagnostics,
    );
    if !report
        .rule_pack_refs
        .iter()
        .all(RulePackRef::is_safe_reference)
    {
        diagnostics.push(Diagnostic::report_blocking(
            "REPORT-RULE-PACK-REF-UNSAFE",
            Reference::new("report", &report.report_id),
            "rule-pack references must include identity, version, checksum, source notice, status, and redacted private payload posture",
        ));
    }
    if !report
        .analysis_status
        .contains(&AnalysisStatus::HumanReviewRequired)
    {
        diagnostics.push(Diagnostic::report_blocking(
            "REPORT-HUMAN-REVIEW-MISSING",
            Reference::new("report", &report.report_id),
            "calculation report must preserve human-review-required status",
        ));
    }
    if !report.professional_boundary.preserves_boundary() {
        diagnostics.push(Diagnostic::report_blocking(
            "REPORT-PROFESSIONAL-BOUNDARY",
            Reference::new("report", &report.report_id),
            "calculation report must not make compliance, certification, sealing, approval, or authentication claims",
        ));
    }
    if !report.provenance.is_complete() {
        diagnostics.push(Diagnostic::report_blocking(
            "REPORT-PROVENANCE-INCOMPLETE",
            Reference::new("report", &report.report_id),
            "calculation report requires source, license, contributor, redistribution, review, and privacy metadata",
        ));
    }
    if report
        .unresolved_runtime_tbds
        .iter()
        .any(|tbd| !tbd.requires_review())
    {
        diagnostics.push(Diagnostic::report_blocking(
            "REPORT-TBD-REVIEW-MISSING",
            Reference::new("report", &report.report_id),
            "unresolved runtime/reporting TBDs must remain explicit and review-needed",
        ));
    }

    validate_template_slots(report, &mut diagnostics);

    ReportValidation { diagnostics }
}

pub fn ordered_sections(report: &CalculationReport) -> Vec<NeutralReportSection> {
    let mut sections: Vec<_> = report
        .rendered_sections
        .iter()
        .filter_map(|section| {
            let ordering_index = report
                .template_slots
                .iter()
                .find(|slot| slot.slot_id == section.slot_id)
                .map(|slot| slot.ordering_index)?;
            Some(NeutralReportSection {
                ordering_index,
                section_id: section.section_id.clone(),
                title: section.title.clone(),
                source_count: section.source_refs.len(),
                content_status: section.content_status.clone(),
            })
        })
        .collect();
    sections.sort_by(|left, right| {
        left.ordering_index
            .cmp(&right.ordering_index)
            .then_with(|| left.section_id.cmp(&right.section_id))
    });
    sections
}

fn check_referenced_envelopes(
    code: &str,
    message: &str,
    report_id: &str,
    refs: &[ReferencedEnvelope],
    diagnostics: &mut Vec<Diagnostic>,
) {
    if refs.is_empty() || !refs.iter().all(ReferencedEnvelope::is_complete) {
        diagnostics.push(Diagnostic::report_blocking(
            code,
            Reference::new("report", report_id),
            message,
        ));
    }
}

fn validate_template_slots(report: &CalculationReport, diagnostics: &mut Vec<Diagnostic>) {
    let mut slot_ids = HashSet::new();
    let mut required_kinds = HashSet::new();
    for slot in &report.template_slots {
        if !slot.is_complete() {
            diagnostics.push(Diagnostic::template_blocking(
                "REPORT-TEMPLATE-SLOT-INCOMPLETE",
                Reference::new("report", &report.report_id),
                "template slots require id, source contract, section kind, and deterministic order",
            ));
        }
        if !slot_ids.insert(slot.slot_id.as_str()) {
            diagnostics.push(Diagnostic::template_blocking(
                "REPORT-TEMPLATE-SLOT-DUPLICATE",
                Reference::new("template_slot", &slot.slot_id),
                "template slot ids must be unique",
            ));
        }
        if slot.required {
            required_kinds.insert(slot.section_kind);
        }
    }

    let section_slot_ids: HashSet<_> = report
        .rendered_sections
        .iter()
        .map(|section| section.slot_id.as_str())
        .collect();
    let section_kinds: HashSet<_> = report
        .rendered_sections
        .iter()
        .map(|section| section.section_kind)
        .collect();

    for slot in report.template_slots.iter().filter(|slot| slot.required) {
        if !section_slot_ids.contains(slot.slot_id.as_str()) {
            diagnostics.push(Diagnostic::template_blocking(
                "REPORT-TEMPLATE-SLOT-UNFILLED",
                Reference::new("template_slot", &slot.slot_id),
                "required template slot has no rendered section",
            ));
        }
    }
    for required_kind in required_kinds {
        if !section_kinds.contains(&required_kind) {
            diagnostics.push(Diagnostic::template_blocking(
                "REPORT-TEMPLATE-KIND-MISSING",
                Reference::new("report", &report.report_id),
                "required section kind has no rendered section",
            ));
        }
    }
    for section in &report.rendered_sections {
        if !section.is_complete() {
            diagnostics.push(Diagnostic::template_blocking(
                "REPORT-SECTION-INCOMPLETE",
                Reference::new("section", &section.section_id),
                "rendered sections require section id, slot id, title, source refs, and content status",
            ));
        }
        if !slot_ids.contains(section.slot_id.as_str()) {
            diagnostics.push(Diagnostic::template_blocking(
                "REPORT-SECTION-UNKNOWN-SLOT",
                Reference::new("section", &section.section_id),
                "rendered section references an unknown template slot",
            ));
        }
    }
}

fn invented_provenance() -> Provenance {
    Provenance {
        source_name: "OpenPipeStress report generator validator".to_string(),
        source_location: "core/reporting/report_generator".to_string(),
        source_license: "project_internal".to_string(),
        contributor: "OpenPipeStress".to_string(),
        contributor_certification: "No protected standards data or private payloads.".to_string(),
        redistribution_status: RedistributionStatus::InventedNonEngineeringExample,
        review_status: "accepted".to_string(),
        privacy_classification: PrivacyClassification::PublicMetadata,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reference(ref_type: &str, ref_id: &str) -> Reference {
        Reference::new(ref_type, ref_id)
    }

    fn provenance() -> Provenance {
        Provenance {
            source_name: "invented report fixture".to_string(),
            source_location: "tests/report_generator/invented".to_string(),
            source_license: "project_fixture".to_string(),
            contributor: "OpenPipeStress".to_string(),
            contributor_certification: "Invented non-engineering data only.".to_string(),
            redistribution_status: RedistributionStatus::InventedNonEngineeringExample,
            review_status: "accepted".to_string(),
            privacy_classification: PrivacyClassification::InventedPublicExample,
        }
    }

    fn checksum(ref_type: &str, ref_id: &str) -> ChecksumRef {
        ChecksumRef {
            algorithm: "sha256".to_string(),
            canonicalization: "jcs_compatible".to_string(),
            payload_ref: reference(ref_type, ref_id),
            value: format!("invented-{ref_id}-hash"),
        }
    }

    fn envelope(ref_type: &str, ref_id: &str, schema_id: &str) -> ReferencedEnvelope {
        ReferencedEnvelope {
            reference: reference(ref_type, ref_id),
            schema_ref: reference("schema", schema_id),
            checksum: checksum(ref_type, ref_id),
            privacy_classification: PrivacyClassification::InventedPublicExample,
            provenance: provenance(),
        }
    }

    fn slot(slot_id: &str, kind: SectionKind, ordering_index: usize) -> TemplateSlot {
        TemplateSlot {
            slot_id: slot_id.to_string(),
            required: true,
            section_kind: kind,
            source_contract: reference("schema", "invented"),
            ordering_index,
        }
    }

    fn section(section_id: &str, slot_id: &str, kind: SectionKind) -> RenderedSection {
        RenderedSection {
            section_id: section_id.to_string(),
            slot_id: slot_id.to_string(),
            section_kind: kind,
            title: section_id.to_string(),
            source_refs: vec![reference("source", section_id)],
            content_status: "qualified".to_string(),
        }
    }

    fn report() -> CalculationReport {
        let kinds = [
            SectionKind::ModelInputSummary,
            SectionKind::LoadCases,
            SectionKind::Results,
            SectionKind::WarningsAssumptionsProvenance,
            SectionKind::AuditManifest,
            SectionKind::RulePackReferences,
            SectionKind::Limitations,
            SectionKind::ProfessionalBoundaryNotice,
        ];
        CalculationReport {
            report_id: "report-1".to_string(),
            model_input_summary: ModelInputSummary {
                project_ref: reference("project", "project-1"),
                model_ref: reference("model", "model-1"),
                persistence_ref: reference("persistence", "persistence-1"),
                unit_system_ref: reference("unit_system", "si"),
                model_hash: checksum("model", "model-1"),
                input_manifest_ref: reference("audit_manifest", "manifest-1"),
                provenance: provenance(),
            },
            load_case_summary: vec![LoadCaseSummary {
                load_ref: reference("load_case", "weight"),
                label: "Invented weight".to_string(),
                basis: "load_case".to_string(),
                source: reference("fixture", "invented"),
                provenance: provenance(),
            }],
            result_export_refs: vec![envelope(
                "result_export",
                "result-export-1",
                "schemas/results.schema.yaml",
            )],
            audit_manifest_refs: vec![envelope(
                "audit_manifest",
                "manifest-1",
                "core/reporting/audit_manifest",
            )],
            report_section_refs: vec![envelope(
                "report_sections",
                "report-sections-1",
                "schemas/report_sections.schema.yaml",
            )],
            rule_pack_refs: vec![RulePackRef {
                rule_pack_id: "invented-rule-pack".to_string(),
                version: "0.1.0".to_string(),
                checksum: checksum("rule_pack", "invented-rule-pack"),
                source_notice: "Invented non-code example only.".to_string(),
                redistribution_status: RedistributionStatus::InventedNonEngineeringExample,
                completeness_status: "not_checked".to_string(),
                private_payload_redacted: true,
            }],
            diagnostics: Vec::new(),
            template_slots: kinds
                .iter()
                .enumerate()
                .map(|(index, kind)| slot(&format!("slot-{index}"), *kind, index))
                .collect(),
            rendered_sections: kinds
                .iter()
                .rev()
                .enumerate()
                .map(|(index, kind)| {
                    section(
                        &format!("section-{index}"),
                        &format!(
                            "slot-{}",
                            kinds.iter().position(|item| item == kind).unwrap()
                        ),
                        *kind,
                    )
                })
                .collect(),
            analysis_status: vec![
                AnalysisStatus::MechanicsSolved,
                AnalysisStatus::HumanReviewRequired,
            ],
            professional_boundary: ProfessionalBoundary::project_default(),
            provenance: provenance(),
            privacy_classification: PrivacyClassification::InventedPublicExample,
            unresolved_runtime_tbds: vec![UnresolvedTbd {
                tbd_id: "TBD-REPORT-REDACTION".to_string(),
                topic: "redaction_export_controls".to_string(),
                affected_scope: reference("deliverable", "DEL-12-02"),
                description: "Redaction controls remain downstream.".to_string(),
                review_needed: true,
            }],
        }
    }

    #[test]
    fn validates_complete_report_without_blocking_diagnostics() {
        let validation = validate_report(&report());
        assert!(!validation.is_blocked());
    }

    #[test]
    fn orders_sections_by_template_slot_index() {
        let ordered = ordered_sections(&report());
        let indices: Vec<_> = ordered
            .iter()
            .map(|section| section.ordering_index)
            .collect();
        assert_eq!(indices, (0..8).collect::<Vec<_>>());
    }

    #[test]
    fn blocks_when_human_review_status_is_missing() {
        let mut report = report();
        report.analysis_status = vec![AnalysisStatus::MechanicsSolved];
        let validation = validate_report(&report);
        assert!(validation
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "REPORT-HUMAN-REVIEW-MISSING"));
    }

    #[test]
    fn blocks_professional_boundary_violations() {
        let mut report = report();
        report.professional_boundary.software_makes_compliance_claim = true;
        let validation = validate_report(&report);
        assert!(validation
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "REPORT-PROFESSIONAL-BOUNDARY"));
    }

    #[test]
    fn blocks_missing_template_sections() {
        let mut report = report();
        report.rendered_sections.clear();
        let validation = validate_report(&report);
        assert!(validation
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "REPORT-TEMPLATE-SLOT-UNFILLED"));
    }
}
