//! File > Export MBF… (DEL-17-04 Design `desktop-mbf-export-import.md` §3.5–§3.7, slice 4a; RT-R7): the native
//! save of a model batch file and its identity file.
//!
//! The webview reviews the export with the wasm writer (`core/handoff/caepipe_mbf_wasm`) and sends the session's
//! canonical model text, its claimed hash, the settings text it reviewed with, the suggested name, the digests it
//! reviewed and its route evidence. The settings are read first, by the engine's one mapping (a title over 4 KiB is
//! refused before the model text is copied). This module then runs, in order (§3.6):
//!
//! 1. the route evidence of the governed route `DOTH-MBF-012` (local-private, explicit intent, lossless, exact
//!    payload; A-4), bound to the SHA-256 of the canonical text;
//! 2. the 64 MiB open cap, re-canonicalization and the hash claim, and
//! 3. the open path's document checks, and
//! 4. the disposition screen of the exact admitted document (a protected disposition refuses
//!    `PROTECTED_CONTENT_BLOCKED`), all three by Save Model Document's admission
//!    (`model_document_save::admit_model_document_text_screened`), so an MBF is only ever written from a document
//!    the desktop opens and may save;
//! 5. the Rust writer (`render_si`, no row limit): a refusal returns the writer's diagnostics before any chooser;
//! 6. the staleness check: the MBF's SHA-256 and the identity file's SHA-256 must equal the reviewed ones
//!    (`MBF-EXPORT-REVIEW-STALE`);
//! 7. the chooser: the suggested name must be `swbpipe-model[-<slug>].mbf` (DEL-07-01 `export-file-names.md`), the
//!    path comes only from the chooser and must end `.mbf`; the identity file is `<chosen stem>.swbpipe-ids.json`
//!    beside it (A-5);
//! 8. the writes. First, before anything is written: both destinations pass the atomic writer's own checks
//!    (extension, parent, no symlink or directory, and off macOS no replacement); an identity file whose MBF is absent
//!    is refused (`MBF-EXPORT-IDENTITY-FILE-EXISTS`); and every file to be replaced is backed up by a hard link to a
//!    hidden name beside it (`.swbpipe-mbf-<hex>.bak`), or the save is refused (`MBF-EXPORT-REPLACE-BACKUP-UNAVAILABLE`).
//!    Then the identity file, then the MBF, each by the atomic writer under its own format ([`MBF_IDS_FORMAT`],
//!    [`MBF_FORMAT`]). After any failure the save is undone: what it wrote is removed and each backup renamed back,
//!    each only after an identity check; a destination that cannot be restored keeps its backup, named in the refusal
//!    (`MBF-EXPORT-ROLLBACK-INCOMPLETE`). On success the backups are removed (a failure there is a warning);
//! 9. the read-back of both in fixed blocks, compared by SHA-256; a mismatch undoes the save
//!    (`MBF-EXPORT-READBACK-MISMATCH`);
//! 10. the receipt: both names, byte counts and digests, and the atomic writer's durability statement.
//!
//! The bytes written are the native writer's output for the hash-checked text; the webview's text is used for the
//! review only. Nothing is redacted: a screening that would alter the payload blocks the save instead.
//!
//! Not connected (§3.7). The in-process writer's memory is not governed: its peak grows with the square of the input
//! (long ids times copies per record, long ids times unknown fields), so no per-byte reservation bounds it. Native
//! writer memory is to be governed by running the writer in the capped worker process (slice 4b). Until 4b's worker
//! path is verified, native writer memory governance and the native export are unfinished, and the production entry
//! point [`save_mbf_export_admitted`] refuses every request with [`NOT_CONNECTED`] before any parse, writer run or
//! file access. Steps 1 to 10 above run only through [`InProcessWriterTestSeam`], which only test code can construct.
//!
//! Host-side memory (§3.7), which 4b keeps: each of these is reserved before it is made, within the caller's memory
//! ([`PoolReadMemory`], seeded with the ingress lease): the settings' parse, the admission's parse and canonical copy,
//! the MBF and identity-file bytes, the response (bounded for a receipt and for a refusal after a write, so a
//! rollback's named backups always reach the user) and a 64 KiB read-back block. A reservation the pool refuses refuses
//! the save (`MBF-EXPORT-MEMORY-UNAVAILABLE`), before any file is chosen. The writer's own run reserves nothing here.
//!
//! The command's registration, ingress admission and menu are slice 4b. Nothing is sent anywhere (F-PIP-1).
//! Standard claim fence applies (claims taxonomy per DEC-081): a written MBF means the listed subset of the model
//! was written; it is not evidence that another program reads or analyses it alike.

