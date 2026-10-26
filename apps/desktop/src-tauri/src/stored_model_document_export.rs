//! Recovery export of a stored model document (W-17, DEL-02-02 Design
//! `project-unit-selection.md` §3).
//!
//! A stored project or saved state whose document the desktop refuses at open
//! (`MODEL-FILE-SHAPE` at `project.units`) is written to a file the user
//! chooses, exactly as the store holds it, so the user can add the map and open
//! the file with Open Model Document…. Nothing is adopted into a session and
//! nothing is computed from the document for one.
//!
//! - The bytes written are the stored text itself: a project's
//!   `local_projects.model_json` (the compact JSON the store serialized when it
//!   saved the project, not necessarily RFC 8785 canonical) or a saved state's
//!   `model_states.model_canonical` (its RFC 8785 text, whose SHA-256 is the
//!   state's model hash).
//! - The store is opened read-only (`SQLITE_OPEN_READ_ONLY`, as the legacy
//!   store's carry-forward opens it) and is never migrated: a store at another
//!   schema version than this build's is refused (`MODEL-DOC-EXPORT-STORE-NOT-CURRENT`).
//! - The arguments (`{projectId, stateId, explicitLocalPrivateIntent}`) arrive
//!   as an `args-json-v1` raw body admitted as the history commands' are
//!   (`command_ingress::admit_within`, [`MAX_ARGS_BYTES`]): a longer body is
//!   refused `INGRESS-ARGS-TOO-LARGE` before it is scanned or decoded, and a
//!   JSON (typed) body is refused. The ingress lease starts the read's
//!   per-invocation accounting (`PoolReadMemory::new`), and the command's own
//!   memory (`stored_read::READ_OVERHEAD_BYTES`) is reserved with it.
//! - The read is metered as the saved-state reader's is (`stored_read`, W-12):
//!   in one read transaction, SQLite's working set is reserved, the row's
//!   length is measured without loading it, a row above the model-document
//!   file cap is refused (`MODEL-DOC-SAVE-TOO-LARGE`, the save's refusal: Open
//!   Model Document refuses such a file), and its load, its parse and the
//!   strict parse are each reserved before they are made. Each library the
//!   screening resolves is read in the same transaction under its own
//!   reservations ([`LibraryReads`]): SQLite's working set for the library
//!   keys, then the document's measured load and its parse, held through the
//!   screening. A reservation the memory pool refuses is
//!   `STORE-READ-MEMORY-UNAVAILABLE`. The text's reservation is held through
//!   the chooser and the write.
//! - The open checks (version, document validator, desktop shape) are not
//!   applied: the document is refused at open for one of them, and the export
//!   exists to recover it.
//! - The save path's privacy controls still apply: the explicit local-private
//!   intent (`MODEL-DOC-SAVE-INTENT-REQUIRED` without it) and the native
//!   disposition screening of the exact document written
//!   (`model_document_save::screen_model_document_dispositions`: a protected,
//!   quarantined or rejected disposition refuses `PROTECTED_CONTENT_BLOCKED`).
//!   The text written is the text screened: a text with a duplicate object
//!   member anywhere is refused (`MODEL-DOC-EXPORT-DUPLICATE-MEMBER`) before
//!   screening, since the parsed document keeps only the last of them.
//! - The chooser and the atomic `.json` writer are the Save Model Document
//!   ones (`lib.rs` `choose_model_document_save_path`,
//!   `model_document_save::write_admitted_model_document`).

use crate::command_ingress::Args;
use crate::export_disposition_screen::diagnostic;
use crate::memory_pool::Lease;
use crate::model_document_file::MAX_MODEL_DOCUMENT_BYTES;
use crate::model_document_save::{
    screen_model_document_dispositions, suggested_model_document_file_name, write_admitted_model_document,
    AdmittedModelDocument, ModelDocumentSaveDiagnostic, ModelDocumentSaveOutcome, PICKER_PATH_INVALID,
};
pub use crate::model_document_save::TOO_LARGE;
use crate::solve_jobs::{SolveJobError, SolveJobRegistry};
use crate::stored_read::{self, PoolReadMemory, ReadMemory, ReadReservation};
pub(crate) use crate::stored_read::STORE_READ_MEMORY_UNAVAILABLE as MEMORY_UNAVAILABLE;
use rusqlite::{params, Connection, OpenFlags, OptionalExtension};
use serde_json::Value;
use std::path::{Path, PathBuf};

pub const INTENT_REQUIRED: &str = "MODEL-DOC-SAVE-INTENT-REQUIRED";
pub const NOT_FOUND: &str = "MODEL-DOC-EXPORT-NOT-FOUND";
pub const STORE_UNAVAILABLE: &str = "MODEL-DOC-EXPORT-STORE-UNAVAILABLE";
pub const STORE_NOT_CURRENT: &str = "MODEL-DOC-EXPORT-STORE-NOT-CURRENT";
pub const NOT_JSON: &str = "MODEL-DOC-EXPORT-NOT-JSON";
pub const DUPLICATE_MEMBER: &str = "MODEL-DOC-EXPORT-DUPLICATE-MEMBER";

/// The command's name at ingress.
pub(crate) const COMMAND: &str = "save_stored_model_document_file";
/// The command's ingress budget (raw argument bytes): the history commands'
/// (`model_states::HISTORY_COMMAND_MAX_ARGS_BYTES`), since it names a stored
/// project and state by the same ids they do. A longer body is refused
/// `INGRESS-ARGS-TOO-LARGE` before any lease or decoding.
pub(crate) const MAX_ARGS_BYTES: usize = crate::model_states::HISTORY_COMMAND_MAX_ARGS_BYTES;

/// The command's decoded arguments.
#[derive(Debug)]
pub(crate) struct StoredExportRequest {
    pub(crate) project_id: String,
    pub(crate) state_id: Option<String>,
    pub(crate) explicit_local_private_intent: bool,
}

