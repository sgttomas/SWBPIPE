# MEMORY - DEL-04-01 3D Frame Stiffness Kernel

## Decisions And Rulings

- 2026-05-01 - Human project authority authorized bounded implementation of
  `DEL-04-01` using
  `execution/_Coordination/DEV-001_DISPATCH_DEL-04-01.md`.
- 2026-05-01 - Implementation preserves the existing Rust crate path
  `core/solver/frame_kernel` and treats the dense solve interface as a
  temporary verification interface until the sparse numerical library is
  accepted.

## Implementation Summary

- `core/solver/frame_kernel` provides the first 3D frame kernel slice:
  two-node frame element stiffness, deterministic DOF ordering, local/global
  transform handling, dense assembly, boundary-condition reduction, and a
  temporary dense solve interface.
- The crate validates finite/positive numeric inputs, degenerate axes,
  repeated element node indices, invalid model node indices, repeated
  restrained DOFs, vector/matrix shape mismatches, and singular systems.
- Upstream unit/schema contracts remain responsible for unit compatibility and
  conversion. This deliverable does not define the final unit catalog,
  conversion constants, solver tolerances, sparse solver library, or canonical
  calculation unit basis.

## Evidence

- `cargo fmt --manifest-path core/solver/frame_kernel/Cargo.toml --check`
  passed.
- `cargo test --manifest-path core/solver/frame_kernel/Cargo.toml` passed:
  11 tests, 0 failures.
- Test coverage includes local stiffness terms, transform behavior, assembly,
  boundary-condition reduction, dense solve, singular-system detection,
  repeated element node index rejection, invalid model node index rejection,
  repeated restraint rejection, degenerate orientation rejection, and
  non-finite input rejection.

## Open TBDs

- Accepted sparse numerical library remains `TBD`.
- Solver tolerance policy remains `TBD`.
- Canonical calculation unit basis and conversion constants remain `TBD`.
- Downstream solver adapter boundaries and result-envelope integration remain
  future work.

## Boundaries Preserved

- No protected standards text, protected tables, code-specific allowables,
  SIF/flexibility factors, proprietary catalog values, private project data, or
  professional/code-compliance claims were introduced.
- No lifecycle state transition, dependency-register edit, candidate-edge
  promotion, or blocker-queue refresh was performed.

## 2026-05-11 TP-RECON-01 Reconciliation

- Reconciled archived `DEV-001` history for `DEL-04-01` to this deliverable
  record. The assigned `TP-RECON-01` dispatch row maps `DEL-04-01` / `PKG-04`
  to committed evidence `1506cc0` (`core: complete frame stiffness kernel`,
  2026-04-30); the archived evidence/status rows mark the implementation
  evidence as `COMMITTED`, and the lifecycle snapshot preserves `CHECKING`.
- Evidence-bearing implementation slice remains `core/solver/frame_kernel`:
  deterministic two-node 3D frame stiffness mechanics, documented DOF ordering,
  local/global transform handling, dense assembly, boundary-condition
  reduction, and a temporary dense solve interface. Commit `1506cc0` changed
  `core/solver/frame_kernel/.gitignore`, `README.md`, `src/lib.rs`,
  `docs/SPEC.md`, `docs/TYPES.md`, and this deliverable `MEMORY.md`.
- Verification evidence remains the committed/local record:
  `cargo fmt --manifest-path core/solver/frame_kernel/Cargo.toml --check` and
  `cargo test --manifest-path core/solver/frame_kernel/Cargo.toml` with 11
  tests passed. `TP-RECON-01` did not rerun solver verification or change code.
- Preserved boundaries and deferrals: no standards-derived/private data or
  reliance/compliance claims were introduced; sparse solver library, tolerance
  policy, canonical calculation unit basis/conversions, downstream
  adapter/result-envelope integration, and `TP-MAC-01-E` mechanics preview
  dispatch remain later gated work.

## 2026-05-12 TP-PHYS-001 Verification Hardening

- Executed one approved `TP-PHYS-001` TASK slice for `DEL-04-01` / `PKG-04`
  with write scope limited to `core/solver/frame_kernel/**`, this `MEMORY.md`,
  and deliverable-local `_run_records/**`.
- Strengthened the Rust crate verification surface in
  `core/solver/frame_kernel/src/lib.rs` with focused unit tests for rotated
  transform axis mapping, shared-node assembly accumulation, boundary
  out-of-range and force-length validation, dense solve partial pivoting,
  zero-pivot singularity handling, invalid dense matrix/non-finite entry
  rejection, and non-positive section/length rejection.
- Verification passed:
  `cargo fmt --manifest-path core/solver/frame_kernel/Cargo.toml --check`;
  `cargo test --manifest-path core/solver/frame_kernel/Cargo.toml` with 19
  tests passed; scoped `git diff --check` passed.
- Preserved open TBDs: sparse solver library, tolerance policy, canonical unit
  basis/conversion constants, release thresholds, and professional reliance.
