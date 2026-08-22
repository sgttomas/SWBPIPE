# DEL-07-02 Memory

## 2026-07-20 - R14-W2-XREPAIR-01 wind schema-mirror test repair

- Cross-package repair under disposition `R14-W2-XREPAIR-01`: R14-W2 T4
  (DEL-05-01 sub-span wind exposure, landed `a854d43a1`) changed
  `schemas/model.schema.yaml` `WindEquivalentStaticInput` so `required` is
  `[pressure, shape_factor, direction]` with `exposed_element_refs` and the
  new `exposed_spans` as alternative exposure properties; this deliverable's
  mirror assertion in
  `apps/desktop/src/features/model-tree/schemaSlotEmission.test.tsx` still
  asserted the old four-key required set and truthfully failed the DEC-025
  evidence sweep.
- The mirror assertion cluster now asserts the landed schema truth
  equal-or-stronger: exact new required triple plus existence of both
  `exposed_element_refs` and `exposed_spans` in `wind.properties`, with the
  preview-key mapping comment updated (preview `exposed_pipe_refs` maps to
  canonical `exposed_element_refs`; `exposed_spans` is the sub-span
  alternative with no preview counterpart). No other line in the file
  changed.
- Focused desktop vitest file passed 10/10; registered `evidence-sweep`
  re-ran PASS with exactly one new sweep JSON
  (`SWEEP_20260720T055048Z_27110b28074a-dirty.json`).
- The sub-span GUI-emit follow-on (surfacing `exposed_spans` in the desktop
  authoring UI) is out of this repair's scope and is recorded on the
  campaign slate. `_STATUS.md` Remaining is untouched — this repair closes
  no Remaining row.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-07-20_R14W2_XREPAIR01.md`.

## 2026-07-12 - D-41 R5 T1 DEC-074 O2 ownership delegation

- Accepted delegation: `DEL-00-05` owns GUI state/interaction architecture;
  `DEL-07-02` owns model-tree/property-inspector behavior implementation
  within that architecture.
- `Specification.md`, `Datasheet.md`, `Guidance.md`, and `Procedure.md` now
  record that boundary and the bounded current implementation evidence,
  resolving `PDU-009` and its corresponding ownership clarification under
  `DEC-074` O2.
- Four genuine product/hardening residuals remain: model-level
  `modulus_basis_records` entry, broader unit entry/pickers, test/factoring
  hardening, and broader canvas/tree/editor UX. O2 is landed, but the D-41
  program bootstrap remains in `_STATUS.md` Remaining until applicable
  `PDU-054`/`PDU-055` declaration-currentness updates are backchecked.
- Lifecycle remains `IN_PROGRESS`. No product code, other-deliverable file,
  dependency, DAG, register, review disposition, protected/private content,
  release claim, professional ruling, certification, sealing, or
  code-compliance claim changed.
- Evidence: `_run_records/WORKING_ITEMS_RUN_2026-07-12_D41-R5-T1-DEC074-O2.md`.

## 2026-06-18 - TP-UNITS-BTAIL-PROPERTYINSPECTORLINTUNITS-001 supporting report-lint inventory evidence

- Supporting role for DEL-07-02: Report Content Lint now inventories the
  existing Property Inspector operation unit-validation surface through
  `property-inspector-unit-validation-surface`.
- The lint packet now includes
  `apps/desktop/src/features/model-tree/PropertyInspector.tsx` and
  `target:desktop-property-inspector-template`; public unit-policy targets
  increase from 39 to 40 while conversion-witness targets remain two.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-PROPERTYINSPECTORLINTUNITS-001.md`;
  `apps/desktop/SMOKE.md` TP-MAC-257; completion log entry; primary
  DEL-08-05 run record and supporting DEL-16-02/DEL-02-02 run records.
- Validation passed: focused App Vitest workspace-render; focused Property
  Inspector create-intent App tests 4/4; focused R2 Playwright smoke 2/2;
  full desktop Vitest 399/399; single-worker R2/R3 Playwright smoke 18/18;
  and desktop production build with the existing Vite large-chunk warning.
- Boundary preserved: no GUI behavior, editor-intent construction, operation
  validation/application, unit conversion behavior, protected standards
  content, private data, lifecycle transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## 2026-06-17 - TP-UNITS-BTAIL-SUPPORTUNITS-001 support stiffness unit authoring

- WORKING_ITEMS added support creation linear-stiffness unit/value controls to
  the Property Inspector and preserves the entered `{value, unit}` in the
  queued `create_support` operation payload.
