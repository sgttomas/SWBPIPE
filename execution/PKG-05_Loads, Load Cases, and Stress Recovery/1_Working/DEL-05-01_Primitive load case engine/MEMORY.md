# MEMORY - DEL-05-01 Primitive Load Case Engine

## Implementation Notes

- 2026-05-01: Implemented the bounded primitive load case engine under
  `core/loads/primitive_loads`.
- The slice defines mechanics-only primitive categories for weight, pressure,
  thermal, imposed displacement, hydrotest, wind, seismic, and occasional
  loads.
- The implementation prepares deterministic nodal, element-uniform, and
  imposed-displacement contributions for later consumers.
- Missing targets, missing magnitudes, invalid target ranges, invalid
  dimensions, invalid directions, and unsupported category/target combinations
  are findings, not silent defaults.

## Boundaries Preserved

- No code-specific load combinations were added.
- No protected standards data, proprietary engineering values, public catalog
  defaults, rule-pack checks, stress recovery, GUI behavior, headless runner, or
  professional/code-compliance claims were introduced.
- Wind and seismic are represented only as user-supplied equivalent mechanics
  loads. Dynamic or code-procedure generation remains `TBD`.

## Remaining TBDs

- Canonical calculation unit basis and conversion constants.
- Final result-envelope integration and concrete application-service API.
- Load-case storage representation.
- Wind/seismic dynamic treatment and any future lawful procedure generators.
- Production tolerance policy.

## 2026-05-11 TP-RECON-01 Reconciliation

- Primitive-load evidence: original commit `e3c9695` and the archived
  `DEV-001_DISPATCH_DEL-05-01.md` match the current
  `core/loads/primitive_loads` slice: explicit mechanics-only primitive
  categories, deterministic findings, nodal/element/imposed-displacement
  contributions, and no code-specific combinations or protected data.
- Thermal/pressure preview evidence: TP-MAC-06 and TP-MAC-09 are downstream
  product-preview mechanics slices. Current `core/product_physics` consumes
  DEL-05-01 thermal and pressure primitive load shapes, then applies bounded
  uniform axial thermal and closed-end pressure-thrust interpretation from
  explicit invented inputs; those behaviors do not expand this deliverable's
  primitive-load ownership.
- Verification recorded: DEL-05-01 historical evidence remains `COMMITTED`
  at `e3c9695` and `CHECKING`; TP-MAC-06 and TP-MAC-09 closeouts record focused
  `core/loads/primitive_loads`, `core/product_physics`, fixture-generation,
  product-preview, desktop, browser-smoke, and `git diff --check` verification
  for their downstream preview slices.
- Deferred boundaries: load-case algebra, stress recovery ownership,
  result-envelope/API integration, wind/seismic dynamic treatment, broader
  thermal/pressure behavior, protected rule/code checks, private data,
  release-readiness, and professional acceptance remain outside DEL-05-01.

## 2026-05-12 TP-PHYS-001 Mechanics Verification

- Executed one approved `TP-PHYS-001` TASK slice for `DEL-05-01` / `PKG-05`
  with write scope limited to `core/loads/primitive_loads/**`, this
  `MEMORY.md`, and deliverable-local `_run_records/**`.
- Added focused primitive-load tests under `core/loads/primitive_loads` for
  wind and seismic as explicit equivalent mechanics loads, rejection of
  acceleration/dynamic placeholder dimensions in this slice, and pressure/
  thermal rejection as nodal user-load stand-ins.
- Verification passed:
  `cargo fmt --manifest-path core/loads/primitive_loads/Cargo.toml --check`;
  `cargo test --manifest-path core/loads/primitive_loads/Cargo.toml` with 14
  tests passed; `git diff --check` passed.
- Preserved open TBDs: canonical unit basis/conversion constants, final
  result-envelope/API integration, load-case storage representation,
  wind/seismic dynamic treatment, production tolerance policy, release
  thresholds, and professional reliance.
- No load-case algebra, user-load module, stress-recovery behavior,
  GUI/product-preview behavior, report behavior, protected standards data,
  code combinations, allowables, private data, compliance claims, lifecycle
  edit, dependency edit, coordination edit, or DAG edit was introduced.

## 2026-05-15 TP-PHYS-002 Primitive ForcePerLength Lumping