- No protected standards text, protected tables, allowables, SIF/flexibility
  data, private data, code-compliance claim, lifecycle edit, dependency edit,
  coordination edit, or DAG edit was introduced.

## 2026-05-15 TP-PHYS-002 Prescribed-Displacement Reduction

- Executed Worker A for approved tranche `TP-PHYS-002` on `DEL-04-01` /
  `PKG-04` with write scope limited to `core/solver/frame_kernel/**`, this
  `MEMORY.md`, and deliverable-local `_run_records/**`.
- Added `reduce_system_with_prescribed_displacements` to
  `core/solver/frame_kernel/src/lib.rs`. The path removes prescribed DOFs and
  adjusts reduced free-DOF forces as `F_f - K_fc * u_c`.
- Preserved `reduce_system` as the backward-compatible zero-displacement
  boundary. Legacy restrained-DOF repeated and out-of-range error variants
  remain unchanged for existing callers.
- Added validation and tests for nonzero prescribed displacement force
  adjustment, zero prescribed displacement equivalence, repeated/out-of-range
  prescribed DOFs, and invalid prescribed displacement vector length/non-finite
  values. Updated the frame-kernel README to name the new reduction capability.
- Verification passed:
  `cargo fmt --manifest-path core/solver/frame_kernel/Cargo.toml --check`;
  `cargo test --manifest-path core/solver/frame_kernel/Cargo.toml` with 23
  tests passed. Scoped `git diff --check` was run after code and record edits.
- Preserved open TBDs: sparse solver library, tolerance policy, canonical unit
  basis/conversion constants, release thresholds, result-envelope integration,
  and professional reliance.
- No sparse solver, result envelope, units policy, release thresholds,
  protected standards text/tables, allowables, SIF/flexibility data, private
  data, code-compliance claim, lifecycle edit, dependency edit, coordination
  edit, DAG edit, GUI/app harness edit, or cross-deliverable edit was
  introduced.

## 2026-05-16 - DEV-001 downstream PKG-02 audit memory addendum

Durable context preserved after reconciliation review:
- DEV-001 package-worker audit reviewed this deliverable for downstream compatibility with the accepted PKG-02 foundation contracts.
- Local audit artifacts are `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-01_3D frame stiffness kernel/_REVIEW.md` and `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-01_3D frame stiffness kernel/Review_Findings.csv`.
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

