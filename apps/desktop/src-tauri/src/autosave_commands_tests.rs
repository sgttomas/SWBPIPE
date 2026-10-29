//! Tests of `autosave_commands.rs` (DEL-02-05 Design `autosave-and-backup.md`
//! §8, slice 2): each command's admission and arguments, its lease-seeded
//! memory, and the settings file's default, fallbacks and range. Every test
//! uses invented documents in its own temporary directory.

use super::*;
use crate::atomic_report_package_save::test_support::atomic_save_test_lock;
use crate::command_ingress::test_support::{admit_json, headers, registry};
use crate::command_ingress::{lease_bytes, INGRESS_ARGS_JSON_V1};
use crate::memory_pool::test_host::ScriptedHost;
use crate::stored_read::{ReadReservation, UnmeteredReadMemory};
use open_pipe_stress_operation_applier::{canonical_json, sha256_hex};
use serde_json::json;
use std::path::PathBuf;

const DEMO: &str = include_str!("../../../../fixtures/product_preview/invented_demo_model.json");
const MILLIS: u64 = 1_790_000_000_000;

fn demo() -> (String, String) {
    let document: Value = serde_json::from_str(DEMO).unwrap();
    let text = canonical_json(&document);
    let hash = format!("sha256:{}", sha256_hex(&text));
    (text, hash)
}

/// A temporary app data directory (created) and nothing in it.
struct Sandbox {
    root: PathBuf,
}

impl Sandbox {
    fn new(label: &str) -> Self {
        let root = std::env::temp_dir().join(format!(
            "swbpipe-autosave-commands-{label}-{}-{}",
            std::process::id(),
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
        ));
        fs::create_dir_all(&root).unwrap();
        Self { root }
    }

    fn data(&self) -> Result<PathBuf, String> {
        Ok(self.root.clone())
    }

