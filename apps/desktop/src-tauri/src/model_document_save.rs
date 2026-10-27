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
//! - refuses `MODEL-DOC-SAVE-EVIDENCE-UNBOUND` unless the screening evidence
//!   names the SHA-256 of exactly this text;
//! - applies the open path's document checks (`check_model_document`: kind,
//!   current version, the W-13 document validator) and the desktop shape check
//!   (`model_document_shape`, the port of `modelDocumentShape.ts`), so every
//!   saved file re-opens;
//! - screens the dispositions of the exact admitted document itself (review
//!   repair 1: evidence is never trusted for content): every
//!   `provenance_record` at any depth, every `rule_pack_refs[]` entry and every
//!   `library_ref`, resolved against the local library store. The library
//!   import's policy applies (`library_import_document/src/lib.rs:297-372`:
//!   an item's own field wins, else its `provenance`'s); `protected_suspected`,
//!   `quarantined` and `rejected` refuse with the webview screener's code
//!   `PROTECTED_CONTENT_BLOCKED`, naming the record. A store read error refuses
//!   (`MODEL-DOC-SAVE-LIBRARY-STORE-UNAVAILABLE`); a library or record the
//!   store does not hold is no disposition: the save proceeds with a
//!   `MODEL-LIBRARY-REF-UNRESOLVED` warning naming it. Each library is read
//!   within the command's memory, started from its ingress lease
//!   (`stored_model_document_export::StoreLibraryReads`, the recovery export's
//!   metered read): measured, its load and its prescan-bounded parse each
//!   reserved before they are made, and held until the command returns. A
//!   read the pool refuses refuses the save `STORE-READ-MEMORY-UNAVAILABLE`.
//!
//! The file is written by the extension-neutral atomic writer under the
//! `.json` model-document format. Nothing is sent anywhere (F-PIP-1); the
//! chooser starts in the user's documents folder. The session's model, hash
//! and solve standing are not touched by a save.

use crate::atomic_report_package_save::{self, AtomicSaveError, MODEL_DOCUMENT_FORMAT};
use crate::command_ingress::Args;
use crate::model_document_file::{check_model_document, ModelDocumentOpenOutcome, MAX_MODEL_DOCUMENT_BYTES};
use crate::model_document_shape::{model_document_shape_diagnostics, MODEL_FILE_SHAPE};
use crate::memory_pool::Lease;
use crate::solve_jobs::{SolveJobError, SolveJobRegistry};
use crate::stored_model_document_export::StoreLibraryReads;
use crate::stored_read::PoolReadMemory;
pub use crate::export_disposition_screen::DispositionDiagnostic as ModelDocumentSaveDiagnostic;
pub use crate::export_disposition_screen::{LibraryLookup, PROTECTED_CONTENT_BLOCKED};
#[cfg(test)]
pub use crate::export_disposition_screen::LIBRARY_REF_UNRESOLVED;
use crate::export_disposition_screen::{diagnostic, screen_dispositions_with, PathStyle};
pub(crate) use crate::export_disposition_screen::StoredMembers;
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
pub const EVIDENCE_UNBOUND: &str = "MODEL-DOC-SAVE-EVIDENCE-UNBOUND";
pub const LIBRARY_STORE_UNAVAILABLE: &str = "MODEL-DOC-SAVE-LIBRARY-STORE-UNAVAILABLE";

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
    /// `sha256:<hex>` of the canonical text the webview screened; it must be
    /// the SHA-256 of the text admitted.
    pub payload_sha256: String,
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
        /// Non-blocking findings, e.g. `MODEL-LIBRARY-REF-UNRESOLVED`.
        warnings: Vec<ModelDocumentSaveDiagnostic>,
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
    validate_local_private_route_evidence(MODEL_DOCUMENT_SAVE_ROUTE_ID, screening, local_first)
}

/// The same re-check for another governed local-private, lossless route with
/// explicit intent (File > Save Handoff…, `DOTH-HANDOFF-PACKAGE-008`, W-6b).
pub fn validate_local_private_route_evidence(
    route_id: &str,
    screening: &ModelDocumentSaveScreening,
    local_first: &ModelDocumentSaveLocalFirst,
) -> Result<(), ModelDocumentSaveDiagnostic> {
    let s = screening;
    let l = local_first;
    let admitted = s.route_id == route_id
        && s.export_context == "local_private"
        && s.explicit_local_private_intent
        && !s.blocked
        && !s.materialization_withheld
        && s.lossless_required
        && s.exact_payload_match
        && s.blocking_count == 0
        && l.route_id == route_id
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
            format!("{route_id} local-private, lossless route evidence with explicit intent is missing or blocked."),
        ))
    }
}

/// A model document admitted for writing.
#[derive(Debug)]
pub struct AdmittedModelDocument {
    /// The canonical text, exactly as it will be written; its SHA-256 is the
    /// claimed model hash.
    pub text: String,
    pub suggested_file_name: String,
    pub warnings: Vec<ModelDocumentSaveDiagnostic>,
}

/// Native disposition screening of the exact admitted document (review repair
/// 1b/1c), by the shared store-resolved screen (`export_disposition_screen`).
/// A stored disposition member the verdict cannot read is refused by name
/// ([`StoredMembers::Refuse`]); Save Handoff, which screens each stored
/// library structurally itself, uses [`screen_model_document_dispositions_with`].
pub fn screen_model_document_dispositions<V: std::borrow::Borrow<Value>>(
    document: &Value,
    lookup: &mut dyn FnMut(&str, &str, &str) -> Result<Option<V>, String>,
) -> Result<Vec<ModelDocumentSaveDiagnostic>, Vec<ModelDocumentSaveDiagnostic>> {
    screen_model_document_dispositions_with(document, StoredMembers::Refuse, lookup)
}

/// [`screen_model_document_dispositions`] with the caller's [`StoredMembers`] policy.
pub(crate) fn screen_model_document_dispositions_with<V: std::borrow::Borrow<Value>>(
    document: &Value,
    stored_members: StoredMembers,
    lookup: &mut dyn FnMut(&str, &str, &str) -> Result<Option<V>, String>,
) -> Result<Vec<ModelDocumentSaveDiagnostic>, Vec<ModelDocumentSaveDiagnostic>> {
    let project_id = document.pointer("/project/id").and_then(Value::as_str).unwrap_or("");
    screen_dispositions_with(document, "", PathStyle::Dotted, project_id, LIBRARY_STORE_UNAVAILABLE, stored_members, lookup)
}

