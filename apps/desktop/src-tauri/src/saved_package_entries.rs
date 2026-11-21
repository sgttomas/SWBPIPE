//! The project's saved-package entries (DEL-08-01 REQ-023; DEL-02-05 REQ-039,
//! AC-017): the record, kept by the project store, that a report package was
//! saved from the project.
//!
//! An entry holds exactly what DEL-08-01's Ontology names and nothing of the
//! package's content: the package's identity (the manifest hash), the
//! container's SHA-256 and byte count, the file name, the location, the time
//! of the save, the export context, whether durability was confirmed, and the
//! basis (the model hash, the SHA-256 of the result as received, and the
//! analysis run's identity and record hash). It is written from the native
//! save's own outcome ([`PublishedPackage`], built at the save's end from what
//! the save wrote and bound), only after the package is published, and is
//! never edited: the table refuses every `UPDATE` and every `INSERT` that
//! meets an existing entry. The project's report list is its entries in the
//! order saved (`sequence`).
//!
//! The location belongs to this machine (REQ-023 "This machine"). It is read
//! only by the state check, here, and leaves this module in no reply, error,
//! diagnostic or log line: [`SavedPackageEntry`] has no location member, and
//! every message built here from an error has the location removed
//! ([`withheld`]). An entry is named by its `entry_id`, a random token
//! allocated when it is written.
//!
//! The store keeps them as REQ-039 says: they are read back unchanged and in
//! order, removed with the project (the foreign key cascades), and adding or
//! removing one changes no other record. Removing an entry removes the record
//! and never the file. Nothing here reads or writes a portable project
//! package, and an autosave recovery record holds no entry.
//!
//! Not built here (lane 4, narrowed by Agent 0): the session-held entry
//! before a project's first save (a save from a project the store does not
//! hold writes no entry, and its outcome says so); matching an entry's basis
//! across the project's recorded runs; the embedded agent's read.

use crate::command_ingress::CommandError;
use crate::model_states::{now_unix_millis, random_token, utc_iso_millis};
use rusqlite::{params, Connection, OptionalExtension};
use serde::Serialize;
use std::io::Read;
use std::path::{Path, PathBuf};
use tauri::AppHandle;

/// Store v15: the entries' table and its immutability triggers. Every
/// statement is `IF NOT EXISTS`, so the step is safe to re-run; it runs in one
/// transaction with its `user_version` update (`apply_store_migrations`).
/// `saved_package_entries_immutable` refuses any `UPDATE`;
/// `saved_package_entries_no_replace` refuses an `INSERT` that meets an
/// existing entry of the project on either key (`entry_id` or `sequence`),
/// before conflict resolution, so `INSERT OR REPLACE` cannot overwrite one.
/// Deleting the project row cascades to its entries.
pub(crate) const STORE_V15_SAVED_PACKAGE_ENTRIES_SQL: &str = "
CREATE TABLE IF NOT EXISTS saved_package_entries(
  project_id TEXT NOT NULL, entry_id TEXT NOT NULL, sequence INTEGER NOT NULL,
  package_identity_sha256 TEXT NOT NULL, container_sha256 TEXT NOT NULL,
  container_byte_count INTEGER NOT NULL CHECK (container_byte_count >= 0),
  file_name TEXT NOT NULL, location BLOB NOT NULL, saved_at TEXT NOT NULL,
  export_context TEXT NOT NULL,
  durability_confirmed INTEGER NOT NULL CHECK (durability_confirmed IN (0, 1)),
  model_hash TEXT NOT NULL, received_result_sha256 TEXT NOT NULL,
  run_id TEXT NOT NULL, run_record_sha256 TEXT NOT NULL,
  PRIMARY KEY(project_id, entry_id), UNIQUE(project_id, sequence),
  FOREIGN KEY(project_id) REFERENCES local_projects(project_id) ON DELETE CASCADE);
CREATE TRIGGER IF NOT EXISTS saved_package_entries_immutable BEFORE UPDATE ON saved_package_entries
  BEGIN SELECT RAISE(ABORT, 'SAVED-PACKAGE-ENTRY-IMMUTABLE'); END;
CREATE TRIGGER IF NOT EXISTS saved_package_entries_no_replace BEFORE INSERT ON saved_package_entries
  WHEN EXISTS (SELECT 1 FROM saved_package_entries WHERE project_id = NEW.project_id
    AND (entry_id = NEW.entry_id OR sequence = NEW.sequence))
  BEGIN SELECT RAISE(ABORT, 'SAVED-PACKAGE-ENTRY-IMMUTABLE'); END;
";

