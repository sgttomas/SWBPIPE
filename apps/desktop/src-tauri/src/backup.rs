//! Backups of the last saved version (D5 slice 3; DEL-02-05 Design
//! `autosave-and-backup.md` §4, §5 and §6).
//!
//! Backups are off until the user turns them on with
//! `configure_project_backup`: the explicit choice to copy private project
//! data to a local folder (`explicitLocalPrivateIntent`), then the native
//! folder chooser (`blocking_pick_folder`, starting in the documents folder).
//! The folder and the intent are recorded together in the settings file
//! (`autosave_commands::SettingsRecord`) under the settings lock; no folder
//! ever comes from the webview, and a folder without the recorded intent is
//! never used. Turning backups off clears the folder and deletes nothing.
//!
//! After a landed Save Local Project the desktop calls `write_project_backup`
//! with the saved project's id and the full model hash the save returned. On
//! a worker of its own (the save's response is already sent), the backup:
//! 1. reads the settings: backups off is `disabled`, and the store is not opened;
//! 2. takes the app's backup lock (`swbpipe-backup.lock` in the app's local
//!    data directory, slice 1's `flock` lock), so one backup transaction runs
//!    at a time across threads, windows and processes;
//! 3. reads the stored project row from the store opened read-only and not
//!    migrated (`stored_model_document_export::open_store_read_only`), metered:
//!    the row's model-hash claim first, which must be the requested
//!    `expectedModelHash` (else `BACKUP-STALE`: the store holds another model
//!    now, and nothing is written); then the stored model text (compact JSON,
//!    not necessarily canonical), its strict parse (a duplicate member anywhere
//!    is refused), its RFC 8785 canonical text, whose SHA-256 must be the
//!    claim (`BACKUP-STORED-HASH-MISMATCH`), the open path's checks (kind,
//!    current version, validator, desktop shape) and the native disposition
//!    screening (`PROTECTED_CONTENT_BLOCKED`), each reservation made first;
//! 4. writes the canonical bytes, so the file's SHA-256 is the model hash and
//!    it opens with Open Model Document.
//!
//! Every file operation in the folder goes through one directory handle
//! opened `O_DIRECTORY | O_NOFOLLOW` (a symlinked folder is refused), one
//! path component at a time, never following a link: the new file is created
//! `O_CREAT | O_EXCL | O_NOFOLLOW` under a temporary name `.swbpipe-backup-*.tmp`,
//! written, fsynced and published by `renameatx_np(RENAME_EXCL)` (elsewhere
//! `linkat`), which never replaces an existing entry; a name already in use
//! is refused `BACKUP-NAME-IN-USE`, or is `unchanged` when it holds exactly
//! these bytes. It is read back through the handle (the published entry must
//! be the inode written, streamed through SHA-256 in 64 KiB blocks), and the
//! folder is synced (bounded, at most one sync worker at a time).
//!
//! The name is `<stem>.<slot8>.<UTC yyyymmddThhmmssZ>.<hash12>.swbpipe-backup.json`.
//! Its slot and hash are truncated, and anything in the folder can carry any
//! name, so a name is never authority for a write or a removal:
//! - `unchanged` needs the project's newest backup by name to stream through
//!   SHA-256 as the full model hash, which identifies exactly these bytes;
//! - rotation removes a file only once its content is verified as a backup of
//!   this project: a regular file within the open cap whose bytes hash with
//!   the name's prefix, are canonical text and hold a document whose project
//!   id is this project's full id. Files that do not verify, or cannot be
//!   checked, are kept, counted and never counted towards N. The new file is
//!   always kept. Removal is `unlinkat` of the entry by name, only while the
//!   name still names the inode verified.
//!
//! Rotation (keep the newest N, by the time in the names; N null is All,
//! which deletes nothing) runs only after the new file has read back and the
//! folder sync completed. A failed or timed-out sync, a read-back that could
//! not be made, or a name in use keeps every older file (`BACKUP-PRUNE-DEFERRED`).
//! The enumeration keeps at most [`MAX_CANDIDATES`] of the project's newest
//! names (reserved first) and stops after [`MAX_SCANNED_ENTRIES`] entries;
//! files beyond are kept and counted in a warning.
//!
//! Residual: the check-to-call window between an identity check and the
//! name-based `unlinkat` it guards cannot be closed on macOS (no unlink by
//! descriptor); its reach is that one entry, never a link's target. Content
//! written by another process after its verification is not detected. The
//! backup lock coordinates SWBPIPE's own writers. Nothing here approves,
//! accepts or issues anything: a backup is a recovery copy of the user's model.

use crate::autosave::{self, LOCK_WAIT, READBACK_BLOCK_BYTES};
use crate::autosave_commands::{
    data_directory, on_worker, update_settings_at, AutosaveSettings, AutosaveSettingsWriteOutcome, SettingsRecord, BACKUP_KEEP_CHOICES,
    RECORD_COMMAND_MAX_ARGS_BYTES, SETTINGS_COMMAND_MAX_ARGS_BYTES, SETTINGS_OUT_OF_RANGE,
};
use crate::command_ingress::{self, Args, ArgsDecoding, CommandError};
use crate::export_disposition_screen::{diagnostic, DispositionDiagnostic};
use crate::model_document_file::MAX_MODEL_DOCUMENT_BYTES;
use crate::model_document_save::{default_save_directory, open_path_checks, screen_model_document_dispositions, suggested_model_document_file_name};
use crate::solve_jobs::{SolveJobError, SolveJobRegistry};
use crate::stored_model_document_export::{bounded_diagnostics, open_store_read_only, stored_document_name, StoreLibraryReads};
use crate::stored_read::{self, is_jcs_text, loaded_text_bytes, parse_bytes, Metered, ReadMemory, STORE_READ_MEMORY_UNAVAILABLE};
use open_pipe_stress_operation_applier::sha256_hex;
use rusqlite::{params, OptionalExtension};
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, Read, Write};
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::DialogExt;

pub(crate) const CONFIGURE_COMMAND: &str = "configure_project_backup";
pub(crate) const WRITE_COMMAND: &str = "write_project_backup";
/// The configure command's argument budget (§6: scalars).
pub(crate) const CONFIGURE_COMMAND_MAX_ARGS_BYTES: usize = SETTINGS_COMMAND_MAX_ARGS_BYTES;
/// The write command's argument budget (§6: the history commands' 256 KiB).
pub(crate) const WRITE_COMMAND_MAX_ARGS_BYTES: usize = RECORD_COMMAND_MAX_ARGS_BYTES;

/// Every backup's name ends so.
pub(crate) const BACKUP_SUFFIX: &str = ".swbpipe-backup.json";
/// The temporary files this module owns; no listing or rotation parses them.
pub(crate) const TEMP_PREFIX: &str = ".swbpipe-backup-";
const TEMP_SUFFIX: &str = ".tmp";
/// The app's backup lock in its local data directory (`swbpipe-backup.lock`).
pub(crate) const BACKUP_LOCK_SLOT: &str = "swbpipe-backup";
/// The most of one project's backup names a transaction keeps (newest first).
pub(crate) const MAX_CANDIDATES: usize = 256;
/// One kept candidate: its heap slot, its name and its parsed parts, with margin.
pub(crate) const CANDIDATE_ENTRY_BYTES: u64 = 1024;
/// The most folder entries one enumeration reads; the rest are not considered.
pub(crate) const MAX_SCANNED_ENTRIES: u64 = 100_000;
/// The most warnings named; the rest are counted in one more.
const MAX_WARNINGS: usize = 16;
/// How long the folder sync after a write may take before durability is uncertain.
const DIRECTORY_SYNC_TIMEOUT: Duration = Duration::from_secs(2);
/// Backups hold private project data: readable by their owner only.
const FILE_MODE: libc::c_uint = 0o600;

