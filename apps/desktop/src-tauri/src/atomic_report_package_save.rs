//! The desktop's atomic file writer: same-directory temporary file, write,
//! flush, file sync, rename over the destination, bounded parent-directory
//! sync. It is extension-neutral; each caller names its format
//! ([`AtomicSaveFormat`]), whose one admitted extension, diagnostic code
//! prefix and owned temporary-file prefix the writer applies. The report
//! package save (`.opsproj`, [`REPORT_PACKAGE_FORMAT`]) keeps exactly its
//! former checks, codes and temporary names; File > Save Model Document…
//! admits `.json` ([`MODEL_DOCUMENT_FORMAT`], DEL-15-01 W-6a).
use serde::Serialize;
use std::{
    collections::hash_map::DefaultHasher,
    fs::{self, File, OpenOptions},
    hash::{Hash, Hasher},
    io::{self, Write},
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc,
    },
    thread,
    time::Duration,
};
#[cfg(test)]
use std::{
    process,
    time::{SystemTime, UNIX_EPOCH},
};

pub const PATH_CONTAINMENT: &str = "best_effort_non_adversarial";
pub const TOCTOU_LIMITATION: &str = "Static symlink checks and same-parent create/rename do not protect against a malicious concurrent parent or destination swap. On directory-sync timeout, at most one detached sync worker may remain blocked; later saves suppress additional directory-sync workers until it exits.";
const DIRECTORY_SYNC_TIMEOUT: Duration = Duration::from_secs(2);
static ACTIVE_DIRECTORY_SYNC_WORKERS: AtomicUsize = AtomicUsize::new(0);

/// One format the atomic writer admits: the single destination extension it
/// accepts, the prefix of every diagnostic code it reports, the prefix of the
/// temporary files it owns, and the label of its log lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AtomicSaveFormat {
    pub extension: &'static str,
    pub code_prefix: &'static str,
    pub temp_prefix: &'static str,
    pub log_label: &'static str,
}

impl AtomicSaveFormat {
    fn code(&self, suffix: &str) -> String {
        format!("{}-{suffix}", self.code_prefix)
    }
}

/// The report package (`.opsproj`): the writer's original and unchanged format.
pub const REPORT_PACKAGE_FORMAT: AtomicSaveFormat = AtomicSaveFormat {
    extension: "opsproj",
    code_prefix: "REPORT-PACKAGE",
    temp_prefix: ".openpipestress-report-package-",
    log_label: "report-package",
};

/// The model document (`.json`, File > Save Model Document…).
pub const MODEL_DOCUMENT_FORMAT: AtomicSaveFormat = AtomicSaveFormat {
    extension: "json",
    code_prefix: "MODEL-DOC-SAVE",
    temp_prefix: ".openpipestress-model-document-",
    log_label: "model-document",
};

