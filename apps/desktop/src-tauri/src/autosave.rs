//! Autosave recovery records (D5; DEL-02-05 Design `autosave-and-backup.md`
//! §1 and §5, slice 1).
//!
//! A record is one file in the app's `autosave/` directory, the store file's
//! sibling. Its bytes are exactly the session's model document as unchecked
//! RFC 8785 (JCS) canonical text, so its SHA-256 is the session's model hash.
//! Its name `<slot>-<written unix millis, 13 digits>-<model hash hex>.json`
//! carries its identity: `slot` is the first 32 hex digits of the SHA-256 of
//! the document's `project.id`, so no project name or id is in a name, and the
//! hash lets every read check the bytes with no second file.
//!
//! Before anything is written, the text must be within the open cap, hash to
//! the claim, be its own canonical form and pass the open path's checks
//! (`model_document_save::open_path_checks`), so every record re-opens. The
//! record is written by the extension-neutral atomic writer (temporary file,
//! flush, fsync, rename, bounded directory sync), then read back through
//! SHA-256 in fixed blocks. Only after that are the slot's other records
//! removed (regular files whose names parse as that slot's records). A failure
//! at any step leaves the previous record in place and is reported by name;
//! the store is never opened. Nothing here approves, accepts or issues
//! anything: a record is a recovery copy of the user's model.
//!
//! Memory: each function takes the command's `ReadMemory` and reserves the
//! text's parse, its canonical copy, a file's load and the read-back block
//! before it makes them (DEL-04-07 "Model-sized interactive commands").

// Slice 2 wires these functions into native commands; until then only the
// tests call them.
#![cfg_attr(not(test), allow(dead_code))]

use crate::atomic_report_package_save::{atomic_save_bytes_as_with_io, AtomicSaveFormat, AtomicSaveIo, SystemAtomicSaveIo};
use crate::export_disposition_screen::{diagnostic, DispositionDiagnostic};
use crate::model_document_file::MAX_MODEL_DOCUMENT_BYTES;
use crate::model_document_save::{open_path_checks, writer_diagnostic};
use crate::stored_read::{is_jcs_text, loaded_text_bytes, parse_bytes, Metered, ReadMemory, ReadReservation, STORE_READ_MEMORY_UNAVAILABLE};
use open_pipe_stress_operation_applier::sha256_hex;
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::Path;

/// The records' directory, beside the store file in the app's local data directory.
pub(crate) const AUTOSAVE_DIRECTORY: &str = "autosave";

/// The atomic writer's admission of a record: `.json`, `AUTOSAVE-*` codes, and
/// its own temporary-file prefix, which no listing or read ever parses.
pub(crate) const AUTOSAVE_FORMAT: AtomicSaveFormat = AtomicSaveFormat {
    extension: "json",
    code_prefix: "AUTOSAVE",
    temp_prefix: ".swbpipe-autosave-",
    log_label: "autosave",
};

pub(crate) const HASH_MISMATCH: &str = "AUTOSAVE-HASH-MISMATCH";
pub(crate) const TOO_LARGE: &str = "AUTOSAVE-TOO-LARGE";
pub(crate) const VERSION_NOT_CURRENT: &str = "AUTOSAVE-VERSION-NOT-CURRENT";
pub(crate) const DIRECTORY_UNAVAILABLE: &str = "AUTOSAVE-DIRECTORY-UNAVAILABLE";
pub(crate) const CLOCK_INVALID: &str = "AUTOSAVE-CLOCK-INVALID";
pub(crate) const READBACK_MISMATCH: &str = "AUTOSAVE-READBACK-MISMATCH";
pub(crate) const READBACK_FAILED: &str = "AUTOSAVE-READBACK-FAILED";
pub(crate) const PRUNE_FAILED: &str = "AUTOSAVE-PRUNE-FAILED";
pub(crate) const NAME_INVALID: &str = "AUTOSAVE-NAME-INVALID";
pub(crate) const NOT_FOUND: &str = "AUTOSAVE-NOT-FOUND";
pub(crate) const UNREADABLE: &str = "AUTOSAVE-UNREADABLE";
pub(crate) const INTEGRITY: &str = "AUTOSAVE-INTEGRITY";

/// The read-back's block: the only buffer the hash of a written file needs.
pub(crate) const READBACK_BLOCK_BYTES: usize = 64 * 1024;
/// The most records one listing names; the rest are counted.
pub(crate) const MAX_LISTED_AUTOSAVES: usize = 1024;

const SLOT_HEX_DIGITS: usize = 32;
const MILLIS_DIGITS: usize = 13;
const HASH_HEX_DIGITS: usize = 64;

/// The slot of a project id: the first 32 hex digits of its UTF-8 SHA-256.
pub(crate) fn autosave_slot(project_id: &str) -> String {
    let mut slot = sha256_hex(project_id);
    slot.truncate(SLOT_HEX_DIGITS);
    slot
}

fn is_lower_hex(text: &str, digits: usize) -> bool {
    text.len() == digits && text.bytes().all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
}

/// A record's identity, as its file name carries it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AutosaveName {
    pub(crate) slot: String,
    pub(crate) written_unix_millis: u64,
    /// The model hash, bare lowercase hex.
    pub(crate) model_hash_hex: String,
}

impl AutosaveName {
    pub(crate) fn file_name(&self) -> String {
        format!("{}-{:013}-{}.json", self.slot, self.written_unix_millis, self.model_hash_hex)
    }