const SLOT_DIGITS: usize = 8;
const HASH_DIGITS: usize = 12;
const STAMP_LEN: usize = 16;
const MAX_STEM_CHARS: usize = 96;

pub(crate) const INTENT_REQUIRED: &str = "BACKUP-INTENT-REQUIRED";
pub(crate) const FOLDER_INVALID: &str = "BACKUP-FOLDER-INVALID";
pub(crate) const FOLDER_UNAVAILABLE: &str = "BACKUP-FOLDER-UNAVAILABLE";
pub(crate) const HASH_INVALID: &str = "BACKUP-HASH-INVALID";
pub(crate) const PROJECT_NOT_FOUND: &str = "BACKUP-PROJECT-NOT-FOUND";
pub(crate) const PROJECT_MISMATCH: &str = "BACKUP-PROJECT-MISMATCH";
pub(crate) const STALE: &str = "BACKUP-STALE";
pub(crate) const STORED_HASH_MISSING: &str = "BACKUP-STORED-HASH-MISSING";
pub(crate) const STORED_HASH_MISMATCH: &str = "BACKUP-STORED-HASH-MISMATCH";
pub(crate) const STORED_NOT_JSON: &str = "BACKUP-STORED-NOT-JSON";
pub(crate) const STORED_DUPLICATE_MEMBER: &str = "BACKUP-STORED-DUPLICATE-MEMBER";
pub(crate) const STORE_UNAVAILABLE: &str = "BACKUP-STORE-UNAVAILABLE";
pub(crate) const TOO_LARGE: &str = "BACKUP-TOO-LARGE";
pub(crate) const VERSION_NOT_CURRENT: &str = "BACKUP-VERSION-NOT-CURRENT";
pub(crate) const CLOCK_INVALID: &str = "BACKUP-CLOCK-INVALID";
pub(crate) const TEMP_CREATE_FAILED: &str = "BACKUP-TEMP-CREATE-FAILED";
pub(crate) const WRITE_FAILED: &str = "BACKUP-WRITE-FAILED";
pub(crate) const FILE_SYNC_FAILED: &str = "BACKUP-FILE-SYNC-FAILED";
pub(crate) const PUBLISH_FAILED: &str = "BACKUP-PUBLISH-FAILED";
pub(crate) const PUBLISH_UNSUPPORTED: &str = "BACKUP-PUBLISH-UNSUPPORTED";
pub(crate) const NAME_IN_USE: &str = "BACKUP-NAME-IN-USE";
pub(crate) const READBACK_MISMATCH: &str = "BACKUP-READBACK-MISMATCH";
pub(crate) const READBACK_FAILED: &str = "BACKUP-READBACK-FAILED";
pub(crate) const PRUNE_DEFERRED: &str = "BACKUP-PRUNE-DEFERRED";
pub(crate) const PRUNE_FAILED: &str = "BACKUP-PRUNE-FAILED";
pub(crate) const ROTATION_UNVERIFIED: &str = "BACKUP-ROTATION-UNVERIFIED";
pub(crate) const ROTATION_PARTIAL: &str = "BACKUP-ROTATION-PARTIAL";

fn one(code: &str, message: impl Into<String>) -> Vec<DispositionDiagnostic> {
    vec![diagnostic(code, "", message)]
}

fn is_lower_hex(text: &str, digits: usize) -> bool {
    text.len() == digits && text.bytes().all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
}

fn hex_of(digest: impl AsRef<[u8]>) -> String {
    digest.as_ref().iter().map(|byte| format!("{byte:02x}")).collect()
}

// ---------------------------------------------------------------------------
// Names.
// ---------------------------------------------------------------------------

/// A backup's name as its parts: `<stem>.<slot>.<stamp>.<hash>.swbpipe-backup.json`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BackupName {
    pub(crate) stem: String,
    /// The first 8 hex digits of the SHA-256 of the project id.
    pub(crate) slot: String,
    /// The UTC time written, `yyyymmddThhmmssZ`.
    pub(crate) stamp: String,
    /// The first 12 hex digits of the model hash.
    pub(crate) hash: String,
}

fn is_stem(stem: &str) -> bool {
    !stem.is_empty() && stem.chars().count() <= MAX_STEM_CHARS && stem.chars().all(|character| character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-'))
}

fn is_stamp(stamp: &str) -> bool {
    let bytes = stamp.as_bytes();
    bytes.len() == STAMP_LEN
        && bytes[8] == b'T'
        && bytes[15] == b'Z'
        && bytes[..8].iter().chain(&bytes[9..15]).all(u8::is_ascii_digit)
}

impl BackupName {
    pub(crate) fn file_name(&self) -> String {
        format!("{}.{}.{}.{}{BACKUP_SUFFIX}", self.stem, self.slot, self.stamp, self.hash)
    }

    /// The parts of a backup's name, or `None` for any other name (a
    /// temporary file, a path with a separator, another file). The stem may
    /// hold dots, so the parts are taken from the right.
    pub(crate) fn parse(file_name: &str) -> Option<Self> {
        let rest = file_name.strip_suffix(BACKUP_SUFFIX)?;
        let mut parts = rest.rsplitn(4, '.');
        let (hash, stamp, slot, stem) = (parts.next()?, parts.next()?, parts.next()?, parts.next()?);
        if !is_lower_hex(hash, HASH_DIGITS) || !is_stamp(stamp) || !is_lower_hex(slot, SLOT_DIGITS) || !is_stem(stem) {
            return None;
        }
        Some(Self { stem: stem.to_string(), slot: slot.to_string(), stamp: stamp.to_string(), hash: hash.to_string() })
    }

    /// The order of a project's backups: by the time in the name, then by name.
    fn key(&self) -> (String, String) {
        (self.stamp.clone(), self.file_name())
    }
}

/// The name's slot of a project id: the first 8 hex digits of the autosave slot.
pub(crate) fn backup_slot(project_id: &str) -> String {
    let mut slot = autosave::autosave_slot(project_id);
    slot.truncate(SLOT_DIGITS);
    slot
}

/// Save Model Document's suggested stem for `document` (its file name without `.json`).
pub(crate) fn backup_stem(document: &Value) -> String {
    let name = suggested_model_document_file_name(document);
    name.strip_suffix(".json").unwrap_or(&name).to_string()
}

/// The days since 1970-01-01 as a proleptic Gregorian date (Hinnant's
/// `civil_from_days`).
fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = (if z >= 0 { z } else { z - 146_096 }) / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let month = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32;
    let year = yoe as i64 + era * 400 + i64::from(month <= 2);
    (year, month, day)
}

/// `yyyymmddThhmmssZ` of a UTC time in whole seconds since the epoch, or
/// `None` beyond year 9999.
pub(crate) fn utc_stamp(unix_seconds: u64) -> Option<String> {
    let days = i64::try_from(unix_seconds / 86_400).ok()?;
    let seconds = unix_seconds % 86_400;
    let (year, month, day) = civil_from_days(days);
    if year > 9999 {
        return None;
    }
    Some(format!("{year:04}{month:02}{day:02}T{:02}{:02}{:02}Z", seconds / 3600, seconds % 3600 / 60, seconds % 60))
}

fn now_unix_seconds() -> Option<u64> {
    SystemTime::now().duration_since(UNIX_EPOCH).ok().map(|elapsed| elapsed.as_secs())
}

// ---------------------------------------------------------------------------
// The folder's handle.
// ---------------------------------------------------------------------------

const DIR_FLAGS: libc::c_int = libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_NONBLOCK | libc::O_CLOEXEC;
const FILE_READ_FLAGS: libc::c_int = libc::O_RDONLY | libc::O_NOFOLLOW | libc::O_NONBLOCK | libc::O_CLOEXEC;
const FILE_CREATE_FLAGS: libc::c_int = libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL | libc::O_NOFOLLOW | libc::O_CLOEXEC;

fn check(result: libc::c_int) -> io::Result<libc::c_int> {
    if result < 0 { Err(io::Error::last_os_error()) } else { Ok(result) }
}

