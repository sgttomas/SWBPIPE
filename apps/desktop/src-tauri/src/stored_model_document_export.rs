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
//! - The shared screener's own allocations (copied paths, recorded
//!   references, diagnostics, a hanger library's collected objects) are
//!   bounded from the parsed model and each parsed library and reserved
//!   before it builds them ([`ScreeningBound`], [`library_screening_bytes`]).
//! - The outcome is serialized within the same accounting while every
//!   reservation and the ingress lease are held ([`serialize_outcome`]); a
//!   Saved response's bound is reserved before the chooser.
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
    // The libraries' key measurement scans `local_libraries` on this connection.
    bound_library_read_cache(&connection).map_err(unavailable)?;
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

/// SQLite's working set for the library keys, as priced so far in one
/// command: the largest keyed row it was priced for, and its reservations.
#[derive(Default)]
pub(crate) struct KeyedWorkingSet {
    priced: i64,
    held: Vec<ReadReservation>,
}

impl KeyedWorkingSet {
    /// Re-measures the largest keyed row in the caller's transaction and grows
    /// the reservation to it before any index search: a key another connection
    /// wrote after an earlier read's transaction is priced before a search can
    /// materialize it (#1641 Codex r1 P1).
    fn admit(&mut self, connection: &Connection, memory: &dyn ReadMemory) -> Result<(), String> {
        let largest: i64 = connection.query_row(LIBRARY_KEYED_ROW_SQL, [], |row| row.get(0)).map_err(|error| stored_read::sql_error(&error))?;
        if self.held.is_empty() || largest > self.priced {
            let what = format!("SQLite's working set for the stored libraries (their largest keyed row holds {largest} bytes)");
            self.held.push(stored_read::reserve_sqlite_value(memory, &what, largest.saturating_sub(self.priced))?);
            self.priced = largest;
        }
        Ok(())
    }

    fn into_held(self) -> Vec<ReadReservation> {
        self.held
    }

    #[cfg(test)]
    fn reserved_bytes(&self) -> u64 {
        self.held.iter().map(ReadReservation::bytes).sum()
    }
}

/// One stored library read within `memory` (Codex r3 P1): SQLite's working
/// set for the library keys (`stored_read::SQLITE_WORKING_SET_FACTOR` times
/// the largest keyed row, which an index seek may materialize), re-measured in
/// this read's transaction and grown before the search ([`KeyedWorkingSet`]);
/// then the library's `document_json` measured without loading it, its load
/// reserved (`stored_read::loaded_text_bytes`) before it is loaded, and its
/// `Value` materialization, bounded by the ingress prescan
/// (`stored_read::parse_bytes`), reserved before it is parsed. The
/// reservations are pushed to `held`. Errors are bounded: a SQLite error as
/// `stored_read::sql_error` describes it, a text the prescan refuses by its
/// reason and offset, a serde error by `stored_read::json_error`, never the
/// stored text. A reservation the pool refuses is
/// `STORE-READ-MEMORY-UNAVAILABLE`. Every measurement and the load are made on
/// `connection` in the caller's read transaction, so the row measured is the
/// row loaded. Shared by the recovery export's screening ([`LibraryReads`])
/// and by Save Model Document's and Save Handoff's lookups
/// ([`StoreLibraryReads`]).
pub(crate) fn read_stored_library(
    connection: &Connection,
    memory: &dyn ReadMemory,
    keyed: &mut KeyedWorkingSet,
    held: &mut Vec<ReadReservation>,
    project_id: &str,
    library_kind: &str,
    library_id: &str,
) -> Result<Option<StoredLibrary>, String> {
    let sql_error = |error: rusqlite::Error| stored_read::sql_error(&error);
    keyed.admit(connection, memory)?;
    let key = params![project_id, library_kind, library_id];
    let length: Option<Option<i64>> = connection.query_row(LIBRARY_LENGTH_SQL, key, |row| row.get(0)).optional().map_err(sql_error)?;
    let Some(length) = length else { return Ok(None) };
    let length = u64::try_from(length.unwrap_or(0)).unwrap_or(0);
    let what = format!("the stored {} library `{}` ({length} bytes)", stored_read::shown(library_kind), stored_read::shown(library_id));
    held.push(memory.reserve(&what, stored_read::loaded_text_bytes(length))?);
    let Some(text) = connection.query_row(LIBRARY_TEXT_SQL, key, |row| row.get::<_, String>(0)).optional().map_err(sql_error)? else {
        return Ok(None);
    };
    let parse = stored_read::parse_bytes(&text)?;
    held.push(memory.reserve(&what, parse)?);
    let document: Value = serde_json::from_str(&text).map_err(|error| stored_read::json_error(&error))?;
    drop(text);
    Ok(Some(StoredLibrary { document, parse, what }))
}

/// The page cache, in KiB, of a connection that reads stored libraries
/// (`PRAGMA cache_size = -LIBRARY_READ_CACHE_KIB`): it bounds what SQLite caches
/// while the key measurement scans `local_libraries` and while a library's
/// overflow pages are read (#1641 Codex r1 P2). The default (2 MiB) grew
/// with the table. `sqlite3_soft_heap_limit64` and `sqlite3_hard_heap_limit64`
/// are ruled out: they are process-wide, not per connection.
pub(crate) const LIBRARY_READ_CACHE_KIB: i64 = 256;

/// Bounds the page cache of a connection that reads stored libraries.
pub(crate) fn bound_library_read_cache(connection: &Connection) -> Result<(), String> {
    connection.execute_batch(&format!("PRAGMA cache_size = -{LIBRARY_READ_CACHE_KIB};")).map_err(|error| stored_read::sql_error(&error))
}

/// What Save Model Document's and Save Handoff's library store connection
/// holds besides its priced reads, reserved before it is opened: SQLite's
/// bounded page cache ([`LIBRARY_READ_CACHE_KIB`], with its page headers),
/// the connection's lookaside, the parsed store schema, the open's migration
/// check and one prepared statement at a time, and the command's own Rust
/// memory for them (`stored_read::READ_OVERHEAD_BYTES`). The SQLite part was
/// measured against a store with 30,000 library rows
/// (`the_library_store_connection_is_bounded_whatever_the_table_size`).
pub(crate) const LIBRARY_STORE_BYTES: u64 = stored_read::READ_OVERHEAD_BYTES + (1 << 20);

/// A stored library read by [`read_stored_library`]: the document, its
/// prescan's materialization bound, and what the reservations name it.
pub(crate) struct StoredLibrary {
    pub(crate) document: Value,
    pub(crate) parse: u64,
    pub(crate) what: String,
}

/// The screening's library lookup (Save Model Document's `load_library`
/// query), metered within the export's memory, in its read transaction
/// (Codex r3 P1): each library is read by [`read_stored_library`], and the
/// screening's bound for it is reserved before it is returned to the screener.
/// The reservations are held until the screening ends (this value is dropped).
/// A refusal of the pool is kept in `refused`, so the export refuses by its
/// name rather than as the screening's store failure.
struct LibraryReads<'a> {
    connection: &'a Connection,
    memory: &'a dyn ReadMemory,
    /// The model's longest `library_ref` path and identity, which bound the
    /// diagnostics a library's screening can add ([`library_screening_bytes`]).
    references: &'a ScreeningBound,
    keyed: KeyedWorkingSet,
    held: Vec<ReadReservation>,
    refused: Option<String>,
}

impl<'a> LibraryReads<'a> {
    fn new(connection: &'a Connection, memory: &'a dyn ReadMemory, references: &'a ScreeningBound) -> Self {
        Self { connection, memory, references, keyed: KeyedWorkingSet::default(), held: Vec::new(), refused: None }
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
        let Some(StoredLibrary { document: library, parse, what }) =
            read_stored_library(self.connection, self.memory, &mut self.keyed, &mut self.held, project_id, library_kind, library_id)?
        else {
            return Ok(None);
        };
        // What the screener builds from this library (Codex r4 P1), reserved
        // before it is returned to the screener.
        let screening = library_screening_bytes(library_kind, &library, parse, self.references);
        self.held.push(self.memory.reserve(&format!("the screening of {what}"), screening)?);
        Ok(Some(library))
    }
}

/// Save Model Document's and Save Handoff's library lookup (DEL-04-07
/// memory-budget decision, model-sized interactive commands): the local
/// library store opened by `open` only when a library is first referenced,
/// with [`LIBRARY_STORE_BYTES`] reserved before it is opened and its page cache
/// bounded ([`bound_library_read_cache`]); then each library read by
/// [`read_stored_library`] in its own read transaction, the identities trimmed
/// as `open_local_library` trims them. `memory` starts from the command's
/// ingress lease (`PoolReadMemory::new`), so every reservation is counted with
/// it against the per-invocation budget.
///
/// What the command builds from the libraries is reserved here too, before it
/// is built:
/// - the screening's allocations, as the recovery export bounds them
///   ([`ScreeningBound`] of the model, reserved by [`Self::screen_model`]
///   before the screening starts, and [`library_screening_bytes`] for each
///   library before it is returned to the screener);
/// - for the handoff ([`Self::with_reference_metadata`]), its structural
///   screens of each library, priced from their allocation-free count
///   ([`structural_screening_bytes`]), and the reference metadata built from
///   each library: its canonical text, hashed once
///   (`stored_read::grown_text_bytes` of its exact `jcs_length`); its
///   classification is folded over the library as it is walked, collecting
///   nothing from it (`handoff_package::library_refs`). The handoff shares each
///   parsed library between its screening and its references (`Rc`) and
///   copies none.
///
/// The reservations are held until this value is dropped, which the saves do
/// after the write, so they last until the command returns. A refusal of the
/// pool is kept, and the save refuses by its name
/// ([`StoreLibraryReads::refusal`]) rather than as the screening's store
/// failure, as the recovery export does.
pub(crate) struct StoreLibraryReads<'a> {
    memory: &'a dyn ReadMemory,
    open: Box<dyn FnMut() -> Result<Connection, String> + 'a>,
    connection: Option<Connection>,
    overhead: Option<ReadReservation>,
    keyed: KeyedWorkingSet,
    held: Vec<ReadReservation>,
    references: Option<ScreeningBound>,
    reference_metadata: bool,
    refused: Option<String>,
}

impl<'a> StoreLibraryReads<'a> {
    pub(crate) fn new(memory: &'a dyn ReadMemory, open: impl FnMut() -> Result<Connection, String> + 'a) -> Self {
        Self {
            memory,
            open: Box::new(open),
            connection: None,
            overhead: None,
            keyed: KeyedWorkingSet::default(),
            held: Vec::new(),
            references: None,
            reference_metadata: false,
            refused: None,
        }
    }

    /// Also reserves, for each library, the handoff's reference metadata.
    pub(crate) fn with_reference_metadata(mut self) -> Self {
        self.reference_metadata = true;
        self
    }

    /// Reserves the screener's allocations over the admitted `document`
    /// before its screening starts, and keeps its bound for the libraries'.
    pub(crate) fn screen_model(&mut self, document: &Value) -> Result<(), Refusal> {
        let references = ScreeningBound::of(document);
        match self.memory.reserve("the screening of the model document", references.bytes()) {
            Ok(reservation) => self.held.push(reservation),
            Err(error) => {
                self.refused.get_or_insert(error);
                return Err(self.refusal().unwrap_or_default());
            }
        }
        self.references = Some(references);
        Ok(())
    }

    /// The screener's and the handoff's `LibraryLookup`.
    pub(crate) fn read(&mut self, project_id: &str, library_kind: &str, library_id: &str) -> Result<Option<Value>, String> {
        #[cfg(test)]
        if library_read_probe::active() {
            let before = self.reserved_bytes();
            let (read, measured) = crate::ingress::counting::measure(|| self.read_kept(project_id, library_kind, library_id));
            library_read_probe::record(self.reserved_bytes() - before, measured.peak_priced);
            return read;
        }
        self.read_kept(project_id, library_kind, library_id)
    }

    fn read_kept(&mut self, project_id: &str, library_kind: &str, library_id: &str) -> Result<Option<Value>, String> {
        let read = self.read_metered(project_id.trim(), library_kind.trim(), library_id.trim());
        if let Err(error) = &read {
            if error.starts_with(MEMORY_UNAVAILABLE) && self.refused.is_none() {
                self.refused = Some(error.clone());
            }
        }
        read
    }

