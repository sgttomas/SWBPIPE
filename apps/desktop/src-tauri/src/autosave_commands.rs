//! Autosave commands and their settings (D5 slice 2; DEL-02-05 Design
//! `autosave-and-backup.md` §2, §3 and §6).
//!
//! The commands wrap slice 1's `autosave.rs`: `write_project_autosave` (a
//! model-sized command, admitted with its whole body), `list_project_autosaves`,
//! `read_project_autosave` (the verified read for recovery) and
//! `discard_project_autosave` (within the history commands' 256 KiB), and the
//! settings read and write `get_autosave_settings` and `set_autosave_settings`
//! (within 4 KiB). Every command's memory is seeded with its ingress lease
//! (`PoolReadMemory`), and slice 1's functions reserve their own terms in it;
//! the response is serialized within it (`respond_metered`). Each command runs
//! on a blocking worker, so a write's fsyncs or a slot lock's wait never hold
//! the main thread.
//!
//! The settings are a native file, `<app local data>/autosave-settings.json`,
//! written by the same atomic writer. A missing file reads as the default (on,
//! every 5 minutes); an unreadable, unknown-version or out-of-range file reads
//! as the default with a status naming it, and is rewritten only when the user
//! changes a setting. `backup_folder` and `backup_keep` are carried and are
//! null until backups exist (slice 3). No command here takes a path from the
//! webview: every path is the app's own local data directory.
//!
//! This module holds no `unsafe` code: every system call it needs is in
//! `autosave.rs` or the standard library.

use crate::atomic_report_package_save::{atomic_save_bytes_as, AtomicSaveFormat};
use crate::autosave::{self, AutosaveDiscardOutcome, AutosaveEntry, AutosaveWriteOutcome, AUTOSAVE_DIRECTORY};
use crate::command_ingress::{self, Args, ArgsDecoding, CommandError};
use crate::export_disposition_screen::{diagnostic, DispositionDiagnostic};
use crate::memory_pool::Lease;
use crate::model_document_save::writer_diagnostic;
use crate::solve_jobs::{SolveJobError, SolveJobRegistry};
use crate::stored_read::{self, loaded_text_bytes, Metered, PoolReadMemory, ReadMemory};
use serde::Serialize;
use serde_json::Value;
use std::fs;
use std::io::{self, Read};
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::http::HeaderMap;
use tauri::ipc::InvokeBody;
use tauri::{AppHandle, Manager};

pub(crate) const WRITE_COMMAND: &str = "write_project_autosave";
pub(crate) const LIST_COMMAND: &str = "list_project_autosaves";
pub(crate) const READ_COMMAND: &str = "read_project_autosave";
pub(crate) const DISCARD_COMMAND: &str = "discard_project_autosave";
pub(crate) const GET_SETTINGS_COMMAND: &str = "get_autosave_settings";
pub(crate) const SET_SETTINGS_COMMAND: &str = "set_autosave_settings";

/// The record commands' argument budget: the history commands' (§6).
pub(crate) const RECORD_COMMAND_MAX_ARGS_BYTES: usize = crate::model_states::HISTORY_COMMAND_MAX_ARGS_BYTES;
/// The settings commands' argument budget (§6: scalars).
pub(crate) const SETTINGS_COMMAND_MAX_ARGS_BYTES: usize = 4 * 1024;

/// A worker that did not complete.
pub(crate) const WORKER_FAILED: &str = "AUTOSAVE-WORKER-FAILED";

// ---------------------------------------------------------------------------
// Admission.
// ---------------------------------------------------------------------------

/// The raw-body budget of each command: `None` for the model-sized write,
/// which is admitted with its whole body (envelope + M + 4 KiB).
pub(crate) fn args_budget(command: &str) -> Option<usize> {
    match command {
        WRITE_COMMAND => None,
        GET_SETTINGS_COMMAND | SET_SETTINGS_COMMAND => Some(SETTINGS_COMMAND_MAX_ARGS_BYTES),
        _ => Some(RECORD_COMMAND_MAX_ARGS_BYTES),
    }
}

/// Admits one autosave command's raw body under its rule (§6).
pub(crate) fn admit_body(
    registry: &SolveJobRegistry,
    body: &InvokeBody,
    headers: &HeaderMap,
    command: &'static str,
) -> Result<(Args, Lease), SolveJobError> {
    match args_budget(command) {
        None => command_ingress::admit_body(registry, body, headers, command, ArgsDecoding::Moved),
        Some(max) => command_ingress::admit_body_within(registry, body, headers, command, ArgsDecoding::Moved, max),
    }
}

