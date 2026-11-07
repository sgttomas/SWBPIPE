//! File > Import MBF… (DEL-17-04 Design `desktop-mbf-export-import.md` §4.1, §4.3 and §4.5 to §4.8; slice 5; RT-R8):
//! the native source, worker task 5 and the four commands `open_mbf_import`, `review_mbf_import`, `adopt_mbf_import`
//! and `cancel_mbf_import` (`lib.rs` registers them).
//!
//! **Low custody (§4.8.1 (e), §4.8.2).** The host keeps one fixed-size [`Source`] record: the chosen MBF and, when one
//! is present, its identity file `<stem>.swbpipe-ids.json`, each as a pinned read-only handle with its file identity,
//! size and SHA-256 at selection, or the identity file's absence. It keeps no file bytes, model, review or report
//! between commands. Every review and Adopt re-reads the exact selected bytes through duplicated handles into the
//! command's own request buffer, by position, and refuses any change (`MBF-IMPORT-SOURCE-CHANGED`): an edit in place,
//! a replacement, rename or removal at the name, growth or shrinkage, an identity file that appeared, or a symlink or
//! other non-regular file at a name. No command takes a path: the paths come only from the native open chooser.
//!
//! **Selection.** `lstat` must report a regular file that is not a symlink; the file is then opened read-only with
//! `O_RDONLY | O_NOFOLLOW | O_NONBLOCK` (safe `OpenOptions` flags), and the acquired handle's `fstat` must report a
//! regular file with the `lstat` identity and a size within its cap (MBF 16 MiB, identity file 64 MiB) before any read.
//! A FIFO substituted after the `lstat` therefore cannot block the open waiting for a writer. Cancellation is checked
//! before and immediately after acquisition.
//!
//! **Worker task 5 (§4.8.3).** [`worker_import`] runs in the capped worker process: the importer with the identity
//! file and the material declaration only (no pressure closure, rebinding, shear model or source document, §4.5); a
//! bound import's regeneration runs the writer there, inside the grant; the canonical model's native document check
//! (`model_document_file::check_model_document`); the bounded review (`import_review`, at most R = 1 MiB); and the
//! envelope `MBFI1\n<mode>\n<status>\n<adoptable>\n<model_sha256|->\n<review_sha256>\n<model_bytes>\n<model_len>\n
//! <review_len>\n` with the model (only in `adopt` mode when adoptable) and the review. The host reads it by
//! [`decode_import_result`]: borrowed views, bounded header tokens, the review's fixed leading members compared from a
//! fixed stack buffer, and SHA-256 checks of the review and (adopt mode only) the carried ASCII model.
//!
//! **The command's ledger (§4.8.4).** Each start is C1's form: the raw body (at most 4 KiB for Open, 1 MiB otherwise)
//! copied under a capped interactive lease that seeds the command's `CommandLedger` (its budget captured as
//! min(that lease's budget, 12 GiB)), the arguments' parse grown into it. Then H3 (the declaration's re-serialization),
//! one `LedgerBound` holding H4 (the request buffer, into which the files are read straight), H5 (the worker's maximum
//! grant) and H6 (the result share), the bounded grant wait, H4's admission, the read, supervision (the worker reaped
//! before the grant is released), result custody (the bound shrunk to the result lease), H8 (4 KiB of decoder control
//! scratch, admitted before the decode and held through the response) and H7 (the reply, its exact length counted over
//! borrowed views, escapes included, before its buffer is allocated). The response takes H7's buffer; the result lease,
//! the bound, H8 and H7 are released after the response is built. Between commands the import holds no pool or ledger
//! charge.
//!
//! **Cancellation (§4.8.6).** At most one import run is in progress; any newer start cancels it. A keyed retirement
//! (`cancel_mbf_import`, the export's id rule and limits, applied to this module's own state) cancels the run and drops
//! the source; the dispatch ticket `expectedEpoch` binds each start to the retirement epoch it was issued under. Import
//! writes nothing, so there is no commit point. The export's runs, retirement and the performance reset are separate.
//!
//! Standard claim fence applies (claims taxonomy per DEC-081): a successful import means the listed subset of the file
//! was read; it is not evidence that two programs analyse the model alike.

use crate::command_ingress::{self, Args, ArgsDecoding, CommandError};
use crate::ingress::allocation_envelope;
use crate::memory_pool::{CommandLease, Lease, LeasePurpose, LedgerBound, ResourceRefusal, WaitOutcome};
use crate::model_document_file::{check_model_document, ModelDocumentOpenOutcome, MAX_MODEL_DOCUMENT_BYTES};
use crate::solve_jobs::{SolveJobError, SolveJobRegistry};
use crate::solve_worker::{initial_grant, panic_message, supervise, SupervisorHooks, WorkerControl, WorkerOutcome, WorkerTask, WorkerTerms};
use crate::stored_read::{exact_text_bytes, NativeLeaseMemory};
use crate::worker_alloc::resident_envelope;
use open_pipe_stress_caepipe_mbf as mbf;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant};
use tauri::http::HeaderMap;
use tauri::ipc::InvokeBody;

// -- Names and limits -------------------------------------------------------------------------------------------------

pub(crate) const OPEN_COMMAND: &str = "open_mbf_import";
pub(crate) const REVIEW_COMMAND: &str = "review_mbf_import";
pub(crate) const ADOPT_COMMAND: &str = "adopt_mbf_import";
pub(crate) const CANCEL_COMMAND: &str = "cancel_mbf_import";

pub(crate) const EPOCH_INVALID: &str = "MBF-IMPORT-EPOCH-INVALID";
pub(crate) const EPOCH_EXHAUSTED: &str = "MBF-IMPORT-EPOCH-EXHAUSTED";
pub(crate) const CANCELLED: &str = "MBF-IMPORT-CANCELLED";
pub(crate) const ARGS_INVALID: &str = "MBF-IMPORT-ARGS-INVALID";
pub(crate) const RETIREMENT_ID_INVALID: &str = "MBF-IMPORT-RETIREMENT-ID-INVALID";
pub(crate) const OPEN_NOT_A_FILE: &str = "MBF-OPEN-NOT-A-FILE";
pub(crate) const OPEN_TOO_LARGE: &str = "MBF-OPEN-TOO-LARGE";
pub(crate) const OPEN_UNREADABLE: &str = "MBF-OPEN-UNREADABLE";
pub(crate) const OPEN_NAME_INVALID: &str = "MBF-OPEN-NAME-INVALID";
pub(crate) const SOURCE_RETIRED: &str = "MBF-IMPORT-SOURCE-RETIRED";
pub(crate) const SOURCE_CHANGED: &str = "MBF-IMPORT-SOURCE-CHANGED";
pub(crate) const DECLARATION_INVALID: &str = "MBF-IMPORT-DECLARATION-INVALID";
pub(crate) const MEMORY_UNAVAILABLE: &str = "MBF-IMPORT-MEMORY-UNAVAILABLE";
pub(crate) const WORKER_PROTOCOL: &str = "MBF-IMPORT-WORKER-PROTOCOL";
pub(crate) const WORKER_REQUEST_INVALID: &str = "MBF-IMPORT-WORKER-REQUEST-INVALID";
pub(crate) const ENGINE_FAILED: &str = "MBF-IMPORT-ENGINE-FAILED";
pub(crate) const REVIEW_STALE: &str = "MBF-IMPORT-REVIEW-STALE";
pub(crate) const SCHEMA_NOT_CURRENT: &str = "MBF-IMPORT-SCHEMA-NOT-CURRENT";
pub(crate) const NOT_ADOPTABLE: &str = mbf::NOT_ADOPTABLE;
pub(crate) const REVIEW_TRUNCATED: &str = mbf::IMPORT_REVIEW_TRUNCATED;
pub(crate) const MODEL_TOO_LARGE: &str = mbf::MODEL_TOO_LARGE;
pub(crate) const ASSUMPTION_UNAVAILABLE: &str = mbf::ASSUMPTION_UNAVAILABLE;

const CANCELLED_MESSAGE: &str = "The import was cancelled. Nothing was imported and your model is unchanged.";
const CHANGED_MESSAGE: &str = "The chosen file or its identity file changed since it was chosen. Choose it again.";

/// The import route's own bounds (§4.8.4: new payload bounds on new commands; no budget, ceiling or pricing moves).
pub(crate) const MBF_MAX_BYTES: u64 = 16 << 20;
pub(crate) const IDENTITY_MAX_BYTES: u64 = MAX_MODEL_DOCUMENT_BYTES;
pub(crate) const OPEN_ARGS_MAX_BYTES: usize = 4 << 10;
pub(crate) const ARGS_MAX_BYTES: usize = 1 << 20;
pub(crate) const LABEL_MAX_BYTES: usize = 1 << 10;
pub(crate) const DECLARATION_MAX_BYTES: usize = 1 << 20;
/// R, reused (`MBF_REVIEW_MAX_BYTES`).
pub(crate) const REVIEW_MAX_BYTES: usize = mbf::MBF_REVIEW_MAX_BYTES;
/// `review` mode's `result_limit`: R plus the envelope header.
pub(crate) const REVIEW_RESULT_LIMIT: u64 = REVIEW_MAX_BYTES as u64 + (4 << 10);
/// `adopt` mode's `result_limit`, and the final adopt reply's limit (`MBF_IMPORT_RESULT_MAX_BYTES`, the export's output
/// ceiling's number, not raised).
pub(crate) const MBF_IMPORT_RESULT_MAX_BYTES: u64 = mbf::MBF_EXPORT_OUTPUT_MAX_BYTES;
/// H8: the decoder's fixed control scratch.
pub(crate) const DECODER_SCRATCH_BYTES: u64 = 4 << 10;
/// The fixed prefix buffer inside H8.
const PREFIX_BUFFER_BYTES: usize = 1 << 10;
pub(crate) const READ_BLOCK_BYTES: usize = 64 << 10;
const MAX_DIAGNOSTICS: usize = 8;
const MAX_DIAGNOSTIC_MESSAGE_BYTES: usize = 2 << 10;
const MAX_WORKER_FAILURE_BYTES: usize = 64 << 10;
/// The identity file beside a chosen MBF (A-5, the name the export writes).
pub(crate) const IDS_SUFFIX: &str = ".swbpipe-ids.json";
/// The grant wait (`MBF_START_WAIT`'s value).
pub(crate) const START_WAIT: Duration = crate::solve_jobs::MODEL_CHECKS_START_WAIT;
/// The schema status a worker-checked candidate must have.
const CURRENT_SCHEMA_STATUS: &str = "current";
/// The most page tokens the import keeps (as the export).
pub(crate) const RETIREMENT_PAGES_MAX: usize = 64;
const PAGE_TOKEN_HEX_LEN: usize = 32;
const RETIREMENT_SEQUENCE_MAX: u64 = (1 << 53) - 1;
const CANCEL_ARGS_MAX_BYTES: usize = 4 << 10;
pub(crate) const EXPECTED_EPOCH: &str = "expectedEpoch";
pub(crate) const REQUEST_SEQUENCE: &str = "requestSequence";
pub(crate) const RETIREMENT_ID: &str = "retirementId";

// -- Diagnostics ------------------------------------------------------------------------------------------------------

/// One refusal diagnostic `{code, path, message}`; the message is native text of at most 2 KiB, never shown (the dialog
/// maps the code to its product text).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Diag {
    pub(crate) code: &'static str,
    pub(crate) path: &'static str,
    pub(crate) message: String,
}