/// `{projectId, stateId?, explicitLocalPrivateIntent}`, moved out of the
/// admitted arguments.
pub(crate) fn stored_export_args(args: &mut Args) -> Result<StoredExportRequest, SolveJobError> {
    Ok(StoredExportRequest {
        project_id: args.decode("projectId")?,
        state_id: args.decode("stateId")?,
        explicit_local_private_intent: args.decode("explicitLocalPrivateIntent")?,
    })
}

type Refusal = Vec<ModelDocumentSaveDiagnostic>;

fn refusal(code: &str, message: impl Into<String>) -> Refusal {
    vec![diagnostic(code, "", message)]
}

/// What is exported, for the refusals (ids cut as a message quotes them).
pub fn stored_document_name(project_id: &str, state_id: Option<&str>) -> String {
    let project_id = stored_read::shown(project_id);
    match state_id {
        None => format!("project `{project_id}`"),
        Some(state_id) => format!("saved state `{}` of project `{project_id}`", stored_read::shown(state_id)),
    }
}

/// A refusal of the read: the memory pool's by its name, any other as the
/// store's.
fn read_refusal(named: &str, error: String) -> Refusal {
    if error.starts_with(MEMORY_UNAVAILABLE) {
        return refusal(MEMORY_UNAVAILABLE, format!("{error} Nothing was written."));
    }
    refusal(STORE_UNAVAILABLE, format!("The local project store could not be read for {named} ({}); nothing was written.", stored_read::capped(error)))
}

/// Opens the store read-only, as it is: nothing is created, migrated or
/// written. A store at another schema version than this build's is refused by
/// name (an older one needs the migration a normal open applies).
pub(crate) fn open_store_read_only(path: &Path, named: &str) -> Result<Connection, Refusal> {
    let unavailable = |error: String| read_refusal(named, error);
    let connection = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX)
        .map_err(|error| unavailable(error.to_string()))?;
    connection.busy_timeout(std::time::Duration::from_secs(5)).map_err(|error| unavailable(error.to_string()))?;
    let version = crate::store_user_version(&connection).map_err(unavailable)?;
    crate::refuse_newer_store(version).map_err(|message| refusal(STORE_NOT_CURRENT, message))?;
    if version < crate::STORE_SCHEMA_TARGET_VERSION {
        return Err(refusal(
            STORE_NOT_CURRENT,
            format!(
                "The local project store is at store schema v{version}, older than this build's v{}. The export opens the store read-only and does not migrate it; nothing was read or written. Open a local project first (which migrates the store), then save the stored document again.",
                crate::STORE_SCHEMA_TARGET_VERSION
            ),
        ));
    }
    Ok(connection)
}

/// The stored text's column, measured (`octet_length`) or loaded.
fn stored_text_sql(state_id: Option<&str>, expression: &str) -> String {
    match state_id {
        None => format!("SELECT {expression}(model_json) FROM local_projects WHERE project_id = ?1"),
        Some(_) => format!("SELECT {expression}(model_canonical) FROM model_states WHERE project_id = ?1 AND state_id = ?2"),
    }
}

fn query_stored<T: rusqlite::types::FromSql>(
    connection: &Connection,
    sql: &str,
    project_id: &str,
    state_id: Option<&str>,
) -> rusqlite::Result<Option<T>> {
    match state_id {
        None => connection.query_row(sql, params![project_id], |row| row.get(0)),
        Some(state_id) => connection.query_row(sql, params![project_id, state_id], |row| row.get(0)),
    }
    .optional()
}

/// Refuses a text that holds an object member more than once, anywhere: the
/// parsed document keeps only the last of them, so the document screened
/// would not be the bytes written. The strict reader is the checked
/// canonicalizer's duplicate-refusing parse (`canonical_json_checked_v1_text`),
/// run with its memory reserved by `stored_read::checked_sha256_text` (the
/// stored-history reader's use of it). The profile's value refusals (an unsafe
/// integer, for example) come only after that parse completed, so such a text
/// has no duplicate member and is not refused for them.
fn refuse_duplicate_members(memory: &dyn ReadMemory, what: &str, named: &str, text: &str, document: &Value) -> Result<(), Refusal> {
    const VALUE_REFUSALS: [&str; 3] = ["CHECKED-JSON-UNSAFE-", "CHECKED-JSON-NON-FINITE", "CHECKED-JSON-NUMBER-INVALID"];
    match stored_read::checked_sha256_text(memory, what, text, document) {
        Ok(_) => Ok(()),
        Err(error) if VALUE_REFUSALS.iter().any(|prefix| error.starts_with(prefix)) => Ok(()),
        Err(error) if error.starts_with(MEMORY_UNAVAILABLE) => Err(read_refusal(named, error)),
        Err(error) if error.contains("duplicate object member") => Err(refusal(
            DUPLICATE_MEMBER,
            format!("The stored document of {named} holds an object member more than once ({error}); its screening would not cover the bytes written, so nothing was written."),
        )),
        Err(error) => Err(refusal(NOT_JSON, format!("The stored document of {named} is not strict JSON ({error}); nothing was written."))),
    }
}

/// The largest stored keyed row of `local_libraries`, in bytes, measured
/// without SQLite loading a value (`octet_length` reads the record header and
/// `NOT INDEXED` scans the table's own b-tree), as
/// `stored_read::admit_sqlite_working_set` measures the history tables, which
/// do not include the libraries.
const LIBRARY_KEYED_ROW_SQL: &str = "SELECT COALESCE(MAX(COALESCE(octet_length(project_id), 0) + COALESCE(octet_length(library_kind), 0) + COALESCE(octet_length(library_id), 0)), 0) FROM local_libraries NOT INDEXED";
const LIBRARY_LENGTH_SQL: &str = "SELECT octet_length(document_json) FROM local_libraries WHERE project_id = ?1 AND library_kind = ?2 AND library_id = ?3";
const LIBRARY_TEXT_SQL: &str = "SELECT document_json FROM local_libraries WHERE project_id = ?1 AND library_kind = ?2 AND library_id = ?3";