    fn settings_path(&self) -> PathBuf {
        self.root.join(SETTINGS_FILE)
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        // Only this test's own temporary directory.
        if self.root.starts_with(std::env::temp_dir()) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
}

fn raw(args: &Value) -> InvokeBody {
    InvokeBody::Raw(serde_json::to_vec(args).unwrap())
}

fn invalid_message(error: SolveJobError) -> String {
    match error {
        SolveJobError::InvalidRequest { message } => message,
        other => panic!("not INVALID_REQUEST: {other:?}"),
    }
}

fn admitted(registry: &SolveJobRegistry, command: &'static str, args: &Value) -> Result<(Args, Lease), SolveJobError> {
    admit_body(registry, &raw(args), &headers(Some(INGRESS_ARGS_JSON_V1)), command)
}

/// A body of exactly `len` bytes: `{"fileName":"xxx…"}`.
fn body_of(len: usize) -> Value {
    let overhead = r#"{"fileName":""}"#.len();
    json!({ "fileName": "x".repeat(len - overhead) })
}

// ---------------------------------------------------------------------------
// Admission and arguments (§6).
// ---------------------------------------------------------------------------

#[test]
fn each_command_is_admitted_under_its_section_6_rule() {
    let registry = registry(ScriptedHost::normal());
    // The write is model-sized: its whole body is admitted and leased as
    // envelope(raw) + M + 4 KiB, above the record commands' budget.
    let (text, hash) = demo();
    let big = json!({ "canonicalText": format!("{text}{}", " ".repeat(RECORD_COMMAND_MAX_ARGS_BYTES)), "claimedModelHash": hash });
    let bytes = serde_json::to_vec(&big).unwrap();
    assert!(bytes.len() > RECORD_COMMAND_MAX_ARGS_BYTES);
    let (_, lease) = admitted(&registry, WRITE_COMMAND, &big).expect("the write is admitted with its whole body");
    let rule = lease_bytes(crate::ingress::prescan(&bytes).unwrap(), ArgsDecoding::Moved).unwrap();
    assert_eq!(lease.bytes(), rule, "the write's lease is envelope(raw) + M + 4 KiB");
    drop(lease);
    // The record commands: within the history commands' 256 KiB.
    assert_eq!(RECORD_COMMAND_MAX_ARGS_BYTES, 256 * 1024);
    for command in [LIST_COMMAND, READ_COMMAND, DISCARD_COMMAND] {
        let (_, lease) = admitted(&registry, command, &body_of(RECORD_COMMAND_MAX_ARGS_BYTES)).unwrap_or_else(|error| panic!("{command}: {error:?}"));
        drop(lease);
        let refused = admitted(&registry, command, &body_of(RECORD_COMMAND_MAX_ARGS_BYTES + 1)).map(drop).unwrap_err();
        assert!(invalid_message(refused).starts_with("INGRESS-ARGS-TOO-LARGE"), "{command}");
    }
    // The settings commands: within 4 KiB.
    for command in [GET_SETTINGS_COMMAND, SET_SETTINGS_COMMAND] {
        let (_, lease) = admitted(&registry, command, &body_of(SETTINGS_COMMAND_MAX_ARGS_BYTES)).unwrap_or_else(|error| panic!("{command}: {error:?}"));
        drop(lease);
        let refused = admitted(&registry, command, &body_of(SETTINGS_COMMAND_MAX_ARGS_BYTES + 1)).map(drop).unwrap_err();
        assert!(invalid_message(refused).starts_with("INGRESS-ARGS-TOO-LARGE"), "{command}");
    }
    assert_eq!(registry.pool_status().reserved_bytes, 0, "every refused or dropped admission released its lease");
}

#[test]
fn each_command_takes_its_named_arguments() {
    let registry = registry(ScriptedHost::normal());
    let (text, hash) = demo();
    let (mut args, _lease) = admit_json(&registry, WRITE_COMMAND, ArgsDecoding::Moved, &json!({"canonicalText": text, "claimedModelHash": hash})).unwrap();
    let request = write_request(&mut args).unwrap();
    assert_eq!((request.canonical_text.as_str(), request.claimed_model_hash.as_str()), (text.as_str(), hash.as_str()));
    for broken in [json!({"canonicalText": text}), json!({"claimedModelHash": hash}), json!({"canonicalText": 1, "claimedModelHash": hash})] {
        let (mut args, _lease) = admit_json(&registry, WRITE_COMMAND, ArgsDecoding::Moved, &broken).unwrap();
        assert!(invalid_message(write_request(&mut args).unwrap_err()).starts_with("INGRESS-ARGS-INVALID"), "{broken}");
    }
    let (mut args, _lease) = admit_json(&registry, READ_COMMAND, ArgsDecoding::Moved, &json!({"fileName": "a.json"})).unwrap();
    assert_eq!(file_name_request(&mut args).unwrap(), "a.json");
    let (mut args, _lease) = admit_json(&registry, DISCARD_COMMAND, ArgsDecoding::Moved, &json!({})).unwrap();
    assert!(invalid_message(file_name_request(&mut args).unwrap_err()).starts_with("INGRESS-ARGS-INVALID"));
    let (mut args, _lease) = admit_json(&registry, SET_SETTINGS_COMMAND, ArgsDecoding::Moved, &json!({"autosaveMinutes": 15})).unwrap();
    assert_eq!(set_settings_request(&mut args).unwrap(), 15);
    for broken in [json!({}), json!({"autosaveMinutes": "5"}), json!({"autosaveMinutes": -1}), json!({"autosaveMinutes": 2.5})] {
        let (mut args, _lease) = admit_json(&registry, SET_SETTINGS_COMMAND, ArgsDecoding::Moved, &broken).unwrap();
        assert!(invalid_message(set_settings_request(&mut args).unwrap_err()).starts_with("INGRESS-ARGS-INVALID"), "{broken}");
    }
}

/// §6: the write's memory is seeded with its ingress lease, and slice 1's
/// reservations (the parse, the canonical copy, the read-back block) are made
/// within it. The whole command, from the body's parse to the outcome, is
/// measured by the counting allocator: its priced peak is within the most the
/// command held at once, less the buffered body the lease also prices.
#[test]
fn the_write_runs_within_its_lease_and_its_measured_peak_within_what_it_held() {
    let _save_lock = atomic_save_test_lock();
    let sandbox = Sandbox::new("lease");
    let registry = registry(ScriptedHost::normal());
    let (text, hash) = demo();
    let body = raw(&json!({"canonicalText": text, "claimedModelHash": hash}));
    let InvokeBody::Raw(bytes) = &body else { unreachable!() };
    let body_envelope = crate::ingress::allocation_envelope(bytes.len() as u64).unwrap();
    let header = headers(Some(INGRESS_ARGS_JSON_V1));
    let ((outcome, lease_held, seeded, peak), measured) = crate::ingress::counting::measure(|| {
        let (mut args, lease) = admit_body(&registry, &body, &header, WRITE_COMMAND).unwrap();
        let request = write_request(&mut args).unwrap();
        drop(args);
        let memory = seeded_memory(&registry, &lease);
        let seeded = memory.held();
        let outcome = write_at(sandbox.data(), request, MILLIS, &memory);
        let peak = memory.peak_reserved();
        (outcome, lease.bytes(), seeded, peak)
    });
    assert!(matches!(outcome, AutosaveWriteOutcome::Written { .. }), "{outcome:?}");
    assert_eq!(seeded, lease_held, "the memory starts from the ingress lease");
    assert!(peak > lease_held, "the parse, the canonical copy and the read-back block are reserved within it: {peak}");
    eprintln!("WRITE_PROJECT_AUTOSAVE text_bytes={} lease={lease_held} reserved_peak={peak} measured_peak={}", text.len(), measured.peak_priced);
    assert!(measured.peak_priced <= peak - body_envelope, "measured peak {} exceeds what the command held {peak} less the body {body_envelope}", measured.peak_priced);
    assert_eq!(registry.pool_status().reserved_bytes, 0, "everything is released when the command returns");
    let written = fs::read(sandbox.root.join(AUTOSAVE_DIRECTORY).join(match &outcome {
        AutosaveWriteOutcome::Written { file_name, .. } => file_name.clone(),
        _ => unreachable!(),
    }))
    .unwrap();
    assert_eq!(written, text.as_bytes());
}

#[test]
fn a_record_is_listed_read_verified_and_discarded_in_the_app_data_directory() {
    let _save_lock = atomic_save_test_lock();
    let sandbox = Sandbox::new("records");
    let (text, hash) = demo();
    // No directory yet: an empty listing, and a discard finds nothing.
    assert_eq!(list_at(sandbox.data(), &UnmeteredReadMemory).into_unreserved().unwrap(), AutosaveListOutcome::Listed { entries: vec![], not_listed: 0 });
    let request = WriteRequest { canonical_text: text.clone(), claimed_model_hash: hash.clone() };
    let AutosaveWriteOutcome::Written { file_name, slot, .. } = write_at(sandbox.data(), request, MILLIS, &UnmeteredReadMemory) else { panic!("not written") };
    let AutosaveListOutcome::Listed { entries, not_listed } = list_at(sandbox.data(), &UnmeteredReadMemory).into_unreserved().unwrap() else { panic!("not listed") };
    assert_eq!(not_listed, 0);
    assert_eq!(entries.len(), 1);
    assert_eq!((entries[0].file_name.as_str(), entries[0].slot.as_str(), entries[0].model_hash.as_str()), (file_name.as_str(), slot.as_str(), hash.as_str()));
    assert_eq!(entries[0].written_unix_millis, MILLIS);
    let AutosaveReadOutcome::Read { document, model_hash, byte_count, .. } = read_at(sandbox.data(), &file_name, &UnmeteredReadMemory).into_unreserved().unwrap() else { panic!("not read") };
    assert_eq!(model_hash, hash);
    assert_eq!(byte_count, text.len() as u64);
    assert_eq!(canonical_json(&document), text);
    assert_eq!(discard_at(sandbox.data(), &file_name), AutosaveDiscardOutcome::Discarded { file_name: file_name.clone() });
    assert!(!sandbox.root.join(AUTOSAVE_DIRECTORY).join(&file_name).exists());
    assert_eq!(discard_at(sandbox.data(), &file_name), AutosaveDiscardOutcome::NotFound { file_name: file_name.clone() });
    let AutosaveReadOutcome::Refused { diagnostics } = read_at(sandbox.data(), &file_name, &UnmeteredReadMemory).into_unreserved().unwrap() else { panic!("read a discarded record") };
    assert!(!diagnostics.is_empty());
    // A name that is not a record's is refused, and nothing is removed.
    for name in ["../autosave-settings.json", "a.json", ""] {
        let AutosaveDiscardOutcome::Refused { diagnostics } = discard_at(sandbox.data(), name) else { panic!("{name}") };
        assert_eq!(diagnostics[0].code, autosave::NAME_INVALID);
    }
}

#[test]
fn a_discard_never_follows_or_removes_a_symlink() {
    let sandbox = Sandbox::new("discard-link");
    let directory = sandbox.root.join(AUTOSAVE_DIRECTORY);
    fs::create_dir_all(&directory).unwrap();
    let target = sandbox.root.join("outside.json");
    fs::write(&target, b"invented bytes outside the records").unwrap();
    let name = format!("{}-{MILLIS:013}-{}.json", autosave::autosave_slot("project:invented-link"), "a".repeat(64));
    std::os::unix::fs::symlink(&target, directory.join(&name)).unwrap();
    let AutosaveDiscardOutcome::Refused { diagnostics } = discard_at(sandbox.data(), &name) else { panic!("a symlink was discarded") };
    assert_eq!(diagnostics[0].code, autosave::UNREADABLE);
    assert!(fs::symlink_metadata(directory.join(&name)).unwrap().file_type().is_symlink(), "the link is left in place");
    assert_eq!(fs::read(&target).unwrap(), b"invented bytes outside the records");
}

// ---------------------------------------------------------------------------
// Settings (§2).
// ---------------------------------------------------------------------------

#[test]
fn the_settings_default_is_on_every_five_minutes() {
    let sandbox = Sandbox::new("settings-default");
    let read = get_settings_at(sandbox.data(), &UnmeteredReadMemory).into_unreserved().unwrap();
    assert_eq!(read, AutosaveSettingsRead { settings: AutosaveSettings { autosave_minutes: 5, backup_folder: None, backup_keep: None }, source: "default", status: None });
    assert!(!sandbox.settings_path().exists(), "a read writes nothing");
    assert_eq!(
        settings_text(&AutosaveSettings::default_settings()),
        r#"{"document_kind":"swbpipe.desktop.autosave_settings","version":1,"autosave_minutes":5,"backup_folder":null,"backup_keep":null}"#
    );
    // The default is one of the choices, and the choices are the Design's.
    assert_eq!(AUTOSAVE_MINUTE_CHOICES, [0, 1, 2, 5, 10, 15, 30]);
    assert!(AUTOSAVE_MINUTE_CHOICES.contains(&DEFAULT_AUTOSAVE_MINUTES) && DEFAULT_AUTOSAVE_MINUTES != 0);
}

fn file_with(minutes: Value) -> String {
    json!({"document_kind": SETTINGS_KIND, "version": 1, "autosave_minutes": minutes, "backup_folder": null, "backup_keep": null}).to_string()
}

#[test]
fn each_bad_settings_file_reads_as_the_default_with_a_status_naming_it() {
    let cases: Vec<(&str, Vec<u8>, &str)> = vec![
        ("not JSON", b"{\"version\":1,".to_vec(), SETTINGS_UNREADABLE),
        ("not an object", b"[1]".to_vec(), SETTINGS_UNREADABLE),
        ("another kind", json!({"document_kind": "other", "version": 1, "autosave_minutes": 5, "backup_folder": null, "backup_keep": null}).to_string().into_bytes(), SETTINGS_UNREADABLE),
        ("an unknown member", json!({"document_kind": SETTINGS_KIND, "version": 1, "autosave_minutes": 5, "backup_folder": null, "backup_keep": null, "extra": 1}).to_string().into_bytes(), SETTINGS_UNREADABLE),
        ("a folder that is not a path", json!({"document_kind": SETTINGS_KIND, "version": 1, "autosave_minutes": 5, "backup_folder": 3, "backup_keep": null}).to_string().into_bytes(), SETTINGS_UNREADABLE),
        ("too large", format!("{}{}", file_with(json!(10)), " ".repeat(MAX_SETTINGS_FILE_BYTES as usize)).into_bytes(), SETTINGS_UNREADABLE),
        ("version 2", json!({"document_kind": SETTINGS_KIND, "version": 2, "autosave_minutes": 10, "backup_folder": null, "backup_keep": null}).to_string().into_bytes(), SETTINGS_VERSION_UNKNOWN),
        ("no version", json!({"document_kind": SETTINGS_KIND, "autosave_minutes": 10, "backup_folder": null, "backup_keep": null}).to_string().into_bytes(), SETTINGS_VERSION_UNKNOWN),
        ("7 minutes", file_with(json!(7)).into_bytes(), SETTINGS_OUT_OF_RANGE),
        ("31 minutes", file_with(json!(31)).into_bytes(), SETTINGS_OUT_OF_RANGE),
        ("-1 minutes", file_with(json!(-1)).into_bytes(), SETTINGS_OUT_OF_RANGE),
        ("2.5 minutes", file_with(json!(2.5)).into_bytes(), SETTINGS_OUT_OF_RANGE),
        ("\"5\" minutes", file_with(json!("5")).into_bytes(), SETTINGS_OUT_OF_RANGE),
        ("no minutes", json!({"document_kind": SETTINGS_KIND, "version": 1, "backup_folder": null, "backup_keep": null}).to_string().into_bytes(), SETTINGS_OUT_OF_RANGE),
        ("keep 4", json!({"document_kind": SETTINGS_KIND, "version": 1, "autosave_minutes": 10, "backup_folder": null, "backup_keep": 4}).to_string().into_bytes(), SETTINGS_OUT_OF_RANGE),
    ];
    for (label, bytes, code) in cases {
        let sandbox = Sandbox::new("settings-bad");
        fs::write(sandbox.settings_path(), &bytes).unwrap();
        let read = get_settings_at(sandbox.data(), &UnmeteredReadMemory).into_unreserved().unwrap();
        assert_eq!((read.settings.clone(), read.source), (AutosaveSettings::default_settings(), "default"), "{label}");
        let status = read.status.unwrap_or_else(|| panic!("{label}: no status"));
        assert_eq!(status.code, code, "{label}: {}", status.message);
        assert!(status.message.contains("every 5 minutes"), "{label}: {}", status.message);
        assert_eq!(fs::read(sandbox.settings_path()).unwrap(), bytes, "{label}: a read never rewrites the file");
    }
    // Not a regular file: a directory, and a symlink (never followed) to a valid file.
    let sandbox = Sandbox::new("settings-dir");
    fs::create_dir(sandbox.settings_path()).unwrap();
    let read = get_settings_at(sandbox.data(), &UnmeteredReadMemory).into_unreserved().unwrap();
    assert_eq!((read.settings.autosave_minutes, read.status.map(|status| status.code)), (5, Some(SETTINGS_UNREADABLE.to_string())));
    let sandbox = Sandbox::new("settings-link");
    let elsewhere = sandbox.root.join("elsewhere.json");
    fs::write(&elsewhere, file_with(json!(30))).unwrap();
    std::os::unix::fs::symlink(&elsewhere, sandbox.settings_path()).unwrap();
    let read = get_settings_at(sandbox.data(), &UnmeteredReadMemory).into_unreserved().unwrap();
    assert_eq!((read.settings.autosave_minutes, read.status.map(|status| status.code)), (5, Some(SETTINGS_UNREADABLE.to_string())));
    // A valid file is read as it is.
    let sandbox = Sandbox::new("settings-good");
    fs::write(sandbox.settings_path(), file_with(json!(0))).unwrap();
    let read = get_settings_at(sandbox.data(), &UnmeteredReadMemory).into_unreserved().unwrap();
    assert_eq!((read.settings.autosave_minutes, read.source, read.status), (0, "file", None));
}

#[test]
fn a_settings_read_that_cannot_be_reserved_reads_as_the_default_with_its_status() {
    struct Refusing;
    impl ReadMemory for Refusing {
        fn reserve(&self, what: &str, _bytes: u64) -> Result<ReadReservation, String> {
            Err(format!("STORE-READ-MEMORY-UNAVAILABLE: {what} refused (invented)"))
        }
    }
    let sandbox = Sandbox::new("settings-memory");
    fs::write(sandbox.settings_path(), file_with(json!(30))).unwrap();
    let read = get_settings_at(sandbox.data(), &Refusing).into_unreserved().unwrap();
    assert_eq!((read.settings.autosave_minutes, read.source), (5, "default"));
    assert_eq!(read.status.unwrap().code, stored_read::STORE_READ_MEMORY_UNAVAILABLE);
}

#[test]
fn setting_the_interval_writes_the_file_atomically_and_refuses_values_that_are_not_choices() {
    let _save_lock = atomic_save_test_lock();
    let sandbox = Sandbox::new("settings-set");
    for minutes in AUTOSAVE_MINUTE_CHOICES {
        let outcome = set_settings_at(sandbox.data(), minutes, &UnmeteredReadMemory);
        assert_eq!(outcome, AutosaveSettingsWriteOutcome::Saved { settings: AutosaveSettings { autosave_minutes: minutes, backup_folder: None, backup_keep: None } });
        assert_eq!(fs::read_to_string(sandbox.settings_path()).unwrap(), settings_text(&AutosaveSettings { autosave_minutes: minutes, backup_folder: None, backup_keep: None }));
        let read = get_settings_at(sandbox.data(), &UnmeteredReadMemory).into_unreserved().unwrap();
        assert_eq!((read.settings.autosave_minutes, read.source, read.status), (minutes, "file", None));
    }
    let before = fs::read(sandbox.settings_path()).unwrap();
    for minutes in [3, 4, 6, 7, 20, 31, 60, u32::MAX] {
        let AutosaveSettingsWriteOutcome::Refused { diagnostics } = set_settings_at(sandbox.data(), minutes, &UnmeteredReadMemory) else { panic!("{minutes} accepted") };
        assert_eq!(diagnostics[0].code, SETTINGS_OUT_OF_RANGE, "{minutes}");
        assert_eq!(fs::read(sandbox.settings_path()).unwrap(), before, "{minutes}: nothing was written");
    }
    let names: Vec<String> = fs::read_dir(&sandbox.root).unwrap().map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned()).collect();
    assert_eq!(names, vec![SETTINGS_FILE.to_string()], "no temporary file is left");
    // A readable file's backup members are kept; a bad file is replaced whole.
    fs::write(sandbox.settings_path(), json!({"document_kind": SETTINGS_KIND, "version": 1, "autosave_minutes": 5, "backup_folder": "/invented/folder", "backup_keep": 10}).to_string()).unwrap();
    let AutosaveSettingsWriteOutcome::Saved { settings } = set_settings_at(sandbox.data(), 2, &UnmeteredReadMemory) else { panic!() };
    assert_eq!(settings, AutosaveSettings { autosave_minutes: 2, backup_folder: Some("/invented/folder".to_string()), backup_keep: Some(10) });
    fs::write(sandbox.settings_path(), b"not json").unwrap();
    let AutosaveSettingsWriteOutcome::Saved { settings } = set_settings_at(sandbox.data(), 15, &UnmeteredReadMemory) else { panic!() };
    assert_eq!(settings, AutosaveSettings { autosave_minutes: 15, backup_folder: None, backup_keep: None });
    assert_eq!(fs::read_to_string(sandbox.settings_path()).unwrap(), settings_text(&settings));
}
