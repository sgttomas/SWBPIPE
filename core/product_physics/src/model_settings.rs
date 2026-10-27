//! Model-level settings carried by the model document (DEL-02-01 Design
//! `model-document-0.7.0.md` §3.8): the optional top-level `model_settings`
//! record. Its one member, `node_proximity_tolerance`, is the length within which
//! the near-duplicate node check treats two nodes as near duplicates. The owner's
//! ruling fixes a 5 mm default "and give the user the means to change it"; the
//! setting lives in the document so the desktop and the headless runner warn
//! identically. An absent record or member means the default, and the product
//! never writes the default into a document the user did not author.
//!
//! The topology check (`topology::append_topology_diagnostics`) reads it through
//! [`PreviewModel::node_proximity_tolerance_m`]. The solve's validation
//! (`validation::validate_model_settings`) and the document validator
//! (`operation_applier::document_validator`) refuse a malformed value with the
//! same codes and messages as the accessor below.
use crate::{PreviewModel, Quantity};
use open_pipe_stress_units::{canonical_unit, convert_for_dimension, unit_by_symbol, Dimension};
use serde::Deserialize;

/// The default near-duplicate node tolerance, 5 mm, in metres.
pub const DEFAULT_NODE_PROXIMITY_TOLERANCE_M: f64 = 0.005;

/// The tolerance's unit is not a length unit of the units catalog (unknown or another dimension).
pub const MODEL_SETTING_TOLERANCE_UNIT_NOT_LENGTH: &str = "MODEL-SETTING-TOLERANCE-UNIT-NOT-LENGTH";
/// The tolerance's value is zero or negative.
pub const MODEL_SETTING_TOLERANCE_NOT_POSITIVE: &str = "MODEL-SETTING-TOLERANCE-NOT-POSITIVE";
/// The tolerance's value is not a finite number.
pub const MODEL_SETTING_TOLERANCE_NOT_FINITE: &str = "MODEL-SETTING-TOLERANCE-NOT-FINITE";

/// The path of the setting in the document.
pub const NODE_PROXIMITY_TOLERANCE_PATH: &str = "model_settings.node_proximity_tolerance";

/// `model_settings`: a closed record; every member optional.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelSettings {
    /// The near-duplicate node tolerance, a length quantity. Absent: 5 mm. An authored `null`
    /// is not absence: it is refused, as for the `model_settings` record itself.
    #[serde(default, deserialize_with = "crate::deserialize_present_object")]
    pub node_proximity_tolerance: Option<Quantity>,
}

/// A refused setting: its named code and the message the validator also gives.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelSettingError {
    pub code: &'static str,
    pub message: String,
}

/// The refusal message for a non-finite tolerance value.
pub fn tolerance_not_finite_message(value: impl std::fmt::Display) -> String {
    format!("{NODE_PROXIMITY_TOLERANCE_PATH}.value is {value}; a finite length greater than zero is required.")
}

/// The refusal message for a zero or negative tolerance value.
pub fn tolerance_not_positive_message(value: impl std::fmt::Display) -> String {
    format!("{NODE_PROXIMITY_TOLERANCE_PATH}.value is {value}; the tolerance must be greater than zero.")
}

/// The refusal message for a unit that is not a catalog length unit.
pub fn tolerance_unit_not_length_message(unit: &str) -> String {
    format!("{NODE_PROXIMITY_TOLERANCE_PATH}.unit `{unit}` is not a length unit of the units catalog.")
}

/// The refusal message for a positive value that is zero in metres (the conversion underflows).
pub fn tolerance_zero_in_metres_message(value: f64, unit: &str) -> String {
    format!("{NODE_PROXIMITY_TOLERANCE_PATH} is {value:?} {unit}, which is zero in metres; the tolerance must be greater than zero.")
}

/// The refusal message for a finite value that is not finite in metres (the conversion overflows).
pub fn tolerance_not_finite_in_metres_message(value: f64, unit: &str) -> String {
    format!("{NODE_PROXIMITY_TOLERANCE_PATH} is {value:?} {unit}, which is not finite in metres; a finite length greater than zero is required.")
}

/// Whether `unit` is a length unit of the units catalog.
pub fn is_catalog_length_unit(unit: &str) -> bool {
    unit_by_symbol(unit, Dimension::Length).is_ok()
}

/// The effective tolerance in metres of a quantity: finite, then positive, then a catalog length unit,
/// then finite and greater than zero once converted to metres.
pub fn node_proximity_tolerance_m(quantity: &Quantity) -> Result<f64, ModelSettingError> {
    if !quantity.value.is_finite() {
        return Err(ModelSettingError {
            code: MODEL_SETTING_TOLERANCE_NOT_FINITE,
            message: tolerance_not_finite_message(quantity.value),
        });
    }
    if quantity.value <= 0.0 {
        return Err(ModelSettingError {
            code: MODEL_SETTING_TOLERANCE_NOT_POSITIVE,
            message: tolerance_not_positive_message(quantity.value),
        });
    }
    let not_length = || ModelSettingError {
        code: MODEL_SETTING_TOLERANCE_UNIT_NOT_LENGTH,
        message: tolerance_unit_not_length_message(&quantity.unit),
    };
    let from = unit_by_symbol(&quantity.unit, Dimension::Length).map_err(|_| not_length())?;
    let to = canonical_unit(Dimension::Length).ok_or_else(not_length)?;
    let metres = convert_for_dimension(quantity.value, Dimension::Length, from, to).map_err(|_| not_length())?;
    if !metres.is_finite() {
        return Err(ModelSettingError {
            code: MODEL_SETTING_TOLERANCE_NOT_FINITE,
            message: tolerance_not_finite_in_metres_message(quantity.value, &quantity.unit),
        });
    }
    if metres <= 0.0 {
        return Err(ModelSettingError {
            code: MODEL_SETTING_TOLERANCE_NOT_POSITIVE,
            message: tolerance_zero_in_metres_message(quantity.value, &quantity.unit),
        });
    }
    Ok(metres)
}