- Executed the approved `TP-PHYS-002` Worker C slice for `DEL-05-01` /
  `PKG-05` with write scope limited to `core/loads/primitive_loads/**`, this
  `MEMORY.md`, and the deliverable-local `_run_records/**` entry.
- Added `ElementLoadSpan` and `prepare_lumped_nodal_loads` to convert explicit
  uniform translational/global `ForcePerLength` element loads into two equal
  nodal force contributions using caller-supplied element span and
  connectivity. Existing `prepare_loads` and `LoadApplication` behavior remain
  backward-compatible and still preserve element-uniform contributions.
- Added findings for missing span/connectivity, duplicate or invalid spans,
  invalid node/element indices, repeated element nodes, pressure/thermal
  categories in the lumping API, rotational directions, unsupported targets,
  and non-`ForcePerLength` inputs such as acceleration.
- Verification passed:
  `cargo fmt --manifest-path core/loads/primitive_loads/Cargo.toml --check`;
  `cargo test --manifest-path core/loads/primitive_loads/Cargo.toml` with 20
  tests passed.
- Preserved open TBDs: canonical unit basis/conversion constants, final
  result-envelope/API integration, load-case storage representation,
  wind/seismic dynamic treatment, production tolerance policy, release
  thresholds, and professional reliance.
- No pressure formulas, thermal generation, dynamic wind/seismic procedures,
  load-case algebra, user-load module changes, stress-recovery behavior,
  GUI/app harness behavior, report behavior, protected standards data, code
  combinations, allowables, private data, compliance claims, lifecycle edit,
  dependency edit, coordination edit, DAG edit, or validation benchmark edit
  was introduced.

## 2026-05-17 TP-PHYS-004 Deterministic Solver Load Assembly

- Executed approved `TP-PHYS-004-C` TASK slice for `DEL-05-01` / `PKG-05`
  with write scope limited to `core/loads/primitive_loads/**`, this
  `MEMORY.md`, and deliverable-local `_run_records/**`.
- Added `SolverNodalLoadContribution` and `assemble_solver_load_vector` for
  deterministic straight-pipe solver load-vector assembly from explicit nodal
  contributions.
- Assembly sorts accepted contributions by node, global DOF, and source ID,
  sums repeated node/DOF entries, and blocks without returning a partial vector
  when any contribution is out of range, non-finite, or inconsistent with the
  requested node/DOF space.
- Verification passed:
  `cargo fmt --manifest-path core/loads/primitive_loads/Cargo.toml`;
  `cargo test --manifest-path core/loads/primitive_loads/Cargo.toml` with 25
  tests passed.
- Remaining TBDs: load-case algebra beyond deterministic vector assembly,
  load-case storage representation, final result-envelope/API integration,
  production tolerance policy, release thresholds, and professional reliance.
- No `_STATUS.md`, dependency register, DAG, blocker queue, candidate row,
  DEV-001 finding disposition, protected standards data, code combinations,
  allowables, private data, code-compliance claim, or professional reliance
  claim was changed or introduced by this TASK slice.

## 2026-05-17 TP-PHYS-008 Primitive Axial Effects

- Executed approved `TP-PHYS-008-B` TASK slice for `DEL-05-01` / `PKG-05`
  with write scope limited to `core/loads/primitive_loads/**`, this
  `MEMORY.md`, and deliverable-local `_run_records/**`.
- Added explicit straight-pipe axial-effect property and contribution records:
  `ElementAxialEffectProperties`, `PrimitiveAxialEffectContribution`, and
  `PrimitiveAxialEffectApplication`.
- Added `prepare_straight_pipe_axial_effects(...)` for mechanics-only thermal
  axial force `E*A*alpha*DeltaT` and pressure thrust `p*A_internal` from
  caller-supplied properties.
- The helper returns deterministic findings for missing targets/magnitudes,
  wrong targets, wrong dimensions, missing/invalid properties, non-finite
  load magnitudes, non-finite computed axial effects, and out-of-range element
  indices. Any finding blocks all returned axial-effect contributions.
- Verification passed:
  `cargo fmt --manifest-path core/loads/primitive_loads/Cargo.toml --check`;
  `cargo test --manifest-path core/loads/primitive_loads/Cargo.toml` with 28
  tests passed.