fn diag(code: &'static str, path: &'static str, message: impl Into<String>) -> Diag {
    let mut message = message.into();
    if message.len() > MAX_DIAGNOSTIC_MESSAGE_BYTES {
        let mut cut = MAX_DIAGNOSTIC_MESSAGE_BYTES;
        while !message.is_char_boundary(cut) {
            cut -= 1;
        }
        message.truncate(cut);
    }
    Diag { code, path, message }
}

/// A run's failure: a refusal to reply, or a cancellation.
#[derive(Debug)]
enum Stop {
    Refused(Vec<Diag>),
    Cancelled,
}

fn refused(code: &'static str, path: &'static str, message: impl Into<String>) -> Stop {
    Stop::Refused(vec![diag(code, path, message)])
}

// -- Retirement and runs ----------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
struct RunState {
    cancelled: bool,
    handed_off: bool,
    worker: Option<Arc<WorkerControl>>,
}

/// One run's cancellation: a flag the wait, the reads, the supervisor and the install read, and kill access to its
/// worker while unreaped. Import has no commit.
#[derive(Debug, Default)]
pub(crate) struct RunControl {
    state: Mutex<RunState>,
}

impl RunControl {
    fn lock(&self) -> MutexGuard<'_, RunState> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }

    pub(crate) fn cancelled(&self) -> bool {
        self.lock().cancelled
    }

    /// Cancels and kills the worker if one is attached; true if this cancel stopped it.
    fn cancel(&self) -> bool {
        let mut state = self.lock();
        if state.cancelled || state.handed_off {
            return false;
        }
        state.cancelled = true;
        if let Some(worker) = &state.worker {
            worker.kill();
        }
        true
    }
}

/// One logical retirement's id (the export's rule: a page token of 32 lowercase hex digits and a sequence from 1 to
/// 2^53 - 1), read by this module's own reader with import wording.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct RetirementId {
    pub(crate) page: String,
    pub(crate) sequence: u64,
}

impl RetirementId {
    pub(crate) fn read(value: Option<serde_json::Value>) -> Result<Self, Diag> {
        let malformed = |what: &str| diag(RETIREMENT_ID_INVALID, "retirementId", format!("The import was not retired: its request {what}. Nothing was cancelled."));
        let mut map = match value {
            None => return Err(malformed("carries no retirement id")),
            Some(serde_json::Value::Object(map)) => map,
            Some(_) => return Err(malformed("carries a malformed retirement id")),
        };
        let page = match map.remove("page") {
            Some(serde_json::Value::String(page))
                if page.len() == PAGE_TOKEN_HEX_LEN && page.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)) =>
            {
                page
            }
            _ => return Err(malformed("carries a malformed retirement id")),
        };
        let sequence = map.remove("sequence").and_then(|value| value.as_u64()).filter(|sequence| (1..=RETIREMENT_SEQUENCE_MAX).contains(sequence));
        match sequence {
            Some(sequence) if map.is_empty() => Ok(RetirementId { page, sequence }),
            _ => Err(malformed("carries a malformed retirement id")),
        }
    }
}

/// `cancel_mbf_import`'s reply: `{cancelled, epoch, retirement_id}`, with `duplicate` or `exhausted` only when true.
/// There is no `too_late`: import has no commit.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct MbfImportRetirement {
    pub(crate) cancelled: u32,
    pub(crate) epoch: u64,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub(crate) exhausted: bool,
    pub(crate) retirement_id: RetirementId,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub(crate) duplicate: bool,
}

/// The file identity a re-read compares: device and inode on Unix; elsewhere the size and both times, each present.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct FileId {
    #[cfg(unix)]
    dev: u64,
    #[cfg(unix)]
    ino: u64,
    #[cfg(not(unix))]
    len: u64,
    #[cfg(not(unix))]
    modified: std::time::SystemTime,
    #[cfg(not(unix))]
    created: std::time::SystemTime,
}

fn file_id(meta: &fs::Metadata) -> io::Result<FileId> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        Ok(FileId { dev: meta.dev(), ino: meta.ino() })
    }
    #[cfg(not(unix))]
    {
        Ok(FileId { len: meta.len(), modified: meta.modified()?, created: meta.created()? })
    }
}

/// One chosen file: its path (never sent to the webview), its open read-only handle, its identity and its size and
/// SHA-256 as read at selection.
#[derive(Debug)]
pub(crate) struct Pinned {
    path: PathBuf,
    file: File,
    id: FileId,
    size: u64,
    sha256: [u8; 32],
}

#[derive(Debug)]
pub(crate) enum IdentityFile {
    Absent { expected_path: PathBuf },
    Present(Pinned),
}

/// The one source the host keeps between commands (§4.8.2): fixed size, not charged to the pool.
#[derive(Debug)]
pub(crate) struct Source {
    id: u64,
    entry_epoch: u64,
    file_name: String,
    mbf: Pinned,
    identity: IdentityFile,
}

#[derive(Debug)]
struct ImportState {
    epoch: u64,
    latest_sequence: u64,
    exhausted: bool,
    applied: Vec<(String, u64)>,
    run: Option<Arc<RunControl>>,
    source: Option<Source>,
    next_source_id: u64,
}

impl Default for ImportState {
    fn default() -> Self {
        ImportState { epoch: 0, latest_sequence: 0, exhausted: false, applied: Vec::new(), run: None, source: None, next_source_id: 1 }
    }
}

/// The MBF import's managed state (`MbfImportState` in §4.8.2): its retirement epoch and tokens, its one run in
/// progress and its one source. Separate from the export's `MbfRuns`.
#[derive(Clone, Debug, Default)]
pub(crate) struct MbfImports {
    state: Arc<Mutex<ImportState>>,
}

/// A command's entry: the epoch its ticket named, current when read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ImportEntry {
    epoch: u64,
    sequence: u64,
}

/// A run's registration: dropping it (as its command ends) stops tracking it.
struct Registration {
    imports: MbfImports,
    control: Arc<RunControl>,
}

impl Drop for Registration {
    fn drop(&mut self) {
        let mut state = self.imports.lock();
        if state.run.as_ref().is_some_and(|run| Arc::ptr_eq(run, &self.control)) {
            state.run = None;
        }
    }
}

impl MbfImports {
    fn lock(&self) -> MutexGuard<'_, ImportState> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// The command's entry from its ticket (read from the admitted arguments before any other member is used): below
    /// the current epoch `MBF-IMPORT-CANCELLED`; missing, malformed or future `MBF-IMPORT-EPOCH-INVALID`; exhausted
    /// `MBF-IMPORT-EPOCH-EXHAUSTED`.
    pub(crate) fn enter(&self, ticket: Option<serde_json::Value>, sequence: Option<serde_json::Value>) -> Result<ImportEntry, Diag> {
        let invalid = |what: &str| diag(EPOCH_INVALID, "expectedEpoch", format!("The import did not start: its request {what}. Nothing was imported."));
        let expected = match ticket {
            None => return Err(invalid("carries no retirement epoch")),
            Some(value) => value.as_u64().ok_or_else(|| invalid("carries a malformed retirement epoch"))?,
        };
        let mut state = self.lock();
        if state.exhausted {
            return Err(diag(EPOCH_EXHAUSTED, "", "The import did not start: it cannot be retired any further in this session. Restart SWBPIPE to import again."));
        }
        if expected > state.epoch {
            return Err(invalid("names a retirement epoch the host has not reached"));
        }
        if expected < state.epoch {
            return Err(diag(CANCELLED, "", CANCELLED_MESSAGE));
        }
        let sequence = sequence.and_then(|value| value.as_u64()).filter(|value| (1..=RETIREMENT_SEQUENCE_MAX).contains(value))
            .ok_or_else(|| diag(ARGS_INVALID, REQUEST_SEQUENCE, "The start requires a positive safe-integer request sequence."))?;
        if sequence <= state.latest_sequence {
            return Err(diag(CANCELLED, "", CANCELLED_MESSAGE));
        }
        state.latest_sequence = sequence;
        Ok(ImportEntry { epoch: expected, sequence })
    }

    /// Registers a run, cancelling the run in progress (any kind) and waking the pool's waiters. A command that entered
    /// before the latest retirement, or once exhausted, is cancelled from the start.
    fn register(&self, entry: ImportEntry, registry: &SolveJobRegistry) -> Registration {
        #[cfg(test)]
        if let Some(hook) = test_hooks::BEFORE_REGISTRATION.with(|hook| hook.borrow_mut().take()) {
            hook();
        }
        let mut state = self.lock();
        let control = Arc::new(RunControl::default());
        if entry.epoch < state.epoch || entry.sequence < state.latest_sequence || state.exhausted {
            control.cancel();
            return Registration { imports: self.clone(), control };
        }
        let stopped = state.run.replace(Arc::clone(&control)).is_some_and(|older| older.cancel());
        drop(state);
        if stopped {
            registry.pool().wake_waiters();
        }
        Registration { imports: self.clone(), control }
    }

    /// The keyed retirement: applied once per id; a duplicate or older delivery is a truthful no-op. An applied
    /// retirement moves the epoch on (or exhausts it at `u64::MAX`), cancels the run in progress and drops the source.
    /// A new page token beyond [`RETIREMENT_PAGES_MAX`] exhausts the import.
    pub(crate) fn retire_keyed(&self, id: RetirementId, registry: &SolveJobRegistry) -> MbfImportRetirement {
        let mut state = self.lock();
        match state.applied.iter().position(|(page, _)| *page == id.page) {
            Some(known) if id.sequence <= state.applied[known].1 => {
                return MbfImportRetirement { cancelled: 0, epoch: state.epoch, exhausted: state.exhausted, retirement_id: id, duplicate: true };
            }
            Some(known) => state.applied[known].1 = id.sequence,
            None if state.applied.len() < RETIREMENT_PAGES_MAX => state.applied.push((id.page.clone(), id.sequence)),
            None => state.exhausted = true,
        }
        if !state.exhausted {
            match state.epoch.checked_add(1) {
                Some(next) => state.epoch = next,
                None => state.exhausted = true,
            }
        }
        state.latest_sequence = 0;
        let cancelled = state.run.as_ref().is_some_and(|run| run.cancel());
        let source = state.source.take();
        let (epoch, exhausted) = (state.epoch, state.exhausted);
        drop(state);
        drop(source);
        if cancelled {
            registry.pool().wake_waiters();
        }
        MbfImportRetirement { cancelled: u32::from(cancelled), epoch, exhausted, retirement_id: id, duplicate: false }
    }

    #[cfg(test)]
    pub(crate) fn epoch(&self) -> u64 {
        self.lock().epoch
    }

    #[cfg(test)]
    pub(crate) fn seed_epoch_for_tests(&self, epoch: u64) {
        self.lock().epoch = epoch;
    }

    #[cfg(test)]
    pub(crate) fn seed_next_source_id_for_tests(&self, id: u64) {
        self.lock().next_source_id = id;
    }

    #[cfg(test)]
    pub(crate) fn source_id(&self) -> Option<u64> {
        self.lock().source.as_ref().map(|source| source.id)
    }

    #[cfg(test)]
    pub(crate) fn run_in_progress(&self) -> bool {
        self.lock().run.is_some()
    }