fn claimed_hex(claimed: &str) -> Option<&str> {
    claimed
        .strip_prefix("sha256:")
        .filter(|hex| hex.len() == 64 && hex.bytes().all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f')))
}

/// The admission of §2 over the webview's canonical text, before any file is
/// chosen, with no reservation before the screening (tests).
#[cfg(test)]
pub fn admit_model_document_text<V: std::borrow::Borrow<Value>>(
    canonical_text: String,
    claimed_model_hash: &str,
    evidence_payload_sha256: &str,
    lookup: &mut dyn FnMut(&str, &str, &str) -> Result<Option<V>, String>,
) -> Result<AdmittedModelDocument, Vec<ModelDocumentSaveDiagnostic>> {
    admit_model_document_text_screened(canonical_text, claimed_model_hash, evidence_payload_sha256, &mut |_| Ok(()), lookup)
}

/// [`admit_model_document_text`], with `before_screening` given the checked
/// document before the screening starts (the saves reserve the screening's
/// memory there).
pub fn admit_model_document_text_screened<V: std::borrow::Borrow<Value>>(
    canonical_text: String,
    claimed_model_hash: &str,
    evidence_payload_sha256: &str,
    before_screening: &mut dyn FnMut(&Value) -> Result<(), Vec<ModelDocumentSaveDiagnostic>>,
    lookup: &mut dyn FnMut(&str, &str, &str) -> Result<Option<V>, String>,
) -> Result<AdmittedModelDocument, Vec<ModelDocumentSaveDiagnostic>> {
    admit_model_document_text_screened_with(canonical_text, claimed_model_hash, evidence_payload_sha256, before_screening, StoredMembers::Refuse, lookup)
}

/// [`admit_model_document_text_screened`] with the caller's [`StoredMembers`]
/// policy (Save Handoff: [`StoredMembers::Structural`]).
pub(crate) fn admit_model_document_text_screened_with<V: std::borrow::Borrow<Value>>(
    canonical_text: String,
    claimed_model_hash: &str,
    evidence_payload_sha256: &str,
    before_screening: &mut dyn FnMut(&Value) -> Result<(), Vec<ModelDocumentSaveDiagnostic>>,
    stored_members: StoredMembers,
    lookup: &mut dyn FnMut(&str, &str, &str) -> Result<Option<V>, String>,
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
    if evidence_payload_sha256 != format!("sha256:{hex}") {
        return Err(vec![diagnostic(
            EVIDENCE_UNBOUND,
            "",
            format!("The screening evidence is for {evidence_payload_sha256}, not for this text (sha256:{hex})."),
        )]);
    }
    let suggested_file_name = suggested_model_document_file_name(&document);
    let document = match check_model_document(String::new(), byte_count, document) {
        ModelDocumentOpenOutcome::Opened { schema_status, document, .. } if schema_status == "current" => document,
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
    };
    let shape = model_document_shape_diagnostics(&document);
    if !shape.is_empty() {
        return Err(shape.into_iter().map(|item| diagnostic(MODEL_FILE_SHAPE, &item.path, item.message)).collect());
    }
    before_screening(&document)?;
    let warnings = screen_model_document_dispositions_with(&document, stored_members, lookup)?;
    drop(document);
    Ok(AdmittedModelDocument { text: canonical_text, suggested_file_name, warnings })
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
            warnings: admitted.warnings,
        },
        Err(error) => ModelDocumentSaveOutcome::refused(vec![writer_diagnostic(error)]),
    }
}

/// The save after its ingress and its route-evidence check (`lib.rs`
/// `save_model_document_decoded`), on one blocking worker: the admission of
/// §2, with each referenced library read from the local store (`open_store`,
/// opened only if the document references one) within the command's memory
/// (`StoreLibraryReads`: started from the ingress lease `ingress`, so every
/// library reservation is counted with it against the per-invocation budget);
/// then the chooser (`choose`, given the suggested file name) and the atomic
/// write. The screening's allocations are reserved before it builds them, as
/// the recovery export reserves them (`StoreLibraryReads::screen_model`). A
/// library read or screening the memory pool refuses refuses the save by the
/// pool's name (`STORE-READ-MEMORY-UNAVAILABLE`) before any file is chosen.
/// The outcome is serialized within the same memory while every reservation
/// is held (`stored_model_document_export::serialize_outcome`; a Saved
/// response's bound is reserved before the chooser), and `respond` receives
/// it and its text before any reservation is released.
pub(crate) fn save_model_document_admitted<R>(
    canonical_text: String,
    claimed_model_hash: &str,
    evidence_payload_sha256: &str,
    open_store: impl FnMut() -> Result<rusqlite::Connection, String>,
    registry: &SolveJobRegistry,
    ingress: &Lease,
    choose: impl FnOnce(&str) -> Result<Option<PathBuf>, String>,
    respond: impl FnOnce(&ModelDocumentSaveOutcome, String) -> R,
) -> R {
    use crate::stored_model_document_export::serialize_outcome;
    let memory = PoolReadMemory::new(registry, ingress);
    let mut libraries = StoreLibraryReads::new(&memory, open_store);
    let (outcome, reserved) = save_model_document_outcome(canonical_text, claimed_model_hash, evidence_payload_sha256, &memory, &mut libraries, choose);
    // The outcome is serialized while every reservation its diagnostics live
    // under is held (#1641 Codex r2 P2-b), and the response built from it.
    let (sent, text, buffer) = serialize_outcome(&memory, "Save Model Document", "save", outcome, reserved);
    let response = respond(&sent, text);
    drop(sent);
    drop(buffer);
    drop(libraries);
    #[cfg(test)]
    crate::stored_model_document_export::library_read_probe::record_save_peak(memory.peak_reserved());
    response
}

/// What a Saved outcome adds to its warnings' JSON: its tag, the file name (at
/// most 255 bytes, each escaped to at most six), the byte count, the hash and
/// the durability.
pub(crate) const SAVED_FIELDS_BYTES: u64 = 4096;