- Applied support rows now display the preserved linear-stiffness value/unit in
  the selected support properties readback.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-SUPPORTUNITS-001.md`
  and `apps/desktop/SMOKE.md` TP-MAC-191.
- Validation passed: operation-applier cargo suite, focused App Vitest, full
  desktop Vitest 390/390, desktop build, and Rust formatting check.
- Boundary unchanged: local preview authoring only; no protected content,
  private data, network/telemetry path, lifecycle state transition,
  professional approval, release-readiness, certification, sealing,
  authentication, or code-compliance claim changed.

## 2026-05-08 Type 2 Implementation

Implemented a deterministic model-tree/property-inspector contract slice under
`core/gui/model_tree/` with focused coverage in
`tests/test_model_tree_property_inspector.py`.

The implementation records tree nodes, selected entity state, inspector fields,
unit/provenance/privacy metadata, unresolved `TBD` values, and diagnostics. It
does not mutate persisted project data, run solvers, fill missing engineering
values, or make professional/code-compliance claims.

## 2026-05-11 TP-RECON-01 Reconciliation

Archived TP-RECON-01 evidence reconciles the 2026-05-08 Type 2 slice and
2026-05-09 evidence promotion into this deliverable-local history. Evidence
rows record `COMMITTED` implementation evidence at `6e0b8f4` (`core: implement
tranche l gui contracts`) while lifecycle remains `CHECKING`.

Implemented evidence covers deterministic model-tree/property-inspector
contract records for navigation nodes, selected entity state, field
descriptors, unresolved `TBD` values, unit/provenance/privacy metadata,
validation messages, and diagnostics. Commit name-status evidence lists
`core/gui/model_tree/__init__.py`, `core/gui/model_tree/engine.py`,
`tests/test_model_tree_property_inspector.py`, this `MEMORY.md`, this
`_STATUS.md`, and `_run_records/TASK_RUN_2026-05-08_type2_implementation.md`.

Verification evidence: the local run record reports
`python3 tests/test_model_tree_property_inspector.py` passed; Tranche L handoff
and scope review record the focused `PYTHONDONTWRITEBYTECODE=1 python3
tests/test_model_tree_property_inspector.py` check, adjacent schema, security,
and report checks, and `git diff --check` as passed.

Deferred scope remains full GUI runtime/desktop shell, production persistence,
direct solver/domain mutation, hidden missing-data repair, private/protected
data payloads, and authority/reliance claims.

## 2026-05-16 - DEV-001 downstream PKG-02 audit memory addendum

Durable context preserved after reconciliation review:
- DEV-001 package-worker audit reviewed this deliverable for downstream compatibility with the accepted PKG-02 foundation contracts.
- Local audit artifacts are `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_REVIEW.md` and `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/Review_Findings.csv`.
- Package audit summary is `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/_audit/PKG02_DOWNSTREAM_REVIEW_2026-05-16.md`; package run record is `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/_run_records/TASK_RUN_2026-05-16_PKG02_DOWNSTREAM_PACKAGE_AUDIT.md`.
- This was audit evidence only. It did not change lifecycle state, authorize release, or make a professional, certification, sealing, approval, or code-compliance claim.
- The May 16 package-worker TASK run record did not fully preserve canonical per-deliverable TASK documentation context; this addendum preserves the durable deliverable-local pointer without modifying the completed run record.

## 2026-06-04 - TP-AUTHORITY-REFRESH-0_7-DAG006

- WORKING_ITEMS/TASK workforce current-authority refresh applied to active deliverable-local surfaces for `DEL-07-02`.
- Current authority basis is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7` plus approved `execution/_DAG/DAG-006/` active graph authority.
- Historical run records, historical DAG row IDs, review dispositions, lifecycle `_STATUS.md`, aggregate DAG artifacts, candidate edges, repo-level governance files, schemas, code, and tools were intentionally not changed by this refresh.
- Preserved historical references remain evidence of earlier work, not current authority claims.

## 2026-06-06 - PKG-07 test discovery evidence

- Parent `TASK-PKG07-TESTDISC-001` added pytest wrapper discovery for the existing DEL-07-02 Python contract test: `tests/test_model_tree_property_inspector.py::test_model_tree_property_inspector_main`.
- Parent evidence reports pytest collection across eight PKG-07 Python test files found 11 tests; the same pytest set passed; direct script invocations for the seven wrapper files passed; `npm test --workspace apps/desktop` passed; `cargo test --manifest-path core/gui/viewport_editor/Cargo.toml` passed.
- Conclusion: DEL-07-02's model-tree/property-inspector evidence remains technically supported at the test-discovery and contract-test level.
- This addendum does not change lifecycle, acceptance, release, professional, code-compliance, certification, sealing, `ISSUED`, or `HumanDisposition` posture. Local `Review_Findings.csv` contains no non-empty finding rows as read for this task; no human-disposition edits were made, and `HumanDisposition=TBD` remains preserved where applicable.

## 2026-06-06 - CHECKING readiness review

- TASK review worker read the required deliverable-local basis, four documents, latest DEL-07-02 test-discovery evidence, package fan-in, and package human-disposition record.
- Added a dated CHECKING-readiness section to `_REVIEW.md` and created `_run_records/TASK_RUN_2026-06-06_DEL-07-02_CHECKING_READINESS_REVIEW.md`.
- Recommendation: `MOVE_TO_CHECKING` for bounded DEL-07-02 local lifecycle consideration. Basis: prior PASS review, no non-empty `Review_Findings.csv` rows, DEL-07-02 wrapper discovery evidence, pytest 11/11, direct scripts passed, desktop Vitest 5/5, viewport Rust tests 6/6, and no DEL-07-02 human-disposition blocker.
- Remaining upstream `TBD` dependency satisfaction rows remain recorded as future interface/dependency evidence and were not changed or closed by this review.
- No lifecycle, dependency, finding-disposition, four-document, code, schema, fixture, test, DAG, release, professional, code-compliance, certification, sealing, approval, `ISSUED`, or public-readiness state was changed.

## 2026-06-10 - TP-APP-R2-VIEWSELECT-001 viewport-to-inspector binding

- WORKING_ITEMS app-integration tranche connected the viewport's rendered
  model entities to this deliverable's shared model-tree/property-inspector
  selection contract. Selecting a loaded node, pipe, support, or component
  from the viewport updates the inspector and active model-tree row.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-10_viewport_selection_binding.md`
  and `apps/desktop/SMOKE.md` TP-MAC-84. Validation passed:
  `npm test --workspace apps/desktop` (26/26) and
  `npm run build --workspace apps/desktop`; live browser smoke on
  `http://127.0.0.1:5174/` confirmed viewport selection state with no console
  warnings/errors.
- The tranche does not broaden property editing semantics, supply missing
  engineering values, perform unit conversion, mutate persisted project data,
  or make professional/release/code-compliance claims. Structured operation
  apply remains the only model mutation path.
- No lifecycle state, review finding status, dependency authority, release
  readiness, professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards data, private project data,
  network path, or telemetry path changed.

## 2026-06-12 - TP-UNITS-B2-INSPECTOREDITUNITS-001 Property Inspector quantity unit edits

- WORKING_ITEMS app-integration tranche added `Unit` selectors to existing
  material and pipe-section quantity edits in the Property Inspector.
- Editable material modulus, material thermal-expansion, pipe outside-diameter,
  and pipe wall-thickness fields now label the proposed value with the active
  unit basis and queue atomic `{ value, unit }` payloads.
