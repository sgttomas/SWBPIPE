//! Required-input completeness checker for user-owned rule packs.
//!
//! This crate validates declarative required-input declarations against
//! caller-supplied input evidence. It does not parse rule-pack files, evaluate
//! formulas, provide code-specific defaults, store private data, import
//! protected standards content, or emit professional/code-compliance claims.

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dimension {
    Dimensionless,
    Length,
    Mass,
    Time,
    Temperature,
    TemperatureInterval,
    Angle,
    Rotation,
    Force,
    Moment,
    Pressure,
    Stress,
    Area,
    Volume,
    Density,
    LinearStiffness,
    RotationalStiffness,
    Displacement,
    Velocity,
    Acceleration,
    ThermalConductivity,
    SpecificHeat,
    ThermalExpansionCoefficient,
    SecondMomentArea,
    SectionModulus,
    MassPerLength,
    VolumePerLength,
    Slope,
    Tbd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceKind {
    SolverResult,
    ModelInput,
    UserSuppliedRuleValue,
    PrivateLibraryValue,
    OwnerDesignBasis,
    Tbd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequiredFor {
    RuleCheck,
    Reporting,
    Diagnostic,
    Tbd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueStatus {
    NotProvided,
    PrivateUserSupplied,
    PublicPermissiveReviewed,
    InventedNonEngineeringExample,
    ProtectedSuspected,
    Tbd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProvenanceStatus {
    Accepted,
    Missing,
    Unknown,
    ProtectedSuspected,
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
pub enum AutomaticAnalysisStatus {
    ModelIncomplete,
    MechanicsSolved,
    RuleInputsIncomplete,
    UserRuleChecked,
    UserRuleFailed,
    HumanReviewRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Readiness {
    ReadyForRuleCheck,
    Blocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindingSeverity {
    Info,
    Warning,
    Blocking,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindingCode {
    RuleCheckBlocking,
    RuleInputMissing,
    RuleUnitMissing,
    RuleUnitMismatch,
    RuleProvenanceWarning,
    RuleRedistributionWarning,
    RuleProtectedContentWarning,
    RuleReviewWarning,
    RuleIncompleteData,
    DuplicateRequiredInput,
    DuplicateSuppliedInput,
    UnexpectedSuppliedInput,
    InvalidReference,
    ProfessionalBoundaryViolation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantityIntent {
    pub dimension: Dimension,
    pub unit_ref: String,
    pub unit_required: bool,
    pub dimension_check_required: bool,
}

impl QuantityIntent {
    pub fn new(dimension: Dimension, unit_ref: impl Into<String>) -> Self {
        Self {
            dimension,
            unit_ref: unit_ref.into(),
            unit_required: true,
            dimension_check_required: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequiredInput {
    pub input_id: String,
    pub name: String,
    pub source_kind: SourceKind,
    pub quantity_intent: QuantityIntent,
    pub required_for: RequiredFor,
    pub provenance_required: bool,
    pub redistribution_status_required: bool,
}

impl RequiredInput {
    pub fn rule_check_input(
        input_id: impl Into<String>,
        name: impl Into<String>,
        source_kind: SourceKind,
        quantity_intent: QuantityIntent,
    ) -> Self {
        Self {
            input_id: input_id.into(),
            name: name.into(),
            source_kind,
            quantity_intent,
            required_for: RequiredFor::RuleCheck,
            provenance_required: true,
            redistribution_status_required: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuppliedInput {
    pub input_id: String,
    pub value_status: ValueStatus,
    pub dimension: Option<Dimension>,
    pub unit_ref: Option<String>,
    pub provenance_status: ProvenanceStatus,
    pub redistribution_status: RedistributionStatus,
    pub review_status: ReviewStatus,
}

impl SuppliedInput {
    pub fn private_reviewed(
        input_id: impl Into<String>,
        dimension: Dimension,
        unit_ref: impl Into<String>,
    ) -> Self {
        Self {
            input_id: input_id.into(),
            value_status: ValueStatus::PrivateUserSupplied,
            dimension: Some(dimension),
            unit_ref: Some(unit_ref.into()),
            provenance_status: ProvenanceStatus::Accepted,
            redistribution_status: RedistributionStatus::PrivateOnly,
            review_status: ReviewStatus::Accepted,
        }
    }

    pub fn missing(input_id: impl Into<String>) -> Self {
        Self {
            input_id: input_id.into(),
            value_status: ValueStatus::NotProvided,
            dimension: None,
            unit_ref: None,
            provenance_status: ProvenanceStatus::Missing,
            redistribution_status: RedistributionStatus::Unknown,
            review_status: ReviewStatus::Pending,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletenessFinding {
    pub code: FindingCode,
    pub severity: FindingSeverity,
    pub input_id: String,
    pub message: String,
}

impl CompletenessFinding {
    fn new(
        code: FindingCode,
        severity: FindingSeverity,
        input_id: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            severity,
            input_id: input_id.into(),
            message: message.into(),
        }
    }

    pub fn is_blocking(&self) -> bool {
        self.severity == FindingSeverity::Blocking
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletenessCheckInput {
    pub required_inputs: Vec<RequiredInput>,
    pub supplied_inputs: Vec<SuppliedInput>,
    pub current_statuses: Vec<AutomaticAnalysisStatus>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletenessResult {
    pub readiness: Readiness,
    pub blocking_status: Option<AutomaticAnalysisStatus>,
    pub preserved_statuses: Vec<AutomaticAnalysisStatus>,
    pub complete_input_ids: Vec<String>,
    pub missing_input_ids: Vec<String>,
    pub findings: Vec<CompletenessFinding>,
}

impl CompletenessResult {
    pub fn is_rule_check_blocked(&self) -> bool {
        self.readiness == Readiness::Blocked
    }
}

pub fn check_completeness(input: &CompletenessCheckInput) -> CompletenessResult {
    let mut findings = Vec::new();
    let mut complete_input_ids = Vec::new();
    let mut missing_input_ids = Vec::new();

    let required_map = build_required_map(&input.required_inputs, &mut findings);
    let supplied_map = build_supplied_map(&input.supplied_inputs, &required_map, &mut findings);

    for required in &input.required_inputs {
        if required.required_for != RequiredFor::RuleCheck {
            continue;
        }

        if required.input_id.trim().is_empty() {
            continue;
        }

        match supplied_map.get(required.input_id.as_str()) {
            Some(supplied) => {
                let before = findings.len();
                check_supplied_required_input(required, supplied, &mut findings);
                if findings[before..]
                    .iter()
                    .any(CompletenessFinding::is_blocking)
                {
                    missing_input_ids.push(required.input_id.clone());
                } else {
                    complete_input_ids.push(required.input_id.clone());
                }
            }
            None => {
                findings.push(CompletenessFinding::new(
                    FindingCode::RuleInputMissing,
                    FindingSeverity::Blocking,
                    &required.input_id,
                    "required rule-check input is not supplied",
                ));
                missing_input_ids.push(required.input_id.clone());
            }
        }
    }

    let blocking = findings.iter().any(CompletenessFinding::is_blocking);
    let mut preserved_statuses = collect_automatic_statuses(&input.current_statuses, &mut findings);
    if blocking && !preserved_statuses.contains(&AutomaticAnalysisStatus::RuleInputsIncomplete) {
        preserved_statuses.push(AutomaticAnalysisStatus::RuleInputsIncomplete);
    }

    complete_input_ids.sort();
    complete_input_ids.dedup();
    missing_input_ids.sort();
    missing_input_ids.dedup();

    CompletenessResult {
        readiness: if blocking {
            Readiness::Blocked
        } else {
            Readiness::ReadyForRuleCheck
        },
        blocking_status: if blocking {
            Some(AutomaticAnalysisStatus::RuleInputsIncomplete)
        } else {
            None
        },
        preserved_statuses,
        complete_input_ids,
        missing_input_ids,
        findings,
    }
}

fn build_required_map<'a>(
    required_inputs: &'a [RequiredInput],
    findings: &mut Vec<CompletenessFinding>,
) -> HashMap<&'a str, &'a RequiredInput> {
    let mut map = HashMap::new();
    let mut seen = HashSet::new();
    for required in required_inputs {
        if required.input_id.trim().is_empty() {
            findings.push(CompletenessFinding::new(
                FindingCode::InvalidReference,
                FindingSeverity::Blocking,
                "required_input",
                "required input id must not be empty",
            ));
            continue;
        }

        if !seen.insert(required.input_id.as_str()) {
            findings.push(CompletenessFinding::new(
                FindingCode::DuplicateRequiredInput,
                FindingSeverity::Blocking,
                &required.input_id,
                "required input ids must be unique",
            ));
            continue;
        }

        map.insert(required.input_id.as_str(), required);
    }
    map
}

fn build_supplied_map<'a>(
    supplied_inputs: &'a [SuppliedInput],
    required_map: &HashMap<&str, &RequiredInput>,
    findings: &mut Vec<CompletenessFinding>,
) -> HashMap<&'a str, &'a SuppliedInput> {
    let mut map = HashMap::new();
    let mut seen = HashSet::new();
    for supplied in supplied_inputs {
        if supplied.input_id.trim().is_empty() {
            findings.push(CompletenessFinding::new(
                FindingCode::InvalidReference,
                FindingSeverity::Blocking,
                "supplied_input",
                "supplied input id must not be empty",
            ));
            continue;
        }

        if !seen.insert(supplied.input_id.as_str()) {
            findings.push(CompletenessFinding::new(
                FindingCode::DuplicateSuppliedInput,
                FindingSeverity::Blocking,
                &supplied.input_id,
                "supplied input ids must be unique",
            ));
            continue;
        }

        if !required_map.contains_key(supplied.input_id.as_str()) {
            findings.push(CompletenessFinding::new(
                FindingCode::UnexpectedSuppliedInput,
                FindingSeverity::Warning,
                &supplied.input_id,
                "supplied input is not declared as a rule-pack required input",
            ));
        }

        map.insert(supplied.input_id.as_str(), supplied);
    }
    map
}

fn check_supplied_required_input(
    required: &RequiredInput,
    supplied: &SuppliedInput,
    findings: &mut Vec<CompletenessFinding>,
) {
    if matches!(
        supplied.value_status,
        ValueStatus::NotProvided | ValueStatus::Tbd
    ) {
        findings.push(CompletenessFinding::new(
            FindingCode::RuleInputMissing,
            FindingSeverity::Blocking,
            &required.input_id,
            "required rule-check value is not provided",
        ));
    }

    if supplied.value_status == ValueStatus::ProtectedSuspected {
        findings.push(CompletenessFinding::new(
            FindingCode::RuleProtectedContentWarning,
            FindingSeverity::Blocking,
            &required.input_id,
            "required input is marked as suspected protected content",
        ));
    }

    if required.quantity_intent.unit_required
        && supplied
            .unit_ref
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
    {
        findings.push(CompletenessFinding::new(
            FindingCode::RuleUnitMissing,
            FindingSeverity::Blocking,
            &required.input_id,
            "required rule-check input is missing a unit reference",
        ));
    }

    if required.quantity_intent.dimension_check_required {
        match supplied.dimension {
            Some(actual) if actual == required.quantity_intent.dimension => {}
            Some(_) => findings.push(CompletenessFinding::new(
                FindingCode::RuleUnitMismatch,
                FindingSeverity::Blocking,
                &required.input_id,
                "supplied input dimension does not match the required declaration",
            )),
            None => findings.push(CompletenessFinding::new(
                FindingCode::RuleUnitMismatch,
                FindingSeverity::Blocking,
                &required.input_id,
                "required rule-check input is missing dimension metadata",
            )),
        }
    }

    if required.provenance_required {
        match supplied.provenance_status {
            ProvenanceStatus::Accepted => {}
            ProvenanceStatus::ProtectedSuspected => findings.push(CompletenessFinding::new(
                FindingCode::RuleProtectedContentWarning,
                FindingSeverity::Blocking,
                &required.input_id,
                "required input provenance is suspected protected content",
            )),
            ProvenanceStatus::Missing | ProvenanceStatus::Unknown | ProvenanceStatus::Tbd => {
                findings.push(CompletenessFinding::new(
                    FindingCode::RuleProvenanceWarning,
                    FindingSeverity::Blocking,
                    &required.input_id,
                    "required rule-check input lacks accepted provenance",
                ));
            }
        }
    }

    if required.redistribution_status_required {
        match supplied.redistribution_status {
            RedistributionStatus::PublicPermissive
            | RedistributionStatus::PrivateOnly
            | RedistributionStatus::InventedNonEngineeringExample => {}
            RedistributionStatus::ProtectedSuspected => findings.push(CompletenessFinding::new(
                FindingCode::RuleProtectedContentWarning,
                FindingSeverity::Blocking,
                &required.input_id,
                "required input redistribution status is suspected protected content",
            )),
            RedistributionStatus::Unknown | RedistributionStatus::Tbd => {
                findings.push(CompletenessFinding::new(
                    FindingCode::RuleRedistributionWarning,
                    FindingSeverity::Blocking,
                    &required.input_id,
                    "required rule-check input lacks accepted redistribution status",
                ));
            }
        }
    }

    match supplied.review_status {
        ReviewStatus::Accepted => {}
        ReviewStatus::Rejected | ReviewStatus::Quarantined => {
            findings.push(CompletenessFinding::new(
                FindingCode::RuleProtectedContentWarning,
                FindingSeverity::Blocking,
                &required.input_id,
                "required rule-check input is rejected or quarantined",
            ));
        }
        ReviewStatus::Pending | ReviewStatus::Tbd => findings.push(CompletenessFinding::new(
            FindingCode::RuleReviewWarning,
            FindingSeverity::Blocking,
            &required.input_id,
            "required rule-check input has not been accepted for use",
        )),
    }
}

fn collect_automatic_statuses(
    statuses: &[AutomaticAnalysisStatus],
    findings: &mut Vec<CompletenessFinding>,
) -> Vec<AutomaticAnalysisStatus> {
    let mut collected = Vec::new();
    for status in statuses {
        if !collected.contains(status) {
            collected.push(*status);
        }
    }

    if collected.is_empty() {
        collected.push(AutomaticAnalysisStatus::HumanReviewRequired);
    }

    findings.extend(
        statuses
            .iter()
            .filter(|status| **status == AutomaticAnalysisStatus::UserRuleChecked)
            .map(|_| {
                CompletenessFinding::new(
                    FindingCode::RuleIncompleteData,
                    FindingSeverity::Info,
                    "analysis_status",
                    "existing user-rule checked status is caller-supplied and not produced by completeness checking",
                )
            }),
    );

    collected
}

#[cfg(test)]
mod tests {
    use super::*;

    fn required_stress_input() -> RequiredInput {
        RequiredInput::rule_check_input(
            "allowable_stress",
            "Invented allowable stress",
            SourceKind::UserSuppliedRuleValue,
            QuantityIntent::new(Dimension::Stress, "MPa"),
        )
    }

    #[test]
    fn dimension_enum_tracks_pkg02_canonical_vocabulary() {
        let dimensions = [
            Dimension::Dimensionless,
            Dimension::Length,
            Dimension::Mass,
            Dimension::Time,
            Dimension::Temperature,
            Dimension::TemperatureInterval,
            Dimension::Angle,
            Dimension::Rotation,
            Dimension::Force,
            Dimension::Moment,
            Dimension::Pressure,
            Dimension::Stress,
            Dimension::Area,
            Dimension::Volume,
            Dimension::Density,
            Dimension::LinearStiffness,
            Dimension::RotationalStiffness,
            Dimension::Displacement,
            Dimension::Velocity,
            Dimension::Acceleration,
            Dimension::ThermalConductivity,
            Dimension::SpecificHeat,
            Dimension::ThermalExpansionCoefficient,
            Dimension::SecondMomentArea,
            Dimension::SectionModulus,
            Dimension::MassPerLength,
            Dimension::VolumePerLength,
            Dimension::Slope,
            Dimension::Tbd,
        ];
        assert_eq!(dimensions.len(), 29);
        let rendered = format!("{dimensions:?}");
        assert!(!rendered.contains("TemperatureDifference"));
        assert!(!rendered.contains("AreaMoment"));
        assert!(!rendered.contains("[Stiffness"));
        assert!(!rendered.contains(", Stiffness,"));
        assert!(!rendered.contains(", Stiffness]"));
        assert!(rendered.contains("LinearStiffness"));
        assert!(rendered.contains("RotationalStiffness"));
        assert!(rendered.contains("SecondMomentArea"));
    }

    #[test]
    fn complete_private_input_is_ready_for_rule_check() {
        let result = check_completeness(&CompletenessCheckInput {
            required_inputs: vec![required_stress_input()],
            supplied_inputs: vec![SuppliedInput::private_reviewed(
                "allowable_stress",
                Dimension::Stress,
                "MPa",
            )],
            current_statuses: vec![AutomaticAnalysisStatus::MechanicsSolved],
        });

        assert_eq!(result.readiness, Readiness::ReadyForRuleCheck);
        assert_eq!(result.blocking_status, None);
        assert_eq!(result.complete_input_ids, vec!["allowable_stress"]);
        assert!(result.missing_input_ids.is_empty());
        assert!(result.findings.is_empty());
        assert!(result
            .preserved_statuses
            .contains(&AutomaticAnalysisStatus::MechanicsSolved));
    }

    #[test]
    fn missing_required_value_blocks_rule_check_status() {
        let result = check_completeness(&CompletenessCheckInput {
            required_inputs: vec![required_stress_input()],
            supplied_inputs: vec![],
            current_statuses: vec![AutomaticAnalysisStatus::MechanicsSolved],
        });

        assert!(result.is_rule_check_blocked());
        assert_eq!(
            result.blocking_status,
            Some(AutomaticAnalysisStatus::RuleInputsIncomplete)
        );
        assert_eq!(result.missing_input_ids, vec!["allowable_stress"]);
        assert!(result.findings.iter().any(|finding| {
            finding.code == FindingCode::RuleInputMissing && finding.is_blocking()
        }));
        assert!(result
            .preserved_statuses
            .contains(&AutomaticAnalysisStatus::MechanicsSolved));
        assert!(result
            .preserved_statuses
            .contains(&AutomaticAnalysisStatus::RuleInputsIncomplete));
    }

    #[test]
    fn not_provided_value_record_blocks_rule_check_status() {
        let result = check_completeness(&CompletenessCheckInput {
            required_inputs: vec![required_stress_input()],
            supplied_inputs: vec![SuppliedInput::missing("allowable_stress")],
            current_statuses: vec![],
        });

        assert_eq!(result.readiness, Readiness::Blocked);
        assert!(result.findings.iter().any(|finding| {
            finding.code == FindingCode::RuleInputMissing && finding.is_blocking()
        }));
    }

    #[test]
    fn missing_unit_and_dimension_mismatch_are_blocking() {
        let mut supplied =
            SuppliedInput::private_reviewed("allowable_stress", Dimension::Force, "");
        supplied.unit_ref = None;

        let result = check_completeness(&CompletenessCheckInput {
            required_inputs: vec![required_stress_input()],
            supplied_inputs: vec![supplied],
            current_statuses: vec![],
        });

        assert_eq!(result.readiness, Readiness::Blocked);
        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == FindingCode::RuleUnitMissing));
        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == FindingCode::RuleUnitMismatch));
    }

    #[test]
    fn provenance_and_redistribution_gaps_are_explicit_blocking_findings() {
        let mut supplied =
            SuppliedInput::private_reviewed("allowable_stress", Dimension::Stress, "MPa");
        supplied.provenance_status = ProvenanceStatus::Unknown;
        supplied.redistribution_status = RedistributionStatus::Unknown;

        let result = check_completeness(&CompletenessCheckInput {
            required_inputs: vec![required_stress_input()],
            supplied_inputs: vec![supplied],
            current_statuses: vec![],
        });

        assert_eq!(result.readiness, Readiness::Blocked);
        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == FindingCode::RuleProvenanceWarning));
        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == FindingCode::RuleRedistributionWarning));
    }

    #[test]
    fn protected_content_suspicion_blocks_rule_check() {
        let mut supplied =
            SuppliedInput::private_reviewed("allowable_stress", Dimension::Stress, "MPa");
        supplied.value_status = ValueStatus::ProtectedSuspected;
        supplied.provenance_status = ProvenanceStatus::ProtectedSuspected;

        let result = check_completeness(&CompletenessCheckInput {
            required_inputs: vec![required_stress_input()],
            supplied_inputs: vec![supplied],
            current_statuses: vec![],
        });

        assert_eq!(result.readiness, Readiness::Blocked);
        assert!(result.findings.iter().any(|finding| {
            finding.code == FindingCode::RuleProtectedContentWarning && finding.is_blocking()
        }));
    }

    #[test]
    fn pending_review_blocks_rule_check_without_silent_acceptance() {
        let mut supplied =
            SuppliedInput::private_reviewed("allowable_stress", Dimension::Stress, "MPa");
        supplied.review_status = ReviewStatus::Pending;

        let result = check_completeness(&CompletenessCheckInput {
            required_inputs: vec![required_stress_input()],
            supplied_inputs: vec![supplied],
            current_statuses: vec![],
        });

        assert_eq!(result.readiness, Readiness::Blocked);
        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == FindingCode::RuleReviewWarning));
    }

    #[test]
    fn duplicate_declarations_and_duplicate_supplied_inputs_are_blocking() {
        let result = check_completeness(&CompletenessCheckInput {
            required_inputs: vec![required_stress_input(), required_stress_input()],
            supplied_inputs: vec![
                SuppliedInput::private_reviewed("allowable_stress", Dimension::Stress, "MPa"),
                SuppliedInput::private_reviewed("allowable_stress", Dimension::Stress, "MPa"),
            ],
            current_statuses: vec![],
        });

        assert_eq!(result.readiness, Readiness::Blocked);
        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == FindingCode::DuplicateRequiredInput));
        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == FindingCode::DuplicateSuppliedInput));
    }

    #[test]
    fn unexpected_supplied_input_is_warning_not_blocking() {
        let result = check_completeness(&CompletenessCheckInput {
            required_inputs: vec![],
            supplied_inputs: vec![SuppliedInput::private_reviewed(
                "extra_value",
                Dimension::Stress,
                "MPa",
            )],
            current_statuses: vec![],
        });

        assert_eq!(result.readiness, Readiness::ReadyForRuleCheck);
        assert!(result.findings.iter().any(|finding| {
            finding.code == FindingCode::UnexpectedSuppliedInput
                && finding.severity == FindingSeverity::Warning
        }));
    }

    #[test]
    fn reporting_only_required_input_does_not_block_rule_check() {
        let mut reporting = required_stress_input();
        reporting.required_for = RequiredFor::Reporting;

        let result = check_completeness(&CompletenessCheckInput {
            required_inputs: vec![reporting],
            supplied_inputs: vec![],
            current_statuses: vec![],
        });

        assert_eq!(result.readiness, Readiness::ReadyForRuleCheck);
        assert!(result.missing_input_ids.is_empty());
    }

    #[test]
    fn automatic_statuses_exclude_professional_or_code_compliance_claims() {
        let automatic_statuses = [
            AutomaticAnalysisStatus::ModelIncomplete,
            AutomaticAnalysisStatus::MechanicsSolved,
            AutomaticAnalysisStatus::RuleInputsIncomplete,
            AutomaticAnalysisStatus::UserRuleChecked,
            AutomaticAnalysisStatus::UserRuleFailed,
            AutomaticAnalysisStatus::HumanReviewRequired,
        ];

        let rendered = format!("{automatic_statuses:?}");
        assert!(!rendered.contains("CodeCompliant"));
        assert!(!rendered.contains("Certified"));
        assert!(!rendered.contains("Sealed"));
        assert!(!rendered.contains("HumanApprovedForProject"));
    }
}
