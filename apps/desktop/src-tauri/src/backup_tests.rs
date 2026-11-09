//! Tests of `backup.rs` (DEL-02-05 Design `autosave-and-backup.md` §4, §5,
//! §8 slice 3). Every test uses invented documents in its own temporary
//! directories: the app data directory (store, settings, locks) and a
//! separate backup folder. Older backups are planted under names built from
//! the Design's rule (§4 "Name"), not by the writer under test.

use super::*;
use crate::atomic_report_package_save::test_support::atomic_save_test_lock;
use crate::autosave_commands::{settings_text, SETTINGS_FILE};
use crate::command_ingress::test_support::{admit_json, headers, registry};
use crate::command_ingress::INGRESS_ARGS_JSON_V1;
use crate::memory_pool::test_host::ScriptedHost;
use crate::model_document_save::PROTECTED_CONTENT_BLOCKED;
use crate::stored_read::{ReadReservation, UnmeteredReadMemory};
use open_pipe_stress_operation_applier::canonical_json;
use serde_json::json;
use std::cell::Cell;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::sync::{Mutex, MutexGuard, PoisonError};

const DEMO: &str = include_str!("../../../../fixtures/product_preview/invented_demo_model.json");
const PROJECT: &str = "project:invented-demo-loop-01";
const STEM: &str = "invented-demo-loop-01";
/// 2026-09-21T13:46:40Z, an invented save time.
const T0: u64 = 1_790_000_800;

/// Tests that reach the folder's real sync hold this, so a parallel test's
/// sync worker cannot make theirs uncertain.
static REAL_SYNC: Mutex<()> = Mutex::new(());

fn real_sync_lock() -> MutexGuard<'static, ()> {
    REAL_SYNC.lock().unwrap_or_else(PoisonError::into_inner)
}

fn demo() -> Value {
    serde_json::from_str(DEMO).unwrap()
}

/// The demo with another project name: another model of the same project.
fn variant(index: usize) -> Value {
    let mut model = demo();
    model["project"]["name"] = json!(format!("Invented Utility Loop Demo, variant {index}"));
    model
}

fn hex(text: &str) -> String {
    sha256_hex(text)
}

/// A temporary app data directory and a separate backup folder.
struct Sandbox {
    root: PathBuf,
}

impl Sandbox {
    fn new(label: &str) -> Self {
        let root = std::env::temp_dir().join(format!(
            "swbpipe-backup-{label}-{}-{}",
            std::process::id(),
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
        ));
        fs::create_dir_all(root.join("data")).unwrap();
        fs::create_dir_all(root.join("folder")).unwrap();
        Self { root }
    }

    fn data(&self) -> PathBuf {
        self.root.join("data")
    }

    fn folder(&self) -> PathBuf {
        self.root.join("folder")
    }

    fn store(&self) -> PathBuf {
        self.data().join(crate::PROJECT_STORE_FILE)
    }

    /// The folder's entries, sorted.
    fn entries(&self) -> Vec<String> {
        let mut names: Vec<String> = fs::read_dir(self.folder()).unwrap().map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned()).collect();
        names.sort();
        names
    }

    /// Backups on: the folder recorded with the explicit intent.
    fn enable(&self, keep: Option<u32>) {
        let record = SettingsRecord { autosave_minutes: 5, backup_folder: Some(self.folder().to_str().unwrap().to_string()), backup_keep: keep, backup_local_private_intent: true };
        fs::write(self.data().join(SETTINGS_FILE), settings_text(&record)).unwrap();
    }

    /// The project saved by the store's own create path, with its computed model hash.
    fn save(&self, model: &Value) -> String {
        let model_hash = crate::computed_model_hash(model, PROJECT);
        crate::create_project_in_store(self.store(), model.clone(), None, None, None, None, None, Some(model_hash.clone()), None, None).unwrap();
        model_hash["value"].as_str().unwrap().to_string()
    }

    /// A project row written with raw SQL: `model_json` and `model_hash_json` as given.
    fn save_raw(&self, model_json: &str, model_hash_json: &str) {
        let (connection, _) = crate::open_project_store(&self.store()).unwrap();
        connection
            .execute(
                "INSERT OR REPLACE INTO local_projects(project_id, project_name, model_json, model_hash_json, created_at_unix, updated_at_unix) VALUES (?1, 'Invented', ?2, ?3, 1, 1)",
                params![PROJECT, model_json, model_hash_json],
            )
            .unwrap();
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        // Only this test's own temporary directory; a read-only folder is made writable first.
        if self.root.starts_with(std::env::temp_dir()) {
            let _ = fs::set_permissions(self.folder(), fs::Permissions::from_mode(0o700));
            let _ = fs::remove_dir_all(&self.root);
        }
    }
}

fn request(model_hash: &str) -> WriteRequest {
    WriteRequest { project_id: PROJECT.to_string(), expected_model_hash: model_hash.to_string() }
}

fn hash_claim(model_hash: &str) -> String {
    json!({"algorithm": "sha256", "canonicalization": "rfc8785_jcs", "payload_scope": "model_payload", "payload_ref": PROJECT, "value": model_hash, "hash_status": "computed_local_preview"}).to_string()
}