    fn read_metered(&mut self, project_id: &str, library_kind: &str, library_id: &str) -> Result<Option<Value>, String> {
        if self.connection.is_none() {
            if self.overhead.is_none() {
                self.overhead = Some(self.memory.reserve("the local library store (its connection and the command's own memory)", LIBRARY_STORE_BYTES)?);
            }
            let connection = (self.open)().map_err(stored_read::capped)?;
            bound_library_read_cache(&connection)?;
            self.connection = Some(connection);
        }
        let connection = self.connection.as_mut().expect("opened above");
        // One read transaction per library: the rows measured are the rows loaded.
        let transaction = stored_read::read_transaction(connection)?;
        let read = read_stored_library(&transaction, self.memory, &mut self.keyed, &mut self.held, project_id, library_kind, library_id)?;
        drop(transaction);
        let Some(StoredLibrary { document, parse, what }) = read else { return Ok(None) };
        if let Some(references) = &self.references {
            let screening = library_screening_bytes(library_kind, &document, parse, references);
            self.held.push(self.memory.reserve(&format!("the screening of {what}"), screening)?);
        }
        if self.reference_metadata {
            let canonical = stored_read::grown_text_bytes(stored_read::jcs_length(&document));
            self.held.push(self.memory.reserve(&format!("the canonical text of {what} (its reference digest)"), canonical)?);
            let structural = crate::export_disposition_screen::structural_bound(&document);
            let unscreened = ScreeningBound::default();
            let structural = structural_screening_bytes(&structural, self.references.as_ref().unwrap_or(&unscreened));
            self.held.push(self.memory.reserve(&format!("the structural screening of {what}"), structural)?);
        }
        Ok(Some(document))
    }

    /// The save's refusal when the memory pool refused a library read or the
    /// screening's reservation, by the pool's name
    /// (`STORE-READ-MEMORY-UNAVAILABLE`); the message quotes the pool's typed
    /// refusal and is bounded (ids are cut as messages quote them).
    pub(crate) fn refusal(&mut self) -> Option<Refusal> {
        self.refused.take().map(|error| refusal(MEMORY_UNAVAILABLE, format!("{error} Nothing was written.")))
    }

    /// The bytes this lookup holds reserved now (tests).
    #[cfg(test)]
    pub(crate) fn reserved_bytes(&self) -> u64 {
        self.overhead.iter().chain(self.held.iter()).map(ReadReservation::bytes).sum::<u64>() + self.keyed.reserved_bytes()
    }
}

/// Each [`StoreLibraryReads::read`] measured by the counting allocator while
/// started on this thread (tests): `(bytes it reserved, its measured peak)`,
/// the peak priced as the ingress prescan prices allocations.
#[cfg(test)]
pub(crate) mod library_read_probe {
    use std::cell::RefCell;

    thread_local! {
        static READS: RefCell<Option<Vec<(u64, u64)>>> = const { RefCell::new(None) };
    }

    pub(crate) fn start() {
        READS.with(|reads| *reads.borrow_mut() = Some(Vec::new()));
    }

    pub(crate) fn take() -> Vec<(u64, u64)> {
        READS.with(|reads| reads.borrow_mut().take().unwrap_or_default())
    }

    pub(super) fn active() -> bool {
        READS.with(|reads| reads.borrow().is_some())
    }

    thread_local! {
        static SAVE_PEAKS: RefCell<Vec<u64>> = const { RefCell::new(Vec::new()) };
    }

    /// The most a save's memory held at once, including its ingress lease
    /// (`PoolReadMemory::peak_reserved`), recorded by the saves as they return.
    pub(crate) fn record_save_peak(peak_reserved: u64) {
        SAVE_PEAKS.with(|peaks| peaks.borrow_mut().push(peak_reserved));
    }

    pub(crate) fn take_save_peaks() -> Vec<u64> {
        SAVE_PEAKS.with(|peaks| std::mem::take(&mut *peaks.borrow_mut()))
    }

    pub(super) fn record(reserved: u64, peak_priced: u64) {
        READS.with(|reads| {
            if let Some(reads) = reads.borrow_mut().as_mut() {
                reads.push((reserved, peak_priced));
            }
        });
    }

    /// A representative referenced library: the committed invented material
    /// library fixture (`fixtures/material/invented_material_library_valid.json`)
    /// with its dispositions made public and reviewed, so it saves, under the
    /// id `library:invented-materials` and with its first record `steel`, which
    /// the every-slot model fixture references.
    pub(crate) fn representative_material_library() -> serde_json::Value {
        use serde_json::{json, Value};
        fn reviewed_public(value: &mut Value) {
            match value {
                Value::Object(object) => {
                    for (key, value) in object.iter_mut() {
                        match key.as_str() {
                            "redistribution_status" => *value = json!("public_permissive"),
                            "review_status" => *value = json!("accepted"),
                            _ => reviewed_public(value),
                        }
                    }
                }
                Value::Array(items) => items.iter_mut().for_each(reviewed_public),
                _ => {}
            }
        }
        let mut library: Value = serde_json::from_str(include_str!("../../../../fixtures/material/invented_material_library_valid.json")).unwrap();
        reviewed_public(&mut library);
        library["material_library"]["library_id"] = json!("library:invented-materials");
        library["material_records"][0]["material_id"] = json!("steel");
        library
    }

    /// #1641 Codex r2 P2-a's shape: [`representative_material_library`] with
    /// `depth` nested objects under its wrapper, each under a key of
    /// `key_length` bytes, holding no disposition member (invented, public).
    pub(crate) fn nested_long_key_library(depth: usize, key_length: usize) -> serde_json::Value {
        let mut library = representative_material_library();
        let mut tail = serde_json::json!({ "note": "invented" });
        for level in 0..depth {
            tail = serde_json::json!({ format!("{level:03}{}", "k".repeat(key_length)): tail });
        }
        library["material_library"]["invented_notes"] = tail;
        library
    }

    /// #1641 Codex r2 P2-b's shape: [`representative_material_library`]'s
    /// wrapper with `records` records `steel`, each `rejected`, so W-6a's
    /// screening refuses with one diagnostic per record.
    pub(crate) fn rejected_records_library(records: usize) -> serde_json::Value {
        let mut library = representative_material_library();
        library["material_records"] = serde_json::Value::Array(vec![serde_json::json!({ "material_id": "steel", "review_status": "rejected" }); records]);
        library
    }

    /// [`representative_material_library`] with `records` records: its record
    /// repeated under new ids (`steel` first), about 3 KB each.
    pub(crate) fn large_material_library(records: usize) -> serde_json::Value {
        let mut library = representative_material_library();
        let record = library["material_records"][0].clone();
        let mut all = vec![record.clone()];
        for index in 1..records {
            let mut copy = record.clone();
            copy["material_id"] = serde_json::json!(format!("mat.invented.r{index:05}"));
            all.push(copy);
        }
        library["material_records"] = serde_json::Value::Array(all);
        library
    }
}

// ---------------------------------------------------------------------------
// The screening's memory (Codex r4 P1). The shared disposition screener
// (`export_disposition_screen::screen_dispositions_in`, also Save Model
// Document's, the handoff's and the report package's) is not metered: it copies
// a path for every site it records, builds a diagnostic for every finding and
// unresolved reference, and, for a hanger library, a path for every object in
// it. Its allocations are bounded here from the parsed model and each parsed
// library, by allocation-free walks that follow the screener's own walks
// (`walk_document`, `collect_objects`), and the bound is reserved before the
// screener builds anything: the model's before the screening starts, each
// library's in its lookup, before the screener receives it. Every bound is
// held until the outcome is serialized, since the diagnostics are the
// response. The screening itself is unchanged and complete. The
// counting-allocator tests check the bound against the screener's measured
// peak for the shapes Codex reported.
// ---------------------------------------------------------------------------

/// One string of `length` bytes the screener builds: a `format!` grows to at
/// most twice its length (`to_string` is exact), priced by its envelope.
fn string_bytes(length: u64) -> u64 {
    stored_read::exact_text_bytes(length.saturating_mul(2).saturating_add(16))
}

/// One element of `size` bytes in a growing vector: twice its share of the
/// doubled capacity, so the old buffer of a reallocation is covered too. Linear,
/// so the shares add across vectors.
fn slot_bytes(size: u64) -> u64 {
    size.saturating_mul(8)
}

/// A growing vector's first capacity (four elements), for the largest element
/// the screener keeps (a diagnostic).
const VECTOR_BASE_BYTES: u64 = 16 * DIAGNOSTIC_SIZE;
/// A diagnostic: three `String`s.
const DIAGNOSTIC_SIZE: u64 = 72;
/// A recorded reference: its path and record id (`(String, Option<String>)`).
const REFERENCE_SIZE: u64 = 48;
/// A hanger object collected with its path (`(String, &Map)`).
const COLLECTED_SIZE: u64 = 32;
/// A string in a vector (`String`).
const STRING_SIZE: u64 = 24;
/// One `BTreeMap` node per distinct reference key, and its key's transients.
const KEY_NODE_BYTES: u64 = 2048;
/// A diagnostic's fixed text (the screener's longest is under 100 bytes).
const MESSAGE_FIXED_BYTES: u64 = 256;
/// One finding (`member value`, the longest `provenance redistribution_status
/// protected_suspected` at 52 bytes).
const FINDING_BYTES: u64 = 64;
/// The joined findings of one diagnostic: at most the eight distinct findings
/// a hanger's deduplicated list can hold.
const FINDINGS_JOIN_BYTES: u64 = 512;
/// What a site's diagnostic names it by (`provenance_record`, `stored library
/// record`, …).
const SITE_WHAT_BYTES: u64 = 32;
/// A library lookup's refusal, quoted once in the store-failure diagnostic.
const STORE_ERROR_BYTES: u64 = 8192;

fn decimal_digits(index: usize) -> u64 {
    index.checked_ilog10().map_or(1, |digits| u64::from(digits) + 1)
}

fn usize_bytes(length: usize) -> u64 {
    u64::try_from(length).unwrap_or(u64::MAX)
}

/// A diagnostic at a path of `path` bytes whose message quotes `quoted` bytes.
fn diagnostic_bytes(path: u64, quoted: u64) -> u64 {
    stored_read::total(&[
        string_bytes(40),
        string_bytes(path),
        string_bytes(quoted.saturating_add(MESSAGE_FIXED_BYTES)),
        slot_bytes(DIAGNOSTIC_SIZE),
    ])
}

/// One screening of a record (`blocking_dispositions` or a hanger's
/// `stored_item_blocking`): `count` findings in their vector, sorted (its
/// scratch), and their join.
fn findings_bytes(count: u64) -> u64 {
    stored_read::total(&[
        count.saturating_mul(string_bytes(FINDING_BYTES).saturating_add(slot_bytes(STRING_SIZE).saturating_mul(2))),
        string_bytes(FINDINGS_JOIN_BYTES),
        VECTOR_BASE_BYTES,
    ])
}

/// A text member of a `library_ref` as the screener reads it
/// (`library_ref_identity`: a non-empty string, trimmed, non-empty).
fn identity_member(reference: &serde_json::Map<String, Value>, key: &str) -> Option<u64> {
    reference.get(key).and_then(Value::as_str).filter(|text| !text.is_empty()).map(str::trim).filter(|text| !text.is_empty()).map(|text| usize_bytes(text.len()))
}

/// The bound of the screener's allocations over the model document: what it
/// keeps to the end (diagnostics, recorded references, keys) and its largest
/// live working set (the walk's path stack, one site's findings, one library
/// step's names), with the longest `library_ref` path and identity, which bound
/// what each library's screening adds.
#[derive(Debug, Default)]
struct ScreeningBound {
    retained: u64,
    walk_peak: u64,
    library_step: u64,
    project_id: u64,
    path_max: u64,
    identity_max: u64,
}

impl ScreeningBound {
    /// Follows `walk_document` (dotted paths from the root `""`).
    fn of(document: &Value) -> Self {
        let mut bound = Self::default();
        bound.project_id = document.pointer("/project/id").and_then(Value::as_str).map_or(0, |id| usize_bytes(id.trim().len()));
        bound.walk(document, 0, 0);
        bound
    }

    fn bytes(&self) -> u64 {
        stored_read::total(&[
            self.retained,
            self.walk_peak,
            self.library_step,
            string_bytes(self.project_id),
            VECTOR_BASE_BYTES.saturating_mul(4),
            // A library lookup's refusal, quoted once (the screening then stops).
            diagnostic_bytes(self.path_max, self.identity_max.saturating_add(STORE_ERROR_BYTES)),
        ])
    }

