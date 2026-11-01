//! Model document 0.9.0's `spectra[]` (DEL-02-01 Design `model-document-0.9.0.md`
//! §2.4; the dynamics lane's text): user-entered response spectra, data only, so
//! that a spectrum is entered once. They are referenced only from DEL-04-10's
//! spectrum analysis request, never from load cases, and no solve reads them in
//! 0.9.0: the lowering removes them, so a document's static results do not
//! depend on them.
//!
//! The record is closed and an authored `null` is refused for every member (the
//! typed decode). Any other violation is `SPECTRUM_RECORD_INVALID`, naming the
//! record and the field (document-level, blocking, at the validator and the solve).
use crate::document_members::{Finding, MemberFindings};
use crate::Quantity;
use open_pipe_stress_units::{unit_by_symbol, Dimension};
use serde::Deserialize;
use std::collections::HashSet;

/// A spectrum record violates a rule of §2.4 other than its id's.
pub const SPECTRUM_RECORD_INVALID: &str = "SPECTRUM_RECORD_INVALID";
/// Two spectra share an id.
pub const SPECTRUM_ID_DUPLICATE: &str = "SPECTRUM_ID_DUPLICATE";
/// The entity name of a spectrum in `EMPTY_ID`.
pub const SPECTRUM_ENTITY: &str = "spectrum";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpectrumAbscissa {
    Frequency,
    Period,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpectrumOrdinate {
    Acceleration,
    GMultiple,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpectrumScale {
    Linear,
    Log,
}

/// One spectrum (closed).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpectrumInput {
    pub id: String,
    #[serde(default, deserialize_with = "crate::deserialize_present_value")]
    pub label: Option<String>,
    pub abscissa: SpectrumAbscissa,
    pub ordinate: SpectrumOrdinate,
    pub units: SpectrumUnitsInput,
    #[serde(default, deserialize_with = "crate::model_fields::closed_present_quantity")]
    pub g_value: Option<Quantity>,
    pub damping_ratio: f64,
    pub interpolation: SpectrumInterpolationInput,
    pub points: Vec<[f64; 2]>,
    #[serde(default)]
    pub tags: serde::de::IgnoredAny,
    #[serde(default)]
    pub notes: serde::de::IgnoredAny,
}

/// The catalog unit of each axis (closed).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpectrumUnitsInput {
    pub abscissa: String,
    #[serde(default, deserialize_with = "crate::deserialize_present_value")]
    pub ordinate: Option<String>,
}

/// The interpolation of each axis (closed); both required, no default.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpectrumInterpolationInput {
    pub abscissa: SpectrumScale,
    pub ordinate: SpectrumScale,
}

