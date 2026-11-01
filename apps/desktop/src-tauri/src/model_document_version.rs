//! Model-document version check (DEL-02-05 CLM-003; DEL-04-08 Design
//! `b7-plan.md`, "Current model format").
//!
//! The in-document `schema_version` is the sole authority for the model-document
//! version. Model document 0.9.0 is the one current version; its
//! `pressure_contract` is optional and its absence makes the document
//! pressure-free. Every other version is refused by name, and nothing is
//! migrated, upgraded or down-migrated:
//! - `retired_schema`: an older version (0.1.0, 0.2.0 or any other version
//!   below 0.9.0 that is not accepted), with a re-author diagnostic;
//! - `newer_than_supported`: a version above 0.9.0;
//! - `unsupported_schema`: a missing or non-semver version.
//!
//! The stored bytes are never rewritten by an open or a save. Contract and
//! section completeness remain separate from this version-only persistence check.

use serde::Serialize;
use serde_json::Value;

/// What a refused retired project or saved model state means for its recorded
/// runs (owner ruling, 2026-10-26: "R-c1 A, 0.8.0 a2"; DEL-02-01 Design
/// `model-document-0.8.0.md` §3.3). The project-store open and save refusal
/// (`refuse_model_document`) and the saved-state refusal (`model_states.rs`
/// `refuse_version`) append it to a `retired_schema` refusal; the browser-preview
/// store appends the same text (`modelSchemaVersion.ts`).
pub const RETIRED_PROJECT_RUNS_NOTE: &str = "Recorded runs keep their own format and existing read and compare behaviour; this change adds no model-document-version gate for recorded runs. Existing integrity checks and retained-precision/source-reader restrictions still apply. Run history is listed for the active project's id, so this project's runs are reached through a current project with the same project id, and run comparison keeps its eligibility rules.";

/// The version statuses that refuse a document for editing.
pub const REFUSED_MODEL_DOCUMENT_STATUSES: [&str; 3] =
    ["retired_schema", "newer_than_supported", "unsupported_schema"];

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ModelDocumentVersionStatus {
    /// `current`, `retired_schema`, `newer_than_supported` or `unsupported_schema`.
    pub status: String,
    pub source_schema_version: String,
    /// The version the document is accepted as: its own when current, otherwise
    /// the one current version (the re-author target).
    pub target_schema_version: String,
    pub detail: String,
}

impl ModelDocumentVersionStatus {
    pub fn is_refused(&self) -> bool {
        REFUSED_MODEL_DOCUMENT_STATUSES.contains(&self.status.as_str())
    }

    /// The text a refusal of this status appends after its detail: the recorded-runs
    /// note for a retired document, nothing otherwise.
    pub fn refusal_suffix(&self) -> &'static str {
        if self.status == "retired_schema" { RETIRED_PROJECT_RUNS_NOTE } else { "" }
    }
}

