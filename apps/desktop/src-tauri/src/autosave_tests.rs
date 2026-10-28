//! Tests of `autosave.rs` (DEL-02-05 Design `autosave-and-backup.md` §8, slice 1).
//! Every test uses invented documents in its own temporary directory.

use super::*;
use crate::atomic_report_package_save::test_support::atomic_save_test_lock;
use crate::model_document_file::{read_model_document_file, ModelDocumentOpenOutcome};
use crate::stored_read::UnmeteredReadMemory;
use open_pipe_stress_operation_applier::canonical_json;
use serde_json::json;
use std::io::Write;
use std::path::PathBuf;
use std::sync::mpsc;

const DEMO: &str = include_str!("../../../../fixtures/product_preview/invented_demo_model.json");
const EVERY_SLOT: &str = include_str!("../../../../fixtures/model_document/invented_every_slot_model.json");
const STORE_SENTINEL: &[u8] = b"invented store bytes: never opened by an autosave";
const DEMO_PROJECT: &str = "project:invented-demo-loop-01";

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

fn demo_named(name: &str) -> (String, String) {
    canonical(DEMO, |document| document["project"]["name"] = json!(name))
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

    /// The directory's entries other than lock files, sorted.
    fn names(&self) -> Vec<String> {
        let mut names: Vec<String> = fs::read_dir(self.directory())
            .map(|entries| entries.map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned()).collect())
            .unwrap_or_default();
        names.retain(|name| !name.ends_with(".lock"));
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
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(self.directory(), fs::Permissions::from_mode(0o755));
        // Only this test's own temporary directory.
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

fn listed(directory: &Path) -> Vec<String> {
    list_autosaves(directory, &UnmeteredReadMemory).unwrap().into_unreserved().unwrap().entries.into_iter().map(|entry| entry.file_name).collect()
}

fn assert_no_temp(sandbox: &Sandbox) {
    assert!(sandbox.names().iter().all(|name| !name.starts_with(AUTOSAVE_FORMAT.temp_prefix)), "{:?}", sandbox.names());
}

#[derive(Clone, Copy, PartialEq)]
enum Fault {
    None,
    DiskFullAtWrite,
    InterruptedRename,
    CorruptWrite,
}

/// The writer's I/O with an injected fault, an optional failed directory sync
/// of one path, and a record of the directories synced.
struct FaultyIo {
    fault: Fault,
    fail_sync_of: Option<PathBuf>,
    synced: Vec<PathBuf>,
}

impl FaultyIo {
    fn new(fault: Fault) -> Self {
        Self { fault, fail_sync_of: None, synced: Vec::new() }
    }

    fn failing_sync_of(path: PathBuf) -> Self {
        Self { fault: Fault::None, fail_sync_of: Some(path), synced: Vec::new() }
    }
}

impl AtomicSaveIo for FaultyIo {
    fn write_all(&mut self, temp: &mut File, bytes: &[u8]) -> io::Result<()> {
        match self.fault {
            // ENOSPC as the OS reports a full disk.
            Fault::DiskFullAtWrite => {
                temp.write_all(&bytes[..bytes.len() / 2])?;
                Err(io::Error::from_raw_os_error(libc::ENOSPC))
            }
            // A write the OS reports as complete whose bytes differ.
            Fault::CorruptWrite => {
                let mut altered = bytes.to_vec();
                let last = altered.len() - 2;
                altered[last] = if altered[last] == b'1' { b'2' } else { b'1' };
                temp.write_all(&altered)
            }
            Fault::None | Fault::InterruptedRename => temp.write_all(bytes),
        }
    }

    fn flush(&mut self, temp: &mut File) -> io::Result<()> {
        temp.flush()
    }

    fn sync_temp(&mut self, temp: &File) -> io::Result<()> {
        temp.sync_all()
    }

    fn rename(&mut self, source: &Path, destination: &Path) -> io::Result<()> {
        if self.fault == Fault::InterruptedRename {
            return Err(io::Error::new(io::ErrorKind::Interrupted, "injected interrupted rename"));
        }
        fs::rename(source, destination)
    }

    fn cleanup_temp(&mut self, temp_path: &Path) -> io::Result<()> {
        fs::remove_file(temp_path)
    }

    fn sync_parent(&mut self, parent: &Path) -> DirectorySyncResult {
        self.synced.push(parent.to_path_buf());
        if self.fail_sync_of.as_deref() == Some(parent) {
            return DirectorySyncResult::Failed { kind: "Other".to_string(), raw_os_error: Some(libc::EIO) };
        }
        File::open(parent).and_then(|directory| directory.sync_all()).map_or_else(
            |error| DirectorySyncResult::Failed { kind: format!("{:?}", error.kind()), raw_os_error: error.raw_os_error() },
            |()| DirectorySyncResult::Durable,
        )
    }
}

fn write_with(directory: &Path, text: &str, hash: &str, millis: u64, io: &mut FaultyIo) -> AutosaveWriteOutcome {
    write_autosave_with_io(directory, text, hash, millis, &UnmeteredReadMemory, io)
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
    assert_eq!(slot, autosave_slot(DEMO_PROJECT));
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
    let listing = list_autosaves(&sandbox.directory(), &UnmeteredReadMemory).unwrap().into_unreserved().unwrap();
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
    assert_eq!(again, AutosaveWriteOutcome::Unchanged { file_name: first.clone(), slot: autosave_slot(DEMO_PROJECT), model_hash: hash });
    assert_eq!(sandbox.names(), vec![first]);
}

/// Review r1 P2: "unchanged" verifies the newest record first. A record
/// truncated under its name does not stop a new, verified record of the
/// same model, and the broken one is then pruned as older.
#[test]
fn a_same_model_record_that_does_not_verify_is_written_again() {
    let _lock = atomic_save_test_lock();
    let sandbox = Sandbox::new("unchanged-broken");
    let directory = sandbox.directory();
    let (text, hash) = demo();
    let first = written_name(&write(&directory, &text, &hash, 1_000));
    fs::write(directory.join(&first), &text.as_bytes()[..text.len() / 2]).unwrap();
    let second = written_name(&write(&directory, &text, &hash, 2_000));
    assert_ne!(second, first);
    assert_eq!(sandbox.names(), vec![second.clone()]);
    assert_eq!(read_codes(read_autosave(&directory, &second, &UnmeteredReadMemory)), vec!["read".to_string()]);
}

/// A newer record of the slot removes the slot's strictly older records, and
/// only them: another project's record, other files, a symlink and a
/// directory named like the slot's records stay.
#[test]
fn a_new_record_removes_only_its_own_slots_older_records() {
    let _lock = atomic_save_test_lock();
    let sandbox = Sandbox::new("prune");
    let directory = sandbox.directory();
    let (first, first_hash) = demo();
    let (second, second_hash) = demo_named("Invented Utility Loop Demo, edited");
    let (third, third_hash) = demo_named("Invented, written later by another window");
    let (other, other_hash) = canonical(EVERY_SLOT, |_| {});
    let first_name = written_name(&write(&directory, &first, &first_hash, 1_000));
    let other_name = written_name(&write(&directory, &other, &other_hash, 1_500));
    // A record of the slot newer than the next write (another window's).
    let third_name = AutosaveName { slot: autosave_slot(DEMO_PROJECT), written_unix_millis: 3_000, model_hash_hex: third_hash["sha256:".len()..].to_string() }.file_name();
    fs::write(directory.join(&third_name), &third).unwrap();
    let slot = autosave_slot(DEMO_PROJECT);
    let lookalike_dir = format!("{slot}-0000000000900-{}.json", "a".repeat(64));
    fs::create_dir(directory.join(&lookalike_dir)).unwrap();
    fs::write(directory.join("notes.json"), b"invented").unwrap();
    let lookalike_link = format!("{slot}-0000000000800-{}.json", "b".repeat(64));
    std::os::unix::fs::symlink(directory.join("notes.json"), directory.join(&lookalike_link)).unwrap();
    let second_name = written_name(&write(&directory, &second, &second_hash, 2_000));
    let names = sandbox.names();
    assert!(!names.contains(&first_name), "the slot's older record is removed: {names:?}");
    for kept in [&second_name, &third_name, &other_name, &lookalike_dir, &lookalike_link, &"notes.json".to_string()] {
        assert!(names.contains(kept), "{kept} is kept: {names:?}");
    }
    assert_eq!(listed(&directory), vec![third_name, second_name, other_name], "newest first; only regular record files are listed");
    sandbox.assert_store_untouched();
}

/// Review r1 P1: two writers of one slot, as two windows, never leave it
/// without a valid record. After every write, under the slot's lock, the
/// slot's newest record reads back verified; at the end at least one does.
#[test]
fn two_writers_on_one_slot_always_leave_a_valid_record() {
    let _lock = atomic_save_test_lock();
    let sandbox = Sandbox::new("concurrent");
    let directory = sandbox.directory();
    fs::create_dir(&directory).unwrap();
    let slot = autosave_slot(DEMO_PROJECT);
    let rounds = 60_u64;
    let workers: Vec<_> = (0..2_u64)
        .map(|writer| {
            let directory = directory.clone();
            let slot = slot.clone();
            std::thread::spawn(move || {
                for round in 0..rounds {
                    let (text, hash) = demo_named(&format!("Invented, window {writer} round {round}"));
                    let outcome = write(&directory, &text, &hash, 10_000 + round * 2 + writer);
                    assert!(matches!(outcome, AutosaveWriteOutcome::Written { .. }), "{outcome:?}");
                    let _held = lock_slot_at(&directory, &slot, LOCK_WAIT).unwrap();
                    let records = listed(&directory);
                    let newest = records.first().unwrap_or_else(|| panic!("window {writer} round {round}: no record survives"));
                    assert_eq!(read_codes(read_autosave(&directory, newest, &UnmeteredReadMemory)), vec!["read".to_string()], "{newest}");
                }
            })
        })
        .collect();
    for worker in workers {
        worker.join().unwrap();
    }
    let records = listed(&directory);
    assert!(!records.is_empty());
    for record in &records {
        assert_eq!(read_codes(read_autosave(&directory, record, &UnmeteredReadMemory)), vec!["read".to_string()], "{record}");
    }
    sandbox.assert_store_untouched();
}

/// A write takes its slot's lock for the whole transaction and waits while
/// another holder has it; another slot's write does not wait.
#[test]
fn a_write_waits_for_its_slots_lock_and_another_slot_does_not() {
    let _lock = atomic_save_test_lock();
    let sandbox = Sandbox::new("lock");
    let directory = sandbox.directory();
    fs::create_dir(&directory).unwrap();
    let held = lock_slot_at(&directory, &autosave_slot(DEMO_PROJECT), LOCK_WAIT).unwrap();
    let (sender, receiver) = mpsc::channel();
    let writer = {
        let directory = directory.clone();
        std::thread::spawn(move || {
            let (text, hash) = demo();
            sender.send(write(&directory, &text, &hash, 1_000)).unwrap();
        })
    };
    assert!(receiver.recv_timeout(Duration::from_millis(300)).is_err(), "the write did not wait for the slot's lock");
    let (other, other_hash) = canonical(EVERY_SLOT, |_| {});
    written_name(&write(&directory, &other, &other_hash, 1_500));
    drop(held);
    written_name(&receiver.recv_timeout(Duration::from_secs(5)).unwrap());
    writer.join().unwrap();
    // A lock held past the wait refuses by name.
    let _held = lock_slot_at(&directory, &autosave_slot(DEMO_PROJECT), LOCK_WAIT).unwrap();
    let busy = lock_slot_at(&directory, &autosave_slot(DEMO_PROJECT), Duration::from_millis(50)).err().unwrap();
    assert_eq!(busy.code, LOCK_BUSY);
}

/// Review r1 P2: a write whose directory sync did not complete keeps the
/// older records; the next durable write removes every older one.
#[test]
fn older_records_are_kept_until_a_write_is_durable() {
    let _lock = atomic_save_test_lock();
    let sandbox = Sandbox::new("durability");
    let directory = sandbox.directory();
    let (first, first_hash) = demo();
    let (second, second_hash) = demo_named("edited");
    let (third, third_hash) = demo_named("edited again");
    let first_name = written_name(&write(&directory, &first, &first_hash, 1_000));
    let outcome = write_with(&directory, &second, &second_hash, 2_000, &mut FaultyIo::failing_sync_of(directory.clone()));
    let AutosaveWriteOutcome::Written { file_name: second_name, durability, warnings, .. } = outcome.clone() else { panic!("{outcome:?}") };
    assert_eq!(durability, "saved_durability_uncertain");
    assert_eq!(warnings.iter().map(|item| item.code.as_str()).collect::<Vec<_>>(), vec![PRUNE_DEFERRED]);
    assert_eq!(sandbox.names(), vec![first_name, second_name]);
    let third_name = written_name(&write(&directory, &third, &third_hash, 3_000));
    assert_eq!(sandbox.names(), vec![third_name]);
}

/// Review r1 P2: creating `autosave/` syncs its parent, and a failed sync
/// makes the first record's durability uncertain.
#[test]
fn creating_the_directory_syncs_its_parent() {
    let _lock = atomic_save_test_lock();
    let sandbox = Sandbox::new("mkdir-sync");
    let directory = sandbox.directory();
    let (text, hash) = demo();
    let mut io = FaultyIo::new(Fault::None);
    let outcome = write_with(&directory, &text, &hash, 1_000, &mut io);
    assert!(matches!(&outcome, AutosaveWriteOutcome::Written { durability, .. } if durability == "durable"), "{outcome:?}");
    assert_eq!(io.synced, vec![sandbox.root.clone(), directory.clone()], "the parent after the mkdir, then the directory after the rename");
    let other = Sandbox::new("mkdir-sync-fails");
    let outcome = write_with(&other.directory(), &text, &hash, 1_000, &mut FaultyIo::failing_sync_of(other.root.clone()));
    assert!(matches!(&outcome, AutosaveWriteOutcome::Written { durability, .. } if durability == "saved_durability_uncertain"), "{outcome:?}");
    // An existing directory is not synced again.
    let mut io = FaultyIo::new(Fault::None);
    let (edited, edited_hash) = demo_named("edited");
    written_name(&write_with(&directory, &edited, &edited_hash, 2_000, &mut io));
    assert_eq!(io.synced, vec![directory]);
}

/// Content refusals come before the directory is touched.
#[test]
fn a_text_that_is_not_a_current_canonical_document_of_its_hash_is_refused_before_any_write() {
    let sandbox = Sandbox::new("refusals");
    let directory = sandbox.directory();
    let (text, _) = demo();
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
    sandbox.assert_store_untouched();
}

struct Refusing;

impl ReadMemory for Refusing {
    fn reserve(&self, what: &str, bytes: u64) -> Result<ReadReservation, String> {
        Err(format!("{STORE_READ_MEMORY_UNAVAILABLE}: reading {what} needs {bytes} bytes; refused."))
    }
}

/// A memory refusal refuses the record before its text is parsed.
#[test]
fn a_memory_refusal_refuses_before_parsing() {
    let sandbox = Sandbox::new("memory");
    let (text, hash) = demo();
    let outcome = write_autosave(&sandbox.directory(), &text, &hash, 1_000, &Refusing);
    assert_eq!(refused_codes(&outcome), vec![STORE_READ_MEMORY_UNAVAILABLE.to_string()]);
    assert!(!sandbox.directory().exists());
}

/// Review r1 P2: a listing keeps at most its limit of the newest records,
/// reserved first, and counts the rest; writes into that directory still work.
#[test]
fn a_listing_keeps_the_newest_within_its_limit_reserved_first_and_counts_the_rest() {
    let _lock = atomic_save_test_lock();
    let sandbox = Sandbox::new("listing-bound");
    let directory = sandbox.directory();
    fs::create_dir(&directory).unwrap();
    let extra = 76_u64;
    let total = MAX_LISTED_AUTOSAVES as u64 + extra;
    for index in 0..total {
        let name = AutosaveName { slot: autosave_slot(&format!("project:invented-{index}")), written_unix_millis: 1_000 + index, model_hash_hex: format!("{index:064x}") };
        File::create(directory.join(name.file_name())).unwrap();
    }
    let listing = list_autosaves(&directory, &UnmeteredReadMemory).unwrap();
    assert_eq!(listing.reserved_bytes(), MAX_LISTED_AUTOSAVES as u64 * LISTED_ENTRY_BYTES);
    let listing = listing.into_unreserved().unwrap();
    assert_eq!((listing.entries.len(), listing.not_listed), (MAX_LISTED_AUTOSAVES, extra as usize));
    assert_eq!(listing.entries.first().unwrap().written_unix_millis, 1_000 + total - 1);
    assert_eq!(listing.entries.last().unwrap().written_unix_millis, 1_000 + extra);
    assert!(listing.entries.windows(2).all(|pair| pair[0].written_unix_millis > pair[1].written_unix_millis));
    assert_eq!(list_autosaves(&directory, &Refusing).unwrap_err().code, STORE_READ_MEMORY_UNAVAILABLE);
    let (text, hash) = demo();
    written_name(&write(&directory, &text, &hash, 5_000));
}

/// Disk full during the write: refused by name with the OS error, the
/// temporary file removed, the previous record kept and the store untouched.
#[test]
fn disk_full_is_reported_and_keeps_the_previous_record() {
    let _lock = atomic_save_test_lock();
    let sandbox = Sandbox::new("enospc");
    let directory = sandbox.directory();
    let (first, first_hash) = demo();
    let (second, second_hash) = demo_named("edited");
    let first_name = written_name(&write(&directory, &first, &first_hash, 1_000));
    let outcome = write_with(&directory, &second, &second_hash, 2_000, &mut FaultyIo::new(Fault::DiskFullAtWrite));
    let AutosaveWriteOutcome::Refused { diagnostics } = &outcome else { panic!("{outcome:?}") };
    assert_eq!(diagnostics[0].code, "AUTOSAVE-WRITE-FAILED");
    assert!(diagnostics[0].message.contains(&format!("os error {}", libc::ENOSPC)), "{}", diagnostics[0].message);
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
    let (second, second_hash) = demo_named("edited");
    let first_name = written_name(&write(&directory, &first, &first_hash, 1_000));
    let outcome = write_with(&directory, &second, &second_hash, 2_000, &mut FaultyIo::new(Fault::InterruptedRename));
    let AutosaveWriteOutcome::Refused { diagnostics } = &outcome else { panic!("{outcome:?}") };
    assert_eq!(diagnostics[0].code, "AUTOSAVE-RENAME-FAILED");
    assert!(diagnostics[0].message.contains("rename stage"), "{}", diagnostics[0].message);
    assert_eq!(sandbox.names(), vec![first_name.clone()]);
    assert_eq!(read_codes(read_autosave(&directory, &first_name, &UnmeteredReadMemory)), vec!["read".to_string()]);
    assert_no_temp(&sandbox);
    sandbox.assert_store_untouched();
}

/// Permission denied (a real read-only directory): refused by name, the
/// previous record kept. A user who can write a read-only directory (root)
/// cannot exercise this; the skip is written to the test output, uncaptured.
#[test]
fn permission_denied_is_reported_and_keeps_the_previous_record() {
    use std::os::unix::fs::PermissionsExt;
    let _lock = atomic_save_test_lock();
    let sandbox = Sandbox::new("eacces");
    let directory = sandbox.directory();
    let (first, first_hash) = demo();
    let (second, second_hash) = demo_named("edited");
    let first_name = written_name(&write(&directory, &first, &first_hash, 1_000));
    fs::set_permissions(&directory, fs::Permissions::from_mode(0o555)).unwrap();
    if File::create(directory.join("probe.tmp")).is_ok() {
        let _ = io::stderr().write_all(
            b"SKIPPED autosave::tests::permission_denied_is_reported_and_keeps_the_previous_record: this user can write a read-only directory\n",
        );
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
    let (second, second_hash) = demo_named("edited");
    let first_name = written_name(&write(&directory, &first, &first_hash, 1_000));
    let outcome = write_with(&directory, &second, &second_hash, 2_000, &mut FaultyIo::new(Fault::CorruptWrite));
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
    assert_eq!(listed(&directory), vec![first_name]);
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
    let (tampered, _) = demo_named("Invented, altered");
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
    let link = AutosaveName { written_unix_millis: 4_000, ..parsed.clone() }.file_name();
    std::os::unix::fs::symlink(directory.join(&name), directory.join(&link)).unwrap();
    assert_eq!(read_codes(read_autosave(&directory, &link, &UnmeteredReadMemory)), vec![UNREADABLE.to_string()]);
    assert_eq!(read_codes(read_autosave(&directory, &name, &UnmeteredReadMemory)), vec!["read".to_string()]);
    sandbox.assert_store_untouched();
}

/// Review r1 P2: a symlinked `autosave/` directory is refused by the read,
/// the listing and the write alike.
#[test]
fn a_symlinked_records_directory_is_refused_everywhere() {
    let _lock = atomic_save_test_lock();
    let sandbox = Sandbox::new("dir-link");
    let real = sandbox.root.join("elsewhere");
    let (text, hash) = demo();
    let name = written_name(&write(&real, &text, &hash, 1_000));
    std::os::unix::fs::symlink(&real, sandbox.directory()).unwrap();
    assert_eq!(read_codes(read_autosave(&sandbox.directory(), &name, &UnmeteredReadMemory)), vec![DIRECTORY_UNAVAILABLE.to_string()]);
    assert_eq!(list_autosaves(&sandbox.directory(), &UnmeteredReadMemory).unwrap_err().code, DIRECTORY_UNAVAILABLE);
    let (edited, edited_hash) = demo_named("edited");
    assert_eq!(refused_codes(&write(&sandbox.directory(), &edited, &edited_hash, 2_000)), vec![DIRECTORY_UNAVAILABLE.to_string()]);
}

fn before_open(hook: impl FnOnce() + 'static) {
    BEFORE_OPEN.with(|slot| *slot.borrow_mut() = Some(Box::new(hook)));
}

/// Review r1 P2: a record swapped for a symlink (to a valid record) after it
/// was inspected is refused by the descriptor's open and `fstat`.
#[test]
fn a_record_swapped_for_a_symlink_after_inspection_is_refused() {
    let _lock = atomic_save_test_lock();
    let sandbox = Sandbox::new("swap");
    let directory = sandbox.directory();
    let (text, hash) = demo();
    let name = written_name(&write(&directory, &text, &hash, 1_000));
    let elsewhere = sandbox.root.join("valid-copy.json");
    fs::copy(directory.join(&name), &elsewhere).unwrap();
    let path = directory.join(&name);
    before_open(move || {
        fs::remove_file(&path).unwrap();
        std::os::unix::fs::symlink(&elsewhere, &path).unwrap();
    });
    assert_eq!(read_codes(read_autosave(&directory, &name, &UnmeteredReadMemory)), vec![UNREADABLE.to_string()]);
}

/// Review r1 P2: a record that grows after it was inspected (its load was
/// reserved at the inspected size) is refused, not read past that size.
#[test]
fn a_record_that_grows_after_inspection_is_refused() {
    let _lock = atomic_save_test_lock();
    let sandbox = Sandbox::new("grow");
    let directory = sandbox.directory();
    let (text, hash) = demo();
    let name = written_name(&write(&directory, &text, &hash, 1_000));
    let path = directory.join(&name);
    before_open(move || {
        let mut file = fs::OpenOptions::new().append(true).open(&path).unwrap();
        file.write_all(&vec![b' '; 1 << 20]).unwrap();
    });
    let refused = read_autosave(&directory, &name, &UnmeteredReadMemory).err().unwrap();
    assert_eq!(refused[0].code, UNREADABLE);
    assert!(refused[0].message.contains("changed size"), "{}", refused[0].message);
}

/// The hard cap: a file longer than the inspected size is refused after at
/// most `len + 1` bytes, in a buffer allocated once.
#[test]
fn a_capped_read_never_reads_past_its_cap() {
    let sandbox = Sandbox::new("cap");
    let path = sandbox.root.join("long.bin");
    fs::write(&path, vec![b'x'; 10_000]).unwrap();
    let file = File::open(&path).unwrap();
    assert!(matches!(read_capped(&file, 100), Err(Access::Changed(_))));
    let file = File::open(&path).unwrap();
    assert!(matches!(stream_sha256_hex(&file, 100), Err(Access::Changed(_))));
    let file = File::open(&path).unwrap();
    let bytes = read_capped(&file, 10_000).ok().unwrap();
    assert_eq!((bytes.len(), bytes.capacity()), (10_000, 10_001));
    // A source longer than the cap yields at most `len + 1` bytes to the read.
    struct Counting<'a>(io::Cursor<Vec<u8>>, &'a std::cell::Cell<usize>);
    impl Read for Counting<'_> {
        fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
            let count = self.0.read(buffer)?;
            self.1.set(self.1.get() + count);
            Ok(count)
        }
    }
    let delivered = std::cell::Cell::new(0);
    assert!(matches!(read_at_most(Counting(io::Cursor::new(vec![b'x'; 1 << 20]), &delivered), 100), Err(Access::Changed(_))));
    assert!(delivered.get() <= 101, "read {} bytes past a 100-byte cap", delivered.get());
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
        format!("{slot}.lock"),
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
    assert_eq!(list_autosaves(&sandbox.directory(), &UnmeteredReadMemory).unwrap_err().code, DIRECTORY_UNAVAILABLE);
}
