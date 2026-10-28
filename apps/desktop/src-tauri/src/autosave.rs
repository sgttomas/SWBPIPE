//! Autosave recovery records (D5; DEL-02-05 Design `autosave-and-backup.md`
//! §1 and §5, slice 1).
//!
//! A record is one file in the app's `autosave/` directory, the store file's
//! sibling. Its bytes are exactly the session's model document as unchecked
//! RFC 8785 (JCS) canonical text, so its SHA-256 is the session's model hash.
//! Its name `<slot>-<written unix millis, 13 digits>-<model hash hex>.json`
//! carries its identity: `slot` is the first 32 hex digits of the SHA-256 of
//! the document's `project.id`, so no project name or id is in a name, and the
//! hash lets every read check the bytes with no second file. The slot is a
//! consistency check, not authentication: anything that can write the app's
//! data directory can write a record, so recovery is always an explicit offer.
//!
//! Before anything is written, the text must be within the open cap, hash to
//! the claim, be its own canonical form and pass the open path's checks
//! (`model_document_save::open_path_checks`), so every record re-opens. Then,
//! holding the slot's exclusive lock (`<slot>.lock`, `flock` through
//! `File::try_lock`, across threads, windows and processes), the transaction
//! checks the slot's newest record, writes the new one with the
//! extension-neutral atomic writer (temporary file, flush, fsync, rename,
//! bounded directory sync), reads it back through SHA-256 in fixed blocks and,
//! only when the write is durable, removes the slot's records strictly older
//! than it. A failure at any step leaves the previous record in place and is
//! reported by name; the store is never opened. Nothing here approves,
//! accepts or issues anything: a record is a recovery copy of the user's model.
//!
//! Every read is contained: the directory is opened `O_DIRECTORY |
//! O_NOFOLLOW`, each name is resolved once relative to that handle, files are
//! opened `O_NOFOLLOW | O_NONBLOCK` and checked by `fstat` against what was
//! inspected (a regular file, the same device and inode, the same size), and
//! read with a hard cap at that size. Directory enumeration keeps O(1) state
//! for writes and pruning, and at most [`MAX_LISTED_AUTOSAVES`] reserved
//! entries for a listing.
//!
//! Memory: each function takes the command's `ReadMemory` and reserves the
//! text's parse, its canonical copy, a file's load, the read-back block and a
//! listing's entries before it makes them (DEL-04-07 "Model-sized interactive
//! commands").

// Slice 2 wires these functions into native commands; until then only the
// tests call them.
#![cfg_attr(not(test), allow(dead_code))]

use crate::atomic_report_package_save::{
    atomic_save_bytes_as_with_io, AtomicSaveFormat, AtomicSaveIo, DirectorySyncResult, SystemAtomicSaveIo,
};
use crate::export_disposition_screen::{diagnostic, DispositionDiagnostic};
use crate::model_document_file::MAX_MODEL_DOCUMENT_BYTES;
use crate::model_document_save::{open_path_checks, writer_diagnostic};
use crate::stored_read::{is_jcs_text, loaded_text_bytes, parse_bytes, Metered, ReadMemory, ReadReservation, STORE_READ_MEMORY_UNAVAILABLE};
use open_pipe_stress_operation_applier::sha256_hex;
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::{self, File, TryLockError};
use std::io::{self, Read};
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::time::{Duration, Instant};

/// The records' directory, beside the store file in the app's local data directory.
pub(crate) const AUTOSAVE_DIRECTORY: &str = "autosave";

/// The atomic writer's admission of a record: `.json`, `AUTOSAVE-*` codes, and
/// its own temporary-file prefix, which no listing or read ever parses.
pub(crate) const AUTOSAVE_FORMAT: AtomicSaveFormat = AtomicSaveFormat {
    extension: "json",
    code_prefix: "AUTOSAVE",
    temp_prefix: ".swbpipe-autosave-",
    log_label: "autosave",
};

pub(crate) const HASH_MISMATCH: &str = "AUTOSAVE-HASH-MISMATCH";
pub(crate) const TOO_LARGE: &str = "AUTOSAVE-TOO-LARGE";
pub(crate) const VERSION_NOT_CURRENT: &str = "AUTOSAVE-VERSION-NOT-CURRENT";
pub(crate) const DIRECTORY_UNAVAILABLE: &str = "AUTOSAVE-DIRECTORY-UNAVAILABLE";
pub(crate) const CLOCK_INVALID: &str = "AUTOSAVE-CLOCK-INVALID";
pub(crate) const LOCK_BUSY: &str = "AUTOSAVE-LOCK-BUSY";
pub(crate) const LOCK_FAILED: &str = "AUTOSAVE-LOCK-FAILED";
pub(crate) const READBACK_MISMATCH: &str = "AUTOSAVE-READBACK-MISMATCH";
pub(crate) const READBACK_FAILED: &str = "AUTOSAVE-READBACK-FAILED";
pub(crate) const PRUNE_FAILED: &str = "AUTOSAVE-PRUNE-FAILED";
pub(crate) const PRUNE_DEFERRED: &str = "AUTOSAVE-PRUNE-DEFERRED";
pub(crate) const NAME_INVALID: &str = "AUTOSAVE-NAME-INVALID";
pub(crate) const NAME_IN_USE: &str = "AUTOSAVE-NAME-IN-USE";
pub(crate) const VERIFY_UNAVAILABLE: &str = "AUTOSAVE-VERIFY-UNAVAILABLE";
pub(crate) const NOT_FOUND: &str = "AUTOSAVE-NOT-FOUND";
pub(crate) const UNREADABLE: &str = "AUTOSAVE-UNREADABLE";
pub(crate) const INTEGRITY: &str = "AUTOSAVE-INTEGRITY";

