//! Closed authored inputs for `openpipestress.load_reference_state/1.0.0`.
//!
//! Every object denies unknown fields and every union is tagged, so an
//! unrecognized field or discriminant is rejected at the typed boundary rather
//! than disappearing. Field-free branches are empty struct variants: serde's
//! internally tagged unit variants would otherwise ignore sibling keys. Reviewed branches that this first capability does not
//! implement still parse and then block with a targeted diagnostic.
use crate::Quantity;
use serde::{Deserialize, Deserializer};

/// The shared `Quantity` stays open for pre-0.4 documents; every quantity in
/// this namespace is closed, so an extra key is refused rather than dropped.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ClosedQuantity {
    value: f64,
    unit: String,
}

fn closed_quantity<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Quantity, D::Error> {
    let ClosedQuantity { value, unit } = ClosedQuantity::deserialize(deserializer)?;
    Ok(Quantity { value, unit })
}

fn closed_optional_quantity<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Quantity>, D::Error> {
    Ok(Option::<ClosedQuantity>::deserialize(deserializer)?
        .map(|ClosedQuantity { value, unit }| Quantity { value, unit }))
}

/// A new-namespace key as authored: absent, explicit `null`, or a value. An
/// explicit null is authored presence; it is never silently equal to absence.
#[derive(Debug, Clone, Default)]
pub enum Authored<T> {
    #[default]
    Absent,
    Null,
    Value(T),
}

impl<T> Authored<T> {
    pub fn value(&self) -> Option<&T> {
        match self {
            Self::Value(value) => Some(value),
            _ => None,
        }
    }
    pub fn is_authored(&self) -> bool {
        !matches!(self, Self::Absent)
    }
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Authored<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // Reached only when the key is present; `#[serde(default)]` is Absent.
        Ok(match Option::<T>::deserialize(deserializer)? {
            None => Self::Null,
            Some(value) => Self::Value(value),
        })
    }
}

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
    AuthoredModelGeometry {},
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
        #[serde(deserialize_with = "closed_quantity")]
        installation_temperature: Quantity,
    },
    /// Interval-only thermal routes; supplies no absolute temperature.
    DirectStrainReference {},
}

/// Absence of fit is an explicit selection, never an inferred default.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum FitReferenceInput {
    #[serde(rename = "none")]
    NoFit {},
    NaturalLengthChange {
        #[serde(deserialize_with = "closed_quantity")]
        length_change: Quantity,
    },
    FitStrain {
        #[serde(deserialize_with = "closed_quantity")]
        strain: Quantity,
    },
}

/// User-owned expansion definition carried by its material record.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "definition", rename_all = "snake_case", deny_unknown_fields)]
pub enum ExpansionLawInput {
    EngineeringSecant {
        id: String,
        #[serde(deserialize_with = "closed_quantity")]
        datum_temperature: Quantity,
        data: SecantDataInput,
        provenance: String,
    },
    EngineeringDilation {
        id: String,
        #[serde(deserialize_with = "closed_quantity")]
        datum_temperature: Quantity,
        data: DilationDataInput,
        provenance: String,
    },
    DifferentialPerDatumLength {
        id: String,
        #[serde(deserialize_with = "closed_quantity")]
        datum_temperature: Quantity,
        data: CoefficientTableInput,
        provenance: String,
    },
    LogarithmicPerCurrentLength {
        id: String,
        #[serde(deserialize_with = "closed_quantity")]
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
        #[serde(deserialize_with = "closed_quantity")]
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
    #[serde(deserialize_with = "closed_quantity")]
    pub temperature: Quantity,
    #[serde(deserialize_with = "closed_quantity")]
    pub coefficient: Quantity,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DilationPointInput {
    #[serde(deserialize_with = "closed_quantity")]
    pub temperature: Quantity,
    #[serde(deserialize_with = "closed_quantity")]
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
    #[serde(default, deserialize_with = "closed_optional_quantity")]
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
        #[serde(deserialize_with = "closed_quantity")]
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
        #[serde(deserialize_with = "closed_quantity")]
        strain: Quantity,
        interval_reference: String,
        provenance: String,
    },
    ConstantAlphaInterval {
        #[serde(deserialize_with = "closed_quantity")]
        coefficient: Quantity,
        #[serde(deserialize_with = "closed_quantity")]
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
    ActiveModelDevice {},
    Inactive {},
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
        #[serde(deserialize_with = "closed_quantity")]
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
    #[serde(deserialize_with = "closed_quantity")]
    pub value: Quantity,
    pub meaning: MotionMeaning,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum DeviceReferenceInput {
    ForceAtReference {
        #[serde(deserialize_with = "closed_quantity")]
        reference_position: Quantity,
        #[serde(deserialize_with = "closed_quantity")]
        force: Quantity,
    },
    UnloadedReference {
        #[serde(deserialize_with = "closed_quantity")]
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
    IndependentEquilibrium {},
}
