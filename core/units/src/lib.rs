//! Deterministic unit catalog and conversion primitives.
//!
//! This crate implements the Phase B1 unit catalog accepted by DEC-018:
//! SI-canonical calculations, a closed dual SI/US display catalog,
//! reviewed public definitional conversion constants, explicit absolute vs
//! interval temperature semantics, and explicit gauge vs absolute pressure
//! semantics. It does not encode protected standards data, user project data,
//! rule-pack values, material allowables, or professional approval.

use std::error::Error;
use std::fmt;

pub const INCH_IN_METERS: f64 = 0.0254;
pub const FOOT_IN_METERS: f64 = 0.3048;
pub const POUND_MASS_IN_KILOGRAMS: f64 = 0.453_592_37;
pub const STANDARD_GRAVITY_M_PER_S2: f64 = 9.806_65;
pub const POUND_FORCE_IN_NEWTONS: f64 = 4.448_221_615_260_5;
pub const SQUARE_INCH_IN_SQUARE_METERS: f64 = 0.000_645_16;
pub const PSI_IN_PASCALS: f64 = POUND_FORCE_IN_NEWTONS / SQUARE_INCH_IN_SQUARE_METERS;
pub const KSI_IN_PASCALS: f64 = 1_000.0 * PSI_IN_PASCALS;
pub const STANDARD_ATMOSPHERE_IN_PASCALS: f64 = 101_325.0;
pub const CELSIUS_ABSOLUTE_ZERO_OFFSET_K: f64 = 273.15;
pub const FAHRENHEIT_ABSOLUTE_ZERO_OFFSET_R: f64 = 459.67;

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

impl Dimension {
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