/// The Design's name for a backup of `model` written at `seconds` (§4).
fn design_name(model: &Value, seconds: u64) -> String {
    let text = canonical_json(model);
    format!("{STEM}.{}.{}.{}.swbpipe-backup.json", &hex(PROJECT)[..8], utc_stamp(seconds).unwrap(), &hex(&text)[..12])
}

/// Plants a verified backup of `model` written at `seconds`; returns its name.
fn plant(sandbox: &Sandbox, model: &Value, seconds: u64) -> String {
    let name = design_name(model, seconds);
    fs::write(sandbox.folder().join(&name), canonical_json(model)).unwrap();
    name
}

/// The project's model as `write_backup_in_folder` takes it (no store).
fn stored(model: &Value) -> StoredBackup {
    let text = canonical_json(model);
    StoredBackup { model_hex: hex(&text), canonical: Metered::new(text), project_id: PROJECT.to_string(), stem: STEM.to_string(), warnings: Vec::new() }
}

fn codes(outcome: &BackupWriteOutcome) -> Vec<String> {
    match outcome {
        BackupWriteOutcome::Refused { diagnostics } => diagnostics.iter().map(|item| item.code.clone()).collect(),
        BackupWriteOutcome::Written { warnings, .. } => warnings.iter().map(|item| item.code.clone()).collect(),
        _ => Vec::new(),
    }
}

/// The system's operations, with a sync that reports durable without the
/// process-wide sync worker (folder-level tests run in parallel).
struct Durable;

impl BackupIo for Durable {
    fn sync_directory(&mut self, _dir: &BackupDir) -> bool {
        true
    }
}

// ---------------------------------------------------------------------------
// Names.
// ---------------------------------------------------------------------------

#[test]
fn utc_stamps_are_the_civil_utc_time() {
    assert_eq!(utc_stamp(0).as_deref(), Some("19700101T000000Z"));
    assert_eq!(utc_stamp(951_782_400).as_deref(), Some("20000229T000000Z"), "a leap day");
    assert_eq!(utc_stamp(4_107_542_400).as_deref(), Some("21000301T000000Z"), "2100 is not a leap year");
    assert_eq!(utc_stamp(T0).as_deref(), Some("20260921T134640Z"));
    assert_eq!(utc_stamp(253_402_300_799).as_deref(), Some("99991231T235959Z"));
    assert_eq!(utc_stamp(253_402_300_800), None, "beyond year 9999");
}

#[test]
fn names_parse_from_the_right_and_nothing_else_parses() {
    let name = BackupName { stem: "a.b-c_1".to_string(), slot: "0123abcd".to_string(), stamp: "20260921T134640Z".to_string(), hash: "0123456789ab".to_string() };
    assert_eq!(name.file_name(), "a.b-c_1.0123abcd.20260921T134640Z.0123456789ab.swbpipe-backup.json");
    assert_eq!(BackupName::parse(&name.file_name()), Some(name));
    for other in [
        ".swbpipe-backup-0123456789abcdef0123456789abcdef.tmp",
        "x.0123abcd.20260921T134640Z.0123456789ab.json",
        "x.0123ABCD.20260921T134640Z.0123456789ab.swbpipe-backup.json",
        "x.0123abcd.20260921T134640.0123456789ab.swbpipe-backup.json",
        "x.0123abcd.20260921T134640Z.0123456789a.swbpipe-backup.json",
        ".0123abcd.20260921T134640Z.0123456789ab.swbpipe-backup.json",
        "x/y.0123abcd.20260921T134640Z.0123456789ab.swbpipe-backup.json",
        "x y.0123abcd.20260921T134640Z.0123456789ab.swbpipe-backup.json",
        "0123abcd.20260921T134640Z.0123456789ab.swbpipe-backup.json",
    ] {
        assert_eq!(BackupName::parse(other), None, "{other}");
    }
    assert_eq!(backup_slot(PROJECT), hex(PROJECT)[..8]);
    assert_eq!(backup_stem(&demo()), STEM);
}

// ---------------------------------------------------------------------------
// The write, through the store and the settings (`write_at`).
// ---------------------------------------------------------------------------

