# MEMORY - DEL-09-03 Nonlinear support regression suite

## Scope

Implemented a bounded nonlinear support regression suite for DEV-001 revision
0.5 Tranche A using invented, non-project fixtures only.

## Evidence

- Added `validation/benchmarks/nonlinear/` as an executable Rust benchmark crate.
- Covered active-set one-way support, gap closure, lift-off, friction
  stick/slide behavior, and unresolved non-convergence at the iteration limit.
- Added `tests/test_nonlinear_support_regression.py` as the focused repo-level
  regression entry point for the nonlinear benchmark crate.

## Validation

- `cargo test --quiet` in `validation/benchmarks/nonlinear`: passed, 5 tests.
- `python3 -m pytest -q tests/test_nonlinear_support_regression.py`: passed, 2 tests.
- Existing `cargo test --quiet` in `core/solver/nonlinear_supports`: passed, 8 tests.
- Existing `cargo test --quiet` in `core/solver/diagnostics`: passed, 10 tests.
- `git diff --check`: passed.
- Protected-content, private-data, credential, and professional-claim scans over the
  DEL-09-03 changed files: passed.

## Remaining TBDs

- Production release tolerance policy remains `TBD`.
- Release thresholds for nonlinear support regression evidence remain `TBD`.
- External validation claims remain `TBD`; this suite is software verification
  evidence only.

## 2026-05-11 TP-RECON-01 Reconciliation

- Source bundle reviewed: TP-RECON-01 dispatch row for `DEL-09-03`, archived
  DEV-001 evidence rows, lifecycle snapshot, sealed Tranche A brief, Tranche A
  proposal and review closeout, commit `abdecbd`, and current deliverable
  records.
- Reconciled history: DEV-001 revision 0.5 Tranche A produced the nonlinear
  benchmark crate under `validation/benchmarks/nonlinear/`, the focused
  regression entry point `tests/test_nonlinear_support_regression.py`, and this
  deliverable `MEMORY.md`; archived evidence records promote that work to
  `COMMITTED` at `abdecbd` on 2026-05-04, with lifecycle preserved as
  `CHECKING`.
- Verification evidence preserved: nonlinear benchmark crate `cargo test`, the
  focused Python regression test, existing nonlinear support and diagnostics
  cargo tests, `git diff --check`, and protected-content/private-data/prohibited
  claim scans over the DEL-09-03 changed files.
- Deferred scope remains `TBD`: production tolerance policy, release thresholds,
  CI/publication policy, and external validation claims. This reconciliation
  records software verification and workflow evidence only.

## 2026-05-16 - DEV-001 downstream PKG-02 audit memory addendum

Durable context preserved after reconciliation review:
- DEV-001 package-worker audit reviewed this deliverable for downstream compatibility with the accepted PKG-02 foundation contracts.
- Local audit artifacts are `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-03_Nonlinear support regression suite/_REVIEW.md` and `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-03_Nonlinear support regression suite/Review_Findings.csv`.
- Package audit summary is `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/_audit/PKG02_DOWNSTREAM_REVIEW_2026-05-16.md`; package run record is `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/_run_records/TASK_RUN_2026-05-16_PKG09_PKG02_DOWNSTREAM_AUDIT.md`.
- This was audit evidence only. It did not change lifecycle state, authorize release, or make a professional, certification, sealing, approval, or code-compliance claim.
- The May 16 package-worker TASK run record did not fully preserve canonical per-deliverable TASK documentation context; this addendum preserves the durable deliverable-local pointer without modifying the completed run record.

## 2026-05-16 - DEV-001 PKG-02 grounded finding-resolution memory addendum

Durable context preserved after PKG-02 grounded finding resolution:
- Stage 2 technical resolution used the accepted PKG-02 contract as the governing source for this deliverable's downstream compatibility evidence.
- Original audit finding count for this deliverable: 2 (BLOCKER=1, WARNING=1). Current technical status count in the resolution matrix: TECHNICALLY_ADDRESSED_PENDING_HUMAN=2.
- Resolution evidence is indexed in `execution/_Reconciliation/Reviews/DEV001_FINDING_RESOLUTION_PKG02_GROUNDED_2026-05-16/RESOLUTION_MATRIX.csv`; validation evidence is summarized in `execution/_Reconciliation/Reviews/DEV001_FINDING_RESOLUTION_PKG02_GROUNDED_2026-05-16/VALIDATION_SUMMARY.md`.
- Local `Review_Findings.csv` entries remain subject to the human disposition gate. `HumanDisposition` stays `TBD` until review, and `Status` must not be changed to `RESOLVED` automatically.
- No lifecycle promotion, release claim, or professional/code-compliance claim is implied by the technical closeout.

