//! Model document format (DEL-05-02 Design `b7-plan.md`, "Current model format").
//!
//! Model document 0.5.0 is the one current version. Its behaviour keys on what
//! the document carries, never on the version string: a 0.5.0 document without
//! a `pressure_contract` is pressure-free and solves on the preview route. The
//! contract-present form of 0.5.0 lands with B7b; until then 0.3.0 and 0.4.0
//! stay accepted for exact pressure models and a 0.5.0 document that declares a
//! contract is refused by name. Model documents 0.1.0 and 0.2.0 are retired:
//! they are refused by name with a re-author diagnostic, and nothing migrates.

use crate::PreviewModel;

/// The current model document version.
pub const MODEL_SCHEMA_VERSION: &str = "0.5.0";
/// Retired model document versions, refused by name (re-author, no migration).
pub const RETIRED_MODEL_SCHEMA_VERSIONS: [&str; 2] = ["0.1.0", "0.2.0"];
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn model(version: &str, contract: serde_json::Value) -> PreviewModel {
        serde_json::from_value(json!({
            "schema_version": version,
            "document_kind": "openpipestress.product_preview.model",
            "project": {"id": "project:format"},
            "analysis_status": {"mechanics": "MODEL_INCOMPLETE", "rule_check": "RULE_INPUTS_INCOMPLETE", "professional_acceptance": "HUMAN_REVIEW_REQUIRED"},
            "nodes": [], "pipe_segments": [], "supports": [],
            "pressure_contract": contract
        }))
        .unwrap()
    }

    /// Pressure-freedom keys on the contract's absence on the current version. Keying it on
    /// the version string alone (a 0.5.0 document with a contract would count) or on the
    /// retired versions fails here.
    #[test]
    fn pressure_free_keys_on_the_contracts_absence_not_the_version() {
        assert!(is_pressure_free(&model(MODEL_SCHEMA_VERSION, json!(null))));
        let exact = json!({"version": "2.0.0", "mode": "exact_straight_pressure_v2"});
        assert!(!is_pressure_free(&model(MODEL_SCHEMA_VERSION, exact.clone())));
        for version in RETIRED_MODEL_SCHEMA_VERSIONS {
            assert!(is_retired_model_schema_version(version));
            assert!(!is_pressure_free(&model(version, json!(null))), "{version} is retired");
        }
        for version in ["0.3.0", "0.4.0", "0.5.1", "1.0.0"] {
            assert!(!is_retired_model_schema_version(version), "{version}");
        }
        assert!(!is_pressure_free(&model("0.3.0", exact)));
        let message = retired_model_schema_message("0.2.0");
        assert!(message.contains("0.2.0") && message.contains("re-author") && message.contains("0.5.0"), "{message}");
    }
}