#[test]
fn a_backup_is_the_stored_model_s_canonical_text_under_its_design_name() {
    let _sync = real_sync_lock();
    let sandbox = Sandbox::new("canonical");
    let model = demo();
    let text = canonical_json(&model);
    let model_hash = format!("sha256:{}", hex(&text));
    // The stored text is compact JSON, not canonical: spaces and another member order.
    let pretty = serde_json::to_string_pretty(&model).unwrap();
    assert_ne!(pretty, text);
    sandbox.save_raw(&pretty, &hash_claim(&model_hash));
    sandbox.enable(None);
    let store_before = fs::read(sandbox.store()).unwrap();
    let outcome = write_at(Ok(sandbox.data()), &request(&model_hash), Some(T0), &UnmeteredReadMemory);
    let BackupWriteOutcome::Written { file_name, model_hash: written_hash, durability, warnings } = &outcome else { panic!("{outcome:?}") };
    assert_eq!(file_name, &design_name(&model, T0));
    assert_eq!((written_hash, durability.as_str()), (&model_hash, "durable"));
    assert!(warnings.is_empty(), "{warnings:?}");
    let path = sandbox.folder().join(file_name);
    assert_eq!(fs::read_to_string(&path).unwrap(), text, "the canonical bytes, not the stored text");
    assert_eq!(fs::metadata(&path).unwrap().permissions().mode() & 0o777, 0o600, "private data: owner only");
    assert_eq!(sandbox.entries(), vec![file_name.clone()], "no temporary file is left");
    assert_eq!(fs::read(sandbox.store()).unwrap(), store_before, "the store is read only");
    // The same stored model again: nothing is written.
    let again = write_at(Ok(sandbox.data()), &request(&model_hash), Some(T0 + 60), &UnmeteredReadMemory);
    assert_eq!(again, BackupWriteOutcome::Unchanged { file_name: file_name.clone(), model_hash: model_hash.clone() });
    assert_eq!(sandbox.entries(), vec![file_name.clone()]);
}

#[test]
fn a_backup_written_after_a_real_save_reads_back_through_the_open_path() {
    let _sync = real_sync_lock();
    let sandbox = Sandbox::new("saved");
    let model_hash = sandbox.save(&demo());
    sandbox.enable(Some(3));
    let BackupWriteOutcome::Written { file_name, .. } = write_at(Ok(sandbox.data()), &request(&model_hash), Some(T0), &UnmeteredReadMemory) else { panic!() };
    let bytes = fs::read(sandbox.folder().join(&file_name)).unwrap();
    let outcome = crate::model_document_file::check_model_document(file_name, bytes.len() as u64, serde_json::from_slice(&bytes).unwrap());
    assert!(matches!(outcome, crate::model_document_file::ModelDocumentOpenOutcome::Opened { .. }), "the backup opens with Open Model Document");
}

#[test]
fn backups_off_are_disabled_and_never_open_the_store_or_touch_a_folder() {
    let sandbox = Sandbox::new("off");
    let model_hash = format!("sha256:{}", "a".repeat(64));
    // No settings file; no store either: nothing is opened.
    assert_eq!(write_at(Ok(sandbox.data()), &request(&model_hash), Some(T0), &UnmeteredReadMemory), BackupWriteOutcome::Disabled);
    assert!(!sandbox.store().exists());
    // A folder in a file written before slice 3, without the recorded intent.
    let legacy = json!({"document_kind": "swbpipe.desktop.autosave_settings", "version": 1, "autosave_minutes": 5, "backup_folder": sandbox.folder().to_str().unwrap(), "backup_keep": 3});
    fs::write(sandbox.data().join(SETTINGS_FILE), legacy.to_string()).unwrap();
    let model_hash = sandbox.save(&demo());
    assert_eq!(write_at(Ok(sandbox.data()), &request(&model_hash), Some(T0), &UnmeteredReadMemory), BackupWriteOutcome::Disabled);
    assert!(sandbox.entries().is_empty(), "the legacy folder is never written to");
    assert_eq!(fs::read_to_string(sandbox.data().join(SETTINGS_FILE)).unwrap(), legacy.to_string(), "the settings are not rewritten");
}

#[test]
fn a_stale_or_inconsistent_store_refuses_and_writes_nothing() {
    let sandbox = Sandbox::new("stale");
    sandbox.enable(None);
    let model = demo();
    let text = canonical_json(&model);
    let model_hash = format!("sha256:{}", hex(&text));
    let other_hash = format!("sha256:{}", "b".repeat(64));
    let refused = |expected: &str| codes(&write_at(Ok(sandbox.data()), &request(expected), Some(T0), &UnmeteredReadMemory));
    assert_eq!(refused("sha256:ABC"), vec![HASH_INVALID]);
    // No store, then no row.
    assert_eq!(refused(&model_hash), vec![crate::stored_model_document_export::STORE_UNAVAILABLE]);
    assert!(!sandbox.store().exists(), "a read-only open creates no store");
    sandbox.save_raw("{}", "null");
    let (connection, _) = crate::open_project_store(&sandbox.store()).unwrap();
    connection.execute("DELETE FROM local_projects", []).unwrap();
    drop(connection);
    assert_eq!(refused(&model_hash), vec![PROJECT_NOT_FOUND]);
    // The store moved on: its row holds another model than the save asked for.
    sandbox.save_raw(&text, &hash_claim(&other_hash));
    assert_eq!(refused(&model_hash), vec![STALE]);
    // The row's claim is not the hash of its own model.
    assert_eq!(refused(&other_hash), vec![STORED_HASH_MISMATCH]);
    // No claim recorded.
    sandbox.save_raw(&text, "null");
    assert_eq!(refused(&model_hash), vec![STORED_HASH_MISSING]);
    // A member held twice: the parsed document would not be the stored text.
    let duplicated = text.replacen("{", "{\"schema_version\":\"0.9.0\",", 1);
    sandbox.save_raw(&duplicated, &hash_claim(&model_hash));
    assert_eq!(refused(&model_hash), vec![STORED_DUPLICATE_MEMBER]);
    assert!(sandbox.entries().is_empty(), "nothing was written for any refusal");
}

