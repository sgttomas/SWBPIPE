//! Tests of the project's saved-package entries (DEL-08-01 REQ-023; DEL-02-05
//! REQ-039, AC-017, VER-013). Every value is invented. The route-010 save's
//! entry, with its row inspected and the store compared before and after
//! (DEL-08-01 VER-010), is in `user_release_save_tests.rs`.

use super::*;
use crate::report_package_save::test_support::{desktop_request, model as package_model, no_store, sandbox, save_request, software};
use crate::report_package_save::{admit_report_package_in, keep_no_entry, no_custody, save_admitted_report_package_keeping, AdmittedReportPackage};
use crate::stored_read::UnmeteredReadMemory;
use rusqlite::types::Value as SqlValue;
use serde_json::json;
use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::sync::Arc;

/// The list as the command builds it, unmetered.
pub(crate) fn listed(path: &Path, session: &SessionHeldEntries, project_id: &str, holder: Option<&str>) -> SavedPackageList {
    list_entries(Ok(path.to_path_buf()), session, project_id, holder, &UnmeteredReadMemory).unwrap().into_unreserved().unwrap()
}

/// The project's stored entries, as a caller with no holder lists them.
/// The keep call as the command makes it, unmetered.
pub(crate) fn keep_held(store: Result<PathBuf, String>, session: &SessionHeldEntries, project_id: &str, holder: &str) -> HeldEntriesWrite {
    keep_held_entries(store, session, project_id, holder, &UnmeteredReadMemory).unwrap().into_unreserved().unwrap()
}

pub(crate) fn stored(path: &Path, project_id: &str) -> Vec<SavedPackageEntry> {
    listed(path, &SessionHeldEntries::default(), project_id, None).entries
}

fn checked(path: &Path, session: &SessionHeldEntries, project_id: &str, holder: Option<&str>, entry_id: &str) -> Result<SavedPackageState, String> {
    check_entry(Ok(path.to_path_buf()), session, project_id, holder, entry_id, &UnmeteredReadMemory).map(|state| state.into_unreserved().unwrap())
}

fn removed(path: &Path, session: &SessionHeldEntries, project_id: &str, holder: Option<&str>, entry_id: &str) -> Result<SavedPackageRemoval, String> {
    remove_entry(Ok(path.to_path_buf()), session, project_id, holder, entry_id)
}

fn ids(entries: &[SavedPackageEntry]) -> Vec<String> {
    entries.iter().map(|entry| entry.entry_id.clone()).collect()
}

/// A hook the list runs between its read of the session and its read of the
/// store, on this thread (review round 1, R5).
pub(crate) mod hooks {
    use std::cell::RefCell;

    thread_local! {
        static BETWEEN: RefCell<Option<Box<dyn FnOnce()>>> = RefCell::new(None);
    }

    pub(crate) fn arm_between_list_reads(hook: impl FnOnce() + 'static) {
        BETWEEN.with(|between| *between.borrow_mut() = Some(Box::new(hook)));
    }

    pub(in crate::saved_package_entries) fn between_list_reads() {
        if let Some(hook) = BETWEEN.with(|between| between.borrow_mut().take()) {
            hook();
        }
    }
}

/// A fault injected into every entry write after its inserts and before its
/// commit, carrying a location so that its withholding is shown.
pub(crate) mod fault {
    use std::cell::RefCell;

    thread_local! {
        static FAULT: RefCell<Option<String>> = const { RefCell::new(None) };
    }

    /// Arms (or, with `None`, disarms) the fault on this thread.
    pub(crate) fn arm(message: Option<String>) {
        FAULT.with(|fault| *fault.borrow_mut() = message);
    }

    pub(in crate::saved_package_entries) fn before_commit() -> Result<(), String> {
        FAULT.with(|fault| match fault.borrow().as_ref() {
            Some(message) => Err(message.clone()),
            None => Ok(()),
        })
    }
}

/// A directory name planted in every saved location: it must appear in no
/// reply, outcome, diagnostic or message.
pub(crate) const LOCATION_MARKER: &str = "location-marker-q7v3z";

pub(crate) fn scratch(label: &str) -> PathBuf {
    let dir = sandbox(&format!("entries-{label}")).join(LOCATION_MARKER);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

pub(crate) fn store_in(dir: &Path) -> PathBuf {
    dir.join(crate::PROJECT_STORE_FILE)
}

pub(crate) fn write_project(path: &Path, project_id: &str, model: &serde_json::Value) {
    let (mut connection, _) = crate::open_project_store(path).unwrap();
    crate::upsert_project(
        &mut connection, project_id, "Invented project", model, &json!([]), &serde_json::Value::Null, &serde_json::Value::Null,
        &serde_json::Value::Null, &serde_json::Value::Null, &serde_json::Value::Null, &json!({ "value": "sha256:base-1" }),
    )
    .unwrap();
}

/// Every table's rows as SQLite holds them, in rowid order; the entries'
/// table, which has no rowid, in each project's order kept.
pub(crate) fn dump(path: &Path) -> BTreeMap<String, Vec<Vec<SqlValue>>> {
    let connection = Connection::open(path).unwrap();
    let mut names = connection.prepare("SELECT name FROM sqlite_master WHERE type = 'table' AND name NOT LIKE 'sqlite_%' ORDER BY name").unwrap();
    let names: Vec<String> = names.query_map([], |row| row.get(0)).unwrap().collect::<Result<_, _>>().unwrap();
    names
        .into_iter()
        .map(|name| {
            let order = if name == "saved_package_entries" { "project_id, sequence" } else { "rowid" };
            let mut statement = connection.prepare(&format!("SELECT * FROM \"{name}\" ORDER BY {order}")).unwrap();
            let columns = statement.column_count();
            let rows = statement
                .query_map([], |row| (0..columns).map(|index| row.get::<_, SqlValue>(index)).collect::<Result<Vec<_>, _>>())
                .unwrap()
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
            (name, rows)
        })
        .collect()
}

/// [`dump`] without the entries' table: every other record of the store.
pub(crate) fn others(path: &Path) -> BTreeMap<String, Vec<Vec<SqlValue>>> {
    let mut all = dump(path);
    all.remove("saved_package_entries");
    all
}

pub(crate) fn entry_rows(path: &Path) -> Vec<Vec<SqlValue>> {
    dump(path).remove("saved_package_entries").unwrap_or_default()
}

fn sha(bytes: &[u8]) -> String {
    sha2::Digest::finalize(<sha2::Sha256 as sha2::Digest>::new_with_prefix(bytes)).iter().map(|byte| format!("{byte:02x}")).collect()
}

fn basis(n: u8) -> SavedPackageBasis {
    SavedPackageBasis {
        model_hash: format!("{n:x}").repeat(64),
        received_result_sha256: "b".repeat(64),
        run_id: "run:preview-linear-static-001".to_string(),
        run_record_sha256: "c".repeat(64),
    }
}

/// An invented published package of `bytes` at `location`.
fn published(project_id: &str, location: &Path, bytes: &[u8], durable: bool, n: u8) -> PublishedPackage {
    PublishedPackage {
        project_id: project_id.to_string(),
        package_identity_sha256: "d".repeat(64),
        container_sha256: sha(bytes),
        container_byte_count: bytes.len() as u64,
        location: location.to_path_buf(),
        saved_at: format!("2026-11-21T10:00:0{n}.000Z"),
        export_context: "local_private".to_string(),
        durability_confirmed: durable,
        basis: EntryBasis::Bound(basis(n)),
    }
}

const PROJECT: &str = "project:invented-entries";

fn invented_model() -> serde_json::Value {
    json!({ "project": { "id": PROJECT, "name": "Invented project" } })
}

fn store_with_project(label: &str) -> (PathBuf, PathBuf) {
    let dir = scratch(label);
    let path = store_in(&dir);
    write_project(&path, PROJECT, &invented_model());
    (dir, path)
}

#[track_caller]
fn kept(outcome: &SavedPackageEntryOutcome) -> String {
    assert_eq!((outcome.status.as_str(), outcome.code.as_str()), ("kept", ENTRY_KEPT), "{outcome:?}");
    outcome.entry_id.clone()
}

#[track_caller]
fn withholds_location(text: &str) {
    assert!(!text.contains(LOCATION_MARKER), "the location leaked: {text}");
}

// --- The table and its migration (REQ-027, REQ-039, AC-017) -------------------

#[test]
fn the_table_holds_exactly_the_members_the_definition_names() {
    let (_dir, path) = store_with_project("columns");
    let connection = Connection::open(&path).unwrap();
    let mut statement = connection.prepare("PRAGMA table_info(saved_package_entries)").unwrap();
    let columns: Vec<String> = statement.query_map([], |row| row.get(1)).unwrap().collect::<Result<_, _>>().unwrap();
    assert_eq!(
        columns,
        [
            "project_id", "entry_id", "sequence", "package_identity_sha256", "container_sha256", "container_byte_count", "file_name",
            "location", "saved_at", "export_context", "durability_confirmed", "model_hash", "received_result_sha256", "run_id",
            "run_record_sha256",
        ]
    );
}

/// The steps of the build before the entries' table (store v14).
fn steps_before_v15() -> Vec<crate::StoreMigration> {
    crate::store_migrations().into_iter().filter(|step| step.version <= 14).collect()
}

/// A v14 store with a project row, as the build before the table wrote it.
fn v14_store(label: &str) -> PathBuf {
    let path = store_in(&scratch(label));
    let connection = Connection::open(&path).unwrap();
    crate::apply_store_migrations_to(&connection, 14, &steps_before_v15()).unwrap();
    let model = serde_json::to_string(&invented_model()).unwrap();
    connection
        .execute(
            "INSERT INTO local_projects (project_id, project_name, model_json, created_at_unix, updated_at_unix) VALUES (?1, 'Invented', ?2, 1, 2)",
            params![PROJECT, model],
        )
        .unwrap();
    path
}

#[test]
fn a_store_of_the_version_before_opens_with_the_table_added_and_every_earlier_record_unchanged() {
    let path = v14_store("v14-to-v15");
    let before = dump(&path);
    assert!(!before.contains_key("saved_package_entries"));
    let (connection, migration) = crate::open_project_store(&path).unwrap();
    assert_eq!((migration.store_schema_version_before_open, migration.store_schema_version), (14, 15));
    assert_eq!(migration.migrations_applied_on_open, ["store-v15-saved-package-entries-table"]);
    drop(connection);
    let mut after = dump(&path);
    assert_eq!(after.remove("saved_package_entries"), Some(Vec::new()), "the table is added, empty");
    assert_eq!(after, before, "every earlier record is unchanged");
}

#[test]
fn a_failed_migration_leaves_the_store_as_it_was() {
    let path = v14_store("v15-fails");
    let before = dump(&path);
    let mut steps = steps_before_v15();
    steps.push(crate::StoreMigration {
        version: 15,
        migration_id: "store-v15-saved-package-entries-table",
        deferrable: false,
        // The real step, then a failure: the transaction is rolled back whole.
        apply: |connection| {
            migrate_v15_saved_package_entries(connection)?;
            Err("invented failure after the table was created".to_string())
        },
    });
    let connection = Connection::open(&path).unwrap();
    let error = crate::apply_store_migrations_to(&connection, 15, &steps).unwrap_err();
    assert!(error.contains("invented failure"), "{error}");
    assert_eq!(crate::store_user_version(&connection).unwrap(), 14);
    drop(connection);
    assert_eq!(dump(&path), before, "no table, no row changed");
}

#[test]
fn a_store_newer_than_the_build_is_refused_before_anything_is_written() {
    let path = store_in(&scratch("newer"));
    {
        let connection = Connection::open(&path).unwrap();
        connection.execute_batch("CREATE TABLE marker (name TEXT NOT NULL); PRAGMA user_version = 16;").unwrap();
    }
    let bytes = std::fs::read(&path).unwrap();
    let error = crate::open_project_store(&path).map(|_| ()).unwrap_err();
    assert!(error.starts_with("STORE-SCHEMA-NEWER") && error.contains("v16") && error.contains("v15"), "{error}");
    assert_eq!(std::fs::read(&path).unwrap(), bytes, "the store's bytes are unchanged");
    assert!(!crate::store_sidecar_path(&path, "-wal").exists(), "no journal was started");
}

// --- Entry, Content, Removal and the store's keeping (REQ-023, REQ-039) -------

#[test]
fn entries_reopen_byte_identical_and_in_the_order_saved() {
    let (dir, path) = store_with_project("reopen");
    let mut ids = Vec::new();
    for n in 1..=3u8 {
        let location = dir.join(format!("saved-{n}.opsproj"));
        let bytes = vec![n; 64];
        std::fs::write(&location, &bytes).unwrap();
        ids.push(kept(&keep_entry_in_store(&path, &published(PROJECT, &location, &bytes, n != 2, n))));
    }
    let written = entry_rows(&path);
    // Reopened, as the desktop does after it is closed.
    let listed = SavedPackageList { project_id: PROJECT.to_string(), entries: stored(&path, PROJECT), store_failure: None };
    assert_eq!(entry_rows(&path), written, "each stored record byte for byte as it was written");
    assert_eq!(listed.entries.iter().map(|entry| entry.entry_id.clone()).collect::<Vec<_>>(), ids, "in the order saved");
    assert_eq!(listed.entries.iter().map(|entry| entry.sequence).collect::<Vec<_>>(), [1, 2, 3]);
    for (n, entry) in (1..=3u8).zip(&listed.entries) {
        let expected = published(PROJECT, &dir.join(format!("saved-{n}.opsproj")), &vec![n; 64], n != 2, n);
        assert_eq!(entry.package_identity_sha256, expected.package_identity_sha256);
        assert_eq!(entry.container_sha256, expected.container_sha256);
        assert_eq!(entry.container_byte_count, 64);
        assert_eq!(entry.file_name, format!("saved-{n}.opsproj"));
        assert_eq!(entry.saved_at, expected.saved_at);
        assert_eq!(entry.export_context, "local_private");
        assert_eq!(entry.durability_confirmed, n != 2, "an entry whose durability was not confirmed says so");
        assert_eq!(entry.basis, basis(n));
        assert_eq!(entry.kept, "kept_with_project");
    }
    // The stored location is the exact location written.
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;
        assert_eq!(written[0][7], SqlValue::Blob(dir.join("saved-1.opsproj").as_os_str().as_bytes().to_vec()));
    }
}

