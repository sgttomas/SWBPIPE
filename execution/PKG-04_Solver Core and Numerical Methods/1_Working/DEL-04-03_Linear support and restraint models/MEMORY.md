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

## 2026-06-05 - TP-DEL-04-03-04-06 Review Readiness Worker A

- Aligned `Datasheet.md`, `Specification.md`, `Guidance.md`, and
  `Procedure.md` with current `DEL-04-03` implementation evidence instead of
  setup/future-implementation wording.
- Preserved current implemented facts: `FrameDof` re-export from
  `open_pipe_stress_frame_kernel`, DOF indexing through
  `node_dof_index`, `SupportQuantity` unit metadata retention,
  `prepare_boundary`, `apply_linear_supports`, and June 5 evidence of
  14 passing `linear_supports` tests.
- Preserved deferrals and boundaries for nonlinear supports, sparse solver
  integration, final result-envelope integration, support coordinate policy,
  release claims, professional/code-compliance claims, protected standards
  content, lifecycle state, review disposition, dependency registers, DAG
  artifacts, coordination files, and code files.

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

## 2026-06-11 - TP-APP-R2-IMPOSED-001 imposed-displacement primitive-load creation editor

- WORKING_ITEMS app-integration tranche used existing support records as
  explicit targets for desktop Load Cases manager imposed-displacement
  primitive creation.
- The authored payload records `{ type: "support", support: <support id>,
  dof: <DOF> }`; validation requires an existing support and a target DOF
  matching the primitive-load direction. Translational DOFs use the project
  length unit and dimension `displacement`; rotational DOFs use the project
  angle unit and dimension `rotation`.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_imposed_displacement_load_creation_editor.md`.
  Validation passed operation-applier format check, Rust operation-applier
  tests 30/30, src-tauri Rust tests 26/26, desktop Vitest 50/50, desktop
  build, desktop Playwright smoke 1/1, in-app browser support-target smoke,
  and `git diff --check`.
- No `core/solver/linear_supports` code, support coordinate policy, support
  stiffness/default-restraint behavior, solver boundary behavior, professional
  approval, code-compliance claim, protected standards content, or private
  data was changed or introduced.

## 2026-06-17 - Lifecycle Housekeeping

- Housekeeping lifecycle reset: `_STATUS.md` current state set to `IN_PROGRESS` to reflect current code development in progress. This does not change review, issuance, release readiness, professional approval, certification, sealing, authentication, or code-compliance status.

## 2026-06-21 - TP-R4-D5-HANGERDATA-001 spring-hanger user data

- WORKING_ITEMS implemented the D5 minimal dedicated user-entered
  spring-hanger model ruled by `DEC-049`.
- Added named support `hanger` data for invented variable-rate and
  constant-effort examples, with explicit stiffness/load/travel fields,
  source/manufacturer notes, load-side review notes, mechanics consumption
  posture, and no catalog/protected/default/professional claim.
- `core/product_physics` validates hanger units and required fields. Missing
  variable-hanger stiffness blocks with `SPRING_HANGER_STIFFNESS_MISSING`
  rather than applying a hidden default.
- The variable hanger consumes explicit user-entered stiffness through the
  existing linear spring primitive. Constant-effort support rows are evidence
  only; no global constant-effort solve behavior is claimed.
- Desktop model tree/detail/report/rendered-report/project-validation/native
  package surfaces now expose hanger provenance and user-entered review rows.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-21_TP-R4-D5-HANGERDATA-001.md`.
  Validation passed product-physics tests 43/43, desktop Vitest 407/407,
  desktop production build, and JSON fixture checks.

## 2026-07-19 - R14-W1-T2 constant-effort assembled-solve consumption

- Executed under sealed brief `CB-2026-07-19-T2-DEL-04-03-CONSTANT-EFFORT-001`
  (run `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14`, wave W1, tranche
  T2; v3 COMMIT-SAFE verifier chain). This is the DEC-049-anticipated deeper
  assembled-solve tranche for constant-effort supports.
