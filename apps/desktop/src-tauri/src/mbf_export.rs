//! File > Export MBF… (DEL-17-04 Design `desktop-mbf-export-import.md` §3.5–§3.7 and §3.9.5, slices 4a and 4b.1;
//! RT-R7): the native review and save of a model batch file and its identity file.
//!
//! The webview never runs the writer. It asks the native review (`review_mbf_export`) for the bounded review of the
//! session's canonical model text with the settings text, then sends the save the same text, its claimed hash, the
//! settings text, the suggested name, the digests it reviewed and its route evidence. The save runs, in order (§3.6):
//!
//! 1. the route evidence of the governed route `DOTH-MBF-012` (local-private, explicit intent, lossless, exact
//!    payload; A-4), bound to the SHA-256 of the canonical text; then the settings, read by the engine's one mapping
//!    (a title over 4 KiB is refused before the model text is copied);
//! 2. the 64 MiB open cap, re-canonicalization and the hash claim, and
//! 3. the open path's document checks, and
//! 4. the disposition screen of the exact admitted document (a protected disposition refuses
//!    `PROTECTED_CONTENT_BLOCKED`), all three by Save Model Document's admission
//!    (`model_document_save::admit_model_document_text_screened`), so an MBF is only ever written from a document
//!    the desktop opens and may save;
//! 5. worker task 3 in `save` mode ([`worker_render`], run through [`MbfWriterRun`]): it runs the writer, forms the
//!    bounded review (`export_review`, at most R = 1 MiB) and returns the result envelope. A writer refusal
//!    ([`WRITER_REFUSED`]), a truncated review ([`REVIEW_TRUNCATED`], whatever its blocking count) or a result above
//!    the 64 MiB output ceiling ([`OUTPUT_TOO_LARGE`], decided from the computed lengths before the result is built)
//!    returns only the review, and the save replies with the code and that review before any chooser. The parent
//!    reads the envelope by [`decode_worker_result`]: views, a shape fixed by the mode, and a review that agrees;
//! 6. the staleness check: the MBF's SHA-256 and the identity file's SHA-256, computed from the envelope's bytes, must
//!    equal the reviewed ones (`MBF-EXPORT-REVIEW-STALE`);
//! 7. the chooser: the suggested name must be `swbpipe-model[-<slug>].mbf` (DEL-07-01 `export-file-names.md`), the
//!    path comes only from the chooser and must end `.mbf`; the identity file is `<chosen stem>.swbpipe-ids.json`
//!    beside it (A-5);
//! 8. the writes of the envelope's bytes. First, before anything is written: both destinations pass the atomic
//!    writer's own checks (extension, parent, no symlink or directory, and off macOS no replacement); an identity file
//!    whose MBF is absent is refused (`MBF-EXPORT-IDENTITY-FILE-EXISTS`); and every file to be replaced is backed up by
//!    a hard link to a hidden name beside it (`.swbpipe-mbf-<hex>.bak`), or the save is refused
//!    (`MBF-EXPORT-REPLACE-BACKUP-UNAVAILABLE`). Then the identity file, then the MBF, each by the atomic writer under
//!    its own format ([`MBF_IDS_FORMAT`], [`MBF_FORMAT`]). After any failure the save is undone: what it wrote is
//!    removed and each backup renamed back, each only after an identity check; a destination that cannot be restored
//!    keeps its backup, named in the refusal (`MBF-EXPORT-ROLLBACK-INCOMPLETE`). On success the backups are removed (a
//!    failure there is a warning);
//! 9. the read-back of both in fixed blocks, compared by SHA-256; a mismatch undoes the save
//!    (`MBF-EXPORT-READBACK-MISMATCH`);
//! 10. the receipt: both names, byte counts and digests, and the atomic writer's durability statement.
//!
//! The bytes written are the native writer's output for the hash-checked text, from the save's own worker run; the
//! review's digests serve only the staleness check. A review is not an authorization. Nothing is redacted: a screening
//! that would alter the payload blocks the save instead.
//!
//! The review ([`MbfReviewRequest`]: the text, its claimed hash and the settings text) takes the same settings
//! mapping and §3.6 steps 2 and 3, then task 3 in `review` mode, and replies with the bounded review. It holds no route
//! evidence and opens no store, file, destination or chooser.
//!
//! Task 3's request is `u32 mode ‖ u32 settings_len ‖ settings JSON ‖ model canonical text` (little-endian; mode 1
//! `review`, 2 `save`). Its result is the ASCII envelope `MBF1\n<mode>\n<status>\n<mbf_len>\n<ids_len>\n<review_len>\n`
//! followed by the bodies; `review` mode's status is `reviewed` with no MBF or identity bytes, `save` mode's is
//! `written` with both, or `writer-refused`, `review-truncated` or `output-too-large` with only the review.
//!
//! Not connected (§3.7). The writer's memory is to be governed by running task 3 in the capped worker process (slice
//! 4b.2, after the desktop performance work integrates). Until that path is verified the production entry points
//! [`save_mbf_export_admitted`] and [`review_mbf_export_admitted`] refuse every request with [`NOT_CONNECTED`] before
//! any parse, writer run or file access. The pipelines run only through [`InProcessWriterRun`], which needs the
//! [`InProcessWriterTestSeam`] that only test code can construct.
//!
//! Host-side memory (§3.7): each of these is reserved before it is made, within the caller's memory
//! ([`PoolReadMemory`], seeded with the ingress lease): the settings' parse, the admission's parse and canonical copy,
//! the worker's result, the reply's copy of the review, the response (bounded for a receipt and for a refusal after a
//! write, so a rollback's named backups always reach the user) and a 64 KiB read-back block. A reservation the pool
//! refuses refuses the save (`MBF-EXPORT-MEMORY-UNAVAILABLE`) or the review (`MBF-REVIEW-MEMORY-UNAVAILABLE`), before
//! any file is chosen. The writer's own run is the worker's, under its grant.
//!
//! The commands' registration, ingress admission and menu are slice 4b.2. Nothing is sent anywhere (F-PIP-1).
//! Standard claim fence applies (claims taxonomy per DEC-081): a written MBF means the listed subset of the model
//! was written; it is not evidence that another program reads or analyses it alike.

// Slice 4b.2 registers the commands; until then nothing outside this module's tests calls them.
#![cfg_attr(not(test), allow(dead_code))]

use crate::atomic_report_package_save::{self, AtomicSaveError, AtomicSaveFormat, AtomicSaveOutcome};
use crate::command_ingress::Args;
use crate::export_disposition_screen::diagnostic;
use crate::memory_pool::Lease;
use crate::model_document_save::{
    admit_model_document_text_screened, validate_local_private_route_evidence, writer_diagnostic,
    ModelDocumentSaveDiagnostic as Diagnostic, ModelDocumentSaveLocalFirst, ModelDocumentSaveScreening,
};
use crate::solve_jobs::{SolveJobError, SolveJobRegistry};
use crate::stored_model_document_export::{bounded_diagnostics, serialize_outcome, shortened_refusal, SentOutcome, StoreLibraryReads};
use crate::stored_read::{self, PoolReadMemory, ReadMemory, ReadReservation};
use open_pipe_stress_caepipe_mbf as mbf;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

/// The governed local-first route of this save (A-4).
pub const MBF_EXPORT_ROUTE_ID: &str = "DOTH-MBF-012";
/// The future native command's name (slice 4b registers it).
pub const COMMAND: &str = "save_mbf_export";

/// The model batch file: `.mbf`, under its own codes and temporary files.
pub const MBF_FORMAT: AtomicSaveFormat =
    AtomicSaveFormat { extension: "mbf", code_prefix: "MBF-EXPORT-SAVE", temp_prefix: ".swbpipe-mbf-", log_label: "mbf-export" };
/// The identity file `<stem>.swbpipe-ids.json` (the writer sidecar).
pub const MBF_IDS_FORMAT: AtomicSaveFormat = AtomicSaveFormat {
    extension: "json",
    code_prefix: "MBF-EXPORT-IDS-SAVE",
    temp_prefix: ".swbpipe-mbf-ids-",
    log_label: "mbf-identity-file",
};
/// What native code appends to the chosen MBF's stem to name the identity file beside it (A-5).
pub const IDS_SUFFIX: &str = ".swbpipe-ids.json";
/// The hidden backup of a file the save replaces: `.swbpipe-mbf-<32 hex>.bak`, in the same folder.
pub const BACKUP_PREFIX: &str = ".swbpipe-mbf-";
pub const BACKUP_SUFFIX: &str = ".bak";

pub const SETTINGS_INVALID: &str = mbf::EXPORT_SETTINGS_INVALID;
pub const MODEL_JSON_INVALID: &str = "MBF-EXPORT-MODEL-JSON-INVALID";
pub const ENGINE_FAILED: &str = "MBF-ENGINE-FAILED";
pub const REVIEW_STALE: &str = "MBF-EXPORT-REVIEW-STALE";
pub const SUGGESTED_NAME_INVALID: &str = "MBF-EXPORT-SUGGESTED-NAME-INVALID";
pub const PICKER_PATH_INVALID: &str = "MBF-EXPORT-PICKER-PATH-INVALID";
pub const DESTINATION_EXTENSION: &str = "MBF-EXPORT-DESTINATION-EXTENSION";
pub const READBACK_MISMATCH: &str = "MBF-EXPORT-READBACK-MISMATCH";
pub const READBACK_FAILED: &str = "MBF-EXPORT-READBACK-FAILED";
pub const MEMORY_UNAVAILABLE: &str = "MBF-EXPORT-MEMORY-UNAVAILABLE";
/// The production entry point's refusal until slice 4b's capped worker path is verified (§3.7).
pub const NOT_CONNECTED: &str = "MBF-EXPORT-NOT-CONNECTED";
/// An identity file of the chosen stem exists while its MBF does not: it may belong to an MBF that was moved, so it
/// is not replaced.
pub const IDENTITY_FILE_EXISTS: &str = "MBF-EXPORT-IDENTITY-FILE-EXISTS";
/// A file to be replaced could not be backed up by a hard link in its folder (FAT, exFAT, some network volumes), so
/// nothing is written: no replacement is ever made without a backup.
pub const REPLACE_BACKUP_UNAVAILABLE: &str = "MBF-EXPORT-REPLACE-BACKUP-UNAVAILABLE";
/// A failed save could not put a replaced file back; its backup is kept and named.
pub const ROLLBACK_INCOMPLETE: &str = "MBF-EXPORT-ROLLBACK-INCOMPLETE";
/// A backup that is no longer needed could not be removed (a warning; the save itself completed or was undone).
pub const BACKUP_CLEANUP_FAILED: &str = "MBF-EXPORT-BACKUP-CLEANUP-FAILED";
/// A written file was left in place: it is no longer the file this save wrote, or it could not be inspected.
pub const CLEANUP_SKIPPED: &str = "MBF-EXPORT-CLEANUP-SKIPPED";
pub const CLEANUP_FAILED: &str = "MBF-EXPORT-CLEANUP-FAILED";

/// The read-back block (§3.7).
pub const READBACK_BLOCK_BYTES: usize = 64 * 1024;
/// The settings text's parse and mapping, at most [`mbf::MAX_EXPORT_SETTINGS_JSON_BYTES`] of text
/// (`the_settings_parse_stays_within_its_reservation`).
pub const SETTINGS_WORKING_BYTES: u64 = 1 << 20;
/// What a Saved receipt adds to its warnings' JSON: two file names (each at most 255 bytes escaped to at most six),
/// byte counts, digests, the durability and the writer's limitation statement, and up to two backup-cleanup warnings
/// ([`DIAGNOSTIC_JSON_BYTES`] each).
pub const SAVED_FIELDS_BYTES: u64 = 8192 + 2 * DIAGNOSTIC_JSON_BYTES;
/// One diagnostic's JSON as a response bounds it: its code, and its path and message cut to
/// `MAX_DIAGNOSTIC_TEXT_BYTES` each, escaped to at most six bytes per byte.
const DIAGNOSTIC_JSON_BYTES: u64 = 34 + 64 + 2 * 6 * crate::stored_model_document_export::MAX_DIAGNOSTIC_TEXT_BYTES as u64;
/// The largest refusal after a write: two read-back problems (or one write failure) and one rollback diagnostic for
/// each destination. It is reserved with the receipt, so a rollback's named backups always reach the user.
pub const AFTER_WRITE_REFUSAL_BYTES: u64 = 4 * DIAGNOSTIC_JSON_BYTES + 256;
/// The most bytes of a suggested name.
const MAX_SUGGESTED_NAME_BYTES: usize = 200;

/// The test seam of the in-process writer run ([`InProcessWriterRun`]). Its only constructor is compiled for tests, so
/// no production code can run the writer in process or touch a file through this module: the production entry points
/// are [`save_mbf_export_admitted`] and [`review_mbf_export_admitted`], which refuse. The writer's memory is not governed
/// in process (§3.7); slice 4b.2 runs task 3 in the capped worker process.
pub(crate) struct InProcessWriterTestSeam {
    _private: (),
}

impl InProcessWriterTestSeam {
    #[cfg(test)]
    pub(crate) fn for_tests() -> Self {
        Self { _private: () }
    }
}

/// The digests the webview reviewed (lowercase hex SHA-256 of the MBF text and of the identity file's bytes).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MbfReviewed {
    pub mbf_sha256: String,
    pub sidecar_sha256: String,
}

/// `{canonicalText, claimedModelHash, settingsJson, suggestedName, reviewed, screening, localFirstEvidence}` (§3.6;
/// the route evidence keeps the member name of the sibling saves).
#[derive(Debug)]
pub struct MbfExportRequest {
    pub canonical_text: String,
    pub claimed_model_hash: String,
    /// The settings' JSON text, `{title, referenceTemperature: {value, unit}, maxLayoutRows: null}`: the text the
    /// webview reviewed with, read by the engine's one mapping ([`writer_settings`]).
    pub settings_json: String,
    pub suggested_name: String,
    pub reviewed: MbfReviewed,
    pub screening: ModelDocumentSaveScreening,
    pub local_first: ModelDocumentSaveLocalFirst,
}

/// Decodes the command's raw-body arguments (strings and structs of strings and scalars, moved), for the
/// command slice 4b registers.
pub(crate) fn save_mbf_export_args(args: &mut Args) -> Result<MbfExportRequest, SolveJobError> {
    Ok(MbfExportRequest {
        canonical_text: args.decode("canonicalText")?,
        claimed_model_hash: args.decode("claimedModelHash")?,
        settings_json: args.decode("settingsJson")?,
        suggested_name: args.decode("suggestedName")?,
        reviewed: args.decode("reviewed")?,
        screening: args.decode("screening")?,
        local_first: args.decode("localFirstEvidence")?,
    })
}

/// One file the save wrote.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct MbfSavedFile {
    pub file_name: String,
    pub byte_count: u64,
    /// Lowercase hex SHA-256 of the bytes written and read back.
    pub sha256: String,
    pub replaced_existing: bool,
    /// `durable`, or `saved_durability_uncertain` when the directory sync after the rename did not complete.
    pub durability: String,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum MbfExportOutcome {
    /// The user dismissed the chooser. Not an error; nothing was written.
    Cancelled,
    Saved {
        mbf: MbfSavedFile,
        identity_file: MbfSavedFile,
        /// `durable` when both files are, else `saved_durability_uncertain`.
        durability: String,
        /// The atomic writer's statement of what its path checks do not cover.
        path_containment: String,
        limitation: String,
        /// Non-blocking findings, e.g. `MODEL-LIBRARY-REF-UNRESOLVED`.
        warnings: Vec<Diagnostic>,
    },
    Refused {
        diagnostics: Vec<Diagnostic>,
        /// A refusal before the chooser from the save's own worker run (the writer refused, the review is truncated,
        /// or the result is above the output ceiling) carries the bounded review, and nothing of the writer's own.
        #[serde(skip_serializing_if = "Option::is_none")]
        review: Option<MbfReviewText>,
    },
}

impl MbfExportOutcome {
    pub fn refused(diagnostics: Vec<Diagnostic>) -> Self {
        Self::Refused { diagnostics, review: None }
    }

    pub fn refused_one(code: &str, message: impl Into<String>) -> Self {
        Self::refused(vec![diagnostic(code, "", message)])
    }
}

impl SentOutcome for MbfExportOutcome {
    fn bounded(self) -> (Self, usize) {
        match self {
            MbfExportOutcome::Saved { mbf, identity_file, durability, path_containment, limitation, warnings } => {
                let listed = warnings.len();
                let warnings = bounded_diagnostics(warnings);
                (MbfExportOutcome::Saved { mbf, identity_file, durability, path_containment, limitation, warnings }, listed)
            }
            MbfExportOutcome::Refused { diagnostics, review } => {
                let listed = diagnostics.len();
                (MbfExportOutcome::Refused { diagnostics: bounded_diagnostics(diagnostics), review }, listed)
            }
            MbfExportOutcome::Cancelled => (MbfExportOutcome::Cancelled, 0),
        }
    }

    fn shortened(self, refusal: String, noun: &str, listed: usize) -> Self {
        match self {
            MbfExportOutcome::Saved { mbf, identity_file, durability, path_containment, limitation, .. } => {
                let warnings = vec![diagnostic(
                    MEMORY_UNAVAILABLE,
                    "",
                    format!("{} Both files were saved; their {listed} warnings are not shown.", stored_read::capped(refusal)),
                )];
                MbfExportOutcome::Saved { mbf, identity_file, durability, path_containment, limitation, warnings }
            }
            MbfExportOutcome::Refused { diagnostics, .. } => {
                MbfExportOutcome::refused_one(MEMORY_UNAVAILABLE, shortened_refusal(refusal, noun, listed, &diagnostics))
            }
            MbfExportOutcome::Cancelled => MbfExportOutcome::Cancelled,
        }
    }
}

/// Lowercase hex SHA-256.
pub fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes).iter().map(|byte| format!("{byte:02x}")).collect()
}

fn is_sha256_hex(text: &str) -> bool {
    text.len() == 64 && text.bytes().all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
}

/// The dialog's settings text as the writer's arguments, by the engine's one mapping (`export_settings_from_json`,
/// which the webview wrapper calls on the same text): exactly `{title, referenceTemperature: {value, unit},
/// maxLayoutRows: null}`; a title of at most 4 KiB of UTF-8, as given; `degC` as `reference_temperature_c`, `degF` as
/// `reference_temperature_f` (exact, as the engine's reader reads the number); no row limit (Q-4). The recorded
/// digests (`fixtures/caepipe_mbf/invented/desktop_export/export_digests.json`) hold the two to the same result.
pub fn writer_settings(settings_json: &str) -> Result<mbf::WriterSettings, Diagnostic> {
    mbf::export_settings_from_json(settings_json.as_bytes()).map_err(|error| diagnostic(SETTINGS_INVALID, "settings", error.message))
}

/// The files a save writes: views into the worker's result, with the digests computed from them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MbfFiles<'a> {
    pub mbf: &'a [u8],
    pub mbf_sha256: String,
    pub sidecar: &'a [u8],
    pub sidecar_sha256: String,
}

// -- Worker task 3 (slice 4b.1: built and tested through the test seam; 4b.2 runs it in the capped worker) ---------

/// Task 3's two modes (§3.9.5): `review` returns only the bounded review; `save` returns the MBF, the identity file
/// and the bounded review, or on a refusal only the review.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MbfMode {
    Review,
    Save,
}

impl MbfMode {
    /// The request's `u32` mode field.
    pub const fn code(self) -> u32 {
        match self {
            MbfMode::Review => 1,
            MbfMode::Save => 2,
        }
    }

    /// The result envelope's mode line.
    pub const fn token(self) -> &'static str {
        match self {
            MbfMode::Review => "review",
            MbfMode::Save => "save",
        }
    }
}

/// The result envelope's status line. `review` mode returns only [`WorkerStatus::Reviewed`]; `save` mode returns
/// [`WorkerStatus::Written`] with the files, or one of the three refusals with only the review.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkerStatus {
    Reviewed,
    Written,
    /// The writer refused the model, or an entry is not written.
    WriterRefused,
    /// The review is truncated ([`REVIEW_TRUNCATED`]), whatever its blocking count.
    ReviewTruncated,
    /// The save result would exceed the output ceiling ([`OUTPUT_TOO_LARGE`]).
    OutputTooLarge,
}

impl WorkerStatus {
    const ALL: [WorkerStatus; 5] =
        [WorkerStatus::Reviewed, WorkerStatus::Written, WorkerStatus::WriterRefused, WorkerStatus::ReviewTruncated, WorkerStatus::OutputTooLarge];

    pub const fn token(self) -> &'static str {
        match self {
            WorkerStatus::Reviewed => "reviewed",
            WorkerStatus::Written => "written",
            WorkerStatus::WriterRefused => "writer-refused",
            WorkerStatus::ReviewTruncated => "review-truncated",
            WorkerStatus::OutputTooLarge => "output-too-large",
        }
    }

    const fn mode(self) -> MbfMode {
        match self {
            WorkerStatus::Reviewed => MbfMode::Review,
            _ => MbfMode::Save,
        }
    }
}

