# MEMORY - DEL-05-05 Concentrated and Distributed User Load Application

## Decisions And Rulings

- 2026-05-02 - Human project authority authorized `DEL-05-05`
  implementation after sealed dispatch brief preparation.
- 2026-05-02 - Implementation remains code-neutral: concentrated forces,
  concentrated moments, and distributed user loads are explicit mechanics
  inputs, not design-code load combinations or public default factors.

## Implementation Notes

- Added `core/loads/user_loads` as a focused Rust crate.
- The crate emits nodal force/moment contributions, element distributed-load
  contributions, and traceable result-recovery hooks.
- Missing or invalid targets, quantities, dimensions, directions, spans, and
  element lengths are deterministic findings.
- Downstream integration remains `TBD` for final result envelopes,
  persistence, API, CLI, GUI, reports, and production tolerance policy.

## Boundary Notes

- No protected standards data, proprietary engineering values, public default
  code factors, rule-pack checks, or professional/code-compliance claims were
  introduced.
- Artificial test values are invented mechanics fixtures only.

## 2026-05-11 TP-RECON-01 Reconciliation

- Reconciled history for `DEL-05-05` from the TP-RECON-01 dispatch row and the
  archived DEV-001 evidence bundle. Historical implementation evidence remains
  `COMMITTED` at `3cfcfd2` (`core: add user load application`) with lifecycle
  state `CHECKING`.
- Implemented slice recorded by the archive: `core/loads/user_loads` added
  explicit concentrated force, concentrated moment, and uniform distributed
  user-load records; deterministic nodal/element solver-boundary
  contributions; traceable result-recovery hooks; and deterministic findings
  for missing or invalid targets, quantities, dimensions, directions, spans,
  and element lengths.
- Verification recorded in the archived dispatch: formatting and Rust tests
  passed for `core/loads/user_loads`, adjacent load crates, and the frame
  kernel; protected-content/prohibited-claim scan found only boundary or
  exclusion language.
- Deferred scope remains unchanged: final result-envelope integration,
  persistence representation, application-service API, GUI/report/API/CLI
  presentation, production tolerance policy, and downstream product-preview
  consumption remain future work unless separately dispatched.
- Boundary preserved: TP-MAC-01 references this deliverable only as invented
  user-load preview/workflow evidence and does not add rule-pack checks,
  protected standards data, engineering defaults, or reliance authority.

## 2026-05-12 TP-PHYS-001 Mechanics Verification

- Added focused regression tests under `core/loads/user_loads` for missing
  user-load quantities and unsupported target/load-kind pairings. Existing
  behavior remains mechanics-only and continues to block invalid loads without
  emitting solver contributions or recovery hooks.
- Verification passed for `cargo fmt --manifest-path
  core/loads/user_loads/Cargo.toml --check`, `cargo test --manifest-path
  core/loads/user_loads/Cargo.toml`, and `cargo test --manifest-path
  core/loads/primitive_loads/Cargo.toml`.
- Boundaries preserved: no edits to `primitive_loads`, `load_case_algebra`,
  `stress_recovery`, status/dependency/coordination artifacts, GUI/product
  preview code, protected standards data, code combinations, public default
  factors, allowables, private data, or professional reliance claims.
- Deferred scope remains unchanged: final result-envelope integration,
  persistence representation, application-service API, GUI/report/API/CLI
  presentation, production tolerance policy, release thresholds, and
  professional reliance remain `TBD`.

## 2026-05-16 - DEV-001 downstream PKG-02 audit memory addendum

Durable context preserved after reconciliation review:
- DEV-001 package-worker audit reviewed this deliverable for downstream compatibility with the accepted PKG-02 foundation contracts.
- Local audit artifacts are `execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-05_Concentrated and distributed user load application/_REVIEW.md` and `execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-05_Concentrated and distributed user load application/Review_Findings.csv`.
- Package audit summary is `execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/_audit/PKG02_DOWNSTREAM_REVIEW_2026-05-16.md`; package run record is `execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/_run_records/TASK_RUN_2026-05-16_PKG02_DOWNSTREAM_AUDIT.md`.
- This was audit evidence only. It did not change lifecycle state, authorize release, or make a professional, certification, sealing, approval, or code-compliance claim.
- The May 16 package-worker TASK run record did not fully preserve canonical per-deliverable TASK documentation context; this addendum preserves the durable deliverable-local pointer without modifying the completed run record.

