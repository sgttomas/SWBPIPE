# MEMORY - DEL-04-06 Solver Diagnostics And Singularity Detection

<!-- D41-R5-T7-PDU055-CURRENTNESS -->
## D-41 R5 T7 PDU-055 current declaration

Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.

Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

PDU-055 cited claim(s): `DEL-04-06-DECL-006`.

## Decisions And Rulings

- 2026-05-01 - Human project authority authorized proceeding with
  `DEL-04-06` as the recommended continuity item after `DEL-04-01`.
- 2026-05-01 - Candidate edge `DAG-001-E0622` remains non-gating; nonlinear
  support warning classes requiring `DEL-04-04` are not finalized in this
  slice.

## Implementation Summary

- Added `core/solver/diagnostics`, a Rust mechanics-diagnostics crate with a
  path dependency on `core/solver/frame_kernel`.
- The crate maps `FrameKernelError` variants into deterministic diagnostic
  records with code, severity, source, message, and optional affected
  reference.
- Added conditioning-ratio classification, nonconvergence diagnostics, and
  explicit sparse-solver/tolerance-policy `TBD` warning diagnostics.

## Evidence

- `cargo fmt --manifest-path core/solver/diagnostics/Cargo.toml --check`
  passed.
- `cargo test --manifest-path core/solver/diagnostics/Cargo.toml` passed:
  10 tests, 0 failures.

## Open TBDs

- Accepted sparse numerical library remains `TBD`.
- Release-quality solver tolerance thresholds remain `TBD`.
- Nonlinear-support warning classes remain future work pending `DEL-04-04`.
- Final result-envelope and application-service integration remains future
  work.

## Boundaries Preserved

- No protected standards text, protected tables, code-specific allowables,
  SIF/flexibility factors, proprietary catalog values, private project data, or
  professional/code-compliance claims were introduced.
- No lifecycle state transition, dependency-register edit, candidate-edge
  promotion, or blocker-queue refresh was performed.

## 2026-05-11 TP-RECON-01 Reconciliation

- TP-RECON-01 reconciled DEL-04-06 from archived DEV-001 evidence only. The
  dispatch matrix row maps this deliverable to commit `fdb0252` (`core: add
  solver diagnostics module`) and allows writes only to this `MEMORY.md` and
  `_STATUS.md`.
- Archived evidence records identify DEL-04-06 as `COMMITTED` on 2026-04-30
  for a bounded solver-diagnostics item; the REV05 evidence-status row notes
  that completeness still depends on refreshed graph/context review.
- The archived DEL-04-06 dispatch authorized deterministic solver diagnostics
  for singularity, invalid restraints/topology/numeric input, conditioning, and
  nonconvergence reporting, while leaving sparse solver selection, tolerance
  policy, nonlinear-support warning classes, and final result-envelope
  integration as `TBD` or future scope.
- `git show --name-status fdb0252` confirms the committed slice added
  `core/solver/diagnostics` files, updated `docs/SPEC.md` and `docs/TYPES.md`,
  added this deliverable `MEMORY.md`, and wrote coordination state for the
  original DEV-001 dispatch.
- Existing deliverable memory records verification at implementation time:
  `cargo fmt --manifest-path core/solver/diagnostics/Cargo.toml --check` and
  `cargo test --manifest-path core/solver/diagnostics/Cargo.toml` passed, with
  10 tests and 0 failures.
- TP-MAC-01 archive briefs later referenced DEL-04-06 as a source for
  mechanics/solver diagnostic classes, warning semantics, deterministic
  diagnostics, and failure states in preview workflows. Those references are
  treated as preview/workflow usage evidence, not a lifecycle release or
  engineering reliance status.
- Current state is preserved as `CHECKING` because committed implementation
  evidence exists and the source bundle does not authorize a stronger lifecycle
  state.

## 2026-05-15 TP-PHYS-002 Worker D

- Worker D fanned in current TP-PHYS-002 upstream frame/support/load failure
  shapes into `core/solver/diagnostics` only. The frame/support/load crates were
  already modified by other workers and were used as read-only API evidence.
- `diagnostic_from_frame_error` and `report_frame_error` signatures were
  preserved. New `FrameKernelError::RepeatedPrescribedDof` and
  `FrameKernelError::PrescribedDofOutOfRange` map to
  `InvalidRestraint` / `Blocking` / `ModelValidation` diagnostics with
  `dof:*` affected references.
- Added direct diagnostics helpers for linear support errors/findings,
  support-application errors, primitive load errors, and primitive load
  findings. Support/load finding affected references use the public
  `support_id` or `load_id`; application DOF errors use `dof:*`.
- Sparse solver selection and tolerance policy remain explicit warning-level
  `TBD` diagnostics. No sparse solver library, release tolerance threshold, or
  nonlinear-support diagnostic policy was selected in this slice.
