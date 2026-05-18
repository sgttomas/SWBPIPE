//! Report protected-content lint support.
//!
//! This crate evaluates caller-supplied public report/template/example text
//! against deterministic synthetic markers and professional-boundary phrases.
//! It does not read arbitrary project files, scan private user paths by
//! default, move files into quarantine, implement redaction/export controls,
//! run GUI/CLI/API/adapter workflows, choose CI/release policy, or provide
//! legal clearance, security sufficiency, professional approval, or
//! code-compliance proof.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SurfaceKind {
    PublicReportTemplate,
    PublicReportExample,
    PublicFixture,
    PrivateUserTemplate,
    PrivateProjectExport,
}

impl SurfaceKind {
    fn is_public_by_default(self) -> bool {
        matches!(
            self,
            SurfaceKind::PublicReportTemplate
                | SurfaceKind::PublicReportExample
                | SurfaceKind::PublicFixture
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FindingCode {
    ProtectedContentSyntheticMarker,
    PrivateDataSyntheticMarker,
    ProprietarySourceSyntheticMarker,
    UnknownProvenanceReviewRequired,
    ProhibitedProfessionalClaim,
    SafeMetadataAllowed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindingClass {
    IpBoundaryWarning,
    PrivateDataWarning,
    ProvenanceWarning,
    ProfessionalBoundaryWarning,
    SafeMetadata,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindingSeverity {
    Info,
    Warning,
    Blocking,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReviewRoute {
    HumanIpReview,
    HumanPrivacyReview,
    HumanProfessionalBoundaryReview,
    MaintainerProvenanceReview,
    NoAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReviewStatus {
    Pending,
    Accepted,
    Rejected,
    Quarantined,
    NotApplicable,
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

    pub fn is_complete(&self) -> bool {
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
pub struct LintTarget {
    pub target_id: String,
    pub path: String,
    pub surface: SurfaceKind,
    pub text: String,
    pub provenance: Provenance,
}

impl LintTarget {
    fn is_complete(&self) -> bool {
        !self.target_id.trim().is_empty()
            && !self.path.trim().is_empty()
            && self.provenance.is_complete()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLocation {
    pub path: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LintFinding {
    pub finding_id: String,
    pub code: FindingCode,
    pub class: FindingClass,
    pub severity: FindingSeverity,
    pub target_ref: Reference,
    pub source_location: SourceLocation,
    pub matched_policy: String,
    pub excerpt: String,
    pub message: String,
    pub remediation: String,
    pub review_route: ReviewRoute,
    pub disposition: ReviewStatus,
    pub provenance: Provenance,
}

impl LintFinding {
    fn ordering_key(&self) -> (&str, usize, usize, FindingCode, &str) {
        (
            self.source_location.path.as_str(),
            self.source_location.line,
            self.source_location.column,
            self.code,
            self.finding_id.as_str(),
        )
    }

    pub fn is_blocking(&self) -> bool {
        self.severity == FindingSeverity::Blocking
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LintConfiguration {
    pub configuration_id: String,
    pub public_surface_roots: Vec<String>,
    pub scan_private_surfaces: bool,
}

impl LintConfiguration {
    pub fn public_surfaces_only(configuration_id: impl Into<String>) -> Self {
        Self {
            configuration_id: configuration_id.into(),
            public_surface_roots: vec![
                "schemas/".to_string(),
                "fixtures/".to_string(),
                "docs/".to_string(),
            ],
            scan_private_surfaces: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LintSummary {
    pub target_count: usize,
    pub scanned_target_count: usize,
    pub skipped_private_target_count: usize,
    pub finding_count: usize,
    pub blocking_finding_count: usize,
    pub clean_scan_is_clearance: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LintRun {
    pub run_id: String,
    pub configuration: LintConfiguration,
    pub targets: Vec<LintTarget>,
    pub findings: Vec<LintFinding>,
    pub summary: LintSummary,
}

pub fn lint_targets(
    run_id: impl Into<String>,
    configuration: LintConfiguration,
    targets: Vec<LintTarget>,
) -> LintRun {
    let mut findings = Vec::new();
    let mut scanned_target_count = 0;
    let mut skipped_private_target_count = 0;

    for target in &targets {
        if should_scan_target(target, &configuration) {
            scanned_target_count += 1;
            findings.extend(lint_target(target));
        } else {
            skipped_private_target_count += 1;
        }
    }

    findings.sort_by(|left, right| left.ordering_key().cmp(&right.ordering_key()));

    for (index, finding) in findings.iter_mut().enumerate() {
        finding.finding_id = format!("RPLC-F{:04}", index + 1);
    }

    let blocking_finding_count = findings
        .iter()
        .filter(|finding| finding.is_blocking())
        .count();

    LintRun {
        run_id: run_id.into(),
        summary: LintSummary {
            target_count: targets.len(),
            scanned_target_count,
            skipped_private_target_count,
            finding_count: findings.len(),
            blocking_finding_count,
            clean_scan_is_clearance: false,
        },
        configuration,
        targets,
        findings,
    }
}

pub fn should_scan_target(target: &LintTarget, configuration: &LintConfiguration) -> bool {
    if !target.is_complete() {
        return false;
    }

    target.surface.is_public_by_default() || configuration.scan_private_surfaces
}

pub fn lint_target(target: &LintTarget) -> Vec<LintFinding> {
    let mut findings = Vec::new();

    for (line_index, line) in target.text.lines().enumerate() {
        let line_number = line_index + 1;
        let lower = line.to_ascii_lowercase();

        for marker in synthetic_markers() {
            if let Some(column) = line.find(marker.pattern) {
                findings.push(make_finding(
                    target,
                    marker.code,
                    marker.class,
                    marker.severity,
                    line_number,
                    column + 1,
                    marker.pattern,
                    marker.policy,
                    marker.message,
                    marker.remediation,
                    marker.review_route,
                ));
            }
        }

        for phrase in prohibited_claim_phrases() {
            if let Some(byte_column) = lower.find(phrase) {
                findings.push(make_finding(
                    target,
                    FindingCode::ProhibitedProfessionalClaim,
                    FindingClass::ProfessionalBoundaryWarning,
                    FindingSeverity::Blocking,
                    line_number,
                    byte_column + 1,
                    *phrase,
                    "OPS-K-AUTH-1",
                    "Public report language appears to claim software professional authority.",
                    "Replace with decision-support and human-review-required wording.",
                    ReviewRoute::HumanProfessionalBoundaryReview,
                ));
            }
        }
    }

    if target.provenance.redistribution_status == RedistributionStatus::Unknown
        || target.provenance.review_status == ReviewStatus::Pending
    {
        findings.push(make_finding(
            target,
            FindingCode::UnknownProvenanceReviewRequired,
            FindingClass::ProvenanceWarning,
            FindingSeverity::Warning,
            1,
            1,
            "provenance",
            "OPS-K-IP-2",
            "Public report surface lacks accepted redistribution or review evidence.",
            "Record source, license, contributor certification, and review disposition.",
            ReviewRoute::MaintainerProvenanceReview,
        ));
    }

    findings.sort_by(|left, right| left.ordering_key().cmp(&right.ordering_key()));
    findings
}

struct Marker {
    pattern: &'static str,
    code: FindingCode,
    class: FindingClass,
    severity: FindingSeverity,
    policy: &'static str,
    message: &'static str,
    remediation: &'static str,
    review_route: ReviewRoute,
}

fn synthetic_markers() -> &'static [Marker] {
    &[
        Marker {
            pattern: "OPS_SYNTHETIC_PROTECTED_TABLE",
            code: FindingCode::ProtectedContentSyntheticMarker,
            class: FindingClass::IpBoundaryWarning,
            severity: FindingSeverity::Blocking,
            policy: "OPS-K-IP-1",
            message: "Synthetic marker represents protected table risk.",
            remediation:
                "Remove table-like protected content and route suspected source to review.",
            review_route: ReviewRoute::HumanIpReview,
        },
        Marker {
            pattern: "OPS_SYNTHETIC_CODE_FORMULA",
            code: FindingCode::ProtectedContentSyntheticMarker,
            class: FindingClass::IpBoundaryWarning,
            severity: FindingSeverity::Blocking,
            policy: "OPS-K-IP-1",
            message: "Synthetic marker represents copied protected formula risk.",
            remediation:
                "Replace with user-supplied/private references or original open mechanics.",
            review_route: ReviewRoute::HumanIpReview,
        },
        Marker {
            pattern: "OPS_SYNTHETIC_PRIVATE_RULE_PAYLOAD",
            code: FindingCode::PrivateDataSyntheticMarker,
            class: FindingClass::PrivateDataWarning,
            severity: FindingSeverity::Blocking,
            policy: "OPS-K-PRIV-1",
            message: "Synthetic marker represents private rule-pack payload risk.",
            remediation:
                "Redact private payload details and keep only safe identity/checksum metadata.",
            review_route: ReviewRoute::HumanPrivacyReview,
        },
        Marker {
            pattern: "OPS_SYNTHETIC_VENDOR_CATALOG",
            code: FindingCode::ProprietarySourceSyntheticMarker,
            class: FindingClass::ProvenanceWarning,
            severity: FindingSeverity::Warning,
            policy: "OPS-K-IP-2",
            message: "Synthetic marker represents proprietary source or vendor catalog risk.",
            remediation: "Require documented redistribution rights before public inclusion.",
            review_route: ReviewRoute::MaintainerProvenanceReview,
        },
    ]
}

fn prohibited_claim_phrases() -> &'static [&'static str] {
    &[
        "code compliant",
        "certified by openpipestress",
        "sealed by openpipestress",
        "approved by openpipestress",
        "authenticated by openpipestress",
        "professional approval by the software",
    ]
}

#[allow(clippy::too_many_arguments)]
fn make_finding(
    target: &LintTarget,
    code: FindingCode,
    class: FindingClass,
    severity: FindingSeverity,
    line: usize,
    column: usize,
    excerpt: impl Into<String>,
    matched_policy: impl Into<String>,
    message: impl Into<String>,
    remediation: impl Into<String>,
    review_route: ReviewRoute,
) -> LintFinding {
    LintFinding {
        finding_id: "PENDING_ORDERING".to_string(),
        code,
        class,
        severity,
        target_ref: Reference::new("lint_target", target.target_id.clone()),
        source_location: SourceLocation {
            path: target.path.clone(),
            line,
            column,
        },
        matched_policy: matched_policy.into(),
        excerpt: excerpt.into(),
        message: message.into(),
        remediation: remediation.into(),
        review_route,
        disposition: ReviewStatus::Pending,
        provenance: invented_provenance(),
    }
}

pub fn invented_provenance() -> Provenance {
    Provenance {
        source_name: "OpenPipeStress invented synthetic linter fixture".to_string(),
        source_location: "fixtures/report_lint/invented".to_string(),
        source_license: "TBD".to_string(),
        contributor: "OpenPipeStress".to_string(),
        contributor_certification:
            "Invented non-engineering synthetic marker; no protected source copied.".to_string(),
        redistribution_status: RedistributionStatus::InventedNonEngineeringExample,
        review_status: ReviewStatus::Accepted,
        privacy_classification: PrivacyClassification::InventedPublicExample,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn target(id: &str, path: &str, surface: SurfaceKind, text: &str) -> LintTarget {
        LintTarget {
            target_id: id.to_string(),
            path: path.to_string(),
            surface,
            text: text.to_string(),
            provenance: invented_provenance(),
        }
    }

    #[test]
    fn synthetic_markers_emit_blocking_findings_in_stable_order() {
        let run = lint_targets(
            "run-001",
            LintConfiguration::public_surfaces_only("cfg-001"),
            vec![target(
                "target-001",
                "fixtures/report_lint/invented/synthetic_risks.txt",
                SurfaceKind::PublicReportTemplate,
                "OPS_SYNTHETIC_PRIVATE_RULE_PAYLOAD\nOPS_SYNTHETIC_PROTECTED_TABLE",
            )],
        );

        assert_eq!(run.summary.scanned_target_count, 1);
        assert_eq!(run.summary.blocking_finding_count, 2);
        assert_eq!(
            run.findings[0].code,
            FindingCode::PrivateDataSyntheticMarker
        );
        assert_eq!(
            run.findings[1].code,
            FindingCode::ProtectedContentSyntheticMarker
        );
        assert_eq!(run.findings[0].finding_id, "RPLC-F0001");
    }

    #[test]
    fn private_surfaces_are_skipped_by_default() {
        let run = lint_targets(
            "run-002",
            LintConfiguration::public_surfaces_only("cfg-001"),
            vec![target(
                "target-private",
                "/private/report-template.txt",
                SurfaceKind::PrivateUserTemplate,
                "OPS_SYNTHETIC_PROTECTED_TABLE",
            )],
        );

        assert_eq!(run.summary.scanned_target_count, 0);
        assert_eq!(run.summary.skipped_private_target_count, 1);
        assert!(run.findings.is_empty());
        assert!(!run.summary.clean_scan_is_clearance);
    }

    #[test]
    fn safe_metadata_has_no_findings() {
        let run = lint_targets(
            "run-003",
            LintConfiguration::public_surfaces_only("cfg-001"),
            vec![target(
                "target-safe",
                "fixtures/report_lint/invented/safe_metadata.txt",
                SurfaceKind::PublicReportExample,
                "Rule pack id: invented-demo; version: 0.1.0; checksum: sha256:abc123; human review required.",
            )],
        );

        assert_eq!(run.summary.finding_count, 0);
        assert!(!run.summary.clean_scan_is_clearance);
    }

    #[test]
    fn professional_claims_are_blocking() {
        let run = lint_targets(
            "run-004",
            LintConfiguration::public_surfaces_only("cfg-001"),
            vec![target(
                "target-claim",
                "fixtures/report_lint/invented/prohibited_claim.txt",
                SurfaceKind::PublicReportTemplate,
                "This report is code compliant.",
            )],
        );

        assert_eq!(run.summary.blocking_finding_count, 1);
        assert_eq!(
            run.findings[0].code,
            FindingCode::ProhibitedProfessionalClaim
        );
        assert_eq!(
            run.findings[0].review_route,
            ReviewRoute::HumanProfessionalBoundaryReview
        );
    }
}