/// A request task 3 refuses: a bad mode, length or field.
pub const WORKER_REQUEST_INVALID: &str = "MBF-WORKER-REQUEST-INVALID";
/// A result the parent refuses: a malformed envelope, or one whose shape does not match the requested mode.
pub const WORKER_PROTOCOL: &str = "MBF-EXPORT-WORKER-PROTOCOL";
/// The worker run was cancelled (4b.2: a newer review, the dialog closed, the session changed).
pub const CANCELLED: &str = "MBF-EXPORT-CANCELLED";
/// The writer refused the model: the reply carries the bounded review, which lists what is not written.
pub const WRITER_REFUSED: &str = "MBF-EXPORT-WRITER-REFUSED";
/// Any truncated review blocks the save (§3.3).
pub const REVIEW_TRUNCATED: &str = mbf::REVIEW_TRUNCATED;
/// A save result above [`mbf::MBF_EXPORT_OUTPUT_MAX_BYTES`] (§3.7).
pub const OUTPUT_TOO_LARGE: &str = mbf::EXPORT_OUTPUT_TOO_LARGE;
/// The review's worker run was refused memory, or its result exceeded R and the frame (§3.9.5).
pub const REVIEW_MEMORY_UNAVAILABLE: &str = "MBF-REVIEW-MEMORY-UNAVAILABLE";
/// The envelope's first line.
pub const ENVELOPE_MAGIC: &str = "MBF1";
/// The most bytes of `worker_render`'s failure message.
pub const MAX_WORKER_FAILURE_BYTES: usize = 64 * 1024;
/// The request's fixed header: `u32 mode ‖ u32 settings_len`, little-endian.
pub const WORKER_REQUEST_HEADER_BYTES: usize = 8;
/// The longest status or mode line, and the longest length line (20 digits).
const MAX_ENVELOPE_LINE_BYTES: usize = 20;

/// Task 3's request: `u32 mode ‖ u32 settings_len ‖ settings JSON (at most 25 KiB) ‖ model canonical text`, the two
/// integers little-endian. Both modes carry exactly these fields; the model text runs to the end.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorkerRequest<'a> {
    pub mode: MbfMode,
    pub settings_json: &'a str,
    pub model_text: &'a str,
}

impl WorkerRequest<'_> {
    /// The request's bytes. In 4b.2 the parent writes the same layout to the worker.
    pub fn encode(&self) -> Vec<u8> {
        let mut raw = Vec::with_capacity(WORKER_REQUEST_HEADER_BYTES + self.settings_json.len() + self.model_text.len());
        raw.extend_from_slice(&self.mode.code().to_le_bytes());
        raw.extend_from_slice(&(self.settings_json.len() as u32).to_le_bytes());
        raw.extend_from_slice(self.settings_json.as_bytes());
        raw.extend_from_slice(self.model_text.as_bytes());
        raw
    }
}

/// A named, bounded failure message: `<code>: <message>`, never the engine's message text.
fn worker_failure(code: &str, message: impl std::fmt::Display) -> String {
    let mut text = format!("{code}: {message}");
    if text.len() > MAX_WORKER_FAILURE_BYTES {
        let mut cut = MAX_WORKER_FAILURE_BYTES;
        while !text.is_char_boundary(cut) {
            cut -= 1;
        }
        text.truncate(cut);
    }
    text
}

fn request_invalid(detail: impl std::fmt::Display) -> String {
    worker_failure(WORKER_REQUEST_INVALID, format_args!("The MBF worker request is malformed: {detail}."))
}

/// Reads task 3's request with its strict invariants: a mode of 1 (`review`) or 2 (`save`), a settings length within
/// [`mbf::MAX_EXPORT_SETTINGS_JSON_BYTES`] and the request, both fields UTF-8 and a model text that is not empty.
/// Views into `raw`; nothing is copied.
pub fn decode_worker_request(raw: &[u8]) -> Result<WorkerRequest<'_>, String> {
    if raw.len() < WORKER_REQUEST_HEADER_BYTES {
        return Err(request_invalid(format_args!("it is {} bytes, shorter than its {WORKER_REQUEST_HEADER_BYTES}-byte header", raw.len())));
    }
    let field = |at: usize| u32::from_le_bytes([raw[at], raw[at + 1], raw[at + 2], raw[at + 3]]);
    let mode = match field(0) {
        1 => MbfMode::Review,
        2 => MbfMode::Save,
        other => return Err(request_invalid(format_args!("mode {other} is neither review (1) nor save (2)"))),
    };
    let settings_len = field(4) as usize;
    if settings_len > mbf::MAX_EXPORT_SETTINGS_JSON_BYTES {
        return Err(request_invalid(format_args!("the settings are {settings_len} bytes, more than {}", mbf::MAX_EXPORT_SETTINGS_JSON_BYTES)));
    }
    let body = &raw[WORKER_REQUEST_HEADER_BYTES..];
    if settings_len > body.len() {
        return Err(request_invalid(format_args!("the settings length {settings_len} runs past the request's {} body bytes", body.len())));
    }
    let (settings, model) = body.split_at(settings_len);
    let Ok(settings_json) = std::str::from_utf8(settings) else { return Err(request_invalid("the settings are not UTF-8")) };
    let Ok(model_text) = std::str::from_utf8(model) else { return Err(request_invalid("the model text is not UTF-8")) };
    if model_text.is_empty() {
        return Err(request_invalid("the model text is missing"));
    }
    Ok(WorkerRequest { mode, settings_json, model_text })
}

/// The bounds task 3 runs under: R for the review and the output ceiling for a save result. Only the test seam runs
/// it under smaller ones.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct RenderLimits {
    review_allowance: usize,
    output_ceiling: u64,
}

impl RenderLimits {
    pub(crate) const PRODUCTION: RenderLimits =
        RenderLimits { review_allowance: mbf::MBF_REVIEW_MAX_BYTES, output_ceiling: mbf::MBF_EXPORT_OUTPUT_MAX_BYTES };

    #[cfg(test)]
    pub(crate) fn for_tests(review_allowance: usize, output_ceiling: u64) -> Self {
        RenderLimits { review_allowance, output_ceiling }
    }
}

fn decimal_digits(value: u64) -> u64 {
    value.checked_ilog10().map_or(1, |digits| u64::from(digits) + 1)
}

/// The envelope's exact length: `MBF1\n<mode>\n<status>\n<mbf_len>\n<ids_len>\n<review_len>\n` and the bodies.
pub fn envelope_length(mode: MbfMode, status: WorkerStatus, mbf_len: u64, ids_len: u64, review_len: u64) -> u64 {
    let header = ENVELOPE_MAGIC.len() as u64 + mode.token().len() as u64 + status.token().len() as u64 + 6
        + decimal_digits(mbf_len)
        + decimal_digits(ids_len)
        + decimal_digits(review_len);
    header.saturating_add(mbf_len).saturating_add(ids_len).saturating_add(review_len)
}

/// The envelope, built only after its exact length is known to be within `ceiling`.
fn envelope(mode: MbfMode, status: WorkerStatus, mbf_text: &str, ids: &str, review: &str, ceiling: u64) -> Result<String, String> {
    let length = envelope_length(mode, status, mbf_text.len() as u64, ids.len() as u64, review.len() as u64);
    if length > ceiling {
        return Err(worker_failure(OUTPUT_TOO_LARGE, format_args!("the result would be {length} bytes, above the {ceiling} it may hold")));
    }
    let mut out = String::with_capacity(length as usize);
    for line in [ENVELOPE_MAGIC, mode.token(), status.token()] {
        out.push_str(line);
        out.push('\n');
    }
    for part in [mbf_text.len(), ids.len(), review.len()] {
        out.push_str(&part.to_string());
        out.push('\n');
    }
    out.push_str(mbf_text);
    out.push_str(ids);
    out.push_str(review);
    Ok(out)
}

/// Worker task 3 (§3.9.5): reads the request, runs the writer over the model with the settings (both read as the
/// engine reads JSON, by its one settings mapping), forms the bounded review and returns the result envelope
/// (`MBF1\n<mode>\n<status>\n<mbf_len>\n<ids_len>\n<review_len>\n` and the bodies, lengths in decimal bytes).
///
/// - `review` mode returns only the review (status `reviewed`): no MBF or identity-file bytes.
/// - `save` mode forms the same review and refuses, with only the review, when the writer refused
///   (`writer-refused`), when the review is truncated (`review-truncated`, whatever its blocking count) or when the
///   save result would exceed the output ceiling (`output-too-large`, decided from the computed lengths before the
///   result is built). Otherwise it returns the MBF text, the identity file and the review (`written`).
///
/// The writer's diagnostics and messages never leave this function. `Err` is a bounded failure message
/// (`<code>: <message>`, at most [`MAX_WORKER_FAILURE_BYTES`]) with no engine message text. The old worker task codes
/// are unchanged; 4b.2 runs this as task 3, after an `ATST` status-0 frame, under the worker's grant.
pub fn worker_render(raw: &[u8]) -> Result<String, String> {
    worker_render_limited(raw, RenderLimits::PRODUCTION)
}

pub(crate) fn worker_render_limited(raw: &[u8], limits: RenderLimits) -> Result<String, String> {
    let request = decode_worker_request(raw)?;
    let settings = mbf::export_settings_from_json(request.settings_json.as_bytes())
        .map_err(|_| worker_failure(SETTINGS_INVALID, "The export settings are not {title, referenceTemperature: {value, unit}, maxLayoutRows: null}."))?;
    let model = mbf::parse_sidecar_json(request.model_text.as_bytes())
        .map_err(|_| worker_failure(MODEL_JSON_INVALID, "The model text is not JSON the MBF engine reads."))?;
    // The engine's error is named by its kind only: its message is the engine's text.
    let engine_failed = |error: mbf::ImportError| {
        let kind = match &error {
            mbf::ImportError::Python(error) => error.kind,
            mbf::ImportError::Unsupported(_) => "unsupported input",
        };
        worker_failure(ENGINE_FAILED, format_args!("The MBF engine returned no result ({kind})."))
    };
    let mut result = mbf::render_si(&model, &settings).map_err(engine_failed)?;
    mbf::value::dispose(model);
    drop(settings);
    let text = result.text.take().filter(|_| result.status == "written");
    let written = match text {
        Some(text) => {
            let mbf_sha256 = sha256_hex(text.as_bytes());
            let sidecar = result.sidecar_bytes(&mbf_sha256).map_err(engine_failed)?;
            let Ok(sidecar) = String::from_utf8(sidecar) else {
                return Err(worker_failure(ENGINE_FAILED, "The identity file is not UTF-8."));
            };
            let sidecar_sha256 = sha256_hex(sidecar.as_bytes());
            Some((text, mbf_sha256, sidecar, sidecar_sha256))
        }
        None => None,
    };
    let files = written.as_ref().map(|(text, mbf_sha256, sidecar, sidecar_sha256)| mbf::ReviewFiles {
        mbf_sha256,
        sidecar_sha256,
        mbf_bytes: text.len() as u64,
        sidecar_bytes: sidecar.len() as u64,
    });
    let (mbf_len, ids_len) = files.map_or((0, 0), |files| (files.mbf_bytes, files.sidecar_bytes));
    // Whether the save result (the files, this review and the framing) would be within the output ceiling.
    let fits = |review_len: u64| envelope_length(MbfMode::Save, WorkerStatus::Written, mbf_len, ids_len, review_len) <= limits.output_ceiling;
    let review = mbf::export_review(&result, files, limits.review_allowance, fits)
        .map_err(|error| worker_failure(ENGINE_FAILED, format_args!("The review's fixed fields need {} bytes, more than {}.", error.needed, error.allowance)))?;
    drop(result);
    let summary = &review.summary;
    let status = match request.mode {
        MbfMode::Review => WorkerStatus::Reviewed,
        MbfMode::Save if !summary.written || summary.blocking => WorkerStatus::WriterRefused,
        MbfMode::Save if !summary.complete => WorkerStatus::ReviewTruncated,
        MbfMode::Save if !summary.output_within_ceiling => WorkerStatus::OutputTooLarge,
        MbfMode::Save => WorkerStatus::Written,
    };
    match (status, &written) {
        (WorkerStatus::Written, Some((text, _, sidecar, _))) => envelope(request.mode, status, text, sidecar, &review.text, limits.output_ceiling),
        // Only the review: the MBF and identity-file bytes do not leave the worker.
        _ => envelope(request.mode, status, "", "", &review.text, limits.output_ceiling),
    }
}

/// What the parent reads from the review it is given (the rest of the review is passed on as it is).
#[derive(Debug, Deserialize)]
struct ReviewHead {
    document_kind: String,
    review_format: u64,
    status: String,
    blocking: bool,
    completeness: String,
    output_ceiling: String,
    mbf_sha256: Option<String>,
    sidecar_sha256: Option<String>,
    mbf_bytes: u64,
    sidecar_bytes: u64,
}

/// A decoded result: views into the result text.
#[derive(Debug)]
pub struct DecodedResult<'a> {
    pub status: WorkerStatus,
    pub mbf: &'a str,
    pub identity_file: &'a str,
    /// The bounded review's JSON text.
    pub review: &'a str,
    /// For `written`: the SHA-256 of the MBF and identity-file bytes, computed here and equal to the review's.
    pub mbf_sha256: Option<String>,
    pub sidecar_sha256: Option<String>,
}

fn protocol(detail: impl std::fmt::Display) -> Diagnostic {
    diagnostic(WORKER_PROTOCOL, "", format!("The MBF worker's result is malformed ({detail}); nothing was written."))
}

/// One envelope line of at most [`MAX_ENVELOPE_LINE_BYTES`], scanned no further.
fn envelope_line<'a>(rest: &mut &'a str) -> Option<&'a str> {
    let at = rest.as_bytes().iter().take(MAX_ENVELOPE_LINE_BYTES + 1).position(|byte| *byte == b'\n')?;
    let line = &rest[..at];
    *rest = &rest[at + 1..];
    Some(line)
}

/// A decimal length: digits only, no leading zero but `0` itself.
fn envelope_length_line(line: &str) -> Option<u64> {
    let canonical = !line.is_empty() && line.bytes().all(|byte| byte.is_ascii_digit()) && (line == "0" || !line.starts_with('0'));
    canonical.then(|| line.parse().ok()).flatten()
}

/// The parent's reading of task 3's result for the requested `mode`: the header, the lengths (exactly the bodies,
/// on character boundaries, within the output ceiling and the review within R), the mode, a status of that mode
/// with its fixed shape (a `review` result carries no MBF or identity bytes; a save refusal only the review), and
/// the review's agreement with the status. A save may continue only from `written`, whose review is complete,
/// within the ceiling, not blocking and names the digests and lengths of exactly the bytes carried.
pub fn decode_worker_result(mode: MbfMode, result: &str) -> Result<DecodedResult<'_>, Diagnostic> {
    if result.len() as u64 > mbf::MBF_EXPORT_OUTPUT_MAX_BYTES {
        return Err(protocol(format_args!("{} bytes, above the {} a result may hold", result.len(), mbf::MBF_EXPORT_OUTPUT_MAX_BYTES)));
    }
    let mut rest = result;
    let mut line = || envelope_line(&mut rest).ok_or_else(|| protocol("the header is incomplete"));
    if line()? != ENVELOPE_MAGIC {
        return Err(protocol("it is not an MBF1 envelope"));
    }
    let found_mode = line()?;
    if found_mode != mode.token() {
        return Err(protocol(format_args!("a {} result for a {} request", stored_read::shown(found_mode), mode.token())));
    }
    let token = line()?;
    let Some(status) = WorkerStatus::ALL.into_iter().find(|status| status.token() == token) else {
        return Err(protocol(format_args!("the status `{}` is unknown", stored_read::shown(token))));
    };
    if status.mode() != mode {
        return Err(protocol(format_args!("the status {} belongs to the other mode", status.token())));
    }
    let mut length = || line().and_then(|line| envelope_length_line(line).ok_or_else(|| protocol("a length is not a decimal byte count")));
    let (mbf_len, ids_len, review_len) = (length()?, length()?, length()?);
    let bodies = mbf_len.checked_add(ids_len).and_then(|sum| sum.checked_add(review_len));
    if bodies != Some(rest.len() as u64) {
        return Err(protocol(format_args!("its lengths do not add up to its {} body bytes", rest.len())));
    }
    if review_len == 0 || review_len > mbf::MBF_REVIEW_MAX_BYTES as u64 {
        return Err(protocol(format_args!("a review of {review_len} bytes, outside 1 to {}", mbf::MBF_REVIEW_MAX_BYTES)));
    }
    let carries_files = status == WorkerStatus::Written;
    if carries_files != (mbf_len > 0) || carries_files != (ids_len > 0) {
        return Err(protocol(format_args!("a {} result {} MBF or identity-file bytes", status.token(), if carries_files { "without" } else { "with" })));
    }
    let (mbf_len, ids_len) = (mbf_len as usize, ids_len as usize);
    let (Some(mbf_text), Some(identity_file), Some(review)) =
        (rest.get(..mbf_len), rest.get(mbf_len..mbf_len + ids_len), rest.get(mbf_len + ids_len..))
    else {
        return Err(protocol("a length ends inside a character"));
    };
    let head: ReviewHead = serde_json::from_str(review).map_err(|_| protocol("the review is not a review"))?;
    if head.document_kind != mbf::REVIEW_DOCUMENT_KIND || head.review_format != mbf::REVIEW_FORMAT {
        return Err(protocol("the review is not a review of this format"));
    }
    let complete = match &head.completeness[..] {
        "complete" => true,
        "truncated" => false,
        _ => return Err(protocol("the review's completeness is unknown")),
    };
    let within = match &head.output_ceiling[..] {
        "within" => true,
        "beyond" => false,
        _ => return Err(protocol("the review's output ceiling is unknown")),
    };
    let written = head.status == "written" && head.mbf_sha256.is_some() && head.sidecar_sha256.is_some();
    let agrees = match status {
        WorkerStatus::Reviewed => true,
        WorkerStatus::Written => written && !head.blocking && complete && within,
        WorkerStatus::WriterRefused => !written || head.blocking,
        WorkerStatus::ReviewTruncated => written && !head.blocking && !complete,
        WorkerStatus::OutputTooLarge => written && !head.blocking && complete && !within,
    };
    if !agrees {
        return Err(protocol(format_args!("its review does not support the status {}", status.token())));
    }
    let (mut mbf_sha256, mut sidecar_sha256) = (None, None);
    if carries_files {
        let (mbf_digest, ids_digest) = (sha256_hex(mbf_text.as_bytes()), sha256_hex(identity_file.as_bytes()));
        if head.mbf_sha256.as_deref() != Some(&mbf_digest)
            || head.sidecar_sha256.as_deref() != Some(&ids_digest)
            || head.mbf_bytes != mbf_len as u64
            || head.sidecar_bytes != ids_len as u64
        {
            return Err(protocol("its review names other files than the ones it carries"));
        }
        (mbf_sha256, sidecar_sha256) = (Some(mbf_digest), Some(ids_digest));
    }
    Ok(DecodedResult { status, mbf: mbf_text, identity_file, review, mbf_sha256, sidecar_sha256 })
}

/// Why a writer run returned no result. In process only [`WriterRunFailure::Failed`] occurs; the others are the
/// capped worker's (4b.2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum WriterRunFailure {
    /// The worker's bounded failure message (`worker_render`'s `Err`).
    Failed(String),
    /// The worker's grant or a host reservation was refused.
    Memory(String),
    /// The announced result exceeded the output ceiling (`result_limit`, before any result lease).
    OutputTooLarge,
    /// A malformed frame or a worker that ended without a result.
    Protocol(String),
    Cancelled,
}

/// How §3.6 step 5 and the review run task 3: the capped worker process (4b.2), or in process through the test seam.
pub(crate) trait MbfWriterRun {
    fn run(&mut self, request: &WorkerRequest<'_>) -> Result<String, WriterRunFailure>;
}

/// Task 3 in process. Its constructor needs the test seam, which only tests can construct: no production code runs
/// the writer in process (§3.7).
pub(crate) struct InProcessWriterRun<'s> {
    _seam: &'s InProcessWriterTestSeam,
    limits: RenderLimits,
}

impl<'s> InProcessWriterRun<'s> {
    pub(crate) fn new(seam: &'s InProcessWriterTestSeam, limits: RenderLimits) -> Self {
        InProcessWriterRun { _seam: seam, limits }
    }
}

impl MbfWriterRun for InProcessWriterRun<'_> {
    fn run(&mut self, request: &WorkerRequest<'_>) -> Result<String, WriterRunFailure> {
        worker_render_limited(&request.encode(), self.limits).map_err(WriterRunFailure::Failed)
    }
}

/// A run failure as a diagnostic: a worker failure by its own code (a malformed one is a protocol failure), a memory
/// refusal by `memory_code`.
fn run_failure(failure: WriterRunFailure, memory_code: &str, too_large_code: &str) -> Diagnostic {
    let named = |text: &str| {
        let (code, message) = text.split_once(": ")?;
        (!code.is_empty() && code.len() <= 64 && code.bytes().all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'-'))
            .then(|| diagnostic(code, "", format!("{message} Nothing was written.")))
    };
    match failure {
        WriterRunFailure::Failed(text) => named(&text).unwrap_or_else(|| protocol("its failure is not named")),
        WriterRunFailure::Memory(refusal) => diagnostic(memory_code, "", format!("{} Nothing was written.", stored_read::capped(refusal))),
        WriterRunFailure::OutputTooLarge => diagnostic(
            too_large_code,
            "",
            format!("The export result is larger than the {} bytes an export may hold. Nothing was written.", mbf::MBF_EXPORT_OUTPUT_MAX_BYTES),
        ),
        WriterRunFailure::Protocol(detail) => protocol(stored_read::capped(detail)),
        WriterRunFailure::Cancelled => diagnostic(CANCELLED, "", "The export was cancelled. Nothing was written."),
    }
}

