//! Authoring of the model-level settings (DEL-02-01 Design
//! `model-document-0.7.0.md` §3.8; DEL-04-06 Design `model-topology-checks.md`,
//! "User control of the node-proximity tolerance"): `set_model_setting` and
//! `clear_model_setting` on `model_settings.node_proximity_tolerance`.
//!
//! The owner ruled a 5 mm default "and give the user the means to change it".
//! The product never writes the default into a document the user did not
//! author: setting writes the value the user authored, with its unit (an
//! authored 5 mm included), and clearing removes the member so the default
//! applies again.
//!
//! Wire (the `Project` singleton target, as DEL-02-02 Design
//! `project-unit-selection.md` §2.7 defines it for project-level edits):
//! - `target`: object type `Project`, `ref` the document's `project.id`; any
//!   other `ref` is refused (`OP-TARGET-NOT-FOUND`).
//! - `field_path`: `model_settings.node_proximity_tolerance` or, since model
//!   document 0.8.0, `model_settings.shear_model`.
//! - `before`: the current member's canonical JSON, or `not_present`; a
//!   `before` that no longer matches is stale (`OP-STALE-BEFORE-VALUE`).
//! - **Set** (`set_model_setting`, operation kind `modify`): `after` is the
//!   entered value as number text, `unit` its length unit and `dimension`
//!   `length`. It writes `{value, unit}` and creates `model_settings` when
//!   absent. The value and unit are judged by product_physics' accessor
//!   (`model_settings::node_proximity_tolerance_m`), so a refusal carries the
//!   accessor's code and message, which the document validator also gives:
//!   `MODEL-SETTING-TOLERANCE-NOT-FINITE`, `-NOT-POSITIVE`, `-UNIT-NOT-LENGTH`.
//! - **Clear** (`clear_model_setting`, operation kind `modify`): `after`
//!   `not_present`, unit `none`, dimension `dimensionless`. It removes the
//!   member, and removes `model_settings` when that leaves it empty, so a
//!   set followed by a clear restores the document exactly when the record was
//!   absent before the set. Clearing an absent setting, or setting the value
//!   already stored, is refused as `OP-MODEL-SETTING-UNCHANGED`.
//!
//! - **Shear model** (model document 0.8.0): set takes `after` the name, unit
//!   `none` and dimension `dimensionless`, and writes the string; it is judged by
//!   product_physics' `model_settings::shear_model_refusal`, so a value other
//!   than the two names (`MODEL-SETTING-SHEAR-MODEL-UNKNOWN`) and a document with
//!   `pressure_contract` (`-WITH-PRESSURE-CONTRACT`) are refused by name; both
//!   names are admitted on a pressure-free document (DEL-05-03 Design
//!   `pf-timoshenko.md` §1). An explicit set of `euler_bernoulli` writes the member, as for the
//!   tolerance (Agent 0, DEL-02-01 Design `model-document-0.8.0.md` §0, R7-1);
//!   clear removes the member as for the tolerance. Nothing writes the default
//!   unless the user sets it.
//!
//! Each operation's inverse is the other one, built from the diff row's
//! `before`; the desktop undoes through its session checkpoint, which restores
//! the prior document exactly.

use crate::geometry_operations::{GeometryEdit, GeometryError};
use crate::{canonical_json, DiffPreviewRow};
use open_pipe_stress_product_physics::model_settings as ms;
use serde_json::{Map, Value};

/// Sets a model-level setting to an authored value.
pub(crate) const SET: &str = "set_model_setting";
/// Clears a model-level setting, so its default applies.
pub(crate) const CLEAR: &str = "clear_model_setting";

/// True for a change kind this module resolves.
pub(crate) fn is_change_kind(change_kind: &str) -> bool {
    matches!(change_kind, SET | CLEAR)
}

fn refusal(code: &'static str, message: impl Into<String>, refs: &[String]) -> GeometryError {
    GeometryError { code, message: message.into(), affected_refs: refs.to_vec() }
}