    /// `stack` is the path strings live on entry (this value's own included):
    /// it is recorded here for every value, an array item, a scalar or an
    /// empty container as much as an object member (Codex r5 P1).
    fn walk(&mut self, value: &Value, path: u64, stack: u64) {
        self.walk_peak = self.walk_peak.max(stack);
        match value {
            Value::Array(items) => {
                for (index, item) in items.iter().enumerate() {
                    let at = path.saturating_add(2).saturating_add(decimal_digits(index));
                    self.walk(item, at, stack.saturating_add(string_bytes(at)));
                }
            }
            Value::Object(object) => {
                let owner = object.get("id").and_then(Value::as_str).filter(|id| !id.is_empty()).map(|id| usize_bytes(id.len()));
                for (key, child) in object {
                    let at = if path == 0 { usize_bytes(key.len()) } else { path.saturating_add(1).saturating_add(usize_bytes(key.len())) };
                    let stack = stack.saturating_add(string_bytes(at));
                    self.walk_peak = self.walk_peak.max(stack);
                    match (key.as_str(), child) {
                        ("provenance_record", Value::Object(_)) => {
                            let name = owner.unwrap_or(if path == 0 { 1 } else { path });
                            self.site(stack.saturating_add(string_bytes(name)), at, name);
                        }
                        ("rule_pack_refs", Value::Array(entries)) => {
                            for (index, entry) in entries.iter().enumerate() {
                                let entry_at = at.saturating_add(2).saturating_add(decimal_digits(index));
                                match entry.as_object() {
                                    Some(entry) => {
                                        let name = entry.get("rule_pack_id").and_then(Value::as_str).filter(|id| !id.is_empty()).map_or(entry_at, |id| usize_bytes(id.len()));
                                        self.site(stack.saturating_add(string_bytes(entry_at)).saturating_add(string_bytes(name)), entry_at, name);
                                    }
                                    None => self.not_an_object(stack.saturating_add(string_bytes(entry_at)), entry_at),
                                }
                            }
                        }
                        ("library_ref", Value::Object(reference)) => self.library_ref(reference, at),
                        // W-17 object form: a screened record that is not an
                        // object (`rule_pack_refs`: not a list) is refused by name.
                        ("provenance_record" | "rule_pack_refs" | "library_ref", _) => self.not_an_object(stack, at),
                        _ => {}
                    }
                    self.walk(child, at, stack);
                }
            }
            _ => {}
        }
    }

    /// A provenance record or rule-pack reference: its findings while it is
    /// screened, and its diagnostic if it blocks.
    fn site(&mut self, live: u64, at: u64, name: u64) {
        self.walk_peak = self.walk_peak.max(live.saturating_add(findings_bytes(2)));
        self.retained = self.retained.saturating_add(diagnostic_bytes(at, name.saturating_add(SITE_WHAT_BYTES).saturating_add(FINDINGS_JOIN_BYTES)));
    }

    /// A screened record that is not an object: its refusal's diagnostic,
    /// built while the walk holds `live`.
    fn not_an_object(&mut self, live: u64, at: u64) {
        self.walk_peak = self.walk_peak.max(live.saturating_add(string_bytes(SITE_WHAT_BYTES)));
        self.retained = self.retained.saturating_add(diagnostic_bytes(at, SITE_WHAT_BYTES));
    }

    /// A `library_ref` with an identity: its key, its recorded path and record
    /// id, and, in the library step, its unresolved-reference warning, its
    /// screened record id and the names built for it.
    fn library_ref(&mut self, reference: &serde_json::Map<String, Value>, at: u64) {
        let (Some(kind), Some(id)) = (identity_member(reference, "library_kind"), identity_member(reference, "library_id")) else { return };
        let record = identity_member(reference, "record_id").unwrap_or(0);
        let identity = kind.saturating_add(id).saturating_add(record);
        self.retained = stored_read::total(&[
            self.retained,
            string_bytes(kind),
            string_bytes(id),
            string_bytes(at),
            string_bytes(record),
            slot_bytes(REFERENCE_SIZE),
            KEY_NODE_BYTES,
            diagnostic_bytes(at, identity),
            string_bytes(record),
            slot_bytes(STRING_SIZE),
        ]);
        let step = stored_read::total(&[string_bytes(kind.saturating_add(id).saturating_add(1)), string_bytes(kind.saturating_add(8)), string_bytes(identity.saturating_add(2)), VECTOR_BASE_BYTES]);
        self.library_step = self.library_step.max(step);
        self.path_max = self.path_max.max(at);
        self.identity_max = self.identity_max.max(identity);
    }
}

/// A hanger library's objects as `collect_objects` collects them (paths from
/// `$`), over the whole stored document: every object the screener collects,
/// from the library object or from a record, is in it, at a path no longer.
#[derive(Debug, Default)]
struct Collected {
    objects: u64,
    retained: u64,
    peak: u64,
}

impl Collected {
    fn object(&mut self, object: &serde_json::Map<String, Value>, path: u64, stack: u64) {
        self.objects = self.objects.saturating_add(1);
        self.retained = self.retained.saturating_add(string_bytes(path)).saturating_add(slot_bytes(COLLECTED_SIZE));
        for (key, value) in object {
            let at = path.saturating_add(1).saturating_add(usize_bytes(key.len()));
            self.value(value, at, stack.saturating_add(string_bytes(at)));
        }
    }

    fn value(&mut self, value: &Value, path: u64, stack: u64) {
        self.peak = self.peak.max(stack);
        match value {
            Value::Object(object) => self.object(object, path, stack),
            Value::Array(items) => {
                for (index, item) in items.iter().enumerate() {
                    let at = path.saturating_add(2).saturating_add(decimal_digits(index));
                    self.value(item, at, stack.saturating_add(string_bytes(at)));
                }
            }
            _ => {}
        }
    }
}

/// What screening one stored library can add to the model's bound: one record
/// screening at a time (a hanger's collected objects and their findings, and a
/// copy of a conflicting provenance, at most the library's parse `parse`), the
/// matching records' list, and a diagnostic for the library and for each
/// record, at the model's longest reference path, quoting its longest identity.
fn library_screening_bytes(library_kind: &str, library: &Value, parse: u64, references: &ScreeningBound) -> u64 {
    let records = crate::library_records_key(library_kind)
        .and_then(|(records_key, _)| library.get(records_key))
        .and_then(Value::as_array)
        .map_or(0, |records| usize_bytes(records.len()));
    let step = match (library_kind, library.as_object()) {
        ("hanger", Some(object)) => {
            let mut collected = Collected::default();
            collected.object(object, 1, 0);
            stored_read::total(&[collected.retained, collected.peak, findings_bytes(collected.objects.saturating_mul(4)), parse])
        }
        _ => findings_bytes(2),
    };
    let quoted = references.identity_max.saturating_add(SITE_WHAT_BYTES).saturating_add(FINDINGS_JOIN_BYTES).saturating_add(2);
    stored_read::total(&[
        step,
        VECTOR_BASE_BYTES.saturating_add(slot_bytes(8).saturating_mul(records)),
        records.saturating_add(1).saturating_mul(diagnostic_bytes(references.path_max, quoted)),
    ])
}

/// What the handoff's structural screens of one stored library build
/// (`handoff_package::library_refs`: `screen_value` and `malformed_values`,
/// #1641 Codex r2 P2-a), from their allocation-free count
/// (`export_disposition_screen::structural_bound`): the walks' one pointer
/// buffer at its longest (growing, one walk at a time); for each walk, every
/// finding built at its exact length, their vector, the repeats' check (a
/// flag and a borrowed key per finding) and the deduplicated vector; and the
/// refusal quoting them, held to the end: the joined findings, the message
/// built around them and its diagnostic, at the model's longest reference
/// path, quoting its longest identity.
fn structural_screening_bytes(bound: &crate::export_disposition_screen::StructuralBound, references: &ScreeningBound) -> u64 {
    let walk = |found: crate::export_disposition_screen::FindingCount| {
        let joined = found.text_bytes.saturating_add(found.count.saturating_mul(2));
        stored_read::total(&[
            found.text_bytes.saturating_mul(2).saturating_add(found.count.saturating_mul(16)),
            found.count.saturating_mul(slot_bytes(STRING_SIZE).saturating_mul(2).saturating_add(slot_bytes(16)).saturating_add(slot_bytes(1))),
            stored_read::exact_text_bytes(joined),
            diagnostic_bytes(references.path_max, stored_read::total(&[joined, references.identity_max, MESSAGE_FIXED_BYTES])),
            VECTOR_BASE_BYTES.saturating_mul(2),
        ])
    };
    stored_read::total(&[stored_read::text_copy_bytes(bound.pointer_max), walk(bound.blocking), walk(bound.malformed)])
}

/// Reads the stored text of a project's document (`state_id` absent) or of one
/// of its saved states from the store at `store`, opened read-only, and admits
/// it for writing unchanged, within `memory`. The reservations the admitted
/// document and its screening's diagnostics live under (the text's load, the
/// screening bounds) are pushed to `held`, which the caller holds until the
/// outcome is serialized; a refusal's diagnostics live under them too.
pub(crate) fn admit_stored_model_document(
    store: &Path,
    project_id: &str,
    state_id: Option<&str>,
    memory: &dyn ReadMemory,
    held: &mut Vec<ReadReservation>,
) -> Result<AdmittedModelDocument, Refusal> {
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
    held.push(memory.reserve(&what, stored_read::loaded_text_bytes(length)).map_err(unavailable)?);
    let text: String = query_stored(&transaction, &stored_text_sql(state_id, ""), project_id, state_id)
        .map_err(|error| unavailable(stored_read::sql_error(&error)))?
        .ok_or_else(|| refusal(NOT_FOUND, format!("The local project store holds no {named}; nothing was written.")))?;
    let not_json = |detail: String| refusal(NOT_JSON, format!("The stored document of {named} is not JSON ({detail}); nothing was written."));
    let parse = stored_read::parse_bytes(&text).map_err(not_json)?;
    let parsing = memory.reserve(&what, parse).map_err(unavailable)?;
    let document: Value = serde_json::from_str(&text).map_err(|error| not_json(stored_read::json_error(&error)))?;
    refuse_duplicate_members(memory, &what, &named, &text, &document)?;
    // The screener's allocations over the model, reserved before it runs
    // (Codex r4 P1).
    let references = ScreeningBound::of(&document);
    held.push(memory.reserve(&format!("the screening of {what}"), references.bytes()).map_err(unavailable)?);
    // Libraries the document references are read from the same read-only
    // transaction, each within the export's memory with its screening's bound.
    let mut libraries = LibraryReads::new(&transaction, memory, &references);
    let screened = screen_model_document_dispositions(&document, &mut |project_id: &str, library_kind: &str, library_id: &str| {
        libraries.read(project_id, library_kind, library_id)
    });
    let refused = libraries.refused.take();
    held.append(&mut libraries.held);
    held.append(&mut std::mem::take(&mut libraries.keyed).into_held());
    if let Some(error) = refused {
        return Err(read_refusal(&named, error));
    }
    let warnings = screened?;
    let suggested_file_name = suggested_model_document_file_name(&document);
    drop(document);
    drop(parsing);
    Ok(AdmittedModelDocument { text, suggested_file_name, warnings })
}

/// The command after its ingress (`lib.rs` `save_stored_model_document_file`):
/// the explicit intent, the store's path (`store`, located only after the
/// intent), then [`export_stored_model_document`] under the command's ingress
/// lease `ingress`. `respond` receives the outcome sent and its serialized
/// text while every reservation is still held (the command builds its
/// response there).
pub(crate) fn export_stored_model_document_request<R>(
    request: &StoredExportRequest,
    store: impl FnOnce() -> Result<PathBuf, String>,
    registry: &SolveJobRegistry,
    ingress: &Lease,
    choose: impl FnOnce(&str) -> Result<Option<PathBuf>, String>,
    respond: impl FnOnce(&ModelDocumentSaveOutcome, String) -> R,
) -> R {
    let state_id = request.state_id.as_deref();
    let outcome = if !request.explicit_local_private_intent {
        ModelDocumentSaveOutcome::refused_one(INTENT_REQUIRED, "Saving the stored document needs the explicit choice to save it, including private project data, to a file.")
    } else {
        match store() {
            Ok(store) => return export_stored_model_document(&store, &request.project_id, state_id, registry, ingress, choose, respond),
            Err(error) => {
                let named = stored_document_name(&request.project_id, state_id);
                ModelDocumentSaveOutcome::refused_one(
                    STORE_UNAVAILABLE,
                    format!("The local project store could not be located for {named} ({}); nothing was written.", stored_read::capped(error)),
                )
            }
        }
    };
    // These refusals quote nothing unbounded (ids are cut and the error is
    // capped as messages quote them): within the ingress lease's error allowance.
    let text = serde_json::to_string(&outcome).unwrap_or_default();
    respond(&outcome, text)
}

/// What a Saved outcome adds to its warnings' JSON: its tag, the file name (at
/// most 255 bytes, each escaped to at most six), the byte count, the hash, the
/// durability and the member names.
const SAVED_FIELDS_BYTES: u64 = 4096;

/// An outcome a command serializes within its memory ([`serialize_outcome`]),
/// with the short form it sends when its whole response is refused.
pub(crate) trait SentOutcome: serde::Serialize + Sized {
    /// The outcome with its diagnostics (a refusal's) or warnings (a Saved
    /// outcome's) bounded by [`bounded_diagnostics`], and how many it listed.
    fn bounded(self) -> (Self, usize);

    /// The bounded outcome sent instead when the pool refuses the whole
    /// response (`refusal`): a refusal's first diagnostic and count, or a
    /// Saved outcome without its warnings, `noun` naming the command (`export`,
    /// `save`), `listed` the diagnostics or warnings the outcome listed. Quoted
    /// texts are capped.
    fn shortened(self, refusal: String, noun: &str, listed: usize) -> Self;
}

