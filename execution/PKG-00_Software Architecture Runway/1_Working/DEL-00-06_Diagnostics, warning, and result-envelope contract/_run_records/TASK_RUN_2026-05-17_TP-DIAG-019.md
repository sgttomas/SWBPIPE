---
run_id: TASK_RUN_2026-05-17_TP-DIAG-019
task: TP-DIAG-019 Diagnostic Vocabulary Ruling
deliverable_id: DEL-00-06
package_id: PKG-00
requested_by: WORKING_ITEMS orchestrator
execution_mode: parent_orchestrator_direct_task
date: 2026-05-17
---

# TP-DIAG-019 Diagnostic Vocabulary Ruling

## Loaded Truth Set

- Global: `AGENTS.md`, `agents/AGENT_WORKING_ITEMS.md`,
  `agents/AGENT_TASK.md`, `docs/CONTRACT.md`, `docs/SPEC.md`,
  `docs/TYPES.md`, `docs/IP_AND_DATA_BOUNDARY.md`,
  `execution/_Coordination/_COORDINATION.md`, `execution/_DAG/_LATEST.md`,
  and `execution/_Coordination/DEV-001_BLOCKER_QUEUE.md`.
- Deliverable-local: `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`,
  `_DEPENDENCIES.md`, `Datasheet.md`, `Guidance.md`, `Procedure.md`, and
  `Specification.md`.
- Note: no `Dependencies.csv` or pre-existing `MEMORY.md` was present for
  this architecture-runway deliverable before this tranche.

## Decision Applied

- Keep local diagnostic class vocabularies per boundary.
- Use mapping rules when diagnostics cross result/export/runner/adapter
  boundaries rather than creating a shared enum in this tranche.

## Mapping Rules Recorded

- Preserve original diagnostic `code`, `source`, `affected_object`, `message`,
  `remediation`, and provenance whenever a diagnostic crosses a boundary.
- Map only the diagnostic class when the receiving schema has local class
  vocabulary requirements.
- Runner `RUNNER_BLOCKING` maps to result-export `EXPORT_BLOCKING` when
  serialized as result-export blocking evidence.
- Runner `PRIVACY_WARNING` maps to result-export `IP_BOUNDARY_WARNING` where
  the result-export boundary carries privacy/IP warning evidence.
- Adapter/result `SOLVE_BLOCKING` and `ASSUMPTION_WARNING` remain local unless
  a receiving boundary explicitly requires a mapped class.

## Changes

- Created this deliverable `MEMORY.md` because none existed.
- Added this run record.
- No schema/code/test/runtime implementation was added for `DEL-00-06`.

## Validation

- `python3 tests/test_results_schema.py`
- `python3 tests/test_headless_runner_contract.py`
- `python3 -m pytest tests/test_physical_to_analytical_transform.py tests/test_analytical_solver_boundary_adapter.py`
- `cargo test --manifest-path core/reporting/result_export/Cargo.toml`
- `cargo test --manifest-path core/runner/headless/Cargo.toml`
- `cargo test --manifest-path validation/benchmarks/mechanics/Cargo.toml`
- `cargo test --manifest-path validation/benchmarks/stress/Cargo.toml`
- `git diff --check`

## Gaps

- No shared diagnostic enum is authorized.
- Any future shared-enum implementation or public diagnostic transport
  behavior requires a separate approved tranche.

## Boundaries Preserved

- No lifecycle/status changes.
- No dependency register, DAG, blocker, candidate-promotion, review-finding,
  release, or acceptance changes.
- No public API, CLI, GUI, report, persistence, process policy, or runtime
  behavior expansion.
- No professional reliance claim, code-compliance claim, release statement, or
  human-acceptance statement.

## No-Claim Closeout

This record documents a diagnostic-boundary decision and local evidence only.
It is not an acceptance record, release record, professional approval,
certification, sealing, authentication, or code-compliance claim.
