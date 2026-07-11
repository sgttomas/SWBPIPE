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

    findings.extend(standards_table_signature_findings(target));

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

/// Standards-table signature detection (DEC-058 check (b), bounded tranche
/// TP-E7-SCANEXT-001).
///
/// Detects the SHAPE of protected standards-table content: a standards
/// designator or standards-clause label adjacent to a dense numeric grid.
/// The tokens below are designator NAMES and generic clause labels only —
/// no standards text, table values, or other protected content is embedded
/// here. Detection is heuristic; the failure direction is
/// `UnknownProvenanceReviewRequired` routed to human IP review, never a
/// silent pass, and a match is not a determination that content is
/// protected.
const STANDARDS_SIGNAL_WINDOW_LINES: usize = 8;
const GRID_MIN_CONSECUTIVE_DENSE_ROWS: usize = 3;
const DENSE_ROW_MIN_NUMERIC_FIELDS: usize = 4;

pub const STANDARDS_TABLE_SIGNATURE_POLICY: &str = "OPS-K-IP-3";

/// Designator-name and clause-label tokens (lowercase). Tokens ending in
/// `.` or `-` are prefixes (e.g. `b16.` matches `b16.5`).
fn standards_signal_tokens() -> &'static [&'static str] {
    &[
        "asme",
        "b31.1",
        "b31.3",
        "b31.4",
        "b31.5",
        "b31.8",
        "b31.9",
        "b31.12",
        "b31j",
        "b16.",
        "b36.10",
        "b36.19",
        "mss sp-",
        "en 13480",
        "iso 14692",
        "csa z662",
        "allowable stress",
        "material allowable",
        "stress intensification",
        "flexibility factor",
        "sif table",
        "code table",
    ]
}

fn is_token_char(character: char) -> bool {
    character.is_ascii_alphanumeric()
}

/// Case-insensitive token search with boundary checks: the character before
/// the match must not be alphanumeric, and — unless the token is an explicit
/// prefix ending in `.` or `-` — the character after must not be
/// alphanumeric either (so `carb31.3x`-style substrings do not match, while
/// `b16.` still matches `b16.5`).
fn find_standards_signal(lower_line: &str) -> Option<&'static str> {
    for token in standards_signal_tokens() {
        let mut search_from = 0;
        while let Some(offset) = lower_line[search_from..].find(token) {
            let start = search_from + offset;
            let end = start + token.len();
            let boundary_before = lower_line[..start]
                .chars()
                .next_back()
                .is_none_or(|character| !is_token_char(character));
            let is_prefix_token = token.ends_with('.') || token.ends_with('-');
            let boundary_after = is_prefix_token
                || lower_line[end..]
                    .chars()
                    .next()
                    .is_none_or(|character| !is_token_char(character));
            if boundary_before && boundary_after {
                return Some(token);
            }
            search_from = end;
        }
    }
    None
}

fn is_table_markup_token(field: &str) -> bool {
    matches!(
        field,
        "td" | "th" | "tr" | "tbody" | "thead" | "table" | "span" | "div" | "p" | "br"
    )
}

fn is_numeric_field(field: &str) -> bool {
    !field.is_empty() && field.parse::<f64>().is_ok()
}

/// A dense numeric row looks like one row of a code-table-like grid: at
/// least `DENSE_ROW_MIN_NUMERIC_FIELDS` numeric fields making up at least
/// 60 percent of the non-markup fields on the line.
fn is_dense_numeric_row(line: &str) -> bool {
    let fields: Vec<&str> = line
        .split(|character: char| {
            character.is_whitespace()
                || matches!(
                    character,
                    ',' | ';'
                        | '|'
                        | '['
                        | ']'
                        | '('
                        | ')'
                        | '{'
                        | '}'
                        | ':'
                        | '"'
                        | '\''
                        | '='
                        | '<'
                        | '>'
                        | '/'
                )
        })
        .filter(|field| !field.is_empty())
        .filter(|field| !is_table_markup_token(&field.to_ascii_lowercase()))
        .collect();

    if fields.is_empty() {
        return false;
    }

    let numeric_count = fields
        .iter()
        .filter(|field| is_numeric_field(field))
        .count();

    numeric_count >= DENSE_ROW_MIN_NUMERIC_FIELDS && numeric_count * 5 >= fields.len() * 3
}

