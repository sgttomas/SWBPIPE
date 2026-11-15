//! Nondeleting backups of the verified stored model.
//!
//! The native chooser and explicit private-copy intent authorize a local folder.
//! Only All is implemented: no backup-copy path removes, renames or replaces any
//! entry, including temporary, partial and unverified outputs. Automatic
//! rotation remains unfinished. Finite settings are read without modification
//! but cannot authorize a backup in this unit.
//!
//! macOS publication uses fclonefileat from the still-held created source FD
//! to one absent name relative to the held folder FD. No source name is
//! resolved again and there is no copy, rename or link fallback. The folder's
//! volume must report a valid VOL_CAP_INT_CLONE capability at enable time and
//! before each backup. Actual clone, source/final sync, read-back and folder
//! durability remain separate checks. Supported volume qualification is open.
//!
//! The source is created mode 0600, written and synced; the final file is read
//! through a nonfollowing handle, checked as the full expected hash, synced and
//! rechecked by identity before bounded directory sync. A failed check keeps
//! everything and reports a created/unverified or unknown outcome truthfully.
//! Later external mutation is not prevented. Advisory locks serialize only
//! cooperating SWBPIPE operations, and do not enforce same-UID custody.
//!
//! Lock order is backup -> settings. Backup holds its lock from settings
//! authorization through effects; configuration takes that lock before changing
//! settings. An acknowledged Disable cannot be followed by a queued backup
//! authorized by older settings. Completion/diagnostic/serialization memory is
//! admitted before effects and held until the IPC response exists.

use crate::autosave::{self, LOCK_WAIT, READBACK_BLOCK_BYTES};
use crate::autosave_commands::{
    data_directory, seeded_memory, update_settings_at, AutosaveSettings,
    AutosaveSettingsWriteOutcome, SettingsRecord, RECORD_COMMAND_MAX_ARGS_BYTES,
    SETTINGS_COMMAND_MAX_ARGS_BYTES,
};
use crate::command_ingress::{self, Args, ArgsDecoding, CommandError};
use crate::export_disposition_screen::{diagnostic, DispositionDiagnostic};
use crate::model_document_file::MAX_MODEL_DOCUMENT_BYTES;
use crate::model_document_save::{
    default_save_directory, open_path_checks, screen_model_document_dispositions,
    suggested_model_document_file_name,
};
use crate::solve_jobs::{SolveJobError, SolveJobRegistry};
use crate::stored_model_document_export::{
    bounded_diagnostics, open_store_read_only, stored_document_name, StoreLibraryReads,
    LIBRARY_STORE_BYTES, MAX_RESPONSE_BYTES,
};
use crate::stored_read::{
    self, loaded_text_bytes, parse_bytes, Metered, ReadMemory, STORE_READ_MEMORY_UNAVAILABLE,
};
use open_pipe_stress_operation_applier::sha256_hex;
use rusqlite::{params, OptionalExtension};
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{self, Read, Write};
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc,
};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::DialogExt;

pub(crate) const CONFIGURE_COMMAND: &str = "configure_project_backup";
pub(crate) const WRITE_COMMAND: &str = "write_project_backup";
pub(crate) const CONFIGURE_COMMAND_MAX_ARGS_BYTES: usize = SETTINGS_COMMAND_MAX_ARGS_BYTES;
pub(crate) const WRITE_COMMAND_MAX_ARGS_BYTES: usize = RECORD_COMMAND_MAX_ARGS_BYTES;
pub(crate) const BACKUP_SUFFIX: &str = ".swbpipe-backup.json";
pub(crate) const TEMP_PREFIX: &str = ".swbpipe-backup-";
pub(crate) const BACKUP_LOCK_SLOT: &str = "swbpipe-backup";
const MAX_SCANNED_ENTRIES: u64 = 100_000;
const DIRECTORY_SYNC_TIMEOUT: Duration = Duration::from_secs(2);
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
pub(crate) const DURABILITY_UNVERIFIED: &str = "BACKUP-DURABILITY-UNVERIFIED";
pub(crate) const TEMP_KEPT: &str = "BACKUP-TEMP-KEPT";
pub(crate) const ROTATION_UNAVAILABLE: &str = "BACKUP-ROTATION-UNAVAILABLE";

fn one(code: &str, message: impl Into<String>) -> Vec<DispositionDiagnostic> {
    vec![diagnostic(code, "", message)]
}
fn is_lower_hex(text: &str, digits: usize) -> bool {
    text.len() == digits
        && text
            .bytes()
            .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
}

fn hex_of(digest: impl AsRef<[u8]>) -> String {
    digest
        .as_ref()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
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
    !stem.is_empty()
        && stem.chars().count() <= MAX_STEM_CHARS
        && stem.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-')
        })
}

fn is_stamp(stamp: &str) -> bool {
    let bytes = stamp.as_bytes();
    bytes.len() == STAMP_LEN
        && bytes[8] == b'T'
        && bytes[15] == b'Z'
        && bytes[..8]
            .iter()
            .chain(&bytes[9..15])
            .all(u8::is_ascii_digit)
}

impl BackupName {
    pub(crate) fn file_name(&self) -> String {
        format!(
            "{}.{}.{}.{}{BACKUP_SUFFIX}",
            self.stem, self.slot, self.stamp, self.hash
        )
    }

    /// The parts of a backup's name, or `None` for any other name (a
    /// temporary file, a path with a separator, another file). The stem may
    /// hold dots, so the parts are taken from the right.
    pub(crate) fn parse(file_name: &str) -> Option<Self> {
        let rest = file_name.strip_suffix(BACKUP_SUFFIX)?;
        let mut parts = rest.rsplitn(4, '.');
        let (hash, stamp, slot, stem) =
            (parts.next()?, parts.next()?, parts.next()?, parts.next()?);
        if !is_lower_hex(hash, HASH_DIGITS)
            || !is_stamp(stamp)
            || !is_lower_hex(slot, SLOT_DIGITS)
            || !is_stem(stem)
        {
            return None;
        }
        Some(Self {
            stem: stem.to_string(),
            slot: slot.to_string(),
            stamp: stamp.to_string(),
            hash: hash.to_string(),
        })
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
    Some(format!(
        "{year:04}{month:02}{day:02}T{:02}{:02}{:02}Z",
        seconds / 3600,
        seconds % 3600 / 60,
        seconds % 60
    ))
}

fn now_unix_seconds() -> Option<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .map(|elapsed| elapsed.as_secs())
}

