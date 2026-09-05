# MEMORY - DEL-07-01 3D Viewport and Centerline Editor

## 2026-07-12 - D-41 R5 T5 PDU-008 bounded viewport behavior

- Existing interaction evidence covers structured node and straight-pipe authoring, support handoff, component reference insertion, selection, and inspection through current application seams.
- Dedicated bend authoring and full component-symbol geometry authoring remain absent. No rotational result visualization is claimed here.
- Evidence: `_run_records/WORKING_ITEMS_RUN_2026-07-12_D41-R5-T5-PDU008.md`. Lifecycle remains `IN_PROGRESS`; the D-41 bootstrap remains for T7.

## Session 2026-05-02

Human project authority approved a small coordination cleanup followed by
`DEL-07-01` implementation from a fresh sealed dispatch brief.

## Work Completed

- Added `schemas/viewport_editor.schema.yaml` as a strict-JSON JSON Schema
  2020-12 contract for the first 3D centerline viewport/editor slice.
- Added `core/gui/viewport_editor/` as a bounded dependency-free Rust support
  crate for transient viewport state, view primitives, selection, diagnostics,
  application-service command intents, and its generated `Cargo.lock`.
- Added invented non-engineering fixture
  `fixtures/gui/invented/viewport_editor_session.json`.
- Added `tests/test_viewport_editor_contract.py` for deterministic schema and
  fixture checks.
- Updated focused `docs/SPEC.md` and `docs/TYPES.md` entries for the
  viewport/editor boundary.
- Set lifecycle display state to `CHECKING`.
- Annotated active non-architecture local dependency mirror rows
  `DAG-001-E0478` through `DAG-001-E0485` as `SATISFIED`.
- Added `DEL-07-01` as `WORKING_TREE` implementation evidence pending commit.
- Committed the implementation/closeout patch as
  `4785806 schema: add viewport editor contract`.
- Promoted `DEL-07-01` evidence to `COMMITTED` for commit `4785806` in
  `3d74e63 coordination: promote del-07-01 evidence`.

## Boundaries Preserved

- No Tauri/React/Vite app shell was created.
- No package manifests, frontend dependency versions, state-management library,
  Three.js runtime renderer, or Playwright rendering tests were introduced.
- No model tree, property inspector, material/component/rule-pack editor,
  solve-execution UX, or results-viewer behavior was implemented.
- Durable model changes are represented as application-service command intents,
  not direct persisted-project mutations.
- Transient camera, hover, selection, drag, and snap state remain separate from
  durable project payloads.
- No protected standards content, proprietary component/catalog data, private
  project data, private rule packs, private libraries, real secrets, or
  professional/code-compliance claims were introduced.

## Verification

- `python3 tests/test_viewport_editor_contract.py` passed.
- `cargo fmt --manifest-path core/gui/viewport_editor/Cargo.toml -- --check`
  passed after formatting.
- `cargo test --manifest-path core/gui/viewport_editor/Cargo.toml` passed with
  6 unit tests.
- `python3 tests/test_model_schema.py` passed.
- `python3 tests/test_component_section_schema.py` passed.
- `python3 tools/validation/validate_dependencies_schema.py
  "execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/Dependencies.csv"`
  passed.
- `python3 tools/coordination/build_dev001_blocker_queue.py --generated-date
  2026-05-02` passed with 68 unblocked / 5 blocked after commit-backed
  evidence promotion.

## Remaining TBDs

- Frontend application scaffold and package manifests.
- Exact GUI dependency versions and state-management library.
- Three.js runtime rendering integration.
- Browser/Playwright rendering tests.
- Application-service command transport and physical project container.
- Adjacent PKG-07 GUI slices: model tree/property inspector, editors,
  missing-data UX, results viewer, accessibility, and solve execution UX.

## 2026-05-11 TP-RECON-01 Reconciliation

- Reconciled `DEL-07-01` history from the TP-RECON-01 dispatch row and archived
  DEV-001 evidence bundle. The committed implementation evidence remains
  `4785806 schema: add viewport editor contract` dated 2026-05-02.