## 2026-05-16 - DEV-001 PKG-02 grounded finding-resolution memory addendum

Durable context preserved after PKG-02 grounded finding resolution:
- Stage 2 technical resolution used the accepted PKG-02 contract as the governing source for this deliverable's downstream compatibility evidence.
- Original audit finding count for this deliverable: 2 (WARNING=2). Current technical status count in the resolution matrix: TECHNICALLY_ADDRESSED_PENDING_HUMAN=2.
- Resolution evidence is indexed in `execution/_Reconciliation/Reviews/DEV001_FINDING_RESOLUTION_PKG02_GROUNDED_2026-05-16/RESOLUTION_MATRIX.csv`; validation evidence is summarized in `execution/_Reconciliation/Reviews/DEV001_FINDING_RESOLUTION_PKG02_GROUNDED_2026-05-16/VALIDATION_SUMMARY.md`.
- Local `Review_Findings.csv` entries remain subject to the human disposition gate. `HumanDisposition` stays `TBD` until review, and `Status` must not be changed to `RESOLVED` automatically.
- No lifecycle promotion, release claim, or professional/code-compliance claim is implied by the technical closeout.

## 2026-05-17 TP-PHYS-004 Straight-Pipe Equivalent User Loads

- Executed approved `TP-PHYS-004-B` TASK slice for `DEL-05-05` / `PKG-05`
  with write scope limited to `core/loads/user_loads/**`, this `MEMORY.md`,
  and deliverable-local `_run_records/**`.
- Added `UserLoadTarget::ElementStation` and
  `UserLoad::element_concentrated_force` for explicit point-force user loads
  at a station fraction.
- Added `apply_straight_pipe_equivalent_user_loads` to recover full-span
  distributed loads and station point forces into solver nodal contributions
  through the straight-pipe mechanics crate.
- Preserved deterministic blocking behavior: the generic user-load API reports
  `MissingElementGeometry` for element-station loads without a concrete pipe,
  and the straight-pipe API rejects partial spans, unsupported rotational
  directions, concentrated moments at element stations, invalid station
  fractions, wrong dimensions, and wrong element indices.
- Verification passed:
  `cargo fmt --manifest-path core/loads/user_loads/Cargo.toml`;
  `cargo test --manifest-path core/loads/user_loads/Cargo.toml` with 18 tests
  passed.
- Remaining TBDs: arbitrary-orientation global-to-local load mapping beyond
  the aligned straight-pipe scope, partial-span consistent loads, final
  result-envelope/API integration, production tolerance policy, release
  thresholds, and professional reliance.
- No `_STATUS.md`, dependency register, DAG, blocker queue, candidate row,
  DEV-001 finding disposition, protected standards data, public default load
  factors, rule-pack checks, code-compliance claim, or professional reliance
  claim was changed or introduced by this TASK slice.

## 2026-05-17 TP-PHYS-014-C Force-Per-Length Boundary Dimension

- Executed the `DEL-05-05` portion of `TP-PHYS-014-C` with write scope limited
  to `core/loads/user_loads/**`, README boundary text, this `MEMORY.md`, and
  deliverable-local `_run_records/**`.
- Updated the focused distributed user-load boundary metadata test so
  `ForcePerLength` quantities use canonical `force_per_length` metadata instead
  of the prior `TBD` placeholder.
- Updated README boundary text to align with the governed PKG-02 dimension.
- No user-load solver behavior, public default load factor, rule-pack behavior,
  lifecycle/status file, dependency register, DAG file, blocker queue, review
  disposition, protected standards content, private/proprietary data,
  finding-resolution statement, professional reliance statement,
  code-compliance statement, release statement, or human-acceptance statement
  was changed or introduced.

## 2026-05-16 TP-PHYS-006 Partial-Span Straight-Pipe User Loads

- Executed approved `TP-PHYS-006-B` TASK slice for `DEL-05-05` / `PKG-05`
  with write scope limited to `core/loads/user_loads/**`, this `MEMORY.md`,
  and deliverable-local `_run_records/**`.
- Updated `apply_straight_pipe_equivalent_user_loads` so valid partial-span
  distributed loads route through `SpannedGlobalUniformLoad` and the
  straight-pipe spanned equivalent-load helper.
- Valid partial spans now produce global equivalent nodal loads and recovery
  hooks; invalid spans remain deterministic `InvalidDistributionSpan`
  findings.
- Preserved deterministic blocking behavior for rotational distributed loads,
  wrong dimensions, wrong element index, mismatched element length, and
  unresolved geometry.
