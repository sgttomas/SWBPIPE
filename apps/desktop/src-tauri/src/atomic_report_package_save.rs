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
    fn validation(code: &str, message: impl Into<String>, destination: &Path) -> Self {
        Self {
            code: code.to_string(),
            stage: "validate".to_string(),
            message: message.into(),
            os_error_kind: None,
            os_error_code: None,
            cleanup_code: None,
            cleanup_message: None,
            selected_basename: basename(destination),
            path_containment: PATH_CONTAINMENT.to_string(),
            limitation: TOCTOU_LIMITATION.to_string(),
        }
    }

    fn io(stage: &str, code: &str, error: io::Error, destination: &Path) -> Self {
        Self {
            code: code.to_string(),
            stage: stage.to_string(),
            message: error.to_string(),
            os_error_kind: Some(format!("{:?}", error.kind())),
            os_error_code: error.raw_os_error(),
            cleanup_code: None,
            cleanup_message: None,
            selected_basename: basename(destination),
            path_containment: PATH_CONTAINMENT.to_string(),
            limitation: TOCTOU_LIMITATION.to_string(),
        }
    }

    fn with_cleanup(mut self, cleanup: io::Result<()>) -> Self {
        if let Err(error) = cleanup {
            self.cleanup_code = Some("REPORT-PACKAGE-TEMP-CLEANUP-FAILED".to_string());
            self.cleanup_message = Some(error.to_string());
        }
        self
    }
}

fn basename(path: &Path) -> String {
    path.file_name()
        .and_then(|item| item.to_str())
        .unwrap_or("unknown.opsproj")
        .to_string()
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

fn validate_destination(destination: &Path) -> Result<(PathBuf, bool), AtomicSaveError> {
    if destination.extension().and_then(|item| item.to_str()) != Some("opsproj") {
        return Err(AtomicSaveError::validation(
            "REPORT-PACKAGE-DESTINATION-EXTENSION",
            "destination must use the .opsproj extension",
            destination,
        ));
    }
    let parent = destination.parent().ok_or_else(|| {
        AtomicSaveError::validation(
            "REPORT-PACKAGE-PARENT-MISSING",
            "destination must have an existing parent directory",
            destination,
        )
    })?;
    let parent_meta = fs::symlink_metadata(parent).map_err(|error| {
        AtomicSaveError::io(
            "validate",
            "REPORT-PACKAGE-PARENT-UNAVAILABLE",
            error,
            destination,
        )
    })?;
    if !parent_meta.is_dir() || parent_meta.file_type().is_symlink() {
        return Err(AtomicSaveError::validation(
            "REPORT-PACKAGE-PARENT-UNSAFE",
            "destination parent must be an existing non-symlink directory",
            destination,
        ));
    }
    let replaced_existing = match fs::symlink_metadata(destination) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            return Err(AtomicSaveError::validation(
                "REPORT-PACKAGE-DESTINATION-SYMLINK",
                "symlink destinations are refused",
                destination,
            ))
        }
        Ok(metadata) if metadata.is_dir() => {
            return Err(AtomicSaveError::validation(
                "REPORT-PACKAGE-DESTINATION-DIRECTORY",
                "destination must not be a directory",
                destination,
            ))
        }
        Ok(_) => true,
        Err(error) if error.kind() == io::ErrorKind::NotFound => false,
        Err(error) => {
            return Err(AtomicSaveError::io(
                "validate",
                "REPORT-PACKAGE-DESTINATION-INSPECTION-FAILED",
                error,
                destination,
            ))
        }
    };
    Ok((parent.to_path_buf(), replaced_existing))
}

fn create_temp(parent: &Path, destination: &Path) -> Result<(PathBuf, File), AtomicSaveError> {
    for _ in 0..128_u32 {
        let mut token_bytes = [0_u8; 16];
        getrandom::fill(&mut token_bytes).map_err(|error| {
            AtomicSaveError::io(
                "create_temp",
                "REPORT-PACKAGE-TEMP-RANDOM-FAILED",
                io::Error::other(error.to_string()),
                destination,
            )
        })?;
        let token = token_bytes
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>();
        let path = parent.join(format!(".openpipestress-report-package-{token}.tmp"));
        match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(file) => return Ok((path, file)),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(error) => {
                return Err(AtomicSaveError::io(
                    "create_temp",
                    "REPORT-PACKAGE-TEMP-CREATE-FAILED",
                    error,
                    destination,
                ))
            }
        }
    }
    Err(AtomicSaveError::validation(
        "REPORT-PACKAGE-TEMP-COLLISION-EXHAUSTED",
        "could not allocate an owned same-directory temporary file",
        destination,
    ))
}

trait AtomicSaveIo {
    fn write_all(&mut self, temp: &mut File, bytes: &[u8]) -> io::Result<()>;
    fn flush(&mut self, temp: &mut File) -> io::Result<()>;
    fn sync_temp(&mut self, temp: &File) -> io::Result<()>;
    fn rename(&mut self, source: &Path, destination: &Path) -> io::Result<()>;
    fn cleanup_temp(&mut self, temp_path: &Path) -> io::Result<()>;
}

