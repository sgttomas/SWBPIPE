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