    /// Installs `source` if the run is not cancelled and its entry epoch is still current, replacing (and closing) the
    /// previous source; returns its id. Otherwise the new handles are dropped.
    fn install(&self, control: &RunControl, entry: ImportEntry, file_name: String, mbf: Pinned, identity: IdentityFile) -> Result<u64, Stop> {
        let mut state = self.lock();
        if control.cancelled() || entry.epoch != state.epoch || state.exhausted {
            return Err(Stop::Cancelled);
        }
        let id = state.next_source_id;
        let Some(next) = id.checked_add(1) else {
            state.exhausted = true;
            return Err(refused(EPOCH_EXHAUSTED, "", "The import cannot choose another file in this session. Restart SWBPIPE to import again."));
        };
        state.next_source_id = next;
        let previous = state.source.replace(Source { id, entry_epoch: entry.epoch, file_name, mbf, identity });
        drop(state);
        drop(previous);
        Ok(id)
    }

    /// The current source's record with duplicated handles (§4.8.2 re-read step 1), or `MBF-IMPORT-SOURCE-RETIRED`.
    fn take_source(&self, source_id: u64) -> Result<Source, Stop> {
        let state = self.lock();
        let retired = || refused(SOURCE_RETIRED, "sourceId", "The chosen file is no longer current. Choose it again.");
        let Some(source) = state.source.as_ref().filter(|source| source.id == source_id && source.entry_epoch == state.epoch) else {
            return Err(retired());
        };
        let duplicate = |pinned: &Pinned| -> Result<Pinned, Stop> {
            let file = pinned.file.try_clone().map_err(|error| refused(OPEN_UNREADABLE, "mbf", format!("The chosen file's handle could not be duplicated: {error}.")))?;
            Ok(Pinned { path: pinned.path.clone(), file, id: pinned.id, size: pinned.size, sha256: pinned.sha256 })
        };
        let identity = match &source.identity {
            IdentityFile::Absent { expected_path } => IdentityFile::Absent { expected_path: expected_path.clone() },
            IdentityFile::Present(pinned) => IdentityFile::Present(duplicate(pinned)?),
        };
        Ok(Source { id: source.id, entry_epoch: source.entry_epoch, file_name: source.file_name.clone(), mbf: duplicate(&source.mbf)?, identity })
    }
}

#[cfg(test)]
pub(crate) mod test_hooks {
    use std::cell::RefCell;
    use std::path::Path;
    thread_local! {
        /// Run just before a run registers (retirement before registration).
        pub(crate) static BEFORE_REGISTRATION: RefCell<Option<Box<dyn FnOnce()>>> = RefCell::new(None);
        /// Run after a chosen path's `lstat` and before its open (the FIFO acquisition race).
        pub(crate) static BEFORE_OPEN: RefCell<Option<Box<dyn FnMut(&Path)>>> = RefCell::new(None);
        /// Run after each read block, with the file's path and the offset reached (cancellation and growth races).
        pub(crate) static AFTER_BLOCK: RefCell<Option<Box<dyn FnMut(&Path, u64)>>> = RefCell::new(None);
        /// Run just before a command is admitted (retirement before admission).
        pub(crate) static BEFORE_ADMISSION: RefCell<Option<Box<dyn FnOnce()>>> = RefCell::new(None);
    }
    pub(crate) fn before_open(path: &Path) {
        BEFORE_OPEN.with(|hook| {
            if let Some(hook) = hook.borrow_mut().as_mut() {
                hook(path);
            }
        });
    }
    pub(crate) fn after_block(path: &Path, offset: u64) {
        AFTER_BLOCK.with(|hook| {
            if let Some(hook) = hook.borrow_mut().as_mut() {
                hook(path, offset);
            }
        });
    }
}

// -- Selection and re-read --------------------------------------------------------------------------------------------

fn not_a_file(path: &'static str) -> Stop {
    refused(OPEN_NOT_A_FILE, path, "The chosen name is not a regular file (a symlink, folder or other kind of file).")
}

#[cfg(unix)]
fn open_read_only(path: &Path) -> io::Result<File> {
    use std::os::unix::fs::OpenOptionsExt;
    fs::OpenOptions::new().read(true).custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK).open(path)
}

#[cfg(windows)]
fn open_read_only(path: &Path) -> io::Result<File> {
    use std::os::windows::fs::OpenOptionsExt;
    // FILE_FLAG_OPEN_REPARSE_POINT: a reparse point is opened as itself, never followed; the handle checks refuse it.
    fs::OpenOptions::new().read(true).custom_flags(0x0020_0000).open(path)
}

#[cfg(not(any(unix, windows)))]
fn open_read_only(path: &Path) -> io::Result<File> {
    fs::OpenOptions::new().read(true).open(path)
}

#[cfg(unix)]
fn read_at(file: &File, buffer: &mut [u8], offset: u64) -> io::Result<usize> {
    std::os::unix::fs::FileExt::read_at(file, buffer, offset)
}

#[cfg(windows)]
fn read_at(file: &File, buffer: &mut [u8], offset: u64) -> io::Result<usize> {
    std::os::windows::fs::FileExt::seek_read(file, buffer, offset)
}

#[cfg(not(any(unix, windows)))]
fn read_at(_file: &File, _buffer: &mut [u8], _offset: u64) -> io::Result<usize> {
    Err(io::Error::new(io::ErrorKind::Unsupported, "positional reads are unavailable on this platform"))
}

/// A chosen file acquired (§4.8.2 "Selection"), not yet read: `None` when `optional` and it does not exist.
fn acquire(path: &Path, cap: u64, member: &'static str, optional: bool, control: &RunControl) -> Result<Option<(File, FileId, u64)>, Stop> {
    let unreadable = |what: &str, error: io::Error| refused(OPEN_UNREADABLE, member, format!("The chosen file could not be {what}: {error}."));
    let meta = match fs::symlink_metadata(path) {
        Ok(meta) => meta,
        Err(error) if optional && error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(unreadable("examined", error)),
    };
    if !meta.file_type().is_file() {
        return Err(not_a_file(member));
    }
    if meta.len() > cap {
        return Err(refused(OPEN_TOO_LARGE, member, format!("The file is {} bytes; at most {cap} bytes can be imported.", meta.len())));
    }
    let id = file_id(&meta).map_err(|error| unreadable("examined", error))?;
    #[cfg(test)]
    test_hooks::before_open(path);
    if control.cancelled() {
        return Err(Stop::Cancelled);
    }
    let file = match open_read_only(path) {
        Ok(file) => file,
        #[cfg(unix)]
        Err(error) if error.raw_os_error() == Some(libc::ELOOP) => return Err(not_a_file(member)),
        Err(error) => return Err(unreadable("opened", error)),
    };
    if control.cancelled() {
        drop(file);
        return Err(Stop::Cancelled);
    }
    let held = file.metadata().map_err(|error| unreadable("examined", error))?;
    if !held.file_type().is_file() {
        return Err(not_a_file(member));
    }
    if file_id(&held).map_err(|error| unreadable("examined", error))? != id {
        return Err(refused(OPEN_UNREADABLE, member, "The chosen file was replaced while it was being opened. Choose it again."));
    }
    if held.len() > cap {
        return Err(refused(OPEN_TOO_LARGE, member, format!("The file is {} bytes; at most {cap} bytes can be imported.", held.len())));
    }
    Ok(Some((file, id, held.len())))
}

/// Reads exactly `size` bytes of `file` by position into `buffer` in 64 KiB blocks, checking cancellation between
/// blocks, then requires a one-byte read at `size` to return 0; returns their SHA-256. A short read or growth is
/// `changed` (`Ok(None)`).
fn read_exact_pinned(file: &File, path: &Path, size: u64, buffer: &mut Vec<u8>, control: &RunControl) -> Result<Option<[u8; 32]>, Stop> {
    let unreadable = |error: io::Error| refused(OPEN_UNREADABLE, "mbf", format!("The chosen file could not be read: {error}."));
    let start = buffer.len();
    let size_usize = usize::try_from(size).map_err(|_| refused(OPEN_TOO_LARGE, "mbf", "The file is too large."))?;
    buffer.resize(start + size_usize, 0);
    let mut hasher = Sha256::new();
    let mut offset = 0_usize;
    while offset < size_usize {
        if control.cancelled() {
            return Err(Stop::Cancelled);
        }
        let end = (offset + READ_BLOCK_BYTES).min(size_usize);
        let read = read_at(file, &mut buffer[start + offset..start + end], offset as u64).map_err(unreadable)?;
        if read == 0 {
            return Ok(None);
        }
        hasher.update(&buffer[start + offset..start + offset + read]);
        offset += read;
        #[cfg(test)]
        test_hooks::after_block(path, offset as u64);
    }
    let _ = path;
    let mut probe = [0_u8; 1];
    if read_at(file, &mut probe, size).map_err(unreadable)? != 0 {
        return Ok(None);
    }
    Ok(Some(hasher.finalize().into()))
}

/// The name checks of a re-read (§4.8.2 step 2): the path still names a regular non-symlink file of the recorded
/// identity and size.
fn names_pinned(pinned: &Pinned) -> bool {
    match fs::symlink_metadata(&pinned.path) {
        Ok(meta) => meta.file_type().is_file() && meta.len() == pinned.size && file_id(&meta).ok() == Some(pinned.id),
        Err(_) => false,
    }
}

fn changed() -> Stop {
    refused(SOURCE_CHANGED, "", CHANGED_MESSAGE)
}

/// The identity file's derived path beside the chosen MBF (A-5).
pub(crate) fn identity_path(chosen: &Path) -> Option<PathBuf> {
    let stem = chosen.file_stem()?.to_str()?;
    Some(chosen.with_file_name(format!("{stem}{IDS_SUFFIX}")))
}

// -- The worker request and envelope (task 5) -------------------------------------------------------------------------

/// Mode 1 is `review`, 2 is `adopt`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ImportMode {
    Review,
    Adopt,
}

impl ImportMode {
    fn code(self) -> u32 {
        match self {
            ImportMode::Review => 1,
            ImportMode::Adopt => 2,
        }
    }

    fn digit(self) -> u8 {
        b'0' + self.code() as u8
    }

    pub(crate) fn result_limit(self) -> u64 {
        match self {
            ImportMode::Review => REVIEW_RESULT_LIMIT,
            ImportMode::Adopt => MBF_IMPORT_RESULT_MAX_BYTES,
        }
    }
}

const IDENTITY_ABSENT: u32 = u32::MAX;

/// The request's exact length from its parts' lengths (`identity` None when absent).
pub(crate) fn request_len(label: usize, declaration: usize, mbf: u64, identity: Option<u64>) -> u64 {
    20 + label as u64 + declaration as u64 + mbf + identity.unwrap_or(0)
}

/// The request's fixed parts, written into the request buffer before the files.
fn push_u32(buffer: &mut Vec<u8>, value: u32) {
    buffer.extend_from_slice(&value.to_le_bytes());
}

/// A decoded request (views into the raw request).
#[derive(Debug)]
pub(crate) struct WorkerRequest<'a> {
    pub(crate) mode: ImportMode,
    pub(crate) label: &'a str,
    pub(crate) declaration: &'a [u8],
    pub(crate) mbf: &'a [u8],
    pub(crate) identity: Option<&'a [u8]>,
}

fn worker_failure(code: &str, message: impl std::fmt::Display) -> String {
    let mut text = format!("{code}: {message}");
    if text.len() > MAX_WORKER_FAILURE_BYTES {
        let mut cut = MAX_WORKER_FAILURE_BYTES;
        while !text.is_char_boundary(cut) {
            cut -= 1;
        }
        text.truncate(cut);
    }
    text
}

