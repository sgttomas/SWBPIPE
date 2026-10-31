//! Result injection for the desktop performance measurement (RT-U4, DEL-04-05
//! `Work/criterion-9-measurement.md`, "Desktop measurement").
//!
//! Compiled only with the `native-smoke` cargo feature, which no product build
//! enables: this module is declared under `#[cfg(feature = "native-smoke")]`
//! in `lib.rs`, its one call site in `solve_jobs.rs` (`run_job`) carries the
//! same attribute, and `native_smoke_reach_scan_tests.rs` fails if the
//! variable's name or this module is named anywhere else.
//!
//! When [`INJECT_RESULT_ENV`] names a file, a started preview mechanics job
//! skips its solve worker process: after the job's queue wait and starting
//! grant, as for any job, the supervisor makes the outcome a worker would have
//! reported from that file's bytes instead of starting a worker. From there the
//! job is the product's: the result lease admitted while the grant is held
//! (as `solve_worker::supervise` admits it for a result frame), publication,
//! retention, the attestation table, poll, take (2R custody) and the IPC
//! response. The attestation is computed by the worker's own functions: `d_M`
//! over the request model materialized from the job's raw body, and `d_S` over
//! the file's bytes parsed back (`solve_worker::result_digest`).
//!
//! The file must be a regular file, not a symbolic link, inside the directory
//! [`INJECT_ROOT_ENV`] names (the perf runner's results directory): its path is
//! resolved and checked inside that root, opened without following a final
//! link, and the opened file's device and inode must be those of the checked
//! path. A missing root, a link, a file outside it or anything but a regular
//! file fails the job.
//!
//! **A native-smoke build must never be distributed.** Injection publishes
//! arbitrary bytes as this session's solve, with a solve attestation computed
//! over them: it forges solve provenance. Nothing in a product build can reach
//! it (the reach scan above, and the PR's release-binary check); the release
//! build (`tools/release/build_signed_app.py`) runs a fixed `BUILD_COMMAND`
//! that passes no `--features`, though no check refuses a feature build by name.
//!
//! What differs from a solve: no worker process runs, so nothing is solved,
//! `worker_peak_footprint_bytes` stays unset, and `d_M`, `d_S` and their
//! transient parsed values are computed in this process (a worker computes
//! them inside its own capped grant). The file must be the product's output
//! for exactly the model the app sends (`release_timing --write` from the same
//! model document); nothing here checks it, and the webview's own binding
//! checks apply to it unchanged.

use crate::memory_pool::{LeasePurpose, MemoryPool, ResampleStop};
use crate::solve_attestation::{streamed_digest, SolveAttestation, SOLVED_MODEL_DOMAIN};
use crate::solve_worker::{result_digest, WorkerOutcome};
use crate::worker_alloc::resident_envelope;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;

/// Names the file whose bytes a started solve job publishes as its result.
pub(crate) const INJECT_RESULT_ENV: &str = "SWBPIPE_NATIVE_SMOKE_INJECT_RESULT";
/// Names the directory the injected file must lie in.
pub(crate) const INJECT_ROOT_ENV: &str = "SWBPIPE_NATIVE_SMOKE_INJECT_ROOT";

/// The injected outcome for a job whose raw request body is `raw_model`, or
/// `None` when [`INJECT_RESULT_ENV`] is unset (the job starts its worker).
pub(crate) fn injected_outcome(pool: &Arc<MemoryPool>, raw_model: &[u8], cancelled: &dyn Fn() -> bool) -> Option<WorkerOutcome> {
    let path = std::env::var_os(INJECT_RESULT_ENV)?;
    let Some(root) = std::env::var_os(INJECT_ROOT_ENV) else {
        return Some(WorkerOutcome::Failed(format!("NATIVE-SMOKE-INJECT: {INJECT_ROOT_ENV} is unset; nothing is injected")));
    };
    Some(outcome_from_file(pool, raw_model, Path::new(&path), Path::new(&root), cancelled))
}

/// `path` opened for reading, when it is a regular file (not a link) inside `root`.
#[cfg(not(unix))]
fn open_inside(_path: &Path, _root: &Path) -> Result<File, String> {
    Err("injection checks the file with unix file identities; it is not available on this platform".into())
}