pub(crate) fn migrate_v15_saved_package_entries(connection: &Connection) -> Result<(), String> {
    connection
        .execute_batch(STORE_V15_SAVED_PACKAGE_ENTRIES_SQL)
        .map_err(|error| error.to_string())
}

pub(crate) const ENTRY_KEPT: &str = "SAVED-PACKAGE-ENTRY-KEPT";
/// No package was published (a cancelled chooser), so no entry was added.
pub(crate) const ENTRY_NOT_APPLICABLE: &str = "SAVED-PACKAGE-ENTRY-NOT-APPLICABLE";
/// The store does not hold the project the package was saved from.
pub(crate) const ENTRY_PROJECT_NOT_SAVED: &str = "SAVED-PACKAGE-ENTRY-PROJECT-NOT-SAVED";
/// The save was given no run record, so the entry's basis is incomplete.
pub(crate) const ENTRY_NO_RUN_RECORD: &str = "SAVED-PACKAGE-ENTRY-NO-RUN-RECORD";
/// The save was given a run record that did not verify against the package's
/// source result and input manifest.
pub(crate) const ENTRY_RUN_RECORD_UNVERIFIED: &str = "SAVED-PACKAGE-ENTRY-RUN-RECORD-UNVERIFIED";
/// Keeping the entry failed, or could not be confirmed.
pub(crate) const ENTRY_NOT_KEPT: &str = "SAVED-PACKAGE-ENTRY-NOT-KEPT";
/// A command named an entry the project does not hold.
pub(crate) const ENTRY_UNKNOWN: &str = "SAVED-PACKAGE-ENTRY-UNKNOWN";
pub(crate) const ENTRY_STORE_FAILED: &str = "SAVED-PACKAGE-ENTRY-STORE-FAILED";

/// The text that stands in a message for a location.
const LOCATION_WITHHELD: &str = "<saved location withheld>";

/// The basis the native save bound: the model text's SHA-256 (the audit model
/// hash), and the run record the save checked: its `run_id`, its recomputed
/// record hash and its received-result checksum, which the check found equal
/// to the checked SHA-256 of the result the package was produced from.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct SavedPackageBasis {
    pub(crate) model_hash: String,
    pub(crate) received_result_sha256: String,
    pub(crate) run_id: String,
    pub(crate) run_record_sha256: String,
}

/// What the save bound for an entry, at admission.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum EntryBasis {
    Bound(SavedPackageBasis),
    /// The save was given no run record.
    NoRunRecord,
    /// The save was given a run record that did not verify, with the reason.
    Unverified(String),
}

/// A published package, as the native save hands it to the store: built at
/// the save's end from the package it wrote, the destination it wrote it to,
/// the writer's durability and what admission bound. Never built from the
/// receipt's serialized form or from anything else the web view sends.
#[derive(Clone, Debug)]
pub struct PublishedPackage {
    pub(crate) project_id: String,
    pub(crate) package_identity_sha256: String,
    pub(crate) container_sha256: String,
    pub(crate) container_byte_count: u64,
    pub(crate) location: PathBuf,
    /// UTC ISO-8601 with milliseconds, taken when the writer returned.
    pub(crate) saved_at: String,
    pub(crate) export_context: String,
    pub(crate) durability_confirmed: bool,
    pub(crate) basis: EntryBasis,
}

impl PublishedPackage {
    /// The saved file's name: the location's last component.
    pub(crate) fn file_name(&self) -> String {
        self.location.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default()
    }
}

/// The entry-storage outcome of a save, reported apart from the package's
/// outcome (REQ-023 "Entry"). `status` is `kept`, `not_kept` or
/// `not_applicable`; only `kept` says an entry was kept, and names it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct SavedPackageEntryOutcome {
    pub status: String,
    pub code: String,
    pub entry_id: String,
    pub message: String,
}

impl SavedPackageEntryOutcome {
    fn kept(entry_id: String) -> Self {
        Self { status: "kept".to_string(), code: ENTRY_KEPT.to_string(), entry_id, message: "The saved package is listed in the project.".to_string() }
    }

    pub(crate) fn not_kept(code: &str, message: impl Into<String>) -> Self {
        Self { status: "not_kept".to_string(), code: code.to_string(), entry_id: String::new(), message: message.into() }
    }

    pub(crate) fn not_applicable() -> Self {
        Self {
            status: "not_applicable".to_string(),
            code: ENTRY_NOT_APPLICABLE.to_string(),
            entry_id: String::new(),
            message: "No package was saved, so no entry was added.".to_string(),
        }
    }
}

