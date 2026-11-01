//! File > Export MBF… (DEL-17-04 Design `desktop-mbf-export-import.md` §3.5–§3.7, slice 4a; RT-R7): the native
//! save of a model batch file and its identity file.
//!
//! The webview reviews the export with the wasm writer (`core/handoff/caepipe_mbf_wasm`) and sends the session's
//! canonical model text, its claimed hash, the writer settings, the suggested name, the digests it reviewed and its
//! route evidence. This module then runs, in order (§3.6):
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
//! 8. the writes: the identity file first, then the MBF, each by the atomic writer under its own format
//!    ([`MBF_IDS_FORMAT`], [`MBF_FORMAT`]); a failed MBF write removes the identity file it wrote, after an identity
//!    check;
//! 9. the read-back of both in fixed blocks, compared by SHA-256; on a mismatch both written files are removed,
//!    after an identity check (`MBF-EXPORT-READBACK-MISMATCH`);
//! 10. the receipt: both names, byte counts and digests, and the atomic writer's durability statement.
//!
//! The bytes written are the native writer's output for the hash-checked text; the webview's text is used for the
//! review only. Nothing is redacted: a screening that would alter the payload blocks the save instead.
//!
//! Memory (§3.7): the command runs within its caller's memory ([`PoolReadMemory`], seeded with the ingress lease).
//! Each of these is reserved before it is made: the admission's parse and canonical copy, the writer's working set
//! ([`writer_working_set`]), the MBF and identity-file bytes, the response and a 64 KiB read-back block. A
//! reservation the pool refuses refuses the save (`MBF-EXPORT-MEMORY-UNAVAILABLE`), before any file is chosen.
//!
//! The command's registration, ingress admission and menu are slice 4b. Nothing is sent anywhere (F-PIP-1).
//! Standard claim fence applies (claims taxonomy per DEC-081): a written MBF means the listed subset of the model
//! was written; it is not evidence that another program reads or analyses it alike.

// Slice 4b registers the command and calls this module; until then only its tests do.
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
use serde_json::Value;
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

pub const SETTINGS_INVALID: &str = "MBF-EXPORT-SETTINGS-INVALID";
pub const MODEL_JSON_INVALID: &str = "MBF-EXPORT-MODEL-JSON-INVALID";
pub const ENGINE_FAILED: &str = "MBF-ENGINE-FAILED";
pub const REVIEW_STALE: &str = "MBF-EXPORT-REVIEW-STALE";
pub const SUGGESTED_NAME_INVALID: &str = "MBF-EXPORT-SUGGESTED-NAME-INVALID";
pub const PICKER_PATH_INVALID: &str = "MBF-EXPORT-PICKER-PATH-INVALID";
pub const DESTINATION_EXTENSION: &str = "MBF-EXPORT-DESTINATION-EXTENSION";
pub const READBACK_MISMATCH: &str = "MBF-EXPORT-READBACK-MISMATCH";
pub const READBACK_FAILED: &str = "MBF-EXPORT-READBACK-FAILED";
pub const MEMORY_UNAVAILABLE: &str = "MBF-EXPORT-MEMORY-UNAVAILABLE";
/// A written file was left in place: it is no longer the file this save wrote, or it could not be inspected.
pub const CLEANUP_SKIPPED: &str = "MBF-EXPORT-CLEANUP-SKIPPED";
pub const CLEANUP_FAILED: &str = "MBF-EXPORT-CLEANUP-FAILED";

/// The read-back block (§3.7).
pub const READBACK_BLOCK_BYTES: usize = 64 * 1024;
/// The writer's working set per byte of model text: `parse_sidecar_json` (four bytes a character, the scanner's
/// tree and the model's values), the writer's state and its preflight import and regeneration of the written text,
/// its result (text, maps, loss report) and the sidecar. Fixed from the counting allocator's priced peak
/// (`the_writer_peak_stays_within_its_working_set`): 176.8, 170.5 and 168.3 bytes per model byte for the export
/// fixture (5.5 KB), and its last run continued by 48 and by 480 straight pipes (34 KB and 296 KB, the largest test
/// model); k is at least 1.5 times the largest model's ratio.
pub const WRITER_WORKING_SET_FACTOR: u64 = 256;
/// What the writer holds whatever the model's size (tables, small maps, the preflight's fixed state).
pub const WRITER_WORKING_SET_BASE_BYTES: u64 = 2 << 20;
/// What a Saved receipt adds to its warnings' JSON: two file names (each at most 255 bytes escaped to at most six),
/// byte counts, digests, the durability and the writer's limitation statement.
pub const SAVED_FIELDS_BYTES: u64 = 8192;
/// The most bytes of a suggested name.
const MAX_SUGGESTED_NAME_BYTES: usize = 200;