/// The read-back's block: the only buffer the hash of a written file needs.
pub(crate) const READBACK_BLOCK_BYTES: usize = 64 * 1024;
/// The most records one listing names; the rest are counted.
pub(crate) const MAX_LISTED_AUTOSAVES: usize = 1024;
/// One retained listing entry: its heap slot and name, and the entry built
/// from it (three strings of at most 64 bytes and two integers), with margin.
pub(crate) const LISTED_ENTRY_BYTES: u64 = 512;
/// How long a write waits for its slot's lock before it is refused.
pub(crate) const LOCK_WAIT: Duration = Duration::from_secs(5);
const LOCK_POLL: Duration = Duration::from_millis(10);
const LOCK_CREATE_RETRIES: u32 = 16;
/// How often a lock whose file was replaced while it was taken is re-opened.
const LOCK_REOPEN_RETRIES: u32 = 8;
/// The most pruning warnings named; the rest are counted in one more.
const MAX_PRUNE_WARNINGS: usize = 16;

const SLOT_HEX_DIGITS: usize = 32;
const MILLIS_DIGITS: usize = 13;
const HASH_HEX_DIGITS: usize = 64;

/// The slot of a project id: the first 32 hex digits of its UTF-8 SHA-256.
pub(crate) fn autosave_slot(project_id: &str) -> String {
    let mut slot = sha256_hex(project_id);
    slot.truncate(SLOT_HEX_DIGITS);
    slot
}

fn is_lower_hex(text: &str, digits: usize) -> bool {
    text.len() == digits && text.bytes().all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
}

/// A record's identity, as its file name carries it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AutosaveName {
    pub(crate) slot: String,
    pub(crate) written_unix_millis: u64,
    /// The model hash, bare lowercase hex.
    pub(crate) model_hash_hex: String,
}

impl AutosaveName {
    pub(crate) fn file_name(&self) -> String {
        format!("{}-{:013}-{}.json", self.slot, self.written_unix_millis, self.model_hash_hex)
    }

    /// The name of a record, or `None` for any other name (a temporary file, a
    /// lock file, a path with a separator, another file).
    pub(crate) fn parse(file_name: &str) -> Option<Self> {
        let stem = file_name.strip_suffix(".json")?;
        let mut parts = stem.split('-');
        let (slot, millis, hash) = (parts.next()?, parts.next()?, parts.next()?);
        if parts.next().is_some()
            || !is_lower_hex(slot, SLOT_HEX_DIGITS)
            || millis.len() != MILLIS_DIGITS
            || !millis.bytes().all(|byte| byte.is_ascii_digit())
            || !is_lower_hex(hash, HASH_HEX_DIGITS)
        {
            return None;
        }
        Some(Self { slot: slot.to_string(), written_unix_millis: millis.parse().ok()?, model_hash_hex: hash.to_string() })
    }

    /// The order of records: by written time, then by name.
    fn key(&self) -> (u64, String) {
        (self.written_unix_millis, self.file_name())
    }
}

// ---------------------------------------------------------------------------
// The contained directory handle.
// ---------------------------------------------------------------------------

const DIR_FLAGS: libc::c_int = libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_NONBLOCK | libc::O_CLOEXEC;
const FILE_READ_FLAGS: libc::c_int = libc::O_RDONLY | libc::O_NOFOLLOW | libc::O_NONBLOCK | libc::O_CLOEXEC;
const LOCK_FILE_FLAGS: libc::c_int = libc::O_RDWR | libc::O_CREAT | libc::O_NOFOLLOW | libc::O_NONBLOCK | libc::O_CLOEXEC;

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

/// The `autosave/` directory, opened `O_DIRECTORY | O_NOFOLLOW` (a symlinked
/// directory is refused). Every name below is one component resolved relative
/// to this handle, and no symlink is followed.
struct RecordDir(OwnedFd);

/// A record entry as inspected (`fstatat`, no symlink followed).
#[derive(Clone, Copy)]
struct Inspected {
    device: u64,
    inode: u64,
    len: u64,
}

/// Why a contained open did not yield the inspected regular file.
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