- Preserved `CHECKING`: the Rev 0.5 lifecycle snapshot lists lifecycle
  `CHECKING`, present deliverable-local status/context/dependency surfaces, and
  committed implementation evidence. The Rev 0.5 evidence-status row notes that
  completeness still depends on refreshed graph/context review.
- Recorded the implemented slice as a schema-first viewport/editor contract,
  bounded Rust support module, invented fixture, deterministic checks, focused
  docs, lifecycle display update, local dependency mirror update, and
  control-plane evidence update.
- Deferred runtime/product scope remains downstream: Tauri/React/Vite app shell,
  package manifests, Three.js runtime renderer, Playwright/browser rendering
  checks, model tree/property inspector, adjacent GUI slices, public transport,
  and physical project container.
- TP-MAC-01-B and the macOS desktop tranche plan identify later desktop-preview
  work using `DEL-07-01` as a governing contract, but worker launch,
  implementation dispatch, lifecycle/evidence changes, release status, and
  professional reliance remain outside this reconciliation.

## 2026-05-16 - DEV-001 downstream PKG-02 audit memory addendum

Durable context preserved after reconciliation review:
- DEV-001 package-worker audit reviewed this deliverable for downstream compatibility with the accepted PKG-02 foundation contracts.
- Local audit artifacts are `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_REVIEW.md` and `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/Review_Findings.csv`.
- Package audit summary is `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/_audit/PKG02_DOWNSTREAM_REVIEW_2026-05-16.md`; package run record is `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/_run_records/TASK_RUN_2026-05-16_PKG02_DOWNSTREAM_PACKAGE_AUDIT.md`.
- This was audit evidence only. It did not change lifecycle state, authorize release, or make a professional, certification, sealing, approval, or code-compliance claim.
- The May 16 package-worker TASK run record did not fully preserve canonical per-deliverable TASK documentation context; this addendum preserves the durable deliverable-local pointer without modifying the completed run record.

## 2026-06-04 - TP-AUTHORITY-REFRESH-0_7-DAG006

