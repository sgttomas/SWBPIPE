# MEMORY - DEL-16-02 Operation validation and diff preview

## 2026-06-18 - TP-UNITS-BTAIL-DIFFPREVIEWLINTUNITS-001

- Supporting role for DEL-08-05 report-lint inventory: the desktop Report
  Content Lint inventory now includes the Operation Diff Preview unit-witness
  surface.
- The lint inventory records `operation-diff-unit-witnesses` and reports
  `unit_targets=21`, `conversion_witness_targets=2`, and
  `lint_conversion=false`. The diff-preview packet itself continues to
  preserve declared operation change units and dimensions without conversion.
- Validation passed: focused App Vitest workspace-render and queued-intent
  selected tests, focused Chromium desktop R2 smoke, full desktop Vitest
  399/399, desktop production build with the existing Vite large-chunk
  warning, and single-worker R2/R3 Playwright smoke 18/18.
- Boundary preserved: no operation validation, diff preview, operation
  application, accepted-state mutation, operation audit persistence,
  unit-conversion API, DEC-018 catalog constant, schema dimension enum,
  protected standards content, private data, lifecycle transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## 2026-06-17 - TP-UNITS-BTAIL-OPAPPLYUNITS-001

- The desktop Apply Operations panel now exposes a visible unit-policy summary
  for queued intents, operation outcomes, and applied receipts.
- The summary records queued unit-bearing/dimensionless counts, outcome
  unit-validation statuses, applied receipt count, `receipt_units=not_serialized`,
  and `conversion=false`; it does not change operation validation, apply
  semantics, receipt schema, or solver behavior.
- Validation passed: focused App Vitest 1/1 selected test; focused Chromium
  desktop Playwright smoke 1/1; `git diff --check`; full desktop Vitest
  399/399; full R2/R3 Playwright smoke 18/18; desktop production build with
  the existing Vite large-chunk warning.
- Boundary preserved: no operation-applier semantics, backend validation,
  receipt schema, unit-conversion API, DEC-018 catalog constant, schema
  dimension enum, protected standards content, private data, lifecycle
  transition, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim changed.

## 2026-05-06 Implementation Notes

- Implemented `core/model_operations/validation_preview/engine.py` as a
  structured operation validation and deterministic diff-preview engine.
- Added checks for required upstream operation-envelope fields, supported
  operation/change taxonomy, unit metadata, unresolved target references,
  direct accepted-model mutation attempts, and blocking constraint diagnostics.
- Added preview output that records before/after rows while preserving
  `application_status: not_applied`.
- Added `tests/test_operation_validation_preview.py` for focused validation,
  preview determinism, no-mutation, and professional-boundary checks.

## Boundary Decisions

- The engine produces reviewable previews only; it does not mutate accepted
  model state or perform autonomous engineering acceptance.
- GUI runtime behavior, user acceptance workflows, operation audit trail, and
  external prover/commercial-tool integrations remain downstream or out of
  scope for this deliverable.

## 2026-05-11 TP-RECON-01 Reconciliation

- Reconciled DEL-16-02 history from the TP-RECON-01 dispatch row and archived
  DEV-001 revision 0.5 Tranche H evidence only.
- Evidence basis: commit `c08b0a2` (`core: implement tranche h contracts`) is
  recorded for DEL-16-02 as `COMMITTED` on 2026-05-06; the lifecycle snapshot
  preserves `CHECKING` with committed implementation evidence.
- Implemented slice recorded for this deliverable: operation validation and
  deterministic diff-preview module, focused validation/diff-preview tests,
  and deliverable memory.
- Handoff verification recorded `python3 tests/test_operation_validation_preview.py`,
  adjacent contract/schema checks, `git diff --check`, and focused protected
  data/private data/prohibited-claim scans over Tranche H surfaces.
- Boundaries remain unchanged: preview-only review output, no hidden accepted
  model mutation, and GUI runtime, user acceptance/audit trail, and authority
  or compliance conclusions remain downstream or out of scope.

## 2026-05-16 - DEV-001 downstream PKG-02 audit memory addendum

Durable context preserved after reconciliation review:
- DEV-001 package-worker audit reviewed this deliverable for downstream compatibility with the accepted PKG-02 foundation contracts.
- Local audit artifacts are `execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_REVIEW.md` and `execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/Review_Findings.csv`.
- Package audit summary is `execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/_audit/PKG02_DOWNSTREAM_REVIEW_2026-05-16.md`; package run record is `execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/_run_records/TASK_RUN_2026-05-16_PKG16_PKG02_DOWNSTREAM_AUDIT.md`.
- This was audit evidence only. It did not change lifecycle state, authorize release, or make a professional, certification, sealing, approval, or code-compliance claim.
- The May 16 package-worker TASK run record did not fully preserve canonical per-deliverable TASK documentation context; this addendum preserves the durable deliverable-local pointer without modifying the completed run record.

## 2026-05-16 - DEV-001 PKG-02 grounded finding-resolution memory addendum

Durable context preserved after PKG-02 grounded finding resolution:
- Stage 2 technical resolution used the accepted PKG-02 contract as the governing source for this deliverable's downstream compatibility evidence.
- Original audit finding count for this deliverable: 3 (BLOCKER=1, WARNING=2). Current technical status count in the resolution matrix: TECHNICALLY_ADDRESSED_PENDING_HUMAN=3.
- Resolution evidence is indexed in `execution/_Reconciliation/Reviews/DEV001_FINDING_RESOLUTION_PKG02_GROUNDED_2026-05-16/RESOLUTION_MATRIX.csv`; validation evidence is summarized in `execution/_Reconciliation/Reviews/DEV001_FINDING_RESOLUTION_PKG02_GROUNDED_2026-05-16/VALIDATION_SUMMARY.md`.
- Local `Review_Findings.csv` entries remain subject to the human disposition gate. `HumanDisposition` stays `TBD` until review, and `Status` must not be changed to `RESOLVED` automatically.
- No lifecycle promotion, release claim, or professional/code-compliance claim is implied by the technical closeout.

## 2026-06-04 - TP-AUTHORITY-REFRESH-0_7-DAG006