    /// The name of a record, or `None` for any other name (a temporary file, a
    /// path with a separator, another file).
    pub(crate) fn parse(file_name: &str) -> Option<Self> {
        let stem = file_name.strip_suffix(".json")?;
        let mut parts = stem.split('-');
        let (slot, millis, hash) = (parts.next()?, parts.next()?, parts.next()?);
        if parts.next().is_some()
            || !is_lower_hex(slot, SLOT_HEX_DIGITS)
            || millis.len() != MILLIS_DIGITS
            || !millis.bytes().all(|byte| byte.is_ascii_digit())
            || !is_lower_hex(hash, HASH_HEX_DIGITS)
        {
            return None;
        }
        Some(Self { slot: slot.to_string(), written_unix_millis: millis.parse().ok()?, model_hash_hex: hash.to_string() })
    }
}

/// One record a listing names (no content is read).
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub(crate) struct AutosaveEntry {
    pub(crate) file_name: String,
    pub(crate) slot: String,
    pub(crate) written_unix_millis: u64,
    /// `sha256:<hex>`.
    pub(crate) model_hash: String,
    pub(crate) byte_count: u64,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub(crate) struct AutosaveListing {
    /// Newest first.
    pub(crate) entries: Vec<AutosaveEntry>,
    /// Records beyond [`MAX_LISTED_AUTOSAVES`], counted and not named.
    pub(crate) not_listed: usize,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub(crate) enum AutosaveWriteOutcome {
    Written {
        file_name: String,
        slot: String,
        written_unix_millis: u64,
        /// `sha256:<hex>` of the bytes written and read back: the model hash.
        model_hash: String,
        byte_count: u64,
        /// `durable`, or `saved_durability_uncertain` when the directory sync
        /// after the rename did not complete.
        durability: String,
        /// Older records of the slot that could not be removed.
        warnings: Vec<DispositionDiagnostic>,
    },
    /// The slot's newest record already holds this model; nothing was written.
    Unchanged { file_name: String, slot: String, model_hash: String },
    Refused { diagnostics: Vec<DispositionDiagnostic> },
}

impl AutosaveWriteOutcome {
    fn refused_one(code: &str, message: impl Into<String>) -> Self {
        Self::Refused { diagnostics: vec![diagnostic(code, "", message)] }
    }
}

/// A record read for recovery, after every check of §5.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub(crate) struct VerifiedAutosave {
    pub(crate) file_name: String,
    pub(crate) slot: String,
    pub(crate) written_unix_millis: u64,
    pub(crate) model_hash: String,
    pub(crate) byte_count: u64,
    pub(crate) document: Value,
}

fn memory_refusal(message: String) -> Vec<DispositionDiagnostic> {
    vec![diagnostic(STORE_READ_MEMORY_UNAVAILABLE, "", message)]
}

/// A model document text admitted as a record: its parsed document (under its
/// reservations), its slot and its hash.
struct AdmittedText {
    document: Metered<Value>,
    slot: String,
    model_hash_hex: String,
}

/// The content checks of a record's text against a claimed hash (bare hex):
/// the open cap, the hash, the canonical form (each parse and copy reserved
/// first) and the open path's checks. `mismatch` names a text that is not the
/// canonical text of the claimed hash.
fn admit_text(text: &str, claimed_hex: &str, mismatch: &str, memory: &dyn ReadMemory) -> Result<AdmittedText, Vec<DispositionDiagnostic>> {
    let byte_count = text.len() as u64;
    if byte_count > MAX_MODEL_DOCUMENT_BYTES {
        return Err(vec![diagnostic(
            TOO_LARGE,
            "",
            format!("The model document is {byte_count} bytes; a model document may be at most {MAX_MODEL_DOCUMENT_BYTES} bytes, so it would not re-open."),
        )]);
    }
    let hex = sha256_hex(text);
    if hex != claimed_hex {
        return Err(vec![diagnostic(mismatch, "", format!("The text's SHA-256 is sha256:{hex}, not sha256:{claimed_hex}."))]);
    }
    let parse = parse_bytes(text)
        .map_err(|detail| vec![diagnostic(mismatch, "", format!("The text is not one JSON value: {detail}."))])?;
    let reservation = memory.reserve("the autosave's model document", parse).map_err(memory_refusal)?;
    let document: Value = serde_json::from_str(text)
        .map_err(|error| vec![diagnostic(mismatch, "", format!("The text is not JSON ({}).", crate::stored_read::json_error(&error)))])?;
    if !is_jcs_text(memory, "the autosave's canonical text", &document, text).map_err(memory_refusal)? {
        return Err(vec![diagnostic(mismatch, "", "The text is not its own RFC 8785 (JCS) canonical form.")]);
    }
    let slot = autosave_slot(&crate::project_id_from_model(&document));
    let document = open_path_checks(byte_count, document, VERSION_NOT_CURRENT)?;
    Ok(AdmittedText { document: Metered::new(document).holding(reservation), slot, model_hash_hex: hex })
}

/// The directory exists as a real directory (created if absent, under an
/// existing parent); a symlink or another file there is refused.
fn ensure_directory(directory: &Path) -> Result<(), DispositionDiagnostic> {
    let unavailable = |detail: String| diagnostic(DIRECTORY_UNAVAILABLE, "", format!("The autosave directory is unavailable: {detail}. Nothing was written."));
    match fs::symlink_metadata(directory) {
        Ok(metadata) if metadata.is_dir() && !metadata.file_type().is_symlink() => Ok(()),
        Ok(_) => Err(unavailable("it is not a directory".to_string())),
        Err(error) if error.kind() == io::ErrorKind::NotFound => fs::create_dir(directory).map_err(|error| unavailable(error.to_string())),
        Err(error) => Err(unavailable(error.to_string())),
    }
}

/// Every record in `directory` (regular files whose names parse), with its
/// length; a missing directory holds none. Names only: no content is read.
fn scan(directory: &Path) -> io::Result<Vec<(AutosaveName, u64)>> {
    let entries = match fs::read_dir(directory) {
        Ok(entries) => entries,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => return Err(error),
    };
    let mut records = Vec::new();
    for entry in entries {
        let entry = entry?;
        // `DirEntry::file_type` does not follow a symlink.
        if !entry.file_type()?.is_file() {
            continue;
        }
        let Some(name) = entry.file_name().to_str().and_then(AutosaveName::parse) else {
            continue;
        };
        records.push((name, entry.metadata()?.len()));
    }
    records.sort_by(|(left, _), (right, _)| {
        right.written_unix_millis.cmp(&left.written_unix_millis).then_with(|| right.file_name().cmp(&left.file_name()))
    });
    Ok(records)
}

/// The records in `directory`, newest first, at most [`MAX_LISTED_AUTOSAVES`].
pub(crate) fn list_autosaves(directory: &Path) -> Result<AutosaveListing, DispositionDiagnostic> {
    if let Ok(metadata) = fs::symlink_metadata(directory) {
        if !metadata.is_dir() || metadata.file_type().is_symlink() {
            return Err(diagnostic(DIRECTORY_UNAVAILABLE, "", "The autosave directory is not a directory."));
        }
    }
    let records = scan(directory)
        .map_err(|error| diagnostic(DIRECTORY_UNAVAILABLE, "", format!("The autosave directory could not be listed: {error}.")))?;
    let not_listed = records.len().saturating_sub(MAX_LISTED_AUTOSAVES);
    let entries = records
        .into_iter()
        .take(MAX_LISTED_AUTOSAVES)
        .map(|(name, byte_count)| AutosaveEntry {
            file_name: name.file_name(),
            slot: name.slot,
            written_unix_millis: name.written_unix_millis,
            model_hash: format!("sha256:{}", name.model_hash_hex),
            byte_count,
        })
        .collect();
    Ok(AutosaveListing { entries, not_listed })
}

/// SHA-256 (bare hex) of a file, streamed through one block of
/// [`READBACK_BLOCK_BYTES`], which `memory` reserves first.
fn file_sha256_hex(path: &Path, memory: &dyn ReadMemory) -> Result<String, String> {
    let _block: ReadReservation = memory.reserve("the autosave read-back block", READBACK_BLOCK_BYTES as u64)?;
    let read = || -> io::Result<String> {
        let mut file = File::open(path)?;
        let mut hasher = Sha256::new();
        let mut block = vec![0_u8; READBACK_BLOCK_BYTES];
        loop {
            let count = file.read(&mut block)?;
            if count == 0 {
                break;
            }
            hasher.update(&block[..count]);
        }
        Ok(hasher.finalize().iter().map(|byte| format!("{byte:02x}")).collect())
    };
    read().map_err(|error| error.to_string())
}

/// Removes the slot's records other than `keep`: regular files whose names
/// parse as that slot's records, in `directory` only. Each failure is a warning.
fn remove_slot_records(directory: &Path, slot: &str, keep: Option<&str>) -> Vec<DispositionDiagnostic> {
    let records = match scan(directory) {
        Ok(records) => records,
        Err(error) => return vec![diagnostic(PRUNE_FAILED, "", format!("The slot's older autosaves could not be listed: {error}."))],
    };
    records
        .into_iter()
        .filter(|(name, _)| name.slot == slot && Some(name.file_name().as_str()) != keep)
        .filter_map(|(name, _)| {
            let file_name = name.file_name();
            fs::remove_file(directory.join(&file_name))
                .err()
                .map(|error| diagnostic(PRUNE_FAILED, "", format!("The older autosave `{file_name}` was not removed: {error}.")))
        })
        .collect()
}

/// Writes the session's canonical model document text as the newest record of
/// its slot in `directory` (§1, §5).
pub(crate) fn write_autosave(
    directory: &Path,
    canonical_text: &str,
    claimed_model_hash: &str,
    written_unix_millis: u64,
    memory: &dyn ReadMemory,
) -> AutosaveWriteOutcome {
    write_autosave_with_io(directory, canonical_text, claimed_model_hash, written_unix_millis, memory, &mut SystemAtomicSaveIo)
}

pub(crate) fn write_autosave_with_io(
    directory: &Path,
    canonical_text: &str,
    claimed_model_hash: &str,
    written_unix_millis: u64,
    memory: &dyn ReadMemory,
    io: &mut impl AtomicSaveIo,
) -> AutosaveWriteOutcome {
    let Some(claimed_hex) = claimed_model_hash.strip_prefix("sha256:").filter(|hex| is_lower_hex(hex, HASH_HEX_DIGITS)) else {
        return AutosaveWriteOutcome::refused_one(HASH_MISMATCH, "The claimed model hash is not `sha256:` and 64 lowercase hex digits.");
    };
    if written_unix_millis.to_string().len() > MILLIS_DIGITS {
        return AutosaveWriteOutcome::refused_one(CLOCK_INVALID, format!("The clock reads {written_unix_millis} ms, beyond a record name's 13 digits. Nothing was written."));
    }
    let admitted = match admit_text(canonical_text, claimed_hex, HASH_MISMATCH, memory) {
        Ok(admitted) => admitted,
        Err(diagnostics) => return AutosaveWriteOutcome::Refused { diagnostics },
    };
    if let Err(refusal) = ensure_directory(directory) {
        return AutosaveWriteOutcome::Refused { diagnostics: vec![refusal] };
    }
    let records = match scan(directory) {
        Ok(records) => records,
        Err(error) => return AutosaveWriteOutcome::refused_one(DIRECTORY_UNAVAILABLE, format!("The autosave directory could not be listed: {error}. Nothing was written.")),
    };
    let model_hash = format!("sha256:{}", admitted.model_hash_hex);
    if let Some((newest, _)) = records.iter().find(|(name, _)| name.slot == admitted.slot) {
        if newest.model_hash_hex == admitted.model_hash_hex {
            return AutosaveWriteOutcome::Unchanged { file_name: newest.file_name(), slot: admitted.slot, model_hash };
        }
    }
    let name = AutosaveName { slot: admitted.slot.clone(), written_unix_millis, model_hash_hex: admitted.model_hash_hex.clone() };
    let file_name = name.file_name();
    let destination = directory.join(&file_name);
    let saved = match atomic_save_bytes_as_with_io(&AUTOSAVE_FORMAT, &destination, canonical_text.as_bytes(), io) {
        Ok(saved) => saved,
        Err(error) => return AutosaveWriteOutcome::Refused { diagnostics: vec![writer_diagnostic(error)] },
    };
    // The record is read back before any older record is removed; a record
    // that does not read back as its hash is removed and the older one kept.
    match file_sha256_hex(&destination, memory) {
        Ok(read_back) if read_back == admitted.model_hash_hex => {}
        Ok(read_back) => {
            let removal = fs::remove_file(&destination).err().map(|error| format!(" It could not be removed: {error}.")).unwrap_or_default();
            return AutosaveWriteOutcome::refused_one(
                READBACK_MISMATCH,
                format!("The written autosave reads back as sha256:{read_back}, not {model_hash}; the previous autosave is kept.{removal}"),
            );
        }
        Err(error) => {
            let removal = fs::remove_file(&destination).err().map(|error| format!(" It could not be removed: {error}.")).unwrap_or_default();
            return AutosaveWriteOutcome::refused_one(
                READBACK_FAILED,
                format!("The written autosave could not be read back ({error}); the previous autosave is kept.{removal}"),
            );
        }
    }
    drop(admitted.document);
    let warnings = remove_slot_records(directory, &name.slot, Some(&file_name));
    AutosaveWriteOutcome::Written {
        file_name,
        slot: name.slot,
        written_unix_millis,
        model_hash,
        byte_count: canonical_text.len() as u64,
        durability: saved.durability,
        warnings,
    }
}

/// Reads one record for recovery (§3, §5): the name must parse, the file must
/// be a regular file within the open cap whose SHA-256 is its name's hash, its
/// text must be canonical and pass the open path's checks, and its document's
/// slot must be its name's. The file's load is reserved before it is read.
pub(crate) fn read_autosave(directory: &Path, file_name: &str, memory: &dyn ReadMemory) -> Result<Metered<VerifiedAutosave>, Vec<DispositionDiagnostic>> {
    let one = |code: &str, message: String| vec![diagnostic(code, "", message)];
    let Some(name) = AutosaveName::parse(file_name) else {
        return Err(one(NAME_INVALID, "The name is not an autosave record's name.".to_string()));
    };
    let path = directory.join(file_name);
    let metadata = match fs::symlink_metadata(&path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Err(one(NOT_FOUND, format!("The autosave `{file_name}` is no longer there."))),
        Err(error) => return Err(one(UNREADABLE, format!("The autosave `{file_name}` could not be inspected: {error}."))),
    };
    if !metadata.file_type().is_file() {
        return Err(one(UNREADABLE, format!("The autosave `{file_name}` is not a regular file.")));
    }
    if metadata.len() > MAX_MODEL_DOCUMENT_BYTES {
        return Err(one(TOO_LARGE, format!("The autosave is {} bytes; a model document may be at most {MAX_MODEL_DOCUMENT_BYTES} bytes.", metadata.len())));
    }
    let load = memory.reserve("the autosave's file", loaded_text_bytes(metadata.len())).map_err(memory_refusal)?;
    let mut bytes = Vec::new();
    File::open(&path)
        .and_then(|file| file.take(MAX_MODEL_DOCUMENT_BYTES + 1).read_to_end(&mut bytes))
        .map_err(|error| one(UNREADABLE, format!("The autosave `{file_name}` could not be read: {error}.")))?;
    let hex = {
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        hasher.finalize().iter().map(|byte| format!("{byte:02x}")).collect::<String>()
    };
    if hex != name.model_hash_hex {
        return Err(one(INTEGRITY, format!("The autosave's bytes hash to sha256:{hex}, not the sha256:{} its name records.", name.model_hash_hex)));
    }
    let text = String::from_utf8(bytes).map_err(|_| one(INTEGRITY, "The autosave is not UTF-8 text.".to_string()))?;
    let admitted = admit_text(&text, &name.model_hash_hex, INTEGRITY, memory)?;
    if admitted.slot != name.slot {
        return Err(one(INTEGRITY, "The autosave's document belongs to another project than its name records.".to_string()));
    }
    let byte_count = text.len() as u64;
    drop(text);
    let verified = admitted.document.map(|document| VerifiedAutosave {
        file_name: file_name.to_string(),
        slot: name.slot,
        written_unix_millis: name.written_unix_millis,
        model_hash: format!("sha256:{hex}"),
        byte_count,
        document,
    });
    Ok(verified.holding(load))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::atomic_report_package_save::test_support::atomic_save_test_lock;
    use crate::model_document_file::{read_model_document_file, ModelDocumentOpenOutcome};
    use crate::stored_read::UnmeteredReadMemory;
    use open_pipe_stress_operation_applier::canonical_json;
    use serde_json::json;
    use std::io::Write;
    use std::path::PathBuf;

    const DEMO: &str = include_str!("../../../../fixtures/product_preview/invented_demo_model.json");
    const EVERY_SLOT: &str = include_str!("../../../../fixtures/model_document/invented_every_slot_model.json");
    const STORE_SENTINEL: &[u8] = b"invented store bytes: never opened by an autosave";

    fn canonical(source: &str, edit: impl FnOnce(&mut Value)) -> (String, String) {
        let mut document: Value = serde_json::from_str(source).unwrap();
        edit(&mut document);
        let text = canonical_json(&document);
        let hash = format!("sha256:{}", sha256_hex(&text));
        (text, hash)
    }

    fn demo() -> (String, String) {
        canonical(DEMO, |_| {})
    }

    /// A temporary app data directory holding a sentinel store file, and its
    /// autosave directory path (not created).
    struct Sandbox {
        root: PathBuf,
    }

    impl Sandbox {
        fn new(label: &str) -> Self {
            let root = std::env::temp_dir().join(format!(
                "swbpipe-autosave-{label}-{}-{}",
                std::process::id(),
                std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
            ));
            fs::create_dir_all(&root).unwrap();
            fs::write(root.join("openpipestress-projects.sqlite3"), STORE_SENTINEL).unwrap();
            Self { root }
        }

        fn directory(&self) -> PathBuf {
            self.root.join(AUTOSAVE_DIRECTORY)
        }

        fn names(&self) -> Vec<String> {
            let mut names: Vec<String> = fs::read_dir(self.directory())
                .map(|entries| entries.map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned()).collect())
                .unwrap_or_default();
            names.sort();
            names
        }

        fn assert_store_untouched(&self) {
            assert_eq!(fs::read(self.root.join("openpipestress-projects.sqlite3")).unwrap(), STORE_SENTINEL);
            let mut root_names: Vec<String> = fs::read_dir(&self.root).unwrap().map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned()).collect();
            root_names.sort();
            root_names.retain(|name| name != AUTOSAVE_DIRECTORY);
            assert_eq!(root_names, vec!["openpipestress-projects.sqlite3".to_string()], "nothing else is written beside the store");
        }
    }

    impl Drop for Sandbox {
        fn drop(&mut self) {
            // Only this test's own temporary directory.
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let _ = fs::set_permissions(self.directory(), fs::Permissions::from_mode(0o755));
            }
            if self.root.starts_with(std::env::temp_dir()) {
                let _ = fs::remove_dir_all(&self.root);
            }
        }
    }

    fn write(directory: &Path, text: &str, hash: &str, millis: u64) -> AutosaveWriteOutcome {
        write_autosave(directory, text, hash, millis, &UnmeteredReadMemory)
    }

    fn written_name(outcome: &AutosaveWriteOutcome) -> String {
        match outcome {
            AutosaveWriteOutcome::Written { file_name, .. } => file_name.clone(),
            other => panic!("not written: {other:?}"),
        }
    }

    fn refused_codes(outcome: &AutosaveWriteOutcome) -> Vec<String> {
        match outcome {
            AutosaveWriteOutcome::Refused { diagnostics } => diagnostics.iter().map(|item| item.code.clone()).collect(),
            other => panic!("not refused: {other:?}"),
        }
    }

    fn read_codes(result: Result<Metered<VerifiedAutosave>, Vec<DispositionDiagnostic>>) -> Vec<String> {
        match result {
            Ok(_) => vec!["read".to_string()],
            Err(diagnostics) => diagnostics.into_iter().map(|item| item.code).collect(),
        }
    }

    #[derive(Clone, Copy, PartialEq)]
    enum Fault {
        DiskFullAtWrite,
        InterruptedRename,
        CorruptWrite,
    }

    struct FaultyIo(Fault);

    impl AtomicSaveIo for FaultyIo {
        fn write_all(&mut self, temp: &mut File, bytes: &[u8]) -> io::Result<()> {
            match self.0 {
                // ENOSPC as the OS reports a full disk.
                Fault::DiskFullAtWrite => {
                    temp.write_all(&bytes[..bytes.len() / 2])?;
                    Err(io::Error::from_raw_os_error(28))
                }
                // A write the OS reports as complete whose bytes differ.
                Fault::CorruptWrite => {
                    let mut altered = bytes.to_vec();
                    let last = altered.len() - 2;
                    altered[last] = if altered[last] == b'1' { b'2' } else { b'1' };
                    temp.write_all(&altered)
                }
                Fault::InterruptedRename => temp.write_all(bytes),
            }
        }

        fn flush(&mut self, temp: &mut File) -> io::Result<()> {
            temp.flush()
        }

        fn sync_temp(&mut self, temp: &File) -> io::Result<()> {
            temp.sync_all()
        }

        fn rename(&mut self, source: &Path, destination: &Path) -> io::Result<()> {
            if self.0 == Fault::InterruptedRename {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "injected interrupted rename"));
            }
            fs::rename(source, destination)
        }

        fn cleanup_temp(&mut self, temp_path: &Path) -> io::Result<()> {
            fs::remove_file(temp_path)
        }
    }

    fn assert_no_temp(sandbox: &Sandbox) {
        assert!(sandbox.names().iter().all(|name| !name.starts_with(AUTOSAVE_FORMAT.temp_prefix)), "{:?}", sandbox.names());
    }

    /// A record holds exactly the canonical bytes, is named by slot, time and
    /// hash, re-opens through Open Model Document as the current version, and
    /// reads back verified as the same document and hash.
    #[test]
    fn a_record_is_the_canonical_document_named_by_its_slot_time_and_hash() {
        let _lock = atomic_save_test_lock();
        let sandbox = Sandbox::new("exact");
        let (text, hash) = demo();
        let outcome = write(&sandbox.directory(), &text, &hash, 1_700_000_000_123);
        let AutosaveWriteOutcome::Written { file_name, slot, model_hash, byte_count, durability, warnings, .. } = outcome.clone() else {
            panic!("{outcome:?}");
        };
        assert_eq!(slot, autosave_slot("project:invented-demo-loop-01"));
        assert_eq!(file_name, format!("{slot}-1700000000123-{}.json", &hash["sha256:".len()..]));
        assert!(!file_name.contains("invented"), "a record's name carries no project name or id");
        assert_eq!((model_hash.as_str(), byte_count, durability.as_str(), warnings.len()), (hash.as_str(), text.len() as u64, "durable", 0));
        let path = sandbox.directory().join(&file_name);
        assert_eq!(fs::read(&path).unwrap(), text.as_bytes());
        let ModelDocumentOpenOutcome::Opened { schema_status, document, .. } = read_model_document_file(&path) else {
            panic!("the record does not re-open");
        };
        assert_eq!(schema_status, "current");
        assert_eq!(canonical_json(&document), text);
        let verified = read_autosave(&sandbox.directory(), &file_name, &UnmeteredReadMemory).unwrap().into_unreserved().unwrap();
        assert_eq!((verified.model_hash.as_str(), verified.written_unix_millis, verified.byte_count), (hash.as_str(), 1_700_000_000_123, text.len() as u64));
        assert_eq!(canonical_json(&verified.document), text);
        let listing = list_autosaves(&sandbox.directory()).unwrap();
        assert_eq!(listing.entries.len(), 1);
        assert_eq!(listing.entries[0].file_name, file_name);
        assert_eq!(listing.not_listed, 0);
        assert_no_temp(&sandbox);
        sandbox.assert_store_untouched();
    }

    #[test]
    fn the_same_model_again_is_unchanged_and_writes_nothing() {
        let _lock = atomic_save_test_lock();
        let sandbox = Sandbox::new("unchanged");
        let (text, hash) = demo();
        let first = written_name(&write(&sandbox.directory(), &text, &hash, 1_000));
        let again = write(&sandbox.directory(), &text, &hash, 2_000);
        assert_eq!(again, AutosaveWriteOutcome::Unchanged { file_name: first.clone(), slot: autosave_slot("project:invented-demo-loop-01"), model_hash: hash });
        assert_eq!(sandbox.names(), vec![first]);
    }

    /// A newer record of the slot removes the slot's older records, and only
    /// them: another project's record, other files, a symlink and a directory
    /// named like the slot's records stay.
    #[test]
    fn a_new_record_removes_only_its_own_slots_older_records() {
        let _lock = atomic_save_test_lock();
        let sandbox = Sandbox::new("prune");
        let directory = sandbox.directory();
        let (first, first_hash) = demo();
        let (second, second_hash) = canonical(DEMO, |document| document["project"]["name"] = json!("Invented Utility Loop Demo, edited"));
        let (other, other_hash) = canonical(EVERY_SLOT, |_| {});
        let first_name = written_name(&write(&directory, &first, &first_hash, 1_000));
        let other_name = written_name(&write(&directory, &other, &other_hash, 1_500));
        let slot = autosave_slot("project:invented-demo-loop-01");
        let lookalike_dir = format!("{slot}-0000000000900-{}.json", "a".repeat(64));
        fs::create_dir(directory.join(&lookalike_dir)).unwrap();
        fs::write(directory.join("notes.json"), b"invented").unwrap();
        #[cfg(unix)]
        let lookalike_link = {
            let link = format!("{slot}-0000000000800-{}.json", "b".repeat(64));
            std::os::unix::fs::symlink(directory.join("notes.json"), directory.join(&link)).unwrap();
            link
        };
        let second_name = written_name(&write(&directory, &second, &second_hash, 2_000));
        let names = sandbox.names();
        assert!(!names.contains(&first_name), "the slot's older record is removed: {names:?}");
        for kept in [&second_name, &other_name, &lookalike_dir, &"notes.json".to_string()] {
            assert!(names.contains(kept), "{kept} is kept: {names:?}");
        }
        #[cfg(unix)]
        assert!(names.contains(&lookalike_link), "a symlink is never removed: {names:?}");
        let listed: Vec<String> = list_autosaves(&directory).unwrap().entries.into_iter().map(|entry| entry.file_name).collect();
        assert_eq!(listed, vec![second_name, other_name], "newest first; only regular record files are listed");
        sandbox.assert_store_untouched();
    }

    /// Content refusals come before the directory is touched.
    #[test]
    fn a_text_that_is_not_a_current_canonical_document_of_its_hash_is_refused_before_any_write() {
        let sandbox = Sandbox::new("refusals");
        let directory = sandbox.directory();
        let (text, hash) = demo();
        let other_hash = format!("sha256:{}", "0".repeat(64));
        let pretty = serde_json::to_string_pretty(&serde_json::from_str::<Value>(&text).unwrap()).unwrap();
        let pretty_hash = format!("sha256:{}", sha256_hex(&pretty));
        let (retired, retired_hash) = canonical(DEMO, |document| document["schema_version"] = json!("0.7.0"));
        let (newer, newer_hash) = canonical(DEMO, |document| document["schema_version"] = json!("0.9.0"));
        let (kind, kind_hash) = canonical(DEMO, |document| document["document_kind"] = json!("openpipestress.other"));
        for (label, text, claim, code) in [
            ("hash", text.as_str(), other_hash.as_str(), HASH_MISMATCH),
            ("claim form", text.as_str(), "sha256:ABC", HASH_MISMATCH),
            ("not canonical", pretty.as_str(), pretty_hash.as_str(), HASH_MISMATCH),
            ("retired", retired.as_str(), retired_hash.as_str(), "MODEL-FILE-SCHEMA-REFUSED"),
            ("newer", newer.as_str(), newer_hash.as_str(), "MODEL-FILE-SCHEMA-REFUSED"),
            ("kind", kind.as_str(), kind_hash.as_str(), "MODEL-FILE-DOCUMENT-KIND"),
        ] {
            let outcome = write(&directory, text, claim, 1_000);
            assert_eq!(refused_codes(&outcome)[0], code, "{label}");
            assert!(!directory.exists(), "{label}: nothing is created");
        }
        let _ = hash;
        sandbox.assert_store_untouched();
    }

    /// A memory refusal refuses the record before its text is parsed.
    #[test]
    fn a_memory_refusal_refuses_before_parsing() {
        struct Refusing;
        impl ReadMemory for Refusing {
            fn reserve(&self, what: &str, bytes: u64) -> Result<ReadReservation, String> {
                Err(format!("{STORE_READ_MEMORY_UNAVAILABLE}: reading {what} needs {bytes} bytes; refused."))
            }
        }
        let sandbox = Sandbox::new("memory");
        let (text, hash) = demo();
        let outcome = write_autosave(&sandbox.directory(), &text, &hash, 1_000, &Refusing);
        assert_eq!(refused_codes(&outcome), vec![STORE_READ_MEMORY_UNAVAILABLE.to_string()]);
        assert!(!sandbox.directory().exists());
    }

    /// Disk full during the write: refused by name with the OS error, the
    /// temporary file removed, the previous record kept and the store untouched.
    #[test]
    fn disk_full_is_reported_and_keeps_the_previous_record() {
        let _lock = atomic_save_test_lock();
        let sandbox = Sandbox::new("enospc");
        let directory = sandbox.directory();
        let (first, first_hash) = demo();
        let (second, second_hash) = canonical(DEMO, |document| document["project"]["name"] = json!("edited"));
        let first_name = written_name(&write(&directory, &first, &first_hash, 1_000));
        let outcome = write_autosave_with_io(&directory, &second, &second_hash, 2_000, &UnmeteredReadMemory, &mut FaultyIo(Fault::DiskFullAtWrite));
        let AutosaveWriteOutcome::Refused { diagnostics } = &outcome else { panic!("{outcome:?}") };
        assert_eq!(diagnostics[0].code, "AUTOSAVE-WRITE-FAILED");
        assert!(diagnostics[0].message.contains("os error 28"), "{}", diagnostics[0].message);
        assert_eq!(sandbox.names(), vec![first_name.clone()]);
        assert_eq!(fs::read(directory.join(&first_name)).unwrap(), first.as_bytes());
        assert_no_temp(&sandbox);
        sandbox.assert_store_untouched();
    }

    /// Interrupted rename: refused by name, the temporary file removed, the
    /// previous record kept.
    #[test]
    fn an_interrupted_rename_is_reported_and_keeps_the_previous_record() {
        let _lock = atomic_save_test_lock();
        let sandbox = Sandbox::new("rename");
        let directory = sandbox.directory();
        let (first, first_hash) = demo();
        let (second, second_hash) = canonical(DEMO, |document| document["project"]["name"] = json!("edited"));
        let first_name = written_name(&write(&directory, &first, &first_hash, 1_000));
        let outcome = write_autosave_with_io(&directory, &second, &second_hash, 2_000, &UnmeteredReadMemory, &mut FaultyIo(Fault::InterruptedRename));
        let AutosaveWriteOutcome::Refused { diagnostics } = &outcome else { panic!("{outcome:?}") };
        assert_eq!(diagnostics[0].code, "AUTOSAVE-RENAME-FAILED");
        assert!(diagnostics[0].message.contains("rename stage"), "{}", diagnostics[0].message);
        assert_eq!(sandbox.names(), vec![first_name.clone()]);
        assert_eq!(read_codes(read_autosave(&directory, &first_name, &UnmeteredReadMemory)), vec!["read".to_string()]);
        assert_no_temp(&sandbox);
        sandbox.assert_store_untouched();
    }

    /// Permission denied (a real read-only directory): refused by name, the
    /// previous record kept.
    #[cfg(unix)]
    #[test]
    fn permission_denied_is_reported_and_keeps_the_previous_record() {
        use std::os::unix::fs::PermissionsExt;
        let _lock = atomic_save_test_lock();
        let sandbox = Sandbox::new("eacces");
        let directory = sandbox.directory();
        let (first, first_hash) = demo();
        let (second, second_hash) = canonical(DEMO, |document| document["project"]["name"] = json!("edited"));
        let first_name = written_name(&write(&directory, &first, &first_hash, 1_000));
        fs::set_permissions(&directory, fs::Permissions::from_mode(0o555)).unwrap();
        if File::create(directory.join("probe.tmp")).is_ok() {
            eprintln!("skipped: this user can write a read-only directory");
            return;
        }
        let outcome = write(&directory, &second, &second_hash, 2_000);
        let AutosaveWriteOutcome::Refused { diagnostics } = &outcome else { panic!("{outcome:?}") };
        assert_eq!(diagnostics[0].code, "AUTOSAVE-TEMP-CREATE-FAILED");
        assert!(diagnostics[0].message.to_lowercase().contains("permission denied"), "{}", diagnostics[0].message);
        fs::set_permissions(&directory, fs::Permissions::from_mode(0o755)).unwrap();
        assert_eq!(sandbox.names(), vec![first_name]);
        sandbox.assert_store_untouched();
    }

    /// A write the OS reports complete but whose bytes differ is caught by the
    /// read-back: that new file alone is removed and the previous record kept.
    #[test]
    fn a_record_that_does_not_read_back_as_its_hash_is_removed() {
        let _lock = atomic_save_test_lock();
        let sandbox = Sandbox::new("readback");
        let directory = sandbox.directory();
        let (first, first_hash) = demo();
        let (second, second_hash) = canonical(DEMO, |document| document["project"]["name"] = json!("edited"));
        let first_name = written_name(&write(&directory, &first, &first_hash, 1_000));
        let outcome = write_autosave_with_io(&directory, &second, &second_hash, 2_000, &UnmeteredReadMemory, &mut FaultyIo(Fault::CorruptWrite));
        assert_eq!(refused_codes(&outcome), vec![READBACK_MISMATCH.to_string()]);
        assert_eq!(sandbox.names(), vec![first_name]);
        sandbox.assert_store_untouched();
    }

    /// A crash before the rename leaves a temporary file: it is never listed or
    /// read, and the previous record is still the one recovery finds.
    #[test]
    fn a_temporary_file_left_by_a_crash_is_never_listed_or_read() {
        let _lock = atomic_save_test_lock();
        let sandbox = Sandbox::new("crash");
        let directory = sandbox.directory();
        let (first, first_hash) = demo();
        let first_name = written_name(&write(&directory, &first, &first_hash, 1_000));
        let temp = format!("{}{}.tmp", AUTOSAVE_FORMAT.temp_prefix, "c".repeat(32));
        fs::write(directory.join(&temp), &first.as_bytes()[..first.len() / 3]).unwrap();
        let listed: Vec<String> = list_autosaves(&directory).unwrap().entries.into_iter().map(|entry| entry.file_name).collect();
        assert_eq!(listed, vec![first_name]);
        assert_eq!(read_codes(read_autosave(&directory, &temp, &UnmeteredReadMemory)), vec![NAME_INVALID.to_string()]);
    }

    /// A read verifies the record's bytes against its name, its content and its
    /// slot; a name that is not a record's (with a separator, say) is refused.
    #[test]
    fn a_read_refuses_a_tampered_renamed_or_foreign_record() {
        let _lock = atomic_save_test_lock();
        let sandbox = Sandbox::new("read");
        let directory = sandbox.directory();
        let (text, hash) = demo();
        let name = written_name(&write(&directory, &text, &hash, 1_000));
        let parsed = AutosaveName::parse(&name).unwrap();
        // Another valid canonical document of the slot under this record's
        // hash: only the check of the bytes against the name refuses it.
        let (tampered, _) = canonical(DEMO, |document| document["project"]["name"] = json!("Invented, altered"));
        let tampered_name = AutosaveName { written_unix_millis: 2_000, ..parsed.clone() }.file_name();
        fs::write(directory.join(&tampered_name), &tampered).unwrap();
        assert_eq!(read_codes(read_autosave(&directory, &tampered_name, &UnmeteredReadMemory)), vec![INTEGRITY.to_string()]);
        // The right bytes under another project's slot.
        let foreign_name = AutosaveName { slot: autosave_slot("project:another"), ..parsed.clone() }.file_name();
        fs::copy(directory.join(&name), directory.join(&foreign_name)).unwrap();
        assert_eq!(read_codes(read_autosave(&directory, &foreign_name, &UnmeteredReadMemory)), vec![INTEGRITY.to_string()]);
        // A canonical document of another version, honestly named.
        let (retired, retired_hash) = canonical(DEMO, |document| document["schema_version"] = json!("0.7.0"));
        let retired_name = AutosaveName { written_unix_millis: 3_000, model_hash_hex: retired_hash["sha256:".len()..].to_string(), ..parsed.clone() }.file_name();
        fs::write(directory.join(&retired_name), &retired).unwrap();
        assert_eq!(read_codes(read_autosave(&directory, &retired_name, &UnmeteredReadMemory)), vec!["MODEL-FILE-SCHEMA-REFUSED".to_string()]);
        for bad in ["../openpipestress-projects.sqlite3", "notes.json", &format!("{}/{}", AUTOSAVE_DIRECTORY, name)] {
            assert_eq!(read_codes(read_autosave(&directory, bad, &UnmeteredReadMemory)), vec![NAME_INVALID.to_string()], "{bad}");
        }
        let missing = AutosaveName { written_unix_millis: 9_000, ..parsed.clone() }.file_name();
        assert_eq!(read_codes(read_autosave(&directory, &missing, &UnmeteredReadMemory)), vec![NOT_FOUND.to_string()]);
        #[cfg(unix)]
        {
            let link = AutosaveName { written_unix_millis: 4_000, ..parsed.clone() }.file_name();
            std::os::unix::fs::symlink(directory.join(&name), directory.join(&link)).unwrap();
            assert_eq!(read_codes(read_autosave(&directory, &link, &UnmeteredReadMemory)), vec![UNREADABLE.to_string()]);
        }
        assert_eq!(read_codes(read_autosave(&directory, &name, &UnmeteredReadMemory)), vec!["read".to_string()]);
        sandbox.assert_store_untouched();
    }

    #[test]
    fn names_parse_only_in_their_exact_form() {
        let hex = "a".repeat(64);
        let slot = "b".repeat(32);
        let good = format!("{slot}-0000000001000-{hex}.json");
        assert_eq!(AutosaveName::parse(&good).map(|name| name.file_name()), Some(good));
        for bad in [
            format!("{slot}-1000-{hex}.json"),
            format!("{slot}-0000000001000-{hex}.JSON"),
            format!("{}-0000000001000-{hex}.json", "B".repeat(32)),
            format!("{slot}-0000000001000-{hex}-x.json"),
            format!("{slot}-000000000100a-{hex}.json"),
            format!("x/{slot}-0000000001000-{hex}.json"),
        ] {
            assert_eq!(AutosaveName::parse(&bad), None, "{bad}");
        }
        assert_eq!(autosave_slot("project:x").len(), 32);
        assert_ne!(autosave_slot("project:x"), autosave_slot("project:y"));
    }

    #[test]
    fn a_directory_that_is_not_a_directory_is_refused() {
        let sandbox = Sandbox::new("not-dir");
        fs::write(sandbox.directory(), b"invented").unwrap();
        let (text, hash) = demo();
        assert_eq!(refused_codes(&write(&sandbox.directory(), &text, &hash, 1_000)), vec![DIRECTORY_UNAVAILABLE.to_string()]);
        assert_eq!(list_autosaves(&sandbox.directory()).unwrap_err().code, DIRECTORY_UNAVAILABLE);
    }
}