/// The screening's library lookup (Save Model Document's `load_library`
/// query), metered within the export's memory, in its read transaction
/// (Codex r3 P1): before the first lookup, SQLite's working set for the
/// library keys (`stored_read::SQLITE_WORKING_SET_FACTOR` times the largest
/// keyed row, which an index seek may materialize); for each library found,
/// its `document_json` measured, then its load and its parse each reserved
/// before they are made. The reservations are held until the screening ends
/// (this value is dropped). A refusal of the pool is kept in `refused`, so the
/// export refuses by its name rather than as the screening's store failure.
struct LibraryReads<'a> {
    connection: &'a Connection,
    memory: &'a dyn ReadMemory,
    keyed: Option<ReadReservation>,
    held: Vec<ReadReservation>,
    refused: Option<String>,
}

impl<'a> LibraryReads<'a> {
    fn new(connection: &'a Connection, memory: &'a dyn ReadMemory) -> Self {
        Self { connection, memory, keyed: None, held: Vec::new(), refused: None }
    }

    fn read(&mut self, project_id: &str, library_kind: &str, library_id: &str) -> Result<Option<Value>, String> {
        let read = self.read_metered(project_id.trim(), library_kind.trim(), library_id.trim());
        if let Err(error) = &read {
            if error.starts_with(MEMORY_UNAVAILABLE) && self.refused.is_none() {
                self.refused = Some(error.clone());
            }
        }
        read
    }

    fn read_metered(&mut self, project_id: &str, library_kind: &str, library_id: &str) -> Result<Option<Value>, String> {
        let sql_error = |error: rusqlite::Error| stored_read::sql_error(&error);
        if self.keyed.is_none() {
            let largest: i64 = self.connection.query_row(LIBRARY_KEYED_ROW_SQL, [], |row| row.get(0)).map_err(sql_error)?;
            let what = format!("SQLite's working set for the stored libraries (their largest keyed row holds {largest} bytes)");
            self.keyed = Some(stored_read::reserve_sqlite_value(self.memory, &what, largest)?);
        }
        let key = params![project_id, library_kind, library_id];
        let length: Option<Option<i64>> = self.connection.query_row(LIBRARY_LENGTH_SQL, key, |row| row.get(0)).optional().map_err(sql_error)?;
        let Some(length) = length else { return Ok(None) };
        let length = u64::try_from(length.unwrap_or(0)).unwrap_or(0);
        let what = format!("the stored {} library `{}` ({length} bytes)", stored_read::shown(library_kind), stored_read::shown(library_id));
        self.held.push(self.memory.reserve(&what, stored_read::loaded_text_bytes(length))?);
        let Some(text) = self.connection.query_row(LIBRARY_TEXT_SQL, key, |row| row.get::<_, String>(0)).optional().map_err(sql_error)? else {
            return Ok(None);
        };
        self.held.push(self.memory.reserve(&what, stored_read::parse_bytes(&text)?)?);
        serde_json::from_str(&text).map(Some).map_err(|error| stored_read::json_error(&error))
    }
}

/// Reads the stored text of a project's document (`state_id` absent) or of one
/// of its saved states from the store at `store`, opened read-only, and admits
/// it for writing unchanged, within `memory`. Returns the admitted document and
/// the reservation its text lives under; the caller holds that through the
/// write.
pub(crate) fn admit_stored_model_document(
    store: &Path,
    project_id: &str,
    state_id: Option<&str>,
    memory: &dyn ReadMemory,
) -> Result<(AdmittedModelDocument, ReadReservation), Refusal> {
    let named = stored_document_name(project_id, state_id);
    let what = format!("the stored document of {named}");
    let unavailable = |error: String| read_refusal(&named, error);
    let mut connection = open_store_read_only(store, &named)?;
    // One read transaction: the row measured is the row loaded.
    let transaction = stored_read::read_transaction(&mut connection).map_err(unavailable)?;
    let _sqlite = stored_read::admit_sqlite_working_set(&transaction, memory).map_err(unavailable)?;
    let length: i64 = query_stored(&transaction, &stored_text_sql(state_id, "octet_length"), project_id, state_id)
        .map_err(|error| unavailable(stored_read::sql_error(&error)))?
        .ok_or_else(|| refusal(NOT_FOUND, format!("The local project store holds no {named}; nothing was written.")))?;
    let length = u64::try_from(length).unwrap_or(0);
    if length > MAX_MODEL_DOCUMENT_BYTES {
        return Err(refusal(
            TOO_LARGE,
            format!("The stored document of {named} is {length} bytes; a model document may be at most {MAX_MODEL_DOCUMENT_BYTES} bytes, so it would not re-open. Nothing was read or written."),
        ));
    }
    let load = memory.reserve(&what, stored_read::loaded_text_bytes(length)).map_err(unavailable)?;
    let text: String = query_stored(&transaction, &stored_text_sql(state_id, ""), project_id, state_id)
        .map_err(|error| unavailable(stored_read::sql_error(&error)))?
        .ok_or_else(|| refusal(NOT_FOUND, format!("The local project store holds no {named}; nothing was written.")))?;
    let not_json = |detail: String| refusal(NOT_JSON, format!("The stored document of {named} is not JSON ({detail}); nothing was written."));
    let parse = stored_read::parse_bytes(&text).map_err(not_json)?;
    let parsing = memory.reserve(&what, parse).map_err(unavailable)?;
    let document: Value = serde_json::from_str(&text).map_err(|error| not_json(stored_read::json_error(&error)))?;
    refuse_duplicate_members(memory, &what, &named, &text, &document)?;
    // Libraries the document references are read from the same read-only
    // transaction, each within the export's memory, and their reservations
    // are held through the screening.
    let mut libraries = LibraryReads::new(&transaction, memory);
    let screened = screen_model_document_dispositions(&document, &mut |project_id: &str, library_kind: &str, library_id: &str| {
        libraries.read(project_id, library_kind, library_id)
    });
    if let Some(error) = libraries.refused.take() {
        return Err(read_refusal(&named, error));
    }
    let warnings = screened?;
    drop(libraries);
    let suggested_file_name = suggested_model_document_file_name(&document);
    drop(document);
    drop(parsing);
    Ok((AdmittedModelDocument { text, suggested_file_name, warnings }, load))
}