impl Access {
    fn describe(&self, file_name: &str) -> (String, &'static str) {
        match self {
            Access::NotFound => (format!("The autosave `{file_name}` is no longer there."), NOT_FOUND),
            Access::NotRegular => (format!("The autosave `{file_name}` is not a regular file."), UNREADABLE),
            Access::TooLarge(len) => (format!("The autosave is {len} bytes; a model document may be at most {MAX_MODEL_DOCUMENT_BYTES} bytes."), TOO_LARGE),
            Access::Changed(what) => (format!("The autosave `{file_name}` {what}."), UNREADABLE),
            Access::Io(error) => (format!("The autosave `{file_name}` could not be read: {error}."), UNREADABLE),
        }
    }
}

#[cfg(test)]
thread_local! {
    /// Runs once between a record's inspection and its open (the swap and
    /// growth tests).
    static BEFORE_OPEN: std::cell::RefCell<Option<Box<dyn FnOnce()>>> = std::cell::RefCell::new(None);
    /// Runs once between a write's read-back and its prune, while the slot's
    /// lock is held (the forced-interleaving test).
    static BEFORE_PRUNE: std::cell::RefCell<Option<Box<dyn FnOnce()>>> = std::cell::RefCell::new(None);
}

impl RecordDir {
    fn open(path: &Path) -> io::Result<Self> {
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
        Ok(Inspected { device: stat.st_dev as u64, inode: stat.st_ino as u64, len })
    }

    /// Opens `name` as inspected: `O_NOFOLLOW | O_NONBLOCK` (a symlink is
    /// refused, a FIFO never blocks), and `fstat` on the descriptor must show a
    /// regular file with the inspected device, inode and size.
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
        if (metadata.dev(), metadata.ino()) != (inspected.device, inspected.inode) {
            return Err(Access::Changed("was replaced after it was inspected"));
        }
        if metadata.len() != inspected.len {
            return Err(Access::Changed("changed size after it was inspected"));
        }
        Ok(file)
    }

    /// The slot's lock file `<slot>.lock`, created if absent, never through a link.
    /// Two openers creating it at once can see `ENOENT` from `O_CREAT` on
    /// macOS (observed by the two-writer test); the open is then retried, a
    /// bounded number of times, while the directory handle stays valid.
    fn open_lock(&self, slot: &str) -> io::Result<File> {
        let c = c_name(&format!("{slot}.lock"))?;
        let mut attempts = 0;
        loop {
            // SAFETY: as above; mode 0600.
            match check(unsafe { libc::openat(self.fd(), c.as_ptr(), LOCK_FILE_FLAGS, 0o600 as libc::c_uint) }) {
                Ok(fd) => return Ok(unsafe { File::from_raw_fd(fd) }),
                Err(error) if error.kind() == io::ErrorKind::NotFound && attempts < LOCK_CREATE_RETRIES => {
                    attempts += 1;
                    std::thread::yield_now();
                }
                Err(error) => return Err(error),
            }
        }
    }

    fn unlink(&self, name: &str) -> io::Result<()> {
        let c = c_name(name)?;
        // SAFETY: as above; `unlinkat` never follows a link.
        check(unsafe { libc::unlinkat(self.fd(), c.as_ptr(), 0) }).map(|_| ())
    }

    /// Calls `each` with every record entry: a name that parses as a record and
    /// a regular file (inspected without following a link), with its length.
    /// The enumeration holds one entry at a time.
    fn each_record(&self, mut each: impl FnMut(AutosaveName, u64)) -> io::Result<()> {
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
        loop {
            let entry = unsafe { libc::readdir(stream.0) };
            if entry.is_null() {
                return Ok(());
            }
            let name = unsafe { std::ffi::CStr::from_ptr((*entry).d_name.as_ptr()) };
            let Some(name) = name.to_str().ok().and_then(AutosaveName::parse) else {
                continue;
            };
            match self.stat_child(&name.file_name()) {
                Ok(stat) if is_regular(&stat) => each(name, stat.st_size.max(0) as u64),
                Ok(_) => {}
                // Removed since it was listed.
                Err(error) if error.kind() == io::ErrorKind::NotFound => {}
                Err(error) => return Err(error),
            }
        }
    }
}

/// Reads an opened record of `len` bytes with a hard cap at `len`: the buffer
/// is allocated once at `len + 1`, and a file that yields more or fewer bytes
/// is refused.
fn read_capped(file: &File, len: u64) -> Result<Vec<u8>, Access> {
    let bytes = read_at_most(file, len)?;
    if file.metadata().map_err(Access::Io)?.len() != len {
        return Err(Access::Changed("changed while it was read"));
    }
    Ok(bytes)
}

/// The capped read itself: at most `len + 1` bytes are taken from `reader`.
fn read_at_most(reader: impl Read, len: u64) -> Result<Vec<u8>, Access> {
    let capacity = usize::try_from(len.saturating_add(1)).map_err(|_| Access::TooLarge(len))?;
    let mut bytes = Vec::with_capacity(capacity);
    reader.take(len.saturating_add(1)).read_to_end(&mut bytes).map_err(Access::Io)?;
    if bytes.len() as u64 != len {
        return Err(Access::Changed("changed while it was read"));
    }
    Ok(bytes)
}