- Remaining TBDs: canonical unit conversions, production tolerance policy,
  load-case storage, final result-envelope/API integration, release
  thresholds, and professional reliance.
- No `_STATUS.md`, dependency register, DAG, blocker queue, candidate row,
  DEV-001 finding disposition, protected standards data, code combinations,
  allowables, private data, code-compliance claim, release claim, or
  professional reliance claim was changed or introduced by this TASK slice.

## 2026-05-17 TP-PHYS-008-B2 Explicit Axial-Effect Findings

- Executed approved follow-up `TP-PHYS-008-B2` TASK slice for `DEL-05-01` /
  `PKG-05` with write scope limited to `core/loads/primitive_loads/**`, this
  `MEMORY.md`, and deliverable-local `_run_records/**`.
- Restored explicit axial-effect finding codes for missing/invalid
  element-property records, missing/invalid physical properties, non-finite
  load magnitudes, and non-finite computed axial effects.
- This replaces the temporary fan-in compatibility mapping through generic
  span/dimension finding codes. `DEL-04-06` now maps the explicit finding
  variants directly to solver diagnostics.
- Verification passed:
  `cargo fmt --manifest-path core/loads/primitive_loads/Cargo.toml --check`;
  `cargo test --manifest-path core/loads/primitive_loads/Cargo.toml straight_pipe_axial_effects`
  with 3 focused tests passed.
- Remaining TBDs: canonical unit conversions, production tolerance policy,
  load-case storage, final result-envelope/API integration, release
  thresholds, and professional reliance.
- No `_STATUS.md`, dependency register, DAG, blocker queue, candidate row,
  DEV-001 finding disposition, protected standards data, code combinations,
  allowables, private data, code-compliance claim, release claim, or
  professional reliance claim was changed or introduced by this follow-up
  slice.

## 2026-05-17 TP-PHYS-014-C Force-Per-Length Boundary Dimension

- Executed `TP-PHYS-014-C` for `DEL-05-01` / `PKG-05` with write scope limited
  to primitive-load boundary metadata, adjacent README text, this `MEMORY.md`,
  and deliverable-local `_run_records/**`.
- Added governed canonical dimension support for `force_per_length` in
  `core/loads/primitive_loads`: schema parsing accepts it, stringification
  returns it, and `LoadDimension::ForcePerLength` maps to it instead of `TBD`.
- Updated primitive-load and user-load README boundary text to remove the
  prior upstream-dimension gap statement.
- Parent fan-in validation covers the Rust formatting and test commands.
- No solver behavior, user-load mechanics behavior, lifecycle/status file,
  dependency register, DAG file, blocker queue, review disposition, protected
  standards content, owner criteria, private/proprietary data, lifecycle
  advancement, finding-resolution statement, professional reliance statement,
  code-compliance statement, release statement, or human-acceptance statement
  was changed or introduced.

## 2026-06-04 - TP-AUTHORITY-REFRESH-0_7-DAG006