/// The bounded review as a reply carries it, verbatim (at most R; JSON checked when it is made).
#[derive(Debug, Clone)]
pub struct MbfReviewText(Box<serde_json::value::RawValue>);

impl Serialize for MbfReviewText {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

impl MbfReviewText {
    fn new(review: &str) -> Option<Self> {
        serde_json::value::RawValue::from_string(review.to_owned()).ok().map(MbfReviewText)
    }

    pub fn get(&self) -> &str {
        self.0.get()
    }
}

impl PartialEq for MbfReviewText {
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}

/// The refusal before the chooser of a save whose worker run refused: its code and product text, and the bounded
/// review only (§3.6 step 5).
fn worker_refusal_message(status: WorkerStatus) -> (&'static str, &'static str) {
    match status {
        WorkerStatus::ReviewTruncated => (REVIEW_TRUNCATED, "The loss report is too large to show in full, so the export is not offered. Nothing was written."),
        WorkerStatus::OutputTooLarge => (
            OUTPUT_TOO_LARGE,
            "The model batch file, its identity file and the review together are larger than an export may write, so the export is not offered. Nothing was written.",
        ),
        _ => (WRITER_REFUSED, "SWBPIPE does not write the file: the review lists what is not written. Nothing was written."),
    }
}

/// Step 6: the files must be the reviewed ones.
pub fn check_reviewed(files: &MbfFiles<'_>, reviewed: &MbfReviewed) -> Result<(), Diagnostic> {
    if files.mbf_sha256 == reviewed.mbf_sha256 && files.sidecar_sha256 == reviewed.sidecar_sha256 {
        return Ok(());
    }
    let well_formed = is_sha256_hex(&reviewed.mbf_sha256) && is_sha256_hex(&reviewed.sidecar_sha256);
    Err(diagnostic(
        REVIEW_STALE,
        "reviewed",
        if well_formed {
            "The model or the export settings changed since the review: the file this save would write is not the one reviewed. Review the export again; nothing was written."
        } else {
            "The reviewed digests are not two lowercase hex SHA-256 values. Review the export again; nothing was written."
        },
    ))
}

/// Step 7: the suggested name is `swbpipe-model.mbf` or `swbpipe-model-<slug>.mbf` (DEL-07-01 `export-file-names.md`,
/// kind `model`): lowercase ASCII letters and digits in single-hyphen tokens.
pub fn valid_suggested_name(name: &str) -> bool {
    let Some(stem) = name.strip_suffix(".mbf") else { return false };
    let Some(rest) = stem.strip_prefix("swbpipe-model") else { return false };
    name.len() <= MAX_SUGGESTED_NAME_BYTES
        && (rest.is_empty()
            || rest.strip_prefix('-').is_some_and(|tokens| {
                tokens.split('-').all(|token| !token.is_empty() && token.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit()))
            }))
}

/// The identity file beside the chosen MBF: `<stem>.swbpipe-ids.json` (A-5), or the refusal of a chosen path that
/// is not an `.mbf` file name.
pub fn identity_file_path(chosen: &Path) -> Result<PathBuf, Diagnostic> {
    if chosen.extension().and_then(|item| item.to_str()) != Some("mbf") {
        return Err(diagnostic(DESTINATION_EXTENSION, "", "A model batch file must be saved with the .mbf extension; nothing was written."));
    }
    let stem = chosen.file_stem().and_then(|item| item.to_str()).filter(|stem| !stem.is_empty());
    match (stem, chosen.parent()) {
        (Some(stem), Some(parent)) => Ok(parent.join(format!("{stem}{IDS_SUFFIX}"))),
        _ => Err(diagnostic(PICKER_PATH_INVALID, "", "The chosen path has no usable file name; nothing was written.")),
    }
}

/// The file operations of steps 8 and 9: the system's, or a test's injected failures.
trait MbfExportIo {
    fn write(&mut self, format: &AtomicSaveFormat, destination: &Path, bytes: &[u8]) -> Result<AtomicSaveOutcome, AtomicSaveError> {
        atomic_report_package_save::atomic_save_bytes_as(format, destination, bytes)
    }
    fn open(&mut self, path: &Path) -> io::Result<File> {
        File::open(path)
    }
    fn remove(&mut self, path: &Path) -> io::Result<()> {
        fs::remove_file(path)
    }
    /// Makes the backup of a file about to be replaced.
    fn hard_link(&mut self, original: &Path, link: &Path) -> io::Result<()> {
        fs::hard_link(original, link)
    }
    /// Puts a backup back over its original's name.
    fn rename(&mut self, from: &Path, to: &Path) -> io::Result<()> {
        fs::rename(from, to)
    }
}

struct SystemMbfExportIo;

impl MbfExportIo for SystemMbfExportIo {}

/// Which file a path names, so a cleanup or a restore never touches a file the save did not write or back up: the
/// device and inode where the platform has them; elsewhere the size and both times, all of which must be present and
/// equal (a missing time is not identical).
#[derive(Debug, Clone, PartialEq, Eq)]
struct FileIdentity {
    #[cfg(unix)]
    device_inode: (u64, u64),
    #[cfg(not(unix))]
    size_created_modified: (u64, Option<std::time::SystemTime>, Option<std::time::SystemTime>),
}

impl FileIdentity {
    fn same_file(&self, other: &FileIdentity) -> bool {
        #[cfg(unix)]
        {
            self.device_inode == other.device_inode
        }
        #[cfg(not(unix))]
        {
            let (size, created, modified) = &self.size_created_modified;
            let (other_size, other_created, other_modified) = &other.size_created_modified;
            size == other_size
                && created.is_some()
                && modified.is_some()
                && created == other_created
                && modified == other_modified
        }
    }
}

fn file_identity(path: &Path) -> io::Result<FileIdentity> {
    let metadata = fs::symlink_metadata(path)?;
    if !metadata.file_type().is_file() {
        return Err(io::Error::other("not a regular file"));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        Ok(FileIdentity { device_inode: (metadata.dev(), metadata.ino()) })
    }
    #[cfg(not(unix))]
    {
        Ok(FileIdentity { size_created_modified: (metadata.len(), metadata.created().ok(), metadata.modified().ok()) })
    }
}

/// Whether `path` names the file `expected` identified, by [`FileIdentity::same_file`]; `Err` when it is absent
/// (`NotFound`) or cannot be inspected.
fn names_file(path: &Path, expected: &FileIdentity) -> io::Result<bool> {
    file_identity(path).map(|found| expected.same_file(&found))
}

/// A file this save wrote, and its identity just after the write.
struct WrittenFile {
    path: PathBuf,
    identity: Option<FileIdentity>,
    saved: MbfSavedFile,
}

fn basename(path: &Path) -> String {
    path.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default()
}

/// Removes a file this save wrote if it is still that file; otherwise leaves it and says why.
fn remove_written(io: &mut impl MbfExportIo, written: &WrittenFile) -> Option<Diagnostic> {
    let name = stored_read::shown(&written.saved.file_name).into_owned();
    let found = match &written.identity {
        Some(expected) => names_file(&written.path, expected),
        None => Ok(false),
    };
    match found {
        Ok(true) => match io.remove(&written.path) {
            Ok(()) => None,
            Err(error) => Some(diagnostic(CLEANUP_FAILED, "", format!("`{name}` could not be removed ({error}); delete it before using the folder's files."))),
        },
        Err(error) if error.kind() == io::ErrorKind::NotFound => None,
        _ => Some(diagnostic(
            CLEANUP_SKIPPED,
            "",
            format!("`{name}` was left in place: it is no longer the file this save wrote, or it could not be inspected."),
        )),
    }
}

/// Writes one file and records its identity.
fn write_one(io: &mut impl MbfExportIo, format: &AtomicSaveFormat, path: &Path, bytes: &[u8], sha256: &str) -> Result<WrittenFile, Diagnostic> {
    let outcome = io.write(format, path, bytes).map_err(writer_diagnostic)?;
    Ok(WrittenFile {
        path: path.to_path_buf(),
        identity: file_identity(path).ok(),
        saved: MbfSavedFile {
            file_name: outcome.selected_basename,
            byte_count: bytes.len() as u64,
            sha256: sha256.to_string(),
            replaced_existing: outcome.replaced_existing,
            durability: outcome.durability,
        },
    })
}

/// The SHA-256 of a file read in fixed blocks of `block`.
fn read_back(io: &mut impl MbfExportIo, path: &Path, block: &mut [u8]) -> io::Result<String> {
    let mut file = io.open(path)?;
    let mut hasher = Sha256::new();
    loop {
        let read = file.read(block)?;
        if read == 0 {
            break;
        }
        hasher.update(&block[..read]);
    }
    Ok(hasher.finalize().iter().map(|byte| format!("{byte:02x}")).collect())
}

/// The destinations' checks before any write (step 8): the identity file's path; both destinations by the atomic
/// writer's own checks (extension, parent, no symlink or directory, and off macOS no replacement); and no identity file
/// without its MBF. Returns the identity file's path and whether each destination holds a file to be replaced.
fn check_destinations(chosen: &Path) -> Result<(PathBuf, bool, bool), Diagnostic> {
    let ids_path = identity_file_path(chosen)?;
    let replaces_mbf = atomic_report_package_save::check_destination_as(&MBF_FORMAT, chosen).map_err(writer_diagnostic)?;
    if !replaces_mbf && fs::symlink_metadata(&ids_path).is_ok() {
        return Err(diagnostic(
            IDENTITY_FILE_EXISTS,
            "",
            format!(
                "`{}` already exists beside the chosen name, but `{}` does not. It may be the identity file of a model batch file that was moved, so it is not replaced. Choose another name, or move that file first; nothing was written.",
                stored_read::shown(&basename(&ids_path)),
                stored_read::shown(&basename(chosen)),
            ),
        ));
    }
    let replaces_ids = atomic_report_package_save::check_destination_as(&MBF_IDS_FORMAT, &ids_path).map_err(writer_diagnostic)?;
    Ok((ids_path, replaces_mbf, replaces_ids))
}

/// The hidden hard link that keeps a replaced file until the save completes or is undone.
struct Backup {
    original: PathBuf,
    path: PathBuf,
    identity: FileIdentity,
}

impl Backup {
    /// The original's name as a message quotes it, and the backup's whole (its own 49 ASCII characters, never cut).
    fn names(&self) -> (String, String) {
        (stored_read::shown(&basename(&self.original)).into_owned(), basename(&self.path))
    }
}

fn backup_unavailable(original: &Path, detail: impl std::fmt::Display) -> Diagnostic {
    diagnostic(
        REPLACE_BACKUP_UNAVAILABLE,
        "",
        format!(
            "`{}` would be replaced, but it could not be backed up in its folder first ({detail}); a file is never replaced without a backup. Choose a new name or another folder; nothing was written.",
            stored_read::shown(&basename(original))
        ),
    )
}

/// Backs up the regular file at `original` by a hard link to a fresh hidden name beside it, or refuses.
fn back_up(io: &mut impl MbfExportIo, original: &Path) -> Result<Backup, Diagnostic> {
    let identity = file_identity(original).map_err(|error| backup_unavailable(original, error))?;
    let parent = original.parent().ok_or_else(|| backup_unavailable(original, "no folder"))?;
    for _ in 0..128_u32 {
        let mut token = [0_u8; 16];
        getrandom::fill(&mut token).map_err(|error| backup_unavailable(original, error))?;
        let token: String = token.iter().map(|byte| format!("{byte:02x}")).collect();
        let path = parent.join(format!("{BACKUP_PREFIX}{token}{BACKUP_SUFFIX}"));
        match io.hard_link(original, &path) {
            Ok(()) => {}
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(backup_unavailable(original, error)),
        }
        return match names_file(&path, &identity) {
            Ok(true) => Ok(Backup { original: original.to_path_buf(), path, identity }),
            // The original changed between the inspection and the link: the link is this save's, and is removed.
            _ => {
                let _ = io.remove(&path);
                Err(backup_unavailable(original, "the file changed while it was backed up"))
            }
        };
    }
    Err(backup_unavailable(original, "no free backup name"))
}

/// Removes a backup no longer needed, if it is still that backup; otherwise says what is left and where.
fn discard_backup(io: &mut impl MbfExportIo, backup: &Backup) -> Option<Diagnostic> {
    let (original, kept) = backup.names();
    let result = match names_file(&backup.path, &backup.identity) {
        Ok(true) => io.remove(&backup.path).map_err(|error| error.to_string()),
        Ok(false) => Err("it is no longer the backup this save made".to_string()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error.to_string()),
    };
    result.err().map(|reason| {
        diagnostic(
            BACKUP_CLEANUP_FAILED,
            "",
            format!("Left in the same folder: `{kept}`, the backup of the previous `{original}` ({reason}). It is no longer needed and may be deleted."),
        )
    })
}

/// A rollback that could not put the previous file back: the backup is kept, and both names are quoted in full (a file
/// name is at most 255 bytes, so the message stays within the 2048 bytes a diagnostic's text is cut to), so the user
/// can always rename it back.
fn rollback_incomplete(backup: &Backup, reason: impl std::fmt::Display) -> Diagnostic {
    let (original, kept) = (basename(&backup.original), basename(&backup.path));
    diagnostic(
        ROLLBACK_INCOMPLETE,
        "",
        format!(
            "Kept as `{kept}` in the same folder: the previous `{original}`, unchanged, which could not be put back ({reason}). Rename `{kept}` to `{original}` to recover it."
        ),
    )
}

/// One destination of the pair: its path, the backup of the file it replaces, and what this save wrote there.
struct Destination {
    path: PathBuf,
    backup: Option<Backup>,
    written: Option<WrittenFile>,
}

/// Undoes the save at one destination after a failure: what this save wrote is removed, and the replaced file is put
/// back from its backup, each only after the identity check. A destination that cannot be restored keeps its backup
/// and is named ([`ROLLBACK_INCOMPLETE`]).
fn roll_back(io: &mut impl MbfExportIo, destination: &Destination) -> Vec<Diagnostic> {
    let Some(backup) = &destination.backup else {
        return destination.written.iter().filter_map(|written| remove_written(io, written)).collect();
    };
    match &destination.written {
        // This save's file replaced the original: put the backup back over it in one rename.
        Some(written) => {
            let ours = match &written.identity {
                Some(identity) => names_file(&destination.path, identity),
                None => Ok(false),
            };
            match ours {
                Ok(true) => {}
                Err(error) if error.kind() == io::ErrorKind::NotFound => {}
                _ => return vec![rollback_incomplete(backup, "the file now there is not the one this save wrote, so it was left in place")],
            }
            match names_file(&backup.path, &backup.identity) {
                Ok(true) => {}
                _ => return vec![rollback_incomplete(backup, "the backup is no longer the file this save kept")],
            }
            match io.rename(&backup.path, &destination.path) {
                Ok(()) => Vec::new(),
                Err(error) => vec![rollback_incomplete(backup, error)],
            }
        }
        // The write did not replace it: the original should still be there, and its backup is discarded.
        None => match names_file(&destination.path, &backup.identity) {
            Ok(true) => discard_backup(io, backup).into_iter().collect(),
            Err(error) if error.kind() == io::ErrorKind::NotFound => match io.rename(&backup.path, &destination.path) {
                Ok(()) => Vec::new(),
                Err(error) => vec![rollback_incomplete(backup, error)],
            },
            _ => vec![rollback_incomplete(backup, "another file is now at its name, so it was left in place")],
        },
    }
}

/// Refuses with `diagnostics` after undoing the save at every destination (the MBF first).
fn undone(io: &mut impl MbfExportIo, mut diagnostics: Vec<Diagnostic>, destinations: &[&Destination]) -> MbfExportOutcome {
    for destination in destinations {
        diagnostics.extend(roll_back(io, destination));
    }
    MbfExportOutcome::refused(diagnostics)
}

/// Steps 8 to 10 for the chosen MBF path. Every destination is checked, and every file to be replaced is backed up,
/// before the first write; after any later failure the save is undone, and on success the backups are removed.
fn write_pair(io: &mut impl MbfExportIo, chosen: &Path, files: &MbfFiles<'_>, block: &mut [u8], mut warnings: Vec<Diagnostic>) -> MbfExportOutcome {
    let (ids_path, replaces_mbf, replaces_ids) = match check_destinations(chosen) {
        Ok(checked) => checked,
        Err(refusal) => return MbfExportOutcome::refused(vec![refusal]),
    };
    let mut backups: Vec<Option<Backup>> = Vec::new();
    for (path, replaces) in [(ids_path.as_path(), replaces_ids), (chosen, replaces_mbf)] {
        if !replaces {
            backups.push(None);
            continue;
        }
        match back_up(io, path) {
            Ok(backup) => backups.push(Some(backup)),
            Err(refusal) => {
                let mut diagnostics = vec![refusal];
                diagnostics.extend(backups.iter().flatten().filter_map(|backup| discard_backup(io, backup)));
                return MbfExportOutcome::refused(diagnostics);
            }
        }
    }
    let mbf_backup = backups.pop().flatten();
    let ids_backup = backups.pop().flatten();
    let mut ids = Destination { path: ids_path, backup: ids_backup, written: None };
    let mut mbf = Destination { path: chosen.to_path_buf(), backup: mbf_backup, written: None };
    match write_one(io, &MBF_IDS_FORMAT, &ids.path, files.sidecar, &files.sidecar_sha256) {
        Ok(written) => ids.written = Some(written),
        Err(refusal) => return undone(io, vec![refusal], &[&mbf, &ids]),
    }
    match write_one(io, &MBF_FORMAT, &mbf.path, files.mbf, &files.mbf_sha256) {
        Ok(written) => mbf.written = Some(written),
        Err(refusal) => return undone(io, vec![refusal], &[&mbf, &ids]),
    }
    let mut problems = Vec::new();
    for written in [&ids, &mbf].into_iter().filter_map(|destination| destination.written.as_ref()) {
        match read_back(io, &written.path, block) {
            Ok(found) if found == written.saved.sha256 => {}
            Ok(found) => problems.push(diagnostic(
                READBACK_MISMATCH,
                "",
                format!(
                    "`{}` reads back with SHA-256 {found}, not the {} written; the save is undone.",
                    stored_read::shown(&written.saved.file_name),
                    written.saved.sha256
                ),
            )),
            Err(error) => problems.push(diagnostic(
                READBACK_FAILED,
                "",
                format!("`{}` could not be read back ({error}); the save is undone.", stored_read::shown(&written.saved.file_name)),
            )),
        }
    }
    if !problems.is_empty() {
        return undone(io, problems, &[&mbf, &ids]);
    }
    for backup in [&ids, &mbf].into_iter().filter_map(|destination| destination.backup.as_ref()) {
        warnings.extend(discard_backup(io, backup));
    }
    let (Some(ids), Some(mbf)) = (ids.written, mbf.written) else { unreachable!("both files were written") };
    let durable = ids.saved.durability == "durable" && mbf.saved.durability == "durable";
    MbfExportOutcome::Saved {
        identity_file: ids.saved,
        mbf: mbf.saved,
        durability: if durable { "durable" } else { "saved_durability_uncertain" }.to_string(),
        path_containment: atomic_report_package_save::PATH_CONTAINMENT.to_string(),
        limitation: atomic_report_package_save::TOCTOU_LIMITATION.to_string(),
        warnings,
    }
}

/// The library reads' and the screen's memory refusals under this command's own code.
fn remap_memory_code(diagnostics: &mut [Diagnostic]) {
    for item in diagnostics {
        if item.code == stored_read::STORE_READ_MEMORY_UNAVAILABLE {
            item.code = MEMORY_UNAVAILABLE.to_string();
        }
    }
}

fn memory_refusal(error: String) -> MbfExportOutcome {
    MbfExportOutcome::refused_one(MEMORY_UNAVAILABLE, format!("{error} Nothing was written."))
}

/// The admission's parse of the text and its canonical copy (step 2), reserved before they are made.
fn admission_bytes(text: &str) -> u64 {
    let length = text.len() as u64;
    let parse = crate::ingress::prescan(text.as_bytes()).map(|bound| bound.materialization_bytes).unwrap_or(0);
    parse.saturating_add(stored_read::text_copy_bytes(length))
}

/// What the save holds until it returns: every reservation, so the response is built within them.
#[derive(Default)]
struct Held {
    reservations: Vec<ReadReservation>,
    response: Option<(ReadReservation, u64)>,
}