/// SHA-256 (bare hex) of an opened record of `len` bytes, streamed through one
/// block of [`READBACK_BLOCK_BYTES`] with a hard cap at `len`.
fn stream_sha256_hex(file: &File, len: u64) -> Result<String, Access> {
    let mut hasher = Sha256::new();
    let mut block = vec![0_u8; READBACK_BLOCK_BYTES];
    let mut reader = file.take(len.saturating_add(1));
    let mut total = 0_u64;
    loop {
        let count = reader.read(&mut block).map_err(Access::Io)?;
        if count == 0 {
            break;
        }
        total += count as u64;
        hasher.update(&block[..count]);
    }
    if total != len {
        return Err(Access::Changed("changed while it was read"));
    }
    Ok(hasher.finalize().iter().map(|byte| format!("{byte:02x}")).collect())
}

/// What a check of a record against its hash established.
#[derive(Debug, PartialEq, Eq)]
enum Verified {
    /// It reads back as the hash.
    Matches,
    /// It verifiably does not: its bytes hash otherwise, or it exceeds the
    /// cap. Only such a record may be replaced or removed as corrupt.
    Differs(String),
    /// It could not be checked: a reservation refused, a permission or I/O
    /// error, an entry that vanished, changed or is not a regular file. The
    /// record is then never rewritten or removed.
    Unavailable(String),
}

/// Checks the record `name` against `expected_hex`, contained, within `memory`.
fn verify_record(dir: &RecordDir, name: &str, expected_hex: &str, memory: &dyn ReadMemory) -> Verified {
    let _block: ReadReservation = match memory.reserve("the autosave read-back block", READBACK_BLOCK_BYTES as u64) {
        Ok(block) => block,
        Err(message) => return Verified::Unavailable(message),
    };
    let hashed = dir.inspect(name, MAX_MODEL_DOCUMENT_BYTES).and_then(|inspected| {
        let file = dir.open_inspected(name, inspected)?;
        stream_sha256_hex(&file, inspected.len)
    });
    match hashed {
        Ok(hex) if hex == expected_hex => Verified::Matches,
        Ok(hex) => Verified::Differs(format!("it reads back as sha256:{hex}, not sha256:{expected_hex}")),
        Err(Access::TooLarge(len)) => Verified::Differs(format!("it is {len} bytes, above the open cap")),
        Err(access) => Verified::Unavailable(access.describe(name).0),
    }
}

// ---------------------------------------------------------------------------
// The slot lock.
// ---------------------------------------------------------------------------

/// A slot's exclusive lock, held until dropped. It is `flock(LOCK_EX)` (std
/// `File::try_lock`) on `<slot>.lock` in the records' directory: an advisory
/// lock of the open file description, so two threads that each open the file,
/// two windows and two processes exclude each other, while different slots
/// never contend. The kernel releases it when the file closes, also when the
/// process ends or crashes. The lock file is never removed (removing it would
/// let a later opener lock a different file).
pub(crate) struct SlotLock {
    _file: File,
}

/// The lock file must be a regular file (`fstat` on the opened descriptor),
/// and, once locked, its name must still name that file (`fstatat`): a lock
/// taken on a file that was replaced meanwhile is dropped and the new file is
/// opened again, at most [`LOCK_REOPEN_RETRIES`] times. Residual: the lock
/// coordinates SWBPIPE's own writers. Something that can rewrite the app's
/// data directory can still split it, as it can write records, which is why
/// every record is verified and recovery is an explicit offer.
fn lock_slot(dir: &RecordDir, slot: &str, wait: Duration) -> Result<SlotLock, DispositionDiagnostic> {
    let failed = |detail: String| diagnostic(LOCK_FAILED, "", format!("The autosave lock {detail}. Nothing was written."));
    let lock_name = format!("{slot}.lock");
    let deadline = Instant::now() + wait;
    let mut reopened = 0;
    loop {
        let file = dir.open_lock(slot).map_err(|error| failed(format!("could not be opened: {error}")))?;
        let opened = file.metadata().map_err(|error| failed(format!("could not be inspected: {error}")))?;
        if !opened.is_file() {
            return Err(failed("is not a regular file".to_string()));
        }
        loop {
            match file.try_lock() {
                Ok(()) => break,
                Err(TryLockError::WouldBlock) if Instant::now() < deadline => std::thread::sleep(LOCK_POLL),
                Err(TryLockError::WouldBlock) => {
                    return Err(diagnostic(
                        LOCK_BUSY,
                        "",
                        format!("Another autosave of this project held its lock for over {} ms. Nothing was written.", wait.as_millis()),
                    ))
                }
                Err(TryLockError::Error(error)) => return Err(failed(format!("could not be taken: {error}"))),
            }
        }
        match dir.stat_child(&lock_name) {
            Ok(stat) if (stat.st_dev as u64, stat.st_ino as u64) == (opened.dev(), opened.ino()) => return Ok(SlotLock { _file: file }),
            _ if reopened < LOCK_REOPEN_RETRIES => reopened += 1,
            _ => return Err(failed("file was replaced while it was taken".to_string())),
        }
    }
}

