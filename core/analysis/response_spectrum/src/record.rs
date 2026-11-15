//! The model document's spectrum record (`spectra[]`, DEL-02-01 Design
//! `model-document-0.9.0.md` §2.4; `schemas/model.schema.yaml` `Spectrum`), as authored.
//!
//! The record is closed: an unknown member, a missing required member or an authored `null`
//! is a decode error, so no member is defaulted. `label`, `tags` and `notes` carry no solve
//! meaning and are read only so that a document's record decodes unchanged.
use serde::{Deserialize, Deserializer};

/// The authored abscissa of a table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Abscissa {
    Frequency,
    Period,
}

/// The authored ordinate: an acceleration in a catalog unit, or a multiple of the user's g.
/// Either is a pseudo-spectral acceleration (REQ-001).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrdinateKind {
    Acceleration,
    GMultiple,
}

/// The interpolation scale of one axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AxisScale {
    Linear,
    Log,
}

/// The interpolation of each axis; both are required and neither has a default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Interpolation {
    pub abscissa: AxisScale,
    pub ordinate: AxisScale,
}

/// An authored quantity, `{value, unit}`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QuantityInput {
    pub value: f64,
    pub unit: String,
}

/// The catalog unit of each axis. `ordinate` is present exactly when the ordinate is an
/// acceleration.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordUnits {
    pub abscissa: String,
    #[serde(default, deserialize_with = "present")]
    pub ordinate: Option<String>,
}

/// One spectrum record as authored.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpectrumRecord {
    pub id: String,
    #[serde(default, deserialize_with = "present")]
    pub label: Option<String>,
    pub abscissa: Abscissa,
    pub ordinate: OrdinateKind,
    pub units: RecordUnits,
    /// Present exactly when the ordinate is a g multiple.
    #[serde(default, deserialize_with = "present")]
    pub g_value: Option<QuantityInput>,
    pub damping_ratio: f64,
    pub interpolation: Interpolation,
    /// `[abscissa, ordinate]` pairs in the authored units.
    pub points: Vec<[f64; 2]>,
    #[serde(default)]
    pub tags: Option<serde_json::Value>,
    #[serde(default)]
    pub notes: Option<serde_json::Value>,
}

/// An optional member that, when present, must hold a value: an authored `null` is refused.
fn present<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(deserializer).map(Some)
}