fn c_name(name: &str) -> io::Result<std::ffi::CString> {
    if name.is_empty() || name.contains('/') || name == "." || name == ".." {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "not one path component"));
    }
    Ok(std::ffi::CString::new(name)?)
}

fn is_regular(stat: &libc::stat) -> bool {
    (stat.st_mode as u32 & libc::S_IFMT as u32) == libc::S_IFREG as u32
}

/// A file's identity: device and inode.
pub(crate) type EntryId = (u64, u64);

/// A regular file as inspected (`fstatat`, no link followed).
#[derive(Debug, Clone, Copy)]
struct Inspected {
    id: EntryId,
    len: u64,
}

/// Why a contained open did not yield the inspected regular file.
#[derive(Debug)]
enum Access {
    NotFound,
    NotRegular,
    TooLarge(u64),
    Changed(&'static str),
    Io(io::Error),
}

impl From<io::Error> for Access {
    fn from(error: io::Error) -> Self {
        if error.kind() == io::ErrorKind::NotFound { Access::NotFound } else { Access::Io(error) }
    }
}

impl std::fmt::Display for Access {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Access::NotFound => formatter.write_str("it is no longer there"),
            Access::NotRegular => formatter.write_str("it is not a regular file"),
            Access::TooLarge(len) => formatter.write_fmt(format_args!("it is {len} bytes, above the {MAX_MODEL_DOCUMENT_BYTES}-byte open cap")),
            Access::Changed(what) => formatter.write_str(what),
            Access::Io(error) => formatter.write_fmt(format_args!("it could not be read: {error}")),
        }
    }
}

#[cfg(test)]
thread_local! {
    /// Runs once between a file's inspection and its open (the swap tests).
    pub(crate) static BEFORE_OPEN: std::cell::RefCell<Option<Box<dyn FnOnce()>>> = std::cell::RefCell::new(None);
    /// Runs once between a rotation's verification of a file and its removal.
    pub(crate) static BEFORE_UNLINK: std::cell::RefCell<Option<Box<dyn FnOnce()>>> = std::cell::RefCell::new(None);
}

/// The backup folder, opened `O_DIRECTORY | O_NOFOLLOW`. Every name below is
/// one component resolved relative to this handle, and no link is followed.
#[derive(Debug)]
pub(crate) struct BackupDir(OwnedFd);

impl BackupDir {
    /// Opens the folder; its last component must not be a symlink.
    pub(crate) fn open(path: &Path) -> io::Result<Self> {
        use std::os::unix::ffi::OsStrExt;
        let path = std::ffi::CString::new(path.as_os_str().as_bytes())?;
        // SAFETY: a NUL-terminated path; the descriptor is owned on success.
        let fd = check(unsafe { libc::open(path.as_ptr(), DIR_FLAGS) })?;
        Ok(Self(unsafe { OwnedFd::from_raw_fd(fd) }))
    }

    fn fd(&self) -> libc::c_int {
        self.0.as_raw_fd()
    }

    fn stat_child(&self, name: &str) -> io::Result<libc::stat> {
        let name = c_name(name)?;
        // SAFETY: `stat` is plain data that `fstatat` fills; AT_SYMLINK_NOFOLLOW
        // reports a link itself.
        let mut stat: libc::stat = unsafe { std::mem::zeroed() };
        check(unsafe { libc::fstatat(self.fd(), name.as_ptr(), &mut stat, libc::AT_SYMLINK_NOFOLLOW) })?;
        Ok(stat)
    }

    /// The entry `name`: a regular file within `bound` bytes.
    fn inspect(&self, name: &str, bound: u64) -> Result<Inspected, Access> {
        let stat = self.stat_child(name)?;
        if !is_regular(&stat) {
            return Err(Access::NotRegular);
        }
        let len = stat.st_size.max(0) as u64;
        if len > bound {
            return Err(Access::TooLarge(len));
        }
        Ok(Inspected { id: (stat.st_dev as u64, stat.st_ino as u64), len })
    }

    /// Opens `name` as inspected: `O_NOFOLLOW | O_NONBLOCK` (a link is
    /// refused, a FIFO never blocks), and `fstat` on the descriptor must show
    /// a regular file with the inspected identity and size.
    fn open_inspected(&self, name: &str, inspected: Inspected) -> Result<File, Access> {
        #[cfg(test)]
        BEFORE_OPEN.with(|hook| {
            let hook = hook.borrow_mut().take();
            if let Some(hook) = hook {
                hook()
            }
        });
        let c = c_name(name)?;
        // SAFETY: a valid directory descriptor and a NUL-terminated name; the
        // descriptor is owned by the File.
        let fd = check(unsafe { libc::openat(self.fd(), c.as_ptr(), FILE_READ_FLAGS) })?;
        let file = unsafe { File::from_raw_fd(fd) };
        let metadata = file.metadata()?;
        if !metadata.is_file() {
            return Err(Access::NotRegular);
        }
        if (metadata.dev(), metadata.ino()) != inspected.id {
            return Err(Access::Changed("it was replaced after it was inspected"));
        }
        if metadata.len() != inspected.len {
            return Err(Access::Changed("it changed size after it was inspected"));
        }
        Ok(file)
    }

    /// Creates `name` (never an existing entry or a link), mode 0600; returns
    /// it and its identity.
    fn create_new(&self, name: &str) -> io::Result<(File, EntryId)> {
        let c = c_name(name)?;
        // SAFETY: as above; the descriptor is owned by the File.
        let fd = check(unsafe { libc::openat(self.fd(), c.as_ptr(), FILE_CREATE_FLAGS, FILE_MODE) })?;
        let file = unsafe { File::from_raw_fd(fd) };
        let metadata = file.metadata()?;
        Ok((file, (metadata.dev(), metadata.ino())))
    }

    /// Removes the non-directory entry `name` only while it is `expected`.
    fn unlink_if(&self, name: &str, expected: EntryId) -> io::Result<()> {
        let stat = self.stat_child(name)?;
        if (stat.st_dev as u64, stat.st_ino as u64) != expected || !is_regular(&stat) {
            return Err(io::Error::other("it is no longer the file that was verified"));
        }
        let c = c_name(name)?;
        // SAFETY: as above; `unlinkat` never follows a link.
        check(unsafe { libc::unlinkat(self.fd(), c.as_ptr(), 0) }).map(drop)
    }

    /// Publishes `from` as `to`, never replacing an entry `to` names:
    /// `renameatx_np(RENAME_EXCL)` on macOS, `linkat` then `unlinkat` elsewhere.
    pub(crate) fn publish_new(&self, from: &str, to: &str) -> io::Result<()> {
        let (from_c, to_c) = (c_name(from)?, c_name(to)?);
        #[cfg(target_os = "macos")]
        {
            // SAFETY: valid descriptors and NUL-terminated names, both relative to this directory.
            check(unsafe { libc::renameatx_np(self.fd(), from_c.as_ptr(), self.fd(), to_c.as_ptr(), libc::RENAME_EXCL) }).map(drop)
        }
        #[cfg(not(target_os = "macos"))]
        {
            // SAFETY: as above; `linkat` fails with EEXIST rather than replace `to`.
            check(unsafe { libc::linkat(self.fd(), from_c.as_ptr(), self.fd(), to_c.as_ptr(), 0) })?;
            check(unsafe { libc::unlinkat(self.fd(), from_c.as_ptr(), 0) }).map(drop)
        }
    }