#[test]
fn saving_the_same_package_again_adds_another_entry() {
    let (dir, path) = store_with_project("twice");
    let location = dir.join("same.opsproj");
    std::fs::write(&location, b"same").unwrap();
    let package = published(PROJECT, &location, b"same", true, 1);
    let first = kept(&keep_entry_in_store(&path, &package));
    let second = kept(&keep_entry_in_store(&path, &package));
    assert_ne!(first, second);
    assert_eq!(stored(&path, PROJECT).len(), 2);
}

#[test]
fn each_addition_and_each_removal_leaves_every_other_record_byte_identical() {
    // A project with a stored run slot, saved states and recorded runs.
    let path = crate::analysis_runs::test_fixtures::store_with_project("entries-others");
    let solved = crate::analysis_runs::test_fixtures::model("Invented project");
    crate::analysis_runs::test_fixtures::save_state(&path, &solved, "Basis", "000000000000000000000000000000e7", 1_000);
    crate::analysis_runs::test_fixtures::record(&path, crate::analysis_runs::test_fixtures::request(&solved, "Run 1"), crate::analysis_runs::test_fixtures::T0).unwrap();
    let project = crate::analysis_runs::test_fixtures::PROJECT;
    let before = others(&path);
    for table in ["local_projects", "model_states", "model_state_labels", "analysis_runs", "analysis_run_labels"] {
        assert!(!before[table].is_empty(), "{table} is seeded");
    }
    let dir = scratch("entries-others-files");
    let mut ids = Vec::new();
    for n in 1..=2u8 {
        let location = dir.join(format!("p{n}.opsproj"));
        std::fs::write(&location, [n]).unwrap();
        ids.push(kept(&keep_entry_in_store(&path, &published(project, &location, &[n], true, n))));
        assert_eq!(others(&path), before, "after adding entry {n}");
    }
    let kept_rows = entry_rows(&path);
    removed(&path, &SessionHeldEntries::default(), project, None, &ids[0]).unwrap();
    assert_eq!(others(&path), before, "after removing an entry");
    assert_eq!(entry_rows(&path), kept_rows[1..].to_vec(), "the other entry is unchanged");
}

#[test]
fn an_entry_is_never_edited() {
    let (dir, path) = store_with_project("immutable");
    let location = dir.join("p.opsproj");
    std::fs::write(&location, b"p").unwrap();
    let id = kept(&keep_entry_in_store(&path, &published(PROJECT, &location, b"p", true, 1)));
    let rows = entry_rows(&path);
    let connection = Connection::open(&path).unwrap();
    let update = connection.execute("UPDATE saved_package_entries SET file_name = 'other' WHERE entry_id = ?1", params![id]).unwrap_err();
    assert!(update.to_string().contains("SAVED-PACKAGE-ENTRY-IMMUTABLE"), "{update}");
    let replace = connection
        .execute(
            "INSERT OR REPLACE INTO saved_package_entries SELECT project_id, entry_id, sequence, package_identity_sha256, container_sha256,
               container_byte_count, 'other', location, saved_at, export_context, durability_confirmed, model_hash,
               received_result_sha256, run_id, run_record_sha256 FROM saved_package_entries WHERE entry_id = ?1",
            params![id],
        )
        .unwrap_err();
    assert!(replace.to_string().contains("SAVED-PACKAGE-ENTRY-IMMUTABLE"), "{replace}");
    drop(connection);
    assert_eq!(entry_rows(&path), rows);
}

#[test]
fn removing_an_entry_or_the_project_removes_the_record_and_never_the_file() {
    let (dir, path) = store_with_project("removal");
    write_project(&path, "project:other", &json!({ "project": { "id": "project:other" } }));
    let location = dir.join("kept.opsproj");
    std::fs::write(&location, b"package bytes").unwrap();
    let first = kept(&keep_entry_in_store(&path, &published(PROJECT, &location, b"package bytes", true, 1)));
    kept(&keep_entry_in_store(&path, &published(PROJECT, &location, b"package bytes", true, 2)));
    let other = kept(&keep_entry_in_store(&path, &published("project:other", &location, b"package bytes", true, 3)));

    let session = SessionHeldEntries::default();
    let removal = removed(&path, &session, PROJECT, None, &first).unwrap();
    assert!(removal.removed);
    withholds_location(&serde_json::to_string(&removal).unwrap());
    assert_eq!(std::fs::read(&location).unwrap(), b"package bytes", "the file is untouched");
    assert_eq!(stored(&path, PROJECT).len(), 1);
    let unknown = removed(&path, &session, PROJECT, None, &first).unwrap_err();
    assert!(unknown.starts_with(ENTRY_UNKNOWN), "{unknown}");

    // Removing the project removes its entries (the foreign key cascades).
    let (connection, _) = crate::open_project_store(&path).unwrap();
    connection.execute("DELETE FROM local_projects WHERE project_id = ?1", params![PROJECT]).unwrap();
    drop(connection);
    assert!(stored(&path, PROJECT).is_empty());
    assert_eq!(stored(&path, "project:other")[0].entry_id, other, "another project's entries stay");
    assert_eq!(std::fs::read(&location).unwrap(), b"package bytes", "the file is untouched");
}

#[test]
fn a_save_from_a_project_the_store_does_not_hold_keeps_no_entry_and_says_so() {
    let (dir, path) = store_with_project("not-saved");
    let before = dump(&path);
    let location = dir.join("p.opsproj");
    std::fs::write(&location, b"p").unwrap();
    let outcome = keep_entry_in_store(&path, &published("project:never-saved", &location, b"p", true, 1));
    assert_eq!((outcome.status.as_str(), outcome.code.as_str()), ("not_kept", ENTRY_PROJECT_NOT_SAVED));
    assert!(outcome.entry_id.is_empty());
    assert!(outcome.message.contains("not yet saved"), "{}", outcome.message);
    assert_eq!(dump(&path), before, "nothing was written");
}

#[test]
fn a_save_that_bound_no_run_record_keeps_no_entry_and_says_so() {
    let (dir, path) = store_with_project("no-run");
    let before = dump(&path);
    let mut package = published(PROJECT, &dir.join("p.opsproj"), b"p", true, 1);
    package.basis = EntryBasis::NoRunRecord;
    let outcome = keep_entry_in_store(&path, &package);
    assert_eq!((outcome.status.as_str(), outcome.code.as_str()), ("not_kept", ENTRY_NO_RUN_RECORD));
    assert_eq!(dump(&path), before);
}

#[test]
fn keeping_that_fails_before_its_commit_keeps_nothing_and_withholds_the_location() {
    let (dir, path) = store_with_project("fault");
    let before = dump(&path);
    let location = dir.join("p.opsproj");
    std::fs::write(&location, b"p").unwrap();
    fault::arm(Some(format!("invented fault while writing {}", location.display())));
    let outcome = keep_entry_in_store(&path, &published(PROJECT, &location, b"p", true, 1));
    fault::arm(None);
    assert_eq!((outcome.status.as_str(), outcome.code.as_str()), ("not_kept", ENTRY_NOT_KEPT));
    assert!(outcome.entry_id.is_empty(), "nothing claims the entry was kept");
    assert!(!outcome.message.contains("invented fault"), "no text of the error is passed on: {}", outcome.message);
    withholds_location(&outcome.message);
    assert_eq!(dump(&path), before, "the insert was rolled back");
    assert_eq!(std::fs::read(&location).unwrap(), b"p");
}

#[test]
fn a_store_that_cannot_be_found_keeps_no_entry() {
    let dir = scratch("no-store");
    let location = dir.join("p.opsproj");
    let session = SessionHeldEntries::default();
    let outcome = keep_or_hold_in(Err(format!("no store near {}", location.display())), &session, None, &published(PROJECT, &location, b"p", true, 1));
    assert_eq!(outcome.code, ENTRY_NOT_KEPT);
    assert!(session.lock().entries.is_empty(), "nothing is held for a store that cannot be found");
    withholds_location(&outcome.message);
}

// --- State (REQ-023 "State") -------------------------------------------------

#[test]
fn the_state_check_reads_the_file_and_names_what_it_found() {
    let (dir, path) = store_with_project("state");
    let location = dir.join("checked.opsproj");
    std::fs::write(&location, b"the saved package").unwrap();
    let id = kept(&keep_entry_in_store(&path, &published(PROJECT, &location, b"the saved package", true, 1)));
    let state = |label: &str| {
        let before = dump(&path);
        let found = checked(&path, &SessionHeldEntries::default(), PROJECT, None, &id).unwrap();
        assert_eq!(dump(&path), before, "{label}: the check writes nothing");
        withholds_location(&serde_json::to_string(&found).unwrap());
        (found.state, found.reason)
    };
    assert_eq!(state("untouched"), ("present_identical", ""));

    // Replaced by different bytes of another length, of the same length, and
    // by a different valid package.
    std::fs::write(&location, b"other bytes").unwrap();
    assert_eq!(state("replaced"), ("changed", OTHER_SIZE));
    std::fs::write(&location, b"the saved packagX").unwrap();
    let (found, reason) = state("same length");
    assert_eq!(found, "changed");
    assert!(reason.contains("SHA-256"), "{reason}");
    let request = desktop_request(&package_model());
    let other = admit_report_package_in(save_request(&request, &package_model()), &software(), &mut no_store, &no_custody).unwrap();
    std::fs::write(&location, &other.package.container_bytes).unwrap();
    assert_eq!(state("another package").0, "changed");
    assert_eq!(std::fs::read(&location).unwrap(), other.package.container_bytes, "the check repairs nothing");

    // Moved.
    let moved = dir.join("moved.opsproj");
    std::fs::rename(&location, &moved).unwrap();
    assert_eq!(state("moved").0, "not_found");
    assert!(!location.exists(), "the check does not search for or restore a moved package");

    // Not a file.
    std::fs::create_dir(&location).unwrap();
    assert_eq!(state("a folder").0, "could_not_be_read");
    std::fs::remove_dir(&location).unwrap();

    // Unreadable.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::write(&location, b"the saved package").unwrap();
        std::fs::set_permissions(&location, std::fs::Permissions::from_mode(0o000)).unwrap();
        if std::fs::File::open(&location).is_err() {
            let (found, reason) = state("protected");
            assert_eq!(found, "could_not_be_read");
            assert!(reason.contains("Permission"), "{reason}");
        }
        std::fs::set_permissions(&location, std::fs::Permissions::from_mode(0o600)).unwrap();
        assert_eq!(state("restored").0, "present_identical");
    }

    let unknown = checked(&path, &SessionHeldEntries::default(), PROJECT, None, "saved-package:unknown").unwrap_err();
    assert!(unknown.starts_with(ENTRY_UNKNOWN), "{unknown}");
}