- WORKING_ITEMS/TASK workforce current-authority refresh applied to active deliverable-local surfaces for `DEL-16-02`.
- Current authority basis is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7` plus approved `execution/_DAG/DAG-006/` active graph authority.
- Historical run records, historical DAG row IDs, review dispositions, lifecycle `_STATUS.md`, aggregate DAG artifacts, candidate edges, repo-level governance files, schemas, code, and tools were intentionally not changed by this refresh.
- Preserved historical references remain evidence of earlier work, not current authority claims.

## 2026-06-06 - TASK verification addendum

- Verified the current DEL-16-02 validation-preview implementation against the bounded brief for canonical JSON Schema validation, canonical dimension vocabulary blocking, stale current-hash blocking, non-physical accepted-model-role blocking, and preview non-mutation behavior.
- Evidence run: `python3 -m pytest tests/test_operation_validation_preview.py` passed with 8 tests.
- Narrow scan confirmed `Draft202012Validator` use, `CANONICAL_DIMENSIONS`, stale hash diagnostics, model-role diagnostics, `accepted_model_state_unchanged`, and `application_status: not_applied` in the implementation/test evidence.
- No concrete code or test gap was found; `core/model_operations/validation_preview/engine.py` and `tests/test_operation_validation_preview.py` were not edited.
- Review findings remain technically addressed pending human disposition; this addendum does not change lifecycle state, finding statuses, human dispositions, release readiness, or professional/code-compliance claims.

## 2026-06-07 - TASK evidence-alignment addendum

- Aligned `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` with current implementation evidence from `core/model_operations/validation_preview/engine.py`, `schemas/model_operation.schema.json`, `fixtures/model_operations/invented_operation_set_valid.json`, `fixtures/model_operations/invented_accepted_model_state.json`, `tests/test_operation_validation_preview.py`, and `tests/test_model_operation_schema.py`.
- Current established facts: validation uses `Draft202012Validator` against the DEL-16-01 JSON Schema; operation preview checks required envelope fields, schema validity, accepted model role/hash, operation model-basis hash, operation required current hashes, direct-mutation signals, canonical dimension vocabulary, unresolved targets, and injected blocking constraint diagnostics; preview output remains deterministic for current fixtures and reports `application_status: not_applied`.
- Residual TBDs preserved: final DEL-13-03 constraint-engine API, final DEL-14-03/DEL-14-05 diff payload/tolerance contract beyond current fixture-backed rows, persistence/application behavior outside this slice, final cross-package diagnostic/result-envelope mapping, and human dispositions for existing review findings.
- Boundary controls preserved: no code, schema, fixture, test, dependency, status, review, or lifecycle files were edited; this run does not claim release readiness, human approval, professional approval, certification, sealing, authentication, or code compliance.

## 2026-06-07 - Human disposition of PKG-02 findings

- Human project authority accepted the recommendation to resolve each DEL-16-02 PKG-02 finding.
- `Review_Findings.csv` now records `HumanDisposition=ACCEPT_AS_IS` and `Status=RESOLVED` for `PKG16-DEL1602-PKG02-001`, `PKG16-DEL1602-PKG02-002`, and `PKG16-DEL1602-PKG02-003`.
- Supporting ruling packet: `execution/_Reconciliation/Reviews/REV_PKG-16_2026-06-06_1648/HUMAN_DISPOSITION_RULING_PACKET_DEL-16-02-PKG02-FINDINGS.md`.
- No lifecycle transition, release assertion, legal conclusion, professional approval, certification, sealing, authentication, approval, or code-compliance claim is made by this disposition.

## 2026-06-10 - TP-APP-R2-EDITLOOP-001 apply-operation command path (A1)

- The desktop app now has a runtime apply seam for this deliverable's
  contract: new Rust crate `core/model_operations/operation_applier`
  (validate → diff-preview → apply for structured editor intents) behind
  new Tauri commands `validate_model_operation` / `apply_model_operation`,
  with an Apply Operations panel and a browser-fixture local engine
  honestly labeled by `application_route`.
- Boundary semantics preserved from this deliverable: input model never
  mutated; blocked operations are findings (stale before-value, unit
  mismatch without conversion, unknown dimension, unresolved refs, geometry
  inputs incomplete, deferred fields); `application_status` vocabulary now
  includes `applied_to_session_model` only for the new returned document;
  acceptance receipts record `user_initiated_apply_in_local_session` and
  never professional approval (DEL-16-03 alignment).
- Known cross-surface notes: the Python engine's `CANONICAL_DIMENSIONS`
  omitted `force_per_length` (accepted PKG-02 set has 30 ids) — surfaced in
  the D-01 decision packet and reconciled in-tree by the human project
  authority in the same session (three Python sets + a schema parity test
  in `tests/test_operation_validation_preview.py`); cross-language
  canonical-JSON float formatting prevents UI↔backend hash equality claims
  (echo-only binding status) — D-08-adjacent follow-up.
- Evidence: run record
  `_run_records/WORKING_ITEMS_RUN_2026-06-10_apply_operation_command_path.md`;
  smoke `apps/desktop/SMOKE.md` TP-MAC-82; suites 19/19 crate, 14/14
  src-tauri, 21/21 Vitest, 342/342 pytest, build green.
- No lifecycle change: `_STATUS.md` remains `CHECKING`; this is
  app-integration absorption of the deliverable's design authority, not
  issuance.

## 2026-06-10 - TP-APP-R2-INLINEVALID-001 property-inspector inline validation

- The desktop property inspector now exposes this deliverable's validate-only
  operation seam inline for draft editor intents before queue/apply. The UI
  displays application status, schema/unit/before-state states, diff rows,
  diagnostics, and a no-mutation/professional-boundary note.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-10_property_inspector_inline_validation.md`
  and `apps/desktop/SMOKE.md` TP-MAC-85. Validation passed:
  `npm test --workspace apps/desktop` (26/26) and
  `npm run build --workspace apps/desktop`; timestamp-filtered live browser
  smoke on `http://127.0.0.1:5174/` confirmed validate-only inline feedback
  for `material:invented-carbon-steel` with no new console warnings/errors
  after the final reload.
- Boundary semantics remain unchanged: inline validation does not apply or
  persist operations, mutate accepted model state, perform unit conversion,
  supply code-specific defaults, or make professional/release/code-compliance
  claims.
- No lifecycle state, finding status, dependency authority, release readiness,
  professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards data, private project data,
  network path, or telemetry path changed.

## 2026-06-10 - TP-APP-R2-CREATENODE-001 explicit node create operation