/// Check a model document's version. Never mutates or rewrites the document. The
/// states and texts are the applier's (`model_document_version`), which the TS and
/// Python checks mirror against `fixtures/model_document/version_states.json`.
pub fn evaluate_model_document(document: &Value) -> ModelDocumentVersionStatus {
    let version = open_pipe_stress_operation_applier::evaluate_model_document_version(document);
    ModelDocumentVersionStatus {
        status: version.status.to_string(),
        source_schema_version: version.source_schema_version,
        target_schema_version: version.target_schema_version,
        detail: version.detail,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use open_pipe_stress_product_physics::{MODEL_SCHEMA_VERSION, MODEL_SCHEMA_VERSION_RETIRED};
    use serde_json::json;

    fn document(version: &str) -> Value {
        json!({
            "schema_version": version,
            "document_kind": "openpipestress.product_preview.model",
            "project": { "id": "project:test", "name": "Test" },
            "nodes": []
        })
    }

    #[test]
    fn the_current_version_is_current_and_never_rewritten() {
        let doc = document(MODEL_SCHEMA_VERSION);
        let before = doc.clone();
        let evaluated = evaluate_model_document(&doc);
        assert_eq!(evaluated.status, "current");
        assert_eq!(evaluated.source_schema_version, "0.9.0");
        assert_eq!(evaluated.target_schema_version, "0.9.0");
        assert!(!evaluated.is_refused());
        assert_eq!(doc, before);
    }

    #[test]
    fn retired_versions_are_refused_by_name_with_a_re_author_diagnostic() {
        for version in ["0.1.0", "0.2.0", "0.3.0", "0.4.0", "0.5.0", "0.6.0", "0.7.0", "0.8.0"] {
            let mut doc = document(version);
            // A carried contract (here the current one) changes nothing: the version refuses.
            doc["pressure_contract"] = json!({"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"});
            let evaluated = evaluate_model_document(&doc);
            assert_eq!(evaluated.status, "retired_schema", "{version}");
            assert!(evaluated.is_refused());
            assert_eq!(evaluated.source_schema_version, version);
            assert_eq!(evaluated.target_schema_version, MODEL_SCHEMA_VERSION);
            assert!(evaluated.detail.starts_with(MODEL_SCHEMA_VERSION_RETIRED), "{}", evaluated.detail);
            assert!(evaluated.detail.contains(version) && evaluated.detail.contains("re-author"), "{}", evaluated.detail);
            assert!(!evaluated.detail.contains("migrated in memory"));
        }
        // Any other older version is refused the same way, never migrated.
        for version in ["0.0.9", "0.3.1", "0.4.9", "0.5.1", "0.6.1", "0.7.1", "0.8.1"] {
            let evaluated = evaluate_model_document(&document(version));
            assert_eq!(evaluated.status, "retired_schema", "{version}");
            assert!(evaluated.detail.contains("re-author"), "{version}");
        }
    }

    #[test]
    fn incomplete_current_contract_bytes_remain_saveable_and_unchanged() {
        for contract in [Value::Null, json!({}), json!({"version": "3.0.0", "mode": "exact_pressure_v3"})] {
            let mut doc = document(MODEL_SCHEMA_VERSION);
            doc["pressure_contract"] = contract;
            let before = doc.clone();
            assert_eq!(evaluate_model_document(&doc).status, "current");
            assert_eq!(doc, before);
        }
    }

    #[test]
    fn newer_versions_are_refused_without_down_migration() {
        for version in ["0.9.1", "0.10.0", "1.0.0"] {
            let evaluated = evaluate_model_document(&document(version));
            assert_eq!(evaluated.status, "newer_than_supported", "{version}");
            assert!(evaluated.is_refused());
            assert!(evaluated.detail.contains("no down-migration"));
        }
    }

    #[test]
    fn invalid_or_missing_versions_are_unsupported_not_coerced() {
        let mut missing = document("x");
        missing.as_object_mut().unwrap().remove("schema_version");
        for doc in [document("not-semver"), document("1.2"), document(" 0.9.0"), missing] {
            let evaluated = evaluate_model_document(&doc);
            assert_eq!(evaluated.status, "unsupported_schema");
            assert!(evaluated.is_refused());
        }
    }

    /// The desktop gives every state in the shared table (the applier's, the TS and Python checks').
    #[test]
    fn every_version_state_matches_the_shared_table() {
        let table: Value = serde_json::from_str(include_str!("../../../../fixtures/model_document/version_states.json")).unwrap();
        for case in table["cases"].as_array().unwrap() {
            let mut doc = document(MODEL_SCHEMA_VERSION);
            match case.get("schema_version") {
                Some(version) => doc["schema_version"] = version.clone(),
                None => {
                    doc.as_object_mut().unwrap().remove("schema_version");
                }
            }
            let evaluated = evaluate_model_document(&doc);
            let label = case["label"].as_str().unwrap();
            assert_eq!(evaluated.status, case["status"].as_str().unwrap(), "{label}");
            assert_eq!(evaluated.source_schema_version, case["source_schema_version"].as_str().unwrap(), "{label}");
            assert_eq!(evaluated.target_schema_version, case["target_schema_version"].as_str().unwrap(), "{label}");
            assert_eq!(evaluated.detail, case["detail"].as_str().unwrap(), "{label}");
            assert_eq!(evaluated.is_refused(), case["code"].is_string(), "{label}");
        }
    }
}