    /// Calls `each` with every entry name, at most `limit` of them; returns
    /// whether the enumeration stopped at the limit. One name at a time is held.
    fn each_name(&self, limit: u64, mut each: impl FnMut(&str)) -> io::Result<bool> {
        // SAFETY: `fdopendir` takes ownership of the duplicated descriptor and
        // `closedir` releases both; each `readdir` entry is read before the next call.
        let duplicate = check(unsafe { libc::fcntl(self.fd(), libc::F_DUPFD_CLOEXEC, 0) })?;
        let stream = unsafe { libc::fdopendir(duplicate) };
        if stream.is_null() {
            let error = io::Error::last_os_error();
            unsafe { libc::close(duplicate) };
            return Err(error);
        }
        struct Stream(*mut libc::DIR);
        impl Drop for Stream {
            fn drop(&mut self) {
                unsafe { libc::closedir(self.0) };
            }
        }
        let stream = Stream(stream);
        // The duplicate shares the handle's offset: read from the start each time.
        unsafe { libc::rewinddir(stream.0) };
        let mut seen = 0_u64;
        loop {
            if seen >= limit {
                return Ok(true);
            }
            let entry = unsafe { libc::readdir(stream.0) };
            if entry.is_null() {
                return Ok(false);
            }
            seen += 1;
            let name = unsafe { std::ffi::CStr::from_ptr((*entry).d_name.as_ptr()) };
            if let Ok(name) = name.to_str() {
                each(name);
            }
        }
    }
}

/// The folder's bounded sync: at most one sync worker at a time; a sync that
/// fails, times out or cannot start is not durable.
static ACTIVE_DIRECTORY_SYNC: AtomicBool = AtomicBool::new(false);

fn sync_directory_bounded(dir: &BackupDir, timeout: Duration) -> bool {
    let Ok(handle) = dir.0.try_clone() else { return false };
    if ACTIVE_DIRECTORY_SYNC.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
        return false;
    }
    let (sender, receiver) = mpsc::sync_channel(1);
    std::thread::spawn(move || {
        let synced = File::from(handle).sync_all().is_ok();
        ACTIVE_DIRECTORY_SYNC.store(false, Ordering::Release);
        let _ = sender.send(synced);
    });
    receiver.recv_timeout(timeout).unwrap_or(false)
}

/// The write's file operations: the system's, or a test's injected failures.
pub(crate) trait BackupIo {
    fn write_all(&mut self, file: &mut File, bytes: &[u8]) -> io::Result<()> {
        file.write_all(bytes)
    }
    fn sync_file(&mut self, file: &File) -> io::Result<()> {
        file.sync_all()
    }
    fn publish(&mut self, dir: &BackupDir, temp: &str, name: &str) -> io::Result<()> {
        dir.publish_new(temp, name)
    }
    /// The folder's sync after the publish: whether it completed.
    fn sync_directory(&mut self, dir: &BackupDir) -> bool {
        sync_directory_bounded(dir, DIRECTORY_SYNC_TIMEOUT)
    }
}

pub(crate) struct SystemBackupIo;

impl BackupIo for SystemBackupIo {}

// ---------------------------------------------------------------------------
// Verification.
// ---------------------------------------------------------------------------

/// What a check of a file against a full hash established.
#[derive(Debug, PartialEq, Eq)]
enum Streamed {
    Matches,
    Differs,
    Unavailable(String),
}

/// SHA-256 (bare hex) of an opened file of `len` bytes, streamed through
/// `block` with a hard cap at `len`.
fn stream_sha256_hex(file: &File, len: u64, block: &mut [u8]) -> Result<String, Access> {
    let mut hasher = Sha256::new();
    let mut reader = file.take(len.saturating_add(1));
    let mut total = 0_u64;
    loop {
        let count = reader.read(block).map_err(Access::Io)?;
        if count == 0 {
            break;
        }
        total += count as u64;
        hasher.update(&block[..count]);
    }
    if total != len || file.metadata().map_err(Access::Io)?.len() != len {
        return Err(Access::Changed("it changed while it was read"));
    }
    Ok(hex_of(hasher.finalize()))
}

/// Streams `name` through SHA-256 against `expected_hex`; `expected_id`, when
/// given, is the inode it must be.
fn stream_check(dir: &BackupDir, name: &str, expected_hex: &str, expected_id: Option<EntryId>, block: &mut [u8]) -> Streamed {
    let inspected = match dir.inspect(name, MAX_MODEL_DOCUMENT_BYTES) {
        Ok(inspected) => inspected,
        Err(Access::TooLarge(_)) | Err(Access::NotRegular) => return Streamed::Differs,
        Err(access) => return Streamed::Unavailable(access.to_string()),
    };
    if expected_id.is_some_and(|id| id != inspected.id) {
        return Streamed::Unavailable("it is not the file this backup wrote".to_string());
    }
    match dir.open_inspected(name, inspected).and_then(|file| stream_sha256_hex(&file, inspected.len, block)) {
        Ok(hex) if hex == expected_hex => Streamed::Matches,
        Ok(_) => Streamed::Differs,
        Err(access) => Streamed::Unavailable(access.to_string()),
    }
}

/// What a check of a candidate as a backup of the project established.
#[derive(Debug)]
enum Candidate {
    /// Its content is a backup of this project, as the inode `EntryId`.
    Backup(EntryId),
    /// It verifiably is not.
    Other,
    /// It could not be checked; it is kept.
    Unavailable,
}

/// Checks `name` as a backup of `project_id`, contained and within `memory`:
/// a regular file within the open cap, whose bytes hash with the name's hash
/// prefix, are UTF-8 canonical text of one JSON value, and hold a document
/// whose project id is `project_id`.
fn verify_candidate(dir: &BackupDir, name: &BackupName, project_id: &str, memory: &dyn ReadMemory) -> Candidate {
    let file_name = name.file_name();
    let inspected = match dir.inspect(&file_name, MAX_MODEL_DOCUMENT_BYTES) {
        Ok(inspected) => inspected,
        Err(Access::TooLarge(_)) | Err(Access::NotRegular) => return Candidate::Other,
        Err(_) => return Candidate::Unavailable,
    };
    let Ok(_load) = memory.reserve("a backup's file (rotation)", loaded_text_bytes(inspected.len)) else { return Candidate::Unavailable };
    let Ok(file) = dir.open_inspected(&file_name, inspected) else { return Candidate::Unavailable };
    let Ok(capacity) = usize::try_from(inspected.len.saturating_add(1)) else { return Candidate::Unavailable };
    let mut bytes = Vec::with_capacity(capacity);
    if (&file).take(inspected.len.saturating_add(1)).read_to_end(&mut bytes).is_err()
        || bytes.len() as u64 != inspected.len
        || file.metadata().map(|metadata| metadata.len()).ok() != Some(inspected.len)
    {
        return Candidate::Unavailable;
    }
    drop(file);
    if !hex_of(Sha256::digest(&bytes)).starts_with(&name.hash) {
        return Candidate::Other;
    }
    let Ok(text) = String::from_utf8(bytes) else { return Candidate::Other };
    let Ok(parse) = parse_bytes(&text) else { return Candidate::Other };
    let Ok(_parsing) = memory.reserve("a backup's document (rotation)", parse) else { return Candidate::Unavailable };
    let Ok(document) = serde_json::from_str::<Value>(&text) else { return Candidate::Other };
    if crate::project_id_from_model(&document) != project_id {
        return Candidate::Other;
    }
    match is_jcs_text(memory, "a backup's canonical text (rotation)", &document, &text) {
        Ok(true) => Candidate::Backup(inspected.id),
        Ok(false) => Candidate::Other,
        Err(_) => Candidate::Unavailable,
    }
}

// ---------------------------------------------------------------------------
// The project's backups in the folder.
// ---------------------------------------------------------------------------

/// The project's backup names in the folder, newest first, at most
/// [`MAX_CANDIDATES`] of them; regular files only (inspected without
/// following a link).
#[derive(Debug, Default)]
struct Scan {
    newest_first: Vec<BackupName>,
    /// The project's names beyond [`MAX_CANDIDATES`], counted.
    not_considered: usize,
    /// The enumeration stopped at [`MAX_SCANNED_ENTRIES`].
    partial: bool,
}

