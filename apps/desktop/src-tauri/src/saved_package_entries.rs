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
//! An entry made while the store does not hold the project is held by the
//! native session ([`SessionHeldEntries`]), listed as held, and written when
//! the project is first saved ([`write_held_entries_after_save`], after each
//! project save command). Nothing persists it before then: the autosave
//! recovery record holds only the model document, so an entry still held when
//! the desktop ends is lost and recovery does not reconstruct it.
//!
//! Not built here (deferred by Agent 0): matching an entry's basis across the
//! project's recorded runs; the embedded agent's read.

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
/// The store does not yet hold the project: the session holds the entry until
/// the project's first save writes it (REQ-023 "Before the first project save").
pub(crate) const ENTRY_HELD_BY_SESSION: &str = "SAVED-PACKAGE-ENTRY-HELD-BY-SESSION";
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

    fn held(entry_id: String, message: impl Into<String>) -> Self {
        Self { status: "held".to_string(), code: ENTRY_HELD_BY_SESSION.to_string(), entry_id, message: message.into() }
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
    /// `kept_with_project`, or `held_by_session` for an entry made while the
    /// store did not hold the project (its `sequence` is 0 until the first
    /// save writes it and allocates one).
    pub(crate) kept: &'static str,
    /// For a held entry whose writing at a project save failed: why. It stays
    /// held and is written at the next project save or package save.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) write_failure: Option<String>,
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

/// The basis of a published package that an entry can be made of, or the
/// outcome that says why none can.
fn bound_basis(published: &PublishedPackage) -> Result<&SavedPackageBasis, SavedPackageEntryOutcome> {
    match &published.basis {
        EntryBasis::Bound(basis) => Ok(basis),
        EntryBasis::NoRunRecord => Err(SavedPackageEntryOutcome::not_kept(
            ENTRY_NO_RUN_RECORD,
            "The package was saved, but it is not listed in the project: the save was given no analysis run record, so the entry's basis is incomplete.",
        )),
        EntryBasis::Unverified(reason) => Err(SavedPackageEntryOutcome::not_kept(
            ENTRY_RUN_RECORD_UNVERIFIED,
            format!(
                "The package was saved, but it is not listed in the project: the analysis run record given with it did not verify ({}).",
                withheld(reason, &published.location)
            ),
        )),
    }
}

/// Whether the store holds `project_id`, on `transaction`.
fn project_held(transaction: &rusqlite::Transaction<'_>, project_id: &str) -> Result<bool, String> {
    let held: Option<i64> = transaction
        .query_row("SELECT 1 FROM local_projects WHERE project_id = ?1", params![project_id], |row| row.get(0))
        .optional()
        .map_err(|error| error.to_string())?;
    Ok(held.is_some())
}

/// Inserts one entry with `entry_id` at the project's next sequence, on
/// `transaction`.
fn insert_entry(transaction: &rusqlite::Transaction<'_>, entry_id: &str, published: &PublishedPackage, basis: &SavedPackageBasis) -> Result<(), String> {
    let location = location_bytes(&published.location).ok_or_else(|| "the saved location cannot be recorded on this machine".to_string())?;
    let byte_count = i64::try_from(published.container_byte_count).map_err(|_| "the package's byte count is out of range".to_string())?;
    let sequence: i64 = transaction
        .query_row(
            "SELECT COALESCE(MAX(sequence), 0) + 1 FROM saved_package_entries WHERE project_id = ?1",
            params![published.project_id],
            |row| row.get(0),
        )
        .map_err(|error| error.to_string())?;
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
    Ok(())
}

/// What writing entries into the store did.
enum Kept {
    Written,
    /// The store does not hold the project; nothing was written.
    ProjectNotStored,
}

/// One `BEGIN IMMEDIATE` transaction: if the store holds the project, the
/// entries are inserted in their order and committed; otherwise nothing is
/// written. Any failure, the commit's included, is returned, and the caller
/// claims nothing was kept.
fn write_entries(path: &Path, project_id: &str, entries: &[(&str, &PublishedPackage, &SavedPackageBasis)]) -> Result<Kept, String> {
    let (mut connection, _migration) = crate::open_project_store(path)?;
    let transaction = connection
        .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
        .map_err(|error| error.to_string())?;
    if !project_held(&transaction, project_id)? {
        return Ok(Kept::ProjectNotStored);
    }
    for (entry_id, published, basis) in entries {
        insert_entry(&transaction, entry_id, published, basis)?;
    }
    #[cfg(test)]
    tests::fault::before_commit()?;
    transaction.commit().map_err(|error| error.to_string())?;
    Ok(Kept::Written)
}

fn fresh_entry_id() -> Result<String, String> {
    Ok(format!("saved-package:{}", random_token()?))
}

fn not_kept_failure(error: &str, location: &Path) -> SavedPackageEntryOutcome {
    SavedPackageEntryOutcome::not_kept(
        ENTRY_NOT_KEPT,
        format!(
            "The package was saved, but its entry could not be kept in the project, or keeping it could not be confirmed: {}",
            withheld(error, location)
        ),
    )
}

/// Keeps a published package's entry in the store at `path`, with no session
/// to hold it: one transaction that checks the store holds the project,
/// allocates the next sequence and a fresh entry id, inserts the row and
/// commits. A project the store does not hold is `not_kept`
/// ([`ENTRY_PROJECT_NOT_SAVED`]); the command keeps through
/// [`keep_or_hold`], which holds such an entry instead. Any failure, the
/// commit's included, is `not_kept`: nothing claims the entry was kept unless
/// the commit returned. The package is never touched here.
pub(crate) fn keep_entry_in_store(path: &Path, published: &PublishedPackage) -> SavedPackageEntryOutcome {
    let basis = match bound_basis(published) {
        Ok(basis) => basis,
        Err(outcome) => return outcome,
    };
    let outcome = fresh_entry_id().and_then(|entry_id| write_entries(path, &published.project_id, &[(&entry_id, published, basis)]).map(|kept| (entry_id, kept)));
    match outcome {
        Ok((entry_id, Kept::Written)) => SavedPackageEntryOutcome::kept(entry_id),
        Ok((_, Kept::ProjectNotStored)) => SavedPackageEntryOutcome::not_kept(
            ENTRY_PROJECT_NOT_SAVED,
            "The package was saved, but it is not listed in the project: the project is not yet saved.",
        ),
        Err(error) => not_kept_failure(&error, &published.location),
    }
}

// ---------------------------------------------------------------------------
// The session's held entries (REQ-023 "Before the first project save").
// ---------------------------------------------------------------------------

/// One entry the session holds: made from a published package while the
/// store did not hold its project.
#[derive(Clone, Debug)]
struct HeldEntry {
    entry_id: String,
    published: PublishedPackage,
    basis: SavedPackageBasis,
    write_failure: Option<String>,
}

/// The entries the native session holds, in the order saved, until their
/// project's first save to the store writes them. They live in this process's
/// memory only: no recovery record, file or store holds them, so they end
/// with the desktop and nothing reconstructs them. Opening or creating
/// another project, or reloading the web view, keeps them: they stay held for
/// their own project until it is saved to the store, or the desktop ends.
#[derive(Default)]
pub struct SessionHeldEntries {
    held: std::sync::Mutex<Vec<HeldEntry>>,
}

impl SessionHeldEntries {
    fn lock(&self) -> std::sync::MutexGuard<'_, Vec<HeldEntry>> {
        self.held.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn find(&self, project_id: &str, entry_id: &str) -> Option<HeldEntry> {
        self.lock().iter().find(|held| held.published.project_id == project_id && held.entry_id == entry_id).cloned()
    }

    fn of_project(&self, project_id: &str) -> Vec<SavedPackageEntry> {
        self.lock().iter().filter(|held| held.published.project_id == project_id).map(held_entry_view).collect()
    }

    fn remove(&self, project_id: &str, entry_id: &str) -> bool {
        let mut held = self.lock();
        let before = held.len();
        held.retain(|entry| !(entry.published.project_id == project_id && entry.entry_id == entry_id));
        held.len() != before
    }
}

/// Writes `project_id`'s held entries into the store at `path`, in their
/// order, in one transaction, and removes them from the session once the
/// commit returns. If the store does not hold the project they stay held; if
/// writing fails they stay held, each with the failure's reason.
fn write_held_for(path: &Path, project_id: &str, held: &mut Vec<HeldEntry>) {
    let pending: Vec<HeldEntry> = held.iter().filter(|entry| entry.published.project_id == project_id).cloned().collect();
    if pending.is_empty() {
        return;
    }
    let rows: Vec<(&str, &PublishedPackage, &SavedPackageBasis)> =
        pending.iter().map(|entry| (entry.entry_id.as_str(), &entry.published, &entry.basis)).collect();
    match write_entries(path, project_id, &rows) {
        Ok(Kept::Written) => held.retain(|entry| entry.published.project_id != project_id),
        Ok(Kept::ProjectNotStored) => {}
        Err(error) => {
            for entry in held.iter_mut().filter(|entry| entry.published.project_id == project_id) {
                entry.write_failure = Some(format!("writing it at the project's save failed: {}", withheld(&error, &entry.published.location)));
            }
        }
    }
}

/// Keeps a published package's entry (REQ-023 "Entry", "Before the first
/// project save"). Entries the session already holds for the project are
/// written first, so the list keeps the order saved. If the store holds the
/// project and no earlier entry stays held, the entry is kept; if the store
/// does not hold the project, or an earlier held entry could not be written,
/// the session holds it ([`ENTRY_HELD_BY_SESSION`]).
pub(crate) fn keep_or_hold(path: &Path, session: &SessionHeldEntries, published: &PublishedPackage) -> SavedPackageEntryOutcome {
    let basis = match bound_basis(published) {
        Ok(basis) => basis.clone(),
        Err(outcome) => return outcome,
    };
    let entry_id = match fresh_entry_id() {
        Ok(entry_id) => entry_id,
        Err(error) => return not_kept_failure(&error, &published.location),
    };
    let mut held = session.lock();
    write_held_for(path, &published.project_id, &mut held);
    let earlier_held = held.iter().any(|entry| entry.published.project_id == published.project_id);
    if !earlier_held {
        match write_entries(path, &published.project_id, &[(&entry_id, published, &basis)]) {
            Ok(Kept::Written) => return SavedPackageEntryOutcome::kept(entry_id),
            Ok(Kept::ProjectNotStored) => {}
            Err(error) => return not_kept_failure(&error, &published.location),
        }
    }
    held.push(HeldEntry { entry_id: entry_id.clone(), published: published.clone(), basis, write_failure: None });
    SavedPackageEntryOutcome::held(
        entry_id,
        "The package was saved. Its entry is not yet kept with the project: the session holds it until the project is first saved, and it is lost if the desktop ends before then.",
    )
}

/// [`keep_or_hold`] for the application's store; a store path that cannot be
/// found is `not_kept`.
pub(crate) fn keep_or_hold_in(store: Result<PathBuf, String>, session: &SessionHeldEntries, published: &PublishedPackage) -> SavedPackageEntryOutcome {
    match store {
        Ok(path) => keep_or_hold(&path, session, published),
        Err(error) => SavedPackageEntryOutcome::not_kept(
            ENTRY_NOT_KEPT,
            format!("The package was saved, but its entry could not be kept in the project: {}", withheld(&error, &published.location)),
        ),
    }
}

/// After a project save: writes every held entry whose project the store now
/// holds, each project's in one transaction. The project save's own outcome
/// is not changed by this: an entry that cannot be written stays held, with
/// its reason in the list, and is written at the next project or package save.
pub(crate) fn write_held_entries(path: &Path, session: &SessionHeldEntries) {
    let mut held = session.lock();
    let mut projects: Vec<String> = Vec::new();
    for entry in held.iter() {
        if !projects.contains(&entry.published.project_id) {
            projects.push(entry.published.project_id.clone());
        }
    }
    for project_id in projects {
        write_held_for(path, &project_id, &mut held);
    }
}

/// [`write_held_entries`] for the application, after a project save command
/// has its outcome; a session that holds nothing opens no store.
pub(crate) fn write_held_entries_after_save(app: &AppHandle) {
    use tauri::Manager;
    let session = app.state::<SessionHeldEntries>();
    if session.lock().is_empty() {
        return;
    }
    if let Ok(path) = crate::app_store_path(app) {
        write_held_entries(&path, &session);
    }
}

fn held_entry_view(entry: &HeldEntry) -> SavedPackageEntry {
    let published = &entry.published;
    SavedPackageEntry {
        entry_id: entry.entry_id.clone(),
        sequence: 0,
        package_identity_sha256: published.package_identity_sha256.clone(),
        container_sha256: published.container_sha256.clone(),
        container_byte_count: i64::try_from(published.container_byte_count).unwrap_or(i64::MAX),
        file_name: published.file_name(),
        saved_at: published.saved_at.clone(),
        export_context: published.export_context.clone(),
        durability_confirmed: published.durability_confirmed,
        basis: entry.basis.clone(),
        kept: "held_by_session",
        write_failure: entry.write_failure.clone(),
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
        write_failure: None,
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

/// The project's report list: its stored entries in the order saved, then the
/// entries the session holds for it, in the order saved, marked
/// `held_by_session`.
pub(crate) fn list_entries_with(path: &Path, session: &SessionHeldEntries, project_id: &str) -> Result<SavedPackageList, String> {
    let mut list = list_entries_in_store(path, project_id)?;
    list.entries.extend(session.of_project(project_id));
    Ok(list)
}

/// The state check of one entry, held by the session or stored.
pub(crate) fn check_entry_with(path: &Path, session: &SessionHeldEntries, project_id: &str, entry_id: &str) -> Result<SavedPackageState, String> {
    match session.find(project_id, entry_id) {
        Some(held) => {
            let location = &held.published.location;
            let (state, reason) = state_at(location, &held.published.container_sha256);
            Ok(SavedPackageState { entry_id: entry_id.to_string(), state, reason: withheld(&reason, location) })
        }
        None => check_entry_in_store(path, project_id, entry_id),
    }
}

/// Removes one entry's record, held by the session or stored; never the file.
pub(crate) fn remove_entry_with(path: &Path, session: &SessionHeldEntries, project_id: &str, entry_id: &str) -> Result<SavedPackageRemoval, String> {
    if session.remove(project_id, entry_id) {
        return Ok(SavedPackageRemoval {
            entry_id: entry_id.to_string(),
            removed: true,
            message: "The entry was removed from the project's report list. The saved file was not touched.".to_string(),
        });
    }
    remove_entry_in_store(path, project_id, entry_id)
}

/// The time of a save, taken when its writer returns.
pub(crate) fn save_time() -> String {
    now_unix_millis().map(utc_iso_millis).unwrap_or_default()
}

/// The project's report list: its entries in the order saved, without their
/// locations, the session's held entries last and marked as held.
#[tauri::command]
pub fn list_saved_package_entries(app: AppHandle, session: tauri::State<'_, SessionHeldEntries>, project_id: String) -> Result<SavedPackageList, CommandError> {
    crate::app_store_path(&app)
        .and_then(|path| list_entries_with(&path, &session, &project_id))
        .map_err(CommandError::Command)
}

/// What is found now at one entry's location; reads the file, writes nothing.
#[tauri::command]
pub fn check_saved_package_entry(
    app: AppHandle,
    session: tauri::State<'_, SessionHeldEntries>,
    project_id: String,
    entry_id: String,
) -> Result<SavedPackageState, CommandError> {
    crate::app_store_path(&app)
        .and_then(|path| check_entry_with(&path, &session, &project_id, &entry_id))
        .map_err(CommandError::Command)
}

/// Removes one entry's record; never the file.
#[tauri::command]
pub fn remove_saved_package_entry(
    app: AppHandle,
    session: tauri::State<'_, SessionHeldEntries>,
    project_id: String,
    entry_id: String,
) -> Result<SavedPackageRemoval, CommandError> {
    crate::app_store_path(&app)
        .and_then(|path| remove_entry_with(&path, &session, &project_id, &entry_id))
        .map_err(CommandError::Command)
}

#[cfg(test)]
#[path = "saved_package_entries_tests.rs"]
pub(crate) mod tests;