- WORKING_ITEMS/TASK workforce current-authority refresh applied to active deliverable-local surfaces for `DEL-07-01`.
- Current authority basis is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7` plus approved `execution/_DAG/DAG-006/` active graph authority.
- Historical run records, historical DAG row IDs, review dispositions, lifecycle `_STATUS.md`, aggregate DAG artifacts, candidate edges, repo-level governance files, schemas, code, and tools were intentionally not changed by this refresh.
- Preserved historical references remain evidence of earlier work, not current authority claims.

## 2026-06-06 - PKG-07 test-discovery evidence addendum

- Parent transcript `TASK-PKG07-TESTDISC-001` changed PKG-07 Python tests to add
  pytest wrapper functions; the `DEL-07-01` wrapper is
  `tests/test_viewport_editor_contract.py::test_viewport_editor_contract_main`.
- Parent evidence reported 11 collected and 11 passing pytest tests across the
  listed PKG-07 files, passing direct script invocations for the wrapper files,
  passing `npm test --workspace apps/desktop`, and 6 passing
  `core/gui/viewport_editor` cargo tests.
- `DEL-07-01` evidence remains technically supported by this discovery change.
  This addendum records evidence support only; it does not change lifecycle
  state, review disposition, release status, professional reliance,
  code-compliance, certification, sealing, approval, or `ISSUED` status.
- Local `Review_Findings.csv` contained no non-empty finding rows during this
  worker run. `HumanDisposition=TBD` remains preserved.

## 2026-06-06 - CHECKING-readiness review addendum

- TASK deliverable-local review read the required local surfaces, the latest
  local test-discovery evidence run, the PKG-07 test-discovery fan-in, and the
  PKG-07 human-disposition record.
- Recommendation recorded in `_REVIEW.md` and
  `_run_records/TASK_RUN_2026-06-06_DEL-07-01_CHECKING_READINESS_REVIEW.md`:
  `HOLD_IN_PROGRESS`.
- Basis: the 2026-05-16 compatibility review remains `PASS`, local
  `Review_Findings.csv` has header only, and June 6 validation evidence remains
  technically supportive; however `_STATUS.md` still records the 2026-05-11
  lifecycle correction that reset the deliverable to `IN_PROGRESS` pending
  further development. The June 6 work was test-discovery/evidence support, not
  closure of the deferred viewport/editor implementation scope.
- No lifecycle state, dependency state, review findings, four-document
  artifacts, schemas, code, fixtures, tests, release status, professional
  reliance status, certification, sealing, approval, code-compliance status, or
  `ISSUED` status was changed by this review.

## 2026-06-06 - Viewport closure tranche

- WORKING_ITEMS closure implementation added a bounded frontend viewport
  editor-intent bridge in `apps/desktop/src/features/viewport/PipeViewport.tsx`.
  The bridge records create-node, connect-pipe-run, and simple-component-symbol
  command intents as in-memory `pending_service_validation` records.
- The new intent bridge does not mutate persisted project payloads directly and
  preserves `unit_aware_domain_validation_required` for each intent.
- `apps/desktop/src/App.test.tsx` now verifies the DEL-07-01 desktop viewport
  intent behavior alongside the existing Three.js viewport smoke evidence.
- This tranche closes the deferred runtime/product evidence called out in the
  prior `HOLD_IN_PROGRESS` review for the bounded CHECKING gate: frontend app
  scaffold/package manifest, Three.js runtime integration, and visible
  command-intent behavior now have local technical-preview evidence.
- Remaining product-level items remain downstream or broader-program scope:
  production application-service transport, physical project container,
  browser/Playwright rendering checks, exact GUI dependency governance, and
  adjacent PKG-07 surfaces.
- Recommendation recorded in `_REVIEW.md` and
  `_run_records/TASK_RUN_2026-06-06_DEL-07-01_VIEWPORT_CLOSURE.md`:
  `MOVE_TO_CHECKING_WITH_HUMAN_LIFECYCLE_APPROVAL`.
- No lifecycle state, DAG authority, dependency authority, release status,
  professional reliance status, certification, sealing, approval,
  code-compliance status, protected-content disposition, or `ISSUED` status was
  changed by this tranche.

## 2026-06-06 - Lifecycle update to CHECKING

- Human project authority instructed: "Change the status to CHECKING".
- `_STATUS.md` now records `Current State: CHECKING` with `Last Updated:
  2026-06-06`.
- Basis is the viewport closure recommendation
  `MOVE_TO_CHECKING_WITH_HUMAN_LIFECYCLE_APPROVAL` in `_REVIEW.md` and
  `_run_records/TASK_RUN_2026-06-06_DEL-07-01_VIEWPORT_CLOSURE.md`.
- This lifecycle update moves the deliverable into review only. It does not
  issue the deliverable, accept release readiness, certify, seal, authenticate,
  approve engineering work, make a code-compliance claim, change DAG authority,
  change dependency authority, or alter protected/private-data disposition.

## 2026-06-10 - TP-APP-R2-VIEWSELECT-001 viewport-selection binding

- WORKING_ITEMS app-integration tranche added DOM-accessible viewport
  selection controls in `apps/desktop/src/features/viewport/PipeViewport.tsx`
  for loaded nodes, straight pipe segments, supports, and component markers.
- The controls update the shared app selection state, model-tree active row,
  property inspector, and viewport active highlight. They do not create or
  apply durable model operations; operation mutation remains behind the
  structured operation queue and explicit apply path.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-10_viewport_selection_binding.md`
  and `apps/desktop/SMOKE.md` TP-MAC-84. Validation passed:
  `npm test --workspace apps/desktop` (26/26) and
  `npm run build --workspace apps/desktop`; live browser smoke on
  `http://127.0.0.1:5174/` confirmed viewport selection state with no console
  warnings/errors.