/// Steps 1 to 10 within `memory`, the library store opened by `open_store` only if the document references a
/// library; `choose` is given the suggested name.
fn save_mbf_export_outcome(
    run: &mut impl MbfWriterRun,
    request: MbfExportRequest,
    memory: &PoolReadMemory<'_>,
    libraries: &mut StoreLibraryReads<'_>,
    choose: impl FnOnce(&str) -> Result<Option<PathBuf>, String>,
    io: &mut impl MbfExportIo,
    held: &mut Held,
) -> MbfExportOutcome {
    let MbfExportRequest { canonical_text, claimed_model_hash, settings_json, suggested_name, reviewed, screening, local_first } = request;
    // 1. Route evidence; its binding to this text's SHA-256 is checked with the hash (2).
    if let Err(refusal) = validate_local_private_route_evidence(MBF_EXPORT_ROUTE_ID, &screening, &local_first) {
        return MbfExportOutcome::refused(vec![refusal]);
    }
    // The settings, by the engine's one mapping, before any copy of the model text: a text or title over its bound is
    // refused before it is read.
    if let Err(refusal) = read_settings(memory, &settings_json) {
        return refusal.map_or_else(memory_refusal, |refusal| MbfExportOutcome::refused(vec![refusal]));
    }
    // 2-4. The open cap, the canonical text and hash, the open path's checks and the disposition screen.
    let parse = match memory.reserve("the parse and canonical copy of the model text", admission_bytes(&canonical_text)) {
        Ok(reservation) => reservation,
        Err(error) => return memory_refusal(error),
    };
    let reads = std::cell::RefCell::new(libraries);
    let admitted = admit_model_document_text_screened(
        canonical_text,
        &claimed_model_hash,
        &screening.payload_sha256,
        &mut |document: &serde_json::Value| reads.borrow_mut().screen_model(document),
        &mut |project_id: &str, kind: &str, id: &str| reads.borrow_mut().read(project_id, kind, id),
    );
    drop(parse);
    if let Some(mut refusal) = reads.into_inner().refusal() {
        remap_memory_code(&mut refusal);
        return MbfExportOutcome::refused(refusal);
    }
    let admitted = match admitted {
        Ok(admitted) => admitted,
        // The screen's own reservation refused inside the admission comes back here, by the pool's name.
        Err(mut diagnostics) => {
            remap_memory_code(&mut diagnostics);
            return MbfExportOutcome::refused(diagnostics);
        }
    };
    // 5. Task 3 in `save` mode, over the admitted text and the settings text (the worker reads both itself). Its
    // result is reserved before the admitted text is released; its MBF and identity-file bytes are views into it.
    let worker = WorkerRequest { mode: MbfMode::Save, settings_json: &settings_json, model_text: &admitted.text };
    let result = match run.run(&worker) {
        Ok(result) => result,
        Err(failure) => return MbfExportOutcome::refused(vec![run_failure(failure, MEMORY_UNAVAILABLE, OUTPUT_TOO_LARGE)]),
    };
    match memory.reserve("the MBF worker's save result", stored_read::exact_text_bytes(result.len() as u64)) {
        Ok(reservation) => held.reservations.push(reservation),
        Err(error) => return memory_refusal(error),
    }
    drop(admitted.text);
    drop(settings_json);
    let decoded = match decode_worker_result(MbfMode::Save, &result) {
        Ok(decoded) => decoded,
        Err(refusal) => return MbfExportOutcome::refused(vec![refusal]),
    };
    if decoded.status != WorkerStatus::Written {
        // Before any chooser: the code and the bounded review only.
        let (code, message) = worker_refusal_message(decoded.status);
        return match reply_review(memory, decoded.review, held) {
            Ok(review) => MbfExportOutcome::Refused { diagnostics: vec![diagnostic(code, "", message)], review: Some(review) },
            Err(refusal) => refusal.map_or_else(memory_refusal, |refusal| MbfExportOutcome::refused(vec![refusal])),
        };
    }
    let (Some(mbf_sha256), Some(sidecar_sha256)) = (decoded.mbf_sha256.clone(), decoded.sidecar_sha256.clone()) else {
        return MbfExportOutcome::refused(vec![protocol("a written result without digests")]);
    };
    let files = MbfFiles { mbf: decoded.mbf.as_bytes(), mbf_sha256, sidecar: decoded.identity_file.as_bytes(), sidecar_sha256 };
    // 6. Staleness.
    if let Err(refusal) = check_reviewed(&files, &reviewed) {
        return MbfExportOutcome::refused(vec![refusal]);
    }
    // The receipt's bound and the read-back block, reserved before the chooser: no response is refused after a
    // file was written.
    let bound = stored_read::serialized_length(&admitted.warnings)
        .unwrap_or(u64::MAX)
        .saturating_add(SAVED_FIELDS_BYTES)
        .max(AFTER_WRITE_REFUSAL_BYTES);
    match memory.reserve("the response to Export MBF", stored_read::exact_text_bytes(bound)) {
        Ok(reservation) => held.response = Some((reservation, bound)),
        Err(error) => return memory_refusal(error),
    }
    match memory.reserve("the read-back block", stored_read::exact_text_bytes(READBACK_BLOCK_BYTES as u64)) {
        Ok(reservation) => held.reservations.push(reservation),
        Err(error) => return memory_refusal(error),
    }
    // 7. The chooser.
    if !valid_suggested_name(&suggested_name) {
        return MbfExportOutcome::refused_one(
            SUGGESTED_NAME_INVALID,
            "The suggested name must be swbpipe-model.mbf or swbpipe-model-<label>.mbf in lowercase letters, digits and single hyphens; nothing was written.",
        );
    }
    let chosen = match choose(&suggested_name) {
        Ok(Some(chosen)) => chosen,
        Ok(None) => return MbfExportOutcome::Cancelled,
        Err(message) => return MbfExportOutcome::refused_one(PICKER_PATH_INVALID, message),
    };
    // 8-10.
    let mut block = vec![0_u8; READBACK_BLOCK_BYTES];
    write_pair(io, &chosen, &files, &mut block, admitted.warnings)
}

/// The settings text read by the engine's one mapping within its reservation: `Err(Ok(refusal))` refuses the settings
/// by name, `Err(Err(message))` is a refused reservation.
fn read_settings(memory: &PoolReadMemory<'_>, settings_json: &str) -> Result<(), Result<Diagnostic, String>> {
    let _parse = memory.reserve("the export settings' parse", SETTINGS_WORKING_BYTES).map_err(Err)?;
    writer_settings(settings_json).map(drop).map_err(Ok)
}

/// The reply's copy of the bounded review, reserved before it is made and held until the reply is sent.
fn reply_review(memory: &PoolReadMemory<'_>, review: &str, held: &mut Held) -> Result<MbfReviewText, Result<Diagnostic, String>> {
    let reservation = memory.reserve("the bounded MBF review of the reply", stored_read::exact_text_bytes(review.len() as u64)).map_err(Err)?;
    held.reservations.push(reservation);
    MbfReviewText::new(review).ok_or_else(|| Ok(protocol("the review is not JSON")))
}

/// The save within `memory` (the caller's: seeded with the command's ingress lease), with task 3 run by `run`, and its
/// response: `respond` receives the outcome and its text while every reservation is still held. Until 4b.2 only the
/// test seam's in-process run reaches it.
fn save_mbf_export_with<R>(
    run: &mut impl MbfWriterRun,
    request: MbfExportRequest,
    memory: &PoolReadMemory<'_>,
    open_store: impl FnMut() -> Result<rusqlite::Connection, String>,
    choose: impl FnOnce(&str) -> Result<Option<PathBuf>, String>,
    io: &mut impl MbfExportIo,
    respond: impl FnOnce(&MbfExportOutcome, String) -> R,
) -> R {
    let mut libraries = StoreLibraryReads::new(memory, open_store);
    let mut held = Held::default();
    let outcome = save_mbf_export_outcome(run, request, memory, &mut libraries, choose, io, &mut held);
    let (sent, text, buffer) = serialize_outcome(memory, "Export MBF", "save", outcome, held.response.take());
    let response = respond(&sent, text);
    drop(sent);
    drop(buffer);
    drop(held);
    drop(libraries);
    response
}

/// The production entry point, everything after the arguments are decoded (the command slice 4b registers): within
/// the pool of `registry`, started from the ingress lease `ingress`. Not connected: it refuses every request with
/// [`NOT_CONNECTED`] before any parse, writer run, store opening, chooser or file access, because the in-process
/// writer's memory is not governed (§3.7). Slice 4b connects it to the writer in the capped worker process.
pub(crate) fn save_mbf_export_admitted<R>(
    request: MbfExportRequest,
    _open_store: impl FnMut() -> Result<rusqlite::Connection, String>,
    registry: &SolveJobRegistry,
    ingress: &Lease,
    _choose: impl FnOnce(&str) -> Result<Option<PathBuf>, String>,
    respond: impl FnOnce(&MbfExportOutcome, String) -> R,
) -> R {
    drop(request);
    let memory = PoolReadMemory::new(registry, ingress);
    let outcome = MbfExportOutcome::refused_one(
        NOT_CONNECTED,
        "Export MBF is not connected yet: native export memory governance is unfinished. Nothing was read or written.",
    );
    let (sent, text, buffer) = serialize_outcome(&memory, "Export MBF", "save", outcome, None);
    let response = respond(&sent, text);
    drop(sent);
    drop(buffer);
    response
}

// -- review_mbf_export (§3.9.5; slice 4b.1: built, not registered) -------------------------------------------------

/// The future native review command's name (slice 4b.2 registers it).
pub const REVIEW_COMMAND: &str = "review_mbf_export";

/// `{canonicalText, claimedModelHash, settingsJson}` (§3.9.5): the same model, settings and binding a save would
/// write, with no route evidence.
#[derive(Debug)]
pub struct MbfReviewRequest {
    pub canonical_text: String,
    pub claimed_model_hash: String,
    pub settings_json: String,
}

/// Decodes the review command's raw-body arguments (moved), for the command slice 4b.2 registers.
pub(crate) fn review_mbf_export_args(args: &mut Args) -> Result<MbfReviewRequest, SolveJobError> {
    Ok(MbfReviewRequest {
        canonical_text: args.decode("canonicalText")?,
        claimed_model_hash: args.decode("claimedModelHash")?,
        settings_json: args.decode("settingsJson")?,
    })
}

/// The review command's outcome: the bounded review, or a refusal.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum MbfReviewOutcome {
    Reviewed { review: MbfReviewText },
    Refused { diagnostics: Vec<Diagnostic> },
}

impl MbfReviewOutcome {
    fn refused_one(code: &str, message: impl Into<String>) -> Self {
        MbfReviewOutcome::Refused { diagnostics: vec![diagnostic(code, "", message)] }
    }
}

impl SentOutcome for MbfReviewOutcome {
    fn bounded(self) -> (Self, usize) {
        match self {
            MbfReviewOutcome::Refused { diagnostics } => {
                let listed = diagnostics.len();
                (MbfReviewOutcome::Refused { diagnostics: bounded_diagnostics(diagnostics) }, listed)
            }
            reviewed => (reviewed, 0),
        }
    }

    fn shortened(self, refusal: String, noun: &str, listed: usize) -> Self {
        match self {
            MbfReviewOutcome::Refused { diagnostics } => {
                MbfReviewOutcome::refused_one(REVIEW_MEMORY_UNAVAILABLE, shortened_refusal(refusal, noun, listed, &diagnostics))
            }
            MbfReviewOutcome::Reviewed { .. } => {
                MbfReviewOutcome::refused_one(REVIEW_MEMORY_UNAVAILABLE, format!("{} The review is not shown.", stored_read::capped(refusal)))
            }
        }
    }
}

/// The review's memory refusal, by its own code.
fn review_memory_refusal(error: String) -> MbfReviewOutcome {
    MbfReviewOutcome::refused_one(REVIEW_MEMORY_UNAVAILABLE, stored_read::capped(error))
}

/// The review within `memory` (§3.9.5): the settings by the engine's one mapping, §3.6 steps 2 and 3 (the cap, the
/// canonical text and its hash against the claim, the open path's checks), then task 3 in `review` mode, and the
/// bounded review. It opens no store, file, destination or chooser and writes nothing; a review is not an
/// authorization.
fn review_mbf_export_outcome(run: &mut impl MbfWriterRun, request: MbfReviewRequest, memory: &PoolReadMemory<'_>, held: &mut Held) -> MbfReviewOutcome {
    let MbfReviewRequest { canonical_text, claimed_model_hash, settings_json } = request;
    if let Err(refusal) = read_settings(memory, &settings_json) {
        return refusal.map_or_else(review_memory_refusal, |refusal| MbfReviewOutcome::Refused { diagnostics: vec![refusal] });
    }
    let parse = match memory.reserve("the parse and canonical copy of the model text", admission_bytes(&canonical_text)) {
        Ok(reservation) => reservation,
        Err(error) => return review_memory_refusal(error),
    };
    let admitted = crate::model_document_save::admit_model_document_text_unscreened(canonical_text, &claimed_model_hash);
    drop(parse);
    let text = match admitted {
        Ok(text) => text,
        Err(diagnostics) => return MbfReviewOutcome::Refused { diagnostics },
    };
    let worker = WorkerRequest { mode: MbfMode::Review, settings_json: &settings_json, model_text: &text };
    let result = match run.run(&worker) {
        Ok(result) => result,
        Err(failure) => {
            return MbfReviewOutcome::Refused { diagnostics: vec![run_failure(failure, REVIEW_MEMORY_UNAVAILABLE, REVIEW_MEMORY_UNAVAILABLE)] }
        }
    };
    match memory.reserve("the MBF worker's review result", stored_read::exact_text_bytes(result.len() as u64)) {
        Ok(reservation) => held.reservations.push(reservation),
        Err(error) => return review_memory_refusal(error),
    }
    drop(text);
    let decoded = match decode_worker_result(MbfMode::Review, &result) {
        Ok(decoded) => decoded,
        Err(refusal) => return MbfReviewOutcome::Refused { diagnostics: vec![refusal] },
    };
    match reply_review(memory, decoded.review, held) {
        Ok(review) => MbfReviewOutcome::Reviewed { review },
        Err(refusal) => refusal.map_or_else(review_memory_refusal, |refusal| MbfReviewOutcome::Refused { diagnostics: vec![refusal] }),
    }
}

/// The review within `memory`, with task 3 run by `run`, and its response while every reservation is held. Until
/// 4b.2 only the test seam's in-process run reaches it.
fn review_mbf_export_with<R>(
    run: &mut impl MbfWriterRun,
    request: MbfReviewRequest,
    memory: &PoolReadMemory<'_>,
    respond: impl FnOnce(&MbfReviewOutcome, String) -> R,
) -> R {
    let mut held = Held::default();
    let outcome = review_mbf_export_outcome(run, request, memory, &mut held);
    let (sent, text, buffer) = serialize_outcome(memory, "Review MBF export", "review", outcome, None);
    let response = respond(&sent, text);
    drop(sent);
    drop(buffer);
    drop(held);
    response
}