pub(crate) fn decode_worker_request(raw: &[u8]) -> Result<WorkerRequest<'_>, String> {
    let invalid = |what: &str| worker_failure(WORKER_REQUEST_INVALID, format!("the import request {what}"));
    let mut rest = raw;
    fn take_u32(rest: &mut &[u8]) -> Option<u32> {
        let (head, tail) = rest.split_first_chunk::<4>()?;
        *rest = tail;
        Some(u32::from_le_bytes(*head))
    }
    fn take<'a>(rest: &mut &'a [u8], len: usize) -> Option<&'a [u8]> {
        if rest.len() < len {
            return None;
        }
        let (head, tail) = rest.split_at(len);
        *rest = tail;
        Some(head)
    }
    let mode = match take_u32(&mut rest) {
        Some(1) => ImportMode::Review,
        Some(2) => ImportMode::Adopt,
        _ => return Err(invalid("has an unknown mode")),
    };
    let label_len = take_u32(&mut rest).ok_or_else(|| invalid("is truncated"))? as usize;
    if label_len == 0 || label_len > LABEL_MAX_BYTES {
        return Err(invalid("has a file name of the wrong length"));
    }
    let label = take(&mut rest, label_len).and_then(|bytes| std::str::from_utf8(bytes).ok()).ok_or_else(|| invalid("has a file name that is not UTF-8"))?;
    let declaration_len = take_u32(&mut rest).ok_or_else(|| invalid("is truncated"))? as usize;
    if declaration_len > DECLARATION_MAX_BYTES {
        return Err(invalid("has a declaration above 1 MiB"));
    }
    let declaration = take(&mut rest, declaration_len).ok_or_else(|| invalid("is truncated"))?;
    let mbf_len = take_u32(&mut rest).ok_or_else(|| invalid("is truncated"))? as u64;
    if mbf_len > MBF_MAX_BYTES {
        return Err(invalid("has an MBF above 16 MiB"));
    }
    let mbf_bytes = take(&mut rest, mbf_len as usize).ok_or_else(|| invalid("is truncated"))?;
    let identity_len = take_u32(&mut rest).ok_or_else(|| invalid("is truncated"))?;
    let identity = if identity_len == IDENTITY_ABSENT {
        None
    } else {
        if u64::from(identity_len) > IDENTITY_MAX_BYTES {
            return Err(invalid("has an identity file above 64 MiB"));
        }
        Some(take(&mut rest, identity_len as usize).ok_or_else(|| invalid("is truncated"))?)
    };
    if !rest.is_empty() {
        return Err(invalid("has trailing bytes"));
    }
    Ok(WorkerRequest { mode, label, declaration, mbf: mbf_bytes, identity })
}

/// Encodes a request (tests and the in-process checks; the host writes its request buffer in place).
#[cfg(test)]
pub(crate) fn encode_worker_request(request: &WorkerRequest<'_>) -> Vec<u8> {
    let mut raw = Vec::new();
    push_u32(&mut raw, request.mode.code());
    push_u32(&mut raw, request.label.len() as u32);
    raw.extend_from_slice(request.label.as_bytes());
    push_u32(&mut raw, request.declaration.len() as u32);
    raw.extend_from_slice(request.declaration);
    push_u32(&mut raw, request.mbf.len() as u32);
    raw.extend_from_slice(request.mbf);
    match request.identity {
        Some(identity) => {
            push_u32(&mut raw, identity.len() as u32);
            raw.extend_from_slice(identity);
        }
        None => push_u32(&mut raw, IDENTITY_ABSENT),
    }
    raw
}

fn hex64(digest: &[u8]) -> [u8; 64] {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = [0_u8; 64];
    for (index, byte) in digest.iter().take(32).enumerate() {
        out[2 * index] = HEX[(byte >> 4) as usize];
        out[2 * index + 1] = HEX[(byte & 15) as usize];
    }
    out
}

fn sha256_hex(bytes: &[u8]) -> [u8; 64] {
    hex64(&Sha256::digest(bytes))
}

fn ascii(hex: &[u8; 64]) -> &str {
    std::str::from_utf8(hex).expect("ASCII hex")
}

fn decimal_len(value: u64) -> u64 {
    value.checked_ilog10().map_or(1, |digits| u64::from(digits) + 1)
}

/// The envelope's exact length.
pub(crate) fn envelope_len(mode: ImportMode, imported: bool, has_candidate: bool, model_bytes: u64, model_len: u64, review_len: u64) -> u64 {
    let header = 6 // "MBFI1\n"
        + 2 // mode
        + if imported { 9 } else { 8 } // status
        + 2 // adoptable
        + if has_candidate { 65 } else { 2 } // model digest
        + 65 // review digest
        + decimal_len(model_bytes) + 1
        + decimal_len(model_len) + 1
        + decimal_len(review_len) + 1;
    let _ = mode;
    header + model_len + review_len
}

// -- JSON reply writing (counted exactly before allocation) ----------------------------------------------------------

trait Out {
    fn put(&mut self, bytes: &[u8]);
}

struct Counter(u64);

impl Out for Counter {
    fn put(&mut self, bytes: &[u8]) {
        self.0 = self.0.saturating_add(bytes.len() as u64);
    }
}

impl Out for Vec<u8> {
    fn put(&mut self, bytes: &[u8]) {
        self.extend_from_slice(bytes);
    }
}

/// `text` as serde_json writes a string: `"` and `\` escaped, `\b \f \n \r \t`, other control characters as `\u00XX`,
/// everything else (non-ASCII included) as itself.
fn put_json_str<O: Out>(out: &mut O, text: &str) {
    out.put(b"\"");
    let bytes = text.as_bytes();
    let mut start = 0;
    for (index, byte) in bytes.iter().enumerate() {
        let escape: Option<&[u8]> = match byte {
            b'"' => Some(b"\\\""),
            b'\\' => Some(b"\\\\"),
            b'\n' => Some(b"\\n"),
            b'\r' => Some(b"\\r"),
            b'\t' => Some(b"\\t"),
            0x08 => Some(b"\\b"),
            0x0c => Some(b"\\f"),
            0x00..=0x1f => None,
            _ => continue,
        };
        out.put(&bytes[start..index]);
        match escape {
            Some(escape) => out.put(escape),
            None => {
                const HEX: &[u8; 16] = b"0123456789abcdef";
                out.put(&[b'\\', b'u', b'0', b'0', HEX[(byte >> 4) as usize], HEX[(byte & 15) as usize]]);
            }
        }
        start = index + 1;
    }
    out.put(&bytes[start..]);
    out.put(b"\"");
}

fn put_u64<O: Out>(out: &mut O, mut value: u64) {
    let mut digits = [0_u8; 20];
    let mut at = digits.len();
    loop {
        at -= 1;
        digits[at] = b'0' + (value % 10) as u8;
        value /= 10;
        if value == 0 {
            break;
        }
    }
    out.put(&digits[at..]);
}

/// The final adopt reply (§4.7): the `NativeModelDocumentOutcome` shape `checkNativeOutcome` takes, with the model's
/// canonical bytes embedded as the document.
fn put_adopt_reply<O: Out>(out: &mut O, file_name: &str, model: &[u8], review_sha: &[u8; 64], model_sha: &[u8; 64]) {
    out.put(b"{\"outcome\":\"opened\",\"file_name\":");
    put_json_str(out, file_name);
    out.put(b",\"byte_count\":");
    put_u64(out, model.len() as u64);
    out.put(b",\"schema_status\":\"");
    out.put(CURRENT_SCHEMA_STATUS.as_bytes());
    out.put(b"\",\"document\":");
    out.put(model);
    out.put(b",\"warnings\":[],\"reviewSha256\":\"");
    out.put(review_sha);
    out.put(b"\",\"modelSha256\":\"");
    out.put(model_sha);
    out.put(b"\"}");
}

/// The adopt reply's exact length for a model of `model_len` bytes (the worker's transfer decision and the host's
/// final check use this one count).
pub(crate) fn adopt_reply_len(file_name: &str, model_len: u64) -> u64 {
    let mut count = Counter(0);
    let zero = [b'0'; 64];
    put_adopt_reply(&mut count, file_name, &[], &zero, &zero);
    count.0 + model_len + decimal_len(model_len) - 1
}

fn put_reviewed_reply<O: Out>(out: &mut O, source_id: u64, file_name: &str, review_sha: &[u8; 64], review: &[u8]) {
    out.put(b"{\"outcome\":\"reviewed\",\"sourceId\":");
    put_u64(out, source_id);
    out.put(b",\"fileName\":");
    put_json_str(out, file_name);
    out.put(b",\"reviewSha256\":\"");
    out.put(review_sha);
    out.put(b"\",\"review\":");
    out.put(review);
    out.put(b"}");
}

fn put_refused_reply<O: Out>(out: &mut O, adopt: bool, file_name: &str, diagnostics: &[Diag]) {
    out.put(if adopt { b"{\"outcome\":\"refused\",\"file_name\":" as &[u8] } else { b"{\"outcome\":\"refused\",\"fileName\":" });
    put_json_str(out, file_name);
    out.put(b",\"diagnostics\":[");
    for (index, item) in diagnostics.iter().take(MAX_DIAGNOSTICS).enumerate() {
        if index > 0 {
            out.put(b",");
        }
        out.put(b"{\"code\":");
        put_json_str(out, item.code);
        out.put(b",\"path\":");
        put_json_str(out, item.path);
        out.put(b",\"message\":");
        put_json_str(out, &item.message);
        out.put(b"}");
    }
    out.put(b"]}");
}

// -- The worker (task 5) ----------------------------------------------------------------------------------------------

/// The declaration as the engine reads it: none, or the parsed object.
fn worker_declaration(bytes: &[u8]) -> Result<Option<mbf::Value>, String> {
    if bytes.is_empty() {
        return Ok(None);
    }
    mbf::parse_sidecar_json(bytes).map(Some).map_err(|_| worker_failure(WORKER_REQUEST_INVALID, "the declaration is not strict JSON"))
}