/// The command after its ingress (`lib.rs` `save_stored_model_document_file`):
/// the explicit intent, the store's path (`store`, located only after the
/// intent), then [`export_stored_model_document`] under the command's ingress
/// lease `ingress`.
pub(crate) fn export_stored_model_document_request(
    request: &StoredExportRequest,
    store: impl FnOnce() -> Result<PathBuf, String>,
    registry: &SolveJobRegistry,
    ingress: &Lease,
    choose: impl FnOnce(&str) -> Result<Option<PathBuf>, String>,
) -> ModelDocumentSaveOutcome {
    if !request.explicit_local_private_intent {
        return ModelDocumentSaveOutcome::refused_one(
            INTENT_REQUIRED,
            "Saving the stored document needs the explicit choice to save it, including private project data, to a file.",
        );
    }
    let state_id = request.state_id.as_deref();
    let store = match store() {
        Ok(store) => store,
        Err(error) => {
            let named = stored_document_name(&request.project_id, state_id);
            return ModelDocumentSaveOutcome::refused_one(
                STORE_UNAVAILABLE,
                format!("The local project store could not be located for {named} ({}); nothing was written.", stored_read::capped(error)),
            );
        }
    };
    export_stored_model_document(&store, &request.project_id, state_id, registry, ingress, choose)
}