/// The app's local data directory: the store file's directory, which holds
/// `autosave/` and the settings file.
fn data_directory(app: &AppHandle) -> Result<PathBuf, String> {
    app.path().app_local_data_dir().map_err(|error| error.to_string())
}

fn now_unix_millis() -> u64 {
    // A clock before the epoch reads as out of range, which the write refuses
    // (`AUTOSAVE-CLOCK-INVALID`).
    SystemTime::now().duration_since(UNIX_EPOCH).map(|elapsed| u64::try_from(elapsed.as_millis()).unwrap_or(u64::MAX)).unwrap_or(u64::MAX)
}

fn refused_one(code: &str, message: impl Into<String>) -> Vec<DispositionDiagnostic> {
    vec![diagnostic(code, "", message)]
}

// ---------------------------------------------------------------------------
// Records.
// ---------------------------------------------------------------------------

/// `{canonicalText, claimedModelHash}`, both moved out of the body.
#[derive(Debug)]
pub(crate) struct WriteRequest {
    pub(crate) canonical_text: String,
    pub(crate) claimed_model_hash: String,
}

pub(crate) fn write_request(args: &mut Args) -> Result<WriteRequest, SolveJobError> {
    Ok(WriteRequest { canonical_text: args.decode("canonicalText")?, claimed_model_hash: args.decode("claimedModelHash")? })
}