// Slice 4b registers the command; until then nothing outside this module's tests calls it.
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

/// The test seam of the in-process writer pipeline (steps 1 to 10). Its only constructor is compiled for tests, so no
/// production code can run the in-process writer or touch a file through this module: the production entry point is
/// [`save_mbf_export_admitted`], which refuses. The writer's memory is not governed in process (§3.7); slice 4b runs it
/// in the capped worker process.
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
    },
}

impl MbfExportOutcome {
    pub fn refused(diagnostics: Vec<Diagnostic>) -> Self {
        Self::Refused { diagnostics }
    }

    pub fn refused_one(code: &str, message: impl Into<String>) -> Self {
        Self::Refused { diagnostics: vec![diagnostic(code, "", message)] }
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
            MbfExportOutcome::Refused { diagnostics } => {
                let listed = diagnostics.len();
                (MbfExportOutcome::Refused { diagnostics: bounded_diagnostics(diagnostics) }, listed)
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
            MbfExportOutcome::Refused { diagnostics } => {
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

/// The writer's file contents for an admitted text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MbfFiles {
    pub mbf: Vec<u8>,
    pub mbf_sha256: String,
    pub sidecar: Vec<u8>,
    pub sidecar_sha256: String,
}

/// The writer's own refusal: its error diagnostics, by their codes, entities and messages.
fn writer_refusal(result: &mbf::WriterResult) -> Vec<Diagnostic> {
    let text = |item: &mbf::Value, key: &str| item.get_or_null(key).as_str().unwrap_or("").to_string();
    let mut diagnostics: Vec<Diagnostic> = result
        .diagnostics
        .iter()
        .filter(|item| item.get_or_null("severity").as_str() == Some("error"))
        .map(|item| Diagnostic { code: text(item, "code"), path: text(item, "entity"), message: text(item, "message") })
        .collect();
    if diagnostics.is_empty() {
        diagnostics.push(diagnostic(ENGINE_FAILED, "", "The MBF writer refused the model without naming an error."));
    }
    diagnostics
}

/// Step 5: runs the writer over `text` (read as the engine reads JSON, as the webview wrapper does), returning the
/// written files or the writer's refusal. In process, so only through the test seam; nothing reserves its memory.
fn render_mbf_files(_seam: &InProcessWriterTestSeam, text: &str, settings: &mbf::WriterSettings) -> Result<MbfFiles, Vec<Diagnostic>> {
    let engine_failed = |error: mbf::ImportError| vec![diagnostic(ENGINE_FAILED, "", format!("The MBF engine returned no result ({error})."))];
    let model = mbf::parse_sidecar_json(text.as_bytes())
        .map_err(|error| vec![diagnostic(MODEL_JSON_INVALID, "", format!("The model text is not JSON the MBF engine reads ({error:?})."))])?;
    let result = mbf::render_si(&model, settings).map_err(engine_failed)?;
    mbf::value::dispose(model);
    let Some(mbf_text) = result.text.as_ref().filter(|_| result.status == "written") else {
        return Err(writer_refusal(&result));
    };
    let mbf_sha256 = sha256_hex(mbf_text.as_bytes());
    let sidecar = result.sidecar_bytes(&mbf_sha256).map_err(engine_failed)?;
    let sidecar_sha256 = sha256_hex(&sidecar);
    let mbf_bytes = mbf_text.as_bytes().to_vec();
    drop(result);
    Ok(MbfFiles { mbf: mbf_bytes, mbf_sha256, sidecar, sidecar_sha256 })
}

/// Step 6: the files must be the reviewed ones.
pub fn check_reviewed(files: &MbfFiles, reviewed: &MbfReviewed) -> Result<(), Diagnostic> {
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
fn write_pair(io: &mut impl MbfExportIo, chosen: &Path, files: &MbfFiles, block: &mut [u8], mut warnings: Vec<Diagnostic>) -> MbfExportOutcome {
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
    match write_one(io, &MBF_IDS_FORMAT, &ids.path, &files.sidecar, &files.sidecar_sha256) {
        Ok(written) => ids.written = Some(written),
        Err(refusal) => return undone(io, vec![refusal], &[&mbf, &ids]),
    }
    match write_one(io, &MBF_FORMAT, &mbf.path, &files.mbf, &files.mbf_sha256) {
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
    seam: &InProcessWriterTestSeam,
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
    let settings = {
        let _parse = match memory.reserve("the export settings' parse", SETTINGS_WORKING_BYTES) {
            Ok(reservation) => reservation,
            Err(error) => return memory_refusal(error),
        };
        match writer_settings(&settings_json) {
            Ok(settings) => settings,
            Err(refusal) => return MbfExportOutcome::refused(vec![refusal]),
        }
    };
    drop(settings_json);
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
    // 5. The writer, in process: its run reserves nothing (§3.7; slice 4b runs it in the capped worker process). Its
    // output is reserved before the admitted text is released.
    let files = match render_mbf_files(seam, &admitted.text, &settings) {
        Ok(files) => files,
        Err(refusal) => return MbfExportOutcome::refused(refusal),
    };
    let output_bytes = stored_read::exact_text_bytes(files.mbf.len() as u64).saturating_add(stored_read::exact_text_bytes(files.sidecar.len() as u64));
    match memory.reserve("the MBF and its identity file", output_bytes) {
        Ok(reservation) => held.reservations.push(reservation),
        Err(error) => return memory_refusal(error),
    }
    drop(admitted.text);
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

/// The in-process save within `memory` (the caller's: seeded with the command's ingress lease), and its response:
/// `respond` receives the outcome and its text while every reservation is still held. Only through the test seam.
fn save_mbf_export_with<R>(
    seam: InProcessWriterTestSeam,
    request: MbfExportRequest,
    memory: &PoolReadMemory<'_>,
    open_store: impl FnMut() -> Result<rusqlite::Connection, String>,
    choose: impl FnOnce(&str) -> Result<Option<PathBuf>, String>,
    io: &mut impl MbfExportIo,
    respond: impl FnOnce(&MbfExportOutcome, String) -> R,
) -> R {
    let mut libraries = StoreLibraryReads::new(memory, open_store);
    let mut held = Held::default();
    let outcome = save_mbf_export_outcome(&seam, request, memory, &mut libraries, choose, io, &mut held);
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
        "Export MBF is not connected yet: native export memory governance is not connected yet. Nothing was read or written.",
    );
    let (sent, text, buffer) = serialize_outcome(&memory, "Export MBF", "save", outcome, None);
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

    /// The engine's files for `text` with `settings` (the webview's review computes the same digests).
    fn engine_files(text: &str, settings: &str) -> MbfFiles {
        render_mbf_files(&InProcessWriterTestSeam::for_tests(), text, &writer_settings(settings).unwrap()).unwrap()
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
        let outcome = save_mbf_export_with(
            InProcessWriterTestSeam::for_tests(),
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
            MbfExportOutcome::Refused { diagnostics } => diagnostics.iter().map(|item| item.code.clone()).collect(),
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

    /// The webview wrapper's recorded digests (`caepipe_mbf_wasm` tests) are the native writer's for the same text
    /// and settings: both map the dialog's settings to the same writer arguments.
    #[test]
    fn the_webview_wrappers_recorded_digests_are_the_native_writers() {
        let recorded: Value = serde_json::from_str(EXPORT_DIGESTS).unwrap();
        let cases = recorded["cases"].as_object().unwrap();
        assert_eq!(cases.len(), 5);
        for (name, case) in cases {
            let text = if name == "degf-project" { edited(|document| document["project"]["units"]["temperature"] = json!("degF")) } else { canonical_of(EXPORT_MODEL) };
            assert_eq!(case["model_sha256"], sha256_hex(text.as_bytes()), "{name}");
            let settings = writer_settings(case["settings_json"].as_str().unwrap()).unwrap();
            match render_mbf_files(&InProcessWriterTestSeam::for_tests(), &text, &settings) {
                Ok(files) => {
                    assert_eq!(case["status"], "written", "{name}");
                    assert_eq!(case["mbf_sha256"], files.mbf_sha256, "{name}");
                    assert_eq!(case["sidecar_sha256"], files.sidecar_sha256, "{name}");
                }
                Err(refusal) => {
                    assert_eq!(case["status"], "refused", "{name}");
                    let found: Vec<Value> = refusal.iter().map(|item| json!(item.code)).collect();
                    assert_eq!(case["refusal_codes"], Value::Array(found), "{name}");
                }
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

    /// A writer refusal returns the writer's own diagnostics and opens no chooser.
    #[test]
    fn a_writer_refusal_returns_its_diagnostics_before_any_chooser() {
        let text = edited(|document| {
            document["components"][0]["kind"] = json!("tee");
        });
        let opened = Cell::new(false);
        let mut request = request_unreviewed(&text);
        request.reviewed = MbfReviewed { mbf_sha256: "0".repeat(64), sidecar_sha256: "0".repeat(64) };
        let outcome = save(request, Some(std::env::temp_dir().join("never-written.mbf")), &opened);
        let found = codes(&outcome);
        assert!(found.iter().all(|code| code.starts_with("MBF-SI-")), "{found:?}");
        assert!(!found.is_empty());
        assert!(!opened.get());
        // Every slot of the model document (its one library reference dropped, as the test has no library
        // store): the writer refuses its spectra by name.
        let mut every_slot: Value = serde_json::from_str(EVERY_SLOT).unwrap();
        every_slot["materials"][0]["provenance_record"].as_object_mut().unwrap().remove("library_ref");
        let every_slot = canonical_json(&every_slot);
        let outcome = save(request_unreviewed(&every_slot), None, &opened);
        assert!(codes(&outcome).contains(&"MBF-SI-SPECTRUM-UNSUPPORTED".to_string()), "{outcome:?}");
        assert!(!opened.get());
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
                MbfExportOutcome::Refused { diagnostics } => {
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
        let MbfExportOutcome::Refused { diagnostics } = &outcome else { unreachable!() };
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
            let MbfExportOutcome::Refused { diagnostics } = &outcome else { unreachable!() };
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
        let (MbfExportOutcome::Refused { diagnostics }, _) = outcome.bounded() else { unreachable!() };
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
        let MbfExportOutcome::Refused { diagnostics } = &outcome else { unreachable!() };
        assert!(diagnostics[0].message.contains("native export memory governance is not connected yet"), "{}", diagnostics[0].message);
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
            let outcome = save_mbf_export_with(
                InProcessWriterTestSeam::for_tests(),
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
        let cancelled = save_mbf_export_with(InProcessWriterTestSeam::for_tests(), request(&text), &memory, no_store, |_| { opened.set(true); Ok(None) }, &mut SystemMbfExportIo, |outcome, _| outcome.clone());
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
        let cancelled = save_mbf_export_with(InProcessWriterTestSeam::for_tests(), request(&text), &memory, no_store, |_| { opened.set(true); Ok(None) }, &mut SystemMbfExportIo, |outcome, _| outcome.clone());
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
        let MbfExportOutcome::Refused { diagnostics } = &outcome else { unreachable!() };
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
}