#[test]
fn the_open_checks_and_the_screening_refuse_before_any_write() {
    let sandbox = Sandbox::new("screened");
    sandbox.enable(None);
    let mut quarantined = demo();
    quarantined["materials"][0]["provenance_record"]["review_status"] = json!("quarantined");
    let model_hash = sandbox.save(&quarantined);
    let outcome = write_at(Ok(sandbox.data()), &request(&model_hash), Some(T0), &UnmeteredReadMemory);
    let BackupWriteOutcome::Refused { diagnostics } = &outcome else { panic!("{outcome:?}") };
    assert_eq!((diagnostics[0].code.as_str(), diagnostics[0].path.as_str()), (PROTECTED_CONTENT_BLOCKED, "materials[0].provenance_record"));
    // A document the open path refuses (its validator's blocking finding).
    let mut invalid = demo();
    invalid["lines"][0]["pipe_refs"] = json!(["pipe:P-absent"]);
    let text = canonical_json(&invalid);
    let model_hash = format!("sha256:{}", hex(&text));
    sandbox.save_raw(&text, &hash_claim(&model_hash));
    let outcome = write_at(Ok(sandbox.data()), &request(&model_hash), Some(T0), &UnmeteredReadMemory);
    assert!(codes(&outcome).contains(&"MODEL-LINE-PIPE-MISSING".to_string()), "{outcome:?}");
    assert!(sandbox.entries().is_empty());
}

#[test]
fn a_reservation_the_pool_refuses_writes_nothing() {
    struct RefusesCanonical;
    impl ReadMemory for RefusesCanonical {
        fn reserve(&self, what: &str, bytes: u64) -> Result<ReadReservation, String> {
            if what.contains("canonical text") {
                return Err(format!("{STORE_READ_MEMORY_UNAVAILABLE}: {what} refused (invented)"));
            }
            UnmeteredReadMemory.reserve(what, bytes)
        }
    }
    let sandbox = Sandbox::new("memory");
    sandbox.enable(None);
    let model_hash = sandbox.save(&demo());
    let outcome = write_at(Ok(sandbox.data()), &request(&model_hash), Some(T0), &RefusesCanonical);
    assert_eq!(codes(&outcome), vec![STORE_READ_MEMORY_UNAVAILABLE]);
    assert!(sandbox.entries().is_empty());
}

// ---------------------------------------------------------------------------
// The folder.
// ---------------------------------------------------------------------------

#[test]
fn a_missing_symlinked_or_read_only_folder_refuses() {
    let sandbox = Sandbox::new("folders");
    let document = stored(&demo());
    let missing = sandbox.root.join("missing");
    assert_eq!(codes(&write_backup_in_folder(&missing, &document, None, T0, &UnmeteredReadMemory, &mut Durable)), vec![FOLDER_UNAVAILABLE]);
    // A symlink to a real folder is refused, and that folder is untouched.
    let link = sandbox.root.join("link");
    std::os::unix::fs::symlink(sandbox.folder(), &link).unwrap();
    assert_eq!(codes(&write_backup_in_folder(&link, &document, None, T0, &UnmeteredReadMemory, &mut Durable)), vec![FOLDER_UNAVAILABLE]);
    assert!(sandbox.entries().is_empty());
    // A read-only folder: the temporary file cannot be created.
    fs::set_permissions(sandbox.folder(), fs::Permissions::from_mode(0o500)).unwrap();
    let outcome = write_backup_in_folder(&sandbox.folder(), &document, None, T0, &UnmeteredReadMemory, &mut Durable);
    fs::set_permissions(sandbox.folder(), fs::Permissions::from_mode(0o700)).unwrap();
    assert_eq!(codes(&outcome), vec![TEMP_CREATE_FAILED], "{outcome:?}");
    assert!(sandbox.entries().is_empty());
}

#[test]
fn a_full_disk_or_a_failed_publish_leaves_no_file_and_keeps_older_backups() {
    struct DiskFull;
    impl BackupIo for DiskFull {
        fn write_all(&mut self, _file: &mut File, _bytes: &[u8]) -> io::Result<()> {
            Err(io::Error::from_raw_os_error(libc::ENOSPC))
        }
    }
    struct PublishFails;
    impl BackupIo for PublishFails {
        fn publish(&mut self, _dir: &BackupDir, _temp: &str, _name: &str) -> io::Result<()> {
            Err(io::Error::from_raw_os_error(libc::EIO))
        }
    }
    let sandbox = Sandbox::new("injected");
    let older = plant(&sandbox, &variant(1), T0 - 3600);
    let before = sandbox.entries();
    let outcome = write_backup_in_folder(&sandbox.folder(), &stored(&demo()), Some(3), T0, &UnmeteredReadMemory, &mut DiskFull);
    let BackupWriteOutcome::Refused { diagnostics } = &outcome else { panic!("{outcome:?}") };
    assert_eq!(diagnostics[0].code, WRITE_FAILED);
    assert!(diagnostics[0].message.contains(&format!("os error {}", libc::ENOSPC)), "{}", diagnostics[0].message);
    assert_eq!(sandbox.entries(), before, "no temporary file, the older backup kept");
    assert_eq!(codes(&write_backup_in_folder(&sandbox.folder(), &stored(&demo()), Some(3), T0, &UnmeteredReadMemory, &mut PublishFails)), vec![PUBLISH_FAILED]);
    assert_eq!(sandbox.entries(), vec![older]);
}