/// One entry as the store returns it: every stored member but the location.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct SavedPackageEntry {
    pub(crate) entry_id: String,
    pub(crate) sequence: i64,
    pub(crate) package_identity_sha256: String,
    pub(crate) container_sha256: String,
    pub(crate) container_byte_count: i64,
    pub(crate) file_name: String,
    pub(crate) saved_at: String,
    pub(crate) export_context: String,
    pub(crate) durability_confirmed: bool,
    pub(crate) basis: SavedPackageBasis,
    /// Always `kept_with_project` today: no entry is held by the session.
    pub(crate) kept: &'static str,
}

/// The list reply: the project's entries in the order saved.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct SavedPackageList {
    pub(crate) project_id: String,
    pub(crate) entries: Vec<SavedPackageEntry>,
}

/// What the state check found at an entry's location (REQ-023 "State").
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct SavedPackageState {
    pub(crate) entry_id: String,
    /// `present_identical`, `changed`, `not_found` or `could_not_be_read`.
    pub(crate) state: &'static str,
    /// Why, for every state but `present_identical`; never the location.
    pub(crate) reason: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct SavedPackageRemoval {
    pub(crate) entry_id: String,
    /// The record was removed; the file was not touched.
    pub(crate) removed: bool,
    pub(crate) message: String,
}

/// `text` with every occurrence of the location replaced.
pub(crate) fn withheld(text: &str, location: &Path) -> String {
    let shown = location.to_string_lossy();
    if shown.is_empty() {
        return text.to_string();
    }
    text.replace(shown.as_ref(), LOCATION_WITHHELD)
}

#[cfg(unix)]
fn location_bytes(location: &Path) -> Option<Vec<u8>> {
    use std::os::unix::ffi::OsStrExt;
    Some(location.as_os_str().as_bytes().to_vec())
}

#[cfg(not(unix))]
fn location_bytes(location: &Path) -> Option<Vec<u8>> {
    location.to_str().map(|text| text.as_bytes().to_vec())
}

#[cfg(unix)]
fn location_path(bytes: Vec<u8>) -> Option<PathBuf> {
    use std::os::unix::ffi::OsStringExt;
    Some(PathBuf::from(std::ffi::OsString::from_vec(bytes)))
}

#[cfg(not(unix))]
fn location_path(bytes: Vec<u8>) -> Option<PathBuf> {
    String::from_utf8(bytes).ok().map(PathBuf::from)
}

/// Keeps a published package's entry in the store at `path`: one `BEGIN
/// IMMEDIATE` transaction that checks the store holds the project, allocates
/// the next sequence and a fresh entry id, inserts the row and commits. Any
/// failure, the commit's included, is `not_kept`: nothing claims the entry
/// was kept unless the commit returned. The package is never touched here.
pub(crate) fn keep_entry_in_store(path: &Path, published: &PublishedPackage) -> SavedPackageEntryOutcome {
    let basis = match &published.basis {
        EntryBasis::Bound(basis) => basis,
        EntryBasis::NoRunRecord => {
            return SavedPackageEntryOutcome::not_kept(
                ENTRY_NO_RUN_RECORD,
                "The package was saved, but it is not listed in the project: the save was given no analysis run record, so the entry's basis is incomplete.",
            )
        }
        EntryBasis::Unverified(reason) => {
            return SavedPackageEntryOutcome::not_kept(
                ENTRY_RUN_RECORD_UNVERIFIED,
                format!(
                    "The package was saved, but it is not listed in the project: the analysis run record given with it did not verify ({}).",
                    withheld(reason, &published.location)
                ),
            )
        }
    };
    let outcome = (|| -> Result<Option<String>, String> {
        let location = location_bytes(&published.location).ok_or_else(|| "the saved location cannot be recorded on this machine".to_string())?;
        let byte_count = i64::try_from(published.container_byte_count).map_err(|_| "the package's byte count is out of range".to_string())?;
        let (mut connection, _migration) = crate::open_project_store(path)?;
        let transaction = connection
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
            .map_err(|error| error.to_string())?;
        let held: Option<i64> = transaction
            .query_row("SELECT 1 FROM local_projects WHERE project_id = ?1", params![published.project_id], |row| row.get(0))
            .optional()
            .map_err(|error| error.to_string())?;
        if held.is_none() {
            return Ok(None);
        }
        let sequence: i64 = transaction
            .query_row(
                "SELECT COALESCE(MAX(sequence), 0) + 1 FROM saved_package_entries WHERE project_id = ?1",
                params![published.project_id],
                |row| row.get(0),
            )
            .map_err(|error| error.to_string())?;
        let entry_id = format!("saved-package:{}", random_token()?);
        transaction
            .execute(
                "INSERT INTO saved_package_entries (project_id, entry_id, sequence, package_identity_sha256, container_sha256,
                   container_byte_count, file_name, location, saved_at, export_context, durability_confirmed,
                   model_hash, received_result_sha256, run_id, run_record_sha256)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
                params![
                    published.project_id,
                    entry_id,
                    sequence,
                    published.package_identity_sha256,
                    published.container_sha256,
                    byte_count,
                    published.file_name(),
                    location,
                    published.saved_at,
                    published.export_context,
                    i64::from(published.durability_confirmed),
                    basis.model_hash,
                    basis.received_result_sha256,
                    basis.run_id,
                    basis.run_record_sha256,
                ],
            )
            .map_err(|error| error.to_string())?;
        #[cfg(test)]
        tests::fault::before_commit()?;
        transaction.commit().map_err(|error| error.to_string())?;
        Ok(Some(entry_id))
    })();
    match outcome {
        Ok(Some(entry_id)) => SavedPackageEntryOutcome::kept(entry_id),
        Ok(None) => SavedPackageEntryOutcome::not_kept(
            ENTRY_PROJECT_NOT_SAVED,
            "The package was saved, but it is not listed in the project: the project is not yet saved.",
        ),
        Err(error) => SavedPackageEntryOutcome::not_kept(
            ENTRY_NOT_KEPT,
            format!(
                "The package was saved, but its entry could not be kept in the project, or keeping it could not be confirmed: {}",
                withheld(&error, &published.location)
            ),
        ),
    }
}