// All file names are single components relative to this held folder.
const DIR_FLAGS: libc::c_int =
    libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_NONBLOCK | libc::O_CLOEXEC;
const FILE_READ_FLAGS: libc::c_int =
    libc::O_RDONLY | libc::O_NOFOLLOW | libc::O_NONBLOCK | libc::O_CLOEXEC;
const FILE_CREATE_FLAGS: libc::c_int =
    libc::O_RDWR | libc::O_CREAT | libc::O_EXCL | libc::O_NOFOLLOW | libc::O_CLOEXEC;
fn check(result: libc::c_int) -> io::Result<libc::c_int> {
    if result < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(result)
    }
}
fn c_name(name: &str) -> io::Result<std::ffi::CString> {
    if name.is_empty() || name.contains('/') || name == "." || name == ".." {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "not one path component",
        ));
    }
    Ok(std::ffi::CString::new(name)?)
}
fn is_regular(stat: &libc::stat) -> bool {
    (stat.st_mode as u32 & libc::S_IFMT as u32) == libc::S_IFREG as u32
}
#[derive(Debug)]
pub(crate) struct BackupDir(OwnedFd);
#[cfg(test)]
thread_local! {
    static BEFORE_FINAL_OPEN: std::cell::RefCell<Option<Box<dyn FnOnce()>>> = std::cell::RefCell::new(None);
    static AFTER_FINAL_READ: std::cell::RefCell<Option<Box<dyn FnOnce()>>> = std::cell::RefCell::new(None);
    static BEFORE_BACKUP_LOCK: std::cell::RefCell<Option<Box<dyn FnOnce()>>> = std::cell::RefCell::new(None);
}
impl BackupDir {
    pub(crate) fn open(path: &Path) -> io::Result<Self> {
        use std::os::unix::ffi::OsStrExt;
        let path = std::ffi::CString::new(path.as_os_str().as_bytes())?;
        // SAFETY: terminated path, returned FD is owned on success.
        let fd = check(unsafe { libc::open(path.as_ptr(), DIR_FLAGS) })?;
        Ok(Self(unsafe { OwnedFd::from_raw_fd(fd) }))
    }
    fn fd(&self) -> libc::c_int {
        self.0.as_raw_fd()
    }
    fn stat_child(&self, name: &str) -> io::Result<libc::stat> {
        let name = c_name(name)?;
        // SAFETY: valid held FD, terminated component, output fully initialized by syscall.
        let mut stat: libc::stat = unsafe { std::mem::zeroed() };
        check(unsafe {
            libc::fstatat(
                self.fd(),
                name.as_ptr(),
                &mut stat,
                libc::AT_SYMLINK_NOFOLLOW,
            )
        })?;
        Ok(stat)
    }
    fn create_new(&self, name: &str) -> io::Result<File> {
        let name = c_name(name)?;
        // SAFETY: valid held FD and component; no existing entry is opened.
        let fd =
            check(unsafe { libc::openat(self.fd(), name.as_ptr(), FILE_CREATE_FLAGS, FILE_MODE) })?;
        Ok(unsafe { File::from_raw_fd(fd) })
    }
    /// Read-only capability metadata. Both valid and supported bits are required.
    fn require_clone_capability(&self) -> io::Result<()> {
        #[cfg(target_os = "macos")]
        {
            // getattrlist's four-byte-packed result is a u32 length followed
            // by vol_capabilities_attr_t: capabilities[4], then valid[4].
            // An integer buffer preserves that ABI without a local C struct.
            let mut result = [0_u32; 9];
            // SAFETY: attrlist has plain integer fields; bitmap requests exactly caps.
            let mut attrs: libc::attrlist = unsafe { std::mem::zeroed() };
            attrs.bitmapcount = libc::ATTR_BIT_MAP_COUNT as u16;
            attrs.volattr = libc::ATTR_VOL_INFO | libc::ATTR_VOL_CAPABILITIES;
            check(unsafe {
                libc::fgetattrlist(
                    self.fd(),
                    &mut attrs as *mut libc::attrlist as *mut libc::c_void,
                    result.as_mut_ptr().cast::<libc::c_void>(),
                    std::mem::size_of_val(&result),
                    0,
                )
            })?;
            if result[0] as usize != std::mem::size_of_val(&result) {
                return Err(io::Error::other(
                    "volume clone capability metadata is incomplete",
                ));
            }
            clone_capability(
                result[1 + 4 + libc::VOL_CAPABILITIES_INTERFACES],
                result[1 + libc::VOL_CAPABILITIES_INTERFACES],
            )
        }
        #[cfg(not(target_os = "macos"))]
        {
            Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "FD-source atomic clone publication is unavailable on this platform",
            ))
        }
    }
    /// No source pathname, no overwrite and no fallback.
    pub(crate) fn publish_new(&self, source: &File, name: &str) -> io::Result<()> {
        let name = c_name(name)?;
        #[cfg(target_os = "macos")]
        {
            // SAFETY: held regular source FD, held folder FD, one terminated destination.
            check(unsafe {
                libc::fclonefileat(
                    source.as_raw_fd(),
                    self.fd(),
                    name.as_ptr(),
                    0x0002, /* CLONE_NOOWNERCOPY, macOS sys/clonefile.h */
                )
            })
            .map(drop)
        }
        #[cfg(not(target_os = "macos"))]
        {
            let _ = (source, name);
            Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "no supported FD-source atomic clone primitive",
            ))
        }
    }
    /// Existing names only; temporary files and nonregular entries are ignored.
    fn newest(&self, slot: &str) -> io::Result<Option<BackupName>> {
        let duplicate = check(unsafe { libc::fcntl(self.fd(), libc::F_DUPFD_CLOEXEC, 0) })?;
        // SAFETY: fdopendir takes the duplicate; close it manually only on failure.
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
        unsafe { libc::rewinddir(stream.0) };
        let mut newest: Option<BackupName> = None;
        for _ in 0..MAX_SCANNED_ENTRIES {
            #[cfg(target_os = "macos")]
            unsafe {
                *libc::__error() = 0;
            }
            let entry = unsafe { libc::readdir(stream.0) };
            if entry.is_null() {
                #[cfg(target_os = "macos")]
                if unsafe { *libc::__error() } != 0 {
                    return Err(io::Error::last_os_error());
                }
                return Ok(newest);
            }
            let name = unsafe { std::ffi::CStr::from_ptr((*entry).d_name.as_ptr()) };
            let Some(name) = name.to_str().ok().and_then(BackupName::parse) else {
                continue;
            };
            if name.slot != slot
                || !self
                    .stat_child(&name.file_name())
                    .is_ok_and(|stat| is_regular(&stat))
            {
                continue;
            }
            if newest
                .as_ref()
                .is_none_or(|previous| name.key() > previous.key())
            {
                newest = Some(name);
            }
        }
        // An incomplete enumeration cannot establish which copy is newest.
        Ok(None)
    }
    fn read_verified(
        &self,
        name: &str,
        expected: &str,
        block: &mut [u8],
    ) -> Result<File, (&'static str, String)> {
        let unavailable = |error: io::Error| (READBACK_FAILED, error.to_string());
        let inspected = self.stat_child(name).map_err(unavailable)?;
        if !is_regular(&inspected)
            || inspected.st_size < 0
            || inspected.st_size as u64 > MAX_MODEL_DOCUMENT_BYTES
        {
            return Err((
                READBACK_FAILED,
                "the entry is not a regular model-sized file".into(),
            ));
        }
        #[cfg(test)]
        BEFORE_FINAL_OPEN.with(|hook| {
            if let Some(hook) = hook.borrow_mut().take() {
                hook();
            }
        });
        let cname = c_name(name).map_err(unavailable)?;
        let fd = check(unsafe { libc::openat(self.fd(), cname.as_ptr(), FILE_READ_FLAGS) })
            .map_err(unavailable)?;
        let file = unsafe { File::from_raw_fd(fd) };
        let before = file.metadata().map_err(unavailable)?;
        if before.mode() & 0o777 != FILE_MODE {
            return Err((READBACK_FAILED, "the copy does not have mode 0600".into()));
        }
        if !before.is_file()
            || (before.dev(), before.ino(), before.len())
                != (
                    inspected.st_dev as u64,
                    inspected.st_ino as u64,
                    inspected.st_size as u64,
                )
        {
            return Err((
                READBACK_FAILED,
                "the entry changed between inspection and open".into(),
            ));
        }
        let mut reader = (&file).take(before.len().saturating_add(1));
        let mut read = 0_u64;
        let mut hash = Sha256::new();
        loop {
            let n = reader.read(block).map_err(unavailable)?;
            if n == 0 {
                break;
            }
            read += n as u64;
            hash.update(&block[..n]);
        }
        #[cfg(test)]
        AFTER_FINAL_READ.with(|hook| {
            if let Some(hook) = hook.borrow_mut().take() {
                hook();
            }
        });
        let after = file.metadata().map_err(unavailable)?;
        let named = self.stat_child(name).map_err(unavailable)?;
        if read != before.len()
            || after.len() != before.len()
            || (
                before.mtime(),
                before.mtime_nsec(),
                before.ctime(),
                before.ctime_nsec(),
            ) != (
                after.mtime(),
                after.mtime_nsec(),
                after.ctime(),
                after.ctime_nsec(),
            )
            || !is_regular(&named)
            || (named.st_dev as u64, named.st_ino as u64) != (after.dev(), after.ino())
        {
            return Err((
                READBACK_FAILED,
                "the file or its name changed during read-back".into(),
            ));
        }
        if hex_of(hash.finalize()) != expected {
            return Err((
                READBACK_MISMATCH,
                "the full expected model hash did not match".into(),
            ));
        }
        Ok(file)
    }
}
#[cfg(target_os = "macos")]
fn clone_capability(valid: u32, capabilities: u32) -> io::Result<()> {
    if valid & libc::VOL_CAP_INT_CLONE == 0 {
        return Err(io::Error::other(
            "volume clone support is unknown (validity bit absent)",
        ));
    }
    if capabilities & libc::VOL_CAP_INT_CLONE == 0 {
        return Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "volume reports no clone support",
        ));
    }
    Ok(())
}
static ACTIVE_DIRECTORY_SYNC: AtomicBool = AtomicBool::new(false);
fn sync_directory_bounded(dir: &BackupDir) -> bool {
    let Ok(handle) = dir.0.try_clone() else {
        return false;
    };
    if ACTIVE_DIRECTORY_SYNC
        .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
        .is_err()
    {
        return false;
    }
    let (sender, receiver) = mpsc::sync_channel(1);
    std::thread::spawn(move || {
        let synced = File::from(handle).sync_all().is_ok();
        ACTIVE_DIRECTORY_SYNC.store(false, Ordering::Release);
        let _ = sender.send(synced);
    });
    receiver
        .recv_timeout(DIRECTORY_SYNC_TIMEOUT)
        .unwrap_or(false)
}
pub(crate) trait BackupIo {
    fn capability(&mut self, dir: &BackupDir) -> io::Result<()> {
        dir.require_clone_capability()
    }
    fn write_all(&mut self, file: &mut File, bytes: &[u8]) -> io::Result<()> {
        file.write_all(bytes)
    }
    fn sync_source(&mut self, file: &File) -> io::Result<()> {
        file.sync_all()
    }
    fn publish(&mut self, dir: &BackupDir, source: &File, name: &str) -> io::Result<()> {
        dir.publish_new(source, name)
    }
    fn sync_final(&mut self, file: &File) -> io::Result<()> {
        file.sync_all()
    }
    fn sync_directory(&mut self, dir: &BackupDir) -> bool {
        sync_directory_bounded(dir)
    }
}
pub(crate) struct SystemBackupIo;
impl BackupIo for SystemBackupIo {}
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
        return one(
            STORE_READ_MEMORY_UNAVAILABLE,
            format!("{} No backup was written.", stored_read::capped(error)),
        );
    }
    one(
        STORE_UNAVAILABLE,
        format!(
            "The local project store could not be read for {named} ({}); no backup was written.",
            stored_read::capped(error)
        ),
    )
}