/// The recovery export: the store's read, the chooser (`choose`, given the
/// suggested file name) and the write, under one memory admission. The
/// accounting starts from the command's ingress lease (`ingress`, which holds
/// its arguments); the command's own memory (connection, statements, ids) is
/// reserved with it as `stored_read::READ_OVERHEAD_BYTES`, and every other
/// reservation is counted with both against the per-invocation budget
/// (`PoolReadMemory`). Nothing is chosen or written after a refusal.
pub(crate) fn export_stored_model_document(
    store: &Path,
    project_id: &str,
    state_id: Option<&str>,
    registry: &SolveJobRegistry,
    ingress: &Lease,
    choose: impl FnOnce(&str) -> Result<Option<PathBuf>, String>,
) -> ModelDocumentSaveOutcome {
    let named = stored_document_name(project_id, state_id);
    let memory = PoolReadMemory::new(registry, ingress);
    let _overhead = match memory.reserve(&format!("the stored document of {named} (the command's own memory)"), stored_read::READ_OVERHEAD_BYTES) {
        Ok(reservation) => reservation,
        Err(error) => return ModelDocumentSaveOutcome::refused_one(MEMORY_UNAVAILABLE, format!("{error} Nothing was read or written.")),
    };
    let (admitted, _text) = match admit_stored_model_document(store, project_id, state_id, &memory) {
        Ok(admitted) => admitted,
        Err(diagnostics) => return ModelDocumentSaveOutcome::refused(diagnostics),
    };
    let destination = match choose(&admitted.suggested_file_name) {
        Ok(Some(destination)) => destination,
        Ok(None) => return ModelDocumentSaveOutcome::Cancelled,
        Err(message) => return ModelDocumentSaveOutcome::refused_one(PICKER_PATH_INVALID, message),
    };
    write_admitted_model_document(&destination, admitted)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command_ingress::test_support::{headers, registry};
    use crate::command_ingress::{admit_body_within, ArgsDecoding, INGRESS_ARGS_JSON_V1};
    use crate::memory_pool::test_host::ScriptedHost;
    use crate::model_document_save::PROTECTED_CONTENT_BLOCKED;
    use open_pipe_stress_operation_applier::sha256_hex;
    use serde_json::json;
    use std::cell::Cell;
    use std::fs;
    use tauri::ipc::InvokeBody;

    const PROJECT: &str = "project:invented-no-units";
    const STATE: &str = "state:invented-1";

    /// A document without `project.units`, stored as the store serializes it.
    fn unitless_text() -> String {
        let document = json!({
            "schema_version": "0.7.0",
            "document_kind": "openpipestress.model",
            "project": { "id": PROJECT, "name": "Invented", "description": "" },
            "materials": [{ "id": "material:invented", "provenance_record": { "review_status": "accepted" } }]
        });
        serde_json::to_string(&document).unwrap()
    }

    /// Key order and spacing that a re-serialization would change.
    const STATE_TEXT: &str = "{\"schema_version\":\"0.7.0\",\"project\":{\"name\":\"Invented\",\"id\":\"project:invented-no-units\"}, \"x\":1.50}";

    struct Scratch(PathBuf);

    impl Scratch {
        fn new(name: &str) -> Self {
            let root = std::env::temp_dir().join(format!(
                "swbpipe-stored-export-{name}-{}-{}",
                std::process::id(),
                std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
            ));
            fs::create_dir_all(&root).unwrap();
            Self(root)
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    /// A store at this build's schema holding the project row and one saved
    /// state, written with raw SQL (a corrupt row is what the export recovers).
    fn current_store(scratch: &Scratch, model_json: &str, state_text: &str) -> PathBuf {
        let path = scratch.0.join("store.sqlite3");
        let (connection, _) = crate::open_project_store(&path).unwrap();
        connection
            .execute(
                "INSERT INTO local_projects(project_id, project_name, model_json, created_at_unix, updated_at_unix) VALUES (?1, 'Invented', ?2, 1, 1)",
                params![PROJECT, model_json],
            )
            .unwrap();
        connection
            .execute(
                "INSERT INTO model_states(project_id, state_id, state_kind, created_at, parent_state_id, model_canonical, model_hash, record_json, record_hash)
                 VALUES (?1, ?2, 'snapshot', '2026-01-01T00:00:00Z', NULL, ?3, ?4, '{}', 'sha256:0')",
                params![PROJECT, STATE, state_text, format!("sha256:{}", sha256_hex(state_text))],
            )
            .unwrap();
        drop(connection);
        path
    }

    /// The command's `args-json-v1` body for a row.
    fn envelope(project_id: &str, state_id: Option<&str>) -> InvokeBody {
        InvokeBody::Raw(serde_json::to_vec(&json!({ "projectId": project_id, "stateId": state_id, "explicitLocalPrivateIntent": true })).unwrap())
    }

    /// The command's ingress (`lib.rs` `save_stored_model_document_file`):
    /// `admit_within`'s rules at the command's budget, then its arguments.
    fn admitted(pool: &SolveJobRegistry, body: &InvokeBody) -> Result<(StoredExportRequest, Lease), SolveJobError> {
        let (mut args, lease) = admit_body_within(pool, body, &headers(Some(INGRESS_ARGS_JSON_V1)), COMMAND, ArgsDecoding::Moved, MAX_ARGS_BYTES)?;
        Ok((stored_export_args(&mut args)?, lease))
    }

    /// The command, from its body to its outcome, with the store at `store`.
    fn command(
        store: &Path,
        body: &InvokeBody,
        pool: &SolveJobRegistry,
        choose: impl FnOnce(&str) -> Result<Option<PathBuf>, String>,
    ) -> ModelDocumentSaveOutcome {
        let (request, lease) = admitted(pool, body).expect("the envelope is admitted");
        export_stored_model_document_request(&request, || Ok(store.to_path_buf()), pool, &lease, choose)
    }

    /// The export with a chooser that records whether it was shown.
    fn export(store: &Path, state_id: Option<&str>, pool: &SolveJobRegistry, destination: &Path, shown: &Cell<bool>) -> ModelDocumentSaveOutcome {
        command(store, &envelope(PROJECT, state_id), pool, |_| {
            shown.set(true);
            Ok(Some(destination.to_path_buf()))
        })
    }

    fn refused_codes(outcome: &ModelDocumentSaveOutcome) -> Vec<(&str, &str)> {
        match outcome {
            ModelDocumentSaveOutcome::Refused { diagnostics } => diagnostics.iter().map(|item| (item.code.as_str(), item.path.as_str())).collect(),
            other => panic!("not refused: {other:?}"),
        }
    }

    fn assert_refused_nothing_written(outcome: &ModelDocumentSaveOutcome, code: &str, destination: &Path, shown: &Cell<bool>) {
        assert_eq!(refused_codes(outcome).first().map(|(code, _)| *code), Some(code), "{outcome:?}");
        assert!(!shown.get(), "the chooser is not shown after a refusal");
        assert!(!destination.exists(), "nothing is written");
    }

    #[test]
    fn writes_the_stored_bytes_unchanged_and_leaves_the_store_unchanged() {
        let scratch = Scratch::new("bytes");
        let store = current_store(&scratch, &unitless_text(), STATE_TEXT);
        let before = fs::read(&store).unwrap();
        let pool = registry(ScriptedHost::normal());
        for (state_id, stored, name) in [(None, unitless_text(), "invented-no-units.json"), (Some(STATE), STATE_TEXT.to_string(), "invented-no-units.json")] {
            let destination = scratch.0.join(format!("recovered-{}.json", state_id.is_some()));
            let suggested = Cell::new(String::new());
            let outcome = command(&store, &envelope(PROJECT, state_id), &pool, |offered| {
                suggested.set(offered.to_string());
                Ok(Some(destination.clone()))
            });
            match outcome {
                ModelDocumentSaveOutcome::Saved { byte_count, model_hash, .. } => {
                    assert_eq!(byte_count, stored.len() as u64);
                    assert_eq!(model_hash, format!("sha256:{}", sha256_hex(&stored)));
                }
                other => panic!("not saved: {other:?}"),
            }
            assert_eq!(suggested.take(), name);
            assert_eq!(fs::read(&destination).unwrap(), stored.as_bytes());
        }
        assert_eq!(pool.pool_status().reserved_bytes, 0, "every reservation is released");
        assert_eq!(fs::read(&store).unwrap(), before, "the store file is unchanged");
        let shown = Cell::new(false);
        let missing = command(&store, &envelope("project:other", None), &pool, |_| {
            shown.set(true);
            Ok(None)
        });
        assert_eq!(refused_codes(&missing), vec![(NOT_FOUND, "")]);
        let missing = command(&store, &envelope(PROJECT, Some("state:other")), &pool, |_| Ok(None));
        assert_eq!(refused_codes(&missing), vec![(NOT_FOUND, "")]);
        assert!(!shown.get());
    }

    #[test]
    fn screens_dispositions_and_refuses_text_that_is_not_json() {
        let scratch = Scratch::new("screen");
        let blocked = unitless_text().replace("\"accepted\"", "\"quarantined\"");
        let store = current_store(&scratch, &blocked, "{not json");
        let pool = registry(ScriptedHost::normal());
        let destination = scratch.0.join("recovered.json");
        let shown = Cell::new(false);
        let refused = export(&store, None, &pool, &destination, &shown);
        assert_eq!(refused_codes(&refused), vec![(PROTECTED_CONTENT_BLOCKED, "materials[0].provenance_record")]);
        assert_refused_nothing_written(&refused, PROTECTED_CONTENT_BLOCKED, &destination, &shown);
        let refused = export(&store, Some(STATE), &pool, &destination, &shown);
        assert_refused_nothing_written(&refused, NOT_JSON, &destination, &shown);
    }

    /// Codex r2 finding 1: a corrupt row holding `provenance_record` twice, the
    /// first quarantined and the second accepted. serde_json keeps the last
    /// member, so screening the parsed document alone would admit bytes that
    /// carry the quarantined record.
    #[test]
    fn duplicate_members_are_refused_before_screening() {
        let scratch = Scratch::new("duplicate");
        let duplicated = "{\"schema_version\":\"0.7.0\",\"document_kind\":\"openpipestress.model\",\"project\":{\"id\":\"project:invented-no-units\",\"name\":\"Invented\"},\"materials\":[{\"id\":\"material:invented\",\"provenance_record\":{\"review_status\":\"quarantined\"},\"provenance_record\":{\"review_status\":\"accepted\"}}]}";
        let top_level = "{\"project\":{\"id\":\"project:invented-no-units\"},\"x\":1,\"x\":2}";
        let store = current_store(&scratch, duplicated, top_level);
        let pool = registry(ScriptedHost::normal());
        let destination = scratch.0.join("recovered.json");
        for state_id in [None, Some(STATE)] {
            let shown = Cell::new(false);
            let refused = export(&store, state_id, &pool, &destination, &shown);
            assert_eq!(refused_codes(&refused), vec![(DUPLICATE_MEMBER, "")]);
            assert_refused_nothing_written(&refused, DUPLICATE_MEMBER, &destination, &shown);
        }
        // A value the checked profile refuses after a complete strict parse is
        // not a duplicate member, and is exported unchanged.
        let scratch = Scratch::new("unsafe-integer");
        let store = current_store(&scratch, "{\"project\":{\"id\":\"project:invented-no-units\"},\"x\":9007199254740993}", STATE_TEXT);
        let shown = Cell::new(false);
        assert!(matches!(export(&store, None, &pool, &destination, &shown), ModelDocumentSaveOutcome::Saved { .. }));
    }

    /// Codex r2 finding 2: the read is admitted by the memory pool before it
    /// loads or parses the row; a refused admission is named and writes nothing.
    #[test]
    fn a_read_the_memory_pool_refuses_is_refused_by_name_and_writes_nothing() {
        let scratch = Scratch::new("memory");
        let padded = serde_json::to_string(&json!({ "project": { "id": PROJECT }, "pad": "x".repeat(300 * 1024) })).unwrap();
        let store = current_store(&scratch, &padded, STATE_TEXT);
        let destination = scratch.0.join("recovered.json");
        // The pool admits the command's own lease but not the row's load.
        let small = registry(ScriptedHost::normal());
        small.configure_pool(1 << 20, 1 << 20, false).unwrap();
        // The pool admits nothing of the export.
        let tiny = registry(ScriptedHost::normal());
        tiny.configure_pool(64 << 10, 64 << 10, false).unwrap();
        // The host's headroom does not cover the row's load.
        let host = ScriptedHost::normal();
        let short = registry(host.clone());
        host.set(|script| script.headroom_bytes = Some(800 << 10));
        for pool in [&small, &tiny, &short] {
            let shown = Cell::new(false);
            let refused = export(&store, None, pool, &destination, &shown);
            assert_refused_nothing_written(&refused, MEMORY_UNAVAILABLE, &destination, &shown);
            assert_eq!(pool.pool_status().reserved_bytes, 0, "a refusal releases every reservation");
        }
        // The same read under an ordinary pool is admitted.
        let shown = Cell::new(false);
        assert!(matches!(export(&store, None, &registry(ScriptedHost::normal()), &destination, &shown), ModelDocumentSaveOutcome::Saved { .. }));
    }

    /// Codex r2 finding 3: a row above the model-document file cap is refused
    /// as the save refuses it, from its measured length.
    #[test]
    fn a_row_above_the_model_document_cap_is_refused_as_the_save_refuses_it() {
        let scratch = Scratch::new("cap");
        let prefix = "{\"project\":{\"id\":\"project:invented-no-units\"},\"pad\":\"";
        let fill = MAX_MODEL_DOCUMENT_BYTES as usize + 1 - prefix.len() - 2;
        let over = format!("{prefix}{}\"}}", "x".repeat(fill));
        assert_eq!(over.len() as u64, MAX_MODEL_DOCUMENT_BYTES + 1);
        let at = format!("{prefix}{}\"}}", "x".repeat(fill - 1));
        let store = current_store(&scratch, &over, &at);
        let pool = registry(ScriptedHost::normal());
        let destination = scratch.0.join("recovered.json");
        let shown = Cell::new(false);
        let refused = export(&store, None, &pool, &destination, &shown);
        assert_eq!(refused_codes(&refused), vec![(TOO_LARGE, "")]);
        assert_refused_nothing_written(&refused, TOO_LARGE, &destination, &shown);
        // At the cap it is written.
        assert!(matches!(export(&store, Some(STATE), &pool, &destination, &shown), ModelDocumentSaveOutcome::Saved { byte_count, .. } if byte_count == MAX_MODEL_DOCUMENT_BYTES));
    }

    /// Codex r2 finding 4: the store is opened read-only and never migrated; a
    /// store at another schema version is refused by name and left unchanged.
    #[test]
    fn a_store_at_another_schema_version_is_refused_and_left_unchanged() {
        let pool = registry(ScriptedHost::normal());
        for version in [crate::STORE_SCHEMA_TARGET_VERSION - 1, crate::STORE_SCHEMA_TARGET_VERSION + 1] {
            let scratch = Scratch::new("schema");
            let store = current_store(&scratch, &unitless_text(), STATE_TEXT);
            Connection::open(&store).unwrap().execute_batch(&format!("PRAGMA user_version = {version};")).unwrap();
            let before = fs::read(&store).unwrap();
            let destination = scratch.0.join("recovered.json");
            let shown = Cell::new(false);
            let refused = export(&store, None, &pool, &destination, &shown);
            assert_refused_nothing_written(&refused, STORE_NOT_CURRENT, &destination, &shown);
            assert_eq!(fs::read(&store).unwrap(), before, "the store file is unchanged");
            let version_after: i64 = Connection::open_with_flags(&store, OpenFlags::SQLITE_OPEN_READ_ONLY)
                .unwrap()
                .query_row("PRAGMA user_version", [], |row| row.get(0))
                .unwrap();
            assert_eq!(version_after, version);
        }
        // A missing store is not created.
        let scratch = Scratch::new("missing");
        let store = scratch.0.join("absent.sqlite3");
        let destination = scratch.0.join("recovered.json");
        let shown = Cell::new(false);
        let refused = export(&store, None, &pool, &destination, &shown);
        assert_refused_nothing_written(&refused, STORE_UNAVAILABLE, &destination, &shown);
        assert!(!store.exists());
    }

    const LIBRARY: &str = "library:invented-large";

    /// A small document without `project.units` that references one stored
    /// material library, and a store holding it with `library` as the
    /// library's `document_json`.
    fn library_store(scratch: &Scratch, library: &str) -> (PathBuf, String) {
        let model = format!(
            "{{\"project\":{{\"id\":\"{PROJECT}\"}},\"materials\":[{{\"id\":\"m\",\"library_ref\":{{\"library_kind\":\"material\",\"library_id\":\"{LIBRARY}\"}}}}]}}"
        );
        let store = current_store(scratch, &model, STATE_TEXT);
        Connection::open(&store)
            .unwrap()
            .execute(
                "INSERT INTO local_libraries(project_id, library_kind, library_id, document_json, created_at_unix, updated_at_unix) VALUES (?1, 'material', ?2, ?3, 1, 1)",
                params![PROJECT, LIBRARY, library],
            )
            .unwrap();
        (store, model)
    }

    /// A stored material library of about `pad` bytes, with `review_status`.
    fn library_text(pad: usize, review_status: &str) -> String {
        serde_json::to_string(&json!({ "material_library": { "library_id": LIBRARY, "review_status": review_status, "pad": "x".repeat(pad) } })).unwrap()
    }

    /// Codex r3 P1: the screening's lookup of a referenced library loaded and
    /// parsed its `document_json` unmetered (a 112-byte model referencing a
    /// 4 MiB library exported under a 1 MiB pool). Its load and parse are
    /// reserved before they are made, with the read's other reservations, and
    /// a refusal is named and writes nothing.
    #[test]
    fn a_referenced_library_the_memory_pool_refuses_is_refused_by_name_and_writes_nothing() {
        let scratch = Scratch::new("library-memory");
        let library = library_text(4 << 20, "accepted");
        let (store, model) = library_store(&scratch, &library);
        assert!(model.len() < 160, "the model itself is small ({} bytes)", model.len());
        let destination = scratch.0.join("recovered.json");
        // The pool admits the model's read but not the library's.
        let small = registry(ScriptedHost::normal());
        small.configure_pool(1 << 20, 1 << 20, false).unwrap();
        // The library alone fits the per-invocation budget, but not with the
        // command's own memory and the model's read held with it.
        let library_cost = stored_read::loaded_text_bytes(library.len() as u64) + stored_read::parse_bytes(&library).unwrap();
        let remaining = registry(ScriptedHost::normal());
        let budget = library_cost + stored_read::READ_OVERHEAD_BYTES / 2;
        remaining.configure_pool(budget * 2, budget, false).unwrap();
        for pool in [&small, &remaining] {
            let shown = Cell::new(false);
            let refused = export(&store, None, pool, &destination, &shown);
            assert_refused_nothing_written(&refused, MEMORY_UNAVAILABLE, &destination, &shown);
            let ModelDocumentSaveOutcome::Refused { diagnostics } = &refused else { unreachable!() };
            assert!(diagnostics[0].message.contains(&format!("the stored material library `{LIBRARY}`")), "{refused:?}");
            assert_eq!(pool.pool_status().reserved_bytes, 0, "a refusal releases every reservation");
        }
        // Under an ordinary pool the same library is read, screened and the
        // model written unchanged; every reservation is released afterwards.
        let pool = registry(ScriptedHost::normal());
        let shown = Cell::new(false);
        match export(&store, None, &pool, &destination, &shown) {
            ModelDocumentSaveOutcome::Saved { byte_count, warnings, .. } => {
                assert_eq!(byte_count, model.len() as u64);
                assert!(warnings.is_empty(), "the library resolved: {warnings:?}");
            }
            other => panic!("not saved: {other:?}"),
        }
        assert_eq!(fs::read(&destination).unwrap(), model.as_bytes());
        assert_eq!(pool.pool_status().reserved_bytes, 0, "every reservation is released");
        // The metered lookup still feeds the screening: a quarantined library blocks.
        let scratch = Scratch::new("library-blocked");
        let (store, _) = library_store(&scratch, &library_text(16, "quarantined"));
        let destination = scratch.0.join("recovered.json");
        let shown = Cell::new(false);
        let refused = export(&store, None, &pool, &destination, &shown);
        assert_refused_nothing_written(&refused, PROTECTED_CONTENT_BLOCKED, &destination, &shown);
        assert_eq!(pool.pool_status().reserved_bytes, 0);
    }

    /// Codex r3 P2: the command took typed Tauri arguments, decoded unbounded
    /// before any lease. It takes an `args-json-v1` body within the history
    /// commands' budget: a longer body is refused before it is scanned or
    /// decoded (no lease is taken), a JSON body or a missing header is
    /// refused, and a well-formed envelope exports.
    #[test]
    fn the_arguments_are_an_args_json_v1_body_within_the_history_budget() {
        let scratch = Scratch::new("ingress");
        let store = current_store(&scratch, &unitless_text(), STATE_TEXT);
        let pool = registry(ScriptedHost::normal());
        assert_eq!(MAX_ARGS_BYTES, crate::model_states::HISTORY_COMMAND_MAX_ARGS_BYTES);
        let padded = |pad: usize| InvokeBody::Raw(format!("{{\"projectId\":\"{}\",\"explicitLocalPrivateIntent\":true}}", "p".repeat(pad)).into_bytes());
        let fixed = "{\"projectId\":\"\",\"explicitLocalPrivateIntent\":true}".len();
        let InvokeBody::Raw(at) = padded(MAX_ARGS_BYTES - fixed) else { unreachable!() };
        assert_eq!(at.len(), MAX_ARGS_BYTES);
        assert!(admitted(&pool, &InvokeBody::Raw(at)).is_ok(), "a body at the budget is admitted");
        match admitted(&pool, &padded(MAX_ARGS_BYTES - fixed + 1)) {
            Err(SolveJobError::InvalidRequest { message }) => assert_eq!(
                message,
                format!("INGRESS-ARGS-TOO-LARGE: `{COMMAND}` takes at most {MAX_ARGS_BYTES} bytes of arguments; {} were sent.", MAX_ARGS_BYTES + 1)
            ),
            other => panic!("not refused: {other:?}"),
        }
        assert_eq!(pool.pool_status().reserved_bytes, 0, "no lease is held after a refusal");
        // The typed form (a JSON body, which Tauri has already decoded) and a
        // body without the header are refused.
        let typed = InvokeBody::Json(json!({ "projectId": PROJECT, "stateId": null, "explicitLocalPrivateIntent": true }));
        assert!(matches!(admitted(&pool, &typed), Err(SolveJobError::InvalidRequest { message }) if message.starts_with("INGRESS-RAW-BODY-REQUIRED")));
        let unheaded = admit_body_within(&pool, &envelope(PROJECT, None), &headers(None), COMMAND, ArgsDecoding::Moved, MAX_ARGS_BYTES);
        assert!(matches!(unheaded, Err(SolveJobError::InvalidRequest { message }) if message.starts_with("INGRESS-HEADER-REQUIRED")));
        // An argument of the wrong type is refused by name.
        let wrong = InvokeBody::Raw(b"{\"projectId\":\"p\",\"explicitLocalPrivateIntent\":\"yes\"}".to_vec());
        assert!(matches!(admitted(&pool, &wrong), Err(SolveJobError::InvalidRequest { message }) if message.contains("`explicitLocalPrivateIntent`")));
        // Without the intent nothing is located, read or written.
        let refusing = InvokeBody::Raw(serde_json::to_vec(&json!({ "projectId": PROJECT, "explicitLocalPrivateIntent": false })).unwrap());
        let (request, lease) = admitted(&pool, &refusing).unwrap();
        let located = Cell::new(false);
        let outcome = export_stored_model_document_request(&request, || { located.set(true); Ok(store.clone()) }, &pool, &lease, |_| panic!("no chooser"));
        assert_eq!(refused_codes(&outcome), vec![(INTENT_REQUIRED, "")]);
        assert!(!located.get());
        drop(lease);
        // A well-formed envelope exports, for a project and for a saved state.
        for state_id in [None, Some(STATE)] {
            let destination = scratch.0.join(format!("recovered-{}.json", state_id.is_some()));
            let shown = Cell::new(false);
            assert!(matches!(export(&store, state_id, &pool, &destination, &shown), ModelDocumentSaveOutcome::Saved { .. }));
            assert!(destination.exists());
        }
        assert_eq!(pool.pool_status().reserved_bytes, 0);
    }

    /// Codex r3 P2: the ingress lease is counted with the read's reservations
    /// against the per-invocation budget. With a budget below the lease and the
    /// command's own memory together (and a pool large enough for both), the
    /// export is refused by name before it reads; counted without the lease it
    /// would fit.
    #[test]
    fn the_ingress_lease_is_counted_in_the_per_invocation_budget() {
        let scratch = Scratch::new("ingress-lease");
        let store = current_store(&scratch, &unitless_text(), STATE_TEXT);
        let body = InvokeBody::Raw(
            serde_json::to_vec(&json!({ "projectId": PROJECT, "explicitLocalPrivateIntent": true, "pad": "x".repeat(200 * 1024) })).unwrap(),
        );
        let lease_bytes = {
            let probe = registry(ScriptedHost::normal());
            let (_, lease) = admitted(&probe, &body).unwrap();
            lease.bytes()
        };
        assert!(lease_bytes > 200 * 1024, "the padded body's lease ({lease_bytes} bytes) is large");
        let pool = registry(ScriptedHost::normal());
        let budget = lease_bytes + stored_read::READ_OVERHEAD_BYTES - 1;
        pool.configure_pool(16 << 20, budget, false).unwrap();
        let destination = scratch.0.join("recovered.json");
        let shown = Cell::new(false);
        let refused = command(&store, &body, &pool, |_| {
            shown.set(true);
            Ok(Some(destination.clone()))
        });
        assert_refused_nothing_written(&refused, MEMORY_UNAVAILABLE, &destination, &shown);
        assert_eq!(pool.pool_status().reserved_bytes, 0);
        // The same read without the padded arguments fits that budget.
        let shown = Cell::new(false);
        assert!(matches!(export(&store, None, &pool, &destination, &shown), ModelDocumentSaveOutcome::Saved { .. }));
    }
}