- Browser preview remains model-metadata-only. Desktop/Tauri mode can use
  accepted DEC-018 catalog options.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-12_property_inspector_quantity_unit_edits.md`
  and `apps/desktop/SMOKE.md` TP-MAC-138. Validation passed:
  operation-applier cargo suites, src-tauri Rust tests 32/32, focused desktop
  Vitest 165/165, full desktop Vitest 216/216, desktop build, and Playwright
  R2 smoke 2/2.
- No lifecycle state, review finding status, dependency authority,
  release-readiness, professional approval, certification, sealing,
  authentication, code-compliance claim, protected/private data, network path,
  telemetry path, project-wide unit picker, or project unit mutation changed.

## 2026-06-12 - TP-UNITS-B2-UNITPICKERS-001 Property Inspector unit selectors

- WORKING_ITEMS app-integration tranche added unit selectors to the Property
  Inspector material and pipe-section creation forms: `Length unit`,
  `Modulus unit`, and `Thermal expansion unit`.
- Browser preview remains model-metadata-only with no synthesized fallback
  catalog; desktop/Tauri mode can show accepted DEC-018 catalog-backed
  options.
- Applying the created material/section operations preserves entered units in
  the local session model after operation-seam validation.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-12_property_inspector_unit_selectors.md`
  and `apps/desktop/SMOKE.md` TP-MAC-134. Validation passed:
  operation-applier cargo suites, src-tauri Rust tests 32/32, focused desktop
  Vitest 165/165, full desktop Vitest 216/216, desktop build, and Playwright
  R2 smoke 2/2.
- No lifecycle state, release-readiness, professional approval,
  certification, sealing, authentication, code-compliance claim,
  protected/private data, network path, or telemetry path changed.

## 2026-06-12 - TP-UNITS-B2-LOADPICKERS-001 primitive-load unit selectors

- WORKING_ITEMS app-integration tranche added visible unit selection to the
  Load Cases manager primitive-load creation form. The selector follows the
  selected category/direction and labels the magnitude field with the current
  unit basis.
- Browser preview remains model-metadata-only. Desktop/Tauri mode can show
  accepted DEC-018 catalog options filtered to the selected primitive-load
  dimension.
- Applying created primitive-load operations preserves entered units in the
  local session model after operation-seam validation.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-12_primitive_load_unit_selectors.md`
  and `apps/desktop/SMOKE.md` TP-MAC-136. Validation passed:
  operation-applier cargo suites, src-tauri Rust tests 32/32, focused desktop
  Vitest 165/165, full desktop Vitest 216/216, desktop build, and Playwright
  R2 smoke 2/2.
- No lifecycle state, release-readiness, professional approval,
  certification, sealing, authentication, code-compliance claim,
  protected/private data, network path, or telemetry path changed.

## 2026-06-12 - TP-UNITS-B2-PRIMEDITUNITS-001 primitive-load magnitude unit edits

- WORKING_ITEMS app-integration tranche added a `Magnitude unit` selector to
  the selected primitive-load edit panel in the Load Cases manager.
- The edit preview now queues an atomic `{ value, unit }` payload for existing
  primitive-load magnitudes; browser preview remains model-metadata-only and
  desktop/Tauri mode can use accepted DEC-018 catalog options.
- Applying a validated edit preserves the entered unit in the local session
  model by updating the sibling `.unit` field.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-12_primitive_load_magnitude_unit_edits.md`
  and `apps/desktop/SMOKE.md` TP-MAC-137. Validation passed:
  operation-applier cargo suites, src-tauri Rust tests 32/32, focused desktop
  Vitest 165/165, full desktop Vitest 216/216, desktop build, and Playwright
  R2 smoke 2/2.
- No lifecycle state, release-readiness, professional approval,
  certification, sealing, authentication, code-compliance claim,
  protected/private data, network path, or telemetry path changed.

## 2026-06-12 - TP-UNITS-B2-INSPECTORLABELS-001 property-inspector unit basis labels

- WORKING_ITEMS app-integration tranche added a visible `Unit basis` panel to
  the Property Inspector and connected material/section creation labels to
  catalog-aware frontend helpers.
- Browser preview now states that it is using model metadata only for these
  labels (`m, model metadata`, `Pa, model metadata`, `1/degC, model
  metadata`) instead of implying a reviewed desktop catalog is available.
- The operation payload units remain unchanged and still come from the model
  document; no unit picker or conversion behavior was added.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-12_property_inspector_unit_basis_labels.md`,
  the corresponding DEL-02-02 run record, and `apps/desktop/SMOKE.md`
  TP-MAC-131. Validation passed: focused unit-catalog/App Vitest 48/48,
  Playwright R2 smoke 2/2, full desktop Vitest 216/216, and desktop build.
- No lifecycle state, review finding status, dependency authority, release
  readiness, professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards data, private project data,
  network path, telemetry path, fallback browser catalog, unit conversion, or
  hidden engineering default changed.

## 2026-06-12 - TP-APP-R2-DELCOMBINATION-001 full-combination deletion authoring

- WORKING_ITEMS app-integration tranche extended the Load Cases manager with
  a full-combination `Queue delete combo` control in the selected-combination
  editor.
- The UI queues a structured `delete_combination` operation with the selected
  combination id/label/basis/terms as the before-state guard and does not
  mutate the model directly.
- Applying the operation through `OperationApplyPanel` removes the
  combination row and updates the summary to `0 combinations`; the existing
  term-level delete path remains separate as `delete_combination_term`.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_combination_authoring.md`
  and `apps/desktop/SMOKE.md` TP-MAC-122. Validation passed:
  focused App Vitest 1/1, full desktop Vitest 195/195, desktop build,
  Playwright smoke 1/1 with delete-preview coverage, and in-app browser
  full-combination delete smoke.
- No lifecycle state, review finding status, dependency authority, release
  readiness, professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards data, private project data,
  network path, or telemetry path changed.

## 2026-06-12 - TP-APP-R2-DELPRIMLOAD-001 primitive-load deletion authoring

- WORKING_ITEMS app-integration tranche extended the Load Cases manager with a
  primitive-load `Queue delete` intent for the selected primitive row.
- The UI queues a structured `delete_primitive_load` operation with the
  selected primitive load's display and unit/dimension metadata as guards and
  does not mutate the model directly.
