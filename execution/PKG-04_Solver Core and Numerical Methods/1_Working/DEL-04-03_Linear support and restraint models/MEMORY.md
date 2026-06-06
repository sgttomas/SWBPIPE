# MEMORY - DEL-04-03 Linear Support And Restraint Models

## Decisions And Rulings

- 2026-05-01: Human project authority authorized bounded implementation after
  the sealed dispatch brief was prepared.
- Implementation stays mechanics-only and linear-only. Nonlinear gap,
  lift-off, one-way, friction, and active-set behavior remain assigned to
  `DEL-04-04`.
- Support quantities retain dimension intent but do not resolve canonical
  calculation units, conversion constants, coordinate-frame policy, rigid
  restraint numerical method, sparse-solver integration, or final
  result-envelope integration.

## Implementation Notes

- Added `core/solver/linear_supports`, a Rust crate for anchors, guides, line
  stops, vertical supports, linear springs, and imposed displacement boundary
  data.
- The crate maps support records to frame-kernel global DOF indices and records
  deterministic findings for missing or invalid solve-required support data.
- No protected support defaults, catalog values, code-specific checks,
  dependency-register edits, lifecycle transitions, or blocker-queue refreshes
  were introduced.

## Verification

- `cargo fmt --manifest-path core/solver/linear_supports/Cargo.toml`
- `cargo test --manifest-path core/solver/linear_supports/Cargo.toml` passed
  8 tests.
- Adjacent solver checks passed:
  `cargo test --manifest-path core/solver/frame_kernel/Cargo.toml`,
  `cargo test --manifest-path core/solver/diagnostics/Cargo.toml`, and
  `cargo test --manifest-path core/solver/straight_pipe/Cargo.toml`.
- `git diff --check` passed.
- Focused protected-content/prohibited-claim scan found only boundary/negative
  statements and no bundled protected data or positive compliance claims.

## Open Items

- Canonical calculation unit basis and conversion constants remain `TBD`.
- Support coordinate-frame convention remains `TBD`; the current crate maps
  explicit node DOFs rather than arbitrary vector directions.
- Rigid-restraint numerical method and constraint-elimination or penalty
  strategy remain `TBD`.
- Sparse-solver and final result-envelope integration remain downstream work.

## 2026-05-11 TP-RECON-01 Reconciliation

- Archived DEV-001 evidence maps `DEL-04-03` to committed implementation
  `d227a27` (`core: add linear support models`, 2026-05-01). The revision 0.5
  evidence status row and lifecycle snapshot both preserve `CHECKING` with
  committed evidence present.
- `git show --name-status d227a27` corroborates the bounded implementation
  slice: new `core/solver/linear_supports` crate files, updates to `docs/SPEC.md`
  and `docs/TYPES.md`, and deliverable-local memory/coordination notes.
- Existing deliverable memory records the implemented mechanics-only support
  families, deterministic missing/invalid data findings, `cargo fmt`,
  `cargo test` for `linear_supports`, adjacent solver tests, `git diff --check`,
  and protected-content/prohibited-claim scan results.
- TP-MAC-01 planning later names `DEL-04-03` as reusable support/restraint
  input for the macOS technical-preview mechanics slice; `TP-MAC-01-E` remained
  a sealed brief for later dispatch and did not change lifecycle or evidence
  state.
- Deferred boundaries remain: nonlinear support behavior belongs to
  `DEL-04-04`; coordinate convention, canonical unit constants, rigid-restraint
  numerical method, sparse-solver integration, and final result-envelope
  integration remain downstream `TBD`s. This reconciliation records evidence
  only and does not add a release gate or engineering reliance decision.

## 2026-05-15 TP-PHYS-002 Worker B

- Added a solver-boundary application API in `core/solver/linear_supports`:
  `apply_linear_supports` validates the dense global system through the
  frame-kernel prescribed-displacement reducer, prepares supports with existing
  `prepare_boundary` behavior, blocks on support findings, adds prepared spring
  stiffness to global stiffness diagonals, and reduces rigid plus imposed
  displacement DOFs through
  `frame_kernel::reduce_system_with_prescribed_displacements`.
- Rigid restraints are represented as zero prescribed displacement entries for
  the frame-kernel boundary. Imposed displacement entries carry their prepared
  values. Existing duplicate, out-of-range, and missing-data findings remain
  blocking before spring application or solve reduction.
- Added deterministic tests for spring diagonal application, imposed
  displacement force adjustment through the frame-kernel boundary, rigid
  zero-restraint equivalence to legacy reduction, and support findings blocking
  application.
- Validation for this slice: `cargo fmt --manifest-path
  core/solver/linear_supports/Cargo.toml --check`; `cargo test
  --manifest-path core/solver/linear_supports/Cargo.toml` passed 12 tests;
  `cargo test --manifest-path core/solver/frame_kernel/Cargo.toml` passed 23
  tests; `git diff --check` for the assigned paths passed.
- Preserved boundaries: no nonlinear support behavior, penalty method, sparse
  solver, result envelope, coordinate convention resolution, canonical unit
  constants, release thresholds, professional/code-compliance claims, or
  protected standards data were introduced.

## 2026-05-16 - DEV-001 downstream PKG-02 audit memory addendum

Durable context preserved after reconciliation review:
- DEV-001 package-worker audit reviewed this deliverable for downstream compatibility with the accepted PKG-02 foundation contracts.
- Local audit artifacts are `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-03_Linear support and restraint models/_REVIEW.md` and `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-03_Linear support and restraint models/Review_Findings.csv`.
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

- WORKING_ITEMS/TASK workforce current-authority refresh applied to active deliverable-local surfaces for `DEL-04-03`.
- Current authority basis is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7` plus approved `execution/_DAG/DAG-006/` active graph authority.
- Historical run records, historical DAG row IDs, review dispositions, lifecycle `_STATUS.md`, aggregate DAG artifacts, candidate edges, repo-level governance files, schemas, code, and tools were intentionally not changed by this refresh.
- Preserved historical references remain evidence of earlier work, not current authority claims.

## 2026-06-05 - TP-DEL-04-03-04-06 Support Boundary Hardening Worker A

- Executed Worker A for tranche
  `TP-DEL-04-03-04-06-SUPPORT-BOUNDARY-HARDENING-001` with write scope limited
  to `core/solver/linear_supports/**`, this `MEMORY.md`, and
  deliverable-local `_run_records/**`.
- Re-exported the frame-kernel `FrameDof` through
  `open_pipe_stress_linear_supports::FrameDof` instead of maintaining a
  duplicate local enum, preserving the downstream import path while binding
  support DOF semantics to the upstream frame-kernel boundary.
- Updated `NodeDof::global_index()` to use
  `open_pipe_stress_frame_kernel::node_dof_index` and retained a private
  translational-DOF helper for guide, line-stop, and dimensional validation.
- Added focused test coverage proving `FrameDof` re-export compatibility and
  DOF index parity with the frame-kernel `NODE_DOF_ORDER` /
  `node_dof_index` boundary.
- Validation passed:
  `cargo fmt --manifest-path core/solver/linear_supports/Cargo.toml --check`;
  `cargo test --manifest-path core/solver/linear_supports/Cargo.toml --locked`
  with 14 tests passed; and
  `cargo test --manifest-path core/solver/frame_kernel/Cargo.toml --locked`
  with 33 tests passed.
- No lifecycle state, review finding disposition, dependency register, DAG
  artifact, coordination prompt, sparse solver, tolerance policy, nonlinear
  support behavior, release claim, professional approval, code-compliance
  claim, protected standards content, or private data was changed or
  introduced.