#[test]
fn a_copied_store_with_its_saved_locations_unavailable_reads_not_found() {
    let (dir, path) = store_with_project("copied");
    let location = dir.join("away.opsproj");
    std::fs::write(&location, b"p").unwrap();
    let id = kept(&keep_entry_in_store(&path, &published(PROJECT, &location, b"p", true, 1)));
    let copy = store_in(&scratch("copied-elsewhere"));
    {
        let connection = Connection::open(&path).unwrap();
        connection.execute("VACUUM INTO ?1", [copy.to_str().unwrap()]).unwrap();
    }
    std::fs::remove_file(&location).unwrap();
    let found = checked(&copy, &SessionHeldEntries::default(), PROJECT, None, &id).unwrap();
    assert_eq!(found.state, "not_found");
    assert_eq!(stored(&copy, PROJECT), stored(&path, PROJECT));
}

// --- This machine (REQ-023 "This machine") -------------------------------------

#[test]
fn the_location_is_in_no_reply() {
    let (dir, path) = store_with_project("this-machine");
    let location = dir.join("marked.opsproj");
    std::fs::write(&location, b"p").unwrap();
    let outcome = keep_entry_in_store(&path, &published(PROJECT, &location, b"p", false, 1));
    let id = kept(&outcome);
    withholds_location(&serde_json::to_string(&outcome).unwrap());
    let session = SessionHeldEntries::default();
    let list = listed(&path, &session, PROJECT, None);
    let list_json = serde_json::to_string(&list).unwrap();
    withholds_location(&list_json);
    assert!(!list_json.contains("location"), "{list_json}");
    withholds_location(&serde_json::to_string(&checked(&path, &session, PROJECT, None, &id).unwrap()).unwrap());
    std::fs::remove_file(&location).unwrap();
    withholds_location(&serde_json::to_string(&checked(&path, &session, PROJECT, None, &id).unwrap()).unwrap());
    withholds_location(&serde_json::to_string(&removed(&path, &session, PROJECT, None, &id).unwrap()).unwrap());
    withholds_location(&checked(&path, &session, PROJECT, None, &id).unwrap_err());
}

// --- The native save's outcome (REQ-023 "Entry") -------------------------------

fn admitted_local_private() -> AdmittedReportPackage {
    let request = desktop_request(&package_model());
    admit_report_package_in(save_request(&request, &package_model()), &software(), &mut no_store, &no_custody).unwrap()
}

#[test]
fn a_cancelled_or_failed_save_calls_no_keeper_and_adds_no_entry() {
    let _lock = crate::atomic_report_package_save::test_support::atomic_save_test_lock();
    let calls = Cell::new(0);
    let mut keep = |_: &PublishedPackage| {
        calls.set(calls.get() + 1);
        SavedPackageEntryOutcome::not_applicable()
    };
    let cancelled = save_admitted_report_package_keeping(admitted_local_private(), &no_custody, |_| None, &mut keep).unwrap();
    assert_eq!(cancelled.outcome, "cancelled");
    assert_eq!(cancelled.saved_package_entry, SavedPackageEntryOutcome::not_applicable());
    let dir = scratch("save-failed");
    let failed = save_admitted_report_package_keeping(admitted_local_private(), &no_custody, |_| Some(Ok(dir.join("missing").join("p.opsproj"))), &mut keep);
    assert!(failed.is_err(), "the write fails: no receipt, no entry");
    let unusable = save_admitted_report_package_keeping(admitted_local_private(), &no_custody, |_| Some(Err("not a path".to_string())), &mut keep);
    assert!(unusable.is_err());
    assert_eq!(calls.get(), 0, "no keeper ran");
}

#[test]
fn a_published_save_hands_the_keeper_what_it_wrote_and_bound_and_reports_the_two_outcomes_apart() {
    let _lock = crate::atomic_report_package_save::test_support::atomic_save_test_lock();
    let dir = scratch("save-published");
    let seen = RefCell::new(Vec::new());
    let mut keep = |published: &PublishedPackage| {
        seen.borrow_mut().push(published.clone());
        keep_no_entry(published)
    };
    let admitted = admitted_local_private();
    let expected_model_hash = admitted.entry_basis.clone();
    let receipt = save_admitted_report_package_keeping(admitted, &no_custody, |name| Some(Ok(dir.join(name))), &mut keep).unwrap();
    assert_eq!(receipt.outcome, "saved");
    let seen = seen.into_inner();
    assert_eq!(seen.len(), 1, "one keeper call, after publication");
    let package = &seen[0];
    assert_eq!(package.package_identity_sha256, receipt.package_identity_sha256_hex);
    assert_eq!(package.container_sha256, receipt.container_sha256_hex);
    assert_eq!(package.container_byte_count, receipt.byte_count as u64);
    assert_eq!(package.file_name(), receipt.selected_basename);
    assert_eq!(package.durability_confirmed, receipt.durability == "durable");
    assert_eq!(package.export_context, "local_private");
    assert_eq!(package.project_id, package_model()["project"]["id"].as_str().unwrap());
    assert_eq!(package.basis, EntryBasis::NoRunRecord, "route 009 binds no run record");
    assert_eq!(expected_model_hash, EntryBasis::NoRunRecord);
    assert!(!package.saved_at.is_empty());
    // The receipt reports the entry's outcome apart from the package's.
    assert_eq!((receipt.saved_package_entry.status.as_str(), receipt.code.as_str()), ("not_kept", "REPORT-PACKAGE-SAVED"));
    withholds_location(&serde_json::to_string(&receipt).unwrap());
}

#[test]
fn a_keeper_that_fails_leaves_the_published_package_as_it_was_saved() {
    let _lock = crate::atomic_report_package_save::test_support::atomic_save_test_lock();
    let dir = scratch("save-keeper-fails");
    let admitted = admitted_local_private();
    let bytes = admitted.package.container_bytes.clone();
    let mut keep = |_: &PublishedPackage| -> SavedPackageEntryOutcome { panic!("invented keeper failure") };
    let receipt = save_admitted_report_package_keeping(admitted, &no_custody, |name| Some(Ok(dir.join(name))), &mut keep).unwrap();
    assert_eq!(receipt.outcome, "saved", "the package's outcome stands");
    assert_eq!((receipt.saved_package_entry.status.as_str(), receipt.saved_package_entry.code.as_str()), ("not_kept", ENTRY_NOT_KEPT));
    assert!(receipt.saved_package_entry.entry_id.is_empty());
    assert_eq!(std::fs::read(dir.join(&receipt.container_file_name)).unwrap(), bytes, "neither deleted nor rewritten");
}

#[test]
fn a_save_whose_durability_was_not_confirmed_hands_an_entry_that_says_so() {
    let admitted = admitted_local_private();
    let location = scratch("uncertain").join("p.opsproj");
    let written = |durability: &str| crate::atomic_report_package_save::AtomicSaveOutcome {
        replaced_existing: false,
        durability: durability.to_string(),
        selected_basename: "p.opsproj".to_string(),
        path_containment: String::new(),
        limitation: String::new(),
    };
    assert!(!admitted.published(&location, &written("saved_durability_uncertain"), String::new()).durability_confirmed);
    assert!(admitted.published(&location, &written("durable"), String::new()).durability_confirmed);
}

// --- Reading a saved package ---------------------------------------------------

/// No member of a saved report package is a saved-package entry, and reading
/// one adds no entry and removes none. (The desktop has no portable project
/// package writer or reader for REQ-023 "This machine" to be held against.)
#[test]
fn reading_a_report_package_adds_no_entry_and_removes_none() {
    let (dir, path) = store_with_project("package-read");
    let location = dir.join("p.opsproj");
    std::fs::write(&location, b"p").unwrap();
    kept(&keep_entry_in_store(&path, &published(PROJECT, &location, b"p", true, 1)));
    let before = dump(&path);
    let admitted = admitted_local_private();
    let read = open_pipe_stress_report_package::read_report_package_container(&admitted.package.container_bytes).unwrap();
    assert!(read.members.iter().all(|member| !member.file_name.contains("saved_package") && !member.role.contains("saved_package")));
    assert_eq!(dump(&path), before);
}

// --- Route 009's run record (REQ-023 "Basis"; part 1b) -------------------------

const VECTORS: &str = include_str!("../../../../fixtures/reports/invented/user_release_derivation_vectors.json");
const VECTOR_SOURCE: &str = include_str!("../../../../fixtures/results/preview_physics_connected_sparse.json");

/// A local-private save of the user-release vectors' basis: the committed
/// model, its committed sparse solve, its input manifest and its transient
/// run record, with the desktop request derived from them.
struct Route009 {
    model: serde_json::Value,
    manifest: serde_json::Value,
    run_record: serde_json::Value,
    request: serde_json::Value,
}

fn route_009() -> Route009 {
    let vector = serde_json::from_str::<serde_json::Value>(VECTORS).unwrap()["vectors"][0].clone();
    let model = package_model();
    let source: serde_json::Value = serde_json::from_str(VECTOR_SOURCE).unwrap();
    let manifest = vector["input_manifest"].clone();
    let request = crate::report_package_save::test_support::desktop_request_from(&model, &manifest, &source);
    Route009 { model, manifest, run_record: vector["run_record"].clone(), request }
}

impl Route009 {
    fn args(&self, run_text: Option<String>) -> crate::report_package_save::ReportPackageSaveRequest {
        let mut args = crate::report_package_save::test_support::save_request_with(&self.request, &self.model, &self.manifest);
        args.analysis_run_text = run_text;
        args
    }
}

/// Admits `args` on the command's path: the route-aware lease growth, then
/// admission with the session (route 009 checks a record within its memory).
fn admit_009(args: crate::report_package_save::ReportPackageSaveRequest) -> Result<AdmittedReportPackage, String> {
    let registry = crate::command_ingress::test_support::registry(crate::memory_pool::test_host::ScriptedHost::normal());
    let mut lease = registry.acquire_interactive_ingress(1024).unwrap();
    let growth = crate::report_package_save::ingress_lease_bytes(&args).map_err(|error| error.code)?;
    lease.grow_to(1024 + growth).unwrap();
    let mut session = crate::report_package_save::UserReleaseSession { registry: &registry, lease: &mut lease };
    crate::report_package_save::admit_report_package_session(args, &software(), &mut no_store, &no_custody, Some(&mut session)).map_err(|refusal| match refusal {
        crate::report_package_save::SaveRefusal::Command(error) => error.code,
        crate::report_package_save::SaveRefusal::Resource(refusal) => format!("{refusal:?}"),
    })
}

/// Saves an admitted route-009 package into `dir`, keeping its entry in the
/// store at `path` with `session`.
fn save_009(admitted: AdmittedReportPackage, dir: &Path, path: &Path, session: &SessionHeldEntries) -> crate::report_package_save::ReportPackageSaveReceipt {
    let mut keep = |published: &PublishedPackage| keep_or_hold(path, session, None, published);
    save_admitted_report_package_keeping(admitted, &no_custody, |name| Some(Ok(dir.join(name))), &mut keep).unwrap()
}

fn vector_project_store(label: &str) -> (PathBuf, PathBuf, Route009) {
    let basis = route_009();
    let dir = scratch(label);
    let path = store_in(&dir);
    write_project(&path, basis.model["project"]["id"].as_str().unwrap(), &basis.model);
    (dir, path, basis)
}

#[test]
fn a_local_private_save_with_a_verified_run_record_keeps_its_entry_with_that_basis() {
    let _lock = crate::atomic_report_package_save::test_support::atomic_save_test_lock();
    let (dir, path, basis) = vector_project_store("009-verified");
    let admitted = admit_009(basis.bound_args(&basis.run_record)).unwrap();
    let receipt = save_009(admitted, &dir, &path, &SessionHeldEntries::default());
    assert_eq!(receipt.outcome, "saved");
    assert_eq!((receipt.saved_package_entry.status.as_str(), receipt.saved_package_entry.code.as_str()), ("kept", ENTRY_KEPT), "{:?}", receipt.saved_package_entry);
    let entry = &stored(&path, basis.model["project"]["id"].as_str().unwrap())[0];
    let record = &basis.run_record["analysis_run"];
    let checksum = |scope: &str| record["hashes"].as_array().unwrap().iter().find(|hash| hash["payload_scope"] == scope).unwrap()["value"].as_str().unwrap().to_string();
    assert_eq!(entry.export_context, "local_private");
    assert_eq!(entry.basis.model_hash, open_pipe_stress_operation_applier::sha256_hex(&canonical(&basis.model)));
    assert_eq!(entry.basis.run_id, record["run_id"].as_str().unwrap());
    assert_eq!(entry.basis.run_record_sha256, checksum("analysis_run_record"));
    assert_eq!(entry.basis.received_result_sha256, checksum("received_result"));
    assert_eq!((entry.package_identity_sha256.as_str(), entry.container_sha256.as_str()), (receipt.package_identity_sha256_hex.as_str(), receipt.container_sha256_hex.as_str()));
}