fn scan_project(dir: &BackupDir, stem: &str, slot: &str, limit: u64) -> io::Result<Scan> {
    let mut newest: BinaryHeap<Reverse<(String, String)>> = BinaryHeap::with_capacity(MAX_CANDIDATES + 1);
    let mut total = 0_usize;
    let partial = dir.each_name(limit, |file_name| {
        let Some(name) = BackupName::parse(file_name) else { return };
        if name.stem != stem || name.slot != slot {
            return;
        }
        if !dir.stat_child(file_name).is_ok_and(|stat| is_regular(&stat)) {
            return;
        }
        total += 1;
        newest.push(Reverse(name.key()));
        if newest.len() > MAX_CANDIDATES {
            newest.pop();
        }
    })?;
    // `into_sorted_vec` sorts the `Reverse` keys ascending: newest first.
    let newest_first: Vec<BackupName> = newest.into_sorted_vec().into_iter().filter_map(|Reverse((_, file_name))| BackupName::parse(&file_name)).collect();
    let not_considered = total - newest_first.len();
    Ok(Scan { newest_first, not_considered, partial })
}

/// Warnings, at most [`MAX_WARNINGS`] named.
#[derive(Default)]
struct Warnings {
    named: Vec<DispositionDiagnostic>,
    unnamed: usize,
}

impl Warnings {
    fn push(&mut self, code: &str, message: String) {
        if self.named.len() < MAX_WARNINGS {
            self.named.push(diagnostic(code, "", message));
        } else {
            self.unnamed += 1;
        }
    }

    fn finish(mut self) -> Vec<DispositionDiagnostic> {
        if self.unnamed > 0 {
            self.named.push(diagnostic(PRUNE_FAILED, "", format!("{} more backups were not removed.", self.unnamed)));
        }
        self.named
    }
}

/// Keeps the project's newest `keep` verified backups, the new one always
/// among them, and removes its older verified backups beyond them. Files that
/// do not verify as this project's backups are kept and not counted.
fn rotate(dir: &BackupDir, scan: &Scan, new_name: &str, keep: u32, project_id: &str, memory: &dyn ReadMemory) -> Vec<DispositionDiagnostic> {
    let mut warnings = Warnings::default();
    let mut kept = 1_u32;
    let mut unverified = 0_usize;
    for name in &scan.newest_first {
        let file_name = name.file_name();
        if file_name == new_name {
            continue;
        }
        match verify_candidate(dir, name, project_id, memory) {
            Candidate::Backup(_) if kept < keep => kept += 1,
            Candidate::Backup(id) => {
                #[cfg(test)]
                BEFORE_UNLINK.with(|hook| {
                    let hook = hook.borrow_mut().take();
                    if let Some(hook) = hook {
                        hook()
                    }
                });
                match dir.unlink_if(&file_name, id) {
                    Ok(()) => {}
                    Err(error) if error.kind() == io::ErrorKind::NotFound => {}
                    Err(error) => warnings.push(PRUNE_FAILED, format!("The older backup `{file_name}` was not removed: {error}.")),
                }
            }
            Candidate::Other | Candidate::Unavailable => unverified += 1,
        }
    }
    let mut finished = warnings.finish();
    if unverified > 0 {
        finished.push(diagnostic(
            ROTATION_UNVERIFIED,
            "",
            format!("{unverified} files named like this project's backups were not verified as its backups; they were kept and not counted."),
        ));
    }
    if scan.partial {
        finished.push(diagnostic(
            ROTATION_PARTIAL,
            "",
            format!("The backup folder holds more than the {MAX_SCANNED_ENTRIES} entries one backup reads; backups among the rest were not considered and were kept."),
        ));
    }
    if scan.not_considered > 0 {
        finished.push(diagnostic(
            ROTATION_PARTIAL,
            "",
            format!("{} of this project's oldest backup names, beyond the newest {MAX_CANDIDATES}, were not considered and were kept.", scan.not_considered),
        ));
    }
    finished
}

// ---------------------------------------------------------------------------
// The stored document.
// ---------------------------------------------------------------------------

/// The stored project's model document, admitted for a backup.
#[derive(Debug)]
pub(crate) struct StoredBackup {
    /// The RFC 8785 canonical text written; its SHA-256 is `model_hex`.
    pub(crate) canonical: Metered<String>,
    pub(crate) model_hex: String,
    pub(crate) project_id: String,
    pub(crate) stem: String,
    /// The screening's warnings (an unresolved library reference).
    pub(crate) warnings: Vec<DispositionDiagnostic>,
}

/// A refusal of the read: the memory pool's by its name, any other as the store's.
fn read_refusal(named: &str, error: String) -> Vec<DispositionDiagnostic> {
    if error.starts_with(STORE_READ_MEMORY_UNAVAILABLE) {
        return one(STORE_READ_MEMORY_UNAVAILABLE, format!("{} No backup was written.", stored_read::capped(error)));
    }
    one(STORE_UNAVAILABLE, format!("The local project store could not be read for {named} ({}); no backup was written.", stored_read::capped(error)))
}

/// The bare hex of a `sha256:<64 hex>` hash, or `None`.
fn full_hex(hash: &str) -> Option<&str> {
    hash.strip_prefix("sha256:").filter(|hex| is_lower_hex(hex, 64))
}

/// Refuses a text that holds an object member more than once, anywhere (the
/// checked canonicalizer's duplicate-refusing parse, its memory reserved by
/// `stored_read::checked_sha256_text`): the parsed document keeps only the
/// last of them. The profile's value refusals (an unsafe integer, for
/// example) come only after that parse completed and are not refused here; the
/// canonical text written is the unchecked one the model hash uses.
fn refuse_duplicate_members(memory: &dyn ReadMemory, named: &str, text: &str, document: &Value) -> Result<(), Vec<DispositionDiagnostic>> {
    const VALUE_REFUSALS: [&str; 3] = ["CHECKED-JSON-UNSAFE-", "CHECKED-JSON-NON-FINITE", "CHECKED-JSON-NUMBER-INVALID"];
    match stored_read::checked_sha256_text(memory, "the stored model document's strict parse", text, document) {
        Ok(_) => Ok(()),
        Err(error) if VALUE_REFUSALS.iter().any(|prefix| error.starts_with(prefix)) => Ok(()),
        Err(error) if error.starts_with(STORE_READ_MEMORY_UNAVAILABLE) => Err(read_refusal(named, error)),
        Err(error) if error.contains("duplicate object member") => Err(one(
            STORED_DUPLICATE_MEMBER,
            format!("The stored document of {named} holds an object member more than once ({error}); no backup was written."),
        )),
        Err(error) => Err(one(STORED_NOT_JSON, format!("The stored document of {named} is not strict JSON ({error}); no backup was written."))),
    }
}

const STORED_LENGTHS_SQL: &str = "SELECT octet_length(model_json), octet_length(model_hash_json) FROM local_projects WHERE project_id = ?1";
const STORED_HASH_SQL: &str = "SELECT model_hash_json FROM local_projects WHERE project_id = ?1";
const STORED_MODEL_SQL: &str = "SELECT model_json FROM local_projects WHERE project_id = ?1";