struct SystemAtomicSaveIo;

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

pub fn atomic_save_bytes(
    destination: &Path,
    bytes: &[u8],
) -> Result<AtomicSaveOutcome, AtomicSaveError> {
    atomic_save_bytes_with_io(destination, bytes, &mut SystemAtomicSaveIo)
}

fn atomic_save_bytes_with_io(
    destination: &Path,
    bytes: &[u8],
    operations: &mut impl AtomicSaveIo,
) -> Result<AtomicSaveOutcome, AtomicSaveError> {
    let (parent, replaced_existing) = validate_destination(destination)?;
    let (parent_class, parent_hash) = parent_diagnostic(&parent);
    eprintln!(
        "report-package atomic save selected basename={} parent_class={} parent_hash={}",
        basename(destination),
        parent_class,
        parent_hash
    );
    #[cfg(not(target_os = "macos"))]
    if replaced_existing {
        return Err(AtomicSaveError::validation(
            "REPORT-PACKAGE-REPLACE-UNSUPPORTED-PLATFORM",
            "atomic replacement is currently claimed only for macOS",
            destination,
        ));
    }

    let (temp_path, mut temp) = create_temp(&parent, destination)?;
    if let Err(error) = operations.write_all(&mut temp, bytes) {
        drop(temp);
        return Err(AtomicSaveError::io(
            "write",
            "REPORT-PACKAGE-WRITE-FAILED",
            error,
            destination,
        )
        .with_cleanup(operations.cleanup_temp(&temp_path)));
    }
    if let Err(error) = operations.flush(&mut temp) {
        drop(temp);
        return Err(AtomicSaveError::io(
            "flush",
            "REPORT-PACKAGE-FLUSH-FAILED",
            error,
            destination,
        )
        .with_cleanup(operations.cleanup_temp(&temp_path)));
    }
    if let Err(error) = operations.sync_temp(&temp) {
        drop(temp);
        return Err(AtomicSaveError::io(
            "temp_sync",
            "REPORT-PACKAGE-FILE-SYNC-FAILED",
            error,
            destination,
        )
        .with_cleanup(operations.cleanup_temp(&temp_path)));
    }
    drop(temp);

    if let Err(error) = operations.rename(&temp_path, destination) {
        return Err(AtomicSaveError::io(
            "rename",
            "REPORT-PACKAGE-RENAME-FAILED",
            error,
            destination,
        )
        .with_cleanup(operations.cleanup_temp(&temp_path)));
    }

    let directory_sync = sync_parent_directory_bounded(&parent);
    let durability = match &directory_sync {
        DirectorySyncResult::Durable => "durable",
        DirectorySyncResult::Failed { kind, raw_os_error } => {
            eprintln!(
                "report-package directory sync failed after rename basename={} parent_class={} parent_hash={} error_kind={} os_error={:?}",
                basename(destination), parent_class, parent_hash, kind, raw_os_error
            );
            "saved_durability_uncertain"
        }
        DirectorySyncResult::TimedOut => {
            eprintln!(
                "report-package directory sync timed out after rename basename={} parent_class={} parent_hash={}; at most one worker remains active and later workers are suppressed until it exits",
                basename(destination), parent_class, parent_hash
            );
            "saved_durability_uncertain"
        }
        DirectorySyncResult::WorkerAlreadyActive => {
            eprintln!(
                "report-package directory sync suppressed after rename basename={} parent_class={} parent_hash={} because one prior sync worker remains active",
                basename(destination), parent_class, parent_hash
            );
            "saved_durability_uncertain"
        }
        DirectorySyncResult::WorkerDisconnected => {
            eprintln!(
                "report-package directory sync worker disconnected after rename basename={} parent_class={} parent_hash={}",
                basename(destination), parent_class, parent_hash
            );
            "saved_durability_uncertain"
        }
    };
    Ok(AtomicSaveOutcome {
        replaced_existing,
        durability: durability.to_string(),
        selected_basename: basename(destination),
        path_containment: PATH_CONTAINMENT.to_string(),
        limitation: TOCTOU_LIMITATION.to_string(),
    })
}

#[cfg(test)]
mod tests {
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
        let result = run_directory_sync_bounded(Duration::from_millis(5), || {
            thread::sleep(Duration::from_millis(50));
            Ok(())
        });
        assert_eq!(result, DirectorySyncResult::TimedOut);
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
        let first = run_directory_sync_guarded(active_workers, Duration::from_millis(5), || {
            thread::sleep(Duration::from_millis(50));
            Ok(())
        });
        assert_eq!(first, DirectorySyncResult::TimedOut);
        assert_eq!(active_workers.load(Ordering::Acquire), 1);
        let second =
            run_directory_sync_guarded(active_workers, Duration::from_millis(5), || Ok(()));
        assert_eq!(second, DirectorySyncResult::WorkerAlreadyActive);
        thread::sleep(Duration::from_millis(75));
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