- WORKING_ITEMS/TASK workforce current-authority refresh applied to active deliverable-local surfaces for `DEL-04-01`.
- Current authority basis is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7` plus approved `execution/_DAG/DAG-006/` active graph authority.
- Historical run records, historical DAG row IDs, review dispositions, lifecycle `_STATUS.md`, aggregate DAG artifacts, candidate edges, repo-level governance files, schemas, code, and tools were intentionally not changed by this refresh.
- Preserved historical references remain evidence of earlier work, not current authority claims.

## 2026-06-05 - Foundational hardening Worker A

- Executed Worker A for the approved foundational-hardening tranche on
  `DEL-04-01` / `PKG-04` with write scope limited to
  `core/solver/frame_kernel/**`, this `MEMORY.md`, and deliverable-local
  `_run_records/**`.
- Hardened `core/solver/frame_kernel` by adding explicit public six-DOF mapping
  helpers (`FrameDof`, `NODE_DOF_ORDER`, `node_dof_index`, and
  `element_dof_map`), a validated `FrameOrientation::new` constructor,
  stricter boundary metadata ID normalization, and documentation that the dense
  zero-pivot guard is internal verification behavior rather than a project
  tolerance policy.
- Expanded frame-kernel tests around DOF mapping, transform block mapping,
  orientation validation, nonzero node-index assembly, prescribed-displacement
  reduction ordering, singular zero-pivot guard behavior, and metadata
  boundaries.
- Verification passed:
  `cargo test --manifest-path core/solver/frame_kernel/Cargo.toml --locked`
  with 33 tests passed; `git diff --check` passed.
- Preserved open TBDs: sparse solver library, solver tolerance policy,
  canonical unit basis/conversion constants, release thresholds,
  result-envelope integration, and professional reliance.
- No sparse solver, tolerance policy, protected standards text/tables,
  allowables, SIF/flexibility data, private data, code-compliance claim,
  lifecycle edit, dependency edit, coordination edit, DAG edit, or
  cross-deliverable edit was introduced by this worker.

## 2026-06-05 - Foundational hardening parent fan-in

- WORKING_ITEMS completed parent fan-in for the approved foundational-hardening
  worker tranche spanning `DEL-04-01`, `DEL-05-01`, `DEL-05-04`, and
  `DEL-06-01`.
- Validation evidence passed: frame-kernel format check, frame-kernel locked
  test run with 33 tests, primitive-load format check, primitive-load locked
  test run with 40 tests, and the schema/status/rule-pack pytest suite with 17
  tests.
- Fan-in record:
  `_run_records/WORKING_ITEMS_RUN_2026-06-05_0705_FOUNDATIONAL-HARDENING_FANIN.md`.
- No lifecycle state, DAG artifact, dependency register, review disposition,
  release claim, professional approval, code-compliance claim, protected
  standards data, private data, or new implementation scope was changed by
  parent fan-in.

## 2026-06-05 - TP-DEL-04-03-04-06 Frame DOF Compatibility Shim

- During tranche `TP-DEL-04-03-04-06-SUPPORT-BOUNDARY-HARDENING-001`,
  `DEL-04-03` re-exported the frame-kernel `FrameDof` as the public support
  DOF type. Existing downstream callers in primitive-load and product-physics
  crates used `FrameDof::is_translational()`, so the helper was added to the
  frame-kernel `FrameDof` boundary rather than patching each downstream caller.
- Extended the existing frame-kernel six-DOF mapping test to assert
  translational/rotational classification.
- Validation passed:
  `cargo fmt --manifest-path core/solver/frame_kernel/Cargo.toml --check`;
  `cargo test --manifest-path core/solver/frame_kernel/Cargo.toml --locked`
  with 33 tests passed. Downstream compatibility was also validated by
  primitive-load and product-physics test runs recorded in the Worker B run
  record.
- No solver algorithm, sparse solver, tolerance policy, lifecycle state,
  dependency register, DAG artifact, review disposition, release claim,
  professional approval, code-compliance claim, protected standards content,
  or private data was changed or introduced.

## 2026-06-05 - Force-per-length canonical dimension propagation

- Executed Worker A for `TP-DEL-04-01-04-02-05-03-FORCE-PER-LENGTH-BOUNDARY-001`
  with write scope limited to `core/solver/frame_kernel/**`, this `MEMORY.md`,
  and deliverable-local `_run_records/**`.
- Added `CanonicalDimension::ForcePerLength` and mapped it to the accepted
  canonical dimension identifier `force_per_length` for downstream
  solver/load metadata boundaries.
- Updated `core/solver/frame_kernel/README.md` to state that the frame-kernel
  metadata vocabulary exposes `force_per_length` without defining unit
  catalogs, conversion constants, load defaults, or tolerance policy.
- Verification passed:
  `cargo fmt --manifest-path core/solver/frame_kernel/Cargo.toml --check`;
  `cargo test --manifest-path core/solver/frame_kernel/Cargo.toml --locked`
  with 34 tests passed.
- No `_STATUS.md`, review disposition, dependency register, DAG artifact,
  schema file, repo governance file, lifecycle state, release claim,
  professional approval, code-compliance claim, protected standards content,
  private data, unit conversion, load default, or tolerance policy was changed
  or introduced.

## 2026-06-05 - Force-per-length boundary parent fan-in

- WORKING_ITEMS completed parent fan-in for
  `TP-DEL-04-01-04-02-05-03-FORCE-PER-LENGTH-BOUNDARY-001` spanning
  `DEL-04-01`, `DEL-04-02`, and `DEL-05-03`.
- Validation evidence passed: frame-kernel format check and 34 locked tests;
  straight-pipe format check and 33 locked tests; stress-recovery format check
  and 24 locked tests; primitive-load locked tests with 40 tests;
  product-physics locked tests with 23 tests; `git diff --check`.
- Fan-in record:
  `_run_records/WORKING_ITEMS_RUN_2026-06-05_2000_FORCE-PER-LENGTH-BOUNDARY_FANIN.md`.
- No lifecycle state, review disposition, dependency register, DAG artifact,
  coordination prompt, schema file, repo governance file, release claim,
  professional approval, code-compliance claim, protected standards content,
  private data, conversion constant, load default, or tolerance policy was
  changed by parent fan-in.

## 2026-06-05 - Blocker closure and lifecycle-readiness review

- Human ruling packet: `execution/_Reconciliation/Reviews/PKG05_BLOCKER_CLOSURE_RULING_PACKET_2026-06-05_2120.md`.
- Review snapshot: `execution/_Reconciliation/Reviews/REV_DEL-04-01_2026-06-05_2120/`.
- Recommendation: `RECOMMEND_ADVANCE_TO_CHECKING` for `IN_PROGRESS -> CHECKING`.
- Lifecycle action: CHECKING applied by approved blocker-closure ruling.
- Finding disposition: Finding `PKG04-DEL0401-PKG02-001` was accepted as technically resolved by human ruling and set to `HumanDisposition=ACCEPT_AS_IS`, `Status=RESOLVED`.
- Dependency update: PKG-02 upstream rows `DAG-002-E0429`, `DAG-002-E0430`, and `DAG-002-E0431` were updated to `SATISFIED` from current frame-kernel boundary metadata, unit metadata, model-reference, and mechanics-boundary evidence.
- Validation: Frame-kernel format check passed; locked crate tests passed with 34 unit tests and 0 doctests.
- Residual boundaries: Sparse solver library, production tolerance policy, release thresholds, final result-envelope integration, and professional reliance remain explicit downstream or human-governed TBDs.
- No release, professional approval, certification, sealing, authentication, code-compliance claim, protected standards data, or private data was introduced.