#[test]
fn a_local_private_save_without_a_run_record_is_saved_as_today_and_keeps_no_entry() {
    let _lock = crate::atomic_report_package_save::test_support::atomic_save_test_lock();
    let (dir, path, basis) = vector_project_store("009-absent");
    let before = dump(&path);
    let receipt = save_009(admit_009(basis.args(None)).unwrap(), &dir, &path, &SessionHeldEntries::default());
    assert_eq!((receipt.outcome.as_str(), receipt.code.as_str()), ("saved", "REPORT-PACKAGE-SAVED"));
    assert_eq!((receipt.saved_package_entry.status.as_str(), receipt.saved_package_entry.code.as_str()), ("not_kept", ENTRY_NO_RUN_RECORD));
    assert_eq!(dump(&path), before, "nothing was written");
}

#[test]
fn a_local_private_save_with_a_run_record_that_does_not_verify_is_saved_and_keeps_no_entry() {
    let _lock = crate::atomic_report_package_save::test_support::atomic_save_test_lock();
    let (dir, path, basis) = vector_project_store("009-unverified");
    let before = dump(&path);
    let mut altered = basis.run_record.clone();
    altered["analysis_run"]["run_name"] = json!("an altered record, not resealed");
    let mut other_result = basis.run_record.clone();
    for hash in other_result["analysis_run"]["hashes"].as_array_mut().unwrap() {
        if hash["payload_scope"] == "received_result" {
            hash["value"] = json!("0".repeat(64));
        }
    }
    let texts = [
        ("an altered record", canonical(&altered)),
        ("a record of another result", canonical(&other_result)),
        ("a record that is not canonical", serde_json::to_string_pretty(&basis.run_record).unwrap()),
        ("a text that is not JSON", "{ not json".to_string()),
    ];
    for (label, text) in texts {
        let receipt = save_009(admit_009(basis.args(Some(text))).unwrap_or_else(|code| panic!("{label}: refused {code}")), &dir, &path, &SessionHeldEntries::default());
        assert_eq!(receipt.outcome, "saved", "{label}");
        let outcome = &receipt.saved_package_entry;
        assert_eq!((outcome.status.as_str(), outcome.code.as_str()), ("not_kept", ENTRY_RUN_RECORD_UNVERIFIED), "{label}: {outcome:?}");
        assert!(outcome.message.contains("does not verify"), "{label}: {}", outcome.message);
        withholds_location(&outcome.message);
    }
    assert_eq!(dump(&path), before, "nothing was written");
}

#[test]
fn a_run_record_changes_neither_the_package_nor_route_009_admission() {
    let basis = route_009();
    let with = admit_009(basis.args(Some(canonical(&basis.run_record)))).unwrap();
    let without = admit_009(basis.args(None)).unwrap();
    assert_eq!(with.package.container_bytes, without.package.container_bytes, "the same package, byte for byte");
    assert_eq!(with.package.package_identity_sha256_hex, without.package.package_identity_sha256_hex);
    // A save refused today is refused with a record too, by the same code.
    let mut unbound = basis.request.clone();
    unbound["audit_manifest"]["model_hash"]["value"] = json!("0".repeat(64));
    let refused = |run_text: Option<String>| {
        let mut args = crate::report_package_save::test_support::save_request_with(&unbound, &basis.model, &basis.manifest);
        args.analysis_run_text = run_text;
        admit_009(args).map(|_| ()).unwrap_err()
    };
    assert_eq!(refused(None), crate::report_package_save::MODEL_UNBOUND);
    assert_eq!(refused(Some(canonical(&basis.run_record))), crate::report_package_save::MODEL_UNBOUND);
}

/// A current model document of `PROJECT`, which the store's create path admits.
fn current_model() -> serde_json::Value {
    let mut model = crate::analysis_runs::test_fixtures::model("Invented project");
    model["project"]["id"] = json!(PROJECT);
    model
}

fn files_under(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            files.extend(files_under(&path));
        } else {
            files.push(path);
        }
    }
    files
}

fn canonical(value: &serde_json::Value) -> String {
    open_pipe_stress_operation_applier::canonical_json(value)
}

// --- Before the first project save (REQ-023; AC-014 "First save and recovery") --

/// A store with no project, a published package of `PROJECT` saved into its
/// folder, and a session.
fn unsaved(label: &str) -> (PathBuf, PathBuf, PublishedPackage) {
    let dir = scratch(label);
    let path = store_in(&dir);
    drop(crate::open_project_store(&path).unwrap());
    let location = dir.join("before-first-save.opsproj");
    std::fs::write(&location, b"saved before the first project save").unwrap();
    let package = published(PROJECT, &location, b"saved before the first project save", true, 1);
    (dir, path, package)
}

/// A held outcome's entry id and holder token.
#[track_caller]
fn held(outcome: &SavedPackageEntryOutcome) -> (String, String) {
    assert_eq!((outcome.status.as_str(), outcome.code.as_str()), ("held", ENTRY_HELD_BY_SESSION), "{outcome:?}");
    assert!(outcome.message.contains("not yet kept with the project"), "{}", outcome.message);
    let holder = outcome.holder.clone().expect("a held outcome names its holder");
    assert!(!outcome.entry_id.is_empty() && !holder.is_empty());
    (outcome.entry_id.clone(), holder)
}

#[track_caller]
fn all_kept(write: &HeldEntriesWrite) -> Vec<String> {
    for outcome in &write.entries {
        assert_eq!((outcome.status.as_str(), outcome.code.as_str()), ("kept", ENTRY_KEPT), "{outcome:?}");
    }
    write.entries.iter().map(|outcome| outcome.entry_id.clone()).collect()
}

#[test]
fn an_entry_made_before_the_first_save_is_held_by_the_session_and_listed_as_not_yet_kept() {
    let (_dir, path, package) = unsaved("held");
    let before = dump(&path);
    let session = SessionHeldEntries::default();
    let (id, holder) = held(&keep_or_hold(&path, &session, None, &package));
    assert_eq!(dump(&path), before, "the store holds nothing of it");
    let list = listed(&path, &session, PROJECT, Some(&holder));
    assert_eq!(list.entries.len(), 1);
    let entry = &list.entries[0];
    assert_eq!((entry.entry_id.as_str(), entry.kept, entry.sequence), (id.as_str(), "held_by_session", 0));
    assert_eq!(entry.container_sha256, package.container_sha256);
    withholds_location(&serde_json::to_string(&list).unwrap());
    // Without its holder a caller sees stored entries only.
    assert!(listed(&path, &session, PROJECT, None).entries.is_empty());
    assert!(checked(&path, &session, PROJECT, None, &id).unwrap_err().starts_with(ENTRY_UNKNOWN));
    assert!(removed(&path, &session, PROJECT, None, &id).unwrap_err().starts_with(ENTRY_UNKNOWN));
    // With it, the state check and removal work on it by its entry id.
    assert_eq!(checked(&path, &session, PROJECT, Some(&holder), &id).unwrap().state, "present_identical");
    std::fs::write(&package.location, b"other").unwrap();
    assert_eq!(checked(&path, &session, PROJECT, Some(&holder), &id).unwrap().state, "changed");
    let removal = removed(&path, &session, PROJECT, Some(&holder), &id).unwrap();
    assert!(removal.removed);
    assert!(listed(&path, &session, PROJECT, Some(&holder)).entries.is_empty());
    assert_eq!(std::fs::read(&package.location).unwrap(), b"other", "the file is untouched");
}

#[test]
fn the_first_project_save_writes_the_held_entries_in_the_order_saved() {
    let (dir, path, first) = unsaved("first-save");
    let session = SessionHeldEntries::default();
    let (one, holder) = held(&keep_or_hold(&path, &session, None, &first));
    let second = published(PROJECT, &dir.join("second.opsproj"), b"second", false, 2);
    let (two, joined) = held(&keep_or_hold(&path, &session, Some(&holder), &second));
    assert_eq!(joined, holder, "a later held save presenting the token joins its holder");
    let held_view = listed(&path, &session, PROJECT, Some(&holder)).entries;
    // The project's first save, by the store's own create path, writes no
    // entry; the page then writes its holder's.
    crate::create_project_in_store(path.clone(), current_model(), None, None, None, None, None, None, None, None).unwrap();
    assert!(entry_rows(&path).is_empty(), "the project save writes no held entry");
    assert_eq!(all_kept(&keep_held(Ok(path.clone()), &session, PROJECT, &holder)), [one.clone(), two.clone()]);
    let list = listed(&path, &session, PROJECT, Some(&holder));
    assert_eq!(list.entries.iter().map(|entry| (entry.entry_id.clone(), entry.sequence, entry.kept)).collect::<Vec<_>>(),
        [(one.clone(), 1, "kept_with_project"), (two.clone(), 2, "kept_with_project")]);
    for (stored, was_held) in list.entries.iter().zip(&held_view) {
        let mut as_held = stored.clone();
        as_held.sequence = 0;
        as_held.kept = "held_by_session";
        assert_eq!(&as_held, was_held, "written as it was held");
    }
    // Later saves, with the token and without, are kept directly, after them.
    let third = kept(&keep_or_hold(&path, &session, Some(&holder), &published(PROJECT, &dir.join("third.opsproj"), b"third", true, 3)));
    let fourth = kept(&keep_or_hold(&path, &session, None, &published(PROJECT, &dir.join("fourth.opsproj"), b"fourth", true, 4)));
    let order: Vec<(String, i64)> = stored(&path, PROJECT).into_iter().map(|entry| (entry.entry_id, entry.sequence)).collect();
    assert_eq!(order, [(one, 1), (two, 2), (third, 3), (fourth, 4)], "the order across held and later stored entries is kept");
    // Writing again writes nothing more.
    assert!(keep_held(Ok(path.clone()), &session, PROJECT, &holder).entries.is_empty());
    assert_eq!(entry_rows(&path).len(), 4);
}

#[test]
fn a_held_entry_that_cannot_be_written_stays_held_with_its_reason_and_the_project_save_stands() {
    let (dir, path, package) = unsaved("first-save-fails");
    let session = SessionHeldEntries::default();
    let (id, holder) = held(&keep_or_hold(&path, &session, None, &package));
    crate::create_project_in_store(path.clone(), current_model(), None, None, None, None, None, None, None, None).unwrap();
    fault::arm(Some(format!("invented fault writing {}", package.location.display())));
    let write = keep_held(Ok(path.clone()), &session, PROJECT, &holder);
    fault::arm(None);
    assert_eq!(write.entries.len(), 1);
    let outcome = &write.entries[0];
    assert_eq!((outcome.status.as_str(), outcome.code.as_str(), outcome.entry_id.as_str()), ("held", ENTRY_STORE_FAILED, id.as_str()));
    withholds_location(&serde_json::to_string(&write).unwrap());
    assert!(crate::load_project(&Connection::open(&path).unwrap(), Some(PROJECT)).unwrap().is_some(), "the project save stands");
    assert!(stored(&path, PROJECT).is_empty(), "nothing was written");
    let list = listed(&path, &session, PROJECT, Some(&holder));
    let entry = &list.entries[0];
    assert_eq!((entry.entry_id.as_str(), entry.kept), (id.as_str(), "held_by_session"));
    assert_eq!(entry.write_failure, Some(STORE_FAILED), "its reason is a code and a fixed sentence");
    // The next package save presenting the token is held with it, and says
    // so; the command, called again, writes both, in order.
    let next = keep_or_hold(&path, &session, Some(&holder), &published(PROJECT, &dir.join("next.opsproj"), b"next", true, 2));
    assert_eq!((next.status.as_str(), next.holder.as_deref(), next.message.as_str()), ("held", Some(holder.as_str()), HELD_WITH_EARLIER));
    assert!(stored(&path, PROJECT).is_empty(), "a package save writes no held entry");
    assert_eq!(all_kept(&keep_held(Ok(path.clone()), &session, PROJECT, &holder)), [id.clone(), next.entry_id.clone()]);
    let order: Vec<(String, i64)> = stored(&path, PROJECT).into_iter().map(|entry| (entry.entry_id, entry.sequence)).collect();
    assert_eq!(order, [(id, 1), (next.entry_id, 2)]);
}