- WORKING_ITEMS app-integration tranche extended the browser local operation
  engine and Rust `operation_applier` to validate, diff, and apply explicit
  `create_node` intents.
- Accepted create-node intents must target `Node` with `field_path=nodes`,
  `before=not_present`, matching project length unit, `dimension=length`, no
  duplicate node id, and a JSON payload containing matching id, non-empty
  label/provenance, and finite numeric x/y/z coordinates.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-10_explicit_node_create_operation.md`
  and `apps/desktop/SMOKE.md` TP-MAC-86. Validation passed:
  Rust operation-applier tests 20/20, desktop Vitest 28/28, desktop build,
  and targeted diff whitespace check.
- Boundary semantics remain unchanged: no direct input-model mutation, no
  silent unit conversion, no coordinate inference, no durable persistence, no
  protected/private data, and no professional/release/code-compliance claim.

## 2026-06-11 - TP-APP-R2-CONNECTPIPE-001 explicit straight-pipe connectivity

- WORKING_ITEMS app-integration tranche extended the browser local operation
  engine and Rust `operation_applier` to validate, diff, and apply explicit
  `connect_pipe_run` intents.
- Accepted connect-pipe intents must target `Element` with
  `field_path=pipe_segments`, `before=not_present`, matching project length
  unit, `dimension=length`, no duplicate pipe id, existing endpoint nodes and
  material, positive OD/wall quantities, non-empty provenance, and a non-zero
  `y_reference` vector.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_explicit_straight_pipe_connectivity.md`
  and `apps/desktop/SMOKE.md` TP-MAC-92. Validation passed:
  Rust operation-applier tests 22/22, desktop Vitest 33/33, desktop build,
  and desktop Playwright smoke 1/1.
- Boundary semantics remain unchanged: no direct input-model mutation, no
  silent unit conversion, no geometry inference, no durable persistence, no
  protected/private data, and no professional/release/code-compliance claim.

## 2026-06-11 - TP-APP-R2-LOADMETA-001 load-case metadata editor

- WORKING_ITEMS app-integration tranche extended the browser local operation
  engine and Rust `operation_applier` to validate, diff, and apply explicit
  `Load.status` and `Load.kind` text metadata updates.
- `apps/desktop/src/services/operationService.ts` and
  `core/model_operations/operation_applier/src/lib.rs` now treat those fields
  as supported text fields. Existing `Combination.basis` and
  `Combination.terms` remain explicitly deferred for the later A4 combination
  editor scope.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_load_case_metadata_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-95. Validation passed:
  operation-applier format check, Rust operation-applier tests 23/23,
  src-tauri Rust tests 26/26, desktop Vitest 36/36, desktop build, desktop
  Playwright smoke 1/1, and targeted browser smoke.
- Boundary semantics remain unchanged: no direct input-model mutation, no
  silent unit conversion, no hidden load defaults, no durable persistence, no
  protected/private data, and no professional/release/code-compliance claim.

## 2026-06-11 - TP-APP-R2-COMBFACTOR-001 combination term-factor editor

- WORKING_ITEMS app-integration tranche extended the browser local operation
  engine and Rust `operation_applier` to validate, diff, and apply dynamic
  `Combination.terms.N.factor` edits as dimensionless numeric fields.
- Whole `Combination.terms` replacement and `Combination.basis` remain
  explicitly deferred, preserving the A4 residual boundary for term
  creation/deletion, basis editing, code/rule combinations, and broader
  algebra authoring.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_combination_term_factor_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-96. Validation passed:
  operation-applier format check, Rust operation-applier tests 24/24,
  src-tauri Rust tests 26/26, desktop Vitest 37/37, desktop build, desktop
  Playwright smoke 1/1, and targeted browser smoke.
- Boundary semantics remain unchanged: no direct input-model mutation, no
  silent unit conversion, no hidden load defaults, no durable persistence, no
  protected/private data, and no professional/release/code-compliance claim.

## 2026-06-11 - TP-APP-R2-LOADCREATE-001 empty load-case creation editor

- WORKING_ITEMS app-integration tranche extended the browser local operation
  engine and Rust `operation_applier` to validate, diff, and apply explicit
  empty `create_load_case` operations.
- Accepted intents must target `Load` with `field_path=load_cases`, use
  `before=not_present`, unit `none`, dimension `dimensionless`, a
  non-duplicate id, matching JSON id, non-empty label/kind/status/provenance,
  and absent or empty `primitive_loads`.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_load_case_creation_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-97. Validation passed:
  operation-applier format check, Rust operation-applier tests 25/25,
  src-tauri Rust tests 26/26, desktop Vitest 39/39, desktop build, desktop
  Playwright smoke 1/1, and targeted browser smoke.
- Boundary semantics remain unchanged: no direct input-model mutation, no
  silent unit conversion, no hidden primitive-load defaults, no durable
  persistence, no protected/private data, and no professional, release, or
  code-compliance claim.

## 2026-06-11 - TP-APP-R2-PRIMCREATE-001 concentrated primitive-load creation editor

- WORKING_ITEMS app-integration tranche extended the browser local operation
  engine and Rust `operation_applier` to validate, diff, and apply explicit
  concentrated-force `create_primitive_load` operations.
- Accepted intents must target an existing `Load` with
  `field_path=primitive_loads`, use `before=not_present`, project force unit,
  dimension `force`, a unique primitive id, category `concentrated_force`,
  existing node target, global direction, finite magnitude, and provenance.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_primitive_load_creation_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-98. Validation passed:
  operation-applier format check, Rust operation-applier tests 26/26,
  src-tauri Rust tests 26/26, desktop Vitest 41/41, desktop build, desktop
  Playwright smoke 1/1, and targeted browser smoke.
- Boundary semantics remain unchanged: no direct input-model mutation, no
  silent unit conversion, no hidden target/default load inference, no durable
  persistence, no protected/private data, and no professional, release, or
  code-compliance claim.

## 2026-06-11 - TP-APP-R2-MOMENTCREATE-001 concentrated moment primitive-load creation editor

- WORKING_ITEMS app-integration tranche extended the browser local operation
  engine and Rust `operation_applier` to validate, diff, and apply explicit
  concentrated-moment `create_primitive_load` operations.