- Residual A3 scope remains: true canvas raycast/gesture geometry capture,
  node/straight-pipe creation tools with explicit coordinates/connectivity,
  undo/redo, and inline validation messages. Viewport geometry-creation
  intents still block at apply rather than inventing values.
- No lifecycle state, release readiness, professional approval,
  certification, sealing, authentication, code-compliance claim, protected
  standards data, private project data, network path, telemetry path, or DAG
  authority changed.

## 2026-06-16 - TP-UNITS-B2-VIEWPORTDRAFTUNITS-001 viewport draft length-unit controls

- WORKING_ITEMS B2/B3 units tranche added visible length-unit selectors and
  unit-basis text to the viewport create-node and straight-pipe draft forms.
- Browser preview reports the explicit model-metadata fallback for viewport
  length units; Tauri-capable runs load the DEC-018 catalog and filter
  accepted length entries.
- Draft node coordinates and pipe geometry now queue structured operation
  intents with explicit selected length units.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2-VIEWPORTDRAFTUNITS-001.md`;
  supporting DEL-16-02 and DEL-02-02 records; SMOKE TP-MAC-181; completion
  log entry.
- Validation passed: focused App Vitest 54/54; full desktop Vitest 388/388;
  desktop build with existing Vite chunk-size warning; focused R2 Playwright
  smoke 2/2; full desktop Playwright 10/10 with `--workers=1`; in-app
  Browser viewport unit verification pass; DEC-025 dirty-tree sweep pass.
- Boundary unchanged: no project-unit mutation, hidden unit fallback,
  protected content, private data, network/telemetry path, release-readiness
  claim, or professional/code-compliance claim changed.

## 2026-06-11 - TP-APP-R2-CANVASNODE-001 viewport canvas node drafting

- WORKING_ITEMS app-integration tranche added primary-pointer canvas drafting
  for explicit node creation in `PipeViewport`.
- In WebGL mode the handler raycasts to the `y=0` drafting plane; in
  fallback/test mode it maps the pointer into the same bounded drafting plane.
  The gesture fills visible node id/label/x/y/z fields and does not queue or
  apply without user action.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_viewport_canvas_node_drafting.md`
  and `apps/desktop/SMOKE.md` TP-MAC-93. Validation passed:
  `npm test --workspace apps/desktop` (34/34),
  `npm run build --workspace apps/desktop`, `npm run test:e2e:desktop`
  (1/1), and in-app browser smoke at `http://127.0.0.1:5175/`.
- Residual A3 scope remains canvas gesture capture beyond node drafting,
  including pipe/connectivity and component/rigid authoring.
- No lifecycle state, release readiness, professional approval,
  certification, sealing, authentication, code-compliance claim, protected
  standards data, private project data, network path, telemetry path, or DAG
  authority changed.

## 2026-06-11 - TP-APP-R2-CONNECTPIPE-001 explicit straight-pipe connectivity

- WORKING_ITEMS app-integration tranche added an explicit straight-pipe form
  to the viewport editor for user-entered pipe id, label, endpoint nodes,
  material, outside diameter, wall thickness, non-zero `y_reference`, and
  provenance.
- The form queues `connect_pipe_run` intents for the structured operation seam;
  legacy one-click viewport pipe-run gestures remain blocked rather than
  inventing connectivity or section geometry.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_explicit_straight_pipe_connectivity.md`
  and `apps/desktop/SMOKE.md` TP-MAC-92. Validation passed:
  `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  (22/22), `npm test --workspace apps/desktop` (33/33),
  `npm run build --workspace apps/desktop`, and
  `npm run test:e2e:desktop` (1/1).
- Residual A3 scope remains: true canvas raycast/gesture geometry capture,
  rigid/component authoring, and broader editor coverage as new authoring
  surfaces land.
- No lifecycle state, release readiness, professional approval,
  certification, sealing, authentication, code-compliance claim, protected
  standards data, private project data, network path, telemetry path, or DAG
  authority changed.