- Applying the operation through `OperationApplyPanel` removes the primitive
  row and updates load-case summary counts.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_primitive_load_authoring.md`
  and `apps/desktop/SMOKE.md` TP-MAC-121. Validation passed:
  focused App Vitest 1/1, full desktop Vitest 192/192, desktop build,
  Playwright smoke 1/1, and in-app browser primitive-load delete smoke.
- No lifecycle state, review finding status, dependency authority, release
  readiness, professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards data, private project data,
  network path, or telemetry path changed.

## 2026-06-12 - TP-APP-R2-CREATESUPPORT-001 support creation inspector form

- WORKING_ITEMS app-integration tranche added a Property Inspector
  `Create support` form with support id, label, existing-node selector,
  restraint checkboxes, provenance, and `Queue support`.
- The form emits a review-only structured `create_support` operation intent;
  it does not mutate the session model directly.
- Applying the queued intent through the existing Apply Operations panel
  selects the created support in the model tree and shows node/restraint
  fields in the inspector.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-12_create_support_authoring.md`
  and `apps/desktop/SMOKE.md` TP-MAC-117. Validation passed:
  focused App Vitest 1/1, full desktop Vitest 177/177, desktop build,
  Playwright smoke 1/1, and live Chrome smoke with zero console errors.
- No lifecycle state, review finding status, dependency authority, release
  readiness, professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards data, private project data,
  network path, or telemetry path changed.

## 2026-06-12 - TP-APP-R2-CREATEMATERIAL-001 material creation inspector form

- WORKING_ITEMS app-integration tranche added a Property Inspector
  `Create material` form with material id, label, elastic modulus, shear
  modulus, optional thermal expansion coefficient, provenance, and
  `Queue material`.
- The form emits a review-only structured `create_material` operation intent;
  it does not mutate the session model directly.
- Applying the queued intent through the existing Apply Operations panel
  selects the created material in the model tree and shows the material
  quantities in the inspector.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-12_create_material_authoring.md`
  and `apps/desktop/SMOKE.md` TP-MAC-118. Validation passed:
  focused App Vitest 1/1, full desktop Vitest 180/180, desktop build,
  Playwright smoke 1/1, and live Chrome smoke with zero console errors.
- No lifecycle state, review finding status, dependency authority, release
  readiness, professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards data, private project data,
  network path, or telemetry path changed.

## 2026-06-12 - TP-APP-R2-CREATESECTION-001 section creation inspector form

- WORKING_ITEMS app-integration tranche added a model-tree `Sections` group
  and Property Inspector `Create section` form with section id, name, type,
  outside diameter, wall thickness, provenance, and `Queue section`.
- The form emits a review-only structured `create_section` operation intent;
  it does not mutate the session model directly.
- Applying the queued intent through the existing Apply Operations panel
  selects the created section in the model tree and shows the section
  quantities in the inspector.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-12_create_section_authoring.md`
  and `apps/desktop/SMOKE.md` TP-MAC-119. Validation passed:
  focused App Vitest 1/1, full desktop Vitest 183/183, desktop build,
  Playwright smoke 1/1, and live Chrome smoke with zero console errors.
- No lifecycle state, review finding status, dependency authority, release
  readiness, professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards data, private project data,
  network path, or telemetry path changed.

## 2026-06-11 - TP-APP-R2-COMBCREATE-001 combination creation editor

- WORKING_ITEMS app-integration tranche extended the Load Cases manager with a
  create-combination form for a new mechanics-basis combination.
- The app test applies `op:load-manager-create-combination:C-300`, verifies
  the manager summary changes to `2 load cases; 7 primitive loads; 2
  combinations`, verifies the new `combination:C-300` row with
  `basis=mechanics` and `load:L-100 x 1`, and checks the property inspector
  for the created combination.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_combination_creation_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-106. Validation passed:
  operation-applier format check, Rust operation-applier tests 34/34,
  src-tauri Rust tests 26/26, desktop Vitest 58/58, desktop build, desktop
  Playwright smoke 1/1, and in-app browser combination-create smoke.
- No lifecycle state, review finding status, dependency authority, release
  readiness, professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards data, private project data,
  network path, or telemetry path changed.

## 2026-06-11 - TP-APP-R2-COMBTERMDELETE-001 combination term deletion editor

- WORKING_ITEMS app-integration tranche extended the Load Cases manager with
  a selected-term delete control for existing load combinations.
- The app test selects `combination:C-OPER-ALT` term 1, queues and applies
  `op:load-manager-combination:C-OPER-ALT-term-1-delete`, verifies
  `load:L-200 x 0.5` is removed from the manager/property-inspector view, and
  verifies `load:L-100 x 1` remains.
- The rendered preview records `after=not_present`, unit `none`, dimension
  `dimensionless`, `direct_model_mutation_allowed=false`, and
  `professional_approval=false`.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_combination_term_deletion_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-105. Validation passed:
  operation-applier format check, Rust operation-applier tests 33/33,
  src-tauri Rust tests 26/26, desktop Vitest 56/56, desktop build, desktop
  Playwright smoke 1/1, and in-app browser combination-term deletion smoke.
- No lifecycle state, review finding status, dependency authority, release
  readiness, professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards data, private project data,
  network path, or telemetry path changed.

## 2026-06-11 - TP-APP-R2-COMBTERMCREATE-001 combination term creation editor

- WORKING_ITEMS app-integration tranche extended the Load Cases manager with
  a create-term editor for existing combinations.
- The app regression creates `load:L-300`, selects it in the combination-term
  form, queues `op:load-manager-combination:C-OPER-ALT-term-2-create`, applies
  it through `OperationApplyPanel`, and verifies the manager row and property
  inspector include the new term.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_combination_term_creation_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-104. Validation passed:
  operation-applier format check, Rust operation-applier tests 32/32,
  src-tauri Rust tests 26/26, desktop Vitest 54/54, desktop build, desktop
  Playwright smoke 1/1, and in-app browser combination-term-create smoke.
- No lifecycle state, review finding status, dependency authority, release
  readiness, professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards data, private project data,
  network path, or telemetry path changed.

## 2026-06-11 - TP-APP-R2-IMPOSED-001 imposed-displacement primitive-load creation editor

- WORKING_ITEMS app-integration tranche extended the Load Cases manager with
  create-primitive support for explicit support-target imposed displacements.
- The category selector now exposes `imposed_displacement`, switches the
  target picker to existing supports, switches direction options to
  `UX|UY|UZ|RX|RY|RZ`, and previews translational DOFs with unit `m` /
  dimension `displacement` and rotational DOFs with unit `rad` / dimension
  `rotation`.