#[test]
fn a_name_already_in_use_is_never_replaced() {
    /// Another writer creates the backup's name between the check and the publish.
    struct RaceAtPublish(PathBuf, Vec<u8>);
    impl BackupIo for RaceAtPublish {
        fn publish(&mut self, dir: &BackupDir, temp: &str, name: &str) -> io::Result<()> {
            fs::write(self.0.join(name), &self.1)?;
            dir.publish_new(temp, name)
        }
    }
    let sandbox = Sandbox::new("name-in-use");
    let model = demo();
    let name = design_name(&model, T0);
    // A foreign file already under the very name the backup would take.
    fs::write(sandbox.folder().join(&name), b"invented foreign bytes").unwrap();
    let outcome = write_backup_in_folder(&sandbox.folder(), &stored(&model), Some(3), T0, &UnmeteredReadMemory, &mut Durable);
    assert_eq!(codes(&outcome), vec![NAME_IN_USE], "{outcome:?}");
    assert_eq!(fs::read(sandbox.folder().join(&name)).unwrap(), b"invented foreign bytes", "never replaced");
    assert_eq!(sandbox.entries(), vec![name.clone()], "the temporary file was removed");
    // Foreign bytes created under the name during the write.
    fs::remove_file(sandbox.folder().join(&name)).unwrap();
    let mut race = RaceAtPublish(sandbox.folder(), b"invented racing bytes".to_vec());
    let outcome = write_backup_in_folder(&sandbox.folder(), &stored(&model), Some(3), T0, &UnmeteredReadMemory, &mut race);
    assert_eq!(codes(&outcome), vec![NAME_IN_USE], "{outcome:?}");
    assert_eq!(fs::read(sandbox.folder().join(&name)).unwrap(), b"invented racing bytes");
    assert_eq!(sandbox.entries(), vec![name.clone()]);
    // Exactly these bytes created under the name during the write: unchanged.
    fs::remove_file(sandbox.folder().join(&name)).unwrap();
    let mut race = RaceAtPublish(sandbox.folder(), canonical_json(&model).into_bytes());
    let outcome = write_backup_in_folder(&sandbox.folder(), &stored(&model), Some(3), T0, &UnmeteredReadMemory, &mut race);
    assert_eq!(outcome, BackupWriteOutcome::Unchanged { file_name: name.clone(), model_hash: format!("sha256:{}", hex(&canonical_json(&model))) });
    assert_eq!(sandbox.entries(), vec![name]);
}

#[test]
fn an_unchanged_check_needs_the_full_hash_not_the_name() {
    let sandbox = Sandbox::new("unchanged-name");
    let model = demo();
    // The newest name carries this model's hash prefix but other bytes.
    let forged = design_name(&model, T0 - 60);
    fs::write(sandbox.folder().join(&forged), b"invented bytes under a forged name").unwrap();
    let outcome = write_backup_in_folder(&sandbox.folder(), &stored(&model), None, T0, &UnmeteredReadMemory, &mut Durable);
    let BackupWriteOutcome::Written { file_name, .. } = &outcome else { panic!("{outcome:?}") };
    assert_eq!(file_name, &design_name(&model, T0));
    assert_eq!(fs::read(sandbox.folder().join(&forged)).unwrap(), b"invented bytes under a forged name");
}

#[test]
fn a_read_back_that_differs_removes_only_the_new_file_and_one_that_cannot_be_made_keeps_it() {
    /// Rewrites the published file in place (same inode) before the read-back.
    struct Corrupts(PathBuf);
    impl BackupIo for Corrupts {
        fn publish(&mut self, dir: &BackupDir, temp: &str, name: &str) -> io::Result<()> {
            dir.publish_new(temp, name)?;
            fs::OpenOptions::new().write(true).open(self.0.join(name))?.write_all(b"X")
        }
    }
    /// Replaces the published entry by another file before the read-back.
    struct Replaces(PathBuf);
    impl BackupIo for Replaces {
        fn publish(&mut self, dir: &BackupDir, temp: &str, name: &str) -> io::Result<()> {
            dir.publish_new(temp, name)?;
            let other = self.0.join("invented-other.bin");
            fs::write(&other, b"invented replacement")?;
            fs::rename(other, self.0.join(name))
        }
    }
    let sandbox = Sandbox::new("readback");
    let older = plant(&sandbox, &variant(1), T0 - 3600);
    let model = demo();
    let outcome = write_backup_in_folder(&sandbox.folder(), &stored(&model), Some(3), T0, &UnmeteredReadMemory, &mut Corrupts(sandbox.folder()));
    assert_eq!(codes(&outcome), vec![READBACK_MISMATCH], "{outcome:?}");
    assert_eq!(sandbox.entries(), vec![older.clone()], "only the new file was removed");
    let outcome = write_backup_in_folder(&sandbox.folder(), &stored(&model), Some(1), T0, &UnmeteredReadMemory, &mut Replaces(sandbox.folder()));
    assert_eq!(codes(&outcome), vec![READBACK_FAILED], "{outcome:?}");
    let name = design_name(&model, T0);
    assert_eq!(fs::read(sandbox.folder().join(&name)).unwrap(), b"invented replacement", "a file this backup did not write is never removed");
    assert!(sandbox.folder().join(&older).exists(), "no rotation after a read-back that could not be made");
}