/// [`save_model_document_admitted`]'s outcome, and the Saved response's
/// bound reserved before the chooser (with its bytes), when the document was
/// admitted: no response is refused after a file was written.
fn save_model_document_outcome(
    canonical_text: String,
    claimed_model_hash: &str,
    evidence_payload_sha256: &str,
    memory: &PoolReadMemory<'_>,
    libraries: &mut StoreLibraryReads<'_>,
    choose: impl FnOnce(&str) -> Result<Option<PathBuf>, String>,
) -> (ModelDocumentSaveOutcome, Option<(crate::stored_read::ReadReservation, u64)>) {
    use crate::stored_read::ReadMemory;
    let reads = std::cell::RefCell::new(libraries);
    let admitted = admit_model_document_text_screened(
        canonical_text,
        claimed_model_hash,
        evidence_payload_sha256,
        &mut |document: &Value| reads.borrow_mut().screen_model(document),
        &mut |project_id: &str, library_kind: &str, library_id: &str| reads.borrow_mut().read(project_id, library_kind, library_id),
    );
    let libraries = reads.into_inner();
    if let Some(refusal) = libraries.refusal() {
        return (ModelDocumentSaveOutcome::refused(refusal), None);
    }
    let admitted = match admitted {
        Ok(admitted) => admitted,
        Err(diagnostics) => return (ModelDocumentSaveOutcome::refused(diagnostics), None),
    };
    let bound = crate::stored_read::serialized_length(&admitted.warnings).unwrap_or(u64::MAX).saturating_add(SAVED_FIELDS_BYTES);
    let response = match memory.reserve("the response to Save Model Document", crate::stored_read::exact_text_bytes(bound)) {
        Ok(response) => response,
        Err(error) => return (ModelDocumentSaveOutcome::refused_one(crate::stored_read::STORE_READ_MEMORY_UNAVAILABLE, format!("{error} Nothing was written.")), None),
    };
    let reserved = Some((response, bound));
    let destination = match choose(&admitted.suggested_file_name) {
        Ok(Some(destination)) => destination,
        Ok(None) => return (ModelDocumentSaveOutcome::Cancelled, reserved),
        Err(message) => return (ModelDocumentSaveOutcome::refused_one(PICKER_PATH_INVALID, message), reserved),
    };
    (write_admitted_model_document(&destination, admitted), reserved)
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

    fn no_store(_: &str, _: &str, _: &str) -> Result<Option<Value>, String> {
        Ok(None)
    }

    /// Admission with evidence bound to the text and an empty library store.
    fn admit(text: String, claimed: &str) -> Result<AdmittedModelDocument, Vec<ModelDocumentSaveDiagnostic>> {
        let bound = hash_of(&text);
        admit_model_document_text(text, claimed, &bound, &mut no_store)
    }

    fn admit_with_store(
        text: String,
        stored: Result<Option<Value>, String>,
    ) -> (Result<AdmittedModelDocument, Vec<ModelDocumentSaveDiagnostic>>, Vec<(String, String, String)>) {
        let hash = hash_of(&text);
        let mut calls = Vec::new();
        let mut lookup = |project: &str, kind: &str, id: &str| {
            calls.push((project.to_string(), kind.to_string(), id.to_string()));
            stored.clone()
        };
        let result = admit_model_document_text(text, &hash, &hash, &mut lookup);
        (result, calls)
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
            payload_sha256: hash_of(&canonical_of(DEMO)),
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
        let _save_lock = crate::atomic_report_package_save::test_support::atomic_save_test_lock();
        for (label, source) in [("demo", DEMO), ("every-slot", EVERY_SLOT)] {
            let text = canonical_of(source);
            let claimed = hash_of(&text);
            let admitted = admit(text.clone(), &claimed).unwrap();
            assert_eq!(hash_of(&admitted.text), claimed);
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
        assert_eq!(codes(admit(text.clone(), &hash_of("{}"))), mismatch());
        assert_eq!(codes(admit(text.clone(), "sha256:ABC")), mismatch());
        assert_eq!(codes(admit(text.clone(), claimed.trim_start_matches("sha256:"))), mismatch());
        assert_eq!(codes(admit(text.clone(), &claimed.to_uppercase())), mismatch());
        // Non-canonical text of the same document: pretty-printed, a trailing
        // newline, the fixture as authored. Each is refused even with its own hash.
        let pretty = serde_json::to_string_pretty(&serde_json::from_str::<Value>(DEMO).unwrap()).unwrap();
        let newline = format!("{text}\n");
        let as_authored = DEMO.to_string();
        for variant in [pretty, newline, as_authored] {
            assert_ne!(variant, text);
            let own = hash_of(&variant);
            assert_eq!(codes(admit(variant, &own)), mismatch());
        }
        assert_eq!(codes(admit("{not json".to_string(), &hash_of("{not json"))), mismatch());
        assert_eq!(codes(admit(text, &claimed)), vec![("admitted".to_string(), String::new())]);
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
        for (version, code) in [("0.6.0", "MODEL-FILE-SCHEMA-REFUSED"), ("0.7.0", "MODEL-FILE-SCHEMA-REFUSED"), ("0.9.0", "MODEL-FILE-SCHEMA-REFUSED"), ("x", "MODEL-FILE-SCHEMA-REFUSED")] {
            let (text, hash) = canonical_variant(|document| document["schema_version"] = json!(version));
            assert_eq!(codes(admit(text, &hash)), vec![(code.to_string(), "schema_version".to_string())], "{version}");
        }
        let (text, hash) = canonical_variant(|document| {
            document.as_object_mut().unwrap().remove("schema_version");
        });
        assert_eq!(codes(admit(text, &hash))[0].0, "MODEL-FILE-SCHEMA-REFUSED");
        let (text, hash) = canonical_variant(|document| document["document_kind"] = json!("openpipestress.other"));
        assert_eq!(codes(admit(text, &hash)), vec![("MODEL-FILE-DOCUMENT-KIND".to_string(), "document_kind".to_string())]);
        let (text, hash) = canonical_variant(|document| {
            document["lines"][0]["pipe_refs"] = json!(["pipe:P-absent"]);
            document["unresolved_assumptions"][0]["status"] = json!("resolved");
        });
        assert_eq!(codes(admit(text, &hash)), vec![
            ("MODEL-LINE-PIPE-MISSING".to_string(), "lines[0].pipe_refs[0]".to_string()),
            ("MODEL-ASSUMPTION-RESOLUTION-NOTE-REQUIRED".to_string(), "unresolved_assumptions[0].resolution_note".to_string()),
        ]);
        // A quarantined provenance disposition is only a validator warning;
        // the native disposition screening refuses it (review repair 1).
        let (text, hash) = canonical_variant(|document| document["materials"][0]["provenance_record"]["review_status"] = json!("quarantined"));
        assert_eq!(codes(admit(text, &hash)), vec![(PROTECTED_CONTENT_BLOCKED.to_string(), "materials[0].provenance_record".to_string())]);
    }

    #[test]
    fn the_open_cap_applies_before_parsing() {
        let limit = MAX_MODEL_DOCUMENT_BYTES as usize;
        // One byte over the cap: refused by size, whatever the text is.
        let over = "x".repeat(limit + 1);
        let claim = hash_of("x");
        assert_eq!(codes(admit(over, &claim)), vec![(TOO_LARGE.to_string(), String::new())]);
        // At the cap the size check passes and the next check (canonical text) speaks.
        let at = "x".repeat(limit);
        assert_eq!(codes(admit(at, &claim)), vec![(HASH_MISMATCH.to_string(), String::new())]);
    }

    /// Review repair 1 (reviewer regression): complete caller-authored "clear"
    /// evidence does not admit quarantined content; the native screening does.
    #[test]
    fn an_unbound_clean_evidence_object_cannot_admit_quarantined_content() {
        let (text, hash) = canonical_variant(|document| {
            document["materials"][0]["provenance_record"]["review_status"] = json!("quarantined");
        });
        assert!(validate_model_document_save_evidence(&screening(), &local_first()).is_ok());
        let refused = admit(text, &hash).expect_err("a complete caller-authored evidence object must not bypass native disposition admission");
        assert_eq!(refused.len(), 1);
        assert_eq!((refused[0].code.as_str(), refused[0].path.as_str()), (PROTECTED_CONTENT_BLOCKED, "materials[0].provenance_record"));
        let every_slot: Value = serde_json::from_str(EVERY_SLOT).unwrap();
        assert!(refused[0].message.contains(every_slot["materials"][0]["id"].as_str().unwrap()), "{}", refused[0].message);
    }

    /// Review repair 1 (reviewer regression): the webview's open shape check, natively.
    #[test]
    fn a_saved_document_must_meet_the_webviews_required_shape() {
        let (text, hash) = canonical_variant(|document| {
            document.as_object_mut().unwrap().remove("nodes");
        });
        let refused = admit(text, &hash).expect_err("a file without nodes is refused by the existing webview open path");
        assert_eq!(codes(Err(refused)), vec![("MODEL-FILE-SHAPE".to_string(), "nodes".to_string())]);
    }

    #[test]
    fn evidence_for_other_bytes_is_refused() {
        let text = canonical_of(DEMO);
        let hash = hash_of(&text);
        let refused = admit_model_document_text(text, &hash, &hash_of("{}"), &mut no_store).unwrap_err();
        assert_eq!(refused[0].code, EVIDENCE_UNBOUND);
    }

    #[test]
    fn every_disposition_at_any_depth_is_screened() {
        let blocked = |edit: fn(&mut Value)| codes(admit(canonical_variant(edit).0, &canonical_variant(edit).1));
        let protected = |path: &str| vec![(PROTECTED_CONTENT_BLOCKED.to_string(), path.to_string())];
        // A quarantined record nested inside another provenance record.
        assert_eq!(blocked(|d| d["review_extension"] = json!({"provenance_record": {"source_name": "outer", "review_status": "accepted", "nested": {"provenance_record": {"source_name": "inner", "review_status": "quarantined"}}}})),
            protected("review_extension.provenance_record.nested.provenance_record"));
        assert_eq!(blocked(|d| d["sections"][0]["provenance_record"]["review_status"] = json!("rejected")), protected("sections[0].provenance_record"));
        assert_eq!(blocked(|d| d["supports"][0]["provenance_record"]["redistribution_status"] = json!("protected_suspected")), protected("supports[0].provenance_record"));
        assert_eq!(blocked(|d| d["rule_pack_refs"][0]["redistribution_status"] = json!("protected_suspected")), protected("rule_pack_refs[0]"));
        // `rejected` is outside the rule-pack reference vocabulary: the validator refuses it first.
        assert_eq!(blocked(|d| d["rule_pack_refs"][0]["redistribution_status"] = json!("rejected")),
            vec![("MODEL-RULE-PACK-REF-INVALID".to_string(), "rule_pack_refs[0].redistribution_status".to_string())]);
        // private_only and unknown save.
        let (text, hash) = canonical_variant(|d| {
            d["materials"][0]["provenance_record"]["redistribution_status"] = json!("private_only");
            d["sections"][0]["provenance_record"]["redistribution_status"] = json!("unknown");
            d["rule_pack_refs"][0]["redistribution_status"] = json!("private_only");
        });
        assert!(admit(text, &hash).is_ok());
    }

    fn stored_library(library: Value, records: Value) -> Value {
        json!({"schema_version": "0.1.0", "material_library": library, "material_records": records})
    }

    #[test]
    fn library_references_resolve_against_the_store_with_the_import_policy() {
        let every_slot: Value = serde_json::from_str(EVERY_SLOT).unwrap();
        let reference = &every_slot["materials"][0]["provenance_record"]["library_ref"];
        let (kind, id, record) = (reference["library_kind"].as_str().unwrap(), reference["library_id"].as_str().unwrap(), reference["record_id"].as_str().unwrap());
        let text = canonical_of(EVERY_SLOT);
        let accepted = json!({"redistribution_status": "private_only", "review_status": "accepted"});
        let path = "materials[0].provenance_record.library_ref";
        let refused = |stored: Value| match admit_with_store(text.clone(), Ok(Some(stored))).0 {
            Err(diagnostics) => diagnostics.into_iter().map(|item| (item.code, item.path)).collect::<Vec<_>>(),
            Ok(_) => vec![],
        };
        let protected = vec![(PROTECTED_CONTENT_BLOCKED.to_string(), path.to_string())];
        // The library's own field wins over permissive provenance.
        assert_eq!(refused(stored_library(json!({"redistribution_status": "protected_suspected", "review_status": "quarantined", "provenance": accepted}), json!([]))), protected);
        // A record quarantined through its own provenance.
        assert_eq!(refused(stored_library(json!({"provenance": accepted}), json!([{"material_id": record, "provenance": {"review_status": "quarantined"}}]))), protected);
        // The import's rejection vocabulary: a rejected redistribution with an accepted review.
        assert_eq!(refused(stored_library(json!({"provenance": accepted}), json!([{"material_id": record, "redistribution_status": "rejected", "review_status": "accepted"}]))), protected);
        // Accepted library and record: saved, the store asked once with the document's project.
        let (result, calls) = admit_with_store(text.clone(), Ok(Some(stored_library(json!({"provenance": accepted}), json!([{"material_id": record, "provenance": accepted}])))));
        assert!(result.unwrap().warnings.is_empty());
        assert_eq!(calls, vec![(every_slot["project"]["id"].as_str().unwrap().to_string(), kind.to_string(), id.to_string())]);
        // An absent record or library is no disposition: saved with a warning naming it.
        let (result, _) = admit_with_store(text.clone(), Ok(Some(stored_library(json!({"provenance": accepted}), json!([])))));
        let warnings = result.unwrap().warnings;
        assert_eq!((warnings[0].code.as_str(), warnings[0].path.as_str()), (LIBRARY_REF_UNRESOLVED, path));
        assert!(warnings[0].message.contains(record));
        let (result, _) = admit_with_store(text.clone(), Ok(None));
        let warnings = result.unwrap().warnings;
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].code, LIBRARY_REF_UNRESOLVED);
        assert!(warnings[0].message.contains(id));
        // A store read error refuses.
        let (result, _) = admit_with_store(text, Err("invented store failure".to_string()));
        assert_eq!(result.unwrap_err()[0].code, LIBRARY_STORE_UNAVAILABLE);
        // A document without library references never reads the store.
        let (result, calls) = admit_with_store(canonical_of(DEMO), Err("unread".to_string()));
        assert!(result.is_ok() && calls.is_empty());
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
        let admitted = admit(text.clone(), &hash_of(&text)).unwrap();
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
            warnings: vec![diagnostic(LIBRARY_REF_UNRESOLVED, "p", "w")],
        };
        assert_eq!(
            serde_json::to_value(saved).unwrap(),
            json!({"outcome": "saved", "file_name": "a.json", "byte_count": 2, "model_hash": hash_of("{}"), "replaced_existing": false, "durability": "durable",
                "warnings": [{"code": LIBRARY_REF_UNRESOLVED, "path": "p", "message": "w"}]})
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

    // Independent review round 3 (the reviewer's regressions): members the
    // workspace dereferences on adoption.
    #[test]
    fn save_refuses_unit_values_that_crash_adoption() {
        let (text, hash) = canonical_variant(|d| d["project"]["units"]["force"] = json!(17));
        assert_eq!(codes(admit(text, &hash)), vec![("MODEL-FILE-SHAPE".to_string(), "project.units.force".to_string())], "normalizeSymbol calls symbol.trim() after adoption");
    }

    #[test]
    fn save_refuses_component_completeness_that_crashes_adoption() {
        let (text, hash) = canonical_variant(|d| d["components"][0]["completeness"] = json!({}));
        assert_eq!(codes(admit(text, &hash)), vec![("MODEL-FILE-SHAPE".to_string(), "components[0].completeness".to_string())], "ModelTree calls component.completeness?.map() after adoption");
    }

    #[test]
    fn save_refuses_diagnostic_identity_that_crashes_adoption() {
        let (text, hash) = canonical_variant(|d| d["diagnostics"] = json!([{"id":{},"code":"INVENTED","severity":"warning","message":"Invented"}]));
        assert_eq!(codes(admit(text, &hash)), vec![("MODEL-FILE-SHAPE".to_string(), "diagnostics[0].id".to_string())], "ModelTree renders diagnostic.id as a React child after adoption");
    }

    // Independent review round 2 (the reviewer's regressions).
    #[test]
    fn save_refuses_a_document_the_webview_cannot_index() {
        let (text, hash) = canonical_variant(|d| d["pipe_segments"][0]["section_ref"] = json!(17));
        let refused = admit(text, &hash).expect_err("the actual open's buildModelIndex calls section_ref.trim() and refuses this document");
        assert_eq!(codes(Err(refused)), vec![("MODEL-FILE-SHAPE".to_string(), "pipe_segments[0].section_ref".to_string())]);
    }

    #[test]
    fn library_lookup_normalizes_identity_like_open_local_library() {
        let (text, hash) = canonical_variant(|d| d["materials"][0]["provenance_record"]["library_ref"]["library_id"] = json!(" library:invented-materials "));
        let bound = hash_of(&text);
        let mut lookup = |_: &str, _: &str, id: &str| Ok(if id == "library:invented-materials" {
            Some(stored_library(json!({"review_status": "quarantined"}), json!([])))
        } else { None });
        let refused = admit_model_document_text(text, &hash, &bound, &mut lookup).expect_err("normal open resolves this padded id to a quarantined library");
        assert_eq!(refused[0].code, PROTECTED_CONTENT_BLOCKED);
        // A padded record id is screened against the record it names.
        let (text, hash) = canonical_variant(|d| d["materials"][0]["provenance_record"]["library_ref"]["record_id"] = json!(" steel "));
        let bound = hash_of(&text);
        let mut lookup = |_: &str, _: &str, _: &str| Ok(Some(stored_library(json!({"review_status": "accepted"}), json!([{"material_id": "steel", "review_status": "quarantined"}]))));
        assert_eq!(admit_model_document_text(text, &hash, &bound, &mut lookup).unwrap_err()[0].code, PROTECTED_CONTENT_BLOCKED);
    }

    #[test]
    fn resolved_hanger_disposition_agrees_with_the_actual_import() {
        let mut library: Value = serde_json::from_str(include_str!("../../../../fixtures/hanger/invented_hanger_library_valid.json")).unwrap();
        library["hanger_records"][0]["review_status"] = json!("accepted");
        library["hanger_records"][0]["provenance"]["review_status"] = json!("quarantined");
        let imported = open_pipe_stress_library_import_document::validate_library_import_tokens(&library, "hanger", "private").unwrap();
        assert_eq!(imported.outcome, "QUARANTINE");
        let (text, _) = canonical_variant(|d| d["materials"][0]["provenance_record"]["library_ref"] = json!({"library_kind":"hanger", "library_id":"invented.hangers", "record_id":"invented.variable"}));
        let refused = admit_with_store(text, Ok(Some(library))).0.expect_err("the actual hanger import quarantines this record");
        assert_eq!(refused[0].code, PROTECTED_CONTENT_BLOCKED);
    }

    /// W-18a (b1) review round 1 probe: W-6a's model save does not run the
    /// structural screen (its stored-item verdicts are the library import's,
    /// `disposition_verdicts_match_the_library_import`), so a `classification`
    /// object is judged as the import judges it. Pinned here so a change to
    /// that scope is a visible decision.
    #[test]
    fn b1r1_classification_objects_follow_the_import_verdict() {
        let every_slot: Value = serde_json::from_str(EVERY_SLOT).unwrap();
        let record = every_slot["materials"][0]["provenance_record"]["library_ref"]["record_id"].as_str().unwrap().to_string();
        let library = stored_library(json!({"provenance": {"review_status": "accepted"}}),
            json!([{"material_id": record, "provenance": {"review_status": "accepted"}, "classification": {"value": "protected_suspected"}}]));
        let imported = open_pipe_stress_library_import_document::validate_library_import_tokens(&library, "material", "private").unwrap();
        let import_blocks = imported.findings.iter().any(|finding| matches!(finding.code.as_str(), "IMPORT_PROTECTED_CONTENT_SUSPECTED" | "IMPORT_REJECTED_SOURCE"));
        let (result, _) = admit_with_store(canonical_of(EVERY_SLOT), Ok(Some(library)));
        assert_eq!(result.is_err(), import_blocks, "the stored record: {:?}", codes(result));
        let (text, hash) = canonical_variant(|d| d["materials"][0]["provenance_record"]["classification"] = json!({"value": "protected_suspected"}));
        println!("B1R1_W6A_MODEL_PROVENANCE {:?}", codes(admit(text, &hash)));
    }

    #[test]
    fn duplicate_matching_library_records_do_not_hide_a_quarantine() {
        let text = canonical_of(EVERY_SLOT);
        let library = stored_library(json!({"review_status":"accepted"}), json!([
            {"material_id":"steel", "review_status":"accepted"},
            {"material_id":"steel", "review_status":"quarantined"}
        ]));
        assert!(admit_with_store(text, Ok(Some(library))).0.is_err(), "duplicate identities must be refused or every matching record screened");
    }

    /// Main's authoring and library-apply outputs (W-13 S2a/S2b emission goldens,
    /// including `library_material_insert`) save and re-open unchanged. With an
    /// empty store each library reference is a `MODEL-LIBRARY-REF-UNRESOLVED`
    /// warning, never a refusal.
    #[test]
    fn every_emitted_authoring_and_library_apply_document_saves_and_reopens() {
        let _save_lock = crate::atomic_report_package_save::test_support::atomic_save_test_lock();
        let root = sandbox("emitted");
        let mut saved = 0;
        for (fixture, member) in [
            (include_str!("../../../../fixtures/model_document/desktop_emitted.json"), "document"),
            (include_str!("../../../../fixtures/model_document/applier_emitted.json"), "applied_model"),
        ] {
            let golden: Value = serde_json::from_str(fixture).unwrap();
            for (index, entry) in golden["entries"].as_array().unwrap().iter().enumerate() {
                let label = format!("{member}[{index}] {}", entry.get("path").or_else(|| entry.get("change_kind")).unwrap());
                let text = canonical_json(&entry[member]);
                let admitted = admit(text.clone(), &hash_of(&text)).unwrap_or_else(|diagnostics| panic!("{label} refused: {diagnostics:?}"));
                assert!(admitted.warnings.iter().all(|item| item.code == LIBRARY_REF_UNRESOLVED), "{label}: {:?}", admitted.warnings);
                let destination = root.join(format!("emitted-{saved}.json"));
                assert!(matches!(write_admitted_model_document(&destination, admitted), ModelDocumentSaveOutcome::Saved { .. }), "{label}");
                let ModelDocumentOpenOutcome::Opened { document, .. } = read_model_document_file(&destination) else { panic!("{label} does not re-open") };
                assert_eq!(canonical_json(&document), text, "{label}");
                saved += 1;
            }
        }
        assert_eq!(saved, 36); // 0.8.0 adds a reducer's eccentric offset to each golden
        fs::remove_dir_all(root).unwrap();
    }

    /// Every case of the shared shape corpus: admitted exactly when its shape is
    /// clean, and every admitted case, once saved, re-opens through the native
    /// open with a clean shape (the webview half is in modelDocumentSave.test.tsx).
    #[test]
    fn every_shape_corpus_case_that_admits_reopens() {
        let _save_lock = crate::atomic_report_package_save::test_support::atomic_save_test_lock();
        let corpus: Value = serde_json::from_str(include_str!("../../../../fixtures/model_document/desktop_shape_cases.json")).unwrap();
        let root = sandbox("corpus-reopen");
        let mut admitted_cases = 0;
        for case in corpus["cases"].as_array().unwrap() {
            let id = case["case_id"].as_str().unwrap();
            let text = canonical_json(&crate::model_document_shape::corpus_case_document(&corpus, case));
            let hash = hash_of(&text);
            let shape_clean = case["expected"].as_array().unwrap().is_empty();
            match admit(text.clone(), &hash) {
                Ok(admitted) => {
                    assert!(shape_clean, "{id} admitted despite shape findings");
                    let destination = root.join(format!("{id}.json"));
                    assert!(matches!(write_admitted_model_document(&destination, admitted), ModelDocumentSaveOutcome::Saved { .. }), "{id}");
                    let ModelDocumentOpenOutcome::Opened { document, .. } = read_model_document_file(&destination) else {
                        panic!("{id}: the saved file does not re-open");
                    };
                    assert!(crate::model_document_shape::model_document_shape_diagnostics(&document).is_empty(), "{id}");
                    assert_eq!(canonical_json(&document), text, "{id}");
                    admitted_cases += 1;
                }
                Err(diagnostics) => assert!(!shape_clean, "{id} refused although its shape is clean: {diagnostics:?}"),
            }
        }
        assert!(admitted_cases >= 4, "{admitted_cases}");
        fs::remove_dir_all(root).unwrap();
    }

    // --- The library reads' memory (DEL-04-07) ------------------------------

    /// A store (opened as the command opens it) holding `library` as the
    /// every-slot model's referenced material library's `document_json`.
    fn library_store(label: &str, library: &str) -> (PathBuf, PathBuf) {
        let root = sandbox(label);
        let store = root.join(crate::PROJECT_STORE_FILE);
        let (connection, _) = crate::open_project_store(&store).unwrap();
        connection
            .execute(
                "INSERT INTO local_libraries(project_id, library_kind, library_id, document_json, created_at_unix, updated_at_unix) VALUES ('project:slots', 'material', 'library:invented-materials', ?1, 1, 1)",
                [library],
            )
            .unwrap();
        (root, store)
    }

    fn representative_text() -> String {
        serde_json::to_string(&crate::stored_model_document_export::library_read_probe::representative_material_library()).unwrap()
    }

    /// The command's ingress for the every-slot model, then its arguments.
    fn admitted_request(pool: &SolveJobRegistry) -> (ModelDocumentSaveRequest, Lease) {
        let text = canonical_of(EVERY_SLOT);
        let mut evidence = screening();
        evidence.payload_sha256 = hash_of(&text);
        let body = json!({
            "canonicalText": text, "claimedModelHash": hash_of(&text),
            "screening": serde_json::to_value(evidence).unwrap(),
            "localFirstEvidence": serde_json::to_value(local_first()).unwrap(),
        });
        let (mut args, lease) = admit_json(pool, "save_model_document_file", ArgsDecoding::Moved, &body).unwrap();
        (save_model_document_args(&mut args).unwrap(), lease)
    }

    /// The command after its ingress, the store at `store`; `shown` records the
    /// pool's reserved bytes when the chooser is shown.
    fn save_at(
        store: &Path,
        pool: &SolveJobRegistry,
        request: ModelDocumentSaveRequest,
        lease: &Lease,
        chosen: Result<Option<PathBuf>, String>,
        shown: &std::cell::Cell<Option<u64>>,
    ) -> ModelDocumentSaveOutcome {
        let open = || crate::open_project_store(store).map(|(connection, _)| connection);
        save_model_document_admitted(
            request.canonical_text,
            &request.claimed_model_hash,
            &request.screening.payload_sha256,
            open,
            pool,
            lease,
            |_| {
                shown.set(Some(pool.pool_status().reserved_bytes));
                chosen
            },
            |outcome, text| {
                assert_eq!(serde_json::to_string(outcome).unwrap(), text, "the response is the outcome sent");
                outcome.clone()
            },
        )
    }

    /// The counting-allocator check: Save Model Document reads the
    /// representative library within its reservations (the read's measured
    /// peak, priced as the ingress prescan prices allocations, is within what
    /// it reserved), holds them with the ingress lease through the chooser and
    /// the write, and releases every one when it returns. The file is
    /// byte-identical to the one the unmetered lookup wrote.
    #[test]
    fn save_reads_libraries_within_their_reservations_and_writes_the_same_bytes() {
        let _save_lock = crate::atomic_report_package_save::test_support::atomic_save_test_lock();
        let library = representative_text();
        let (root, store) = library_store("library-lease", &library);
        let pool = registry(ScriptedHost::normal());
        let (request, lease) = admitted_request(&pool);
        let text = request.canonical_text.clone();
        let destination = root.join("metered.json");
        let shown = std::cell::Cell::new(None);
        crate::stored_model_document_export::library_read_probe::start();
        let outcome = save_at(&store, &pool, request, &lease, Ok(Some(destination.clone())), &shown);
        let reads = crate::stored_model_document_export::library_read_probe::take();
        let ModelDocumentSaveOutcome::Saved { warnings, .. } = &outcome else { panic!("not saved: {outcome:?}") };
        assert!(warnings.is_empty(), "the library resolved: {warnings:?}");
        assert_eq!(reads.len(), 1, "{reads:?}");
        let (reserved, peak) = reads[0];
        let library_bytes = library.len() as u64;
        let load_and_parse = crate::stored_read::loaded_text_bytes(library_bytes) + crate::stored_read::parse_bytes(&library).unwrap();
        eprintln!("SAVE_MODEL_DOCUMENT_LIBRARY_READ library_bytes={library_bytes} reserved={reserved} load_and_parse={load_and_parse} measured_peak={peak}");
        assert!(reserved >= load_and_parse + crate::stored_read::READ_OVERHEAD_BYTES);
        assert!(peak <= reserved, "the read's measured peak {peak} exceeds its reservations {reserved}");
        let held = shown.get().expect("the chooser was shown");
        assert!(held >= lease.bytes() + reserved, "the library reservations are held with the lease through the chooser: {held}");
        assert_eq!(pool.pool_status().reserved_bytes, lease.bytes(), "the save releases its reservations when it returns");
        drop(lease);
        assert_eq!(pool.pool_status().reserved_bytes, 0);
        // The same text admitted with the unmetered lookup writes the same bytes.
        let connection = rusqlite::Connection::open(&store).unwrap();
        let mut unmetered = |project: &str, kind: &str, id: &str| crate::load_library(&connection, project.trim(), kind.trim(), id.trim()).map(|row| row.map(|row| row.document));
        let hash = hash_of(&text);
        let admitted = admit_model_document_text(text.clone(), &hash, &hash, &mut unmetered).unwrap();
        let before = root.join("unmetered.json");
        let ModelDocumentSaveOutcome::Saved { warnings: before_warnings, model_hash, byte_count, .. } = write_admitted_model_document(&before, admitted) else { panic!("not saved") };
        assert_eq!(fs::read(&destination).unwrap(), fs::read(&before).unwrap());
        assert_eq!(fs::read(&destination).unwrap(), text.as_bytes());
        assert!(matches!(&outcome, ModelDocumentSaveOutcome::Saved { warnings, model_hash: hash, byte_count: count, .. } if *warnings == before_warnings && *hash == model_hash && *count == byte_count));
        fs::remove_dir_all(root).unwrap();
    }

    /// #1641 Codex r1 P2: the whole save's measured peak (the admission, the
    /// screening, the library read and the write, priced as the ingress prescan
    /// prices allocations) is within the most its memory held at once, the
    /// ingress lease included, for a large library: 1,024 records, about 3 MB.
    #[test]
    fn the_whole_save_is_within_its_reservations_for_a_large_library() {
        let _save_lock = crate::atomic_report_package_save::test_support::atomic_save_test_lock();
        let library = serde_json::to_string(&crate::stored_model_document_export::library_read_probe::large_material_library(1024)).unwrap();
        let (root, store) = library_store("large-library", &library);
        let pool = registry(ScriptedHost::normal());
        let (request, lease) = admitted_request(&pool);
        let destination = root.join("large.json");
        crate::stored_model_document_export::library_read_probe::take_save_peaks();
        let (outcome, measured) = crate::ingress::counting::measure(|| save_at(&store, &pool, request, &lease, Ok(Some(destination.clone())), &std::cell::Cell::new(None)));
        assert!(matches!(outcome, ModelDocumentSaveOutcome::Saved { .. }), "{outcome:?}");
        let reserved = *crate::stored_model_document_export::library_read_probe::take_save_peaks().last().expect("the save recorded its peak");
        eprintln!("SAVE_MODEL_DOCUMENT_LARGE library_bytes={} lease={} reserved_peak={reserved} measured_peak={}", library.len(), lease.bytes(), measured.peak_priced);
        assert!(library.len() > 3_000_000);
        assert!(measured.peak_priced <= reserved, "the save's measured peak {} exceeds its reservations {reserved}", measured.peak_priced);
        drop(lease);
        assert_eq!(pool.pool_status().reserved_bytes, 0);
        fs::remove_dir_all(root).unwrap();
    }

    /// #1641 Codex r2 P2-b: a refusal's diagnostics outlived the library
    /// reservations and were serialized unreserved (100,000 matching rejected
    /// records: 77.6 MB live with 53 KB reserved, a 26.4 MB response). The
    /// outcome is now serialized within the save's memory while every
    /// reservation is held: when the response is built, the live bytes are
    /// within what is held, and the whole peak is within the most held. The
    /// deeply nested library of r2 P2-a saves within its reservations too.
    #[test]
    fn a_refusal_with_100000_diagnostics_is_serialized_within_the_saves_reservations() {
        use crate::stored_model_document_export::library_read_probe::{nested_long_key_library, rejected_records_library, take_save_peaks};
        let library = serde_json::to_string(&rejected_records_library(100_000)).unwrap();
        let (root, store) = library_store("rejected-records", &library);
        let pool = registry(ScriptedHost::normal());
        let (request, lease) = admitted_request(&pool);
        let open = || crate::open_project_store(&store).map(|(connection, _)| connection);
        take_save_peaks();
        let ((outcome, live, held, length), measured) = crate::ingress::counting::measure(|| {
            save_model_document_admitted(request.canonical_text, &request.claimed_model_hash, &request.screening.payload_sha256, open, &pool, &lease, |_| panic!("no chooser after a refusal"), |outcome, text| {
                (outcome.clone(), crate::ingress::counting::live_priced(), pool.pool_status().reserved_bytes, text.len())
            })
        });
        let reserved = *take_save_peaks().last().unwrap();
        let ModelDocumentSaveOutcome::Refused { diagnostics } = &outcome else { panic!("not refused") };
        eprintln!("SAVE_MODEL_DOCUMENT_REJECTED diagnostics={} response_bytes={length} live_at_response={live} held_at_response={held} reserved_peak={reserved} measured_peak={}", diagnostics.len(), measured.peak_priced);
        assert!(u64::try_from(live).unwrap() <= held, "{live} bytes live when the response is built, {held} held");
        assert!(measured.peak_priced <= reserved, "the measured peak {} exceeds the most held {reserved}", measured.peak_priced);
        // #1641 Codex r3: the response, which outlives the reservations, is
        // bounded: the first 64 of the unbounded list, then their count.
        use crate::stored_model_document_export::{DIAGNOSTICS_NOT_SHOWN, MAX_NAMED_DIAGNOSTICS, MAX_RESPONSE_BYTES};
        assert!(length as u64 <= MAX_RESPONSE_BYTES, "a {length}-byte response");
        let connection = rusqlite::Connection::open(&store).unwrap();
        let mut unmetered = |project: &str, kind: &str, id: &str| crate::load_library(&connection, project.trim(), kind.trim(), id.trim()).map(|row| row.map(|row| row.document));
        let text = canonical_of(EVERY_SLOT);
        let hash = hash_of(&text);
        let unbounded = admit_model_document_text(text, &hash, &hash, &mut unmetered).unwrap_err();
        assert_eq!(unbounded.len(), 100_000);
        assert_eq!(diagnostics.len(), MAX_NAMED_DIAGNOSTICS + 1);
        assert_eq!(&diagnostics[..MAX_NAMED_DIAGNOSTICS], &unbounded[..MAX_NAMED_DIAGNOSTICS], "the named diagnostics are the unbounded list's first");
        assert_eq!(diagnostics[MAX_NAMED_DIAGNOSTICS].code, DIAGNOSTICS_NOT_SHOWN);
        assert!(diagnostics[MAX_NAMED_DIAGNOSTICS].message.starts_with("99936 more of the 100000 diagnostics"), "{}", diagnostics[MAX_NAMED_DIAGNOSTICS].message);
        drop(lease);
        assert_eq!(pool.pool_status().reserved_bytes, 0);
        fs::remove_dir_all(root).unwrap();
        // r2 P2-a's library through Save Model Document.
        let _save_lock = crate::atomic_report_package_save::test_support::atomic_save_test_lock();
        let library = serde_json::to_string(&nested_long_key_library(100, 32 * 1024)).unwrap();
        let (root, store) = library_store("nested-library", &library);
        let (request, lease) = admitted_request(&pool);
        let (outcome, measured) = crate::ingress::counting::measure(|| save_at(&store, &pool, request, &lease, Ok(Some(root.join("nested.json"))), &std::cell::Cell::new(None)));
        assert!(matches!(outcome, ModelDocumentSaveOutcome::Saved { .. }), "{outcome:?}");
        let reserved = *take_save_peaks().last().unwrap();
        eprintln!("SAVE_MODEL_DOCUMENT_NESTED library_bytes={} reserved_peak={reserved} measured_peak={}", library.len(), measured.peak_priced);
        assert!(measured.peak_priced <= reserved);
        drop(lease);
        fs::remove_dir_all(root).unwrap();
    }

    /// A library read the memory pool refuses ends the save by the pool's name
    /// (no headroom: before the store is opened; a per-invocation budget one
    /// byte short: before the library is parsed), with nothing chosen or
    /// written and every reservation released.
    #[test]
    fn a_library_read_the_pool_refuses_refuses_the_save_and_writes_nothing() {
        let library = representative_text();
        let (root, store) = library_store("library-refused", &library);
        let destination = root.join("refused.json");
        let host = ScriptedHost::normal();
        let pool = registry(host.clone());
        let (request, lease) = admitted_request(&pool);
        host.set(|script| script.headroom_bytes = Some(0));
        let shown = std::cell::Cell::new(None);
        let outcome = save_at(&store, &pool, request, &lease, Ok(Some(destination.clone())), &shown);
        let ModelDocumentSaveOutcome::Refused { diagnostics } = &outcome else { panic!("not refused: {outcome:?}") };
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code, crate::stored_read::STORE_READ_MEMORY_UNAVAILABLE);
        assert!(diagnostics[0].message.contains("headroom_insufficient"), "{}", diagnostics[0].message);
        assert!(shown.get().is_none() && !destination.exists());
        assert_eq!(pool.pool_status().reserved_bytes, lease.bytes());
        drop(lease);
        assert_eq!(pool.pool_status().reserved_bytes, 0);
        // The smallest budget the save needs, less one byte: refused at the parse.
        let probe = registry(ScriptedHost::normal());
        let (request, lease) = admitted_request(&probe);
        crate::stored_model_document_export::library_read_probe::start();
        let saved = save_at(&store, &probe, request, &lease, Ok(Some(root.join("probe.json"))), &std::cell::Cell::new(None));
        assert!(matches!(saved, ModelDocumentSaveOutcome::Saved { .. }), "{saved:?}");
        let reserved = crate::stored_model_document_export::library_read_probe::take()[0].0;
        let budget = lease.bytes() + reserved - 1;
        drop(lease);
        let small = registry(ScriptedHost::normal());
        small.configure_pool(budget.max(64 << 20), budget, false).unwrap();
        let (request, lease) = admitted_request(&small);
        let shown = std::cell::Cell::new(None);
        crate::stored_model_document_export::library_read_probe::start();
        let outcome = save_at(&store, &small, request, &lease, Ok(Some(destination.clone())), &shown);
        let reads = crate::stored_model_document_export::library_read_probe::take();
        let ModelDocumentSaveOutcome::Refused { diagnostics } = &outcome else { panic!("not refused: {outcome:?}") };
        assert_eq!(diagnostics[0].code, crate::stored_read::STORE_READ_MEMORY_UNAVAILABLE);
        assert!(diagnostics[0].message.contains("the stored material library `library:invented-materials`"), "{}", diagnostics[0].message);
        let parse = crate::stored_read::parse_bytes(&library).unwrap();
        assert!(reads[0].1 < parse, "refused before the parse: its measured peak {} is below the parse bound {parse}", reads[0].1);
        assert!(shown.get().is_none() && !destination.exists());
        assert_eq!(small.pool_status().reserved_bytes, lease.bytes());
        drop(lease);
        assert_eq!(small.pool_status().reserved_bytes, 0);
        fs::remove_dir_all(root).unwrap();
    }

    /// A stored library that is not JSON refuses the save with the store
    /// failure's code, quoting the read's bounded description, never the
    /// stored text. Cancel and a chooser error release the reservations too.
    #[test]
    fn library_decode_errors_are_bounded_and_every_path_releases_its_reservations() {
        let pad = "x".repeat(256 * 1024);
        for (label, text) in [("unterminated", format!("{{\"material_library\":\"{pad}")), ("out-of-range", format!("{{\"pad\":\"{pad}\",\"n\":1e400}}"))] {
            let (root, store) = library_store(&format!("library-decode-{label}"), &text);
            let destination = root.join("decode.json");
            let pool = registry(ScriptedHost::normal());
            let (request, lease) = admitted_request(&pool);
            let shown = std::cell::Cell::new(None);
            let outcome = save_at(&store, &pool, request, &lease, Ok(Some(destination.clone())), &shown);
            let ModelDocumentSaveOutcome::Refused { diagnostics } = &outcome else { panic!("{label}: not refused: {outcome:?}") };
            assert_eq!(diagnostics[0].code, LIBRARY_STORE_UNAVAILABLE, "{label}");
            let message = &diagnostics[0].message;
            eprintln!("{label}: {message}");
            assert!(message.len() < 1024 && !message.contains("xxxx"), "{label}: {message}");
            assert!(shown.get().is_none() && !destination.exists());
            assert_eq!(pool.pool_status().reserved_bytes, lease.bytes(), "{label}");
            drop(lease);
            assert_eq!(pool.pool_status().reserved_bytes, 0, "{label}");
            fs::remove_dir_all(root).unwrap();
        }
        let (root, store) = library_store("library-cancel", &representative_text());
        let pool = registry(ScriptedHost::normal());
        for chosen in [Ok(None), Err("invented chooser failure".to_string())] {
            let (request, lease) = admitted_request(&pool);
            let outcome = save_at(&store, &pool, request, &lease, chosen, &std::cell::Cell::new(None));
            assert!(matches!(&outcome, ModelDocumentSaveOutcome::Cancelled) || matches!(&outcome, ModelDocumentSaveOutcome::Refused { diagnostics } if diagnostics[0].code == PICKER_PATH_INVALID), "{outcome:?}");
            assert_eq!(pool.pool_status().reserved_bytes, lease.bytes());
            drop(lease);
            assert_eq!(pool.pool_status().reserved_bytes, 0);
        }
        fs::remove_dir_all(root).unwrap();
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
