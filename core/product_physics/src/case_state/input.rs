//! Closed authored inputs for `openpipestress.load_reference_state/1.0.0`.
//!
//! Every object denies unknown fields and every union is tagged, so an
//! unrecognized field or discriminant is rejected at the typed boundary rather
//! than disappearing. Reviewed branches that this first capability does not
//! implement still parse and then block with a targeted diagnostic.
use crate::Quantity;
use serde::Deserialize;

/// Model-level reference configuration: the stress-free installed state.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReferenceConfigurationInput {
    pub id: String,
    #[serde(default)]
    pub label: Option<String>,
    pub geometry_ref: GeometryRefInput,
    pub member_references: Vec<MemberReferenceInput>,
    pub provenance: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum GeometryRefInput {
    /// The model's own normalized nodes, connectivity and pipe frames.
    AuthoredModelGeometry,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemberReferenceInput {
    pub pipe_ref: String,
    pub basis: ReferenceBasisInput,
    pub fit: FitReferenceInput,
    pub provenance: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ReferenceBasisInput {
    TemperatureReference {
        installation_temperature: Quantity,
    },
    /// Interval-only thermal routes; supplies no absolute temperature.
    DirectStrainReference,
}

/// Absence of fit is an explicit selection, never an inferred default.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum FitReferenceInput {
    #[serde(rename = "none")]
    NoFit,
    NaturalLengthChange {
        length_change: Quantity,
    },
    FitStrain {
        strain: Quantity,
    },
}

/// User-owned expansion definition carried by its material record.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "definition", rename_all = "snake_case", deny_unknown_fields)]
pub enum ExpansionLawInput {
    EngineeringSecant {
        id: String,
        datum_temperature: Quantity,
        data: SecantDataInput,
        provenance: String,
    },
    EngineeringDilation {
        id: String,
        datum_temperature: Quantity,
        data: DilationDataInput,
        provenance: String,
    },
    DifferentialPerDatumLength {
        id: String,
        datum_temperature: Quantity,
        data: CoefficientTableInput,
        provenance: String,
    },
    LogarithmicPerCurrentLength {
        id: String,
        datum_temperature: Quantity,
        data: CoefficientTableInput,
        provenance: String,
    },
}

impl ExpansionLawInput {
    pub(crate) fn id(&self) -> &str {
        match self {
            Self::EngineeringSecant { id, .. }
            | Self::EngineeringDilation { id, .. }
            | Self::DifferentialPerDatumLength { id, .. }
            | Self::LogarithmicPerCurrentLength { id, .. } => id,
        }
    }
    pub(crate) fn provenance(&self) -> &str {
        match self {
            Self::EngineeringSecant { provenance, .. }
            | Self::EngineeringDilation { provenance, .. }
            | Self::DifferentialPerDatumLength { provenance, .. }
            | Self::LogarithmicPerCurrentLength { provenance, .. } => provenance,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CoefficientInterpolation {
    LinearCoefficient,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DilationInterpolation {
    LinearDilation,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum SecantDataInput {
    Constant {
        coefficient: Quantity,
    },
    Table {
        interpolation: CoefficientInterpolation,
        points: Vec<CoefficientPointInput>,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum DilationDataInput {
    Table {
        interpolation: DilationInterpolation,
        points: Vec<DilationPointInput>,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum CoefficientTableInput {
    Table {
        interpolation: CoefficientInterpolation,
        points: Vec<CoefficientPointInput>,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoefficientPointInput {
    pub temperature: Quantity,
    pub coefficient: Quantity,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DilationPointInput {
    pub temperature: Quantity,
    pub dilation: Quantity,
}

/// Case-owned resolved-state request.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnalysisStateInput {
    pub contract: String,
    pub reference_configuration_ref: String,
    pub element_states: Vec<ElementStateInput>,
    pub support_states: Vec<SupportStateInput>,
    pub load_sources: Vec<LoadSourceInput>,
    pub history: HistoryInput,
    pub provenance: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ElementStateInput {
    pub pipe_ref: String,
    /// Actual physical temperature; never inferred from a property point.
    #[serde(default)]
    pub operating_temperature: Option<Quantity>,
    pub material_selection: MaterialSelectionInput,
    pub thermal_state: ThermalStateInput,
    #[serde(default)]
    pub analysis_basis_override: Option<AnalysisBasisOverrideInput>,
    /// Reviewed M21 mass-state seam; parsed so it can be refused explicitly.
    #[serde(default)]
    pub mass_state_ref: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InterpolationPolicy {
    PiecewiseLinear,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExtrapolationPolicy {
    Forbidden,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum MaterialSelectionInput {
    ExplicitBaseProperties {
        material_ref: String,
        applicability_reference: String,
    },
    ExactPoint {
        material_ref: String,
        point_ref: String,
    },
    TemperatureInterpolation {
        material_ref: String,
        temperature: Quantity,
        interpolation: InterpolationPolicy,
        extrapolation: ExtrapolationPolicy,
    },
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CoefficientMeaning {
    EngineeringInterval,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ThermalStateInput {
    UnchangedReference {
        provenance: String,
    },
    ExplicitIntervalStrain {
        strain: Quantity,
        interval_reference: String,
        provenance: String,
    },
    ConstantAlphaInterval {
        coefficient: Quantity,
        temperature_change: Quantity,
        coefficient_meaning: CoefficientMeaning,
        provenance: String,
    },
    FreeLengthState {
        expansion_law_ref: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnalysisBasisOverrideInput {
    pub reason: String,
    pub provenance: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SupportStateInput {
    pub support_ref: String,
    pub participation: ParticipationInput,
    #[serde(default)]
    pub boundary_motion: Option<Vec<MotionInput>>,
    #[serde(default)]
    pub base_motion: Option<Vec<MotionInput>>,
    #[serde(default)]
    pub device_reference: Option<DeviceReferenceInput>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ParticipationInput {
    ActiveModelDevice,
    Inactive,
    LockedEquivalentSupport {
        components: Vec<LockedComponentInput>,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LockedComponentInput {
    pub dof: String,
    pub position_source: PositionSourceInput,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum PositionSourceInput {
    Entered {
        value: Quantity,
    },
    PredecessorValue {
        case_ref: String,
        state_hash: String,
        support_ref: String,
        dof: String,
    },
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MotionMeaning {
    AbsoluteReferenceDisplacement,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MotionInput {
    pub dof: String,
    pub value: Quantity,
    pub meaning: MotionMeaning,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum DeviceReferenceInput {
    ForceAtReference {
        reference_position: Quantity,
        force: Quantity,
    },
    UnloadedReference {
        reference_position: Quantity,
    },
}

/// Complete inclusion entry for one stored ordinary primitive.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LoadSourceInput {
    pub source_ref: String,
    pub factor: f64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum HistoryInput {
    IndependentEquilibrium,
}