/// Task 5 (§4.8.3): the import, the candidate's document check, the bounded review and the envelope; or
/// `<code>: <message>` of at most 64 KiB, with no engine text.
pub(crate) fn worker_import(raw: &[u8]) -> Result<String, String> {
    let request = decode_worker_request(raw)?;
    let material_provenance = worker_declaration(request.declaration)?;
    let declaration_sha = (!request.declaration.is_empty()).then(|| sha256_hex(request.declaration));
    let mut options = mbf::ImportOptions::new(request.label);
    options.sidecar = request.identity.map(|bytes| mbf::SidecarInput::Json(bytes.to_vec()));
    options.material_provenance = material_provenance;
    let mut result = mbf::import(request.mbf, &options).map_err(|_| worker_failure(ENGINE_FAILED, "the importer returned no result"))?;
    drop(options);
    let model_bytes = match result.model_bytes() {
        Ok(bytes) => bytes,
        Err(_) => return Err(worker_failure(ENGINE_FAILED, "the imported model has no canonical form")),
    };
    // The native half of the open path's checks over the same bytes.
    let mut findings: Vec<(String, String)> = Vec::new();
    let mut schema_status = String::new();
    if let Some(bytes) = &model_bytes {
        match serde_json::from_slice::<serde_json::Value>(bytes) {
            Ok(document) => match check_model_document(request.label.to_string(), bytes.len() as u64, document) {
                ModelDocumentOpenOutcome::Opened { schema_status: status, document, .. } => {
                    drop(document);
                    if status != CURRENT_SCHEMA_STATUS {
                        findings.push((SCHEMA_NOT_CURRENT.to_string(), "schema_version".to_string()));
                    }
                    schema_status = status;
                }
                ModelDocumentOpenOutcome::Refused { diagnostics, .. } => {
                    findings.extend(diagnostics.into_iter().map(|item| (item.code.to_string(), item.path)));
                }
                ModelDocumentOpenOutcome::Cancelled => findings.push((ENGINE_FAILED.to_string(), String::new())),
            },
            Err(_) => findings.push(("MODEL-FILE-NOT-JSON".to_string(), String::new())),
        }
    }
    let mbf_sha = sha256_hex(request.mbf);
    let identity_sha = request.identity.map(sha256_hex);
    let model_sha = model_bytes.as_deref().map(sha256_hex);
    let model_len = model_bytes.as_ref().map_or(0, |bytes| bytes.len() as u64);
    let identity_file = match (request.identity, &identity_sha) {
        (Some(bytes), Some(sha)) => mbf::ReviewedIdentityFile::Present { sha256: ascii(sha), bytes: bytes.len() as u64 },
        _ => mbf::ReviewedIdentityFile::Absent,
    };
    let source = mbf::ImportReviewSource { file_name: request.label, mbf_sha256: ascii(&mbf_sha), mbf_bytes: request.mbf.len() as u64, identity_file };
    let check = match (&model_bytes, findings.is_empty()) {
        (None, _) => mbf::ReviewDocumentCheck::NotRun,
        (Some(_), true) => mbf::ReviewDocumentCheck::Passed { schema_status: &schema_status },
        (Some(_), false) => mbf::ReviewDocumentCheck::Blocked(&findings),
    };
    let candidate = model_sha.as_ref().map(|sha| mbf::ReviewModel { sha256: ascii(sha), bytes: model_len });
    let imported = result.status == "imported";
    let label = request.label;
    // The transfer limit (§4.8.4): the adopt envelope and the final escaped adopt reply each within 64 MiB, and the
    // model within the Open cap; decided from the computed lengths before the result is built.
    let transfer_fits = |review_len: u64| {
        envelope_len(ImportMode::Adopt, imported, true, model_len, model_len, review_len) <= MBF_IMPORT_RESULT_MAX_BYTES
            && adopt_reply_len(label, model_len) <= MBF_IMPORT_RESULT_MAX_BYTES
            && model_len <= MAX_MODEL_DOCUMENT_BYTES
    };
    let review = mbf::import_review(&result, source, declaration_sha.as_ref().map(ascii), candidate, check, REVIEW_MAX_BYTES, transfer_fits)
        .map_err(|too_small| worker_failure(ENGINE_FAILED, format!("the review needs {} bytes of its {} byte allowance", too_small.needed, too_small.allowance)))?;
    if let Some(model) = result.model.take() {
        mbf::value::dispose(model);
    }
    mbf::value::dispose(std::mem::replace(&mut result.id_map, mbf::Value::Null));
    drop(result);
    let review_sha = sha256_hex(review.text.as_bytes());
    let adoptable = review.summary.adoptable;
    let carried = if request.mode == ImportMode::Adopt && adoptable { model_bytes.as_deref() } else { None };
    let carried_len = carried.map_or(0, |bytes| bytes.len() as u64);
    let total = envelope_len(request.mode, imported, model_sha.is_some(), model_len, carried_len, review.text.len() as u64);
    if total > request.mode.result_limit() {
        return Err(worker_failure(MODEL_TOO_LARGE, "the import's result is above its transfer limit"));
    }
    let mut envelope = String::with_capacity(total as usize);
    envelope.push_str("MBFI1\n");
    envelope.push(request.mode.digit() as char);
    envelope.push('\n');
    envelope.push_str(if imported { "imported" } else { "refused" });
    envelope.push('\n');
    envelope.push(if adoptable { '1' } else { '0' });
    envelope.push('\n');
    match &model_sha {
        Some(sha) => envelope.push_str(ascii(sha)),
        None => envelope.push('-'),
    }
    envelope.push('\n');
    envelope.push_str(ascii(&review_sha));
    envelope.push('\n');
    envelope.push_str(&model_len.to_string());
    envelope.push('\n');
    envelope.push_str(&carried_len.to_string());
    envelope.push('\n');
    envelope.push_str(&review.text.len().to_string());
    envelope.push('\n');
    if let Some(model) = carried {
        envelope.push_str(std::str::from_utf8(model).map_err(|_| worker_failure(ENGINE_FAILED, "the canonical model is not ASCII"))?);
    }
    envelope.push_str(&review.text);
    if envelope.len() as u64 != total {
        return Err(worker_failure(ENGINE_FAILED, "the envelope's length is not its counted length"));
    }
    Ok(envelope)
}

// -- Decode (host) ----------------------------------------------------------------------------------------------------

/// The decoded envelope: fixed-size header facts and byte ranges into the result (no derived storage).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Decoded {
    pub(crate) imported: bool,
    pub(crate) adoptable: bool,
    pub(crate) model_sha: Option<[u8; 64]>,
    pub(crate) review_sha: [u8; 64],
    pub(crate) model_bytes: u64,
    pub(crate) model: std::ops::Range<usize>,
    pub(crate) review: std::ops::Range<usize>,
}

fn protocol(detail: impl std::fmt::Display) -> Diag {
    diag(WORKER_PROTOCOL, "", format!("The import worker's result is malformed: {detail}."))
}

/// The next header token (up to `\n`), at most `max` bytes, as a borrowed slice; an overlong one refuses before any
/// copy.
fn token<'a>(rest: &mut &'a [u8], max: usize) -> Result<&'a [u8], Diag> {
    let window = &rest[..rest.len().min(max + 1)];
    let Some(end) = window.iter().position(|byte| *byte == b'\n') else {
        return Err(protocol("a header field is overlong or unterminated"));
    };
    let (head, tail) = rest.split_at(end);
    *rest = &tail[1..];
    Ok(head)
}

fn length_token(bytes: &[u8]) -> Result<u64, Diag> {
    let canonical = !bytes.is_empty() && bytes.len() <= 20 && bytes.iter().all(u8::is_ascii_digit) && (bytes.len() == 1 || bytes[0] != b'0');
    if !canonical {
        return Err(protocol("a length is not a canonical decimal"));
    }
    std::str::from_utf8(bytes).ok().and_then(|text| text.parse::<u64>().ok()).ok_or_else(|| protocol("a length is out of range"))
}

fn sha_token(bytes: &[u8]) -> Result<[u8; 64], Diag> {
    if bytes.len() != 64 || !bytes.iter().all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f')) {
        return Err(protocol("a digest is not 64 lowercase hexadecimal digits"));
    }
    let mut out = [0_u8; 64];
    out.copy_from_slice(bytes);
    Ok(out)
}

/// A fixed stack buffer (inside H8) that the review's expected leading members are written into.
struct Prefix {
    bytes: [u8; PREFIX_BUFFER_BYTES],
    len: usize,
    overflow: bool,
}

impl Out for Prefix {
    fn put(&mut self, bytes: &[u8]) {
        let end = self.len + bytes.len();
        if end > PREFIX_BUFFER_BYTES {
            self.overflow = true;
            return;
        }
        self.bytes[self.len..end].copy_from_slice(bytes);
        self.len = end;
    }
}

/// Reads task 5's envelope for `mode` (§4.8.3 "Decode"): views and checked header tokens, the mode's fixed payload
/// shape, the review's fixed leading members byte for byte, its SHA-256, and in `adopt` mode with an adoptable result
/// the carried ASCII model and its SHA-256. The host never parses the model or the review.
pub(crate) fn decode_import_result(mode: ImportMode, result: &[u8]) -> Result<Decoded, Diag> {
    let mut rest = result;
    if token(&mut rest, 5)? != b"MBFI1" {
        return Err(protocol("the magic is wrong"));
    }
    if token(&mut rest, 1)? != [mode.digit()] {
        return Err(protocol("the mode is not the request's"));
    }
    let imported = match token(&mut rest, 8)? {
        b"imported" => true,
        b"refused" => false,
        _ => return Err(protocol("the import status is unknown")),
    };
    let adoptable = match token(&mut rest, 1)? {
        b"1" => true,
        b"0" => false,
        _ => return Err(protocol("the adoptable flag is not 0 or 1")),
    };
    if adoptable && !imported {
        return Err(protocol("a refused import is marked adoptable"));
    }
    let model_digest = token(&mut rest, 64)?;
    let review_sha = sha_token(token(&mut rest, 64)?)?;
    let model_bytes = length_token(token(&mut rest, 20)?)?;
    let model_len = length_token(token(&mut rest, 20)?)?;
    let review_len = length_token(token(&mut rest, 20)?)?;
    let model_sha = if model_digest == b"-" {
        if model_bytes != 0 || imported {
            return Err(protocol("the model sentinel does not match the candidate"));
        }
        None
    } else {
        if model_bytes == 0 {
            return Err(protocol("a model digest without a candidate"));
        }
        Some(sha_token(model_digest)?)
    };
    if imported != model_sha.is_some() {
        return Err(protocol("the candidate does not match the import status"));
    }
    let header_len = result.len() - rest.len();
    let body_len = model_len.checked_add(review_len).ok_or_else(|| protocol("the lengths overflow"))?;
    if body_len != rest.len() as u64 {
        return Err(protocol("the lengths do not cover the bodies"));
    }
    match mode {
        ImportMode::Review if model_len != 0 => return Err(protocol("a review-mode result carries a model")),
        ImportMode::Adopt if adoptable && model_len != model_bytes => return Err(protocol("the carried model is not the candidate")),
        ImportMode::Adopt if !adoptable && model_len != 0 => return Err(protocol("a non-adoptable result carries a model")),
        _ => {}
    }
    if review_len > REVIEW_MAX_BYTES as u64 {
        return Err(protocol("the review is above R"));
    }
    let model = header_len..header_len + model_len as usize;
    let review = model.end..result.len();
    let review_view = &result[review.clone()];
    // The fixed leading members, from the validated fields only.
    let mut prefix = Prefix { bytes: [0; PREFIX_BUFFER_BYTES], len: 0, overflow: false };
    prefix.put(b"{\"document_kind\":\"swbpipe.mbf_import_review\",\"format\":1,\"status\":\"");
    prefix.put(if imported { b"imported" } else { b"refused" });
    prefix.put(b"\",\"adoptable\":");
    prefix.put(if adoptable { b"true" } else { b"false" });
    prefix.put(b",\"model_sha256\":");
    match &model_sha {
        Some(sha) => {
            prefix.put(b"\"");
            prefix.put(sha);
            prefix.put(b"\"");
        }
        None => prefix.put(b"null"),
    }
    prefix.put(b",\"model_bytes\":");
    put_u64(&mut prefix, model_bytes);
    prefix.put(b",");
    if prefix.overflow || !review_view.starts_with(&prefix.bytes[..prefix.len]) {
        return Err(protocol("the review's leading members do not match the header"));
    }
    if sha256_hex(review_view) != review_sha {
        return Err(protocol("the review's digest does not match"));
    }
    if mode == ImportMode::Adopt && adoptable {
        let model_view = &result[model.clone()];
        if !model_view.is_ascii() {
            return Err(protocol("the carried model is not ASCII"));
        }
        if Some(sha256_hex(model_view)) != model_sha {
            return Err(protocol("the carried model's digest does not match"));
        }
    }
    Ok(Decoded { imported, adoptable, model_sha, review_sha, model_bytes, model, review })
}