- Verification passed:
  `cargo fmt --manifest-path core/loads/user_loads/Cargo.toml --check`;
  `cargo test --manifest-path core/loads/user_loads/Cargo.toml` with 20 tests
  passed.
- Remaining TBDs: final result-envelope/API integration, production tolerance
  policy, release thresholds, and professional reliance.
- No `_STATUS.md`, dependency register, DAG, blocker queue, candidate row,
  DEV-001 finding disposition, protected standards data, public default load
  factors, rule-pack checks, code-compliance claim, or professional reliance
  claim was changed or introduced by this TASK slice.

## 2026-05-16 TP-PHYS-005 Oriented Straight-Pipe User Loads

- Executed approved `TP-PHYS-005-B` TASK slice for `DEL-05-05` / `PKG-05`
  with write scope limited to `core/loads/user_loads/**`, this `MEMORY.md`,
  and deliverable-local `_run_records/**`.
- Updated `apply_straight_pipe_equivalent_user_loads` to route straight-pipe
  distributed and station point-force loads through the orientation-aware
  global equivalent-load helper from `open_pipe_stress_straight_pipe`.
- `GlobalX`, `GlobalY`, and `GlobalZ` user-load directions are now converted
  to global force vectors before the straight-pipe helper projects them into
  local mechanics axes and returns global solver nodal contributions.
- Preserved deterministic blocking behavior for partial spans, rotational
  station loads, concentrated moments at element stations, invalid station
  fractions, wrong dimensions, wrong element indices, and unresolved geometry.
- Added a focused regression test for a global `X` user load on a pipe aligned
  with global `Y`.
- Verification passed:
  `cargo fmt --manifest-path core/loads/user_loads/Cargo.toml --check`;
  `cargo test --manifest-path core/loads/user_loads/Cargo.toml` with 19 tests
  passed.
- Fan-in regression also passed for transform and stress-recovery adjacency:
  `python3 -m pytest tests/test_physical_to_analytical_transform.py` with 7
  tests passed; `cargo test --manifest-path core/loads/stress_recovery/Cargo.toml`
  with 20 tests passed.
- Remaining TBDs: partial-span consistent loads, final
  result-envelope/API integration, production tolerance policy, release
  thresholds, and professional reliance.
- No `_STATUS.md`, dependency register, DAG, blocker queue, candidate row,
  DEV-001 finding disposition, protected standards data, public default load
  factors, rule-pack checks, code-compliance claim, or professional reliance
  claim was changed or introduced by this TASK slice.

## 2026-05-17 TP-PHYS-009 Axial Effects Into Solver Load Application

- Executed approved `TP-PHYS-009-A` TASK slice for `DEL-05-05` / `PKG-05`
  with write scope limited to `core/loads/user_loads/**`, this `MEMORY.md`,
  and deliverable-local `_run_records/**`.
- Added `apply_straight_pipe_equivalent_user_loads_with_axial_effects` as a
  sibling API that preserves the existing straight-pipe user-load assembly and
  appends already-prepared primitive axial-effect contributions.
- Reused `StraightPipeAxialEffect` and
  `StraightPipeElement::equivalent_global_axial_effect_loads`; no duplicate
  axial-effect mechanics were introduced in `user_loads`.
- Accepted axial-effect contributions emit nonzero global nodal force
  contributions with `UserLoadKind::ConcentratedForce` trace kind, the
  primitive `load_id`, and a deterministic
  `element:{index}:axial-effect:{load_id}` recovery hook using
  `RecoveryHookKind::ElementAxialEffect`.
- Invalid axial effects are deterministic findings: wrong element index emits
  `ElementOutOfRange`, nonfinite axial force emits `NonFiniteAxialEffect`, and
  straight-pipe equivalent-load generation failures surface as
  `MissingElementGeometry`.
- Verification passed:
  `cargo fmt --manifest-path core/loads/user_loads/Cargo.toml --check`;
  `cargo test --manifest-path core/loads/user_loads/Cargo.toml` with 23 tests
  passed.
- Remaining TBDs: primitive axial-effect provenance beyond `load_id`,
  final result-envelope/API integration, production tolerance policy, release
  thresholds, and professional reliance.
- No `_STATUS.md`, dependency register, DAG, blocker queue, candidate row,
  DEV-001 finding disposition, protected standards data, public default load
  factors, rule-pack checks, code-compliance claim, or professional reliance
  claim was changed or introduced by this TASK slice.