- Accepted intents must target an existing `Load` with
  `field_path=primitive_loads`, use `before=not_present`, project force and
  length unit metadata, unit `N*m`, dimension `moment`, a unique primitive id,
  category `concentrated_moment`, existing node target, rotational direction,
  finite magnitude, and provenance.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_moment_load_creation_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-100. Validation passed:
  operation-applier format check, Rust operation-applier tests 28/28,
  src-tauri Rust tests 26/26, desktop Vitest 45/45, desktop build, desktop
  Playwright smoke 1/1, and targeted browser smoke.
- Boundary semantics remain unchanged: no direct input-model mutation, no
  silent unit conversion, no hidden target/default load inference, no durable
  persistence, no protected/private data, and no professional, release, or
  code-compliance claim.

## 2026-06-11 - TP-APP-R2-DISTLOAD-001 distributed primitive-load creation editor

- WORKING_ITEMS app-integration tranche extended the browser local operation
  engine and Rust `operation_applier` to validate, diff, and apply explicit
  distributed-force `create_primitive_load` operations.
- Accepted intents must target an existing `Load` with
  `field_path=primitive_loads`, use `before=not_present`, project force and
  length unit metadata, unit `N/m`, dimension `force_per_length`, a unique
  primitive id, category `distributed_force`, existing pipe target, global
  direction, finite magnitude, and provenance.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_distributed_load_creation_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-99. Validation passed:
  operation-applier format check, Rust operation-applier tests 27/27,
  src-tauri Rust tests 26/26, desktop Vitest 43/43, desktop build, desktop
  Playwright smoke 1/1, and targeted browser smoke.
- Boundary semantics remain unchanged: no direct input-model mutation, no
  silent unit conversion, no hidden target/default load inference, no durable
  persistence, no protected/private data, and no professional, release, or
  code-compliance claim.

## 2026-06-11 - TP-APP-R2-PRESSTEMP-001 pressure and thermal primitive-load creation editor

- WORKING_ITEMS app-integration tranche extended the browser local operation
  engine and Rust `operation_applier` to validate, diff, and apply explicit
  pressure and thermal `create_primitive_load` operations.
- Accepted pressure intents must target an existing `Load` with
  `field_path=primitive_loads`, use `before=not_present`, project pressure
  unit metadata `Pa`, dimension `pressure`, a unique primitive id, category
  `pressure`, existing pipe target, global direction, finite magnitude, and
  provenance. Thermal uses project temperature metadata `degC` and dimension
  `temperature_interval`.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_pressure_thermal_load_creation_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-101. Validation passed:
  operation-applier format check, Rust operation-applier tests 29/29,
  src-tauri Rust tests 26/26, desktop Vitest 48/48, desktop build, desktop
  Playwright smoke 1/1, and targeted browser smoke.
- Boundary semantics remain unchanged: no direct input-model mutation, no
  silent unit conversion, no gauge/absolute pressure default, no hidden
  target/default load inference, no durable persistence, no protected/private
  data, and no professional, release, or code-compliance claim.

## 2026-06-11 - TP-APP-R2-IMPOSED-001 imposed-displacement primitive-load creation editor

- WORKING_ITEMS app-integration tranche extended the browser local operation
  engine and Rust `operation_applier` to validate, diff, and apply explicit
  imposed-displacement `create_primitive_load` operations.
- Accepted intents must target an existing `Load` with
  `field_path=primitive_loads`, use `before=not_present`, a unique primitive
  id, category `imposed_displacement`, existing support target, matching
  target DOF, finite magnitude, and provenance. Translational DOFs require
  project length unit metadata and dimension `displacement`; rotational DOFs
  require project angle unit metadata and dimension `rotation`.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_imposed_displacement_load_creation_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-102. Validation passed:
  operation-applier format check, Rust operation-applier tests 30/30,
  src-tauri Rust tests 26/26, desktop Vitest 50/50, desktop build, desktop
  Playwright smoke 1/1, and targeted browser smoke.
- Boundary semantics remain unchanged: no direct input-model mutation, no
  silent unit conversion, no support coordinate policy, no solver boundary
  behavior, no durable persistence, no protected/private data, and no
  professional, release, or code-compliance claim.

## 2026-06-11 - TP-APP-R2-COMBBASIS-001 combination basis editor

- WORKING_ITEMS app-integration tranche extended the browser local operation
  engine and Rust `operation_applier` to validate, diff, and apply explicit
  `Combination.basis` text edits.
- Accepted intents must target an existing `Combination` with
  `field_path=basis`, use `change_kind=update_load`, carry the current
  before-value, non-empty after-value, unit `none`, and dimension
  `dimensionless`. Whole `Combination.terms` replacement remains deferred.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_combination_basis_editor.md` and
  `apps/desktop/SMOKE.md` TP-MAC-103. Validation passed:
  operation-applier format check, Rust operation-applier tests 31/31,
  src-tauri Rust tests 26/26, desktop Vitest 52/52, desktop build, desktop
  Playwright smoke 1/1, and targeted browser smoke.
- Boundary semantics remain unchanged: no direct input-model mutation, no
  silent unit conversion, no whole-term replacement, no durable persistence,
  no protected/private data, and no professional, release, or code-compliance
  claim.

## 2026-06-12 - TP-APP-R2-CREATESUPPORT-001 support creation authoring

- WORKING_ITEMS app-integration tranche extended the Rust/wasm
  `operation_applier` with `create_support`.
- Accepted intents must target object type `Support` with
  `field_path=supports`, `operation_kind=create`, `before=not_present`, unit
  `none`, dimension `dimensionless`, and an explicit JSON payload carrying
  matching id, label, existing node reference, restraint tokens, and
  provenance.
- Duplicate support ids, missing node references, invalid support payloads,
  wrong unit/dimension metadata, and missing supports collections are blocked.
  Restraint tokens use the existing `UX/UY/UZ/RX/RY/RZ` vocabulary.
- Contract corpus coverage now includes
  `fixtures/model_operations/contract_corpus/case_45_accept_create_support.json`;
  both Rust and browser wasm corpus runners require accepted
  `create_support` coverage.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-12_create_support_authoring.md`
  and `apps/desktop/SMOKE.md` TP-MAC-117. Validation passed:
  operation-applier cargo suites (36 unit + canonical hash + 45-case corpus),
  corpus bless/rerun, desktop operationContractCorpus 93/93, full desktop
  Vitest 177/177, desktop build, src-tauri Rust tests 29/29, Playwright smoke
  1/1, and live Chrome support-create smoke with zero console errors.