/// Reads the stored project `project_id` from the store at `store` (opened
/// read-only, not migrated), within `memory`, and admits its model document
/// for a backup: the row's model-hash claim must be `expected_hex`; the stored
/// text's strict parse, its canonical text (whose SHA-256 must be the claim),
/// the open path's checks and the disposition screening, each reserved first.
pub(crate) fn read_stored_backup(store: &Path, project_id: &str, expected_hex: &str, memory: &dyn ReadMemory) -> Result<StoredBackup, Vec<DispositionDiagnostic>> {
    let named = stored_document_name(project_id, None);
    let what = format!("the stored document of {named}");
    let unavailable = |error: String| read_refusal(&named, error);
    let sql = |error: rusqlite::Error| read_refusal(&named, stored_read::sql_error(&error));
    let not_found = || one(PROJECT_NOT_FOUND, format!("The local project store holds no {named}; no backup was written."));
    let mut connection = open_store_read_only(store, &named)?;
    // One read transaction: the row measured is the row loaded.
    let transaction = stored_read::read_transaction(&mut connection).map_err(unavailable)?;
    let sqlite = stored_read::admit_sqlite_working_set(&transaction, memory).map_err(unavailable)?;
    let (model_len, hash_len): (Option<i64>, Option<i64>) =
        transaction.query_row(STORED_LENGTHS_SQL, params![project_id], |row| Ok((row.get(0)?, row.get(1)?))).optional().map_err(sql)?.ok_or_else(not_found)?;
    let (model_len, hash_len) = (u64::try_from(model_len.unwrap_or(0)).unwrap_or(0), u64::try_from(hash_len.unwrap_or(0)).unwrap_or(0));
    if hash_len > stored_read::MAX_STATUS_BYTES {
        return Err(one(STORED_HASH_MISSING, format!("The stored model hash of {named} is {hash_len} bytes, not a model hash; no backup was written.")));
    }
    let hash_text: String = transaction.query_row(STORED_HASH_SQL, params![project_id], |row| row.get(0)).optional().map_err(sql)?.ok_or_else(not_found)?;
    let claim = serde_json::from_str::<Value>(&hash_text)
        .ok()
        .and_then(|hash| hash.get("value").and_then(Value::as_str).and_then(full_hex).map(str::to_string))
        .ok_or_else(|| one(STORED_HASH_MISSING, format!("The stored row of {named} holds no `sha256:` model hash; no backup was written.")))?;
    if claim != expected_hex {
        return Err(one(
            STALE,
            format!("The store now holds another model of {named} (sha256:{claim}) than the save that asked for this backup (sha256:{expected_hex}); no backup was written."),
        ));
    }
    if model_len > MAX_MODEL_DOCUMENT_BYTES {
        return Err(one(TOO_LARGE, format!("The stored document of {named} is {model_len} bytes; a model document may be at most {MAX_MODEL_DOCUMENT_BYTES} bytes. No backup was written.")));
    }
    let load = memory.reserve(&what, loaded_text_bytes(model_len)).map_err(unavailable)?;
    let text: String = transaction.query_row(STORED_MODEL_SQL, params![project_id], |row| row.get(0)).optional().map_err(sql)?.ok_or_else(not_found)?;
    drop(transaction);
    drop(connection);
    drop(sqlite);
    let not_json = |detail: String| one(STORED_NOT_JSON, format!("The stored document of {named} is not JSON ({detail}); no backup was written."));
    let parse = parse_bytes(&text).map_err(not_json)?;
    let parsing = memory.reserve(&what, parse).map_err(unavailable)?;
    let document: Value = serde_json::from_str(&text).map_err(|error| not_json(stored_read::json_error(&error)))?;
    refuse_duplicate_members(memory, &named, &text, &document)?;
    drop(text);
    drop(load);
    if crate::project_id_from_model(&document) != project_id {
        return Err(one(PROJECT_MISMATCH, format!("The stored document of {named} names another project id; no backup was written.")));
    }
    let canonical_len = stored_read::jcs_length(&document);
    if canonical_len > MAX_MODEL_DOCUMENT_BYTES {
        return Err(one(TOO_LARGE, format!("The canonical text of {what} is {canonical_len} bytes; a model document may be at most {MAX_MODEL_DOCUMENT_BYTES} bytes. No backup was written.")));
    }
    let canonical = stored_read::jcs_text(memory, &format!("the canonical text of {what}"), &document).map_err(unavailable)?;
    let model_hex = sha256_hex(canonical.get());
    if model_hex != claim {
        return Err(one(
            STORED_HASH_MISMATCH,
            format!("The canonical text of {what} hashes to sha256:{model_hex}, not the sha256:{claim} its row records; no backup was written."),
        ));
    }
    let stem = backup_stem(&document);
    let document = open_path_checks(canonical_len, document, VERSION_NOT_CURRENT)?;
    // The libraries the document references, read-only as the store above,
    // each within the command's memory with its screening's bound.
    let open = || open_store_read_only(store, &named).map_err(|refusal| refusal.into_iter().next().map(|item| item.message).unwrap_or_default());
    let mut libraries = StoreLibraryReads::new(memory, open);
    libraries.screen_model(&document)?;
    let screened = screen_model_document_dispositions(&document, &mut |project_id: &str, library_kind: &str, library_id: &str| {
        libraries.read(project_id, library_kind, library_id)
    });
    if let Some(refusal) = libraries.refusal() {
        return Err(refusal);
    }
    let warnings = screened?;
    drop(libraries);
    drop(document);
    drop(parsing);
    Ok(StoredBackup { canonical, model_hex, project_id: project_id.to_string(), stem, warnings })
}

// ---------------------------------------------------------------------------
// The write.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub(crate) enum BackupWriteOutcome {
    Written {
        file_name: String,
        /// `sha256:<hex>` of the bytes written and read back: the model hash.
        model_hash: String,
        /// `durable`, or `saved_durability_uncertain` when the folder's sync
        /// did not complete; older backups are then kept.
        durability: String,
        warnings: Vec<DispositionDiagnostic>,
    },
    /// The project's newest backup in the folder holds exactly these bytes.
    Unchanged { file_name: String, model_hash: String },
    /// Backups are off; nothing was read or written.
    Disabled,
    Refused { diagnostics: Vec<DispositionDiagnostic> },
}

impl BackupWriteOutcome {
    fn refused(code: &str, message: impl Into<String>) -> Self {
        Self::Refused { diagnostics: one(code, message) }
    }

    /// The outcome as a response sends it: its diagnostics or warnings bounded.
    fn bounded(self) -> Self {
        match self {
            Self::Written { file_name, model_hash, durability, warnings } => Self::Written { file_name, model_hash, durability, warnings: bounded_diagnostics(warnings) },
            Self::Refused { diagnostics } => Self::Refused { diagnostics: bounded_diagnostics(diagnostics) },
            other => other,
        }
    }
}

fn io_refusal(code: &str, stage: &str, error: &io::Error, removal: &str) -> BackupWriteOutcome {
    BackupWriteOutcome::refused(
        code,
        format!("The backup could not be written at its {stage} ({error}; {:?}); no older backup was touched.{removal}", error.kind()),
    )
}

/// The temporary file's removal after a failure, as a note.
fn remove_temp(dir: &BackupDir, temp: &str, id: EntryId) -> String {
    match dir.unlink_if(temp, id) {
        Ok(()) => String::new(),
        Err(error) if error.kind() == io::ErrorKind::NotFound => String::new(),
        Err(error) => format!(" Its temporary file `{temp}` was left: {error}."),
    }
}

fn temp_name() -> io::Result<String> {
    let mut token = [0_u8; 16];
    getrandom::fill(&mut token).map_err(|error| io::Error::other(error.to_string()))?;
    Ok(format!("{TEMP_PREFIX}{}{TEMP_SUFFIX}", hex_of(token)))
}