/// [`keep_entry_in_store`] for the application's store; a store path that
/// cannot be found is `not_kept`.
pub(crate) fn keep_entry(store: impl FnOnce() -> Result<PathBuf, String>, published: &PublishedPackage) -> SavedPackageEntryOutcome {
    match store() {
        Ok(path) => keep_entry_in_store(&path, published),
        Err(error) => SavedPackageEntryOutcome::not_kept(
            ENTRY_NOT_KEPT,
            format!("The package was saved, but its entry could not be kept in the project: {}", withheld(&error, &published.location)),
        ),
    }
}

const ENTRY_COLUMNS: &str = "entry_id, sequence, package_identity_sha256, container_sha256, container_byte_count, file_name,
  saved_at, export_context, durability_confirmed, model_hash, received_result_sha256, run_id, run_record_sha256";

fn entry_of(row: &rusqlite::Row<'_>) -> rusqlite::Result<SavedPackageEntry> {
    Ok(SavedPackageEntry {
        entry_id: row.get(0)?,
        sequence: row.get(1)?,
        package_identity_sha256: row.get(2)?,
        container_sha256: row.get(3)?,
        container_byte_count: row.get(4)?,
        file_name: row.get(5)?,
        saved_at: row.get(6)?,
        export_context: row.get(7)?,
        durability_confirmed: row.get::<_, i64>(8)? == 1,
        basis: SavedPackageBasis { model_hash: row.get(9)?, received_result_sha256: row.get(10)?, run_id: row.get(11)?, run_record_sha256: row.get(12)? },
        kept: "kept_with_project",
    })
}

/// The project's entries, unchanged and in the order saved.
pub(crate) fn list_entries(connection: &Connection, project_id: &str) -> Result<Vec<SavedPackageEntry>, String> {
    let mut statement = connection
        .prepare(&format!("SELECT {ENTRY_COLUMNS} FROM saved_package_entries WHERE project_id = ?1 ORDER BY sequence"))
        .map_err(|error| error.to_string())?;
    let entries = statement
        .query_map(params![project_id], entry_of)
        .map_err(|error| error.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())?;
    Ok(entries)
}