- The app test applies `load:L-100-I300`, verifies the manager summary changes
  to `2 load cases; 8 primitive loads; 1 combinations`, verifies
  `load:L-100` primitive count, and verifies the new support-targeted row.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_imposed_displacement_load_creation_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-102. Validation passed:
  operation-applier format check, Rust operation-applier tests 30/30,
  src-tauri Rust tests 26/26, desktop Vitest 50/50, desktop build, desktop
  Playwright smoke 1/1, and in-app browser imposed-displacement smoke.
- No lifecycle state, review finding status, dependency authority, support
  coordinate policy, solver boundary behavior, release readiness,
  professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards data, private project data,
  network path, or telemetry path changed.

## 2026-06-11 - TP-APP-R2-MOMENTCREATE-001 concentrated moment primitive-load creation editor

- WORKING_ITEMS app-integration tranche extended the Load Cases manager with a
  create-primitive category and rotational direction control for explicit
  concentrated nodal moments.
- The app test applies `load:L-100-M300`, verifies the manager summary changes
  to `2 load cases; 8 primitive loads; 1 combinations`, verifies the
  `load:L-100` primitive count, and verifies the new moment primitive row.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_moment_load_creation_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-100. Validation passed:
  operation-applier format check, Rust operation-applier tests 28/28,
  src-tauri Rust tests 26/26, desktop Vitest 45/45, desktop build, desktop
  Playwright smoke 1/1, and in-app browser concentrated moment-create smoke.
- No lifecycle state, review finding status, dependency authority, release
  readiness, professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards data, private project data,
  network path, or telemetry path changed.

## 2026-06-11 - TP-APP-R2-COMBBASIS-001 combination basis editor

- WORKING_ITEMS app-integration tranche extended the Load Cases manager with a
  selected-combination basis editor.
- The app test applies `op:load-manager-combination:C-OPER-ALT-basis`, updates
  the row from `basis=mechanics` to `basis=mechanics_user_review`, verifies the
  property inspector, confirms zero pending operations, and confirms stale
  solve state is reset.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_combination_basis_editor.md` and
  `apps/desktop/SMOKE.md` TP-MAC-103. Validation passed:
  operation-applier format check, Rust operation-applier tests 31/31,
  src-tauri Rust tests 26/26, desktop Vitest 52/52, desktop build, desktop
  Playwright smoke 1/1, and in-app browser combination-basis smoke.
- No lifecycle state, review finding status, dependency authority, term
  creation/deletion, broader algebra authoring, code/rule combination claim,
  release readiness, professional approval, certification, sealing,
  authentication, code-compliance claim, protected standards data, private
  project data, network path, or telemetry path changed.

## 2026-06-11 - TP-APP-R2-DISTLOAD-001 distributed primitive-load creation editor

- WORKING_ITEMS app-integration tranche extended the Load Cases manager with a
  create-primitive category selector and pipe target control for explicit
  distributed element forces.
- The app test applies `load:L-100-D300`, verifies the manager summary changes
  to `2 load cases; 8 primitive loads; 1 combinations`, verifies the
  `load:L-100` primitive count, and verifies the new distributed primitive
  row.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_distributed_load_creation_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-99. Validation passed:
  operation-applier format check, Rust operation-applier tests 27/27,
  src-tauri Rust tests 26/26, desktop Vitest 43/43, desktop build, desktop
  Playwright smoke 1/1, and in-app browser distributed primitive-create smoke.
- No lifecycle state, review finding status, dependency authority, release
  readiness, professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards data, private project data,
  network path, or telemetry path changed.

## 2026-06-11 - TP-APP-R2-CONNECTPIPE-001 explicit straight-pipe connectivity

- WORKING_ITEMS app-integration tranche connected explicit viewport
  straight-pipe creation back into this deliverable's model
  tree/property-inspector contract. After an accepted connect-pipe apply, the
  created pipe becomes the active tree, viewport selection-layer, and
  inspector selection.
- The app test created `pipe:P-150` between `node:N-100` and `node:N-140`
  with `material:invented-carbon-steel`, OD `0.114 m`, wall `0.006 m`, and
  provenance `user_entered_local_preview`.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_explicit_straight_pipe_connectivity.md`
  and `apps/desktop/SMOKE.md` TP-MAC-92. Validation passed:
  `npm test --workspace apps/desktop` (33/33), `npm run build --workspace
  apps/desktop`, Rust operation-applier tests (22/22), and
  `npm run test:e2e:desktop` (1/1).
- No endpoint-edit semantics were added to the property inspector. No
  lifecycle state, review finding status, dependency authority, release
  readiness, professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards data, private project data,
  network path, or telemetry path changed.

## 2026-06-11 - TP-APP-R2-CANVASNODE-001 viewport canvas node drafting

- WORKING_ITEMS app-integration tranche connected canvas-drafted node
  geometry to this deliverable's model tree/property-inspector path after the
  user queues and applies the draft.
- Vitest creates `node:V-001` from a viewport pointer event, applies it
  through `OperationApplyPanel`, and verifies the active tree row plus
  captured inspector position.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_viewport_canvas_node_drafting.md`
  and `apps/desktop/SMOKE.md` TP-MAC-93. Validation passed:
  `npm test --workspace apps/desktop` (34/34), `npm run build --workspace
  apps/desktop`, `npm run test:e2e:desktop` (1/1), and in-app browser smoke.
- No property-inspector edit semantics, lifecycle state, review finding
  status, dependency authority, release readiness, professional approval,
  certification, sealing, authentication, code-compliance claim, protected
  standards data, private project data, network path, or telemetry path
  changed.

## 2026-06-11 - TP-APP-R2-LOADMGR-001 load-case primitive magnitude manager

- WORKING_ITEMS app-integration tranche added a right-rail Load Cases manager
  to the desktop preview UI. The manager surfaces load-case rows,
  primitive-load rows, combination terms, and the single-unit preview posture.
- The first editable scope is existing primitive-load magnitude fields. The
  manager queues `update_load` structured-operation intents such as
  `op:load-manager-load:L-100-load:L-100-P-magnitude` for
  `primitive_loads.2.magnitude.value`.
- Applying a queued manager intent uses the existing OperationApplyPanel,
  local-session acceptance record, undo/redo checkpoint path, and stale-solve
  clearing behavior.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_load_case_manager_primitive_magnitude.md`
  and `apps/desktop/SMOKE.md` TP-MAC-91. Validation passed:
  `npm test --workspace apps/desktop` (32/32), `npm run build --workspace
  apps/desktop`, `npm run test:e2e:desktop` (1/1), and `git diff --check -- .
  ':!init/init-prompt.md'`.