/// Writes the session's text as its slot's newest record in
/// `<data>/autosave/` (the data directory is created if absent, as the store's
/// is). The text is held, as part of the ingress lease, until the write returns.
pub(crate) fn write_at(data: Result<PathBuf, String>, request: WriteRequest, now_millis: u64, memory: &dyn ReadMemory) -> AutosaveWriteOutcome {
    let data = match data.and_then(|data| fs::create_dir_all(&data).map(|()| data).map_err(|error| error.to_string())) {
        Ok(data) => data,
        Err(error) => {
            return AutosaveWriteOutcome::Refused {
                diagnostics: refused_one(autosave::DIRECTORY_UNAVAILABLE, format!("The app's local data directory is unavailable: {error}. Nothing was written.")),
            }
        }
    };
    autosave::write_autosave(&data.join(AUTOSAVE_DIRECTORY), &request.canonical_text, &request.claimed_model_hash, now_millis, memory)
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub(crate) enum AutosaveListOutcome {
    Listed { entries: Vec<AutosaveEntry>, not_listed: usize },
    Refused { diagnostics: Vec<DispositionDiagnostic> },
}

/// The records, newest first (names and sizes only; no content is read).
pub(crate) fn list_at(data: Result<PathBuf, String>, memory: &dyn ReadMemory) -> Metered<AutosaveListOutcome> {
    let refused = |diagnostics| Metered::new(AutosaveListOutcome::Refused { diagnostics });
    let data = match data {
        Ok(data) => data,
        Err(error) => return refused(refused_one(autosave::DIRECTORY_UNAVAILABLE, format!("The app's local data directory is unavailable: {error}."))),
    };
    match autosave::list_autosaves(&data.join(AUTOSAVE_DIRECTORY), memory) {
        Ok(listing) => listing.map(|listing| AutosaveListOutcome::Listed { entries: listing.entries, not_listed: listing.not_listed }),
        Err(refusal) => refused(vec![refusal]),
    }
}

/// `{fileName}`.
pub(crate) fn file_name_request(args: &mut Args) -> Result<String, SolveJobError> {
    args.decode("fileName")
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub(crate) enum AutosaveReadOutcome {
    /// Every check of §5 passed.
    Read { file_name: String, slot: String, written_unix_millis: u64, model_hash: String, byte_count: u64, document: Value },
    Refused { diagnostics: Vec<DispositionDiagnostic> },
}

/// One record, read verified for recovery (§3, §5).
pub(crate) fn read_at(data: Result<PathBuf, String>, file_name: &str, memory: &dyn ReadMemory) -> Metered<AutosaveReadOutcome> {
    let refused = |diagnostics| Metered::new(AutosaveReadOutcome::Refused { diagnostics });
    let data = match data {
        Ok(data) => data,
        Err(error) => return refused(refused_one(autosave::DIRECTORY_UNAVAILABLE, format!("The app's local data directory is unavailable: {error}."))),
    };
    match autosave::read_autosave(&data.join(AUTOSAVE_DIRECTORY), file_name, memory) {
        Ok(verified) => verified.map(|record| AutosaveReadOutcome::Read {
            file_name: record.file_name,
            slot: record.slot,
            written_unix_millis: record.written_unix_millis,
            model_hash: record.model_hash,
            byte_count: record.byte_count,
            document: record.document,
        }),
        Err(diagnostics) => refused(diagnostics),
    }
}

/// Removes one record the user chose to discard.
pub(crate) fn discard_at(data: Result<PathBuf, String>, file_name: &str) -> AutosaveDiscardOutcome {
    match data {
        Ok(data) => autosave::discard_autosave(&data.join(AUTOSAVE_DIRECTORY), file_name),
        Err(error) => AutosaveDiscardOutcome::Refused {
            diagnostics: refused_one(autosave::DIRECTORY_UNAVAILABLE, format!("The app's local data directory is unavailable: {error}. Nothing was removed.")),
        },
    }
}

// ---------------------------------------------------------------------------
// Settings (§2).
// ---------------------------------------------------------------------------

pub(crate) const SETTINGS_FILE: &str = "autosave-settings.json";
pub(crate) const SETTINGS_KIND: &str = "swbpipe.desktop.autosave_settings";
pub(crate) const SETTINGS_VERSION: u64 = 1;
/// On by default, every 5 minutes (Agent 0, 2026-10-28, row RT-U3).
pub(crate) const DEFAULT_AUTOSAVE_MINUTES: u32 = 5;
/// The interval choices; 0 is off.
pub(crate) const AUTOSAVE_MINUTE_CHOICES: [u32; 7] = [0, 1, 2, 5, 10, 15, 30];
/// The rotation choices of slice 3; null is All.
pub(crate) const BACKUP_KEEP_CHOICES: [u32; 5] = [3, 5, 10, 20, 50];
/// The largest settings file read; a larger one is unreadable.
pub(crate) const MAX_SETTINGS_FILE_BYTES: u64 = 4096;

pub(crate) const SETTINGS_UNREADABLE: &str = "AUTOSAVE-SETTINGS-UNREADABLE";
pub(crate) const SETTINGS_VERSION_UNKNOWN: &str = "AUTOSAVE-SETTINGS-VERSION-UNKNOWN";
pub(crate) const SETTINGS_OUT_OF_RANGE: &str = "AUTOSAVE-SETTINGS-OUT-OF-RANGE";

/// The settings file's own admission to the atomic writer.
pub(crate) const SETTINGS_FORMAT: AtomicSaveFormat = AtomicSaveFormat {
    extension: "json",
    code_prefix: "AUTOSAVE-SETTINGS",
    temp_prefix: ".swbpipe-autosave-settings-",
    log_label: "autosave settings",
};

/// The settings, in the file's member order.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub(crate) struct AutosaveSettings {
    pub(crate) autosave_minutes: u32,
    pub(crate) backup_folder: Option<String>,
    pub(crate) backup_keep: Option<u32>,
}

impl AutosaveSettings {
    pub(crate) fn default_settings() -> Self {
        Self { autosave_minutes: DEFAULT_AUTOSAVE_MINUTES, backup_folder: None, backup_keep: None }
    }
}

#[derive(Serialize)]
struct SettingsFile<'a> {
    document_kind: &'static str,
    version: u64,
    autosave_minutes: u32,
    backup_folder: &'a Option<String>,
    backup_keep: Option<u32>,
}

/// The settings file's exact text for `settings`.
pub(crate) fn settings_text(settings: &AutosaveSettings) -> String {
    serde_json::to_string(&SettingsFile {
        document_kind: SETTINGS_KIND,
        version: SETTINGS_VERSION,
        autosave_minutes: settings.autosave_minutes,
        backup_folder: &settings.backup_folder,
        backup_keep: settings.backup_keep,
    })
    .unwrap_or_default()
}