- Ideal constant-effort element landed in `core/product_physics/src/lib.rs`:
  a support with `family = constant_effort_support`, no `nonlinear` field,
  exactly one declared translational restraint DOF, and a finite positive
  user-entered `hanger.constant_load` contributes a constant nodal force of
  that magnitude along the positive axis of the declared DOF in every solved
  load case. The force enters the per-load-case assembled force vector before
  `reduce_system` — the single seam shared by dense, sparse, and nonlinear
  active-set solves. Zero stiffness contribution; no restraint row.
- Consumption is data-driven opt-in: a constant-effort support not meeting
  the conditions (zero declared translational DOFs — the accepted pinned
  fixture shape — more than one, an unparseable declared DOF, an unresolvable
  node, or a missing/non-positive constant load) stays user-data review
  evidence and the solve emits one non-blocking
  `SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED` warning naming the unmet condition.
  No defaulting, no catalog value, no direction inference, and no
  previously-valid input became blocking (missing/non-positive constant load
  keeps the pre-existing `CONSTANT_EFFORT_LOAD_MISSING` validation block).
  A support with a `nonlinear` field keeps the existing nonlinear-path
  handling.
- Each consuming support emits a per-load-case
  `constant_effort_support_applied_load` row (basis cites `dec_ref=DEC-049`,
  `mechanics_consumption=assembled_solve`, `consumed_dof`; verbatim
  positive-axis sign convention). The existing
  `constant_effort_user_input_review` rows are preserved with their
  sign-convention disclosure updated truthfully to state the landed
  assembled-solve consumption semantics. User-entered `movement_limit` /
  `travel_range` are compared per load case against the computed displacement
  at the support node along the acting DOF; exceedance emits non-blocking
  `SUPPORT_CONSTANT_EFFORT_USER_LIMIT_EXCEEDED` (user-data comparison only;
  no software threshold, tolerance, or acceptance criterion).
- Evidence: new hand-calc witness
  `validation/hand_calcs/mechanics/constant_effort_support_applied_load.md`
  (invented, project-original; closed-form superposition values); new suite
  fixture `MECH-CONSTANT-EFFORT-SUPPORT-APPLIED-LOAD` in
  `validation/benchmarks/mechanics` (registered in `fixture_inventory()`,
  count assertion 21→22, one additive inventory line in each of the two
  mirror READMEs, compared at the already-recorded DEC-026 analytic-class
  `1.0e-9` tier); eight new product-physics unit tests (superposition
  identity, every non-consumption shape, direction convention, two-load-case
  application, nonlinear coexistence and nonlinear-field precedence, no-op
  guard, review-row updates). Product-physics tests 83/83, suite tests
  34/34, headless tests 23+1+15 all pass; the five
  `del1005_payload_binding_*` cases remain byte-identical to their committed
  witnesses; the pinned `tp_runner_015_final_cli_solve_input.json` case
  keeps exit 0/COMPLETED with output changed only by the new non-consumption
  warning, the truthful review-row text, and the consequent envelope
  checksum (before/after SHA-256 digests in the run record).
- Follow-on for HELP_HUMAN (docs lane; no docs write in this tranche): the
  reproduction-manual case-1 documented solve expectations
  (`docs/validation_manual/headless_runner_reproduction.md` Part 1) are
  stale for post-tranche sources — the same frozen command now also emits
  the `SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED` warning — and need a dated note
  through a later docs-lane selection per the R12→R13 case-3 precedent. The
  committed witnesses remain truthful for their pinned pre-#287 commits and
  were not edited.
- Boundaries: catalog sizing, protected/default values, hidden defaults,
  nonlinear constant-effort behavior (DEL-04-04 owner surface), DEC-046
  threshold state, and all lifecycle/issuance/release/professional claims
  remain untouched. Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-07-19_R14_W1_T2_CONSTANT_EFFORT.md`.