/// The lock of `slot`'s records in `directory` (an existing records directory).
pub(crate) fn lock_slot_at(directory: &Path, slot: &str, wait: Duration) -> Result<SlotLock, DispositionDiagnostic> {
    let dir = RecordDir::open(directory)
        .map_err(|error| diagnostic(DIRECTORY_UNAVAILABLE, "", format!("The autosave directory is unavailable: {error}.")))?;
    lock_slot(&dir, slot, wait)
}

// ---------------------------------------------------------------------------
// Records.
// ---------------------------------------------------------------------------

/// One record a listing names (no content is read).
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub(crate) struct AutosaveEntry {
    pub(crate) file_name: String,
    pub(crate) slot: String,
    pub(crate) written_unix_millis: u64,
    /// `sha256:<hex>`.
    pub(crate) model_hash: String,
    pub(crate) byte_count: u64,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub(crate) struct AutosaveListing {
    /// Newest first.
    pub(crate) entries: Vec<AutosaveEntry>,
    /// Records beyond [`MAX_LISTED_AUTOSAVES`], counted and not named.
    pub(crate) not_listed: usize,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub(crate) enum AutosaveWriteOutcome {
    Written {
        file_name: String,
        slot: String,
        written_unix_millis: u64,
        /// `sha256:<hex>` of the bytes written and read back: the model hash.
        model_hash: String,
        byte_count: u64,
        /// `durable`, or `saved_durability_uncertain` when a directory sync
        /// (after the rename, or after creating the directory) did not
        /// complete; the older records are then kept.
        durability: String,
        /// Older records kept (`AUTOSAVE-PRUNE-DEFERRED`) or not removed
        /// (`AUTOSAVE-PRUNE-FAILED`).
        warnings: Vec<DispositionDiagnostic>,
    },
    /// The slot's newest record holds this model and reads back as its hash;
    /// nothing was written.
    Unchanged { file_name: String, slot: String, model_hash: String },
    Refused { diagnostics: Vec<DispositionDiagnostic> },
}

impl AutosaveWriteOutcome {
    fn refused_one(code: &str, message: impl Into<String>) -> Self {
        Self::Refused { diagnostics: vec![diagnostic(code, "", message)] }
    }
}

/// A record read for recovery, after every check of §5.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub(crate) struct VerifiedAutosave {
    pub(crate) file_name: String,
    pub(crate) slot: String,
    pub(crate) written_unix_millis: u64,
    pub(crate) model_hash: String,
    pub(crate) byte_count: u64,
    pub(crate) document: Value,
}

fn memory_refusal(message: String) -> Vec<DispositionDiagnostic> {
    vec![diagnostic(STORE_READ_MEMORY_UNAVAILABLE, "", message)]
}

/// A model document text admitted as a record: its parsed document (under its
/// reservations), its slot and its hash.
struct AdmittedText {
    document: Metered<Value>,
    slot: String,
    model_hash_hex: String,
}

/// The content checks of a record's text against a claimed hash (bare hex):
/// the open cap, the hash, the canonical form (each parse and copy reserved
/// first) and the open path's checks. `mismatch` names a text that is not the
/// canonical text of the claimed hash.
fn admit_text(text: &str, claimed_hex: &str, mismatch: &str, memory: &dyn ReadMemory) -> Result<AdmittedText, Vec<DispositionDiagnostic>> {
    let byte_count = text.len() as u64;
    if byte_count > MAX_MODEL_DOCUMENT_BYTES {
        return Err(vec![diagnostic(
            TOO_LARGE,
            "",
            format!("The model document is {byte_count} bytes; a model document may be at most {MAX_MODEL_DOCUMENT_BYTES} bytes, so it would not re-open."),
        )]);
    }
    let hex = sha256_hex(text);
    if hex != claimed_hex {
        return Err(vec![diagnostic(mismatch, "", format!("The text's SHA-256 is sha256:{hex}, not sha256:{claimed_hex}."))]);
    }
    let parse = parse_bytes(text)
        .map_err(|detail| vec![diagnostic(mismatch, "", format!("The text is not one JSON value: {detail}."))])?;
    let reservation = memory.reserve("the autosave's model document", parse).map_err(memory_refusal)?;
    let document: Value = serde_json::from_str(text)
        .map_err(|error| vec![diagnostic(mismatch, "", format!("The text is not JSON ({}).", crate::stored_read::json_error(&error)))])?;
    if !is_jcs_text(memory, "the autosave's canonical text", &document, text).map_err(memory_refusal)? {
        return Err(vec![diagnostic(mismatch, "", "The text is not its own RFC 8785 (JCS) canonical form.")]);
    }
    let slot = autosave_slot(&crate::project_id_from_model(&document));
    let document = open_path_checks(byte_count, document, VERSION_NOT_CURRENT)?;
    Ok(AdmittedText { document: Metered::new(document).holding(reservation), slot, model_hash_hex: hex })
}

/// The directory exists as a real directory (created here if absent, under an
/// existing parent). Its durability is never inferred from its existence: every
/// write syncs the parent before it may prune (`write_autosave_with_io`).
fn ensure_directory(directory: &Path) -> Result<(), DispositionDiagnostic> {
    let unavailable = |detail: String| diagnostic(DIRECTORY_UNAVAILABLE, "", format!("The autosave directory is unavailable: {detail}. Nothing was written."));
    let existing = |metadata: fs::Metadata| {
        if metadata.is_dir() && !metadata.file_type().is_symlink() { Ok(()) } else { Err(unavailable("it is not a directory".to_string())) }
    };
    match fs::symlink_metadata(directory) {
        Ok(metadata) => existing(metadata),
        Err(error) if error.kind() == io::ErrorKind::NotFound => match fs::create_dir(directory) {
            Ok(()) => Ok(()),
            // Another writer created it first.
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                fs::symlink_metadata(directory).map_err(|error| unavailable(error.to_string())).and_then(existing)
            }
            Err(error) => Err(unavailable(error.to_string())),
        },
        Err(error) => Err(unavailable(error.to_string())),
    }
}

