# TASK-PKG12-IMPL-001 Amendment v3 — Review Repair

- DetectionLayer: fresh read-only software-code-review attempt 1.
- FailureClass: `SECURITY_FAIL_OPEN`.
- ReasonCode: `MALFORMED_TRUTHY_LOCAL_PRIVATE_INTENT`.
- RequiredRepair:
  1. At Python route-control entry, normalize authorization exactly once as `explicit_local_private_intent is True` (or reject non-Boolean values) and pass only the normalized Boolean to both local-first storage and redaction controls.
  2. Ensure returned local-first evidence records the normalized value.
  3. Add route-level regression cases for `"false"`, `1`, `None`, and mapping values.
  4. Add writer-level regression proving malformed intent leaves the `REXC-CORE-007` result blocked and creates no output directory/files.
- AddedAllowedWriteTargets:
  - existing Python local-first/redaction files and tests from v1
  - `projects/chirality-piping/core/handoff/caepipe_external/run.py` only if implementation logic needs repair (test-only proof preferred)
  - `projects/chirality-piping/tests/test_caepipe_external_run_package.py`
- Checks: focused local-first/redaction tests; focused CAEPIPE writer tests; registered/pinned full Piping and DEC-025 after repair; refreshed exact hashes; fresh 100%-diff review.
- UnchangedExclusions: no scope expansion, payload inspection, network/cloud/telemetry behavior, storage-root selection, coordination/status writes, or Git mutation by child.
