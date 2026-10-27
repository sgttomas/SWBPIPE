//! The model-document version check (DEL-02-05 CLM-003; DEL-02-01 Design
//! `model-document-0.8.0.md` §3.1): one current version, every other refused by
//! name, nothing migrated. The desktop's native check (src-tauri
//! `model_document_version.rs`) delegates here, the TS evaluator
//! (`projectService.ts` `evaluateModelDocumentLocal`) and Python persistence
//! (`core/project_persistence/service.py`) mirror it, and all three are checked
//! against `fixtures/model_document/version_states.json`.
//!
//! States, each with the code that names its refusal:
//! - `current`: the one current version (no refusal);
//! - `retired_schema` (`MODEL_SCHEMA_VERSION_RETIRED`): any other semver version
//!   not above the current one (one written with leading zeros included), with the
//!   re-author text (the named one for a listed retired version);
//! - `newer_than_supported` (`MODEL_SCHEMA_VERSION_NEWER_THAN_SUPPORTED`): a semver
//!   version above the current one;
//! - `unsupported_schema` (`MODEL_SCHEMA_VERSION_UNSUPPORTED`): a missing,
//!   non-string or non-semver version.
//!
//! Version parts compare as digit strings (leading zeros dropped, then by length,
//! then digit by digit), never as bounded integers, so every implementation gives
//! a part of any length the same state.

use open_pipe_stress_product_physics::{
    is_retired_model_schema_version, retired_model_schema_message, MODEL_SCHEMA_VERSION,
    MODEL_SCHEMA_VERSION_RETIRED,
};
use serde_json::Value;

/// `newer_than_supported`'s refusal code.
pub const MODEL_SCHEMA_VERSION_NEWER_THAN_SUPPORTED: &str = "MODEL_SCHEMA_VERSION_NEWER_THAN_SUPPORTED";
/// `unsupported_schema`'s refusal code.
pub const MODEL_SCHEMA_VERSION_UNSUPPORTED: &str = "MODEL_SCHEMA_VERSION_UNSUPPORTED";

/// A document's version state. Never mutates or rewrites the document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelDocumentVersion {
    /// `current`, `retired_schema`, `newer_than_supported` or `unsupported_schema`.
    pub status: &'static str,
    /// The document's version, or `missing_schema_version` when it has none.
    pub source_schema_version: String,
    /// The version the document is accepted as: its own when current, otherwise
    /// the one current version (the re-author target).
    pub target_schema_version: String,
    /// The state's text; a `retired_schema` detail begins with its code.
    pub detail: String,
}

impl ModelDocumentVersion {
    /// True when the document is the current version.
    pub fn is_current(&self) -> bool {
        self.status == "current"
    }

    /// The refusal code of a non-current state; `None` when current.
    pub fn refusal_code(&self) -> Option<&'static str> {
        match self.status {
            "retired_schema" => Some(MODEL_SCHEMA_VERSION_RETIRED),
            "newer_than_supported" => Some(MODEL_SCHEMA_VERSION_NEWER_THAN_SUPPORTED),
            "unsupported_schema" => Some(MODEL_SCHEMA_VERSION_UNSUPPORTED),
            _ => None,
        }
    }

    /// The refusal text without the code prefix a `retired_schema` detail carries.
    pub fn refusal_message(&self) -> &str {
        let prefix = format!("{MODEL_SCHEMA_VERSION_RETIRED}: ");
        self.detail.strip_prefix(prefix.as_str()).unwrap_or(&self.detail)
    }
}