/// The newest record of `slot` (O(1) state over the enumeration).
fn newest_of_slot(dir: &RecordDir, slot: &str) -> io::Result<Option<AutosaveName>> {
    let mut newest: Option<AutosaveName> = None;
    dir.each_record(|name, _| {
        if name.slot == slot && newest.as_ref().is_none_or(|current| name.key() > current.key()) {
            newest = Some(name);
        }
    })?;
    Ok(newest)
}

/// Removes the records of `slot` strictly older than `written` (by written
/// time): regular files whose names parse as that slot's records, in the
/// handle's directory only. Each failure is a warning.
fn prune_older(dir: &RecordDir, slot: &str, written: &AutosaveName) -> Vec<DispositionDiagnostic> {
    let mut warnings = Vec::new();
    let mut unnamed = 0_usize;
    let enumerated = dir.each_record(|name, _| {
        if name.slot != slot || name.written_unix_millis >= written.written_unix_millis {
            return;
        }
        let file_name = name.file_name();
        if let Err(error) = dir.unlink(&file_name) {
            if error.kind() == io::ErrorKind::NotFound {
                return;
            }
            if warnings.len() < MAX_PRUNE_WARNINGS {
                warnings.push(diagnostic(PRUNE_FAILED, "", format!("The older autosave `{file_name}` was not removed: {error}.")));
            } else {
                unnamed += 1;
            }
        }
    });
    if let Err(error) = enumerated {
        warnings.push(diagnostic(PRUNE_FAILED, "", format!("The slot's older autosaves could not be listed: {error}.")));
    }
    if unnamed > 0 {
        warnings.push(diagnostic(PRUNE_FAILED, "", format!("{unnamed} more older autosaves were not removed.")));
    }
    warnings
}

/// The records in `directory`, newest first, at most [`MAX_LISTED_AUTOSAVES`]
/// of them; the rest are counted. The retained entries are reserved in
/// `memory` before the enumeration, which keeps at most that many.
pub(crate) fn list_autosaves(directory: &Path, memory: &dyn ReadMemory) -> Result<Metered<AutosaveListing>, DispositionDiagnostic> {
    let unavailable = |detail: String| diagnostic(DIRECTORY_UNAVAILABLE, "", format!("The autosave directory could not be listed: {detail}."));
    match fs::symlink_metadata(directory) {
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return Ok(Metered::new(AutosaveListing { entries: Vec::new(), not_listed: 0 }))
        }
        Err(error) => return Err(unavailable(error.to_string())),
        Ok(metadata) if !metadata.is_dir() || metadata.file_type().is_symlink() => {
            return Err(diagnostic(DIRECTORY_UNAVAILABLE, "", "The autosave directory is not a directory."))
        }
        Ok(_) => {}
    }
    let reservation = memory
        .reserve("the autosave listing", (MAX_LISTED_AUTOSAVES as u64).saturating_mul(LISTED_ENTRY_BYTES))
        .map_err(|message| diagnostic(STORE_READ_MEMORY_UNAVAILABLE, "", message))?;
    let dir = RecordDir::open(directory).map_err(|error| unavailable(error.to_string()))?;
    // A min-heap of the newest `MAX_LISTED_AUTOSAVES`: the oldest retained is popped.
    let mut newest: BinaryHeap<Reverse<((u64, String), u64)>> = BinaryHeap::with_capacity(MAX_LISTED_AUTOSAVES + 1);
    let mut total = 0_usize;
    dir.each_record(|name, len| {
        total += 1;
        newest.push(Reverse((name.key(), len)));
        if newest.len() > MAX_LISTED_AUTOSAVES {
            newest.pop();
        }
    })
    .map_err(|error| unavailable(error.to_string()))?;
    let entries: Vec<AutosaveEntry> = newest
        .into_sorted_vec()
        .into_iter()
        .filter_map(|Reverse(((_, file_name), byte_count))| {
            AutosaveName::parse(&file_name).map(|name| AutosaveEntry {
                slot: name.slot,
                written_unix_millis: name.written_unix_millis,
                model_hash: format!("sha256:{}", name.model_hash_hex),
                file_name,
                byte_count,
            })
        })
        .collect();
    let not_listed = total - entries.len();
    Ok(Metered::new(AutosaveListing { entries, not_listed }).holding(reservation))
}