/// The review's blocking codes, read from its fixed members after the leading ones (`completeness`, `transfer_limit`,
/// `blocks`), in the order `ImportReviewSummary::blocking_codes` gives them.
fn blocking_codes(decoded: &Decoded, review: &[u8]) -> Vec<&'static str> {
    let window = &review[..review.len().min(PREFIX_BUFFER_BYTES)];
    let has = |needle: &[u8]| window.windows(needle.len()).any(|candidate| candidate == needle);
    let mut codes = Vec::new();
    if !decoded.imported || has(b"\"document_check\":true") {
        codes.push(NOT_ADOPTABLE);
    }
    if has(b"\"completeness\":\"truncated\"") {
        codes.push(REVIEW_TRUNCATED);
    }
    if has(b"\"assumption_unavailable\":true") {
        codes.push(ASSUMPTION_UNAVAILABLE);
    }
    if has(b"\"transfer_limit\":\"beyond\"") {
        codes.push(MODEL_TOO_LARGE);
    }
    if codes.is_empty() {
        codes.push(NOT_ADOPTABLE);
    }
    codes
}

// -- The run in the capped worker -------------------------------------------------------------------------------------

/// A completed result held until the response is built: its bytes, the worker's result lease and the command's bound,
/// shrunk to it.
pub(crate) struct Completed {
    pub(crate) result: Vec<u8>,
    _lease: Lease,
    _bound: LedgerBound,
}

struct Hooks<'c> {
    control: &'c RunControl,
    limit: u64,
}

impl SupervisorHooks for Hooks<'_> {
    fn attach(&mut self, worker: Arc<WorkerControl>) -> bool {
        let mut state = self.control.lock();
        state.worker = Some(worker);
        !state.cancelled
    }

    fn cancelled(&self) -> bool {
        self.control.cancelled()
    }

    fn grant_changed(&mut self, _grant_bytes: u64) {}

    fn footprint_observed(&mut self, _footprint_bytes: u64) {}

    fn result_limit(&self) -> Option<u64> {
        Some(self.limit)
    }
}

/// One command's context: the registry, its ledger-backed memory and the run's control.
pub(crate) struct Command<'r> {
    pub(crate) registry: &'r SolveJobRegistry,
    pub(crate) memory: &'r NativeLeaseMemory<'r>,
    pub(crate) args: &'r CommandLease,
    pub(crate) control: Arc<RunControl>,
    pub(crate) start_wait: Duration,
}

impl Command<'_> {
    fn memory_refusal(&self, refusal: ResourceRefusal) -> Stop {
        let message = match self.registry.refuse(refusal) {
            SolveJobError::ResourceUnavailable { refusal, .. } => {
                format!("The import's memory was refused ({}). Nothing was imported.", serde_json::to_string(&refusal).unwrap_or_else(|_| "refused".to_string()))
            }
            SolveJobError::InvalidRequest { message } => message,
        };
        refused(MEMORY_UNAVAILABLE, "", message)
    }

    fn lease(&self, what: &str, bytes: u64) -> Result<CommandLease, Stop> {
        self.memory.named_lease(what, bytes, "nothing was imported.").map_err(|message| refused(MEMORY_UNAVAILABLE, "", message))
    }

    /// §4.8.4 steps 1 to 6 for a request of `raw_len` bytes that `fill` writes into the request buffer (H4).
    fn run_worker(&self, mode: ImportMode, raw_len: u64, fill: impl FnOnce(&mut Vec<u8>) -> Result<(), Stop>) -> Result<Completed, Stop> {
        let registry = self.registry;
        let ledger = self.args.ledger();
        if self.control.cancelled() {
            return Err(Stop::Cancelled);
        }
        let reserve = registry.process_reserve();
        let request_bytes = allocation_envelope(raw_len).unwrap_or(u64::MAX);
        let result_share = resident_envelope(mode.result_limit());
        let start = initial_grant(reserve, resident_envelope(raw_len)).unwrap_or(u64::MAX);
        let budget = ledger.budget_bytes();
        let fixed = ledger.total_bytes().saturating_add(request_bytes).saturating_add(result_share);
        let max_grant = budget.saturating_sub(fixed);
        if start > max_grant {
            return Err(self.memory_refusal(ResourceRefusal::IngressExceedsBudget { required_bytes: fixed.saturating_add(start), budget_bytes: budget }));
        }
        // 1. The bound: H4, H5's maximum and H6's share, before any is admitted.
        let mut bound = ledger
            .bound(request_bytes + max_grant + result_share)
            .map_err(|over| self.memory_refusal(ResourceRefusal::IngressExceedsBudget { required_bytes: over.required_bytes, budget_bytes: over.budget_bytes }))?;
        // 2. The grant's queue wait, bounded, ended by cancellation or a newer run.
        let control = Arc::clone(&self.control);
        let deadline = Instant::now() + self.start_wait;
        let mut queued: Option<ResourceRefusal> = None;
        let waited = registry.pool().wait_acquire(
            start,
            LeasePurpose::Worker,
            Some(max_grant),
            &|| control.cancelled() || Instant::now() >= deadline,
            &mut |reason| queued = Some(reason.clone()),
        );
        let mut grant = match waited {
            Ok(grant) => grant,
            Err(WaitOutcome::Cancelled) if control.cancelled() => return Err(Stop::Cancelled),
            Err(WaitOutcome::Cancelled) => return Err(self.memory_refusal(queued.unwrap_or(ResourceRefusal::HeadroomUnknown))),
        };
        // 3. H4, admitted before it is made; 4. the read into it.
        let copy = match registry.acquire_interactive_ingress_capped(request_bytes) {
            Ok(copy) => copy,
            Err(SolveJobError::ResourceUnavailable { refusal, .. }) => return Err(self.memory_refusal(refusal)),
            Err(SolveJobError::InvalidRequest { message }) => return Err(refused(MEMORY_UNAVAILABLE, "", message)),
        };
        let mut raw = Vec::with_capacity(usize::try_from(raw_len).map_err(|_| refused(MEMORY_UNAVAILABLE, "", "The request is too large."))?);
        fill(&mut raw)?;
        if raw.len() as u64 != raw_len {
            return Err(refused(WORKER_PROTOCOL, "", "The import request's length is not its parts'."));
        }
        // 5. Supervision; the worker is reaped inside `supervise`, then the grant released.
        let terms = WorkerTerms { pool: registry.pool(), launcher: registry.worker_launcher(), task: WorkerTask::MbfImport, max_grant, reserve };
        let mut hooks = Hooks { control: &control, limit: mode.result_limit() };
        let outcome = catch_unwind(AssertUnwindSafe(|| supervise(&terms, &mut grant, (raw, copy), &mut hooks)));
        control.lock().worker = None;
        drop(grant);
        if control.cancelled() {
            drop(outcome);
            return Err(Stop::Cancelled);
        }
        match outcome {
            Err(payload) => Err(refused(WORKER_PROTOCOL, "", format!("The import worker's supervisor panicked: {}", panic_message(payload.as_ref())))),
            Ok(WorkerOutcome::Completed { attestation: Some(_), .. }) => Err(refused(WORKER_PROTOCOL, "", "Task 5 attests nothing, but its result was attested.")),
            Ok(WorkerOutcome::Completed { result, lease, attestation: None }) => {
                // 6. Result custody: the bound shrinks to the result lease; one above its share is refused.
                if lease.bytes() > result_share {
                    return Err(refused(WORKER_PROTOCOL, "", format!("The result lease of {} bytes exceeds the {result_share} bytes its bound holds.", lease.bytes())));
                }
                bound.shrink_to(lease.bytes());
                Ok(Completed { result, _lease: lease, _bound: bound })
            }
            Ok(WorkerOutcome::Failed(text)) => Err(task_failure(&text)),
            Ok(WorkerOutcome::Refused(refusal)) => Err(self.memory_refusal(refusal)),
            Ok(WorkerOutcome::ResultTooLarge { .. }) if mode == ImportMode::Adopt => Err(refused(MODEL_TOO_LARGE, "", "The imported model is above the transfer limit.")),
            Ok(WorkerOutcome::ResultTooLarge { .. }) => Err(refused(WORKER_PROTOCOL, "", "The review result is above its bound.")),
            Ok(WorkerOutcome::Terminated) => Err(refused(WORKER_PROTOCOL, "", "The import worker was stopped.")),
        }
    }
}

/// Task 5's own failure by its code; anything else is a protocol failure.
fn task_failure(text: &str) -> Stop {
    let own = [WORKER_REQUEST_INVALID, ENGINE_FAILED, MODEL_TOO_LARGE];
    match text.split_once(": ") {
        Some((code, message)) => match own.iter().find(|own| **own == code) {
            Some(code) => refused(code, "", message.to_string()),
            None => refused(WORKER_PROTOCOL, "", text.to_string()),
        },
        None => refused(WORKER_PROTOCOL, "", text.to_string()),
    }
}

// -- The commands' flows ----------------------------------------------------------------------------------------------

/// What a start replies, borrowing the held result's views (the file name, at most 1 KiB, is the reply's own).
pub(crate) enum Reply<'a> {
    Cancelled,
    Reviewed { source_id: u64, file_name: String, review_sha: [u8; 64], review: &'a [u8] },
    Opened { file_name: String, model: &'a [u8], review_sha: [u8; 64], model_sha: [u8; 64] },
    Refused { adopt: bool, file_name: String, diagnostics: Vec<Diag> },
}

impl Reply<'_> {
    fn write<O: Out>(&self, out: &mut O) {
        match self {
            Reply::Cancelled => out.put(b"{\"outcome\":\"cancelled\"}"),
            Reply::Reviewed { source_id, file_name, review_sha, review } => put_reviewed_reply(out, *source_id, file_name, review_sha, review),
            Reply::Opened { file_name, model, review_sha, model_sha } => put_adopt_reply(out, file_name, model, review_sha, model_sha),
            Reply::Refused { adopt, file_name, diagnostics } => put_refused_reply(out, *adopt, file_name, diagnostics),
        }
    }

    /// The exact serialized length, counted over borrowed views (H7's count).
    pub(crate) fn len(&self) -> u64 {
        let mut count = Counter(0);
        self.write(&mut count);
        count.0
    }
}

/// The held parts of a command until its response is built (the result lease and bound, H8, H7).
#[derive(Default)]
pub(crate) struct Held {
    pub(crate) completed: Option<Completed>,
    scratch: Option<CommandLease>,
    reply: Option<CommandLease>,
}

fn stop_reply(stop: Stop, adopt: bool, file_name: String) -> Reply<'static> {
    match stop {
        Stop::Cancelled => Reply::Refused { adopt, file_name, diagnostics: vec![diag(CANCELLED, "", CANCELLED_MESSAGE)] },
        Stop::Refused(diagnostics) => Reply::Refused { adopt, file_name, diagnostics },
    }
}

/// The request buffer's fixed parts and the files read into it, with the digests checked against `expected` (a re-read)
/// or returned (a selection).
fn fill_request(
    buffer: &mut Vec<u8>,
    mode: ImportMode,
    label: &str,
    declaration: &[u8],
    mbf_file: (&File, &Path, u64),
    identity: Option<(&File, &Path, u64)>,
    control: &RunControl,
) -> Result<([u8; 32], Option<[u8; 32]>), Stop> {
    push_u32(buffer, mode.code());
    push_u32(buffer, label.len() as u32);
    buffer.extend_from_slice(label.as_bytes());
    push_u32(buffer, declaration.len() as u32);
    buffer.extend_from_slice(declaration);
    push_u32(buffer, mbf_file.2 as u32);
    let mbf_sha = read_exact_pinned(mbf_file.0, mbf_file.1, mbf_file.2, buffer, control)?.ok_or_else(changed)?;
    let identity_sha = match identity {
        Some((file, path, size)) => {
            push_u32(buffer, size as u32);
            Some(read_exact_pinned(file, path, size, buffer, control)?.ok_or_else(changed)?)
        }
        None => {
            push_u32(buffer, IDENTITY_ABSENT);
            None
        }
    };
    Ok((mbf_sha, identity_sha))
}