- Boundary semantics remain unchanged: no direct input-model mutation, no
  silent unit conversion, no hidden support defaults, no durable persistence,
  no protected/private data, and no professional, release, or code-compliance
  claim.

## 2026-06-12 - TP-APP-R2-CREATEMATERIAL-001 material creation authoring

- WORKING_ITEMS app-integration tranche extended the Rust/wasm
  `operation_applier` with `create_material`.
- Accepted intents must target object type `Material` with
  `field_path=materials`, `operation_kind=create`, `before=not_present`, the
  project pressure unit, dimension `stress`, and an explicit JSON payload
  carrying matching id, label, elastic modulus, shear modulus, optional
  thermal expansion coefficient, and provenance.
- Duplicate material ids, invalid or non-positive elastic/shear quantities,
  invalid thermal-expansion quantities, wrong unit/dimension metadata, and
  malformed materials collections are blocked. No material defaults or unit
  conversions are applied.
- Contract corpus coverage now includes
  `fixtures/model_operations/contract_corpus/case_46_accept_create_material.json`;
  both Rust and browser wasm corpus runners require accepted
  `create_material` coverage.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-12_create_material_authoring.md`
  and `apps/desktop/SMOKE.md` TP-MAC-118. Validation passed:
  operation-applier cargo suites (38 unit + canonical hash + 46-case corpus),
  corpus bless/rerun, desktop operationContractCorpus 95/95, full desktop
  Vitest 180/180, desktop build, src-tauri Rust tests 29/29, Playwright smoke
  1/1, and live Chrome material-create smoke with zero console errors.
- Boundary semantics remain unchanged: no direct input-model mutation, no
  silent unit conversion, no hidden material defaults, no durable persistence,
  no protected/private data, and no professional, release, or code-compliance
  claim.

## 2026-06-12 - TP-APP-R2-CREATESECTION-001 section creation authoring

- WORKING_ITEMS app-integration tranche extended the Rust/wasm
  `operation_applier` with `create_section`.
- Accepted intents must target object type `Section` with
  `field_path=sections`, `operation_kind=create`, `before=not_present`, the
  project length unit, dimension `length`, and an explicit JSON payload
  carrying matching id, name, `section_type=pipe`, outside diameter, wall
  thickness, and provenance.
- Duplicate section ids, invalid or non-positive OD/wall quantities, wall
  thickness at or beyond OD radius, wrong unit/dimension metadata, and
  malformed sections collections are blocked. No section defaults or unit
  conversions are applied.
- Contract corpus coverage now includes
  `fixtures/model_operations/contract_corpus/case_47_accept_create_section.json`;
  both Rust and browser wasm corpus runners require accepted
  `create_section` coverage.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-12_create_section_authoring.md`
  and `apps/desktop/SMOKE.md` TP-MAC-119. Validation passed:
  operation-applier cargo suites (40 unit + canonical hash + 47-case corpus),
  corpus bless/rerun, desktop operationContractCorpus 97/97, full desktop
  Vitest 183/183, desktop build, src-tauri Rust tests 29/29, Playwright smoke
  1/1, and live Chrome section-create smoke with zero console errors.
- Boundary semantics remain unchanged: no direct input-model mutation, no
  silent unit conversion, no hidden section defaults, no durable persistence,
  no protected/private data, and no professional, release, or code-compliance
  claim.

## 2026-06-11 - TP-APP-R2-COMBCREATE-001 combination creation editor

- WORKING_ITEMS app-integration tranche extended the browser local operation
  engine and Rust `operation_applier` to validate, diff, and apply explicit
  `create_combination` operations.
- Accepted intents must target a new `Combination` with
  `field_path=combinations`, use `operation_kind=create`, carry
  `before=not_present`, unit `none`, dimension `dimensionless`, and JSON
  payload `{ id, label, basis: "mechanics", terms, provenance }`.
- Duplicate ids, missing referenced load cases, empty terms, invalid
  payloads, non-finite factors, duplicate initial operands, non-mechanics
  basis values, and wrong unit/dimension metadata are blocked. Subtraction and
  range-expression authoring remain deferred.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_combination_creation_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-106. Validation passed:
  operation-applier format check, Rust operation-applier tests 34/34,
  src-tauri Rust tests 26/26, desktop Vitest 58/58, desktop build, desktop
  Playwright smoke 1/1, and targeted browser smoke.
- Boundary semantics remain unchanged: no direct input-model mutation, no
  silent unit conversion, no durable persistence, no protected/private data,
  and no professional, release, or code-compliance claim.

## 2026-06-11 - TP-APP-R2-COMBTERMDELETE-001 combination term deletion editor

- WORKING_ITEMS app-integration tranche extended the browser local operation
  engine and Rust `operation_applier` to validate, diff, and apply explicit
  `delete_combination_term` operations.
- Accepted intents must target an existing `Combination` with
  `field_path=terms.N`, use `operation_kind=delete`, carry a current
  before-value formatted as `<load_case> x <factor>`, use
  `after=not_present`, unit `none`, and dimension `dimensionless`.
- Stale before-values, out-of-range term indices, invalid existing term
  payloads, wrong unit/dimension metadata, and missing combination `terms`
  arrays are blocked. Whole `Combination.terms` replacement remains deferred.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_combination_term_deletion_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-105. Validation passed:
  operation-applier format check, Rust operation-applier tests 33/33,
  src-tauri Rust tests 26/26, desktop Vitest 56/56, desktop build, desktop
  Playwright smoke 1/1, and targeted browser smoke.
- Boundary semantics remain unchanged: no direct input-model mutation, no
  silent unit conversion, no whole-term replacement, no durable persistence,
  no protected/private data, and no professional, release, or code-compliance
  claim.

## 2026-06-11 - TP-APP-R2-COMBTERMCREATE-001 combination term creation editor

- WORKING_ITEMS app-integration tranche extended the browser local operation
  engine and Rust `operation_applier` to validate, diff, and apply explicit
  `create_combination_term` operations.
- Accepted intents must target an existing `Combination` with
  `field_path=terms`, use `operation_kind=create`, carry
  `before=not_present`, unit `none`, dimension `dimensionless`, and JSON
  payload `{ load_case: <existing load id>, factor: <finite number> }`.