/// The review's production entry point, everything after the arguments are decoded (the command slice 4b.2
/// registers). Not connected: it refuses every request with [`NOT_CONNECTED`] before any parse or worker run, and
/// opens no store, file or chooser.
pub(crate) fn review_mbf_export_admitted<R>(
    request: MbfReviewRequest,
    registry: &SolveJobRegistry,
    ingress: &Lease,
    respond: impl FnOnce(&MbfReviewOutcome, String) -> R,
) -> R {
    drop(request);
    let memory = PoolReadMemory::new(registry, ingress);
    let outcome = MbfReviewOutcome::refused_one(
        NOT_CONNECTED,
        "Review MBF export is not connected yet: native export memory governance is unfinished. Nothing was read or written.",
    );
    let (sent, text, buffer) = serialize_outcome(&memory, "Review MBF export", "review", outcome, None);
    let response = respond(&sent, text);
    drop(sent);
    drop(buffer);
    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::atomic_report_package_save::{test_support::atomic_save_test_lock, AtomicSaveIo, DirectorySyncResult, SystemAtomicSaveIo};
    use crate::command_ingress::test_support::{admit_json, check_decode_bound, registry};
    use crate::command_ingress::ArgsDecoding;
    use crate::export_disposition_screen::PROTECTED_CONTENT_BLOCKED;
    use crate::memory_pool::test_host::ScriptedHost;
    use open_pipe_stress_operation_applier::canonical_json;
    use serde_json::{json, Value};
    use std::cell::{Cell, RefCell};

    const EXPORT_MODEL: &str = include_str!("../../../../fixtures/caepipe_mbf/invented/desktop_export/export_model.json");
    const EXPORT_DIGESTS: &str = include_str!("../../../../fixtures/caepipe_mbf/invented/desktop_export/export_digests.json");
    const EVERY_SLOT: &str = include_str!("../../../../fixtures/model_document/invented_every_slot_model.json");

    fn canonical_of(text: &str) -> String {
        canonical_json(&serde_json::from_str::<Value>(text).unwrap())
    }

    fn edited(edit: impl FnOnce(&mut Value)) -> String {
        let mut document: Value = serde_json::from_str(EXPORT_MODEL).unwrap();
        edit(&mut document);
        canonical_json(&document)
    }

    /// The dialog's settings text (`JSON.stringify` of its settings).
    fn settings_text(title: &str, value: &str, unit: &str) -> String {
        format!(r#"{{"title":{},"referenceTemperature":{{"value":{value},"unit":"{unit}"}},"maxLayoutRows":null}}"#, json!(title))
    }

    fn settings() -> String {
        settings_text("Invented MBF export", "20", "degC")
    }

    fn screening(text: &str) -> ModelDocumentSaveScreening {
        ModelDocumentSaveScreening {
            route_id: MBF_EXPORT_ROUTE_ID.to_string(),
            export_context: "local_private".to_string(),
            explicit_local_private_intent: true,
            blocked: false,
            materialization_withheld: false,
            lossless_required: true,
            exact_payload_match: true,
            blocking_count: 0,
            payload_sha256: format!("sha256:{}", sha256_hex(text.as_bytes())),
        }
    }

    fn local_first() -> ModelDocumentSaveLocalFirst {
        ModelDocumentSaveLocalFirst {
            route_id: MBF_EXPORT_ROUTE_ID.to_string(),
            export_context: "local_private".to_string(),
            storage_context: "local_private".to_string(),
            action: "include_metadata_only".to_string(),
            reason_code: "PRIVATE_LOCAL_METADATA_ALLOWED".to_string(),
            blocked: false,
            metadata_only: true,
            explicit_local_private_intent: true,
        }
    }

    /// The engine's files for `text` with `settings`, from task 3 in `save` mode (the review's digests are the same).
    #[derive(Debug)]
    struct OwnedFiles {
        mbf: Vec<u8>,
        mbf_sha256: String,
        sidecar: Vec<u8>,
        sidecar_sha256: String,
    }

    fn engine_files(text: &str, settings: &str) -> OwnedFiles {
        let result = worker_render(&WorkerRequest { mode: MbfMode::Save, settings_json: settings, model_text: text }.encode()).unwrap();
        let decoded = decode_worker_result(MbfMode::Save, &result).unwrap();
        assert_eq!(decoded.status, WorkerStatus::Written);
        OwnedFiles {
            mbf: decoded.mbf.as_bytes().to_vec(),
            mbf_sha256: decoded.mbf_sha256.unwrap(),
            sidecar: decoded.identity_file.as_bytes().to_vec(),
            sidecar_sha256: decoded.sidecar_sha256.unwrap(),
        }
    }

    /// Task 3 in process, under the production limits.
    fn in_process(seam: &InProcessWriterTestSeam) -> InProcessWriterRun<'_> {
        InProcessWriterRun::new(seam, RenderLimits::PRODUCTION)
    }

    /// A request whose evidence and reviewed digests are those of `text`.
    fn request(text: &str) -> MbfExportRequest {
        let files = engine_files(text, &settings());
        MbfExportRequest {
            canonical_text: text.to_string(),
            claimed_model_hash: format!("sha256:{}", sha256_hex(text.as_bytes())),
            settings_json: settings(),
            suggested_name: "swbpipe-model.mbf".to_string(),
            reviewed: MbfReviewed { mbf_sha256: files.mbf_sha256, sidecar_sha256: files.sidecar_sha256 },
            screening: screening(text),
            local_first: local_first(),
        }
    }

    struct Sandbox(PathBuf);

    impl Sandbox {
        fn new(label: &str) -> Self {
            let path = std::env::temp_dir().join(format!(
                "swbpipe-mbf-export-{label}-{}-{}",
                std::process::id(),
                std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
            ));
            fs::create_dir_all(&path).unwrap();
            Sandbox(path)
        }

        fn entries(&self) -> Vec<String> {
            let mut names: Vec<String> = fs::read_dir(&self.0).unwrap().map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned()).collect();
            names.sort();
            names
        }
    }

    impl Drop for Sandbox {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn no_store() -> Result<rusqlite::Connection, String> {
        Err("the test has no library store".to_string())
    }

    /// Runs the save into `chosen` (None: Cancel) with `io`, recording whether the chooser opened.
    fn save_with(request: MbfExportRequest, pool: &SolveJobRegistry, chosen: Option<PathBuf>, io: &mut impl MbfExportIo, opened: &Cell<bool>) -> MbfExportOutcome {
        let memory = PoolReadMemory::unseeded(pool);
        let seam = InProcessWriterTestSeam::for_tests();
        let outcome = save_mbf_export_with(
            &mut in_process(&seam),
            request,
            &memory,
            no_store,
            |suggested| {
                assert!(valid_suggested_name(suggested));
                opened.set(true);
                Ok(chosen)
            },
            io,
            |outcome, text| {
                assert_eq!(serde_json::to_string(outcome).unwrap(), text);
                outcome.clone()
            },
        );
        drop(memory);
        assert_eq!(pool.pool_status().reserved_bytes, 0, "every reservation is released");
        outcome
    }

    fn save(request: MbfExportRequest, chosen: Option<PathBuf>, opened: &Cell<bool>) -> MbfExportOutcome {
        save_with(request, &registry(ScriptedHost::normal()), chosen, &mut SystemMbfExportIo, opened)
    }

    fn codes(outcome: &MbfExportOutcome) -> Vec<String> {
        match outcome {
            MbfExportOutcome::Refused { diagnostics, .. } => diagnostics.iter().map(|item| item.code.clone()).collect(),
            other => panic!("not refused: {other:?}"),
        }
    }

    fn model_ids(document: &Value, member: &str) -> Vec<String> {
        let mut ids: Vec<String> = document[member].as_array().unwrap().iter().map(|item| item["id"].as_str().unwrap().to_string()).collect();
        ids.sort();
        ids
    }

    /// Save, then import the files read from disk (C6-3, and the round trip): the bytes are the engine's, the
    /// identity file is named from the chosen stem, and a bound import restores every entity id.
    #[test]
    fn the_saved_pair_is_the_engines_bytes_and_imports_bound_with_the_same_ids() {
        let _lock = atomic_save_test_lock();
        let text = canonical_of(EXPORT_MODEL);
        let files = engine_files(&text, &settings());
        let sandbox = Sandbox::new("round-trip");
        let chosen = sandbox.0.join("Invented run.mbf");
        let opened = Cell::new(false);
        let outcome = save(request(&text), Some(chosen.clone()), &opened);
        let MbfExportOutcome::Saved { mbf, identity_file, durability, limitation, warnings, .. } = outcome else { panic!("not saved: {outcome:?}") };
        assert!(opened.get());
        assert_eq!(durability, "durable");
        assert_eq!(limitation, atomic_report_package_save::TOCTOU_LIMITATION);
        assert!(warnings.is_empty());
        assert_eq!(sandbox.entries(), vec!["Invented run.mbf".to_string(), "Invented run.swbpipe-ids.json".to_string()]);
        let mbf_bytes = fs::read(&chosen).unwrap();
        let ids_bytes = fs::read(sandbox.0.join("Invented run.swbpipe-ids.json")).unwrap();
        assert_eq!(mbf_bytes, files.mbf);
        assert_eq!(ids_bytes, files.sidecar);
        assert_eq!((mbf.file_name.as_str(), mbf.byte_count, mbf.sha256.as_str()), ("Invented run.mbf", mbf_bytes.len() as u64, files.mbf_sha256.as_str()));
        assert_eq!(identity_file.file_name, "Invented run.swbpipe-ids.json");
        assert_eq!((identity_file.byte_count, identity_file.sha256.as_str()), (ids_bytes.len() as u64, files.sidecar_sha256.as_str()));
        assert!(!mbf.replaced_existing && !identity_file.replaced_existing);
        // Bound import of the files as read from disk.
        let mut options = mbf::ImportOptions::new("Invented run.mbf");
        options.sidecar = Some(mbf::SidecarInput::Json(ids_bytes));
        let imported = mbf::import(&mbf_bytes, &options).unwrap();
        assert_eq!(imported.status, "imported", "{:?}", imported.diagnostics);
        assert!(imported.diagnostics.iter().all(|item| item.get_or_null("severity").as_str() != Some("error")));
        let back: Value = serde_json::from_slice(&imported.model_bytes().unwrap().unwrap()).unwrap();
        let source: Value = serde_json::from_str(&text).unwrap();
        for member in ["nodes", "pipe_segments", "supports", "load_cases", "materials", "sections"] {
            assert_eq!(model_ids(&back, member), model_ids(&source, member), "{member}");
        }
        let primitive = |document: &Value| {
            let mut ids: Vec<String> = document["load_cases"].as_array().unwrap().iter()
                .flat_map(|case| case["primitive_loads"].as_array().unwrap().iter().map(|load| load["id"].as_str().unwrap().to_string()))
                .collect();
            ids.sort();
            ids
        };
        assert_eq!(primitive(&back), primitive(&source));
        assert_eq!(back["pipe_segments"][0]["section_ref"], source["pipe_segments"][0]["section_ref"]);
        // Saving again over the same MBF replaces both files and leaves no backup behind (macOS); elsewhere a
        // replacement is refused before anything is written.
        let opened = Cell::new(false);
        let again = save(request(&text), Some(chosen.clone()), &opened);
        #[cfg(target_os = "macos")]
        {
            let MbfExportOutcome::Saved { mbf, identity_file, warnings, .. } = again else { panic!("not saved: {again:?}") };
            assert!(mbf.replaced_existing && identity_file.replaced_existing);
            assert!(warnings.is_empty(), "{warnings:?}");
        }
        #[cfg(not(target_os = "macos"))]
        assert_eq!(codes(&again), vec!["MBF-EXPORT-SAVE-REPLACE-UNSUPPORTED-PLATFORM".to_string()]);
        assert_eq!(sandbox.entries(), vec!["Invented run.mbf".to_string(), "Invented run.swbpipe-ids.json".to_string()]);
        assert_eq!(fs::read(&chosen).unwrap(), files.mbf);
    }

    /// The webview wrapper's recorded digests (`caepipe_mbf_wasm` tests) are the native worker's for the same text
    /// and settings, in review mode and in save mode: both map the dialog's settings to the same writer arguments.
    #[test]
    fn the_webview_wrappers_recorded_digests_are_the_native_writers() {
        let recorded: Value = serde_json::from_str(EXPORT_DIGESTS).unwrap();
        let cases = recorded["cases"].as_object().unwrap();
        assert_eq!(cases.len(), 5);
        for (name, case) in cases {
            let text = if name == "degf-project" { edited(|document| document["project"]["units"]["temperature"] = json!("degF")) } else { canonical_of(EXPORT_MODEL) };
            assert_eq!(case["model_sha256"], sha256_hex(text.as_bytes()), "{name}");
            let settings = case["settings_json"].as_str().unwrap();
            let review = worker_render(&WorkerRequest { mode: MbfMode::Review, settings_json: settings, model_text: &text }.encode()).unwrap();
            let review: Value = serde_json::from_str(decode_worker_result(MbfMode::Review, &review).unwrap().review).unwrap();
            assert_eq!(review["status"], case["status"], "{name}");
            if case["status"] == "written" {
                assert_eq!(review["mbf_sha256"], case["mbf_sha256"], "{name}");
                assert_eq!(review["sidecar_sha256"], case["sidecar_sha256"], "{name}");
                let files = engine_files(&text, settings);
                assert_eq!((json!(files.mbf_sha256), json!(files.sidecar_sha256)), (case["mbf_sha256"].clone(), case["sidecar_sha256"].clone()), "{name}");
            } else {
                // The writer's error codes, as the review lists them under Not written.
                let found: Vec<Value> = review["entries"].as_array().unwrap().iter().filter(|entry| entry["list"] == "not_written").map(|entry| entry["code"].clone()).collect();
                assert_eq!(case["refusal_codes"], Value::Array(found), "{name}");
                assert_eq!(review["blocking"], true, "{name}");
            }
        }
        // The cases between 2^63 and 2^64 and between 2^64 and 10^21 are read as the same ints on both sides.
        for (name, value) in [("degc-integer-above-2p63", "9223372036854776000"), ("degf-integer-above-2p64", "100000000000000000000")] {
            assert!(cases[name]["settings_json"].as_str().unwrap().contains(&format!(r#""value":{value},"#)), "{name}");
        }
    }

    /// Each refusal before the chooser names its code, opens no chooser and writes nothing.
    #[test]
    fn every_refusal_before_the_chooser_opens_none_and_writes_nothing() {
        let text = canonical_of(EXPORT_MODEL);
        let cases: Vec<(&str, MbfExportRequest)> = vec![
            ("MODEL-DOC-SAVE-EVIDENCE-INVALID", MbfExportRequest { screening: ModelDocumentSaveScreening { route_id: "DOTH-MODEL-DOC-007".into(), ..screening(&text) }, ..request(&text) }),
            ("MODEL-DOC-SAVE-EVIDENCE-INVALID", MbfExportRequest { local_first: ModelDocumentSaveLocalFirst { explicit_local_private_intent: false, ..local_first() }, ..request(&text) }),
            ("MODEL-DOC-SAVE-EVIDENCE-INVALID", MbfExportRequest { screening: ModelDocumentSaveScreening { exact_payload_match: false, ..screening(&text) }, ..request(&text) }),
            ("MODEL-DOC-SAVE-EVIDENCE-INVALID", MbfExportRequest { screening: ModelDocumentSaveScreening { materialization_withheld: true, ..screening(&text) }, ..request(&text) }),
            ("MODEL-DOC-SAVE-HASH-MISMATCH", MbfExportRequest { claimed_model_hash: format!("sha256:{}", sha256_hex(b"{}")), ..request(&text) }),
            ("MODEL-DOC-SAVE-HASH-MISMATCH", MbfExportRequest { canonical_text: format!("{text}\n"), claimed_model_hash: format!("sha256:{}", sha256_hex(format!("{text}\n").as_bytes())), ..request(&text) }),
            ("MODEL-DOC-SAVE-EVIDENCE-UNBOUND", MbfExportRequest { screening: screening("{}"), ..request(&text) }),
            (SETTINGS_INVALID, MbfExportRequest { settings_json: r#"{"title":"T","referenceTemperature":{"value":20,"unit":"degC"},"maxLayoutRows":20}"#.into(), ..request(&text) }),
            (SETTINGS_INVALID, MbfExportRequest { settings_json: settings_text("T", "20", "K"), ..request(&text) }),
            // The title bound, before the admission reads the model text (here not even JSON).
            (SETTINGS_INVALID, MbfExportRequest { settings_json: settings_text(&"a".repeat(4097), "20", "degC"), canonical_text: "not json".into(), ..request(&text) }),
            (SETTINGS_INVALID, MbfExportRequest { settings_json: settings_text(&"é".repeat(2049), "20", "degC"), canonical_text: "not json".into(), ..request(&text) }),
            (REVIEW_STALE, MbfExportRequest { reviewed: MbfReviewed { mbf_sha256: sha256_hex(b"other"), ..request(&text).reviewed }, ..request(&text) }),
            (REVIEW_STALE, MbfExportRequest { reviewed: MbfReviewed { sidecar_sha256: sha256_hex(b"other"), ..request(&text).reviewed }, ..request(&text) }),
            // The settings changed after the review: the same model, another reference temperature.
            (REVIEW_STALE, MbfExportRequest { settings_json: settings_text("Invented MBF export", "25", "degC"), ..request(&text) }),
            (SUGGESTED_NAME_INVALID, MbfExportRequest { suggested_name: "openpipestress-preview-project.mbf".into(), ..request(&text) }),
            (SUGGESTED_NAME_INVALID, MbfExportRequest { suggested_name: "swbpipe-model.json".into(), ..request(&text) }),
        ];
        for (code, request) in cases {
            let opened = Cell::new(false);
            let outcome = save(request, Some(std::env::temp_dir().join("never-written.mbf")), &opened);
            assert_eq!(codes(&outcome)[0], code);
            assert!(!opened.get(), "{code}: no chooser");
        }
        assert!(!std::env::temp_dir().join("never-written.mbf").exists());
    }

    /// The model is stale against the review: an edit after the review changes the files.
    #[test]
    fn an_edit_after_the_review_is_stale() {
        let text = canonical_of(EXPORT_MODEL);
        let edited_text = edited(|document| document["nodes"][4]["position"]["z"] = json!(6.5));
        let reviewed = request(&text).reviewed;
        let opened = Cell::new(false);
        let outcome = save(MbfExportRequest { reviewed, ..request(&edited_text) }, None, &opened);
        assert_eq!(codes(&outcome), vec![REVIEW_STALE.to_string()]);
        assert!(!opened.get());
    }

    /// A writer refusal returns the bounded review only, before any chooser: the writer's codes are in the review,
    /// and none of its messages leaves the worker.
    #[test]
    fn a_writer_refusal_returns_its_review_before_any_chooser() {
        let text = edited(|document| {
            document["components"][0]["kind"] = json!("tee");
        });
        let opened = Cell::new(false);
        let mut request = request_unreviewed(&text);
        request.reviewed = MbfReviewed { mbf_sha256: "0".repeat(64), sidecar_sha256: "0".repeat(64) };
        let outcome = save(request, Some(std::env::temp_dir().join("never-written.mbf")), &opened);
        assert_eq!(codes(&outcome), vec![WRITER_REFUSED.to_string()]);
        let review = refusal_review(&outcome);
        assert_eq!(review["blocking"], true);
        let blocked: Vec<&str> = review["entries"].as_array().unwrap().iter().filter(|entry| entry["list"] == "not_written").map(|entry| entry["code"].as_str().unwrap()).collect();
        assert!(!blocked.is_empty() && blocked.iter().all(|code| code.starts_with("MBF-SI-")), "{blocked:?}");
        assert!(!serde_json::to_string(&outcome).unwrap().contains("CAEPIPE"));
        assert!(!opened.get());
        // Every slot of the model document (its one library reference dropped, as the test has no library
        // store): the review names its spectra.
        let mut every_slot: Value = serde_json::from_str(EVERY_SLOT).unwrap();
        every_slot["materials"][0]["provenance_record"].as_object_mut().unwrap().remove("library_ref");
        let every_slot = canonical_json(&every_slot);
        let outcome = save(request_unreviewed(&every_slot), None, &opened);
        assert_eq!(codes(&outcome), vec![WRITER_REFUSED.to_string()]);
        assert!(refusal_review(&outcome)["entries"].as_array().unwrap().iter().any(|entry| entry["code"] == "MBF-SI-SPECTRUM-UNSUPPORTED"), "{outcome:?}");
        assert!(!opened.get());
    }

    /// The bounded review a refusal carries, as JSON.
    fn refusal_review(outcome: &MbfExportOutcome) -> Value {
        match outcome {
            MbfExportOutcome::Refused { review: Some(review), .. } => serde_json::from_str(review.get()).unwrap(),
            other => panic!("no review: {other:?}"),
        }
    }

    fn request_unreviewed(text: &str) -> MbfExportRequest {
        MbfExportRequest {
            canonical_text: text.to_string(),
            claimed_model_hash: format!("sha256:{}", sha256_hex(text.as_bytes())),
            settings_json: settings(),
            suggested_name: "swbpipe-model.mbf".to_string(),
            reviewed: MbfReviewed { mbf_sha256: "0".repeat(64), sidecar_sha256: "0".repeat(64) },
            screening: screening(text),
            local_first: local_first(),
        }
    }

    /// A protected disposition in the exact admitted model refuses before the writer and the chooser.
    #[test]
    fn a_protected_disposition_is_refused() {
        for (status, value) in [("review_status", "quarantined"), ("review_status", "rejected"), ("redistribution_status", "protected_suspected")] {
            let text = edited(|document| document["materials"][0]["provenance_record"][status] = json!(value));
            let opened = Cell::new(false);
            let outcome = save(request_unreviewed(&text), None, &opened);
            match &outcome {
                MbfExportOutcome::Refused { diagnostics, .. } => {
                    assert_eq!((diagnostics[0].code.as_str(), diagnostics[0].path.as_str()), (PROTECTED_CONTENT_BLOCKED, "materials[0].provenance_record"), "{value}");
                }
                other => panic!("{value}: {other:?}"),
            }
            assert!(!opened.get());
        }
    }

    #[test]
    fn cancel_writes_nothing() {
        let sandbox = Sandbox::new("cancel");
        let opened = Cell::new(false);
        assert_eq!(save(request(&canonical_of(EXPORT_MODEL)), None, &opened), MbfExportOutcome::Cancelled);
        assert!(opened.get());
        assert!(sandbox.entries().is_empty());
    }

    #[test]
    fn the_chosen_path_must_be_an_mbf() {
        let sandbox = Sandbox::new("extension");
        for name in ["model.txt", "model.MBF", "model"] {
            let opened = Cell::new(false);
            let outcome = save(request(&canonical_of(EXPORT_MODEL)), Some(sandbox.0.join(name)), &opened);
            assert_eq!(codes(&outcome), vec![DESTINATION_EXTENSION.to_string()], "{name}");
        }
        assert!(sandbox.entries().is_empty());
    }

    /// File operations that record the writes and can fail or tamper on request.
    #[derive(Default)]
    struct ScriptedIo {
        writes: RefCell<Vec<String>>,
        fail_mbf_write: bool,
        fail_ids_write: bool,
        tamper_mbf: bool,
        swap_ids_before_cleanup: bool,
        /// Hard links made before every later one fails (as on a volume without them): None never fails.
        fail_hard_link_after: Option<usize>,
        fail_restore: bool,
        links: Vec<PathBuf>,
    }

    /// The atomic writer's own operations with a rename that fails, so the failure is the writer's.
    struct FailingRename(SystemAtomicSaveIo);

    impl AtomicSaveIo for FailingRename {
        fn write_all(&mut self, temp: &mut File, bytes: &[u8]) -> io::Result<()> {
            self.0.write_all(temp, bytes)
        }
        fn flush(&mut self, temp: &mut File) -> io::Result<()> {
            self.0.flush(temp)
        }
        fn sync_temp(&mut self, temp: &File) -> io::Result<()> {
            self.0.sync_temp(temp)
        }
        fn rename(&mut self, _: &Path, _: &Path) -> io::Result<()> {
            Err(io::Error::other("injected rename failure"))
        }
        fn cleanup_temp(&mut self, temp_path: &Path) -> io::Result<()> {
            self.0.cleanup_temp(temp_path)
        }
        fn sync_parent(&mut self, _: &Path) -> DirectorySyncResult {
            DirectorySyncResult::Durable
        }
    }

    impl MbfExportIo for ScriptedIo {
        fn write(&mut self, format: &AtomicSaveFormat, destination: &Path, bytes: &[u8]) -> Result<AtomicSaveOutcome, AtomicSaveError> {
            self.writes.borrow_mut().push(basename(destination));
            if (format == &MBF_FORMAT && self.fail_mbf_write) || (format == &MBF_IDS_FORMAT && self.fail_ids_write) {
                return atomic_report_package_save::atomic_save_bytes_as_with_io(format, destination, bytes, &mut FailingRename(SystemAtomicSaveIo));
            }
            let outcome = atomic_report_package_save::atomic_save_bytes_as(format, destination, bytes)?;
            if format == &MBF_FORMAT && self.tamper_mbf {
                use std::io::Write;
                fs::OpenOptions::new().append(true).open(destination).unwrap().write_all(b"!").unwrap();
            }
            Ok(outcome)
        }
        fn remove(&mut self, path: &Path) -> io::Result<()> {
            fs::remove_file(path)
        }
        fn hard_link(&mut self, original: &Path, link: &Path) -> io::Result<()> {
            if self.fail_hard_link_after.is_some_and(|allowed| self.links.len() >= allowed) {
                // As on a volume without hard links (FAT, exFAT).
                return Err(io::Error::new(io::ErrorKind::Unsupported, "injected: hard links are not supported here"));
            }
            self.links.push(link.to_path_buf());
            fs::hard_link(original, link)
        }
        fn rename(&mut self, from: &Path, to: &Path) -> io::Result<()> {
            if self.fail_restore {
                return Err(io::Error::other("injected restore failure"));
            }
            fs::rename(from, to)
        }
    }

    fn save_scripted(io: &mut ScriptedIo, chosen: PathBuf) -> MbfExportOutcome {
        let opened = Cell::new(false);
        save_with(request(&canonical_of(EXPORT_MODEL)), &registry(ScriptedHost::normal()), Some(chosen), io, &opened)
    }

    #[test]
    fn the_identity_file_is_written_before_the_mbf() {
        let _lock = atomic_save_test_lock();
        let sandbox = Sandbox::new("order");
        let mut io = ScriptedIo::default();
        assert!(matches!(save_scripted(&mut io, sandbox.0.join("swbpipe-model.mbf")), MbfExportOutcome::Saved { .. }));
        assert_eq!(*io.writes.borrow(), vec!["swbpipe-model.swbpipe-ids.json".to_string(), "swbpipe-model.mbf".to_string()]);
    }

    #[test]
    fn a_failed_mbf_write_removes_the_identity_file_it_wrote() {
        let _lock = atomic_save_test_lock();
        let sandbox = Sandbox::new("second-write");
        let mut io = ScriptedIo { fail_mbf_write: true, ..ScriptedIo::default() };
        let outcome = save_scripted(&mut io, sandbox.0.join("swbpipe-model.mbf"));
        assert_eq!(codes(&outcome), vec!["MBF-EXPORT-SAVE-RENAME-FAILED".to_string()]);
        assert_eq!(io.writes.borrow().len(), 2);
        assert!(sandbox.entries().is_empty(), "{:?}", sandbox.entries());
    }

    /// The identity check: a file that is no longer the one this save wrote is left in place, by name.
    #[test]
    fn a_cleanup_leaves_a_file_that_is_no_longer_the_one_written() {
        let _lock = atomic_save_test_lock();
        let sandbox = Sandbox::new("swapped");
        struct SwapIds(ScriptedIo);
        impl MbfExportIo for SwapIds {
            fn write(&mut self, format: &AtomicSaveFormat, destination: &Path, bytes: &[u8]) -> Result<AtomicSaveOutcome, AtomicSaveError> {
                if format == &MBF_FORMAT {
                    // Another program replaces the identity file between the two writes.
                    let ids = identity_file_path(destination).unwrap();
                    fs::remove_file(&ids).unwrap();
                    fs::write(&ids, b"someone else's file").unwrap();
                }
                self.0.write(format, destination, bytes)
            }
        }
        let mut io = SwapIds(ScriptedIo { fail_mbf_write: true, swap_ids_before_cleanup: true, ..ScriptedIo::default() });
        let opened = Cell::new(false);
        let outcome = save_with(request(&canonical_of(EXPORT_MODEL)), &registry(ScriptedHost::normal()), Some(sandbox.0.join("swbpipe-model.mbf")), &mut io, &opened);
        assert_eq!(codes(&outcome), vec!["MBF-EXPORT-SAVE-RENAME-FAILED".to_string(), CLEANUP_SKIPPED.to_string()]);
        assert!(io.0.swap_ids_before_cleanup);
        assert_eq!(fs::read(sandbox.0.join("swbpipe-model.swbpipe-ids.json")).unwrap(), b"someone else's file");
    }

    #[test]
    fn a_read_back_mismatch_removes_both_files() {
        let _lock = atomic_save_test_lock();
        let sandbox = Sandbox::new("read-back");
        let mut io = ScriptedIo { tamper_mbf: true, ..ScriptedIo::default() };
        let outcome = save_scripted(&mut io, sandbox.0.join("swbpipe-model.mbf"));
        assert_eq!(codes(&outcome), vec![READBACK_MISMATCH.to_string()]);
        assert!(sandbox.entries().is_empty(), "{:?}", sandbox.entries());
    }

    const OLD_MBF: &[u8] = b"an earlier model batch file\n";
    const OLD_IDS: &[u8] = b"{\"an earlier identity file\": true}\n";

    /// An earlier pair at `swbpipe-model.mbf`: the chosen path, the identity file's, and their identities.
    fn earlier_pair(sandbox: &Sandbox) -> (PathBuf, PathBuf, FileIdentity, FileIdentity) {
        let chosen = sandbox.0.join("swbpipe-model.mbf");
        let ids = sandbox.0.join("swbpipe-model.swbpipe-ids.json");
        fs::write(&chosen, OLD_MBF).unwrap();
        fs::write(&ids, OLD_IDS).unwrap();
        let (mbf_identity, ids_identity) = (file_identity(&chosen).unwrap(), file_identity(&ids).unwrap());
        (chosen, ids, mbf_identity, ids_identity)
    }

    /// The earlier pair is there, byte-identical and the same files, and nothing else is in the folder.
    fn assert_earlier_pair(sandbox: &Sandbox, label: &str, (chosen, ids, mbf_identity, ids_identity): &(PathBuf, PathBuf, FileIdentity, FileIdentity)) {
        assert_eq!(sandbox.entries(), vec!["swbpipe-model.mbf".to_string(), "swbpipe-model.swbpipe-ids.json".to_string()], "{label}");
        assert_eq!(fs::read(chosen).unwrap(), OLD_MBF, "{label}");
        assert_eq!(fs::read(ids).unwrap(), OLD_IDS, "{label}");
        assert!(file_identity(chosen).unwrap().same_file(mbf_identity), "{label}: the earlier MBF itself");
        assert!(file_identity(ids).unwrap().same_file(ids_identity), "{label}: the earlier identity file itself");
    }

    /// P1: a replacement that fails at any step after the backups (the identity file's write, the MBF's write, the
    /// read-back) puts the earlier pair back, byte-identical, and leaves no backup behind.
    #[cfg(target_os = "macos")]
    #[test]
    fn a_failed_replacement_leaves_the_earlier_pair_byte_identical() {
        let _lock = atomic_save_test_lock();
        let cases: Vec<(&str, ScriptedIo, &str)> = vec![
            ("mbf write", ScriptedIo { fail_mbf_write: true, ..ScriptedIo::default() }, "MBF-EXPORT-SAVE-RENAME-FAILED"),
            ("identity file write", ScriptedIo { fail_ids_write: true, ..ScriptedIo::default() }, "MBF-EXPORT-IDS-SAVE-RENAME-FAILED"),
            ("read-back", ScriptedIo { tamper_mbf: true, ..ScriptedIo::default() }, READBACK_MISMATCH),
        ];
        for (label, mut io, code) in cases {
            let sandbox = Sandbox::new("replace-fails");
            let earlier = earlier_pair(&sandbox);
            let outcome = save_scripted(&mut io, earlier.0.clone());
            assert_eq!(codes(&outcome), vec![code.to_string()], "{label}");
            assert_eq!(io.links.len(), 2, "{label}: both files were backed up before the first write");
            assert_earlier_pair(&sandbox, label, &earlier);
        }
    }

    /// P1: a symlink or a directory at the chosen `.mbf` is refused by the atomic writer's own checks before anything
    /// is backed up or written; the identity file beside it is untouched.
    #[test]
    fn a_symlinked_or_directory_mbf_beside_an_identity_file_writes_nothing() {
        let _lock = atomic_save_test_lock();
        let mut cases: Vec<(&str, &str)> = vec![("directory", "MBF-EXPORT-SAVE-DESTINATION-DIRECTORY")];
        #[cfg(unix)]
        cases.push(("symlink", "MBF-EXPORT-SAVE-DESTINATION-SYMLINK"));
        for (kind, code) in cases {
            let sandbox = Sandbox::new("unsafe-destination");
            let chosen = sandbox.0.join("swbpipe-model.mbf");
            let ids = sandbox.0.join("swbpipe-model.swbpipe-ids.json");
            fs::write(&ids, OLD_IDS).unwrap();
            let ids_identity = file_identity(&ids).unwrap();
            match kind {
                "directory" => fs::create_dir(&chosen).unwrap(),
                #[cfg(unix)]
                _ => {
                    fs::write(sandbox.0.join("target.txt"), b"target").unwrap();
                    std::os::unix::fs::symlink(sandbox.0.join("target.txt"), &chosen).unwrap();
                }
                #[cfg(not(unix))]
                _ => unreachable!(),
            }
            let before = sandbox.entries();
            let mut io = ScriptedIo::default();
            let outcome = save_scripted(&mut io, chosen.clone());
            assert_eq!(codes(&outcome), vec![code.to_string()], "{kind}");
            assert!(io.writes.borrow().is_empty() && io.links.is_empty(), "{kind}: nothing backed up or written");
            assert_eq!(sandbox.entries(), before, "{kind}");
            assert_eq!(fs::read(&ids).unwrap(), OLD_IDS, "{kind}");
            assert!(file_identity(&ids).unwrap().same_file(&ids_identity), "{kind}");
        }
    }

    /// P2-1: an identity file whose MBF is absent may belong to an MBF that was moved; it is not replaced.
    #[test]
    fn an_identity_file_without_its_mbf_is_refused_before_writing() {
        let _lock = atomic_save_test_lock();
        let sandbox = Sandbox::new("lone-identity-file");
        let ids = sandbox.0.join("swbpipe-model.swbpipe-ids.json");
        fs::write(&ids, OLD_IDS).unwrap();
        let mut io = ScriptedIo::default();
        let outcome = save_scripted(&mut io, sandbox.0.join("swbpipe-model.mbf"));
        assert_eq!(codes(&outcome), vec![IDENTITY_FILE_EXISTS.to_string()]);
        let MbfExportOutcome::Refused { diagnostics, .. } = &outcome else { unreachable!() };
        assert!(diagnostics[0].message.contains("may be the identity file of a model batch file that was moved"), "{}", diagnostics[0].message);
        assert!(io.writes.borrow().is_empty() && io.links.is_empty());
        assert_eq!(sandbox.entries(), vec!["swbpipe-model.swbpipe-ids.json".to_string()]);
        assert_eq!(fs::read(&ids).unwrap(), OLD_IDS);
    }

    /// P1: a file to be replaced that cannot be backed up by a hard link refuses the save before any write; a backup
    /// already made for the other file is removed.
    #[cfg(target_os = "macos")]
    #[test]
    fn a_replacement_without_a_backup_is_refused_before_any_write() {
        let _lock = atomic_save_test_lock();
        for allowed in [0, 1] {
            let sandbox = Sandbox::new("backup-unavailable");
            let earlier = earlier_pair(&sandbox);
            let mut io = ScriptedIo { fail_hard_link_after: Some(allowed), ..ScriptedIo::default() };
            let outcome = save_scripted(&mut io, earlier.0.clone());
            assert_eq!(codes(&outcome), vec![REPLACE_BACKUP_UNAVAILABLE.to_string()], "after {allowed}");
            assert!(io.writes.borrow().is_empty(), "after {allowed}: nothing written");
            assert_eq!(io.links.len(), allowed);
            assert_earlier_pair(&sandbox, "backup unavailable", &earlier);
        }
    }

    /// P1: a successful replacement removes its backups.
    #[cfg(target_os = "macos")]
    #[test]
    fn a_successful_replacement_leaves_no_backup() {
        let _lock = atomic_save_test_lock();
        let sandbox = Sandbox::new("replace-succeeds");
        let earlier = earlier_pair(&sandbox);
        let mut io = ScriptedIo::default();
        let outcome = save_scripted(&mut io, earlier.0.clone());
        let MbfExportOutcome::Saved { mbf, identity_file, warnings, .. } = outcome else { panic!("not saved: {outcome:?}") };
        assert!(mbf.replaced_existing && identity_file.replaced_existing && warnings.is_empty());
        assert_eq!(io.links.len(), 2);
        assert!(io.links.iter().all(|link| !link.exists()));
        assert_eq!(sandbox.entries(), vec!["swbpipe-model.mbf".to_string(), "swbpipe-model.swbpipe-ids.json".to_string()]);
        let files = engine_files(&canonical_of(EXPORT_MODEL), &settings());
        assert_eq!((fs::read(&earlier.0).unwrap(), fs::read(&earlier.1).unwrap()), (files.mbf, files.sidecar));
    }

    /// Decision A: a rollback that cannot put an earlier file back is not reported as success; the backup is kept,
    /// unchanged, and named with its original, so the user can recover it.
    #[cfg(target_os = "macos")]
    #[test]
    fn a_rollback_that_cannot_restore_keeps_and_names_each_backup() {
        let _lock = atomic_save_test_lock();
        let cases: Vec<(&str, ScriptedIo, Vec<&str>, usize)> = vec![
            // The MBF write fails: the earlier MBF is still in place; the identity file cannot be put back.
            ("mbf write", ScriptedIo { fail_mbf_write: true, fail_restore: true, ..ScriptedIo::default() }, vec!["MBF-EXPORT-SAVE-RENAME-FAILED", ROLLBACK_INCOMPLETE], 1),
            // Both written, the read-back fails: neither can be put back.
            ("read-back", ScriptedIo { tamper_mbf: true, fail_restore: true, ..ScriptedIo::default() }, vec![READBACK_MISMATCH, ROLLBACK_INCOMPLETE, ROLLBACK_INCOMPLETE], 2),
        ];
        for (label, mut io, expected, kept) in cases {
            let sandbox = Sandbox::new("rollback-incomplete");
            let earlier = earlier_pair(&sandbox);
            let outcome = save_scripted(&mut io, earlier.0.clone());
            assert_eq!(codes(&outcome), expected, "{label}");
            let MbfExportOutcome::Refused { diagnostics, .. } = &outcome else { unreachable!() };
            let backups: Vec<&PathBuf> = io.links.iter().filter(|link| link.exists()).collect();
            assert_eq!(backups.len(), kept, "{label}: the backups not restored are kept");
            for backup in backups {
                let name = basename(backup);
                assert!(name.starts_with(BACKUP_PREFIX) && name.ends_with(BACKUP_SUFFIX), "{name}");
                let bytes = fs::read(backup).unwrap();
                let original = if bytes == OLD_MBF { "swbpipe-model.mbf" } else { assert_eq!(bytes, OLD_IDS); "swbpipe-model.swbpipe-ids.json" };
                assert!(
                    diagnostics.iter().any(|item| item.code == ROLLBACK_INCOMPLETE && item.message.starts_with(&format!("Kept as `{name}`")) && item.message.contains(&format!("Rename `{name}` to `{original}`"))),
                    "{label}: {diagnostics:?}"
                );
            }
            if label == "mbf write" {
                assert_eq!(fs::read(&earlier.0).unwrap(), OLD_MBF, "the earlier MBF was never replaced");
            }
        }
    }

    /// P3-2: a kept backup's original is named in full however long its name: here an identity file name of 255 bytes
    /// (the longest a file name can be) and its MBF's.
    #[test]
    fn a_rollback_names_a_long_original_in_full() {
        let _lock = atomic_save_test_lock();
        let sandbox = Sandbox::new("rollback-long-name");
        let stem = format!("swbpipe-model-{}", "l".repeat(255 - IDS_SUFFIX.len() - "swbpipe-model-".len()));
        let chosen = sandbox.0.join(format!("{stem}.mbf"));
        let ids = sandbox.0.join(format!("{stem}{IDS_SUFFIX}"));
        assert_eq!(basename(&ids).len(), 255);
        fs::write(&chosen, OLD_MBF).unwrap();
        fs::write(&ids, OLD_IDS).unwrap();
        let mut io = ScriptedIo { tamper_mbf: true, fail_restore: true, ..ScriptedIo::default() };
        let outcome = save_scripted(&mut io, chosen.clone());
        assert_eq!(codes(&outcome), vec![READBACK_MISMATCH, ROLLBACK_INCOMPLETE, ROLLBACK_INCOMPLETE]);
        // As the response bounds it, each name is still whole.
        let (MbfExportOutcome::Refused { diagnostics, .. }, _) = outcome.bounded() else { unreachable!() };
        for original in [basename(&chosen), basename(&ids)] {
            assert!(original.len() > 120, "longer than a quoted name is cut to");
            assert!(
                diagnostics.iter().any(|item| item.code == ROLLBACK_INCOMPLETE && item.message.contains(&format!("the previous `{original}`, unchanged")) && item.message.ends_with(&format!("to `{original}` to recover it."))),
                "{original}: {diagnostics:?}"
            );
        }
    }

    /// An identity-file write that fails on a new pair leaves nothing: the MBF is never written.
    #[test]
    fn a_failed_identity_file_write_writes_nothing_else() {
        let _lock = atomic_save_test_lock();
        let sandbox = Sandbox::new("ids-write");
        let mut io = ScriptedIo { fail_ids_write: true, ..ScriptedIo::default() };
        let outcome = save_scripted(&mut io, sandbox.0.join("swbpipe-model.mbf"));
        assert_eq!(codes(&outcome), vec!["MBF-EXPORT-IDS-SAVE-RENAME-FAILED".to_string()]);
        assert_eq!(*io.writes.borrow(), vec!["swbpipe-model.swbpipe-ids.json".to_string()]);
        assert!(sandbox.entries().is_empty(), "{:?}", sandbox.entries());
    }

    /// P3-4: off Unix a file is the same only when its size and both times are present and equal.
    #[test]
    fn file_identity_needs_every_part_it_compares() {
        let sandbox = Sandbox::new("identity");
        let path = sandbox.0.join("a.mbf");
        fs::write(&path, b"a").unwrap();
        let identity = file_identity(&path).unwrap();
        assert!(identity.same_file(&file_identity(&path).unwrap()));
        #[cfg(not(unix))]
        {
            let (size, created, modified) = identity.size_created_modified;
            for other in [(size + 1, created, modified), (size, None, modified), (size, created, None), (size, None, None)] {
                assert!(!identity.same_file(&FileIdentity { size_created_modified: other }), "{other:?}");
            }
            // A missing time is not identical, even to itself.
            assert!(!FileIdentity { size_created_modified: (size, None, None) }.same_file(&FileIdentity { size_created_modified: (size, None, None) }));
        }
        let other = sandbox.0.join("b.mbf");
        fs::write(&other, b"a").unwrap();
        #[cfg(unix)]
        assert!(!identity.same_file(&file_identity(&other).unwrap()));
    }

    #[test]
    fn suggested_names_follow_the_file_name_convention() {
        for name in ["swbpipe-model.mbf", "swbpipe-model-rev-b.mbf", "swbpipe-model-2.mbf"] {
            assert!(valid_suggested_name(name), "{name}");
        }
        for name in ["swbpipe-model-.mbf", "swbpipe-model--a.mbf", "swbpipe-model-A.mbf", "swbpipe-modelx.mbf", "swbpipe-model.MBF", "model.mbf", "swbpipe-model-a b.mbf", "../swbpipe-model.mbf", ""] {
            assert!(!valid_suggested_name(name), "{name}");
        }
        assert!(!valid_suggested_name(&format!("swbpipe-model-{}.mbf", "a".repeat(200))));
        assert_eq!(identity_file_path(Path::new("/tmp/a.b.mbf")).unwrap(), Path::new("/tmp/a.b.swbpipe-ids.json"));
    }

    #[test]
    fn the_named_arguments_decode_and_unknown_members_are_refused() {
        let text = canonical_of(EXPORT_MODEL);
        let request = request(&text);
        let body = json!({
            "canonicalText": text,
            "claimedModelHash": request.claimed_model_hash,
            "settingsJson": settings(),
            "suggestedName": "swbpipe-model.mbf",
            "reviewed": {"mbfSha256": request.reviewed.mbf_sha256, "sidecarSha256": request.reviewed.sidecar_sha256},
            "screening": serde_json::to_value(screening(&text)).unwrap(),
            "localFirstEvidence": serde_json::to_value(local_first()).unwrap(),
        });
        let pool = registry(ScriptedHost::normal());
        let (mut args, _lease) = admit_json(&pool, COMMAND, ArgsDecoding::Moved, &body).unwrap();
        let decoded = save_mbf_export_args(&mut args).unwrap();
        assert_eq!(decoded.canonical_text, text);
        assert_eq!(decoded.settings_json, settings());
        assert_eq!(decoded.reviewed, request.reviewed);
        assert_eq!(decoded.screening, screening(&text));
        let raw = serde_json::to_vec(&body).unwrap();
        let _ = check_decode_bound(&raw, ArgsDecoding::Moved, |args| save_mbf_export_args(args).map(drop));
        for (member, edit) in [("reviewed", json!({"mbfSha256": "a", "sidecarSha256": "b", "extra": 1})), ("screening", json!(null))] {
            let mut bad = body.clone();
            bad[member] = edit;
            let (mut args, _lease) = admit_json(&pool, COMMAND, ArgsDecoding::Moved, &bad).unwrap();
            assert!(save_mbf_export_args(&mut args).is_err(), "{member}");
        }
    }

    /// The largest test model: the fixture with its last run continued by `extra` straight pipes.
    fn long_model(extra: usize) -> String {
        edited(|document| {
            let nodes = document["nodes"].as_array_mut().unwrap();
            for index in 0..extra {
                nodes.push(json!({"id": format!("node:X{index}"), "label": format!("Run node {index}"), "position": {"x": 4.0, "y": 0.0, "z": 7.0 + index as f64}, "provenance": "invented_example"}));
            }
            let pipes = document["pipe_segments"].as_array_mut().unwrap();
            let template = pipes[3].clone();
            for index in 0..extra {
                let mut pipe = template.clone();
                pipe["id"] = json!(format!("pipe:X{index}"));
                pipe["label"] = json!(format!("Run {index}"));
                pipe["from"] = json!(if index == 0 { "node:C".to_string() } else { format!("node:X{}", index - 1) });
                pipe["to"] = json!(format!("node:X{index}"));
                pipes.push(pipe);
            }
            let loads = document["load_cases"][1]["primitive_loads"].as_array_mut().unwrap();
            let template = loads[0].clone();
            for index in 0..extra {
                let mut load = template.clone();
                load["id"] = json!(format!("load:t-X{index}"));
                load["target"]["pipe"] = json!(format!("pipe:X{index}"));
                loads.push(load);
            }
        })
    }

    /// The production entry point refuses before anything else (§3.7): with otherwise valid arguments and a chooser
    /// that would name a folder, it returns `MBF-EXPORT-NOT-CONNECTED`, opens no store and no chooser, writes nothing,
    /// and releases what it held.
    #[test]
    fn the_production_entry_point_refuses_not_connected_and_touches_nothing() {
        let _lock = atomic_save_test_lock();
        let sandbox = Sandbox::new("not-connected");
        let text = canonical_of(EXPORT_MODEL);
        let request = request(&text);
        let body = json!({"canonicalText": text});
        let pool = registry(ScriptedHost::normal());
        let (_, lease) = admit_json(&pool, COMMAND, ArgsDecoding::Moved, &body).unwrap();
        let (store_opened, chooser_opened) = (Cell::new(false), Cell::new(false));
        let chosen = sandbox.0.join("swbpipe-model.mbf");
        let (outcome, text) = save_mbf_export_admitted(
            request,
            || {
                store_opened.set(true);
                no_store()
            },
            &pool,
            &lease,
            |_| {
                chooser_opened.set(true);
                Ok(Some(chosen.clone()))
            },
            |outcome, text| (outcome.clone(), text),
        );
        assert_eq!(codes(&outcome), vec![NOT_CONNECTED.to_string()]);
        let MbfExportOutcome::Refused { diagnostics, .. } = &outcome else { unreachable!() };
        assert!(diagnostics[0].message.contains("native export memory governance is unfinished"), "{}", diagnostics[0].message);
        assert_eq!(serde_json::to_string(&outcome).unwrap(), text);
        assert!(!store_opened.get() && !chooser_opened.get());
        assert!(sandbox.entries().is_empty(), "{:?}", sandbox.entries());
        assert!(!chosen.exists());
        drop(lease);
        assert_eq!(pool.pool_status().reserved_bytes, 0);
    }

    /// Through the test seam, the ingress lease of the arguments is counted with every host-side reservation, so a
    /// budget that admits the save without the lease refuses it with the lease, before the chooser.
    #[test]
    fn the_ingress_lease_is_counted_with_the_saves_reservations() {
        let _lock = atomic_save_test_lock();
        let sandbox = Sandbox::new("lease");
        let text = canonical_of(EXPORT_MODEL);
        let body = json!({"canonicalText": text, "pad": "x".repeat(1 << 20)});
        let run = |pool: &SolveJobRegistry, chosen: PathBuf, opened: &Cell<bool>| {
            let (_, lease) = admit_json(pool, COMMAND, ArgsDecoding::Moved, &body).unwrap();
            let memory = PoolReadMemory::new(pool, &lease);
            let seam = InProcessWriterTestSeam::for_tests();
            let outcome = save_mbf_export_with(
                &mut in_process(&seam),
                request(&text),
                &memory,
                no_store,
                |_| {
                    opened.set(true);
                    Ok(Some(chosen))
                },
                &mut SystemMbfExportIo,
                |outcome, _| outcome.clone(),
            );
            drop(memory);
            (outcome, lease.bytes())
        };
        let pool = registry(ScriptedHost::normal());
        let opened = Cell::new(false);
        let (outcome, lease) = run(&pool, sandbox.0.join("swbpipe-model.mbf"), &opened);
        assert!(matches!(outcome, MbfExportOutcome::Saved { .. }), "{outcome:?}");
        assert!(lease > 1 << 20);
        assert_eq!(pool.pool_status().reserved_bytes, 0);
        // The most the save holds without a lease, through Cancel (every host-side reservation is made before the
        // chooser).
        let unseeded = registry(ScriptedHost::normal());
        let memory = PoolReadMemory::unseeded(&unseeded);
        let opened = Cell::new(false);
        let cancelled = save_mbf_export_with(&mut in_process(&InProcessWriterTestSeam::for_tests()), request(&text), &memory, no_store, |_| { opened.set(true); Ok(None) }, &mut SystemMbfExportIo, |outcome, _| outcome.clone());
        assert_eq!(cancelled, MbfExportOutcome::Cancelled);
        let held = memory.peak_reserved();
        drop(memory);
        // A budget that admits the arguments and fits the save without the lease, but not with it.
        let budget = lease + held / 2;
        assert!(held <= budget && held / 2 < held, "held {held} lease {lease}");
        let tight = registry(ScriptedHost::normal());
        tight.configure_pool(64 << 20, budget, false).unwrap();
        let opened = Cell::new(false);
        let (outcome, _) = run(&tight, sandbox.0.join("other.mbf"), &opened);
        assert_eq!(codes(&outcome), vec![MEMORY_UNAVAILABLE.to_string()]);
        assert!(!opened.get());
        assert_eq!(tight.pool_status().reserved_bytes, 0);
    }

    /// A host-side reservation the pool refuses refuses the save by name, before any chooser, and writes nothing.
    #[test]
    fn a_refused_reservation_refuses_the_save_before_the_chooser() {
        let text = long_model(480);
        // The most the save holds, through Cancel.
        let pool = registry(ScriptedHost::normal());
        let opened = Cell::new(false);
        let memory = PoolReadMemory::unseeded(&pool);
        let cancelled = save_mbf_export_with(&mut in_process(&InProcessWriterTestSeam::for_tests()), request(&text), &memory, no_store, |_| { opened.set(true); Ok(None) }, &mut SystemMbfExportIo, |outcome, _| outcome.clone());
        assert_eq!(cancelled, MbfExportOutcome::Cancelled);
        let held = memory.peak_reserved();
        drop(memory);
        // The settings' parse, then the reservation that reaches the peak.
        for budget in [64 * 1024, held - 1] {
            let pool = registry(ScriptedHost::normal());
            pool.configure_pool(budget.max(1 << 20), budget, false).unwrap();
            let opened = Cell::new(false);
            let outcome = save_with(request(&text), &pool, Some(std::env::temp_dir().join("never-written.mbf")), &mut SystemMbfExportIo, &opened);
            assert_eq!(codes(&outcome), vec![MEMORY_UNAVAILABLE.to_string()], "budget {budget}");
            assert!(!opened.get(), "budget {budget}");
        }
        assert!(!std::env::temp_dir().join("never-written.mbf").exists());
    }

    /// P3-1: the memory refusal raised inside the admission's screen (`screen_model`) comes back under this command's
    /// code, like the library reads' refusals.
    #[test]
    fn a_refused_screening_reservation_uses_the_exports_memory_code() {
        let text = long_model(480);
        let parse = admission_bytes(&text);
        assert!(parse > SETTINGS_WORKING_BYTES);
        // The settings' reservation and the admission's parse fit; the screen's reservation, made with the parse held,
        // does not.
        let pool = registry(ScriptedHost::normal());
        pool.configure_pool(64 << 20, parse, false).unwrap();
        let opened = Cell::new(false);
        let outcome = save_with(request(&text), &pool, None, &mut SystemMbfExportIo, &opened);
        assert_eq!(codes(&outcome), vec![MEMORY_UNAVAILABLE.to_string()]);
        let MbfExportOutcome::Refused { diagnostics, .. } = &outcome else { unreachable!() };
        assert!(diagnostics[0].message.contains("the screening of the model document"), "{}", diagnostics[0].message);
        assert!(!opened.get());
    }

    /// The settings' parse and mapping peak within their reservation at the largest settings text admitted.
    #[test]
    fn the_settings_parse_stays_within_its_reservation() {
        let escaped = "\\u0001".repeat(mbf::MAX_EXPORT_TITLE_BYTES);
        let text = format!(r#"{{"title":"{escaped}","referenceTemperature":{{"value":20,"unit":"degC"}},"maxLayoutRows":null}}"#);
        let longest = format!("{text}{}", " ".repeat(mbf::MAX_EXPORT_SETTINGS_JSON_BYTES - text.len()));
        for (label, text) in [("escaped title", text), ("longest text", longest)] {
            let (settings, measured) = crate::ingress::counting::measure(|| writer_settings(&text).map(drop));
            assert!(settings.is_ok(), "{label}");
            eprintln!("mbf settings peak: {label} bytes={} peak_priced={}", text.len(), measured.peak_priced);
            assert!(measured.peak_priced <= SETTINGS_WORKING_BYTES, "{label}: {}", measured.peak_priced);
        }
    }

    // -- Slice 4b.1: worker task 3, its envelope, the review and the output ceiling ------------------------------------

    const REVIEW_CASES: &str = "../../../fixtures/caepipe_mbf/invented/desktop_export/review_cases.json";

    fn worker(mode: MbfMode, settings: &str, text: &str, limits: RenderLimits) -> Result<String, String> {
        worker_render_limited(&WorkerRequest { mode, settings_json: settings, model_text: text }.encode(), limits)
    }

    /// The envelope's parts: (mode, status, mbf, ids, review), read independently of the decoder.
    fn envelope_parts(result: &str) -> (String, String, String, String, String) {
        let lines: Vec<&str> = result.splitn(7, '\n').collect();
        assert_eq!(lines.len(), 7, "{}", &result[..result.len().min(200)]);
        assert_eq!(lines[0], ENVELOPE_MAGIC);
        let lengths: Vec<usize> = lines[3..6].iter().map(|line| line.parse().unwrap()).collect();
        let body = lines[6];
        assert_eq!(body.len(), lengths.iter().sum::<usize>());
        let (mbf, rest) = body.split_at(lengths[0]);
        let (ids, review) = rest.split_at(lengths[1]);
        (lines[1].into(), lines[2].into(), mbf.into(), ids.into(), review.into())
    }

    /// An envelope written by hand, with the lengths given.
    fn raw_envelope(mode: &str, status: &str, lengths: [usize; 3], body: &str) -> String {
        format!("{ENVELOPE_MAGIC}\n{mode}\n{status}\n{}\n{}\n{}\n{body}", lengths[0], lengths[1], lengths[2])
    }

    fn review_of(result: &str, mode: MbfMode) -> Value {
        serde_json::from_str(decode_worker_result(mode, result).unwrap().review).unwrap()
    }

    #[test]
    fn the_worker_request_round_trips_and_its_malformed_forms_are_refused_by_name() {
        let text = canonical_of(EXPORT_MODEL);
        let request = WorkerRequest { mode: MbfMode::Save, settings_json: &settings(), model_text: &text }.encode();
        let settings_text = settings();
        assert_eq!(decode_worker_request(&request).unwrap(), WorkerRequest { mode: MbfMode::Save, settings_json: &settings_text, model_text: &text });
        let with = |mode: u32, settings_len: u32, body: &[u8]| {
            let mut raw = mode.to_le_bytes().to_vec();
            raw.extend_from_slice(&settings_len.to_le_bytes());
            raw.extend_from_slice(body);
            raw
        };
        let settings_len = settings_text.len() as u32;
        let body = [settings_text.as_bytes(), text.as_bytes()].concat();
        let cases: Vec<(&str, Vec<u8>)> = vec![
            ("short", vec![1, 0, 0, 0, 0]),
            ("mode 0", with(0, settings_len, &body)),
            ("mode 3", with(3, settings_len, &body)),
            ("settings too long", with(1, mbf::MAX_EXPORT_SETTINGS_JSON_BYTES as u32 + 1, &body)),
            ("settings past the end", with(1, body.len() as u32 + 1, &body)),
            ("settings not UTF-8", with(1, 2, &[b"\xff\xfe".as_slice(), text.as_bytes()].concat())),
            ("model not UTF-8", with(2, settings_len, &[settings_text.as_bytes(), b"\xff"].concat())),
            ("model missing", with(2, settings_len, settings_text.as_bytes())),
        ];
        for (name, raw) in cases {
            let error = worker_render(&raw).unwrap_err();
            assert!(error.starts_with(&format!("{WORKER_REQUEST_INVALID}: ")), "{name}: {error}");
            assert!(error.len() <= MAX_WORKER_FAILURE_BYTES);
        }
        // The fields themselves are read by the engine: named refusals with no engine text.
        let error = worker(MbfMode::Review, r#"{"title":1}"#, &text, RenderLimits::PRODUCTION).unwrap_err();
        assert!(error.starts_with(&format!("{SETTINGS_INVALID}: ")), "{error}");
        let error = worker(MbfMode::Review, &settings(), "not json", RenderLimits::PRODUCTION).unwrap_err();
        assert!(error.starts_with(&format!("{MODEL_JSON_INVALID}: ")), "{error}");
        assert!(worker_failure("CODE", "x".repeat(2 * MAX_WORKER_FAILURE_BYTES)).len() <= MAX_WORKER_FAILURE_BYTES);
    }

    /// Each mode's result has its fixed shape, and decodes to views of exactly what the worker formed.
    #[test]
    fn each_modes_result_has_its_fixed_shape_and_decodes() {
        let text = canonical_of(EXPORT_MODEL);
        let review = worker(MbfMode::Review, &settings(), &text, RenderLimits::PRODUCTION).unwrap();
        let (mode, status, mbf_text, ids, review_body) = envelope_parts(&review);
        assert_eq!((mode.as_str(), status.as_str(), mbf_text.as_str(), ids.as_str()), ("review", "reviewed", "", ""));
        let decoded = decode_worker_result(MbfMode::Review, &review).unwrap();
        assert_eq!((decoded.status, decoded.mbf, decoded.identity_file, decoded.review), (WorkerStatus::Reviewed, "", "", review_body.as_str()));
        let save = worker(MbfMode::Save, &settings(), &text, RenderLimits::PRODUCTION).unwrap();
        let (mode, status, mbf_text, ids, save_review) = envelope_parts(&save);
        assert_eq!((mode.as_str(), status.as_str()), ("save", "written"));
        // The same review in both modes.
        assert_eq!(save_review, review_body);
        let decoded = decode_worker_result(MbfMode::Save, &save).unwrap();
        assert_eq!((decoded.mbf, decoded.identity_file), (mbf_text.as_str(), ids.as_str()));
        assert_eq!(decoded.mbf_sha256.as_deref(), Some(sha256_hex(mbf_text.as_bytes()).as_str()));
        let parsed: Value = serde_json::from_str(&review_body).unwrap();
        assert_eq!(parsed["mbf_sha256"], json!(decoded.mbf_sha256));
        assert_eq!(parsed["sidecar_sha256"], json!(decoded.sidecar_sha256));
        assert_eq!(save.len() as u64, envelope_length(MbfMode::Save, WorkerStatus::Written, mbf_text.len() as u64, ids.len() as u64, review_body.len() as u64));
        // No engine message text in either result.
        let model = mbf::parse_sidecar_json(text.as_bytes()).unwrap();
        let result = mbf::render_si(&model, &writer_settings(&settings()).unwrap()).unwrap();
        for item in result.diagnostics.iter().chain(&result.loss_report) {
            for key in ["message", "reason"] {
                let message = item.get_or_null(key).as_str().unwrap_or("");
                if message.len() > 12 {
                    assert!(!review.contains(message) && !save_review.contains(message), "{message}");
                }
            }
        }
    }

    /// The parent refuses every malformed envelope, and a result whose shape does not match the requested mode.
    #[test]
    fn every_malformed_or_mismatched_envelope_is_refused() {
        let text = canonical_of(EXPORT_MODEL);
        let review = worker(MbfMode::Review, &settings(), &text, RenderLimits::PRODUCTION).unwrap();
        let save = worker(MbfMode::Save, &settings(), &text, RenderLimits::PRODUCTION).unwrap();
        let (_, _, mbf_text, ids, body) = envelope_parts(&save);
        let truncated_review = {
            let full = review_of(&review, MbfMode::Review).to_string();
            worker(MbfMode::Save, &settings(), &text, RenderLimits::for_tests(full.len() - 1, mbf::MBF_EXPORT_OUTPUT_MAX_BYTES)).unwrap()
        };
        let (_, _, _, _, truncated_body) = envelope_parts(&truncated_review);
        let both = format!("{mbf_text}{ids}{body}");
        let mut tampered = mbf_text.clone().into_bytes();
        tampered[0] ^= 1;
        let tampered = format!("{}{ids}{body}", String::from_utf8(tampered).unwrap());
        let cases: Vec<(&str, MbfMode, String)> = vec![
            ("a review result for a save", MbfMode::Save, review.clone()),
            ("a save result for a review", MbfMode::Review, save.clone()),
            ("a review result carrying MBF bytes", MbfMode::Review, raw_envelope("review", "reviewed", [mbf_text.len(), ids.len(), body.len()], &both)),
            ("a save status in a review", MbfMode::Review, raw_envelope("review", "written", [mbf_text.len(), ids.len(), body.len()], &both)),
            ("a review status in a save", MbfMode::Save, raw_envelope("save", "reviewed", [0, 0, body.len()], &body)),
            ("written without files", MbfMode::Save, raw_envelope("save", "written", [0, 0, body.len()], &body)),
            ("written without the identity file", MbfMode::Save, raw_envelope("save", "written", [mbf_text.len(), 0, ids.len() + body.len()], &both)),
            ("a refusal carrying files", MbfMode::Save, raw_envelope("save", "writer-refused", [mbf_text.len(), ids.len(), body.len()], &both)),
            ("another magic", MbfMode::Save, save.replacen(ENVELOPE_MAGIC, "MBF2", 1)),
            ("an unknown status", MbfMode::Save, raw_envelope("save", "saved", [mbf_text.len(), ids.len(), body.len()], &both)),
            ("a leading zero", MbfMode::Save, save.replacen(&format!("\n{}\n", mbf_text.len()), &format!("\n0{}\n", mbf_text.len()), 1)),
            ("a signed length", MbfMode::Review, review.replacen("\n0\n0\n", "\n+0\n0\n", 1)),
            ("trailing bytes", MbfMode::Save, format!("{save} ")),
            ("missing bytes", MbfMode::Save, save[..save.len() - 1].to_string()),
            ("an incomplete header", MbfMode::Save, "MBF1\nsave\n".to_string()),
            ("an overlong header line", MbfMode::Save, raw_envelope("save", &"w".repeat(40), [0, 0, body.len()], &body)),
            ("an empty review", MbfMode::Review, raw_envelope("review", "reviewed", [0, 0, 0], "")),
            ("a review above R", MbfMode::Review, raw_envelope("review", "reviewed", [0, 0, mbf::MBF_REVIEW_MAX_BYTES + 1], &" ".repeat(mbf::MBF_REVIEW_MAX_BYTES + 1))),
            ("a review that is not JSON", MbfMode::Review, raw_envelope("review", "reviewed", [0, 0, 4], "nope")),
            ("a review of another kind", MbfMode::Review, raw_envelope("review", "reviewed", [0, 0, 2], "{}")),
            ("a length inside a character", MbfMode::Save, raw_envelope("save", "written", [1, ids.len() + 1, body.len()], &format!("é{ids}{body}"))),
            ("files with another digest", MbfMode::Save, raw_envelope("save", "written", [mbf_text.len(), ids.len(), body.len()], &tampered)),
            ("written with a truncated review", MbfMode::Save, raw_envelope("save", "written", [mbf_text.len(), ids.len(), truncated_body.len()], &format!("{mbf_text}{ids}{truncated_body}"))),
            ("a truncation claimed of a complete review", MbfMode::Save, raw_envelope("save", "review-truncated", [0, 0, body.len()], &body)),
            ("a ceiling refusal of a review within it", MbfMode::Save, raw_envelope("save", "output-too-large", [0, 0, body.len()], &body)),
            ("a writer refusal of a written review", MbfMode::Save, raw_envelope("save", "writer-refused", [0, 0, body.len()], &body)),
        ];
        for (name, mode, result) in cases {
            let refusal = decode_worker_result(mode, &result).map(|decoded| decoded.status).unwrap_err();
            assert_eq!(refusal.code, WORKER_PROTOCOL, "{name}");
        }
        // The valid forms decode.
        assert_eq!(decode_worker_result(MbfMode::Save, &truncated_review).unwrap().status, WorkerStatus::ReviewTruncated);
        assert_eq!(decode_worker_result(MbfMode::Save, &save).unwrap().status, WorkerStatus::Written);
    }

    /// A recording run: counts its runs and passes them to task 3 in process.
    struct Recording<'s> {
        inner: InProcessWriterRun<'s>,
        runs: Vec<MbfMode>,
    }

    impl MbfWriterRun for Recording<'_> {
        fn run(&mut self, request: &WorkerRequest<'_>) -> Result<String, WriterRunFailure> {
            self.runs.push(request.mode);
            self.inner.run(request)
        }
    }

    fn review_request(text: &str, settings: &str) -> MbfReviewRequest {
        MbfReviewRequest {
            canonical_text: text.to_string(),
            claimed_model_hash: format!("sha256:{}", sha256_hex(text.as_bytes())),
            settings_json: settings.to_string(),
        }
    }

    /// The review through the test seam, with every reservation released after its response.
    fn review_with(run: &mut impl MbfWriterRun, request: MbfReviewRequest) -> (MbfReviewOutcome, String) {
        let pool = registry(ScriptedHost::normal());
        let memory = PoolReadMemory::unseeded(&pool);
        let (outcome, text) = review_mbf_export_with(run, request, &memory, |outcome, text| (outcome.clone(), text));
        assert_eq!(serde_json::to_string(&outcome).unwrap(), text);
        drop(memory);
        assert_eq!(pool.pool_status().reserved_bytes, 0, "every reservation is released");
        (outcome, text)
    }

    fn reviewed_json(outcome: &MbfReviewOutcome) -> Value {
        match outcome {
            MbfReviewOutcome::Reviewed { review } => serde_json::from_str(review.get()).unwrap(),
            other => panic!("not reviewed: {other:?}"),
        }
    }

    /// The review admits as the save does, runs task 3 in `review` mode, touches no file, store or chooser (it has
    /// none to touch) and writes nothing; its digests are the save's and the recorded ones. Its admission refuses
    /// before any worker run.
    #[test]
    fn a_review_admits_like_the_save_and_touches_nothing() {
        let sandbox = Sandbox::new("review");
        let text = canonical_of(EXPORT_MODEL);
        let seam = InProcessWriterTestSeam::for_tests();
        let mut run = Recording { inner: in_process(&seam), runs: Vec::new() };
        let (outcome, response) = review_with(&mut run, review_request(&text, &settings()));
        assert_eq!(run.runs, vec![MbfMode::Review]);
        let review = reviewed_json(&outcome);
        let recorded: Value = serde_json::from_str(EXPORT_DIGESTS).unwrap();
        assert_eq!(review["mbf_sha256"], recorded["cases"]["degc-default"]["mbf_sha256"]);
        assert_eq!(review["sidecar_sha256"], recorded["cases"]["degc-default"]["sidecar_sha256"]);
        assert_eq!((review["completeness"].as_str(), review["output_ceiling"].as_str(), review["blocking"].as_bool()), (Some("complete"), Some("within"), Some(false)));
        // The reply is `{outcome: "reviewed", review}`, the review verbatim.
        let reply: Value = serde_json::from_str(&response).unwrap();
        assert_eq!(reply["outcome"], "reviewed");
        assert_eq!(reply["review"], review);
        assert!(sandbox.entries().is_empty());
        // Refusals before the worker runs.
        let cases: Vec<(&str, MbfReviewRequest)> = vec![
            (SETTINGS_INVALID, review_request(&text, &settings_text("T", "20", "K"))),
            (SETTINGS_INVALID, MbfReviewRequest { canonical_text: "not json".into(), ..review_request(&text, &settings_text(&"a".repeat(4097), "20", "degC")) }),
            ("MODEL-DOC-SAVE-HASH-MISMATCH", MbfReviewRequest { claimed_model_hash: format!("sha256:{}", sha256_hex(b"{}")), ..review_request(&text, &settings()) }),
            ("MODEL-DOC-SAVE-HASH-MISMATCH", review_request(&format!("{text}\n"), &settings())),
        ];
        for (code, request) in cases {
            let mut run = Recording { inner: in_process(&seam), runs: Vec::new() };
            let (outcome, _) = review_with(&mut run, request);
            let MbfReviewOutcome::Refused { diagnostics } = &outcome else { panic!("{code}: {outcome:?}") };
            assert_eq!(diagnostics[0].code, code);
            assert!(run.runs.is_empty(), "{code}: no worker run");
        }
        // A protected disposition is not the review's to screen (§3.9.5): the save refuses it (step 4).
        let protected = edited(|document| document["materials"][0]["provenance_record"]["review_status"] = json!("quarantined"));
        let (outcome, _) = review_with(&mut in_process(&seam), review_request(&protected, &settings()));
        assert!(matches!(outcome, MbfReviewOutcome::Reviewed { .. }), "{outcome:?}");
        assert!(sandbox.entries().is_empty());
    }

    /// A truncated review blocks the save even with no blocking entry: the save's own run refuses before the files
    /// leave the worker, before any chooser or write; an earlier pair is untouched.
    #[test]
    fn a_truncated_review_refuses_the_save_before_any_chooser_or_write() {
        let _lock = atomic_save_test_lock();
        let sandbox = Sandbox::new("truncated");
        let earlier = earlier_pair(&sandbox);
        let text = canonical_of(EXPORT_MODEL);
        let full = review_of(&worker(MbfMode::Review, &settings(), &text, RenderLimits::PRODUCTION).unwrap(), MbfMode::Review).to_string();
        let limits = RenderLimits::for_tests(full.len() - 1, mbf::MBF_EXPORT_OUTPUT_MAX_BYTES);
        // The worker's save result carries the review only.
        let result = worker(MbfMode::Save, &settings(), &text, limits).unwrap();
        let (_, status, mbf_text, ids, body) = envelope_parts(&result);
        assert_eq!((status.as_str(), mbf_text.as_str(), ids.as_str()), ("review-truncated", "", ""));
        let body: Value = serde_json::from_str(&body).unwrap();
        assert_eq!((body["completeness"].as_str(), body["blocking"].as_bool(), body["not_shown_blocking"].as_u64()), (Some("truncated"), Some(false), Some(0)));
        assert!(body["not_shown"].as_u64().unwrap() >= 1);
        // The save refuses with the review whatever the request claims about it: here its digests are the full
        // review's.
        let seam = InProcessWriterTestSeam::for_tests();
        let opened = Cell::new(false);
        let pool = registry(ScriptedHost::normal());
        let memory = PoolReadMemory::unseeded(&pool);
        let outcome = save_mbf_export_with(
            &mut InProcessWriterRun::new(&seam, limits),
            request(&text),
            &memory,
            no_store,
            |_| {
                opened.set(true);
                Ok(Some(earlier.0.clone()))
            },
            &mut SystemMbfExportIo,
            |outcome, _| outcome.clone(),
        );
        drop(memory);
        assert_eq!(pool.pool_status().reserved_bytes, 0);
        assert_eq!(codes(&outcome), vec![REVIEW_TRUNCATED.to_string()]);
        assert_eq!(refusal_review(&outcome)["completeness"], "truncated");
        assert!(!opened.get());
        assert_earlier_pair(&sandbox, "truncated", &earlier);
    }

    /// The output ceiling is decided from the computed lengths: a result exactly at the ceiling is saved, one byte
    /// above is refused before the result is built (only the review leaves the worker), before any chooser or write.
    /// A small ceiling stands for 64 MiB.
    #[test]
    fn the_output_ceiling_refuses_from_the_lengths_and_admits_at_the_limit() {
        let _lock = atomic_save_test_lock();
        let text = canonical_of(EXPORT_MODEL);
        let at_limit = worker(MbfMode::Save, &settings(), &text, RenderLimits::PRODUCTION).unwrap().len() as u64;
        let fits = RenderLimits::for_tests(mbf::MBF_REVIEW_MAX_BYTES, at_limit);
        let over = RenderLimits::for_tests(mbf::MBF_REVIEW_MAX_BYTES, at_limit - 1);
        assert_eq!(decode_worker_result(MbfMode::Save, &worker(MbfMode::Save, &settings(), &text, fits).unwrap()).unwrap().status, WorkerStatus::Written);
        let refused = worker(MbfMode::Save, &settings(), &text, over).unwrap();
        let (_, status, mbf_text, ids, body) = envelope_parts(&refused);
        assert_eq!((status.as_str(), mbf_text.as_str(), ids.as_str()), ("output-too-large", "", ""));
        assert_eq!(refused.len() as u64, envelope_length(MbfMode::Save, WorkerStatus::OutputTooLarge, 0, 0, body.len() as u64));
        // The review reports it in both modes.
        assert_eq!(review_of(&worker(MbfMode::Review, &settings(), &text, over).unwrap(), MbfMode::Review)["output_ceiling"], "beyond");
        assert_eq!(review_of(&worker(MbfMode::Review, &settings(), &text, fits).unwrap(), MbfMode::Review)["output_ceiling"], "within");
        // The envelope itself is never built above the ceiling.
        assert!(envelope(MbfMode::Save, WorkerStatus::Written, "a", "b", "{}", 10).unwrap_err().starts_with(OUTPUT_TOO_LARGE));
        let seam = InProcessWriterTestSeam::for_tests();
        for (limits, saved) in [(over, false), (fits, true)] {
            let sandbox = Sandbox::new("ceiling");
            let opened = Cell::new(false);
            let outcome = save_with_run(&mut InProcessWriterRun::new(&seam, limits), request(&text), Some(sandbox.0.join("swbpipe-model.mbf")), &opened);
            if saved {
                assert!(matches!(outcome, MbfExportOutcome::Saved { .. }), "{outcome:?}");
                assert_eq!(fs::read(sandbox.0.join("swbpipe-model.mbf")).unwrap(), engine_files(&text, &settings()).mbf);
            } else {
                assert_eq!(codes(&outcome), vec![OUTPUT_TOO_LARGE.to_string()]);
                assert_eq!(refusal_review(&outcome)["output_ceiling"], "beyond");
                assert!(!opened.get());
                assert!(sandbox.entries().is_empty());
            }
        }
    }

    fn save_with_run(run: &mut impl MbfWriterRun, request: MbfExportRequest, chosen: Option<PathBuf>, opened: &Cell<bool>) -> MbfExportOutcome {
        let pool = registry(ScriptedHost::normal());
        let memory = PoolReadMemory::unseeded(&pool);
        let outcome = save_mbf_export_with(run, request, &memory, no_store, |_| {
            opened.set(true);
            Ok(chosen)
        }, &mut SystemMbfExportIo, |outcome, text| {
            assert_eq!(serde_json::to_string(outcome).unwrap(), text);
            outcome.clone()
        });
        drop(memory);
        assert_eq!(pool.pool_status().reserved_bytes, 0);
        outcome
    }

    /// A save after a complete review sends the review's digests and writes the engine's bytes from its own run.
    #[test]
    fn a_save_after_a_complete_review_writes_the_engines_bytes() {
        let _lock = atomic_save_test_lock();
        let sandbox = Sandbox::new("after-review");
        let text = edited(|document| document["project"]["units"]["temperature"] = json!("degF"));
        let settings = settings_text("Invented MBF export", "68", "degF");
        let seam = InProcessWriterTestSeam::for_tests();
        let (outcome, _) = review_with(&mut in_process(&seam), review_request(&text, &settings));
        let review = reviewed_json(&outcome);
        assert_eq!(review["completeness"], "complete");
        let reviewed = MbfReviewed { mbf_sha256: review["mbf_sha256"].as_str().unwrap().into(), sidecar_sha256: review["sidecar_sha256"].as_str().unwrap().into() };
        let chosen = sandbox.0.join("swbpipe-model.mbf");
        let mut run = Recording { inner: in_process(&seam), runs: Vec::new() };
        let request = MbfExportRequest { settings_json: settings.clone(), reviewed, ..request_unreviewed(&text) };
        let opened = Cell::new(false);
        let outcome = save_with_run(&mut run, request, Some(chosen.clone()), &opened);
        assert!(matches!(outcome, MbfExportOutcome::Saved { .. }), "{outcome:?}");
        assert_eq!(run.runs, vec![MbfMode::Save], "the save runs the writer itself");
        let files = engine_files(&text, &settings);
        assert_eq!(fs::read(&chosen).unwrap(), files.mbf);
        assert_eq!(fs::read(sandbox.0.join("swbpipe-model.swbpipe-ids.json")).unwrap(), files.sidecar);
        assert_eq!(sha256_hex(&files.mbf), review["mbf_sha256"].as_str().unwrap());
    }

    /// The worker's failures, by name: memory, the output ceiling at `result_limit`, protocol, cancel, an unnamed
    /// failure; in the review the memory and ceiling refusals are the review's memory refusal.
    #[test]
    fn worker_run_failures_are_named_in_the_save_and_the_review() {
        struct Failing(WriterRunFailure);
        impl MbfWriterRun for Failing {
            fn run(&mut self, _: &WorkerRequest<'_>) -> Result<String, WriterRunFailure> {
                Err(self.0.clone())
            }
        }
        let text = canonical_of(EXPORT_MODEL);
        let cases = [
            (WriterRunFailure::Memory("the grant was refused".into()), MEMORY_UNAVAILABLE, REVIEW_MEMORY_UNAVAILABLE),
            (WriterRunFailure::OutputTooLarge, OUTPUT_TOO_LARGE, REVIEW_MEMORY_UNAVAILABLE),
            (WriterRunFailure::Protocol("a short frame".into()), WORKER_PROTOCOL, WORKER_PROTOCOL),
            (WriterRunFailure::Cancelled, CANCELLED, CANCELLED),
            (WriterRunFailure::Failed("no code here".into()), WORKER_PROTOCOL, WORKER_PROTOCOL),
            (WriterRunFailure::Failed(format!("{ENGINE_FAILED}: The MBF engine returned no result (KeyError).")), ENGINE_FAILED, ENGINE_FAILED),
        ];
        for (failure, save_code, review_code) in cases {
            let opened = Cell::new(false);
            let outcome = save_with_run(&mut Failing(failure.clone()), request(&text), None, &opened);
            assert_eq!(codes(&outcome), vec![save_code.to_string()], "{failure:?}");
            assert!(!opened.get());
            let (outcome, _) = review_with(&mut Failing(failure.clone()), review_request(&text, &settings()));
            let MbfReviewOutcome::Refused { diagnostics } = outcome else { panic!("{failure:?}") };
            assert_eq!(diagnostics[0].code, review_code, "{failure:?}");
        }
    }

    /// The refusal before the chooser carries the code and the bounded review only, verbatim, within the response
    /// bound even at R; the short form drops the review.
    #[test]
    fn the_refusal_reply_is_bounded_and_carries_only_the_review() {
        let text = edited(|document| document["components"][0]["kind"] = json!("tee"));
        let opened = Cell::new(false);
        let outcome = save(request_unreviewed(&text), None, &opened);
        let response = serde_json::to_value(&outcome).unwrap();
        assert_eq!(response["outcome"], "refused");
        assert_eq!(response["diagnostics"].as_array().unwrap().len(), 1);
        assert_eq!(response["diagnostics"][0]["code"], WRITER_REFUSED);
        assert_eq!(response["review"]["document_kind"], mbf::REVIEW_DOCUMENT_KIND);
        let MbfExportOutcome::Refused { review: Some(review), .. } = &outcome else { unreachable!() };
        assert!(review.get().len() <= mbf::MBF_REVIEW_MAX_BYTES);
        // A review of R bytes still fits a response, and is sent whole.
        let padding = mbf::MBF_REVIEW_MAX_BYTES - 2;
        let largest = MbfExportOutcome::Refused {
            diagnostics: vec![diagnostic(REVIEW_TRUNCATED, "", "x".repeat(4096))],
            review: Some(MbfReviewText::new(&format!("\"{}\"", "\\\\".repeat(padding / 2))).unwrap()),
        };
        let pool = registry(ScriptedHost::normal());
        let memory = PoolReadMemory::unseeded(&pool);
        let (sent, sent_text, buffer) = serialize_outcome(&memory, "Export MBF", "save", largest, None);
        assert!(matches!(&sent, MbfExportOutcome::Refused { review: Some(_), .. }));
        assert!(sent_text.len() as u64 <= crate::stored_model_document_export::MAX_RESPONSE_BYTES);
        drop((sent, buffer));
        // The short form drops it.
        let MbfExportOutcome::Refused { review, .. } = outcome.shortened("refused".into(), "save", 1) else { unreachable!() };
        assert!(review.is_none());
    }

    /// The review's production entry point refuses before anything else and releases what it held.
    #[test]
    fn the_reviews_production_entry_point_refuses_not_connected() {
        let text = canonical_of(EXPORT_MODEL);
        let body = json!({"canonicalText": text, "claimedModelHash": format!("sha256:{}", sha256_hex(text.as_bytes())), "settingsJson": settings()});
        let pool = registry(ScriptedHost::normal());
        let (mut args, lease) = admit_json(&pool, REVIEW_COMMAND, ArgsDecoding::Moved, &body).unwrap();
        let decoded = review_mbf_export_args(&mut args).unwrap();
        assert_eq!((decoded.canonical_text.as_str(), decoded.settings_json.as_str()), (text.as_str(), settings().as_str()));
        let (outcome, response) = review_mbf_export_admitted(decoded, &pool, &lease, |outcome, text| (outcome.clone(), text));
        let MbfReviewOutcome::Refused { diagnostics } = &outcome else { panic!("{outcome:?}") };
        assert_eq!(diagnostics[0].code, NOT_CONNECTED);
        assert_eq!(serde_json::to_string(&outcome).unwrap(), response);
        let raw = serde_json::to_vec(&body).unwrap();
        let _ = check_decode_bound(&raw, ArgsDecoding::Moved, |args| review_mbf_export_args(args).map(drop));
        drop(lease);
        assert_eq!(pool.pool_status().reserved_bytes, 0);
    }

    /// A string longer than the per-field bound is carried as its prefix, length and digest, and the review's counted
    /// peak holds no copy of the whole string, escaped or not.
    #[test]
    fn a_long_string_is_reviewed_without_a_copy_of_itself() {
        let model = mbf::parse_sidecar_json(canonical_of(EXPORT_MODEL).as_bytes()).unwrap();
        let mut result = mbf::render_si(&model, &writer_settings(&settings()).unwrap()).unwrap();
        // 4 MiB of control characters: 24 MiB escaped.
        let entity = "\u{1}".repeat(4 << 20);
        let mut item = mbf::Dict::new();
        for (key, value) in [("entity", entity.as_str()), ("category", "omitted"), ("code", "MBF-SI-FIELD-NOT-EXPORTED"), ("reason", "r")] {
            item.insert(key.to_string(), mbf::Value::str(value));
        }
        result.loss_report.push(mbf::Value::Dict(item));
        let (review, measured) = crate::ingress::counting::measure(|| mbf::export_review(&result, None, mbf::MBF_REVIEW_MAX_BYTES, |_| true).unwrap());
        eprintln!("mbf review long-string peak: entity={} peak_requested={} review={}", entity.len(), measured.peak_requested, review.text.len());
        assert!(measured.peak_requested < 3 * mbf::MBF_REVIEW_MAX_BYTES as u64, "{measured:?}");
        assert!((measured.peak_requested as usize) < entity.len());
        let parsed: Value = serde_json::from_str(&review.text).unwrap();
        let carried = parsed["entries"].as_array().unwrap().iter().find(|entry| entry["entity"].is_object()).unwrap();
        assert_eq!(carried["entity"]["bytes"], entity.len());
        assert_eq!(carried["entity"]["sha256"], sha256_hex(entity.as_bytes()));
        assert_eq!(carried["entity"]["prefix"].as_str().unwrap().len(), mbf::REVIEW_FIELD_MAX_BYTES);
    }

    /// The dialog's review payloads, generated from the engine through task 3 (`review` mode) for the invented export
    /// fixture and its variants: `review_cases.json`, read by `MbfExportDialog.test.tsx`. Regenerate with
    /// `MBF_REVIEW_CASES_REGENERATE=1`; otherwise the committed file must equal what the worker forms now.
    #[test]
    fn the_dialogs_review_cases_are_the_workers() {
        let default = canonical_of(EXPORT_MODEL);
        let degf = edited(|document| document["project"]["units"]["temperature"] = json!("degF"));
        let tee = edited(|document| document["components"][0]["kind"] = json!("tee"));
        let far = edited(|document| {
            let last = document["nodes"].as_array().unwrap().len() - 1;
            document["nodes"][last]["position"]["z"] = json!(1e306);
        });
        let title = "Invented MBF export";
        let full = worker(MbfMode::Review, &settings(), &default, RenderLimits::PRODUCTION).unwrap();
        let full_len = decode_worker_result(MbfMode::Review, &full).unwrap().review.len();
        let at_limit = worker(MbfMode::Save, &settings(), &default, RenderLimits::PRODUCTION).unwrap().len() as u64;
        let production = RenderLimits::PRODUCTION;
        let cases: Vec<(&str, &str, &String, String, RenderLimits)> = vec![
            ("degc-default", "none", &default, settings(), production),
            ("degc-fractional", "none", &default, settings_text(title, "21.5", "degC"), production),
            ("degc-another-title", "none", &default, settings_text("Another title", "21.5", "degC"), production),
            ("degf-project", "project.units.temperature = degF", &degf, settings_text(title, "68", "degF"), production),
            ("refused-component", "components[0].kind = tee", &tee, settings(), production),
            ("refused-unpaired", "nodes[last].position.z = 1e306", &far, settings(), production),
            ("truncated", "none (review allowance one byte below the full review)", &default, settings(), RenderLimits::for_tests(full_len - 1, mbf::MBF_EXPORT_OUTPUT_MAX_BYTES)),
            ("output-beyond", "none (output ceiling one byte below the save result)", &default, settings(), RenderLimits::for_tests(mbf::MBF_REVIEW_MAX_BYTES, at_limit - 1)),
        ];
        let mut out = serde_json::Map::new();
        for (name, edit, text, settings, limits) in cases {
            let result = worker(MbfMode::Review, &settings, text, limits).unwrap();
            let review = decode_worker_result(MbfMode::Review, &result).unwrap().review.to_string();
            out.insert(name.to_string(), json!({
                "model_edit": edit,
                "model_sha256": sha256_hex(text.as_bytes()),
                "settings_json": settings,
                "review_allowance": limits.review_allowance,
                "output_ceiling": limits.output_ceiling,
                "review_text": review,
            }));
        }
        let document = json!({
            "description": "Bounded MBF export reviews (DEL-17-04 §3.9.5) formed by the desktop's worker task 3 in review mode for the invented export model and its variants; generated by mbf_export.rs the_dialogs_review_cases_are_the_workers",
            "cases": out,
        });
        let text = format!("{}\n", serde_json::to_string_pretty(&document).unwrap());
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(REVIEW_CASES);
        if std::env::var_os("MBF_REVIEW_CASES_REGENERATE").is_some() {
            fs::write(&path, &text).unwrap();
        }
        assert_eq!(fs::read_to_string(&path).unwrap_or_default(), text, "regenerate with MBF_REVIEW_CASES_REGENERATE=1");
        // The variants are the shapes the dialog test needs.
        let review = |name: &str| -> Value { serde_json::from_str(document["cases"][name]["review_text"].as_str().unwrap()).unwrap() };
        assert_eq!(review("truncated")["completeness"], "truncated");
        assert_eq!(review("truncated")["blocking"], false);
        assert_eq!(review("output-beyond")["output_ceiling"], "beyond");
        assert_eq!(review("refused-component")["blocking"], true);
        assert!(review("degf-project")["unit_conversions"].as_array().is_some_and(|items| !items.is_empty()));
    }
}