#[test]
fn held_entries_stay_with_their_own_project_when_another_is_opened_or_created() {
    let (_dir, path, package) = unsaved("replaced");
    let session = SessionHeldEntries::default();
    let (id, holder) = held(&keep_or_hold(&path, &session, None, &package));
    // Another project is created and saved first; its page writes its own
    // holder's entries, and this holder's only for this project.
    let other = json!({ "schema_version": open_pipe_stress_product_physics::MODEL_SCHEMA_VERSION, "project": { "id": "project:another", "name": "Another" } });
    write_project(&path, "project:another", &other);
    assert!(keep_held(Ok(path.clone()), &session, "project:another", &holder).entries.is_empty());
    assert!(stored(&path, "project:another").is_empty(), "nothing is written into another project");
    assert_eq!(listed(&path, &session, PROJECT, Some(&holder)).entries[0].kept, "held_by_session");
    // Returning to the project and saving it, its page writes the entry.
    write_project(&path, PROJECT, &invented_model());
    assert_eq!(all_kept(&keep_held(Ok(path.clone()), &session, PROJECT, &holder)), [id.clone()]);
    assert_eq!(stored(&path, PROJECT)[0].entry_id, id);
}

#[test]
fn a_session_that_ends_before_the_first_save_loses_its_held_entry_and_recovery_does_not_reconstruct_it() {
    let (dir, path, package) = unsaved("lost");
    let autosave = dir.join("autosave");
    std::fs::create_dir_all(&autosave).unwrap();
    // A full committed model document, as the session's autosave writes one.
    let model = package_model();
    let project_id = model["project"]["id"].as_str().unwrap().to_string();
    let project = project_id.as_str();
    let mut package = package;
    package.project_id = project.to_string();
    let session = SessionHeldEntries::default();
    let (id, holder) = held(&keep_or_hold(&path, &session, None, &package));
    // A recovery record written while the entry is held holds the document only.
    let text = canonical(&model);
    let hash = format!("sha256:{}", open_pipe_stress_operation_applier::sha256_hex(&text));
    let written = crate::autosave::write_autosave(&autosave, &text, &hash, 1_793_100_645_123, &crate::stored_read::UnmeteredReadMemory);
    assert!(matches!(written, crate::autosave::AutosaveWriteOutcome::Written { .. }), "{written:?}");
    let records = files_under(&autosave);
    assert!(!records.is_empty());
    for record in &records {
        let bytes = std::fs::read(record).unwrap();
        let shown = String::from_utf8_lossy(&bytes);
        for member in [id.as_str(), holder.as_str(), package.container_sha256.as_str(), package.package_identity_sha256.as_str(), "before-first-save.opsproj", LOCATION_MARKER] {
            assert!(!shown.contains(member), "{}: holds {member}", record.display());
        }
    }
    // The desktop ends: the session's memory is gone.
    drop(session);
    let restarted = SessionHeldEntries::default();
    // Recovery: the recovered document is saved as the project's first save.
    crate::create_project_in_store(path.clone(), model, None, None, None, None, None, None, None, None).unwrap();
    assert!(keep_held(Ok(path.clone()), &restarted, project, &holder).entries.is_empty());
    assert!(listed(&path, &restarted, project, Some(&holder)).entries.is_empty(), "nothing reconstructs the entry");
    assert!(package.location.exists(), "the saved file is a file the project does not list");
}

#[test]
fn an_entry_the_store_already_holds_is_unaffected_by_a_recovery() {
    let (dir, path) = store_with_project("recovered");
    let location = dir.join("stored.opsproj");
    std::fs::write(&location, b"stored").unwrap();
    let session = SessionHeldEntries::default();
    kept(&keep_or_hold(&path, &session, None, &published(PROJECT, &location, b"stored", true, 1)));
    let rows = entry_rows(&path);
    // Recovery replaces the project's document by the recovered one and saves it.
    write_project(&path, PROJECT, &json!({ "project": { "id": PROJECT, "name": "Recovered" } }));
    assert_eq!(entry_rows(&path), rows, "the stored entry is byte for byte as it was");
}

// --- Review round 1: route 009's run record ------------------------------

/// R2: a run record whose run name is changed and whose own checksum is
/// recomputed does not verify on route 009, as on route 010.
#[test]
fn a_renamed_record_with_its_checksum_recomputed_does_not_verify_on_route_009() {
    let _lock = crate::atomic_report_package_save::test_support::atomic_save_test_lock();
    let (dir, path, basis) = vector_project_store("r2-renamed");
    let before = dump(&path);
    let record = rehashed(&basis.run_record, |run| run["run_name"] = json!("another run name"));
    let receipt = save_009(admit_009(basis.args(Some(canonical(&record)))).unwrap(), &dir, &path, &SessionHeldEntries::default());
    assert_eq!(receipt.outcome, "saved");
    assert_eq!(receipt.saved_package_entry.code, ENTRY_RUN_RECORD_UNVERIFIED, "{:?}", receipt.saved_package_entry);
    assert_eq!(dump(&path), before, "nothing was written");
}

/// R2: route 009 applies the solve builder's status rule, not route 010's
/// no-aggregate rule, so the record of a model with rule packs, whose status
/// carries the rule-check aggregate the builder writes, still verifies.
#[test]
fn a_record_with_the_rule_check_aggregate_verifies_on_route_009() {
    let _lock = crate::atomic_report_package_save::test_support::atomic_save_test_lock();
    let (dir, path, basis) = vector_project_store("r2-aggregate");
    let record = rehashed(&basis.run_record, |run| {
        let mut statuses: Vec<String> =
            run["analysis_status"].as_array().unwrap().iter().map(|status| status.as_str().unwrap().replace("RULE_INPUTS_INCOMPLETE", "USER_RULE_CHECKED")).collect();
        statuses.sort();
        run["analysis_status"] = json!(statuses);
    });
    let receipt = save_009(admit_009(basis.bound_args(&record)).unwrap(), &dir, &path, &SessionHeldEntries::default());
    assert_eq!(receipt.saved_package_entry.code, ENTRY_KEPT, "{:?}", receipt.saved_package_entry);
}

/// The record with `edit` applied to its run and its own checksum recomputed.
fn rehashed(record: &serde_json::Value, edit: impl FnOnce(&mut serde_json::Value)) -> serde_json::Value {
    let mut record = record.clone();
    edit(&mut record["analysis_run"]);
    let hash = crate::analysis_runs::run_record_hash(&record).unwrap();
    for entry in record["analysis_run"]["hashes"].as_array_mut().unwrap() {
        if entry["payload_scope"] == "analysis_run_record" {
            entry["value"] = json!(hash);
        }
    }
    record
}

/// R3: the record is optional to the save, so the save's own lease does not
/// grow for it.
#[test]
fn the_save_lease_does_not_grow_for_the_optional_record() {
    let basis = route_009();
    let without = crate::report_package_save::ingress_lease_bytes(&basis.args(None)).unwrap();
    let with = crate::report_package_save::ingress_lease_bytes(&basis.args(Some(canonical(&basis.run_record)))).unwrap();
    assert_eq!(with, without);
}

/// R3: a run text the prescan refuses is never handed to the parser: a long
/// valid prefix with a syntax error at its end allocates nothing for it.
#[test]
fn a_prescan_refused_record_is_never_parsed() {
    let _lock = crate::atomic_report_package_save::test_support::atomic_save_test_lock();
    let basis = route_009();
    let mut bad = String::from("[");
    bad.push_str(&"1,".repeat(1_000_000));
    bad.push('x');
    let (plain, plain_peak) = crate::ingress::counting::measure(|| admit_009(basis.args(None)).map(|admitted| admitted.entry_basis));
    let (with_bad, bad_peak) = crate::ingress::counting::measure(|| admit_009(basis.args(Some(bad.clone()))).map(|admitted| admitted.entry_basis));
    assert_eq!((plain, with_bad), (Ok(EntryBasis::NoRunRecord), Ok(EntryBasis::Unverified)));
    let extra = bad_peak.peak_priced.saturating_sub(plain_peak.peak_priced);
    assert!(extra < 4 << 20, "a prescan-refused record allocated {extra} more bytes at its peak");
}

/// R3: when the record's extra lease is refused, the package is saved exactly
/// as without the record and the entry is not kept, with its own code.
#[test]
fn a_record_whose_extra_lease_is_refused_leaves_the_save_as_without_it() {
    let _lock = crate::atomic_report_package_save::test_support::atomic_save_test_lock();
    let (dir, path, basis) = vector_project_store("r3-lease-refused");
    let args = basis.args(Some(canonical(&basis.run_record)));
    let registry = crate::command_ingress::test_support::registry(crate::memory_pool::test_host::ScriptedHost::normal());
    let growth = crate::report_package_save::ingress_lease_bytes(&args).unwrap();
    let mut lease = registry.acquire_interactive_ingress(1024).unwrap();
    lease.grow_to(1024 + growth).unwrap();
    // The budget admits the save's own lease and nothing more.
    let (pool_bytes, _) = registry.pool().settings();
    registry.configure_pool(pool_bytes, lease.bytes(), false).unwrap();
    let admitted = {
        let mut session = crate::report_package_save::UserReleaseSession { registry: &registry, lease: &mut lease };
        crate::report_package_save::admit_report_package_session(args, &software(), &mut no_store, &no_custody, Some(&mut session))
            .unwrap_or_else(|refusal| panic!("the save is refused for its optional record: {refusal:?}"))
    };
    assert_eq!(admitted.entry_basis, EntryBasis::NotChecked);
    let without = admit_009(basis.args(None)).unwrap();
    assert_eq!(admitted.package.container_bytes, without.package.container_bytes, "the package exactly as without the record");
    let receipt = save_009(admitted, &dir, &path, &SessionHeldEntries::default());
    assert_eq!(receipt.outcome, "saved");
    assert_eq!((receipt.saved_package_entry.status.as_str(), receipt.saved_package_entry.code.as_str()), ("not_kept", ENTRY_RUN_RECORD_NOT_CHECKED));
}

// --- Review round 1 --------------------------------------------------------

/// R1: a second document of the same project id, after the first, neither
/// sees nor inherits the first one's held entries; its first save writes only
/// its own.
#[test]
fn a_second_document_of_the_same_id_neither_sees_nor_inherits_the_first_ones_held_entries() {
    let (dir, path, package) = unsaved("r1-same-id");
    let session = SessionHeldEntries::default();
    let (first_id, first_holder) = held(&keep_or_hold(&path, &session, None, &package));
    // The second document saves a package with no token of its own yet.
    let second_package = published(PROJECT, &dir.join("second.opsproj"), b"second", true, 2);
    let (second_id, second_holder) = held(&keep_or_hold(&path, &session, None, &second_package));
    assert_ne!(first_holder, second_holder);
    assert_eq!(ids(&listed(&path, &session, PROJECT, Some(&second_holder)).entries), [second_id.clone()]);
    assert!(listed(&path, &session, PROJECT, None).entries.is_empty(), "without a token a caller sees stored entries only");
    assert!(checked(&path, &session, PROJECT, Some(&second_holder), &first_id).unwrap_err().starts_with(ENTRY_UNKNOWN));
    assert!(removed(&path, &session, PROJECT, Some(&second_holder), &first_id).unwrap_err().starts_with(ENTRY_UNKNOWN));
    // A token the session did not mint for the project is not joined.
    let invented = "saved-package-holder:invented";
    let (_, minted) = held(&keep_or_hold(&path, &session, Some(invented), &published(PROJECT, &dir.join("third.opsproj"), b"third", true, 3)));
    assert_ne!(minted, invented);
    // The second document's first save, then its call: only its own entry.
    crate::create_project_in_store(path.clone(), current_model(), None, None, None, None, None, None, None, None).unwrap();
    assert_eq!(all_kept(&keep_held(Ok(path.clone()), &session, PROJECT, &second_holder)), [second_id.clone()]);
    assert_eq!(ids(&stored(&path, PROJECT)), [second_id]);
    let first_view = listed(&path, &session, PROJECT, Some(&first_holder)).entries;
    assert_eq!(first_view.iter().map(|entry| (entry.entry_id.clone(), entry.kept)).last(), Some((first_id, "held_by_session")));
}

