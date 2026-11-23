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
//! meets an existing entry. A save whose time could not be taken keeps no
//! entry ([`ENTRY_NO_SAVE_TIME`]).
//!
//! The project's report list is its entries in the order they were kept
//! (`sequence`). Two package saves can overlap (each runs on its own blocking
//! worker, from one window or several); each keeps its entry after its own
//! writer returns, in one `BEGIN IMMEDIATE` transaction, so `sequence` records
//! the order in which the entries were kept, which can differ from the order
//! in which the files were published (the writer synchronizes the directory
//! after its rename). Held entries are kept in the same order: the session's
//! lock is held across each keep.
//!
//! The location belongs to this machine (REQ-023 "This machine"). It is read
//! only by the state check, here, and leaves this module in no reply, error,
//! diagnostic or log line: [`SavedPackageEntry`] has no location member, and
//! every reply and reason built here is a code with a fixed sentence; no text
//! of an I/O or SQL error is passed on. An entry is named by its `entry_id`, a
//! random token allocated when it is made.
//!
//! The store keeps them as REQ-039 says: they are read back unchanged and in
//! order, removed with the project (the foreign key cascades), and adding or
//! removing one changes no other record. Removing an entry removes the record
//! and never the file. Nothing here reads or writes a portable project
//! package, and an autosave recovery record holds no entry.
//!
//! **Before the first project save** (REQ-023). An entry made while the store
//! does not hold the project is held by the native session
//! ([`SessionHeldEntries`]) under its project id and a holder token, which the
//! session mints at the first held save of a document session and returns in
//! the receipt's `saved_package_entry.holder`. A later package save presents
//! the token to join the same holder; an unknown token is not joined (a new
//! one is minted), and a save presenting another project's live token keeps
//! no entry. Listing, checking and removing a held entry need its
//! token; without it a caller sees stored entries only, so a second document
//! with the same project id neither sees nor inherits the first one's held
//! entries. No project save writes held entries: the page calls
//! [`keep_held_saved_package_entries`] after its own successful first save of
//! the project, with the project id and its token, and that writes that
//! holder's entries, in order, in one transaction, and retires the token. A
//! package save that presents a token the session holds is held with that
//! holder's other entries, whether or not the store holds the project (another
//! document of the same id may have created it), so only the page's call
//! writes them. An entry that cannot be written stays held with its reason,
//! and the command can be called again.
//!
//! Held entries live in this process's memory only: no recovery record, file
//! or store holds them, so they end with the desktop and nothing reconstructs
//! them. A page reload keeps them, but the page then has the token only if it
//! kept it; without it they stay unreachable (not listed, checked, removed or
//! written) until the desktop ends.
//!
//! The commands are admitted through the store's argument ingress with a
//! capped raw body and meter what they read and reply
//! ([`crate::stored_read`]), as the run and state commands do. The held
//! entries a list or a write copies, and the write's reply, are reserved under
//! the session's lock before they are copied, and the write's before it opens
//! its transaction, so no refusal for memory follows its commit.
//!
//! Not built here (deferred by Agent 0): matching an entry's basis across the
//! project's recorded runs; the embedded agent's read.

use crate::command_ingress::{self, ArgsDecoding, CommandError};
use crate::model_states::{now_unix_millis, random_token, utc_iso_millis};
use crate::solve_jobs::SolveJobRegistry;
use crate::stored_read::{self, Metered, PoolReadMemory, ReadMemory};
use rusqlite::{params, Connection, OptionalExtension};
use serde::Serialize;
use std::io::Read;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager};

/// Store v15: the entries' table and its immutability triggers. Every
/// statement is `IF NOT EXISTS`, so the step is safe to re-run; it runs in one
/// transaction with its `user_version` update (`apply_store_migrations`).
/// `saved_package_entries_immutable` refuses any `UPDATE`;
/// `saved_package_entries_no_replace` refuses an `INSERT` that meets an
/// existing entry of the project on either key (`entry_id` or `sequence`),
/// before conflict resolution, so `INSERT OR REPLACE` cannot overwrite one.
/// The table has no rowid (`WITHOUT ROWID`), so no statement can name a row
/// by an implicit key the triggers do not see. Deleting the project row
/// cascades to its entries.
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
  FOREIGN KEY(project_id) REFERENCES local_projects(project_id) ON DELETE CASCADE) WITHOUT ROWID;
CREATE TRIGGER IF NOT EXISTS saved_package_entries_immutable BEFORE UPDATE ON saved_package_entries
  BEGIN SELECT RAISE(ABORT, 'SAVED-PACKAGE-ENTRY-IMMUTABLE'); END;
CREATE TRIGGER IF NOT EXISTS saved_package_entries_no_replace BEFORE INSERT ON saved_package_entries
  WHEN EXISTS (SELECT 1 FROM saved_package_entries WHERE project_id = NEW.project_id
    AND (entry_id = NEW.entry_id OR sequence = NEW.sequence))
  BEGIN SELECT RAISE(ABORT, 'SAVED-PACKAGE-ENTRY-IMMUTABLE'); END;
";

/// A failed v15 step, by a code and a fixed sentence: SQLite's own text is
/// not passed on (its transaction is rolled back whole).
pub(crate) const MIGRATION_FAILED: &str = "SAVED-PACKAGE-ENTRIES-MIGRATION-FAILED: the saved-package entries table could not be created.";

pub(crate) fn migrate_v15_saved_package_entries(connection: &Connection) -> Result<(), String> {
    connection.execute_batch(STORE_V15_SAVED_PACKAGE_ENTRIES_SQL).map_err(|_| MIGRATION_FAILED.to_string())
}

