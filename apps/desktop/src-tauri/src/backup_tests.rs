//! Invented inputs only. Every filesystem mutation is within an owned sandbox.
//! These tests require the lane's guarded window before execution; they do not
//! establish qualification for other volumes or prevent later external edits.
use super::*;
use crate::autosave_commands::{
    read_settings, read_settings_record, set_settings_at, settings_text, SETTINGS_FILE,
};
use crate::stored_read::{ReadReservation, UnmeteredReadMemory};
use open_pipe_stress_operation_applier::canonical_json;
use serde_json::json;
use std::os::unix::fs::FileExt;
use std::{
    cell::{Cell, RefCell},
    fs,
};

const DEMO: &str = include_str!("../../../../fixtures/product_preview/invented_demo_model.json");
const PROJECT: &str = "project:invented-demo-loop-01";
// 2026-09-21 13:46:40 UTC, independently converted from the calendar date.
const T0: u64 = 1_789_998_400;
struct Sandbox(PathBuf);
impl Sandbox {
    fn new(label: &str) -> Self {
        let root = std::env::temp_dir().join(format!(
            "swbpipe-backup-{label}-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(root.join("data")).unwrap();
        fs::create_dir(root.join("folder")).unwrap();
        Self(root)
    }
    fn data(&self) -> PathBuf {
        self.0.join("data")
    }
    fn folder(&self) -> PathBuf {
        self.0.join("folder")
    }
    fn store(&self) -> PathBuf {
        self.data().join(crate::PROJECT_STORE_FILE)
    }
    fn settings(&self) -> PathBuf {
        self.data().join(SETTINGS_FILE)
    }
    fn enable(&self, keep: Option<u32>) {
        fs::write(
            self.settings(),
            settings_text(&SettingsRecord {
                autosave_minutes: 5,
                backup_folder: Some(self.folder().to_str().unwrap().into()),
                backup_keep: keep,
                backup_local_private_intent: true,
            }),
        )
        .unwrap();
    }
    fn save(&self, model: &Value) -> String {
        let hash = crate::computed_model_hash(model, PROJECT);
        crate::create_project_in_store(
            self.store(),
            model.clone(),
            None,
            None,
            None,
            None,
            None,
            Some(hash.clone()),
            None,
            None,
        )
        .unwrap();
        hash["value"].as_str().unwrap().into()
    }
    fn raw(&self, text: &str, hash: &str) {
        let (connection, _) = crate::open_project_store(&self.store()).unwrap();
        connection.execute("INSERT OR REPLACE INTO local_projects(project_id, project_name, model_json, model_hash_json, created_at_unix, updated_at_unix) VALUES (?1, 'Invented', ?2, ?3, 1, 1)", params![PROJECT,text,hash]).unwrap();
    }
    fn entries(&self) -> Vec<String> {
        let mut entries: Vec<_> = fs::read_dir(self.folder())
            .unwrap()
            .map(|entry| entry.unwrap().file_name().to_str().unwrap().to_string())
            .collect();
        entries.sort();
        entries
    }
}
impl Drop for Sandbox {
    fn drop(&mut self) {
        // Test teardown only. This root is created by this Sandbox and cannot
        // be a real project folder; product backup code has no removal calls.
        let _ = fs::set_permissions(self.folder(), fs::Permissions::from_mode(0o700));
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn demo() -> Value {
    serde_json::from_str(DEMO).unwrap()
}
fn hash(text: &str) -> String {
    format!("sha256:{}", sha256_hex(text))
}
fn claim(hash: &str) -> String {
    json!({"value":hash}).to_string()
}
fn request(hash: &str) -> WriteRequest {
    WriteRequest {
        project_id: PROJECT.into(),
        expected_model_hash: hash.into(),
    }
}
fn name(model: &Value, time: u64) -> String {
    format!(
        "{}.{}.{}.{}{BACKUP_SUFFIX}",
        backup_stem(model),
        backup_slot(PROJECT),
        utc_stamp(time).unwrap(),
        &sha256_hex(&canonical_json(model))[..12]
    )
}
fn codes(outcome: &BackupWriteOutcome) -> Vec<String> {
    match outcome {
        BackupWriteOutcome::Written { warnings, .. }
        | BackupWriteOutcome::Unchanged { warnings, .. } => {
            warnings.iter().map(|d| d.code.clone()).collect()
        }
        BackupWriteOutcome::Refused { diagnostics }
        | BackupWriteOutcome::Unverified { diagnostics, .. } => {
            diagnostics.iter().map(|d| d.code.clone()).collect()
        }
        _ => vec![],
    }
}
fn write(s: &Sandbox, hash: &str, io: &mut impl BackupIo) -> BackupWriteOutcome {
    write_at_with_io(
        Ok(s.data()),
        &request(hash),
        Some(T0),
        &UnmeteredReadMemory,
        io,
    )
    .into_unreserved()
    .unwrap()
}
// Folder sync is injected for fault tests. A separate test exercises the real
// supported-volume metadata, FD-source clone, file sync and bounded folder sync.
struct Durable;
impl BackupIo for Durable {
    fn sync_directory(&mut self, _dir: &BackupDir) -> bool {
        true
    }
}
fn configure(
    s: &Sandbox,
    enabled: bool,
    keep: Option<u32>,
    intent: bool,
    chosen: Option<PathBuf>,
) -> BackupConfigureOutcome {
    configure_with_capability(
        Ok(s.data()),
        &ConfigureRequest {
            enabled,
            backup_keep: keep,
            explicit_local_private_intent: intent,
        },
        || Ok(chosen),
        &UnmeteredReadMemory,
        |_| Ok(()),
    )
}
fn record(s: &Sandbox) -> SettingsRecord {
    read_settings_record(&s.data(), &UnmeteredReadMemory)
        .get()
        .record
        .clone()
}

#[test]
fn names_and_time_are_strict_and_short_slot_is_only_a_filter() {
    assert_eq!(utc_stamp(0).as_deref(), Some("19700101T000000Z"));
    assert_eq!(utc_stamp(T0).as_deref(), Some("20260921T134640Z"));
    let valid = name(&demo(), T0);
    assert_eq!(BackupName::parse(&valid).unwrap().file_name(), valid);
    for invalid in [
        "../x.json",
        ".swbpipe-backup-abc.tmp",
        "x.0123ABCD.20260921T134640Z.0123456789ab.swbpipe-backup.json",
    ] {
        assert!(BackupName::parse(invalid).is_none());
    }
}
#[cfg(target_os = "macos")]
#[test]
fn volume_capability_requires_validity_and_support_separately() {
    let bit = libc::VOL_CAP_INT_CLONE;
    assert!(
        clone_capability(0, bit).is_err(),
        "set but invalid is unknown"
    );
    assert!(
        clone_capability(bit, 0).is_err(),
        "valid but unset is unsupported"
    );
    assert!(clone_capability(bit, bit).is_ok());
}
#[test]
fn enable_preflights_metadata_without_probe_files_and_preserves_settings_on_refusal() {
    let s = Sandbox::new("capability-refusal");
    s.enable(None);
    let before = fs::read(s.settings()).unwrap();
    for kind in [io::ErrorKind::Unsupported, io::ErrorKind::Other] {
        let result = configure_with_capability(
            Ok(s.data()),
            &ConfigureRequest {
                enabled: true,
                backup_keep: None,
                explicit_local_private_intent: true,
            },
            || Ok(Some(s.folder())),
            &UnmeteredReadMemory,
            |_| {
                Err(io::Error::new(
                    kind,
                    "invented unsupported/unknown metadata",
                ))
            },
        );
        assert!(matches!(result, BackupConfigureOutcome::Refused { .. }));
        assert_eq!(fs::read(s.settings()).unwrap(), before);
        assert!(
            s.entries().is_empty(),
            "metadata preflight creates no probe file"
        );
    }
}
#[test]
fn finite_retention_and_missing_intent_refuse_before_chooser_and_cancel_changes_nothing() {
    let s = Sandbox::new("configure");
    let asked = Cell::new(false);
    for (keep, intent) in [(Some(3), true), (None, false)] {
        let result = configure_with_capability(
            Ok(s.data()),
            &ConfigureRequest {
                enabled: true,
                backup_keep: keep,
                explicit_local_private_intent: intent,
            },
            || {
                asked.set(true);
                Ok(Some(s.folder()))
            },
            &UnmeteredReadMemory,
            |_| Ok(()),
        );
        assert!(matches!(result, BackupConfigureOutcome::Refused { .. }));
        assert!(!asked.get());
    }
    assert_eq!(
        configure(&s, true, None, true, None),
        BackupConfigureOutcome::Cancelled
    );
    assert!(!s.settings().exists());
    assert!(s.entries().is_empty());
}
#[test]
fn finite_legacy_setting_is_kept_but_never_used_and_disable_still_works() {
    let s = Sandbox::new("finite");
    s.enable(Some(3));
    let before = fs::read(s.settings()).unwrap();
    let outcome = write(&s, &format!("sha256:{}", "a".repeat(64)), &mut Durable);
    assert_eq!(codes(&outcome), [ROTATION_UNAVAILABLE]);
    assert!(s.entries().is_empty());
    assert_eq!(fs::read(s.settings()).unwrap(), before);
    assert_eq!(
        read_settings(&s.data(), &UnmeteredReadMemory)
            .get()
            .status
            .as_ref()
            .unwrap()
            .code,
        ROTATION_UNAVAILABLE
    );
    assert!(matches!(
        set_settings_at(Ok(s.data()), 10, &UnmeteredReadMemory),
        AutosaveSettingsWriteOutcome::Saved { .. }
    ));
    assert_eq!(record(&s).backup_keep, Some(3));
    fs::write(s.folder().join("invented-existing.json"), b"kept").unwrap();
    assert!(matches!(
        configure(&s, false, None, false, None),
        BackupConfigureOutcome::Saved { .. }
    ));
    assert_eq!(
        record(&s).backup_keep,
        Some(3),
        "disable does not silently rewrite retention"
    );
    assert_eq!(
        fs::read(s.folder().join("invented-existing.json")).unwrap(),
        b"kept"
    );
    assert!(matches!(
        configure(&s, true, None, true, Some(s.folder())),
        BackupConfigureOutcome::Saved { .. }
    ));
    assert_eq!(record(&s).backup_keep, None, "All is explicitly chosen");
    assert_eq!(record(&s).autosave_minutes, 10);
}
#[test]
fn off_and_legacy_folder_without_intent_never_open_the_store() {
    let s = Sandbox::new("off");
    let expected = format!("sha256:{}", "a".repeat(64));
    assert_eq!(
        write(&s, &expected, &mut Durable),
        BackupWriteOutcome::Disabled
    );
    fs::write(s.settings(),json!({"document_kind":"swbpipe.desktop.autosave_settings","version":1,"autosave_minutes":5,"backup_folder":s.folder(),"backup_keep":null}).to_string()).unwrap();
    let before = fs::read(s.settings()).unwrap();
    assert_eq!(
        write(&s, &expected, &mut Durable),
        BackupWriteOutcome::Disabled
    );
    assert!(!s.store().exists());
    assert!(s.entries().is_empty());
    assert_eq!(fs::read(s.settings()).unwrap(), before);
}
#[test]
fn settings_read_refusal_is_not_disabled_and_interval_change_preserves_unread_settings() {
    struct NoSettings;
    impl ReadMemory for NoSettings {
        fn reserve(&self, what: &str, bytes: u64) -> Result<ReadReservation, String> {
            if what == "the autosave settings file" {
                Err(format!("{STORE_READ_MEMORY_UNAVAILABLE}: invented refusal"))
            } else {
                UnmeteredReadMemory.reserve(what, bytes)
            }
        }
    }
    let s = Sandbox::new("settings-memory");
    s.enable(None);
    let before = fs::read(s.settings()).unwrap();
    let outcome = write_at_with_io(
        Ok(s.data()),
        &request(&format!("sha256:{}", "a".repeat(64))),
        Some(T0),
        &NoSettings,
        &mut Durable,
    )
    .into_unreserved()
    .unwrap();
    assert_eq!(codes(&outcome), [STORE_READ_MEMORY_UNAVAILABLE]);
    assert!(matches!(
        set_settings_at(Ok(s.data()), 1, &NoSettings),
        AutosaveSettingsWriteOutcome::Refused { .. }
    ));
    assert_eq!(fs::read(s.settings()).unwrap(), before);
    assert!(s.entries().is_empty());
}
#[test]
fn completion_refusal_precedes_all_effects_and_response_serialization_has_no_later_admission() {
    struct Memory {
        refuse: bool,
        requests: RefCell<Vec<String>>,
    }
    impl ReadMemory for Memory {
        fn reserve(&self, what: &str, bytes: u64) -> Result<ReadReservation, String> {
            self.requests.borrow_mut().push(what.into());
            if self.refuse && what == "backup completion diagnostics and serialization" {
                Err(format!("{STORE_READ_MEMORY_UNAVAILABLE}: invented"))
            } else {
                UnmeteredReadMemory.reserve(what, bytes)
            }
        }
    }
    let s = Sandbox::new("completion");
    s.enable(None);
    let memory = Memory {
        refuse: true,
        requests: RefCell::new(vec![]),
    };
    let result = write_at(
        Ok(s.data()),
        &request(&format!("sha256:{}", "a".repeat(64))),
        Some(T0),
        &memory,
    );
    assert_eq!(codes(result.get()), [STORE_READ_MEMORY_UNAVAILABLE]);
    assert!(!s.store().exists());
    assert!(s.entries().is_empty());
    let count = memory.requests.borrow().len();
    assert!(response(result).is_ok());
    assert_eq!(
        memory.requests.borrow().len(),
        count,
        "serialization admits nothing after effects"
    );
}
#[test]
fn a_queued_backup_rechecks_settings_after_disable_linearizes() {
    let s = Sandbox::new("disable-queued");
    s.enable(None);
    let data = s.data();
    BEFORE_BACKUP_LOCK.with(|hook| {
        *hook.borrow_mut() = Some(Box::new(move || {
            let result = configure_with_capability(
                Ok(data),
                &ConfigureRequest {
                    enabled: false,
                    backup_keep: None,
                    explicit_local_private_intent: false,
                },
                || panic!("off has no chooser"),
                &UnmeteredReadMemory,
                |_| panic!("off has no probe"),
            );
            assert!(matches!(result, BackupConfigureOutcome::Saved { .. }));
        }))
    });
    assert_eq!(
        write(&s, &format!("sha256:{}", "a".repeat(64)), &mut Durable),
        BackupWriteOutcome::Disabled
    );
    assert!(!s.store().exists());
    assert!(s.entries().is_empty());
}
#[test]
fn stored_hash_identity_duplicate_and_protected_checks_precede_folder_writes() {
    let s = Sandbox::new("stored-checks");
    s.enable(None);
    let text = canonical_json(&demo());
    let expected = hash(&text);
    s.raw(&text, &claim(&expected));
    let wrong = format!("sha256:{}", "0".repeat(64));
    assert_eq!(codes(&write(&s, &wrong, &mut Durable)), [STALE]);
    s.raw(&text, &claim(&wrong));
    assert_eq!(
        codes(&write(&s, &wrong, &mut Durable)),
        [STORED_HASH_MISMATCH]
    );
    let duplicate = text.replacen('{', "{\"duplicate\":0,\"duplicate\":1,", 1);
    s.raw(&duplicate, &claim(&expected));
    assert_eq!(
        codes(&write(&s, &expected, &mut Durable)),
        [STORED_DUPLICATE_MEMBER]
    );
    let mut other = demo();
    other["project"]["id"] = json!("project:invented-other");
    let other = canonical_json(&other);
    let other_hash = hash(&other);
    s.raw(&other, &claim(&other_hash));
    assert_eq!(
        codes(&write(&s, &other_hash, &mut Durable)),
        [PROJECT_MISMATCH]
    );
    let mut blocked = demo();
    blocked["materials"][0]["provenance_record"] = json!({
        "source_name": "invented backup screening test",
        "review_status": "quarantined"
    });
    let blocked = canonical_json(&blocked);
    let blocked_hash = hash(&blocked);
    s.raw(&blocked, &claim(&blocked_hash));
    let blocked_outcome = write(&s, &blocked_hash, &mut Durable);
    assert!(
        codes(&blocked_outcome).contains(&"PROTECTED_CONTENT_BLOCKED".into()),
        "{blocked_outcome:?}"
    );
    assert!(s.entries().is_empty());
}

#[cfg(target_os = "macos")]
mod clone_route {
    use super::*;
    #[test]
    fn actual_fd_clone_of_canonical_stored_model_is_durable_and_opens() {
        let s = Sandbox::new("actual-clone");
        let model = demo();
        let text = canonical_json(&model);
        let expected = hash(&text);
        s.raw(
            &serde_json::to_string_pretty(&model).unwrap(),
            &claim(&expected),
        );
        s.enable(None);
        let store = fs::read(s.store()).unwrap();
        let result = write(&s, &expected, &mut SystemBackupIo);
        let BackupWriteOutcome::Written {
            file_name,
            durability,
            ..
        } = &result
        else {
            panic!("{result:?}")
        };
        assert_eq!(durability, "durable");
        assert_eq!(
            fs::read_to_string(s.folder().join(file_name)).unwrap(),
            text
        );
        assert_eq!(
            fs::metadata(s.folder().join(file_name)).unwrap().mode() & 0o777,
            0o600
        );
        assert_eq!(
            s.entries().len(),
            2,
            "source temporary copy is deliberately retained"
        );
        assert_eq!(fs::read(s.store()).unwrap(), store);
        let opened = crate::model_document_file::check_model_document(
            file_name.clone(),
            text.len() as u64,
            serde_json::from_str(&text).unwrap(),
        );
        assert!(matches!(
            opened,
            crate::model_document_file::ModelDocumentOpenOutcome::Opened { .. }
        ));
        assert!(matches!(
            write(&s, &expected, &mut Durable),
            BackupWriteOutcome::Unchanged { .. }
        ));
        assert_eq!(s.entries().len(), 2);
    }
    #[test]
    fn source_name_swap_cannot_publish_the_substituted_sibling() {
        struct Swap(PathBuf);
        impl BackupIo for Swap {
            fn publish(&mut self, dir: &BackupDir, source: &File, name: &str) -> io::Result<()> {
                let temp = fs::read_dir(&self.0)?
                    .map(|e| e.unwrap().path())
                    .find(|p| {
                        p.file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .starts_with(TEMP_PREFIX)
                    })
                    .unwrap();
                fs::rename(&temp, self.0.join("invented-held-source"))?;
                fs::write(&temp, b"invented malicious sibling")?;
                dir.publish_new(source, name)
            }
            fn sync_directory(&mut self, _: &BackupDir) -> bool {
                true
            }
        }
        let s = Sandbox::new("source-swap");
        let expected = s.save(&demo());
        s.enable(None);
        let result = write(&s, &expected, &mut Swap(s.folder()));
        let BackupWriteOutcome::Written { file_name, .. } = result else {
            panic!("{result:?}")
        };
        assert_eq!(
            fs::read_to_string(s.folder().join(file_name)).unwrap(),
            canonical_json(&demo())
        );
        let temp = s
            .entries()
            .into_iter()
            .find(|n| n.starts_with(TEMP_PREFIX))
            .unwrap();
        assert_eq!(
            fs::read(s.folder().join(temp)).unwrap(),
            b"invented malicious sibling"
        );
        assert!(s.folder().join("invented-held-source").exists());
    }
    #[test]
    fn existing_name_is_never_replaced_and_unsupported_clone_has_no_fallback() {
        let s = Sandbox::new("collision");
        let expected = s.save(&demo());
        s.enable(None);
        let existing = name(&demo(), T0);
        fs::write(s.folder().join(&existing), b"invented collision").unwrap();
        assert_eq!(codes(&write(&s, &expected, &mut Durable)), [NAME_IN_USE]);
        assert_eq!(
            fs::read(s.folder().join(existing)).unwrap(),
            b"invented collision"
        );
        struct Unsupported;
        impl BackupIo for Unsupported {
            fn publish(&mut self, _: &BackupDir, _: &File, _: &str) -> io::Result<()> {
                Err(io::Error::from_raw_os_error(libc::ENOTSUP))
            }
        }
        let result = write(&s, &expected, &mut Unsupported);
        assert_eq!(codes(&result), [PUBLISH_UNSUPPORTED]);
        assert!(
            s.entries()
                .iter()
                .filter(|n| n.starts_with(TEMP_PREFIX))
                .count()
                >= 2,
            "no cleanup deletes temp copies"
        );
    }
    #[test]
    fn failures_keep_partial_temporary_final_and_unrelated_entries() {
        struct Fault(u8);
        impl BackupIo for Fault {
            fn write_all(&mut self, file: &mut File, bytes: &[u8]) -> io::Result<()> {
                if self.0 == 0 {
                    file.write_all(&bytes[..32])?;
                    Err(io::Error::from_raw_os_error(libc::ENOSPC))
                } else {
                    file.write_all(bytes)
                }
            }
            fn sync_source(&mut self, file: &File) -> io::Result<()> {
                if self.0 == 1 {
                    Err(io::Error::other("invented source sync failure"))
                } else {
                    file.sync_all()
                }
            }
            fn sync_final(&mut self, file: &File) -> io::Result<()> {
                if self.0 == 2 {
                    Err(io::Error::other("invented final sync failure"))
                } else {
                    file.sync_all()
                }
            }
            fn sync_directory(&mut self, _: &BackupDir) -> bool {
                self.0 != 3
            }
        }
        for stage in 0..4 {
            let s = Sandbox::new("fault");
            let expected = s.save(&demo());
            s.enable(None);
            fs::write(s.folder().join("invented-old-copy"), b"kept old").unwrap();
            let result = write(&s, &expected, &mut Fault(stage));
            if stage < 2 {
                assert!(matches!(result, BackupWriteOutcome::Refused { .. }));
            } else {
                assert!(matches!(
                    result,
                    BackupWriteOutcome::Unverified {
                        creation: "created",
                        ..
                    }
                ));
            }
            assert_eq!(
                fs::read(s.folder().join("invented-old-copy")).unwrap(),
                b"kept old"
            );
            assert!(s.entries().iter().any(|n| n.starts_with(TEMP_PREFIX)));
            assert_eq!(
                s.entries().iter().any(|n| n.ends_with(BACKUP_SUFFIX)),
                stage >= 2
            );
        }
    }
    #[test]
    fn source_and_final_mutation_do_not_become_verified_success() {
        struct SourceMutation;
        impl BackupIo for SourceMutation {
            fn publish(&mut self, dir: &BackupDir, source: &File, name: &str) -> io::Result<()> {
                source.write_at(b"!", 0)?;
                dir.publish_new(source, name)
            }
        }
        let s = Sandbox::new("source-mutate");
        let expected = s.save(&demo());
        s.enable(None);
        let result = write(&s, &expected, &mut SourceMutation);
        assert!(matches!(result, BackupWriteOutcome::Unverified { .. }));
        assert!(codes(&result).contains(&READBACK_MISMATCH.into()));
        assert_eq!(s.entries().len(), 2);
        let s = Sandbox::new("final-mutate");
        let expected = s.save(&demo());
        s.enable(None);
        let final_path = s.folder().join(name(&demo(), T0));
        AFTER_FINAL_READ.with(|hook| {
            *hook.borrow_mut() = Some(Box::new(move || {
                let file = fs::OpenOptions::new().write(true).open(final_path).unwrap();
                file.write_at(b"!", 0).unwrap();
            }))
        });
        let result = write(&s, &expected, &mut Durable);
        assert!(matches!(result, BackupWriteOutcome::Unverified { .. }));
        assert_eq!(s.entries().len(), 2);
    }
    #[test]
    fn same_content_replacement_or_rewrite_during_sync_is_not_durable_success() {
        // Interference during the folder sync: (a) the synced file is renamed
        // away and an unsynced mode-0600 file with identical bytes takes its
        // name; (b) the synced file is rewritten in place with identical bytes.
        struct Interfere(PathBuf, bool);
        impl BackupIo for Interfere {
            fn sync_directory(&mut self, _: &BackupDir) -> bool {
                let bytes = fs::read(&self.0).unwrap();
                if self.1 {
                    fs::rename(&self.0, self.0.with_file_name("invented-synced-copy")).unwrap();
                    fs::write(&self.0, &bytes).unwrap();
                    fs::set_permissions(&self.0, fs::Permissions::from_mode(0o600)).unwrap();
                } else {
                    std::thread::sleep(std::time::Duration::from_millis(20));
                    fs::OpenOptions::new()
                        .write(true)
                        .open(&self.0)
                        .unwrap()
                        .write_all_at(&bytes, 0)
                        .unwrap();
                }
                true
            }
        }
        for replace in [true, false] {
            let s = Sandbox::new("sync-interference");
            let expected = s.save(&demo());
            s.enable(None);
            let final_path = s.folder().join(name(&demo(), T0));
            let result = write(&s, &expected, &mut Interfere(final_path.clone(), replace));
            assert!(
                matches!(
                    result,
                    BackupWriteOutcome::Unverified {
                        creation: "created",
                        ..
                    }
                ),
                "{result:?}"
            );
            assert_eq!(codes(&result), [READBACK_FAILED]);
            assert_eq!(
                fs::read_to_string(&final_path).unwrap(),
                canonical_json(&demo())
            );
            assert_eq!(
                s.folder().join("invented-synced-copy").exists(),
                replace,
                "the synced copy is kept, never removed"
            );
            assert!(s.entries().iter().any(|n| n.starts_with(TEMP_PREFIX)));
        }
    }
    #[test]
    fn final_name_swap_or_symlink_during_readback_keeps_all_entries_and_targets() {
        let s = Sandbox::new("final-swap");
        let expected = s.save(&demo());
        s.enable(None);
        let target = s.0.join("invented-outside");
        fs::write(&target, b"invented outside bytes").unwrap();
        let final_path = s.folder().join(name(&demo(), T0));
        let outside = target.clone();
        BEFORE_FINAL_OPEN.with(|hook| {
            *hook.borrow_mut() = Some(Box::new(move || {
                fs::rename(
                    &final_path,
                    final_path.with_file_name("invented-created-copy"),
                )
                .unwrap();
                std::os::unix::fs::symlink(outside, &final_path).unwrap();
            }))
        });
        let result = write(&s, &expected, &mut Durable);
        assert!(matches!(result, BackupWriteOutcome::Unverified { .. }));
        assert_eq!(fs::read(target).unwrap(), b"invented outside bytes");
        assert!(fs::symlink_metadata(s.folder().join(name(&demo(), T0)))
            .unwrap()
            .file_type()
            .is_symlink());
        assert!(s.folder().join("invented-created-copy").exists());
    }
}

#[test]
fn command_admission_preserves_small_control_and_identity_only_arguments() {
    use crate::command_ingress::test_support::{headers, registry};
    use crate::memory_pool::test_host::ScriptedHost;
    let registry = registry(ScriptedHost::normal());
    let header = headers(Some(crate::command_ingress::INGRESS_ARGS_JSON_V1));
    for (command, max, args) in [
        (
            WRITE_COMMAND,
            WRITE_COMMAND_MAX_ARGS_BYTES,
            json!({"projectId":PROJECT,"expectedModelHash":format!("sha256:{}","a".repeat(64))}),
        ),
        (
            CONFIGURE_COMMAND,
            CONFIGURE_COMMAND_MAX_ARGS_BYTES,
            json!({"enabled":true,"backupKeep":null,"explicitLocalPrivateIntent":true}),
        ),
    ] {
        let body = tauri::ipc::InvokeBody::Raw(serde_json::to_vec(&args).unwrap());
        let (mut admitted, lease) = admit_body(&registry, &body, &header, command).unwrap();
        if command == WRITE_COMMAND {
            assert_eq!(write_request(&mut admitted).unwrap().project_id, PROJECT);
        } else {
            assert!(
                configure_request(&mut admitted)
                    .unwrap()
                    .explicit_local_private_intent
            );
        }
        drop(admitted);
        drop(lease);
        let oversized = tauri::ipc::InvokeBody::Raw(vec![b' '; max + 1]);
        assert!(admit_body(&registry, &oversized, &header, command).is_err());
    }
    assert_eq!(registry.pool_status().reserved_bytes, 0);
}

#[cfg(target_os = "macos")]
#[test]
fn full_backup_ingress_store_clone_and_response_fit_the_held_reservations() {
    use crate::command_ingress::test_support::{headers, registry};
    use crate::memory_pool::test_host::ScriptedHost;
    let s = Sandbox::new("measured-command");
    let expected = s.save(&demo());
    s.enable(None);
    let registry = registry(ScriptedHost::normal());
    let bytes =
        serde_json::to_vec(&json!({"projectId":PROJECT,"expectedModelHash":expected})).unwrap();
    let body_envelope = crate::ingress::allocation_envelope(bytes.len() as u64).unwrap();
    let body = tauri::ipc::InvokeBody::Raw(bytes);
    let header = headers(Some(crate::command_ingress::INGRESS_ARGS_JSON_V1));
    let ((written, sent, held, peak), measured) = crate::ingress::counting::measure(|| {
        let (mut args, lease) = admit_body(&registry, &body, &header, WRITE_COMMAND).unwrap();
        let request = write_request(&mut args).unwrap();
        drop(args);
        let memory = seeded_memory(&registry, &lease);
        let held = lease.bytes();
        assert_eq!(memory.held(), held);
        let outcome = write_at_with_io(Ok(s.data()), &request, Some(T0), &memory, &mut Durable);
        let written = matches!(outcome.get(), BackupWriteOutcome::Written { .. });
        let sent = response(outcome).is_ok();
        let peak = memory.peak_reserved();
        drop(memory);
        drop(lease);
        (written, sent, held, peak)
    });
    assert!(written && sent);
    assert!(peak > held);
    assert!(
        measured.peak_priced <= peak - body_envelope,
        "measured {} exceeds held {} less body {}",
        measured.peak_priced,
        peak,
        body_envelope
    );
    assert_eq!(registry.pool_status().reserved_bytes, 0);
}