/// Writes the session's canonical model document text as the newest record of
/// its slot in `directory` (§1, §5).
pub(crate) fn write_autosave(
    directory: &Path,
    canonical_text: &str,
    claimed_model_hash: &str,
    written_unix_millis: u64,
    memory: &dyn ReadMemory,
) -> AutosaveWriteOutcome {
    write_autosave_with_io(directory, canonical_text, claimed_model_hash, written_unix_millis, memory, &mut SystemAtomicSaveIo)
}

pub(crate) fn write_autosave_with_io(
    directory: &Path,
    canonical_text: &str,
    claimed_model_hash: &str,
    written_unix_millis: u64,
    memory: &dyn ReadMemory,
    io: &mut impl AtomicSaveIo,
) -> AutosaveWriteOutcome {
    let Some(claimed_hex) = claimed_model_hash.strip_prefix("sha256:").filter(|hex| is_lower_hex(hex, HASH_HEX_DIGITS)) else {
        return AutosaveWriteOutcome::refused_one(HASH_MISMATCH, "The claimed model hash is not `sha256:` and 64 lowercase hex digits.");
    };
    if written_unix_millis.to_string().len() > MILLIS_DIGITS {
        return AutosaveWriteOutcome::refused_one(CLOCK_INVALID, format!("The clock reads {written_unix_millis} ms, beyond a record name's 13 digits. Nothing was written."));
    }
    let admitted = match admit_text(canonical_text, claimed_hex, HASH_MISMATCH, memory) {
        Ok(admitted) => admitted,
        Err(diagnostics) => return AutosaveWriteOutcome::Refused { diagnostics },
    };
    if let Err(refusal) = ensure_directory(directory) {
        return AutosaveWriteOutcome::Refused { diagnostics: vec![refusal] };
    }
    let dir = match RecordDir::open(directory) {
        Ok(dir) => dir,
        Err(error) => return AutosaveWriteOutcome::refused_one(DIRECTORY_UNAVAILABLE, format!("The autosave directory is unavailable: {error}. Nothing was written.")),
    };
    // The whole check, write, read-back and prune is one transaction of the slot.
    let _lock = match lock_slot(&dir, &admitted.slot, LOCK_WAIT) {
        Ok(lock) => lock,
        Err(refusal) => return AutosaveWriteOutcome::Refused { diagnostics: vec![refusal] },
    };
    let model_hash = format!("sha256:{}", admitted.model_hash_hex);
    match newest_of_slot(&dir, &admitted.slot) {
        // The newest record already holds this model: unchanged only if it
        // still reads back as its hash; otherwise a new record is written.
        // A check that could not be made writes nothing and keeps the record.
        Ok(Some(newest)) if newest.model_hash_hex == admitted.model_hash_hex => match verify_record(&dir, &newest.file_name(), &admitted.model_hash_hex, memory) {
            Verified::Matches => return AutosaveWriteOutcome::Unchanged { file_name: newest.file_name(), slot: admitted.slot, model_hash },
            Verified::Differs(_) => {}
            Verified::Unavailable(detail) => {
                return AutosaveWriteOutcome::refused_one(
                    VERIFY_UNAVAILABLE,
                    format!("The newest autosave `{}` holds this model but could not be checked ({detail}); nothing was written and it is kept.", newest.file_name()),
                )
            }
        },
        Ok(_) => {}
        Err(error) => return AutosaveWriteOutcome::refused_one(DIRECTORY_UNAVAILABLE, format!("The autosave directory could not be listed: {error}. Nothing was written.")),
    }
    let name = AutosaveName { slot: admitted.slot.clone(), written_unix_millis, model_hash_hex: admitted.model_hash_hex.clone() };
    let file_name = name.file_name();
    // An existing entry of this name is replaced only when it verifiably is
    // not this model's record (a corrupt record of the same time and hash).
    match dir.stat_child(&file_name) {
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => return AutosaveWriteOutcome::refused_one(VERIFY_UNAVAILABLE, format!("The autosave name `{file_name}` could not be inspected ({error}); nothing was written.")),
        Ok(_) => match verify_record(&dir, &file_name, &admitted.model_hash_hex, memory) {
            Verified::Differs(_) => {}
            Verified::Matches => {
                return AutosaveWriteOutcome::refused_one(NAME_IN_USE, format!("A valid autosave `{file_name}` already has this name though it is not the newest; nothing was written."))
            }
            Verified::Unavailable(detail) => {
                return AutosaveWriteOutcome::refused_one(VERIFY_UNAVAILABLE, format!("The existing autosave `{file_name}` could not be checked ({detail}); nothing was written and it is kept."))
            }
        },
    }
    let saved = match atomic_save_bytes_as_with_io(&AUTOSAVE_FORMAT, &directory.join(&file_name), canonical_text.as_bytes(), io) {
        Ok(saved) => saved,
        Err(error) => return AutosaveWriteOutcome::Refused { diagnostics: vec![writer_diagnostic(error)] },
    };
    // The record is read back before any older record is removed. A record
    // that verifiably does not read back as its hash is removed; one that
    // could not be checked is kept, unverified, and nothing is pruned.
    match verify_record(&dir, &file_name, &admitted.model_hash_hex, memory) {
        Verified::Matches => {}
        Verified::Differs(detail) => {
            let removal = dir.unlink(&file_name).err().map(|error| format!(" It could not be removed: {error}.")).unwrap_or_default();
            return AutosaveWriteOutcome::refused_one(
                READBACK_MISMATCH,
                format!("The written autosave does not read back as {model_hash}: {detail}; the previous autosave is kept.{removal}"),
            );
        }
        Verified::Unavailable(detail) => {
            return AutosaveWriteOutcome::refused_one(
                READBACK_FAILED,
                format!("The written autosave `{file_name}` could not be read back ({detail}); it is kept unverified and the previous autosave is kept."),
            );
        }
    }
    drop(admitted.document);
    // Older records are removed only once this one is durable: its directory
    // synced after the rename, and the directory's own entry synced in its
    // parent, on every write (never inferred from the directory existing).
    let parent_durable = directory.parent().is_some_and(|parent| io.sync_parent(parent) == DirectorySyncResult::Durable);
    let durable = parent_durable && saved.durability == "durable";
    #[cfg(test)]
    BEFORE_PRUNE.with(|hook| {
        let hook = hook.borrow_mut().take();
        if let Some(hook) = hook {
            hook()
        }
    });
    let warnings = if durable {
        prune_older(&dir, &name.slot, &name)
    } else {
        vec![diagnostic(PRUNE_DEFERRED, "", "The autosave's directory sync did not complete, so the older autosaves are kept until a later autosave is durable.")]
    };
    AutosaveWriteOutcome::Written {
        file_name,
        slot: name.slot,
        written_unix_millis,
        model_hash,
        byte_count: canonical_text.len() as u64,
        durability: if durable { "durable" } else { "saved_durability_uncertain" }.to_string(),
        warnings,
    }
}

