# TASK-PKG12-IMPL-001 Amendment v2

- Reason: manager fan-in identified an in-scope native persistence bypass and one compatibility omission.
- AddedAllowedWriteTargets:
  - `projects/chirality-piping/apps/desktop/src-tauri/src/lib.rs`
  - existing desktop report save service/type/tests from v1
  - existing local-first route control/tests from v1
- AddedTasks:
  1. Carry the already-produced metadata-only local-first route evidence to `save_report_package` as a separate native command argument.
  2. Validate exact route/context/action/reason/blocked/metadata-only/explicit-intent values inside the Rust command before package assembly or the file picker. Extract a pure validator and add Rust unit tests for allowed, missing/mismatched, and blocked evidence.
  3. Preserve/report native receipt evidence where proportionate.
  4. Add `public_example` to the Python route-context mapping so composition does not narrow the existing redaction API.
- AddedChecks: focused desktop service tests/build; focused Rust unit test(s) in the existing `src-tauri` crate; prior Python focused suites; `git diff --check` and path containment.
- UnchangedExclusions: no native storage redesign, no new policy, no payload inspection, no network/cloud/telemetry behavior, no Git or coordination/status writes by the child.