// ---------------------------------------------------------------------------
// Rotation.
// ---------------------------------------------------------------------------

#[test]
fn rotation_with_all_deletes_nothing() {
    let sandbox = Sandbox::new("rotation-all");
    let planted: Vec<String> = (1..=6).map(|index| plant(&sandbox, &variant(index), T0 - 3600 * index as u64)).collect();
    let outcome = write_backup_in_folder(&sandbox.folder(), &stored(&demo()), None, T0, &UnmeteredReadMemory, &mut Durable);
    assert!(matches!(outcome, BackupWriteOutcome::Written { .. }), "{outcome:?}");
    for name in &planted {
        assert!(sandbox.folder().join(name).exists(), "{name}");
    }
    assert_eq!(sandbox.entries().len(), 7);
}

#[test]
fn rotation_keeps_the_newest_n_verified_backups_and_never_touches_another_file() {
    let sandbox = Sandbox::new("rotation-n");
    let folder = sandbox.folder();
    // Five older verified backups, newest first: hours 1 to 5 before T0.
    let older: Vec<String> = (1..=5).map(|index| plant(&sandbox, &variant(index), T0 - 3600 * index as u64)).collect();
    // Files that must survive, each named like this project's older backup:
    // bytes that do not hash with the name's prefix;
    let wrong_hash = design_name(&variant(6), T0 - 3600 * 10);
    fs::write(folder.join(&wrong_hash), b"invented bytes").unwrap();
    // another project's canonical model whose own hash the name carries;
    let mut other_project = variant(7);
    other_project["project"]["id"] = json!("project:invented-other");
    let other_text = canonical_json(&other_project);
    let other_name = format!("{STEM}.{}.{}.{}.swbpipe-backup.json", &hex(PROJECT)[..8], utc_stamp(T0 - 3600 * 11).unwrap(), &hex(&other_text)[..12]);
    fs::write(folder.join(&other_name), &other_text).unwrap();
    // a symlink to a verified backup outside the folder, and a folder;
    let outside = sandbox.root.join("outside.json");
    fs::write(&outside, canonical_json(&variant(8))).unwrap();
    let link = design_name(&variant(8), T0 - 3600 * 12);
    std::os::unix::fs::symlink(&outside, folder.join(&link)).unwrap();
    let directory = design_name(&variant(9), T0 - 3600 * 13);
    fs::create_dir(folder.join(&directory)).unwrap();
    // and unrelated files.
    fs::write(folder.join("notes.txt"), b"invented notes").unwrap();
    fs::write(folder.join(".swbpipe-backup-0123456789abcdef0123456789abcdef.tmp"), b"invented leftover").unwrap();
    let outcome = write_backup_in_folder(&folder, &stored(&demo()), Some(3), T0, &UnmeteredReadMemory, &mut Durable);
    let BackupWriteOutcome::Written { file_name, warnings, durability, .. } = &outcome else { panic!("{outcome:?}") };
    assert_eq!(durability, "durable");
    // Kept: the new backup and the two newest verified older ones.
    assert!(folder.join(file_name).exists());
    assert!(folder.join(&older[0]).exists() && folder.join(&older[1]).exists());
    for removed in &older[2..] {
        assert!(!folder.join(removed).exists(), "{removed}");
    }
    for kept in [&wrong_hash, &other_name, &directory, &"notes.txt".to_string(), &".swbpipe-backup-0123456789abcdef0123456789abcdef.tmp".to_string()] {
        assert!(fs::symlink_metadata(folder.join(kept)).is_ok(), "{kept}");
    }
    assert!(fs::symlink_metadata(folder.join(&link)).unwrap().file_type().is_symlink(), "the link is left in place");
    assert_eq!(fs::read(&outside).unwrap(), canonical_json(&variant(8)).into_bytes(), "its target is untouched");
    let unverified = warnings.iter().find(|item| item.code == ROTATION_UNVERIFIED).expect("the unverified files are counted");
    assert!(unverified.message.starts_with("2 files"), "{}", unverified.message);
}

