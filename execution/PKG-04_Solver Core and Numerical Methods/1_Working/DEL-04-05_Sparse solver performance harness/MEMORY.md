# MEMORY - DEL-04-05 Sparse Solver Performance Harness

## Decisions And Rulings

- 2026-05-01 - Human project authority authorized proceeding with
  `DEL-04-05` as a bounded implementation item. The user typed
  `DEL-04-005`; project registers use `DEL-04-05`.
- 2026-05-01 - This slice consumes committed `DEL-04-01` frame-kernel evidence
  and committed `DEL-04-06` solver-diagnostics evidence.

## Implementation Summary

- Added `core/solver/performance_harness`, a Rust observer/regression crate
  with path dependencies on `core/solver/frame_kernel` and
  `core/solver/diagnostics`.
- Added an invented cantilever-chain fixture generator with explicit public
  provenance posture and rejection of unknown/protected fixture statuses.
- Added repeat-run harness records for matrix size, reduced DOFs, nonzero
  counts, force nonzero counts, repeatability delta, residual, condition-ratio
  estimate, diagnostics, assumptions, limitations, and provenance notes.
- The harness reports sparse-solver and tolerance-policy `TBD` diagnostics
  rather than choosing a sparse library or release threshold policy.

## Evidence

- `cargo fmt --manifest-path core/solver/performance_harness/Cargo.toml --check`
  passed.
- `cargo test --manifest-path core/solver/performance_harness/Cargo.toml`
  passed: 6 tests, 0 failures.
- Upstream Rust checks passed:
  `cargo test --manifest-path core/solver/diagnostics/Cargo.toml` and
  `cargo test --manifest-path core/solver/frame_kernel/Cargo.toml`.
- `git diff --check` passed.
- Focused protected-content/prohibited-claim scan found only boundary/negative
  statements, no bundled protected data or positive compliance claims.

## Open TBDs

- Accepted sparse numerical library remains `TBD`.
- Release timing, memory, practical-size bands, conditioning, and CI threshold
  policies remain `TBD`.
- Hardware-normalized performance methodology remains `TBD`.
- Future sparse-adapter integration remains downstream work.

## Boundaries Preserved

- No protected standards text, protected tables, code-specific allowables,
  SIF/flexibility factors, proprietary benchmark models, private project data,
  or professional/code-compliance claims were introduced.
- No solver logic, lifecycle state, dependency register, candidate edge, or
  blocker queue was changed.

## 2026-05-11 TP-RECON-01 Reconciliation

- Reconciled `DEL-04-05` history from the TP-RECON-01 dispatch row and
  archived DEV-001 evidence: commit `75f6688` (`core: add solver performance
  harness`, committed 2026-05-01) is the recorded implementation evidence.
- Evidence maps to `PKG-04` `TEST_SUITE`, scope `SOW-035`, and objectives
  `OBJ-003` and `OBJ-008`; the archived lifecycle snapshot records `CHECKING`
  with present status, context, dependency document, and local dependency CSV.
- Recorded changed implementation surfaces from the commit are
  `core/solver/performance_harness`, `docs/SPEC.md`, `docs/TYPES.md`, this
  deliverable `MEMORY.md`, and coordination handoff files.
- Historical verification evidence remains the DEV-001 summary in this file:
  cargo fmt/test checks, upstream diagnostics/frame-kernel tests,
  `git diff --check`, and protected-content/prohibited-claim scan.
- Deferred scope remains `TBD`: sparse numerical library, release
  timing/memory thresholds, practical model-size bands, conditioning and CI
  threshold policy, hardware-normalized methodology, and future sparse-adapter
  integration.
- Boundary preserved: TP-RECON-01 records history only, keeps lifecycle at
  `CHECKING`, and adds no code/schema/test changes, protected standards
  content, private/proprietary benchmark data, bundled standards-derived
  values, or reliance claims.

## 2026-05-16 - DEV-001 downstream PKG-02 audit memory addendum

Durable context preserved after reconciliation review:
- DEV-001 package-worker audit reviewed this deliverable for downstream compatibility with the accepted PKG-02 foundation contracts.
- Local audit artifacts are `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-05_Sparse solver performance harness/_REVIEW.md` and `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-05_Sparse solver performance harness/Review_Findings.csv`.
- Package audit summary is `execution/PKG-04_Solver Core and Numerical Methods/1_Working/_audit/PKG02_DOWNSTREAM_REVIEW_2026-05-16.md`; package run record is `execution/PKG-04_Solver Core and Numerical Methods/1_Working/_run_records/TASK_RUN_2026-05-16_PKG04_PKG02_DOWNSTREAM_AUDIT.md`.
- This was audit evidence only. It did not change lifecycle state, authorize release, or make a professional, certification, sealing, approval, or code-compliance claim.
- The May 16 package-worker TASK run record did not fully preserve canonical per-deliverable TASK documentation context; this addendum preserves the durable deliverable-local pointer without modifying the completed run record.

## 2026-05-16 - DEV-001 PKG-02 grounded finding-resolution memory addendum

Durable context preserved after PKG-02 grounded finding resolution:
- Stage 2 technical resolution used the accepted PKG-02 contract as the governing source for this deliverable's downstream compatibility evidence.
- Original audit finding count for this deliverable: 1 (WARNING=1). Current technical status count in the resolution matrix: TECHNICALLY_ADDRESSED_PENDING_HUMAN=1.
- Resolution evidence is indexed in `execution/_Reconciliation/Reviews/DEV001_FINDING_RESOLUTION_PKG02_GROUNDED_2026-05-16/RESOLUTION_MATRIX.csv`; validation evidence is summarized in `execution/_Reconciliation/Reviews/DEV001_FINDING_RESOLUTION_PKG02_GROUNDED_2026-05-16/VALIDATION_SUMMARY.md`.
- Local `Review_Findings.csv` entries remain subject to the human disposition gate. `HumanDisposition` stays `TBD` until review, and `Status` must not be changed to `RESOLVED` automatically.
- No lifecycle promotion, release claim, or professional/code-compliance claim is implied by the technical closeout.