fn text_at<'a>(intent: &'a Value, pointer: &str) -> &'a str {
    intent.pointer(pointer).and_then(Value::as_str).unwrap_or("")
}

/// Resolve one set or clear of `model_settings.node_proximity_tolerance` to
/// its candidate document and diff row.
pub(crate) fn resolve(model: &Value, intent: &Value) -> Result<GeometryEdit, GeometryError> {
    let change_kind = text_at(intent, "/change/change_kind");
    let object_type = text_at(intent, "/target/object_type");
    let target_ref = text_at(intent, "/target/ref");
    let field_path = text_at(intent, "/change/field_path");
    let before = text_at(intent, "/change/before");
    let after = text_at(intent, "/change/after");
    let unit = text_at(intent, "/change/unit");
    let dimension = text_at(intent, "/change/dimension");
    let refs = [target_ref.to_string()];
    let settings_list = format!("`{}` and `{}`", ms::NODE_PROXIMITY_TOLERANCE_PATH, ms::SHEAR_MODEL_PATH);

    if object_type != "Project" {
        return Err(refusal(
            "OP-MODEL-SETTING-SHAPE-INVALID",
            format!("{change_kind} targets the Project singleton, not {object_type}."),
            &refs,
        ));
    }
    let (setting, member) = match field_path {
        ms::NODE_PROXIMITY_TOLERANCE_PATH => (ms::NODE_PROXIMITY_TOLERANCE_PATH, "node_proximity_tolerance"),
        ms::SHEAR_MODEL_PATH => (ms::SHEAR_MODEL_PATH, "shear_model"),
        _ => {
            return Err(refusal(
                "OP-MODEL-SETTING-UNKNOWN",
                format!("`{field_path}` is not a model setting; the settings are {settings_list}."),
                &refs,
            ));
        }
    };
    if model.get("document_kind").and_then(Value::as_str) != Some("openpipestress.product_preview.model")
        || model.get("schema_version").and_then(Value::as_str) != Some(open_pipe_stress_product_physics::MODEL_SCHEMA_VERSION)
    {
        return Err(refusal(
            "OP-MODEL-SETTING-DOCUMENT-INVALID",
            format!(
                "Model settings are authored on the current product_preview.model document {}.",
                open_pipe_stress_product_physics::MODEL_SCHEMA_VERSION
            ),
            &refs,
        ));
    }
    let project_id = model.pointer("/project/id").and_then(Value::as_str);
    if project_id != Some(target_ref) {
        return Err(refusal(
            "OP-TARGET-NOT-FOUND",
            format!("Project `{target_ref}` is not this document's project; model settings target the document's project id."),
            &refs,
        ));
    }
    let settings = match model.get("model_settings") {
        None => None,
        Some(Value::Object(settings)) => Some(settings),
        Some(_) => {
            return Err(refusal(
                "OP-MODEL-SETTING-SHAPE-INVALID",
                "`model_settings` is not an object; repair the document first.",
                &refs,
            ));
        }
    };
    let current = settings.and_then(|settings| settings.get(member));
    let current_text = current.map(canonical_json).unwrap_or_else(|| "not_present".into());
    if before != current_text {
        return Err(refusal(
            "OP-STALE-BEFORE-VALUE",
            format!("Intent before-value `{before}` no longer matches the current `{setting}` (`{current_text}`)."),
            &refs,
        ));
    }

    let mut candidate = model.clone();
    let root = candidate
        .as_object_mut()
        .ok_or_else(|| refusal("OP-MODEL-SETTING-DOCUMENT-INVALID", "The model must be an object.", &refs))?;
    let (after_text, row_unit, row_dimension) = if change_kind == SET && member == "shear_model" {
        if unit != "none" || dimension != "dimensionless" {
            return Err(refusal(
                "OP-MODEL-SETTING-SHAPE-INVALID",
                format!("`{setting}` is a name: unit none and dimension dimensionless."),
                &refs,
            ));
        }
        let next = Value::String(after.to_string());
        let has_contract = model.get("pressure_contract").is_some_and(|contract| !contract.is_null());
        if let Some(error) = ms::shear_model_refusal(ms::ShearModelSetting::of(&next), has_contract) {
            return Err(refusal(error.code, error.message, &refs));
        }
        let next_text = canonical_json(&next);
        if current.map(canonical_json).as_deref() == Some(next_text.as_str()) {
            return Err(refusal(
                "OP-MODEL-SETTING-UNCHANGED",
                format!("`{setting}` is already `{after}`; nothing to apply."),
                &refs,
            ));
        }
        root.entry("model_settings")
            .or_insert_with(|| Value::Object(Map::new()))
            .as_object_mut()
            .expect("checked object")
            .insert(member.into(), next);
        (next_text, "none".to_string(), "dimensionless".to_string())
    } else if change_kind == SET {
        if dimension != "length" {
            return Err(refusal(
                "OP-UNIT-DIMENSION-MISMATCH",
                format!("`{setting}` carries dimension `length`; intent dimension `{dimension}` is not accepted."),
                &refs,
            ));
        }
        let Ok(value) = after.trim().parse::<f64>() else {
            return Err(refusal(
                "OP-VALUE-NOT-NUMERIC",
                format!("Replacement value `{after}` for `{setting}` is not a number."),
                &refs,
            ));
        };
        // The setting reader's own refusals, codes and messages (finite, then
        // positive, then a catalog length unit, then finite and positive in metres).
        let quantity = open_pipe_stress_product_physics::Quantity { value, unit: unit.to_string() };
        if let Err(error) = ms::node_proximity_tolerance_m(&quantity) {
            return Err(refusal(error.code, error.message, &refs));
        }
        let mut record = Map::new();
        record.insert("value".into(), Value::from(value));
        record.insert("unit".into(), Value::String(unit.to_string()));
        let next = Value::Object(record);
        let next_text = canonical_json(&next);
        if current.map(canonical_json).as_deref() == Some(next_text.as_str()) {
            return Err(refusal(
                "OP-MODEL-SETTING-UNCHANGED",
                format!("`{setting}` is already {value} {unit}; nothing to apply."),
                &refs,
            ));
        }
        root.entry("model_settings")
            .or_insert_with(|| Value::Object(Map::new()))
            .as_object_mut()
            .expect("checked object")
            .insert("node_proximity_tolerance".into(), next);
        (next_text, unit.to_string(), "length".to_string())
    } else {
        if unit != "none" || dimension != "dimensionless" {
            return Err(refusal(
                "OP-MODEL-SETTING-SHAPE-INVALID",
                format!("{CLEAR} carries unit none and dimension dimensionless."),
                &refs,
            ));
        }
        if after != "not_present" {
            return Err(refusal(
                "OP-MODEL-SETTING-SHAPE-INVALID",
                format!("{CLEAR} uses after-value `not_present`; the default then applies."),
                &refs,
            ));
        }
        if current.is_none() {
            let default = if member == "shear_model" { "Euler–Bernoulli" } else { "the 5 mm default" };
            return Err(refusal(
                "OP-MODEL-SETTING-UNCHANGED",
                format!("`{setting}` is not set ({default} applies); nothing to clear."),
                &refs,
            ));
        }
        let settings = root.get_mut("model_settings").and_then(Value::as_object_mut).expect("checked object");
        settings.remove(member);
        // Absence means the default: an emptied record is removed, never left behind.
        if settings.is_empty() {
            root.remove("model_settings");
        }
        ("not_present".to_string(), "none".to_string(), "dimensionless".to_string())
    };
    Ok(GeometryEdit {
        model: candidate,
        diff_preview: vec![DiffPreviewRow {
            entity_ref: target_ref.to_string(),
            object_type: "Project".into(),
            field_path: setting.to_string(),
            before: current_text,
            after: after_text,
            unit: row_unit,
            dimension: row_dimension,
            change_kind: change_kind.to_string(),
        }],
    })
}