pub(crate) const ENTRY_KEPT: &str = "SAVED-PACKAGE-ENTRY-KEPT";
/// No package was published (a cancelled chooser), so no entry was added.
pub(crate) const ENTRY_NOT_APPLICABLE: &str = "SAVED-PACKAGE-ENTRY-NOT-APPLICABLE";
/// The store does not hold the project the entries are for.
pub(crate) const ENTRY_PROJECT_NOT_SAVED: &str = "SAVED-PACKAGE-ENTRY-PROJECT-NOT-SAVED";
/// The save was given no run record, so the entry's basis is incomplete.
pub(crate) const ENTRY_NO_RUN_RECORD: &str = "SAVED-PACKAGE-ENTRY-NO-RUN-RECORD";
/// The save was given a run record that did not verify against the package's
/// source result, input manifest and producer.
pub(crate) const ENTRY_RUN_RECORD_UNVERIFIED: &str = "SAVED-PACKAGE-ENTRY-RUN-RECORD-UNVERIFIED";
/// The save was given a run record that could not be checked within the
/// memory available to the save; the package was saved as without it.
pub(crate) const ENTRY_RUN_RECORD_NOT_CHECKED: &str = "SAVED-PACKAGE-ENTRY-RUN-RECORD-NOT-CHECKED";
/// The time of the save could not be taken, so no entry was made.
pub(crate) const ENTRY_NO_SAVE_TIME: &str = "SAVED-PACKAGE-ENTRY-NO-SAVE-TIME";
/// The session holds the entry under its holder token (REQ-023 "Before the
/// first project save").
pub(crate) const ENTRY_HELD_BY_SESSION: &str = "SAVED-PACKAGE-ENTRY-HELD-BY-SESSION";
/// Keeping the entry failed, or could not be confirmed.
pub(crate) const ENTRY_NOT_KEPT: &str = "SAVED-PACKAGE-ENTRY-NOT-KEPT";
/// A command named an entry the caller cannot reach (none of the project's
/// stored entries, nor one of its holder's).
pub(crate) const ENTRY_UNKNOWN: &str = "SAVED-PACKAGE-ENTRY-UNKNOWN";
/// The project store could not be found, opened, read or written.
pub(crate) const ENTRY_STORE_FAILED: &str = "SAVED-PACKAGE-ENTRY-STORE-FAILED";

/// The most raw argument bytes a saved-package entry command takes: its ids
/// and token are small by contract.
pub(crate) const ENTRY_COMMAND_MAX_ARGS_BYTES: usize = crate::model_states::HISTORY_COMMAND_MAX_ARGS_BYTES;

const STORE_FAILED_SENTENCE: &str = "The project store could not be found, opened, read or written.";
const PROJECT_NOT_STORED_SENTENCE: &str = "The project store does not hold the project, so its held entries were not written.";

/// A problem a reply states: a code and a fixed sentence, never an error's
/// own text.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct EntryProblem {
    pub(crate) code: &'static str,
    pub(crate) message: &'static str,
}

const STORE_FAILED: EntryProblem = EntryProblem { code: ENTRY_STORE_FAILED, message: STORE_FAILED_SENTENCE };
/// The keep command's memory was refused before it wrote: nothing was
/// written, and the entries can be written again.
const MEMORY_REFUSED: EntryProblem = EntryProblem {
    code: stored_read::STORE_READ_MEMORY_UNAVAILABLE,
    message: "The held entries could not be written within the memory available; nothing was written, and they can be written again.",
};
const PROJECT_NOT_STORED: EntryProblem = EntryProblem { code: ENTRY_PROJECT_NOT_SAVED, message: PROJECT_NOT_STORED_SENTENCE };

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
    /// The save was given a run record that did not verify.
    Unverified,
    /// The save was given a run record that could not be checked within its
    /// memory (its extra lease, or a reservation of its check, was refused).
    NotChecked,
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
    /// UTC ISO-8601 with milliseconds, taken when the writer returned; empty
    /// if the clock could not be read.
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
/// outcome (REQ-023 "Entry"). `status` is `kept`, `held`, `not_kept` or
/// `not_applicable`; only `kept` says an entry was kept, and names it. A
/// `held` outcome names the entry and the holder token it is held under.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct SavedPackageEntryOutcome {
    pub status: String,
    pub code: String,
    pub entry_id: String,
    pub message: String,
    /// The holder token of a held entry; absent otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder: Option<String>,
}

impl SavedPackageEntryOutcome {
    fn kept(entry_id: String) -> Self {
        Self {
            status: "kept".to_string(),
            code: ENTRY_KEPT.to_string(),
            entry_id,
            message: "The saved package is listed in the project.".to_string(),
            holder: None,
        }
    }

    pub(crate) fn not_kept(code: &str, message: impl Into<String>) -> Self {
        Self { status: "not_kept".to_string(), code: code.to_string(), entry_id: String::new(), message: message.into(), holder: None }
    }

    fn held(entry_id: String, holder: String, message: &str) -> Self {
        Self { status: "held".to_string(), code: ENTRY_HELD_BY_SESSION.to_string(), entry_id, message: message.to_string(), holder: Some(holder) }
    }

    /// A held entry that the keep command could not write, with `problem`.
    fn still_held(entry_id: String, holder: &str, problem: EntryProblem) -> Self {
        Self { status: "held".to_string(), code: problem.code.to_string(), entry_id, message: problem.message.to_string(), holder: Some(holder.to_string()) }
    }