/// What the settings read found: the settings in force, whether they came
/// from the file or are the default, and why a file present was not used.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub(crate) struct AutosaveSettingsRead {
    pub(crate) settings: AutosaveSettings,
    /// `file` or `default`.
    pub(crate) source: &'static str,
    /// Absent when the file was read, or is missing.
    pub(crate) status: Option<DispositionDiagnostic>,
}

impl AutosaveSettingsRead {
    fn defaulted(status: Option<DispositionDiagnostic>) -> Self {
        Self { settings: AutosaveSettings::default_settings(), source: "default", status }
    }
}

const SETTINGS_MEMBERS: [&str; 5] = ["document_kind", "version", "autosave_minutes", "backup_folder", "backup_keep"];

fn fallback(code: &str, detail: impl std::fmt::Display) -> DispositionDiagnostic {
    diagnostic(
        code,
        "",
        format!("The autosave settings file {detail}; autosave uses the default, every {DEFAULT_AUTOSAVE_MINUTES} minutes. The file is rewritten only when a setting is changed."),
    )
}

/// The settings a file's bytes hold, or the reason they are not used.
pub(crate) fn settings_from_bytes(bytes: &[u8]) -> Result<AutosaveSettings, DispositionDiagnostic> {
    let value: Value = serde_json::from_slice(bytes).map_err(|error| fallback(SETTINGS_UNREADABLE, format_args!("is not JSON ({})", stored_read::json_error(&error))))?;
    let Value::Object(members) = value else {
        return Err(fallback(SETTINGS_UNREADABLE, "is not a JSON object"));
    };
    match members.get("version") {
        Some(version) if version.as_u64() == Some(SETTINGS_VERSION) => {}
        Some(version) => return Err(fallback(SETTINGS_VERSION_UNKNOWN, format_args!("has version {version}, which this app does not read"))),
        None => return Err(fallback(SETTINGS_VERSION_UNKNOWN, "has no version")),
    }
    if members.get("document_kind").and_then(Value::as_str) != Some(SETTINGS_KIND) {
        return Err(fallback(SETTINGS_UNREADABLE, format_args!("is not a `{SETTINGS_KIND}` document")));
    }
    if let Some(unknown) = members.keys().find(|key| !SETTINGS_MEMBERS.contains(&key.as_str())) {
        return Err(fallback(SETTINGS_UNREADABLE, format_args!("has an unknown member `{}`", stored_read::shown(unknown))));
    }
    let autosave_minutes = members
        .get("autosave_minutes")
        .and_then(Value::as_u64)
        .and_then(|minutes| u32::try_from(minutes).ok())
        .filter(|minutes| AUTOSAVE_MINUTE_CHOICES.contains(minutes))
        .ok_or_else(|| {
            let shown = members.get("autosave_minutes").map_or_else(|| "absent".to_string(), Value::to_string);
            fallback(SETTINGS_OUT_OF_RANGE, format_args!("sets autosave_minutes to {}, not 0 (off), 1, 2, 5, 10, 15 or 30", stored_read::shown(&shown)))
        })?;
    let backup_folder = match members.get("backup_folder") {
        None | Some(Value::Null) => None,
        Some(Value::String(folder)) => Some(folder.clone()),
        Some(_) => return Err(fallback(SETTINGS_UNREADABLE, "has a backup_folder that is not a path or null")),
    };
    let backup_keep = match members.get("backup_keep") {
        None | Some(Value::Null) => None,
        Some(keep) => Some(
            keep.as_u64()
                .and_then(|keep| u32::try_from(keep).ok())
                .filter(|keep| BACKUP_KEEP_CHOICES.contains(keep))
                .ok_or_else(|| fallback(SETTINGS_OUT_OF_RANGE, format_args!("sets backup_keep to {}, not null (all), 3, 5, 10, 20 or 50", stored_read::shown(&keep.to_string()))))?,
        ),
    };
    Ok(AutosaveSettings { autosave_minutes, backup_folder, backup_keep })
}