- Missing referenced load cases, invalid payloads, wrong unit/dimension
  metadata, and missing combination `terms` arrays are blocked. Whole
  `Combination.terms` replacement remains deferred.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_combination_term_creation_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-104. Validation passed:
  operation-applier format check, Rust operation-applier tests 32/32,
  src-tauri Rust tests 26/26, desktop Vitest 54/54, desktop build, desktop
  Playwright smoke 1/1, and targeted browser smoke.
- Boundary semantics remain unchanged: no direct input-model mutation, no
  silent unit conversion, no whole-term replacement, no durable persistence,
  no protected/private data, and no professional, release, or code-compliance
  claim.

## 2026-06-12 - TP-APP-R2-DELSUPPORT-001 support deletion authoring

- WORKING_ITEMS app-integration tranche extended the Rust/wasm
  `operation_applier` to validate, diff, and apply explicit
  `delete_support` operations.
- Accepted intents must target an existing `Support` with
  `field_path=supports`, use `operation_kind=delete`, carry the current
  support label as `before`, carry `after=not_present`, use unit `none`, and
  use dimension `dimensionless`.
- Missing supports, stale before-values, wrong unit/dimension metadata, and
  support-targeted primitive-load references are blocked. Referenced support
  deletion emits `OP-SUPPORT-DELETE-REFERENCED`.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_support_authoring.md`
  and `apps/desktop/SMOKE.md` TP-MAC-120. Validation passed:
  operation-applier cargo suites (42 unit + canonical hash + 49-case corpus),
  corpus bless/rerun, desktop operationContractCorpus 101/101, full desktop
  Vitest 189/189, desktop build, src-tauri Rust tests 29/29, Playwright smoke
  1/1, and live Chrome support-delete smoke with zero console errors.
- Boundary semantics remain unchanged: no direct input-model mutation, no
  silent unit conversion, no hidden deletion cascade, no durable persistence,
  no protected/private data, and no professional, release, or code-compliance
  claim.

## 2026-06-12 - TP-UNITS-B2-INSPECTOREDITUNITS-001 sibling quantity unit edits

- WORKING_ITEMS app-integration tranche generalized the Rust/wasm
  `operation_applier` sibling-unit quantity edit contract.
- Sibling-unit quantity paths now accept `{ value, unit }` JSON payloads, check
  the selected unit against the declared DEC-018 dimension, and apply both the
  `.value` and sibling `.unit` writes atomically on the returned session model.
- Compatible non-stored units are accepted only when the sibling unit is
  written by the same validated edit; raw numeric value-only edits still
  require the stored unit.
- Native regression coverage includes material modulus edited to `MPa`, pipe
  outside diameter edited to `mm`, existing primitive-load magnitude edited to
  `lbf/ft`, and incompatible material-unit rejection.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-12_property_inspector_quantity_unit_edits.md`
  and `apps/desktop/SMOKE.md` TP-MAC-138. Validation passed:
  operation-applier cargo suites (54 unit + canonical hash + contract corpus),
  src-tauri Rust tests 32/32, focused desktop Vitest 165/165, full desktop
  Vitest 216/216, desktop build, and Playwright R2 smoke 2/2.
- Boundary semantics remain unchanged: no direct input-model mutation, no
  hidden fallback unit, no project-unit-system mutation, no durable persistence,
  no import/export conversion, no protected/private data, and no professional,
  release, or code-compliance claim.

## 2026-06-12 - TP-UNITS-B2-UNITPICKERS-001 create-material/section compatible units

- WORKING_ITEMS app-integration tranche connected create-section and
  create-material operation validation to `core/units` for DEC-018-compatible
  entered units.
- Section geometry accepts compatible length units and checks wall thickness
  against outside diameter after converting to the project length basis for
  comparison. Material creation accepts compatible stress units and
  thermal-expansion coefficient units. Applied records preserve entered units.
- Incompatible dimensions remain blocking operation diagnostics; no hidden
  unit fallback or protected/private lookup was added.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-12_property_inspector_unit_selectors.md`
  and `apps/desktop/SMOKE.md` TP-MAC-134. Validation passed:
  operation-applier cargo suites (51 unit tests plus canonical-hash and
  contract-corpus tests), wasm build, focused desktop Vitest 165/165, full
  desktop Vitest 216/216, and Playwright R2 smoke 2/2.

## 2026-06-12 - TP-UNITS-B2-LOADPICKERS-001 primitive-load unit selectors

- WORKING_ITEMS app-integration tranche extended the Rust/wasm
  `operation_applier` primitive-load creation path to validate entered load
  units by DEC-018 dimension compatibility rather than exact project-unit
  equality.
- Accepted create-primitive-load intents still require matching payload
  `magnitude.unit`, intent unit, dimension, target, direction, finite value,
  and provenance. Compatible entered units are preserved in the applied
  session model; incompatible dimensions remain blocked by
  `OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE`.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-12_primitive_load_unit_selectors.md`
  and `apps/desktop/SMOKE.md` TP-MAC-136. Validation passed:
  operation-applier cargo suites (52 unit tests plus canonical-hash and
  contract-corpus tests), src-tauri Rust tests 32/32, focused desktop Vitest
  165/165, full desktop Vitest 216/216, desktop build, and Playwright R2 smoke
  2/2.
- Boundary semantics remain unchanged: no direct input-model mutation, no
  hidden deletion cascade, no durable persistence, no protected/private data,
  and no professional, release, or code-compliance claim.

## 2026-06-12 - TP-UNITS-B2-PRIMEDITUNITS-001 primitive-load magnitude unit edits

- WORKING_ITEMS app-integration tranche extended the Rust/wasm
  `operation_applier` dynamic primitive-magnitude edit path to accept a
  `{ value, unit }` payload while preserving legacy numeric-string edits.