- Residual A4 scope remains: load-case creation, load status/kind editing,
  arbitrary primitive-load creation, imposed-displacement authoring breadth,
  full combination editing/algebra authoring, and packaged-Tauri saved-project
  smoke over edited load data.
- No lifecycle state, review finding status, dependency authority, release
  readiness, professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards data, private project data,
  network path, or telemetry path changed.

## 2026-06-10 - TP-APP-R2-INLINEVALID-001 property-inspector inline validation

- WORKING_ITEMS app-integration tranche added validate-only inline feedback to
  the property inspector's draft editor intent. The inspector calls the
  existing structured-operation validation seam and displays status, diff
  rows, diagnostics, and the no-mutation/professional-boundary note before an
  operation is queued or applied.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-10_property_inspector_inline_validation.md`
  and `apps/desktop/SMOKE.md` TP-MAC-85. Validation passed:
  `npm test --workspace apps/desktop` (26/26) and
  `npm run build --workspace apps/desktop`; timestamp-filtered live browser
  smoke on `http://127.0.0.1:5174/` confirmed the inline validate-only state
  with no new console warnings/errors after the final reload.
- The tranche does not apply operations, mutate persisted project data,
  broaden field support, perform unit conversion, supply missing engineering
  values, or make professional/release/code-compliance claims.
- No lifecycle state, review finding status, dependency authority, release
  readiness, professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards data, private project data,
  network path, or telemetry path changed.

## 2026-06-10 - TP-APP-R2-CREATENODE-001 explicit node create operation

- WORKING_ITEMS app-integration tranche connected explicit viewport node
  creation back into this deliverable's shared model tree/property inspector
  contract. After an accepted create-node apply, the created node becomes the
  active tree and inspector selection.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-10_explicit_node_create_operation.md`
  and `apps/desktop/SMOKE.md` TP-MAC-86. Validation passed:
  `npm test --workspace apps/desktop` (28/28), `npm run build --workspace
  apps/desktop`, and the Rust operation-applier suite (20/20). Browser smoke
  confirmed `tree-row-node:N-150` active, inspector position
  `8.4, 2.4, 2.8 m`, and `applied_operations=1`.
- The tranche also fixed live-browser hit-test layout issues for the viewport
  intent row and Apply Operations panel so controls remain actionable.
- No lifecycle state, review finding status, dependency authority, release
  readiness, professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards data, private project data,
  network path, or telemetry path changed.

## 2026-06-10 - TP-APP-R2-UNDOREDO-001 session undo/redo checkpoints

- WORKING_ITEMS app-integration tranche added local-session Undo/Redo
  controls and history counters to the Apply Operations panel.
- Undo/redo restores in-memory model and selection checkpoints, clears stale
  solve results, and remains explicitly labeled local-session-only with
  `saved_project_mutated=false`.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-10_session_undo_redo_checkpoints.md`
  and `apps/desktop/SMOKE.md` TP-MAC-87. Validation passed:
  `npm test --workspace apps/desktop` (28/28) and `npm run build
  --workspace apps/desktop`. Browser smoke created `node:N-155`, undid it,
  and redid it with no timestamp-filtered browser warnings/errors.
- No lifecycle state, review finding status, dependency authority, release
  readiness, professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards data, private project data,
  network path, or telemetry path changed.

## 2026-06-11 - TP-APP-R2-PIPEPICK-001 viewport pipe endpoint picking

- WORKING_ITEMS app-integration tranche extended viewport node-target
  selection so it can fill the explicit straight-pipe `from`/`to` fields
  when the user arms endpoint-pick mode.