    pub(crate) fn not_applicable() -> Self {
        Self {
            status: "not_applicable".to_string(),
            code: ENTRY_NOT_APPLICABLE.to_string(),
            entry_id: String::new(),
            message: "No package was saved, so no entry was added.".to_string(),
            holder: None,
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
    /// `kept_with_project`, or `held_by_session` for an entry the session
    /// holds (its `sequence` is 0 until it is written and allocated one).
    pub(crate) kept: &'static str,
    /// For a held entry whose writing failed: why. It stays held.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) write_failure: Option<EntryProblem>,
}

/// The list reply: the project's stored entries in the order kept, then the
/// entries the session holds for the presented holder, in the order kept.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct SavedPackageList {
    pub(crate) project_id: String,
    pub(crate) entries: Vec<SavedPackageEntry>,
    /// Present when the store could not be read: the entries are then the
    /// held ones only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) store_failure: Option<EntryProblem>,
}

/// What the state check found at an entry's location (REQ-023 "State").
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct SavedPackageState {
    pub(crate) entry_id: String,
    /// `present_identical`, `changed`, `not_found` or `could_not_be_read`.
    pub(crate) state: &'static str,
    /// Why, for every state but `present_identical`: a fixed sentence.
    pub(crate) reason: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct SavedPackageRemoval {
    pub(crate) entry_id: String,
    /// The record was removed; the file was not touched.
    pub(crate) removed: bool,
    pub(crate) message: &'static str,
}

/// The reply of [`keep_held_saved_package_entries`]: one outcome per held
/// entry of the holder, in the order kept.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct HeldEntriesWrite {
    pub(crate) project_id: String,
    pub(crate) entries: Vec<SavedPackageEntryOutcome>,
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
fn entry_basis(published: &PublishedPackage) -> Result<&SavedPackageBasis, SavedPackageEntryOutcome> {
    if published.saved_at.is_empty() {
        return Err(SavedPackageEntryOutcome::not_kept(
            ENTRY_NO_SAVE_TIME,
            "The package was saved, but it is not listed in the project: the time of the save could not be taken.",
        ));
    }
    match &published.basis {
        EntryBasis::Bound(basis) => Ok(basis),
        EntryBasis::NoRunRecord => Err(SavedPackageEntryOutcome::not_kept(
            ENTRY_NO_RUN_RECORD,
            "The package was saved, but it is not listed in the project: the save was given no analysis run record, so the entry's basis is incomplete.",
        )),
        EntryBasis::Unverified => Err(SavedPackageEntryOutcome::not_kept(
            ENTRY_RUN_RECORD_UNVERIFIED,
            "The package was saved, but it is not listed in the project: the analysis run record given with it does not verify against the package's result, input manifest and producer.",
        )),
        EntryBasis::NotChecked => Err(SavedPackageEntryOutcome::not_kept(
            ENTRY_RUN_RECORD_NOT_CHECKED,
            "The package was saved, but it is not listed in the project: the analysis run record given with it could not be checked within the memory available.",
        )),
    }
}

/// Whether the store holds `project_id`, on `transaction`.
fn project_held(transaction: &rusqlite::Transaction<'_>, project_id: &str) -> rusqlite::Result<bool> {
    let held: Option<i64> =
        transaction.query_row("SELECT 1 FROM local_projects WHERE project_id = ?1", params![project_id], |row| row.get(0)).optional()?;
    Ok(held.is_some())
}

/// Why an entry could not be inserted: the store's refusal, or a member this
/// machine cannot record.
enum InsertFailure {
    Store,
    Unrecordable,
}

impl From<rusqlite::Error> for InsertFailure {
    fn from(_: rusqlite::Error) -> Self {
        Self::Store
    }
}

/// Inserts one entry with `entry_id` at the project's next sequence, on
/// `transaction`.
fn insert_entry(transaction: &rusqlite::Transaction<'_>, entry_id: &str, published: &PublishedPackage, basis: &SavedPackageBasis) -> Result<(), InsertFailure> {
    let location = location_bytes(&published.location).ok_or(InsertFailure::Unrecordable)?;
    let byte_count = i64::try_from(published.container_byte_count).map_err(|_| InsertFailure::Unrecordable)?;
    let sequence: i64 = transaction.query_row(
        "SELECT COALESCE(MAX(sequence), 0) + 1 FROM saved_package_entries WHERE project_id = ?1",
        params![published.project_id],
        |row| row.get(0),
    )?;
    transaction.execute(
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
    )?;
    Ok(())
}

/// What writing entries into the store did.
enum Kept {
    Written,
    /// The store does not hold the project; nothing was written.
    ProjectNotStored,
}

/// Writing entries failed; nothing was written. The store's own text is not
/// kept.
#[derive(Debug)]
struct WriteFailed;

/// One `BEGIN IMMEDIATE` transaction: if the store holds the project, the
/// entries are inserted in their order and committed; otherwise nothing is
/// written. Any failure, the commit's included, is [`WriteFailed`], and the
/// caller claims nothing was kept.
fn write_entries(path: &Path, project_id: &str, entries: &[(&str, &PublishedPackage, &SavedPackageBasis)]) -> Result<Kept, WriteFailed> {
    let (mut connection, _migration) = crate::open_project_store(path).map_err(|_| WriteFailed)?;
    let transaction = connection.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate).map_err(|_| WriteFailed)?;
    if !project_held(&transaction, project_id).map_err(|_| WriteFailed)? {
        return Ok(Kept::ProjectNotStored);
    }
    for (entry_id, published, basis) in entries {
        insert_entry(&transaction, entry_id, published, basis).map_err(|_| WriteFailed)?;
    }
    #[cfg(test)]
    tests::fault::before_commit().map_err(|_| WriteFailed)?;
    transaction.commit().map_err(|_| WriteFailed)?;
    Ok(Kept::Written)
}

fn fresh_entry_id() -> Option<String> {
    random_token().ok().map(|token| format!("saved-package:{token}"))
}

fn fresh_holder() -> Option<String> {
    random_token().ok().map(|token| format!("saved-package-holder:{token}"))
}

fn not_kept_failure() -> SavedPackageEntryOutcome {
    SavedPackageEntryOutcome::not_kept(
        ENTRY_NOT_KEPT,
        "The package was saved, but its entry could not be kept in the project, or keeping it could not be confirmed.",
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
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn keep_entry_in_store(path: &Path, published: &PublishedPackage) -> SavedPackageEntryOutcome {
    let basis = match entry_basis(published) {
        Ok(basis) => basis,
        Err(outcome) => return outcome,
    };
    let Some(entry_id) = fresh_entry_id() else {
        return not_kept_failure();
    };
    match write_entries(path, &published.project_id, &[(&entry_id, published, basis)]) {
        Ok(Kept::Written) => SavedPackageEntryOutcome::kept(entry_id),
        Ok(Kept::ProjectNotStored) => SavedPackageEntryOutcome::not_kept(
            ENTRY_PROJECT_NOT_SAVED,
            "The package was saved, but it is not listed in the project: the project is not yet saved.",
        ),
        Err(WriteFailed) => not_kept_failure(),
    }
}

// ---------------------------------------------------------------------------
// The session's held entries (REQ-023 "Before the first project save").
// ---------------------------------------------------------------------------

/// One entry the session holds: made from a published package while the
/// store did not hold its project, under its holder's token.
#[derive(Clone, Debug)]
struct HeldEntry {
    holder: String,
    entry_id: String,
    published: PublishedPackage,
    basis: SavedPackageBasis,
    write_failure: Option<EntryProblem>,
}

impl HeldEntry {
    fn of(&self, project_id: &str, holder: &str) -> bool {
        self.published.project_id == project_id && self.holder == holder
    }
}

#[derive(Default)]
struct Held {
    /// In the order kept.
    entries: Vec<HeldEntry>,
    /// Every (project id, token) the session has minted.
    holders: Vec<(String, String)>,
}

impl Held {
    fn minted(&self, project_id: &str, holder: &str) -> bool {
        self.holders.iter().any(|(project, token)| project == project_id && token == holder)
    }

    fn of(&self, project_id: &str, holder: &str) -> Vec<HeldEntry> {
        self.entries.iter().filter(|entry| entry.of(project_id, holder)).cloned().collect()
    }

    fn mark(&mut self, project_id: &str, holder: &str, problem: EntryProblem) {
        for entry in self.entries.iter_mut().filter(|entry| entry.of(project_id, holder)) {
            entry.write_failure = Some(problem);
        }
    }

    fn release(&mut self, project_id: &str, holder: &str) {
        self.entries.retain(|entry| !entry.of(project_id, holder));
    }

    /// The holder's entries were written: its token is retired, so a later
    /// save presenting it is kept as one presenting none.
    fn retire(&mut self, project_id: &str, holder: &str) {
        self.holders.retain(|(project, token)| !(project == project_id && token == holder));
    }

    /// How many entries the holder holds, and their texts' bytes.
    fn measure(&self, project_id: &str, holder: &str) -> (u64, u64) {
        self.entries
            .iter()
            .filter(|entry| entry.of(project_id, holder))
            .fold((0, 0), |(count, bytes), entry| (count + 1, bytes.saturating_add(held_text_bytes(entry))))
    }
}

/// The bytes of a held entry's texts: its own, its published package's and
/// its basis's (the basis is held twice: in the entry and in the package).
/// The file name counts three bytes for each of its own, the most a lossy
/// conversion makes of one.
fn held_text_bytes(entry: &HeldEntry) -> u64 {
    let published = &entry.published;
    let basis = |basis: &SavedPackageBasis| -> u64 {
        [&basis.model_hash, &basis.received_result_sha256, &basis.run_id, &basis.run_record_sha256].iter().map(|text| text.len() as u64).sum()
    };
    let package_basis = match &published.basis {
        EntryBasis::Bound(bound) => basis(bound),
        _ => 0,
    };
    let texts: u64 = [&entry.holder, &entry.entry_id, &published.project_id, &published.package_identity_sha256, &published.container_sha256, &published.saved_at, &published.export_context]
        .iter()
        .map(|text| text.len() as u64)
        .sum();
    let file_name = published.location.file_name().map_or(0, |name| name.len() as u64).saturating_mul(3);
    stored_read::total(&[texts, published.location.as_os_str().len() as u64, file_name, basis(&entry.basis), package_basis])
}

/// The entries the native session holds, in the order kept, each under its
/// project id and holder token, until the page writes them after its first
/// save of the project ([`keep_held_entries`]) or a package save presenting
/// the token writes them into a project the store holds. They live in this
/// process's memory only and end with the desktop.
#[derive(Default)]
pub struct SessionHeldEntries {
    held: std::sync::Mutex<Held>,
}

impl SessionHeldEntries {
    fn lock(&self) -> std::sync::MutexGuard<'_, Held> {
        self.held.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn find(&self, project_id: &str, holder: Option<&str>, entry_id: &str) -> Option<HeldEntry> {
        let holder = holder?;
        self.lock().entries.iter().find(|entry| entry.of(project_id, holder) && entry.entry_id == entry_id).cloned()
    }

    /// The holder's entries as listed, reserved in `memory` under the lock
    /// before any is copied.
    fn views(&self, project_id: &str, holder: Option<&str>, memory: &dyn ReadMemory) -> Result<Metered<Vec<SavedPackageEntry>>, String> {
        let Some(holder) = holder else {
            return Ok(Metered::new(Vec::new()));
        };
        let held = self.lock();
        let (count, text_bytes) = held.measure(project_id, holder);
        let reservation = memory.reserve("the session's held saved-package entries", listed_bytes(count, text_bytes))?;
        let views = held.entries.iter().filter(|entry| entry.of(project_id, holder)).map(held_entry_view).collect();
        Ok(Metered::new(views).holding(reservation))
    }

    fn remove(&self, project_id: &str, holder: Option<&str>, entry_id: &str) -> bool {
        let Some(holder) = holder else {
            return false;
        };
        let mut held = self.lock();
        let before = held.entries.len();
        held.entries.retain(|entry| !(entry.of(project_id, holder) && entry.entry_id == entry_id));
        held.entries.len() != before
    }
}

const HELD_BEFORE_FIRST_SAVE: &str =
    "The package was saved. Its entry is not yet kept with the project: the session holds it until the project is first saved, and it is lost if the desktop ends before then.";
const HELD_WITH_EARLIER: &str =
    "The package was saved. Its entry is held by the session with this document's earlier entries, which could not yet be written to the project; they are written together when the page writes its held entries.";

/// Keeps a published package's entry in the store at `path`
/// ([`keep_or_hold_in`]).
pub(crate) fn keep_or_hold(path: &Path, session: &SessionHeldEntries, holder: Option<&str>, published: &PublishedPackage) -> SavedPackageEntryOutcome {
    keep_or_hold_in(|| Ok(path.to_path_buf()), session, holder, published)
}

/// Keeps a published package's entry (REQ-023 "Entry", "Before the first
/// project save"). `holder` is the token the page presents, if any. The
/// session's live tokens are read, under its lock, before the store is resolved:
///
/// - With a live token the session minted for this project, the entry is
///   held under it with the holder's other entries, whether or not the store
///   holds the project or can be found: another document of the same id may
///   have created it, and only the page's call after its own first save
///   writes the holder's entries ([`keep_held_entries`]). Nothing is written
///   here.
/// - With a live token minted for another project, the entry is `not_kept`;
///   nothing is written and that project's held entries are untouched.
/// - Otherwise (no token, or one retired or unknown), a store that cannot be
///   found is `not_kept`; if the store holds the project, the entry is
///   written and kept; if it does not, the session holds the entry under a
///   token minted now, which the outcome returns.
pub(crate) fn keep_or_hold_in(
    store: impl FnOnce() -> Result<PathBuf, String>,
    session: &SessionHeldEntries,
    holder: Option<&str>,
    published: &PublishedPackage,
) -> SavedPackageEntryOutcome {
    let basis = match entry_basis(published) {
        Ok(basis) => basis.clone(),
        Err(outcome) => return outcome,
    };
    let Some(entry_id) = fresh_entry_id() else {
        return not_kept_failure();
    };
    let project_id = published.project_id.as_str();
    let mut held = session.lock();
    if let Some(token) = holder.filter(|token| held.minted(project_id, token)).map(str::to_string) {
        let earlier_unwritten = held.entries.iter().any(|entry| entry.of(project_id, &token) && entry.write_failure.is_some());
        held.entries.push(HeldEntry { holder: token.clone(), entry_id: entry_id.clone(), published: published.clone(), basis, write_failure: None });
        let message = if earlier_unwritten { HELD_WITH_EARLIER } else { HELD_BEFORE_FIRST_SAVE };
        return SavedPackageEntryOutcome::held(entry_id, token, message);
    }
    if holder.is_some_and(|token| held.holders.iter().any(|(_, live)| live == token)) {
        return not_kept_failure();
    }
    let Ok(path) = store() else {
        return not_kept_failure();
    };
    match write_entries(&path, project_id, &[(&entry_id, published, &basis)]) {
        Ok(Kept::Written) => SavedPackageEntryOutcome::kept(entry_id),
        Ok(Kept::ProjectNotStored) => {
            let Some(token) = fresh_holder() else {
                return not_kept_failure();
            };
            held.holders.push((project_id.to_string(), token.clone()));
            held.entries.push(HeldEntry { holder: token.clone(), entry_id: entry_id.clone(), published: published.clone(), basis, write_failure: None });
            SavedPackageEntryOutcome::held(entry_id, token, HELD_BEFORE_FIRST_SAVE)
        }
        Err(WriteFailed) => not_kept_failure(),
    }
}

/// The bytes the keep command's reply can serialize to for the holder's
/// entries: its frame, and for each entry its id in the longest of the
/// outcomes it can have (kept, or still held with either problem).
fn keep_reply_bytes(held: &Held, project_id: &str, holder: &str) -> Result<u64, String> {
    let outcomes = [
        SavedPackageEntryOutcome::kept(String::new()),
        SavedPackageEntryOutcome::still_held(String::new(), holder, STORE_FAILED),
        SavedPackageEntryOutcome::still_held(String::new(), holder, PROJECT_NOT_STORED),
    ];
    let mut outcome = 0;
    for candidate in &outcomes {
        // Less the empty id's two quotes, which the entry's own id replaces.
        outcome = outcome.max(stored_read::serialized_length(candidate)?.saturating_sub(2));
    }
    let mut bytes = stored_read::serialized_length(&HeldEntriesWrite { project_id: project_id.to_string(), entries: Vec::new() })?;
    for entry in held.entries.iter().filter(|entry| entry.of(project_id, holder)) {
        // The outcome, its id and the comma before the next.
        bytes = stored_read::total(&[bytes, outcome, stored_read::serialized_length(&entry.entry_id)?, 1]);
    }
    Ok(bytes)
}

/// The keep command's working space for `count` entries with `text_bytes` of
/// texts and a reply of `reply_bytes`: the copies of the entries and their
/// texts, the rows naming them, the outcomes (whose texts the reply bounds),
/// and the store's connection and statements.
fn keep_work_bytes(count: u64, text_bytes: u64, reply_bytes: u64) -> u64 {
    let items = (std::mem::size_of::<HeldEntry>() + std::mem::size_of::<SavedPackageEntryOutcome>() + std::mem::size_of::<(&str, &PublishedPackage, &SavedPackageBasis)>()) as u64;
    stored_read::total(&[
        stored_read::string_list_bytes(count.saturating_mul(20), text_bytes.saturating_add(reply_bytes)),
        count.saturating_mul(items).saturating_mul(2),
        stored_read::READ_OVERHEAD_BYTES,
    ])
}

/// Writes the holder's held entries for `project_id` into the store, in the
/// order kept, in one transaction, and answers for each. Its working space
/// and its reply are reserved in `memory` under the session's lock before
/// any entry is copied or the transaction is opened: a refusal writes
/// nothing, leaves every entry held and the token live, and marks the
/// entries with [`MEMORY_REFUSED`] (a fixed code and sentence, no
/// allocation), so a later save presenting the token says its earlier
/// entries are not yet written. Written entries leave the session
/// once the commit returns, and the holder's token is retired; if the store
/// cannot be found or written, or does not hold the project, they stay held,
/// each with that reason, and this can be called again. Nothing is written
/// for another project or another holder. The reply holds its reservations
/// ([`respond_reserved`]).
pub(crate) fn keep_held_entries(
    store: Result<PathBuf, String>,
    session: &SessionHeldEntries,
    project_id: &str,
    holder: &str,
    memory: &dyn ReadMemory,
) -> Result<Metered<HeldEntriesWrite>, String> {
    let mut held = session.lock();
    let (count, text_bytes) = held.measure(project_id, holder);
    let reply_bytes = keep_reply_bytes(&held, project_id, holder)?;
    let reserved = memory
        .reserve("writing the session's held saved-package entries", keep_work_bytes(count, text_bytes, reply_bytes))
        .and_then(|work| Ok((work, memory.reserve("the response to keep_held_saved_package_entries", stored_read::exact_text_bytes(reply_bytes))?)));
    let (work, reply) = match reserved {
        Ok(reserved) => reserved,
        Err(error) => {
            held.mark(project_id, holder, MEMORY_REFUSED);
            return Err(error);
        }
    };
    let pending = held.of(project_id, holder);
    let written = if pending.is_empty() {
        Ok(Kept::Written)
    } else {
        let rows: Vec<(&str, &PublishedPackage, &SavedPackageBasis)> =
            pending.iter().map(|entry| (entry.entry_id.as_str(), &entry.published, &entry.basis)).collect();
        match store {
            Ok(path) => write_entries(&path, project_id, &rows).map_err(|_| STORE_FAILED),
            Err(_) => Err(STORE_FAILED),
        }
    };
    let problem = match written {
        Ok(Kept::Written) => None,
        Ok(Kept::ProjectNotStored) => Some(PROJECT_NOT_STORED),
        Err(problem) => Some(problem),
    };
    let entries = pending
        .iter()
        .map(|entry| match problem {
            None => SavedPackageEntryOutcome::kept(entry.entry_id.clone()),
            Some(problem) => SavedPackageEntryOutcome::still_held(entry.entry_id.clone(), holder, problem),
        })
        .collect();
    match problem {
        None => {
            held.release(project_id, holder);
            held.retire(project_id, holder);
        }
        Some(problem) => held.mark(project_id, holder, problem),
    }
    Ok(Metered::new(HeldEntriesWrite { project_id: project_id.to_string(), entries }).holding(work).holding(reply))
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
        write_failure: entry.write_failure,
    }
}

/// The bytes one listed entry holds besides its texts.
const ENTRY_VIEW_BYTES: u64 = std::mem::size_of::<SavedPackageEntry>() as u64;
/// The listed entry's text members.
const ENTRY_TEXT_COLUMNS: u64 = 10;

/// What a list of `count` entries with `text_bytes` of texts holds: the
/// strings ([`stored_read::string_list_bytes`]) and the entries in a growing
/// vector.
fn listed_bytes(count: u64, text_bytes: u64) -> u64 {
    stored_read::total(&[stored_read::string_list_bytes(count.saturating_mul(ENTRY_TEXT_COLUMNS), text_bytes), count.saturating_mul(ENTRY_VIEW_BYTES).saturating_mul(2)])
}

const ENTRY_COLUMNS: &str = "entry_id, sequence, package_identity_sha256, container_sha256, container_byte_count, file_name,
  saved_at, export_context, durability_confirmed, model_hash, received_result_sha256, run_id, run_record_sha256";

const ENTRY_TEXT_LENGTH: &str = "octet_length(entry_id) + octet_length(package_identity_sha256) + octet_length(container_sha256)
  + octet_length(file_name) + octet_length(saved_at) + octet_length(export_context) + octet_length(model_hash)
  + octet_length(received_result_sha256) + octet_length(run_id) + octet_length(run_record_sha256)";

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

/// A store failure as a command reply: the code and its fixed sentence.
fn store_failed() -> String {
    format!("{ENTRY_STORE_FAILED}: {STORE_FAILED_SENTENCE}")
}

/// Why a metered store read stopped: the memory pool's refusal (its own
/// bounded text, which names no location), or the store's failure.
enum ReadStop {
    Memory(String),
    Store,
}

fn memory_or_store(error: String) -> ReadStop {
    if error.starts_with(stored_read::STORE_READ_MEMORY_UNAVAILABLE) { ReadStop::Memory(error) } else { ReadStop::Store }
}

/// The largest stored keyed row of the entries' table, in bytes, measured
/// without SQLite loading a value, as
/// [`stored_read::admit_sqlite_working_set`] measures the history tables,
/// which do not include the entries: a search of the table may materialize
/// another project's key.
const ENTRY_KEYED_ROW_SQL: &str =
    "SELECT COALESCE(MAX(COALESCE(octet_length(project_id), 0) + COALESCE(octet_length(entry_id), 0)), 0) FROM saved_package_entries NOT INDEXED";

/// Reserves SQLite's working set for the entries' keys on `connection`, its
/// transaction, before a statement of it searches the table.
fn admit_entry_keys(connection: &Connection, memory: &dyn ReadMemory) -> Result<stored_read::ReadReservation, String> {
    let largest: i64 = connection.query_row(ENTRY_KEYED_ROW_SQL, [], |row| row.get(0)).map_err(|error| stored_read::sql_error(&error))?;
    stored_read::reserve_sqlite_value(memory, &format!("SQLite's working set for the saved-package entries (their largest keyed row holds {largest} bytes)"), largest)
}

/// The project's stored entries, unchanged and in the order kept, read in
/// one read transaction and reserved in `memory` before they are loaded.
fn stored_entries(path: &Path, project_id: &str, memory: &dyn ReadMemory) -> Result<Metered<Vec<SavedPackageEntry>>, ReadStop> {
    let mut connection = stored_read::open_store(path).map_err(|_| ReadStop::Store)?;
    let transaction = stored_read::read_transaction(&mut connection).map_err(|_| ReadStop::Store)?;
    let sqlite = stored_read::admit_sqlite_working_set(&transaction, memory).map_err(memory_or_store)?;
    let keys = admit_entry_keys(&transaction, memory).map_err(memory_or_store)?;
    let (count, text_bytes): (i64, i64) = transaction
        .query_row(
            &format!("SELECT COUNT(*), COALESCE(SUM({ENTRY_TEXT_LENGTH}), 0) FROM saved_package_entries WHERE project_id = ?1"),
            params![project_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|_| ReadStop::Store)?;
    let reservation = memory
        .reserve("the project's saved-package entries", listed_bytes(u64::try_from(count).unwrap_or(0), u64::try_from(text_bytes).unwrap_or(0)))
        .map_err(memory_or_store)?;
    let mut statement = transaction
        .prepare(&format!("SELECT {ENTRY_COLUMNS} FROM saved_package_entries WHERE project_id = ?1 ORDER BY sequence"))
        .map_err(|_| ReadStop::Store)?;
    let entries = statement
        .query_map(params![project_id], entry_of)
        .map_err(|_| ReadStop::Store)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ReadStop::Store)?;
    drop(statement);
    drop(transaction);
    Ok(Metered::new(entries).holding(reservation).holding(keys).holding(sqlite))
}

/// The project's report list (REQ-023; R5 of review round 1): the held
/// entries of `holder` are read first (reserved before they are copied),
/// then the store, so an entry being
/// written while the list runs is seen held or stored, never neither (a write
/// holds the session's lock from before its commit until the entries leave
/// the session). An entry seen both ways is listed once, as stored. Stored
/// entries come first, in the order kept, then the held ones. If the store
/// cannot be read, the held entries are listed with the store's failure.
pub(crate) fn list_entries(
    store: Result<PathBuf, String>,
    session: &SessionHeldEntries,
    project_id: &str,
    holder: Option<&str>,
    memory: &dyn ReadMemory,
) -> Result<Metered<SavedPackageList>, String> {
    let held = session.views(project_id, holder, memory)?;
    #[cfg(test)]
    tests::hooks::between_list_reads();
    let stored = match store {
        Ok(path) => stored_entries(&path, project_id, memory),
        Err(_) => Err(ReadStop::Store),
    };
    let (stored, store_failure) = match stored {
        Ok(stored) => (stored, None),
        Err(ReadStop::Memory(error)) => return Err(error),
        Err(ReadStop::Store) => (Metered::new(Vec::new()), Some(STORE_FAILED)),
    };
    Ok(stored.join(held, |mut entries, held| {
        let stored_ids: std::collections::BTreeSet<String> = entries.iter().map(|entry| entry.entry_id.clone()).collect();
        entries.extend(held.into_iter().filter(|entry| !stored_ids.contains(&entry.entry_id)));
        SavedPackageList { project_id: project_id.to_string(), entries, store_failure }
    }))
}

/// The location, SHA-256 and byte count of one stored entry, read only for
/// the state check; the location's length is reserved before it is loaded.
fn entry_location(path: &Path, project_id: &str, entry_id: &str, memory: &dyn ReadMemory) -> Result<(PathBuf, String, u64, stored_read::ReadReservation), String> {
    let mut connection = stored_read::open_store(path).map_err(|_| store_failed())?;
    let transaction = stored_read::read_transaction(&mut connection).map_err(|_| store_failed())?;
    let stopped = |stop: ReadStop| match stop {
        ReadStop::Memory(error) => error,
        ReadStop::Store => store_failed(),
    };
    let _sqlite = stored_read::admit_sqlite_working_set(&transaction, memory).map_err(|error| stopped(memory_or_store(error)))?;
    let _keys = admit_entry_keys(&transaction, memory).map_err(|error| stopped(memory_or_store(error)))?;
    let length: Option<i64> = transaction
        .query_row(
            "SELECT octet_length(location) + octet_length(container_sha256) FROM saved_package_entries WHERE project_id = ?1 AND entry_id = ?2",
            params![project_id, entry_id],
            |row| row.get(0),
        )
        .optional()
        .map_err(|_| store_failed())?;
    let length = length.ok_or_else(|| format!("{ENTRY_UNKNOWN}: the project holds no such entry that this caller can reach."))?;
    let reservation = memory
        .reserve("a saved-package entry's location", stored_read::loaded_text_bytes(u64::try_from(length).unwrap_or(0)))
        .map_err(|error| stopped(memory_or_store(error)))?;
    let (bytes, sha256, byte_count): (Vec<u8>, String, i64) = transaction
        .query_row(
            "SELECT location, container_sha256, container_byte_count FROM saved_package_entries WHERE project_id = ?1 AND entry_id = ?2",
            params![project_id, entry_id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .map_err(|_| store_failed())?;
    let location = location_path(bytes).ok_or_else(store_failed)?;
    Ok((location, sha256, u64::try_from(byte_count).unwrap_or(0), reservation))
}

/// The most the state check reads at a time.
const STATE_READ_CHUNK_BYTES: usize = 64 * 1024;

const NOT_FOUND: &str = "No file is at the saved location.";
const NOT_REGULAR: &str = "The saved location is not a regular file.";
const PERMISSION_DENIED: &str = "Permission to read the file at the saved location was denied.";
const NOT_READABLE: &str = "The file at the saved location could not be read.";
const OTHER_SIZE: &str = "The file at the saved location is not the saved package: its size differs.";
const OTHER_BYTES: &str = "The file at the saved location is not the saved package: its SHA-256 differs.";

fn unreadable(error: &std::io::Error) -> (&'static str, &'static str) {
    match error.kind() {
        std::io::ErrorKind::NotFound => ("not_found", NOT_FOUND),
        std::io::ErrorKind::PermissionDenied => ("could_not_be_read", PERMISSION_DENIED),
        _ => ("could_not_be_read", NOT_READABLE),
    }
}

/// Opens `location` for reading without blocking on a special file: on Unix
/// with `O_NONBLOCK`, which a regular file's reads ignore and which keeps the
/// open of a pipe from waiting for a writer.
fn open_for_check(location: &Path) -> std::io::Result<std::fs::File> {
    let mut options = std::fs::OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NONBLOCK);
    }
    options.open(location)
}

/// The state found at `location` for a file of `expected_len` bytes whose
/// SHA-256 should be `expected` (REQ-023 "State"). It opens the location as a
/// path, as reopening reads it, so a symbolic link there answers for the file
/// it reaches (a link to the saved bytes is present and identical, to other
/// bytes changed, and a dangling one not found); what was opened must be a
/// regular file on the opened descriptor, so nothing swapped in after a check
/// of the path (a pipe, a device, a directory) is read or waited on. A file
/// of another length is `changed` without being read; otherwise at most
/// `expected_len + 1` bytes are read, in chunks of
/// [`STATE_READ_CHUNK_BYTES`] reserved in `memory`. It writes nothing. Only
/// the container's SHA-256 is compared: the manifest hash is checked when the
/// package is reopened, on the bytes handed to the reader.
pub(crate) fn state_at(location: &Path, expected: &str, expected_len: u64, memory: &dyn ReadMemory) -> Result<(&'static str, &'static str), String> {
    let mut file = match open_for_check(location) {
        Ok(file) => file,
        Err(error) => return Ok(unreadable(&error)),
    };
    let metadata = match file.metadata() {
        Ok(metadata) => metadata,
        Err(error) => return Ok(unreadable(&error)),
    };
    if !metadata.is_file() {
        return Ok(("could_not_be_read", NOT_REGULAR));
    }
    if metadata.len() != expected_len {
        return Ok(("changed", OTHER_SIZE));
    }
    let _buffer_reservation = memory.reserve("the state check's read buffer", STATE_READ_CHUNK_BYTES as u64)?;
    let mut buffer = vec![0u8; STATE_READ_CHUNK_BYTES];
    let mut hasher = <sha2::Sha256 as sha2::Digest>::new();
    let mut remaining = expected_len.saturating_add(1);
    let mut read_total = 0u64;
    while remaining > 0 {
        let want = usize::try_from(remaining.min(STATE_READ_CHUNK_BYTES as u64)).unwrap_or(STATE_READ_CHUNK_BYTES);
        match file.read(&mut buffer[..want]) {
            Ok(0) => break,
            Ok(read) => {
                sha2::Digest::update(&mut hasher, &buffer[..read]);
                read_total = read_total.saturating_add(read as u64);
                remaining = remaining.saturating_sub(read as u64);
            }
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(error) => return Ok(unreadable(&error)),
        }
    }
    if read_total != expected_len {
        return Ok(("changed", OTHER_SIZE));
    }
    let found: String = sha2::Digest::finalize(hasher).iter().map(|byte| format!("{byte:02x}")).collect();
    if found == expected {
        Ok(("present_identical", ""))
    } else {
        Ok(("changed", OTHER_BYTES))
    }
}

/// The state check of one entry, held by the presented holder or stored.
pub(crate) fn check_entry(
    store: Result<PathBuf, String>,
    session: &SessionHeldEntries,
    project_id: &str,
    holder: Option<&str>,
    entry_id: &str,
    memory: &dyn ReadMemory,
) -> Result<Metered<SavedPackageState>, String> {
    let (location, expected, expected_len, reservation) = match session.find(project_id, holder, entry_id) {
        Some(held) => {
            let reservation = memory.reserve("a held entry's location", stored_read::loaded_text_bytes(held.published.location.as_os_str().len() as u64))?;
            (held.published.location, held.published.container_sha256, held.published.container_byte_count, reservation)
        }
        None => entry_location(&store.map_err(|_| store_failed())?, project_id, entry_id, memory)?,
    };
    let (state, reason) = state_at(&location, &expected, expected_len, memory)?;
    Ok(Metered::new(SavedPackageState { entry_id: entry_id.to_string(), state, reason }).holding(reservation))
}

const REMOVED: &str = "The entry was removed from the project's report list. The saved file was not touched.";

/// Removes one entry's record, held by the presented holder or stored; never
/// the file.
pub(crate) fn remove_entry(store: Result<PathBuf, String>, session: &SessionHeldEntries, project_id: &str, holder: Option<&str>, entry_id: &str) -> Result<SavedPackageRemoval, String> {
    if session.remove(project_id, holder, entry_id) {
        return Ok(SavedPackageRemoval { entry_id: entry_id.to_string(), removed: true, message: REMOVED });
    }
    let path = store.map_err(|_| store_failed())?;
    let connection = stored_read::open_store(&path).map_err(|_| store_failed())?;
    let removed = connection
        .execute("DELETE FROM saved_package_entries WHERE project_id = ?1 AND entry_id = ?2", params![project_id, entry_id])
        .map_err(|_| store_failed())?;
    if removed == 0 {
        return Err(format!("{ENTRY_UNKNOWN}: the project holds no such entry that this caller can reach."));
    }
    Ok(SavedPackageRemoval { entry_id: entry_id.to_string(), removed: true, message: REMOVED })
}

/// The time of a save, taken when its writer returns; empty if the clock
/// cannot be read, which keeps no entry ([`ENTRY_NO_SAVE_TIME`]).
pub(crate) fn save_time() -> String {
    now_unix_millis().map(utc_iso_millis).unwrap_or_default()
}

// ---------------------------------------------------------------------------
// The commands.
// ---------------------------------------------------------------------------

/// `{projectId, holder?}`.
pub(crate) fn list_saved_package_entries_args(
    args: &mut command_ingress::Args,
    store: impl FnOnce() -> Result<PathBuf, String>,
    session: &SessionHeldEntries,
    memory: &dyn ReadMemory,
) -> Result<Metered<SavedPackageList>, CommandError> {
    let project_id: String = args.decode("projectId")?;
    let holder: Option<String> = args.decode("holder")?;
    list_entries(store(), session, &project_id, holder.as_deref(), memory).map_err(|error| CommandError::Command(stored_read::capped(error)))
}

/// `{projectId, entryId, holder?}`.
pub(crate) fn check_saved_package_entry_args(
    args: &mut command_ingress::Args,
    store: impl FnOnce() -> Result<PathBuf, String>,
    session: &SessionHeldEntries,
    memory: &dyn ReadMemory,
) -> Result<Metered<SavedPackageState>, CommandError> {
    let project_id: String = args.decode("projectId")?;
    let entry_id: String = args.decode("entryId")?;
    let holder: Option<String> = args.decode("holder")?;
    check_entry(store(), session, &project_id, holder.as_deref(), &entry_id, memory).map_err(|error| CommandError::Command(stored_read::capped(error)))
}

/// `{projectId, entryId, holder?}`.
pub(crate) fn remove_saved_package_entry_args(
    args: &mut command_ingress::Args,
    store: impl FnOnce() -> Result<PathBuf, String>,
    session: &SessionHeldEntries,
) -> Result<Metered<SavedPackageRemoval>, CommandError> {
    let project_id: String = args.decode("projectId")?;
    let entry_id: String = args.decode("entryId")?;
    let holder: Option<String> = args.decode("holder")?;
    remove_entry(store(), session, &project_id, holder.as_deref(), &entry_id)
        .map(Metered::new)
        .map_err(|error| CommandError::Command(stored_read::capped(error)))
}

const KEEP_HELD_INCOMPLETE: &str = "SAVED-PACKAGE-ENTRY-STORE-FAILED: writing the held entries did not complete; list the project's entries with the holder to see which are kept.";

/// `{projectId, holder}`, guarded as the package save's keeper is: a panic
/// while writing is a fixed reply, never a lost one.
pub(crate) fn keep_held_saved_package_entries_args(
    args: &mut command_ingress::Args,
    store: impl FnOnce() -> Result<PathBuf, String>,
    session: &SessionHeldEntries,
    memory: &dyn ReadMemory,
) -> Result<Metered<HeldEntriesWrite>, CommandError> {
    let project_id: String = args.decode("projectId")?;
    let holder: String = args.decode("holder")?;
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| keep_held_entries(store(), session, &project_id, &holder, memory)))
        .map_err(|_| CommandError::Command(KEEP_HELD_INCOMPLETE.to_string()))?
        .map_err(|error| CommandError::Command(stored_read::capped(error)))
}