/// The most diagnostics (or warnings) a response of Save Model Document, Save
/// Handoff or the recovery export names (#1641 Codex r3): the desktop shows at
/// most eight; the rest are counted by one final diagnostic.
pub(crate) const MAX_NAMED_DIAGNOSTICS: usize = 64;
/// The most bytes of a named diagnostic's path or message; a longer one is cut
/// on a character boundary and ends `…`.
pub(crate) const MAX_DIAGNOSTIC_TEXT_BYTES: usize = 2048;
/// The final diagnostic that counts those not named.
pub(crate) const DIAGNOSTICS_NOT_SHOWN: &str = "SAVE-DIAGNOSTICS-NOT-SHOWN";
/// One named diagnostic's JSON at its longest: `{"code":"","path":"","message":""}`
/// (34 bytes), a code under 64 bytes, and a path and a message of
/// [`MAX_DIAGNOSTIC_TEXT_BYTES`] each escaped to at most six bytes per byte.
const DIAGNOSTIC_JSON_BYTES: u64 = 34 + 64 + 2 * 6 * MAX_DIAGNOSTIC_TEXT_BYTES as u64;
/// The longest response of these commands, in bytes (1,636,642): 65
/// diagnostics of [`DIAGNOSTIC_JSON_BYTES`], their commas, and 32 KiB for the outcome's
/// other members (a file or package name of at most 255 bytes, a basis state
/// id of at most `stored_read::MAX_ID_BYTES`, each escaped, the hashes and
/// the durability). A longer response is never sent: its short form is
/// ([`serialize_outcome`]). The transport's copy of it (Wry's `NSData` on
/// macOS) is at most as long again.
pub(crate) const MAX_RESPONSE_BYTES: u64 = (MAX_NAMED_DIAGNOSTICS as u64 + 1) * DIAGNOSTIC_JSON_BYTES + MAX_NAMED_DIAGNOSTICS as u64 + 32 * 1024;

/// `text` cut to at most `max` bytes on a character boundary, ending `…`.
fn cut_text(text: String, max: usize) -> String {
    if text.len() <= max {
        return text;
    }
    let mut cut = max - '…'.len_utf8();
    while !text.is_char_boundary(cut) {
        cut -= 1;
    }
    format!("{}…", &text[..cut])
}

/// `diagnostics` as a response names them: the first
/// [`MAX_NAMED_DIAGNOSTICS`], in order, each path and message cut to
/// [`MAX_DIAGNOSTIC_TEXT_BYTES`], then, when more were listed, one
/// [`DIAGNOSTICS_NOT_SHOWN`] diagnostic counting them.
pub(crate) fn bounded_diagnostics(diagnostics: Vec<ModelDocumentSaveDiagnostic>) -> Vec<ModelDocumentSaveDiagnostic> {
    let listed = diagnostics.len();
    let mut named: Vec<ModelDocumentSaveDiagnostic> = diagnostics
        .into_iter()
        .take(MAX_NAMED_DIAGNOSTICS)
        .map(|item| ModelDocumentSaveDiagnostic {
            code: item.code,
            path: cut_text(item.path, MAX_DIAGNOSTIC_TEXT_BYTES),
            message: cut_text(item.message, MAX_DIAGNOSTIC_TEXT_BYTES),
        })
        .collect();
    if listed > MAX_NAMED_DIAGNOSTICS {
        let more = listed - MAX_NAMED_DIAGNOSTICS;
        named.push(diagnostic(DIAGNOSTICS_NOT_SHOWN, "", format!("{more} more of the {listed} diagnostics were not shown; the first {MAX_NAMED_DIAGNOSTICS} are.")));
    }
    named
}

impl SentOutcome for ModelDocumentSaveOutcome {
    fn bounded(self) -> (Self, usize) {
        match self {
            ModelDocumentSaveOutcome::Saved { file_name, byte_count, model_hash, replaced_existing, durability, warnings } => {
                let listed = warnings.len();
                (ModelDocumentSaveOutcome::Saved { file_name, byte_count, model_hash, replaced_existing, durability, warnings: bounded_diagnostics(warnings) }, listed)
            }
            ModelDocumentSaveOutcome::Refused { diagnostics } => {
                let listed = diagnostics.len();
                (ModelDocumentSaveOutcome::Refused { diagnostics: bounded_diagnostics(diagnostics) }, listed)
            }
            ModelDocumentSaveOutcome::Cancelled => (ModelDocumentSaveOutcome::Cancelled, 0),
        }
    }

    fn shortened(self, refusal: String, noun: &str, listed: usize) -> Self {
        match self {
            ModelDocumentSaveOutcome::Saved { file_name, byte_count, model_hash, replaced_existing, durability, .. } => ModelDocumentSaveOutcome::Saved {
                file_name,
                byte_count,
                model_hash,
                replaced_existing,
                durability,
                warnings: vec![diagnostic(
                    MEMORY_UNAVAILABLE,
                    "",
                    format!("{} The file was saved; its {listed} warnings are not shown.", stored_read::capped(refusal)),
                )],
            },
            ModelDocumentSaveOutcome::Refused { diagnostics } => ModelDocumentSaveOutcome::refused_one(MEMORY_UNAVAILABLE, shortened_refusal(refusal, noun, listed, &diagnostics)),
            ModelDocumentSaveOutcome::Cancelled => ModelDocumentSaveOutcome::Cancelled,
        }
    }
}

/// A refusal whose diagnostics are not shown, named by the response's refusal,
/// their count (`listed`) and the first of them (capped).
pub(crate) fn shortened_refusal(refusal: String, noun: &str, listed: usize, diagnostics: &[ModelDocumentSaveDiagnostic]) -> String {
    let first = diagnostics.first().map_or(String::new(), |item| format!(" The first is {}: {}", item.code, stored_read::capped(item.message.clone())));
    format!("{} The {noun} was refused with {listed} diagnostics, which are not shown.{first}", stored_read::capped(refusal))
}

/// The outcome's text (Codex r4 P2), serialized within `memory` while the
/// outcome's reservations are held: its exact length is counted first, and a
/// buffer of that capacity is written under `reserved` (a bound reserved
/// earlier) when it fits, else under a new reservation. If the pool refuses
/// it, the outcome's short form is sent instead ([`SentOutcome::shortened`]).
/// The outcome is first bounded ([`SentOutcome::bounded`]), so what outlives
/// the reservations, the response and its transport copy, is at most
/// [`MAX_RESPONSE_BYTES`] each.
/// `what` names the response ("the response to {what}"), and `noun` the
/// command in the short form. Returns the outcome
/// sent, its text and the buffer's reservation, which the caller holds until
/// the response exists. Shared by the recovery export and by Save Model
/// Document and Save Handoff (#1641 Codex r2 P2-b).
pub(crate) fn serialize_outcome<O: SentOutcome>(
    memory: &dyn ReadMemory,
    what: &str,
    noun: &str,
    outcome: O,
    reserved: Option<(ReadReservation, u64)>,
) -> (O, String, Option<ReadReservation>) {
    let write = |outcome: &O, capacity: u64| -> Option<String> {
        let mut buffer = Vec::with_capacity(usize::try_from(capacity).ok()?);
        serde_json::to_writer(&mut buffer, outcome).ok()?;
        String::from_utf8(buffer).ok()
    };
    // At most MAX_RESPONSE_BYTES outlives the reservations (#1641 Codex r3).
    let (outcome, listed) = outcome.bounded();
    let length = stored_read::serialized_length(&outcome).unwrap_or(u64::MAX);
    if length > MAX_RESPONSE_BYTES {
        let refusal = format!("{MEMORY_UNAVAILABLE}: the response to {what} would be {length} bytes, above the {MAX_RESPONSE_BYTES} a response may hold.");
        let sent = outcome.shortened(refusal, noun, listed);
        let text = serde_json::to_string(&sent).unwrap_or_default();
        return (sent, text, None);
    }
    let buffer = match reserved {
        Some((reservation, bound)) if length <= bound => Ok(reservation),
        _ => memory.reserve(&format!("the response to {what}"), stored_read::exact_text_bytes(length)),
    };
    let refusal = match buffer {
        Ok(buffer) => match write(&outcome, length) {
            Some(text) => return (outcome, text, Some(buffer)),
            None => format!("{MEMORY_UNAVAILABLE}: the response to {what} could not be serialized."),
        },
        Err(error) => error,
    };
    let sent = outcome.shortened(refusal, noun, listed);
    let text = serde_json::to_string(&sent).unwrap_or_default();
    (sent, text, None)
}

/// The recovery export: the store's read, the chooser (`choose`, given the
/// suggested file name), the write and the response, under one memory
/// admission. The accounting starts from the command's ingress lease
/// (`ingress`, which holds its arguments); the command's own memory
/// (connection, statements, ids) is reserved with it as
/// `stored_read::READ_OVERHEAD_BYTES`, and every other reservation is counted
/// with both against the per-invocation budget (`PoolReadMemory`). The Saved
/// response's bound is reserved before the chooser is shown, so no response
/// is refused after a file was written. Nothing is chosen or written after a
/// refusal. `respond` receives the outcome sent and its text while every
/// reservation is still held.
pub(crate) fn export_stored_model_document<R>(
    store: &Path,
    project_id: &str,
    state_id: Option<&str>,
    registry: &SolveJobRegistry,
    ingress: &Lease,
    choose: impl FnOnce(&str) -> Result<Option<PathBuf>, String>,
    respond: impl FnOnce(&ModelDocumentSaveOutcome, String) -> R,
) -> R {
    let named = stored_document_name(project_id, state_id);
    let memory = PoolReadMemory::new(registry, ingress);
    let overhead = match memory.reserve(&format!("the stored document of {named} (the command's own memory)"), stored_read::READ_OVERHEAD_BYTES) {
        Ok(reservation) => reservation,
        Err(error) => {
            // Bounded (the ids are cut as messages quote them): within the
            // ingress lease's error allowance.
            let outcome = ModelDocumentSaveOutcome::refused_one(MEMORY_UNAVAILABLE, format!("{error} Nothing was read or written."));
            let text = serde_json::to_string(&outcome).unwrap_or_default();
            return respond(&outcome, text);
        }
    };
    let mut held = Vec::new();
    let (outcome, reserved) = export_outcome(store, project_id, state_id, &memory, &named, &mut held, choose);
    let (sent, text, buffer) = serialize_outcome(&memory, &format!("the export of {named}"), "export", outcome, reserved);
    let response = respond(&sent, text);
    drop(sent);
    drop(buffer);
    drop(held);
    drop(overhead);
    response
}