/// The location and SHA-256 of one entry, read only for the state check.
fn entry_location(connection: &Connection, project_id: &str, entry_id: &str) -> Result<(PathBuf, String), String> {
    let row: Option<(Vec<u8>, String)> = connection
        .query_row(
            "SELECT location, container_sha256 FROM saved_package_entries WHERE project_id = ?1 AND entry_id = ?2",
            params![project_id, entry_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()
        .map_err(|error| error.to_string())?;
    let (bytes, sha256) = row.ok_or_else(|| format!("{ENTRY_UNKNOWN}: the project holds no entry {entry_id}."))?;
    let location = location_path(bytes).ok_or_else(|| format!("{ENTRY_STORE_FAILED}: the entry's location cannot be read on this machine."))?;
    Ok((location, sha256))
}

/// The state found at `location` for a file whose SHA-256 should be
/// `expected`: it reads the file and nothing else; it writes nothing.
pub(crate) fn state_at(location: &Path, expected: &str) -> (&'static str, String) {
    let kind_reason = |error: &std::io::Error| match error.kind() {
        std::io::ErrorKind::PermissionDenied => "permission to read the file was denied".to_string(),
        kind => format!("the file could not be read ({kind:?})"),
    };
    let metadata = match std::fs::metadata(location) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return ("not_found", "no file is at the saved location".to_string()),
        Err(error) => return ("could_not_be_read", kind_reason(&error)),
    };
    if !metadata.is_file() {
        return ("could_not_be_read", "the saved location is not a regular file".to_string());
    }
    let mut file = match std::fs::File::open(location) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return ("not_found", "no file is at the saved location".to_string()),
        Err(error) => return ("could_not_be_read", kind_reason(&error)),
    };
    let mut hasher = <sha2::Sha256 as sha2::Digest>::new();
    let mut buffer = vec![0u8; 1 << 20];
    loop {
        match file.read(&mut buffer) {
            Ok(0) => break,
            Ok(read) => sha2::Digest::update(&mut hasher, &buffer[..read]),
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(error) => return ("could_not_be_read", kind_reason(&error)),
        }
    }
    let found: String = sha2::Digest::finalize(hasher).iter().map(|byte| format!("{byte:02x}")).collect();
    if found == expected {
        ("present_identical", String::new())
    } else {
        ("changed", "the file at the saved location is not the saved package (its SHA-256 differs)".to_string())
    }
}

/// The state check of one entry in the store at `path`.
pub(crate) fn check_entry_in_store(path: &Path, project_id: &str, entry_id: &str) -> Result<SavedPackageState, String> {
    let (connection, _migration) = crate::open_project_store(path)?;
    let (location, expected) = entry_location(&connection, project_id, entry_id)?;
    drop(connection);
    let (state, reason) = state_at(&location, &expected);
    Ok(SavedPackageState { entry_id: entry_id.to_string(), state, reason: withheld(&reason, &location) })
}

/// Removes one entry's record in the store at `path`; never the file.
pub(crate) fn remove_entry_in_store(path: &Path, project_id: &str, entry_id: &str) -> Result<SavedPackageRemoval, String> {
    let (connection, _migration) = crate::open_project_store(path)?;
    let removed = connection
        .execute("DELETE FROM saved_package_entries WHERE project_id = ?1 AND entry_id = ?2", params![project_id, entry_id])
        .map_err(|error| error.to_string())?;
    if removed == 0 {
        return Err(format!("{ENTRY_UNKNOWN}: the project holds no entry {entry_id}."));
    }
    Ok(SavedPackageRemoval {
        entry_id: entry_id.to_string(),
        removed: true,
        message: "The entry was removed from the project's report list. The saved file was not touched.".to_string(),
    })
}

pub(crate) fn list_entries_in_store(path: &Path, project_id: &str) -> Result<SavedPackageList, String> {
    let (connection, _migration) = crate::open_project_store(path)?;
    Ok(SavedPackageList { project_id: project_id.to_string(), entries: list_entries(&connection, project_id)? })
}

/// The time of a save, taken when its writer returns.
pub(crate) fn save_time() -> String {
    now_unix_millis().map(utc_iso_millis).unwrap_or_default()
}

/// The project's report list: its entries in the order saved, without their
/// locations.
#[tauri::command]
pub fn list_saved_package_entries(app: AppHandle, project_id: String) -> Result<SavedPackageList, CommandError> {
    crate::app_store_path(&app)
        .and_then(|path| list_entries_in_store(&path, &project_id))
        .map_err(CommandError::Command)
}

/// What is found now at one entry's location; reads the file, writes nothing.
#[tauri::command]
pub fn check_saved_package_entry(app: AppHandle, project_id: String, entry_id: String) -> Result<SavedPackageState, CommandError> {
    crate::app_store_path(&app)
        .and_then(|path| check_entry_in_store(&path, &project_id, &entry_id))
        .map_err(CommandError::Command)
}

/// Removes one entry's record; never the file.
#[tauri::command]
pub fn remove_saved_package_entry(app: AppHandle, project_id: String, entry_id: String) -> Result<SavedPackageRemoval, CommandError> {
    crate::app_store_path(&app)
        .and_then(|path| remove_entry_in_store(&path, &project_id, &entry_id))
        .map_err(CommandError::Command)
}

#[cfg(test)]
#[path = "saved_package_entries_tests.rs"]
pub(crate) mod tests;