/// `path` opened for reading, when it is a regular file (not a link) inside `root`.
#[cfg(unix)]
fn open_inside(path: &Path, root: &Path) -> Result<File, String> {
    use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
    let root = root.canonicalize().map_err(|error| format!("injection root {}: {error}", root.display()))?;
    if !root.is_dir() {
        return Err(format!("injection root {} is not a directory", root.display()));
    }
    let link = path.symlink_metadata().map_err(|error| format!("{}: {error}", path.display()))?;
    if !link.file_type().is_file() {
        return Err(format!("{} is not a regular file (a link or another kind)", path.display()));
    }
    let resolved = path.canonicalize().map_err(|error| format!("{}: {error}", path.display()))?;
    if !resolved.starts_with(&root) || resolved == root {
        return Err(format!("{} is outside the injection root {}", path.display(), root.display()));
    }
    let file = std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW)
        .open(&resolved)
        .map_err(|error| format!("{}: {error}", resolved.display()))?;
    let opened = file.metadata().map_err(|error| format!("{}: {error}", resolved.display()))?;
    let checked = resolved.symlink_metadata().map_err(|error| format!("{}: {error}", resolved.display()))?;
    if !opened.is_file() || (opened.dev(), opened.ino()) != (checked.dev(), checked.ino()) {
        return Err(format!("{} changed while it was checked", resolved.display()));
    }
    Ok(file)
}