#[test]
fn an_uncertain_folder_sync_defers_rotation() {
    struct NotDurable;
    impl BackupIo for NotDurable {
        fn sync_directory(&mut self, _dir: &BackupDir) -> bool {
            false
        }
    }
    let sandbox = Sandbox::new("rotation-deferred");
    let older: Vec<String> = (1..=4).map(|index| plant(&sandbox, &variant(index), T0 - 3600 * index as u64)).collect();
    let outcome = write_backup_in_folder(&sandbox.folder(), &stored(&demo()), Some(3), T0, &UnmeteredReadMemory, &mut NotDurable);
    let BackupWriteOutcome::Written { durability, .. } = &outcome else { panic!("{outcome:?}") };
    assert_eq!(durability, "saved_durability_uncertain");
    assert_eq!(codes(&outcome), vec![PRUNE_DEFERRED]);
    for name in &older {
        assert!(sandbox.folder().join(name).exists(), "{name}");
    }
}

#[test]
fn a_newer_backup_written_during_the_transaction_is_kept() {
    /// Another writer adds a newer backup after the enumeration.
    struct NewerArrives(PathBuf, String);
    impl BackupIo for NewerArrives {
        fn sync_directory(&mut self, _dir: &BackupDir) -> bool {
            let model = variant(42);
            self.1 = design_name(&model, T0 + 60);
            fs::write(self.0.join(&self.1), canonical_json(&model)).unwrap();
            true
        }
    }
    let sandbox = Sandbox::new("rotation-newer");
    let older = plant(&sandbox, &variant(1), T0 - 3600);
    let mut io = NewerArrives(sandbox.folder(), String::new());
    let outcome = write_backup_in_folder(&sandbox.folder(), &stored(&demo()), Some(1), T0, &UnmeteredReadMemory, &mut io);
    let BackupWriteOutcome::Written { file_name, .. } = &outcome else { panic!("{outcome:?}") };
    assert!(sandbox.folder().join(file_name).exists(), "the new backup is always kept");
    assert!(sandbox.folder().join(&io.1).exists(), "the concurrent newer backup is kept");
    assert!(!sandbox.folder().join(&older).exists(), "the older verified backup beyond N is removed");
}

#[test]
fn a_backup_swapped_after_its_verification_is_not_removed() {
    let sandbox = Sandbox::new("rotation-swap");
    let older = plant(&sandbox, &variant(1), T0 - 3600);
    let folder = sandbox.folder();
    let swapped = folder.join(&older);
    BEFORE_UNLINK.with(|hook| {
        *hook.borrow_mut() = Some(Box::new(move || {
            let replacement = swapped.with_file_name("invented-replacement.bin");
            fs::write(&replacement, b"invented replacement").unwrap();
            fs::rename(&replacement, &swapped).unwrap();
        }))
    });
    let outcome = write_backup_in_folder(&folder, &stored(&demo()), Some(1), T0, &UnmeteredReadMemory, &mut Durable);
    assert_eq!(codes(&outcome), vec![PRUNE_FAILED], "{outcome:?}");
    assert_eq!(fs::read(folder.join(&older)).unwrap(), b"invented replacement", "the substitute is left in place");
}

// ---------------------------------------------------------------------------
// Configuration.
// ---------------------------------------------------------------------------

fn configure(sandbox: &Sandbox, enabled: bool, keep: Option<u32>, intent: bool, chosen: Option<PathBuf>, asked: &Cell<u32>) -> BackupConfigureOutcome {
    let request = ConfigureRequest { enabled, backup_keep: keep, explicit_local_private_intent: intent };
    configure_at(
        Ok(sandbox.data()),
        &request,
        || {
            asked.set(asked.get() + 1);
            Ok(chosen)
        },
        &UnmeteredReadMemory,
    )
}

fn record(sandbox: &Sandbox) -> SettingsRecord {
    crate::autosave_commands::read_settings_record(&sandbox.data(), &UnmeteredReadMemory).into_unreserved().unwrap().record
}

#[test]
fn turning_backups_on_records_the_intent_with_the_chosen_folder_and_off_deletes_nothing() {
    let _save_lock = atomic_save_test_lock();
    let sandbox = Sandbox::new("configure");
    let asked = Cell::new(0);
    assert!(matches!(crate::autosave_commands::set_settings_at(Ok(sandbox.data()), 15, &UnmeteredReadMemory), AutosaveSettingsWriteOutcome::Saved { .. }));
    let before = fs::read(sandbox.data().join(SETTINGS_FILE)).unwrap();
    // No intent, or a rotation that is not a choice: refused before the chooser.
    let BackupConfigureOutcome::Refused { diagnostics } = configure(&sandbox, true, None, false, Some(sandbox.folder()), &asked) else { panic!() };
    assert_eq!(diagnostics[0].code, INTENT_REQUIRED);
    let BackupConfigureOutcome::Refused { diagnostics } = configure(&sandbox, true, Some(4), true, Some(sandbox.folder()), &asked) else { panic!() };
    assert_eq!(diagnostics[0].code, SETTINGS_OUT_OF_RANGE);
    assert_eq!(asked.get(), 0, "the chooser was never shown");
    // Cancelled: nothing is written.
    assert_eq!(configure(&sandbox, true, None, true, None, &asked), BackupConfigureOutcome::Cancelled);
    // A symlinked folder is refused.
    let link = sandbox.root.join("link");
    std::os::unix::fs::symlink(sandbox.folder(), &link).unwrap();
    let BackupConfigureOutcome::Refused { diagnostics } = configure(&sandbox, true, None, true, Some(link), &asked) else { panic!() };
    assert_eq!(diagnostics[0].code, FOLDER_INVALID);
    assert_eq!(fs::read(sandbox.data().join(SETTINGS_FILE)).unwrap(), before, "no refusal or cancel wrote anything");
    // Chosen: the folder, the rotation and the intent together; the interval kept.
    let folder = sandbox.folder().to_str().unwrap().to_string();
    let outcome = configure(&sandbox, true, Some(5), true, Some(sandbox.folder()), &asked);
    assert_eq!(outcome, BackupConfigureOutcome::Saved { settings: AutosaveSettings { autosave_minutes: 15, backup_folder: Some(folder.clone()), backup_keep: Some(5) } });
    assert_eq!(record(&sandbox), SettingsRecord { autosave_minutes: 15, backup_folder: Some(folder), backup_keep: Some(5), backup_local_private_intent: true });
    // Off: the folder and the intent cleared, no file in the folder touched.
    let planted = plant(&sandbox, &variant(1), T0);
    let outcome = configure(&sandbox, false, Some(5), false, None, &asked);
    assert_eq!(outcome, BackupConfigureOutcome::Saved { settings: AutosaveSettings { autosave_minutes: 15, backup_folder: None, backup_keep: Some(5) } });
    assert_eq!(record(&sandbox).backup_local_private_intent, false);
    assert_eq!(sandbox.entries(), vec![planted]);
    assert_eq!(asked.get(), 3);
}

