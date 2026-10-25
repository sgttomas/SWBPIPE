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
//! - The read is metered as the saved-state reader's is (`stored_read`, W-12):
//!   in one read transaction, SQLite's working set is reserved, the row's
//!   length is measured without loading it, a row above the model-document
//!   file cap is refused (`MODEL-DOC-SAVE-TOO-LARGE`, the save's refusal: Open
//!   Model Document refuses such a file), and its load, its parse and the
//!   strict parse are each reserved before they are made. A reservation the
//!   memory pool refuses is `STORE-READ-MEMORY-UNAVAILABLE`. The text's
//!   reservation is held through the chooser and the write.
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

use crate::export_disposition_screen::diagnostic;
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
    // transaction (Save Model Document's lookup, `load_library`).
    let mut lookup = |project_id: &str, library_kind: &str, library_id: &str| {
        crate::load_library(&transaction, project_id.trim(), library_kind.trim(), library_id.trim()).map(|row| row.map(|row| row.document))
    };
    let warnings = screen_model_document_dispositions(&document, &mut lookup)?;
    let suggested_file_name = suggested_model_document_file_name(&document);
    drop(document);
    drop(parsing);
    Ok((AdmittedModelDocument { text, suggested_file_name, warnings }, load))
}

/// The recovery export: the store's read, the chooser (`choose`, given the
/// suggested file name) and the write, under one memory admission. The
/// command's own memory (connection, statements, ids) is an interactive lease
/// of `stored_read::READ_OVERHEAD_BYTES`; every other reservation is counted
/// with it against the per-invocation budget (`PoolReadMemory`). Nothing is
/// chosen or written after a refusal.
pub(crate) fn export_stored_model_document(
    store: &Path,
    project_id: &str,
    state_id: Option<&str>,
    registry: &SolveJobRegistry,
    choose: impl FnOnce(&str) -> Result<Option<PathBuf>, String>,
) -> ModelDocumentSaveOutcome {
    let named = stored_document_name(project_id, state_id);
    let lease = match registry.acquire_interactive_ingress(stored_read::READ_OVERHEAD_BYTES) {
        Ok(lease) => lease,
        Err(error) => {
            let detail = match error {
                SolveJobError::ResourceUnavailable { refusal, .. } => serde_json::to_string(&refusal).unwrap_or_else(|_| "refused".to_string()),
                SolveJobError::InvalidRequest { message } => message,
            };
            return ModelDocumentSaveOutcome::refused_one(
                MEMORY_UNAVAILABLE,
                format!("{MEMORY_UNAVAILABLE}: reading the stored document of {named} needs {} bytes of the memory pool, which refused them ({detail}); nothing was read or written.", stored_read::READ_OVERHEAD_BYTES),
            );
        }
    };
    let memory = PoolReadMemory::new(registry, &lease);
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
    use crate::command_ingress::test_support::registry;
    use crate::memory_pool::test_host::ScriptedHost;
    use crate::model_document_save::PROTECTED_CONTENT_BLOCKED;
    use open_pipe_stress_operation_applier::sha256_hex;
    use serde_json::json;
    use std::cell::Cell;
    use std::fs;

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

    /// The export with a chooser that records whether it was shown.
    fn export(store: &Path, state_id: Option<&str>, pool: &SolveJobRegistry, destination: &Path, shown: &Cell<bool>) -> ModelDocumentSaveOutcome {
        export_stored_model_document(store, PROJECT, state_id, pool, |_| {
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
            let outcome = export_stored_model_document(&store, PROJECT, state_id, &pool, |offered| {
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
        let missing = export_stored_model_document(&store, "project:other", None, &pool, |_| {
            shown.set(true);
            Ok(None)
        });
        assert_eq!(refused_codes(&missing), vec![(NOT_FOUND, "")]);
        let missing = export_stored_model_document(&store, PROJECT, Some("state:other"), &pool, |_| Ok(None));
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
}
