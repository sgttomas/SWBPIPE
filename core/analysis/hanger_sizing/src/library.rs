//! Reads an imported hanger library document into sizing records (DEL-04-11
//! Design §8.2). The document has already passed the import boundary; this
//! reader normalizes its quantities to SI through DEL-02-02's unit catalog and
//! keeps a missing or unreadable quantity absent, so that the record fails E1
//! ("not sizable") instead of taking any value in its place.

use crate::{Dof, HangerType, Record};
use open_pipe_stress_units::{canonical_unit, convert, unit_by_symbol, Dimension};
use serde_json::Value;

fn quantity(value: Option<&Value>, dimension: Dimension) -> Option<f64> {
    let value = value?;
    let magnitude = value.get("magnitude")?.as_f64()?;
    let unit = unit_by_symbol(value.get("unit")?.as_str()?, dimension).ok()?;
    let canonical = canonical_unit(dimension)?;
    convert(magnitude, unit, canonical).ok().filter(|v| v.is_finite())
}

fn dof(value: Option<&Value>) -> Option<Dof> {
    match value?.as_str()? {
        "UX" => Some(Dof::Ux),
        "UY" => Some(Dof::Uy),
        "UZ" => Some(Dof::Uz),
        _ => None,
    }
}

/// The library's records, in document order. A record of an unknown hanger type
/// is skipped; the import boundary admits only the two families.
pub fn records(document: &Value) -> Vec<Record> {
    let mut out = Vec::new();
    for record in document.get("hanger_records").and_then(Value::as_array).into_iter().flatten() {
        let Some(hanger) = record.get("hanger") else { continue };
        let hanger_type = match hanger.get("hanger_type").and_then(Value::as_str) {
            Some("variable_spring_hanger") => HangerType::VariableSpring,
            Some("constant_effort_support") => HangerType::ConstantEffort,
            _ => continue,
        };
        let Some(hanger_id) = record.get("hanger_id").and_then(Value::as_str) else { continue };
        let stiffness = hanger.get("stiffness");
        let range = hanger.get("working_load_range");
        let minimum = quantity(range.and_then(|r| r.get("minimum")), Dimension::Force);
        let maximum = quantity(range.and_then(|r| r.get("maximum")), Dimension::Force);
        out.push(Record {
            hanger_id: hanger_id.to_string(),
            hanger_type,
            stiffness_n_per_m: quantity(stiffness.and_then(|s| s.get("value")), Dimension::ForcePerLength),
            stiffness_dof: dof(stiffness.and_then(|s| s.get("dof"))),
            working_load_range_n: minimum.zip(maximum),
            travel_range_m: quantity(hanger.get("travel_range"), Dimension::Length),
        });
    }
    out
}