impl PreviewModel {
    /// The near-duplicate node tolerance in metres: the authored
    /// `model_settings.node_proximity_tolerance`, else 5 mm.
    pub fn node_proximity_tolerance_m(&self) -> Result<f64, ModelSettingError> {
        match self.model_settings.as_ref().and_then(|settings| settings.node_proximity_tolerance.as_ref()) {
            Some(quantity) => node_proximity_tolerance_m(quantity),
            None => Ok(DEFAULT_NODE_PROXIMITY_TOLERANCE_M),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parsed(settings: Option<serde_json::Value>) -> Result<PreviewModel, serde_json::Error> {
        let mut document: serde_json::Value =
            serde_json::from_str(include_str!("../../../fixtures/product_preview/invented_demo_model.json")).unwrap();
        match settings {
            Some(value) => document["model_settings"] = value,
            None => {
                document.as_object_mut().unwrap().remove("model_settings");
            }
        }
        serde_json::from_value(document)
    }

    fn quantity(value: f64, unit: &str) -> Quantity {
        Quantity { value, unit: unit.into() }
    }

    #[test]
    fn absent_settings_use_the_five_millimetre_default() {
        assert_eq!(parsed(None).unwrap().node_proximity_tolerance_m(), Ok(0.005));
        let empty = parsed(Some(serde_json::json!({}))).unwrap();
        assert!(empty.model_settings.is_some());
        assert_eq!(empty.node_proximity_tolerance_m(), Ok(0.005));
    }

    #[test]
    fn an_authored_tolerance_converts_to_metres() {
        let model = parsed(Some(serde_json::json!({"node_proximity_tolerance": {"value": 2.0, "unit": "mm"}}))).unwrap();
        assert_eq!(model.node_proximity_tolerance_m(), Ok(0.002));
        let model = parsed(Some(serde_json::json!({"node_proximity_tolerance": {"value": 0.01, "unit": "m"}}))).unwrap();
        assert_eq!(model.node_proximity_tolerance_m(), Ok(0.01));
    }

    #[test]
    fn the_record_is_closed_and_an_object() {
        let error = parsed(Some(serde_json::json!({"proximity": 1}))).err().expect("unknown member refused").to_string();
        assert!(error.contains("unknown field `proximity`"), "{error}");
        assert!(parsed(Some(serde_json::Value::Null)).is_err(), "null is not an absent record");
        let error = parsed(Some(serde_json::json!({"node_proximity_tolerance": null})))
            .err()
            .expect("an authored null tolerance is not the default")
            .to_string();
        assert!(error.contains("an authored object"), "{error}");
        assert!(parsed(Some(serde_json::json!([]))).is_err());
    }

    #[test]
    fn each_refusal_is_named_with_its_message() {
        assert_eq!(
            node_proximity_tolerance_m(&quantity(f64::NAN, "mm")),
            Err(ModelSettingError {
                code: "MODEL-SETTING-TOLERANCE-NOT-FINITE",
                message: "model_settings.node_proximity_tolerance.value is NaN; a finite length greater than zero is required.".into(),
            })
        );
        assert_eq!(
            node_proximity_tolerance_m(&quantity(f64::INFINITY, "mm")),
            Err(ModelSettingError {
                code: "MODEL-SETTING-TOLERANCE-NOT-FINITE",
                message: "model_settings.node_proximity_tolerance.value is inf; a finite length greater than zero is required.".into(),
            })
        );
        assert_eq!(
            node_proximity_tolerance_m(&quantity(0.0, "mm")),
            Err(ModelSettingError {
                code: "MODEL-SETTING-TOLERANCE-NOT-POSITIVE",
                message: "model_settings.node_proximity_tolerance.value is 0; the tolerance must be greater than zero.".into(),
            })
        );
        assert_eq!(
            node_proximity_tolerance_m(&quantity(-1.5, "mm")),
            Err(ModelSettingError {
                code: "MODEL-SETTING-TOLERANCE-NOT-POSITIVE",
                message: "model_settings.node_proximity_tolerance.value is -1.5; the tolerance must be greater than zero.".into(),
            })
        );
        for unit in ["N", "rad", "mmm"] {
            assert_eq!(
                node_proximity_tolerance_m(&quantity(5.0, unit)),
                Err(ModelSettingError {
                    code: "MODEL-SETTING-TOLERANCE-UNIT-NOT-LENGTH",
                    message: format!("model_settings.node_proximity_tolerance.unit `{unit}` is not a length unit of the units catalog."),
                })
            );
        }
        assert_eq!(
            node_proximity_tolerance_m(&quantity(5e-324, "mm")),
            Err(ModelSettingError {
                code: "MODEL-SETTING-TOLERANCE-NOT-POSITIVE",
                message: "model_settings.node_proximity_tolerance is 5e-324 mm, which is zero in metres; the tolerance must be greater than zero.".into(),
            })
        );
        let model = parsed(Some(serde_json::json!({"node_proximity_tolerance": {"value": 5.0, "unit": "N"}}))).unwrap();
        assert_eq!(model.node_proximity_tolerance_m().unwrap_err().code, MODEL_SETTING_TOLERANCE_UNIT_NOT_LENGTH);
    }
}