/// The writer's working set for a model text of `model_bytes` bytes: k·M plus a fixed base.
pub fn writer_working_set(model_bytes: u64) -> u64 {
    model_bytes.saturating_mul(WRITER_WORKING_SET_FACTOR).saturating_add(WRITER_WORKING_SET_BASE_BYTES)
}

/// The digests the webview reviewed (lowercase hex SHA-256 of the MBF text and of the identity file's bytes).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MbfReviewed {
    pub mbf_sha256: String,
    pub sidecar_sha256: String,
}

/// `{canonicalText, claimedModelHash, settings, suggestedName, reviewed, screening, localFirstEvidence}` (§3.6; the
/// route evidence keeps the member name of the sibling saves).
#[derive(Debug)]
pub struct MbfExportRequest {
    pub canonical_text: String,
    pub claimed_model_hash: String,
    /// `{title, referenceTemperature: {value, unit}, maxLayoutRows: null}` ([`writer_settings`]).
    pub settings: Value,
    pub suggested_name: String,
    pub reviewed: MbfReviewed,
    pub screening: ModelDocumentSaveScreening,
    pub local_first: ModelDocumentSaveLocalFirst,
}

/// Decodes the command's raw-body arguments (strings, a `Value` and structs of strings and scalars, moved), for the
/// command slice 4b registers.
pub(crate) fn save_mbf_export_args(args: &mut Args) -> Result<MbfExportRequest, SolveJobError> {
    Ok(MbfExportRequest {
        canonical_text: args.decode("canonicalText")?,
        claimed_model_hash: args.decode("claimedModelHash")?,
        settings: args.decode("settings")?,
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

/// The dialog's settings as the writer's arguments, by the webview wrapper's rules
/// (`caepipe_mbf_wasm::export_settings`): exactly `{title, referenceTemperature: {value, unit}, maxLayoutRows: null}`;
/// the title as given; `degC` as `reference_temperature_c`, `degF` as `reference_temperature_f` (exact); no row limit
/// (Q-4). The recorded digests (`fixtures/caepipe_mbf/invented/desktop_export/export_digests.json`) hold the two to
/// the same arguments.
pub fn writer_settings(settings: &Value) -> Result<mbf::WriterSettings, Diagnostic> {
    let invalid = |detail: &str| {
        diagnostic(
            SETTINGS_INVALID,
            "settings",
            format!("The export settings must be {{title, referenceTemperature: {{value, unit}}, maxLayoutRows: null}}: {detail}."),
        )
    };
    let Some(object) = settings.as_object() else { return Err(invalid("not an object")) };
    if object.len() != 3 || !["title", "referenceTemperature", "maxLayoutRows"].iter().all(|key| object.contains_key(*key)) {
        return Err(invalid("the members differ"));
    }
    let Some(title) = object["title"].as_str() else { return Err(invalid("the title is not a string")) };
    if !object["maxLayoutRows"].is_null() {
        return Err(invalid("maxLayoutRows must be null (no row limit)"));
    }
    let Some(reference) = object["referenceTemperature"].as_object() else { return Err(invalid("referenceTemperature is not an object")) };
    if reference.len() != 2 || !reference.contains_key("value") || !reference.contains_key("unit") {
        return Err(invalid("referenceTemperature must hold exactly value and unit"));
    }
    // A JSON integer is Python's int and any other number its float, as the engine's own JSON reader takes them.
    let value = match &reference["value"] {
        Value::Number(number) => match (number.as_i64(), number.as_f64()) {
            (Some(integer), _) if !number.is_f64() => mbf::Value::int(integer),
            (_, Some(float)) if number.is_f64() && float.is_finite() => mbf::Value::Float(float),
            _ => return Err(invalid("the reference temperature is not a finite number the writer reads")),
        },
        _ => return Err(invalid("the reference temperature is not a finite number")),
    };
    let mut writer = mbf::WriterSettings::new(title.to_string());
    writer.max_layout_rows = None;
    match reference["unit"].as_str() {
        Some("degC") => writer.reference_temperature_c = value,
        Some("degF") => writer.reference_temperature_f = value,
        _ => return Err(invalid("the reference temperature unit is not degC or degF")),
    }
    Ok(writer)
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
/// written files or the writer's refusal.
pub fn render_mbf_files(text: &str, settings: &mbf::WriterSettings) -> Result<MbfFiles, Vec<Diagnostic>> {
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
pub(crate) trait MbfExportIo {
    fn write(&mut self, format: &AtomicSaveFormat, destination: &Path, bytes: &[u8]) -> Result<AtomicSaveOutcome, AtomicSaveError> {
        atomic_report_package_save::atomic_save_bytes_as(format, destination, bytes)
    }
    fn open(&mut self, path: &Path) -> io::Result<File> {
        File::open(path)
    }
    fn remove(&mut self, path: &Path) -> io::Result<()> {
        fs::remove_file(path)
    }
}

pub(crate) struct SystemMbfExportIo;

impl MbfExportIo for SystemMbfExportIo {}

/// Which file a path names: the same file this save wrote (device and inode where the platform has them), so a
/// cleanup never removes a file someone else put there.
#[derive(Debug, Clone, PartialEq, Eq)]
struct FileIdentity {
    #[cfg(unix)]
    device_inode: (u64, u64),
    #[cfg(not(unix))]
    created_modified: (Option<std::time::SystemTime>, Option<std::time::SystemTime>),
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
        Ok(FileIdentity { created_modified: (metadata.created().ok(), metadata.modified().ok()) })
    }
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
    match (&written.identity, file_identity(&written.path)) {
        (Some(expected), Ok(found)) if *expected == found => match io.remove(&written.path) {
            Ok(()) => None,
            Err(error) => Some(diagnostic(CLEANUP_FAILED, "", format!("`{name}` could not be removed ({error}); delete it before using the folder's files."))),
        },
        (_, Err(error)) if error.kind() == io::ErrorKind::NotFound => None,
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

/// Steps 8 to 10 for the chosen MBF path.
fn write_pair(io: &mut impl MbfExportIo, chosen: &Path, files: &MbfFiles, block: &mut [u8], warnings: Vec<Diagnostic>) -> MbfExportOutcome {
    let ids_path = match identity_file_path(chosen) {
        Ok(path) => path,
        Err(refusal) => return MbfExportOutcome::refused(vec![refusal]),
    };
    let ids = match write_one(io, &MBF_IDS_FORMAT, &ids_path, &files.sidecar, &files.sidecar_sha256) {
        Ok(written) => written,
        Err(refusal) => return MbfExportOutcome::refused(vec![refusal]),
    };
    let mbf = match write_one(io, &MBF_FORMAT, chosen, &files.mbf, &files.mbf_sha256) {
        Ok(written) => written,
        Err(refusal) => {
            let mut diagnostics = vec![refusal];
            diagnostics.extend(remove_written(io, &ids));
            return MbfExportOutcome::refused(diagnostics);
        }
    };
    let mut problems = Vec::new();
    for written in [&ids, &mbf] {
        match read_back(io, &written.path, block) {
            Ok(found) if found == written.saved.sha256 => {}
            Ok(found) => problems.push(diagnostic(
                READBACK_MISMATCH,
                "",
                format!(
                    "`{}` reads back with SHA-256 {found}, not the {} written; the written files are removed.",
                    stored_read::shown(&written.saved.file_name),
                    written.saved.sha256
                ),
            )),
            Err(error) => problems.push(diagnostic(
                READBACK_FAILED,
                "",
                format!("`{}` could not be read back ({error}); the written files are removed.", stored_read::shown(&written.saved.file_name)),
            )),
        }
    }
    if !problems.is_empty() {
        for written in [&mbf, &ids] {
            problems.extend(remove_written(io, written));
        }
        return MbfExportOutcome::refused(problems);
    }
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
    request: MbfExportRequest,
    memory: &PoolReadMemory<'_>,
    libraries: &mut StoreLibraryReads<'_>,
    choose: impl FnOnce(&str) -> Result<Option<PathBuf>, String>,
    io: &mut impl MbfExportIo,
    held: &mut Held,
) -> MbfExportOutcome {
    let MbfExportRequest { canonical_text, claimed_model_hash, settings, suggested_name, reviewed, screening, local_first } = request;
    // 1. Route evidence; its binding to this text's SHA-256 is checked with the hash (2).
    if let Err(refusal) = validate_local_private_route_evidence(MBF_EXPORT_ROUTE_ID, &screening, &local_first) {
        return MbfExportOutcome::refused(vec![refusal]);
    }
    // 2-4. The open cap, the canonical text and hash, the open path's checks and the disposition screen.
    let model_bytes = canonical_text.len() as u64;
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
        for item in &mut refusal {
            if item.code == stored_read::STORE_READ_MEMORY_UNAVAILABLE {
                item.code = MEMORY_UNAVAILABLE.to_string();
            }
        }
        return MbfExportOutcome::refused(refusal);
    }
    let admitted = match admitted {
        Ok(admitted) => admitted,
        Err(diagnostics) => return MbfExportOutcome::refused(diagnostics),
    };
    // 5. The writer, within its working set; its output is reserved before the working set is released.
    let settings = match writer_settings(&settings) {
        Ok(settings) => settings,
        Err(refusal) => return MbfExportOutcome::refused(vec![refusal]),
    };
    let working = match memory.reserve("the MBF writer's working set", writer_working_set(model_bytes)) {
        Ok(reservation) => reservation,
        Err(error) => return memory_refusal(error),
    };
    let files = match render_mbf_files(&admitted.text, &settings) {
        Ok(files) => files,
        Err(refusal) => return MbfExportOutcome::refused(refusal),
    };
    let output_bytes = stored_read::exact_text_bytes(files.mbf.len() as u64).saturating_add(stored_read::exact_text_bytes(files.sidecar.len() as u64));
    match memory.reserve("the MBF and its identity file", output_bytes) {
        Ok(reservation) => held.reservations.push(reservation),
        Err(error) => return memory_refusal(error),
    }
    drop(working);
    drop(admitted.text);
    // 6. Staleness.
    if let Err(refusal) = check_reviewed(&files, &reviewed) {
        return MbfExportOutcome::refused(vec![refusal]);
    }
    // The receipt's bound and the read-back block, reserved before the chooser: no response is refused after a
    // file was written.
    let bound = stored_read::serialized_length(&admitted.warnings).unwrap_or(u64::MAX).saturating_add(SAVED_FIELDS_BYTES);
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

/// The save within `memory` (the caller's: seeded with the command's ingress lease), and its response: `respond`
/// receives the outcome and its text while every reservation is still held.
pub(crate) fn save_mbf_export_with<R>(
    request: MbfExportRequest,
    memory: &PoolReadMemory<'_>,
    open_store: impl FnMut() -> Result<rusqlite::Connection, String>,
    choose: impl FnOnce(&str) -> Result<Option<PathBuf>, String>,
    io: &mut impl MbfExportIo,
    respond: impl FnOnce(&MbfExportOutcome, String) -> R,
) -> R {
    let mut libraries = StoreLibraryReads::new(memory, open_store);
    let mut held = Held::default();
    let outcome = save_mbf_export_outcome(request, memory, &mut libraries, choose, io, &mut held);
    let (sent, text, buffer) = serialize_outcome(memory, "Export MBF", "save", outcome, held.response.take());
    let response = respond(&sent, text);
    drop(sent);
    drop(buffer);
    drop(held);
    drop(libraries);
    response
}

/// The save for the command slice 4b registers: within the pool of `registry`, started from the ingress lease
/// `ingress`, with the system's file operations.
pub(crate) fn save_mbf_export_admitted<R>(
    request: MbfExportRequest,
    open_store: impl FnMut() -> Result<rusqlite::Connection, String>,
    registry: &SolveJobRegistry,
    ingress: &Lease,
    choose: impl FnOnce(&str) -> Result<Option<PathBuf>, String>,
    respond: impl FnOnce(&MbfExportOutcome, String) -> R,
) -> R {
    let memory = PoolReadMemory::new(registry, ingress);
    save_mbf_export_with(request, &memory, open_store, choose, &mut SystemMbfExportIo, respond)
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
    use serde_json::json;
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

    fn settings() -> Value {
        json!({"title": "Invented MBF export", "referenceTemperature": {"value": 20, "unit": "degC"}, "maxLayoutRows": null})
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
    fn engine_files(text: &str, settings: &Value) -> MbfFiles {
        render_mbf_files(text, &writer_settings(settings).unwrap()).unwrap()
    }

    /// A request whose evidence and reviewed digests are those of `text`.
    fn request(text: &str) -> MbfExportRequest {
        let files = engine_files(text, &settings());
        MbfExportRequest {
            canonical_text: text.to_string(),
            claimed_model_hash: format!("sha256:{}", sha256_hex(text.as_bytes())),
            settings: settings(),
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
        // Saving again over the same MBF replaces both files.
        let opened = Cell::new(false);
        let MbfExportOutcome::Saved { mbf, identity_file, .. } = save(request(&text), Some(chosen.clone()), &opened) else { panic!("not saved") };
        assert!(mbf.replaced_existing && identity_file.replaced_existing);
    }

    /// The webview wrapper's recorded digests (`caepipe_mbf_wasm` tests) are the native writer's for the same text
    /// and settings: both map the dialog's settings to the same writer arguments.
    #[test]
    fn the_webview_wrappers_recorded_digests_are_the_native_writers() {
        let recorded: Value = serde_json::from_str(EXPORT_DIGESTS).unwrap();
        let cases = recorded["cases"].as_object().unwrap();
        assert_eq!(cases.len(), 3);
        for (name, case) in cases {
            let text = if name == "degf-project" { edited(|document| document["project"]["units"]["temperature"] = json!("degF")) } else { canonical_of(EXPORT_MODEL) };
            assert_eq!(case["model_sha256"], sha256_hex(text.as_bytes()), "{name}");
            let files = engine_files(&text, &case["settings"]);
            assert_eq!(case["mbf_sha256"], files.mbf_sha256, "{name}");
            assert_eq!(case["sidecar_sha256"], files.sidecar_sha256, "{name}");
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
            (SETTINGS_INVALID, MbfExportRequest { settings: json!({"title": "T", "referenceTemperature": {"value": 20, "unit": "degC"}, "maxLayoutRows": 20}), ..request(&text) }),
            (SETTINGS_INVALID, MbfExportRequest { settings: json!({"title": "T", "referenceTemperature": {"value": 20, "unit": "K"}, "maxLayoutRows": null}), ..request(&text) }),
            (REVIEW_STALE, MbfExportRequest { reviewed: MbfReviewed { mbf_sha256: sha256_hex(b"other"), ..request(&text).reviewed }, ..request(&text) }),
            (REVIEW_STALE, MbfExportRequest { reviewed: MbfReviewed { sidecar_sha256: sha256_hex(b"other"), ..request(&text).reviewed }, ..request(&text) }),
            // The settings changed after the review: the same model, another reference temperature.
            (REVIEW_STALE, MbfExportRequest { settings: json!({"title": "Invented MBF export", "referenceTemperature": {"value": 25, "unit": "degC"}, "maxLayoutRows": null}), ..request(&text) }),
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
            settings: settings(),
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
        tamper_mbf: bool,
        swap_ids_before_cleanup: bool,
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
            if format == &MBF_FORMAT && self.fail_mbf_write {
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
            "settings": settings(),
            "suggestedName": "swbpipe-model.mbf",
            "reviewed": {"mbfSha256": request.reviewed.mbf_sha256, "sidecarSha256": request.reviewed.sidecar_sha256},
            "screening": serde_json::to_value(screening(&text)).unwrap(),
            "localFirstEvidence": serde_json::to_value(local_first()).unwrap(),
        });
        let pool = registry(ScriptedHost::normal());
        let (mut args, _lease) = admit_json(&pool, COMMAND, ArgsDecoding::Moved, &body).unwrap();
        let decoded = save_mbf_export_args(&mut args).unwrap();
        assert_eq!(decoded.canonical_text, text);
        assert_eq!(decoded.settings, settings());
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

    /// §3.7: k fixed from the counting allocator's peak of the writer (parse, render, sidecar) over the test
    /// models, the largest included; every model's peak stays within its working set.
    #[test]
    fn the_writer_peak_stays_within_its_working_set() {
        let mut worst = 0.0_f64;
        for (label, text) in [("fixture", canonical_of(EXPORT_MODEL)), ("long-48", long_model(48)), ("long-480", long_model(480))] {
            let writer = writer_settings(&settings()).unwrap();
            let (files, measured) = crate::ingress::counting::measure(|| render_mbf_files(&text, &writer));
            assert!(files.is_ok(), "{label}: {files:?}");
            let model = text.len() as u64;
            let ratio = measured.peak_priced as f64 / model as f64;
            worst = worst.max(ratio);
            eprintln!("mbf writer peak: {label} M={model} peak_priced={} ratio={ratio:.1} working_set={}", measured.peak_priced, writer_working_set(model));
            assert!(measured.peak_priced <= writer_working_set(model), "{label}: peak {} above the working set {}", measured.peak_priced, writer_working_set(model));
            // The margin: k is at least 1.5 times the measured ratio on the largest model.
            if label == "long-480" {
                assert!((WRITER_WORKING_SET_FACTOR as f64) >= 1.5 * ratio, "{label}: k={WRITER_WORKING_SET_FACTOR} ratio={ratio:.1}");
            }
        }
        assert!(worst > 0.0);
    }

    /// §3.7: the whole save, through its response, peaks within what it held; every reservation is released.
    #[test]
    fn the_save_peaks_within_what_it_held() {
        let _lock = atomic_save_test_lock();
        let sandbox = Sandbox::new("peak");
        let text = long_model(480);
        let request = request(&text);
        let pool = registry(ScriptedHost::normal());
        let memory = PoolReadMemory::unseeded(&pool);
        let chosen = sandbox.0.join("swbpipe-model.mbf");
        let (outcome, measured) = crate::ingress::counting::measure(|| {
            save_mbf_export_with(request, &memory, no_store, |_| Ok(Some(chosen.clone())), &mut SystemMbfExportIo, |outcome, text| (outcome.clone(), text.len()))
        });
        assert!(matches!(outcome.0, MbfExportOutcome::Saved { .. }), "{:?}", outcome.0);
        eprintln!("mbf save peak: M={} peak_priced={} peak_reserved={}", text.len(), measured.peak_priced, memory.peak_reserved());
        assert!(measured.peak_priced <= memory.peak_reserved(), "peak {} above the {} held", measured.peak_priced, memory.peak_reserved());
        assert!(memory.peak_reserved() >= writer_working_set(text.len() as u64));
        drop(memory);
        assert_eq!(pool.pool_status().reserved_bytes, 0);
    }

    /// The command's entry: the ingress lease of its arguments is counted with every reservation, so a budget
    /// that admits the save without the lease refuses it with the lease, before the chooser.
    #[test]
    fn the_ingress_lease_is_counted_with_the_saves_reservations() {
        let _lock = atomic_save_test_lock();
        let sandbox = Sandbox::new("lease");
        let text = canonical_of(EXPORT_MODEL);
        let body = json!({"canonicalText": text, "pad": "x".repeat(1 << 20)});
        let run = |pool: &SolveJobRegistry, chosen: PathBuf, opened: &Cell<bool>| {
            let (_, lease) = admit_json(pool, COMMAND, ArgsDecoding::Moved, &body).unwrap();
            let outcome = save_mbf_export_admitted(request(&text), no_store, pool, &lease, |_| { opened.set(true); Ok(Some(chosen)) }, |outcome, _| outcome.clone());
            (outcome, lease.bytes())
        };
        let pool = registry(ScriptedHost::normal());
        let opened = Cell::new(false);
        let (outcome, lease) = run(&pool, sandbox.0.join("swbpipe-model.mbf"), &opened);
        assert!(matches!(outcome, MbfExportOutcome::Saved { .. }), "{outcome:?}");
        assert!(lease > 1 << 20);
        assert_eq!(pool.pool_status().reserved_bytes, 0);
        // A budget the save fits without the lease (its working set and a margin), but not with it.
        let budget = writer_working_set(text.len() as u64) + (2 << 20);
        assert!(budget < lease + writer_working_set(text.len() as u64));
        let tight = registry(ScriptedHost::normal());
        tight.configure_pool(64 << 20, budget, false).unwrap();
        let opened = Cell::new(false);
        let (outcome, _) = run(&tight, sandbox.0.join("other.mbf"), &opened);
        assert_eq!(codes(&outcome), vec![MEMORY_UNAVAILABLE.to_string()]);
        assert!(!opened.get());
        assert_eq!(tight.pool_status().reserved_bytes, 0);
    }

    /// A reservation the pool refuses refuses the save by name, before any chooser, and writes nothing.
    #[test]
    fn a_refused_reservation_refuses_the_save_before_the_chooser() {
        let text = long_model(480);
        let working = writer_working_set(text.len() as u64);
        // The admission's parse, the writer's working set, then the files' bytes held with it.
        for budget in [64 * 1024, working / 2, working + 1] {
            let pool = registry(ScriptedHost::normal());
            pool.configure_pool(budget.max(1 << 20), budget, false).unwrap();
            let opened = Cell::new(false);
            let outcome = save_with(request(&text), &pool, Some(std::env::temp_dir().join("never-written.mbf")), &mut SystemMbfExportIo, &opened);
            assert_eq!(codes(&outcome), vec![MEMORY_UNAVAILABLE.to_string()], "budget {budget}");
            assert!(!opened.get(), "budget {budget}");
        }
        assert!(!std::env::temp_dir().join("never-written.mbf").exists());
    }
}