/// Reads the settings file in `data` (§2). It must be a regular file (a
/// symlink is not followed) of at most [`MAX_SETTINGS_FILE_BYTES`], read with
/// a hard cap; its load is reserved in `memory` first.
pub(crate) fn read_settings(data: &Path, memory: &dyn ReadMemory) -> Metered<AutosaveSettingsRead> {
    let unreadable = |detail: String| Metered::new(AutosaveSettingsRead::defaulted(Some(fallback(SETTINGS_UNREADABLE, detail))));
    let path = data.join(SETTINGS_FILE);
    match fs::symlink_metadata(&path) {
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Metered::new(AutosaveSettingsRead::defaulted(None)),
        Err(error) => return unreadable(format!("could not be inspected ({error})")),
        Ok(metadata) if !metadata.file_type().is_file() => return unreadable("is not a regular file".to_string()),
        Ok(metadata) if metadata.len() > MAX_SETTINGS_FILE_BYTES => {
            return unreadable(format!("is {} bytes, more than the {MAX_SETTINGS_FILE_BYTES} a settings file may be", metadata.len()))
        }
        Ok(_) => {}
    }
    let reservation = match memory.reserve("the autosave settings file", loaded_text_bytes(MAX_SETTINGS_FILE_BYTES + 1)) {
        Ok(reservation) => reservation,
        Err(message) => return Metered::new(AutosaveSettingsRead::defaulted(Some(diagnostic(stored_read::STORE_READ_MEMORY_UNAVAILABLE, "", message)))),
    };
    let file = match fs::OpenOptions::new().read(true).custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK).open(&path) {
        Ok(file) => file,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Metered::new(AutosaveSettingsRead::defaulted(None)),
        Err(error) => return unreadable(format!("could not be opened ({error})")),
    };
    if !file.metadata().is_ok_and(|metadata| metadata.is_file()) {
        return unreadable("is not a regular file".to_string());
    }
    let mut bytes = Vec::with_capacity(MAX_SETTINGS_FILE_BYTES as usize + 1);
    if let Err(error) = file.take(MAX_SETTINGS_FILE_BYTES + 1).read_to_end(&mut bytes) {
        return unreadable(format!("could not be read ({error})"));
    }
    if bytes.len() as u64 > MAX_SETTINGS_FILE_BYTES {
        return unreadable(format!("grew beyond the {MAX_SETTINGS_FILE_BYTES} bytes a settings file may be"));
    }
    let read = match settings_from_bytes(&bytes) {
        Ok(settings) => AutosaveSettingsRead { settings, source: "file", status: None },
        Err(status) => AutosaveSettingsRead::defaulted(Some(status)),
    };
    Metered::new(read).holding(reservation)
}

pub(crate) fn get_settings_at(data: Result<PathBuf, String>, memory: &dyn ReadMemory) -> Metered<AutosaveSettingsRead> {
    match data {
        Ok(data) => read_settings(&data, memory),
        Err(error) => Metered::new(AutosaveSettingsRead::defaulted(Some(fallback(SETTINGS_UNREADABLE, format_args!("could not be located ({error})"))))),
    }
}

/// `{autosaveMinutes}`.
pub(crate) fn set_settings_request(args: &mut Args) -> Result<u32, SolveJobError> {
    args.decode("autosaveMinutes")
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub(crate) enum AutosaveSettingsWriteOutcome {
    Saved { settings: AutosaveSettings },
    Refused { diagnostics: Vec<DispositionDiagnostic> },
}

/// Sets the interval (0 is off) and writes the settings file with the atomic
/// writer. A value that is not a choice is refused and nothing is written. The
/// backup members are kept as a readable file holds them, and are otherwise null.
pub(crate) fn set_settings_at(data: Result<PathBuf, String>, autosave_minutes: u32, memory: &dyn ReadMemory) -> AutosaveSettingsWriteOutcome {
    if !AUTOSAVE_MINUTE_CHOICES.contains(&autosave_minutes) {
        return AutosaveSettingsWriteOutcome::Refused {
            diagnostics: refused_one(SETTINGS_OUT_OF_RANGE, format!("Autosave every {autosave_minutes} minutes is not a choice (0 for off, 1, 2, 5, 10, 15 or 30); nothing was written.")),
        };
    }
    let data = match data.and_then(|data| fs::create_dir_all(&data).map(|()| data).map_err(|error| error.to_string())) {
        Ok(data) => data,
        Err(error) => {
            return AutosaveSettingsWriteOutcome::Refused {
                diagnostics: refused_one(autosave::DIRECTORY_UNAVAILABLE, format!("The app's local data directory is unavailable: {error}. Nothing was written.")),
            }
        }
    };
    let current = read_settings(&data, memory);
    let mut settings = if current.get().source == "file" { current.get().settings.clone() } else { AutosaveSettings::default_settings() };
    drop(current);
    settings.autosave_minutes = autosave_minutes;
    match atomic_save_bytes_as(&SETTINGS_FORMAT, &data.join(SETTINGS_FILE), settings_text(&settings).as_bytes()) {
        Ok(_) => AutosaveSettingsWriteOutcome::Saved { settings },
        Err(error) => AutosaveSettingsWriteOutcome::Refused { diagnostics: vec![writer_diagnostic(error)] },
    }
}

// ---------------------------------------------------------------------------
// The Tauri commands.
// ---------------------------------------------------------------------------

/// A command's memory: the pool, counted from its ingress lease (§6: "seeded
/// with the lease"), so each reservation is admitted with the lease held.
pub(crate) fn seeded_memory<'a>(registry: &'a SolveJobRegistry, lease: &Lease) -> PoolReadMemory<'a> {
    PoolReadMemory::new(registry, lease)
}