#[derive(Debug, PartialEq, Eq)]
enum DirectorySyncResult {
    Durable,
    Failed {
        kind: String,
        raw_os_error: Option<i32>,
    },
    TimedOut,
    WorkerAlreadyActive,
    WorkerDisconnected,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct AtomicSaveOutcome {
    pub replaced_existing: bool,
    pub durability: String,
    pub selected_basename: String,
    pub path_containment: String,
    pub limitation: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct AtomicSaveError {
    pub code: String,
    pub stage: String,
    pub message: String,
    pub os_error_kind: Option<String>,
    pub os_error_code: Option<i32>,
    pub cleanup_code: Option<String>,
    pub cleanup_message: Option<String>,
    pub selected_basename: String,
    pub path_containment: String,
    pub limitation: String,
}

impl AtomicSaveError {
    fn validation(
        format: &AtomicSaveFormat,
        code_suffix: &str,
        message: impl Into<String>,
        destination: &Path,
    ) -> Self {
        Self {
            code: format.code(code_suffix),
            stage: "validate".to_string(),
            message: message.into(),
            os_error_kind: None,
            os_error_code: None,
            cleanup_code: None,
            cleanup_message: None,
            selected_basename: basename(format, destination),
            path_containment: PATH_CONTAINMENT.to_string(),
            limitation: TOCTOU_LIMITATION.to_string(),
        }
    }

    fn io(
        format: &AtomicSaveFormat,
        stage: &str,
        code_suffix: &str,
        error: io::Error,
        destination: &Path,
    ) -> Self {
        Self {
            code: format.code(code_suffix),
            stage: stage.to_string(),
            message: error.to_string(),
            os_error_kind: Some(format!("{:?}", error.kind())),
            os_error_code: error.raw_os_error(),
            cleanup_code: None,
            cleanup_message: None,
            selected_basename: basename(format, destination),
            path_containment: PATH_CONTAINMENT.to_string(),
            limitation: TOCTOU_LIMITATION.to_string(),
        }
    }

    fn with_cleanup(mut self, format: &AtomicSaveFormat, cleanup: io::Result<()>) -> Self {
        if let Err(error) = cleanup {
            self.cleanup_code = Some(format.code("TEMP-CLEANUP-FAILED"));
            self.cleanup_message = Some(error.to_string());
        }
        self
    }
}

fn basename(format: &AtomicSaveFormat, path: &Path) -> String {
    path.file_name()
        .and_then(|item| item.to_str())
        .map(str::to_string)
        .unwrap_or_else(|| format!("unknown.{}", format.extension))
}

fn parent_diagnostic(parent: &Path) -> (String, String) {
    let class = if parent.starts_with(std::env::temp_dir()) {
        "system_temp"
    } else {
        match parent
            .components()
            .nth(1)
            .and_then(|component| component.as_os_str().to_str())
        {
            Some("Users") => "local_user_tree",
            Some("Volumes") => "mounted_volume",
            Some("private") => "private_system_tree",
            Some(_) => "other_absolute_tree",
            None => "relative_or_root",
        }
    };
    let mut hasher = DefaultHasher::new();
    parent.as_os_str().hash(&mut hasher);
    (class.to_string(), format!("{:016x}", hasher.finish()))
}

fn run_directory_sync_bounded<F>(timeout: Duration, operation: F) -> DirectorySyncResult
where
    F: FnOnce() -> io::Result<()> + Send + 'static,
{
    let (sender, receiver) = mpsc::sync_channel(1);
    thread::spawn(move || {
        let result =
            operation().map_err(|error| (format!("{:?}", error.kind()), error.raw_os_error()));
        let _ = sender.send(result);
    });
    match receiver.recv_timeout(timeout) {
        Ok(Ok(())) => DirectorySyncResult::Durable,
        Ok(Err((kind, raw_os_error))) => DirectorySyncResult::Failed { kind, raw_os_error },
        Err(mpsc::RecvTimeoutError::Timeout) => DirectorySyncResult::TimedOut,
        Err(mpsc::RecvTimeoutError::Disconnected) => DirectorySyncResult::WorkerDisconnected,
    }
}

fn run_directory_sync_guarded<F>(
    active_workers: &'static AtomicUsize,
    timeout: Duration,
    operation: F,
) -> DirectorySyncResult
where
    F: FnOnce() -> io::Result<()> + Send + 'static,
{
    if active_workers
        .compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire)
        .is_err()
    {
        return DirectorySyncResult::WorkerAlreadyActive;
    }
    run_directory_sync_bounded(timeout, move || {
        struct ActiveWorkerGuard(&'static AtomicUsize);
        impl Drop for ActiveWorkerGuard {
            fn drop(&mut self) {
                self.0.fetch_sub(1, Ordering::AcqRel);
            }
        }
        let _guard = ActiveWorkerGuard(active_workers);
        operation()
    })
}

fn sync_parent_directory_bounded(parent: &Path) -> DirectorySyncResult {
    let owned_parent = parent.to_path_buf();
    run_directory_sync_guarded(
        &ACTIVE_DIRECTORY_SYNC_WORKERS,
        DIRECTORY_SYNC_TIMEOUT,
        move || File::open(owned_parent).and_then(|directory| directory.sync_all()),
    )
}

/// Per-format admission: the destination must carry the format's one
/// extension and sit in an existing non-symlink parent directory; a symlink or
/// directory destination is refused.
fn validate_destination(
    format: &AtomicSaveFormat,
    destination: &Path,
) -> Result<(PathBuf, bool), AtomicSaveError> {
    if destination.extension().and_then(|item| item.to_str()) != Some(format.extension) {
        return Err(AtomicSaveError::validation(
            format,
            "DESTINATION-EXTENSION",
            format!("destination must use the .{} extension", format.extension),
            destination,
        ));
    }
    let parent = destination.parent().ok_or_else(|| {
        AtomicSaveError::validation(
            format,
            "PARENT-MISSING",
            "destination must have an existing parent directory",
            destination,
        )
    })?;
    let parent_meta = fs::symlink_metadata(parent).map_err(|error| {
        AtomicSaveError::io(format, "validate", "PARENT-UNAVAILABLE", error, destination)
    })?;
    if !parent_meta.is_dir() || parent_meta.file_type().is_symlink() {
        return Err(AtomicSaveError::validation(
            format,
            "PARENT-UNSAFE",
            "destination parent must be an existing non-symlink directory",
            destination,
        ));
    }
    let replaced_existing = match fs::symlink_metadata(destination) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            return Err(AtomicSaveError::validation(
                format,
                "DESTINATION-SYMLINK",
                "symlink destinations are refused",
                destination,
            ))
        }
        Ok(metadata) if metadata.is_dir() => {
            return Err(AtomicSaveError::validation(
                format,
                "DESTINATION-DIRECTORY",
                "destination must not be a directory",
                destination,
            ))
        }
        Ok(_) => true,
        Err(error) if error.kind() == io::ErrorKind::NotFound => false,
        Err(error) => {
            return Err(AtomicSaveError::io(
                format,
                "validate",
                "DESTINATION-INSPECTION-FAILED",
                error,
                destination,
            ))
        }
    };
    Ok((parent.to_path_buf(), replaced_existing))
}

