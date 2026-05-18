//! Report section disclosure support.
//!
//! This crate validates bounded in-memory report section records for warnings,
//! assumptions, user-supplied values, provenance notes, limitations, unresolved
//! TBDs, and human-review-needed findings. It does not render final reports,
//! read project files, call solver internals, run GUI/CLI/API/adapter workflows,
//! access host resources, or emit professional or code-compliance claims.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnalysisStatus {
    ModelIncomplete,
    MechanicsSolved,
    RuleInputsIncomplete,
    UserRuleChecked,
    UserRuleFailed,
    HumanReviewRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticClass {
    SolveBlocking,
    RuleCheckBlocking,
    ProvenanceWarning,
    AssumptionWarning,
    NonlinearWarning,
    IpBoundaryWarning,
    UnitWarning,
    ReportBlocking,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Info,
    Warning,
    Blocking,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivacyClassification {
    PublicMetadata,
    InventedPublicExample,
    PrivateProjectData,
    PrivateRulePackData,
    ProtectedSuspected,
    Redacted,
    Tbd,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReviewStatus {
    Pending,
    Accepted,
    Rejected,
    Quarantined,
    Tbd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DimensionId {
    Dimensionless,
    Length,
    Angle,
    Force,
    Moment,
    Stress,
    Ratio,
    Time,
    Temperature,
    Pressure,
    Tbd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequiredFor {
    MechanicsSolve,
    UserRuleCheck,
    Reporting,
    HumanReview,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportCompleteness {
    Complete,
    Qualified,
    Incomplete,
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
    pub review_status: ReviewStatus,
    pub privacy_classification: PrivacyClassification,
}

impl Provenance {
    fn is_complete(&self) -> bool {
        !self.source_name.trim().is_empty()
            && !self.source_location.trim().is_empty()
            && !self.source_license.trim().is_empty()
            && !self.contributor.trim().is_empty()
            && !self.contributor_certification.trim().is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub code: String,
    pub class: DiagnosticClass,
    pub severity: Severity,
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
            severity: Severity::Blocking,
            source: Reference::new("report_sections", "DEL-08-03"),
            affected_object,
            message: message.into(),
            remediation:
                "Provide explicit report-section metadata before relying on this report surface."
                    .to_string(),
            provenance: invented_provenance(),
        }
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
pub struct AnalysisStatusDisclosure {
    pub status: AnalysisStatus,
    pub source: Reference,
    pub affected_object: Reference,
    pub explanation: String,
    pub human_review_required: bool,
    pub human_acceptance_ref: Option<Reference>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Quantity {
    pub magnitude: f64,
    pub unit: String,
    pub dimension: DimensionId,
}

impl Quantity {
    fn is_complete(&self) -> bool {
        self.magnitude.is_finite()
            && !self.unit.trim().is_empty()
            && self.dimension != DimensionId::Tbd
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserSuppliedValue {
    pub value_id: String,
    pub value_category: String,
    pub source: Reference,
    pub quantity: Option<Quantity>,
    pub provenance: Provenance,
    pub privacy_classification: PrivacyClassification,
    pub required_for: Vec<RequiredFor>,
    pub review_status: ReviewStatus,
    pub missing_data_finding: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReportEffect {
    pub mechanics_solve_qualified: bool,
    pub user_rule_check_qualified: bool,
    pub report_completeness: ReportCompleteness,
    pub human_review_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assumption {
    pub assumption_id: String,
    pub owner: String,
    pub source: Reference,
    pub affected_scope: Reference,
    pub statement: String,
    pub review_status: ReviewStatus,
    pub expiration_or_re_review_trigger: Option<String>,
    pub effect: ReportEffect,
    pub provenance: Provenance,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Limitation {
    pub limitation_id: String,
    pub source: Reference,
    pub affected_scope: Reference,
    pub statement: String,
    pub effect: ReportEffect,
    pub provenance: Provenance,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnresolvedTbd {
    pub tbd_id: String,
    pub affected_scope: Reference,
    pub description: String,
    pub review_needed: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReportSections {
    pub report_section_id: String,
    pub model_ref: Reference,
    pub run_ref: Reference,
    pub diagnostics: Vec<Diagnostic>,
    pub analysis_status_disclosures: Vec<AnalysisStatusDisclosure>,
    pub provenance_notes: Vec<Provenance>,
    pub user_supplied_values: Vec<UserSuppliedValue>,
    pub assumptions: Vec<Assumption>,
    pub limitations: Vec<Limitation>,
    pub unresolved_tbds: Vec<UnresolvedTbd>,
    pub professional_boundary: ProfessionalBoundary,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportSectionValidation {
    pub diagnostics: Vec<Diagnostic>,
}

impl ReportSectionValidation {
    pub fn has_blocking_diagnostics(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == Severity::Blocking)
    }
}

pub fn validate_report_sections(sections: &ReportSections) -> ReportSectionValidation {
    let mut diagnostics = Vec::new();

    if sections.report_section_id.trim().is_empty()
        || !sections.model_ref.is_complete()
        || !sections.run_ref.is_complete()
    {
        diagnostics.push(Diagnostic::report_blocking(
            "REPORT_SECTION_IDENTITY_MISSING",
            Reference::new("report_sections", "UNKNOWN"),
            "report sections must identify section, model, and run references",
        ));
    }

    if !sections.professional_boundary.preserves_boundary() {
        diagnostics.push(Diagnostic::report_blocking(
            "REPORT_SECTION_PROFESSIONAL_BOUNDARY_VIOLATION",
            Reference::new("report_sections", &sections.report_section_id),
            "report sections must not make compliance, certification, sealing, approval, or authentication claims",
        ));
    }

    if !sections
        .analysis_status_disclosures
        .iter()
        .any(|status| status.status == AnalysisStatus::HumanReviewRequired)
    {
        diagnostics.push(Diagnostic::report_blocking(
            "REPORT_SECTION_HUMAN_REVIEW_STATUS_MISSING",
            Reference::new("report_sections", &sections.report_section_id),
            "report sections must preserve human-review-required status",
        ));
    }

    check_status_disclosures(sections, &mut diagnostics);
    check_provenance(sections, &mut diagnostics);
    check_user_values(sections, &mut diagnostics);
    check_assumptions(sections, &mut diagnostics);
    check_limitations(sections, &mut diagnostics);
    check_tbds(sections, &mut diagnostics);

    ReportSectionValidation { diagnostics }
}

pub fn sorted_diagnostics(sections: &ReportSections) -> Vec<&Diagnostic> {
    let mut diagnostics: Vec<_> = sections.diagnostics.iter().collect();
    diagnostics.sort_by_key(|diagnostic| {
        (
            diagnostic.severity != Severity::Blocking,
            diagnostic.class,
            diagnostic.affected_object.ref_id.as_str(),
            diagnostic.code.as_str(),
        )
    });
    diagnostics
}

fn check_status_disclosures(sections: &ReportSections, diagnostics: &mut Vec<Diagnostic>) {
    for status in &sections.analysis_status_disclosures {
        if !status.source.is_complete()
            || !status.affected_object.is_complete()
            || status.explanation.trim().is_empty()
            || !status.human_review_required
        {
            diagnostics.push(Diagnostic::report_blocking(
                "REPORT_SECTION_STATUS_DISCLOSURE_INCOMPLETE",
                Reference::new("analysis_status", format!("{:?}", status.status)),
                "analysis status disclosures must include source, affected object, explanation, and human review posture",
            ));
        }
    }
}

fn check_provenance(sections: &ReportSections, diagnostics: &mut Vec<Diagnostic>) {
    for provenance in &sections.provenance_notes {
        if !provenance.is_complete() {
            diagnostics.push(Diagnostic::report_blocking(
                "REPORT_SECTION_PROVENANCE_INCOMPLETE",
                Reference::new("provenance", &provenance.source_name),
                "report provenance notes must include source, license, contributor, certification, redistribution, review, and privacy metadata",
            ));
        }

        if provenance.redistribution_status == RedistributionStatus::ProtectedSuspected
            || provenance.privacy_classification == PrivacyClassification::ProtectedSuspected
        {
            diagnostics.push(Diagnostic::report_blocking(
                "REPORT_SECTION_PROTECTED_CONTENT_SUSPECTED",
                Reference::new("provenance", &provenance.source_name),
                "suspected protected content must be reported as a finding and cannot be copied into public report artifacts",
            ));
        }
    }
}

fn check_user_values(sections: &ReportSections, diagnostics: &mut Vec<Diagnostic>) {
    for value in &sections.user_supplied_values {
        if value.value_id.trim().is_empty()
            || value.value_category.trim().is_empty()
            || !value.source.is_complete()
            || !value.provenance.is_complete()
            || value.required_for.is_empty()
        {
            diagnostics.push(Diagnostic::report_blocking(
                "REPORT_SECTION_USER_VALUE_METADATA_INCOMPLETE",
                Reference::new("user_supplied_value", &value.value_id),
                "user-supplied values must include identity, category, source, provenance, and required-use metadata",
            ));
        }

        if value.required_for.iter().any(|required| {
            matches!(
                required,
                RequiredFor::MechanicsSolve | RequiredFor::UserRuleCheck
            )
        }) && value
            .quantity
            .as_ref()
            .map_or(true, |quantity| !quantity.is_complete())
        {
            diagnostics.push(Diagnostic::report_blocking(
                "REPORT_SECTION_USER_VALUE_QUANTITY_INCOMPLETE",
                Reference::new("user_supplied_value", &value.value_id),
                "numeric solve or rule-check values must carry finite magnitude, unit, and dimension metadata",
            ));
        }

        if value.missing_data_finding && value.review_status == ReviewStatus::Accepted {
            diagnostics.push(Diagnostic::report_blocking(
                "REPORT_SECTION_MISSING_VALUE_MARKED_ACCEPTED",
                Reference::new("user_supplied_value", &value.value_id),
                "missing required data must remain an explicit finding rather than accepted input",
            ));
        }
    }
}

fn check_assumptions(sections: &ReportSections, diagnostics: &mut Vec<Diagnostic>) {
    for assumption in &sections.assumptions {
        if assumption.assumption_id.trim().is_empty()
            || assumption.owner.trim().is_empty()
            || !assumption.source.is_complete()
            || !assumption.affected_scope.is_complete()
            || assumption.statement.trim().is_empty()
            || !assumption.provenance.is_complete()
        {
            diagnostics.push(Diagnostic::report_blocking(
                "REPORT_SECTION_ASSUMPTION_INCOMPLETE",
                Reference::new("assumption", &assumption.assumption_id),
                "assumptions must include owner, source, affected scope, statement, review status, effect, and provenance",
            ));
        }
    }
}

fn check_limitations(sections: &ReportSections, diagnostics: &mut Vec<Diagnostic>) {
    for limitation in &sections.limitations {
        if limitation.limitation_id.trim().is_empty()
            || !limitation.source.is_complete()
            || !limitation.affected_scope.is_complete()
            || limitation.statement.trim().is_empty()
            || !limitation.provenance.is_complete()
        {
            diagnostics.push(Diagnostic::report_blocking(
                "REPORT_SECTION_LIMITATION_INCOMPLETE",
                Reference::new("limitation", &limitation.limitation_id),
                "limitations must include source, affected scope, statement, effect, and provenance",
            ));
        }
    }
}

fn check_tbds(sections: &ReportSections, diagnostics: &mut Vec<Diagnostic>) {
    for tbd in &sections.unresolved_tbds {
        if tbd.tbd_id.trim().is_empty()
            || !tbd.affected_scope.is_complete()
            || tbd.description.trim().is_empty()
            || !tbd.review_needed
        {
            diagnostics.push(Diagnostic::report_blocking(
                "REPORT_SECTION_TBD_INCOMPLETE",
                Reference::new("tbd", &tbd.tbd_id),
                "unresolved TBDs must include identity, affected scope, description, and review-needed flag",
            ));
        }
    }
}

fn invented_provenance() -> Provenance {
    Provenance {
        source_name: "OpenPipeStress report section validator".to_string(),
        source_location: "core/reporting/report_sections".to_string(),
        source_license: "project".to_string(),
        contributor: "OpenPipeStress".to_string(),
        contributor_certification: "diagnostic metadata only".to_string(),
        redistribution_status: RedistributionStatus::InventedNonEngineeringExample,
        review_status: ReviewStatus::Accepted,
        privacy_classification: PrivacyClassification::InventedPublicExample,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn provenance() -> Provenance {
        Provenance {
            source_name: "invented report fixture".to_string(),
            source_location: "tests/report_sections/invented".to_string(),
            source_license: "project invented".to_string(),
            contributor: "OpenPipeStress".to_string(),
            contributor_certification: "invented non-engineering example".to_string(),
            redistribution_status: RedistributionStatus::InventedNonEngineeringExample,
            review_status: ReviewStatus::Accepted,
            privacy_classification: PrivacyClassification::InventedPublicExample,
        }
    }

    fn effect() -> ReportEffect {
        ReportEffect {
            mechanics_solve_qualified: false,
            user_rule_check_qualified: true,
            report_completeness: ReportCompleteness::Qualified,
            human_review_required: true,
        }
    }

    fn sections() -> ReportSections {
        ReportSections {
            report_section_id: "report-sections-1".to_string(),
            model_ref: Reference::new("model", "invented-model"),
            run_ref: Reference::new("analysis_run", "run-1"),
            diagnostics: vec![Diagnostic {
                code: "RULE_INPUTS_INCOMPLETE".to_string(),
                class: DiagnosticClass::RuleCheckBlocking,
                severity: Severity::Blocking,
                source: Reference::new("rule_pack", "invented-rule-pack"),
                affected_object: Reference::new("check", "fake-check"),
                message: "invented rule input is missing".to_string(),
                remediation: "supply user-owned input before user-rule checking".to_string(),
                provenance: provenance(),
            }],
            analysis_status_disclosures: vec![
                AnalysisStatusDisclosure {
                    status: AnalysisStatus::RuleInputsIncomplete,
                    source: Reference::new("analysis_status", "status-1"),
                    affected_object: Reference::new("model", "invented-model"),
                    explanation: "invented rule input is incomplete".to_string(),
                    human_review_required: true,
                    human_acceptance_ref: None,
                },
                AnalysisStatusDisclosure {
                    status: AnalysisStatus::HumanReviewRequired,
                    source: Reference::new("analysis_status", "status-2"),
                    affected_object: Reference::new("report", "report-1"),
                    explanation: "software output remains decision support".to_string(),
                    human_review_required: true,
                    human_acceptance_ref: None,
                },
            ],
            provenance_notes: vec![provenance()],
            user_supplied_values: vec![UserSuppliedValue {
                value_id: "user-value-1".to_string(),
                value_category: "rule_pack_value".to_string(),
                source: Reference::new("rule_pack", "invented-rule-pack"),
                quantity: Some(Quantity {
                    magnitude: 1.0,
                    unit: "dimensionless".to_string(),
                    dimension: DimensionId::Dimensionless,
                }),
                provenance: provenance(),
                privacy_classification: PrivacyClassification::PrivateRulePackData,
                required_for: vec![RequiredFor::UserRuleCheck],
                review_status: ReviewStatus::Pending,
                missing_data_finding: false,
            }],
            assumptions: vec![Assumption {
                assumption_id: "A-001".to_string(),
                owner: "human project authority".to_string(),
                source: Reference::new("decision", "invented-assumption"),
                affected_scope: Reference::new("model", "invented-model"),
                statement: "invented non-engineering assumption".to_string(),
                review_status: ReviewStatus::Pending,
                expiration_or_re_review_trigger: Some("model changes".to_string()),
                effect: effect(),
                provenance: provenance(),
            }],
            limitations: vec![Limitation {
                limitation_id: "L-001".to_string(),
                source: Reference::new("report", "limitation"),
                affected_scope: Reference::new("report", "report-1"),
                statement: "invented report is not professional approval".to_string(),
                effect: effect(),
                provenance: provenance(),
            }],
            unresolved_tbds: vec![UnresolvedTbd {
                tbd_id: "TBD-001".to_string(),
                affected_scope: Reference::new("report_renderer", "layout"),
                description: "final report layout remains TBD".to_string(),
                review_needed: true,
            }],
            professional_boundary: ProfessionalBoundary::project_default(),
        }
    }

    #[test]
    fn valid_sections_have_no_generated_blocking_diagnostics() {
        let validation = validate_report_sections(&sections());
        assert!(validation.diagnostics.is_empty());
    }

    #[test]
    fn human_review_required_status_is_mandatory() {
        let mut sections = sections();
        sections
            .analysis_status_disclosures
            .retain(|status| status.status != AnalysisStatus::HumanReviewRequired);

        let validation = validate_report_sections(&sections);

        assert!(validation.has_blocking_diagnostics());
        assert!(validation
            .diagnostics
            .iter()
            .any(|diagnostic| { diagnostic.code == "REPORT_SECTION_HUMAN_REVIEW_STATUS_MISSING" }));
    }

    #[test]
    fn professional_boundary_violation_is_blocking() {
        let mut sections = sections();
        sections
            .professional_boundary
            .software_makes_compliance_claim = true;

        let validation = validate_report_sections(&sections);

        assert!(validation.has_blocking_diagnostics());
        assert!(validation.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "REPORT_SECTION_PROFESSIONAL_BOUNDARY_VIOLATION"
        }));
    }

    #[test]
    fn user_rule_values_need_quantity_metadata() {
        let mut sections = sections();
        sections.user_supplied_values[0].quantity = None;

        let validation = validate_report_sections(&sections);

        assert!(validation.has_blocking_diagnostics());
        assert!(validation.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "REPORT_SECTION_USER_VALUE_QUANTITY_INCOMPLETE"
        }));
    }

    #[test]
    fn protected_suspected_provenance_is_blocking() {
        let mut sections = sections();
        sections.provenance_notes[0].redistribution_status =
            RedistributionStatus::ProtectedSuspected;

        let validation = validate_report_sections(&sections);

        assert!(validation.has_blocking_diagnostics());
        assert!(validation
            .diagnostics
            .iter()
            .any(|diagnostic| { diagnostic.code == "REPORT_SECTION_PROTECTED_CONTENT_SUSPECTED" }));
    }

    #[test]
    fn blocking_diagnostics_sort_first() {
        let mut sections = sections();
        sections.diagnostics.push(Diagnostic {
            code: "INFO_ONLY".to_string(),
            class: DiagnosticClass::AssumptionWarning,
            severity: Severity::Info,
            source: Reference::new("assumption", "A-001"),
            affected_object: Reference::new("model", "invented-model"),
            message: "informational invented assumption".to_string(),
            remediation: "review if relevant".to_string(),
            provenance: provenance(),
        });

        let sorted = sorted_diagnostics(&sections);

        assert_eq!(sorted[0].severity, Severity::Blocking);
        assert_eq!(sorted[0].code, "RULE_INPUTS_INCOMPLETE");
    }
}