/// The outcome a worker would report for `raw_model` had it produced the bytes
/// of `path`, a regular file inside `root`.
fn outcome_from_file(pool: &Arc<MemoryPool>, raw_model: &[u8], path: &Path, root: &Path, cancelled: &dyn Fn() -> bool) -> WorkerOutcome {
    let failed = |message: String| WorkerOutcome::Failed(format!("NATIVE-SMOKE-INJECT: {message}"));
    // `d_M` as the worker computes it, before its solve.
    let model: serde_json::Value = match serde_json::from_slice(raw_model) {
        Ok(model) => model,
        Err(error) => return WorkerOutcome::Failed(format!("PREVIEW-SOLVE-MODEL-INVALID: {error}")),
    };
    let model_sha256 = streamed_digest(SOLVED_MODEL_DOMAIN, &model);
    drop(model);
    let mut file = match open_inside(path, root) {
        Ok(file) => file,
        Err(message) => return failed(message),
    };
    let length = match file.metadata() {
        Ok(metadata) => metadata.len(),
        Err(error) => return failed(format!("{}: {error}", path.display())),
    };
    // The result lease, as `supervise` admits it for a result frame: an
    // uncredited reservation while the bytes are read, then a Result lease.
    let mut lease = match pool.acquire_resampling(resident_envelope(length), LeasePurpose::Ingress, cancelled) {
        Ok(lease) => lease,
        Err(ResampleStop::Refused(refusal)) => return WorkerOutcome::Refused(refusal),
        Err(ResampleStop::Cancelled) => return WorkerOutcome::Terminated,
    };
    let mut result = Vec::with_capacity(usize::try_from(length).unwrap_or(0));
    match file.read_to_end(&mut result) {
        Ok(_) if result.len() as u64 == length => {}
        Ok(_) => return failed(format!("{} changed while it was read", path.display())),
        Err(error) => return failed(format!("{}: {error}", path.display())),
    }
    lease.set_purpose(LeasePurpose::Result);
    // `d_S` as the worker computes it, from the bytes parsed back; none (status
    // 0) when either digest is refused, as for a worker.
    let attestation = std::str::from_utf8(&result)
        .ok()
        .and_then(|text| Some(SolveAttestation { model_sha256: model_sha256?, result_sha256: result_digest(text)? }));
    WorkerOutcome::Completed { result, lease, attestation }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory_pool::test_host::ScriptedHost;
    use std::path::PathBuf;

    /// A fresh directory for one test: `root` (the injection root) and `outside` beside it.
    struct Dirs {
        base: PathBuf,
        root: PathBuf,
        outside: PathBuf,
    }
    impl Dirs {
        fn new(name: &str) -> Self {
            let base = std::env::temp_dir().join(format!("swbpipe-inject-{}-{name}", std::process::id()));
            let _ = std::fs::remove_dir_all(&base);
            let (root, outside) = (base.join("root"), base.join("outside"));
            std::fs::create_dir_all(&root).unwrap();
            std::fs::create_dir_all(&outside).unwrap();
            Self { base, root, outside }
        }
    }
    impl Drop for Dirs {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.base);
        }
    }
    fn failure(outcome: WorkerOutcome) -> String {
        match outcome {
            WorkerOutcome::Failed(message) => message,
            _ => panic!("expected a failed job"),
        }
    }

    #[test]
    fn the_file_is_published_under_a_result_lease_with_the_workers_attestation() {
        let dirs = Dirs::new("ok");
        let pool = MemoryPool::new(ScriptedHost::normal());
        let model = serde_json::json!({"project": {"id": "project:inject"}, "nodes": [{"id": "n", "x": -0.0}]});
        let raw = serde_json::to_vec(&model).unwrap();
        let text = r#"{"model_ref":"project:inject","results":[{"value":1.5}]}"#;
        let path = dirs.root.join("ok.json");
        std::fs::write(&path, text).unwrap();
        let WorkerOutcome::Completed { result, lease, attestation } = outcome_from_file(&pool, &raw, &path, &dirs.root, &|| false) else {
            panic!("an injected file completes");
        };
        assert_eq!(result, text.as_bytes());
        assert_eq!(lease.purpose(), LeasePurpose::Result);
        assert_eq!(lease.bytes(), resident_envelope(text.len() as u64));
        let value: serde_json::Value = serde_json::from_str(text).unwrap();
        assert_eq!(attestation, SolveAttestation::of(&model, &value), "d_M and d_S as the worker computes them");
    }

    #[test]
    fn a_missing_file_or_an_invalid_model_fails_the_job_and_holds_nothing() {
        let dirs = Dirs::new("missing");
        let pool = MemoryPool::new(ScriptedHost::normal());
        assert!(failure(outcome_from_file(&pool, b"{}", &dirs.root.join("missing.json"), &dirs.root, &|| false)).starts_with("NATIVE-SMOKE-INJECT: "));
        let path = dirs.root.join("model.json");
        std::fs::write(&path, b"{}").unwrap();
        assert!(failure(outcome_from_file(&pool, b"not json", &path, &dirs.root, &|| false)).starts_with("PREVIEW-SOLVE-MODEL-INVALID"));
        assert_eq!(pool.status(Default::default()).reserved_bytes, 0, "no lease outlives a failed injection");
    }

    #[test]
    fn only_a_regular_file_inside_the_root_is_read_never_a_link_or_a_file_outside() {
        let dirs = Dirs::new("contained");
        let pool = MemoryPool::new(ScriptedHost::normal());
        let inside = dirs.root.join("inside.json");
        let victim = dirs.outside.join("victim.json");
        std::fs::write(&inside, b"{}").unwrap();
        std::fs::write(&victim, b"{}").unwrap();
        let refused = |path: &Path, root: &Path| failure(outcome_from_file(&pool, b"{}", path, root, &|| false));
        // Outside the root, directly or by `..`.
        assert!(refused(&victim, &dirs.root).contains("outside the injection root"));
        assert!(refused(&dirs.root.join("../outside/victim.json"), &dirs.root).contains("outside the injection root"));
        // A link, to a file outside or even inside the root.
        std::os::unix::fs::symlink(&victim, dirs.root.join("escape.json")).unwrap();
        std::os::unix::fs::symlink(&inside, dirs.root.join("alias.json")).unwrap();
        assert!(refused(&dirs.root.join("escape.json"), &dirs.root).contains("not a regular file"));
        assert!(refused(&dirs.root.join("alias.json"), &dirs.root).contains("not a regular file"));
        // A linked parent directory that leads outside.
        std::os::unix::fs::symlink(&dirs.outside, dirs.root.join("sub")).unwrap();
        assert!(refused(&dirs.root.join("sub").join("victim.json"), &dirs.root).contains("outside the injection root"));
        // Not a file; no root.
        assert!(refused(&dirs.root, &dirs.root).contains("not a regular file"));
        assert!(refused(&inside, &dirs.base.join("no-such-root")).contains("injection root"));
        assert_eq!(pool.status(Default::default()).reserved_bytes, 0, "a refused injection holds no lease");
    }
}