## 2026-06-10 - TP-APP-R2-CREATENODE-001 explicit node create operation

- WORKING_ITEMS app-integration tranche added an explicit node-geometry form
  to the viewport editor for user-entered node id, label, and finite x/y/z
  coordinates in the project length unit.
- The form queues `create_node` intents for the structured operation seam;
  legacy one-click viewport node gestures remain blocked rather than
  inventing geometry.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-10_explicit_node_create_operation.md`
  and `apps/desktop/SMOKE.md` TP-MAC-86. Validation passed:
  `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  (20/20), `npm test --workspace apps/desktop` (28/28), and
  `npm run build --workspace apps/desktop`. Browser smoke on
  `http://127.0.0.1:5174/` created `node:N-150`, selected it, increased
  viewport targets from 14 to 15, and had no timestamp-filtered warnings or
  errors after the final reload.
- Residual A3 scope remains: true canvas raycast/gesture geometry capture,
  straight-pipe connectivity creation, undo/redo, and broader editor
  coverage as new authoring surfaces land.
- No lifecycle state, release readiness, professional approval,
  certification, sealing, authentication, code-compliance claim, protected
  standards data, private project data, network path, telemetry path, or DAG
  authority changed.

## 2026-06-11 - TP-APP-R2-DEFORMEDVIEW-001 viewport displacement overlay

- WORKING_ITEMS app-integration tranche added a solved-result overlay to
  `PipeViewport`: after preview mechanics produces solved displacement
  magnitudes, the Three.js scene draws an overlaid centerline and node markers
  with normalized display offsets.
- The viewport toolbar records the visible boundary:
  `scale=normalized_display_offset_not_physical_length`,
  `vector_direction=TBD`, `unit_basis=mm`, and
  `professional_claim=false`. No overlay is rendered for incomplete mechanics
  results.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_viewport_deformation_overlay.md`
  and `apps/desktop/SMOKE.md` TP-MAC-89. Validation passed:
  `npm test --workspace apps/desktop` (31/31),
  `npm run build --workspace apps/desktop`, touched-scope
  `git diff --check`, and desktop/mobile browser pixel smoke.
- Residual A6 scope remains: true directional deformed shape once
  displacement vectors exist, support reactions, stress/governing-ratio views,
  and richer result-selection coupling.
- No lifecycle state, release readiness, professional approval,
  certification, sealing, authentication, code-compliance claim, protected
  standards data, private project data, network path, telemetry path, or DAG
  authority changed.

## 2026-06-11 - TP-APP-R2-PIPEPICK-001 viewport pipe endpoint picking

- WORKING_ITEMS app-integration tranche added endpoint-pick controls to the
  viewport straight-pipe form. Arming `from` and selecting a rendered node
  fills the `from` endpoint and advances to `to`; selecting the second node
  fills `to` and clears pick mode.
- Endpoint picking supplies only existing node references. Pipe id, material,
  section geometry, non-zero `y_reference`, and provenance remain explicit
  user-entered inputs before Queue pipe can enable.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_viewport_pipe_endpoint_picking.md`
  and `apps/desktop/SMOKE.md` TP-MAC-94. Validation passed:
  `npm test --workspace apps/desktop` (35/35), `npm run build --workspace
  apps/desktop`, `npm run test:e2e:desktop` (1/1), and in-app browser
  endpoint-pick smoke; `git diff --check -- . ':!init/init-prompt.md'`
  passed.
- Residual A3 scope remains: broader canvas creation/edit gestures,
  component/rigid authoring, and broader editor coverage as new authoring
  surfaces land.
- No lifecycle state, release readiness, professional approval,
  certification, sealing, authentication, code-compliance claim, protected
  standards data, private project data, network path, telemetry path, or DAG
  authority changed.

## 2026-06-17 - Lifecycle Housekeeping

- Housekeeping lifecycle reset: `_STATUS.md` current state set to `IN_PROGRESS` to reflect current code development in progress. This does not change review, issuance, release readiness, professional approval, certification, sealing, authentication, or code-compliance status.