- WORKING_ITEMS/TASK workforce current-authority refresh applied to active deliverable-local surfaces for `DEL-05-01`.
- Current authority basis is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7` plus approved `execution/_DAG/DAG-006/` active graph authority.
- Historical run records, historical DAG row IDs, review dispositions, lifecycle `_STATUS.md`, aggregate DAG artifacts, candidate edges, repo-level governance files, schemas, code, and tools were intentionally not changed by this refresh.
- Preserved historical references remain evidence of earlier work, not current authority claims.

## 2026-06-04 - TP-DEL-05-01-LOAD-CASE-RECORD

- Executed approved bounded tranche `TP-DEL-05-01-LOAD-CASE-RECORD` for
  `DEL-05-01` / `PKG-05` with write scope limited to
  `core/loads/primitive_loads/**`, this `MEMORY.md`, and deliverable-local
  `_run_records/**`.
- Added a storage-neutral primitive load-case boundary record in
  `core/loads/primitive_loads` that binds one primitive category to canonical
  model `LoadCase` metadata, provenance, payload, and hash references.
- Added deterministic load-case metadata helpers for schema load-type mapping,
  sorted load IDs, canonical field pairs, and stable round-trip key material.
- Added validation for model `LoadCase` schema binding, non-empty/non-`TBD`
  boundary refs, non-empty case name/provenance, non-empty loads, non-empty
  and unique load IDs, and single-category membership. Mixed-category algebra
  remains downstream `DEL-05-02` scope.
- Verification passed:
  `cargo fmt --manifest-path core/loads/primitive_loads/Cargo.toml`;
  `cargo fmt --manifest-path core/loads/primitive_loads/Cargo.toml --check`;
  `cargo test --manifest-path core/loads/primitive_loads/Cargo.toml` with 32
  tests passed; `git diff --check` passed.
- Remaining TBDs: canonical unit conversions, final result-envelope/API
  integration, production tolerance policy, release thresholds, and
  professional reliance.
- No `_STATUS.md`, `Dependencies.csv`, DAG artifact, schema file, repo-level
  governance file, PKG-01 deliverable, load-case algebra behavior, protected
  standards data, code-specific combination/default, private data, release
  claim, code-compliance claim, or professional reliance claim was changed or
  introduced by this tranche.

## 2026-06-04 - TP-DEL-05-01-FINDING-ENVELOPE-BRIDGE-001

- Executed approved bounded tranche
  `TP-DEL-05-01-FINDING-ENVELOPE-BRIDGE-001` for `DEL-05-01` / `PKG-05` with
  write scope limited to `core/loads/primitive_loads/**`, this `MEMORY.md`, and
  deliverable-local `_run_records/**`.
- Added storage-neutral diagnostic metadata records for primitive-load
  validation and load-case assembly findings. Records preserve local finding
  codes and carry class, severity, source, affected object, message,
  remediation, and provenance-reference fields for later result-envelope
  transport.
- Added conversion helpers for `LoadFinding` and `LoadCaseAssemblyFinding`
  collections without changing existing primitive-load preparation, lumping,
  axial-effect, load-case record, or solver-vector assembly behavior.
- Updated `core/loads/primitive_loads/README.md` to document the diagnostic
  bridge and its no-shared-enum/no-final-result-envelope boundary.
- Verification passed:
  `cargo fmt --manifest-path core/loads/primitive_loads/Cargo.toml --check`;
  `cargo test --manifest-path core/loads/primitive_loads/Cargo.toml` with 35
  tests passed; `git diff --check` passed.
- Remaining TBDs: canonical unit conversions, final result-envelope/API
  integration, production tolerance policy, release thresholds, and
  professional reliance.
- No `_STATUS.md`, `Dependencies.csv`, DAG artifact, schema file, repo-level
  governance file, shared diagnostic enum, load-case algebra behavior,
  protected standards data, code-specific combination/default, private data,
  release claim, code-compliance claim, or professional reliance claim was
  changed or introduced by this tranche.

## 2026-06-04 - TASK Doc Evidence Alignment

- Executed bounded WORKING_ITEMS/TASK documentation alignment for `DEL-05-01`
  with write scope limited to the four deliverable artifacts, this `MEMORY.md`,
  and deliverable-local `_run_records/**`.
- Updated `Datasheet.md`, `Specification.md`, `Guidance.md`, and
  `Procedure.md` to replace stale setup/future-only wording with current
  evidence from `core/loads/primitive_loads/README.md`, `src/lib.rs`, and the
  latest deliverable-local run history.
- Aligned the documents to implemented primitive categories, storage-neutral
  primitive load-case records, boundary metadata, diagnostic bridge records,
  lumped equivalent nodal conversion, straight-pipe axial effects, solver
  load-vector assembly helpers, and the current 35-test evidence surface.
- Preserved open TBDs for canonical unit conversions, final result-envelope/API
  integration, production tolerance policy, release thresholds, wind/seismic
  dynamic treatment, broader property/default sourcing policy, human
  acceptance, code compliance, and professional reliance.
- Verification passed:
  `cargo test --manifest-path core/loads/primitive_loads/Cargo.toml` with 35
  tests passed; scoped `git diff --check` for touched deliverable files passed.
- No `_STATUS.md`, `Dependencies.csv`, `_DEPENDENCIES.md`, review file, schema
  file, DAG artifact, repo-level governance file, coordination file, core code,
  protected standards data, code-specific combination/default, private data,
  release claim, code-compliance claim, or professional reliance claim was
  changed or introduced by this TASK.

## 2026-06-05 - Foundational Primitive-Load Hardening Worker B

- Executed bounded Worker B for `DEL-05-01` / `PKG-05` with write scope
  limited to `core/loads/primitive_loads/**`, this `MEMORY.md`, and the
  deliverable-local `_run_records/**` entry.
- Hardened `core/loads/primitive_loads` category and dimension metadata with
  stable primitive category names, category-to-load-case mapping helpers,
  load-dimension canonical mapping helpers, and explicit rejection of `TBD`
  dimensions for concrete quantity metadata.
- Added equivalent-static mechanics preparation for wind, seismic, and
  occasional primitive categories only when callers supply explicit
  basis/provenance refs. Dynamic procedures, response parameters, code factors,
  environmental defaults, and conversion constants remain `TBD` and outside
  this slice.
- Added missing load/source identifier diagnostics, safe diagnostic
  affected-object labels for missing IDs, non-finite primitive magnitude
  findings across preparation helpers, and non-finite assembled-sum checks in
  deterministic solver load-vector assembly.
- Verification passed:
  `cargo test --manifest-path core/loads/primitive_loads/Cargo.toml --locked`
  with 40 tests passed. `git diff --check` is recorded in the run record.
- Remaining TBDs: canonical unit conversions, production tolerance policy,
  final result-envelope/API integration, release thresholds, wind/seismic
  dynamic treatment, code-specific factors/defaults, code compliance, and
  professional reliance.
- No `_STATUS.md`, `Dependencies.csv`, `_DEPENDENCIES.md`, DAG artifact,
  schema file, repo-level governance file, load-case algebra behavior,
  protected standards data, code-specific combination/default, private data,
  release claim, code-compliance claim, or professional reliance claim was
  changed or introduced by this Worker B tranche.

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

## 2026-06-05 - Review-readiness parent fan-in

- WORKING_ITEMS completed the bounded review-readiness tranche for `DEL-05-01`
  with three TASK workers: evidence audit, four-document alignment, and
  downstream handoff/readiness check.
- Worker A validation passed:
  `cargo fmt --manifest-path core/loads/primitive_loads/Cargo.toml --check`;
  `cargo test --manifest-path core/loads/primitive_loads/Cargo.toml --locked`
  with 40 tests; and `git diff --check`.
- Worker B updated `Datasheet.md`, `Specification.md`, `Guidance.md`, and
  `Procedure.md` to reflect the 2026-06-05 foundational hardening evidence,
  including equivalent-static wind/seismic/occasional handling with explicit
  basis/provenance refs and the 40-test evidence state.
- Worker C recorded `READY_FOR_DOWNSTREAM_HANDOFF_WITH_LIMITS`: `DEL-05-02`
  may consume `DEL-05-01` as primitive one-category load-case and diagnostic
  boundary evidence, not as mixed-category algebra or code-combination
  authority.
- Final parent validation after fan-in passed:
  `cargo fmt --manifest-path core/loads/primitive_loads/Cargo.toml --check`;
  `cargo test --manifest-path core/loads/primitive_loads/Cargo.toml --locked`
  with 40 tests; and `git diff --check`. Focused protected-content,
  private-data, and authority-claim scan results were boundary wording only.
- Parent fan-in record:
  `_run_records/WORKING_ITEMS_RUN_2026-06-05_2016_REVIEW-READINESS-FANIN.md`.
- Remaining TBDs: canonical unit conversions, final result-envelope/API
  integration, production tolerance policy, release thresholds, wind/seismic
  dynamic treatment, occasional-event mapping, code-specific factors/defaults,
  broader property/default sourcing, human acceptance, code compliance, and
  professional reliance.
- No `_STATUS.md`, `Dependencies.csv`, `_DEPENDENCIES.md`, DAG artifact,
  schema file, repo-level governance file, coordination file, code file,
  review disposition, lifecycle state, protected standards data, code-specific
  combination/default, private data, release claim, code-compliance claim, or
  professional reliance claim was changed or introduced by this fan-in.

## 2026-06-11 - TP-APP-R2-LOADMGR-001 load-case primitive magnitude manager

- WORKING_ITEMS app-integration tranche consumed the existing primitive-load
  model evidence in the desktop GUI. No `core/loads/primitive_loads` source
  behavior changed.
- The new Load Cases manager surfaces the invented preview primitive loads,
  including weight, occasional, pressure, and thermal records, with target,
  direction, dimension, magnitude, and unit metadata.
- Existing primitive-load magnitude fields can now be queued as structured
  `update_load` operations. The tested pressure row is `load:L-100-P` at
  `primitive_loads.2.magnitude.value`, updated from `1200000 Pa` to
  `1500000 Pa` in local session state.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_load_case_manager_primitive_magnitude.md`
  and `apps/desktop/SMOKE.md` TP-MAC-91. Validation passed:
  `npm test --workspace apps/desktop` (32/32), `npm run build --workspace
  apps/desktop`, `npm run test:e2e:desktop` (1/1), and `git diff --check -- .
  ':!init/init-prompt.md'`.
- This tranche does not add new primitive category semantics, arbitrary
  primitive-load creation, unit conversion, solver validation, code-specific
  factors/defaults, protected standards data, private data, release claim,
  code-compliance claim, or professional reliance claim.

## 2026-06-11 - TP-APP-R2-LOADMETA-001 load-case metadata editor

- WORKING_ITEMS app-integration tranche added selected load-case `status` and
  `kind` metadata editing to the desktop Load Cases manager for the invented
  preview model. No `core/loads/primitive_loads` source behavior changed.
- The editor queues explicit structured `update_load` intents with
  before/after values, unit `none`, dimension `dimensionless`, and local
  session audit/professional-boundary metadata.
- Manager values are bounded to the current preview model values plus explicit
  `TBD`; this tranche does not introduce new load categories, new load-case
  records, or code-specific defaults.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_load_case_metadata_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-95. Validation passed:
  operation-applier format check, Rust operation-applier tests 23/23,
  src-tauri Rust tests 26/26, desktop Vitest 36/36, desktop build, desktop
  Playwright smoke 1/1, and in-app browser smoke over `load:L-100`.
- A4 residuals remain load-case creation, arbitrary primitive-load creation,
  imposed-displacement authoring breadth, full combination editing/algebra
  authoring, Phase B unit picker/display retirement, and packaged-Tauri
  saved-project smoke over edited load data.

## 2026-06-11 - TP-APP-R2-LOADCREATE-001 empty load-case creation editor

- WORKING_ITEMS app-integration tranche added explicit empty load-case shell
  creation to the desktop Load Cases manager for the invented preview model.
  No `core/loads/primitive_loads` source behavior changed.
- The editor queues a structured `create_load_case` intent with
  `field_path=load_cases`, `before=not_present`, unit `none`, dimension
  `dimensionless`, explicit id/label/kind/status/provenance, and
  `primitive_loads=0`.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_load_case_creation_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-97. Validation passed:
  operation-applier format check, Rust operation-applier tests 25/25,
  src-tauri Rust tests 26/26, desktop Vitest 39/39, desktop build, desktop
  Playwright smoke 1/1, and in-app browser smoke over `load:L-300`.
- A4 residuals remain arbitrary primitive-load creation,
  imposed-displacement authoring breadth, combination basis editing,
  combination term creation/deletion, broader algebra authoring, Phase B unit
  picker/display retirement, and packaged-Tauri saved-project smoke over
  edited load data.

## 2026-06-11 - TP-APP-R2-PRIMCREATE-001 concentrated primitive-load creation editor

- WORKING_ITEMS app-integration tranche added explicit concentrated nodal-force
  primitive-load creation to the desktop Load Cases manager for existing load
  cases. No `core/loads/primitive_loads` source behavior changed.
- The editor queues a structured `create_primitive_load` intent with
  `field_path=primitive_loads`, `before=not_present`, project force unit,
  dimension `force`, existing node target, global direction, finite magnitude,
  and provenance.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_primitive_load_creation_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-98. Validation passed:
  operation-applier format check, Rust operation-applier tests 26/26,
  src-tauri Rust tests 26/26, desktop Vitest 41/41, desktop build, desktop
  Playwright smoke 1/1, and in-app browser smoke over `load:L-100-F300`.
- A4 residuals remain distributed primitive-load creation, concentrated
  moments, pressure/temperature primitive creation, imposed-displacement
  authoring breadth, combination basis editing, combination term
  creation/deletion, broader algebra authoring, Phase B unit picker/display
  retirement, and packaged-Tauri saved-project smoke over edited load data.

## 2026-06-11 - TP-APP-R2-DISTLOAD-001 distributed primitive-load creation editor

- WORKING_ITEMS app-integration tranche added explicit distributed
  element-force primitive-load creation to the desktop Load Cases manager for
  existing load cases. No `core/loads/primitive_loads` source behavior
  changed.
- The editor queues a structured `create_primitive_load` intent with
  `field_path=primitive_loads`, `before=not_present`, category
  `distributed_force`, existing pipe target, global direction, finite
  magnitude, project force/length unit `N/m`, dimension `force_per_length`,
  and provenance.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_distributed_load_creation_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-99. Validation passed:
  operation-applier format check, Rust operation-applier tests 27/27,
  src-tauri Rust tests 26/26, desktop Vitest 43/43, desktop build, desktop
  Playwright smoke 1/1, and in-app browser smoke over `load:L-100-D300`.
- A4 residuals remain concentrated moments, pressure/temperature primitive
  creation, imposed-displacement authoring breadth, combination basis editing,
  combination term creation/deletion, broader algebra authoring, Phase B unit
  picker/display retirement, and packaged-Tauri saved-project smoke over
  edited load data.

## 2026-06-11 - TP-APP-R2-MOMENTCREATE-001 concentrated moment primitive-load creation editor

- WORKING_ITEMS app-integration tranche added explicit concentrated nodal
  moment primitive-load creation to the desktop Load Cases manager for
  existing load cases. No `core/loads/primitive_loads` source behavior
  changed.
- The editor queues a structured `create_primitive_load` intent with
  `field_path=primitive_loads`, `before=not_present`, category
  `concentrated_moment`, existing node target, rotational direction, finite
  magnitude, project force*length unit `N*m`, dimension `moment`, and
  provenance.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_moment_load_creation_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-100. Validation passed:
  operation-applier format check, Rust operation-applier tests 28/28,
  src-tauri Rust tests 26/26, desktop Vitest 45/45, desktop build, desktop
  Playwright smoke 1/1, and in-app browser smoke over `load:L-100-M300`.
- A4 residuals remain pressure/temperature primitive creation,
  imposed-displacement authoring breadth, combination basis editing,
  combination term creation/deletion, broader algebra authoring, Phase B unit
  picker/display retirement, and packaged-Tauri saved-project smoke over
  edited load data.

## 2026-06-11 - TP-APP-R2-PRESSTEMP-001 pressure and thermal primitive-load creation editor

- WORKING_ITEMS app-integration tranche added explicit pressure and thermal
  primitive-load creation to the desktop Load Cases manager for existing load
  cases.
- The editor queues structured `create_primitive_load` intents with
  `field_path=primitive_loads`, `before=not_present`, categories `pressure`
  and `thermal`, existing pipe targets, global directions, finite magnitudes,
  project units `Pa` and `degC`, dimensions `pressure` and
  `temperature_interval`, and provenance. The invented preview model now
  carries explicit `project.units.pressure = "Pa"`.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_pressure_thermal_load_creation_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-101. Validation passed:
  operation-applier format check, Rust operation-applier tests 29/29,
  src-tauri Rust tests 26/26, desktop Vitest 48/48, desktop build, desktop
  Playwright smoke 1/1, and in-app browser pressure/thermal smoke.
- A4 residuals remain imposed-displacement authoring breadth, combination
  basis editing, combination term creation/deletion, broader algebra
  authoring, Phase B unit picker/display retirement, and packaged-Tauri
  saved-project smoke over edited load data.

## 2026-06-11 - TP-APP-R2-IMPOSED-001 imposed-displacement primitive-load creation editor

- WORKING_ITEMS app-integration tranche added explicit support-target
  imposed-displacement primitive-load creation to the desktop Load Cases
  manager for existing load cases.
- The editor queues a structured `create_primitive_load` intent with
  `field_path=primitive_loads`, `before=not_present`, category
  `imposed_displacement`, existing support target, matching support DOF,
  finite magnitude, project length unit `m` for translational DOFs, project
  angle unit `rad` for rotational DOFs, dimension `displacement` or
  `rotation`, and provenance. The invented preview model now carries explicit
  `project.units.angle = "rad"`.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_imposed_displacement_load_creation_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-102. Validation passed:
  operation-applier format check, Rust operation-applier tests 30/30,
  src-tauri Rust tests 26/26, desktop Vitest 50/50, desktop build, desktop
  Playwright smoke 1/1, and in-app browser imposed-displacement smoke.
- A4 residuals remain combination basis editing, combination term
  creation/deletion, broader algebra authoring, Phase B unit picker/display
  retirement, and packaged-Tauri saved-project smoke over edited load data.

## 2026-06-17 - Lifecycle Housekeeping

- Housekeeping lifecycle reset: `_STATUS.md` current state set to `IN_PROGRESS` to reflect current code development in progress. This does not change review, issuance, release readiness, professional approval, certification, sealing, authentication, or code-compliance status.

## 2026-06-17 - TP-UNITS-BTAIL-LOADCASEUNITVALID-001 primitive-load unit validation evidence

- Primary role for bounded B-tail tranche: the desktop Load Cases manager now
  records unit-dimension validation status on unit-bearing primitive-load
  create and magnitude-edit operation intents.
- Browser preview records
  `model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview`
  because the DEC-018 catalog command is desktop-only; desktop/Tauri catalog
  routes can record `dec018_catalog_dimension_match` or explicit
  mismatch/unreviewed statuses.
- Validation passed: focused App Vitest 55/55, focused R2 Playwright smoke
  2/2, full desktop Vitest 397/397, and desktop production build with the
  existing Vite large-chunk warning.
- Boundary preserved: no primitive-load engine behavior, solver behavior,
  operation application semantics, unit conversion API, protected standards
  content, private data, lifecycle transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## 2026-06-17 - TP-UNITS-BTAIL-PRIMDELETEUNITVALID-001 primitive-load delete unit validation evidence

- Primary role for bounded B-tail tranche: the desktop Load Cases manager now
  records unit-dimension validation status on unit-bearing primitive-load
  delete operation intents.
- The delete preview preserves the selected primitive's existing unit and
  dimension, and reports the same validation status family used by
  primitive-load creation and magnitude edits instead of
  `unit_validation=not_run`.
- Validation passed: focused App Vitest 55/55, full desktop Vitest 18/18 files
  and 398/398 tests, focused R2/R3 Playwright smoke file 14/14, desktop
  production build with the existing Vite large-chunk warning, and
  `git diff --check`. DEC-025 sweep evidence is recorded in closeout artifacts.
- Boundary preserved: no primitive-load engine behavior, solver behavior,
  operation application semantics, unit conversion API, protected standards
  content, private data, lifecycle transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim changed.
## 2026-06-18 - TP-UNITS-BTAIL-LOADMANAGERLINTUNITS-001 supporting report-lint inventory evidence

- Supporting role for DEL-05-01: Report Content Lint now inventories the
  existing Load Case Manager primitive-load unit-validation surface through
  `load-manager-unit-validation-surface`.
- The lint packet now includes
  `apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx`; public
  unit-policy targets increase from 38 to 39 while conversion-witness targets
  remain two.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-LOADMANAGERLINTUNITS-001.md`;
  `apps/desktop/SMOKE.md` TP-MAC-256; completion log entry; primary
  DEL-08-05 run record and supporting DEL-07-02/DEL-05-02/DEL-16-02/DEL-02-02
  run records.
- Validation passed: focused App Vitest workspace-render; focused load/unit
  App tests 26/26; focused R2 Playwright smoke 2/2; full desktop Vitest
  399/399; single-worker R2/R3 Playwright smoke 18/18; `git diff --check`;
  and desktop production build with the existing Vite large-chunk warning.
- Boundary preserved: no primitive-load schema or behavior, load algebra,
  operation validation/application, unit conversion behavior, private data,
  lifecycle transition, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.

## 2026-06-22 - TP-R4-D4-EJTHRUST-001 supporting generated pressure-thrust load evidence

- Supporting role for DEL-03-06 D4: the product-physics load side now
  generates expansion-joint pressure-thrust result rows from explicit
  pressure load primitives and user-entered effective area when an
  expansion-joint component maps to the pressured pipe.
- The existing pipe-internal-area pressure-thrust behavior remains the
  fallback for ordinary pipe pressure loads without an eligible expansion
  joint component mapping.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-22_TP-R4-D4-EJTHRUST-001.md`;
  primary DEL-03-06 run record and supporting DEL-08-03 run record.
- Boundary preserved: no protected standards content, hidden default
  pressure-thrust coefficient, private project data, lifecycle transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.
