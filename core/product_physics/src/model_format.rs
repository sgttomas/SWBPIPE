//! Model document format (DEL-04-08 Design `b7-plan.md`, "Current model format").
//!
//! Model document 0.7.0 is the one current version. Its behaviour keys on what
//! the document carries, never on the version string: a 0.7.0 document without
//! a `pressure_contract` is pressure-free and solves on the preview route. The
//! contract-present form of 0.7.0 declares the exact pressure v3 contract.
//! Model documents 0.1.0, 0.2.0, 0.3.0, 0.4.0, 0.5.0 and 0.6.0 are retired:
//! they are refused by name with a re-author diagnostic, and nothing migrates.

use crate::PreviewModel;

/// The current model document version.
pub const MODEL_SCHEMA_VERSION: &str = "0.7.0";
/// Retired model document versions, refused by name (re-author, no migration).
pub const RETIRED_MODEL_SCHEMA_VERSIONS: [&str; 6] = ["0.1.0", "0.2.0", "0.3.0", "0.4.0", "0.5.0", "0.6.0"];
/// The diagnostic code of a retired model document version.
pub const MODEL_SCHEMA_VERSION_RETIRED: &str = "MODEL_SCHEMA_VERSION_RETIRED";

/// Whether `version` names a retired model document version.
pub fn is_retired_model_schema_version(version: &str) -> bool {
    RETIRED_MODEL_SCHEMA_VERSIONS.contains(&version)
}

/// The re-author diagnostic for a retired model document version.
pub fn retired_model_schema_message(version: &str) -> String {
    format!(
        "model document {version} is retired; re-author it as model document {MODEL_SCHEMA_VERSION} (a pressure-free model omits pressure_contract). Nothing is migrated."
    )
}

/// Whether `model` is pressure-free: a current-version document that carries no
/// pressure contract. The presence of the contract decides the route; the
/// version only says the document is in the current format.
pub(crate) fn is_pressure_free(model: &PreviewModel) -> bool {
    model.schema_version == MODEL_SCHEMA_VERSION && model.pressure_contract.is_none()
}

/// Authored raw group presence, including null. This transports the same group
/// selector to authoring; it grants neither model validity nor numerical standing.
pub fn carries_load_reference_state(model: &serde_json::Value) -> bool {
    model.as_object().is_some_and(|object| object.contains_key("reference_configurations"))
        || model.get("materials").and_then(serde_json::Value::as_array)
            .is_some_and(|materials| materials.iter().any(|material|
                material.as_object().is_some_and(|object| object.contains_key("expansion_laws"))))
        || model.get("load_cases").and_then(serde_json::Value::as_array)
            .is_some_and(|cases| cases.iter().any(|case|
                case.as_object().is_some_and(|object| object.contains_key("analysis_state"))))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn model(version: &str, contract: Option<serde_json::Value>) -> PreviewModel {
        let mut raw = json!({
            "schema_version": version,
            "document_kind": "openpipestress.product_preview.model",
            "project": {"id": "project:format"},
            "analysis_status": {"mechanics": "MODEL_INCOMPLETE", "rule_check": "RULE_INPUTS_INCOMPLETE", "professional_acceptance": "HUMAN_REVIEW_REQUIRED"},
            "nodes": [], "pipe_segments": [], "supports": []
        });
        if let Some(contract) = contract {
            raw["pressure_contract"] = contract;
        }
        serde_json::from_value(raw).unwrap()
    }

    /// Pressure-freedom keys on the contract's absence on the current version. Keying it on
    /// the version string alone (a 0.7.0 document with a contract would count) or on the
    /// retired versions fails here.
    #[test]
    fn pressure_free_keys_on_the_contracts_absence_not_the_version() {
        assert!(is_pressure_free(&model(MODEL_SCHEMA_VERSION, None)));
        let exact = json!({"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"});
        assert!(!is_pressure_free(&model(MODEL_SCHEMA_VERSION, Some(exact.clone()))));
        for version in RETIRED_MODEL_SCHEMA_VERSIONS {
            assert!(is_retired_model_schema_version(version));
            assert!(!is_pressure_free(&model(version, None)), "{version} is retired");
        }
        for version in ["0.5.1", "0.6.1", "0.7.1", "1.0.0"] {
            assert!(!is_retired_model_schema_version(version), "{version}");
        }
        assert!(!is_pressure_free(&model("0.3.0", Some(exact))));
        let message = retired_model_schema_message("0.2.0");
        assert!(message.contains("0.2.0") && message.contains("re-author") && message.contains(MODEL_SCHEMA_VERSION), "{message}");
    }
}