- Validation passed:
  `cargo fmt --manifest-path core/solver/diagnostics/Cargo.toml --check`;
  `cargo test --manifest-path core/solver/diagnostics/Cargo.toml` (14 tests);
  `cargo test --manifest-path core/solver/frame_kernel/Cargo.toml` (23 tests);
  `cargo test --manifest-path core/solver/linear_supports/Cargo.toml`
  (12 tests); and
  `cargo test --manifest-path core/loads/primitive_loads/Cargo.toml`
  (20 tests).
- Boundary preserved: no protected standards text, protected tables,
  code-specific allowables, private project data, professional/code-compliance
  claims, lifecycle edits, dependency-register edits, coordination edits, DAG
  edits, benchmark edits, or GUI/app harness edits were introduced.

## 2026-05-16 - DEV-001 downstream PKG-02 audit memory addendum

Durable context preserved after reconciliation review:
- DEV-001 package-worker audit reviewed this deliverable for downstream compatibility with the accepted PKG-02 foundation contracts.
- Local audit artifacts are `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-06_Solver diagnostics and singularity detection/_REVIEW.md` and `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-06_Solver diagnostics and singularity detection/Review_Findings.csv`.
- Package audit summary is `execution/PKG-04_Solver Core and Numerical Methods/1_Working/_audit/PKG02_DOWNSTREAM_REVIEW_2026-05-16.md`; package run record is `execution/PKG-04_Solver Core and Numerical Methods/1_Working/_run_records/TASK_RUN_2026-05-16_PKG04_PKG02_DOWNSTREAM_AUDIT.md`.
- This was audit evidence only. It did not change lifecycle state, authorize release, or make a professional, certification, sealing, approval, or code-compliance claim.
- The May 16 package-worker TASK run record did not fully preserve canonical per-deliverable TASK documentation context; this addendum preserves the durable deliverable-local pointer without modifying the completed run record.

## 2026-05-16 - DEV-001 PKG-02 grounded finding-resolution memory addendum

Durable context preserved after PKG-02 grounded finding resolution:
- Stage 2 technical resolution used the accepted PKG-02 contract as the governing source for this deliverable's downstream compatibility evidence.
- Original audit finding count for this deliverable: 2 (WARNING=2). Current technical status count in the resolution matrix: TECHNICALLY_ADDRESSED_PENDING_HUMAN=2.
- Resolution evidence is indexed in `execution/_Reconciliation/Reviews/DEV001_FINDING_RESOLUTION_PKG02_GROUNDED_2026-05-16/RESOLUTION_MATRIX.csv`; validation evidence is summarized in `execution/_Reconciliation/Reviews/DEV001_FINDING_RESOLUTION_PKG02_GROUNDED_2026-05-16/VALIDATION_SUMMARY.md`.
- Local `Review_Findings.csv` entries remain subject to the human disposition gate. `HumanDisposition` stays `TBD` until review, and `Status` must not be changed to `RESOLVED` automatically.
- No lifecycle promotion, release claim, or professional/code-compliance claim is implied by the technical closeout.

## 2026-05-17 TP-PHYS-008-E Axial-Effect Finding Diagnostics

- Executed approved follow-up `TP-PHYS-008-E` TASK slice for `DEL-04-06` /
  `PKG-04` with write scope limited to `core/solver/diagnostics/**`, this
  `MEMORY.md`, and deliverable-local `_run_records/**`.
- Added deterministic diagnostic mapping for explicit axial-effect
  primitive-load finding codes from `DEL-05-01`.
- Missing axial-effect property findings map to `InvalidModelTopology`;
  invalid/non-finite axial-effect property or computed-force findings map to
  `InvalidNumericInput`. All remain blocking model-validation diagnostics with
  the primitive load ID as affected reference.
- Verification passed:
  `cargo fmt --manifest-path core/solver/diagnostics/Cargo.toml --check`;
  `cargo test --manifest-path core/solver/diagnostics/Cargo.toml axial_effect`
  with 2 focused tests passed.
- Remaining TBDs: sparse solver selection, tolerance policy,
  nonlinear-support diagnostics, final result-envelope integration, release
  thresholds, and professional reliance.
- No `_STATUS.md`, dependency register, DAG, blocker queue, candidate row,
  DEV-001 finding disposition, protected standards content, private data,
  code-compliance claim, release claim, or professional reliance claim was
  changed or introduced by this follow-up slice.

## 2026-06-04 - TP-AUTHORITY-REFRESH-0_7-DAG006