/// Writes `stored` as the project's newest backup in `folder` (§4, §5), and
/// applies the rotation `keep` (None is All) once it is durable. The caller
/// holds the app's backup lock.
pub(crate) fn write_backup_in_folder(
    folder: &Path,
    stored: &StoredBackup,
    keep: Option<u32>,
    unix_seconds: u64,
    memory: &dyn ReadMemory,
    io_ops: &mut impl BackupIo,
) -> BackupWriteOutcome {
    let model_hash = format!("sha256:{}", stored.model_hex);
    let Some(stamp) = utc_stamp(unix_seconds) else {
        return BackupWriteOutcome::refused(CLOCK_INVALID, format!("The clock reads {unix_seconds} s, beyond a backup name's year 9999. No backup was written."));
    };
    if !folder.is_absolute() {
        return BackupWriteOutcome::refused(FOLDER_UNAVAILABLE, "The backup folder in the settings is not an absolute path; no backup was written. Choose the folder again.");
    }
    let dir = match BackupDir::open(folder) {
        Ok(dir) => dir,
        Err(error) => {
            let why = match error.kind() {
                io::ErrorKind::NotFound => "it is no longer there".to_string(),
                _ if error.raw_os_error() == Some(libc::ELOOP) || error.raw_os_error() == Some(libc::ENOTDIR) => "it is not a folder (or is a symbolic link)".to_string(),
                _ => error.to_string(),
            };
            return BackupWriteOutcome::refused(FOLDER_UNAVAILABLE, format!("The backup folder is unavailable: {why}. No backup was written."));
        }
    };
    let slot = backup_slot(&stored.project_id);
    let name = BackupName { stem: stored.stem.clone(), slot: slot.clone(), stamp, hash: stored.model_hex[..HASH_DIGITS].to_string() };
    let file_name = name.file_name();
    let memory_refused = |error: String| BackupWriteOutcome::refused(STORE_READ_MEMORY_UNAVAILABLE, format!("{} No backup was written.", stored_read::capped(error)));
    let _listing = match memory.reserve("the project's backups in the folder", (MAX_CANDIDATES as u64).saturating_mul(CANDIDATE_ENTRY_BYTES)) {
        Ok(reservation) => reservation,
        Err(error) => return memory_refused(error),
    };
    let _block_reservation = match memory.reserve("the backup read-back block", READBACK_BLOCK_BYTES as u64) {
        Ok(reservation) => reservation,
        Err(error) => return memory_refused(error),
    };
    let mut block = vec![0_u8; READBACK_BLOCK_BYTES];
    let scan = match scan_project(&dir, &stored.stem, &slot, MAX_SCANNED_ENTRIES) {
        Ok(scan) => scan,
        Err(error) => return BackupWriteOutcome::refused(FOLDER_UNAVAILABLE, format!("The backup folder could not be listed: {error}. No backup was written.")),
    };
    // Unchanged only when the project's newest backup streams as the full
    // model hash, which identifies exactly these bytes (and so this project).
    if !scan.partial {
        if let Some(newest) = scan.newest_first.first().filter(|newest| newest.hash == name.hash) {
            if stream_check(&dir, &newest.file_name(), &stored.model_hex, None, &mut block) == Streamed::Matches {
                return BackupWriteOutcome::Unchanged { file_name: newest.file_name(), model_hash };
            }
        }
    }
    let bytes = stored.canonical.get().as_bytes();
    let temp = match temp_name() {
        Ok(temp) => temp,
        Err(error) => return io_refusal(TEMP_CREATE_FAILED, "temporary name", &error, ""),
    };
    let (mut file, temp_id) = match dir.create_new(&temp) {
        Ok(created) => created,
        Err(error) => return io_refusal(TEMP_CREATE_FAILED, "temporary file", &error, ""),
    };
    if let Err(error) = io_ops.write_all(&mut file, bytes) {
        drop(file);
        return io_refusal(WRITE_FAILED, "write", &error, &remove_temp(&dir, &temp, temp_id));
    }
    if let Err(error) = io_ops.sync_file(&file) {
        drop(file);
        return io_refusal(FILE_SYNC_FAILED, "file sync", &error, &remove_temp(&dir, &temp, temp_id));
    }
    drop(file);
    // Published under a new name only: an entry of that name is never replaced.
    if let Err(error) = io_ops.publish(&dir, &temp, &file_name) {
        let removal = remove_temp(&dir, &temp, temp_id);
        if error.kind() == io::ErrorKind::AlreadyExists {
            return match stream_check(&dir, &file_name, &stored.model_hex, None, &mut block) {
                Streamed::Matches => BackupWriteOutcome::Unchanged { file_name, model_hash },
                _ => BackupWriteOutcome::refused(
                    NAME_IN_USE,
                    format!("Another file is already named `{file_name}` in the backup folder and is not this backup; it was left unchanged and no backup was written.{removal}"),
                ),
            };
        }
        if matches!(error.raw_os_error(), Some(libc::ENOTSUP) | Some(libc::EINVAL)) {
            return io_refusal(PUBLISH_UNSUPPORTED, "publish (the folder's volume cannot add a name without replacing)", &error, &removal);
        }
        return io_refusal(PUBLISH_FAILED, "publish", &error, &removal);
    }
    // The published entry must be the inode written and read back as its hash.
    match stream_check(&dir, &file_name, &stored.model_hex, Some(temp_id), &mut block) {
        Streamed::Matches => {}
        Streamed::Differs => {
            let removal = match dir.unlink_if(&file_name, temp_id) {
                Ok(()) => " It was removed.".to_string(),
                Err(error) => format!(" It could not be removed: {error}."),
            };
            return BackupWriteOutcome::refused(
                READBACK_MISMATCH,
                format!("The written backup `{file_name}` does not read back as {model_hash}; no older backup was touched.{removal}"),
            );
        }
        Streamed::Unavailable(detail) => {
            return BackupWriteOutcome::refused(
                READBACK_FAILED,
                format!("The written backup `{file_name}` could not be read back ({detail}); it is kept unverified and no older backup was touched."),
            );
        }
    }
    drop(block);
    let durable = io_ops.sync_directory(&dir);
    let mut warnings = stored.warnings.clone();
    if !durable {
        warnings.push(diagnostic(PRUNE_DEFERRED, "", "The backup folder's sync did not complete, so no older backup was removed until a later backup is durable."));
    } else if let Some(keep) = keep {
        warnings.extend(rotate(&dir, &scan, &file_name, keep, &stored.project_id, memory));
    }
    BackupWriteOutcome::Written {
        file_name,
        model_hash,
        durability: if durable { "durable" } else { "saved_durability_uncertain" }.to_string(),
        warnings,
    }
}

/// `{projectId, expectedModelHash}`.
#[derive(Debug, Clone)]
pub(crate) struct WriteRequest {
    pub(crate) project_id: String,
    pub(crate) expected_model_hash: String,
}

pub(crate) fn write_request(args: &mut Args) -> Result<WriteRequest, SolveJobError> {
    Ok(WriteRequest { project_id: args.decode("projectId")?, expected_model_hash: args.decode("expectedModelHash")? })
}

/// The command after its ingress: the settings, the backup lock, the stored
/// read and the write, within `memory` (seeded with the ingress lease). `data`
/// is the app's local data directory, which holds the store, the settings and
/// the locks.
pub(crate) fn write_at(data: Result<PathBuf, String>, request: &WriteRequest, unix_seconds: Option<u64>, memory: &dyn ReadMemory) -> BackupWriteOutcome {
    write_at_with_io(data, request, unix_seconds, memory, &mut SystemBackupIo)
}