#[cfg(test)]
fn create_temp(parent: &Path, destination: &Path) -> Result<(PathBuf, File), AtomicSaveError> {
    create_temp_as(&REPORT_PACKAGE_FORMAT, parent, destination)
}

fn create_temp_as(
    format: &AtomicSaveFormat,
    parent: &Path,
    destination: &Path,
) -> Result<(PathBuf, File), AtomicSaveError> {
    for _ in 0..128_u32 {
        let mut token_bytes = [0_u8; 16];
        getrandom::fill(&mut token_bytes).map_err(|error| {
            AtomicSaveError::io(
                format,
                "create_temp",
                "TEMP-RANDOM-FAILED",
                io::Error::other(error.to_string()),
                destination,
            )
        })?;
        let token = token_bytes
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>();
        let path = parent.join(format!("{}{token}.tmp", format.temp_prefix));
        match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(file) => return Ok((path, file)),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(error) => {
                return Err(AtomicSaveError::io(
                    format,
                    "create_temp",
                    "TEMP-CREATE-FAILED",
                    error,
                    destination,
                ))
            }
        }
    }
    Err(AtomicSaveError::validation(
        format,
        "TEMP-COLLISION-EXHAUSTED",
        "could not allocate an owned same-directory temporary file",
        destination,
    ))
}

/// The writer's file operations: the system's, or a test's injected failures
/// (shared at crate visibility so another format's tests can inject them).
pub(crate) trait AtomicSaveIo {
    fn write_all(&mut self, temp: &mut File, bytes: &[u8]) -> io::Result<()>;
    fn flush(&mut self, temp: &mut File) -> io::Result<()>;
    fn sync_temp(&mut self, temp: &File) -> io::Result<()>;
    fn rename(&mut self, source: &Path, destination: &Path) -> io::Result<()>;
    fn cleanup_temp(&mut self, temp_path: &Path) -> io::Result<()>;
}

pub(crate) struct SystemAtomicSaveIo;

impl AtomicSaveIo for SystemAtomicSaveIo {
    fn write_all(&mut self, temp: &mut File, bytes: &[u8]) -> io::Result<()> {
        temp.write_all(bytes)
    }

    fn flush(&mut self, temp: &mut File) -> io::Result<()> {
        temp.flush()
    }

    fn sync_temp(&mut self, temp: &File) -> io::Result<()> {
        temp.sync_all()
    }

    fn rename(&mut self, source: &Path, destination: &Path) -> io::Result<()> {
        fs::rename(source, destination)
    }