/// R1: no project save writes a held entry, whether it is refused or saves
/// an unrelated project; another window's project of the same id is not
/// written into without the page's call; the call writes only its own
/// project and holder.
#[test]
fn no_project_save_writes_a_held_entry_and_the_call_writes_only_its_own_project_and_holder() {
    let (_dir, path, package) = unsaved("r1-no-call");
    let session = SessionHeldEntries::default();
    let (id, holder) = held(&keep_or_hold(&path, &session, None, &package));
    // Another window creates the project; this document's create is refused.
    crate::create_project_in_store(path.clone(), current_model(), None, None, None, None, None, None, None, None).unwrap();
    let refused = crate::create_project_in_store(path.clone(), current_model(), None, None, None, None, None, None, None, None).unwrap_err();
    assert!(refused.starts_with("PROJECT-ID-EXISTS"), "{refused}");
    // An unrelated project is saved.
    write_project(&path, "project:unrelated", &json!({ "project": { "id": "project:unrelated", "name": "Unrelated" } }));
    assert!(entry_rows(&path).is_empty(), "no project save wrote the held entry");
    // The call for another project, or with another holder, writes nothing.
    assert!(keep_held(Ok(path.clone()), &session, "project:unrelated", &holder).entries.is_empty());
    assert!(keep_held(Ok(path.clone()), &session, PROJECT, "saved-package-holder:another").entries.is_empty());
    assert!(entry_rows(&path).is_empty());
    assert_eq!(ids(&listed(&path, &session, PROJECT, Some(&holder)).entries), [id]);
}

/// R4: the state check answers `changed` from the length, without reading a
/// file far larger than the saved package.
#[test]
fn the_state_check_answers_changed_from_the_length_without_reading_a_larger_file() {
    let dir = scratch("r4-large");
    let location = dir.join("large.opsproj");
    let file = std::fs::File::create(&location).unwrap();
    file.set_len(16 << 30).unwrap();
    drop(file);
    let started = std::time::Instant::now();
    let found = state_at(&location, &sha(b"p"), 1, &UnmeteredReadMemory).unwrap();
    let took = started.elapsed();
    std::fs::remove_file(&location).unwrap();
    assert_eq!(found, ("changed", OTHER_SIZE));
    assert!(took < std::time::Duration::from_secs(2), "the check took {took:?}");
}

/// R4: a pipe at the location is neither read nor waited on; a symbolic link
/// answers for the file it reaches.
#[cfg(unix)]
#[test]
fn a_pipe_is_not_waited_on_and_a_link_answers_for_what_it_reaches() {
    let dir = scratch("r4-special");
    let pipe = dir.join("pipe.opsproj");
    assert!(std::process::Command::new("mkfifo").arg(&pipe).status().unwrap().success());
    let (sender, receiver) = std::sync::mpsc::channel();
    let at = pipe.clone();
    std::thread::spawn(move || sender.send(state_at(&at, &sha(b"p"), 1, &UnmeteredReadMemory)).unwrap());
    let found = receiver.recv_timeout(std::time::Duration::from_secs(5)).expect("the check waited on a pipe");
    assert_eq!(found.unwrap(), ("could_not_be_read", NOT_REGULAR));

    let saved = dir.join("saved.opsproj");
    std::fs::write(&saved, b"p").unwrap();
    let link = dir.join("link.opsproj");
    std::os::unix::fs::symlink(&saved, &link).unwrap();
    assert_eq!(state_at(&link, &sha(b"p"), 1, &UnmeteredReadMemory).unwrap(), ("present_identical", ""));
    std::fs::write(&saved, b"q").unwrap();
    assert_eq!(state_at(&link, &sha(b"p"), 1, &UnmeteredReadMemory).unwrap(), ("changed", OTHER_BYTES));
    std::fs::remove_file(&saved).unwrap();
    assert_eq!(state_at(&link, &sha(b"p"), 1, &UnmeteredReadMemory).unwrap(), ("not_found", NOT_FOUND));
}

/// R4: the commands take their arguments through the store's ingress and
/// meter what they read and reply; a read the pool cannot admit is refused
/// by name, and every reservation is released with the reply.
#[test]
fn the_commands_are_admitted_and_metered_like_the_stores_other_commands() {
    use crate::command_ingress::test_support::{admit_json, registry};
    let (_dir, path, package) = unsaved("r4-ingress");
    let session = SessionHeldEntries::default();
    let (id, holder) = held(&keep_or_hold(&path, &session, None, &package));
    let registry = registry(crate::memory_pool::test_host::ScriptedHost::normal());
    let store = || Ok(path.clone());
    let body = json!({ "projectId": PROJECT, "holder": holder, "entryId": id });
    {
        let (mut args, lease) = admit_json(&registry, "list_saved_package_entries", ArgsDecoding::Moved, &body).unwrap();
        let memory = PoolReadMemory::new(&registry, &lease);
        let outcome = list_saved_package_entries_args(&mut args, store, &session, &memory);
        stored_read::respond_metered(&memory, "list_saved_package_entries", outcome).unwrap();
    }
    {
        let (mut args, lease) = admit_json(&registry, "check_saved_package_entry", ArgsDecoding::Moved, &body).unwrap();
        let memory = PoolReadMemory::new(&registry, &lease);
        let outcome = check_saved_package_entry_args(&mut args, store, &session, &memory);
        stored_read::respond_metered(&memory, "check_saved_package_entry", outcome).unwrap();
    }
    assert_eq!(registry.pool().reserved_total(), 0, "every reservation is released with its reply");
    // A budget that admits the arguments and nothing more.
    let (mut args, lease) = admit_json(&registry, "list_saved_package_entries", ArgsDecoding::Moved, &body).unwrap();
    let (pool_bytes, _) = registry.pool().settings();
    registry.configure_pool(pool_bytes, lease.bytes(), false).unwrap();
    let memory = PoolReadMemory::new(&registry, &lease);
    match list_saved_package_entries_args(&mut args, store, &session, &memory) {
        Err(CommandError::Command(refusal)) => assert!(refusal.starts_with(stored_read::STORE_READ_MEMORY_UNAVAILABLE), "{refusal}"),
        other => panic!("not refused by its memory: {other:?}"),
    }
    // An argument body above the cap is refused before it is parsed.
    let long = json!({ "projectId": "p".repeat(ENTRY_COMMAND_MAX_ARGS_BYTES) });
    let body = tauri::ipc::InvokeBody::Raw(serde_json::to_vec(&long).unwrap());
    let headers = crate::command_ingress::test_support::headers(Some(crate::command_ingress::INGRESS_ARGS_JSON_V1));
    let refused = command_ingress::admit_body_within(&registry, &body, &headers, "list_saved_package_entries", ArgsDecoding::Moved, ENTRY_COMMAND_MAX_ARGS_BYTES);
    assert!(refused.is_err());
}

/// R5: a list while its entry is written sees it once, held or stored, never
/// neither.
#[test]
fn a_list_while_its_entry_is_written_sees_it_once() {
    let (_dir, path, package) = unsaved("r5-between");
    let session = Arc::new(SessionHeldEntries::default());
    let (id, holder) = held(&keep_or_hold(&path, &session, None, &package));
    write_project(&path, PROJECT, &invented_model());
    {
        let (path, session, holder) = (path.clone(), Arc::clone(&session), holder.clone());
        hooks::arm_between_list_reads(move || {
            all_kept(&keep_held(Ok(path), &session, PROJECT, &holder));
        });
    }
    let list = listed(&path, &session, PROJECT, Some(&holder));
    assert_eq!(list.entries.iter().map(|entry| (entry.entry_id.clone(), entry.kept)).collect::<Vec<_>>(), [(id, "kept_with_project")]);
}

/// R5, as a stress run across threads: lists while a write runs never omit
/// the entry.
#[test]
fn lists_during_a_concurrent_write_never_omit_the_entry() {
    let dir = scratch("r5-stress");
    let path = store_in(&dir);
    drop(crate::open_project_store(&path).unwrap());
    let location = dir.join("p.opsproj");
    std::fs::write(&location, b"p").unwrap();
    let session = Arc::new(SessionHeldEntries::default());
    let mut omitted = 0usize;
    for round in 0..200 {
        let project = format!("project:invented-race-{round}");
        let (id, holder) = held(&keep_or_hold(&path, &session, None, &published(&project, &location, b"p", true, 1)));
        write_project(&path, &project, &invented_model());
        let writer = {
            let (path, session, project, holder) = (path.clone(), Arc::clone(&session), project.clone(), holder.clone());
            std::thread::spawn(move || keep_held(Ok(path), &session, &project, &holder))
        };
        loop {
            let entries = listed(&path, &session, &project, Some(&holder)).entries;
            match entries.iter().filter(|entry| entry.entry_id == id).count() {
                0 => omitted += 1,
                1 if entries.iter().any(|entry| entry.entry_id == id && entry.kept == "kept_with_project") => break,
                1 => {}
                _ => panic!("listed twice: {entries:?}"),
            }
            if writer.is_finished() {
                break;
            }
        }
        all_kept(&writer.join().unwrap());
    }
    assert_eq!(omitted, 0, "a list omitted the entry {omitted} times");
}

/// R6: two held entries and a write failure whose error names the first's
/// location: no public reason holds any part of either path.
#[test]
fn a_shared_write_failure_withholds_every_location() {
    let (dir, path, first) = unsaved("r6");
    let second_location = dir.join("second.opsproj");
    std::fs::write(&second_location, b"second").unwrap();
    let second = published(PROJECT, &second_location, b"second", true, 2);
    let session = SessionHeldEntries::default();
    let (_, holder) = held(&keep_or_hold(&path, &session, None, &first));
    held(&keep_or_hold(&path, &session, Some(&holder), &second));
    write_project(&path, PROJECT, &invented_model());
    fault::arm(Some(format!("disk I/O error near {}", first.location.display())));
    let write = keep_held(Ok(path.clone()), &session, PROJECT, &holder);
    fault::arm(None);
    let list = listed(&path, &session, PROJECT, Some(&holder));
    assert_eq!(list.entries.len(), 2);
    let parent = dir.to_string_lossy().into_owned();
    for text in [serde_json::to_string(&list).unwrap(), serde_json::to_string(&write).unwrap()] {
        assert!(!text.contains(&parent) && !text.contains(LOCATION_MARKER) && !text.contains("disk I/O"), "a path or an error's text leaked: {text}");
    }
}

/// R7: an entry is never kept with an empty time; it has its own code.
#[test]
fn an_entry_is_never_kept_with_an_empty_time() {
    let (dir, path) = store_with_project("r7-clock");
    let before = dump(&path);
    let mut package = published(PROJECT, &dir.join("p.opsproj"), b"p", true, 1);
    package.saved_at = String::new();
    for outcome in [keep_entry_in_store(&path, &package), keep_or_hold(&path, &SessionHeldEntries::default(), None, &package)] {
        assert_eq!((outcome.status.as_str(), outcome.code.as_str()), ("not_kept", ENTRY_NO_SAVE_TIME), "{outcome:?}");
    }
    assert_eq!(dump(&path), before);
}

/// R7: a store that cannot be found while writing held entries is recorded
/// as their reason; a list with the holder shows them when the store cannot
/// be opened, and says the store's failure by code.
#[test]
fn a_store_failure_while_writing_or_listing_held_entries_is_stated_by_code() {
    let (dir, path, package) = unsaved("r7-store");
    let session = SessionHeldEntries::default();
    let (id, holder) = held(&keep_or_hold(&path, &session, None, &package));
    let write = keep_held(Err(format!("no store near {}", dir.display())), &session, PROJECT, &holder);
    assert_eq!(write.entries.iter().map(|outcome| (outcome.entry_id.clone(), outcome.code.clone())).collect::<Vec<_>>(), [(id.clone(), ENTRY_STORE_FAILED.to_string())]);
    let broken = dir.join("not-a-store");
    std::fs::create_dir_all(&broken).unwrap();
    let list = list_entries(Ok(broken), &session, PROJECT, Some(&holder), &UnmeteredReadMemory).unwrap().into_unreserved().unwrap();
    assert_eq!(list.store_failure, Some(STORE_FAILED));
    assert_eq!(list.entries.iter().map(|entry| (entry.entry_id.clone(), entry.write_failure)).collect::<Vec<_>>(), [(id, Some(STORE_FAILED))]);
    withholds_location(&serde_json::to_string(&list).unwrap());
}

/// R7 (B's NB2): a save into a stored project whose earlier held entries
/// failed to write says so, not that it awaits the first save.
#[test]
fn a_save_into_a_stored_project_with_earlier_unwritten_entries_says_so() {
    let (dir, path, package) = unsaved("r7-nb2");
    let session = SessionHeldEntries::default();
    let (_, holder) = held(&keep_or_hold(&path, &session, None, &package));
    write_project(&path, PROJECT, &invented_model());
    fault::arm(Some("disk I/O error".to_string()));
    keep_held(Ok(path.clone()), &session, PROJECT, &holder);
    fault::arm(None);
    let outcome = keep_or_hold(&path, &session, Some(&holder), &published(PROJECT, &dir.join("next.opsproj"), b"next", true, 2));
    assert_eq!((outcome.status.as_str(), outcome.holder.as_deref()), ("held", Some(holder.as_str())));
    assert_eq!(outcome.message, HELD_WITH_EARLIER);
    assert!(!outcome.message.contains("first saved"), "{outcome:?}");
}

