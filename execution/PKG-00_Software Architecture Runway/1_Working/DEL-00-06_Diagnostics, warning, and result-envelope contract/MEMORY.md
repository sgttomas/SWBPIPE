---
doc_id: DEL-00-06-MEMORY
doc_kind: deliverable.memory
status: draft
created: 2026-05-17
---

# DEL-00-06 MEMORY

## 2026-05-17 TP-DIAG-019 Diagnostic Vocabulary Ruling

- Executed the user-approved TP-DIAG-019 diagnostic-boundary ruling directly
  in the parent/orchestrator thread and recorded it as a bounded TASK-style
  closeout.
- Preserved local diagnostic vocabularies at each boundary instead of adding a
  shared enum implementation in this tranche.
- Recorded the accepted mapping rule: diagnostics crossing a boundary preserve
  original `code`, `source`, `affected_object`, `message`, `remediation`, and
  provenance; only class vocabulary maps to the receiving boundary where a
  schema requires local classes.
- Applied examples: runner `RUNNER_BLOCKING` may map to result-export
  `EXPORT_BLOCKING`; runner `PRIVACY_WARNING` may map to result-export
  `IP_BOUNDARY_WARNING`; adapter/result `SOLVE_BLOCKING` and
  `ASSUMPTION_WARNING` remain local unless a receiving boundary requires
  mapping.
- Local run record:
  `_run_records/TASK_RUN_2026-05-17_TP-DIAG-019.md`.
- Validation passed: result schema, headless-runner contract, focused
  adapter/transform tests, result-export crate tests, headless-runner crate
  tests, mechanics benchmark tests, stress benchmark tests, and `git diff
  --check`.
- No shared diagnostic enum, lifecycle/status file, dependency register, DAG
  file, blocker queue, review disposition, release record, acceptance record,
  public API/CLI/report/runtime/persistence behavior, professional reliance
  claim, code-compliance claim, release statement, or human-acceptance
  statement was changed or introduced.

## 2026-05-17 TP-VERIFY-013E Diagnostic Boundary Reconciliation

- Reconciled TP-DIAG-019 mapping rules against current result-export,
  headless-runner, and physical-to-analytical adapter diagnostic vocabularies.
- Confirmed local diagnostic vocabularies remain aligned: result-export carries
  `EXPORT_BLOCKING`, runner carries `RUNNER_BLOCKING` and `PRIVACY_WARNING`,
  and the adapter remains on `SOLVE_BLOCKING` / `ASSUMPTION_WARNING`.
- Classified the remaining adapter-diagnostic crossing into result/export or
  runner envelopes as `READY_FOR_RUNTIME_TRACE_TRANCHE`, because receiving
  schemas require reference/provenance-shaped diagnostic fields.
- Classified section-evidence diagnostic transport as
  `READY_FOR_SECTION_EVIDENCE_SCHEMA_TRANCHE`, audit checksum diagnostic policy
  as `READY_FOR_AUDIT_CANONICALIZATION_RULING`, and shared diagnostic enum /
  release / acceptance / professional-reliance policy as `KEEP_AS_TBD`.
- Local run record:
  `_run_records/TASK_RUN_2026-05-17_TP-VERIFY-013E.md`.
- No schema, code, test, lifecycle/status, dependency register, DAG/blocker,
  review-disposition, release, acceptance, public API/CLI/report/persistence,
  runtime behavior, professional reliance, or code-compliance surface was
  changed.