    fn cleanup_temp(&mut self, temp_path: &Path) -> io::Result<()> {
        fs::remove_file(temp_path)
    }
}

/// The report package save (`.opsproj`), unchanged.
pub fn atomic_save_bytes(
    destination: &Path,
    bytes: &[u8],
) -> Result<AtomicSaveOutcome, AtomicSaveError> {
    atomic_save_bytes_as(&REPORT_PACKAGE_FORMAT, destination, bytes)
}

/// Writes `bytes` to `destination` atomically under `format`'s admission.
pub fn atomic_save_bytes_as(
    format: &AtomicSaveFormat,
    destination: &Path,
    bytes: &[u8],
) -> Result<AtomicSaveOutcome, AtomicSaveError> {
    atomic_save_bytes_as_with_io(format, destination, bytes, &mut SystemAtomicSaveIo)
}

#[cfg(test)]
fn atomic_save_bytes_with_io(
    destination: &Path,
    bytes: &[u8],
    operations: &mut impl AtomicSaveIo,
) -> Result<AtomicSaveOutcome, AtomicSaveError> {
    atomic_save_bytes_as_with_io(&REPORT_PACKAGE_FORMAT, destination, bytes, operations)
}

pub(crate) fn atomic_save_bytes_as_with_io(
    format: &AtomicSaveFormat,
    destination: &Path,
    bytes: &[u8],
    operations: &mut impl AtomicSaveIo,
) -> Result<AtomicSaveOutcome, AtomicSaveError> {
    let (parent, replaced_existing) = validate_destination(format, destination)?;
    let (parent_class, parent_hash) = parent_diagnostic(&parent);
    let label = format.log_label;
    eprintln!(
        "{label} atomic save selected basename={} parent_class={} parent_hash={}",
        basename(format, destination),
        parent_class,
        parent_hash
    );
    #[cfg(not(target_os = "macos"))]
    if replaced_existing {
        return Err(AtomicSaveError::validation(
            format,
            "REPLACE-UNSUPPORTED-PLATFORM",
            "atomic replacement is currently claimed only for macOS",
            destination,
        ));
    }

    let (temp_path, mut temp) = create_temp_as(format, &parent, destination)?;
    if let Err(error) = operations.write_all(&mut temp, bytes) {
        drop(temp);
        return Err(AtomicSaveError::io(format, "write", "WRITE-FAILED", error, destination)
            .with_cleanup(format, operations.cleanup_temp(&temp_path)));
    }
    if let Err(error) = operations.flush(&mut temp) {
        drop(temp);
        return Err(AtomicSaveError::io(format, "flush", "FLUSH-FAILED", error, destination)
            .with_cleanup(format, operations.cleanup_temp(&temp_path)));
    }
    if let Err(error) = operations.sync_temp(&temp) {
        drop(temp);
        return Err(
            AtomicSaveError::io(format, "temp_sync", "FILE-SYNC-FAILED", error, destination)
                .with_cleanup(format, operations.cleanup_temp(&temp_path)),
        );
    }
    drop(temp);

    if let Err(error) = operations.rename(&temp_path, destination) {
        return Err(AtomicSaveError::io(format, "rename", "RENAME-FAILED", error, destination)
            .with_cleanup(format, operations.cleanup_temp(&temp_path)));
    }

    let directory_sync = sync_parent_directory_bounded(&parent);
    let durability = match &directory_sync {
        DirectorySyncResult::Durable => "durable",
        DirectorySyncResult::Failed { kind, raw_os_error } => {
            eprintln!(
                "{label} directory sync failed after rename basename={} parent_class={} parent_hash={} error_kind={} os_error={:?}",
                basename(format, destination), parent_class, parent_hash, kind, raw_os_error
            );
            "saved_durability_uncertain"
        }
        DirectorySyncResult::TimedOut => {
            eprintln!(
                "{label} directory sync timed out after rename basename={} parent_class={} parent_hash={}; at most one worker remains active and later workers are suppressed until it exits",
                basename(format, destination), parent_class, parent_hash
            );
            "saved_durability_uncertain"
        }
        DirectorySyncResult::WorkerAlreadyActive => {
            eprintln!(
                "{label} directory sync suppressed after rename basename={} parent_class={} parent_hash={} because one prior sync worker remains active",
                basename(format, destination), parent_class, parent_hash
            );
            "saved_durability_uncertain"
        }
        DirectorySyncResult::WorkerDisconnected => {
            eprintln!(
                "{label} directory sync worker disconnected after rename basename={} parent_class={} parent_hash={}",
                basename(format, destination), parent_class, parent_hash
            );
            "saved_durability_uncertain"
        }
    };
    Ok(AtomicSaveOutcome {
        replaced_existing,
        durability: durability.to_string(),
        selected_basename: basename(format, destination),
        path_containment: PATH_CONTAINMENT.to_string(),
        limitation: TOCTOU_LIMITATION.to_string(),
    })
}