- Compatible entered units are checked against the declared primitive-load
  dimension and are applied by writing both `magnitude.value` and the sibling
  `magnitude.unit` field on the returned session model. Incompatible units
  remain blocked by `OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE`.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-12_primitive_load_magnitude_unit_edits.md`
  and `apps/desktop/SMOKE.md` TP-MAC-137. Validation passed:
  operation-applier cargo suites (53 unit tests plus canonical-hash and
  contract-corpus tests), src-tauri Rust tests 32/32, focused desktop Vitest
  165/165, full desktop Vitest 216/216, desktop build, and Playwright R2 smoke
  2/2.
- Boundary semantics remain unchanged: no direct input-model mutation, no
  hidden cascade, no durable persistence, no protected/private data, and no
  professional, release, or code-compliance claim.

## 2026-06-12 - TP-APP-R2-DELCOMBINATION-001 full-combination deletion authoring

- WORKING_ITEMS app-integration tranche extended the Rust/wasm
  `operation_applier` to validate, diff, and apply explicit
  `delete_combination` operations.
- Accepted intents must target an existing `Combination` with
  `field_path=combinations`, use `operation_kind=delete`, carry the current
  combination id/label/basis/terms display as `before`, carry
  `after=not_present`, use unit `none`, and use dimension `dimensionless`.
- Missing combinations, stale before-values, malformed combination records,
  and wrong unit/dimension metadata are blocked. There is no hidden cascade
  and no load-case or primitive-load deletion.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_combination_authoring.md`
  and `apps/desktop/SMOKE.md` TP-MAC-122. Validation passed:
  operation-applier cargo suites (44 unit + canonical hash + 51-case corpus),
  corpus bless/rerun, desktop operationContractCorpus 105/105, full desktop
  Vitest 195/195, desktop build, src-tauri Rust tests 29/29, Playwright smoke
  1/1 with delete-preview coverage, and live in-app browser full-combination
  delete smoke with zero console errors.
- Boundary semantics remain unchanged: no direct input-model mutation, no
  silent unit conversion, no hidden deletion cascade, no durable persistence,
  no protected/private data, and no professional, release, or code-compliance
  claim.

## 2026-06-12 - TP-APP-R2-DELPRIMLOAD-001 primitive-load deletion authoring

- WORKING_ITEMS app-integration tranche extended the Rust/wasm
  `operation_applier` to validate, diff, and apply explicit
  `delete_primitive_load` operations.
- Accepted intents must target an existing `Load` with
  `field_path=primitive_loads.N`, use `operation_kind=delete`, carry the
  current primitive id/category/target/direction/magnitude display as
  `before`, carry `after=not_present`, and use the selected primitive load's
  current unit and dimension metadata.
- Missing load cases, missing primitive arrays, out-of-range primitive
  indices, stale before-values, and wrong unit/dimension metadata are blocked.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_primitive_load_authoring.md`
  and `apps/desktop/SMOKE.md` TP-MAC-121. Validation passed:
  operation-applier cargo suites (43 unit + canonical hash + 50-case corpus),
  corpus bless/rerun, desktop operationContractCorpus 103/103, full desktop
  Vitest 192/192, desktop build, src-tauri Rust tests 29/29, Playwright smoke
  1/1, and live Chrome primitive-load delete smoke with zero console errors.
- Boundary semantics remain unchanged: no direct input-model mutation, no
  silent unit conversion, no hidden deletion cascade, no durable persistence,
  no protected/private data, and no professional, release, or code-compliance
  claim.

## 2026-06-12 - TP-APP-R2-DELLOADCASE-001 load-case deletion authoring

- WORKING_ITEMS app-integration tranche extended the Rust/wasm
  `operation_applier` to validate, diff, and apply explicit
  `delete_load_case` operations.
- Accepted intents must target an existing `Load` with
  `field_path=load_cases`, use `operation_kind=delete`, carry the current
  load-case id/label/kind/status/primitive-count display as `before`, carry
  `after=not_present`, use unit `none`, and use dimension `dimensionless`.
- Load cases still referenced by combination terms are blocked with
  `OP-LOAD-CASE-DELETE-REFERENCED`; there is no hidden combination-term,
  primitive-load, or solve-result cascade.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_load_case_authoring.md`
  and `apps/desktop/SMOKE.md` TP-MAC-123. Validation passed:
  operation-applier cargo suites (45 unit + canonical hash + 53-case corpus),
  corpus bless/rerun, desktop operationContractCorpus 109/109, full desktop
  Vitest 201/201, desktop build, src-tauri Rust tests 29/29, Playwright smoke
  1/1 with delete-preview coverage, and live in-app browser load-case
  create/delete smoke with zero console errors.
- Boundary semantics remain unchanged: no direct input-model mutation, no
  silent unit conversion, no hidden deletion cascade, no durable persistence,
  no protected/private data, and no professional, release, or code-compliance
  claim.
## 2026-06-15 - TP-UNITS-B2-NODECOORDUNITS-001: node coordinate unit payloads

- Operation applier now accepts project-unit quantity edits as either legacy
  scalar values or explicit `{value, unit}` payloads.
- Bare project-unit node coordinate fields normalize accepted DEC-018 length
  units back to the model document's stored `project.units.length` scalar basis;
  witnesses cover `ft` and `mm` conversion to meters and incompatible stress
  unit refusal.
- Shared operation contract corpus case
  `case_20_block_unit_mismatch_set_field.json` now uses a true incompatible
  length/stress unit mismatch; `ksi` is no longer a mismatch under the accepted
  DEC-018 stress catalog.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-15_TP-UNITS-B2-NODECOORDUNITS-001.md`;
  SMOKE TP-MAC-170; operation-applier cargo suite, desktop Vitest, desktop
  build, and focused Playwright R2 smoke passed.
- Boundary unchanged: no direct mutation, no hidden unit fallback, no protected
  content, private data, network/telemetry path, release-readiness claim, or
  professional/code-compliance claim.

## 2026-06-16 - TP-UNITS-B2-VIEWPORTDRAFTUNITS-001 viewport draft unit operation seam

- Operation applier now accepts explicit viewport `create_node` coordinate
  values in compatible accepted DEC-018 length units and normalizes them to
  the model document's stored `project.units.length` scalar basis.
- Explicit `connect_pipe_run` validation now accepts compatible length units
  for draft pipe section OD/wall values, compares wall thickness after
  conversion, and preserves the entered section quantity unit in the created
  segment.
- Witnesses cover node coordinates entered as millimetres normalized to
  metres and pipe OD/wall values entered as millimetres.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2-VIEWPORTDRAFTUNITS-001.md`;
  primary DEL-07-01 and supporting DEL-02-02 records; SMOKE TP-MAC-181;
  completion log entry.
- Validation passed: operation-applier focused explicit tests 28/28; full
  operation-applier cargo suite 60 unit tests plus canonical hash and contract
  corpus checks; desktop Vitest, build, Playwright suites, and DEC-025
  dirty-tree sweep passed.
- Boundary unchanged: no direct mutation bypass, hidden fallback unit,
  protected content, private data, network/telemetry path, release-readiness
  claim, or professional/code-compliance claim changed.

