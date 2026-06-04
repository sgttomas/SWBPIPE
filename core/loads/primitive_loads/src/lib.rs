//! Code-neutral primitive load case mechanics.
//!
//! This crate prepares explicit primitive loads for solver-boundary consumers.
//! It does not encode design-code load combinations, protected standards
//! content, proprietary project data, rule-pack checks, or professional
//! approval.

use open_pipe_stress_frame_kernel::{DOF_PER_NODE, UX, UY, UZ};
use open_pipe_stress_linear_supports::{FrameDof, NodeDof};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveLoadCategory {
    Weight,
    Pressure,
    Thermal,
    ImposedDisplacement,
    Hydrotest,
    Wind,
    Seismic,
    Occasional,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadTarget {
    Node(usize),
    Element(usize),
    Support { node_index: usize, dof: FrameDof },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadDimension {
    Force,
    Moment,
    ForcePerLength,
    Pressure,
    TemperatureChange,
    Acceleration,
    Displacement,
    Rotation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanonicalDimension {
    Dimensionless,
    Length,
    Mass,
    Time,
    Temperature,
    TemperatureInterval,
    Angle,
    Rotation,
    Force,
    ForcePerLength,
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

impl CanonicalDimension {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Dimensionless => "dimensionless",
            Self::Length => "length",
            Self::Mass => "mass",
            Self::Time => "time",
            Self::Temperature => "temperature",
            Self::TemperatureInterval => "temperature_interval",
            Self::Angle => "angle",
            Self::Rotation => "rotation",
            Self::Force => "force",
            Self::ForcePerLength => "force_per_length",
            Self::Moment => "moment",
            Self::Pressure => "pressure",
            Self::Stress => "stress",
            Self::Area => "area",
            Self::Volume => "volume",
            Self::Density => "density",
            Self::LinearStiffness => "linear_stiffness",
            Self::RotationalStiffness => "rotational_stiffness",
            Self::Displacement => "displacement",
            Self::Velocity => "velocity",
            Self::Acceleration => "acceleration",
            Self::ThermalConductivity => "thermal_conductivity",
            Self::SpecificHeat => "specific_heat",
            Self::ThermalExpansionCoefficient => "thermal_expansion_coefficient",
            Self::SecondMomentArea => "second_moment_area",
            Self::SectionModulus => "section_modulus",
            Self::MassPerLength => "mass_per_length",
            Self::VolumePerLength => "volume_per_length",
            Self::Slope => "slope",
            Self::Tbd => "TBD",
        }
    }

    pub fn from_schema_value(value: &str) -> Result<Self, BoundaryMetadataError> {
        match value {
            "dimensionless" => Ok(Self::Dimensionless),
            "length" => Ok(Self::Length),
            "mass" => Ok(Self::Mass),
            "time" => Ok(Self::Time),
            "temperature" => Ok(Self::Temperature),
            "temperature_interval" => Ok(Self::TemperatureInterval),
            "angle" => Ok(Self::Angle),
            "rotation" => Ok(Self::Rotation),
            "force" => Ok(Self::Force),
            "force_per_length" => Ok(Self::ForcePerLength),
            "moment" => Ok(Self::Moment),
            "pressure" => Ok(Self::Pressure),
            "stress" => Ok(Self::Stress),
            "area" => Ok(Self::Area),
            "volume" => Ok(Self::Volume),
            "density" => Ok(Self::Density),
            "linear_stiffness" => Ok(Self::LinearStiffness),
            "rotational_stiffness" => Ok(Self::RotationalStiffness),
            "displacement" => Ok(Self::Displacement),
            "velocity" => Ok(Self::Velocity),
            "acceleration" => Ok(Self::Acceleration),
            "thermal_conductivity" => Ok(Self::ThermalConductivity),
            "specific_heat" => Ok(Self::SpecificHeat),
            "thermal_expansion_coefficient" => Ok(Self::ThermalExpansionCoefficient),
            "second_moment_area" => Ok(Self::SecondMomentArea),
            "section_modulus" => Ok(Self::SectionModulus),
            "mass_per_length" => Ok(Self::MassPerLength),
            "volume_per_length" => Ok(Self::VolumePerLength),
            "slope" => Ok(Self::Slope),
            "TBD" => Ok(Self::Tbd),
            "temperature_difference" => Err(BoundaryMetadataError::RetiredDimensionAlias {
                alias: value.to_string(),
                replacement: "temperature_interval",
            }),
            "area_moment" => Err(BoundaryMetadataError::RetiredDimensionAlias {
                alias: value.to_string(),
                replacement: "second_moment_area",
            }),
            "stiffness" => Err(BoundaryMetadataError::RetiredDimensionAlias {
                alias: value.to_string(),
                replacement: "linear_stiffness or rotational_stiffness",
            }),
            _ => Err(BoundaryMetadataError::UnknownCanonicalDimension {
                value: value.to_string(),
            }),
        }
    }

    pub fn from_load_dimension(dimension: LoadDimension) -> Self {
        match dimension {
            LoadDimension::Force => Self::Force,
            LoadDimension::Moment => Self::Moment,
            LoadDimension::ForcePerLength => Self::ForcePerLength,
            LoadDimension::Pressure => Self::Pressure,
            LoadDimension::TemperatureChange => Self::TemperatureInterval,
            LoadDimension::Acceleration => Self::Acceleration,
            LoadDimension::Displacement => Self::Displacement,
            LoadDimension::Rotation => Self::Rotation,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoundaryMetadataError {
    MissingField {
        field: &'static str,
    },
    EmptyCollection {
        field: &'static str,
    },
    NonFiniteValue {
        field: &'static str,
    },
    DuplicateLoadId {
        load_id: String,
    },
    LoadCategoryMismatch {
        load_id: String,
        expected: PrimitiveLoadCategory,
        actual: PrimitiveLoadCategory,
    },
    UnknownCanonicalDimension {
        value: String,
    },
    RetiredDimensionAlias {
        alias: String,
        replacement: &'static str,
    },
    DimensionMismatch {
        expected: CanonicalDimension,
        actual: CanonicalDimension,
    },
    SchemaBindingMismatch {
        expected: CanonicalSchemaBinding,
        actual: CanonicalSchemaBinding,
    },
}

impl fmt::Display for BoundaryMetadataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingField { field } => write!(f, "{field} is required at the boundary"),
            Self::EmptyCollection { field } => {
                write!(f, "{field} must contain at least one entry")
            }
            Self::NonFiniteValue { field } => write!(f, "{field} must be finite"),
            Self::DuplicateLoadId { load_id } => {
                write!(f, "duplicate primitive load id {load_id}")
            }
            Self::LoadCategoryMismatch {
                load_id,
                expected,
                actual,
            } => write!(
                f,
                "primitive load {load_id} has category {actual:?}; expected {expected:?}"
            ),
            Self::UnknownCanonicalDimension { value } => {
                write!(f, "{value} is not an accepted canonical dimension")
            }
            Self::RetiredDimensionAlias { alias, replacement } => {
                write!(f, "{alias} is retired; use {replacement}")
            }
            Self::DimensionMismatch { expected, actual } => write!(
                f,
                "canonical dimension mismatch: expected {}, got {}",
                expected.as_str(),
                actual.as_str()
            ),
            Self::SchemaBindingMismatch { expected, actual } => write!(
                f,
                "schema binding mismatch: expected {}, got {}",
                expected.schema_ref(),
                actual.schema_ref()
            ),
        }
    }
}

impl Error for BoundaryMetadataError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantityUnitMetadata {
    pub unit: String,
    pub dimension: CanonicalDimension,
    pub unit_system_ref: String,
}

impl QuantityUnitMetadata {
    pub fn new(
        unit: impl Into<String>,
        dimension: CanonicalDimension,
        unit_system_ref: impl Into<String>,
    ) -> Result<Self, BoundaryMetadataError> {
        let metadata = Self {
            unit: unit.into(),
            dimension,
            unit_system_ref: unit_system_ref.into(),
        };
        metadata.validate()?;
        Ok(metadata)
    }

    pub fn from_schema_values(
        unit: impl Into<String>,
        dimension: &str,
        unit_system_ref: impl Into<String>,
    ) -> Result<Self, BoundaryMetadataError> {
        Self::new(
            unit,
            CanonicalDimension::from_schema_value(dimension)?,
            unit_system_ref,
        )
    }

    pub fn validate(&self) -> Result<(), BoundaryMetadataError> {
        validate_boundary_ref("unit", &self.unit)?;
        validate_boundary_ref("unit_system_ref", &self.unit_system_ref)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanonicalSchemaBinding {
    ModelLoadCase,
    ModelLoadRecord,
    ModelResultValue,
    ResultsQuantityResult,
}

impl CanonicalSchemaBinding {
    pub fn schema_ref(self) -> &'static str {
        match self {
            Self::ModelLoadCase => "schemas/model.schema.yaml#/$defs/LoadCase",
            Self::ModelLoadRecord => "schemas/model.schema.yaml#/$defs/LoadRecord",
            Self::ModelResultValue => "schemas/model.schema.yaml#/$defs/Result/values",
            Self::ResultsQuantityResult => "schemas/results.schema.yaml#/$defs/QuantityResult",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundaryCanonicalization {
    JsonJcs,
}

impl BoundaryCanonicalization {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::JsonJcs => "JCS",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundaryRecordRef {
    pub record_id: String,
    pub schema_binding: CanonicalSchemaBinding,
    pub target_ref: String,
    pub basis_ref: String,
    pub payload_ref: String,
    pub payload_hash_ref: String,
    pub canonicalization: BoundaryCanonicalization,
}

impl BoundaryRecordRef {
    pub fn new(
        record_id: impl Into<String>,
        schema_binding: CanonicalSchemaBinding,
        target_ref: impl Into<String>,
        basis_ref: impl Into<String>,
        payload_ref: impl Into<String>,
        payload_hash_ref: impl Into<String>,
    ) -> Result<Self, BoundaryMetadataError> {
        let record = Self {
            record_id: record_id.into(),
            schema_binding,
            target_ref: target_ref.into(),
            basis_ref: basis_ref.into(),
            payload_ref: payload_ref.into(),
            payload_hash_ref: payload_hash_ref.into(),
            canonicalization: BoundaryCanonicalization::JsonJcs,
        };
        record.validate()?;
        Ok(record)
    }

    pub fn validate(&self) -> Result<(), BoundaryMetadataError> {
        validate_boundary_ref("record_id", &self.record_id)?;
        validate_boundary_ref("target_ref", &self.target_ref)?;
        validate_boundary_ref("basis_ref", &self.basis_ref)?;
        validate_boundary_ref("payload_ref", &self.payload_ref)?;
        validate_boundary_ref("payload_hash_ref", &self.payload_hash_ref)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BoundaryQuantityRecord {
    pub record: BoundaryRecordRef,
    pub value: f64,
    pub unit: QuantityUnitMetadata,
    pub provenance_ref: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveLoadCaseKind {
    Weight,
    Pressure,
    Thermal,
    ImposedDisplacement,
    Hydrotest,
    Wind,
    Seismic,
    Occasional,
}

impl PrimitiveLoadCaseKind {
    pub fn schema_load_type(self) -> &'static str {
        match self {
            Self::Weight => "weight",
            Self::Pressure => "pressure",
            Self::Thermal => "temperature",
            Self::ImposedDisplacement => "imposed_displacement",
            Self::Hydrotest => "hydrotest",
            Self::Wind => "wind",
            Self::Seismic => "seismic",
            Self::Occasional => "user_occasional",
        }
    }

    pub fn primitive_category(self) -> PrimitiveLoadCategory {
        match self {
            Self::Weight => PrimitiveLoadCategory::Weight,
            Self::Pressure => PrimitiveLoadCategory::Pressure,
            Self::Thermal => PrimitiveLoadCategory::Thermal,
            Self::ImposedDisplacement => PrimitiveLoadCategory::ImposedDisplacement,
            Self::Hydrotest => PrimitiveLoadCategory::Hydrotest,
            Self::Wind => PrimitiveLoadCategory::Wind,
            Self::Seismic => PrimitiveLoadCategory::Seismic,
            Self::Occasional => PrimitiveLoadCategory::Occasional,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrimitiveLoadCaseRecord {
    pub record: BoundaryRecordRef,
    pub name: String,
    pub load_type: PrimitiveLoadCaseKind,
    pub loads: Vec<PrimitiveLoad>,
    pub provenance_ref: String,
}

impl PrimitiveLoadCaseRecord {
    pub fn new(
        record: BoundaryRecordRef,
        name: impl Into<String>,
        load_type: PrimitiveLoadCaseKind,
        loads: Vec<PrimitiveLoad>,
        provenance_ref: impl Into<String>,
    ) -> Result<Self, BoundaryMetadataError> {
        require_schema_binding(record.schema_binding, CanonicalSchemaBinding::ModelLoadCase)?;
        record.validate()?;
        let name = name.into();
        validate_boundary_ref("name", &name)?;
        if loads.is_empty() {
            return Err(BoundaryMetadataError::EmptyCollection { field: "loads" });
        }
        let provenance_ref = provenance_ref.into();
        validate_boundary_ref("provenance_ref", &provenance_ref)?;

        let expected_category = load_type.primitive_category();
        let mut sorted_loads = loads;
        sorted_loads.sort_by(|a, b| a.load_id.cmp(&b.load_id));
        let mut previous_id: Option<&str> = None;
        for load in &sorted_loads {
            validate_boundary_ref("load_id", &load.load_id)?;
            if previous_id == Some(load.load_id.as_str()) {
                return Err(BoundaryMetadataError::DuplicateLoadId {
                    load_id: load.load_id.clone(),
                });
            }
            if load.category != expected_category {
                return Err(BoundaryMetadataError::LoadCategoryMismatch {
                    load_id: load.load_id.clone(),
                    expected: expected_category,
                    actual: load.category,
                });
            }
            previous_id = Some(load.load_id.as_str());
        }

        Ok(Self {
            record,
            name,
            load_type,
            loads: sorted_loads,
            provenance_ref,
        })
    }

    pub fn schema_load_type(&self) -> &'static str {
        self.load_type.schema_load_type()
    }

    pub fn load_ids(&self) -> Vec<&str> {
        self.loads
            .iter()
            .map(|load| load.load_id.as_str())
            .collect()
    }

    pub fn canonical_field_pairs(&self) -> Vec<(&'static str, String)> {
        vec![
            ("basis_ref", self.record.basis_ref.clone()),
            (
                "canonicalization",
                self.record.canonicalization.as_str().to_string(),
            ),
            ("load_ids", self.load_ids().join(",")),
            ("load_type", self.schema_load_type().to_string()),
            ("name", self.name.clone()),
            ("payload_hash_ref", self.record.payload_hash_ref.clone()),
            ("payload_ref", self.record.payload_ref.clone()),
            ("provenance_ref", self.provenance_ref.clone()),
            ("record_id", self.record.record_id.clone()),
            (
                "schema_ref",
                self.record.schema_binding.schema_ref().to_string(),
            ),
            ("target_ref", self.record.target_ref.clone()),
        ]
    }

    pub fn round_trip_key(&self) -> String {
        self.canonical_field_pairs()
            .into_iter()
            .map(|(key, value)| format!("{key}={value}"))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl BoundaryQuantityRecord {
    pub fn new(
        record: BoundaryRecordRef,
        value: f64,
        unit: QuantityUnitMetadata,
        provenance_ref: impl Into<String>,
    ) -> Result<Self, BoundaryMetadataError> {
        if !value.is_finite() {
            return Err(BoundaryMetadataError::NonFiniteValue { field: "value" });
        }
        record.validate()?;
        unit.validate()?;
        let boundary_record = Self {
            record,
            value,
            unit,
            provenance_ref: provenance_ref.into(),
        };
        validate_boundary_ref("provenance_ref", &boundary_record.provenance_ref)?;
        Ok(boundary_record)
    }

    pub fn from_load_dimension(
        record: BoundaryRecordRef,
        value: f64,
        dimension: LoadDimension,
        unit: QuantityUnitMetadata,
        provenance_ref: impl Into<String>,
    ) -> Result<Self, BoundaryMetadataError> {
        Self::from_canonical_dimension(
            record,
            value,
            CanonicalDimension::from_load_dimension(dimension),
            unit,
            provenance_ref,
        )
    }

    pub fn from_canonical_dimension(
        record: BoundaryRecordRef,
        value: f64,
        expected_dimension: CanonicalDimension,
        unit: QuantityUnitMetadata,
        provenance_ref: impl Into<String>,
    ) -> Result<Self, BoundaryMetadataError> {
        if unit.dimension != expected_dimension {
            return Err(BoundaryMetadataError::DimensionMismatch {
                expected: expected_dimension,
                actual: unit.dimension,
            });
        }
        Self::new(record, value, unit, provenance_ref)
    }

    pub fn canonical_field_pairs(&self) -> Vec<(&'static str, String)> {
        vec![
            ("basis_ref", self.record.basis_ref.clone()),
            (
                "canonicalization",
                self.record.canonicalization.as_str().to_string(),
            ),
            ("dimension", self.unit.dimension.as_str().to_string()),
            ("payload_hash_ref", self.record.payload_hash_ref.clone()),
            ("payload_ref", self.record.payload_ref.clone()),
            ("provenance_ref", self.provenance_ref.clone()),
            ("record_id", self.record.record_id.clone()),
            (
                "schema_ref",
                self.record.schema_binding.schema_ref().to_string(),
            ),
            ("target_ref", self.record.target_ref.clone()),
            ("unit", self.unit.unit.clone()),
            ("unit_system_ref", self.unit.unit_system_ref.clone()),
            ("value", self.value.to_string()),
        ]
    }

    pub fn round_trip_key(&self) -> String {
        self.canonical_field_pairs()
            .into_iter()
            .map(|(key, value)| format!("{key}={value}"))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn validate_boundary_ref(field: &'static str, value: &str) -> Result<(), BoundaryMetadataError> {
    if value.trim().is_empty() || value.trim() == "TBD" {
        return Err(BoundaryMetadataError::MissingField { field });
    }
    Ok(())
}

fn require_schema_binding(
    actual: CanonicalSchemaBinding,
    expected: CanonicalSchemaBinding,
) -> Result<(), BoundaryMetadataError> {
    if actual != expected {
        return Err(BoundaryMetadataError::SchemaBindingMismatch { expected, actual });
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LoadQuantity {
    pub value: f64,
    pub dimension: LoadDimension,
}

impl LoadQuantity {
    pub fn new(value: f64, dimension: LoadDimension) -> Result<Self, PrimitiveLoadError> {
        validate_finite("load quantity", value)?;
        Ok(Self { value, dimension })
    }

    pub fn positive(value: f64, dimension: LoadDimension) -> Result<Self, PrimitiveLoadError> {
        validate_positive_finite("load quantity", value)?;
        Ok(Self { value, dimension })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadDirection {
    Dof(FrameDof),
    GlobalX,
    GlobalY,
    GlobalZ,
}

impl LoadDirection {
    pub fn dof_index(self) -> usize {
        match self {
            Self::Dof(dof) => dof.local_index(),
            Self::GlobalX => UX,
            Self::GlobalY => UY,
            Self::GlobalZ => UZ,
        }
    }

    pub fn is_rotational(self) -> bool {
        matches!(self, Self::Dof(FrameDof::Rx | FrameDof::Ry | FrameDof::Rz))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrimitiveLoad {
    pub load_id: String,
    pub category: PrimitiveLoadCategory,
    pub target: Option<LoadTarget>,
    pub direction: LoadDirection,
    pub magnitude: Option<LoadQuantity>,
}

impl PrimitiveLoad {
    pub fn nodal_force(
        load_id: impl Into<String>,
        category: PrimitiveLoadCategory,
        node_index: usize,
        direction: LoadDirection,
        magnitude: LoadQuantity,
    ) -> Self {
        Self {
            load_id: load_id.into(),
            category,
            target: Some(LoadTarget::Node(node_index)),
            direction,
            magnitude: Some(magnitude),
        }
    }

    pub fn uniform_element_load(
        load_id: impl Into<String>,
        category: PrimitiveLoadCategory,
        element_index: usize,
        direction: LoadDirection,
        magnitude: LoadQuantity,
    ) -> Self {
        Self {
            load_id: load_id.into(),
            category,
            target: Some(LoadTarget::Element(element_index)),
            direction,
            magnitude: Some(magnitude),
        }
    }

    pub fn imposed_displacement(
        load_id: impl Into<String>,
        node_index: usize,
        dof: FrameDof,
        magnitude: LoadQuantity,
    ) -> Self {
        Self {
            load_id: load_id.into(),
            category: PrimitiveLoadCategory::ImposedDisplacement,
            target: Some(LoadTarget::Support { node_index, dof }),
            direction: LoadDirection::Dof(dof),
            magnitude: Some(magnitude),
        }
    }

    pub fn missing_target(
        load_id: impl Into<String>,
        category: PrimitiveLoadCategory,
        direction: LoadDirection,
        magnitude: Option<LoadQuantity>,
    ) -> Self {
        Self {
            load_id: load_id.into(),
            category,
            target: None,
            direction,
            magnitude,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodalLoadContribution {
    pub load_id: String,
    pub node_index: usize,
    pub global_dof: usize,
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElementUniformLoadContribution {
    pub load_id: String,
    pub element_index: usize,
    pub direction: LoadDirection,
    pub magnitude: LoadQuantity,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ElementLoadSpan {
    pub element_index: usize,
    pub node_i: usize,
    pub node_j: usize,
    pub span: f64,
}

impl ElementLoadSpan {
    pub fn new(
        element_index: usize,
        node_i: usize,
        node_j: usize,
        span: f64,
    ) -> Result<Self, PrimitiveLoadError> {
        validate_positive_finite("element span", span)?;
        Ok(Self {
            element_index,
            node_i,
            node_j,
            span,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ElementAxialEffectProperties {
    pub element_index: usize,
    pub elastic_modulus: Option<f64>,
    pub area: Option<f64>,
    pub thermal_expansion_coefficient: Option<f64>,
    pub internal_area: Option<f64>,
}

impl ElementAxialEffectProperties {
    pub fn new(
        element_index: usize,
        elastic_modulus: Option<f64>,
        area: Option<f64>,
        thermal_expansion_coefficient: Option<f64>,
        internal_area: Option<f64>,
    ) -> Self {
        Self {
            element_index,
            elastic_modulus,
            area,
            thermal_expansion_coefficient,
            internal_area,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrimitiveAxialEffectContribution {
    pub load_id: String,
    pub element_index: usize,
    pub axial_force: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrimitiveAxialEffectApplication {
    pub axial_effects: Vec<PrimitiveAxialEffectContribution>,
    pub findings: Vec<LoadFinding>,
}

impl PrimitiveAxialEffectApplication {
    pub fn is_blocked(&self) -> bool {
        !self.findings.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImposedDisplacementContribution {
    pub load_id: String,
    pub node_dof: NodeDof,
    pub value: LoadQuantity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindingCode {
    MissingLoadTarget,
    MissingLoadMagnitude,
    NodeOutOfRange,
    ElementOutOfRange,
    InvalidLoadDimension,
    InvalidLoadDirection,
    MissingElementSpan,
    InvalidElementSpan,
    InvalidElementConnectivity,
    MissingElementProperties,
    InvalidElementProperties,
    MissingPhysicalProperty,
    InvalidPhysicalProperty,
    NonFiniteLoadMagnitude,
    NonFiniteAxialEffect,
    UnsupportedTargetForCategory,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoadFinding {
    pub code: FindingCode,
    pub load_id: String,
    pub message: String,
}

impl LoadFinding {
    fn new(code: FindingCode, load_id: &str, message: impl Into<String>) -> Self {
        Self {
            code,
            load_id: load_id.to_string(),
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoadApplication {
    pub nodal_loads: Vec<NodalLoadContribution>,
    pub element_uniform_loads: Vec<ElementUniformLoadContribution>,
    pub imposed_displacements: Vec<ImposedDisplacementContribution>,
    pub findings: Vec<LoadFinding>,
}

impl LoadApplication {
    pub fn is_blocked(&self) -> bool {
        !self.findings.is_empty()
    }

    pub fn global_load_vector(&self, node_count: usize) -> Vec<f64> {
        let mut vector = vec![0.0; node_count * DOF_PER_NODE];
        for load in &self.nodal_loads {
            if load.global_dof < vector.len() {
                vector[load.global_dof] += load.value;
            }
        }
        vector
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SolverNodalLoadContribution {
    pub source_id: String,
    pub node_index: usize,
    pub global_dof: usize,
    pub value: f64,
}

impl SolverNodalLoadContribution {
    pub fn new(
        source_id: impl Into<String>,
        node_index: usize,
        global_dof: usize,
        value: f64,
    ) -> Self {
        Self {
            source_id: source_id.into(),
            node_index,
            global_dof,
            value,
        }
    }

    pub fn from_primitive(load: &NodalLoadContribution) -> Self {
        Self {
            source_id: load.load_id.clone(),
            node_index: load.node_index,
            global_dof: load.global_dof,
            value: load.value,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadCaseAssemblyFindingCode {
    NodeOutOfRange,
    DofOutOfRange,
    NonFiniteContribution,
    NodeDofMismatch,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoadCaseAssemblyFinding {
    pub code: LoadCaseAssemblyFindingCode,
    pub source_id: String,
    pub message: String,
}

impl LoadCaseAssemblyFinding {
    fn new(
        code: LoadCaseAssemblyFindingCode,
        source_id: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            source_id: source_id.into(),
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoadCaseAssembly {
    pub global_load_vector: Vec<f64>,
    pub sorted_contributions: Vec<SolverNodalLoadContribution>,
    pub findings: Vec<LoadCaseAssemblyFinding>,
}

impl LoadCaseAssembly {
    pub fn is_blocked(&self) -> bool {
        !self.findings.is_empty()
    }
}

pub fn assemble_solver_load_vector(
    node_count: usize,
    contributions: &[SolverNodalLoadContribution],
) -> LoadCaseAssembly {
    let mut sorted = contributions.to_vec();
    sorted.sort_by(|a, b| {
        a.node_index
            .cmp(&b.node_index)
            .then(a.global_dof.cmp(&b.global_dof))
            .then(a.source_id.cmp(&b.source_id))
    });

    let mut vector = vec![0.0; node_count * DOF_PER_NODE];
    let mut accepted = Vec::new();
    let mut findings = Vec::new();

    for contribution in sorted {
        if contribution.node_index >= node_count {
            findings.push(LoadCaseAssemblyFinding::new(
                LoadCaseAssemblyFindingCode::NodeOutOfRange,
                contribution.source_id,
                format!(
                    "node index {} is outside node count {node_count}",
                    contribution.node_index
                ),
            ));
            continue;
        }
        if contribution.global_dof >= vector.len() {
            findings.push(LoadCaseAssemblyFinding::new(
                LoadCaseAssemblyFindingCode::DofOutOfRange,
                contribution.source_id,
                format!(
                    "global dof {} is outside vector length {}",
                    contribution.global_dof,
                    vector.len()
                ),
            ));
            continue;
        }
        let expected_node = contribution.global_dof / DOF_PER_NODE;
        if expected_node != contribution.node_index {
            findings.push(LoadCaseAssemblyFinding::new(
                LoadCaseAssemblyFindingCode::NodeDofMismatch,
                contribution.source_id,
                format!(
                    "global dof {} maps to node {expected_node}, not node {}",
                    contribution.global_dof, contribution.node_index
                ),
            ));
            continue;
        }
        if !contribution.value.is_finite() {
            findings.push(LoadCaseAssemblyFinding::new(
                LoadCaseAssemblyFindingCode::NonFiniteContribution,
                contribution.source_id,
                "solver load contribution must be finite",
            ));
            continue;
        }
        vector[contribution.global_dof] += contribution.value;
        accepted.push(contribution);
    }

    LoadCaseAssembly {
        global_load_vector: if findings.is_empty() {
            vector
        } else {
            Vec::new()
        },
        sorted_contributions: if findings.is_empty() {
            accepted
        } else {
            Vec::new()
        },
        findings,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveLoadError {
    NonFiniteInput { name: &'static str, value: f64 },
    NonPositiveInput { name: &'static str, value: f64 },
}

impl fmt::Display for PrimitiveLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteInput { name, value } => {
                write!(f, "{name} must be finite, got {value}")
            }
            Self::NonPositiveInput { name, value } => {
                write!(f, "{name} must be positive, got {value}")
            }
        }
    }
}

impl Error for PrimitiveLoadError {}

pub fn prepare_loads(
    node_count: usize,
    element_count: usize,
    loads: &[PrimitiveLoad],
) -> LoadApplication {
    let mut nodal_loads = Vec::new();
    let mut element_uniform_loads = Vec::new();
    let mut imposed_displacements = Vec::new();
    let mut findings = Vec::new();

    for load in loads {
        let Some(target) = load.target else {
            findings.push(LoadFinding::new(
                FindingCode::MissingLoadTarget,
                &load.load_id,
                "primitive load requires an explicit target",
            ));
            continue;
        };
        let Some(magnitude) = load.magnitude else {
            findings.push(LoadFinding::new(
                FindingCode::MissingLoadMagnitude,
                &load.load_id,
                "primitive load requires an explicit magnitude",
            ));
            continue;
        };

        match target {
            LoadTarget::Node(node_index) => prepare_node_load(
                load,
                node_index,
                magnitude,
                node_count,
                &mut nodal_loads,
                &mut findings,
            ),
            LoadTarget::Element(element_index) => prepare_element_load(
                load,
                element_index,
                magnitude,
                element_count,
                &mut element_uniform_loads,
                &mut findings,
            ),
            LoadTarget::Support { node_index, dof } => prepare_support_load(
                load,
                node_index,
                dof,
                magnitude,
                node_count,
                &mut imposed_displacements,
                &mut findings,
            ),
        }
    }

    nodal_loads.sort_by(|a, b| {
        a.node_index
            .cmp(&b.node_index)
            .then(a.global_dof.cmp(&b.global_dof))
            .then(a.load_id.cmp(&b.load_id))
    });
    element_uniform_loads.sort_by(|a, b| {
        a.element_index
            .cmp(&b.element_index)
            .then(a.direction.dof_index().cmp(&b.direction.dof_index()))
            .then(a.load_id.cmp(&b.load_id))
    });
    imposed_displacements.sort_by(|a, b| {
        a.node_dof
            .global_index()
            .cmp(&b.node_dof.global_index())
            .then(a.load_id.cmp(&b.load_id))
    });

    LoadApplication {
        nodal_loads,
        element_uniform_loads,
        imposed_displacements,
        findings,
    }
}

pub fn prepare_lumped_nodal_loads(
    node_count: usize,
    element_count: usize,
    element_spans: &[ElementLoadSpan],
    loads: &[PrimitiveLoad],
) -> LoadApplication {
    let mut nodal_loads = Vec::new();
    let mut findings = Vec::new();

    for load in loads {
        let Some(target) = load.target else {
            findings.push(LoadFinding::new(
                FindingCode::MissingLoadTarget,
                &load.load_id,
                "primitive load requires an explicit target",
            ));
            continue;
        };
        let Some(magnitude) = load.magnitude else {
            findings.push(LoadFinding::new(
                FindingCode::MissingLoadMagnitude,
                &load.load_id,
                "primitive load requires an explicit magnitude",
            ));
            continue;
        };

        let LoadTarget::Element(element_index) = target else {
            findings.push(LoadFinding::new(
                FindingCode::UnsupportedTargetForCategory,
                &load.load_id,
                "lumped equivalent nodal conversion requires an element load target",
            ));
            continue;
        };

        if element_index >= element_count {
            findings.push(LoadFinding::new(
                FindingCode::ElementOutOfRange,
                &load.load_id,
                format!("element index {element_index} is outside element count {element_count}"),
            ));
            continue;
        }

        if !category_allows_lumped_nodal_conversion(load.category) {
            findings.push(LoadFinding::new(
                FindingCode::UnsupportedTargetForCategory,
                &load.load_id,
                format!(
                    "{:?} is not supported by lumped equivalent nodal conversion",
                    load.category
                ),
            ));
            continue;
        }

        if magnitude.dimension != LoadDimension::ForcePerLength {
            findings.push(LoadFinding::new(
                FindingCode::InvalidLoadDimension,
                &load.load_id,
                "lumped equivalent nodal conversion requires ForcePerLength magnitude",
            ));
            continue;
        }

        if !valid_lumped_nodal_direction(load.direction) {
            findings.push(LoadFinding::new(
                FindingCode::InvalidLoadDirection,
                &load.load_id,
                "lumped equivalent nodal conversion requires a translational direction",
            ));
            continue;
        }

        let Some(span) = find_element_span(element_spans, element_index, load, &mut findings)
        else {
            continue;
        };

        if !valid_element_span(span, load, node_count, &mut findings) {
            continue;
        }

        let half_total = magnitude.value * span.span * 0.5;
        let dof_index = load.direction.dof_index();
        nodal_loads.push(NodalLoadContribution {
            load_id: load.load_id.clone(),
            node_index: span.node_i,
            global_dof: span.node_i * DOF_PER_NODE + dof_index,
            value: half_total,
        });
        nodal_loads.push(NodalLoadContribution {
            load_id: load.load_id.clone(),
            node_index: span.node_j,
            global_dof: span.node_j * DOF_PER_NODE + dof_index,
            value: half_total,
        });
    }

    sort_nodal_loads(&mut nodal_loads);

    LoadApplication {
        nodal_loads,
        element_uniform_loads: Vec::new(),
        imposed_displacements: Vec::new(),
        findings,
    }
}

pub fn prepare_straight_pipe_axial_effects(
    element_count: usize,
    loads: &[PrimitiveLoad],
    properties: &[ElementAxialEffectProperties],
) -> PrimitiveAxialEffectApplication {
    let mut axial_effects = Vec::new();
    let mut findings = Vec::new();

    for load in loads {
        let Some(target) = load.target else {
            findings.push(LoadFinding::new(
                FindingCode::MissingLoadTarget,
                &load.load_id,
                "straight-pipe axial effect requires an explicit element target",
            ));
            continue;
        };
        let Some(magnitude) = load.magnitude else {
            findings.push(LoadFinding::new(
                FindingCode::MissingLoadMagnitude,
                &load.load_id,
                "straight-pipe axial effect requires an explicit magnitude",
            ));
            continue;
        };
        let LoadTarget::Element(element_index) = target else {
            findings.push(LoadFinding::new(
                FindingCode::UnsupportedTargetForCategory,
                &load.load_id,
                "straight-pipe axial effect preparation requires an element load target",
            ));
            continue;
        };
        if element_index >= element_count {
            findings.push(LoadFinding::new(
                FindingCode::ElementOutOfRange,
                &load.load_id,
                format!("element index {element_index} is outside element count {element_count}"),
            ));
            continue;
        }
        let Some(magnitude_value) = finite_load_magnitude(load, magnitude, &mut findings) else {
            continue;
        };

        match load.category {
            PrimitiveLoadCategory::Thermal => {
                if magnitude.dimension != LoadDimension::TemperatureChange {
                    findings.push(LoadFinding::new(
                        FindingCode::InvalidLoadDimension,
                        &load.load_id,
                        "thermal axial effect requires TemperatureChange magnitude",
                    ));
                    continue;
                }
                let Some(element_properties) =
                    find_axial_effect_properties(properties, element_index, load, &mut findings)
                else {
                    continue;
                };
                let Some(elastic_modulus) = positive_physical_property(
                    element_properties.elastic_modulus,
                    "elastic_modulus",
                    load,
                    &mut findings,
                ) else {
                    continue;
                };
                let Some(area) = positive_physical_property(
                    element_properties.area,
                    "area",
                    load,
                    &mut findings,
                ) else {
                    continue;
                };
                let Some(thermal_expansion_coefficient) = finite_physical_property(
                    element_properties.thermal_expansion_coefficient,
                    "thermal_expansion_coefficient",
                    load,
                    &mut findings,
                ) else {
                    continue;
                };
                let axial_force =
                    elastic_modulus * area * thermal_expansion_coefficient * magnitude_value;
                push_axial_effect_if_finite(
                    &mut axial_effects,
                    load,
                    element_index,
                    axial_force,
                    &mut findings,
                );
            }
            PrimitiveLoadCategory::Pressure => {
                if magnitude.dimension != LoadDimension::Pressure {
                    findings.push(LoadFinding::new(
                        FindingCode::InvalidLoadDimension,
                        &load.load_id,
                        "pressure thrust axial effect requires Pressure magnitude",
                    ));
                    continue;
                }
                let Some(element_properties) =
                    find_axial_effect_properties(properties, element_index, load, &mut findings)
                else {
                    continue;
                };
                let Some(internal_area) = positive_physical_property(
                    element_properties.internal_area,
                    "internal_area",
                    load,
                    &mut findings,
                ) else {
                    continue;
                };
                let axial_force = magnitude_value * internal_area;
                push_axial_effect_if_finite(
                    &mut axial_effects,
                    load,
                    element_index,
                    axial_force,
                    &mut findings,
                );
            }
            _ => findings.push(LoadFinding::new(
                FindingCode::UnsupportedTargetForCategory,
                &load.load_id,
                format!(
                    "{:?} does not define a straight-pipe axial effect in this helper",
                    load.category
                ),
            )),
        }
    }

    axial_effects.sort_by(|a, b| {
        a.element_index
            .cmp(&b.element_index)
            .then(a.load_id.cmp(&b.load_id))
    });

    PrimitiveAxialEffectApplication {
        axial_effects: if findings.is_empty() {
            axial_effects
        } else {
            Vec::new()
        },
        findings,
    }
}

fn prepare_node_load(
    load: &PrimitiveLoad,
    node_index: usize,
    magnitude: LoadQuantity,
    node_count: usize,
    nodal_loads: &mut Vec<NodalLoadContribution>,
    findings: &mut Vec<LoadFinding>,
) {
    if node_index >= node_count {
        findings.push(LoadFinding::new(
            FindingCode::NodeOutOfRange,
            &load.load_id,
            format!("node index {node_index} is outside node count {node_count}"),
        ));
        return;
    }

    if !category_allows_node_target(load.category) {
        findings.push(LoadFinding::new(
            FindingCode::UnsupportedTargetForCategory,
            &load.load_id,
            format!("{:?} is not a nodal primitive load target", load.category),
        ));
        return;
    }

    if !valid_nodal_dimension(load.direction, magnitude.dimension) {
        findings.push(LoadFinding::new(
            FindingCode::InvalidLoadDimension,
            &load.load_id,
            "nodal load dimension must match translational force or rotational moment",
        ));
        return;
    }

    nodal_loads.push(NodalLoadContribution {
        load_id: load.load_id.clone(),
        node_index,
        global_dof: node_index * DOF_PER_NODE + load.direction.dof_index(),
        value: magnitude.value,
    });
}

fn prepare_element_load(
    load: &PrimitiveLoad,
    element_index: usize,
    magnitude: LoadQuantity,
    element_count: usize,
    element_uniform_loads: &mut Vec<ElementUniformLoadContribution>,
    findings: &mut Vec<LoadFinding>,
) {
    if element_index >= element_count {
        findings.push(LoadFinding::new(
            FindingCode::ElementOutOfRange,
            &load.load_id,
            format!("element index {element_index} is outside element count {element_count}"),
        ));
        return;
    }

    if !category_allows_element_target(load.category) {
        findings.push(LoadFinding::new(
            FindingCode::UnsupportedTargetForCategory,
            &load.load_id,
            format!(
                "{:?} is not an element primitive load target",
                load.category
            ),
        ));
        return;
    }

    if load.category != PrimitiveLoadCategory::Thermal
        && magnitude.dimension != LoadDimension::Pressure
        && load.direction.is_rotational()
    {
        findings.push(LoadFinding::new(
            FindingCode::InvalidLoadDirection,
            &load.load_id,
            "element uniform primitive loads require a translational direction",
        ));
        return;
    }

    if !valid_element_dimension(load.category, magnitude.dimension) {
        findings.push(LoadFinding::new(
            FindingCode::InvalidLoadDimension,
            &load.load_id,
            "element primitive load dimension is not valid for the category",
        ));
        return;
    }

    element_uniform_loads.push(ElementUniformLoadContribution {
        load_id: load.load_id.clone(),
        element_index,
        direction: load.direction,
        magnitude,
    });
}

fn prepare_support_load(
    load: &PrimitiveLoad,
    node_index: usize,
    dof: FrameDof,
    magnitude: LoadQuantity,
    node_count: usize,
    imposed_displacements: &mut Vec<ImposedDisplacementContribution>,
    findings: &mut Vec<LoadFinding>,
) {
    if node_index >= node_count {
        findings.push(LoadFinding::new(
            FindingCode::NodeOutOfRange,
            &load.load_id,
            format!("node index {node_index} is outside node count {node_count}"),
        ));
        return;
    }

    if load.category != PrimitiveLoadCategory::ImposedDisplacement {
        findings.push(LoadFinding::new(
            FindingCode::UnsupportedTargetForCategory,
            &load.load_id,
            "support-target primitive loads are limited to imposed displacement",
        ));
        return;
    }

    if !valid_imposed_dimension(dof, magnitude.dimension) {
        findings.push(LoadFinding::new(
            FindingCode::InvalidLoadDimension,
            &load.load_id,
            "imposed displacement dimension must match translational or rotational DOF",
        ));
        return;
    }

    imposed_displacements.push(ImposedDisplacementContribution {
        load_id: load.load_id.clone(),
        node_dof: NodeDof::new(node_index, dof),
        value: magnitude,
    });
}

fn category_allows_node_target(category: PrimitiveLoadCategory) -> bool {
    matches!(
        category,
        PrimitiveLoadCategory::Wind
            | PrimitiveLoadCategory::Seismic
            | PrimitiveLoadCategory::Occasional
    )
}

fn category_allows_element_target(category: PrimitiveLoadCategory) -> bool {
    matches!(
        category,
        PrimitiveLoadCategory::Weight
            | PrimitiveLoadCategory::Pressure
            | PrimitiveLoadCategory::Thermal
            | PrimitiveLoadCategory::Hydrotest
            | PrimitiveLoadCategory::Wind
            | PrimitiveLoadCategory::Seismic
            | PrimitiveLoadCategory::Occasional
    )
}

fn category_allows_lumped_nodal_conversion(category: PrimitiveLoadCategory) -> bool {
    matches!(
        category,
        PrimitiveLoadCategory::Weight
            | PrimitiveLoadCategory::Hydrotest
            | PrimitiveLoadCategory::Wind
            | PrimitiveLoadCategory::Seismic
            | PrimitiveLoadCategory::Occasional
    )
}

fn valid_nodal_dimension(direction: LoadDirection, dimension: LoadDimension) -> bool {
    match (direction.is_rotational(), dimension) {
        (true, LoadDimension::Moment) => true,
        (false, LoadDimension::Force) => true,
        _ => false,
    }
}

fn valid_element_dimension(category: PrimitiveLoadCategory, dimension: LoadDimension) -> bool {
    match category {
        PrimitiveLoadCategory::Weight
        | PrimitiveLoadCategory::Wind
        | PrimitiveLoadCategory::Seismic
        | PrimitiveLoadCategory::Occasional => dimension == LoadDimension::ForcePerLength,
        PrimitiveLoadCategory::Pressure | PrimitiveLoadCategory::Hydrotest => {
            matches!(
                dimension,
                LoadDimension::Pressure | LoadDimension::ForcePerLength
            )
        }
        PrimitiveLoadCategory::Thermal => dimension == LoadDimension::TemperatureChange,
        PrimitiveLoadCategory::ImposedDisplacement => false,
    }
}

fn valid_imposed_dimension(dof: FrameDof, dimension: LoadDimension) -> bool {
    match (dof.is_translational(), dimension) {
        (true, LoadDimension::Displacement) => true,
        (false, LoadDimension::Rotation) => true,
        _ => false,
    }
}

fn valid_lumped_nodal_direction(direction: LoadDirection) -> bool {
    match direction {
        LoadDirection::Dof(dof) => dof.is_translational(),
        LoadDirection::GlobalX | LoadDirection::GlobalY | LoadDirection::GlobalZ => true,
    }
}

fn find_element_span<'a>(
    element_spans: &'a [ElementLoadSpan],
    element_index: usize,
    load: &PrimitiveLoad,
    findings: &mut Vec<LoadFinding>,
) -> Option<&'a ElementLoadSpan> {
    let mut matches = element_spans
        .iter()
        .filter(|span| span.element_index == element_index);
    let first = matches.next();
    if first.is_none() {
        findings.push(LoadFinding::new(
            FindingCode::MissingElementSpan,
            &load.load_id,
            format!("element {element_index} requires supplied span and connectivity"),
        ));
        return None;
    }
    if matches.next().is_some() {
        findings.push(LoadFinding::new(
            FindingCode::InvalidElementSpan,
            &load.load_id,
            format!("element {element_index} has duplicate span/connectivity records"),
        ));
        return None;
    }
    first
}

fn valid_element_span(
    span: &ElementLoadSpan,
    load: &PrimitiveLoad,
    node_count: usize,
    findings: &mut Vec<LoadFinding>,
) -> bool {
    if !span.span.is_finite() {
        findings.push(LoadFinding::new(
            FindingCode::InvalidElementSpan,
            &load.load_id,
            format!("element span must be finite, got {}", span.span),
        ));
        return false;
    }
    if span.span <= 0.0 {
        findings.push(LoadFinding::new(
            FindingCode::InvalidElementSpan,
            &load.load_id,
            format!("element span must be positive, got {}", span.span),
        ));
        return false;
    }
    if span.node_i >= node_count {
        findings.push(LoadFinding::new(
            FindingCode::NodeOutOfRange,
            &load.load_id,
            format!(
                "element node_i {} is outside node count {node_count}",
                span.node_i
            ),
        ));
        return false;
    }
    if span.node_j >= node_count {
        findings.push(LoadFinding::new(
            FindingCode::NodeOutOfRange,
            &load.load_id,
            format!(
                "element node_j {} is outside node count {node_count}",
                span.node_j
            ),
        ));
        return false;
    }
    if span.node_i == span.node_j {
        findings.push(LoadFinding::new(
            FindingCode::InvalidElementConnectivity,
            &load.load_id,
            format!("element connects node index {} to itself", span.node_i),
        ));
        return false;
    }
    true
}

fn finite_load_magnitude(
    load: &PrimitiveLoad,
    magnitude: LoadQuantity,
    findings: &mut Vec<LoadFinding>,
) -> Option<f64> {
    if !magnitude.value.is_finite() {
        findings.push(LoadFinding::new(
            FindingCode::NonFiniteLoadMagnitude,
            &load.load_id,
            format!("load magnitude must be finite, got {}", magnitude.value),
        ));
        return None;
    }
    Some(magnitude.value)
}

fn find_axial_effect_properties<'a>(
    properties: &'a [ElementAxialEffectProperties],
    element_index: usize,
    load: &PrimitiveLoad,
    findings: &mut Vec<LoadFinding>,
) -> Option<&'a ElementAxialEffectProperties> {
    let mut matches = properties
        .iter()
        .filter(|property| property.element_index == element_index);
    let first = matches.next();
    if first.is_none() {
        findings.push(LoadFinding::new(
            FindingCode::MissingElementProperties,
            &load.load_id,
            format!("element {element_index} requires supplied axial effect properties"),
        ));
        return None;
    }
    if matches.next().is_some() {
        findings.push(LoadFinding::new(
            FindingCode::InvalidElementProperties,
            &load.load_id,
            format!("element {element_index} has duplicate axial effect property records"),
        ));
        return None;
    }
    first
}

fn finite_physical_property(
    value: Option<f64>,
    field: &'static str,
    load: &PrimitiveLoad,
    findings: &mut Vec<LoadFinding>,
) -> Option<f64> {
    let Some(value) = value else {
        findings.push(LoadFinding::new(
            FindingCode::MissingPhysicalProperty,
            &load.load_id,
            format!("{field} is required for straight-pipe axial effect preparation"),
        ));
        return None;
    };
    if !value.is_finite() {
        findings.push(LoadFinding::new(
            FindingCode::InvalidPhysicalProperty,
            &load.load_id,
            format!("{field} must be finite, got {value}"),
        ));
        return None;
    }
    Some(value)
}

fn positive_physical_property(
    value: Option<f64>,
    field: &'static str,
    load: &PrimitiveLoad,
    findings: &mut Vec<LoadFinding>,
) -> Option<f64> {
    let value = finite_physical_property(value, field, load, findings)?;
    if value <= 0.0 {
        findings.push(LoadFinding::new(
            FindingCode::InvalidPhysicalProperty,
            &load.load_id,
            format!("{field} must be positive, got {value}"),
        ));
        return None;
    }
    Some(value)
}

fn push_axial_effect_if_finite(
    axial_effects: &mut Vec<PrimitiveAxialEffectContribution>,
    load: &PrimitiveLoad,
    element_index: usize,
    axial_force: f64,
    findings: &mut Vec<LoadFinding>,
) {
    if !axial_force.is_finite() {
        findings.push(LoadFinding::new(
            FindingCode::NonFiniteAxialEffect,
            &load.load_id,
            format!("computed axial force must be finite, got {axial_force}"),
        ));
        return;
    }
    axial_effects.push(PrimitiveAxialEffectContribution {
        load_id: load.load_id.clone(),
        element_index,
        axial_force,
    });
}

fn sort_nodal_loads(nodal_loads: &mut [NodalLoadContribution]) {
    nodal_loads.sort_by(|a, b| {
        a.node_index
            .cmp(&b.node_index)
            .then(a.global_dof.cmp(&b.global_dof))
            .then(a.load_id.cmp(&b.load_id))
    });
}

fn validate_finite(name: &'static str, value: f64) -> Result<(), PrimitiveLoadError> {
    if !value.is_finite() {
        return Err(PrimitiveLoadError::NonFiniteInput { name, value });
    }
    Ok(())
}

fn validate_positive_finite(name: &'static str, value: f64) -> Result<(), PrimitiveLoadError> {
    validate_finite(name, value)?;
    if value <= 0.0 {
        return Err(PrimitiveLoadError::NonPositiveInput { name, value });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use open_pipe_stress_frame_kernel::RZ;

    fn q(value: f64, dimension: LoadDimension) -> LoadQuantity {
        LoadQuantity::new(value, dimension).unwrap()
    }

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 1.0e-9,
            "expected {actual} to be within tolerance of {expected}"
        );
    }

    fn record_ref(schema_binding: CanonicalSchemaBinding) -> BoundaryRecordRef {
        BoundaryRecordRef::new(
            "load-record:weight-1",
            schema_binding,
            "element:0",
            "load-case:sustain",
            "payload:model",
            "hash:model",
        )
        .unwrap()
    }

    fn load_case_ref() -> BoundaryRecordRef {
        BoundaryRecordRef::new(
            "load-case:weight",
            CanonicalSchemaBinding::ModelLoadCase,
            "model:example",
            "load-basis:manual",
            "payload:model",
            "hash:model",
        )
        .unwrap()
    }

    fn weight_line_load(load_id: &str) -> PrimitiveLoad {
        PrimitiveLoad::uniform_element_load(
            load_id,
            PrimitiveLoadCategory::Weight,
            0,
            LoadDirection::GlobalZ,
            q(-10.0, LoadDimension::ForcePerLength),
        )
    }

    #[test]
    fn canonical_dimension_parser_rejects_retired_aliases() {
        assert_eq!(
            CanonicalDimension::from_schema_value("temperature_interval").unwrap(),
            CanonicalDimension::TemperatureInterval
        );
        assert_eq!(
            CanonicalDimension::from_schema_value("force_per_length").unwrap(),
            CanonicalDimension::ForcePerLength
        );
        assert_eq!(
            CanonicalDimension::from_load_dimension(LoadDimension::ForcePerLength),
            CanonicalDimension::ForcePerLength
        );

        let err = CanonicalDimension::from_schema_value("temperature_difference").unwrap_err();
        assert!(matches!(
            err,
            BoundaryMetadataError::RetiredDimensionAlias {
                replacement: "temperature_interval",
                ..
            }
        ));

        let err = CanonicalDimension::from_schema_value("area_moment").unwrap_err();
        assert!(matches!(
            err,
            BoundaryMetadataError::RetiredDimensionAlias {
                replacement: "second_moment_area",
                ..
            }
        ));

        let err = CanonicalDimension::from_schema_value("stiffness").unwrap_err();
        assert!(matches!(
            err,
            BoundaryMetadataError::RetiredDimensionAlias {
                replacement: "linear_stiffness or rotational_stiffness",
                ..
            }
        ));
    }

    #[test]
    fn boundary_quantity_record_carries_unit_schema_and_hash_metadata() {
        let unit =
            QuantityUnitMetadata::new("N", CanonicalDimension::Force, "unit-system:si").unwrap();

        let record = BoundaryQuantityRecord::from_load_dimension(
            record_ref(CanonicalSchemaBinding::ModelLoadRecord),
            42.0,
            LoadDimension::Force,
            unit,
            "provenance:manual",
        )
        .unwrap();

        assert_eq!(record.unit.unit, "N");
        assert_eq!(record.unit.dimension.as_str(), "force");
        assert_eq!(
            record.record.schema_binding.schema_ref(),
            "schemas/model.schema.yaml#/$defs/LoadRecord"
        );
        assert!(record
            .round_trip_key()
            .contains("payload_hash_ref=hash:model"));
    }

    #[test]
    fn boundary_quantity_record_rejects_missing_unit_and_dimension_mismatch() {
        let missing_unit =
            QuantityUnitMetadata::new("", CanonicalDimension::Force, "unit-system:si").unwrap_err();
        assert_eq!(
            missing_unit,
            BoundaryMetadataError::MissingField { field: "unit" }
        );

        let unit =
            QuantityUnitMetadata::new("N*m", CanonicalDimension::Moment, "unit-system:si").unwrap();
        let err = BoundaryQuantityRecord::from_load_dimension(
            record_ref(CanonicalSchemaBinding::ModelLoadRecord),
            42.0,
            LoadDimension::Force,
            unit,
            "provenance:manual",
        )
        .unwrap_err();
        assert_eq!(
            err,
            BoundaryMetadataError::DimensionMismatch {
                expected: CanonicalDimension::Force,
                actual: CanonicalDimension::Moment,
            }
        );
    }

    #[test]
    fn primitive_load_case_record_binds_model_load_case_and_sorts_load_ids() {
        let record = PrimitiveLoadCaseRecord::new(
            load_case_ref(),
            "Sustained weight",
            PrimitiveLoadCaseKind::Weight,
            vec![weight_line_load("z-load"), weight_line_load("a-load")],
            "provenance:manual",
        )
        .unwrap();

        assert_eq!(
            record.record.schema_binding.schema_ref(),
            "schemas/model.schema.yaml#/$defs/LoadCase"
        );
        assert_eq!(record.schema_load_type(), "weight");
        assert_eq!(record.load_ids(), vec!["a-load", "z-load"]);
        assert!(record.round_trip_key().contains("load_ids=a-load,z-load"));
        assert!(record.round_trip_key().contains("load_type=weight"));
        assert!(record
            .round_trip_key()
            .contains("schema_ref=schemas/model.schema.yaml#/$defs/LoadCase"));
    }

    #[test]
    fn primitive_load_case_kind_maps_to_schema_load_type_values() {
        assert_eq!(
            PrimitiveLoadCaseKind::Thermal.schema_load_type(),
            "temperature"
        );
        assert_eq!(
            PrimitiveLoadCaseKind::Occasional.schema_load_type(),
            "user_occasional"
        );
    }

    #[test]
    fn primitive_load_case_record_rejects_wrong_schema_and_missing_metadata() {
        let wrong_schema = PrimitiveLoadCaseRecord::new(
            record_ref(CanonicalSchemaBinding::ModelLoadRecord),
            "Sustained weight",
            PrimitiveLoadCaseKind::Weight,
            vec![weight_line_load("weight-1")],
            "provenance:manual",
        )
        .unwrap_err();
        assert_eq!(
            wrong_schema,
            BoundaryMetadataError::SchemaBindingMismatch {
                expected: CanonicalSchemaBinding::ModelLoadCase,
                actual: CanonicalSchemaBinding::ModelLoadRecord,
            }
        );

        let missing_name = PrimitiveLoadCaseRecord::new(
            load_case_ref(),
            " ",
            PrimitiveLoadCaseKind::Weight,
            vec![weight_line_load("weight-1")],
            "provenance:manual",
        )
        .unwrap_err();
        assert_eq!(
            missing_name,
            BoundaryMetadataError::MissingField { field: "name" }
        );

        let missing_provenance = PrimitiveLoadCaseRecord::new(
            load_case_ref(),
            "Sustained weight",
            PrimitiveLoadCaseKind::Weight,
            vec![weight_line_load("weight-1")],
            "TBD",
        )
        .unwrap_err();
        assert_eq!(
            missing_provenance,
            BoundaryMetadataError::MissingField {
                field: "provenance_ref"
            }
        );

        let missing_record_id = PrimitiveLoadCaseRecord::new(
            BoundaryRecordRef {
                record_id: "TBD".to_string(),
                schema_binding: CanonicalSchemaBinding::ModelLoadCase,
                target_ref: "model:example".to_string(),
                basis_ref: "load-basis:manual".to_string(),
                payload_ref: "payload:model".to_string(),
                payload_hash_ref: "hash:model".to_string(),
                canonicalization: BoundaryCanonicalization::JsonJcs,
            },
            "Sustained weight",
            PrimitiveLoadCaseKind::Weight,
            vec![weight_line_load("weight-1")],
            "provenance:manual",
        )
        .unwrap_err();
        assert_eq!(
            missing_record_id,
            BoundaryMetadataError::MissingField { field: "record_id" }
        );
    }

    #[test]
    fn primitive_load_case_record_rejects_empty_duplicate_and_mismatched_loads() {
        let empty_loads = PrimitiveLoadCaseRecord::new(
            load_case_ref(),
            "Sustained weight",
            PrimitiveLoadCaseKind::Weight,
            Vec::new(),
            "provenance:manual",
        )
        .unwrap_err();
        assert_eq!(
            empty_loads,
            BoundaryMetadataError::EmptyCollection { field: "loads" }
        );

        let blank_load_id = PrimitiveLoadCaseRecord::new(
            load_case_ref(),
            "Sustained weight",
            PrimitiveLoadCaseKind::Weight,
            vec![weight_line_load(" ")],
            "provenance:manual",
        )
        .unwrap_err();
        assert_eq!(
            blank_load_id,
            BoundaryMetadataError::MissingField { field: "load_id" }
        );

        let duplicate_load_id = PrimitiveLoadCaseRecord::new(
            load_case_ref(),
            "Sustained weight",
            PrimitiveLoadCaseKind::Weight,
            vec![weight_line_load("weight-1"), weight_line_load("weight-1")],
            "provenance:manual",
        )
        .unwrap_err();
        assert_eq!(
            duplicate_load_id,
            BoundaryMetadataError::DuplicateLoadId {
                load_id: "weight-1".to_string(),
            }
        );

        let pressure_load = PrimitiveLoad::uniform_element_load(
            "pressure-1",
            PrimitiveLoadCategory::Pressure,
            0,
            LoadDirection::GlobalX,
            q(1000.0, LoadDimension::Pressure),
        );
        let category_mismatch = PrimitiveLoadCaseRecord::new(
            load_case_ref(),
            "Sustained weight",
            PrimitiveLoadCaseKind::Weight,
            vec![pressure_load],
            "provenance:manual",
        )
        .unwrap_err();
        assert_eq!(
            category_mismatch,
            BoundaryMetadataError::LoadCategoryMismatch {
                load_id: "pressure-1".to_string(),
                expected: PrimitiveLoadCategory::Weight,
                actual: PrimitiveLoadCategory::Pressure,
            }
        );
    }

    #[test]
    fn weight_load_prepares_element_force_per_length() {
        let load = PrimitiveLoad::uniform_element_load(
            "weight-1",
            PrimitiveLoadCategory::Weight,
            0,
            LoadDirection::GlobalZ,
            q(-245.25, LoadDimension::ForcePerLength),
        );

        let prepared = prepare_loads(2, 1, &[load]);

        assert!(!prepared.is_blocked());
        assert_eq!(prepared.element_uniform_loads.len(), 1);
        assert_eq!(prepared.element_uniform_loads[0].magnitude.value, -245.25);
        assert_eq!(
            prepared.element_uniform_loads[0].magnitude.dimension,
            LoadDimension::ForcePerLength
        );
    }

    #[test]
    fn lumped_equivalent_nodal_loads_split_force_per_length_by_connectivity() {
        let span = ElementLoadSpan::new(0, 2, 0, 10.0).unwrap();
        let load = PrimitiveLoad::uniform_element_load(
            "weight-line",
            PrimitiveLoadCategory::Weight,
            0,
            LoadDirection::GlobalY,
            q(4.0, LoadDimension::ForcePerLength),
        );

        let prepared = prepare_lumped_nodal_loads(3, 1, &[span], &[load]);
        let vector = prepared.global_load_vector(3);

        assert!(!prepared.is_blocked());
        assert!(prepared.element_uniform_loads.is_empty());
        assert!(prepared.imposed_displacements.is_empty());
        assert_eq!(
            prepared.nodal_loads,
            vec![
                NodalLoadContribution {
                    load_id: "weight-line".to_string(),
                    node_index: 0,
                    global_dof: UY,
                    value: 20.0,
                },
                NodalLoadContribution {
                    load_id: "weight-line".to_string(),
                    node_index: 2,
                    global_dof: 2 * DOF_PER_NODE + UY,
                    value: 20.0,
                },
            ]
        );
        assert_eq!(vector[UY], 20.0);
        assert_eq!(vector[2 * DOF_PER_NODE + UY], 20.0);
    }

    #[test]
    fn lumped_equivalent_nodal_loads_accept_dof_translation_direction() {
        let span = ElementLoadSpan::new(0, 0, 1, 3.0).unwrap();
        let load = PrimitiveLoad::uniform_element_load(
            "wind-line",
            PrimitiveLoadCategory::Wind,
            0,
            LoadDirection::Dof(FrameDof::Uz),
            q(-2.0, LoadDimension::ForcePerLength),
        );

        let prepared = prepare_lumped_nodal_loads(2, 1, &[span], &[load]);

        assert!(!prepared.is_blocked());
        assert_eq!(prepared.nodal_loads.len(), 2);
        assert_eq!(prepared.nodal_loads[0].global_dof, UZ);
        assert_eq!(prepared.nodal_loads[0].value, -3.0);
        assert_eq!(prepared.nodal_loads[1].global_dof, DOF_PER_NODE + UZ);
        assert_eq!(prepared.nodal_loads[1].value, -3.0);
    }

    #[test]
    fn load_case_assembly_sorts_and_sums_repeated_dofs() {
        let contributions = vec![
            SolverNodalLoadContribution::new("late", 1, DOF_PER_NODE + UY, 3.0),
            SolverNodalLoadContribution::new("early", 0, UX, 2.0),
            SolverNodalLoadContribution::new("same-dof", 1, DOF_PER_NODE + UY, -8.0),
        ];

        let assembly = assemble_solver_load_vector(2, &contributions);

        assert!(!assembly.is_blocked());
        assert_eq!(assembly.sorted_contributions[0].source_id, "early");
        assert_eq!(assembly.sorted_contributions[1].source_id, "late");
        assert_eq!(assembly.sorted_contributions[2].source_id, "same-dof");
        assert_eq!(assembly.global_load_vector[UX], 2.0);
        assert_eq!(assembly.global_load_vector[DOF_PER_NODE + UY], -5.0);
    }

    #[test]
    fn load_case_assembly_reports_invalid_contributions_without_partial_vector() {
        let contributions = vec![
            SolverNodalLoadContribution::new("bad-node", 2, 2 * DOF_PER_NODE, 1.0),
            SolverNodalLoadContribution::new("bad-dof", 0, 12, 1.0),
            SolverNodalLoadContribution::new("bad-mismatch", 0, DOF_PER_NODE + UX, 1.0),
            SolverNodalLoadContribution::new("bad-value", 0, UX, f64::NAN),
        ];

        let assembly = assemble_solver_load_vector(2, &contributions);

        assert!(assembly.is_blocked());
        assert!(assembly.global_load_vector.is_empty());
        assert!(assembly.sorted_contributions.is_empty());
        assert_eq!(
            assembly
                .findings
                .iter()
                .map(|finding| finding.code)
                .collect::<Vec<_>>(),
            vec![
                LoadCaseAssemblyFindingCode::NonFiniteContribution,
                LoadCaseAssemblyFindingCode::NodeDofMismatch,
                LoadCaseAssemblyFindingCode::DofOutOfRange,
                LoadCaseAssemblyFindingCode::NodeOutOfRange,
            ]
        );
    }

    #[test]
    fn lumped_api_does_not_change_prepare_loads_element_behavior() {
        let load = PrimitiveLoad::uniform_element_load(
            "weight-legacy",
            PrimitiveLoadCategory::Weight,
            0,
            LoadDirection::GlobalZ,
            q(-5.0, LoadDimension::ForcePerLength),
        );

        let prepared = prepare_loads(2, 1, &[load]);

        assert!(!prepared.is_blocked());
        assert!(prepared.nodal_loads.is_empty());
        assert_eq!(prepared.element_uniform_loads.len(), 1);
        assert_eq!(prepared.element_uniform_loads[0].load_id, "weight-legacy");
    }

    #[test]
    fn lumped_conversion_rejects_pressure_thermal_rotational_and_dynamic_inputs() {
        let span = ElementLoadSpan::new(0, 0, 1, 8.0).unwrap();
        let loads = vec![
            PrimitiveLoad::uniform_element_load(
                "pressure-line",
                PrimitiveLoadCategory::Pressure,
                0,
                LoadDirection::GlobalX,
                q(1.0, LoadDimension::ForcePerLength),
            ),
            PrimitiveLoad::uniform_element_load(
                "thermal-line",
                PrimitiveLoadCategory::Thermal,
                0,
                LoadDirection::GlobalX,
                q(10.0, LoadDimension::TemperatureChange),
            ),
            PrimitiveLoad::uniform_element_load(
                "rotational-line",
                PrimitiveLoadCategory::Weight,
                0,
                LoadDirection::Dof(FrameDof::Rx),
                q(3.0, LoadDimension::ForcePerLength),
            ),
            PrimitiveLoad::uniform_element_load(
                "seismic-acceleration",
                PrimitiveLoadCategory::Seismic,
                0,
                LoadDirection::GlobalY,
                q(0.4, LoadDimension::Acceleration),
            ),
        ];

        let prepared = prepare_lumped_nodal_loads(2, 1, &[span], &loads);

        assert!(prepared.is_blocked());
        assert!(prepared.nodal_loads.is_empty());
        assert_eq!(
            prepared
                .findings
                .iter()
                .map(|finding| finding.code)
                .collect::<Vec<_>>(),
            vec![
                FindingCode::UnsupportedTargetForCategory,
                FindingCode::UnsupportedTargetForCategory,
                FindingCode::InvalidLoadDirection,
                FindingCode::InvalidLoadDimension,
            ]
        );
    }

    #[test]
    fn lumped_conversion_reports_missing_and_invalid_spans() {
        let missing_span_load = PrimitiveLoad::uniform_element_load(
            "missing-span",
            PrimitiveLoadCategory::Weight,
            0,
            LoadDirection::GlobalZ,
            q(-1.0, LoadDimension::ForcePerLength),
        );

        let missing = prepare_lumped_nodal_loads(2, 1, &[], &[missing_span_load]);

        assert!(missing.is_blocked());
        assert_eq!(missing.findings[0].code, FindingCode::MissingElementSpan);

        let bad_spans = vec![
            ElementLoadSpan {
                element_index: 0,
                node_i: 0,
                node_j: 1,
                span: f64::INFINITY,
            },
            ElementLoadSpan {
                element_index: 1,
                node_i: 0,
                node_j: 1,
                span: 0.0,
            },
        ];
        let loads = vec![
            PrimitiveLoad::uniform_element_load(
                "nonfinite-span",
                PrimitiveLoadCategory::Weight,
                0,
                LoadDirection::GlobalZ,
                q(-1.0, LoadDimension::ForcePerLength),
            ),
            PrimitiveLoad::uniform_element_load(
                "nonpositive-span",
                PrimitiveLoadCategory::Weight,
                1,
                LoadDirection::GlobalZ,
                q(-1.0, LoadDimension::ForcePerLength),
            ),
        ];

        let invalid = prepare_lumped_nodal_loads(2, 2, &bad_spans, &loads);

        assert!(invalid.is_blocked());
        assert_eq!(invalid.findings.len(), 2);
        assert_eq!(invalid.findings[0].code, FindingCode::InvalidElementSpan);
        assert_eq!(invalid.findings[1].code, FindingCode::InvalidElementSpan);

        assert_eq!(
            ElementLoadSpan::new(0, 0, 1, 0.0).unwrap_err(),
            PrimitiveLoadError::NonPositiveInput {
                name: "element span",
                value: 0.0
            }
        );
    }

    #[test]
    fn lumped_conversion_reports_invalid_indices_and_connectivity() {
        let spans = vec![
            ElementLoadSpan {
                element_index: 0,
                node_i: 2,
                node_j: 0,
                span: 1.0,
            },
            ElementLoadSpan {
                element_index: 1,
                node_i: 1,
                node_j: 1,
                span: 1.0,
            },
        ];
        let loads = vec![
            PrimitiveLoad::uniform_element_load(
                "bad-node",
                PrimitiveLoadCategory::Weight,
                0,
                LoadDirection::GlobalZ,
                q(-1.0, LoadDimension::ForcePerLength),
            ),
            PrimitiveLoad::uniform_element_load(
                "repeated-node",
                PrimitiveLoadCategory::Weight,
                1,
                LoadDirection::GlobalZ,
                q(-1.0, LoadDimension::ForcePerLength),
            ),
            PrimitiveLoad::uniform_element_load(
                "bad-element",
                PrimitiveLoadCategory::Weight,
                2,
                LoadDirection::GlobalZ,
                q(-1.0, LoadDimension::ForcePerLength),
            ),
        ];

        let prepared = prepare_lumped_nodal_loads(2, 2, &spans, &loads);

        assert!(prepared.is_blocked());
        assert!(prepared.nodal_loads.is_empty());
        assert_eq!(prepared.findings.len(), 3);
        assert_eq!(prepared.findings[0].code, FindingCode::NodeOutOfRange);
        assert_eq!(
            prepared.findings[1].code,
            FindingCode::InvalidElementConnectivity
        );
        assert_eq!(prepared.findings[2].code, FindingCode::ElementOutOfRange);
    }

    #[test]
    fn pressure_and_hydrotest_accept_pressure_or_equivalent_line_load() {
        let pressure = PrimitiveLoad::uniform_element_load(
            "pressure-1",
            PrimitiveLoadCategory::Pressure,
            0,
            LoadDirection::GlobalX,
            q(1000.0, LoadDimension::Pressure),
        );
        let hydrotest = PrimitiveLoad::uniform_element_load(
            "hydrotest-1",
            PrimitiveLoadCategory::Hydrotest,
            0,
            LoadDirection::GlobalZ,
            q(320.0, LoadDimension::ForcePerLength),
        );

        let prepared = prepare_loads(2, 1, &[pressure, hydrotest]);

        assert!(!prepared.is_blocked());
        assert_eq!(prepared.element_uniform_loads.len(), 2);
        assert_eq!(prepared.element_uniform_loads[0].load_id, "pressure-1");
        assert_eq!(
            prepared.element_uniform_loads[0].magnitude.dimension,
            LoadDimension::Pressure
        );
        assert_eq!(prepared.element_uniform_loads[1].load_id, "hydrotest-1");
    }

    #[test]
    fn scalar_pressure_accepts_legacy_rotational_direction() {
        let pressure = PrimitiveLoad::uniform_element_load(
            "pressure-legacy-direction",
            PrimitiveLoadCategory::Pressure,
            0,
            LoadDirection::Dof(FrameDof::Rz),
            q(1000.0, LoadDimension::Pressure),
        );

        let prepared = prepare_loads(2, 1, &[pressure]);

        assert!(!prepared.is_blocked());
        assert_eq!(prepared.element_uniform_loads.len(), 1);
        assert_eq!(
            prepared.element_uniform_loads[0].magnitude.dimension,
            LoadDimension::Pressure
        );
    }

    #[test]
    fn thermal_load_keeps_explicit_temperature_change_boundary() {
        let load = PrimitiveLoad::uniform_element_load(
            "thermal-1",
            PrimitiveLoadCategory::Thermal,
            0,
            LoadDirection::Dof(FrameDof::Rz),
            q(55.0, LoadDimension::TemperatureChange),
        );

        let prepared = prepare_loads(2, 1, &[load]);

        assert!(!prepared.is_blocked());
        assert_eq!(
            prepared.element_uniform_loads[0].direction,
            LoadDirection::Dof(FrameDof::Rz)
        );
        assert_eq!(prepared.element_uniform_loads[0].magnitude.value, 55.0);
        assert_eq!(
            prepared.element_uniform_loads[0].magnitude.dimension,
            LoadDimension::TemperatureChange
        );
    }

    #[test]
    fn straight_pipe_axial_effects_prepare_thermal_and_pressure_forces() {
        let properties = [ElementAxialEffectProperties::new(
            0,
            Some(200.0e9),
            Some(0.01),
            Some(12.0e-6),
            Some(0.003),
        )];
        let loads = vec![
            PrimitiveLoad::uniform_element_load(
                "thermal-axial",
                PrimitiveLoadCategory::Thermal,
                0,
                LoadDirection::GlobalX,
                q(50.0, LoadDimension::TemperatureChange),
            ),
            PrimitiveLoad::uniform_element_load(
                "pressure-thrust",
                PrimitiveLoadCategory::Pressure,
                0,
                LoadDirection::GlobalX,
                q(1.0e6, LoadDimension::Pressure),
            ),
        ];

        let prepared = prepare_straight_pipe_axial_effects(1, &loads, &properties);

        assert!(!prepared.is_blocked());
        assert_eq!(prepared.axial_effects.len(), 2);
        assert_eq!(prepared.axial_effects[0].load_id, "pressure-thrust");
        assert_close(prepared.axial_effects[0].axial_force, 3000.0);
        assert_eq!(prepared.axial_effects[1].load_id, "thermal-axial");
        assert_close(prepared.axial_effects[1].axial_force, 1_200_000.0);
    }

    #[test]
    fn straight_pipe_axial_effects_block_all_outputs_when_any_finding_exists() {
        let properties = [
            ElementAxialEffectProperties::new(0, Some(200.0e9), Some(0.01), None, Some(0.003)),
            ElementAxialEffectProperties::new(1, Some(200.0e9), Some(0.0), Some(12.0e-6), None),
        ];
        let loads = vec![
            PrimitiveLoad::nodal_force(
                "pressure-node",
                PrimitiveLoadCategory::Pressure,
                0,
                LoadDirection::GlobalX,
                q(1.0, LoadDimension::Force),
            ),
            PrimitiveLoad::uniform_element_load(
                "thermal-wrong-dimension",
                PrimitiveLoadCategory::Thermal,
                0,
                LoadDirection::GlobalX,
                q(1.0, LoadDimension::Pressure),
            ),
            PrimitiveLoad::uniform_element_load(
                "pressure-out-of-range",
                PrimitiveLoadCategory::Pressure,
                3,
                LoadDirection::GlobalX,
                q(1.0, LoadDimension::Pressure),
            ),
            PrimitiveLoad::uniform_element_load(
                "pressure-missing-properties",
                PrimitiveLoadCategory::Pressure,
                2,
                LoadDirection::GlobalX,
                q(1.0, LoadDimension::Pressure),
            ),
            PrimitiveLoad::uniform_element_load(
                "thermal-missing-alpha",
                PrimitiveLoadCategory::Thermal,
                0,
                LoadDirection::GlobalX,
                q(20.0, LoadDimension::TemperatureChange),
            ),
            PrimitiveLoad::uniform_element_load(
                "thermal-nonpositive-area",
                PrimitiveLoadCategory::Thermal,
                1,
                LoadDirection::GlobalX,
                q(20.0, LoadDimension::TemperatureChange),
            ),
            PrimitiveLoad {
                load_id: "pressure-nonfinite".to_string(),
                category: PrimitiveLoadCategory::Pressure,
                target: Some(LoadTarget::Element(0)),
                direction: LoadDirection::GlobalX,
                magnitude: Some(LoadQuantity {
                    value: f64::NAN,
                    dimension: LoadDimension::Pressure,
                }),
            },
        ];

        let prepared = prepare_straight_pipe_axial_effects(3, &loads, &properties);

        assert!(prepared.is_blocked());
        assert!(prepared.axial_effects.is_empty());
        assert_eq!(
            prepared
                .findings
                .iter()
                .map(|finding| finding.code)
                .collect::<Vec<_>>(),
            vec![
                FindingCode::UnsupportedTargetForCategory,
                FindingCode::InvalidLoadDimension,
                FindingCode::ElementOutOfRange,
                FindingCode::MissingElementProperties,
                FindingCode::MissingPhysicalProperty,
                FindingCode::InvalidPhysicalProperty,
                FindingCode::NonFiniteLoadMagnitude,
            ]
        );
    }

    #[test]
    fn straight_pipe_axial_effects_reject_overflowing_computed_force() {
        let properties = [ElementAxialEffectProperties::new(
            0,
            Some(1.0e308),
            Some(10.0),
            Some(10.0),
            Some(0.003),
        )];
        let load = PrimitiveLoad::uniform_element_load(
            "thermal-overflow",
            PrimitiveLoadCategory::Thermal,
            0,
            LoadDirection::GlobalX,
            q(10.0, LoadDimension::TemperatureChange),
        );

        let prepared = prepare_straight_pipe_axial_effects(1, &[load], &properties);

        assert!(prepared.is_blocked());
        assert!(prepared.axial_effects.is_empty());
        assert_eq!(prepared.findings[0].code, FindingCode::NonFiniteAxialEffect);
    }

    #[test]
    fn thermal_load_rejects_force_per_length_fallthrough() {
        let load = PrimitiveLoad::uniform_element_load(
            "thermal-line-load",
            PrimitiveLoadCategory::Thermal,
            0,
            LoadDirection::GlobalX,
            q(12.0, LoadDimension::ForcePerLength),
        );

        let prepared = prepare_loads(2, 1, &[load]);

        assert!(prepared.is_blocked());
        assert_eq!(prepared.findings[0].code, FindingCode::InvalidLoadDimension);
    }

    #[test]
    fn wind_seismic_and_occasional_can_prepare_nodal_loads() {
        let loads = vec![
            PrimitiveLoad::nodal_force(
                "wind-1",
                PrimitiveLoadCategory::Wind,
                0,
                LoadDirection::GlobalY,
                q(12.0, LoadDimension::Force),
            ),
            PrimitiveLoad::nodal_force(
                "seismic-1",
                PrimitiveLoadCategory::Seismic,
                0,
                LoadDirection::GlobalX,
                q(4.0, LoadDimension::Force),
            ),
            PrimitiveLoad::nodal_force(
                "occasional-1",
                PrimitiveLoadCategory::Occasional,
                1,
                LoadDirection::Dof(FrameDof::Rz),
                q(2.5, LoadDimension::Moment),
            ),
        ];

        let prepared = prepare_loads(2, 1, &loads);
        let vector = prepared.global_load_vector(2);

        assert!(!prepared.is_blocked());
        assert_eq!(vector[UX], 4.0);
        assert_eq!(vector[UY], 12.0);
        assert_eq!(vector[DOF_PER_NODE + RZ], 2.5);
    }

    #[test]
    fn wind_and_seismic_prepare_only_explicit_equivalent_mechanics_loads() {
        let loads = vec![
            PrimitiveLoad::uniform_element_load(
                "wind-equivalent-line-load",
                PrimitiveLoadCategory::Wind,
                1,
                LoadDirection::GlobalY,
                q(7.5, LoadDimension::ForcePerLength),
            ),
            PrimitiveLoad::uniform_element_load(
                "seismic-equivalent-line-load",
                PrimitiveLoadCategory::Seismic,
                0,
                LoadDirection::GlobalX,
                q(3.25, LoadDimension::ForcePerLength),
            ),
        ];

        let prepared = prepare_loads(2, 2, &loads);

        assert!(!prepared.is_blocked());
        assert_eq!(prepared.element_uniform_loads.len(), 2);
        assert_eq!(
            prepared.element_uniform_loads[0].load_id,
            "seismic-equivalent-line-load"
        );
        assert_eq!(
            prepared.element_uniform_loads[1].load_id,
            "wind-equivalent-line-load"
        );
        assert!(prepared.nodal_loads.is_empty());
        assert!(prepared.imposed_displacements.is_empty());
    }

    #[test]
    fn wind_and_seismic_reject_dynamic_placeholder_dimensions() {
        let loads = vec![
            PrimitiveLoad::uniform_element_load(
                "wind-acceleration",
                PrimitiveLoadCategory::Wind,
                0,
                LoadDirection::GlobalY,
                q(1.2, LoadDimension::Acceleration),
            ),
            PrimitiveLoad::uniform_element_load(
                "seismic-acceleration",
                PrimitiveLoadCategory::Seismic,
                0,
                LoadDirection::GlobalX,
                q(0.4, LoadDimension::Acceleration),
            ),
        ];

        let prepared = prepare_loads(2, 1, &loads);

        assert!(prepared.is_blocked());
        assert_eq!(prepared.findings.len(), 2);
        assert_eq!(prepared.findings[0].code, FindingCode::InvalidLoadDimension);
        assert_eq!(prepared.findings[1].code, FindingCode::InvalidLoadDimension);
        assert!(prepared.element_uniform_loads.is_empty());
    }

    #[test]
    fn pressure_and_thermal_do_not_prepare_nodal_user_loads() {
        let loads = vec![
            PrimitiveLoad::nodal_force(
                "pressure-node",
                PrimitiveLoadCategory::Pressure,
                0,
                LoadDirection::GlobalX,
                q(10.0, LoadDimension::Force),
            ),
            PrimitiveLoad::nodal_force(
                "thermal-node",
                PrimitiveLoadCategory::Thermal,
                0,
                LoadDirection::GlobalY,
                q(20.0, LoadDimension::Force),
            ),
        ];

        let prepared = prepare_loads(1, 1, &loads);

        assert!(prepared.is_blocked());
        assert_eq!(prepared.findings.len(), 2);
        assert_eq!(
            prepared.findings[0].code,
            FindingCode::UnsupportedTargetForCategory
        );
        assert_eq!(
            prepared.findings[1].code,
            FindingCode::UnsupportedTargetForCategory
        );
        assert!(prepared.nodal_loads.is_empty());
    }

    #[test]
    fn imposed_displacement_preserves_support_dof_boundary() {
        let load = PrimitiveLoad::imposed_displacement(
            "settlement-1",
            1,
            FrameDof::Uz,
            q(-0.006, LoadDimension::Displacement),
        );

        let prepared = prepare_loads(2, 1, &[load]);

        assert!(!prepared.is_blocked());
        assert_eq!(prepared.imposed_displacements[0].node_dof.global_index(), 8);
        assert_eq!(prepared.imposed_displacements[0].value.value, -0.006);
    }

    #[test]
    fn missing_target_and_magnitude_are_findings() {
        let loads = vec![
            PrimitiveLoad::missing_target(
                "missing-target",
                PrimitiveLoadCategory::Weight,
                LoadDirection::GlobalZ,
                Some(q(-1.0, LoadDimension::ForcePerLength)),
            ),
            PrimitiveLoad {
                load_id: "missing-magnitude".to_string(),
                category: PrimitiveLoadCategory::Wind,
                target: Some(LoadTarget::Node(0)),
                direction: LoadDirection::GlobalY,
                magnitude: None,
            },
        ];

        let prepared = prepare_loads(1, 1, &loads);

        assert!(prepared.is_blocked());
        assert_eq!(prepared.findings[0].code, FindingCode::MissingLoadTarget);
        assert_eq!(prepared.findings[1].code, FindingCode::MissingLoadMagnitude);
    }

    #[test]
    fn invalid_targets_and_dimensions_are_findings() {
        let loads = vec![
            PrimitiveLoad::nodal_force(
                "weight-node",
                PrimitiveLoadCategory::Weight,
                0,
                LoadDirection::GlobalZ,
                q(-1.0, LoadDimension::Force),
            ),
            PrimitiveLoad::uniform_element_load(
                "bad-element",
                PrimitiveLoadCategory::Wind,
                2,
                LoadDirection::GlobalY,
                q(1.0, LoadDimension::ForcePerLength),
            ),
            PrimitiveLoad::imposed_displacement(
                "bad-imposed",
                0,
                FrameDof::Rx,
                q(0.1, LoadDimension::Displacement),
            ),
        ];

        let prepared = prepare_loads(1, 1, &loads);

        assert!(prepared.is_blocked());
        assert_eq!(
            prepared.findings[0].code,
            FindingCode::UnsupportedTargetForCategory
        );
        assert_eq!(prepared.findings[1].code, FindingCode::ElementOutOfRange);
        assert_eq!(prepared.findings[2].code, FindingCode::InvalidLoadDimension);
    }

    #[test]
    fn nonfinite_and_nonpositive_quantities_are_rejected_at_construction() {
        match LoadQuantity::new(f64::NAN, LoadDimension::Force).unwrap_err() {
            PrimitiveLoadError::NonFiniteInput { name, value } => {
                assert_eq!(name, "load quantity");
                assert!(value.is_nan());
            }
            other => panic!("expected non-finite input error, got {other:?}"),
        }
        assert_eq!(
            LoadQuantity::positive(0.0, LoadDimension::Force).unwrap_err(),
            PrimitiveLoadError::NonPositiveInput {
                name: "load quantity",
                value: 0.0
            }
        );
    }

    #[test]
    fn deterministic_sorting_stabilizes_application_outputs() {
        let loads = vec![
            PrimitiveLoad::nodal_force(
                "node-2",
                PrimitiveLoadCategory::Wind,
                1,
                LoadDirection::GlobalY,
                q(2.0, LoadDimension::Force),
            ),
            PrimitiveLoad::nodal_force(
                "node-1",
                PrimitiveLoadCategory::Wind,
                0,
                LoadDirection::GlobalX,
                q(1.0, LoadDimension::Force),
            ),
        ];

        let prepared = prepare_loads(2, 1, &loads);

        assert_eq!(prepared.nodal_loads[0].load_id, "node-1");
        assert_eq!(prepared.nodal_loads[1].load_id, "node-2");
    }
}