## 2026-06-04 - TP-AUTHORITY-REFRESH-0_7-DAG006

- WORKING_ITEMS/TASK workforce current-authority refresh applied to active deliverable-local surfaces for `DEL-09-03`.
- Current authority basis is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7` plus approved `execution/_DAG/DAG-006/` active graph authority.
- Historical run records, historical DAG row IDs, review dispositions, lifecycle `_STATUS.md`, aggregate DAG artifacts, candidate edges, repo-level governance files, schemas, code, and tools were intentionally not changed by this refresh.
- Preserved historical references remain evidence of earlier work, not current authority claims.

## 2026-06-06 - Nonlinear dimension allowlist regression

- Changed files: `validation/benchmarks/nonlinear/src/lib.rs`, `tests/test_nonlinear_support_regression.py`, this `MEMORY.md`, and `_run_records/TASK_RUN_2026-06-06_1538.md`.
- Added validation-local `force_per_length` support to the nonlinear fixture dimension allowlist and a source-level regression check that confirms the token is present only in the current `CANONICAL_DIMENSIONS` list.
- Preserved unresolved tolerance policy posture: fixture `tolerance_policy` assignments remain `None`/TBD.
- Validation in this TASK run: `cargo test --manifest-path validation/benchmarks/nonlinear/Cargo.toml` passed; `python3 -m pytest -q tests/test_nonlinear_support_regression.py` passed; `git diff --check` passed.

## 2026-06-06 - TP-FORCE-PER-LENGTH Parent Fan-In

- Parent WORKING_ITEMS fan-in for the cross-deliverable force-per-length
  alignment tranche is recorded in
  `../DEL-09-01_Mechanics benchmark suite/_run_records/PARENT_FANIN_2026-06-06_1549_TP-FORCE-PER-LENGTH.md`.
- Parent validation passed: mechanics, stress, and nonlinear benchmark crates;
  schema/adapter/nonlinear/witness/results pytest suite; focused distributed
  `N/m` scan; and `git diff --check`.
- No lifecycle, review-disposition, release, acceptance, protected-content,
  private-data, professional-approval, or code-compliance surface was changed.

## 2026-06-06 - TP-PKG09-READINESS-DEL-09-03

- Hardened the focused nonlinear readiness pytest surface for fixture-family
  coverage, one hand-calculation note per public-original fixture, provenance
  wording, explicit fixture-local unit basis, unresolved tolerance posture, and
  prohibited protected-content/professional-claim wording.
- Preserved solver behavior and validation fixture data; this tranche added
  source-level readiness checks only.
- Production tolerance policy, release thresholds, CI/publication policy,
  external validation claims, unit catalog, conversion constants, and
  professional reliance remain `TBD`.
- Validation in this TASK run: nonlinear benchmark `cargo fmt --check` passed;
  nonlinear benchmark `cargo test` passed; focused nonlinear pytest passed.

## 2026-06-06 - TP-PKG09-READINESS Parent Fan-In

- Parent WORKING_ITEMS fan-in for the PKG-09 benchmark readiness tranche is
  recorded in
  `../DEL-09-01_Mechanics benchmark suite/_run_records/PARENT_FANIN_2026-06-06_TP-PKG09-READINESS.md`.
- Parent validation passed: mechanics, stress, and nonlinear benchmark crates;
  focused nonlinear/witness pytest; DAG-006 dependency schema validation; and
  `git diff --check`.
- No lifecycle, review-disposition, release, acceptance, protected-content,
  private-data, professional-approval, or code-compliance surface was changed.

## 2026-06-06 - TP-PKG09-READINESS-GATE Review

- Appended a `SELF_CHECK` readiness-gate section to `_REVIEW.md` and recorded
  local review evidence in
  `_run_records/REVIEW_RUN_2026-06-06_TP-PKG09-READINESS-GATE_DEL-09-03.md`.
- Parent review fan-in is recorded in
  `../DEL-09-01_Mechanics benchmark suite/_run_records/PARENT_REVIEW_FANIN_2026-06-06_TP-PKG09-READINESS-GATE.md`.
- Recommendation: mechanically ready for human-approved `IN_PROGRESS ->
  CHECKING`; `_STATUS.md` was not changed.
- Existing findings `PKG09-0903-PKG02-001` and `PKG09-0903-PKG02-002` remain
  technically addressed pending human disposition.