- WORKING_ITEMS/TASK workforce current-authority refresh applied to active deliverable-local surfaces for `DEL-04-06`.
- Current authority basis is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7` plus approved `execution/_DAG/DAG-006/` active graph authority.
- Historical run records, historical DAG row IDs, review dispositions, lifecycle `_STATUS.md`, aggregate DAG artifacts, candidate edges, repo-level governance files, schemas, code, and tools were intentionally not changed by this refresh.
- Preserved historical references remain evidence of earlier work, not current authority claims.

## 2026-06-05 - TP-DEL-04-03-04-06 Support Boundary Hardening Worker B

- Executed Worker B for tranche
  `TP-DEL-04-03-04-06-SUPPORT-BOUNDARY-HARDENING-001` with write scope limited
  to `core/solver/diagnostics/**`, this `MEMORY.md`, and deliverable-local
  `_run_records/**`.
- Repaired diagnostics compatibility with current upstream APIs by mapping
  `FrameKernelError::InvalidOrientation` to a blocking model-validation
  `InvalidModelTopology` diagnostic.
- Repaired primitive-load finding compatibility by mapping
  `MissingLoadId` to a blocking model-validation `InvalidModelTopology`
  diagnostic with stable affected reference `load:<missing-id>` when a load ID
  is missing or blank.
- Added focused tests for invalid frame orientation and missing primitive-load
  IDs while preserving existing support-application and primitive-load
  diagnostic mappings.
- Validation passed:
  `cargo fmt --manifest-path core/solver/diagnostics/Cargo.toml --check`;
  `cargo test --manifest-path core/solver/diagnostics/Cargo.toml --locked`
  with 19 tests passed;
  `cargo test --manifest-path core/loads/primitive_loads/Cargo.toml --locked`
  with 40 tests passed;
  `cargo test --manifest-path core/product_physics/Cargo.toml --locked`
  with 23 tests passed; and `git diff --check` passed.
- No lifecycle state, review finding disposition, dependency register, DAG
  artifact, coordination prompt, sparse solver, tolerance policy, nonlinear
  support behavior, release claim, professional approval, code-compliance
  claim, protected standards content, or private data was changed or
  introduced.

## 2026-06-05 - TP-DEL-04-03-04-06 Review Readiness Worker B

- Executed document/evidence alignment tranche
  `TP-DEL-04-03-04-06-REVIEW-READINESS-001` with write scope limited to
  `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, this
  `MEMORY.md`, and deliverable-local `_run_records/**`.
- Updated the four-document kit from setup-oriented wording to current
  `core/solver/diagnostics` implementation facts, including diagnostic
  envelope fields, provenance/remediation/unit metadata, analysis-boundary
  status mapping, frame/support/primitive-load mappings,
  conditioning/nonconvergence diagnostics, sparse/tolerance TBD diagnostics,
  and June 5 evidence for 19 passing diagnostics tests.
- Preserved deferrals for sparse solver selection, release tolerance
  thresholds, nonlinear support warning finalization, final result-envelope
  integration, release claims, professional approval, and code-compliance
  claims.
- No code, `_STATUS.md`, review finding disposition, dependency register, DAG
  artifact, coordination prompt, protected standards content, or private data
  was changed or introduced.

## 2026-06-05 - TP-DEL-04-03-04-06 Review Readiness Fan-In

- WORKING_ITEMS parent fan-in for
  `TP-DEL-04-03-04-06-REVIEW-READINESS-001` accepted Worker A and Worker B
  outputs as internally consistent for package-level gate preparation.
- Package fan-in record:
  `execution/PKG-04_Solver Core and Numerical Methods/1_Working/_run_records/TASK_RUN_2026-06-05_2226_TP-DEL-04-03-04-06_REVIEW-READINESS_C_FANIN.md`.
- Validation passed: format checks for `linear_supports`, `diagnostics`, and
  `frame_kernel`; tests for `linear_supports` (14), `diagnostics` (19),
  `frame_kernel` (34), and `primitive_loads` (40); `git diff --check`; stale
  wording scan; and protected/prohibited-claim scan.
- Recommendation recorded as `READY_FOR_HUMAN_CHECKING_GATE`. This is a gate
  packet recommendation only; no `_STATUS.md`, review finding disposition,
  dependency register, DAG artifact, lifecycle transition, release claim,
  professional approval, code-compliance claim, protected standards content, or
  private data was changed or introduced.

## 2026-06-11 - Edited-model solve binding guard

- `TP-APP-R2-SOLVEBOUND-001` added app-level diagnostic gating for edited
  browser-session models: stale bundled mechanics rows are not reused after a
  session model change, and browser fixture mode returns
  `BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL` as a blocking diagnostic
  with `MODEL_INCOMPLETE`.
- The solver backend path itself was not changed. Tauri tests verify existing
  `solve_preview_mechanics` command/job callers pass the supplied edited model
  payload through `open_pipe_stress_product_physics` and receive solved
  mechanics results bound to the edited `project.id`.
- Evidence: `_run_records/WORKING_ITEMS_RUN_2026-06-11_edited_model_solve_binding.md`;
  `apps/desktop/SMOKE.md` TP-MAC-88; desktop Vitest 31/31; Tauri Rust tests
  26/26; desktop build green.
- No solver diagnostic crate, lifecycle state, review finding disposition,
  dependency register, DAG artifact, protected/private data, release claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim was changed or introduced.

## 2026-06-17 - Lifecycle Housekeeping

- Housekeeping lifecycle reset: `_STATUS.md` current state set to `IN_PROGRESS` to reflect current code development in progress. This does not change review, issuance, release readiness, professional approval, certification, sealing, authentication, or code-compliance status.