/// The bare hex of a `sha256:<64 hex>` hash, or `None`.
fn full_hex(hash: &str) -> Option<&str> {
    hash.strip_prefix("sha256:")
        .filter(|hex| is_lower_hex(hex, 64))
}

/// Refuses a text that holds an object member more than once, anywhere (the
/// checked canonicalizer's duplicate-refusing parse, its memory reserved by
/// `stored_read::checked_sha256_text`): the parsed document keeps only the
/// last of them. The profile's value refusals (an unsafe integer, for
/// example) come only after that parse completed and are not refused here; the
/// canonical text written is the unchecked one the model hash uses.
fn refuse_duplicate_members(
    memory: &dyn ReadMemory,
    named: &str,
    text: &str,
    document: &Value,
) -> Result<(), Vec<DispositionDiagnostic>> {
    const VALUE_REFUSALS: [&str; 3] = [
        "CHECKED-JSON-UNSAFE-",
        "CHECKED-JSON-NON-FINITE",
        "CHECKED-JSON-NUMBER-INVALID",
    ];
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
pub(crate) fn read_stored_backup(
    store: &Path,
    project_id: &str,
    expected_hex: &str,
    memory: &dyn ReadMemory,
) -> Result<StoredBackup, Vec<DispositionDiagnostic>> {
    let named = stored_document_name(project_id, None);
    let what = format!("the stored document of {named}");
    let unavailable = |error: String| read_refusal(&named, error);
    let sql = |error: rusqlite::Error| read_refusal(&named, stored_read::sql_error(&error));
    let not_found = || {
        one(
            PROJECT_NOT_FOUND,
            format!("The local project store holds no {named}; no backup was written."),
        )
    };
    let mut connection = open_store_read_only(store, &named)?;
    // One read transaction: the row measured is the row loaded.
    let transaction = stored_read::read_transaction(&mut connection).map_err(unavailable)?;
    let sqlite =
        stored_read::admit_sqlite_working_set(&transaction, memory).map_err(unavailable)?;
    let (model_len, hash_len): (Option<i64>, Option<i64>) = transaction
        .query_row(STORED_LENGTHS_SQL, params![project_id], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })
        .optional()
        .map_err(sql)?
        .ok_or_else(not_found)?;
    let (model_len, hash_len) = (
        u64::try_from(model_len.unwrap_or(0)).unwrap_or(0),
        u64::try_from(hash_len.unwrap_or(0)).unwrap_or(0),
    );
    if hash_len > stored_read::MAX_STATUS_BYTES {
        return Err(one(STORED_HASH_MISSING, format!("The stored model hash of {named} is {hash_len} bytes, not a model hash; no backup was written.")));
    }
    let hash_text: String = transaction
        .query_row(STORED_HASH_SQL, params![project_id], |row| row.get(0))
        .optional()
        .map_err(sql)?
        .ok_or_else(not_found)?;
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
    let load = memory
        .reserve(&what, loaded_text_bytes(model_len))
        .map_err(unavailable)?;
    let text: String = transaction
        .query_row(STORED_MODEL_SQL, params![project_id], |row| row.get(0))
        .optional()
        .map_err(sql)?
        .ok_or_else(not_found)?;
    drop(transaction);
    drop(connection);
    drop(sqlite);
    let not_json = |detail: String| {
        one(
            STORED_NOT_JSON,
            format!(
                "The stored document of {named} is not JSON ({detail}); no backup was written."
            ),
        )
    };
    let parse = parse_bytes(&text).map_err(not_json)?;
    let parsing = memory.reserve(&what, parse).map_err(unavailable)?;
    let document: Value =
        serde_json::from_str(&text).map_err(|error| not_json(stored_read::json_error(&error)))?;
    refuse_duplicate_members(memory, &named, &text, &document)?;
    drop(text);
    drop(load);
    if document.pointer("/project/id").and_then(Value::as_str) != Some(project_id) {
        return Err(one(
            PROJECT_MISMATCH,
            format!(
                "The stored document of {named} names another project id; no backup was written."
            ),
        ));
    }
    let canonical_len = stored_read::jcs_length(&document);
    if canonical_len > MAX_MODEL_DOCUMENT_BYTES {
        return Err(one(TOO_LARGE, format!("The canonical text of {what} is {canonical_len} bytes; a model document may be at most {MAX_MODEL_DOCUMENT_BYTES} bytes. No backup was written.")));
    }
    let canonical =
        stored_read::jcs_text(memory, &format!("the canonical text of {what}"), &document)
            .map_err(unavailable)?;
    let model_hex = sha256_hex(canonical.get());
    if model_hex != claim {
        return Err(one(
            STORED_HASH_MISMATCH,
            format!("The canonical text of {what} hashes to sha256:{model_hex}, not the sha256:{claim} its row records; no backup was written."),
        ));
    }
    let stem = backup_stem(&document);
    let document = open_path_checks(canonical_len, document, VERSION_NOT_CURRENT)
        .map_err(bounded_diagnostics)?;
    // The libraries the document references, read-only as the store above,
    // each within the command's memory with its screening's bound.
    let open = || {
        open_store_read_only(store, &named).map_err(|refusal| {
            refusal
                .into_iter()
                .next()
                .map(|item| item.message)
                .unwrap_or_default()
        })
    };
    let mut libraries = StoreLibraryReads::new(memory, open);
    libraries.screen_model(&document)?;
    let screened = screen_model_document_dispositions(
        &document,
        &mut |project_id: &str, library_kind: &str, library_id: &str| {
            libraries.read(project_id, library_kind, library_id)
        },
    );
    if let Some(refusal) = libraries.refusal() {
        return Err(bounded_diagnostics(refusal));
    }
    let warnings = screened
        .map(bounded_diagnostics)
        .map_err(bounded_diagnostics)?;
    drop(libraries);
    drop(document);
    drop(parsing);
    Ok(StoredBackup {
        canonical,
        model_hex,
        project_id: project_id.to_string(),
        stem,
        warnings,
    })
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub(crate) enum BackupWriteOutcome {
    Written {
        file_name: String,
        model_hash: String,
        durability: String,
        warnings: Vec<DispositionDiagnostic>,
    },
    Unchanged {
        file_name: String,
        model_hash: String,
        warnings: Vec<DispositionDiagnostic>,
    },
    /// A final entry was created, already existed, or its creation outcome is
    /// unknown. No verified completion is claimed; the app removed nothing.
    Unverified {
        file_name: String,
        model_hash: String,
        creation: &'static str,
        diagnostics: Vec<DispositionDiagnostic>,
    },
    Disabled,
    /// No new final copy was created by this request; retained temp files are
    /// named in the diagnostics when one was created.
    Refused {
        diagnostics: Vec<DispositionDiagnostic>,
    },
}
impl BackupWriteOutcome {
    fn refused(code: &str, message: impl Into<String>) -> Self {
        Self::Refused {
            diagnostics: one(code, message),
        }
    }
    fn bounded(self) -> Self {
        match self {
            Self::Written {
                file_name,
                model_hash,
                durability,
                warnings,
            } => Self::Written {
                file_name,
                model_hash,
                durability,
                warnings: bounded_diagnostics(warnings),
            },
            Self::Unchanged {
                file_name,
                model_hash,
                warnings,
            } => Self::Unchanged {
                file_name,
                model_hash,
                warnings: bounded_diagnostics(warnings),
            },
            Self::Unverified {
                file_name,
                model_hash,
                creation,
                diagnostics,
            } => Self::Unverified {
                file_name,
                model_hash,
                creation,
                diagnostics: bounded_diagnostics(diagnostics),
            },
            Self::Refused { diagnostics } => Self::Refused {
                diagnostics: bounded_diagnostics(diagnostics),
            },
            other => other,
        }
    }
}
fn temp_name() -> io::Result<String> {
    let mut token = [0_u8; 16];
    getrandom::fill(&mut token).map_err(|error| io::Error::other(error.to_string()))?;
    Ok(format!("{TEMP_PREFIX}{}.tmp", hex_of(token)))
}
fn verified_completion(
    dir: &BackupDir,
    file_name: &str,
    stored: &StoredBackup,
    created: bool,
    temp: Option<&str>,
    block: &mut [u8],
    io_ops: &mut impl BackupIo,
) -> BackupWriteOutcome {
    let hash = format!("sha256:{}", stored.model_hex);
    let kept = temp.map_or(String::new(), |temp| {
        format!(" Temporary file `{temp}` is also kept.")
    });
    let unverified = |code, detail: String| BackupWriteOutcome::Unverified {
        file_name: file_name.into(),
        model_hash: hash.clone(),
        creation: if created { "created" } else { "existing" },
        diagnostics: one(
            code,
            format!(
                "The {} copy `{file_name}` is not verified: {detail}. No file was removed.{kept}",
                if created { "created" } else { "existing" }
            ),
        ),
    };
    // The synced inode, its size and its change times. The post-sync read must
    // find this same unchanged file under the name: identical bytes in another
    // inode, or a rewrite in place, were not what this backup synced.
    let identity = |file: &File| {
        file.metadata().map(|m| {
            (
                m.dev(),
                m.ino(),
                m.len(),
                m.mtime(),
                m.mtime_nsec(),
                m.ctime(),
                m.ctime_nsec(),
            )
        })
    };
    let file = match dir.read_verified(file_name, &stored.model_hex, block) {
        Ok(file) => file,
        Err((code, detail)) => return unverified(code, detail),
    };
    let synced = match identity(&file) {
        Ok(synced) => synced,
        Err(error) => return unverified(READBACK_FAILED, error.to_string()),
    };
    if let Err(error) = io_ops.sync_final(&file) {
        return unverified(
            DURABILITY_UNVERIFIED,
            format!("final file sync failed ({error})"),
        );
    }
    if !io_ops.sync_directory(dir) {
        return unverified(
            DURABILITY_UNVERIFIED,
            "folder sync failed, timed out or was unavailable".into(),
        );
    }
    // Re-read after durability operations. This detects observed interference;
    // it is not a promise against later external mutation.
    let reread = match dir.read_verified(file_name, &stored.model_hex, block) {
        Ok(reread) => reread,
        Err((code, detail)) => return unverified(code, detail),
    };
    if !matches!(identity(&reread), Ok(now) if now == synced) {
        return unverified(
            READBACK_FAILED,
            "the name no longer refers to the synced, unchanged copy".into(),
        );
    }
    let mut warnings = stored.warnings.clone();
    if let Some(temp) = temp {
        warnings.push(diagnostic(TEMP_KEPT, "", format!("Temporary copy `{temp}` is retained. Automatic backups delete nothing; storage grows.")));
    }
    if created {
        BackupWriteOutcome::Written {
            file_name: file_name.into(),
            model_hash: hash,
            durability: "durable".into(),
            warnings,
        }
    } else {
        BackupWriteOutcome::Unchanged {
            file_name: file_name.into(),
            model_hash: hash,
            warnings,
        }
    }
}
/// Called only under the app's backup lock and the preadmitted completion
/// reservation. There is deliberately no delete or source-name movement.
fn write_backup_in_folder(
    folder: &Path,
    stored: &StoredBackup,
    unix_seconds: u64,
    memory: &dyn ReadMemory,
    io_ops: &mut impl BackupIo,
) -> BackupWriteOutcome {
    let Some(stamp) = utc_stamp(unix_seconds) else {
        return BackupWriteOutcome::refused(
            CLOCK_INVALID,
            "The clock cannot be represented in a backup name; no final copy was created.",
        );
    };
    if !folder.is_absolute() {
        return BackupWriteOutcome::refused(
            FOLDER_UNAVAILABLE,
            "The backup folder is not absolute; no final copy was created.",
        );
    }
    let dir = match BackupDir::open(folder) {
        Ok(dir) => dir,
        Err(error) => {
            return BackupWriteOutcome::refused(
                FOLDER_UNAVAILABLE,
                format!("The backup folder is unavailable ({error}); no final copy was created."),
            )
        }
    };
    if let Err(error) = io_ops.capability(&dir) {
        return BackupWriteOutcome::refused(PUBLISH_UNSUPPORTED, format!("The folder has no confirmed supported clone capability ({error}); no final or temporary copy was created."));
    }
    let _block = match memory.reserve(
        "backup read-back and bounded name scan",
        READBACK_BLOCK_BYTES as u64 + 4096,
    ) {
        Ok(held) => held,
        Err(error) => {
            return BackupWriteOutcome::refused(
                STORE_READ_MEMORY_UNAVAILABLE,
                stored_read::capped(error),
            )
        }
    };
    let mut block = vec![0_u8; READBACK_BLOCK_BYTES];
    let slot = backup_slot(&stored.project_id);
    let name = BackupName {
        stem: stored.stem.clone(),
        slot: slot.clone(),
        stamp,
        hash: stored.model_hex[..HASH_DIGITS].into(),
    }
    .file_name();
    match dir.newest(&slot) {
        Ok(Some(newest)) if newest.hash == stored.model_hex[..HASH_DIGITS] => {
            let existing = newest.file_name();
            if dir
                .read_verified(&existing, &stored.model_hex, &mut block)
                .is_ok()
            {
                return verified_completion(
                    &dir, &existing, stored, false, None, &mut block, io_ops,
                );
            }
        }
        Ok(_) => {}
        Err(error) => {
            return BackupWriteOutcome::refused(
                FOLDER_UNAVAILABLE,
                format!("The folder could not be listed ({error}); no final copy was created."),
            )
        }
    }
    let temp = match temp_name() {
        Ok(temp) => temp,
        Err(error) => return BackupWriteOutcome::refused(TEMP_CREATE_FAILED, error.to_string()),
    };
    let mut source = match dir.create_new(&temp) {
        Ok(file) => file,
        Err(error) => {
            return BackupWriteOutcome::refused(
                TEMP_CREATE_FAILED,
                format!(
                    "The temporary file could not be created ({error}); no final copy was created."
                ),
            )
        }
    };
    if let Err(error) = source.set_permissions(std::fs::Permissions::from_mode(FILE_MODE)) {
        return BackupWriteOutcome::refused(WRITE_FAILED, format!("Source permissions failed ({error}); `{temp}` is retained; no final copy was created."));
    }
    if let Err(error) = io_ops.write_all(&mut source, stored.canonical.get().as_bytes()) {
        return BackupWriteOutcome::refused(WRITE_FAILED, format!("Source write failed ({error}); partial `{temp}` is retained; no final copy was created."));
    }
    if let Err(error) = io_ops.sync_source(&source) {
        return BackupWriteOutcome::refused(
            FILE_SYNC_FAILED,
            format!(
                "Source sync failed ({error}); `{temp}` is retained; no final copy was created."
            ),
        );
    }
    if let Err(error) = io_ops.publish(&dir, &source, &name) {
        if error.kind() == io::ErrorKind::AlreadyExists {
            if dir
                .read_verified(&name, &stored.model_hex, &mut block)
                .is_ok()
            {
                return verified_completion(
                    &dir,
                    &name,
                    stored,
                    false,
                    Some(&temp),
                    &mut block,
                    io_ops,
                );
            }
            return BackupWriteOutcome::refused(NAME_IN_USE, format!("Existing `{name}` was left unchanged. No final copy was created; `{temp}` is retained."));
        }
        if error.kind() == io::ErrorKind::Unsupported
            || matches!(
                error.raw_os_error(),
                Some(libc::ENOTSUP) | Some(libc::EXDEV)
            )
        {
            return BackupWriteOutcome::refused(PUBLISH_UNSUPPORTED, format!("Atomic FD-source clone is unsupported ({error}); no fallback was attempted, no final copy was created, and `{temp}` is retained."));
        }
        return BackupWriteOutcome::Unverified { file_name: name, model_hash: format!("sha256:{}", stored.model_hex), creation: "unknown", diagnostics: one(PUBLISH_FAILED, format!("Clone failed ({error}); no verified copy is established. Any final entry and `{temp}` are kept; nothing was removed.")) };
    }
    // Keep the source FD alive through the clone and final verification. A
    // sibling substitution can never become the source of publication.
    verified_completion(&dir, &name, stored, true, Some(&temp), &mut block, io_ops)
}
#[derive(Debug, Clone)]
pub(crate) struct WriteRequest {
    pub(crate) project_id: String,
    pub(crate) expected_model_hash: String,
}
pub(crate) fn write_request(args: &mut Args) -> Result<WriteRequest, SolveJobError> {
    Ok(WriteRequest {
        project_id: args.decode("projectId")?,
        expected_model_hash: args.decode("expectedModelHash")?,
    })
}
/// Fixed conservative cover for bounded output diagnostics, their copies and
/// the completion buffer. This is held from before effects through IPC creation;
/// serialization never makes a new resource request after publication.
fn completion<T>(
    memory: &dyn ReadMemory,
    work: impl FnOnce() -> T,
    refused: impl FnOnce(String) -> T,
) -> Metered<T> {
    let bytes =
        stored_read::exact_text_bytes(MAX_RESPONSE_BYTES).saturating_add(2 * MAX_RESPONSE_BYTES);
    match memory.reserve("backup completion diagnostics and serialization", bytes) {
        Ok(held) => Metered::new(work()).holding(held),
        Err(error) => Metered::new(refused(stored_read::capped(error))),
    }
}
pub(crate) fn write_at(
    data: Result<PathBuf, String>,
    request: &WriteRequest,
    seconds: Option<u64>,
    memory: &dyn ReadMemory,
) -> Metered<BackupWriteOutcome> {
    write_at_with_io(data, request, seconds, memory, &mut SystemBackupIo)
}
pub(crate) fn write_at_with_io(
    data: Result<PathBuf, String>,
    request: &WriteRequest,
    seconds: Option<u64>,
    memory: &dyn ReadMemory,
    io_ops: &mut impl BackupIo,
) -> Metered<BackupWriteOutcome> {
    completion(
        memory,
        || write_body(data, request, seconds, memory, io_ops).bounded(),
        |_error| {
            BackupWriteOutcome::refused(
                STORE_READ_MEMORY_UNAVAILABLE,
                "The backup completion could not be admitted; no filesystem change was attempted.",
            )
        },
    )
}
fn write_body(
    data: Result<PathBuf, String>,
    request: &WriteRequest,
    seconds: Option<u64>,
    memory: &dyn ReadMemory,
    io_ops: &mut impl BackupIo,
) -> BackupWriteOutcome {
    let Some(expected) = full_hex(&request.expected_model_hash) else {
        return BackupWriteOutcome::refused(
            HASH_INVALID,
            "Expected full sha256 model hash is invalid; no final copy was created.",
        );
    };
    let data = match data {
        Ok(data) => data,
        Err(error) => {
            return BackupWriteOutcome::refused(FOLDER_UNAVAILABLE, stored_read::capped(error))
        }
    };
    if std::fs::symlink_metadata(&data).is_err_and(|error| error.kind() == io::ErrorKind::NotFound)
    {
        return BackupWriteOutcome::Disabled;
    }
    #[cfg(test)]
    BEFORE_BACKUP_LOCK.with(|hook| {
        if let Some(hook) = hook.borrow_mut().take() {
            hook();
        }
    });
    let _lock = match autosave::lock_slot_at(&data, BACKUP_LOCK_SLOT, LOCK_WAIT) {
        Ok(lock) => lock,
        Err(refusal) => {
            return BackupWriteOutcome::Refused {
                diagnostics: vec![refusal],
            }
        }
    };
    let settings = crate::autosave_commands::read_settings_record(&data, memory);
    if let Some(status) = &settings.get().status {
        return BackupWriteOutcome::Refused {
            diagnostics: vec![status.clone()],
        };
    }
    let record = settings.get().record.clone();
    drop(settings);
    let Some(folder) = record.backup_folder_in_force() else {
        return BackupWriteOutcome::Disabled;
    };
    if record.backup_keep.is_some() {
        return BackupWriteOutcome::refused(ROTATION_UNAVAILABLE, "Stored finite retention is unavailable. No setting was rewritten and no backup or deletion was attempted. Choose All explicitly to enable nondeleting backups.");
    }
    let Some(seconds) = seconds else {
        return BackupWriteOutcome::refused(
            CLOCK_INVALID,
            "The clock is before 1970; no final copy was created.",
        );
    };
    // 512 KiB command memory + 1 MiB initial SQLite working space, admitted
    // before opening the store. Referenced libraries reserve their own reads.
    let _own = match memory.reserve(
        "backup initial store and command memory",
        LIBRARY_STORE_BYTES,
    ) {
        Ok(held) => held,
        Err(error) => {
            return BackupWriteOutcome::refused(
                STORE_READ_MEMORY_UNAVAILABLE,
                stored_read::capped(error),
            )
        }
    };
    let stored = match read_stored_backup(
        &data.join(crate::PROJECT_STORE_FILE),
        &request.project_id,
        expected,
        memory,
    ) {
        Ok(stored) => stored,
        Err(diagnostics) => return BackupWriteOutcome::Refused { diagnostics },
    };
    write_backup_in_folder(Path::new(folder), &stored, seconds, memory, io_ops)
}
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
    Saved {
        settings: AutosaveSettings,
    },
    /// The folder chooser was cancelled; nothing was written.
    Cancelled,
    Refused {
        diagnostics: Vec<DispositionDiagnostic>,
    },
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
fn admitted_folder(
    folder: &Path,
    capability: impl FnOnce(&BackupDir) -> io::Result<()>,
) -> Result<String, Vec<DispositionDiagnostic>> {
    let invalid = |why: &str| {
        one(
            FOLDER_INVALID,
            format!("The chosen backup folder {why}; nothing was changed."),
        )
    };
    if !folder.is_absolute() {
        return Err(invalid("is not an absolute path"));
    }
    let Some(text) = folder.to_str() else {
        return Err(invalid("is not a UTF-8 path"));
    };
    let dir = BackupDir::open(folder).map_err(|error| {
        invalid(&format!(
            "cannot be opened as a folder that is not a symbolic link ({error})"
        ))
    })?;
    capability(&dir).map_err(|error| one(PUBLISH_UNSUPPORTED, format!("The selected folder's volume has no confirmed supported clone capability ({error}); backups were not enabled and settings were not changed.")))?;
    Ok(text.to_string())
}

/// Lock order: backup -> settings. Configuration acknowledges only after
/// every earlier backup's effects finish and its new record has been written.
/// Interval-only updates acquire just the settings lock and never the backup
/// lock, so they cannot invert that order.
fn update_backup_settings_at(
    data: Result<PathBuf, String>,
    memory: &dyn ReadMemory,
    change: impl FnOnce(SettingsRecord) -> Result<SettingsRecord, Vec<DispositionDiagnostic>>,
) -> BackupConfigureOutcome {
    let data = match data.and_then(|data| {
        std::fs::create_dir_all(&data)
            .map(|()| data)
            .map_err(|error| error.to_string())
    }) {
        Ok(data) => data,
        Err(error) => {
            return BackupConfigureOutcome::Refused {
                diagnostics: one(
                    autosave::DIRECTORY_UNAVAILABLE,
                    format!(
                    "The app's local data directory is unavailable: {error}. Nothing was changed."
                ),
                ),
            }
        }
    };
    let _lock = match autosave::lock_slot_at(&data, BACKUP_LOCK_SLOT, LOCK_WAIT) {
        Ok(lock) => lock,
        Err(refusal) => {
            return BackupConfigureOutcome::Refused {
                diagnostics: vec![refusal],
            }
        }
    };
    update_settings_at(Ok(data), memory, change).into()
}

/// Turns backups on or off. Off clears the folder and the intent and
/// deletes nothing. On needs the explicit local-private intent before the
/// chooser (`choose`) is shown; a cancelled chooser writes nothing. The
/// folder, the rotation and the intent are written together under the
/// settings lock, keeping the interval another window set.
fn configure_with_capability(
    data: Result<PathBuf, String>,
    request: &ConfigureRequest,
    choose: impl FnOnce() -> Result<Option<PathBuf>, String>,
    memory: &dyn ReadMemory,
    capability: impl FnOnce(&BackupDir) -> io::Result<()>,
) -> BackupConfigureOutcome {
    if request.enabled && request.backup_keep.is_some() {
        return BackupConfigureOutcome::Refused {
            diagnostics: one(ROTATION_UNAVAILABLE, "Automatic rotation is unfinished. Only All (delete nothing) can be enabled; settings were not changed."),
        };
    }
    if !request.enabled {
        return update_backup_settings_at(data, memory, |record| {
            Ok(SettingsRecord {
                backup_folder: None,
                backup_local_private_intent: false,
                ..record
            })
        });
    }
    if !request.explicit_local_private_intent {
        return BackupConfigureOutcome::Refused {
            diagnostics: one(INTENT_REQUIRED, "Backups copy the project's private data to a folder you choose; that needs the explicit choice to do so. Nothing was changed."),
        };
    }
    let folder = match choose() {
        Ok(Some(folder)) => folder,
        Ok(None) => return BackupConfigureOutcome::Cancelled,
        Err(error) => {
            return BackupConfigureOutcome::Refused {
                diagnostics: one(
                    FOLDER_INVALID,
                    format!(
                        "The folder chooser failed ({}); nothing was changed.",
                        stored_read::capped(error)
                    ),
                ),
            }
        }
    };
    let folder = match admitted_folder(&folder, capability) {
        Ok(folder) => folder,
        Err(diagnostics) => return BackupConfigureOutcome::Refused { diagnostics },
    };
    update_backup_settings_at(data, memory, |record| {
        Ok(SettingsRecord {
            backup_folder: Some(folder),
            backup_keep: None,
            backup_local_private_intent: true,
            ..record
        })
    })
}

pub(crate) fn configure_at(
    data: Result<PathBuf, String>,
    request: &ConfigureRequest,
    choose: impl FnOnce() -> Result<Option<PathBuf>, String>,
    memory: &dyn ReadMemory,
) -> Metered<BackupConfigureOutcome> {
    completion(
        memory,
        || {
            configure_with_capability(data, request, choose, memory, |dir| {
                dir.require_clone_capability()
            })
        },
        |_error| BackupConfigureOutcome::Refused {
            diagnostics: one(
                STORE_READ_MEMORY_UNAVAILABLE,
                "The backup completion could not be admitted; no setting was changed.",
            ),
        },
    )
}

pub(crate) fn admit_body(
    registry: &SolveJobRegistry,
    body: &tauri::ipc::InvokeBody,
    headers: &tauri::http::HeaderMap,
    command: &'static str,
) -> Result<(Args, crate::memory_pool::Lease), SolveJobError> {
    let max = if command == WRITE_COMMAND {
        WRITE_COMMAND_MAX_ARGS_BYTES
    } else {
        CONFIGURE_COMMAND_MAX_ARGS_BYTES
    };
    command_ingress::admit_body_within(registry, body, headers, command, ArgsDecoding::Moved, max)
}
fn choose_backup_folder(app: &AppHandle) -> Result<Option<PathBuf>, String> {
    #[cfg(feature = "native-smoke")]
    if let Some(path) = crate::native_smoke::take_chosen_path(app, "choose_backup_folder") {
        return Ok(Some(path));
    }
    let mut chooser = app.dialog().file();
    if let Some(directory) =
        default_save_directory(app.path().document_dir().ok(), app.path().home_dir().ok())
    {
        chooser = chooser.set_directory(directory);
    }
    chooser
        .blocking_pick_folder()
        .map(|path| path.into_path().map_err(|error| error.to_string()))
        .transpose()
}
/// No post-effect pool admission: the value's completion reservation already
/// covers the bounded diagnostic objects, serialization buffer and its copy.
fn response<T: Serialize>(outcome: Metered<T>) -> Result<tauri::ipc::Response, CommandError> {
    let length = stored_read::serialized_length(outcome.get()).map_err(CommandError::Response)?;
    if length > MAX_RESPONSE_BYTES {
        return Err(CommandError::Response("BACKUP-OUTCOME-UNKNOWN: bounded completion exceeded its admitted capacity; filesystem outcome must not be inferred".into()));
    }
    let mut buffer = Vec::with_capacity(length as usize);
    serde_json::to_writer(&mut buffer, outcome.get())
        .map_err(|error| CommandError::Response(format!("BACKUP-OUTCOME-UNKNOWN: {error}")))?;
    let text =
        String::from_utf8(buffer).map_err(|error| CommandError::Response(error.to_string()))?;
    let response = tauri::ipc::Response::new(text);
    drop(outcome);
    Ok(response)
}
async fn worker<T: Serialize + Send + 'static>(
    app: AppHandle,
    command: &'static str,
    lease: crate::memory_pool::Lease,
    work: impl FnOnce(&AppHandle, &dyn ReadMemory) -> Metered<T> + Send + 'static,
) -> Result<tauri::ipc::Response, CommandError> {
    tauri::async_runtime::spawn_blocking(move || {
        let registry = app.state::<SolveJobRegistry>();
        let memory = seeded_memory(&registry, &lease);
        let response = response(work(&app, &memory));
        drop(memory); drop(lease); response
    }).await.unwrap_or_else(|_| Err(CommandError::Command(format!("BACKUP-OUTCOME-UNKNOWN: {command} worker did not complete; no filesystem outcome can be inferred"))))
}
#[tauri::command]
pub async fn configure_project_backup(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, lease) = admit_body(
        &registry,
        invoke.body(),
        invoke.headers(),
        CONFIGURE_COMMAND,
    )?;
    let request = configure_request(&mut args)?;
    drop(args);
    worker(app, CONFIGURE_COMMAND, lease, move |app, memory| {
        configure_at(
            data_directory(app),
            &request,
            || choose_backup_folder(app),
            memory,
        )
    })
    .await
}
#[tauri::command]
pub async fn write_project_backup(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, lease) = admit_body(&registry, invoke.body(), invoke.headers(), WRITE_COMMAND)?;
    let request = write_request(&mut args)?;
    drop(args);
    worker(app, WRITE_COMMAND, lease, move |app, memory| {
        write_at(data_directory(app), &request, now_unix_seconds(), memory)
    })
    .await
}
#[cfg(test)]
#[path = "backup_tests.rs"]
mod tests;
