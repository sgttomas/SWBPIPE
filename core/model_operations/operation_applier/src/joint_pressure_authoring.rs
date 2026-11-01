//! Explicit U5a pressure-model replacement and (T4-U5b, DEL-04-07 Design
//! `u4b-u5b-implementation.md` §5.2) hardware replacement on current 0.9.0
//! exact-contract documents. Both are checked by the solver's own connector
//! classifier (`validate_joint_pressure_authoring`): the pressure model by
//! its U5a rules, the hardware by §3.1 (`untied` or `explicit_axial_rods_v1`
//! with valid `bilateral_linear` rods, offsets in the connector axes).
use crate::rich_authoring::{RichEdit, RichError};
use serde_json::Value;
const PRESSURE_PATH: &str = "objective_connector.pressure_model";
const HARDWARE_PATH: &str = "objective_connector.hardware";
pub(crate) fn owns(object_type: &str, path: &str) -> bool {
    object_type == "Component" && matches!(path, PRESSURE_PATH | HARDWARE_PATH)
}
#[allow(clippy::too_many_arguments)]
pub(crate) fn resolve(
    model: &Value,
    object_type: &str,
    target: &str,
    path: &str,
    before: &str,
    after: &str,
    unit: &str,
    dimension: &str,
) -> Result<Option<RichEdit>, RichError> {
    if !owns(object_type, path) {
        return Ok(None);
    }
    let hardware = path == HARDWARE_PATH;
    let field = if hardware { "hardware" } else { "pressure_model" };
    let error = |message: String| RichError {
        code: if hardware { "OP-JOINT-HARDWARE-PAYLOAD-INVALID" } else { "OP-JOINT-PRESSURE-PAYLOAD-INVALID" },
        message,
    };
    if model["schema_version"] != open_pipe_stress_product_physics::MODEL_SCHEMA_VERSION
        || model["pressure_contract"]["version"] != "3.0.0"
        || model["pressure_contract"]["mode"] != "exact_pressure_v3"
    {
        return Err(error(format!(
            "Joint {} authoring requires model {} with 3.0.0/exact_pressure_v3",
            if hardware { "hardware" } else { "pressure" },
            open_pipe_stress_product_physics::MODEL_SCHEMA_VERSION
        )));
    }
    if unit != "none" || dimension != "dimensionless" {
        return Err(error(if hardware {
            "The structured hardware uses unit none and dimension dimensionless; each rod quantity carries its own unit".into()
        } else {
            "The structured pressure model uses unit none and dimension dimensionless; its area quantity carries its own unit".into()
        }));
    }
    let mut candidate = model.clone();
    let components = candidate["components"]
        .as_array_mut()
        .ok_or_else(|| error("components must be an array".into()))?;
    let index = components
        .iter()
        .position(|component| component["id"] == target)
        .ok_or_else(|| error("Unknown connector".into()))?;
    let component = &mut components[index];
    if component["kind"] != "expansion_joint" || !component["objective_connector"].is_object() {
        return Err(error("Target must be an objective connector".into()));
    }
    let current_display = crate::canonical_json(&component["objective_connector"][field]);
    if before != current_display {
        return Err(RichError {
            code: "OP-BEFORE-VALUE-MISMATCH",
            message: if hardware { "Hardware before-value is stale" } else { "Pressure-model before-value is stale" }.into(),
        });
    }
    let after: Value = serde_json::from_str(after).map_err(|e| error(e.to_string()))?;
    if !after.is_object() {
        return Err(error(format!("The connector {field} must be a closed object")));
    }
    component["objective_connector"][field] = after.clone();
    let findings = open_pipe_stress_product_physics::validate_joint_pressure_authoring(&candidate)
        .map_err(error)?;
    if !findings.is_empty() {
        return Err(error(
            findings
                .iter()
                .map(|finding| format!("{}: {}", finding.code, finding.message))
                .collect::<Vec<_>>()
                .join("; "),
        ));
    }
    Ok(Some(RichEdit {
        writes: if crate::canonical_json(&after) == current_display {
            vec![]
        } else {
            vec![(
                vec!["objective_connector".into(), field.into()],
                Some(after),
            )]
        },
        current_display,
        warnings: vec![],
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn u5a_pressure_edit_keeps_old_versions_closed_and_validates_the_dormant_area_payload() {
        let references: Value = serde_json::from_str(include_str!(
            "../../../../validation/references/t4_i12/u3_reference_cases.json"
        ))
        .unwrap();
        let mut model = references["cases"]["U3-SYS-DEMO-CONNECTOR-001"]["inputs"]
            ["document_v3_0.3.0"]["model"]
            .clone();
        let id = model["components"][0]["id"].as_str().unwrap().to_string();
        let before =
            crate::canonical_json(&model["components"][0]["objective_connector"]["pressure_model"]);
        let payload = json!({"kind":"constant_effective_area_reference_v1","effective_area":{"value":0.004,"unit":"m^2"},"pressure_basis":"internal_differential_zero_external_v1","source_reference":"invented authoring control"});
        for version in ["0.3.0", "0.4.0", "0.5.0", "0.6.0", "0.7.0", "0.8.0"] {
            model["schema_version"] = json!(version);
            assert!(resolve(
                &model,
                "Component",
                &id,
                "objective_connector.pressure_model",
                &before,
                &payload.to_string(),
                "none",
                "dimensionless"
            )
            .is_err());
        }
        // This direct helper checks the authored joint slice; general 0.5
        // document activation and its required fields are B7b's responsibility.
        model["schema_version"] = json!("0.9.0");
        let edit = resolve(
            &model,
            "Component",
            &id,
            "objective_connector.pressure_model",
            &before,
            &payload.to_string(),
            "none",
            "dimensionless",
        )
        .unwrap()
        .unwrap();
        assert_eq!(edit.writes[0].1.as_ref(), Some(&payload));
        let mut invalid = payload.clone();
        invalid["effective_area"]["value"] = json!(-1.);
        assert!(resolve(
            &model,
            "Component",
            &id,
            "objective_connector.pressure_model",
            &before,
            &invalid.to_string(),
            "none",
            "dimensionless"
        )
        .unwrap_err()
        .message
        .contains("JOINT_PRESSURE_AREA_MISSING"));
        assert_eq!(
            model["components"][0]["objective_connector"]["pressure_model"],
            json!({"kind":"unpressurized"})
        );
    }

    fn rod(id: &str, y: f64) -> Value {
        let m = |v: f64| json!({"value": v, "unit": "m"});
        json!({"id": id, "law": "bilateral_linear",
            "offset_i": {"x": m(-0.05), "y": m(y), "z": m(0.0)},
            "offset_j": {"x": m(0.05), "y": m(y), "z": m(0.0)},
            "stiffness": {"value": 1.0e4, "unit": "kN/m"},
            "reference_extension": {"value": -0.025, "unit": "mm"},
            "source_reference": "invented authoring control"})
    }

    /// T4-U5b (§3.1, §5.2): the hardware edit path accepts valid rods on a
    /// 0.9.0 exact document only, and refuses each §3.1 defect by name.
    #[test]
    fn u5b_hardware_edit_validates_the_rods_on_current_exact_documents_only() {
        let references: Value = serde_json::from_str(include_str!(
            "../../../../validation/references/t4_i12/u3_reference_cases.json"
        ))
        .unwrap();
        let mut model = references["cases"]["U3-SYS-DEMO-CONNECTOR-001"]["inputs"]
            ["document_v3_0.3.0"]["model"]
            .clone();
        let id = model["components"][0]["id"].as_str().unwrap().to_string();
        let before = crate::canonical_json(&model["components"][0]["objective_connector"]["hardware"]);
        let payload = json!({"kind": "explicit_axial_rods_v1", "rods": [rod("rod:a", 0.1), rod("rod:b", -0.1)]});
        let edit = |model: &Value, after: &Value| {
            resolve(model, "Component", &id, HARDWARE_PATH, &before, &after.to_string(), "none", "dimensionless")
        };
        for version in ["0.3.0", "0.4.0", "0.5.0", "0.6.0", "0.7.0", "0.8.0"] {
            model["schema_version"] = json!(version);
            assert_eq!(edit(&model, &payload).unwrap_err().code, "OP-JOINT-HARDWARE-PAYLOAD-INVALID");
        }
        model["schema_version"] = json!("0.9.0");
        model["pressure_contract"] = json!({"version":"3.0.0","mode":"exact_pressure_v3","shear_model":"euler_bernoulli"});
        for pipe in model["pipe_segments"].as_array_mut().unwrap() {
            pipe["section"]["corrosion_allowance"] = json!({"value": 0.0, "unit": "m"});
        }
        let written = edit(&model, &payload).unwrap().unwrap();
        assert_eq!(written.writes, vec![(vec!["objective_connector".to_string(), "hardware".to_string()], Some(payload.clone()))]);
        // Stale before-value and units are refused before any write.
        assert_eq!(resolve(&model, "Component", &id, HARDWARE_PATH, "{}", &payload.to_string(), "none", "dimensionless").unwrap_err().code, "OP-BEFORE-VALUE-MISMATCH");
        assert!(resolve(&model, "Component", &id, HARDWARE_PATH, &before, &payload.to_string(), "m", "length").is_err());
        let refused = |after: Value, code: &str| {
            let message = edit(&model, &after).unwrap_err().message;
            assert!(message.contains(code), "{after}: {message}");
        };
        refused(json!({"kind": "explicit_axial_rods_v1", "rods": []}), "JOINT_TIE_ROD_INPUT_INVALID");
        refused(json!({"kind": "untied", "rods": [rod("rod:a", 0.1)]}), "JOINT_TIE_ROD_INPUT_INVALID");
        refused(json!({"kind": "explicit_axial_rods_v1", "rods": [rod("rod:a", 0.1), rod("rod:a", -0.1)]}), "JOINT_TIE_ROD_INPUT_INVALID");
        let mut ideal = rod("rod:a", 0.1);
        ideal["law"] = json!("ideal_inextensible");
        refused(json!({"kind": "explicit_axial_rods_v1", "rods": [ideal]}), "JOINT_HARDWARE_LAW_UNSUPPORTED");
        refused(json!({"kind": "collective_axial_constraint_v1"}), "JOINT_HARDWARE_LAW_UNSUPPORTED");
        let mut wrong_unit = rod("rod:a", 0.1);
        wrong_unit["stiffness"] = json!({"value": 1.0, "unit": "m"});
        refused(json!({"kind": "explicit_axial_rods_v1", "rods": [wrong_unit]}), "JOINT_TIE_ROD_INPUT_INVALID");
        assert!(written.current_display.contains("untied"));
    }
}