/// Admits H8 and decodes the held result.
fn decode_held(command: &Command<'_>, mode: ImportMode, held: &mut Held) -> Result<Decoded, Stop> {
    held.scratch = Some(command.lease("the import decoder's control scratch", DECODER_SCRATCH_BYTES)?);
    let completed = held.completed.as_ref().expect("a completed result");
    decode_import_result(mode, &completed.result).map_err(|item| Stop::Refused(vec![item]))
}

/// `open_mbf_import` after admission and its ticket (§4.1 step 2, §4.8.2): the chooser, the selection, the first review
/// in task 5, the install.
fn open_flow<'h>(
    command: &Command<'_>,
    imports: &MbfImports,
    entry: ImportEntry,
    choose: impl FnOnce() -> Result<Option<PathBuf>, String>,
    held: &'h mut Held,
    file_name_out: &mut String,
) -> Result<Reply<'h>, Stop> {
    if command.control.cancelled() {
        return Err(Stop::Cancelled);
    }
    let chosen = match choose() {
        Ok(Some(path)) => path,
        Ok(None) => return Ok(Reply::Cancelled),
        Err(message) => return Err(refused(OPEN_UNREADABLE, "mbf", message)),
    };
    // A run retired while its chooser was open installs no source.
    if command.control.cancelled() {
        return Err(Stop::Cancelled);
    }
    let name = chosen.file_name().and_then(|name| name.to_str()).filter(|name| !name.is_empty() && name.len() <= LABEL_MAX_BYTES);
    let Some(name) = name else {
        return Err(refused(OPEN_NAME_INVALID, "mbf", "The chosen file's name is not UTF-8 or is longer than 1 KiB."));
    };
    *file_name_out = name.to_string();
    let Some(expected_path) = identity_path(&chosen) else {
        return Err(refused(OPEN_NAME_INVALID, "mbf", "The chosen file's name has no stem."));
    };
    let Some((mbf_file, mbf_id, mbf_size)) = acquire(&chosen, MBF_MAX_BYTES, "mbf", false, &command.control)? else {
        return Err(refused(OPEN_UNREADABLE, "mbf", "The chosen file was not found."));
    };
    let identity = acquire(&expected_path, IDENTITY_MAX_BYTES, "identity_file", true, &command.control)?;
    let label = file_name_out.as_str();
    let raw_len = request_len(label.len(), 0, mbf_size, identity.as_ref().map(|(_, _, size)| *size));
    let mut digests = None;
    let completed = command.run_worker(ImportMode::Review, raw_len, |buffer| {
        digests = Some(fill_request(
            buffer,
            ImportMode::Review,
            label,
            &[],
            (&mbf_file, &chosen, mbf_size),
            identity.as_ref().map(|(file, _, size)| (file, expected_path.as_path(), *size)),
            &command.control,
        )?);
        Ok(())
    })?;
    held.completed = Some(completed);
    let decoded = decode_held(command, ImportMode::Review, held)?;
    let (mbf_sha, identity_sha) = digests.expect("the request was filled");
    let mbf = Pinned { path: chosen.clone(), file: mbf_file, id: mbf_id, size: mbf_size, sha256: mbf_sha };
    let identity = match (identity, identity_sha) {
        (Some((file, id, size)), Some(sha256)) => IdentityFile::Present(Pinned { path: expected_path, file, id, size, sha256 }),
        _ => IdentityFile::Absent { expected_path },
    };
    let source_id = imports.install(&command.control, entry, label.to_string(), mbf, identity)?;
    let completed = held.completed.as_ref().expect("held");
    Ok(Reply::Reviewed { source_id, file_name: file_name_out.clone(), review_sha: decoded.review_sha, review: &completed.result[decoded.review] })
}

/// A re-read of the current source into a new request (§4.8.2 "Re-read") and task 5 in `mode`.
fn reread_flow(command: &Command<'_>, imports: &MbfImports, mode: ImportMode, source_id: u64, declaration: &[u8], held: &mut Held, file_name_out: &mut String) -> Result<Decoded, Stop> {
    let source = imports.take_source(source_id)?;
    *file_name_out = source.file_name.clone();
    // The name checks (truthful to the user); the digests below prove what is imported.
    if !names_pinned(&source.mbf) {
        return Err(changed());
    }
    let identity = match &source.identity {
        IdentityFile::Absent { expected_path } => {
            match fs::symlink_metadata(expected_path) {
                Err(error) if error.kind() == io::ErrorKind::NotFound => {}
                _ => return Err(changed()),
            }
            None
        }
        IdentityFile::Present(pinned) => {
            if !names_pinned(pinned) {
                return Err(changed());
            }
            Some(pinned)
        }
    };
    let raw_len = request_len(source.file_name.len(), declaration.len(), source.mbf.size, identity.map(|pinned| pinned.size));
    let completed = command.run_worker(mode, raw_len, |buffer| {
        let (mbf_sha, identity_sha) = fill_request(
            buffer,
            mode,
            &source.file_name,
            declaration,
            (&source.mbf.file, &source.mbf.path, source.mbf.size),
            identity.map(|pinned| (&pinned.file, pinned.path.as_path(), pinned.size)),
            &command.control,
        )?;
        if mbf_sha != source.mbf.sha256 || identity_sha != identity.map(|pinned| pinned.sha256) {
            return Err(changed());
        }
        if let IdentityFile::Absent { expected_path } = &source.identity {
            if !matches!(fs::symlink_metadata(expected_path), Err(error) if error.kind() == io::ErrorKind::NotFound) {
                return Err(changed());
            }
        }
        Ok(())
    })?;
    drop(source);
    held.completed = Some(completed);
    decode_held(command, mode, held)
}

// -- Declarations (H3) ------------------------------------------------------------------------------------------------