/// Test isolation for the process-wide directory-sync guard. Every test whose
/// save reaches the real parent-directory sync holds this lock for its save
/// section, so a parallel test's sync cannot suppress its own and turn a
/// correct save into `saved_durability_uncertain`. The timeout and overlap
/// tests drive their own counter, not the shared guard, and need no lock.
#[cfg(test)]
pub(crate) mod test_support {
    use std::sync::{Mutex, MutexGuard, PoisonError};

    static ATOMIC_SAVE_TEST_LOCK: Mutex<()> = Mutex::new(());

    /// Recovers from poisoning, so one failing save test does not fail the rest.
    pub(crate) fn atomic_save_test_lock() -> MutexGuard<'static, ()> {
        ATOMIC_SAVE_TEST_LOCK.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

#[cfg(test)]
mod tests {
    use super::test_support::atomic_save_test_lock;
    use super::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum InjectedFailureStage {
        Write,
        Flush,
        TempSync,
        Rename,
    }

    struct InjectedAtomicSaveIo {
        failure_stage: InjectedFailureStage,
        report_cleanup_failure: bool,
    }

    impl InjectedAtomicSaveIo {
        fn new(failure_stage: InjectedFailureStage) -> Self {
            Self {
                failure_stage,
                report_cleanup_failure: false,
            }
        }
    }

    impl AtomicSaveIo for InjectedAtomicSaveIo {
        fn write_all(&mut self, temp: &mut File, bytes: &[u8]) -> io::Result<()> {
            if self.failure_stage == InjectedFailureStage::Write {
                return Err(io::Error::other("injected write failure"));
            }
            temp.write_all(bytes)
        }

        fn flush(&mut self, temp: &mut File) -> io::Result<()> {
            if self.failure_stage == InjectedFailureStage::Flush {
                return Err(io::Error::other("injected flush failure"));
            }
            temp.flush()
        }

        fn sync_temp(&mut self, temp: &File) -> io::Result<()> {
            if self.failure_stage == InjectedFailureStage::TempSync {
                return Err(io::Error::other("injected temp sync failure"));
            }
            temp.sync_all()
        }

        fn rename(&mut self, source: &Path, destination: &Path) -> io::Result<()> {
            if self.failure_stage == InjectedFailureStage::Rename {
                return Err(io::Error::other("injected rename failure"));
            }
            fs::rename(source, destination)
        }

        fn cleanup_temp(&mut self, temp_path: &Path) -> io::Result<()> {
            fs::remove_file(temp_path)?;
            if self.report_cleanup_failure {
                return Err(io::Error::other(
                    "injected cleanup diagnostic after successful removal",
                ));
            }
            Ok(())
        }
    }

    fn sandbox(label: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "openpipestress-{label}-{}-{}",
            process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&path).unwrap();
        path
    }

    fn assert_no_temp_files(root: &Path) {
        assert!(fs::read_dir(root).unwrap().all(|entry| {
            !entry
                .unwrap()
                .file_name()
                .to_string_lossy()
                .starts_with(".openpipestress-report-package-")
        }));
    }

    fn expected_failure(stage: InjectedFailureStage) -> (&'static str, &'static str) {
        match stage {
            InjectedFailureStage::Write => ("write", "REPORT-PACKAGE-WRITE-FAILED"),
            InjectedFailureStage::Flush => ("flush", "REPORT-PACKAGE-FLUSH-FAILED"),
            InjectedFailureStage::TempSync => ("temp_sync", "REPORT-PACKAGE-FILE-SYNC-FAILED"),
            InjectedFailureStage::Rename => ("rename", "REPORT-PACKAGE-RENAME-FAILED"),
        }
    }