/// Runs `work` on a blocking worker with the command's memory, seeded from
/// its lease, and serializes the response within that memory.
async fn on_worker<T: Serialize + Send + 'static>(
    app: AppHandle,
    command: &'static str,
    lease: Lease,
    work: impl FnOnce(&AppHandle, &dyn ReadMemory) -> Metered<T> + Send + 'static,
) -> Result<tauri::ipc::Response, CommandError> {
    tauri::async_runtime::spawn_blocking(move || {
        let registry = app.state::<SolveJobRegistry>();
        let memory = seeded_memory(&registry, &lease);
        let outcome = work(&app, &memory);
        let response = stored_read::respond_metered(&memory, command, Ok(outcome));
        drop(memory);
        drop(lease);
        response
    })
    .await
    .unwrap_or_else(|_| Err(CommandError::Command(format!("{WORKER_FAILED}: the `{command}` worker did not complete."))))
}

/// The session's model document text as its slot's newest record (§1, §5).
#[tauri::command]
pub async fn write_project_autosave(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, lease) = admit_body(&registry, invoke.body(), invoke.headers(), WRITE_COMMAND)?;
    let request = write_request(&mut args)?;
    drop(args);
    on_worker(app, WRITE_COMMAND, lease, move |app, memory| Metered::new(write_at(data_directory(app), request, now_unix_millis(), memory))).await
}

/// The records, names and sizes only (§3).
#[tauri::command]
pub async fn list_project_autosaves(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (args, lease) = admit_body(&registry, invoke.body(), invoke.headers(), LIST_COMMAND)?;
    drop(args);
    on_worker(app, LIST_COMMAND, lease, |app, memory| list_at(data_directory(app), memory)).await
}

/// One record, read verified for recovery (§3, §5).
#[tauri::command]
pub async fn read_project_autosave(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, lease) = admit_body(&registry, invoke.body(), invoke.headers(), READ_COMMAND)?;
    let file_name = file_name_request(&mut args)?;
    drop(args);
    on_worker(app, READ_COMMAND, lease, move |app, memory| read_at(data_directory(app), &file_name, memory)).await
}

/// Removes one record the user chose to discard (§3).
#[tauri::command]
pub async fn discard_project_autosave(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, lease) = admit_body(&registry, invoke.body(), invoke.headers(), DISCARD_COMMAND)?;
    let file_name = file_name_request(&mut args)?;
    drop(args);
    on_worker(app, DISCARD_COMMAND, lease, move |app, _| Metered::new(discard_at(data_directory(app), &file_name))).await
}

/// The settings in force (§2).
#[tauri::command]
pub async fn get_autosave_settings(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (args, lease) = admit_body(&registry, invoke.body(), invoke.headers(), GET_SETTINGS_COMMAND)?;
    drop(args);
    on_worker(app, GET_SETTINGS_COMMAND, lease, |app, memory| get_settings_at(data_directory(app), memory)).await
}

/// Sets the interval, or off (§2).
#[tauri::command]
pub async fn set_autosave_settings(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, lease) = admit_body(&registry, invoke.body(), invoke.headers(), SET_SETTINGS_COMMAND)?;
    let minutes = set_settings_request(&mut args)?;
    drop(args);
    on_worker(app, SET_SETTINGS_COMMAND, lease, move |app, memory| Metered::new(set_settings_at(data_directory(app), minutes, memory))).await
}

#[cfg(test)]
#[path = "autosave_commands_tests.rs"]
mod tests;