## 2026-06-17 - TP-UNITS-BTAIL-VIEWPORTGEOMUNITS-001 viewport geometry unit-validation evidence

- WORKING_ITEMS app-integration tranche threaded the existing DEC-018
  unit-catalog route in `PipeViewport` into explicit viewport geometry
  operation intents.
- Explicit create-node and connect-pipe queued intents now record
  `length=<unit-dimension-validation-status>` in
  `validation.unit_validation` instead of `not_run`; browser preview records
  `model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview`.
- Generic one-click viewport gesture placeholders were later covered by
  TP-MAC-225.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-VIEWPORTGEOMUNITS-001.md`
  and `apps/desktop/SMOKE.md` TP-MAC-209. Focused validation passed:
  `npm test --workspace apps/desktop -- App.test.tsx` (55/55) and
  `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts` (14/14).
  Full desktop validation passed: `npm test --workspace apps/desktop`
  (18/18 files, 398/398 tests) and `npm run build --workspace apps/desktop`
  with the existing Vite large-chunk warning.
- No lifecycle state, release readiness, professional approval,
  certification, sealing, authentication, code-compliance claim, protected
  standards data, private project data, network path, telemetry path, unit
  conversion API, or DAG authority changed.

## 2026-06-17 - TP-UNITS-BTAIL-VIEWPORTCOMPUNITVALID-001 viewport component-symbol unit-validation evidence

- WORKING_ITEMS app-integration tranche classified the reference-only
  viewport `insert_component_symbol` placeholder as
  `unit_validation=not_required_dimensionless`.
- Generic `create_node` and `connect_pipe_run` placeholder gestures were
  later covered by TP-MAC-225; explicit create-node and connect-pipe authoring
  paths already carry length unit-validation evidence.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-VIEWPORTCOMPUNITVALID-001.md`
  and `apps/desktop/SMOKE.md` TP-MAC-218. Focused validation passed:
  `npm test --workspace apps/desktop -- src/App.test.tsx -t "viewport editor intents"`
  (1/1) and `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts`
  (16/16). Full desktop validation passed: `npm test --workspace apps/desktop`
  (18/18 files, 399/399 tests) and `npm run build --workspace apps/desktop`
  with the existing Vite large-chunk warning.
- No lifecycle state, release readiness, professional approval,
  certification, sealing, authentication, code-compliance claim, protected
  standards data, private project data, network path, telemetry path, unit
  conversion API, operation application behavior, or DAG authority changed.

## 2026-06-17 - TP-UNITS-BTAIL-VIEWPORTPLACEHOLDERUNITS-001 viewport placeholder unit-validation evidence

- WORKING_ITEMS Phase B-tail tranche threaded the existing viewport unit
  route and model default length unit into generic one-click viewport gesture
  placeholders.
- Generic `create_node` and `connect_pipe_run` placeholders now record
  `unit=m`, `dimension=length`, and `unit_validation=length=<status>`
  instead of `unit_validation=not_run`. Browser preview records
  `length=model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview`.
