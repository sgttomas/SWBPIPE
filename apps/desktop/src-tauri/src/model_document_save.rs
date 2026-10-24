//! File > Save Model Document… (DEL-15-01 Design `canonical-model-export.md`
//! §2, W-6a; FR-HAND-001, FR-MOD-001).
//!
//! The saved bytes are the unchecked RFC 8785 (JCS) text of the session's model
//! document: the serializer the session's model hash uses
//! (`open_pipe_stress_operation_applier::canonical_json`, through wasm
//! `canonicalSha256Hex` in the webview). There is no trailing newline, so the
//! file's SHA-256 is the model hash.
//!
//! The webview sends the canonical text, its claimed model hash and its
//! route evidence. Before any file is chosen this module
//! - re-checks the screening and local-first evidence of the governed route
//!   `DOTH-MODEL-DOC-007` (local-private, explicit intent, lossless);
//! - applies the 64 MiB open cap;
//! - re-parses and re-canonicalizes the text and refuses
//!   `MODEL-DOC-SAVE-HASH-MISMATCH` unless the result equals the text and its
//!   SHA-256 equals the claim;
//! - applies the open path's document checks (`check_model_document`: kind,
//!   current version, the W-13 document validator), so every saved file
//!   re-opens. The webview has applied the desktop shape check
//!   (`modelDocumentShape.ts`) and the disposition screening.
//!
//! The file is written by the extension-neutral atomic writer under the
//! `.json` model-document format. Nothing is sent anywhere (F-PIP-1); the
//! chooser starts in the user's documents folder. The session's model, hash
//! and solve standing are not touched by a save.

use crate::atomic_report_package_save::{self, AtomicSaveError, MODEL_DOCUMENT_FORMAT};
use crate::command_ingress::Args;
use crate::model_document_file::{check_model_document, ModelDocumentOpenOutcome, MAX_MODEL_DOCUMENT_BYTES};
use crate::solve_jobs::SolveJobError;
use open_pipe_stress_operation_applier::{canonical_json, sha256_hex};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::{Path, PathBuf};

/// The governed local-first route of this save.
pub const MODEL_DOCUMENT_SAVE_ROUTE_ID: &str = "DOTH-MODEL-DOC-007";

pub const HASH_MISMATCH: &str = "MODEL-DOC-SAVE-HASH-MISMATCH";
pub const EVIDENCE_INVALID: &str = "MODEL-DOC-SAVE-EVIDENCE-INVALID";
pub const TOO_LARGE: &str = "MODEL-DOC-SAVE-TOO-LARGE";
pub const VERSION_NOT_CURRENT: &str = "MODEL-DOC-SAVE-VERSION-NOT-CURRENT";
pub const PICKER_PATH_INVALID: &str = "MODEL-DOC-SAVE-PICKER-PATH-INVALID";
pub const WORKER_FAILED: &str = "MODEL-DOC-SAVE-WORKER-FAILED";

/// The webview's screening evidence (the `ControlledExportLink` shape of a
/// lossless local-private route).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ModelDocumentSaveScreening {
    pub route_id: String,
    pub export_context: String,
    pub explicit_local_private_intent: bool,
    pub blocked: bool,
    pub materialization_withheld: bool,
    pub lossless_required: bool,
    pub exact_payload_match: bool,
    pub blocking_count: usize,
}

/// The metadata-only local-first route evidence (`enforceLocalFirstRoute`).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ModelDocumentSaveLocalFirst {
    pub route_id: String,
    pub export_context: String,
    pub storage_context: String,
    pub action: String,
    pub reason_code: String,
    pub blocked: bool,
    pub metadata_only: bool,
    pub explicit_local_private_intent: bool,
}

/// `{canonicalText, claimedModelHash, screening, localFirstEvidence}`.
#[derive(Debug)]
pub struct ModelDocumentSaveRequest {
    pub canonical_text: String,
    pub claimed_model_hash: String,
    pub screening: ModelDocumentSaveScreening,
    pub local_first: ModelDocumentSaveLocalFirst,
}