/// The declaration's shape check (§4.6) and its re-serialization for the worker (H3, its exact length counted and
/// leased first): `null`, or an object of MBF material names (1 to 512 bytes, never `*`) to exactly `{origin,
/// source_name}` with `origin` `project_invented` or `user_supplied` and a non-empty `source_name` of at most 512 bytes.
fn declaration_bytes(command: &Command<'_>, value: Option<serde_json::Value>) -> Result<(Vec<u8>, Option<CommandLease>), Stop> {
    let invalid = |what: &str| refused(DECLARATION_INVALID, "declaration", format!("The material declaration {what}."));
    let Some(value) = value else { return Ok((Vec::new(), None)) };
    let serde_json::Value::Object(map) = &value else { return Err(invalid("is not an object")) };
    for (name, entry) in map {
        if name.is_empty() || name.len() > mbf::REVIEW_FIELD_MAX_BYTES || name == "*" {
            return Err(invalid("names a material that cannot be declared"));
        }
        let serde_json::Value::Object(fields) = entry else { return Err(invalid("has an entry that is not an object")) };
        let origin = fields.get("origin").and_then(serde_json::Value::as_str);
        let source_name = fields.get("source_name").and_then(serde_json::Value::as_str);
        let shaped = fields.len() == 2
            && matches!(origin, Some("project_invented" | "user_supplied"))
            && source_name.is_some_and(|name| !name.trim().is_empty() && name.len() <= mbf::REVIEW_FIELD_MAX_BYTES);
        if !shaped {
            return Err(invalid("has an entry that is not {origin: project_invented or user_supplied, source_name}"));
        }
    }
    struct Measure(usize);
    impl io::Write for Measure {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            self.0 += bytes.len();
            Ok(bytes.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    let mut measure = Measure(0);
    serde_json::to_writer(&mut measure, &value).map_err(|_| invalid("cannot be serialized"))?;
    if measure.0 > DECLARATION_MAX_BYTES {
        return Err(invalid("is above 1 MiB"));
    }
    let lease = command.lease("the material declaration's copy for the worker", exact_text_bytes(measure.0 as u64))?;
    let mut bytes = Vec::with_capacity(measure.0);
    serde_json::to_writer(&mut bytes, &value).map_err(|_| invalid("cannot be serialized"))?;
    drop(value);
    Ok((bytes, Some(lease)))
}

// -- Admission and the command entry points ---------------------------------------------------------------------------

/// The executor's part of a start (C1's): the header and raw-body checks, the start's raw bound (4 KiB for Open, 1 MiB
/// otherwise; refused before any lease), and the raw copy reserved as a capped interactive lease before it is made.
pub(crate) fn reserve(registry: &SolveJobRegistry, body: &InvokeBody, headers: &HeaderMap, command: &'static str) -> Result<(Vec<u8>, Lease), SolveJobError> {
    let raw = command_ingress::raw_args_body(body, headers, command)?;
    let max = if command == OPEN_COMMAND { OPEN_ARGS_MAX_BYTES } else { ARGS_MAX_BYTES };
    if raw.len() > max {
        return Err(SolveJobError::InvalidRequest { message: format!("INGRESS-ARGS-TOO-LARGE: `{command}` takes at most {max} bytes of arguments.") });
    }
    let lease = registry.acquire_interactive_ingress_capped(allocation_envelope(raw.len() as u64).unwrap_or(u64::MAX))?;
    Ok((raw.to_vec(), lease))
}

fn admit<'r>(registry: &'r SolveJobRegistry, raw: Vec<u8>, copy: Lease, command: &'static str) -> Result<(NativeLeaseMemory<'r>, Args, CommandLease), SolveJobError> {
    #[cfg(test)]
    if let Some(hook) = test_hooks::BEFORE_ADMISSION.with(|hook| hook.borrow_mut().take()) {
        hook();
    }
    let (memory, copy) = NativeLeaseMemory::seeded(registry, copy);
    let copy_bytes = copy.bytes();
    let (args, lease) = command_ingress::admit_raw(registry, &raw, command, ArgsDecoding::Moved, |bytes| {
        let mut lease = copy;
        memory.grow(&mut lease, copy_bytes.saturating_add(bytes))?;
        Ok(lease)
    })?;
    drop(raw);
    Ok((memory, args, lease))
}

/// The members each start takes (§4.7); any other, a path included, is refused before anything else is read.
fn members(command: &'static str) -> &'static [&'static str] {
    match command {
        OPEN_COMMAND => &[EXPECTED_EPOCH, REQUEST_SEQUENCE],
        REVIEW_COMMAND => &[EXPECTED_EPOCH, REQUEST_SEQUENCE, "sourceId", "declaration"],
        _ => &[EXPECTED_EPOCH, REQUEST_SEQUENCE, "sourceId", "declaration", "reviewSha256"],
    }
}

/// Builds the response: H7 is counted over borrowed views, refused above its limit before any buffer, leased, then
/// written once; the response takes the buffer.
fn respond_with<R>(memory: &NativeLeaseMemory<'_>, reply: Reply<'_>, held: &mut Held, control: Option<&RunControl>, respond: impl FnOnce(String) -> R) -> R {
    let adopt_file = match &reply {
        Reply::Opened { file_name, .. } => Some(file_name.clone()),
        _ => None,
    };
    let mut reply = reply;
    let mut length = reply.len();
    if length > MBF_IMPORT_RESULT_MAX_BYTES {
        reply = Reply::Refused {
            adopt: adopt_file.is_some(),
            file_name: adopt_file.unwrap_or_default(),
            diagnostics: vec![diag(MODEL_TOO_LARGE, "", "The adopt reply is above the transfer limit.")],
        };
        length = reply.len();
    }
    let (adopt, file_name) = match &reply {
        Reply::Opened { file_name, .. } => (true, file_name.as_str()),
        Reply::Reviewed { file_name, .. } => (false, file_name.as_str()),
        Reply::Refused { adopt, file_name, .. } => (*adopt, file_name.as_str()),
        Reply::Cancelled => (false, ""),
    };
    let cancelled = diag(CANCELLED, "", CANCELLED_MESSAGE);
    let mut cancel_length = Counter(0);
    if control.is_some() {
        put_refused_reply(&mut cancel_length, adopt, file_name, std::slice::from_ref(&cancelled));
    }
    // Admission can wait. Reserve enough for either outcome before taking the final cancellation/handoff lock.
    match memory.named_lease("the import's reply", exact_text_bytes(length.max(cancel_length.0)), "nothing was imported.") {
        Ok(lease) => held.reply = Some(lease),
        Err(message) => {
            let adopt = matches!(reply, Reply::Opened { .. }) || matches!(reply, Reply::Refused { adopt: true, .. });
            let mut text = Vec::new();
            put_refused_reply(&mut text, adopt, "", &[diag(MEMORY_UNAVAILABLE, "", message)]);
            return respond(String::from_utf8(text).unwrap_or_default());
        }
    }
    let mut handoff = control.map(RunControl::lock);
    let was_cancelled = handoff.as_ref().is_some_and(|state| state.cancelled);
    let mut buffer = Vec::with_capacity(if was_cancelled { cancel_length.0 } else { length } as usize);
    if was_cancelled {
        put_refused_reply(&mut buffer, adopt, file_name, std::slice::from_ref(&cancelled));
    } else {
        reply.write(&mut buffer);
    }
    // Retirement is ordered with preparation of the final response. Once handed off, it cannot truthfully count
    // this command as stopped. No filesystem commit is added; frontend currency still guards session adoption.
    if let Some(state) = &mut handoff { state.handed_off = true; }
    drop(handoff);
    drop(reply);
    respond(String::from_utf8(buffer).expect("the reply is UTF-8"))
}

/// The admitted start: its members, its ticket, then `flow`.
fn start<R>(
    registry: &SolveJobRegistry,
    imports: &MbfImports,
    raw: Vec<u8>,
    copy: Lease,
    command_name: &'static str,
    start_wait: Duration,
    respond: impl FnOnce(String) -> R,
    flow: impl for<'h> FnOnce(&Command<'_>, ImportEntry, &mut Args, &'h mut Held, &mut String) -> Result<Reply<'h>, Stop>,
) -> Result<R, SolveJobError> {
    let adopt = command_name == ADOPT_COMMAND;
    let (memory, mut args, lease) = admit(registry, raw, copy, command_name)?;
    let mut held = Held::default();
    let early = |code, path, message: String| Reply::Refused { adopt, file_name: String::new(), diagnostics: vec![diag(code, path, message)] };
    let unknown = args.keys().find(|key| !members(command_name).contains(key)).map(|key| crate::stored_read::shown(key).into_owned());
    if let Some(unknown) = unknown {
        let reply = early(ARGS_INVALID, "", format!("`{command_name}` takes no member `{unknown}`."));
        drop(args);
        let response = respond_with(&memory, reply, &mut held, None, respond);
        drop(held);
        drop(lease);
        return Ok(response);
    }
    let entry = match imports.enter(args.opt_value(EXPECTED_EPOCH), args.opt_value(REQUEST_SEQUENCE)) {
        Ok(entry) => entry,
        Err(item) => {
            drop(args);
            let response = respond_with(&memory, Reply::Refused { adopt, file_name: String::new(), diagnostics: vec![item] }, &mut held, None, respond);
            drop(held);
            drop(lease);
            return Ok(response);
        }
    };
    let registration = imports.register(entry, registry);
    let command = Command { registry, memory: &memory, args: &lease, control: Arc::clone(&registration.control), start_wait };
    let mut file_name = String::new();
    // `held` is borrowed by the reply's views; H7's lease is kept apart in `reply_held`, released after the result
    // lease, the bound and H8 (§4.8.4 step 9).
    let mut reply_held = Held::default();
    let outcome = flow(&command, entry, &mut args, &mut held, &mut file_name);
    drop(args);
    let reply = match outcome {
        Ok(reply) => reply,
        Err(stop) => stop_reply(stop, adopt, std::mem::take(&mut file_name)),
    };
    let response = respond_with(&memory, reply, &mut reply_held, Some(&command.control), respond);
    drop(held);
    drop(reply_held);
    drop(command);
    drop(registration);
    drop(lease);
    Ok(response)
}

fn source_id_arg(args: &mut Args) -> Result<u64, Stop> {
    match args.opt_value("sourceId") {
        Some(serde_json::Value::Number(number)) => number.as_u64().filter(|id| *id > 0).ok_or_else(|| refused(ARGS_INVALID, "sourceId", "`sourceId` must be a positive integer.")),
        _ => Err(refused(ARGS_INVALID, "sourceId", "`sourceId` must be a positive integer.")),
    }
}

/// `open_mbf_import` after [`reserve`].
pub(crate) fn open_command<R>(
    registry: &SolveJobRegistry,
    imports: &MbfImports,
    raw: Vec<u8>,
    copy: Lease,
    start_wait: Duration,
    choose: impl FnOnce() -> Result<Option<PathBuf>, String>,
    respond: impl FnOnce(String) -> R,
) -> Result<R, SolveJobError> {
    start(registry, imports, raw, copy, OPEN_COMMAND, start_wait, respond, |command, entry, _args, held, file_name| open_flow(command, imports, entry, choose, held, file_name))
}

/// `review_mbf_import` after [`reserve`].
pub(crate) fn review_command<R>(registry: &SolveJobRegistry, imports: &MbfImports, raw: Vec<u8>, copy: Lease, start_wait: Duration, respond: impl FnOnce(String) -> R) -> Result<R, SolveJobError> {
    start(registry, imports, raw, copy, REVIEW_COMMAND, start_wait, respond, |command, _entry, args, held, file_name| {
        let source_id = source_id_arg(args)?;
        if !args.keys().any(|key| key == "declaration") {
            return Err(refused(ARGS_INVALID, "declaration", "`declaration` is required (null for none)."));
        }
        let (declaration, _h3) = declaration_bytes(command, args.opt_value("declaration"))?;
        let decoded = reread_flow(command, imports, ImportMode::Review, source_id, &declaration, held, file_name)?;
        let completed = held.completed.as_ref().expect("held");
        Ok(Reply::Reviewed { source_id, file_name: file_name.clone(), review_sha: decoded.review_sha, review: &completed.result[decoded.review] })
    })
}

/// `adopt_mbf_import` after [`reserve`] (§4.8.7 step 2): no disk commit; a stale review or a non-adoptable result
/// returns no model.
pub(crate) fn adopt_command<R>(registry: &SolveJobRegistry, imports: &MbfImports, raw: Vec<u8>, copy: Lease, start_wait: Duration, respond: impl FnOnce(String) -> R) -> Result<R, SolveJobError> {
    start(registry, imports, raw, copy, ADOPT_COMMAND, start_wait, respond, |command, _entry, args, held, file_name| {
        let source_id = source_id_arg(args)?;
        let requested = match args.opt_value("reviewSha256") {
            Some(serde_json::Value::String(text)) => sha_token(text.as_bytes()).map_err(|_| refused(ARGS_INVALID, "reviewSha256", "`reviewSha256` must be 64 lowercase hexadecimal digits."))?,
            _ => return Err(refused(ARGS_INVALID, "reviewSha256", "`reviewSha256` must be 64 lowercase hexadecimal digits.")),
        };
        if !args.keys().any(|key| key == "declaration") {
            return Err(refused(ARGS_INVALID, "declaration", "`declaration` is required (null for none)."));
        }
        let (declaration, _h3) = declaration_bytes(command, args.opt_value("declaration"))?;
        let decoded = reread_flow(command, imports, ImportMode::Adopt, source_id, &declaration, held, file_name)?;
        if decoded.review_sha != requested {
            return Err(refused(REVIEW_STALE, "reviewSha256", "The import's review is no longer the one shown. Review it again."));
        }
        let completed = held.completed.as_ref().expect("held");
        if !decoded.adoptable {
            let codes = blocking_codes(&decoded, &completed.result[decoded.review.clone()]);
            return Err(Stop::Refused(codes.into_iter().map(|code| diag(code, "", "The reviewed import cannot be adopted.")).collect()));
        }
        // The command checks its cancellation once more before the reply is built; after that nothing is left to cancel.
        if command.control.cancelled() {
            return Err(Stop::Cancelled);
        }
        Ok(Reply::Opened {
            file_name: file_name.clone(),
            model: &completed.result[decoded.model.clone()],
            review_sha: decoded.review_sha,
            model_sha: decoded.model_sha.expect("an adoptable result has a model"),
        })
    })
}

/// A start whose blocking part did not complete: a fixed refusal.
pub(crate) fn worker_failed(adopt: bool) -> tauri::ipc::Response {
    let mut text = Vec::new();
    put_refused_reply(&mut text, adopt, "", &[diag(WORKER_PROTOCOL, "", "The import's worker did not complete. Nothing was imported.")]);
    tauri::ipc::Response::new(String::from_utf8(text).unwrap_or_default())
}

/// `cancel_mbf_import`: `{retirementId}` as Tauri's JSON arguments (what the barrier sends; it survives the
/// `postMessage` fallback) or as a raw body (admitted within 4 KiB), one keyed path (the export's `cancel_command`
/// rules, on this module's state).
pub(crate) fn cancel_command(registry: &SolveJobRegistry, imports: &MbfImports, body: &InvokeBody, headers: &HeaderMap) -> Result<MbfImportRetirement, CommandError<String>> {
    let (id, lease) = match body {
        InvokeBody::Json(args) if headers.get(command_ingress::INGRESS_HEADER).is_none() => (RetirementId::read(cancel_json_args(args)?), None),
        _ => {
            let (mut args, lease) = command_ingress::admit_body_within(registry, body, headers, CANCEL_COMMAND, ArgsDecoding::Moved, CANCEL_ARGS_MAX_BYTES)?;
            (RetirementId::read(args.opt_value(RETIREMENT_ID)), Some(lease))
        }
    };
    let reply = id.map(|id| imports.retire_keyed(id, registry)).map_err(|item| CommandError::Command(format!("{}: {}", item.code, item.message)));
    drop(lease);
    reply
}

fn cancel_json_args(args: &serde_json::Value) -> Result<Option<serde_json::Value>, SolveJobError> {
    struct Bounded(usize);
    impl io::Write for Bounded {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            self.0 = self.0.saturating_add(bytes.len());
            if self.0 > CANCEL_ARGS_MAX_BYTES {
                return Err(io::ErrorKind::FileTooLarge.into());
            }
            Ok(bytes.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    if serde_json::to_writer(&mut Bounded(0), args).is_err() {
        return Err(SolveJobError::InvalidRequest { message: format!("INGRESS-ARGS-TOO-LARGE: `{CANCEL_COMMAND}` takes at most {CANCEL_ARGS_MAX_BYTES} bytes of arguments.") });
    }
    let serde_json::Value::Object(map) = args else {
        return Err(SolveJobError::InvalidRequest { message: format!("INGRESS-ARGS-NOT-OBJECT: `{CANCEL_COMMAND}`'s arguments must be one JSON object.") });
    };
    Ok(map.get(RETIREMENT_ID).cloned())
}

#[cfg(test)]
#[path = "mbf_import_tests.rs"]
mod tests;
