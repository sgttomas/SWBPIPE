//! Code-neutral load case algebra.
//!
//! This crate combines explicit mechanics quantities and result states. It does
//! not encode design-code load combinations, protected standards content,
//! proprietary project data, rule-pack expression evaluation, or professional
//! approval.

use open_pipe_stress_primitive_loads::{
    BoundaryMetadataError, BoundaryQuantityRecord, BoundaryRecordRef, CanonicalSchemaBinding,
    LoadDimension, QuantityUnitMetadata,
};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnalysisStatus {
    ModelIncomplete,
    MechanicsSolved,
    RuleInputsIncomplete,
    UserRuleChecked,
    UserRuleFailed,
    HumanReviewRequired,
    HumanApprovedForProject,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlgebraOperation {
    LinearCombination,
    ResultStateSubtraction,
    RangeEnvelope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RangeMode {
    Min,
    Max,
    MinAbs,
    MaxAbs,
}

impl RangeMode {
    /// Canonical document token for this mode; the inverse of
    /// [`RangeMode::parse_token`]. Document carriers (preview models,
    /// operation payloads) must use exactly these tokens.
    pub fn token(self) -> &'static str {
        match self {
            Self::Min => "min",
            Self::Max => "max",
            Self::MinAbs => "min_abs",
            Self::MaxAbs => "max_abs",
        }
    }

    /// Parse a canonical document token; unknown tokens return `None` so the
    /// caller can emit a named diagnostic instead of guessing a mode.
    pub fn parse_token(token: &str) -> Option<Self> {
        match token {
            "min" => Some(Self::Min),
            "max" => Some(Self::Max),
            "min_abs" => Some(Self::MinAbs),
            "max_abs" => Some(Self::MaxAbs),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AlgebraQuantity {
    pub value: f64,
    pub dimension: LoadDimension,
}

impl AlgebraQuantity {
    pub fn new(value: f64, dimension: LoadDimension) -> Result<Self, AlgebraError> {
        validate_finite("quantity", value)?;
        Ok(Self { value, dimension })
    }

    pub fn to_boundary_record(
        &self,
        record: BoundaryRecordRef,
        unit: QuantityUnitMetadata,
        provenance_ref: impl Into<String>,
    ) -> Result<BoundaryQuantityRecord, BoundaryMetadataError> {
        BoundaryQuantityRecord::from_load_dimension(
            record,
            self.value,
            self.dimension,
            unit,
            provenance_ref,
        )
    }

    fn scaled(&self, factor: f64) -> Self {
        Self {
            value: self.value * factor,
            dimension: self.dimension,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AlgebraOperand {
    pub operand_id: String,
    pub state_id: String,
    pub quantity: AlgebraQuantity,
    pub statuses: Vec<AnalysisStatus>,
}

impl AlgebraOperand {
    pub fn new(
        operand_id: impl Into<String>,
        state_id: impl Into<String>,
        quantity: AlgebraQuantity,
        statuses: Vec<AnalysisStatus>,
    ) -> Self {
        Self {
            operand_id: operand_id.into(),
            state_id: state_id.into(),
            quantity,
            statuses,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CombinationTerm {
    pub operand_id: String,
    pub factor: f64,
}

impl CombinationTerm {
    pub fn new(operand_id: impl Into<String>, factor: f64) -> Result<Self, AlgebraError> {
        validate_finite("combination factor", factor)?;
        Ok(Self {
            operand_id: operand_id.into(),
            factor,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AlgebraExpression {
    LinearCombination {
        terms: Vec<CombinationTerm>,
    },
    ResultStateSubtraction {
        minuend_id: String,
        subtrahend_id: String,
    },
    RangeEnvelope {
        operand_ids: Vec<String>,
        mode: RangeMode,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindingCode {
    MissingOperand,
    EmptyExpression,
    NonFiniteFactor,
    DimensionMismatch,
    DuplicateOperand,
    UnsupportedExpressionShape,
    MissingResultState,
    StatusBoundaryViolation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AlgebraFinding {
    pub code: FindingCode,
    pub subject_id: String,
    pub message: String,
}

impl AlgebraFinding {
    fn new(code: FindingCode, subject_id: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code,
            subject_id: subject_id.into(),
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AlgebraResult {
    pub operation: AlgebraOperation,
    pub quantity: Option<AlgebraQuantity>,
    pub statuses: Vec<AnalysisStatus>,
    pub source_operand_ids: Vec<String>,
    pub findings: Vec<AlgebraFinding>,
}

impl AlgebraResult {
    pub fn is_blocked(&self) -> bool {
        !self.findings.is_empty()
    }

    pub fn to_result_boundary_record(
        &self,
        record: BoundaryRecordRef,
        unit: QuantityUnitMetadata,
        provenance_ref: impl Into<String>,
    ) -> Result<Option<BoundaryQuantityRecord>, BoundaryMetadataError> {
        require_result_schema_binding(record.schema_binding)?;
        record.validate()?;
        unit.validate()?;
        let provenance_ref = provenance_ref.into();
        validate_boundary_text("provenance_ref", &provenance_ref)?;
        match &self.quantity {
            Some(quantity) => quantity
                .to_boundary_record(record, unit, provenance_ref)
                .map(Some),
            None => Ok(None),
        }
    }
}

fn require_result_schema_binding(
    actual: CanonicalSchemaBinding,
) -> Result<(), BoundaryMetadataError> {
    match actual {
        CanonicalSchemaBinding::ModelResultValue
        | CanonicalSchemaBinding::ResultsQuantityResult => Ok(()),
        CanonicalSchemaBinding::ModelLoadCase | CanonicalSchemaBinding::ModelLoadRecord => {
            Err(BoundaryMetadataError::SchemaBindingMismatch {
                expected: CanonicalSchemaBinding::ModelResultValue,
                actual,
            })
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AlgebraError {
    NonFiniteInput { name: &'static str, value: f64 },
}

impl fmt::Display for AlgebraError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteInput { name, value } => {
                write!(f, "{name} must be finite, got {value}")
            }
        }
    }
}

impl Error for AlgebraError {}

pub fn evaluate_expression(
    operands: &[AlgebraOperand],
    expression: &AlgebraExpression,
) -> AlgebraResult {
    let mut operand_by_id: HashMap<&str, &AlgebraOperand> = HashMap::new();
    let mut seen_operands = HashSet::new();
    let mut input_findings = Vec::new();
    for operand in operands {
        if !seen_operands.insert(operand.operand_id.as_str()) {
            input_findings.push(AlgebraFinding::new(
                FindingCode::DuplicateOperand,
                &operand.operand_id,
                "operand input contains a duplicate operand identifier",
            ));
            continue;
        }
        operand_by_id.insert(operand.operand_id.as_str(), operand);
    }

    let mut result = match expression {
        AlgebraExpression::LinearCombination { terms } => {
            evaluate_linear_combination(&operand_by_id, terms)
        }
        AlgebraExpression::ResultStateSubtraction {
            minuend_id,
            subtrahend_id,
        } => evaluate_result_state_subtraction(&operand_by_id, minuend_id, subtrahend_id),
        AlgebraExpression::RangeEnvelope { operand_ids, mode } => {
            evaluate_range_envelope(&operand_by_id, operand_ids, *mode)
        }
    };
    if !input_findings.is_empty() {
        result.quantity = None;
        result.findings.extend(input_findings);
    }
    result
}

pub fn evaluate_linear_combination(
    operand_by_id: &HashMap<&str, &AlgebraOperand>,
    terms: &[CombinationTerm],
) -> AlgebraResult {
    let mut findings = Vec::new();
    if terms.is_empty() {
        findings.push(AlgebraFinding::new(
            FindingCode::EmptyExpression,
            "linear_combination",
            "linear combination requires at least one term",
        ));
    }

    let mut seen = HashSet::new();
    let mut source_operand_ids = Vec::new();
    let mut statuses = Vec::new();
    let mut dimension = None;
    let mut value = 0.0;

    for term in terms {
        if !term.factor.is_finite() {
            findings.push(AlgebraFinding::new(
                FindingCode::NonFiniteFactor,
                &term.operand_id,
                "combination factor must be finite",
            ));
            continue;
        }
        if !seen.insert(term.operand_id.as_str()) {
            findings.push(AlgebraFinding::new(
                FindingCode::DuplicateOperand,
                &term.operand_id,
                "linear combination contains a duplicate operand",
            ));
            continue;
        }
        let Some(operand) = operand_by_id.get(term.operand_id.as_str()) else {
            findings.push(AlgebraFinding::new(
                FindingCode::MissingOperand,
                &term.operand_id,
                "linear combination operand is missing",
            ));
            continue;
        };
        collect_statuses(operand, &mut statuses, &mut findings);
        source_operand_ids.push(operand.operand_id.clone());
        if !same_dimension_or_record(operand, &mut dimension, &mut findings) {
            continue;
        }
        value += operand.quantity.scaled(term.factor).value;
    }

    finalize_result(
        AlgebraOperation::LinearCombination,
        dimension.map(|dim| AlgebraQuantity {
            value,
            dimension: dim,
        }),
        statuses,
        source_operand_ids,
        findings,
    )
}

pub fn evaluate_result_state_subtraction(
    operand_by_id: &HashMap<&str, &AlgebraOperand>,
    minuend_id: &str,
    subtrahend_id: &str,
) -> AlgebraResult {
    let mut findings = Vec::new();
    if minuend_id == subtrahend_id {
        findings.push(AlgebraFinding::new(
            FindingCode::DuplicateOperand,
            minuend_id,
            "subtraction requires two distinct result states",
        ));
    }

    let mut statuses = Vec::new();
    let mut source_operand_ids = Vec::new();
    let minuend = operand_by_id.get(minuend_id).copied();
    let subtrahend = operand_by_id.get(subtrahend_id).copied();
    let mut missing_checked = HashSet::new();

    for operand_id in [minuend_id, subtrahend_id] {
        if missing_checked.insert(operand_id) && !operand_by_id.contains_key(operand_id) {
            findings.push(AlgebraFinding::new(
                FindingCode::MissingResultState,
                operand_id,
                "subtraction result state is missing",
            ));
        }
    }

    let quantity = match (minuend, subtrahend) {
        (Some(left), Some(right)) => {
            collect_statuses(left, &mut statuses, &mut findings);
            collect_statuses(right, &mut statuses, &mut findings);
            source_operand_ids.push(left.operand_id.clone());
            source_operand_ids.push(right.operand_id.clone());
            if left.quantity.dimension == right.quantity.dimension {
                Some(AlgebraQuantity {
                    value: left.quantity.value - right.quantity.value,
                    dimension: left.quantity.dimension,
                })
            } else {
                findings.push(AlgebraFinding::new(
                    FindingCode::DimensionMismatch,
                    format!("{}:{}", left.operand_id, right.operand_id),
                    "subtraction requires compatible dimensions",
                ));
                None
            }
        }
        _ => None,
    };

    finalize_result(
        AlgebraOperation::ResultStateSubtraction,
        quantity,
        statuses,
        source_operand_ids,
        findings,
    )
}

pub fn evaluate_range_envelope(
    operand_by_id: &HashMap<&str, &AlgebraOperand>,
    operand_ids: &[String],
    mode: RangeMode,
) -> AlgebraResult {
    let mut findings = Vec::new();
    if operand_ids.is_empty() {
        findings.push(AlgebraFinding::new(
            FindingCode::EmptyExpression,
            "range_envelope",
            "range envelope requires at least one operand",
        ));
    }

    let mut seen = HashSet::new();
    let mut statuses = Vec::new();
    let mut source_operand_ids = Vec::new();
    let mut dimension = None;
    let mut selected: Option<(&str, AlgebraQuantity)> = None;
    let mut ordered_operand_ids: Vec<&str> = operand_ids.iter().map(String::as_str).collect();
    ordered_operand_ids.sort_unstable();

    for operand_id in ordered_operand_ids {
        if !seen.insert(operand_id) {
            findings.push(AlgebraFinding::new(
                FindingCode::DuplicateOperand,
                operand_id,
                "range envelope contains a duplicate operand",
            ));
            continue;
        }
        let Some(operand) = operand_by_id.get(operand_id) else {
            findings.push(AlgebraFinding::new(
                FindingCode::MissingOperand,
                operand_id,
                "range envelope operand is missing",
            ));
            continue;
        };
        collect_statuses(operand, &mut statuses, &mut findings);
        source_operand_ids.push(operand.operand_id.clone());
        if !same_dimension_or_record(operand, &mut dimension, &mut findings) {
            continue;
        }
        selected = Some(match selected {
            Some((current_id, current))
                if prefer_current(
                    current_id,
                    current.value,
                    operand.operand_id.as_str(),
                    operand.quantity.value,
                    mode,
                ) =>
            {
                (current_id, current)
            }
            _ => (operand.operand_id.as_str(), operand.quantity.clone()),
        });
    }

    finalize_result(
        AlgebraOperation::RangeEnvelope,
        selected.map(|(_, quantity)| quantity),
        statuses,
        source_operand_ids,
        findings,
    )
}

fn prefer_current(
    current_id: &str,
    current: f64,
    candidate_id: &str,
    candidate: f64,
    mode: RangeMode,
) -> bool {
    let current_is_better = match mode {
        RangeMode::Min => current < candidate,
        RangeMode::Max => current > candidate,
        RangeMode::MinAbs => current.abs() < candidate.abs(),
        RangeMode::MaxAbs => current.abs() > candidate.abs(),
    };
    let candidate_is_better = match mode {
        RangeMode::Min => candidate < current,
        RangeMode::Max => candidate > current,
        RangeMode::MinAbs => candidate.abs() < current.abs(),
        RangeMode::MaxAbs => candidate.abs() > current.abs(),
    };

    if current_is_better {
        true
    } else if candidate_is_better {
        false
    } else {
        current_id <= candidate_id
    }
}

fn same_dimension_or_record(
    operand: &AlgebraOperand,
    dimension: &mut Option<LoadDimension>,
    findings: &mut Vec<AlgebraFinding>,
) -> bool {
    match dimension {
        Some(expected) if *expected != operand.quantity.dimension => {
            findings.push(AlgebraFinding::new(
                FindingCode::DimensionMismatch,
                &operand.operand_id,
                "algebra operands must have compatible dimensions",
            ));
            false
        }
        Some(_) => true,
        None => {
            *dimension = Some(operand.quantity.dimension);
            true
        }
    }
}

fn collect_statuses(
    operand: &AlgebraOperand,
    statuses: &mut Vec<AnalysisStatus>,
    findings: &mut Vec<AlgebraFinding>,
) {
    if operand.statuses.is_empty() {
        findings.push(AlgebraFinding::new(
            FindingCode::StatusBoundaryViolation,
            &operand.operand_id,
            "algebra operands require explicit analysis status metadata",
        ));
    }

    for status in &operand.statuses {
        if !automatic_status_allowed(*status) {
            findings.push(AlgebraFinding::new(
                FindingCode::StatusBoundaryViolation,
                &operand.operand_id,
                "load-case algebra cannot emit human approval or compliance status",
            ));
            continue;
        }
        if !statuses.contains(status) {
            statuses.push(*status);
        }
    }

    if operand.statuses.contains(&AnalysisStatus::ModelIncomplete) {
        findings.push(AlgebraFinding::new(
            FindingCode::StatusBoundaryViolation,
            &operand.operand_id,
            "algebra operands with incomplete model status cannot produce an algebra output",
        ));
    }
}

fn finalize_result(
    operation: AlgebraOperation,
    quantity: Option<AlgebraQuantity>,
    mut statuses: Vec<AnalysisStatus>,
    source_operand_ids: Vec<String>,
    findings: Vec<AlgebraFinding>,
) -> AlgebraResult {
    if !statuses.contains(&AnalysisStatus::HumanReviewRequired) {
        statuses.push(AnalysisStatus::HumanReviewRequired);
    }
    sort_statuses(&mut statuses);
    AlgebraResult {
        operation,
        quantity: if findings.is_empty() { quantity } else { None },
        statuses,
        source_operand_ids,
        findings,
    }
}

fn sort_statuses(statuses: &mut Vec<AnalysisStatus>) {
    statuses.sort_by_key(|status| status_rank(*status));
    statuses.dedup();
}

fn status_rank(status: AnalysisStatus) -> u8 {
    match status {
        AnalysisStatus::ModelIncomplete => 0,
        AnalysisStatus::MechanicsSolved => 1,
        AnalysisStatus::RuleInputsIncomplete => 2,
        AnalysisStatus::UserRuleChecked => 3,
        AnalysisStatus::UserRuleFailed => 4,
        AnalysisStatus::HumanReviewRequired => 5,
        AnalysisStatus::HumanApprovedForProject => 6,
    }
}

fn automatic_status_allowed(status: AnalysisStatus) -> bool {
    matches!(
        status,
        AnalysisStatus::ModelIncomplete
            | AnalysisStatus::MechanicsSolved
            | AnalysisStatus::RuleInputsIncomplete
            | AnalysisStatus::UserRuleChecked
            | AnalysisStatus::UserRuleFailed
            | AnalysisStatus::HumanReviewRequired
    )
}

fn validate_finite(name: &'static str, value: f64) -> Result<(), AlgebraError> {
    if !value.is_finite() {
        return Err(AlgebraError::NonFiniteInput { name, value });
    }
    Ok(())
}

fn validate_boundary_text(field: &'static str, value: &str) -> Result<(), BoundaryMetadataError> {
    if value.trim().is_empty() || value.trim() == "TBD" {
        return Err(BoundaryMetadataError::MissingField { field });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn q(value: f64, dimension: LoadDimension) -> AlgebraQuantity {
        AlgebraQuantity::new(value, dimension).unwrap()
    }

    fn operand(id: &str, state: &str, value: f64, dimension: LoadDimension) -> AlgebraOperand {
        AlgebraOperand::new(
            id,
            state,
            q(value, dimension),
            vec![AnalysisStatus::MechanicsSolved],
        )
    }

    fn boundary_ref(schema_binding: CanonicalSchemaBinding) -> BoundaryRecordRef {
        BoundaryRecordRef::new(
            "result:algebra:1",
            schema_binding,
            "node:1",
            "combination:sustain-plus-thermal",
            "payload:result-envelope",
            "hash:result-envelope",
        )
        .unwrap()
    }

    fn force_unit() -> QuantityUnitMetadata {
        QuantityUnitMetadata::new(
            "N",
            open_pipe_stress_primitive_loads::CanonicalDimension::Force,
            "unit-system:si",
        )
        .unwrap()
    }

    #[test]
    fn linear_combination_sums_compatible_quantities() {
        let operands = vec![
            operand("sustain", "sustain", 10.0, LoadDimension::Force),
            operand("thermal", "thermal", 2.5, LoadDimension::Force),
        ];
        let terms = vec![
            CombinationTerm::new("sustain", 1.0).unwrap(),
            CombinationTerm::new("thermal", 1.5).unwrap(),
        ];

        let result =
            evaluate_expression(&operands, &AlgebraExpression::LinearCombination { terms });

        assert!(!result.is_blocked());
        assert_eq!(result.quantity.unwrap().value, 13.75);
        assert!(result.statuses.contains(&AnalysisStatus::MechanicsSolved));
        assert!(result
            .statuses
            .contains(&AnalysisStatus::HumanReviewRequired));
    }

    #[test]
    fn algebra_result_exports_explicit_unit_and_result_envelope_metadata() {
        let operands = vec![
            operand("sustain", "sustain", 10.0, LoadDimension::Force),
            operand("thermal", "thermal", 2.0, LoadDimension::Force),
        ];
        let terms = vec![
            CombinationTerm::new("sustain", 1.0).unwrap(),
            CombinationTerm::new("thermal", 2.0).unwrap(),
        ];

        let result =
            evaluate_expression(&operands, &AlgebraExpression::LinearCombination { terms });
        let record = result
            .to_result_boundary_record(
                boundary_ref(CanonicalSchemaBinding::ResultsQuantityResult),
                force_unit(),
                "provenance:algebra",
            )
            .unwrap()
            .unwrap();

        assert_eq!(record.value, 14.0);
        assert_eq!(record.unit.unit, "N");
        assert_eq!(
            record.record.schema_binding.schema_ref(),
            "schemas/results.schema.yaml#/$defs/QuantityResult"
        );
        assert!(record
            .round_trip_key()
            .contains("payload_hash_ref=hash:result-envelope"));
    }

    #[test]
    fn algebra_result_boundary_rejects_load_schema_binding() {
        let operands = vec![operand("sustain", "sustain", 10.0, LoadDimension::Force)];
        let terms = vec![CombinationTerm::new("sustain", 1.0).unwrap()];
        let result =
            evaluate_expression(&operands, &AlgebraExpression::LinearCombination { terms });

        let err = result
            .to_result_boundary_record(
                boundary_ref(CanonicalSchemaBinding::ModelLoadRecord),
                force_unit(),
                "provenance:algebra",
            )
            .unwrap_err();

        assert_eq!(
            err,
            BoundaryMetadataError::SchemaBindingMismatch {
                expected: CanonicalSchemaBinding::ModelResultValue,
                actual: CanonicalSchemaBinding::ModelLoadRecord,
            }
        );
    }

    #[test]
    fn algebra_result_boundary_rejects_load_case_schema_binding() {
        let operands = vec![operand("sustain", "sustain", 10.0, LoadDimension::Force)];
        let terms = vec![CombinationTerm::new("sustain", 1.0).unwrap()];
        let result =
            evaluate_expression(&operands, &AlgebraExpression::LinearCombination { terms });

        let err = result
            .to_result_boundary_record(
                boundary_ref(CanonicalSchemaBinding::ModelLoadCase),
                force_unit(),
                "provenance:algebra",
            )
            .unwrap_err();

        assert_eq!(
            err,
            BoundaryMetadataError::SchemaBindingMismatch {
                expected: CanonicalSchemaBinding::ModelResultValue,
                actual: CanonicalSchemaBinding::ModelLoadCase,
            }
        );
    }

    #[test]
    fn algebra_result_boundary_rejects_incompatible_unit_dimension() {
        let operands = vec![operand("sustain", "sustain", 10.0, LoadDimension::Force)];
        let terms = vec![CombinationTerm::new("sustain", 1.0).unwrap()];
        let result =
            evaluate_expression(&operands, &AlgebraExpression::LinearCombination { terms });
        let moment_unit = QuantityUnitMetadata::new(
            "N-m",
            open_pipe_stress_primitive_loads::CanonicalDimension::Moment,
            "unit-system:si",
        )
        .unwrap();

        let err = result
            .to_result_boundary_record(
                boundary_ref(CanonicalSchemaBinding::ResultsQuantityResult),
                moment_unit,
                "provenance:algebra",
            )
            .unwrap_err();

        assert_eq!(
            err,
            BoundaryMetadataError::DimensionMismatch {
                expected: open_pipe_stress_primitive_loads::CanonicalDimension::Force,
                actual: open_pipe_stress_primitive_loads::CanonicalDimension::Moment,
            }
        );
    }

    #[test]
    fn blocked_result_export_still_validates_unit_metadata() {
        let terms = vec![CombinationTerm::new("missing", 1.0).unwrap()];
        let result = evaluate_expression(&[], &AlgebraExpression::LinearCombination { terms });
        let bad_unit = QuantityUnitMetadata {
            unit: "".to_string(),
            dimension: open_pipe_stress_primitive_loads::CanonicalDimension::Force,
            unit_system_ref: "unit-system:si".to_string(),
        };

        let err = result
            .to_result_boundary_record(
                boundary_ref(CanonicalSchemaBinding::ResultsQuantityResult),
                bad_unit,
                "provenance:algebra",
            )
            .unwrap_err();

        assert_eq!(err, BoundaryMetadataError::MissingField { field: "unit" });
    }

    #[test]
    fn subtraction_requires_distinct_compatible_result_states() {
        let operands = vec![
            operand("hot", "operating", 42.0, LoadDimension::Displacement),
            operand("cold", "installed", 10.0, LoadDimension::Displacement),
        ];

        let result = evaluate_expression(
            &operands,
            &AlgebraExpression::ResultStateSubtraction {
                minuend_id: "hot".to_string(),
                subtrahend_id: "cold".to_string(),
            },
        );

        assert!(!result.is_blocked());
        assert_eq!(result.quantity.unwrap().value, 32.0);
    }

    #[test]
    fn subtraction_reports_duplicate_and_missing_state_boundaries() {
        let result = evaluate_expression(
            &[],
            &AlgebraExpression::ResultStateSubtraction {
                minuend_id: "missing".to_string(),
                subtrahend_id: "missing".to_string(),
            },
        );

        assert!(result.is_blocked());
        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == FindingCode::DuplicateOperand));
        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == FindingCode::MissingResultState));
    }

    #[test]
    fn range_envelope_selects_extreme_value() {
        let operands = vec![
            operand("case-a", "a", -4.0, LoadDimension::Moment),
            operand("case-b", "b", 8.0, LoadDimension::Moment),
            operand("case-c", "c", -11.0, LoadDimension::Moment),
        ];
        let ids = vec![
            "case-a".to_string(),
            "case-b".to_string(),
            "case-c".to_string(),
        ];

        let result = evaluate_expression(
            &operands,
            &AlgebraExpression::RangeEnvelope {
                operand_ids: ids,
                mode: RangeMode::MaxAbs,
            },
        );

        assert!(!result.is_blocked());
        assert_eq!(result.quantity.unwrap().value, -11.0);
    }

    #[test]
    fn range_envelope_ties_are_ordered_by_operand_id() {
        let operands = vec![
            operand("zeta", "z", -10.0, LoadDimension::Moment),
            operand("alpha", "a", 10.0, LoadDimension::Moment),
        ];
        let ids = vec!["zeta".to_string(), "alpha".to_string()];

        let result = evaluate_expression(
            &operands,
            &AlgebraExpression::RangeEnvelope {
                operand_ids: ids,
                mode: RangeMode::MaxAbs,
            },
        );

        assert!(!result.is_blocked());
        assert_eq!(result.source_operand_ids, vec!["alpha", "zeta"]);
        assert_eq!(result.quantity.unwrap().value, 10.0);
    }

    #[test]
    fn dimension_mismatch_blocks_result() {
        let operands = vec![
            operand("force", "a", 1.0, LoadDimension::Force),
            operand("moment", "b", 2.0, LoadDimension::Moment),
        ];
        let terms = vec![
            CombinationTerm::new("force", 1.0).unwrap(),
            CombinationTerm::new("moment", 1.0).unwrap(),
        ];

        let result =
            evaluate_expression(&operands, &AlgebraExpression::LinearCombination { terms });

        assert!(result.is_blocked());
        assert_eq!(result.quantity, None);
        assert_eq!(result.findings[0].code, FindingCode::DimensionMismatch);
    }

    #[test]
    fn missing_and_duplicate_operands_are_findings() {
        let operands = vec![operand("case-a", "a", 1.0, LoadDimension::Force)];
        let terms = vec![
            CombinationTerm::new("case-a", 1.0).unwrap(),
            CombinationTerm::new("case-a", 2.0).unwrap(),
            CombinationTerm::new("missing", 1.0).unwrap(),
        ];

        let result =
            evaluate_expression(&operands, &AlgebraExpression::LinearCombination { terms });

        assert!(result.is_blocked());
        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == FindingCode::DuplicateOperand));
        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == FindingCode::MissingOperand));
    }

    #[test]
    fn range_mode_tokens_round_trip() {
        for mode in [
            RangeMode::Min,
            RangeMode::Max,
            RangeMode::MinAbs,
            RangeMode::MaxAbs,
        ] {
            assert_eq!(RangeMode::parse_token(mode.token()), Some(mode));
        }
        assert_eq!(RangeMode::parse_token("MAX"), None);
        assert_eq!(RangeMode::parse_token("maxabs"), None);
        assert_eq!(RangeMode::parse_token(""), None);
    }

    #[test]
    fn non_finite_factor_is_rejected_at_boundary() {
        let err = CombinationTerm::new("case-a", f64::NAN).unwrap_err();
        match err {
            AlgebraError::NonFiniteInput { name, value } => {
                assert_eq!(name, "combination factor");
                assert!(value.is_nan());
            }
        }
    }

    #[test]
    fn status_propagation_preserves_rule_findings_without_human_approval() {
        let operands = vec![AlgebraOperand::new(
            "rule-case",
            "rule",
            q(5.0, LoadDimension::Force),
            vec![
                AnalysisStatus::MechanicsSolved,
                AnalysisStatus::RuleInputsIncomplete,
            ],
        )];
        let terms = vec![CombinationTerm::new("rule-case", 1.0).unwrap()];

        let result =
            evaluate_expression(&operands, &AlgebraExpression::LinearCombination { terms });

        assert!(!result.is_blocked());
        assert!(result
            .statuses
            .contains(&AnalysisStatus::RuleInputsIncomplete));
        assert!(result
            .statuses
            .contains(&AnalysisStatus::HumanReviewRequired));
    }

    #[test]
    fn missing_status_metadata_blocks_algebra_output() {
        let operands = vec![AlgebraOperand::new(
            "case-a",
            "draft",
            q(5.0, LoadDimension::Force),
            Vec::new(),
        )];
        let terms = vec![CombinationTerm::new("case-a", 1.0).unwrap()];

        let result =
            evaluate_expression(&operands, &AlgebraExpression::LinearCombination { terms });

        assert!(result.is_blocked());
        assert_eq!(result.quantity, None);
        assert!(result.findings.iter().any(|finding| finding.code
            == FindingCode::StatusBoundaryViolation
            && finding.subject_id == "case-a"));
        assert_eq!(result.statuses, vec![AnalysisStatus::HumanReviewRequired]);
    }

    #[test]
    fn model_incomplete_status_blocks_algebra_output() {
        let operands = vec![AlgebraOperand::new(
            "incomplete",
            "draft",
            q(5.0, LoadDimension::Force),
            vec![AnalysisStatus::ModelIncomplete],
        )];
        let terms = vec![CombinationTerm::new("incomplete", 1.0).unwrap()];

        let result =
            evaluate_expression(&operands, &AlgebraExpression::LinearCombination { terms });

        assert!(result.is_blocked());
        assert_eq!(result.quantity, None);
        assert_eq!(
            result.statuses,
            vec![
                AnalysisStatus::ModelIncomplete,
                AnalysisStatus::HumanReviewRequired
            ]
        );
        assert!(result.findings.iter().any(|finding| finding.code
            == FindingCode::StatusBoundaryViolation
            && finding.subject_id == "incomplete"));
    }

    #[test]
    fn human_approval_status_blocks_automatic_algebra_output() {
        let operands = vec![AlgebraOperand::new(
            "accepted",
            "external-review",
            q(5.0, LoadDimension::Force),
            vec![AnalysisStatus::HumanApprovedForProject],
        )];
        let terms = vec![CombinationTerm::new("accepted", 1.0).unwrap()];

        let result =
            evaluate_expression(&operands, &AlgebraExpression::LinearCombination { terms });

        assert!(result.is_blocked());
        assert_eq!(result.quantity, None);
        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == FindingCode::StatusBoundaryViolation));
        assert!(!result
            .statuses
            .contains(&AnalysisStatus::HumanApprovedForProject));
    }
}