## 2026-06-17 - Lifecycle Housekeeping

- Housekeeping lifecycle reset: `_STATUS.md` current state set to `IN_PROGRESS` to reflect current code development in progress. This does not change review, issuance, release readiness, professional approval, certification, sealing, authentication, or code-compliance status.

## 2026-06-17 - TP-UNITS-BTAIL-OPDIFFUNITWITNESS-001 operation diff unit witnesses

- Primary role for the bounded Phase B-tail operation diff unit witness
  tranche. The desktop Operation Diff Preview packet now emits DEC-018
  unit-system disclosure and one unit-preservation witness per unit-bearing
  local diff row.
- The diff packet preserves before/after value text, unit, and dimension with
  `conversion_performed=false`; visible UI evidence shows the DEC-018
  disclosure and `count=1` witness for the focused material-modulus edit
  fixture.
- Validation passed: focused App Vitest 55/55, focused R2 Playwright smoke
  2/2, full desktop Vitest 397/397, desktop production build with the
  existing Vite large-chunk warning, and `git diff --check`. Playwright was
  not extended with a witness-specific assertion because the broad R2 smoke
  does not retain a queued diff row without altering later flow; an attempted
  queue-and-clear path hung and was removed.
- Boundary preserved: no operation schema change, operation application,
  accepted model-state mutation, durable acceptance persistence, unit
  conversion API, protected standards content, private payload, lifecycle
  state transition, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.

## 2026-06-17 - TP-UNITS-BTAIL-PROPERTYINTENTUNITVALID-001 supporting operation-intent evidence

- Supporting role for DEL-07-02 Property Inspector tranche: unit-bearing
  editor modify and create intents now carry a pre-application
  unit-dimension validation status in the existing `validation.unit_validation`
  field.
- This is operation-intent metadata only. It makes the review preview more
  explicit before validate/apply, but it does not change the operation applier,
  diff generation, durable persistence, or accepted model-state mutation
  semantics.
- Validation passed: focused App Vitest 55/55, focused R2/R3 Playwright smoke
  file 14/14, full desktop Vitest 397/397, and desktop production build with
  the existing Vite large-chunk warning.
- Boundary preserved: no operation schema change, operation application,
  accepted model-state mutation, durable acceptance persistence, unit
  conversion API, protected standards content, private data, lifecycle
  transition, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim changed.

## 2026-06-17 - TP-UNITS-BTAIL-LOADMANAGERDIMLESSUNITVALID-001 supporting operation-intent evidence

- Supporting role for DEL-05-02/DEL-07-02 Load Cases manager tranche:
  non-unit-bearing load-case and combination operation intents now record
  pre-application unit validation as `not_required_dimensionless`.
- This is operation-intent metadata only. It makes preview/diff input clearer
  while keeping schema validation, constraint validation, diff generation,
  operation application, durable persistence, and audit receipt semantics
  unchanged.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-LOADMANAGERDIMLESSUNITVALID-001.md`;
  `apps/desktop/SMOKE.md` TP-MAC-217; completion log entry.
- Validation passed: focused manager App Vitest 18/18, full desktop Vitest
  399/399, focused R2/R3 Playwright 14/14, and desktop production build with
  the existing Vite large-chunk warning.
- Boundary preserved: no operation schema change, operation application,
  accepted model-state mutation, durable persistence, unit conversion API,
  protected standards content, private data, lifecycle transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## 2026-06-17 - TP-UNITS-BTAIL-VIEWPORTCOMPUNITVALID-001 supporting operation-intent evidence

- Supporting role for DEL-07-01 viewport tranche: the reference-only
  `insert_component_symbol` placeholder intent now records pre-application
  unit validation as `not_required_dimensionless`.
- This is operation-intent metadata only. Generic node and pipe-run gesture
  placeholders remain `not_run`; explicit node/pipe authoring paths remain
  covered by length unit-validation evidence.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-VIEWPORTCOMPUNITVALID-001.md`;
  `apps/desktop/SMOKE.md` TP-MAC-218; completion log entry.
- Validation passed: focused App Vitest 1/1 and focused R2/R3 Playwright
  smoke 16/16; full desktop Vitest 399/399; desktop production build with the
  existing Vite large-chunk warning.
- Boundary preserved: no operation schema change, operation application,
  accepted model-state mutation, durable persistence, unit conversion API,
  protected standards content, private data, lifecycle transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## 2026-06-17 - TP-UNITS-BTAIL-DELETEINTENTUNITVALID-001 supporting operation-intent evidence

- Supporting role for DEL-07-02 Property Inspector tranche: explicit support,
  node, and pipe delete intents now record pre-application unit validation as
  `not_required_dimensionless`.
- This is operation-intent metadata only. It makes the delete preview more
  explicit before validate/apply, while keeping schema/reference validation,
  diff generation, operation application, durable persistence, and accepted
  model-state mutation semantics unchanged.
- Validation evidence is recorded in the DEL-07-02 primary run record and this
  deliverable's supporting run record. DEC-025 sweep evidence is recorded in
  closeout artifacts.
- Boundary preserved: no operation schema change, operation application,
  accepted model-state mutation, durable acceptance persistence, unit
  conversion API, protected standards content, private data, lifecycle
  transition, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim changed.
## 2026-06-18 - TP-UNITS-BTAIL-LOADMANAGERLINTUNITS-001 supporting report-lint inventory evidence

- Supporting role for DEL-16-02: Report Content Lint now inventories the
  existing Load Case Manager operation unit-validation previews through
  `load-manager-unit-validation-surface`.
- The lint packet now includes
  `apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx`; public
  unit-policy targets increase from 38 to 39 while conversion-witness targets
  remain two.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-LOADMANAGERLINTUNITS-001.md`;
  `apps/desktop/SMOKE.md` TP-MAC-256; completion log entry; primary
  DEL-08-05 run record and supporting DEL-07-02/DEL-05-01/DEL-05-02/DEL-02-02
  run records.
- Validation passed: focused App Vitest workspace-render; focused load/unit
  App tests 26/26; focused R2 Playwright smoke 2/2; full desktop Vitest
  399/399; single-worker R2/R3 Playwright smoke 18/18; `git diff --check`;
  and desktop production build with the existing Vite large-chunk warning.
- Boundary preserved: no operation validation, operation application, diff
  preview, unit conversion behavior, private data, lifecycle transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.