/// `major.minor.patch`, each part one or more ASCII digits (as the TS and Python checks
/// read it: no sign, space or other character), with its leading zeros dropped. The
/// parts stay digit strings, so a part of any length has a defined state.
fn parse_semver(raw: &str) -> Option<[&str; 3]> {
    fn part(text: Option<&str>) -> Option<&str> {
        let text = text?;
        if text.is_empty() || !text.bytes().all(|byte| byte.is_ascii_digit()) {
            return None;
        }
        let trimmed = text.trim_start_matches('0');
        Some(if trimmed.is_empty() { "0" } else { trimmed })
    }
    let mut parts = raw.split('.');
    let major = part(parts.next())?;
    let minor = part(parts.next())?;
    let patch = part(parts.next())?;
    if parts.next().is_some() {
        return None;
    }
    Some([major, minor, patch])
}

/// Compares two parsed versions part by part as numbers: a longer part (no leading
/// zeros) is larger, and parts of one length compare digit by digit. The TS and
/// Python checks compare the same way.
fn compare_semver(left: &[&str; 3], right: &[&str; 3]) -> std::cmp::Ordering {
    left.iter()
        .zip(right.iter())
        .map(|(a, b)| a.len().cmp(&b.len()).then_with(|| a.cmp(b)))
        .find(|ordering| ordering.is_ne())
        .unwrap_or(std::cmp::Ordering::Equal)
}

fn state(status: &'static str, source: &str, target: &str, detail: String) -> ModelDocumentVersion {
    ModelDocumentVersion {
        status,
        source_schema_version: source.to_string(),
        target_schema_version: target.to_string(),
        detail,
    }
}

/// The version state of a model document.
pub fn evaluate_model_document_version(document: &Value) -> ModelDocumentVersion {
    let raw = document.get("schema_version").and_then(Value::as_str).unwrap_or("");
    if raw == MODEL_SCHEMA_VERSION {
        return state(
            "current",
            raw,
            raw,
            format!("Model document {MODEL_SCHEMA_VERSION} is the current version; a pressure contract, when present, and solve eligibility are validated separately."),
        );
    }
    let Some(version) = parse_semver(raw) else {
        return state(
            "unsupported_schema",
            if raw.is_empty() { "missing_schema_version" } else { raw },
            MODEL_SCHEMA_VERSION,
            "Model document has no valid semver schema_version; refusing to open it for editing without coercion.".to_string(),
        );
    };
    let current = parse_semver(MODEL_SCHEMA_VERSION).expect("the current version is semver");
    if compare_semver(&version, &current).is_gt() {
        return state(
            "newer_than_supported",
            raw,
            MODEL_SCHEMA_VERSION,
            format!("Model document schema_version {raw} is newer than the current {MODEL_SCHEMA_VERSION}; refusing to open it for editing (no down-migration)."),
        );
    }
    let detail = if is_retired_model_schema_version(raw) {
        retired_model_schema_message(raw)
    } else {
        format!("model document {raw} is not a supported version; re-author it as model document {MODEL_SCHEMA_VERSION}. Nothing is migrated.")
    };
    state("retired_schema", raw, MODEL_SCHEMA_VERSION, format!("{MODEL_SCHEMA_VERSION_RETIRED}: {detail}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// The shared state table (TS and Python check the same file).
    #[test]
    fn every_version_state_matches_the_shared_table() {
        let table: Value = serde_json::from_str(include_str!("../../../../fixtures/model_document/version_states.json")).unwrap();
        let cases = table["cases"].as_array().unwrap();
        assert!(cases.len() >= 10);
        for case in cases {
            let mut document = json!({"document_kind": "openpipestress.product_preview.model"});
            if let Some(version) = case.get("schema_version") {
                document["schema_version"] = version.clone();
            }
            let got = evaluate_model_document_version(&document);
            let label = case["label"].as_str().unwrap();
            assert_eq!(got.status, case["status"], "{label}");
            assert_eq!(got.source_schema_version, case["source_schema_version"], "{label}");
            assert_eq!(got.target_schema_version, case["target_schema_version"], "{label}");
            assert_eq!(got.detail, case["detail"], "{label}");
            assert_eq!(got.refusal_code(), case["code"].as_str(), "{label}");
        }
    }
}