- The app test picks `node:N-100` and `node:N-140`, supplies the remaining
  explicit pipe fields, applies `pipe:P-151`, and verifies the created pipe
  is active in the model tree, viewport layer, and property inspector.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_viewport_pipe_endpoint_picking.md`
  and `apps/desktop/SMOKE.md` TP-MAC-94. Validation passed:
  `npm test --workspace apps/desktop` (35/35), `npm run build --workspace
  apps/desktop`, `npm run test:e2e:desktop` (1/1), and in-app browser
  endpoint-pick smoke; `git diff --check -- . ':!init/init-prompt.md'`
  passed.
- No lifecycle state, review finding status, dependency authority, release
  readiness, professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards data, private project data,
  network path, or telemetry path changed.

## 2026-06-11 - TP-APP-R2-LOADMETA-001 load-case metadata editor

- WORKING_ITEMS app-integration tranche extended the Load Cases manager with a
  selected load-case metadata editor for `status` and `kind`.
- The app test applies `load:L-100` status from `preview_only` to `TBD`,
  verifies the manager row and property inspector update, confirms pending
  operations return to zero, and separately previews `kind` from
  `primitive_user_load` to `TBD`.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_load_case_metadata_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-95. Validation passed:
  operation-applier format check, Rust operation-applier tests 23/23,
  src-tauri Rust tests 26/26, desktop Vitest 36/36, desktop build, desktop
  Playwright smoke 1/1, and in-app browser status-edit smoke.
- No lifecycle state, review finding status, dependency authority, release
  readiness, professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards data, private project data,
  network path, or telemetry path changed.

## 2026-06-11 - TP-APP-R2-COMBFACTOR-001 combination term-factor editor

- WORKING_ITEMS app-integration tranche extended the Load Cases manager with
  selectable existing combination term rows and a factor editor.
- The app test applies `combination:C-OPER-ALT` term 1 from `0.5` to `0.75`,
  verifies the manager row and property inspector update, confirms pending
  operations return to zero, and confirms stale solve state is reset.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_combination_term_factor_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-96. Validation passed:
  operation-applier format check, Rust operation-applier tests 24/24,
  src-tauri Rust tests 26/26, desktop Vitest 37/37, desktop build, desktop
  Playwright smoke 1/1, and in-app browser combination-factor smoke.
- No lifecycle state, review finding status, dependency authority, release
  readiness, professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards data, private project data,
  network path, or telemetry path changed.

## 2026-06-11 - TP-APP-R2-LOADCREATE-001 empty load-case creation editor

- WORKING_ITEMS app-integration tranche extended the Load Cases manager with
  an explicit create-load-case form for empty load-case shells.
- The app test applies `load:L-300`, verifies the manager summary changes to
  `3 load cases; 7 primitive loads; 1 combinations`, verifies
  `load:L-300; primitive_user_load; draft; primitives=0`, and checks the
  property inspector for the created load case.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_load_case_creation_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-97. Validation passed:
  operation-applier format check, Rust operation-applier tests 25/25,
  src-tauri Rust tests 26/26, desktop Vitest 39/39, desktop build, desktop
  Playwright smoke 1/1, and in-app browser load-case-create smoke.
- No lifecycle state, review finding status, dependency authority, release
  readiness, professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards data, private project data,
  network path, or telemetry path changed.

## 2026-06-11 - TP-APP-R2-PRIMCREATE-001 concentrated primitive-load creation editor

- WORKING_ITEMS app-integration tranche extended the Load Cases manager with a
  create-primitive form for explicit concentrated nodal forces.
- The app test applies `load:L-100-F300`, verifies the manager summary changes
  to `2 load cases; 8 primitive loads; 1 combinations`, verifies the
  `load:L-100` primitive count, and verifies the new primitive row.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_primitive_load_creation_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-98. Validation passed:
  operation-applier format check, Rust operation-applier tests 26/26,
  src-tauri Rust tests 26/26, desktop Vitest 41/41, desktop build, desktop
  Playwright smoke 1/1, and in-app browser primitive-create smoke.
- No lifecycle state, review finding status, dependency authority, release
  readiness, professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards data, private project data,
  network path, or telemetry path changed.

## 2026-06-11 - TP-APP-R2-PRESSTEMP-001 pressure and thermal primitive-load creation editor

- WORKING_ITEMS app-integration tranche extended the Load Cases manager with
  create-primitive support for explicit pressure and thermal element loads.
- The app tests apply `load:L-100-P300` and `load:L-100-T300`, verify the
  manager summary changes to `2 load cases; 8 primitive loads; 1 combinations`,
  verify `load:L-100` primitive count, and verify the new element-targeted
  rows with dimensions `pressure` and `temperature_interval`.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_pressure_thermal_load_creation_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-101. Validation passed:
  operation-applier format check, Rust operation-applier tests 29/29,
  src-tauri Rust tests 26/26, desktop Vitest 48/48, desktop build, desktop
  Playwright smoke 1/1, and in-app browser pressure/thermal smoke.
- No lifecycle state, review finding status, dependency authority, release
  readiness, professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards data, private project data,
  network path, or telemetry path changed.

## 2026-06-12 - TP-APP-R2-DELSUPPORT-001 support deletion authoring

- WORKING_ITEMS app-integration tranche extended the Property Inspector with
  a `Delete support` intent section visible only for selected support rows.
- The UI queues a structured `delete_support` operation with the selected
  support label as the before-state guard and does not mutate the model
  directly.
- Applying the operation through `OperationApplyPanel` removes the support row
  and returns selection to the project row; blocking diagnostics remain visible
  when an imposed-displacement primitive load references the support.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_support_authoring.md`
  and `apps/desktop/SMOKE.md` TP-MAC-120. Validation passed:
  focused App Vitest 2/2, full desktop Vitest 189/189, desktop build,
  Playwright smoke 1/1, and in-app browser support-delete smoke.
- No lifecycle state, review finding status, dependency authority, release
  readiness, professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards data, private project data,
  network path, or telemetry path changed.

## 2026-06-12 - TP-APP-R2-DELLOADCASE-001 load-case deletion authoring

- WORKING_ITEMS app-integration tranche extended the Load Cases manager with
  a `Queue delete case` intent section for the selected load-case row.
- The UI queues a structured `delete_load_case` operation with the selected
  load-case id/label/kind/status/primitive-count display as the before-state
  guard and does not mutate the model directly.
- Applying an unreferenced load-case deletion through `OperationApplyPanel`
  removes the load-case row and updates the manager count; referenced load
  cases are refused by the operation seam and surfaced through operation
  diagnostics.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_load_case_authoring.md`
  and `apps/desktop/SMOKE.md` TP-MAC-123. Validation passed:
  focused App Vitest 2/2, full desktop Vitest 201/201, desktop build,
  Playwright smoke 1/1, and in-app browser load-case create/delete smoke.
- No lifecycle state, review finding status, dependency authority, release
  readiness, professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards data, private project data,
  network path, or telemetry path changed.
## 2026-06-15 - TP-UNITS-B2-NODECOORDUNITS-001: node coordinate unit payloads

- Property Inspector node coordinate edits (`position.x`, `position.y`,
  `position.z`) now expose the unit-aware editor control and queue explicit
  `{value, unit}` payloads.
- Browser preview still uses model metadata only; it does not synthesize a
  DEC-018 fallback catalog. The R2 Playwright smoke pins the visible
  `position.y` unit selector and payload preview for `node:N-110`.
- Evidence:
  `DEL-16-02 .../_run_records/WORKING_ITEMS_RUN_2026-06-15_TP-UNITS-B2-NODECOORDUNITS-001.md`;
  SMOKE TP-MAC-170; desktop Vitest 385/385; desktop build clean; focused
  Playwright R2 smoke 2/2.
- Boundary unchanged: local-only editor intent behavior; no durable
  persistence, private data, protected content, network/telemetry path,
  professional approval, release-readiness, certification, sealing,
  authentication, or code-compliance claim.

## 2026-06-16 - TP-UNITS-B2-LOADINSPECTORUNITS-001 load-case inspector primitive unit payloads

- Property Inspector load-case editing now treats "First primitive magnitude"
  (`primitive_loads.0.magnitude.value`) as a unit-editable quantity and queues
  explicit `{value, unit}` payloads through the existing `update_load` operation
  intent path.
- Browser preview remains honest: it uses model metadata (`N/m` for the
  invented `load:L-100` first primitive) and does not synthesize a DEC-018
  fallback catalog.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2-LOADINSPECTORUNITS-001.md`;
  SMOKE TP-MAC-171; focused App Vitest 54/54; full desktop Vitest 386/386;
  desktop build passed; focused Playwright R2 smoke 2/2.
