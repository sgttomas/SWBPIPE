//! Code-neutral mechanics stress recovery.
//!
//! This crate recovers open mechanics stress components from explicit
//! resultants and section properties. It does not encode design-code stress
//! equations, allowables, stress indices, SIF/flexibility tables, protected
//! standards content, proprietary data, rule-pack checks, or professional
//! approval.

use open_pipe_stress_primitive_loads::{
    BoundaryMetadataError, BoundaryQuantityRecord, BoundaryRecordRef, CanonicalDimension,
    CanonicalSchemaBinding, QuantityUnitMetadata,
};
use open_pipe_stress_straight_pipe::StationResultants;
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
pub enum FindingCode {
    MissingResultant,
    MissingSectionProperty,
    MissingPressureInput,
    NonFiniteInput,
    NonPositiveInput,
    IncompleteMechanicsStatus,
    StatusBoundaryViolation,
    BlockedStressState,
    MissingUnitMetadata,
    InvalidUnitMetadata,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StressFinding {
    pub code: FindingCode,
    pub subject_id: String,
    pub message: String,
}

impl StressFinding {
    fn new(code: FindingCode, subject_id: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code,
            subject_id: subject_id.into(),
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForceResultants {
    pub axial_force: Option<f64>,
    pub bending_moment_y: Option<f64>,
    pub bending_moment_z: Option<f64>,
    pub torsional_moment: Option<f64>,
}

impl ForceResultants {
    pub fn new(
        axial_force: Option<f64>,
        bending_moment_y: Option<f64>,
        bending_moment_z: Option<f64>,
        torsional_moment: Option<f64>,
    ) -> Self {
        Self {
            axial_force,
            bending_moment_y,
            bending_moment_z,
            torsional_moment,
        }
    }

    pub fn from_element_end_resultants(
        axial_force: f64,
        bending_moment_y: f64,
        bending_moment_z: f64,
        torsional_moment: f64,
    ) -> Result<Self, StressRecoveryError> {
        Ok(Self {
            axial_force: Some(checked_finite("axial_force", axial_force)?),
            bending_moment_y: Some(checked_finite("bending_moment_y", bending_moment_y)?),
            bending_moment_z: Some(checked_finite("bending_moment_z", bending_moment_z)?),
            torsional_moment: Some(checked_finite("torsional_moment", torsional_moment)?),
        })
    }

    pub fn from_station_resultants(
        resultants: &StationResultants,
    ) -> Result<Self, StressRecoveryError> {
        checked_station_fraction(resultants.station_fraction)?;
        checked_finite("station_distance_from_i", resultants.distance_from_i)?;
        checked_finite("station_shear_force_y", resultants.shear_force_y)?;
        checked_finite("station_shear_force_z", resultants.shear_force_z)?;
        Self::from_element_end_resultants(
            resultants.axial_force,
            resultants.bending_moment_y,
            resultants.bending_moment_z,
            resultants.torsional_moment,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForceResultantUnitMetadata {
    pub axial_force: Option<QuantityUnitMetadata>,
    pub bending_moment_y: Option<QuantityUnitMetadata>,
    pub bending_moment_z: Option<QuantityUnitMetadata>,
    pub torsional_moment: Option<QuantityUnitMetadata>,
}

impl ForceResultantUnitMetadata {
    pub fn new(
        axial_force: Option<QuantityUnitMetadata>,
        bending_moment_y: Option<QuantityUnitMetadata>,
        bending_moment_z: Option<QuantityUnitMetadata>,
        torsional_moment: Option<QuantityUnitMetadata>,
    ) -> Self {
        Self {
            axial_force,
            bending_moment_y,
            bending_moment_z,
            torsional_moment,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StressSectionProperties {
    pub area: Option<f64>,
    pub section_modulus_y: Option<f64>,
    pub section_modulus_z: Option<f64>,
    pub torsion_constant: Option<f64>,
    pub torsion_radius: Option<f64>,
}

impl StressSectionProperties {
    pub fn new(
        area: Option<f64>,
        section_modulus_y: Option<f64>,
        section_modulus_z: Option<f64>,
        torsion_constant: Option<f64>,
        torsion_radius: Option<f64>,
    ) -> Self {
        Self {
            area,
            section_modulus_y,
            section_modulus_z,
            torsion_constant,
            torsion_radius,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StressSectionUnitMetadata {
    pub area: Option<QuantityUnitMetadata>,
    pub section_modulus_y: Option<QuantityUnitMetadata>,
    pub section_modulus_z: Option<QuantityUnitMetadata>,
    pub torsion_constant: Option<QuantityUnitMetadata>,
    pub torsion_radius: Option<QuantityUnitMetadata>,
}

impl StressSectionUnitMetadata {
    pub fn new(
        area: Option<QuantityUnitMetadata>,
        section_modulus_y: Option<QuantityUnitMetadata>,
        section_modulus_z: Option<QuantityUnitMetadata>,
        torsion_constant: Option<QuantityUnitMetadata>,
        torsion_radius: Option<QuantityUnitMetadata>,
    ) -> Self {
        Self {
            area,
            section_modulus_y,
            section_modulus_z,
            torsion_constant,
            torsion_radius,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PressureBasis {
    pub pressure: Option<f64>,
    pub membrane_radius: Option<f64>,
    pub wall_thickness: Option<f64>,
}

impl PressureBasis {
    pub fn new(
        pressure: Option<f64>,
        membrane_radius: Option<f64>,
        wall_thickness: Option<f64>,
    ) -> Self {
        Self {
            pressure,
            membrane_radius,
            wall_thickness,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PressureBasisUnitMetadata {
    pub pressure: Option<QuantityUnitMetadata>,
    pub membrane_radius: Option<QuantityUnitMetadata>,
    pub wall_thickness: Option<QuantityUnitMetadata>,
}

impl PressureBasisUnitMetadata {
    pub fn new(
        pressure: Option<QuantityUnitMetadata>,
        membrane_radius: Option<QuantityUnitMetadata>,
        wall_thickness: Option<QuantityUnitMetadata>,
    ) -> Self {
        Self {
            pressure,
            membrane_radius,
            wall_thickness,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StressComponents {
    pub axial_normal: Option<f64>,
    pub bending_normal_y: Option<f64>,
    pub bending_normal_z: Option<f64>,
    pub torsional_shear: Option<f64>,
    pub pressure_hoop: Option<f64>,
    pub pressure_longitudinal: Option<f64>,
}

impl StressComponents {
    fn empty() -> Self {
        Self {
            axial_normal: None,
            bending_normal_y: None,
            bending_normal_z: None,
            torsional_shear: None,
            pressure_hoop: None,
            pressure_longitudinal: None,
        }
    }

    pub fn to_result_boundary_record(
        &self,
        component: StressComponentKind,
        record: BoundaryRecordRef,
        unit: QuantityUnitMetadata,
        provenance_ref: impl Into<String>,
    ) -> Result<Option<BoundaryQuantityRecord>, BoundaryMetadataError> {
        require_result_schema_binding(record.schema_binding)?;
        match self.value_for(component) {
            Some(value) => BoundaryQuantityRecord::from_canonical_dimension(
                record,
                value,
                CanonicalDimension::Stress,
                unit,
                provenance_ref,
            )
            .map(Some),
            None => Ok(None),
        }
    }

    fn value_for(&self, component: StressComponentKind) -> Option<f64> {
        match component {
            StressComponentKind::AxialNormal => self.axial_normal,
            StressComponentKind::BendingNormalY => self.bending_normal_y,
            StressComponentKind::BendingNormalZ => self.bending_normal_z,
            StressComponentKind::TorsionalShear => self.torsional_shear,
            StressComponentKind::PressureHoop => self.pressure_hoop,
            StressComponentKind::PressureLongitudinal => self.pressure_longitudinal,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StressComponentKind {
    AxialNormal,
    BendingNormalY,
    BendingNormalZ,
    TorsionalShear,
    PressureHoop,
    PressureLongitudinal,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StressSummary {
    pub max_normal: f64,
    pub min_normal: f64,
    pub max_shear_magnitude: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StressRangeComponents {
    pub axial_normal_range: Option<f64>,
    pub bending_normal_y_range: Option<f64>,
    pub bending_normal_z_range: Option<f64>,
    pub torsional_shear_range: Option<f64>,
    pub pressure_hoop_range: Option<f64>,
    pub pressure_longitudinal_range: Option<f64>,
}

impl StressRangeComponents {
    fn empty() -> Self {
        Self {
            axial_normal_range: None,
            bending_normal_y_range: None,
            bending_normal_z_range: None,
            torsional_shear_range: None,
            pressure_hoop_range: None,
            pressure_longitudinal_range: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StressRecoveryInput {
    pub resultants: ForceResultants,
    pub section: StressSectionProperties,
    pub pressure: Option<PressureBasis>,
    pub statuses: Vec<AnalysisStatus>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StressRecoveryInputUnitMetadata {
    pub resultants: ForceResultantUnitMetadata,
    pub section: StressSectionUnitMetadata,
    pub pressure: Option<PressureBasisUnitMetadata>,
}

impl StressRecoveryInputUnitMetadata {
    pub fn new(
        resultants: ForceResultantUnitMetadata,
        section: StressSectionUnitMetadata,
        pressure: Option<PressureBasisUnitMetadata>,
    ) -> Self {
        Self {
            resultants,
            section,
            pressure,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StressRecoveryResult {
    pub components: StressComponents,
    pub summary: Option<StressSummary>,
    pub statuses: Vec<AnalysisStatus>,
    pub findings: Vec<StressFinding>,
}

impl StressRecoveryResult {
    pub fn is_blocked(&self) -> bool {
        !self.findings.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StressRangeResult {
    pub ranges: StressRangeComponents,
    pub findings: Vec<StressFinding>,
}

impl StressRangeResult {
    pub fn is_blocked(&self) -> bool {
        !self.findings.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StationStressRecoveryInput {
    pub station_id: String,
    pub station_fraction: f64,
    pub resultants: ForceResultants,
    pub section: StressSectionProperties,
    pub pressure: Option<PressureBasis>,
    pub statuses: Vec<AnalysisStatus>,
}

impl StationStressRecoveryInput {
    pub fn from_station_resultants(
        station_id: impl Into<String>,
        resultants: &StationResultants,
        section: StressSectionProperties,
        pressure: Option<PressureBasis>,
        statuses: Vec<AnalysisStatus>,
    ) -> Result<Self, StressRecoveryError> {
        Ok(Self {
            station_id: station_id.into(),
            station_fraction: checked_station_fraction(resultants.station_fraction)?,
            resultants: ForceResultants::from_station_resultants(resultants)?,
            section,
            pressure,
            statuses,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StationStressRecoveryResult {
    pub station_id: String,
    pub station_fraction: f64,
    pub stress: StressRecoveryResult,
}

impl StationStressRecoveryResult {
    pub fn is_blocked(&self) -> bool {
        self.stress.is_blocked()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum StressRecoveryError {
    NonFiniteInput { name: &'static str, value: f64 },
    NonPositiveInput { name: &'static str, value: f64 },
    InvalidStationFraction { value: f64 },
}

impl fmt::Display for StressRecoveryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteInput { name, value } => {
                write!(f, "{name} must be finite, got {value}")
            }
            Self::NonPositiveInput { name, value } => {
                write!(f, "{name} must be positive, got {value}")
            }
            Self::InvalidStationFraction { value } => {
                write!(
                    f,
                    "station fraction must satisfy 0 <= value <= 1, got {value}"
                )
            }
        }
    }
}

impl Error for StressRecoveryError {}

pub fn recover_stresses(input: &StressRecoveryInput) -> StressRecoveryResult {
    let mut findings = Vec::new();
    let statuses = collect_statuses(&input.statuses, &mut findings);

    let mut components = StressComponents::empty();
    components.axial_normal = divide_optional(
        input.resultants.axial_force,
        input.section.area,
        "axial_force",
        "area",
        &mut findings,
    );
    components.bending_normal_y = divide_optional(
        input.resultants.bending_moment_y,
        input.section.section_modulus_y,
        "bending_moment_y",
        "section_modulus_y",
        &mut findings,
    );
    components.bending_normal_z = divide_optional(
        input.resultants.bending_moment_z,
        input.section.section_modulus_z,
        "bending_moment_z",
        "section_modulus_z",
        &mut findings,
    );
    components.torsional_shear = torsional_shear(&input.resultants, &input.section, &mut findings);

    if let Some(pressure) = &input.pressure {
        let (hoop, longitudinal) = pressure_membrane(pressure, &mut findings);
        components.pressure_hoop = hoop;
        components.pressure_longitudinal = longitudinal;
    }

    let summary = if findings.is_empty() {
        Some(summarize_components(&components))
    } else {
        None
    };

    StressRecoveryResult {
        components: if findings.is_empty() {
            components
        } else {
            StressComponents::empty()
        },
        summary,
        statuses,
        findings,
    }
}

pub fn recover_station_stresses(input: &StationStressRecoveryInput) -> StationStressRecoveryResult {
    let stress_input = StressRecoveryInput {
        resultants: input.resultants.clone(),
        section: input.section.clone(),
        pressure: input.pressure.clone(),
        statuses: input.statuses.clone(),
    };
    StationStressRecoveryResult {
        station_id: input.station_id.clone(),
        station_fraction: input.station_fraction,
        stress: recover_stresses(&stress_input),
    }
}

pub fn recover_station_stress_sweep(
    station_id_prefix: impl AsRef<str>,
    resultants: &[StationResultants],
    section: StressSectionProperties,
    pressure: Option<PressureBasis>,
    statuses: Vec<AnalysisStatus>,
) -> Result<Vec<StationStressRecoveryResult>, StressRecoveryError> {
    let station_id_prefix = station_id_prefix.as_ref();
    resultants
        .iter()
        .enumerate()
        .map(|(index, resultants)| {
            let input = StationStressRecoveryInput::from_station_resultants(
                format!("{station_id_prefix}:station:{index}"),
                resultants,
                section.clone(),
                pressure.clone(),
                statuses.clone(),
            )?;
            Ok(recover_station_stresses(&input))
        })
        .collect()
}

pub fn recover_stresses_with_unit_metadata(
    input: &StressRecoveryInput,
    units: &StressRecoveryInputUnitMetadata,
) -> StressRecoveryResult {
    let mut findings = validate_stress_input_unit_metadata(input, units);
    if findings.is_empty() {
        return recover_stresses(input);
    }

    let statuses = collect_statuses(&input.statuses, &mut findings);
    StressRecoveryResult {
        components: StressComponents::empty(),
        summary: None,
        statuses,
        findings,
    }
}

pub fn validate_stress_input_unit_metadata(
    input: &StressRecoveryInput,
    units: &StressRecoveryInputUnitMetadata,
) -> Vec<StressFinding> {
    let mut findings = Vec::new();

    check_unit_metadata(
        input.resultants.axial_force,
        units.resultants.axial_force.as_ref(),
        "axial_force",
        CanonicalDimension::Force,
        &mut findings,
    );
    check_unit_metadata(
        input.resultants.bending_moment_y,
        units.resultants.bending_moment_y.as_ref(),
        "bending_moment_y",
        CanonicalDimension::Moment,
        &mut findings,
    );
    check_unit_metadata(
        input.resultants.bending_moment_z,
        units.resultants.bending_moment_z.as_ref(),
        "bending_moment_z",
        CanonicalDimension::Moment,
        &mut findings,
    );
    check_unit_metadata(
        input.resultants.torsional_moment,
        units.resultants.torsional_moment.as_ref(),
        "torsional_moment",
        CanonicalDimension::Moment,
        &mut findings,
    );
    check_unit_metadata(
        input.section.area,
        units.section.area.as_ref(),
        "area",
        CanonicalDimension::Area,
        &mut findings,
    );
    check_unit_metadata(
        input.section.section_modulus_y,
        units.section.section_modulus_y.as_ref(),
        "section_modulus_y",
        CanonicalDimension::SectionModulus,
        &mut findings,
    );
    check_unit_metadata(
        input.section.section_modulus_z,
        units.section.section_modulus_z.as_ref(),
        "section_modulus_z",
        CanonicalDimension::SectionModulus,
        &mut findings,
    );
    check_unit_metadata(
        input.section.torsion_constant,
        units.section.torsion_constant.as_ref(),
        "torsion_constant",
        CanonicalDimension::SecondMomentArea,
        &mut findings,
    );
    check_unit_metadata(
        input.section.torsion_radius,
        units.section.torsion_radius.as_ref(),
        "torsion_radius",
        CanonicalDimension::Length,
        &mut findings,
    );

    if let Some(pressure) = &input.pressure {
        let pressure_units = units.pressure.as_ref();
        check_unit_metadata(
            pressure.pressure,
            pressure_units.and_then(|metadata| metadata.pressure.as_ref()),
            "pressure",
            CanonicalDimension::Pressure,
            &mut findings,
        );
        check_unit_metadata(
            pressure.membrane_radius,
            pressure_units.and_then(|metadata| metadata.membrane_radius.as_ref()),
            "membrane_radius",
            CanonicalDimension::Length,
            &mut findings,
        );
        check_unit_metadata(
            pressure.wall_thickness,
            pressure_units.and_then(|metadata| metadata.wall_thickness.as_ref()),
            "wall_thickness",
            CanonicalDimension::Length,
            &mut findings,
        );
    }

    findings
}

pub fn recover_stress_range(
    first: &StressRecoveryInput,
    second: &StressRecoveryInput,
) -> StressRangeResult {
    let first_result = recover_stresses(first);
    let second_result = recover_stresses(second);
    let mut findings = Vec::new();

    if first_result.is_blocked() {
        findings.push(StressFinding::new(
            FindingCode::BlockedStressState,
            "first_state",
            "stress range requires an unblocked first mechanics state",
        ));
        findings.extend(first_result.findings);
    }
    if second_result.is_blocked() {
        findings.push(StressFinding::new(
            FindingCode::BlockedStressState,
            "second_state",
            "stress range requires an unblocked second mechanics state",
        ));
        findings.extend(second_result.findings);
    }
    if !findings.is_empty() {
        return StressRangeResult {
            ranges: StressRangeComponents::empty(),
            findings,
        };
    }

    let first_components = first_result.components;
    let second_components = second_result.components;
    let ranges = StressRangeComponents {
        axial_normal_range: range_optional(
            first_components.axial_normal,
            second_components.axial_normal,
            "axial_normal_range",
            &mut findings,
        ),
        bending_normal_y_range: range_optional(
            first_components.bending_normal_y,
            second_components.bending_normal_y,
            "bending_normal_y_range",
            &mut findings,
        ),
        bending_normal_z_range: range_optional(
            first_components.bending_normal_z,
            second_components.bending_normal_z,
            "bending_normal_z_range",
            &mut findings,
        ),
        torsional_shear_range: range_optional(
            first_components.torsional_shear,
            second_components.torsional_shear,
            "torsional_shear_range",
            &mut findings,
        ),
        pressure_hoop_range: range_optional(
            first_components.pressure_hoop,
            second_components.pressure_hoop,
            "pressure_hoop_range",
            &mut findings,
        ),
        pressure_longitudinal_range: range_optional(
            first_components.pressure_longitudinal,
            second_components.pressure_longitudinal,
            "pressure_longitudinal_range",
            &mut findings,
        ),
    };

    StressRangeResult {
        ranges: if findings.is_empty() {
            ranges
        } else {
            StressRangeComponents::empty()
        },
        findings,
    }
}

pub fn checked_positive(name: &'static str, value: f64) -> Result<f64, StressRecoveryError> {
    checked_finite(name, value)?;
    if value <= 0.0 {
        return Err(StressRecoveryError::NonPositiveInput { name, value });
    }
    Ok(value)
}

pub fn checked_finite(name: &'static str, value: f64) -> Result<f64, StressRecoveryError> {
    if !value.is_finite() {
        return Err(StressRecoveryError::NonFiniteInput { name, value });
    }
    Ok(value)
}

pub fn checked_station_fraction(value: f64) -> Result<f64, StressRecoveryError> {
    checked_finite("station_fraction", value)?;
    if !(0.0..=1.0).contains(&value) {
        return Err(StressRecoveryError::InvalidStationFraction { value });
    }
    Ok(value)
}

fn check_unit_metadata(
    value: Option<f64>,
    metadata: Option<&QuantityUnitMetadata>,
    subject: &'static str,
    expected_dimension: CanonicalDimension,
    findings: &mut Vec<StressFinding>,
) {
    if value.is_none() {
        return;
    }

    let Some(metadata) = metadata else {
        findings.push(StressFinding::new(
            FindingCode::MissingUnitMetadata,
            subject,
            format!("{subject} requires explicit unit metadata at the stress boundary"),
        ));
        return;
    };

    if metadata.validate().is_err() {
        findings.push(StressFinding::new(
            FindingCode::MissingUnitMetadata,
            subject,
            format!("{subject} unit metadata must include unit and unit-system references"),
        ));
        return;
    }

    if metadata.dimension != expected_dimension {
        findings.push(StressFinding::new(
            FindingCode::InvalidUnitMetadata,
            subject,
            format!(
                "{subject} requires canonical dimension {}, got {}",
                expected_dimension.as_str(),
                metadata.dimension.as_str()
            ),
        ));
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

fn divide_optional(
    numerator: Option<f64>,
    denominator: Option<f64>,
    numerator_name: &'static str,
    denominator_name: &'static str,
    findings: &mut Vec<StressFinding>,
) -> Option<f64> {
    let numerator = require_finite(
        numerator,
        numerator_name,
        FindingCode::MissingResultant,
        findings,
    )?;
    let denominator = require_positive(
        denominator,
        denominator_name,
        FindingCode::MissingSectionProperty,
        findings,
    )?;
    checked_recovered(numerator / denominator, numerator_name, findings)
}

fn torsional_shear(
    resultants: &ForceResultants,
    section: &StressSectionProperties,
    findings: &mut Vec<StressFinding>,
) -> Option<f64> {
    let torque = require_finite(
        resultants.torsional_moment,
        "torsional_moment",
        FindingCode::MissingResultant,
        findings,
    )?;
    let radius = require_positive(
        section.torsion_radius,
        "torsion_radius",
        FindingCode::MissingSectionProperty,
        findings,
    )?;
    let torsion_constant = require_positive(
        section.torsion_constant,
        "torsion_constant",
        FindingCode::MissingSectionProperty,
        findings,
    )?;
    checked_recovered(
        torque * radius / torsion_constant,
        "torsional_shear",
        findings,
    )
}

fn pressure_membrane(
    pressure: &PressureBasis,
    findings: &mut Vec<StressFinding>,
) -> (Option<f64>, Option<f64>) {
    let pressure_value = require_finite(
        pressure.pressure,
        "pressure",
        FindingCode::MissingPressureInput,
        findings,
    );
    let radius = require_positive(
        pressure.membrane_radius,
        "membrane_radius",
        FindingCode::MissingPressureInput,
        findings,
    );
    let wall = require_positive(
        pressure.wall_thickness,
        "wall_thickness",
        FindingCode::MissingPressureInput,
        findings,
    );

    match (pressure_value, radius, wall) {
        (Some(p), Some(r), Some(t)) => {
            let hoop = p * r / t;
            let hoop = checked_recovered(hoop, "pressure_hoop", findings);
            let longitudinal = hoop.and_then(|value| {
                checked_recovered(value / 2.0, "pressure_longitudinal", findings)
            });
            (hoop, longitudinal)
        }
        _ => (None, None),
    }
}

fn checked_recovered(
    value: f64,
    subject: &'static str,
    findings: &mut Vec<StressFinding>,
) -> Option<f64> {
    if value.is_finite() {
        return Some(value);
    }
    findings.push(StressFinding::new(
        FindingCode::NonFiniteInput,
        subject,
        format!("{subject} recovery produced a non-finite value"),
    ));
    None
}

fn range_optional(
    first: Option<f64>,
    second: Option<f64>,
    subject: &'static str,
    findings: &mut Vec<StressFinding>,
) -> Option<f64> {
    match (first, second) {
        (Some(a), Some(b)) => checked_recovered((b - a).abs(), subject, findings),
        (None, None) => None,
        _ => {
            findings.push(StressFinding::new(
                FindingCode::MissingResultant,
                subject,
                format!("{subject} requires both mechanics states or neither state"),
            ));
            None
        }
    }
}

fn require_finite(
    value: Option<f64>,
    subject: &'static str,
    missing_code: FindingCode,
    findings: &mut Vec<StressFinding>,
) -> Option<f64> {
    let Some(value) = value else {
        findings.push(StressFinding::new(
            missing_code,
            subject,
            format!("{subject} is required for this recovery component"),
        ));
        return None;
    };
    if !value.is_finite() {
        findings.push(StressFinding::new(
            FindingCode::NonFiniteInput,
            subject,
            format!("{subject} must be finite"),
        ));
        return None;
    }
    Some(value)
}

fn require_positive(
    value: Option<f64>,
    subject: &'static str,
    missing_code: FindingCode,
    findings: &mut Vec<StressFinding>,
) -> Option<f64> {
    let value = require_finite(value, subject, missing_code, findings)?;
    if value <= 0.0 {
        findings.push(StressFinding::new(
            FindingCode::NonPositiveInput,
            subject,
            format!("{subject} must be positive"),
        ));
        return None;
    }
    Some(value)
}

fn summarize_components(components: &StressComponents) -> StressSummary {
    let axial = components.axial_normal.unwrap_or(0.0);
    let pressure_longitudinal = components.pressure_longitudinal.unwrap_or(0.0);
    let bending_y = components.bending_normal_y.unwrap_or(0.0).abs();
    let bending_z = components.bending_normal_z.unwrap_or(0.0).abs();
    let base_normal = axial + pressure_longitudinal;
    let bending_total = bending_y + bending_z;

    StressSummary {
        max_normal: base_normal + bending_total,
        min_normal: base_normal - bending_total,
        max_shear_magnitude: components.torsional_shear.unwrap_or(0.0).abs(),
    }
}

fn collect_statuses(
    source_statuses: &[AnalysisStatus],
    findings: &mut Vec<StressFinding>,
) -> Vec<AnalysisStatus> {
    let mut statuses = Vec::new();
    if source_statuses.is_empty() {
        findings.push(StressFinding::new(
            FindingCode::IncompleteMechanicsStatus,
            "analysis_status",
            "stress recovery requires explicit mechanics status",
        ));
    }

    for status in source_statuses {
        if !automatic_status_allowed(*status) {
            findings.push(StressFinding::new(
                FindingCode::StatusBoundaryViolation,
                "analysis_status",
                "stress recovery cannot emit human approval or compliance status",
            ));
            continue;
        }
        if !statuses.contains(status) {
            statuses.push(*status);
        }
    }

    if !statuses.contains(&AnalysisStatus::MechanicsSolved) {
        findings.push(StressFinding::new(
            FindingCode::IncompleteMechanicsStatus,
            "analysis_status",
            "stress recovery requires mechanics-solved input",
        ));
    }
    if !statuses.contains(&AnalysisStatus::HumanReviewRequired) {
        statuses.push(AnalysisStatus::HumanReviewRequired);
    }
    statuses
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

#[cfg(test)]
mod tests {
    use super::*;

    fn unit(unit_name: &str, dimension: CanonicalDimension) -> QuantityUnitMetadata {
        QuantityUnitMetadata::new(unit_name, dimension, "unit-system:si").unwrap()
    }

    fn complete_units() -> StressRecoveryInputUnitMetadata {
        StressRecoveryInputUnitMetadata::new(
            ForceResultantUnitMetadata::new(
                Some(unit("N", CanonicalDimension::Force)),
                Some(unit("N*m", CanonicalDimension::Moment)),
                Some(unit("N*m", CanonicalDimension::Moment)),
                Some(unit("N*m", CanonicalDimension::Moment)),
            ),
            StressSectionUnitMetadata::new(
                Some(unit("m^2", CanonicalDimension::Area)),
                Some(unit("m^3", CanonicalDimension::SectionModulus)),
                Some(unit("m^3", CanonicalDimension::SectionModulus)),
                Some(unit("m^4", CanonicalDimension::SecondMomentArea)),
                Some(unit("m", CanonicalDimension::Length)),
            ),
            Some(PressureBasisUnitMetadata::new(
                Some(unit("Pa", CanonicalDimension::Pressure)),
                Some(unit("m", CanonicalDimension::Length)),
                Some(unit("m", CanonicalDimension::Length)),
            )),
        )
    }

    fn boundary_ref(schema_binding: CanonicalSchemaBinding) -> BoundaryRecordRef {
        BoundaryRecordRef::new(
            "stress:axial",
            schema_binding,
            "element:0:station:end-i",
            "result-set:mechanics",
            "payload:result-envelope",
            "hash:result-envelope",
        )
        .unwrap()
    }

    fn complete_input() -> StressRecoveryInput {
        StressRecoveryInput {
            resultants: ForceResultants::new(Some(120.0), Some(50.0), Some(-30.0), Some(40.0)),
            section: StressSectionProperties::new(
                Some(12.0),
                Some(25.0),
                Some(15.0),
                Some(80.0),
                Some(2.0),
            ),
            pressure: Some(PressureBasis::new(Some(100.0), Some(3.0), Some(0.5))),
            statuses: vec![AnalysisStatus::MechanicsSolved],
        }
    }

    #[test]
    fn recovers_axial_bending_torsion_and_pressure_components() {
        let result = recover_stresses(&complete_input());

        assert!(!result.is_blocked());
        assert_eq!(result.components.axial_normal, Some(10.0));
        assert_eq!(result.components.bending_normal_y, Some(2.0));
        assert_eq!(result.components.bending_normal_z, Some(-2.0));
        assert_eq!(result.components.torsional_shear, Some(1.0));
        assert_eq!(result.components.pressure_hoop, Some(600.0));
        assert_eq!(result.components.pressure_longitudinal, Some(300.0));
    }

    #[test]
    fn stress_recovery_with_unit_metadata_validates_boundary_before_recovery() {
        let result = recover_stresses_with_unit_metadata(&complete_input(), &complete_units());

        assert!(!result.is_blocked());
        assert_eq!(result.components.axial_normal, Some(10.0));
        assert_eq!(result.components.pressure_hoop, Some(600.0));
    }

    #[test]
    fn stress_recovery_unit_metadata_blocks_missing_or_wrong_dimensions() {
        let mut units = complete_units();
        units.section.area = None;
        units.section.section_modulus_y = Some(unit("m^2", CanonicalDimension::Area));

        let result = recover_stresses_with_unit_metadata(&complete_input(), &units);

        assert!(result.is_blocked());
        assert!(result.components.axial_normal.is_none());
        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == FindingCode::MissingUnitMetadata
                && finding.subject_id == "area"));
        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == FindingCode::InvalidUnitMetadata
                && finding.subject_id == "section_modulus_y"));
    }

    #[test]
    fn stress_component_boundary_record_carries_result_schema_and_hash_metadata() {
        let result = recover_stresses_with_unit_metadata(&complete_input(), &complete_units());
        let record = result
            .components
            .to_result_boundary_record(
                StressComponentKind::AxialNormal,
                boundary_ref(CanonicalSchemaBinding::ResultsQuantityResult),
                unit("Pa", CanonicalDimension::Stress),
                "provenance:stress",
            )
            .unwrap()
            .unwrap();

        assert_eq!(record.value, 10.0);
        assert_eq!(record.unit.dimension, CanonicalDimension::Stress);
        assert_eq!(
            record.record.schema_binding.schema_ref(),
            "schemas/results.schema.yaml#/$defs/QuantityResult"
        );
        assert!(record
            .round_trip_key()
            .contains("payload_hash_ref=hash:result-envelope"));
    }

    #[test]
    fn stress_component_boundary_record_rejects_load_case_schema_binding() {
        let result = recover_stresses_with_unit_metadata(&complete_input(), &complete_units());

        let err = result
            .components
            .to_result_boundary_record(
                StressComponentKind::AxialNormal,
                boundary_ref(CanonicalSchemaBinding::ModelLoadCase),
                unit("Pa", CanonicalDimension::Stress),
                "provenance:stress",
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
    fn summarizes_extreme_normal_and_shear_without_allowables() {
        let result = recover_stresses(&complete_input());
        let summary = result.summary.unwrap();

        assert_eq!(summary.max_normal, 314.0);
        assert_eq!(summary.min_normal, 306.0);
        assert_eq!(summary.max_shear_magnitude, 1.0);
    }

    #[test]
    fn recovers_mechanics_stress_ranges_without_code_categories() {
        let first = StressRecoveryInput {
            resultants: ForceResultants::new(Some(60.0), Some(-20.0), Some(10.0), Some(20.0)),
            section: StressSectionProperties::new(
                Some(12.0),
                Some(25.0),
                Some(15.0),
                Some(80.0),
                Some(2.0),
            ),
            pressure: None,
            statuses: vec![AnalysisStatus::MechanicsSolved],
        };
        let second = StressRecoveryInput {
            resultants: ForceResultants::new(Some(180.0), Some(80.0), Some(10.0), Some(60.0)),
            section: first.section.clone(),
            pressure: None,
            statuses: vec![AnalysisStatus::MechanicsSolved],
        };

        let result = recover_stress_range(&first, &second);

        assert!(!result.is_blocked());
        assert_eq!(result.ranges.axial_normal_range, Some(10.0));
        assert_eq!(result.ranges.bending_normal_y_range, Some(4.0));
        assert_eq!(result.ranges.bending_normal_z_range, Some(0.0));
        assert_eq!(result.ranges.torsional_shear_range, Some(1.0));
        assert_eq!(result.ranges.pressure_hoop_range, None);
        assert_eq!(result.ranges.pressure_longitudinal_range, None);
    }

    #[test]
    fn stress_range_blocks_when_either_state_is_incomplete() {
        let first = complete_input();
        let mut second = complete_input();
        second.resultants.torsional_moment = None;

        let result = recover_stress_range(&first, &second);

        assert!(result.is_blocked());
        assert_eq!(result.ranges, StressRangeComponents::empty());
        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == FindingCode::BlockedStressState));
        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == FindingCode::MissingResultant));
    }

    #[test]
    fn stress_range_blocks_asymmetric_optional_components() {
        let mut first = complete_input();
        first.pressure = None;
        let second = complete_input();

        let result = recover_stress_range(&first, &second);

        assert!(result.is_blocked());
        assert_eq!(result.ranges, StressRangeComponents::empty());
        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == FindingCode::MissingResultant
                && finding.subject_id == "pressure_hoop_range"));
    }

    #[test]
    fn missing_resultant_blocks_recovery() {
        let mut input = complete_input();
        input.resultants.axial_force = None;

        let result = recover_stresses(&input);

        assert!(result.is_blocked());
        assert_eq!(result.components.axial_normal, None);
        assert_eq!(result.summary, None);
        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == FindingCode::MissingResultant));
    }

    #[test]
    fn non_positive_section_property_blocks_recovery() {
        let mut input = complete_input();
        input.section.area = Some(0.0);

        let result = recover_stresses(&input);

        assert!(result.is_blocked());
        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == FindingCode::NonPositiveInput));
    }

    #[test]
    fn non_finite_pressure_is_reported() {
        let mut input = complete_input();
        input.pressure = Some(PressureBasis::new(Some(f64::NAN), Some(3.0), Some(0.5)));

        let result = recover_stresses(&input);

        assert!(result.is_blocked());
        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == FindingCode::NonFiniteInput));
    }

    #[test]
    fn non_finite_recovered_values_are_reported() {
        let mut input = complete_input();
        input.pressure = Some(PressureBasis::new(
            Some(f64::MAX),
            Some(f64::MAX),
            Some(0.5),
        ));

        let result = recover_stresses(&input);

        assert!(result.is_blocked());
        assert_eq!(result.components.pressure_hoop, None);
        assert_eq!(result.summary, None);
        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == FindingCode::NonFiniteInput
                && finding.subject_id == "pressure_hoop"));
    }

    #[test]
    fn stress_range_reports_non_finite_difference() {
        let first = StressRecoveryInput {
            resultants: ForceResultants::new(Some(-f64::MAX), Some(0.0), Some(0.0), Some(0.0)),
            section: StressSectionProperties::new(
                Some(1.0),
                Some(1.0),
                Some(1.0),
                Some(1.0),
                Some(1.0),
            ),
            pressure: None,
            statuses: vec![AnalysisStatus::MechanicsSolved],
        };
        let second = StressRecoveryInput {
            resultants: ForceResultants::new(Some(f64::MAX), Some(0.0), Some(0.0), Some(0.0)),
            section: first.section.clone(),
            pressure: None,
            statuses: vec![AnalysisStatus::MechanicsSolved],
        };

        let result = recover_stress_range(&first, &second);

        assert!(result.is_blocked());
        assert_eq!(result.ranges.axial_normal_range, None);
        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == FindingCode::NonFiniteInput
                && finding.subject_id == "axial_normal_range"));
    }

    #[test]
    fn incomplete_status_blocks_mechanics_recovery() {
        let mut input = complete_input();
        input.statuses = vec![AnalysisStatus::ModelIncomplete];

        let result = recover_stresses(&input);

        assert!(result.is_blocked());
        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == FindingCode::IncompleteMechanicsStatus));
        assert!(result
            .statuses
            .contains(&AnalysisStatus::HumanReviewRequired));
    }

    #[test]
    fn human_approval_status_is_not_accepted_as_software_output() {
        let mut input = complete_input();
        input.statuses = vec![
            AnalysisStatus::MechanicsSolved,
            AnalysisStatus::HumanApprovedForProject,
        ];

        let result = recover_stresses(&input);

        assert!(result.is_blocked());
        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == FindingCode::StatusBoundaryViolation));
        assert!(!result
            .statuses
            .contains(&AnalysisStatus::HumanApprovedForProject));
    }

    #[test]
    fn element_end_resultants_construct_force_resultants_for_stress_recovery() {
        let resultants = ForceResultants::from_element_end_resultants(16.0, 0.0, 0.0, 6.4).unwrap();
        let input = StressRecoveryInput {
            resultants,
            section: StressSectionProperties::new(
                Some(4.0),
                Some(20.0),
                Some(25.0),
                Some(2.0),
                Some(0.5),
            ),
            pressure: None,
            statuses: vec![AnalysisStatus::MechanicsSolved],
        };

        let result = recover_stresses(&input);

        assert!(!result.is_blocked());
        assert_eq!(result.components.axial_normal, Some(4.0));
        assert_eq!(result.components.bending_normal_y, Some(0.0));
        assert_eq!(result.components.bending_normal_z, Some(0.0));
        assert_eq!(result.components.torsional_shear, Some(1.6));
    }

    #[test]
    fn station_resultants_recover_station_stresses() {
        let station = StationResultants {
            station_fraction: 0.5,
            distance_from_i: 2.0,
            axial_force: 16.0,
            shear_force_y: -4.0,
            shear_force_z: 3.0,
            torsional_moment: 6.4,
            bending_moment_y: 20.0,
            bending_moment_z: -25.0,
        };
        let input = StationStressRecoveryInput::from_station_resultants(
            "station:midspan",
            &station,
            StressSectionProperties::new(Some(4.0), Some(20.0), Some(25.0), Some(2.0), Some(0.5)),
            None,
            vec![AnalysisStatus::MechanicsSolved],
        )
        .unwrap();

        let result = recover_station_stresses(&input);

        assert_eq!(result.station_id, "station:midspan");
        assert_eq!(result.station_fraction, 0.5);
        assert!(!result.is_blocked());
        assert_eq!(result.stress.components.axial_normal, Some(4.0));
        assert_eq!(result.stress.components.bending_normal_y, Some(1.0));
        assert_eq!(result.stress.components.bending_normal_z, Some(-1.0));
        assert_eq!(result.stress.components.torsional_shear, Some(1.6));
    }

    #[test]
    fn station_stress_sweep_preserves_order_and_station_identity() {
        let stations = vec![
            StationResultants {
                station_fraction: 0.75,
                distance_from_i: 3.0,
                axial_force: 0.0,
                shear_force_y: 0.0,
                shear_force_z: 0.0,
                torsional_moment: 0.0,
                bending_moment_y: 0.0,
                bending_moment_z: 0.0,
            },
            StationResultants {
                station_fraction: 0.25,
                distance_from_i: 1.0,
                axial_force: 8.0,
                shear_force_y: 4.0,
                shear_force_z: 0.0,
                torsional_moment: 0.0,
                bending_moment_y: 0.0,
                bending_moment_z: 4.0,
            },
            StationResultants {
                station_fraction: 0.5,
                distance_from_i: 2.0,
                axial_force: 4.0,
                shear_force_y: 2.0,
                shear_force_z: 0.0,
                torsional_moment: 0.0,
                bending_moment_y: 0.0,
                bending_moment_z: 1.0,
            },
        ];

        let sweep = recover_station_stress_sweep(
            "stress-sweep",
            &stations,
            StressSectionProperties::new(Some(4.0), Some(20.0), Some(2.0), Some(1.0), Some(0.5)),
            None,
            vec![AnalysisStatus::MechanicsSolved],
        )
        .unwrap();

        assert_eq!(sweep[0].station_id, "stress-sweep:station:0");
        assert_eq!(sweep[1].station_id, "stress-sweep:station:1");
        assert_eq!(sweep[2].station_id, "stress-sweep:station:2");
        assert_eq!(sweep[0].station_fraction, 0.75);
        assert_eq!(sweep[1].station_fraction, 0.25);
        assert_eq!(sweep[2].station_fraction, 0.5);
        assert_eq!(sweep[0].stress.components.bending_normal_z, Some(0.0));
        assert_eq!(sweep[1].stress.components.axial_normal, Some(2.0));
        assert_eq!(sweep[1].stress.components.bending_normal_z, Some(2.0));
        assert_eq!(sweep[2].stress.components.axial_normal, Some(1.0));
        assert_eq!(sweep[2].stress.components.bending_normal_z, Some(0.5));
    }

    #[test]
    fn station_stress_sweep_rejects_invalid_station_resultants() {
        let stations = vec![StationResultants {
            station_fraction: -0.1,
            distance_from_i: 0.0,
            axial_force: 0.0,
            shear_force_y: 0.0,
            shear_force_z: 0.0,
            torsional_moment: 0.0,
            bending_moment_y: 0.0,
            bending_moment_z: 0.0,
        }];

        let error = recover_station_stress_sweep(
            "stress-sweep",
            &stations,
            StressSectionProperties::new(Some(4.0), Some(20.0), Some(2.0), Some(1.0), Some(0.5)),
            None,
            vec![AnalysisStatus::MechanicsSolved],
        )
        .unwrap_err();

        assert_eq!(
            error,
            StressRecoveryError::InvalidStationFraction { value: -0.1 }
        );
    }

    #[test]
    fn station_resultants_reject_nonfinite_and_invalid_station_inputs() {
        let mut station = StationResultants {
            station_fraction: 0.25,
            distance_from_i: 1.0,
            axial_force: 1.0,
            shear_force_y: 0.0,
            shear_force_z: f64::NAN,
            torsional_moment: 0.0,
            bending_moment_y: 0.0,
            bending_moment_z: 0.0,
        };
        let err = ForceResultants::from_station_resultants(&station).unwrap_err();
        assert!(matches!(
            err,
            StressRecoveryError::NonFiniteInput {
                name: "station_shear_force_z",
                value
            } if value.is_nan()
        ));

        station.shear_force_z = 0.0;
        station.station_fraction = 1.25;
        let err = ForceResultants::from_station_resultants(&station).unwrap_err();
        assert_eq!(
            err,
            StressRecoveryError::InvalidStationFraction { value: 1.25 }
        );
    }

    #[test]
    fn element_end_resultants_reject_non_finite_inputs() {
        let err =
            ForceResultants::from_element_end_resultants(1.0, f64::NAN, 0.0, 0.0).unwrap_err();

        assert!(matches!(
            err,
            StressRecoveryError::NonFiniteInput {
                name: "bending_moment_y",
                value
            } if value.is_nan()
        ));
    }

    #[test]
    fn boundary_constructor_rejects_invalid_positive_values() {
        let err = checked_positive("area", f64::INFINITY).unwrap_err();
        assert!(matches!(
            err,
            StressRecoveryError::NonFiniteInput { name: "area", .. }
        ));

        let err = checked_positive("area", -1.0).unwrap_err();
        assert!(matches!(
            err,
            StressRecoveryError::NonPositiveInput { name: "area", .. }
        ));
    }
}