/// Decodes the command's raw-body arguments (strings and structs of scalars, moved).
pub(crate) fn save_model_document_args(args: &mut Args) -> Result<ModelDocumentSaveRequest, SolveJobError> {
    Ok(ModelDocumentSaveRequest {
        canonical_text: args.decode("canonicalText")?,
        claimed_model_hash: args.decode("claimedModelHash")?,
        screening: args.decode("screening")?,
        local_first: args.decode("localFirstEvidence")?,
    })
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ModelDocumentSaveDiagnostic {
    pub code: String,
    /// JSON path of the offending member, or "" for the whole document.
    pub path: String,
    pub message: String,
}

fn diagnostic(code: &str, path: &str, message: impl Into<String>) -> ModelDocumentSaveDiagnostic {
    ModelDocumentSaveDiagnostic { code: code.to_string(), path: path.to_string(), message: message.into() }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum ModelDocumentSaveOutcome {
    /// The user dismissed the chooser. Not an error; nothing was written.
    Cancelled,
    Saved {
        file_name: String,
        byte_count: u64,
        /// `sha256:<hex>` of the bytes written: the session's model hash.
        model_hash: String,
        replaced_existing: bool,
        /// `durable`, or `saved_durability_uncertain` when the directory sync
        /// after the rename did not complete.
        durability: String,
    },
    Refused {
        diagnostics: Vec<ModelDocumentSaveDiagnostic>,
    },
}

impl ModelDocumentSaveOutcome {
    pub fn refused(diagnostics: Vec<ModelDocumentSaveDiagnostic>) -> Self {
        Self::Refused { diagnostics }
    }

    pub fn refused_one(code: &str, message: impl Into<String>) -> Self {
        Self::Refused { diagnostics: vec![diagnostic(code, "", message)] }
    }
}

/// Re-checks the route evidence as the report package save does
/// (`lib.rs` `validate_report_package_local_first_evidence`): defence in
/// depth, not caller authentication.
pub fn validate_model_document_save_evidence(
    screening: &ModelDocumentSaveScreening,
    local_first: &ModelDocumentSaveLocalFirst,
) -> Result<(), ModelDocumentSaveDiagnostic> {
    let s = screening;
    let l = local_first;
    let admitted = s.route_id == MODEL_DOCUMENT_SAVE_ROUTE_ID
        && s.export_context == "local_private"
        && s.explicit_local_private_intent
        && !s.blocked
        && !s.materialization_withheld
        && s.lossless_required
        && s.exact_payload_match
        && s.blocking_count == 0
        && l.route_id == MODEL_DOCUMENT_SAVE_ROUTE_ID
        && l.export_context == "local_private"
        && l.storage_context == "local_private"
        && l.action == "include_metadata_only"
        && l.reason_code == "PRIVATE_LOCAL_METADATA_ALLOWED"
        && !l.blocked
        && l.metadata_only
        && l.explicit_local_private_intent;
    if admitted {
        Ok(())
    } else {
        Err(diagnostic(
            EVIDENCE_INVALID,
            "",
            format!("{MODEL_DOCUMENT_SAVE_ROUTE_ID} local-private, lossless route evidence with explicit intent is missing or blocked."),
        ))
    }
}

/// A model document admitted for writing.
#[derive(Debug)]
pub struct AdmittedModelDocument {
    /// The canonical text, exactly as it will be written.
    pub text: String,
    /// `sha256:<hex>` of `text`.
    pub model_hash: String,
    pub suggested_file_name: String,
}

fn claimed_hex(claimed: &str) -> Option<&str> {
    claimed
        .strip_prefix("sha256:")
        .filter(|hex| hex.len() == 64 && hex.bytes().all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f')))
}

/// The admission of §2 over the webview's canonical text, before any file is chosen.
pub fn admit_model_document_text(
    canonical_text: String,
    claimed_model_hash: &str,
) -> Result<AdmittedModelDocument, Vec<ModelDocumentSaveDiagnostic>> {
    let byte_count = canonical_text.len() as u64;
    if byte_count > MAX_MODEL_DOCUMENT_BYTES {
        return Err(vec![diagnostic(
            TOO_LARGE,
            "",
            format!("The model document is {byte_count} bytes; a model document may be at most {MAX_MODEL_DOCUMENT_BYTES} bytes, so it would not re-open."),
        )]);
    }
    let Some(claimed) = claimed_hex(claimed_model_hash) else {
        return Err(vec![diagnostic(HASH_MISMATCH, "", "The claimed model hash is not `sha256:` and 64 lowercase hex digits.")]);
    };
    let document: Value = match serde_json::from_str(&canonical_text) {
        Ok(document) => document,
        Err(error) => {
            return Err(vec![diagnostic(HASH_MISMATCH, "", format!("The text is not JSON ({error}), so it is not the canonical text of a model document."))]);
        }
    };
    if canonical_json(&document) != canonical_text {
        return Err(vec![diagnostic(HASH_MISMATCH, "", "The text is not its own RFC 8785 (JCS) canonical form.")]);
    }
    let hex = sha256_hex(&canonical_text);
    if hex != claimed {
        return Err(vec![diagnostic(
            HASH_MISMATCH,
            "",
            format!("The text's SHA-256 is sha256:{hex}; the session claimed {claimed_model_hash}."),
        )]);
    }
    let suggested_file_name = suggested_model_document_file_name(&document);
    match check_model_document(String::new(), byte_count, document) {
        ModelDocumentOpenOutcome::Opened { schema_status, .. } if schema_status == "current" => {}
        ModelDocumentOpenOutcome::Opened { schema_status, .. } => {
            return Err(vec![diagnostic(
                VERSION_NOT_CURRENT,
                "schema_version",
                format!("The model document's version status is `{schema_status}`; only the current version is saved."),
            )]);
        }
        ModelDocumentOpenOutcome::Refused { diagnostics, .. } => {
            return Err(diagnostics
                .into_iter()
                .map(|item| ModelDocumentSaveDiagnostic { code: item.code.to_string(), path: item.path, message: item.message })
                .collect());
        }
        ModelDocumentOpenOutcome::Cancelled => unreachable!("the document checks never cancel"),
    }
    Ok(AdmittedModelDocument { text: canonical_text, model_hash: format!("sha256:{hex}"), suggested_file_name })
}

/// `<project id without "project:">.json`, restricted to `[A-Za-z0-9._-]`;
/// `model-document.json` when nothing usable remains.
pub fn suggested_model_document_file_name(document: &Value) -> String {
    let id = document.pointer("/project/id").and_then(Value::as_str).unwrap_or("");
    let id = id.strip_prefix("project:").unwrap_or(id);
    let mut stem: String = id
        .chars()
        .map(|character| if character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-') { character } else { '-' })
        .take(96)
        .collect();
    stem = stem.trim_matches(|character| matches!(character, '.' | '-' | '_')).to_string();
    if stem.is_empty() {
        stem = "model-document".to_string();
    }
    format!("{stem}.json")
}

/// Where the chooser starts: the user's documents folder, else their home
/// folder. Never the process's working directory or the repository.
pub fn default_save_directory(documents: Option<PathBuf>, home: Option<PathBuf>) -> Option<PathBuf> {
    documents.filter(|path| path.is_absolute()).or_else(|| home.filter(|path| path.is_absolute()))
}

fn writer_diagnostic(error: AtomicSaveError) -> ModelDocumentSaveDiagnostic {
    let mut message = format!("{} ({} stage).", error.message, error.stage);
    if let (Some(code), Some(detail)) = (error.cleanup_code, error.cleanup_message) {
        message.push_str(&format!(" {code}: {detail}."));
    }
    diagnostic(&error.code, "", message)
}

/// Writes an admitted document to the chosen destination and returns the
/// SHA-256 of the bytes written.
pub fn write_admitted_model_document(destination: &Path, admitted: AdmittedModelDocument) -> ModelDocumentSaveOutcome {
    let bytes = admitted.text.as_bytes();
    match atomic_report_package_save::atomic_save_bytes_as(&MODEL_DOCUMENT_FORMAT, destination, bytes) {
        Ok(saved) => ModelDocumentSaveOutcome::Saved {
            file_name: saved.selected_basename,
            byte_count: bytes.len() as u64,
            model_hash: format!("sha256:{}", sha256_hex(&admitted.text)),
            replaced_existing: saved.replaced_existing,
            durability: saved.durability,
        },
        Err(error) => ModelDocumentSaveOutcome::refused(vec![writer_diagnostic(error)]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command_ingress::test_support::{admit_json, check_decode_bound, registry};
    use crate::command_ingress::ArgsDecoding;
    use crate::memory_pool::test_host::ScriptedHost;
    use crate::model_document_file::read_model_document_file;
    use serde_json::json;
    use std::fs;

    const DEMO: &str = include_str!("../../../../fixtures/product_preview/invented_demo_model.json");
    const EVERY_SLOT: &str = include_str!("../../../../fixtures/model_document/invented_every_slot_model.json");

    fn canonical_of(text: &str) -> String {
        canonical_json(&serde_json::from_str::<Value>(text).unwrap())
    }

    fn hash_of(text: &str) -> String {
        format!("sha256:{}", sha256_hex(text))
    }

    fn codes(result: Result<AdmittedModelDocument, Vec<ModelDocumentSaveDiagnostic>>) -> Vec<(String, String)> {
        match result {
            Ok(_) => vec![("admitted".to_string(), String::new())],
            Err(diagnostics) => diagnostics.into_iter().map(|item| (item.code, item.path)).collect(),
        }
    }

    fn sandbox(label: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "swbpipe-model-save-{label}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
        ));
        fs::create_dir_all(&path).unwrap();
        path
    }

    fn screening() -> ModelDocumentSaveScreening {
        ModelDocumentSaveScreening {
            route_id: MODEL_DOCUMENT_SAVE_ROUTE_ID.to_string(),
            export_context: "local_private".to_string(),
            explicit_local_private_intent: true,
            blocked: false,
            materialization_withheld: false,
            lossless_required: true,
            exact_payload_match: true,
            blocking_count: 0,
        }
    }

    fn local_first() -> ModelDocumentSaveLocalFirst {
        ModelDocumentSaveLocalFirst {
            route_id: MODEL_DOCUMENT_SAVE_ROUTE_ID.to_string(),
            export_context: "local_private".to_string(),
            storage_context: "local_private".to_string(),
            action: "include_metadata_only".to_string(),
            reason_code: "PRIVATE_LOCAL_METADATA_ALLOWED".to_string(),
            blocked: false,
            metadata_only: true,
            explicit_local_private_intent: true,
        }
    }

    /// Save → open: the file holds exactly the canonical bytes, has no trailing
    /// newline, re-opens through the open path unchanged, and its SHA-256 (the
    /// returned hash) is the model hash of the document it re-opens as.
    #[test]
    fn save_then_open_gives_identical_bytes_and_an_equal_model_hash() {
        for (label, source) in [("demo", DEMO), ("every-slot", EVERY_SLOT)] {
            let text = canonical_of(source);
            let claimed = hash_of(&text);
            let admitted = admit_model_document_text(text.clone(), &claimed).unwrap();
            assert_eq!(admitted.model_hash, claimed);
            let root = sandbox(label);
            let destination = root.join(&admitted.suggested_file_name);
            let outcome = write_admitted_model_document(&destination, admitted);
            let ModelDocumentSaveOutcome::Saved { file_name, byte_count, model_hash, replaced_existing, .. } = outcome else {
                panic!("{label}: not saved: {outcome:?}");
            };
            assert!(!replaced_existing);
            let written = fs::read(&destination).unwrap();
            assert_eq!(written, text.as_bytes(), "{label}");
            assert_ne!(written.last(), Some(&b'\n'));
            assert_eq!(byte_count, written.len() as u64);
            assert_eq!(model_hash, claimed);
            assert_eq!(file_name, destination.file_name().unwrap().to_string_lossy());
            let ModelDocumentOpenOutcome::Opened { document, byte_count: opened_bytes, schema_status, .. } =
                read_model_document_file(&destination)
            else {
                panic!("{label}: the saved file does not re-open");
            };
            assert_eq!(schema_status, "current");
            assert_eq!(opened_bytes, byte_count);
            // The file re-opens as exactly the saved text's document, whose
            // canonical form is the saved text and the source's canonical form
            // (JCS normalizes number text such as `1.0`, so the source's own
            // `Value` may differ in representation, never in canonical form).
            assert_eq!(document, serde_json::from_str::<Value>(&text).unwrap());
            assert_eq!(canonical_json(&document), text, "{label}");
            assert_eq!(canonical_json(&document), canonical_of(source), "{label}");
            assert_eq!(hash_of(&canonical_json(&document)), model_hash, "{label}: the re-opened model hash");
            fs::remove_dir_all(root).unwrap();
        }
    }

    #[test]
    fn a_hash_or_canonical_mismatch_is_refused_by_name() {
        let text = canonical_of(DEMO);
        let claimed = hash_of(&text);
        let mismatch = || vec![(HASH_MISMATCH.to_string(), String::new())];
        // Another hash, a malformed claim, the hash without its prefix.
        assert_eq!(codes(admit_model_document_text(text.clone(), &hash_of("{}"))), mismatch());
        assert_eq!(codes(admit_model_document_text(text.clone(), "sha256:ABC")), mismatch());
        assert_eq!(codes(admit_model_document_text(text.clone(), claimed.trim_start_matches("sha256:"))), mismatch());
        assert_eq!(codes(admit_model_document_text(text.clone(), &claimed.to_uppercase())), mismatch());
        // Non-canonical text of the same document: pretty-printed, a trailing
        // newline, the fixture as authored. Each is refused even with its own hash.
        let pretty = serde_json::to_string_pretty(&serde_json::from_str::<Value>(DEMO).unwrap()).unwrap();
        let newline = format!("{text}\n");
        let as_authored = DEMO.to_string();
        for variant in [pretty, newline, as_authored] {
            assert_ne!(variant, text);
            let own = hash_of(&variant);
            assert_eq!(codes(admit_model_document_text(variant, &own)), mismatch());
        }
        assert_eq!(codes(admit_model_document_text("{not json".to_string(), &hash_of("{not json"))), mismatch());
        assert_eq!(codes(admit_model_document_text(text, &claimed)), vec![("admitted".to_string(), String::new())]);
    }

    fn canonical_variant(edit: impl FnOnce(&mut Value)) -> (String, String) {
        let mut document: Value = serde_json::from_str(EVERY_SLOT).unwrap();
        edit(&mut document);
        let text = canonical_json(&document);
        let hash = hash_of(&text);
        (text, hash)
    }

    /// The open path's document checks, applied before writing: kind, version
    /// (retired, newer, missing) and the W-13 document validator.
    #[test]
    fn version_kind_and_validator_refusals_are_the_open_paths() {
        for (version, code) in [("0.6.0", "MODEL-FILE-SCHEMA-REFUSED"), ("0.8.0", "MODEL-FILE-SCHEMA-REFUSED"), ("x", "MODEL-FILE-SCHEMA-REFUSED")] {
            let (text, hash) = canonical_variant(|document| document["schema_version"] = json!(version));
            assert_eq!(codes(admit_model_document_text(text, &hash)), vec![(code.to_string(), "schema_version".to_string())], "{version}");
        }
        let (text, hash) = canonical_variant(|document| {
            document.as_object_mut().unwrap().remove("schema_version");
        });
        assert_eq!(codes(admit_model_document_text(text, &hash))[0].0, "MODEL-FILE-SCHEMA-REFUSED");
        let (text, hash) = canonical_variant(|document| document["document_kind"] = json!("openpipestress.other"));
        assert_eq!(codes(admit_model_document_text(text, &hash)), vec![("MODEL-FILE-DOCUMENT-KIND".to_string(), "document_kind".to_string())]);
        let (text, hash) = canonical_variant(|document| {
            document["lines"][0]["pipe_refs"] = json!(["pipe:P-absent"]);
            document["unresolved_assumptions"][0]["status"] = json!("resolved");
        });
        assert_eq!(codes(admit_model_document_text(text, &hash)), vec![
            ("MODEL-LINE-PIPE-MISSING".to_string(), "lines[0].pipe_refs[0]".to_string()),
            ("MODEL-ASSUMPTION-RESOLUTION-NOTE-REQUIRED".to_string(), "unresolved_assumptions[0].resolution_note".to_string()),
        ]);
        // A validator warning (a quarantined provenance disposition) does not
        // refuse here: the webview's disposition screening refuses it (§2).
        let (text, hash) = canonical_variant(|document| document["materials"][0]["provenance_record"]["review_status"] = json!("quarantined"));
        assert!(admit_model_document_text(text, &hash).is_ok());
    }

    #[test]
    fn the_open_cap_applies_before_parsing() {
        let limit = MAX_MODEL_DOCUMENT_BYTES as usize;
        // One byte over the cap: refused by size, whatever the text is.
        let over = "x".repeat(limit + 1);
        let claim = hash_of("x");
        assert_eq!(codes(admit_model_document_text(over, &claim)), vec![(TOO_LARGE.to_string(), String::new())]);
        // At the cap the size check passes and the next check (canonical text) speaks.
        let at = "x".repeat(limit);
        assert_eq!(codes(admit_model_document_text(at, &claim)), vec![(HASH_MISMATCH.to_string(), String::new())]);
    }

    #[test]
    fn route_evidence_is_rechecked() {
        assert!(validate_model_document_save_evidence(&screening(), &local_first()).is_ok());
        let mut refusals: Vec<(ModelDocumentSaveScreening, ModelDocumentSaveLocalFirst)> = Vec::new();
        for edit in [
            (|s: &mut ModelDocumentSaveScreening| s.route_id = "DOTH-JSON-001".to_string()) as fn(&mut ModelDocumentSaveScreening),
            |s| s.export_context = "downstream_tool".to_string(),
            |s| s.explicit_local_private_intent = false,
            |s| s.blocked = true,
            |s| s.materialization_withheld = true,
            |s| s.lossless_required = false,
            |s| s.exact_payload_match = false,
            |s| s.blocking_count = 1,
        ] {
            let mut s = screening();
            edit(&mut s);
            refusals.push((s, local_first()));
        }
        for edit in [
            (|l: &mut ModelDocumentSaveLocalFirst| l.route_id = "DREP-PACKAGE-SAVE-009".to_string()) as fn(&mut ModelDocumentSaveLocalFirst),
            |l| l.export_context = "public_report".to_string(),
            |l| l.storage_context = "public_report".to_string(),
            |l| l.action = "block_storage".to_string(),
            |l| l.reason_code = "LOCAL_PRIVATE_INTENT_REQUIRED".to_string(),
            |l| l.blocked = true,
            |l| l.metadata_only = false,
            |l| l.explicit_local_private_intent = false,
        ] {
            let mut l = local_first();
            edit(&mut l);
            refusals.push((screening(), l));
        }
        for (s, l) in refusals {
            assert_eq!(validate_model_document_save_evidence(&s, &l).unwrap_err().code, EVIDENCE_INVALID, "{s:?} {l:?}");
        }
    }

    #[test]
    fn a_write_failure_is_a_named_refusal_and_writes_nothing() {
        let text = canonical_of(DEMO);
        let admitted = admit_model_document_text(text.clone(), &hash_of(&text)).unwrap();
        let root = sandbox("wrong-extension");
        let destination = root.join("invented-model.opsproj");
        let outcome = write_admitted_model_document(&destination, admitted);
        let ModelDocumentSaveOutcome::Refused { diagnostics } = outcome else { panic!("{outcome:?}") };
        assert_eq!(diagnostics[0].code, "MODEL-DOC-SAVE-DESTINATION-EXTENSION");
        assert!(!destination.exists());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn outcomes_serialize_as_the_webview_reads_them() {
        assert_eq!(serde_json::to_value(ModelDocumentSaveOutcome::Cancelled).unwrap(), json!({"outcome": "cancelled"}));
        assert_eq!(
            serde_json::to_value(ModelDocumentSaveOutcome::refused_one(HASH_MISMATCH, "m")).unwrap(),
            json!({"outcome": "refused", "diagnostics": [{"code": HASH_MISMATCH, "path": "", "message": "m"}]})
        );
        let saved = ModelDocumentSaveOutcome::Saved {
            file_name: "a.json".into(), byte_count: 2, model_hash: hash_of("{}"), replaced_existing: false, durability: "durable".into(),
        };
        assert_eq!(
            serde_json::to_value(saved).unwrap(),
            json!({"outcome": "saved", "file_name": "a.json", "byte_count": 2, "model_hash": hash_of("{}"), "replaced_existing": false, "durability": "durable"})
        );
    }

    #[test]
    fn suggested_names_and_the_default_directory() {
        let named = |id: Value| suggested_model_document_file_name(&json!({"project": {"id": id}}));
        assert_eq!(named(json!("project:invented-demo-loop-01")), "invented-demo-loop-01.json");
        assert_eq!(named(json!("project:a/b c:..")), "a-b-c.json");
        assert_eq!(named(json!("project:")), "model-document.json");
        assert_eq!(named(json!(7)), "model-document.json");
        assert_eq!(suggested_model_document_file_name(&json!({})), "model-document.json");
        assert_eq!(named(json!("x".repeat(200))).len(), 96 + ".json".len());
        let documents = PathBuf::from("/Users/invented/Documents");
        let home = PathBuf::from("/Users/invented");
        assert_eq!(default_save_directory(Some(documents.clone()), Some(home.clone())), Some(documents));
        assert_eq!(default_save_directory(None, Some(home.clone())), Some(home.clone()));
        assert_eq!(default_save_directory(Some(PathBuf::from("relative")), Some(home.clone())), Some(home));
        assert_eq!(default_save_directory(None, None), None);
    }

    /// The raw-body arguments decode within the ingress lease (`Moved`).
    #[test]
    fn arguments_decode_through_the_ingress_lease() {
        let text = canonical_of(DEMO);
        let body = json!({
            "canonicalText": text, "claimedModelHash": hash_of(&text),
            "screening": serde_json::to_value(screening()).unwrap(),
            "localFirstEvidence": serde_json::to_value(local_first()).unwrap(),
        });
        let registry = registry(ScriptedHost::normal());
        let (mut args, _lease) = admit_json(&registry, "save_model_document_file", ArgsDecoding::Moved, &body).unwrap();
        let request = save_model_document_args(&mut args).unwrap();
        assert_eq!(request.canonical_text, text);
        assert_eq!(request.screening, screening());
        assert_eq!(request.local_first, local_first());
        let raw = serde_json::to_vec(&body).unwrap();
        let _ = check_decode_bound(&raw, ArgsDecoding::Moved, |args| save_model_document_args(args).map(drop));
        let mut unknown = body.clone();
        unknown["screening"]["extra"] = json!(true);
        let (mut args, _lease) = admit_json(&registry, "save_model_document_file", ArgsDecoding::Moved, &unknown).unwrap();
        assert!(save_model_document_args(&mut args).is_err());
    }
}
