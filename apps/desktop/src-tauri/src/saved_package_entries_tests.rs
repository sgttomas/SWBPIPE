//! Tests of the project's saved-package entries (DEL-08-01 REQ-023; DEL-02-05
//! REQ-039, AC-017, VER-013). Every value is invented. The route-010 save's
//! entry, with its row inspected and the store compared before and after
//! (DEL-08-01 VER-010), is in `user_release_save_tests.rs`.

use super::*;
use crate::report_package_save::test_support::{desktop_request, model as package_model, no_store, sandbox, save_request, software};
use crate::report_package_save::{admit_report_package_in, keep_no_entry, no_custody, save_admitted_report_package_keeping, AdmittedReportPackage};
use rusqlite::types::Value as SqlValue;
use serde_json::json;
use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;

/// A fault injected into [`keep_entry_in_store`] after the insert and before
/// the commit, carrying the location so that its withholding is shown.
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

/// Every table's rows, in rowid order, as SQLite holds them.
pub(crate) fn dump(path: &Path) -> BTreeMap<String, Vec<Vec<SqlValue>>> {
    let connection = Connection::open(path).unwrap();
    let mut names = connection.prepare("SELECT name FROM sqlite_master WHERE type = 'table' AND name NOT LIKE 'sqlite_%' ORDER BY name").unwrap();
    let names: Vec<String> = names.query_map([], |row| row.get(0)).unwrap().collect::<Result<_, _>>().unwrap();
    names
        .into_iter()
        .map(|name| {
            let mut statement = connection.prepare(&format!("SELECT * FROM \"{name}\" ORDER BY rowid")).unwrap();
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
    let listed = list_entries_in_store(&path, PROJECT).unwrap();
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
    assert_eq!(list_entries_in_store(&path, PROJECT).unwrap().entries.len(), 2);
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
    remove_entry_in_store(&path, project, &ids[0]).unwrap();
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

    let removal = remove_entry_in_store(&path, PROJECT, &first).unwrap();
    assert!(removal.removed);
    withholds_location(&serde_json::to_string(&removal).unwrap());
    assert_eq!(std::fs::read(&location).unwrap(), b"package bytes", "the file is untouched");
    assert_eq!(list_entries_in_store(&path, PROJECT).unwrap().entries.len(), 1);
    let unknown = remove_entry_in_store(&path, PROJECT, &first).unwrap_err();
    assert!(unknown.starts_with(ENTRY_UNKNOWN), "{unknown}");

    // Removing the project removes its entries (the foreign key cascades).
    let (connection, _) = crate::open_project_store(&path).unwrap();
    connection.execute("DELETE FROM local_projects WHERE project_id = ?1", params![PROJECT]).unwrap();
    drop(connection);
    assert!(list_entries_in_store(&path, PROJECT).unwrap().entries.is_empty());
    assert_eq!(list_entries_in_store(&path, "project:other").unwrap().entries[0].entry_id, other, "another project's entries stay");
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
    assert!(outcome.message.contains("invented fault") && outcome.message.contains(LOCATION_WITHHELD), "{}", outcome.message);
    withholds_location(&outcome.message);
    assert_eq!(dump(&path), before, "the insert was rolled back");
    assert_eq!(std::fs::read(&location).unwrap(), b"p");
}

#[test]
fn a_store_that_cannot_be_found_keeps_no_entry() {
    let dir = scratch("no-store");
    let location = dir.join("p.opsproj");
    let outcome = keep_entry(|| Err(format!("no store near {}", location.display())), &published(PROJECT, &location, b"p", true, 1));
    assert_eq!(outcome.code, ENTRY_NOT_KEPT);
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
        let found = check_entry_in_store(&path, PROJECT, &id).unwrap();
        assert_eq!(dump(&path), before, "{label}: the check writes nothing");
        withholds_location(&serde_json::to_string(&found).unwrap());
        (found.state, found.reason)
    };
    assert_eq!(state("untouched"), ("present_identical", String::new()));

    // Replaced by different bytes, and by a different valid package.
    std::fs::write(&location, b"other bytes").unwrap();
    assert_eq!(state("replaced").0, "changed");
    let request = desktop_request(&package_model());
    let other = admit_report_package_in(save_request(&request, &package_model()), &software(), &mut no_store, &no_custody).unwrap();
    std::fs::write(&location, &other.package.container_bytes).unwrap();
    let (found, reason) = state("another package");
    assert_eq!(found, "changed");
    assert!(reason.contains("SHA-256"), "{reason}");
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
            assert!(reason.contains("permission"), "{reason}");
        }
        std::fs::set_permissions(&location, std::fs::Permissions::from_mode(0o600)).unwrap();
        assert_eq!(state("restored").0, "present_identical");
    }

    let unknown = check_entry_in_store(&path, PROJECT, "saved-package:unknown").unwrap_err();
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
    let found = check_entry_in_store(&copy, PROJECT, &id).unwrap();
    assert_eq!(found.state, "not_found");
    assert_eq!(list_entries_in_store(&copy, PROJECT).unwrap(), list_entries_in_store(&path, PROJECT).unwrap());
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
    let list = list_entries_in_store(&path, PROJECT).unwrap();
    let list_json = serde_json::to_string(&list).unwrap();
    withholds_location(&list_json);
    assert!(!list_json.contains("location"), "{list_json}");
    withholds_location(&serde_json::to_string(&check_entry_in_store(&path, PROJECT, &id).unwrap()).unwrap());
    std::fs::remove_file(&location).unwrap();
    withholds_location(&serde_json::to_string(&check_entry_in_store(&path, PROJECT, &id).unwrap()).unwrap());
    withholds_location(&serde_json::to_string(&remove_entry_in_store(&path, PROJECT, &id).unwrap()).unwrap());
    withholds_location(&check_entry_in_store(&path, PROJECT, &id).unwrap_err());
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
/// store at `path`.
fn save_009(admitted: AdmittedReportPackage, dir: &Path, path: &Path) -> crate::report_package_save::ReportPackageSaveReceipt {
    let mut keep = |published: &PublishedPackage| keep_entry_in_store(path, published);
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
    let admitted = admit_009(basis.args(Some(canonical(&basis.run_record)))).unwrap();
    let receipt = save_009(admitted, &dir, &path);
    assert_eq!(receipt.outcome, "saved");
    assert_eq!((receipt.saved_package_entry.status.as_str(), receipt.saved_package_entry.code.as_str()), ("kept", ENTRY_KEPT), "{:?}", receipt.saved_package_entry);
    let entry = &list_entries_in_store(&path, basis.model["project"]["id"].as_str().unwrap()).unwrap().entries[0];
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
    let receipt = save_009(admit_009(basis.args(None)).unwrap(), &dir, &path);
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
        let receipt = save_009(admit_009(basis.args(Some(text))).unwrap_or_else(|code| panic!("{label}: refused {code}")), &dir, &path);
        assert_eq!(receipt.outcome, "saved", "{label}");
        let outcome = &receipt.saved_package_entry;
        assert_eq!((outcome.status.as_str(), outcome.code.as_str()), ("not_kept", ENTRY_RUN_RECORD_UNVERIFIED), "{label}: {outcome:?}");
        assert!(outcome.message.contains("did not verify"), "{label}: {}", outcome.message);
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

fn canonical(value: &serde_json::Value) -> String {
    open_pipe_stress_operation_applier::canonical_json(value)
}