pub(crate) fn write_at_with_io(
    data: Result<PathBuf, String>,
    request: &WriteRequest,
    unix_seconds: Option<u64>,
    memory: &dyn ReadMemory,
    io_ops: &mut impl BackupIo,
) -> BackupWriteOutcome {
    let Some(expected_hex) = full_hex(&request.expected_model_hash) else {
        return BackupWriteOutcome::refused(HASH_INVALID, "The expected model hash is not `sha256:` and 64 lowercase hex digits; no backup was written.");
    };
    let data = match data {
        Ok(data) => data,
        Err(error) => return BackupWriteOutcome::refused(autosave::DIRECTORY_UNAVAILABLE, format!("The app's local data directory is unavailable: {error}. No backup was written.")),
    };
    // Backups are on only with a folder recorded with the explicit intent.
    let settings = crate::autosave_commands::read_settings_record(&data, memory);
    let record: SettingsRecord = settings.get().record.clone();
    drop(settings);
    let Some(folder) = record.backup_folder_in_force().map(PathBuf::from) else {
        return BackupWriteOutcome::Disabled;
    };
    let Some(unix_seconds) = unix_seconds else {
        return BackupWriteOutcome::refused(CLOCK_INVALID, "The clock reads before 1970. No backup was written.");
    };
    let _overhead = match memory.reserve("the backup (the command's own memory)", stored_read::READ_OVERHEAD_BYTES) {
        Ok(reservation) => reservation,
        Err(error) => return BackupWriteOutcome::refused(STORE_READ_MEMORY_UNAVAILABLE, format!("{} No backup was written.", stored_read::capped(error))),
    };
    // One backup transaction at a time, across windows and processes.
    let _lock = match autosave::lock_slot_at(&data, BACKUP_LOCK_SLOT, LOCK_WAIT) {
        Ok(lock) => lock,
        Err(refusal) => return BackupWriteOutcome::Refused { diagnostics: vec![refusal] },
    };
    let stored = match read_stored_backup(&data.join(crate::PROJECT_STORE_FILE), &request.project_id, expected_hex, memory) {
        Ok(stored) => stored,
        Err(diagnostics) => return BackupWriteOutcome::Refused { diagnostics },
    };
    write_backup_in_folder(&folder, &stored, record.backup_keep, unix_seconds, memory, io_ops)
}

// ---------------------------------------------------------------------------
// Configuration.
// ---------------------------------------------------------------------------

/// `{enabled, backupKeep, explicitLocalPrivateIntent}`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ConfigureRequest {
    pub(crate) enabled: bool,
    pub(crate) backup_keep: Option<u32>,
    pub(crate) explicit_local_private_intent: bool,
}

pub(crate) fn configure_request(args: &mut Args) -> Result<ConfigureRequest, SolveJobError> {
    Ok(ConfigureRequest {
        enabled: args.decode("enabled")?,
        backup_keep: args.decode("backupKeep")?,
        explicit_local_private_intent: args.decode("explicitLocalPrivateIntent")?,
    })
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub(crate) enum BackupConfigureOutcome {
    Saved { settings: AutosaveSettings },
    /// The folder chooser was cancelled; nothing was written.
    Cancelled,
    Refused { diagnostics: Vec<DispositionDiagnostic> },
}

impl From<AutosaveSettingsWriteOutcome> for BackupConfigureOutcome {
    fn from(outcome: AutosaveSettingsWriteOutcome) -> Self {
        match outcome {
            AutosaveSettingsWriteOutcome::Saved { settings } => Self::Saved { settings },
            AutosaveSettingsWriteOutcome::Refused { diagnostics } => Self::Refused { diagnostics },
        }
    }
}

/// The chosen folder as the settings record it: an absolute UTF-8 path that
/// opens as a folder whose last component is not a symlink.
fn admitted_folder(folder: &Path) -> Result<String, Vec<DispositionDiagnostic>> {
    let invalid = |why: &str| one(FOLDER_INVALID, format!("The chosen backup folder {why}; nothing was changed."));
    if !folder.is_absolute() {
        return Err(invalid("is not an absolute path"));
    }
    let Some(text) = folder.to_str() else { return Err(invalid("is not a UTF-8 path")) };
    BackupDir::open(folder).map_err(|error| invalid(&format!("cannot be opened as a folder that is not a symbolic link ({error})")))?;
    Ok(text.to_string())
}

/// Turns backups on or off (§4). Off clears the folder and the intent and
/// deletes nothing. On needs the explicit local-private intent before the
/// chooser (`choose`) is shown; a cancelled chooser writes nothing. The
/// folder, the rotation and the intent are written together under the
/// settings lock, keeping the interval another window set.
pub(crate) fn configure_at(
    data: Result<PathBuf, String>,
    request: &ConfigureRequest,
    choose: impl FnOnce() -> Result<Option<PathBuf>, String>,
    memory: &dyn ReadMemory,
) -> BackupConfigureOutcome {
    if let Some(keep) = request.backup_keep.filter(|keep| !BACKUP_KEEP_CHOICES.contains(keep)) {
        return BackupConfigureOutcome::Refused {
            diagnostics: one(SETTINGS_OUT_OF_RANGE, format!("Keeping {keep} backups is not a choice (all, 3, 5, 10, 20 or 50); nothing was changed.")),
        };
    }
    let backup_keep = request.backup_keep;
    if !request.enabled {
        return update_settings_at(data, memory, |record| Ok(SettingsRecord { backup_folder: None, backup_keep, backup_local_private_intent: false, ..record })).into();
    }
    if !request.explicit_local_private_intent {
        return BackupConfigureOutcome::Refused {
            diagnostics: one(INTENT_REQUIRED, "Backups copy the project's private data to a folder you choose; that needs the explicit choice to do so. Nothing was changed."),
        };
    }
    let folder = match choose() {
        Ok(Some(folder)) => folder,
        Ok(None) => return BackupConfigureOutcome::Cancelled,
        Err(error) => return BackupConfigureOutcome::Refused { diagnostics: one(FOLDER_INVALID, format!("The folder chooser failed ({}); nothing was changed.", stored_read::capped(error))) },
    };
    let folder = match admitted_folder(&folder) {
        Ok(folder) => folder,
        Err(diagnostics) => return BackupConfigureOutcome::Refused { diagnostics },
    };
    update_settings_at(data, memory, |record| Ok(SettingsRecord { backup_folder: Some(folder), backup_keep, backup_local_private_intent: true, ..record })).into()
}

// ---------------------------------------------------------------------------
// The Tauri commands.
// ---------------------------------------------------------------------------

/// Admits one backup command's raw body within its budget (§6).
pub(crate) fn admit_body(
    registry: &SolveJobRegistry,
    body: &tauri::ipc::InvokeBody,
    headers: &tauri::http::HeaderMap,
    command: &'static str,
) -> Result<(Args, crate::memory_pool::Lease), SolveJobError> {
    let max = if command == WRITE_COMMAND { WRITE_COMMAND_MAX_ARGS_BYTES } else { CONFIGURE_COMMAND_MAX_ARGS_BYTES };
    command_ingress::admit_body_within(registry, body, headers, command, ArgsDecoding::Moved, max)
}

/// The native folder chooser, starting in the documents folder (else home).
fn choose_backup_folder(app: &AppHandle) -> Result<Option<PathBuf>, String> {
    let mut chooser = app.dialog().file();
    if let Some(directory) = default_save_directory(app.path().document_dir().ok(), app.path().home_dir().ok()) {
        chooser = chooser.set_directory(directory);
    }
    match chooser.blocking_pick_folder() {
        None => Ok(None),
        Some(selected) => selected.into_path().map(Some).map_err(|error| error.to_string()),
    }
}

/// Turns backups on (explicit intent, then the native folder chooser) or off.
/// The chooser blocks a worker, not the main thread.
#[tauri::command]
pub async fn configure_project_backup(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, lease) = admit_body(&registry, invoke.body(), invoke.headers(), CONFIGURE_COMMAND)?;
    let request = configure_request(&mut args)?;
    drop(args);
    on_worker(app, CONFIGURE_COMMAND, lease, move |app, memory| {
        Metered::new(configure_at(data_directory(app), &request, || choose_backup_folder(app), memory))
    })
    .await
}

/// The stored project's last saved model as a backup in the chosen folder,
/// after a landed save; on its own worker, so it never delays or rolls back
/// the save.
#[tauri::command]
pub async fn write_project_backup(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, lease) = admit_body(&registry, invoke.body(), invoke.headers(), WRITE_COMMAND)?;
    let request = write_request(&mut args)?;
    drop(args);
    on_worker(app, WRITE_COMMAND, lease, move |app, memory| Metered::new(write_at(data_directory(app), &request, now_unix_seconds(), memory).bounded())).await
}

#[cfg(test)]
#[path = "backup_tests.rs"]
mod tests;