/// The keep command's reply, serialized within the reservation
/// [`keep_held_entries`] made for it before it wrote (the pricing of
/// [`stored_read::respond_metered`]: a buffer of the reply's exact length), so
/// nothing after the commit is refused for memory.
pub(crate) fn respond_reserved(outcome: Result<Metered<HeldEntriesWrite>, CommandError>) -> Result<tauri::ipc::Response, CommandError> {
    let metered = outcome?;
    let length = stored_read::serialized_length(metered.get()).map_err(CommandError::Response)?;
    let length = usize::try_from(length).map_err(|_| CommandError::Response("STORE-READ: a length does not fit this platform".to_string()))?;
    let mut buffer = Vec::with_capacity(length);
    serde_json::to_writer(&mut buffer, metered.get()).map_err(|error| CommandError::Response(stored_read::json_error(&error)))?;
    let text = String::from_utf8(buffer).map_err(|_| CommandError::Response("STORE-READ: a serialized text is not UTF-8".to_string()))?;
    let response = tauri::ipc::Response::new(text);
    drop(metered);
    Ok(response)
}

/// The project's report list: its stored entries in the order kept, without
/// their locations, then the presented holder's held entries, marked as held.
#[tauri::command]
pub fn list_saved_package_entries(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
    session: tauri::State<'_, SessionHeldEntries>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, lease) =
        command_ingress::admit_within(&registry, &invoke, "list_saved_package_entries", ArgsDecoding::Moved, ENTRY_COMMAND_MAX_ARGS_BYTES)?;
    let memory = PoolReadMemory::new(&registry, &lease);
    stored_read::respond_metered(
        &memory,
        "list_saved_package_entries",
        list_saved_package_entries_args(&mut args, || crate::app_store_path(&app), &session, &memory),
    )
}