/// §2.4's rules over the spectra, in document order.
pub(crate) fn findings(spectra: &[SpectrumInput], found: &mut MemberFindings) {
    let mut seen = HashSet::new();
    let mut reported = HashSet::new();
    for (index, spectrum) in spectra.iter().enumerate() {
        let at = format!("spectra[{index}]");
        let refs = vec![spectrum.id.clone()];
        let invalid = |field: &str, reason: String| {
            let path = format!("{at}.{field}");
            Finding::new(SPECTRUM_RECORD_INVALID, &path, format!("spectrum `{}`: {path} {reason}.", spectrum.id), refs.clone())
        };
        if spectrum.id.trim().is_empty() {
            found.push(Finding::empty_id(SPECTRUM_ENTITY, &format!("{at}.id")));
        } else if !seen.insert(spectrum.id.as_str()) && reported.insert(spectrum.id.as_str()) {
            let path = format!("{at}.id");
            found.push(Finding::new(SPECTRUM_ID_DUPLICATE, &path, format!("{path} `{}` repeats the id of another spectrum; spectrum ids are unique.", spectrum.id), refs.clone()));
        }
        let (dimension, family) = match spectrum.abscissa {
            SpectrumAbscissa::Frequency => (Dimension::Frequency, "frequency"),
            SpectrumAbscissa::Period => (Dimension::Time, "time"),
        };
        if unit_by_symbol(&spectrum.units.abscissa, dimension).is_err() {
            found.push(invalid("units.abscissa", format!("`{}` is not a {family} unit of the units catalog", spectrum.units.abscissa)));
        }
        let acceleration = spectrum.ordinate == SpectrumOrdinate::Acceleration;
        match (&spectrum.units.ordinate, acceleration) {
            (None, true) => found.push(invalid("units.ordinate", "is required when ordinate is acceleration".into())),
            (Some(_), false) => found.push(invalid("units.ordinate", "is present only when ordinate is acceleration".into())),
            (Some(unit), true) if unit_by_symbol(unit, Dimension::Acceleration).is_err() => {
                found.push(invalid("units.ordinate", format!("`{unit}` is not an acceleration unit of the units catalog")));
            }
            _ => {}
        }
        match (&spectrum.g_value, acceleration) {
            (None, false) => found.push(invalid("g_value", "is required when ordinate is g_multiple".into())),
            (Some(_), true) => found.push(invalid("g_value", "is present only when ordinate is g_multiple".into())),
            (Some(g), false) => {
                if unit_by_symbol(&g.unit, Dimension::Acceleration).is_err() {
                    found.push(invalid("g_value", format!("unit `{}` is not an acceleration unit of the units catalog", g.unit)));
                } else if !(g.value.is_finite() && g.value > 0.0) {
                    found.push(invalid("g_value", format!("value {} is not a positive finite acceleration", g.value)));
                }
            }
            (None, true) => {}
        }
        if !(spectrum.damping_ratio.is_finite() && (0.0..1.0).contains(&spectrum.damping_ratio)) {
            found.push(invalid("damping_ratio", format!("{} is not a finite ratio in [0, 1)", spectrum.damping_ratio)));
        }
        if spectrum.points.len() < 2 {
            found.push(invalid("points", format!("has {} points; a spectrum has at least two", spectrum.points.len())));
        }
        let mut previous: Option<f64> = None;
        for (k, [x, y]) in spectrum.points.iter().copied().enumerate() {
            if !(x.is_finite() && x > 0.0) {
                found.push(invalid(&format!("points[{k}]"), format!("abscissa {x} is not positive and finite")));
            } else if previous.is_some_and(|before| x <= before) {
                found.push(invalid(&format!("points[{k}]"), format!("abscissa {x} does not strictly increase")));
            }
            if !(y.is_finite() && y > 0.0) {
                found.push(invalid(&format!("points[{k}]"), format!("ordinate {y} is not positive and finite")));
            }
            if x.is_finite() {
                previous = Some(x);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spectrum(value: serde_json::Value) -> Result<SpectrumInput, serde_json::Error> {
        serde_json::from_value(value)
    }

    fn g_spectrum() -> serde_json::Value {
        serde_json::json!({"id": "sp", "abscissa": "frequency", "ordinate": "g_multiple", "units": {"abscissa": "Hz"},
            "g_value": {"value": 9.80665, "unit": "m/s^2"}, "damping_ratio": 0.05,
            "interpolation": {"abscissa": "log", "ordinate": "log"}, "points": [[1.0, 0.2], [10.0, 0.5]]})
    }

    fn codes(value: serde_json::Value) -> Vec<(&'static str, String)> {
        let mut found = MemberFindings::default();
        findings(&[spectrum(value).unwrap()], &mut found);
        found.into_vec().into_iter().map(|f| (f.code, f.path)).collect()
    }

    #[test]
    fn a_valid_spectrum_has_no_finding_and_each_rule_names_its_field() {
        assert!(codes(g_spectrum()).is_empty());
        let mut acceleration = g_spectrum();
        acceleration["ordinate"] = "acceleration".into();
        acceleration["abscissa"] = "period".into();
        acceleration["units"] = serde_json::json!({"abscissa": "s", "ordinate": "m/s^2"});
        acceleration.as_object_mut().unwrap().remove("g_value");
        assert!(codes(acceleration).is_empty());
        for (mutate, field) in [
            (Box::new(|v: &mut serde_json::Value| v["units"]["abscissa"] = "s".into()) as Box<dyn Fn(&mut serde_json::Value)>, "spectra[0].units.abscissa"),
            (Box::new(|v| v["units"]["ordinate"] = "m/s^2".into()), "spectra[0].units.ordinate"),
            (Box::new(|v| { v.as_object_mut().unwrap().remove("g_value"); }), "spectra[0].g_value"),
            (Box::new(|v| v["g_value"]["value"] = 0.0.into()), "spectra[0].g_value"),
            (Box::new(|v| v["g_value"]["value"] = (-1.0).into()), "spectra[0].g_value"),
            (Box::new(|v| v["g_value"]["unit"] = "Hz".into()), "spectra[0].g_value"),
            (Box::new(|v| v["damping_ratio"] = 1.0.into()), "spectra[0].damping_ratio"),
            (Box::new(|v| v["damping_ratio"] = (-0.01).into()), "spectra[0].damping_ratio"),
            (Box::new(|v| v["points"] = serde_json::json!([[1.0, 0.2]])), "spectra[0].points"),
            (Box::new(|v| v["points"] = serde_json::json!([[1.0, 0.2], [1.0, 0.3]])), "spectra[0].points[1]"),
            (Box::new(|v| v["points"] = serde_json::json!([[0.0, 0.2], [1.0, 0.3]])), "spectra[0].points[0]"),
            (Box::new(|v| v["points"] = serde_json::json!([[1.0, 0.0], [2.0, 0.3]])), "spectra[0].points[0]"),
        ] {
            let mut value = g_spectrum();
            mutate(&mut value);
            assert_eq!(codes(value), [(SPECTRUM_RECORD_INVALID, field.to_string())]);
        }
    }

    #[test]
    fn the_record_is_closed_and_null_is_not_absence() {
        for (key, value) in [("extra", serde_json::json!(1)), ("label", serde_json::Value::Null), ("g_value", serde_json::Value::Null)] {
            let mut record = g_spectrum();
            record[key] = value;
            assert!(spectrum(record).is_err(), "{key}");
        }
        let mut record = g_spectrum();
        record["units"]["ordinate"] = serde_json::Value::Null;
        assert!(spectrum(record).is_err());
        let mut record = g_spectrum();
        record["points"] = serde_json::json!([[1.0, 0.2, 3.0], [2.0, 0.3]]);
        assert!(spectrum(record).is_err());
        let mut record = g_spectrum();
        record["interpolation"] = serde_json::json!({"abscissa": "log"});
        assert!(spectrum(record).is_err());
    }
}