## 2026-06-04 - TP-AUTHORITY-REFRESH-0_7-DAG006

- WORKING_ITEMS/TASK workforce current-authority refresh applied to active deliverable-local surfaces for `DEL-05-05`.
- Current authority basis is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7` plus approved `execution/_DAG/DAG-006/` active graph authority.
- Historical run records, historical DAG row IDs, review dispositions, lifecycle `_STATUS.md`, aggregate DAG artifacts, candidate edges, repo-level governance files, schemas, code, and tools were intentionally not changed by this refresh.
- Preserved historical references remain evidence of earlier work, not current authority claims.

## 2026-06-04 - DEL-05-05 implemented-evidence documentation alignment

- WORKING_ITEMS dispatched a bounded `DELIVERABLE_TASK` to align
  deliverable-local docs with current implementation evidence in
  `core/loads/user_loads/README.md` and `core/loads/user_loads/src/lib.rs`.
- Updated `Datasheet.md`, `Specification.md`, `Guidance.md`, and
  `Procedure.md` from setup/future wording to implemented evidence for
  concentrated nodal forces, concentrated nodal moments, uniform distributed
  loads, element-station concentrated forces through straight-pipe recovery,
  valid partial-span distributed loads, oriented straight-pipe global load
  direction handling, and the primitive axial-effect bridge.
- Current evidence supersedes older historical open-item wording for
  straight-pipe oriented-load handling and straight-pipe partial-span
  distributed-load recovery. Other element families, other distribution
  shapes, and higher-level model assembly remain outside this deliverable
  unless separately dispatched.
- True remaining TBDs: final result-envelope/API/persistence/GUI/CLI/report
  integration, production tolerance policy, release thresholds, primitive
  axial-effect provenance beyond `load_id`, public/product-facing examples,
  and professional reliance.
- Boundary preserved: no `_STATUS.md`, dependency register, DAG,
  coordination artifact, schema, repo-level governance file, or core code was
  intentionally edited; no protected standards data, public default load
  factors, rule-pack checks, code-compliance claim, or professional reliance
  claim was introduced.
- Validation blocker: `cargo test --manifest-path
  core/loads/user_loads/Cargo.toml` failed because `src/lib.rs` does not
  cover `CanonicalSchemaBinding::ModelLoadCase` in the
  `ResultRecoveryHook::to_result_boundary_record` match. Core code was
  read-only for this task, so remediation requires a separate authorized code
  change or human ruling.

## 2026-06-04 - Station/span/transformed user-load hardening

- Executed bounded `DELIVERABLE_TASK` for `DEL-05-05` / `PKG-05` with write
  scope limited to `core/loads/user_loads/**`, this `MEMORY.md`, and
  deliverable-local `_run_records/**`.
- Hardened generic element-station handling so invalid element indices,
  invalid station fractions, unsupported station moments, rotational station
  directions, and wrong dimensions produce specific findings before the true
  straight-pipe geometry gap is reported.
- Hardened straight-pipe distributed-load recovery so nonfinite or
  nonpositive supplied element lengths report `NonPositiveElementLength`
  before geometry comparison.
- Added focused regression tests for generic station validation, transformed
  partial-span distributed loads, transformed station point forces, and
  unsupported rotational distributed-load combinations.
- Verification passed:
  `cargo fmt --manifest-path core/loads/user_loads/Cargo.toml --check`;
  `cargo test --manifest-path core/loads/user_loads/Cargo.toml` with 28 tests
  passed. The prior `ModelLoadCase` validation blocker was not present in the
  current crate state.
- Remaining TBDs: final result-envelope/API/persistence/GUI/CLI/report
  integration, production tolerance policy, release thresholds, primitive
  axial-effect provenance beyond `load_id`, public/product-facing examples,
  other element families, other distribution shapes, and professional
  reliance.
- No `_STATUS.md`, dependency register, DAG, blocker queue, candidate row,
  review disposition, protected standards data, public default load factors,
  rule-pack checks, code-compliance claim, or professional reliance claim was
  changed or introduced by this TASK slice.

## 2026-06-04 - TP-PHYS-024 parent fan-in

- WORKING_ITEMS fan-in reviewed the completed parallel TASK slice for
  `DEL-05-05` together with sibling slices for `DEL-04-04`, `DEL-04-05`, and
  `DEL-05-02`.
- Aggregate validation passed after fan-in:
  `cargo test --manifest-path core/solver/nonlinear_supports/Cargo.toml` (14
  tests), `cargo test --manifest-path core/solver/performance_harness/Cargo.toml`
  (8 tests), `cargo test --manifest-path core/loads/load_case_algebra/Cargo.toml`
  (17 tests), `cargo test --manifest-path core/loads/user_loads/Cargo.toml` (28
  tests), each corresponding `cargo fmt --check`, and `git diff --check`.
- Fan-in found no cross-worker scope drift for this deliverable. Lifecycle, DAG,
  dependency, review-disposition, release, professional-approval, and
  code-compliance surfaces remain unchanged.

## 2026-06-05 - Review readiness preparation

- Prepared lifecycle-review readiness evidence in `_run_records/TASK_RUN_2026-06-05_2052_REVIEW_READINESS_PREP.md` and classified the deliverable as `REVIEW_PREPARED_WITH_BLOCKERS`: current `cargo test --manifest-path core/loads/user_loads/Cargo.toml` passed with 28 tests, but `DEL-05-05` remains `IN_PROGRESS`, local dependencies for `DEL-05-01` and `DEL-04-01` remain `TBD`, upstream `DEL-04-01` remains `IN_PROGRESS`, and the two PKG-02 review findings still have `HumanDisposition=TBD`. This is evidence preparation only, not acceptance, lifecycle change, release, professional approval, or code-compliance certification.

## 2026-06-05 - Blocker closure and lifecycle-readiness review

- Human ruling packet: `execution/_Reconciliation/Reviews/PKG05_BLOCKER_CLOSURE_RULING_PACKET_2026-06-05_2120.md`.
- Review snapshot: `execution/_Reconciliation/Reviews/REV_DEL-05-05_2026-06-05_2120/`.
- Recommendation: `RECOMMEND_ADVANCE_TO_CHECKING` for `IN_PROGRESS -> CHECKING`.
- Lifecycle action: none; `_STATUS.md` remains `IN_PROGRESS` pending later Gate 5 approval.
- Finding disposition: Findings `DEL-05-05-PKG02-W001` and `DEL-05-05-PKG02-W002` were accepted as technically resolved by human ruling and set to `HumanDisposition=ACCEPT_AS_IS`, `Status=RESOLVED`.
- Dependency update: Rows `DAG-002-E0459` and `DAG-002-E0460` were updated to `SATISFIED` using DEL-05-01 and DEL-04-01 current evidence. Downstream row `DEL-05-05-E001` remains visible and non-blocking.
- Validation: Locked user-loads crate tests passed with 28 unit tests and 0 doctests.
- Residual boundaries: Final result-envelope/API/persistence/GUI/CLI/report integration, production tolerance policy, release thresholds, other element families, other distribution shapes, and professional reliance remain explicit TBDs.
- No release, professional approval, certification, sealing, authentication, code-compliance claim, protected standards data, or private data was introduced.

## 2026-06-05 - Gate 5 CHECKING approval applied

- Explicit Gate 5 approval changed `_STATUS.md` from `IN_PROGRESS` to
  `CHECKING` after blocker closure and review snapshot
  `execution/_Reconciliation/Reviews/REV_DEL-05-05_2026-06-05_2120/`.
- No additional dependency register, review finding, source code, schema,
  aggregate DAG artifact, release record, professional approval,
  certification, sealing, authentication, or code-compliance claim was changed.

## 2026-06-11 - TP-APP-R2-PRIMCREATE-001 concentrated primitive-load creation editor

- WORKING_ITEMS app-integration tranche consumed DEL-05-05 concentrated
  user-load design authority in the desktop Load Cases manager. No
  `core/loads/user_loads` source behavior changed.
- The GUI and operation seam now accept a single explicit concentrated nodal
  force primitive: category `concentrated_force`, existing node target,
  `global_x|global_y|global_z` direction, dimension `force`, project force
  unit, finite magnitude, and provenance.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_primitive_load_creation_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-98. Validation passed:
  operation-applier format check, Rust operation-applier tests 26/26,
  src-tauri Rust tests 26/26, desktop Vitest 41/41, desktop build, desktop
  Playwright smoke 1/1, and in-app browser concentrated-force smoke.
- Remaining DEL-05-05 residuals include distributed-load GUI integration,
  concentrated moments, element-station authoring, final result-envelope/API/
  persistence/report integration, production tolerance policy, release
  thresholds, and professional reliance.

## 2026-06-11 - TP-APP-R2-DISTLOAD-001 distributed primitive-load creation editor

- WORKING_ITEMS app-integration tranche consumed DEL-05-05 distributed
  user-load design authority in the desktop Load Cases manager. No
  `core/loads/user_loads` source behavior changed.
- The GUI and operation seam now accept a single explicit distributed element
  force primitive: category `distributed_force`, existing pipe target,
  `global_x|global_y|global_z` direction, dimension `force_per_length`,
  project force/length unit `N/m`, finite magnitude, and provenance.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_distributed_load_creation_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-99. Validation passed:
  operation-applier format check, Rust operation-applier tests 27/27,
  src-tauri Rust tests 26/26, desktop Vitest 43/43, desktop build, desktop
  Playwright smoke 1/1, and in-app browser distributed-force smoke.
- Remaining DEL-05-05 residuals include concentrated moments, element-station
  authoring, pressure/temperature primitive creation, imposed displacements,
  final result-envelope/API/persistence/report integration, production
  tolerance policy, release thresholds, and professional reliance.

## 2026-06-11 - TP-APP-R2-MOMENTCREATE-001 concentrated moment primitive-load creation editor

- WORKING_ITEMS app-integration tranche consumed DEL-05-05 concentrated
  moment design authority in the desktop Load Cases manager. No
  `core/loads/user_loads` source behavior changed.
- The GUI and operation seam now accept a single explicit concentrated nodal
  moment primitive: category `concentrated_moment`, existing node target,
  `rotation_x|rotation_y|rotation_z` direction, dimension `moment`, project
  force*length unit `N*m`, finite magnitude, and provenance.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_moment_load_creation_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-100. Validation passed:
  operation-applier format check, Rust operation-applier tests 28/28,
  src-tauri Rust tests 26/26, desktop Vitest 45/45, desktop build, desktop
  Playwright smoke 1/1, and in-app browser concentrated-moment smoke.
- Remaining DEL-05-05 residuals include pressure/temperature primitive
  creation, imposed displacements, final result-envelope/API/persistence/
  report integration, production tolerance policy, release thresholds, and
  professional reliance.

## 2026-06-11 - TP-APP-R2-PRESSTEMP-001 pressure and thermal primitive-load creation editor

- WORKING_ITEMS app-integration tranche consumed DEL-05-05 element-targeted
  pressure/thermal load design authority in the desktop Load Cases manager.
  No `core/loads/user_loads` source behavior changed.
- The GUI and operation seam now accept explicit pressure and thermal
  primitives: existing pipe target, `global_x|global_y|global_z` direction,
  dimensions `pressure` and `temperature_interval`, project units `Pa` and
  `degC`, finite magnitude, and provenance.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_pressure_thermal_load_creation_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-101. Validation passed:
  operation-applier format check, Rust operation-applier tests 29/29,
  src-tauri Rust tests 26/26, desktop Vitest 48/48, desktop build, desktop
  Playwright smoke 1/1, and in-app browser pressure/thermal smoke.
- Remaining DEL-05-05 residuals include imposed displacements, final
  result-envelope/API/persistence/report integration, production tolerance
  policy, release thresholds, and professional reliance.

## 2026-06-12 - TP-UNITS-B2-LOADPICKERS-001 primitive-load unit selectors

- WORKING_ITEMS app-integration tranche added visible unit selection to
  primitive-load creation in the desktop Load Cases manager. No
  `core/loads/user_loads` source behavior changed.
- The creation form now selects a magnitude unit for concentrated force,
  distributed force, moment, pressure, temperature, and imposed-displacement
  primitives. Browser preview uses the model metadata unit only; desktop/Tauri
  mode can offer accepted DEC-018 units for the selected primitive-load
  dimension.
- The operation seam now accepts compatible entered primitive-load units and
  preserves those units in the local session model. Rust regressions cover
  `lbf` concentrated force, `kPa` pressure, and incompatible `mm` rejection
  for a force load.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-12_primitive_load_unit_selectors.md`
  and `apps/desktop/SMOKE.md` TP-MAC-136. Validation passed:
  operation-applier cargo suites, src-tauri Rust tests 32/32, focused desktop
  Vitest 165/165, full desktop Vitest 216/216, desktop build, and Playwright
  R2 smoke 2/2.
- Remaining DEL-05-05 residuals include existing primitive-load magnitude-edit
  unit handling, final result-envelope/API/persistence/report integration,
  production tolerance policy, release thresholds, and professional reliance.