/// R7: a panic while the command writes held entries is a fixed reply.
#[test]
fn a_panic_while_writing_held_entries_is_a_fixed_reply() {
    let registry = crate::command_ingress::test_support::registry(crate::memory_pool::test_host::ScriptedHost::normal());
    let body = json!({ "projectId": PROJECT, "holder": "saved-package-holder:invented" });
    let (mut args, _lease) = crate::command_ingress::test_support::admit_json(&registry, "keep_held_saved_package_entries", ArgsDecoding::Moved, &body).unwrap();
    let outcome = keep_held_saved_package_entries_args(&mut args, || -> Result<PathBuf, String> { panic!("invented failure") }, &SessionHeldEntries::default(), &UnmeteredReadMemory);
    match outcome {
        Err(CommandError::Command(message)) => assert_eq!(message, KEEP_HELD_INCOMPLETE),
        other => panic!("{other:?}"),
    }
}

// --- Review round 2 ----------------------------------------------------------

/// The analysis-record checksum a package's result document carries: its
/// `run_hashes` entry that names the run as an `AnalysisRun`.
fn package_record_checksum(request: &serde_json::Value) -> String {
    request["result_documents"][0]["result_envelope"]["reproducibility"]["run_hashes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|hash| hash["payload_ref"]["ref_type"] == "AnalysisRun")
        .unwrap()["value"]
        .as_str()
        .unwrap()
        .to_string()
}

/// A run record's own checksum.
fn record_checksum(record: &serde_json::Value) -> String {
    record["analysis_run"]["hashes"].as_array().unwrap().iter().find(|hash| hash["payload_scope"] == "analysis_run_record").unwrap()["value"]
        .as_str()
        .unwrap()
        .to_string()
}

/// `request` as the desktop exports it from the run `record` records: every
/// occurrence of the shared builder's invented record checksum names
/// `record`'s instead, and the result document's derivative hash is
/// recomputed (and replaced wherever the request repeats it).
fn bound_request(request: &serde_json::Value, record: &serde_json::Value) -> serde_json::Value {
    use open_pipe_stress_result_export::derivative::{checksum, reference};
    let text = canonical(request).replace(&package_record_checksum(request), &record_checksum(record));
    let mut request: serde_json::Value = serde_json::from_str(&text).unwrap();
    let document = &mut request["result_documents"][0];
    let earlier = document["result_envelope"]["reproducibility"]["derivative_hash"]["value"].as_str().unwrap().to_string();
    let envelope_id = document["result_envelope"]["envelope_id"].as_str().unwrap().to_string();
    document["result_envelope"]["reproducibility"].as_object_mut().unwrap().remove("derivative_hash");
    let hash = checksum(document, "derivative_document_excludes_own_hash", reference("derivative_document", &envelope_id)).unwrap();
    let now = hash["value"].as_str().unwrap().to_string();
    document["result_envelope"]["reproducibility"]["derivative_hash"] = hash;
    serde_json::from_str(&canonical(&request).replace(&earlier, &now)).unwrap()
}

impl Route009 {
    /// The arguments of a save of the package exported from the run `record`
    /// records, with that record.
    fn bound_args(&self, record: &serde_json::Value) -> crate::report_package_save::ReportPackageSaveRequest {
        let request = bound_request(&self.request, record);
        let mut args = crate::report_package_save::test_support::save_request_with(&request, &self.model, &self.manifest);
        args.analysis_run_text = Some(canonical(record));
        args
    }
}

/// Round 2 (Fix 1): a run record that verifies against the package's source,
/// input manifest and producer but is not the package's own analysis record
/// (the record checksum its result document carries) keeps no entry; the
/// package is saved. Bound to it, the same record keeps its entry.
#[test]
fn a_run_record_that_is_not_the_packages_own_keeps_no_entry() {
    let _lock = crate::atomic_report_package_save::test_support::atomic_save_test_lock();
    let (dir, path, basis) = vector_project_store("r3-not-own");
    assert_ne!(package_record_checksum(&basis.request), record_checksum(&basis.run_record), "the builder's invented record is another record");
    let before = dump(&path);
    let receipt = save_009(admit_009(basis.args(Some(canonical(&basis.run_record)))).unwrap(), &dir, &path, &SessionHeldEntries::default());
    assert_eq!(receipt.outcome, "saved");
    let outcome = &receipt.saved_package_entry;
    assert_eq!((outcome.status.as_str(), outcome.code.as_str()), ("not_kept", ENTRY_RUN_RECORD_UNVERIFIED), "{outcome:?}");
    assert_eq!(dump(&path), before, "nothing was written");
    let receipt = save_009(admit_009(basis.bound_args(&basis.run_record)).unwrap(), &dir, &path, &SessionHeldEntries::default());
    assert_eq!(receipt.saved_package_entry.code, ENTRY_KEPT, "{:?}", receipt.saved_package_entry);
}

/// Round 2 (Fix 1): on route 009 the record's analysis status is the
/// builder's for its source (`HUMAN_REVIEW_REQUIRED`, the source's mechanics
/// status and one rule status, each once, sorted); an empty status, or one
/// that is not that, keeps no entry.
#[test]
fn a_route_009_record_whose_status_is_not_the_builders_keeps_no_entry() {
    let _lock = crate::atomic_report_package_save::test_support::atomic_save_test_lock();
    let (dir, path, basis) = vector_project_store("r3-status");
    let before = dump(&path);
    let statuses: Vec<String> = basis.run_record["analysis_run"]["analysis_status"].as_array().unwrap().iter().map(|status| status.as_str().unwrap().to_string()).collect();
    let rules = ["RULE_INPUTS_INCOMPLETE", "USER_RULE_CHECKED", "USER_RULE_FAILED"];
    let without_rule: Vec<String> = statuses.iter().filter(|status| !rules.contains(&status.as_str())).cloned().collect();
    let mut two_rules = statuses.clone();
    two_rules.push(rules.iter().find(|rule| !statuses.iter().any(|status| status == *rule)).unwrap().to_string());
    two_rules.sort();
    let other_mechanics: Vec<String> = {
        let mut other: Vec<String> = statuses
            .iter()
            .map(|status| match status.as_str() {
                "MECHANICS_SOLVED" => "MODEL_INCOMPLETE".to_string(),
                "MODEL_INCOMPLETE" => "MECHANICS_SOLVED".to_string(),
                _ => status.clone(),
            })
            .collect();
        other.sort();
        other
    };
    assert_ne!(other_mechanics, statuses);
    for (label, status) in [("empty", json!([])), ("without a rule status", json!(without_rule)), ("with two rule statuses", json!(two_rules)), ("with another mechanics status", json!(other_mechanics))] {
        let record = rehashed(&basis.run_record, |run| run["analysis_status"] = status.clone());
        let receipt = save_009(admit_009(basis.bound_args(&record)).unwrap_or_else(|code| panic!("{label}: refused {code}")), &dir, &path, &SessionHeldEntries::default());
        assert_eq!(receipt.outcome, "saved", "{label}");
        assert_eq!(receipt.saved_package_entry.code, ENTRY_RUN_RECORD_UNVERIFIED, "{label}: {:?}", receipt.saved_package_entry);
    }
    assert_eq!(dump(&path), before, "nothing was written");
}

/// Round 2 (Fix 2): an entry held under a token stays held under it when
/// another document creates the project: the holder's next package save is
/// held with it, nothing is written into the store, and a caller without the
/// token sees neither. Only the page's call writes them.
#[test]
fn held_entries_stay_held_under_their_token_when_another_document_creates_the_project() {
    let (dir, path, package) = unsaved("r3-same-id-created");
    let session = SessionHeldEntries::default();
    let (first, token) = held(&keep_or_hold(&path, &session, None, &package));
    // Document B creates the project row.
    crate::create_project_in_store(path.clone(), current_model(), None, None, None, None, None, None, None, None).unwrap();
    // Document A exports a second package with its token.
    let second = keep_or_hold(&path, &session, Some(&token), &published(PROJECT, &dir.join("second.opsproj"), b"second", true, 2));
    assert_eq!((second.status.as_str(), second.holder.as_deref()), ("held", Some(token.as_str())), "{second:?}");
    assert!(entry_rows(&path).is_empty(), "nothing was written into the store");
    assert_eq!(ids(&listed(&path, &session, PROJECT, Some(&token)).entries), [first.clone(), second.entry_id.clone()]);
    assert!(listed(&path, &session, PROJECT, None).entries.is_empty(), "without the token a caller sees neither");
    // Document A's page, after its own first save, writes both, in order; its
    // token is then retired, so its later saves are kept directly.
    assert_eq!(all_kept(&keep_held(Ok(path.clone()), &session, PROJECT, &token)), [first.clone(), second.entry_id.clone()]);
    let third = kept(&keep_or_hold(&path, &session, Some(&token), &published(PROJECT, &dir.join("third.opsproj"), b"third", true, 3)));
    assert_eq!(ids(&stored(&path, PROJECT)), [first, second.entry_id, third]);
}

/// The keep command as the desktop runs it, after its ingress.
fn keep_command(
    args: &mut command_ingress::Args,
    path: &Path,
    session: &SessionHeldEntries,
    memory: &dyn ReadMemory,
) -> Result<tauri::ipc::Response, CommandError> {
    respond_reserved(keep_held_saved_package_entries_args(args, || Ok(path.to_path_buf()), session, memory))
}

/// Round 2 (Fix 3): keeping many held entries reserves its working space and
/// its reply before it writes: under a budget that cannot hold them it is
/// refused with nothing written and every entry still held under its holder;
/// a retry with enough writes them all, in order.
#[test]
fn keeping_many_held_entries_is_refused_before_it_writes_and_a_retry_writes_them_all() {
    use crate::command_ingress::test_support::{admit_json, registry};
    let (dir, path, first) = unsaved("r3-keep-budget");
    let session = SessionHeldEntries::default();
    let (one, holder) = held(&keep_or_hold(&path, &session, None, &first));
    let mut expected = vec![one];
    for n in 2..=200u8 {
        let package = published(PROJECT, &dir.join(format!("p{n}.opsproj")), b"p", true, n);
        expected.push(held(&keep_or_hold(&path, &session, Some(&holder), &package)).0);
    }
    write_project(&path, PROJECT, &invented_model());
    let registry = registry(crate::memory_pool::test_host::ScriptedHost::normal());
    let (pool_bytes, budget) = registry.pool().settings();
    let body = json!({ "projectId": PROJECT, "holder": holder });
    {
        let (mut args, lease) = admit_json(&registry, "keep_held_saved_package_entries", ArgsDecoding::Moved, &body).unwrap();
        registry.configure_pool(pool_bytes, lease.bytes() + 16 * 1024, false).unwrap();
        let memory = PoolReadMemory::new(&registry, &lease);
        match keep_command(&mut args, &path, &session, &memory) {
            Err(CommandError::Command(refusal)) => assert!(refusal.starts_with(stored_read::STORE_READ_MEMORY_UNAVAILABLE), "{refusal}"),
            Err(other) => panic!("refused otherwise: {other:?}"),
            Ok(_) => panic!("admitted within the tight budget"),
        }
    }
    assert!(entry_rows(&path).is_empty(), "nothing was written");
    let list = listed(&path, &session, PROJECT, Some(&holder));
    assert_eq!(ids(&list.entries), expected, "every entry is still held under its holder");
    assert!(list.entries.iter().all(|entry| entry.kept == "held_by_session"));
    registry.configure_pool(pool_bytes, budget, false).unwrap();
    let (mut args, lease) = admit_json(&registry, "keep_held_saved_package_entries", ArgsDecoding::Moved, &body).unwrap();
    let memory = PoolReadMemory::new(&registry, &lease);
    assert!(keep_command(&mut args, &path, &session, &memory).is_ok());
    assert_eq!(ids(&stored(&path, PROJECT)), expected, "a retry writes them all, in order");
}

/// A memory that refuses every reservation.
struct Refusing;

impl ReadMemory for Refusing {
    fn reserve(&self, what: &str, bytes: u64) -> Result<stored_read::ReadReservation, String> {
        Err(format!("{}: {what} needs {bytes} bytes (invented refusal)", stored_read::STORE_READ_MEMORY_UNAVAILABLE))
    }
}

/// Round 2 (Fix 3): a list whose held entries cannot be reserved is refused
/// before it copies them.
#[test]
fn a_list_is_refused_before_it_copies_the_held_entries() {
    let (dir, path, mut first) = unsaved("r3-list-budget");
    let large = "c".repeat(1 << 20);
    first.export_context = large.clone();
    let session = SessionHeldEntries::default();
    let (_, holder) = held(&keep_or_hold(&path, &session, None, &first));
    for n in 2..=16u8 {
        let mut package = published(PROJECT, &dir.join(format!("p{n}.opsproj")), b"p", true, n);
        package.export_context = large.clone();
        held(&keep_or_hold(&path, &session, Some(&holder), &package));
    }
    let (refused, measured) = crate::ingress::counting::measure(|| list_entries(Ok(path.clone()), &session, PROJECT, Some(&holder), &Refusing).map(drop));
    assert!(refused.unwrap_err().starts_with(stored_read::STORE_READ_MEMORY_UNAVAILABLE));
    assert!(measured.peak_requested < 1 << 20, "the refused list allocated {} bytes at its peak", measured.peak_requested);
}

/// Round 2 (Fix 4): through the command's raw ingress, the body's own caps
/// stand, and a route-009 record whose growth the budget cannot hold after
/// the body is dropped, its decoded share released: the save goes on, and
/// its entry is not checked.
#[test]
fn a_route_009_record_that_does_not_fit_after_raw_ingress_is_dropped_and_the_save_goes_on() {
    use crate::command_ingress::test_support::{headers, registry};
    let _lock = crate::atomic_report_package_save::test_support::atomic_save_test_lock();
    let basis = route_009();
    let args = basis.bound_args(&basis.run_record);
    let record_text = args.analysis_run_text.clone().unwrap();
    let mut raw = crate::report_package_save::test_support::args_body(&args);
    raw["analysisRunCanonicalText"] = json!(record_text);
    let body = tauri::ipc::InvokeBody::Raw(serde_json::to_vec(&raw).unwrap());
    let headers = headers(Some(crate::command_ingress::INGRESS_ARGS_JSON_V1));
    let registry = registry(crate::memory_pool::test_host::ScriptedHost::normal());
    let (pool_bytes, _) = registry.pool().settings();
    let body_lease = crate::decode_report_package_save(&registry, &body, &headers).unwrap().1.bytes();
    let growth = crate::report_package_save::ingress_lease_bytes(&args).unwrap();
    let share = crate::ingress::allocation_envelope(record_text.len() as u64).unwrap();
    assert!(growth > share, "the save's growth is above the record's share");
    // The body's own cap: a budget below its lease refuses it.
    registry.configure_pool(pool_bytes, body_lease - 1, false).unwrap();
    assert!(matches!(crate::admit_report_package_save(&registry, &body, &headers), Err(CommandError::Ingress(_))));
    // A budget that holds the body and the save's growth only once the
    // record's decoded share is released.
    let tight = body_lease + growth - share / 2;
    registry.configure_pool(pool_bytes, tight, false).unwrap();
    let (request, mut lease) = crate::admit_report_package_save(&registry, &body, &headers).unwrap_or_else(|error| panic!("the save was refused for its optional record: {error:?}"));
    assert!(request.analysis_run_text.is_none(), "the record was dropped");
    assert!(lease.bytes() <= tight);
    let admitted = {
        let mut session = crate::report_package_save::UserReleaseSession { registry: &registry, lease: &mut lease };
        crate::report_package_save::admit_report_package_session(request, &software(), &mut no_store, &no_custody, Some(&mut session))
            .unwrap_or_else(|refusal| panic!("admission refused: {refusal:?}"))
    };
    assert_eq!(admitted.entry_basis, EntryBasis::NotChecked);
}

/// Round 2 (Fix 5): a failed v15 step says so by a fixed code and sentence,
/// never SQLite's own text.
#[test]
fn a_failed_v15_step_is_stated_by_a_fixed_code_and_sentence() {
    let dir = scratch("r3-migration-text");
    let path = dir.join("read-only.sqlite3");
    Connection::open(&path).unwrap().execute_batch("CREATE TABLE marker(x);").unwrap();
    let read_only = Connection::open_with_flags(&path, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();
    let error = migrate_v15_saved_package_entries(&read_only).unwrap_err();
    assert_eq!(error, "SAVED-PACKAGE-ENTRIES-MIGRATION-FAILED: the saved-package entries table could not be created.");
    assert_eq!(error, MIGRATION_FAILED);
}

/// Round 2 (Fix 6): the table has no rowid, so an `INSERT OR REPLACE` that
/// names an entry's rowid cannot overwrite it.
#[test]
fn a_replace_naming_an_entrys_rowid_cannot_overwrite_it() {
    let (dir, path) = store_with_project("r3-rowid");
    kept(&keep_or_hold(&path, &SessionHeldEntries::default(), None, &published(PROJECT, &dir.join("p.opsproj"), b"p", true, 1)));
    let before = stored(&path, PROJECT);
    let connection = Connection::open(&path).unwrap();
    let replaced = connection.execute(
        "INSERT OR REPLACE INTO saved_package_entries (rowid, project_id, entry_id, sequence, package_identity_sha256, container_sha256,
           container_byte_count, file_name, location, saved_at, export_context, durability_confirmed,
           model_hash, received_result_sha256, run_id, run_record_sha256)
         SELECT rowid, project_id, 'saved-package:another', sequence + 1, package_identity_sha256, 'replaced', container_byte_count, file_name,
           location, saved_at, export_context, durability_confirmed, model_hash, received_result_sha256, run_id, run_record_sha256
         FROM saved_package_entries",
        [],
    );
    drop(connection);
    assert!(replaced.is_err(), "a replace by rowid was admitted");
    assert_eq!(stored(&path, PROJECT), before, "the entry is as it was");
}

/// Round 2 (advisory): a list and a state check reserve SQLite's working set
/// for the entries' keys, measured over every project's, before they search
/// the table: a store holding an oversized entry id of another project is
/// priced before SQLite can load it.
#[test]
fn reads_of_the_entries_reserve_the_working_set_of_their_largest_key() {
    let (dir, path) = store_with_project("r3-keys");
    let location = dir.join("p.opsproj");
    std::fs::write(&location, b"p").unwrap();
    let id = kept(&keep_or_hold(&path, &SessionHeldEntries::default(), None, &published(PROJECT, &location, b"p", true, 1)));
    // Another project's row with a large entry id, written past the store's own paths.
    let large = "e".repeat(1 << 20);
    let other = json!({ "schema_version": open_pipe_stress_product_physics::MODEL_SCHEMA_VERSION, "project": { "id": "project:other", "name": "Other" } });
    write_project(&path, "project:other", &other);
    let connection = Connection::open(&path).unwrap();
    connection
        .execute(
            "INSERT INTO saved_package_entries SELECT 'project:other', ?1, 1, package_identity_sha256, container_sha256, container_byte_count, file_name,
               location, saved_at, export_context, durability_confirmed, model_hash, received_result_sha256, run_id, run_record_sha256
             FROM saved_package_entries WHERE project_id = ?2",
            rusqlite::params![large, PROJECT],
        )
        .unwrap();
    drop(connection);
    let session = SessionHeldEntries::default();
    let key = ("project:other".len() + large.len()) as u64 * stored_read::SQLITE_WORKING_SET_FACTOR;
    for (label, read) in [
        ("list", Box::new(|memory: &Recording| list_entries(Ok(path.clone()), &session, PROJECT, None, memory).map(drop)) as Box<dyn Fn(&Recording) -> Result<(), String>>),
        ("check", Box::new(|memory: &Recording| check_entry(Ok(path.clone()), &session, PROJECT, None, &id, memory).map(drop))),
    ] {
        let memory = Recording::default();
        read(&memory).unwrap();
        let reserved = memory.0.borrow();
        assert!(reserved.iter().any(|(what, bytes)| what.contains("saved-package entries (their largest keyed row") && *bytes == key), "{label}: {reserved:?}");
    }
}

/// A memory that admits every reservation and records it.
#[derive(Default)]
struct Recording(RefCell<Vec<(String, u64)>>);

impl ReadMemory for Recording {
    fn reserve(&self, what: &str, bytes: u64) -> Result<stored_read::ReadReservation, String> {
        self.0.borrow_mut().push((what.to_string(), bytes));
        UnmeteredReadMemory.reserve(what, bytes)
    }
}

// --- Review round 3 ------------------------------------------------------------

/// Round 3 (Finding 1): a package save of stored project B that presents a
/// live token minted for project A is not kept: nothing is written into B,
/// and A's held entries are as they were.
#[test]
fn a_save_presenting_another_projects_live_token_is_not_kept_and_writes_nothing() {
    let (dir, path, package) = unsaved("r4-other-project-token");
    let session = SessionHeldEntries::default();
    let (first, token) = held(&keep_or_hold(&path, &session, None, &package));
    let other = json!({ "schema_version": open_pipe_stress_product_physics::MODEL_SCHEMA_VERSION, "project": { "id": "project:b", "name": "B" } });
    write_project(&path, "project:b", &other);
    let before = listed(&path, &session, PROJECT, Some(&token)).entries;
    let outcome = keep_or_hold(&path, &session, Some(&token), &published("project:b", &dir.join("b.opsproj"), b"b", true, 2));
    assert_eq!((outcome.status.as_str(), outcome.code.as_str(), outcome.holder.as_deref()), ("not_kept", ENTRY_NOT_KEPT, None), "{outcome:?}");
    assert!(stored(&path, "project:b").is_empty(), "nothing was written into B");
    assert!(entry_rows(&path).is_empty(), "nothing was written at all");
    let after = listed(&path, &session, PROJECT, Some(&token)).entries;
    assert_eq!(after, before, "A's held entries are as they were");
    assert_eq!(ids(&after), [first]);
}

/// Round 3 (Finding 1): a package save presenting this project's live token
/// is held under it even when the store cannot be found.
#[test]
fn a_save_presenting_a_live_token_is_held_when_the_store_cannot_be_found() {
    let (dir, path, package) = unsaved("r4-token-no-store");
    let session = SessionHeldEntries::default();
    let (first, token) = held(&keep_or_hold(&path, &session, None, &package));
    let outcome = keep_or_hold_in(Err("no store".to_string()), &session, Some(&token), &published(PROJECT, &dir.join("second.opsproj"), b"second", true, 2));
    assert_eq!((outcome.status.as_str(), outcome.holder.as_deref()), ("held", Some(token.as_str())), "{outcome:?}");
    assert_eq!(ids(&listed(&path, &session, PROJECT, Some(&token)).entries), [first, outcome.entry_id.clone()]);
}

/// Round 3 (Finding 2): a keep refused for its memory marks the holder's
/// entries, keeps them held and the token live; the next save presenting the
/// token says the earlier entries are not yet written; a retry with enough
/// memory writes them all and retires the token.
#[test]
fn a_keep_refused_for_memory_marks_the_entries_and_a_retry_writes_them_all() {
    let (dir, path, package) = unsaved("r4-keep-refused");
    let session = SessionHeldEntries::default();
    let (first, token) = held(&keep_or_hold(&path, &session, None, &package));
    write_project(&path, PROJECT, &invented_model());
    let refused = keep_held_entries(Ok(path.clone()), &session, PROJECT, &token, &Refusing).map(drop).unwrap_err();
    assert!(refused.starts_with(stored_read::STORE_READ_MEMORY_UNAVAILABLE), "{refused}");
    assert!(entry_rows(&path).is_empty(), "nothing was written");
    let list = listed(&path, &session, PROJECT, Some(&token));
    assert_eq!(ids(&list.entries), [first.clone()]);
    assert!(list.entries.iter().all(|entry| entry.kept == "held_by_session" && entry.write_failure.is_some()), "{:?}", list.entries);
    withholds_location(&serde_json::to_string(&list).unwrap());
    let second = keep_or_hold(&path, &session, Some(&token), &published(PROJECT, &dir.join("second.opsproj"), b"second", true, 2));
    assert_eq!((second.status.as_str(), second.holder.as_deref(), second.message.as_str()), ("held", Some(token.as_str()), HELD_WITH_EARLIER));
    assert_eq!(all_kept(&keep_held(Ok(path.clone()), &session, PROJECT, &token)), [first.clone(), second.entry_id.clone()]);
    let third = kept(&keep_or_hold(&path, &session, Some(&token), &published(PROJECT, &dir.join("third.opsproj"), b"third", true, 3)));
    assert_eq!(ids(&stored(&path, PROJECT)), [first, second.entry_id, third], "the token was retired once they were written");
}