/// The export's outcome, and the Saved response's bound reserved before the
/// chooser (with its bytes), when the read was admitted.
fn export_outcome(
    store: &Path,
    project_id: &str,
    state_id: Option<&str>,
    memory: &dyn ReadMemory,
    named: &str,
    held: &mut Vec<ReadReservation>,
    choose: impl FnOnce(&str) -> Result<Option<PathBuf>, String>,
) -> (ModelDocumentSaveOutcome, Option<(ReadReservation, u64)>) {
    let admitted = match admit_stored_model_document(store, project_id, state_id, memory, held) {
        Ok(admitted) => admitted,
        Err(diagnostics) => return (ModelDocumentSaveOutcome::refused(diagnostics), None),
    };
    let bound = stored_read::serialized_length(&admitted.warnings).unwrap_or(u64::MAX).saturating_add(SAVED_FIELDS_BYTES);
    let response = match memory.reserve(&format!("the response to the export of {named}"), stored_read::exact_text_bytes(bound)) {
        Ok(response) => response,
        Err(error) => return (ModelDocumentSaveOutcome::refused_one(MEMORY_UNAVAILABLE, format!("{error} Nothing was written.")), None),
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
    use crate::command_ingress::test_support::{headers, registry};
    use crate::command_ingress::{admit_body_within, ArgsDecoding, INGRESS_ARGS_JSON_V1};
    use crate::memory_pool::test_host::ScriptedHost;
    use crate::export_disposition_screen::DISPOSITION_RECORD_MALFORMED;
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
            "schema_version": "0.8.0",
            "document_kind": "openpipestress.model",
            "project": { "id": PROJECT, "name": "Invented", "description": "" },
            "materials": [{ "id": "material:invented", "provenance_record": { "review_status": "accepted" } }]
        });
        serde_json::to_string(&document).unwrap()
    }

    /// Key order and spacing that a re-serialization would change.
    const STATE_TEXT: &str = "{\"schema_version\":\"0.8.0\",\"project\":{\"name\":\"Invented\",\"id\":\"project:invented-no-units\"}, \"x\":1.50}";

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
        export_stored_model_document_request(&request, || Ok(store.to_path_buf()), pool, &lease, choose, |outcome, text| {
            assert_eq!(serde_json::to_string(outcome).unwrap(), text, "the response is the outcome sent");
            outcome.clone()
        })
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
        let duplicated = "{\"schema_version\":\"0.8.0\",\"document_kind\":\"openpipestress.model\",\"project\":{\"id\":\"project:invented-no-units\",\"name\":\"Invented\"},\"materials\":[{\"id\":\"material:invented\",\"provenance_record\":{\"review_status\":\"quarantined\"},\"provenance_record\":{\"review_status\":\"accepted\"}}]}";
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
        let outcome = export_stored_model_document_request(&request, || { located.set(true); Ok(store.clone()) }, &pool, &lease, |_| panic!("no chooser"), |outcome, _| outcome.clone());
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

    // -----------------------------------------------------------------------
    // Codex r4: the screener's allocations and the response.
    // -----------------------------------------------------------------------

    const HANGERS: &str = "library:invented-hangers";

    /// Codex r4 shape 1: a ~150-byte model referencing a ~33 KB hanger library
    /// whose 160 objects sit under one 32 KiB key, so the screener collects 160
    /// paths of 32 KiB (`collect_objects`).
    fn hanger_shape() -> (String, String) {
        let model = format!(
            "{{\"project\":{{\"id\":\"{PROJECT}\"}},\"hangers\":[{{\"id\":\"h\",\"library_ref\":{{\"library_kind\":\"hanger\",\"library_id\":\"{HANGERS}\"}}}}]}}"
        );
        let objects = vec![json!({ "k": 1 }); 160];
        let library = serde_json::to_string(&json!({ "hanger_library": { "library_id": HANGERS, "k".repeat(32 * 1024): objects } })).unwrap();
        (model, library)
    }

    /// Codex r4 shape 2: a ~42 KB model whose 150 unresolved `library_ref`s sit
    /// under one 32 KiB key, so each recorded reference and each warning copies
    /// a 32 KiB path, and the warnings make a ~5 MB response.
    fn unresolved_shape() -> String {
        let reference = json!({ "library_ref": { "library_kind": "material", "library_id": "library:invented-missing" } });
        serde_json::to_string(&json!({ "project": { "id": PROJECT }, "k".repeat(32 * 1024): vec![reference; 150] })).unwrap()
    }

    /// Codex r5 shape: a 32 KiB key, then 120 nested containers around `null`
    /// (arrays, objects, or alternating), so the screener's walk holds 120
    /// paths of 32 KiB at once at the deepest value, which is not an object
    /// member when the tail ends in arrays.
    fn nested_tail_shape(kind: &str) -> String {
        let mut tail = Value::Null;
        for level in 0..120 {
            let array = match kind {
                "arrays" => true,
                "objects" => false,
                _ => level % 2 == 0,
            };
            tail = if array { json!([tail]) } else { json!({ "a": tail }) };
        }
        serde_json::to_string(&json!({ "project": { "id": PROJECT }, "k".repeat(32 * 1024): tail })).unwrap()
    }

    fn store_with_library(scratch: &Scratch, model: &str, kind: &str, id: &str, library: &str) -> PathBuf {
        let store = current_store(scratch, model, STATE_TEXT);
        Connection::open(&store)
            .unwrap()
            .execute(
                "INSERT INTO local_libraries(project_id, library_kind, library_id, document_json, created_at_unix, updated_at_unix) VALUES (?1, ?2, ?3, ?4, 1, 1)",
                params![PROJECT, kind, id, library],
            )
            .unwrap();
        store
    }

    fn pool_of(budget: u64) -> SolveJobRegistry {
        let pool = registry(ScriptedHost::normal());
        pool.configure_pool(budget, budget, false).unwrap();
        pool
    }

    fn refusal_message(outcome: &ModelDocumentSaveOutcome) -> String {
        match outcome {
            ModelDocumentSaveOutcome::Refused { diagnostics } => diagnostics[0].message.clone(),
            other => panic!("not refused: {other:?}"),
        }
    }

    /// Codex r4 P1: the screener's copied paths, collections and diagnostics
    /// are admitted before it builds them. Both reported shapes are refused by
    /// name under a pool below their peak (4 MiB and 8 MiB; Codex measured 5.07
    /// MB and 10.18 MB), write nothing and release every reservation; under an
    /// ordinary pool they export, the second with all its warnings.
    #[test]
    fn screening_allocations_are_admitted_before_the_screener_builds_them() {
        let scratch = Scratch::new("screening-hanger");
        let (model, library) = hanger_shape();
        let store = store_with_library(&scratch, &model, "hanger", HANGERS, &library);
        let destination = scratch.0.join("recovered.json");
        let small = pool_of(4 << 20);
        let shown = Cell::new(false);
        let refused = export(&store, None, &small, &destination, &shown);
        assert_refused_nothing_written(&refused, MEMORY_UNAVAILABLE, &destination, &shown);
        assert!(refusal_message(&refused).contains("the screening of the stored hanger library"), "{refused:?}");
        assert_eq!(small.pool_status().reserved_bytes, 0);
        let normal = registry(ScriptedHost::normal());
        assert!(matches!(export(&store, None, &normal, &destination, &shown), ModelDocumentSaveOutcome::Saved { .. }));
        assert_eq!(fs::read(&destination).unwrap(), model.as_bytes());
        assert_eq!(normal.pool_status().reserved_bytes, 0);

        let scratch = Scratch::new("screening-unresolved");
        let model = unresolved_shape();
        let store = current_store(&scratch, &model, STATE_TEXT);
        let destination = scratch.0.join("recovered.json");
        let small = pool_of(8 << 20);
        let shown = Cell::new(false);
        let refused = export(&store, None, &small, &destination, &shown);
        assert_refused_nothing_written(&refused, MEMORY_UNAVAILABLE, &destination, &shown);
        assert!(refusal_message(&refused).contains("the screening of the stored document"), "{refused:?}");
        assert_eq!(small.pool_status().reserved_bytes, 0);
        match export(&store, None, &normal, &destination, &shown) {
            ModelDocumentSaveOutcome::Saved { warnings, .. } => {
                // The response names the first 64 of the 150 warnings and counts the rest (#1641 Codex r3).
                assert_eq!(warnings.len(), MAX_NAMED_DIAGNOSTICS + 1);
                assert!(warnings[..MAX_NAMED_DIAGNOSTICS].iter().all(|warning| warning.code == crate::MODEL_LIBRARY_REF_UNRESOLVED));
                assert!(warnings[MAX_NAMED_DIAGNOSTICS].message.starts_with("86 more of the 150"), "{:?}", warnings[MAX_NAMED_DIAGNOSTICS]);
            }
            other => panic!("not saved: {other:?}"),
        }
        assert_eq!(normal.pool_status().reserved_bytes, 0);

        // Codex r5: a 32 KiB key, then 120 nested containers around `null`
        // (a ~33 KB model; Codex measured an 8 MB screening peak for the
        // arrays). Refused under a 2 MiB pool, before the chooser.
        for kind in ["arrays", "objects", "mixed"] {
            let scratch = Scratch::new(&format!("screening-nested-{kind}"));
            let model = nested_tail_shape(kind);
            assert!(model.len() < 34 * 1024, "{kind}: {} bytes", model.len());
            let store = current_store(&scratch, &model, STATE_TEXT);
            let destination = scratch.0.join("recovered.json");
            let small = pool_of(2 << 20);
            let shown = Cell::new(false);
            let refused = export(&store, None, &small, &destination, &shown);
            assert_refused_nothing_written(&refused, MEMORY_UNAVAILABLE, &destination, &shown);
            assert!(refusal_message(&refused).contains("the screening of the stored document"), "{kind}: {refused:?}");
            assert_eq!(small.pool_status().reserved_bytes, 0);
            assert!(matches!(export(&store, None, &normal, &destination, &shown), ModelDocumentSaveOutcome::Saved { .. }), "{kind}");
            assert_eq!(fs::read(&destination).unwrap(), model.as_bytes());
            assert_eq!(normal.pool_status().reserved_bytes, 0);
        }
    }

    /// The bound reserved for the screening covers what the shared screener
    /// allocates, measured by the counting allocator (requested sizes priced by
    /// their envelope), for Codex's two shapes and for the screener's other
    /// paths: blocking provenance and rule-pack sites, hanger objects with
    /// conflicting provenance, matched and unmatched record ids, and a
    /// non-hanger library.
    #[test]
    fn the_screening_bound_covers_the_screeners_measured_peak() {
        let (hanger_model, hanger_library) = hanger_shape();
        let blocked = |status: &str| json!({ "review_status": status, "redistribution_status": "rejected" });
        let deep = "d".repeat(4096);
        let busy_model = serde_json::to_string(&json!({
            "project": { "id": PROJECT },
            deep.clone(): {
                "materials": (0..40).map(|index| json!({
                    "id": format!("m{index}"),
                    "provenance_record": blocked("quarantined"),
                    "library_ref": { "library_kind": if index % 2 == 0 { "hanger" } else { "material" }, "library_id": format!("lib{}", index % 3), "record_id": format!(" r{} ", index % 5) }
                })).collect::<Vec<_>>(),
                "rule_pack_refs": (0..30).map(|index| json!({ "rule_pack_id": if index % 2 == 0 { format!("pack{index}") } else { String::new() }, "lifecycle_status": "rejected", "review_status": "rejected" })).collect::<Vec<_>>()
            }
        }))
        .unwrap();
        let hanger_records = (0..20).map(|index| json!({
            "hanger_id": format!("r{}", index % 4),
            "review_status": "accepted",
            "provenance": { "review_status": "quarantined", "redistribution_status": "protected_suspected", "note": "n".repeat(512) },
            "parts": [{ "provenance": blocked("rejected") }, { "privacy_class": "x" }]
        })).collect::<Vec<_>>();
        let busy_hanger = serde_json::to_string(&json!({ "hanger_library": { "library_id": "lib0", "review_status": "quarantined", deep.clone(): hanger_records.clone() }, "hanger_records": hanger_records })).unwrap();
        let material_records = (0..20).map(|index| json!({ "material_id": format!("r{}", index % 3), "review_status": "rejected" })).collect::<Vec<_>>();
        let busy_material = serde_json::to_string(&json!({ "material_library": { "library_id": "lib1", "redistribution_status": "rejected" }, "material_records": material_records })).unwrap();
        // W-17 object form: screened records that are not objects, records
        // whose disposition members are unreadable, and stored libraries
        // holding positional provenance or a positional library object.
        let malformed_model = serde_json::to_string(&json!({
            "project": { "id": PROJECT },
            deep.clone(): {
                "materials": (0..40).map(|index| match index % 4 {
                    0 => json!({ "id": format!("m{index}"), "provenance_record": ["s", "private_only", "quarantined"] }),
                    1 => json!({ "provenance_record": { "review_status": ["quarantined"], "redistribution_status": {}, "provenance": [] } }),
                    2 => json!({ "id": format!("m{index}"), "library_ref": ["material", "lib1", "r0"] }),
                    _ => json!({ "id": format!("m{index}"), "library_ref": { "library_kind": if index % 8 == 3 { "hanger" } else { "material" }, "library_id": if index % 8 == 3 { "lib0" } else { "lib1" }, "record_id": "r0" } }),
                }).collect::<Vec<_>>(),
                "rule_pack_refs": (0..30).map(|index| if index % 2 == 0 { json!(["pack", "rejected"]) } else { json!(null) }).collect::<Vec<_>>(),
                "nested": { "rule_pack_refs": { "rule_pack_id": "p" }, "library_ref": "text", "provenance_record": 1 },
                "components": [{ "id": "c", "library_ref": { "library_kind": "component", "library_id": "lib2", "record_id": "r0" } }]
            }
        }))
        .unwrap();
        let malformed_hanger_records = (0..20).map(|index| json!({
            "hanger_id": format!("r{index}"),
            "provenance": ["n".repeat(512), "quarantined"],
            "parts": [{ "provenance": ["rejected"] }, { "review_status": "accepted" }]
        })).collect::<Vec<_>>();
        let malformed_hanger = serde_json::to_string(&json!({ "hanger_library": { "library_id": "lib0", "provenance": [], deep.clone(): malformed_hanger_records.clone() }, "hanger_records": malformed_hanger_records })).unwrap();
        let malformed_material = serde_json::to_string(&json!({ "material_library": ["lib1", "rejected"], "material_records": (0..20).map(|_| json!({ "material_id": "r0", "provenance": ["s", "rejected"] })).collect::<Vec<_>>() })).unwrap();
        // #1651 Codex r1: unreadable members at every depth of a stored record.
        let malformed_component = serde_json::to_string(&json!({
            "component_library": { "library_id": "lib2" },
            "component_records": (0..20).map(|index| json!({
                "component_id": "r0",
                "review_status": if index % 2 == 0 { json!(["quarantined"]) } else { json!("accepted") },
                "fields": [{ "provenance": { "redistribution_status": { "status": "rejected" } }, "classification": ["x"], "privacy_class": {} }, { deep.clone(): [[{ "lifecycle_status": ["rejected"] }]] }]
            })).collect::<Vec<_>>()
        }))
        .unwrap();
        let shapes: Vec<(&str, String, Vec<(&str, &str, String)>)> = vec![
            ("malformed", malformed_model, vec![("hanger", "lib0", malformed_hanger), ("material", "lib1", malformed_material), ("component", "lib2", malformed_component)]),
            ("hanger", hanger_model, vec![("hanger", HANGERS, hanger_library)]),
            ("unresolved", unresolved_shape(), vec![]),
            ("nested arrays", nested_tail_shape("arrays"), vec![]),
            ("nested objects", nested_tail_shape("objects"), vec![]),
            ("nested mixed", nested_tail_shape("mixed"), vec![]),
            ("busy", busy_model, vec![("hanger", "lib0", busy_hanger), ("material", "lib1", busy_material)]),
        ];
        for (name, model, libraries) in shapes {
            let document: Value = serde_json::from_str(&model).unwrap();
            let references = ScreeningBound::of(&document);
            let mut bound = references.bytes();
            for (kind, id, text) in &libraries {
                let parse = stored_read::parse_bytes(text).unwrap();
                let library: Value = serde_json::from_str(text).unwrap();
                // The lookup's own parse (reserved by its read) and the screening.
                bound += parse + library_screening_bytes(kind, &library, parse, &references);
                let _ = id;
            }
            let (screened, measured) = crate::ingress::counting::measure(|| {
                screen_model_document_dispositions(&document, &mut |_: &str, kind: &str, id: &str| {
                    Ok(libraries.iter().find(|(k, i, _)| *k == kind.trim() && *i == id.trim()).map(|(_, _, text)| serde_json::from_str::<Value>(text).unwrap()))
                })
            });
            let found = match &screened {
                Ok(warnings) => warnings.len(),
                Err(blocking) => blocking.len(),
            };
            assert!(found > 0 || name == "hanger" || name.starts_with("nested"), "{name}: the screening finds something");
            if name == "malformed" {
                let refused = screened.as_ref().unwrap_err();
                assert!(refused.iter().all(|item| item.code == crate::export_disposition_screen::DISPOSITION_RECORD_MALFORMED), "{refused:?}");
                // The model's 30 materials and 30 references and its 3 nested
                // sites; the hanger library and its record; the material
                // library and its 20 matching records; the component
                // library's 20 matching records.
                assert_eq!(refused.len(), (10 + 10 + 10) + 30 + 3 + (1 + 1) + (1 + 20) + 20, "every malformed site and stored item, once");
            }
            assert!(measured.peak_priced <= bound, "{name}: the screener's peak {} exceeds its reserved bound {bound}", measured.peak_priced);
            drop(screened);
        }
    }

    /// The smallest per-invocation budget under which `export` (on a fresh
    /// pool of that size) sends an outcome `sent` accepts.
    fn smallest_budget(store: &Path, destination: &Path, sent: impl Fn(&ModelDocumentSaveOutcome) -> bool) -> u64 {
        let (mut refused, mut admitted) = (1u64 << 20, 1u64 << 30);
        assert!(sent(&export(store, None, &pool_of(admitted), destination, &Cell::new(false))));
        while admitted - refused > 1 {
            let budget = refused + (admitted - refused) / 2;
            let _ = fs::remove_file(destination);
            if sent(&export(store, None, &pool_of(budget), destination, &Cell::new(false))) { admitted = budget } else { refused = budget }
        }
        let _ = fs::remove_file(destination);
        admitted
    }

    /// Codex r4 P2: the response was serialized after every reservation was
    /// released. Its exact length is reserved with the outcome's reservations
    /// held, and a Saved response's bound before the chooser: one byte below
    /// the smallest budget that sends the whole response, the export is
    /// refused by naming the response, before the chooser, writing nothing; a
    /// refusal whose diagnostics do not fit is sent as one named refusal that
    /// quotes the first of them.
    #[test]
    fn the_response_is_reserved_before_it_is_serialized() {
        let scratch = Scratch::new("response-saved");
        let model = unresolved_shape();
        let store = current_store(&scratch, &model, STATE_TEXT);
        let destination = scratch.0.join("recovered.json");
        let budget = smallest_budget(&store, &destination, |outcome| matches!(outcome, ModelDocumentSaveOutcome::Saved { warnings, .. } if warnings.len() == MAX_NAMED_DIAGNOSTICS + 1 && warnings[MAX_NAMED_DIAGNOSTICS].message.starts_with("86 more of the 150")));
        let shown = Cell::new(false);
        let refused = export(&store, None, &pool_of(budget - 1), &destination, &shown);
        assert_refused_nothing_written(&refused, MEMORY_UNAVAILABLE, &destination, &shown);
        assert!(refusal_message(&refused).contains("the response to the export of"), "{refused:?}");

        let scratch = Scratch::new("response-refused");
        let blocked = json!({ "id": "m", "provenance_record": { "review_status": "quarantined" } });
        let model = serde_json::to_string(&json!({ "project": { "id": PROJECT }, "k".repeat(32 * 1024): vec![blocked; 150] })).unwrap();
        let store = current_store(&scratch, &model, STATE_TEXT);
        let destination = scratch.0.join("recovered.json");
        let whole = |outcome: &ModelDocumentSaveOutcome| matches!(outcome, ModelDocumentSaveOutcome::Refused { diagnostics } if diagnostics.len() == MAX_NAMED_DIAGNOSTICS + 1 && diagnostics[MAX_NAMED_DIAGNOSTICS].message.starts_with("86 more of the 150"));
        let budget = smallest_budget(&store, &destination, whole);
        let shown = Cell::new(false);
        let refused = export(&store, None, &pool_of(budget - 1), &destination, &shown);
        assert_refused_nothing_written(&refused, MEMORY_UNAVAILABLE, &destination, &shown);
        // The bounded response (#1641 Codex r3) is no longer the largest
        // reservation of this export; its own refusal is checked directly: a
        // refusal whose response the pool refuses is sent as one named
        // refusal counting every diagnostic it listed and quoting the first.
        let ModelDocumentSaveOutcome::Refused { diagnostics } = export(&store, None, &registry(ScriptedHost::normal()), &destination, &Cell::new(false)) else { panic!("not refused") };
        assert_eq!(diagnostics.len(), MAX_NAMED_DIAGNOSTICS + 1);
        let listed = ModelDocumentSaveOutcome::Refused { diagnostics: vec![diagnostics[0].clone(); 150] };
        let tiny = pool_of(1 << 20);
        let memory = PoolReadMemory::unseeded(&tiny);
        let _held = memory.reserve("everything but a byte", (1 << 20) - 1).unwrap();
        let (sent, text, buffer) = serialize_outcome(&memory, "the export of a test", "export", listed, None);
        assert!(buffer.is_none());
        assert_eq!(serde_json::to_string(&sent).unwrap(), text);
        let message = refusal_message(&sent);
        assert!(message.contains("the response to the export of"), "{message}");
        assert!(message.contains(&format!("refused with 150 diagnostics, which are not shown. The first is {PROTECTED_CONTENT_BLOCKED}")), "{message}");
    }

    // -----------------------------------------------------------------------
    // #1641 Codex r1: the save-side lookup's key pricing and SQLite's memory.
    // -----------------------------------------------------------------------

    /// A store opened as the saves open it (`open_project_store`), holding
    /// `rows` small libraries of project `PROJECT` besides `lib0` and `lib1`.
    fn save_store(scratch: &Scratch, rows: usize) -> PathBuf {
        let store = scratch.0.join(crate::PROJECT_STORE_FILE);
        let (mut connection, _) = crate::open_project_store(&store).unwrap();
        let transaction = connection.transaction().unwrap();
        {
            let mut insert = transaction
                .prepare("INSERT INTO local_libraries(project_id, library_kind, library_id, document_json, created_at_unix, updated_at_unix) VALUES (?1, 'material', ?2, ?3, 1, 1)")
                .unwrap();
            for index in 0..rows {
                let id = format!("library:invented-row-{index:06}");
                insert.execute(params![PROJECT, id, library_text(160, "accepted")]).unwrap();
            }
            for id in ["lib0", "lib1"] {
                insert.execute(params![PROJECT, id, library_text(1024, "accepted")]).unwrap();
            }
        }
        transaction.commit().unwrap();
        store
    }

    fn save_reads<'a>(memory: &'a dyn ReadMemory, store: &'a Path) -> StoreLibraryReads<'a> {
        StoreLibraryReads::new(memory, move || crate::open_project_store(store).map(|(connection, _)| connection))
    }

    /// #1641 Codex r1 P1: the save-side lookup priced the largest library key
    /// once, but reads each library in its own transaction, so a larger key
    /// another connection wrote after the first read was searched under the
    /// old price (an 8 MiB key read under a 1 MiB budget). Each read now
    /// re-measures in its transaction and grows the price first: the second
    /// read is refused by name and loads nothing.
    #[test]
    fn a_larger_key_written_between_reads_is_priced_before_the_next_search() {
        let scratch = Scratch::new("key-between-reads");
        let store = save_store(&scratch, 0);
        let pool = registry(ScriptedHost::normal());
        pool.configure_pool(16 << 20, 4 << 20, false).unwrap();
        let memory = PoolReadMemory::unseeded(&pool);
        let mut reads = save_reads(&memory, &store);
        assert!(reads.read(PROJECT, "material", "lib0").unwrap().is_some(), "the first read fits the budget");
        let before = reads.reserved_bytes();
        // Another connection writes an 8 MiB key.
        Connection::open(&store)
            .unwrap()
            .execute(
                "INSERT INTO local_libraries(project_id, library_kind, library_id, document_json, created_at_unix, updated_at_unix) VALUES (?1, 'material', ?2, '{}', 1, 1)",
                params![PROJECT, "k".repeat(8 << 20)],
            )
            .unwrap();
        let refused = reads.read(PROJECT, "material", "lib1").unwrap_err();
        assert!(refused.starts_with(MEMORY_UNAVAILABLE), "{refused}");
        assert!(refused.contains("SQLite's working set for the stored libraries (their largest keyed row holds 8388"), "{refused}");
        assert_eq!(reads.reserved_bytes(), before, "nothing more is reserved or loaded");
        let diagnostics = reads.refusal().expect("the refusal is kept by name");
        assert_eq!(diagnostics[0].code, MEMORY_UNAVAILABLE);
        drop(reads);
        assert_eq!(pool.pool_status().reserved_bytes, 0);
        // Under an ordinary pool the larger key is priced and the read proceeds.
        let pool = registry(ScriptedHost::normal());
        let memory = PoolReadMemory::unseeded(&pool);
        let mut reads = save_reads(&memory, &store);
        assert!(reads.read(PROJECT, "material", "lib0").unwrap().is_some());
        assert!(reads.reserved_bytes() >= (8 << 20) * stored_read::SQLITE_WORKING_SET_FACTOR);
    }

    /// Runs the named test of this module alone in a child process: SQLite's
    /// memory status is process-wide (as `analysis_runs_tests` `in_own_process`).
    fn in_own_process(name: &str, body: impl FnOnce()) {
        const CHILD: &str = "OPENPIPESTRESS_SQLITE_STATUS_CHILD";
        if std::env::var_os(CHILD).is_some() {
            body();
            return;
        }
        let path = format!("stored_model_document_export::tests::{name}");
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args([path.as_str(), "--exact", "--test-threads=1", "--nocapture"])
            .env(CHILD, "1")
            .output()
            .unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
        eprintln!("{stdout}{}", String::from_utf8_lossy(&output.stderr));
        assert!(output.status.success() && stdout.contains("1 passed"), "{stdout}\n{}", String::from_utf8_lossy(&output.stderr));
    }

    /// SQLite's own allocation peak during `f`, above what it held when `f`
    /// began (`sqlite3_status64(SQLITE_STATUS_MEMORY_USED)`, its high-water mark
    /// reset first).
    fn sqlite_peak_during<T>(f: impl FnOnce() -> T) -> (T, u64) {
        let (mut current, mut high) = (0i64, 0i64);
        // SAFETY: plain out-pointers to two live i64s; the status call takes no other state.
        unsafe { rusqlite::ffi::sqlite3_status64(rusqlite::ffi::SQLITE_STATUS_MEMORY_USED, &mut current, &mut high, 1) };
        let start = current;
        let out = f();
        // SAFETY: as above.
        unsafe { rusqlite::ffi::sqlite3_status64(rusqlite::ffi::SQLITE_STATUS_MEMORY_USED, &mut current, &mut high, 0) };
        (out, u64::try_from(high - start).unwrap_or(0))
    }

    /// #1641 Codex r1 P2: with 30,000 ordinary library rows, the save-side
    /// lookup's open and key scan peaked at 2.55 MB of SQLite memory while
    /// 512 KiB was held. Its connection's page cache is now bounded
    /// (`LIBRARY_READ_CACHE_KIB`), and SQLite's whole peak over the open, the
    /// scans and two library reads stays within what the lookup reserved less
    /// the Rust share of each load (one `String` envelope) and the command's
    /// own Rust memory. The recovery export's connection has the same bound;
    /// its SQLite peak over the same store stays within its own memory.
    #[test]
    fn the_library_store_connection_is_bounded_whatever_the_table_size() {
        in_own_process("the_library_store_connection_is_bounded_whatever_the_table_size", || {
            let scratch = Scratch::new("thirty-thousand-rows");
            let store = save_store(&scratch, 30_000);
            let pool = registry(ScriptedHost::normal());
            let memory = PoolReadMemory::unseeded(&pool);
            let library = library_text(1024, "accepted");
            let (share, peak) = sqlite_peak_during(|| {
                let mut reads = save_reads(&memory, &store);
                for id in ["lib0", "lib1"] {
                    assert!(reads.read(PROJECT, "material", id).unwrap().is_some());
                }
                let reserved = reads.reserved_bytes();
                // Less the Rust side: the command's own memory, each loaded `String` and each parse.
                let rust = stored_read::READ_OVERHEAD_BYTES + 2 * (stored_read::exact_text_bytes(library.len() as u64) + stored_read::parse_bytes(&library).unwrap());
                eprintln!("SAVE_LOOKUP_SQLITE rows=30000 reserved={reserved} rust_share={rust}");
                reserved - rust
            });
            eprintln!("SAVE_LOOKUP_SQLITE peak={peak} sqlite_share={share}");
            assert!(peak <= share, "SQLite peaked at {peak} bytes over the lookup; its share of the reservations is {share}");
            // The recovery export over the same store, referencing both libraries.
            let model = format!(
                "{{\"project\":{{\"id\":\"{PROJECT}\"}},\"materials\":[{{\"library_ref\":{{\"library_kind\":\"material\",\"library_id\":\"lib0\"}}}},{{\"library_ref\":{{\"library_kind\":\"material\",\"library_id\":\"lib1\"}}}}]}}"
            );
            Connection::open(&store)
                .unwrap()
                .execute("INSERT INTO local_projects(project_id, project_name, model_json, created_at_unix, updated_at_unix) VALUES (?1, 'Invented', ?2, 1, 1)", params![PROJECT, model])
                .unwrap();
            let destination = scratch.0.join("recovered.json");
            let export_pool = registry(ScriptedHost::normal());
            let (request, lease) = admitted(&export_pool, &envelope(PROJECT, None)).unwrap();
            let (held, peak) = sqlite_peak_during(|| {
                export_stored_model_document_request(&request, || Ok(store.clone()), &export_pool, &lease, |_| Ok(Some(destination.clone())), |outcome, _| {
                    assert!(matches!(outcome, ModelDocumentSaveOutcome::Saved { .. }), "{outcome:?}");
                    export_pool.pool_status().reserved_bytes
                })
            });
            eprintln!("EXPORT_SQLITE rows=30000 held={held} peak={peak}");
            assert!(peak <= held, "the export's SQLite peak {peak} exceeds what it held ({held})");
        });
    }

    // -----------------------------------------------------------------------
    // #1641 Codex r3: the response that outlives the reservations is bounded.
    // -----------------------------------------------------------------------

    /// Named diagnostics: the first 64 in order, paths and messages cut on a
    /// character boundary at 2 KiB, then one counting the rest; the longest
    /// such response (control characters escaped six bytes each) is within
    /// `MAX_RESPONSE_BYTES`, and a short list is unchanged.
    #[test]
    fn named_diagnostics_are_bounded_and_short_lists_are_unchanged() {
        assert_eq!(MAX_RESPONSE_BYTES, 1_636_642);
        let short: Vec<ModelDocumentSaveDiagnostic> = (0..64).map(|index| diagnostic("CODE", &format!("p{index}"), format!("m{index}"))).collect();
        assert_eq!(bounded_diagnostics(short.clone()), short);
        let worst = "\u{1}".repeat(10_000) + "é";
        let long: Vec<ModelDocumentSaveDiagnostic> = (0..1000).map(|_| diagnostic(&"C".repeat(63), &worst, worst.clone())).collect();
        let bounded = bounded_diagnostics(long);
        assert_eq!(bounded.len(), 65);
        assert!(bounded[..64].iter().all(|item| item.path.len() <= MAX_DIAGNOSTIC_TEXT_BYTES && item.path.ends_with('…') && item.message.len() <= MAX_DIAGNOSTIC_TEXT_BYTES));
        assert_eq!(bounded[64].code, DIAGNOSTICS_NOT_SHOWN);
        let multibyte = cut_text("é".repeat(2000), MAX_DIAGNOSTIC_TEXT_BYTES);
        assert!(multibyte.len() <= MAX_DIAGNOSTIC_TEXT_BYTES && multibyte.ends_with('…'));
        let refused = ModelDocumentSaveOutcome::Refused { diagnostics: bounded };
        let length = serde_json::to_string(&refused).unwrap().len() as u64;
        eprintln!("WORST_NAMED_RESPONSE bytes={length} max={MAX_RESPONSE_BYTES}");
        assert!(length <= MAX_RESPONSE_BYTES - 32 * 1024, "{length}");
    }

    /// #1641 Codex r3: the recovery export's response was unbounded too (a
    /// library of 100,000 matching rejected records refused it with 100,000
    /// diagnostics). It names the first 64, in order, then their count.
    #[test]
    fn the_exports_response_names_at_most_64_diagnostics() {
        let scratch = Scratch::new("export-rejected-records");
        let model = format!(
            "{{\"project\":{{\"id\":\"{PROJECT}\"}},\"materials\":[{{\"id\":\"m\",\"library_ref\":{{\"library_kind\":\"material\",\"library_id\":\"{LIBRARY}\",\"record_id\":\"steel\"}}}}]}}"
        );
        let library = serde_json::to_string(&library_read_probe::rejected_records_library(100_000)).unwrap();
        let store = store_with_library(&scratch, &model, "material", LIBRARY, &library);
        let destination = scratch.0.join("recovered.json");
        let shown = Cell::new(false);
        let pool = registry(ScriptedHost::normal());
        let (request, lease) = admitted(&pool, &envelope(PROJECT, None)).unwrap();
        let (outcome, length) = export_stored_model_document_request(&request, || Ok(store.clone()), &pool, &lease, |_| { shown.set(true); Ok(Some(destination.clone())) }, |outcome, text| (outcome.clone(), text.len() as u64));
        eprintln!("EXPORT_REJECTED response_bytes={length}");
        assert!(length <= MAX_RESPONSE_BYTES, "a {length}-byte response");
        assert_refused_nothing_written(&outcome, PROTECTED_CONTENT_BLOCKED, &destination, &shown);
        let ModelDocumentSaveOutcome::Refused { diagnostics } = &outcome else { unreachable!() };
        let document: Value = serde_json::from_str(&model).unwrap();
        let stored: Value = serde_json::from_str(&library).unwrap();
        let unbounded = screen_model_document_dispositions(&document, &mut |_: &str, _: &str, _: &str| Ok(Some(stored.clone()))).unwrap_err();
        assert_eq!(unbounded.len(), 100_000);
        assert_eq!(diagnostics.len(), MAX_NAMED_DIAGNOSTICS + 1);
        assert_eq!(&diagnostics[..MAX_NAMED_DIAGNOSTICS], &unbounded[..MAX_NAMED_DIAGNOSTICS]);
        assert_eq!(diagnostics[MAX_NAMED_DIAGNOSTICS].code, DIAGNOSTICS_NOT_SHOWN);
    }

    // --- W-17 object form (DEL-02-02 Design `project-unit-selection.md` §3) ---
    //
    // #1650 found that a struct written as a JSON array (serde's positional
    // form) passed a keyed screen. Recovery does no typed decode, but its
    // screen skipped any `provenance_record`, `rule_pack_refs[]` entry or
    // `library_ref` that is not an object, and any disposition member it could
    // not read as text, so such a stored row was written out unscreened.

    /// A record in positional form: its member values, in order.
    fn positional_record(record: Value) -> Value {
        Value::Array(record.as_object().expect("an object").values().cloned().collect())
    }

    /// A stored document with `material` as its one material.
    fn with_material(material: Value) -> String {
        serde_json::to_string(&json!({
            "schema_version": "0.8.0",
            "document_kind": "openpipestress.model",
            "project": { "id": PROJECT, "name": "Invented", "description": "" },
            "materials": [material]
        }))
        .unwrap()
    }

    fn quarantined_record() -> Value {
        json!({ "source_name": "Invented source", "redistribution_status": "private_only", "review_status": "quarantined" })
    }

    /// The probe of #1650 §2.6 and its variants, on a project row and on a
    /// saved state written raw (a hand-edited store, or a row an earlier build
    /// wrote): each screened record that is not an object, and each record
    /// whose disposition member is not text, is refused by name, shows no
    /// chooser and writes nothing. On `e2f9677175` each was written out.
    #[test]
    fn a_screened_record_that_is_not_an_object_is_refused_by_name() {
        let rule_pack_ref = json!({ "rule_pack_id": "pack:invented", "name": "Invented", "rule_pack_version": "1.0.0", "redistribution_status": "protected_suspected" });
        let cases: Vec<(&str, String, &str)> = vec![
            ("positional provenance", with_material(json!({ "id": "m", "provenance_record": positional_record(quarantined_record()) })), "materials[0].provenance_record"),
            ("text provenance", with_material(json!({ "id": "m", "provenance_record": "quarantined" })), "materials[0].provenance_record"),
            ("null provenance", with_material(json!({ "id": "m", "provenance_record": null })), "materials[0].provenance_record"),
            ("array disposition", with_material(json!({ "id": "m", "provenance_record": { "source_name": "s", "review_status": ["quarantined"] } })), "materials[0].provenance_record"),
            ("object disposition", with_material(json!({ "id": "m", "provenance_record": { "source_name": "s", "redistribution_status": { "status": "rejected" } } })), "materials[0].provenance_record"),
            (
                "positional rule-pack reference",
                serde_json::to_string(&json!({ "project": { "id": PROJECT }, "rule_pack_refs": [positional_record(rule_pack_ref.clone())] })).unwrap(),
                "rule_pack_refs[0]",
            ),
            ("rule-pack references not a list", serde_json::to_string(&json!({ "project": { "id": PROJECT }, "rule_pack_refs": rule_pack_ref })).unwrap(), "rule_pack_refs"),
        ];
        let pool = registry(ScriptedHost::normal());
        for (name, document, path) in cases {
            let scratch = Scratch::new("malformed-record");
            let store = current_store(&scratch, &document, &document);
            let destination = scratch.0.join("recovered.json");
            for state_id in [None, Some(STATE)] {
                let shown = Cell::new(false);
                let refused = export(&store, state_id, &pool, &destination, &shown);
                assert_eq!(refused_codes(&refused), vec![(DISPOSITION_RECORD_MALFORMED, path)], "{name} ({state_id:?}): {refused:?}");
                assert_refused_nothing_written(&refused, DISPOSITION_RECORD_MALFORMED, &destination, &shown);
            }
            assert_eq!(pool.pool_status().reserved_bytes, 0, "{name}");
        }
        // The object form of the positional record is refused by its disposition, as before.
        let scratch = Scratch::new("object-record");
        let document = with_material(json!({ "id": "m", "provenance_record": quarantined_record() }));
        let store = current_store(&scratch, &document, STATE_TEXT);
        let refused = export(&store, None, &pool, &scratch.0.join("recovered.json"), &Cell::new(false));
        assert_eq!(refused_codes(&refused), vec![(PROTECTED_CONTENT_BLOCKED, "materials[0].provenance_record")]);
    }

    /// A positional `library_ref` (on the entity or in its provenance record)
    /// named a rejected stored record that the screen never looked up; it is
    /// refused by name. Its object form is refused by the stored record's
    /// disposition.
    #[test]
    fn a_positional_library_ref_is_refused_by_name() {
        let reference = json!({ "library_kind": "material", "library_id": LIBRARY, "record_id": "steel" });
        let library = serde_json::to_string(&library_read_probe::rejected_records_library(1)).unwrap();
        let pool = registry(ScriptedHost::normal());
        for (document, path) in [
            (with_material(json!({ "id": "m", "library_ref": positional_record(reference.clone()) })), "materials[0].library_ref"),
            (
                with_material(json!({ "id": "m", "provenance_record": { "source_name": "s", "library_ref": positional_record(reference.clone()) } })),
                "materials[0].provenance_record.library_ref",
            ),
        ] {
            let scratch = Scratch::new("positional-library-ref");
            let store = store_with_library(&scratch, &document, "material", LIBRARY, &library);
            let destination = scratch.0.join("recovered.json");
            let shown = Cell::new(false);
            let refused = export(&store, None, &pool, &destination, &shown);
            assert_eq!(refused_codes(&refused), vec![(DISPOSITION_RECORD_MALFORMED, path)], "{refused:?}");
            assert_refused_nothing_written(&refused, DISPOSITION_RECORD_MALFORMED, &destination, &shown);
        }
        let scratch = Scratch::new("object-library-ref");
        let document = with_material(json!({ "id": "m", "library_ref": reference }));
        let store = store_with_library(&scratch, &document, "material", LIBRARY, &library);
        let refused = export(&store, None, &pool, &scratch.0.join("recovered.json"), &Cell::new(false));
        assert_eq!(refused_codes(&refused).first().map(|(code, _)| *code), Some(PROTECTED_CONTENT_BLOCKED), "{refused:?}");
        // A hand-edited stored library: the referenced record's provenance,
        // or the library object itself, in positional form.
        let mut positional = library_read_probe::representative_material_library();
        let record = positional["material_records"][0].clone();
        positional["material_records"][0]["provenance"] = positional_record(record["provenance"].clone());
        positional["material_records"][0]["provenance"].as_array_mut().unwrap().push(json!("rejected"));
        let mut library_object = library_read_probe::representative_material_library();
        library_object["material_library"] = positional_record(library_object["material_library"].clone());
        for stored in [positional, library_object] {
            let scratch = Scratch::new("positional-stored-library");
            let store = store_with_library(&scratch, &document, "material", LIBRARY, &serde_json::to_string(&stored).unwrap());
            let destination = scratch.0.join("recovered.json");
            let shown = Cell::new(false);
            let refused = export(&store, None, &pool, &destination, &shown);
            assert_eq!(refused_codes(&refused), vec![(DISPOSITION_RECORD_MALFORMED, "materials[0].library_ref")], "{refused:?}");
            assert_refused_nothing_written(&refused, DISPOSITION_RECORD_MALFORMED, &destination, &shown);
        }
    }

    /// Recovery still writes a row whose defect is elsewhere: a positional
    /// member that is not a screened record (a node, the project's units) is
    /// not judged here, as before.
    #[test]
    fn a_positional_member_outside_the_screened_records_still_recovers() {
        let document = serde_json::to_string(&json!({
            "schema_version": "0.8.0",
            "project": { "id": PROJECT, "units": ["m", "N"] },
            "nodes": [["node:invented", [0, 0, 0]]],
            "materials": [{ "id": "m", "provenance_record": { "source_name": "s", "review_status": "accepted" } }]
        }))
        .unwrap();
        let scratch = Scratch::new("unrelated-positional");
        let store = current_store(&scratch, &document, STATE_TEXT);
        let destination = scratch.0.join("recovered.json");
        let outcome = export(&store, None, &registry(ScriptedHost::normal()), &destination, &Cell::new(false));
        assert!(matches!(outcome, ModelDocumentSaveOutcome::Saved { .. }), "{outcome:?}");
        assert_eq!(fs::read(&destination).unwrap(), document.as_bytes());
    }

    /// Reachability through a current writer: Save State (`save_model_state`)
    /// keeps a snapshot whose model has blocking slot findings, marked
    /// `MODEL_INCOMPLETE` (W-12 §2), so a session request carrying a
    /// positional provenance record is stored. Opening it refuses; its
    /// recovery export is refused by name (on `e2f9677175` it was written).
    #[test]
    fn a_saved_state_holding_a_positional_record_is_refused_by_name() {
        use crate::model_states::{labels_from_value, save_state_in_store, SaveModelStateRequest, StateWriteHooks};
        use open_pipe_stress_operation_applier::canonical_json;
        let scratch = Scratch::new("saved-state-positional");
        let store = scratch.0.join("store.sqlite3");
        let model = |provenance: Value| {
            json!({
                "schema_version": open_pipe_stress_product_physics::MODEL_SCHEMA_VERSION,
                "document_kind": "openpipestress.product_preview.model",
                "project": { "id": PROJECT, "name": "Invented", "units": { "length": "m" } },
                "nodes": [{ "id": "node:N-100", "coordinates": [0, 0, 0] }],
                "pipe_segments": [], "supports": [], "sections": [], "components": [],
                "materials": [{ "id": "material:invented", "provenance_record": provenance }],
                "load_cases": [], "combinations": []
            })
        };
        let (mut connection, _) = crate::open_project_store(&store).unwrap();
        let base = "sha256:invented-base";
        let current = model(json!({ "source_name": "Invented source", "review_status": "accepted" }));
        crate::upsert_project(&mut connection, PROJECT, "Invented", &current, &json!([]), &Value::Null, &Value::Null, &Value::Null, &Value::Null, &Value::Null, &json!({ "value": base })).unwrap();
        drop(connection);
        let snapshot = model(positional_record(quarantined_record()));
        let model_canonical = canonical_json(&snapshot);
        let request = SaveModelStateRequest {
            project_id: PROJECT.to_string(),
            expected_base: base.to_string(),
            model_hash: format!("sha256:{}", sha256_hex(&model_canonical)),
            model_canonical,
            state_kind: "design_snapshot".to_string(),
            parent_state_id: None,
            labels: labels_from_value(json!({ "state_name": "Invented positional" })).unwrap(),
        };
        let allocate = || Ok(("00000000000000000000000000000f01".to_string(), 1_000));
        let between_rows = |_: &rusqlite::Transaction<'_>| Ok(());
        let saved = save_state_in_store(&store, request, &StateWriteHooks { allocate: &allocate, between_rows: &between_rows }).expect("the writer keeps the snapshot");
        assert_eq!(saved.outcome, "saved");
        assert!(saved.state.analysis_status.contains(&"MODEL_INCOMPLETE".to_string()), "{:?}", saved.state.analysis_status);
        let destination = scratch.0.join("recovered.json");
        let shown = Cell::new(false);
        let refused = export(&store, Some(&saved.state.state_id), &registry(ScriptedHost::normal()), &destination, &shown);
        assert_eq!(refused_codes(&refused), vec![(DISPOSITION_RECORD_MALFORMED, "materials[0].provenance_record")], "{refused:?}");
        assert_refused_nothing_written(&refused, DISPOSITION_RECORD_MALFORMED, &destination, &shown);
    }

    // --- #1651 Codex r1: stored disposition members the verdict cannot read ---

    /// A store whose project row and saved state both hold `model`, with one
    /// stored library.
    fn store_with_library_both_rows(scratch: &Scratch, model: &str, kind: &str, id: &str, library: &str) -> PathBuf {
        let store = current_store(scratch, model, model);
        Connection::open(&store)
            .unwrap()
            .execute(
                "INSERT INTO local_libraries(project_id, library_kind, library_id, document_json, created_at_unix, updated_at_unix) VALUES (?1, ?2, ?3, ?4, 1, 1)",
                params![PROJECT, kind, id, library],
            )
            .unwrap();
        store
    }

    /// The ordinary private-library writer (`save_local_library_value`)
    /// stores a record whose `review_status` or `redistribution_status` is an
    /// array or an object (the import accepts it as `PRIVATE_LOCAL_ONLY`: it
    /// states no disposition). Recovering a project or state that references
    /// it is refused by name; on `20bde6a3a3` both were written.
    #[test]
    fn a_library_the_ordinary_writer_stores_with_an_unreadable_disposition_refuses_recovery() {
        for member in ["review_status", "redistribution_status"] {
            let blocking = if member == "review_status" { "quarantined" } else { "protected_suspected" };
            for value in [json!([blocking]), json!({ "status": "rejected" })] {
                let mut library = library_read_probe::representative_material_library();
                library["material_library"]["library_id"] = json!(LIBRARY);
                library["material_records"][0]["provenance"][member] = value.clone();
                library["material_records"][0][member] = value.clone();
                let text = with_material(json!({ "id": "m", "library_ref": { "library_kind": "material", "library_id": LIBRARY, "record_id": "steel" } }));
                let scratch = Scratch::new("ordinary-writer");
                let store = current_store(&scratch, &text, &text);
                let saved = crate::save_local_library_value(|| Ok(store.clone()), PROJECT.into(), "material".into(), library).unwrap();
                assert!(saved.stored, "{member}={value}: the writer stores it");
                assert_eq!(saved.validation["outcome"], "PRIVATE_LOCAL_ONLY");
                for state_id in [None, Some(STATE)] {
                    let destination = scratch.0.join("recovered.json");
                    let shown = Cell::new(false);
                    let refused = export(&store, state_id, &registry(ScriptedHost::normal()), &destination, &shown);
                    assert_eq!(refused_codes(&refused), vec![(DISPOSITION_RECORD_MALFORMED, "materials[0].library_ref")], "{member}={value} ({state_id:?})");
                    assert_refused_nothing_written(&refused, DISPOSITION_RECORD_MALFORMED, &destination, &shown);
                }
            }
        }
    }

    /// Every library kind, with the unreadable member on the library object,
    /// on a record's provenance or nested deeper in the record, as an array
    /// or an object: recovery of the project and of the state is refused by
    /// name (Codex r1's matrix).
    #[test]
    fn a_stored_unreadable_disposition_refuses_recovery_for_every_kind_and_depth() {
        for kind in ["material", "section", "component", "hanger"] {
            let (records, id_key) = crate::library_records_key(kind).unwrap();
            for site in ["header", "record", "nested"] {
                for value in [json!(["quarantined"]), json!({ "status": "rejected" })] {
                    let mut record = json!({ id_key: "r" });
                    let mut header = json!({ "library_id": LIBRARY });
                    match site {
                        "header" => header["review_status"] = value.clone(),
                        "record" => record["provenance"] = json!({ "review_status": value }),
                        _ => record["nested"] = json!({ "provenance": { "review_status": value } }),
                    }
                    let library = json!({ format!("{kind}_library"): header, records: [record] });
                    let model = json!({ "project": { "id": PROJECT }, "materials": [{ "library_ref": { "library_kind": kind, "library_id": LIBRARY, "record_id": "r" } }] });
                    let scratch = Scratch::new("stored-matrix");
                    let store = store_with_library_both_rows(&scratch, &serde_json::to_string(&model).unwrap(), kind, LIBRARY, &serde_json::to_string(&library).unwrap());
                    for state_id in [None, Some(STATE)] {
                        let refused = export(&store, state_id, &registry(ScriptedHost::normal()), &scratch.0.join("out.json"), &Cell::new(false));
                        assert_eq!(refused_codes(&refused), vec![(DISPOSITION_RECORD_MALFORMED, "materials[0].library_ref")], "{kind}/{site}/{value} ({state_id:?})");
                    }
                }
            }
        }
    }

    /// Save Model Document's admission refuses the same stored shape (on
    /// `20bde6a3a3` it admitted it); Save Handoff keeps its own structural
    /// refusal by pointer (`handoff_package`
    /// `round6_nonstring_blocking_value_is_refused_as_malformed`).
    #[test]
    fn save_model_document_refuses_an_unreadable_stored_disposition() {
        let mut model: Value = serde_json::from_str(include_str!("../../../../fixtures/model_document/invented_every_slot_model.json")).unwrap();
        model["materials"][0]["provenance_record"]["library_ref"] = json!({ "library_kind": "material", "library_id": LIBRARY, "record_id": "steel" });
        let text = open_pipe_stress_operation_applier::canonical_json(&model);
        let hash = format!("sha256:{}", sha256_hex(&text));
        let mut library = library_read_probe::representative_material_library();
        let clean = crate::model_document_save::admit_model_document_text(text.clone(), &hash, &hash, &mut |_: &str, kind: &str, id: &str| Ok((kind == "material" && id == LIBRARY).then(|| library.clone())));
        assert!(clean.is_ok(), "the control saves: {:?}", clean.err());
        library["material_records"][0]["review_status"] = json!(["quarantined"]);
        library["material_records"][0]["provenance"]["review_status"] = json!(["quarantined"]);
        let refused = crate::model_document_save::admit_model_document_text(text, &hash, &hash, &mut |_: &str, kind: &str, id: &str| Ok((kind == "material" && id == LIBRARY).then(|| library.clone()))).unwrap_err();
        let codes: Vec<(&str, &str)> = refused.iter().map(|item| (item.code.as_str(), item.path.as_str())).collect();
        assert_eq!(codes, vec![(DISPOSITION_RECORD_MALFORMED, "materials[0].provenance_record.library_ref")]);
    }

    /// A whole stored record in positional form is not identifiable by its
    /// id: the reference is unresolved (`MODEL-LIBRARY-REF-UNRESOLVED`, a
    /// warning), as an absent record is, and recovery writes the document.
    #[test]
    fn a_whole_positional_stored_record_is_an_unresolved_reference() {
        let library = json!({ "material_library": {}, "material_records": [["steel", "quarantined"]] });
        let text = with_material(json!({ "id": "m", "library_ref": { "library_kind": "material", "library_id": LIBRARY, "record_id": "steel" } }));
        let scratch = Scratch::new("positional-whole-record");
        let store = store_with_library_both_rows(&scratch, &text, "material", LIBRARY, &serde_json::to_string(&library).unwrap());
        match export(&store, None, &registry(ScriptedHost::normal()), &scratch.0.join("out.json"), &Cell::new(false)) {
            ModelDocumentSaveOutcome::Saved { warnings, .. } => {
                assert_eq!(warnings.iter().map(|item| item.code.as_str()).collect::<Vec<_>>(), vec![crate::MODEL_LIBRARY_REF_UNRESOLVED]);
            }
            other => panic!("not saved: {other:?}"),
        }
    }
}
