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
- The harness reports sparse live-path adoption and tolerance-policy `TBD`
  diagnostics rather than asserting default sparse adoption or release threshold
  policy. Sparse strategy selection itself is resolved by `DEC-023`.

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

- Accepted sparse numerical strategy is `DEC-023`
  (`core/solver/sparse_direct`); live solve-path adoption remains `TBD` pending
  D-17 ruling and follow-on implementation.
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

## 2026-06-04 - TP-AUTHORITY-REFRESH-0_7-DAG006

- WORKING_ITEMS/TASK workforce current-authority refresh applied to active deliverable-local surfaces for `DEL-04-05`.
- Current authority basis is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7` plus approved `execution/_DAG/DAG-006/` active graph authority.
- Historical run records, historical DAG row IDs, review dispositions, lifecycle `_STATUS.md`, aggregate DAG artifacts, candidate edges, repo-level governance files, schemas, code, and tools were intentionally not changed by this refresh.
- Preserved historical references remain evidence of earlier work, not current authority claims.

## 2026-06-04 - TASK repeatability and conditioning coverage

- Added deterministic harness record coverage in `core/solver/performance_harness` for per-repeat solution-delta/residual observations and reduced-matrix diagonal conditioning observations.
- Added focused tests confirming conditioning observations are recorded without supplied threshold policy and that under-restrained fixtures record solver diagnostics instead of selecting solver policy.
- Verification passed:
  `cargo fmt --manifest-path core/solver/performance_harness/Cargo.toml --check`,
  `cargo test --manifest-path core/solver/performance_harness/Cargo.toml`
  (8 tests), and `git diff --check`.
- Remaining `TBD`: accepted sparse numerical library, release timing/memory/practical-size bands, conditioning and CI threshold policy, hardware-normalized performance methodology, and future sparse-adapter integration.
- Boundaries preserved: no lifecycle state, DAG, dependency register, review disposition, solver-logic, sparse-library selection, production tolerance policy, protected-content, private-data, code-check, or professional/code-compliance claim changes.

## 2026-06-04 - TP-PHYS-024 parent fan-in

- WORKING_ITEMS fan-in reviewed the completed parallel TASK slice for `DEL-04-05` together with sibling slices for `DEL-04-04`, `DEL-05-02`, and `DEL-05-05`.
- Aggregate validation passed after fan-in: `cargo test --manifest-path core/solver/nonlinear_supports/Cargo.toml` (14 tests), `cargo test --manifest-path core/solver/performance_harness/Cargo.toml` (8 tests), `cargo test --manifest-path core/loads/load_case_algebra/Cargo.toml` (17 tests), `cargo test --manifest-path core/loads/user_loads/Cargo.toml` (28 tests), each corresponding `cargo fmt --check`, and `git diff --check`.
- Fan-in found no cross-worker scope drift for this deliverable. Lifecycle, DAG, dependency, review-disposition, release, professional-approval, and code-compliance surfaces remain unchanged.

## 2026-06-05 - Worker B deterministic suite-runner coverage

- Added `core/solver/performance_harness` suite-runner coverage over explicit
  invented cantilever-chain fixture sizes. The suite record preserves the
  existing per-fixture `HarnessRunRecord` values and adds suite-level counts for
  requested/completed fixtures, nodes, elements, DOFs, matrix nonzeros,
  reduced-matrix nonzeros, force nonzeros, repeat observations, conditioning
  observations, diagnostics, max reduced DOFs, max residual, and max solution
  delta.
- Existing single-fixture APIs remain compatible; new APIs are
  `run_invented_fixture_suite`, `run_default_invented_fixture_suite`, and
  `DEFAULT_INVENTED_SUITE_ELEMENT_COUNTS`.
- Focused tests now cover explicit fixture-size suite execution, default suite
  sizes, suite summary counts, preserved per-fixture assumptions/limitations/
  provenance/diagnostics, empty suite-size rejection, and zero-element fixture
  rejection.
- Verification passed:
  `cargo fmt --manifest-path core/solver/performance_harness/Cargo.toml --check`
  and
  `cargo test --manifest-path core/solver/performance_harness/Cargo.toml --locked`
  (12 tests, 0 failures).
- Remaining `TBD`: accepted sparse numerical library, release timing/memory/
  practical-size bands, conditioning and CI threshold policy,
  hardware-normalized performance methodology, and future sparse-adapter
  integration.
- Boundaries preserved: no lifecycle state, DAG, dependency register, review
  disposition, solver-logic, sparse-library selection, timing/memory threshold,
  CI gate, protected-content, private-data, release-readiness, professional-
  approval, or code-compliance claim changes.

## 2026-06-17 - Lifecycle Housekeeping

- Housekeeping lifecycle reset: `_STATUS.md` current state set to `IN_PROGRESS` to reflect current code development in progress. This does not change review, issuance, release readiness, professional approval, certification, sealing, authentication, or code-compliance status.

## 2026-06-21 - TP-R4-D17-PACKET-001

- WORKING_ITEMS prepared the D-17 sparse live-path adoption decision packet:
  `execution/_Coordination/_DECISIONS/D-17_sparse_solver_live_path_adoption.md`.
- The packet records the DEC-023 truth that the sparse solver strategy is no
  longer `TBD`: `core/solver/sparse_direct` is the accepted in-repo skyline
  profile solver. The open D-17 question is whether/how that solver enters the
  live R4 `frame_kernel` / `product_physics` / `nonlinear_integration` solve
  path.
- Advisory recommendation is Option B: add an R4 live sparse evidence lane while
  dense remains the default product path until measured promotion. This is a
  `PROPOSAL`; no ruling or implementation adoption is implied.
- Evidence basis: `sparse_direct` implements DEC-023; `performance_harness`
  measures sparse observations alongside dense and still emits live-adoption
  `TBD`; live product/nonlinear paths still call `solve_dense`.
- No code, schema, lifecycle state, release-readiness, professional approval,
  certification, sealing, authentication, or code-compliance claim was changed
  by this packet-preparation tranche.

## 2026-06-21 - TP-R4-D7-SPARSELIVE-001 sparse evidence lane adoption

- Implemented the human ruling `DEC-050` Option B: live sparse evidence is now
  bound into the R4 product/nonlinear solve path while dense remains the default
  solver and parity oracle.
- `core/solver/nonlinear_integration` and `core/product_physics` consume
  `core/solver/sparse_direct` as a non-blocking sidecar on dense-reduced
  systems. Evidence records include profile/bandwidth, pivot status, sparse
  residual, and dense-vs-sparse parity delta.
- `core/solver/diagnostics` and `core/solver/performance_harness` wording now
  distinguish the resolved `DEC-023` sparse strategy and landed `DEC-050`
  evidence lane from the remaining follow-on work.
- The invented mechanics fixture now has 802 result rows, including two
  unitless sparse parity evidence rows. Desktop unit and Playwright smoke
  expectations were updated to match that result surface.
- Validation passed, including full DEC-025 sweep
  `validation/evidence/sweeps/SWEEP_20260621T205711Z_c771567ed6a8-dirty.json`.
- Residuals remain explicit: profile-direct sparse assembly, sparse-default
  promotion, timing/memory thresholds, practical-size bands, conditioning/CI
  thresholds, and hardware-normalized methodology remain future work. No
  release-readiness, professional approval, certification, sealing,
  authentication, code-compliance claim, protected standards content, private
  data, network path, or telemetry feature changed.

## 2026-06-22 - TP-R4-D7-SPARSEPROFILE-001 product direct-profile sparse evidence

- Added explicit-entry profile assembly and solve APIs to
  `core/solver/sparse_direct`; duplicate lower-triangle entries are summed and
  invalid entry indices map to sparse numeric diagnostics.
- Product-preview sparse evidence now uses direct reduced profile entries
  assembled from frame elements, user-stiffness elements, and spring entries.
- Diagnostics now record that product direct profile assembly is observed while
  default sparse promotion remains `TBD`.
- Validation passed, including full DEC-025 sweep
  `validation/evidence/sweeps/SWEEP_20260622T085210Z_799ebcc0dee5-dirty.json`.
- Residuals remain explicit: default sparse promotion, nonlinear/core
  profile-direct promotion, timing/memory thresholds, practical-size bands,
  conditioning/CI thresholds, and hardware-normalized methodology remain future
  work.