// ---------------------------------------------------------------------------
// Admission and arguments (§6).
// ---------------------------------------------------------------------------

#[test]
fn each_command_is_admitted_within_its_budget_and_takes_its_named_arguments() {
    let registry = registry(ScriptedHost::normal());
    let raw = |args: &Value| tauri::ipc::InvokeBody::Raw(serde_json::to_vec(args).unwrap());
    let padded = |len: usize| json!({ "projectId": "x".repeat(len - r#"{"projectId":""}"#.len()) });
    for (command, max) in [(CONFIGURE_COMMAND, CONFIGURE_COMMAND_MAX_ARGS_BYTES), (WRITE_COMMAND, WRITE_COMMAND_MAX_ARGS_BYTES)] {
        let (_, lease) = admit_body(&registry, &raw(&padded(max)), &headers(Some(INGRESS_ARGS_JSON_V1)), command).unwrap();
        drop(lease);
        let refused = admit_body(&registry, &raw(&padded(max + 1)), &headers(Some(INGRESS_ARGS_JSON_V1)), command).map(drop).unwrap_err();
        assert!(format!("{refused:?}").contains("INGRESS-ARGS-TOO-LARGE"), "{command}");
    }
    assert_eq!((CONFIGURE_COMMAND_MAX_ARGS_BYTES, WRITE_COMMAND_MAX_ARGS_BYTES), (4 * 1024, 256 * 1024));
    let (mut args, _lease) = admit_json(&registry, CONFIGURE_COMMAND, ArgsDecoding::Moved, &json!({"enabled": true, "backupKeep": null, "explicitLocalPrivateIntent": true})).unwrap();
    assert_eq!(configure_request(&mut args).unwrap(), ConfigureRequest { enabled: true, backup_keep: None, explicit_local_private_intent: true });
    let (mut args, _lease) = admit_json(&registry, CONFIGURE_COMMAND, ArgsDecoding::Moved, &json!({"enabled": false, "backupKeep": 10, "explicitLocalPrivateIntent": false})).unwrap();
    assert_eq!(configure_request(&mut args).unwrap(), ConfigureRequest { enabled: false, backup_keep: Some(10), explicit_local_private_intent: false });
    for broken in [json!({"backupKeep": null, "explicitLocalPrivateIntent": true}), json!({"enabled": true, "backupKeep": -1, "explicitLocalPrivateIntent": true}), json!({"enabled": true, "backupKeep": null})] {
        let (mut args, _lease) = admit_json(&registry, CONFIGURE_COMMAND, ArgsDecoding::Moved, &broken).unwrap();
        assert!(format!("{:?}", configure_request(&mut args).unwrap_err()).contains("INGRESS-ARGS-INVALID"), "{broken}");
    }
    let (mut args, _lease) = admit_json(&registry, WRITE_COMMAND, ArgsDecoding::Moved, &json!({"projectId": PROJECT, "expectedModelHash": "sha256:00"})).unwrap();
    let decoded = write_request(&mut args).unwrap();
    assert_eq!((decoded.project_id.as_str(), decoded.expected_model_hash.as_str()), (PROJECT, "sha256:00"));
    for broken in [json!({"projectId": PROJECT}), json!({"expectedModelHash": "sha256:00"})] {
        let (mut args, _lease) = admit_json(&registry, WRITE_COMMAND, ArgsDecoding::Moved, &broken).unwrap();
        assert!(format!("{:?}", write_request(&mut args).unwrap_err()).contains("INGRESS-ARGS-INVALID"), "{broken}");
    }
    assert_eq!(registry.pool_status().reserved_bytes, 0);
}