/// Reads one record for recovery (§3, §5), contained: the name must parse; the
/// directory must be a real directory; the entry must be a regular file within
/// the open cap, opened without following a link and confirmed by `fstat` as
/// the inspected file; it is read with a hard cap at the inspected size, which
/// is reserved first; its SHA-256 must be its name's hash; its text must be
/// canonical and pass the open path's checks; and its document's slot must be
/// its name's.
pub(crate) fn read_autosave(directory: &Path, file_name: &str, memory: &dyn ReadMemory) -> Result<Metered<VerifiedAutosave>, Vec<DispositionDiagnostic>> {
    let one = |code: &str, message: String| vec![diagnostic(code, "", message)];
    let refused = |access: Access| {
        let (message, code) = access.describe(file_name);
        vec![diagnostic(code, "", message)]
    };
    let Some(name) = AutosaveName::parse(file_name) else {
        return Err(one(NAME_INVALID, "The name is not an autosave record's name.".to_string()));
    };
    let dir = RecordDir::open(directory).map_err(|error| match error.kind() {
        io::ErrorKind::NotFound => one(NOT_FOUND, format!("The autosave `{file_name}` is no longer there.")),
        _ => one(DIRECTORY_UNAVAILABLE, format!("The autosave directory is unavailable: {error}.")),
    })?;
    let inspected = dir.inspect(file_name, MAX_MODEL_DOCUMENT_BYTES).map_err(refused)?;
    let load = memory.reserve("the autosave's file", loaded_text_bytes(inspected.len)).map_err(memory_refusal)?;
    let file = dir.open_inspected(file_name, inspected).map_err(refused)?;
    let bytes = read_capped(&file, inspected.len).map_err(refused)?;
    drop(file);
    let hex = {
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        hasher.finalize().iter().map(|byte| format!("{byte:02x}")).collect::<String>()
    };
    if hex != name.model_hash_hex {
        return Err(one(INTEGRITY, format!("The autosave's bytes hash to sha256:{hex}, not the sha256:{} its name records.", name.model_hash_hex)));
    }
    let text = String::from_utf8(bytes).map_err(|_| one(INTEGRITY, "The autosave is not UTF-8 text.".to_string()))?;
    let admitted = admit_text(&text, &name.model_hash_hex, INTEGRITY, memory)?;
    if admitted.slot != name.slot {
        return Err(one(INTEGRITY, "The autosave's document belongs to another project than its name records.".to_string()));
    }
    let byte_count = text.len() as u64;
    drop(text);
    let verified = admitted.document.map(|document| VerifiedAutosave {
        file_name: file_name.to_string(),
        slot: name.slot,
        written_unix_millis: name.written_unix_millis,
        model_hash: format!("sha256:{hex}"),
        byte_count,
        document,
    });
    Ok(verified.holding(load))
}

#[cfg(test)]
#[path = "autosave_tests.rs"]
mod tests;