- Boundary unchanged: frontend-only; no backend operation, durable
  persistence, private data, protected content, network/telemetry path,
  professional approval, release-readiness, certification, sealing,
  authentication, or code-compliance claim.

## 2026-06-17 - Lifecycle Housekeeping

- Housekeeping lifecycle reset: `_STATUS.md` current state set to `IN_PROGRESS` to reflect current code development in progress. This does not change review, issuance, release readiness, professional approval, certification, sealing, authentication, or code-compliance status.

## 2026-06-17 - TP-UNITS-BTAIL-PROPERTYINTENTUNITVALID-001 property inspector unit validation evidence

- Primary role for a bounded Phase B-tail Property Inspector/unit-validation
  slice while C5.7 remains human-execution gated.
- Unit-bearing Property Inspector modify intents and explicit create-section,
  create-material, and create-support intents now record unit-dimension
  validation status instead of `unit_validation=not_run`.
- Browser preview records model-metadata declaration because the reviewed
  DEC-018 catalog is exposed only through the desktop backend command;
  desktop/Tauri routes can record accepted DEC-018 dimension matches or
  explicit mismatch/unreviewed/loading statuses.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-PROPERTYINTENTUNITVALID-001.md`;
  `apps/desktop/SMOKE.md` TP-MAC-206; completion log entry.
- Validation passed: focused App Vitest 55/55, focused R2/R3 Playwright smoke
  file 14/14, full desktop Vitest 18/18 files and 397/397 tests, and desktop
  production build with the existing Vite large-chunk warning.
- Boundary preserved: no DEC-018 catalog constant change, schema dimension
  enum change, unit conversion API change, solver behavior, operation
  application semantics, accepted model-state mutation, protected standards
  content, private data, lifecycle transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## 2026-06-17 - TP-UNITS-BTAIL-DELETEINTENTUNITVALID-001 delete-intent unit validation evidence

- Primary role for a bounded Phase B-tail Property Inspector/delete-intent
  unit-validation slice while C5.7 remains human-execution gated.
- Explicit support, node, and pipe delete intent previews now record
  `unit_validation=not_required_dimensionless` instead of leaving
  `unit_validation=not_run`.
- Delete intents do not introduce new unit-bearing numeric quantities; schema
  and reference checks remain in the existing validate/apply path.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-DELETEINTENTUNITVALID-001.md`;
  `apps/desktop/SMOKE.md` TP-MAC-216; completion log entry.
- Validation passed: focused App Vitest 56/56. Broader validation is recorded
  in the tranche run record and closeout sweep artifact.
- Boundary preserved: no DEC-018 catalog constant change, schema dimension
  enum change, unit conversion API change, delete operation behavior,
  reference validation, operation application semantics, accepted model-state
  mutation, solver behavior, protected standards content, private data,
  lifecycle transition, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.

## 2026-06-17 - TP-UNITS-BTAIL-LOADMANAGERDIMLESSUNITVALID-001 supporting GUI evidence

- Supporting role for DEL-05-02 Load Cases manager tranche: non-unit-bearing
  load-case and combination operation previews now render
  `unit_validation=not_required_dimensionless` beside their existing
  `unit=none`, `dimensionless`, direct-mutation, and professional-boundary
  markers.
- Primitive load magnitude create/edit previews remain unit-bearing and keep
  the existing unit/dimension validation helper.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-LOADMANAGERDIMLESSUNITVALID-001.md`;
  `apps/desktop/SMOKE.md` TP-MAC-217; completion log entry.
- Validation passed: focused manager App Vitest 18/18, full desktop Vitest
  399/399, focused R2/R3 Playwright 14/14, and desktop production build with
  the existing Vite large-chunk warning.
- Boundary preserved: no GUI behavior changed beyond preview metadata
  rendering; no operation application, accepted model-state mutation, durable
  persistence, unit conversion, protected standards content, private data,
  lifecycle transition, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.
## 2026-06-18 - TP-UNITS-BTAIL-LOADMANAGERLINTUNITS-001 supporting report-lint inventory evidence

- Supporting role for DEL-07-02: Report Content Lint now inventories the
  existing Load Case Manager operation unit-validation surface through
  `load-manager-unit-validation-surface`.
- The lint packet now includes
  `apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx` and
  `target:desktop-load-case-manager-template`; public unit-policy targets
  increase from 38 to 39 while conversion-witness targets remain two.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-LOADMANAGERLINTUNITS-001.md`;
  `apps/desktop/SMOKE.md` TP-MAC-256; completion log entry; primary
  DEL-08-05 run record and supporting DEL-05-01/DEL-05-02/DEL-16-02/DEL-02-02
  run records.
- Validation passed: focused App Vitest workspace-render; focused load/unit
  App tests 26/26; focused R2 Playwright smoke 2/2; full desktop Vitest
  399/399; single-worker R2/R3 Playwright smoke 18/18; `git diff --check`;
  and desktop production build with the existing Vite large-chunk warning.
- Boundary preserved: no GUI behavior, load-case editing, operation
  validation/application, unit conversion behavior, protected standards
  content, private data, lifecycle transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## 2026-08-21 - vocabulary round 3 explicit component-creation forms

- Inspector forms now cover tee, reducer, valve, flange, and expansion joint. Non-bend pipe roles are deliberate; expansion-joint entry requires explicit area, movement, four stiffness axes and units, hardware/manufacturer/pressure-thrust/geometry-source/stiffness-source references, and provenance.
- Failed N1/N2 review V1 records and their repaired terminal V2 reviews are preserved in `HELP-HUMAN-PIPING-20260821-VOCABULARY-R3/`; accepted commits are `8ca1984db45a9a8f6f3111a905b07c7d3da47c33` and `d1a8e20ae413be040b82428cdd0bdbef0809e8de`.
- Vocabulary rows 14 and 15 are closed. Existing Remaining items and IN_PROGRESS lifecycle remain unchanged.
