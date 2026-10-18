//! Explicit U5a pressure-model replacement. T3 owns document activation.
use crate::rich_authoring::{RichEdit, RichError};
use serde_json::Value;
pub(crate) fn owns(object_type: &str, path: &str) -> bool {
    object_type == "Component" && path == "objective_connector.pressure_model"
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
    let error = |message: String| RichError {
        code: "OP-JOINT-PRESSURE-PAYLOAD-INVALID",
        message,
    };
    if model["schema_version"] != "0.6.0"
        || model["pressure_contract"]["version"] != "3.0.0"
        || model["pressure_contract"]["mode"] != "exact_pressure_v3"
    {
        return Err(error(
            "Joint pressure authoring requires model 0.6.0 with 3.0.0/exact_pressure_v3".into(),
        ));
    }
    if unit != "none" || dimension != "dimensionless" {
        return Err(error("The structured pressure model uses unit none and dimension dimensionless; its area quantity carries its own unit".into()));
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
    let current_display =
        crate::canonical_json(&component["objective_connector"]["pressure_model"]);
    if before != current_display {
        return Err(RichError {
            code: "OP-BEFORE-VALUE-MISMATCH",
            message: "Pressure-model before-value is stale".into(),
        });
    }
    let after: Value = serde_json::from_str(after).map_err(|e| error(e.to_string()))?;
    component["objective_connector"]["pressure_model"] = after.clone();
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
                vec!["objective_connector".into(), "pressure_model".into()],
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
        for version in ["0.3.0", "0.4.0", "0.5.0"] {
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
        model["schema_version"] = json!("0.6.0");
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
}