- The placeholders remain review-only `pending_service_validation` intents;
  they still do not mutate accepted model state and still require structured
  application-service validation before durable model changes.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-VIEWPORTPLACEHOLDERUNITS-001.md`
  and `apps/desktop/SMOKE.md` TP-MAC-225; supporting DEL-02-02 run record
  and completion log entry are present.
- Validation passed: focused App Vitest 1/1 selected test, focused
  Playwright placeholder smoke 2/2, full desktop Vitest 399/399,
  single-worker R2/R3 Playwright smoke 18/18, and desktop production build
  with the existing Vite large-chunk warning.
- Boundary preserved: no accepted model-state mutation, operation-applier
  validation semantics, solver behavior, component schema, unit-conversion
  API, DEC-018 catalog constant, schema dimension enum, protected standards
  content, private data, lifecycle transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## 2026-08-21 - vocabulary round 3 component-tool completion

- Tee, reducer, valve, and flange viewport creation now use the same structured-operation seam as bend. Non-bend connectivity roles initialize and reset empty; a three-incident-node regression proves the exact deliberate selections are serialized and applied rather than inferred from collection order.
- Evidence is in `HELP-HUMAN-PIPING-20260821-VOCABULARY-R3/N1_UI_RETURN.md`, terminal `N1_REVIEW_V2.md`, and commit `8ca1984db45a9a8f6f3111a905b07c7d3da47c33`.
- Vocabulary row 15 is closed. DEL-07-01 remains IN_PROGRESS; no lifecycle or professional/release effect is created.

## 2026-09-05 — bounded workspace design implementation

- W7 completed persistent canvas, compact icon/task navigation, transient Toolkit, independently scrolling tool inputs, a bounded lower dock, and renderer resizing when panels change. Existing operation validation, exact batch application, session undo, unit semantics and engineering values are preserved.
- Source snapshot: `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-DESIGN-IMPLEMENTATION/instances/W7/snapshots/SOURCE_V2/`, manifest SHA256 `c53fd038fd753ea39049b37de98343d6427abb11446439fd1b41b95703eda5f2`. Selected evidence: `_run_records/WORKING_ITEMS_RUN_2026-09-05_WORKSPACE_DESIGN.md`; manager `RETURN.md` and current independent `children/R1/RETURN_V2.md`.
- Final source verification: 45 desktop files / 748 tests PASS; production build PASS with existing chunk warning; browser layout 2/2 PASS and 12 workflow states with zero page errors, including exact Wasm batch preview/application/one undo. Browser fixture mechanics and browser-memory storage are not native solve/SQLite evidence; root owns native and committed-head gates.
- The owner’s 1024 × 768 target is an initial client logical window/layout test target, not independent readability/usability acceptance. DEL-07-06 PDU-045/PDU-046 remain held under their existing requirements.
- Remaining was narrowed only to exclude this landed presentation slice. Broader routing/attachment gestures, full specialized property widgets, report MAP-031 inline human Add/Apply, modulus-basis/unit-picker/hardening work where already listed, analytical integration, durable history and D-58 remain under their existing owners and holds. Current Queue → Review → Apply behavior remains. Historical N7 records are preserved; this tranche does not disposition them.
- IN_PROGRESS lifecycle, ScopeOfWork, accepted scope, dependency authority, engineering/professional acceptance and independent validation status are unchanged.

## 2026-09-05 — browser path and diagnostic pointer repair supplement

- Current browser workflows now reach the redesigned disclosures, Toolkit and Review tabs through visible controls. Exact units/payloads, validation/application, selection, save/reopen, solve/results/report and browser limitations are preserved. Ordinary diagnostic clicks exposed and now verify repair of compressed Issues drawer panels; only a scoped non-shrinking direct-panel style changed product behavior.
- Source manifest SHA256 `cf2cc3df5746b76652974df700a78ea0f876ca232ab9835ecf019ec0d5d18daf` supersedes only the applicable live source binding; previous SOURCE_V2, FINAL_SNAPSHOT and run records remain immutable historical evidence. New local evidence: `_run_records/WORKING_ITEMS_RUN_2026-09-05_BROWSER_POINTER_REPAIR.md`; manager `E2E_REPAIR_RETURN.md` and R2 `RETURN_FINAL_V3.json`.
- Full source28, dist3, desktop748/45, build, targeteddiagnostic2 and exact1024x768 browser selection/linked-MPa/required-input-scroll/Close witness PASS. Preflight trace override is recorded; root must still run the unchanged complete clean committed DEC025 and refresh native evidence. Browser tests do not establish native GUI, solver/storage or owner usability acceptance.
- No further Remaining narrowing: this repair belongs to the bounded presentation slice already excluded from broad residuals. IN_PROGRESS, ScopeOfWork, engineering holds, MAP031 inline Apply, full typed widgets, D58 and PDU045/046 are unchanged.