    pub fn from_schema_value(value: &str) -> Result<Self, UnitError> {
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
            "temperature_difference" => Err(UnitError::RetiredDimensionAlias {
                alias: "temperature_difference",
                replacement: "temperature_interval",
            }),
            "area_moment" => Err(UnitError::RetiredDimensionAlias {
                alias: "area_moment",
                replacement: "second_moment_area",
            }),
            _ => Err(UnitError::UnknownDimension(value.to_string())),
        }
    }

    pub fn vector(self) -> DimensionVector {
        match self {
            Self::Dimensionless | Self::Slope => DimensionVector::ZERO,
            Self::Length | Self::Displacement => DimensionVector::new(1, 0, 0, 0, 0),
            Self::Mass => DimensionVector::new(0, 1, 0, 0, 0),
            Self::Time => DimensionVector::new(0, 0, 1, 0, 0),
            Self::Temperature | Self::TemperatureInterval => DimensionVector::new(0, 0, 0, 1, 0),
            Self::Angle | Self::Rotation => DimensionVector::new(0, 0, 0, 0, 1),
            Self::Force => DimensionVector::new(1, 1, -2, 0, 0),
            Self::ForcePerLength => DimensionVector::new(0, 1, -2, 0, 0),
            Self::Moment => DimensionVector::new(2, 1, -2, 0, 0),
            Self::Pressure | Self::Stress => DimensionVector::new(-1, 1, -2, 0, 0),
            Self::Area => DimensionVector::new(2, 0, 0, 0, 0),
            Self::Volume => DimensionVector::new(3, 0, 0, 0, 0),
            Self::Density => DimensionVector::new(-3, 1, 0, 0, 0),
            Self::LinearStiffness => DimensionVector::new(0, 1, -2, 0, 0),
            Self::RotationalStiffness => DimensionVector::new(2, 1, -2, 0, -1),
            Self::Velocity => DimensionVector::new(1, 0, -1, 0, 0),
            Self::Acceleration => DimensionVector::new(1, 0, -2, 0, 0),
            Self::ThermalConductivity => DimensionVector::new(1, 1, -3, -1, 0),
            Self::SpecificHeat => DimensionVector::new(2, 0, -2, -1, 0),
            Self::ThermalExpansionCoefficient => DimensionVector::new(0, 0, 0, -1, 0),
            Self::SecondMomentArea => DimensionVector::new(4, 0, 0, 0, 0),
            Self::SectionModulus => DimensionVector::new(3, 0, 0, 0, 0),
            Self::MassPerLength => DimensionVector::new(-1, 1, 0, 0, 0),
            Self::VolumePerLength => DimensionVector::new(2, 0, 0, 0, 0),
            Self::Tbd => DimensionVector::TBD,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DimensionVector {
    pub length: i8,
    pub mass: i8,
    pub time: i8,
    pub temperature: i8,
    pub angle: i8,
    pub unresolved: bool,
}

impl DimensionVector {
    pub const ZERO: Self = Self::new(0, 0, 0, 0, 0);
    pub const TBD: Self = Self {
        length: 0,
        mass: 0,
        time: 0,
        temperature: 0,
        angle: 0,
        unresolved: true,
    };

    pub const fn new(length: i8, mass: i8, time: i8, temperature: i8, angle: i8) -> Self {
        Self {
            length,
            mass,
            time,
            temperature,
            angle,
            unresolved: false,
        }
    }

    pub fn multiply(self, other: Self) -> Result<Self, UnitError> {
        self.combine(other, 1)
    }

    pub fn divide(self, other: Self) -> Result<Self, UnitError> {
        self.combine(other, -1)
    }

    pub fn powi(self, exponent: i8) -> Result<Self, UnitError> {
        if self.unresolved {
            return Err(UnitError::UnresolvedDimension);
        }
        Ok(Self::new(
            self.length * exponent,
            self.mass * exponent,
            self.time * exponent,
            self.temperature * exponent,
            self.angle * exponent,
        ))
    }

    fn combine(self, other: Self, sign: i8) -> Result<Self, UnitError> {
        if self.unresolved || other.unresolved {
            return Err(UnitError::UnresolvedDimension);
        }
        Ok(Self::new(
            self.length + sign * other.length,
            self.mass + sign * other.mass,
            self.time + sign * other.time,
            self.temperature + sign * other.temperature,
            self.angle + sign * other.angle,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnitId {
    One,
    Meter,
    Millimeter,
    Inch,
    Foot,
    Kilogram,
    PoundMass,
    Second,
    Kelvin,
    DegreeCelsius,
    DegreeFahrenheit,
    DegreeRankine,
    KelvinInterval,
    DegreeCelsiusInterval,
    DegreeFahrenheitInterval,
    DegreeRankineInterval,
    Radian,
    Degree,
    Newton,
    PoundForce,
    NewtonPerMeter,
    PoundForcePerFoot,
    PoundForcePerInch,
    NewtonMeter,
    PoundForceInch,
    PoundForceFoot,
    Pascal,
    Kilopascal,
    Megapascal,
    Psi,
    Ksi,
    SquareMeter,
    SquareMillimeter,
    SquareInch,
    CubicMeter,
    CubicInch,
    KilogramPerCubicMeter,
    PoundMassPerCubicInch,
    NewtonPerMeterLinear,
    NewtonMeterPerRadian,
    MeterPerSecond,
    MeterPerSecondSquared,
    WattPerMeterKelvin,
    JoulePerKilogramKelvin,
    PerKelvin,
    PerDegreeCelsius,
    MeterFourth,
    MillimeterFourth,
    InchFourth,
    MeterCubed,
    MillimeterCubed,
    InchCubed,
    KilogramPerMeter,
    PoundMassPerFoot,
    SquareMeterPerMeter,
}

impl UnitId {
    pub fn definition(self) -> UnitDefinition {
        match self {
            Self::One => def(self, "1", Dimension::Dimensionless, Transform::linear(1.0)),
            Self::Meter => def(self, "m", Dimension::Length, Transform::linear(1.0)),
            Self::Millimeter => def(self, "mm", Dimension::Length, Transform::linear(0.001)),
            Self::Inch => def(
                self,
                "in",
                Dimension::Length,
                Transform::linear(INCH_IN_METERS),
            ),
            Self::Foot => def(
                self,
                "ft",
                Dimension::Length,
                Transform::linear(FOOT_IN_METERS),
            ),
            Self::Kilogram => def(self, "kg", Dimension::Mass, Transform::linear(1.0)),
            Self::PoundMass => def(
                self,
                "lb",
                Dimension::Mass,
                Transform::linear(POUND_MASS_IN_KILOGRAMS),
            ),
            Self::Second => def(self, "s", Dimension::Time, Transform::linear(1.0)),
            Self::Kelvin => def(self, "K", Dimension::Temperature, Transform::linear(1.0)),
            Self::DegreeCelsius => def(
                self,
                "degC",
                Dimension::Temperature,
                Transform::affine(1.0, CELSIUS_ABSOLUTE_ZERO_OFFSET_K),
            ),
            Self::DegreeFahrenheit => def(
                self,
                "degF",
                Dimension::Temperature,
                Transform::affine(5.0 / 9.0, FAHRENHEIT_ABSOLUTE_ZERO_OFFSET_R * 5.0 / 9.0),
            ),
            Self::DegreeRankine => def(
                self,
                "degR",
                Dimension::Temperature,
                Transform::linear(5.0 / 9.0),
            ),
            Self::KelvinInterval => def(
                self,
                "K",
                Dimension::TemperatureInterval,
                Transform::linear(1.0),
            ),
            Self::DegreeCelsiusInterval => def(
                self,
                "degC",
                Dimension::TemperatureInterval,
                Transform::linear(1.0),
            ),
            Self::DegreeFahrenheitInterval => def(
                self,
                "degF",
                Dimension::TemperatureInterval,
                Transform::linear(5.0 / 9.0),
            ),
            Self::DegreeRankineInterval => def(
                self,
                "degR",
                Dimension::TemperatureInterval,
                Transform::linear(5.0 / 9.0),
            ),
            Self::Radian => def(self, "rad", Dimension::Angle, Transform::linear(1.0)),
            Self::Degree => def(
                self,
                "deg",
                Dimension::Angle,
                Transform::linear(std::f64::consts::PI / 180.0),
            ),
            Self::Newton => def(self, "N", Dimension::Force, Transform::linear(1.0)),
            Self::PoundForce => def(
                self,
                "lbf",
                Dimension::Force,
                Transform::linear(POUND_FORCE_IN_NEWTONS),
            ),
            Self::NewtonPerMeter => def(
                self,
                "N/m",
                Dimension::ForcePerLength,
                Transform::linear(1.0),
            ),
            Self::PoundForcePerFoot => def(
                self,
                "lbf/ft",
                Dimension::ForcePerLength,
                Transform::linear(POUND_FORCE_IN_NEWTONS / FOOT_IN_METERS),
            ),
            Self::PoundForcePerInch => def(
                self,
                "lbf/in",
                Dimension::ForcePerLength,
                Transform::linear(POUND_FORCE_IN_NEWTONS / INCH_IN_METERS),
            ),
            Self::NewtonMeter => def(self, "N*m", Dimension::Moment, Transform::linear(1.0)),
            Self::PoundForceInch => def(
                self,
                "lbf*in",
                Dimension::Moment,
                Transform::linear(POUND_FORCE_IN_NEWTONS * INCH_IN_METERS),
            ),
            Self::PoundForceFoot => def(
                self,
                "lbf*ft",
                Dimension::Moment,
                Transform::linear(POUND_FORCE_IN_NEWTONS * FOOT_IN_METERS),
            ),
            Self::Pascal => def(self, "Pa", Dimension::Pressure, Transform::linear(1.0)),
            Self::Kilopascal => def(self, "kPa", Dimension::Pressure, Transform::linear(1_000.0)),
            Self::Megapascal => def(
                self,
                "MPa",
                Dimension::Pressure,
                Transform::linear(1_000_000.0),
            ),
            Self::Psi => def(
                self,
                "psi",
                Dimension::Pressure,
                Transform::linear(PSI_IN_PASCALS),
            ),
            Self::Ksi => def(
                self,
                "ksi",
                Dimension::Pressure,
                Transform::linear(KSI_IN_PASCALS),
            ),
            Self::SquareMeter => def(self, "m^2", Dimension::Area, Transform::linear(1.0)),
            Self::SquareMillimeter => def(self, "mm^2", Dimension::Area, Transform::linear(1.0e-6)),
            Self::SquareInch => def(
                self,
                "in^2",
                Dimension::Area,
                Transform::linear(SQUARE_INCH_IN_SQUARE_METERS),
            ),
            Self::CubicMeter => def(self, "m^3", Dimension::Volume, Transform::linear(1.0)),
            Self::CubicInch => def(
                self,
                "in^3",
                Dimension::Volume,
                Transform::linear(INCH_IN_METERS * INCH_IN_METERS * INCH_IN_METERS),
            ),
            Self::KilogramPerCubicMeter => {
                def(self, "kg/m^3", Dimension::Density, Transform::linear(1.0))
            }
            Self::PoundMassPerCubicInch => def(
                self,
                "lb/in^3",
                Dimension::Density,
                Transform::linear(
                    POUND_MASS_IN_KILOGRAMS / (INCH_IN_METERS * INCH_IN_METERS * INCH_IN_METERS),
                ),
            ),
            Self::NewtonPerMeterLinear => def(
                self,
                "N/m",
                Dimension::LinearStiffness,
                Transform::linear(1.0),
            ),
            Self::NewtonMeterPerRadian => def(
                self,
                "N*m/rad",
                Dimension::RotationalStiffness,
                Transform::linear(1.0),
            ),
            Self::MeterPerSecond => def(self, "m/s", Dimension::Velocity, Transform::linear(1.0)),
            Self::MeterPerSecondSquared => def(
                self,
                "m/s^2",
                Dimension::Acceleration,
                Transform::linear(1.0),
            ),
            Self::WattPerMeterKelvin => def(
                self,
                "W/(m*K)",
                Dimension::ThermalConductivity,
                Transform::linear(1.0),
            ),
            Self::JoulePerKilogramKelvin => def(
                self,
                "J/(kg*K)",
                Dimension::SpecificHeat,
                Transform::linear(1.0),
            ),
            Self::PerKelvin => def(
                self,
                "1/K",
                Dimension::ThermalExpansionCoefficient,
                Transform::linear(1.0),
            ),
            Self::PerDegreeCelsius => def(
                self,
                "1/degC",
                Dimension::ThermalExpansionCoefficient,
                Transform::linear(1.0),
            ),
            Self::MeterFourth => def(
                self,
                "m^4",
                Dimension::SecondMomentArea,
                Transform::linear(1.0),
            ),
            Self::MillimeterFourth => def(
                self,
                "mm^4",
                Dimension::SecondMomentArea,
                Transform::linear(1.0e-12),
            ),
            Self::InchFourth => def(
                self,
                "in^4",
                Dimension::SecondMomentArea,
                Transform::linear(
                    INCH_IN_METERS * INCH_IN_METERS * INCH_IN_METERS * INCH_IN_METERS,
                ),
            ),
            Self::MeterCubed => def(
                self,
                "m^3",
                Dimension::SectionModulus,
                Transform::linear(1.0),
            ),
            Self::MillimeterCubed => def(
                self,
                "mm^3",
                Dimension::SectionModulus,
                Transform::linear(1.0e-9),
            ),
            Self::InchCubed => def(
                self,
                "in^3",
                Dimension::SectionModulus,
                Transform::linear(INCH_IN_METERS * INCH_IN_METERS * INCH_IN_METERS),
            ),
            Self::KilogramPerMeter => def(
                self,
                "kg/m",
                Dimension::MassPerLength,
                Transform::linear(1.0),
            ),
            Self::PoundMassPerFoot => def(
                self,
                "lb/ft",
                Dimension::MassPerLength,
                Transform::linear(POUND_MASS_IN_KILOGRAMS / FOOT_IN_METERS),
            ),
            Self::SquareMeterPerMeter => def(
                self,
                "m^3/m",
                Dimension::VolumePerLength,
                Transform::linear(1.0),
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UnitDefinition {
    pub id: UnitId,
    pub symbol: &'static str,
    pub dimension: Dimension,
    pub transform_to_canonical: Transform,
    pub canonical: bool,
    pub provenance: ConversionProvenance,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub factor: f64,
    pub offset: f64,
    pub kind: TransformKind,
}

impl Transform {
    pub const fn linear(factor: f64) -> Self {
        Self {
            factor,
            offset: 0.0,
            kind: TransformKind::Multiplicative,
        }
    }

    pub const fn affine(factor: f64, offset: f64) -> Self {
        Self {
            factor,
            offset,
            kind: TransformKind::Affine,
        }
    }

    fn to_canonical(self, value: f64) -> f64 {
        value * self.factor + self.offset
    }

    fn canonical_to_unit(self, value: f64) -> f64 {
        (value - self.offset) / self.factor
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransformKind {
    Identity,
    Multiplicative,
    Affine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConversionProvenance {
    SiCanonical,
    ExactPublicDefinition,
    ConventionalPublicConstant,
    ProjectGovernedDecision,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuantityKind {
    Absolute,
    Interval,
    Relative,
    Ratio,
    Percentage,
    Coefficient,
    Dimensionless,
    UnitBearing,
    Tbd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PressureKind {
    Absolute,
    Gauge,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PressureReference {
    pub value: f64,
    pub unit: UnitId,
    pub provenance: String,
}

impl PressureReference {
    pub fn new(value: f64, unit: UnitId, provenance: impl Into<String>) -> Result<Self, UnitError> {
        validate_finite("pressure reference", value)?;
        if unit.definition().dimension != Dimension::Pressure {
            return Err(UnitError::DimensionMismatch {
                expected: Dimension::Pressure,
                actual: unit.definition().dimension,
            });
        }
        let provenance = provenance.into();
        if provenance.trim().is_empty() {
            return Err(UnitError::MissingPressureReferenceProvenance);
        }
        Ok(Self {
            value,
            unit,
            provenance,
        })
    }

    pub fn standard_atmosphere(provenance: impl Into<String>) -> Result<Self, UnitError> {
        Self::new(STANDARD_ATMOSPHERE_IN_PASCALS, UnitId::Pascal, provenance)
    }
}

pub fn catalog() -> &'static [UnitId] {
    CATALOG
}

pub fn catalog_definitions() -> impl Iterator<Item = UnitDefinition> {
    CATALOG.iter().copied().map(UnitId::definition)
}

pub fn canonical_unit(dimension: Dimension) -> Option<UnitId> {
    match dimension {
        Dimension::Dimensionless | Dimension::Slope => Some(UnitId::One),
        Dimension::Length | Dimension::Displacement => Some(UnitId::Meter),
        Dimension::Mass => Some(UnitId::Kilogram),
        Dimension::Time => Some(UnitId::Second),
        Dimension::Temperature => Some(UnitId::Kelvin),
        Dimension::TemperatureInterval => Some(UnitId::KelvinInterval),
        Dimension::Angle | Dimension::Rotation => Some(UnitId::Radian),
        Dimension::Force => Some(UnitId::Newton),
        Dimension::ForcePerLength => Some(UnitId::NewtonPerMeter),
        Dimension::Moment => Some(UnitId::NewtonMeter),
        Dimension::Pressure | Dimension::Stress => Some(UnitId::Pascal),
        Dimension::Area => Some(UnitId::SquareMeter),
        Dimension::Volume => Some(UnitId::CubicMeter),
        Dimension::Density => Some(UnitId::KilogramPerCubicMeter),
        Dimension::LinearStiffness => Some(UnitId::NewtonPerMeterLinear),
        Dimension::RotationalStiffness => Some(UnitId::NewtonMeterPerRadian),
        Dimension::Velocity => Some(UnitId::MeterPerSecond),
        Dimension::Acceleration => Some(UnitId::MeterPerSecondSquared),
        Dimension::ThermalConductivity => Some(UnitId::WattPerMeterKelvin),
        Dimension::SpecificHeat => Some(UnitId::JoulePerKilogramKelvin),
        Dimension::ThermalExpansionCoefficient => Some(UnitId::PerKelvin),
        Dimension::SecondMomentArea => Some(UnitId::MeterFourth),
        Dimension::SectionModulus => Some(UnitId::MeterCubed),
        Dimension::MassPerLength => Some(UnitId::KilogramPerMeter),
        Dimension::VolumePerLength => Some(UnitId::SquareMeterPerMeter),
        Dimension::Tbd => None,
    }
}

pub fn unit_by_symbol(symbol: &str, dimension: Dimension) -> Result<UnitId, UnitError> {
    if let Some(unit) = catalog_definitions()
        .find(|unit| unit.symbol == symbol && unit.dimension == dimension)
        .map(|unit| unit.id)
    {
        return Ok(unit);
    }

    catalog_definitions()
        .find(|unit| unit.symbol == symbol && unit_supports_dimension(unit.dimension, dimension))
        .map(|unit| unit.id)
        .ok_or_else(|| UnitError::UnknownUnit {
            symbol: symbol.to_string(),
            dimension,
        })
}

pub fn convert(value: f64, from: UnitId, to: UnitId) -> Result<f64, UnitError> {
    validate_finite("quantity", value)?;
    let from_def = from.definition();
    let to_def = to.definition();
    if from_def.dimension != to_def.dimension {
        return Err(UnitError::DimensionMismatch {
            expected: from_def.dimension,
            actual: to_def.dimension,
        });
    }
    if from_def.dimension == Dimension::Tbd {
        return Err(UnitError::UnresolvedDimension);
    }
    let canonical_value = from_def.transform_to_canonical.to_canonical(value);
    Ok(to_def
        .transform_to_canonical
        .canonical_to_unit(canonical_value))
}

pub fn convert_for_dimension(
    value: f64,
    dimension: Dimension,
    from: UnitId,
    to: UnitId,
) -> Result<f64, UnitError> {
    validate_finite("quantity", value)?;
    let from_def = from.definition();
    let to_def = to.definition();
    if !unit_supports_dimension(from_def.dimension, dimension) {
        return Err(UnitError::DimensionMismatch {
            expected: dimension,
            actual: from_def.dimension,
        });
    }
    if !unit_supports_dimension(to_def.dimension, dimension) {
        return Err(UnitError::DimensionMismatch {
            expected: dimension,
            actual: to_def.dimension,
        });
    }
    if dimension == Dimension::Tbd {
        return Err(UnitError::UnresolvedDimension);
    }
    let canonical_value = from_def.transform_to_canonical.to_canonical(value);
    Ok(to_def
        .transform_to_canonical
        .canonical_to_unit(canonical_value))
}

pub fn convert_quantity(
    value: f64,
    from: UnitId,
    to: UnitId,
    quantity_kind: QuantityKind,
) -> Result<f64, UnitError> {
    let dimension = from.definition().dimension;
    match (dimension, quantity_kind) {
        (Dimension::Temperature, QuantityKind::Absolute)
        | (Dimension::TemperatureInterval, QuantityKind::Interval)
        | (Dimension::Pressure, QuantityKind::Absolute | QuantityKind::Relative) => {
            convert(value, from, to)
        }
        (Dimension::Temperature | Dimension::TemperatureInterval | Dimension::Pressure, _) => {
            Err(UnitError::InvalidQuantityKind {
                dimension,
                quantity_kind,
            })
        }
        (_, QuantityKind::UnitBearing) => convert(value, from, to),
        (Dimension::Dimensionless, QuantityKind::Dimensionless | QuantityKind::Ratio) => {
            convert(value, from, to)
        }
        (Dimension::ThermalExpansionCoefficient, QuantityKind::Coefficient) => {
            convert(value, from, to)
        }
        _ => Err(UnitError::InvalidQuantityKind {
            dimension,
            quantity_kind,
        }),
    }
}

pub fn temperature_difference(
    minuend_value: f64,
    minuend_unit: UnitId,
    subtrahend_value: f64,
    subtrahend_unit: UnitId,
    target_interval_unit: UnitId,
) -> Result<f64, UnitError> {
    require_dimension(minuend_unit, Dimension::Temperature)?;
    require_dimension(subtrahend_unit, Dimension::Temperature)?;
    require_dimension(target_interval_unit, Dimension::TemperatureInterval)?;
    let minuend_k = convert(minuend_value, minuend_unit, UnitId::Kelvin)?;
    let subtrahend_k = convert(subtrahend_value, subtrahend_unit, UnitId::Kelvin)?;
    convert(
        minuend_k - subtrahend_k,
        UnitId::KelvinInterval,
        target_interval_unit,
    )
}

pub fn shift_temperature(
    absolute_value: f64,
    absolute_unit: UnitId,
    interval_value: f64,
    interval_unit: UnitId,
    target_absolute_unit: UnitId,
) -> Result<f64, UnitError> {
    require_dimension(absolute_unit, Dimension::Temperature)?;
    require_dimension(interval_unit, Dimension::TemperatureInterval)?;
    require_dimension(target_absolute_unit, Dimension::Temperature)?;
    let absolute_k = convert(absolute_value, absolute_unit, UnitId::Kelvin)?;
    let interval_k = convert(interval_value, interval_unit, UnitId::KelvinInterval)?;
    convert(
        absolute_k + interval_k,
        UnitId::Kelvin,
        target_absolute_unit,
    )
}

pub fn convert_pressure_kind(
    value: f64,
    from_unit: UnitId,
    from_kind: PressureKind,
    to_unit: UnitId,
    to_kind: PressureKind,
    reference: Option<&PressureReference>,
) -> Result<f64, UnitError> {
    require_dimension(from_unit, Dimension::Pressure)?;
    require_dimension(to_unit, Dimension::Pressure)?;
    let value_pa = convert(value, from_unit, UnitId::Pascal)?;
    let reference_pa = match (from_kind, to_kind) {
        (PressureKind::Absolute, PressureKind::Absolute)
        | (PressureKind::Gauge, PressureKind::Gauge) => 0.0,
        _ => {
            let reference = reference.ok_or(UnitError::MissingPressureReference)?;
            convert(reference.value, reference.unit, UnitId::Pascal)?
        }
    };
    let target_pa = match (from_kind, to_kind) {
        (PressureKind::Absolute, PressureKind::Absolute)
        | (PressureKind::Gauge, PressureKind::Gauge) => value_pa,
        (PressureKind::Gauge, PressureKind::Absolute) => value_pa + reference_pa,
        (PressureKind::Absolute, PressureKind::Gauge) => value_pa - reference_pa,
    };
    convert(target_pa, UnitId::Pascal, to_unit)
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnitError {
    UnknownDimension(String),
    RetiredDimensionAlias {
        alias: &'static str,
        replacement: &'static str,
    },
    UnknownUnit {
        symbol: String,
        dimension: Dimension,
    },
    DimensionMismatch {
        expected: Dimension,
        actual: Dimension,
    },
    InvalidQuantityKind {
        dimension: Dimension,
        quantity_kind: QuantityKind,
    },
    MissingPressureReference,
    MissingPressureReferenceProvenance,
    NonFiniteInput {
        name: &'static str,
        value: f64,
    },
    UnresolvedDimension,
}

impl fmt::Display for UnitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownDimension(value) => write!(f, "{value} is not an accepted dimension"),
            Self::RetiredDimensionAlias { alias, replacement } => {
                write!(f, "{alias} is retired; use {replacement}")
            }
            Self::UnknownUnit { symbol, dimension } => {
                write!(
                    f,
                    "{symbol} is not in the catalog for {}",
                    dimension.as_str()
                )
            }
            Self::DimensionMismatch { expected, actual } => write!(
                f,
                "dimension mismatch: expected {}, got {}",
                expected.as_str(),
                actual.as_str()
            ),
            Self::InvalidQuantityKind {
                dimension,
                quantity_kind,
            } => write!(
                f,
                "quantity kind {:?} is not valid for {}",
                quantity_kind,
                dimension.as_str()
            ),
            Self::MissingPressureReference => {
                write!(
                    f,
                    "gauge/absolute pressure conversion requires an explicit reference"
                )
            }
            Self::MissingPressureReferenceProvenance => {
                write!(f, "pressure reference requires non-empty provenance")
            }
            Self::NonFiniteInput { name, value } => write!(f, "{name} must be finite, got {value}"),
            Self::UnresolvedDimension => write!(f, "dimension is unresolved"),
        }
    }
}

impl Error for UnitError {}

const CATALOG: &[UnitId] = &[
    UnitId::One,
    UnitId::Meter,
    UnitId::Millimeter,
    UnitId::Inch,
    UnitId::Foot,
    UnitId::Kilogram,
    UnitId::PoundMass,
    UnitId::Second,
    UnitId::Kelvin,
    UnitId::DegreeCelsius,
    UnitId::DegreeFahrenheit,
    UnitId::DegreeRankine,
    UnitId::KelvinInterval,
    UnitId::DegreeCelsiusInterval,
    UnitId::DegreeFahrenheitInterval,
    UnitId::DegreeRankineInterval,
    UnitId::Radian,
    UnitId::Degree,
    UnitId::Newton,
    UnitId::PoundForce,
    UnitId::NewtonPerMeter,
    UnitId::PoundForcePerFoot,
    UnitId::PoundForcePerInch,
    UnitId::NewtonMeter,
    UnitId::PoundForceInch,
    UnitId::PoundForceFoot,
    UnitId::Pascal,
    UnitId::Kilopascal,
    UnitId::Megapascal,
    UnitId::Psi,
    UnitId::Ksi,
    UnitId::SquareMeter,
    UnitId::SquareMillimeter,
    UnitId::SquareInch,
    UnitId::CubicMeter,
    UnitId::CubicInch,
    UnitId::KilogramPerCubicMeter,
    UnitId::PoundMassPerCubicInch,
    UnitId::NewtonPerMeterLinear,
    UnitId::NewtonMeterPerRadian,
    UnitId::MeterPerSecond,
    UnitId::MeterPerSecondSquared,
    UnitId::WattPerMeterKelvin,
    UnitId::JoulePerKilogramKelvin,
    UnitId::PerKelvin,
    UnitId::PerDegreeCelsius,
    UnitId::MeterFourth,
    UnitId::MillimeterFourth,
    UnitId::InchFourth,
    UnitId::MeterCubed,
    UnitId::MillimeterCubed,
    UnitId::InchCubed,
    UnitId::KilogramPerMeter,
    UnitId::PoundMassPerFoot,
    UnitId::SquareMeterPerMeter,
];

fn def(
    id: UnitId,
    symbol: &'static str,
    dimension: Dimension,
    transform_to_canonical: Transform,
) -> UnitDefinition {
    let canonical = canonical_unit(dimension) == Some(id);
    UnitDefinition {
        id,
        symbol,
        dimension,
        transform_to_canonical,
        canonical,
        provenance: if canonical {
            ConversionProvenance::SiCanonical
        } else {
            ConversionProvenance::ExactPublicDefinition
        },
    }
}

fn require_dimension(unit: UnitId, expected: Dimension) -> Result<(), UnitError> {
    let actual = unit.definition().dimension;
    if actual == expected {
        Ok(())
    } else {
        Err(UnitError::DimensionMismatch { expected, actual })
    }
}

fn validate_finite(name: &'static str, value: f64) -> Result<(), UnitError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(UnitError::NonFiniteInput { name, value })
    }
}

fn unit_supports_dimension(unit_dimension: Dimension, requested: Dimension) -> bool {
    unit_dimension == requested
        || matches!(
            (unit_dimension, requested),
            (Dimension::Length, Dimension::Displacement)
                | (Dimension::Pressure, Dimension::Stress)
                | (Dimension::Angle, Dimension::Rotation)
                | (Dimension::Dimensionless, Dimension::Slope)
                | (Dimension::Area, Dimension::VolumePerLength)
                | (Dimension::ForcePerLength, Dimension::LinearStiffness)
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    const REL_TOL: f64 = 1.0e-12;

    fn assert_close(actual: f64, expected: f64) {
        let scale = expected.abs().max(1.0);
        assert!(
            (actual - expected).abs() <= REL_TOL * scale,
            "actual={actual}, expected={expected}"
        );
    }

    #[test]
    fn accepted_dimension_vocabulary_includes_force_per_length_and_rejects_retired_aliases() {
        assert_eq!(
            Dimension::from_schema_value("force_per_length").unwrap(),
            Dimension::ForcePerLength
        );
        assert_eq!(
            Dimension::ForcePerLength.vector(),
            Dimension::Force
                .vector()
                .divide(Dimension::Length.vector())
                .unwrap()
        );
        let err = Dimension::from_schema_value("temperature_difference").unwrap_err();
        assert_eq!(
            err,
            UnitError::RetiredDimensionAlias {
                alias: "temperature_difference",
                replacement: "temperature_interval"
            }
        );
    }

    #[test]
    fn catalog_uses_si_canonical_units_with_dual_display_entries() {
        assert_eq!(canonical_unit(Dimension::Length), Some(UnitId::Meter));
        assert_eq!(canonical_unit(Dimension::Pressure), Some(UnitId::Pascal));
        assert_eq!(canonical_unit(Dimension::Temperature), Some(UnitId::Kelvin));
        assert!(UnitId::PerKelvin.definition().canonical);
        assert!(!UnitId::PerDegreeCelsius.definition().canonical);
        assert_eq!(
            UnitId::PerDegreeCelsius.definition().provenance,
            ConversionProvenance::ExactPublicDefinition
        );
        assert_eq!(
            unit_by_symbol("psi", Dimension::Pressure).unwrap(),
            UnitId::Psi
        );
        assert_eq!(
            unit_by_symbol("degF", Dimension::TemperatureInterval).unwrap(),
            UnitId::DegreeFahrenheitInterval
        );
        assert_eq!(
            unit_by_symbol("MPa", Dimension::Stress).unwrap(),
            UnitId::Megapascal
        );
    }

    #[test]
    fn multiplicative_conversions_use_reviewed_public_definitions() {
        assert_close(
            convert(1.0, UnitId::Inch, UnitId::Millimeter).unwrap(),
            25.4,
        );
        assert_close(
            convert(1.0, UnitId::PoundForce, UnitId::Newton).unwrap(),
            POUND_FORCE_IN_NEWTONS,
        );
        assert_close(
            convert(1.0, UnitId::Psi, UnitId::Pascal).unwrap(),
            PSI_IN_PASCALS,
        );
        assert_close(
            convert(12.0, UnitId::PoundForceInch, UnitId::PoundForceFoot).unwrap(),
            1.0,
        );
    }

    #[test]
    fn incompatible_dimensions_are_rejected() {
        let err = convert(1.0, UnitId::Meter, UnitId::Newton).unwrap_err();
        assert_eq!(
            err,
            UnitError::DimensionMismatch {
                expected: Dimension::Length,
                actual: Dimension::Force
            }
        );
    }

    #[test]
    fn conversion_can_be_bound_to_semantic_dimensions_with_shared_units() {
        assert_close(
            convert_for_dimension(1.0, Dimension::Stress, UnitId::Megapascal, UnitId::Psi).unwrap(),
            1_000_000.0 / PSI_IN_PASCALS,
        );
        assert_close(
            convert_for_dimension(
                25.4,
                Dimension::Displacement,
                UnitId::Millimeter,
                UnitId::Inch,
            )
            .unwrap(),
            1.0,
        );
    }

    #[test]
    fn absolute_temperature_uses_affine_conversion() {
        assert_close(
            convert(25.0, UnitId::DegreeCelsius, UnitId::Kelvin).unwrap(),
            298.15,
        );
        assert_close(
            convert(68.0, UnitId::DegreeFahrenheit, UnitId::Kelvin).unwrap(),
            293.15,
        );
        assert_close(
            convert(293.15, UnitId::Kelvin, UnitId::DegreeFahrenheit).unwrap(),
            68.0,
        );
    }

    #[test]
    fn temperature_intervals_are_not_offset() {
        assert_close(
            convert(
                18.0,
                UnitId::DegreeFahrenheitInterval,
                UnitId::KelvinInterval,
            )
            .unwrap(),
            10.0,
        );
        assert_close(
            temperature_difference(
                30.0,
                UnitId::DegreeCelsius,
                50.0,
                UnitId::DegreeFahrenheit,
                UnitId::DegreeCelsiusInterval,
            )
            .unwrap(),
            20.0,
        );
        assert_close(
            shift_temperature(
                20.0,
                UnitId::DegreeCelsius,
                18.0,
                UnitId::DegreeFahrenheitInterval,
                UnitId::DegreeCelsius,
            )
            .unwrap(),
            30.0,
        );
    }

    #[test]
    fn pressure_unit_conversion_does_not_change_gauge_absolute_kind() {
        assert_close(
            convert_pressure_kind(
                100.0,
                UnitId::Kilopascal,
                PressureKind::Gauge,
                UnitId::Psi,
                PressureKind::Gauge,
                None,
            )
            .unwrap(),
            100_000.0 / PSI_IN_PASCALS,
        );
    }

    #[test]
    fn gauge_absolute_pressure_conversion_requires_explicit_reference() {
        let err = convert_pressure_kind(
            50.0,
            UnitId::Kilopascal,
            PressureKind::Gauge,
            UnitId::Kilopascal,
            PressureKind::Absolute,
            None,
        )
        .unwrap_err();
        assert_eq!(err, UnitError::MissingPressureReference);

        let reference = PressureReference::standard_atmosphere("public conventional constant")
            .expect("reference pressure");
        assert_close(
            convert_pressure_kind(
                50.0,
                UnitId::Kilopascal,
                PressureKind::Gauge,
                UnitId::Kilopascal,
                PressureKind::Absolute,
                Some(&reference),
            )
            .unwrap(),
            151.325,
        );
    }

    #[test]
    fn quantity_kind_blocks_temperature_scale_interval_mixups() {
        let err = convert_quantity(
            10.0,
            UnitId::DegreeCelsius,
            UnitId::Kelvin,
            QuantityKind::Interval,
        )
        .unwrap_err();
        assert_eq!(
            err,
            UnitError::InvalidQuantityKind {
                dimension: Dimension::Temperature,
                quantity_kind: QuantityKind::Interval
            }
        );

        let err = convert_quantity(
            10.0,
            UnitId::DegreeCelsius,
            UnitId::Kelvin,
            QuantityKind::UnitBearing,
        )
        .unwrap_err();
        assert_eq!(
            err,
            UnitError::InvalidQuantityKind {
                dimension: Dimension::Temperature,
                quantity_kind: QuantityKind::UnitBearing
            }
        );

        let err = convert_quantity(
            100.0,
            UnitId::Kilopascal,
            UnitId::Psi,
            QuantityKind::UnitBearing,
        )
        .unwrap_err();
        assert_eq!(
            err,
            UnitError::InvalidQuantityKind {
                dimension: Dimension::Pressure,
                quantity_kind: QuantityKind::UnitBearing
            }
        );
    }

    #[test]
    fn nonfinite_values_are_rejected() {
        let err = convert(f64::NAN, UnitId::Meter, UnitId::Foot).unwrap_err();
        match err {
            UnitError::NonFiniteInput { name, value } => {
                assert_eq!(name, "quantity");
                assert!(value.is_nan());
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
