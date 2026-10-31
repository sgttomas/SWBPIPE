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
use std::path::Path;
use std::sync::Arc;

/// Names the file whose bytes a started solve job publishes as its result.
pub(crate) const INJECT_RESULT_ENV: &str = "SWBPIPE_NATIVE_SMOKE_INJECT_RESULT";

/// The injected outcome for a job whose raw request body is `raw_model`, or
/// `None` when [`INJECT_RESULT_ENV`] is unset (the job starts its worker).
pub(crate) fn injected_outcome(pool: &Arc<MemoryPool>, raw_model: &[u8], cancelled: &dyn Fn() -> bool) -> Option<WorkerOutcome> {
    let path = std::env::var_os(INJECT_RESULT_ENV)?;
    Some(outcome_from_file(pool, raw_model, Path::new(&path), cancelled))
}

/// The outcome a worker would report for `raw_model` had it produced the bytes of `path`.
fn outcome_from_file(pool: &Arc<MemoryPool>, raw_model: &[u8], path: &Path, cancelled: &dyn Fn() -> bool) -> WorkerOutcome {
    let failed = |message: String| WorkerOutcome::Failed(format!("NATIVE-SMOKE-INJECT: {message}"));
    // `d_M` as the worker computes it, before its solve.
    let model: serde_json::Value = match serde_json::from_slice(raw_model) {
        Ok(model) => model,
        Err(error) => return WorkerOutcome::Failed(format!("PREVIEW-SOLVE-MODEL-INVALID: {error}")),
    };
    let model_sha256 = streamed_digest(SOLVED_MODEL_DOMAIN, &model);
    drop(model);
    let length = match std::fs::metadata(path) {
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
    let result = match std::fs::read(path) {
        Ok(bytes) if bytes.len() as u64 == length => bytes,
        Ok(_) => return failed(format!("{} changed while it was read", path.display())),
        Err(error) => return failed(format!("{}: {error}", path.display())),
    };
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

    fn written(name: &str, bytes: &[u8]) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!("swbpipe-inject-{}-{name}", std::process::id()));
        std::fs::write(&path, bytes).unwrap();
        path
    }

    #[test]
    fn the_file_is_published_under_a_result_lease_with_the_workers_attestation() {
        let pool = MemoryPool::new(ScriptedHost::normal());
        let model = serde_json::json!({"project": {"id": "project:inject"}, "nodes": [{"id": "n", "x": -0.0}]});
        let raw = serde_json::to_vec(&model).unwrap();
        let text = r#"{"model_ref":"project:inject","results":[{"value":1.5}]}"#;
        let path = written("ok", text.as_bytes());
        let WorkerOutcome::Completed { result, lease, attestation } = outcome_from_file(&pool, &raw, &path, &|| false) else {
            panic!("an injected file completes");
        };
        std::fs::remove_file(&path).unwrap();
        assert_eq!(result, text.as_bytes());
        assert_eq!(lease.purpose(), LeasePurpose::Result);
        assert_eq!(lease.bytes(), resident_envelope(text.len() as u64));
        let value: serde_json::Value = serde_json::from_str(text).unwrap();
        assert_eq!(attestation, SolveAttestation::of(&model, &value), "d_M and d_S as the worker computes them");
    }

    #[test]
    fn a_missing_file_or_an_invalid_model_fails_the_job_and_holds_nothing() {
        let pool = MemoryPool::new(ScriptedHost::normal());
        let missing = std::env::temp_dir().join(format!("swbpipe-inject-{}-missing", std::process::id()));
        assert!(matches!(outcome_from_file(&pool, b"{}", &missing, &|| false), WorkerOutcome::Failed(m) if m.starts_with("NATIVE-SMOKE-INJECT: ")));
        let path = written("model", b"{}");
        assert!(matches!(outcome_from_file(&pool, b"not json", &path, &|| false), WorkerOutcome::Failed(m) if m.starts_with("PREVIEW-SOLVE-MODEL-INVALID")));
        std::fs::remove_file(&path).unwrap();
        assert_eq!(pool.status(Default::default()).reserved_bytes, 0, "no lease outlives a failed injection");
    }
}