/// What is found now at one entry's location; reads the file, writes nothing.
/// The read runs on a blocking worker, off the async executor.
#[tauri::command]
pub async fn check_saved_package_entry(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, lease) =
        command_ingress::admit_within(&registry, &invoke, "check_saved_package_entry", ArgsDecoding::Moved, ENTRY_COMMAND_MAX_ARGS_BYTES)?;
    let registry = registry.inner().clone();
    tauri::async_runtime::spawn_blocking(move || {
        let session = app.state::<SessionHeldEntries>();
        let memory = PoolReadMemory::new(&registry, &lease);
        let outcome = check_saved_package_entry_args(&mut args, || crate::app_store_path(&app), &session, &memory);
        stored_read::respond_metered(&memory, "check_saved_package_entry", outcome)
    })
    .await
    .unwrap_or_else(|_| Err(CommandError::Command(format!("{ENTRY_STORE_FAILED}: the state check did not complete."))))
}

/// Removes one entry's record; never the file.
#[tauri::command]
pub fn remove_saved_package_entry(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
    session: tauri::State<'_, SessionHeldEntries>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, lease) =
        command_ingress::admit_within(&registry, &invoke, "remove_saved_package_entry", ArgsDecoding::Moved, ENTRY_COMMAND_MAX_ARGS_BYTES)?;
    let memory = PoolReadMemory::new(&registry, &lease);
    stored_read::respond_metered(&memory, "remove_saved_package_entry", remove_saved_package_entry_args(&mut args, || crate::app_store_path(&app), &session))
}

/// Writes one holder's held entries into the project, which the page calls
/// after its own successful first save of the project; answers per entry.
#[tauri::command]
pub fn keep_held_saved_package_entries(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
    session: tauri::State<'_, SessionHeldEntries>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, lease) =
        command_ingress::admit_within(&registry, &invoke, "keep_held_saved_package_entries", ArgsDecoding::Moved, ENTRY_COMMAND_MAX_ARGS_BYTES)?;
    let memory = PoolReadMemory::new(&registry, &lease);
    respond_reserved(keep_held_saved_package_entries_args(&mut args, || crate::app_store_path(&app), &session, &memory))
}

#[cfg(test)]
#[path = "saved_package_entries_tests.rs"]
pub(crate) mod tests;