struct NumericGrid {
    start_line: usize,
    end_line: usize,
    row_count: usize,
}

fn numeric_grids(lines: &[&str]) -> Vec<NumericGrid> {
    let mut grids = Vec::new();
    let mut run_start: Option<usize> = None;

    for (index, line) in lines.iter().enumerate() {
        if is_dense_numeric_row(line) {
            run_start.get_or_insert(index);
        } else if let Some(start) = run_start.take() {
            if index - start >= GRID_MIN_CONSECUTIVE_DENSE_ROWS {
                grids.push(NumericGrid {
                    start_line: start + 1,
                    end_line: index,
                    row_count: index - start,
                });
            }
        }
    }

    if let Some(start) = run_start {
        if lines.len() - start >= GRID_MIN_CONSECUTIVE_DENSE_ROWS {
            grids.push(NumericGrid {
                start_line: start + 1,
                end_line: lines.len(),
                row_count: lines.len() - start,
            });
        }
    }

    grids
}

/// One finding per numeric grid that has a standards designator or clause
/// label within `STANDARDS_SIGNAL_WINDOW_LINES` lines of it. The excerpt is
/// the matched signal token only — grid values are never copied into the
/// finding.
fn standards_table_signature_findings(target: &LintTarget) -> Vec<LintFinding> {
    let lines: Vec<&str> = target.text.lines().collect();
    let lower_lines: Vec<String> = lines
        .iter()
        .map(|line| line.to_ascii_lowercase())
        .collect();
    let mut findings = Vec::new();

    for grid in numeric_grids(&lines) {
        let window_start = grid
            .start_line
            .saturating_sub(STANDARDS_SIGNAL_WINDOW_LINES + 1);
        let window_end = usize::min(
            grid.end_line + STANDARDS_SIGNAL_WINDOW_LINES,
            lower_lines.len(),
        );

        let signal = lower_lines[window_start..window_end]
            .iter()
            .find_map(|line| find_standards_signal(line));

        if let Some(token) = signal {
            findings.push(make_finding(
                target,
                FindingCode::UnknownProvenanceReviewRequired,
                FindingClass::ProvenanceWarning,
                FindingSeverity::Warning,
                grid.start_line,
                1,
                token,
                STANDARDS_TABLE_SIGNATURE_POLICY,
                format!(
                    "Standards-table signature: a {}-row numeric grid appears near a \
                     standards designator or clause label; provenance review is required \
                     before any public release surface may carry it.",
                    grid.row_count
                ),
                "Route to human IP review: verify the table is invented or lawfully \
                 redistributable with recorded provenance, or remove/quarantine it per \
                 docs/IP_AND_DATA_BOUNDARY.md section 5.",
                ReviewRoute::HumanIpReview,
            ));
        }
    }

    findings
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

    // All standards-table signature fixtures below are INVENTED lookalikes
    // for pattern testing only: designator names plus obviously synthetic
    // sequential values. No standards content, table values, or other
    // protected data is reproduced.

    const INVENTED_DESIGNATOR_GRID: &str = "\
Invented lookalike fixture: no standards values copied.\n\
ASME B31.3 Table X-1 (invented placeholder heading)\n\
101.1 202.2 303.3 404.4 505.5\n\
111.1 222.2 333.3 444.4 555.5\n\
121.1 242.2 363.3 484.4 605.5\n";

    const INVENTED_CLAUSE_LABEL_GRID: &str = "\
Invented allowable stress table (synthetic demonstration values only)\n\
| 10.1 | 20.2 | 30.3 | 40.4 |\n\
| 11.1 | 21.2 | 31.3 | 41.4 |\n\
| 12.1 | 22.2 | 32.3 | 42.4 |\n\
| 13.1 | 23.2 | 33.3 | 43.4 |\n";

    #[test]
    fn designator_adjacent_numeric_grid_fails_toward_provenance_review() {
        let run = lint_targets(
            "run-sig-001",
            LintConfiguration::public_surfaces_only("cfg-001"),
            vec![target(
                "target-sig-designator",
                "fixtures/report_lint/invented/designator_grid.txt",
                SurfaceKind::PublicReportExample,
                INVENTED_DESIGNATOR_GRID,
            )],
        );

        assert_eq!(run.summary.finding_count, 1);
        assert_eq!(run.summary.blocking_finding_count, 0);
        let finding = &run.findings[0];
        assert_eq!(finding.code, FindingCode::UnknownProvenanceReviewRequired);
        assert_eq!(finding.class, FindingClass::ProvenanceWarning);
        assert_eq!(finding.severity, FindingSeverity::Warning);
        assert_eq!(finding.review_route, ReviewRoute::HumanIpReview);
        assert_eq!(finding.matched_policy, STANDARDS_TABLE_SIGNATURE_POLICY);
        assert_eq!(finding.source_location.line, 3);
        assert_eq!(finding.excerpt, "asme");
        assert!(!run.summary.clean_scan_is_clearance);
    }

    #[test]
    fn clause_label_adjacent_numeric_grid_fails_toward_provenance_review() {
        let run = lint_targets(
            "run-sig-002",
            LintConfiguration::public_surfaces_only("cfg-001"),
            vec![target(
                "target-sig-clause",
                "fixtures/report_lint/invented/clause_label_grid.txt",
                SurfaceKind::PublicFixture,
                INVENTED_CLAUSE_LABEL_GRID,
            )],
        );

        assert_eq!(run.summary.finding_count, 1);
        assert_eq!(
            run.findings[0].code,
            FindingCode::UnknownProvenanceReviewRequired
        );
        assert_eq!(run.findings[0].excerpt, "allowable stress");
        assert_eq!(run.findings[0].review_route, ReviewRoute::HumanIpReview);
    }

    #[test]
    fn json_shaped_numeric_grid_near_designator_is_detected() {
        let text = "\
{\n\
  \"note\": \"invented lookalike, B16.5-style label only\",\n\
  \"rows\": [\n\
    [15.1, 25.2, 35.3, 45.4],\n\
    [16.1, 26.2, 36.3, 46.4],\n\
    [17.1, 27.2, 37.3, 47.4]\n\
  ]\n\
}\n";
        let run = lint_targets(
            "run-sig-003",
            LintConfiguration::public_surfaces_only("cfg-001"),
            vec![target(
                "target-sig-json",
                "fixtures/report_lint/invented/designator_grid.json",
                SurfaceKind::PublicFixture,
                text,
            )],
        );

        assert_eq!(run.summary.finding_count, 1);
        assert_eq!(run.findings[0].excerpt, "b16.");
    }

    #[test]
    fn designator_prose_without_numeric_grid_is_not_flagged() {
        let run = lint_targets(
            "run-sig-004",
            LintConfiguration::public_surfaces_only("cfg-001"),
            vec![target(
                "target-sig-prose",
                "fixtures/report_lint/invented/designator_prose.txt",
                SurfaceKind::PublicReportExample,
                "Code-specific values such as ASME B31.3 allowables are user-entered \
                 on the private side of the code-neutral boundary and never shipped.",
            )],
        );

        assert_eq!(run.summary.finding_count, 0);
    }

    #[test]
    fn numeric_grid_without_standards_signal_is_not_flagged() {
        let run = lint_targets(
            "run-sig-005",
            LintConfiguration::public_surfaces_only("cfg-001"),
            vec![target(
                "target-sig-plain-grid",
                "fixtures/report_lint/invented/plain_grid.txt",
                SurfaceKind::PublicFixture,
                "Invented node coordinates for the preview model.\n\
                 1.0 2.0 3.0 4.0\n\
                 5.0 6.0 7.0 8.0\n\
                 9.0 10.0 11.0 12.0\n",
            )],
        );

        assert_eq!(run.summary.finding_count, 0);
    }

    #[test]
    fn signal_outside_window_is_not_flagged() {
        let mut text = String::from("ASME (designator name mentioned in prose only)\n");
        for _ in 0..STANDARDS_SIGNAL_WINDOW_LINES + 2 {
            text.push_str("prose line without numbers\n");
        }
        text.push_str("21.1 22.2 23.3 24.4\n25.1 26.2 27.3 28.4\n29.1 30.2 31.3 32.4\n");

        let run = lint_targets(
            "run-sig-006",
            LintConfiguration::public_surfaces_only("cfg-001"),
            vec![target(
                "target-sig-window",
                "fixtures/report_lint/invented/out_of_window.txt",
                SurfaceKind::PublicFixture,
                &text,
            )],
        );

        assert_eq!(run.summary.finding_count, 0);
    }

    #[test]
    fn embedded_token_substrings_do_not_match() {
        let run = lint_targets(
            "run-sig-007",
            LintConfiguration::public_surfaces_only("cfg-001"),
            vec![target(
                "target-sig-substring",
                "fixtures/report_lint/invented/substring.txt",
                SurfaceKind::PublicFixture,
                "carb31.3x label9 basmea\n\
                 31.1 32.2 33.3 34.4\n\
                 35.1 36.2 37.3 38.4\n\
                 39.1 40.2 41.3 42.4\n",
            )],
        );

        assert_eq!(run.summary.finding_count, 0);
    }

    #[test]
    fn short_or_sparse_numeric_rows_do_not_form_a_grid() {
        let run = lint_targets(
            "run-sig-008",
            LintConfiguration::public_surfaces_only("cfg-001"),
            vec![target(
                "target-sig-sparse",
                "fixtures/report_lint/invented/sparse.txt",
                SurfaceKind::PublicReportExample,
                "ASME designator name in prose.\n\
                 case 1 ran in 2.5 seconds with 3 warnings and 4 notes\n\
                 case 2 ran in 3.5 seconds with 1 warning and 2 notes\n\
                 case 3 ran in 4.5 seconds with 0 warnings and 1 note\n\
                 totals: 10.5 4 7\n",
            )],
        );

        assert_eq!(run.summary.finding_count, 0);
    }

    #[test]
    fn two_grids_near_signals_emit_two_findings_in_stable_order() {
        let text = format!(
            "{}\nsafe prose separator without numbers\nrepeat separator\n\
             more separator prose\nanother separator line\nyet another line\n\
             separator seven\nseparator eight\nseparator nine\n{}",
            INVENTED_DESIGNATOR_GRID, INVENTED_CLAUSE_LABEL_GRID
        );
        let run = lint_targets(
            "run-sig-009",
            LintConfiguration::public_surfaces_only("cfg-001"),
            vec![target(
                "target-sig-two-grids",
                "fixtures/report_lint/invented/two_grids.txt",
                SurfaceKind::PublicFixture,
                &text,
            )],
        );

        assert_eq!(run.summary.finding_count, 2);
        assert_eq!(run.findings[0].finding_id, "RPLC-F0001");
        assert_eq!(run.findings[1].finding_id, "RPLC-F0002");
        assert!(
            run.findings[0].source_location.line < run.findings[1].source_location.line
        );
        assert!(run
            .findings
            .iter()
            .all(|finding| finding.code == FindingCode::UnknownProvenanceReviewRequired));
    }

    #[test]
    fn accepted_provenance_does_not_suppress_signature_review() {
        // The shape check is independent of recorded provenance: even a
        // target whose provenance is accepted still routes a
        // standards-table signature to human IP review (fail toward review,
        // never silent pass).
        let mut lint_target_value = target(
            "target-sig-accepted",
            "fixtures/report_lint/invented/accepted_provenance.txt",
            SurfaceKind::PublicFixture,
            INVENTED_DESIGNATOR_GRID,
        );
        lint_target_value.provenance.review_status = ReviewStatus::Accepted;
        lint_target_value.provenance.redistribution_status =
            RedistributionStatus::PublicPermissive;

        let run = lint_targets(
            "run-sig-010",
            LintConfiguration::public_surfaces_only("cfg-001"),
            vec![lint_target_value],
        );

        assert_eq!(run.summary.finding_count, 1);
        assert_eq!(
            run.findings[0].code,
            FindingCode::UnknownProvenanceReviewRequired
        );
    }

    #[test]
    fn markup_wrapped_numeric_grid_is_detected() {
        let text = "\
<h2>Invented flexibility factor demonstration (synthetic values)</h2>\n\
<tr><td>51.1</td><td>52.2</td><td>53.3</td><td>54.4</td></tr>\n\
<tr><td>55.1</td><td>56.2</td><td>57.3</td><td>58.4</td></tr>\n\
<tr><td>59.1</td><td>60.2</td><td>61.3</td><td>62.4</td></tr>\n";
        let run = lint_targets(
            "run-sig-011",
            LintConfiguration::public_surfaces_only("cfg-001"),
            vec![target(
                "target-sig-markup",
                "fixtures/report_lint/invented/markup_grid.html",
                SurfaceKind::PublicReportExample,
                text,
            )],
        );

        assert_eq!(run.summary.finding_count, 1);
        assert_eq!(run.findings[0].excerpt, "flexibility factor");
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