    #[test]
    fn writes_exact_bytes_and_leaves_no_temp() {
        let _save_lock = atomic_save_test_lock();
        let root = sandbox("atomic-new");
        let destination = root.join("invented.opsproj");
        let outcome = atomic_save_bytes(&destination, b"invented package bytes").unwrap();
        assert_eq!(fs::read(&destination).unwrap(), b"invented package bytes");
        assert!(!outcome.replaced_existing);
        assert_no_temp_files(&root);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn temporary_names_are_random_hex_and_unique() {
        let root = sandbox("atomic-temp-names");
        let destination = root.join("invented.opsproj");
        let (first_path, first_file) = create_temp(&root, &destination).unwrap();
        let (second_path, second_file) = create_temp(&root, &destination).unwrap();
        let token = |path: &Path| {
            path.file_name()
                .unwrap()
                .to_string_lossy()
                .strip_prefix(".openpipestress-report-package-")
                .unwrap()
                .strip_suffix(".tmp")
                .unwrap()
                .to_string()
        };
        let first_token = token(&first_path);
        let second_token = token(&second_path);
        assert_eq!(first_token.len(), 32);
        assert_eq!(second_token.len(), 32);
        assert!(first_token
            .chars()
            .all(|character| character.is_ascii_hexdigit()));
        assert!(second_token
            .chars()
            .all(|character| character.is_ascii_hexdigit()));
        assert_ne!(first_path, second_path);
        drop(first_file);
        drop(second_file);
        fs::remove_file(first_path).unwrap();
        fs::remove_file(second_path).unwrap();
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn injected_failures_leave_new_destination_absent_and_cleanup_temp() {
        for stage in [
            InjectedFailureStage::Write,
            InjectedFailureStage::Flush,
            InjectedFailureStage::TempSync,
            InjectedFailureStage::Rename,
        ] {
            let root = sandbox(&format!("atomic-new-{stage:?}"));
            let destination = root.join("invented.opsproj");
            let mut operations = InjectedAtomicSaveIo::new(stage);
            let error =
                atomic_save_bytes_with_io(&destination, b"new bytes", &mut operations).unwrap_err();
            let (expected_stage, expected_code) = expected_failure(stage);
            assert_eq!(error.stage, expected_stage);
            assert_eq!(error.code, expected_code);
            assert!(!destination.exists());
            assert_no_temp_files(&root);
            fs::remove_dir_all(root).unwrap();
        }
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn injected_failures_preserve_existing_destination_bytes_and_inode() {
        use std::os::unix::fs::MetadataExt;

        for stage in [
            InjectedFailureStage::Write,
            InjectedFailureStage::Flush,
            InjectedFailureStage::TempSync,
            InjectedFailureStage::Rename,
        ] {
            let root = sandbox(&format!("atomic-replace-{stage:?}"));
            let destination = root.join("invented.opsproj");
            fs::write(&destination, b"old exact bytes").unwrap();
            let original_inode = fs::metadata(&destination).unwrap().ino();
            let mut operations = InjectedAtomicSaveIo::new(stage);
            let error =
                atomic_save_bytes_with_io(&destination, b"new bytes", &mut operations).unwrap_err();
            let (expected_stage, expected_code) = expected_failure(stage);
            assert_eq!(error.stage, expected_stage);
            assert_eq!(error.code, expected_code);
            assert_eq!(fs::read(&destination).unwrap(), b"old exact bytes");
            assert_eq!(fs::metadata(&destination).unwrap().ino(), original_inode);
            assert_no_temp_files(&root);
            fs::remove_dir_all(root).unwrap();
        }
    }

    #[test]
    fn cleanup_diagnostic_does_not_replace_primary_failure() {
        let root = sandbox("atomic-cleanup-diagnostic");
        let destination = root.join("invented.opsproj");
        let mut operations = InjectedAtomicSaveIo {
            failure_stage: InjectedFailureStage::Write,
            report_cleanup_failure: true,
        };
        let error =
            atomic_save_bytes_with_io(&destination, b"new bytes", &mut operations).unwrap_err();
        assert_eq!(error.stage, "write");
        assert_eq!(error.code, "REPORT-PACKAGE-WRITE-FAILED");
        assert_eq!(
            error.cleanup_code.as_deref(),
            Some("REPORT-PACKAGE-TEMP-CLEANUP-FAILED")
        );
        assert!(error
            .cleanup_message
            .as_deref()
            .unwrap()
            .contains("injected cleanup diagnostic"));
        assert!(!destination.exists());
        assert_no_temp_files(&root);
        fs::remove_dir_all(root).unwrap();
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn replacement_is_atomic_and_exact() {
        let _save_lock = atomic_save_test_lock();
        let root = sandbox("atomic-replace");
        let destination = root.join("invented.opsproj");
        fs::write(&destination, b"old bytes").unwrap();
        let outcome = atomic_save_bytes(&destination, b"new exact bytes").unwrap();
        assert!(outcome.replaced_existing);
        assert_eq!(fs::read(&destination).unwrap(), b"new exact bytes");
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn validation_failure_preserves_existing_file() {
        let root = sandbox("atomic-invalid");
        let destination = root.join("invented.txt");
        fs::write(&destination, b"old bytes").unwrap();
        let error = atomic_save_bytes(&destination, b"new bytes").unwrap_err();
        assert_eq!(error.stage, "validate");
        assert_eq!(fs::read(&destination).unwrap(), b"old bytes");
        fs::remove_dir_all(root).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn rejects_static_symlink_destination() {
        use std::os::unix::fs::symlink;
        let root = sandbox("atomic-symlink");
        let target = root.join("target.opsproj");
        let destination = root.join("link.opsproj");
        fs::write(&target, b"old bytes").unwrap();
        symlink(&target, &destination).unwrap();
        let error = atomic_save_bytes(&destination, b"new bytes").unwrap_err();
        assert_eq!(error.code, "REPORT-PACKAGE-DESTINATION-SYMLINK");
        assert_eq!(fs::read(&target).unwrap(), b"old bytes");
        fs::remove_file(destination).unwrap();
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn directory_sync_timeout_is_bounded_and_reports_uncertain() {
        // The operation is held until the timeout has been observed, so no scheduling
        // delay of the caller can let it finish first.
        let (release, held) = mpsc::channel::<()>();
        let result = run_directory_sync_bounded(Duration::from_millis(5), move || {
            let _ = held.recv();
            Ok(())
        });
        assert_eq!(result, DirectorySyncResult::TimedOut);
        drop(release);
    }

    #[test]
    fn directory_sync_error_is_captured_after_rename() {
        let result = run_directory_sync_bounded(Duration::from_secs(1), || {
            Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "invented unsupported directory sync",
            ))
        });
        assert_eq!(
            result,
            DirectorySyncResult::Failed {
                kind: "Unsupported".to_string(),
                raw_os_error: None,
            }
        );
    }

    #[test]
    fn timed_out_directory_sync_worker_does_not_accumulate() {
        let active_workers = Box::leak(Box::new(AtomicUsize::new(0)));
        // The timed-out operation is held until the test has seen it counted as active and
        // a second call refused, then released; a fixed sleep let it finish early, or not
        // release in time, on a loaded hosted runner.
        let (release, held) = mpsc::channel::<()>();
        let first =
            run_directory_sync_guarded(active_workers, Duration::from_millis(5), move || {
                let _ = held.recv();
                Ok(())
            });
        assert_eq!(first, DirectorySyncResult::TimedOut);
        assert_eq!(active_workers.load(Ordering::Acquire), 1);
        let second =
            run_directory_sync_guarded(active_workers, Duration::from_millis(5), || Ok(()));
        assert_eq!(second, DirectorySyncResult::WorkerAlreadyActive);
        drop(release);
        // The worker's guard releases the slot after the operation returns.
        let deadline = std::time::Instant::now() + Duration::from_secs(10);
        while active_workers.load(Ordering::Acquire) != 0 && std::time::Instant::now() < deadline {
            thread::sleep(Duration::from_millis(5));
        }
        assert_eq!(active_workers.load(Ordering::Acquire), 0);
    }

    #[test]
    fn parent_diagnostic_is_non_sensitive_and_stable() {
        let root = sandbox("parent-diagnostic");
        let (class, hash) = parent_diagnostic(&root);
        assert_eq!(class, "system_temp");
        assert_eq!(hash.len(), 16);
        assert!(!hash.contains(&root.to_string_lossy().to_string()));
        fs::remove_dir_all(root).unwrap();
    }
}

/// The per-format admission of the extension-neutral writer (DEL-15-01 W-6a).
/// The report package tests above are unchanged and pin the `.opsproj` format.
#[cfg(test)]
mod format_admission_tests {
    use super::*;

    fn sandbox(label: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "openpipestress-format-{label}-{}-{}",
            process::id(),
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
        ));
        fs::create_dir_all(&path).unwrap();
        path
    }

    fn temp_files(root: &Path) -> Vec<String> {
        fs::read_dir(root)
            .unwrap()
            .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
            .filter(|name| name.starts_with(".openpipestress-"))
            .collect()
    }

    #[test]
    fn the_model_document_format_admits_json_only() {
        let _save_lock = super::test_support::atomic_save_test_lock();
        let root = sandbox("model-json");
        let destination = root.join("invented-model.json");
        let outcome =
            atomic_save_bytes_as(&MODEL_DOCUMENT_FORMAT, &destination, b"{\"a\":1}").unwrap();
        assert_eq!(fs::read(&destination).unwrap(), b"{\"a\":1}");
        assert!(!outcome.replaced_existing);
        assert_eq!(outcome.selected_basename, "invented-model.json");
        assert!(temp_files(&root).is_empty());
        for refused in ["invented.opsproj", "invented.JSON", "invented.json.txt", "invented"] {
            let destination = root.join(refused);
            let error =
                atomic_save_bytes_as(&MODEL_DOCUMENT_FORMAT, &destination, b"{}").unwrap_err();
            assert_eq!(error.code, "MODEL-DOC-SAVE-DESTINATION-EXTENSION", "{refused}");
            assert_eq!(error.stage, "validate");
            assert!(!destination.exists(), "{refused}");
        }
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn the_report_package_format_still_refuses_json() {
        let root = sandbox("report-json");
        let destination = root.join("invented.json");
        let error = atomic_save_bytes(&destination, b"{}").unwrap_err();
        assert_eq!(error.code, "REPORT-PACKAGE-DESTINATION-EXTENSION");
        assert!(!destination.exists());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn model_document_failures_use_the_model_codes_and_temp_prefix() {
        struct FailRename;
        impl AtomicSaveIo for FailRename {
            fn write_all(&mut self, temp: &mut File, bytes: &[u8]) -> io::Result<()> {
                temp.write_all(bytes)
            }
            fn flush(&mut self, temp: &mut File) -> io::Result<()> {
                temp.flush()
            }
            fn sync_temp(&mut self, temp: &File) -> io::Result<()> {
                temp.sync_all()
            }
            fn rename(&mut self, _: &Path, _: &Path) -> io::Result<()> {
                Err(io::Error::other("injected rename failure"))
            }
            fn cleanup_temp(&mut self, temp_path: &Path) -> io::Result<()> {
                fs::remove_file(temp_path)
            }
        }
        let root = sandbox("model-rename");
        let destination = root.join("invented-model.json");
        fs::write(&destination, b"old bytes").unwrap();
        let error = atomic_save_bytes_as_with_io(
            &MODEL_DOCUMENT_FORMAT,
            &destination,
            b"new bytes",
            &mut FailRename,
        )
        .unwrap_err();
        assert_eq!((error.stage.as_str(), error.code.as_str()), ("rename", "MODEL-DOC-SAVE-RENAME-FAILED"));
        assert_eq!(fs::read(&destination).unwrap(), b"old bytes");
        assert!(temp_files(&root).is_empty());
        let (temp, file) = create_temp_as(&MODEL_DOCUMENT_FORMAT, &root, &destination).unwrap();
        assert!(temp
            .file_name()
            .unwrap()
            .to_string_lossy()
            .starts_with(".openpipestress-model-document-"));
        drop(file);
        fs::remove_dir_all(root).unwrap();
    }
}
