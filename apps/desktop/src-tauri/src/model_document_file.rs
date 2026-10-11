//! Opening a model document file from disk (DEL-07-11 CLM-001, the desktop
//! walking skeleton's ordinary route).
//!
//! The native side reads the file the user chose, parses it and applies the
//! file-level checks: readable, bounded size, one JSON object, the product's
//! model document kind, and a `schema_version` that the existing model-document
//! evaluation does not refuse. The webview then checks the desktop document
//! shape (`src/features/model-file/modelDocumentShape.ts`) before the session
//! adopts the document.
//!
//! The document is returned exactly as read. No migration, defaulting or
//! normalization is applied here, so the session's model hash is the hash of
//! the file's document. Every refusal is named.

use crate::model_document_migration::{evaluate_model_document, model_document_migrations};
use serde::Serialize;
use serde_json::Value;
use std::{fs, io::Read, path::Path};

/// The product model document kind the desktop opens and writes.
pub const MODEL_DOCUMENT_KIND: &str = "openpipestress.product_preview.model";
/// Upper bound on a model document file. Larger files are refused before parsing.
pub const MAX_MODEL_DOCUMENT_BYTES: u64 = 64 * 1024 * 1024;
/// `evaluate_model_document` statuses that project open also refuses.
const REFUSED_SCHEMA_STATUSES: [&str; 3] = ["newer_than_supported", "unsupported_schema", "failed"];

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ModelFileDiagnostic {
    pub code: &'static str,
    /// JSON path of the offending member, or "" for the whole file.
    pub path: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum ModelDocumentOpenOutcome {
    /// The user dismissed the file chooser.
    Cancelled,
    Opened {
        file_name: String,
        byte_count: u64,
        schema_status: String,
        document: Value,
    },
    Refused {
        file_name: String,
        diagnostics: Vec<ModelFileDiagnostic>,
    },
}

fn refused(file_name: String, code: &'static str, path: &str, message: String) -> ModelDocumentOpenOutcome {
    ModelDocumentOpenOutcome::Refused {
        file_name,
        diagnostics: vec![ModelFileDiagnostic { code, path: path.to_string(), message }],
    }
}

/// Read, parse and check one model document file.
pub fn read_model_document_file(path: &Path) -> ModelDocumentOpenOutcome {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default();
    let mut file = match fs::File::open(path) {
        Ok(file) => file,
        Err(error) => {
            return refused(file_name, "MODEL-FILE-UNREADABLE", "", format!("The file could not be opened: {error}."));
        }
    };
    let declared = file.metadata().map(|meta| meta.len()).unwrap_or(0);
    if declared > MAX_MODEL_DOCUMENT_BYTES {
        return refused(
            file_name,
            "MODEL-FILE-TOO-LARGE",
            "",
            format!("The file is {declared} bytes; a model document may be at most {MAX_MODEL_DOCUMENT_BYTES} bytes."),
        );
    }
    let mut bytes = Vec::new();
    if let Err(error) = (&mut file).take(MAX_MODEL_DOCUMENT_BYTES + 1).read_to_end(&mut bytes) {
        return refused(file_name, "MODEL-FILE-UNREADABLE", "", format!("The file could not be read: {error}."));
    }
    if bytes.len() as u64 > MAX_MODEL_DOCUMENT_BYTES {
        return refused(
            file_name,
            "MODEL-FILE-TOO-LARGE",
            "",
            format!("The file exceeds {MAX_MODEL_DOCUMENT_BYTES} bytes; a model document may be at most that size."),
        );
    }
    read_model_document_bytes(file_name, &bytes)
}

/// The checks of `read_model_document_file` over bytes already read.
pub fn read_model_document_bytes(file_name: String, bytes: &[u8]) -> ModelDocumentOpenOutcome {
    let byte_count = bytes.len() as u64;
    let document: Value = match serde_json::from_slice(bytes) {
        Ok(value) => value,
        Err(error) => {
            return refused(file_name, "MODEL-FILE-NOT-JSON", "", format!("The file is not valid UTF-8 JSON: {error}."));
        }
    };
    let Some(object) = document.as_object() else {
        return refused(file_name, "MODEL-FILE-NOT-A-DOCUMENT", "", "The file's JSON is not an object.".to_string());
    };
    match object.get("document_kind").and_then(Value::as_str) {
        Some(MODEL_DOCUMENT_KIND) => {}
        Some(other) => {
            return refused(
                file_name,
                "MODEL-FILE-DOCUMENT-KIND",
                "document_kind",
                format!("document_kind is `{other}`; the desktop opens `{MODEL_DOCUMENT_KIND}` model documents only."),
            );
        }
        None => {
            return refused(
                file_name,
                "MODEL-FILE-DOCUMENT-KIND",
                "document_kind",
                format!("document_kind is missing; the desktop opens `{MODEL_DOCUMENT_KIND}` model documents only."),
            );
        }
    }
    let evaluated = evaluate_model_document(&document, &model_document_migrations());
    if REFUSED_SCHEMA_STATUSES.contains(&evaluated.status.status.as_str()) {
        return refused(
            file_name,
            "MODEL-FILE-SCHEMA-REFUSED",
            "schema_version",
            format!(
                "schema_version `{}` refused ({}): {}",
                evaluated.status.source_schema_version, evaluated.status.status, evaluated.status.detail
            ),
        );
    }
    ModelDocumentOpenOutcome::Opened {
        file_name,
        byte_count,
        schema_status: evaluated.status.status,
        document,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    const DEMO: &str = include_str!("../../../../fixtures/product_preview/invented_demo_model.json");

    fn code(outcome: &ModelDocumentOpenOutcome) -> &'static str {
        match outcome {
            ModelDocumentOpenOutcome::Refused { diagnostics, .. } => diagnostics[0].code,
            _ => "opened",
        }
    }

    #[test]
    fn opens_the_bundled_demo_unchanged() {
        let outcome = read_model_document_bytes("demo.json".into(), DEMO.as_bytes());
        let ModelDocumentOpenOutcome::Opened { document, byte_count, file_name, .. } = outcome else {
            panic!("demo refused: {outcome:?}");
        };
        assert_eq!(document, serde_json::from_str::<Value>(DEMO).unwrap());
        assert_eq!(byte_count, DEMO.len() as u64);
        assert_eq!(file_name, "demo.json");
    }

    #[test]
    fn refusals_are_named() {
        let named = |bytes: &[u8]| code(&read_model_document_bytes("m.json".into(), bytes));
        assert_eq!(named(b"{not json"), "MODEL-FILE-NOT-JSON");
        assert_eq!(named(&[0xff, 0xfe]), "MODEL-FILE-NOT-JSON");
        assert_eq!(named(b"[1,2]"), "MODEL-FILE-NOT-A-DOCUMENT");
        assert_eq!(named(br#"{"schema_version":"0.2.0"}"#), "MODEL-FILE-DOCUMENT-KIND");
        let request = json!({"model": serde_json::from_str::<Value>(DEMO).unwrap(), "materials": []});
        assert_eq!(named(request.to_string().as_bytes()), "MODEL-FILE-DOCUMENT-KIND");
        for version in ["9.0.0", "0.0.9", "not-semver"] {
            let mut document: Value = serde_json::from_str(DEMO).unwrap();
            document["schema_version"] = json!(version);
            assert_eq!(named(document.to_string().as_bytes()), "MODEL-FILE-SCHEMA-REFUSED", "{version}");
        }
        let mut document: Value = serde_json::from_str(DEMO).unwrap();
        document.as_object_mut().unwrap().remove("schema_version");
        assert_eq!(named(document.to_string().as_bytes()), "MODEL-FILE-SCHEMA-REFUSED");
    }

    #[test]
    fn file_level_refusals_are_named() {
        let dir = std::env::temp_dir().join(format!("swbpipe-model-file-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        assert_eq!(code(&read_model_document_file(&dir.join("absent.json"))), "MODEL-FILE-UNREADABLE");
        let large = dir.join("large.json");
        let file = fs::File::create(&large).unwrap();
        file.set_len(MAX_MODEL_DOCUMENT_BYTES + 1).unwrap();
        assert_eq!(code(&read_model_document_file(&large)), "MODEL-FILE-TOO-LARGE");
        let demo = dir.join("demo.json");
        fs::write(&demo, DEMO).unwrap();
        assert_eq!(code(&read_model_document_file(&demo)), "opened");
        fs::remove_dir_all(&dir).unwrap();
    }
}
