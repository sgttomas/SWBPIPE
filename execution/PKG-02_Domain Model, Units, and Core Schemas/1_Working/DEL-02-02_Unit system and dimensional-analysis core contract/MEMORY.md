# MEMORY - DEL-02-02

## 2026-06-17 - TP-UNITS-BTAIL-HANDOFFUNITWITNESS-001 supporting unit evidence

- Supporting role for DEL-15-01 handoff tranche: the desktop handoff package
  now carries DEC-018 unit-system disclosure and per-result
  value/unit/dimension preservation witnesses.
- Witnesses keep `conversion_performed=false` and cite the accepted
  `unit-system:dec-018-si-dual-display` basis; they do not introduce a unit
  conversion API or target-format support claim.
- Validation passed: focused App Vitest 55/55; focused R2 Playwright smoke
  2/2; full desktop Vitest 397/397; desktop production build with the existing
  Vite large-chunk warning.
- Boundary preserved: no DEC-018 catalog constant change, schema dimension
  enum change, unit conversion API change, protected standards content,
  private data, lifecycle transition, release-readiness claim, professional
  approval, certification, sealing, authentication, or code-compliance claim
  changed.

## 2026-06-17 - TP-UNITS-BTAIL-LOCALFEAUNITWITNESS-001 local FEA unit witnesses

- WORKING_ITEMS added explicit source result value/unit witnesses to the
  target-neutral local FEA handoff package for transfer-basis displacement,
  force, and moment refs.
- The witnesses preserve source units by reference and set
  `conversion_performed=false`; no new unit catalog constant, hidden
  conversion default, tolerance policy, or canonical-basis decision was added.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-LOCALFEAUNITWITNESS-001.md`
  and `apps/desktop/SMOKE.md` TP-MAC-192.
- Validation passed: local FEA schema pytest, focused local FEA/App Vitest
  58/58, full desktop Vitest 391/391, and desktop production build.
- Boundary unchanged: no protected standards content, private data,
  target-specific solver format, external solver invocation, lifecycle state
  transition, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim changed.

## 2026-06-17 - TP-UNITS-BTAIL-SUPPORTUNITS-001 support stiffness unit contract

- WORKING_ITEMS extended the structured operation applier support-create path
  to accept either the existing dimensionless restraint payload or a positive
  DEC-018-compatible `linear_stiffness` quantity.
- The change reuses the existing operation-applier quantity validation helpers
  and preserves entered units; it does not add a unit constant, schema enum,
  tolerance policy, or hidden conversion default.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-SUPPORTUNITS-001.md`
  and `apps/desktop/SMOKE.md` TP-MAC-191.
- Validation passed: `cargo fmt --manifest-path
  core/model_operations/operation_applier/Cargo.toml --check`, `cargo test
  --manifest-path core/model_operations/operation_applier/Cargo.toml`, focused
  App Vitest, full desktop Vitest 390/390, and desktop build.
- Boundary unchanged: no protected standards content, private data,
  network/telemetry path, lifecycle state transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## Session 2026-04-30 - DEV-001 Bounded Execution

Human gate:

- Authorized exactly one bounded DEV-001 dispatch for `DEL-02-02 - Unit system and dimensional-analysis core contract`.
- Broad fan-out, lifecycle transition, and candidate-edge promotion were not authorized.

Dispatch evidence:

- Sealed brief: `execution/_Coordination/DEV-001_DISPATCH_DEL-02-02.md`.
- Active upstream prerequisites consumed from `DAG-001`: `DEL-00-01`, `DEL-00-02`, `DEL-00-03`, `DEL-00-04`, `DEL-00-06`, `DEL-00-07`, `DEL-00-08`, and `DEL-02-01`.
- Candidate edges were not used.

Work completed:

- Extended `schemas/units.schema.yaml` with operation rules, test obligations, open decisions, and a shared unit diagnostic code definition.
- Updated `tests/test_units_schema.py` to assert operation-rule, gated conversion-test, open-decision, and diagnostic-code contract coverage.
- Updated `core/units/README.md` with operation rules, open decisions, and downstream test obligations.
- Added a dedicated unit-system section to `docs/SPEC.md`.

Open decisions preserved as `TBD`:

- Unit catalog and conversion source set.
- Base dimension vector and derived-dimension rules.
- Dimensionless classification, ratios, percentages, angle/rotation treatment, offset temperature, and gauge/absolute pressure semantics.
- Numeric representation, conversion tolerance policy, canonical calculation basis, schema file layout, diagnostic-code namespace, and human decision owner.

Guardrails:

- No lifecycle state transition was made.
- No `DAG-001`, candidate edge, blocker queue, `Dependencies.csv`, or `_DEPENDENCIES.md` mutation was made.
- No protected standards text, protected tables, proprietary values, private data, or compliance/certification/sealing claims were introduced.

## 2026-05-11 TP-RECON-01 Reconciliation

Reconciliation scope:

- TP-RECON-01 Wave 2 row for `DEL-02-02` allows writes only to this `MEMORY.md` and `_STATUS.md`.
- Current deliverable files and run records show the 2026-04-30 four-document, semantic, lens, and dependency-refresh setup runs remained deliverable-local and left unresolved unit catalog, conversion constants, dimensional basis, tolerance policy, schema layout, and special-quantity semantics as `TBD`.

Implementation evidence reconciled:

- Archived DEV-001 evidence records map `DEL-02-02` to committed bounded item `a458cba` (`schema: tighten unit system contract`), dated 2026-04-30, with handoff `ce94de3`.
- `git show --name-status a458cba` records the implemented artifact surface as `schemas/units.schema.yaml`, `tests/test_units_schema.py`, `core/units/README.md`, `docs/SPEC.md`, this deliverable `MEMORY.md`, the historical dispatch brief, and `execution/_Coordination/NEXT_INSTANCE_STATE.md`.
- The historical dispatch brief authorized exactly one bounded item and named the acceptance surface as unit-system schema coverage, operation rules, diagnostics, gated deterministic conversion tests, and documentation of remaining `TBD` decisions.

Downstream planning evidence reconciled:

- Revision 0.5 lifecycle and evidence snapshots carry `DEL-02-02` as `CHECKING` with `COMMITTED` evidence and present local status/context/dependency surfaces.
- SCA-002 and DEV-001 tranche records use `DEL-02-02` as committed unit-contract evidence for later constraint, comparison, tolerance, run-comparison, and product-assembly planning, including TP-MAC-01-C application-service bridge expectations for unit metadata and no-silent-default behavior.
- Those downstream records are planning or dependency evidence only; they do not add new `DEL-02-02` implementation scope in this reconciliation.

Preserved boundaries:

- No code, schema, fixture, test, specification, procedure, context, dependency, DAG, blocker, or coordination file was edited by this TP-RECON-01 worker.
- `CHECKING` is preserved; no issued/release/professional reliance state is asserted.
- Protected standards data, proprietary data, private project data, hidden engineering defaults, and final unit/conversion decisions remain out of scope.

## 2026-05-16 PKG-02 Foundation-Slice Hardening

Scope executed:

- Converted `tests/test_units_schema.py` from script-only assertions into
  pytest-collected contract and fixture tests while preserving direct
  `python3` execution.
- Added `fixtures/units/invented_unit_contract_fixture.json` as invented public
  evidence for dimensions, quantity records, operation rules, diagnostics, test
  obligations, and open decisions.

Evidence:

- Pytest now collects and passes `tests/test_units_schema.py`.
- Direct execution with `python3 tests/test_units_schema.py` passes.
- Fixture checks prove missing unit metadata remains a blocking diagnostic and
  conversion constants, tolerance policy, canonical calculation basis, offset
  temperature semantics, and gauge/absolute pressure semantics remain `TBD`.

Boundaries preserved:

- No runtime conversion engine or protected dimensional table added.
- Lifecycle remains `IN_PROGRESS`; no lifecycle transition was made.
- No dependency register, DAG, blocker queue, or candidate-edge edits.

## 2026-05-17 TP-PHYS-014-A Force-Per-Length Dimension Contract

Scope executed:

- Added canonical `force_per_length` to `schemas/units.schema.yaml`.
- Updated the shared canonical dimension vocabulary in `docs/SPEC.md`,
  `docs/TYPES.md`, and `tests/test_units_schema.py`.
- Coordinated with the DEL-02-01 model-schema change that uses
  `force_per_length` for `element_uniform_distributed_force` quantities.

Evidence:

- `python3 tests/test_model_schema.py` passed.
- `python3 tests/test_units_schema.py` passed.
- `python3 -m pytest tests/test_model_schema.py tests/test_units_schema.py`
  passed with 7 tests.

Boundary note:

- No unit catalog, conversion constants, runtime conversion engine, tolerance
  policy, or protected dimensional table was added.

## 2026-06-03 - TP-CODE-EVIDENCE-MIGRATION-RECONCILIATION-001 CHECKING transition
- Human approval accepted non-resolving DEV-001 evidence commits as migration-caused aberrations and approved lifecycle advancement to `CHECKING` for formal review.
- Evidence basis: `TP-CODE-EVIDENCE-AUDIT-001_2026-06-03` found current source/schema/fixture/test evidence and passing targeted/full-gate checks; `TP-CODE-EVIDENCE-MIGRATION-RECONCILIATION-001_2026-06-03` reconciled the migration-era commit-pointer gap.
- Local `_STATUS.md`, DEV-001 blocker queue lifecycle displays, and DAG-005 deliverable display surfaces were aligned to `CHECKING` where applicable.
- Boundary preserved: this is review-readiness only; no `ISSUED`, release-readiness, external compatibility, code-compliance, protected-IP/private-data, or professional-engineering authentication claim is made.

## 2026-06-04 - TP-AUTHORITY-REFRESH-0_7-DAG006

- WORKING_ITEMS/TASK workforce current-authority refresh applied to active deliverable-local surfaces for `DEL-02-02`.
- Current authority basis is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7` plus approved `execution/_DAG/DAG-006/` active graph authority.
- Historical run records, historical DAG row IDs, review dispositions, lifecycle `_STATUS.md`, aggregate DAG artifacts, candidate edges, repo-level governance files, schemas, code, and tools were intentionally not changed by this refresh.
- Preserved historical references remain evidence of earlier work, not current authority claims.

## 2026-06-12 - TP-UNITS-B1-CATALOG-001 Unit catalog and conversion crate

- Implemented the Phase B1 crate-side unit catalog in `core/units` after the D-01 human ruling recorded as `DEC-018`.
- Added `open_pipe_stress_units` with canonical dimension identifiers, exponent-vector algebra, SI-canonical units, common display units, exact public definitional conversion constants, affine absolute-temperature conversion, interval-temperature conversion, and explicit gauge/absolute pressure conversion that requires caller-supplied reference provenance when pressure kind changes.
- Tightened the quantity-kind conversion API during fan-in so temperature and pressure cannot use generic `UnitBearing` conversion to bypass explicit DEC-018 semantics.
- Updated `core/units/README.md` from its pre-DEC-018 `TBD` posture to record the accepted B1 basis and preserve B2/B3 handoffs.
- Evidence: `core/units/_run_records/TASK_RUN_2026-06-12_0136.md` and `_run_records/WORKING_ITEMS_RUN_2026-06-12_unit_catalog_conversion_crate.md`.
- Validation: `cargo fmt --manifest-path core/units/Cargo.toml --check` passed; `cargo test --manifest-path core/units/Cargo.toml` passed with 11 unit tests and 0 doctests; `python3 tests/test_units_schema.py` passed; `python3 -m pytest tests/test_units_schema.py` passed with 3 tests.
- Boundaries preserved: no schema/app/solver/report unit I/O retrofit, no protected standards content, no proprietary vendor data, no private project data, no bundled engineering defaults, and no release-readiness, professional approval, certification, sealing, authentication, or code-compliance claim.
- Residual handoffs: B2 must bind unit-aware I/O through schemas, desktop fields, solver normalization, reports, imports/exports, and rule-pack evaluation; B3 must add mixed-unit round-trip and conversion-witness/tolerance corpus evidence under DEC-026.

## 2026-06-12 - TP-UNITS-B2B3-CONTRACT-001 Schema/crate contract rider

- Added crate-visible conversion metadata to `UnitDefinition`: `factor_representation`, optional `offset_representation`, explicit `ConversionProvenance`, and `ReviewStatus`.
- Replaced the previous binary provenance derivation with explicit provenance assignment, including `ConventionalPublicConstant` for lbf/psi-family conversions and `ProjectGovernedDecision` for project-governed semantic canonical bindings.
- Added the `DIMENSIONS` crate vocabulary constant and a Rust schema-parity regression that parses `schemas/units.schema.yaml` and asserts set equality against the schema `DimensionId` enum.
- Evidence: `_run_records/WORKING_ITEMS_RUN_2026-06-12_units_schema_crate_contract.md`.
- Validation: `cargo fmt --manifest-path core/units/Cargo.toml --check` passed; `cargo test --manifest-path core/units/Cargo.toml` passed with 13 unit tests and 0 doctests; `python3 tests/test_units_schema.py` passed; `python3 -m pytest tests/test_units_schema.py` passed with 3 tests.
- Boundaries preserved: no full B2 desktop unit picker/display retrofit, solver-boundary normalization, report rendering change, import/export change, rule-pack unit evaluator change, protected-content ingestion, private-data ingestion, professional approval, certification, sealing, authentication, release-readiness, or code-compliance claim.
- Residual handoffs: B2 still owns app/solver/report/import/export/rule-pack unit I/O; B3 still owns the broader mixed-unit round-trip, conversion-witness, incompatible-unit rejection, and D-04/DEC-026 tolerance corpus.

## 2026-06-12 - TP-UNITS-B2-CATALOGCMD-001 Desktop unit catalog binding

- Added stable unit catalog IDs and stable string values for transform kind, conversion provenance, and review status in `core/units`.
- Added the desktop backend `get_unit_catalog` command, backed by `open_pipe_stress_units`, returning unit id, symbol, dimension id, canonical flag, transform kind, factor representation, optional offset representation, provenance, review status, and boundary flags.
- Added Tauri coverage proving the command preserves `DEC-018`, entered-unit preservation, inch factor text, Fahrenheit offset text, lbf conventional provenance, project-governed semantic binding, and no protected/private/professional/code-compliance claim.
- Evidence: `_run_records/WORKING_ITEMS_RUN_2026-06-12_desktop_unit_catalog_binding.md`; `apps/desktop/SMOKE.md` TP-MAC-129.
- Validation: `cargo fmt --manifest-path core/units/Cargo.toml --check` passed; `cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml --check` passed; `cargo test --manifest-path core/units/Cargo.toml` passed with 13 unit tests and 0 doctests; focused Tauri command test passed 1/1; full `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with 32 unit tests and 0 doctests; `python3 tests/test_units_schema.py` passed.
- Boundaries preserved: no visible desktop unit picker/display replacement, bulk form retrofit, solver-boundary normalization, report renderer change, import/export change, rule-pack evaluator change, protected-content ingestion, private-data ingestion, professional approval, certification, sealing, authentication, release-readiness, or code-compliance claim.
- Residual handoffs: B2 still owns visible app unit fields, solver-boundary normalization, report unit disclosure, imports/exports, and rule-pack unit I/O; B3 still owns broader conversion witness and tolerance corpus coverage.

## 2026-06-12 - TP-UNITS-B2-FRONTENDSVC-001 Frontend unit catalog service

- Added `apps/desktop/src/services/unitCatalogService.ts` with typed unit-catalog payloads, `loadUnitCatalog`, and `acceptedUnits`.
- Browser preview mode returns explicit `UNIT-CATALOG-DESKTOP-ONLY` unavailability instead of synthesizing a fallback catalog; desktop mode invokes Tauri `get_unit_catalog`.
- Added Vitest coverage for browser unavailability, Tauri invocation, DEC-018 metadata, factor/offset/provenance fields, and boundary flags.
- Evidence: `_run_records/WORKING_ITEMS_RUN_2026-06-12_frontend_unit_catalog_service.md`; `apps/desktop/SMOKE.md` TP-MAC-130.
- Validation: `npm test --workspace apps/desktop -- unitCatalogService` passed with 2 tests; `npm test --workspace apps/desktop` passed with 215 tests across 9 files; `npm run build --workspace apps/desktop` passed with the pre-existing Vite chunk-size warning.
- Boundaries preserved: no visible unit picker/display retrofit, solver-boundary normalization, report renderer change, import/export change, rule-pack evaluator change, browser fallback unit catalog, protected-content ingestion, private-data ingestion, professional approval, certification, sealing, authentication, release-readiness, or code-compliance claim.
- Residual handoffs: B2 still owns visible app unit fields, solver-boundary normalization, report unit disclosure, imports/exports, and rule-pack unit I/O; B3 still owns broader conversion witness and tolerance corpus coverage.

## 2026-06-16 - TP-UNITS-B2B3-PCFCONVWITNESS-001 PCF conversion witnesses

- Added a desktop PCF export conversion-witness package member for DEC-018
  source-to-target length conversions already performed by the conservative PCF
  preview.
- Witnesses record source value/unit/dimension, target PCF millimeter
  value/unit/field, conversion factor, DEC-018/DEL-02-02 basis refs, and
  preview provenance for node coordinate and pipe OD/wall fields.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2B3-PCFCONVWITNESS-001.md`;
  corresponding DEL-17-07 run record; `apps/desktop/SMOKE.md` TP-MAC-172.
- Validation: focused App Vitest 54/54; full desktop Vitest 386/386; desktop
  build passed with the existing Vite chunk-size warning; focused Playwright
  R2 smoke 2/2.
- Boundaries preserved: no unit catalog change, no runtime conversion-engine
  change, no schema-owned Python export contract change, no protected-content
  ingestion, no private-data ingestion, and no professional approval,
  certification, sealing, authentication, release-readiness, target
  compatibility, or code-compliance claim.

## 2026-06-12 - TP-UNITS-B2-INSPECTORLABELS-001 Property Inspector unit basis labels

- Added catalog-aware unit display helpers to the frontend unit-catalog
  service: `unitCatalogEntryForSymbol` and `describeUnitBasis`.
- The helpers match DEC-018 catalog entries by symbol/dimension, support
  display-only equivalents needed by current app fields such as `stress`
  through pressure units, and report browser-preview unavailability or catalog
  misses explicitly.
- Bound the Property Inspector material/section creation labels to these
  helpers and added a visible `Unit basis` status panel. Browser preview shows
  model metadata only; desktop/Tauri mode can show DEC-018 catalog basis when
  the backend command is present.
- Evidence: `_run_records/WORKING_ITEMS_RUN_2026-06-12_property_inspector_unit_basis_labels.md`;
  corresponding DEL-07-02 run record; `apps/desktop/SMOKE.md` TP-MAC-131.
- Validation: focused unit-catalog/App Vitest 48/48; Playwright R2 smoke 2/2
  after wasm engine build; full desktop Vitest 216/216; desktop build passed.
- Boundaries preserved: no unit picker, conversion, solver-boundary
  normalization, report unit-system disclosure, import/export unit handling,
  rule-pack unit I/O, fallback browser catalog, protected-content ingestion,
  private-data ingestion, professional approval, certification, sealing,
  authentication, release-readiness, or code-compliance claim.
- Residual handoffs: B2 still owns broader visible unit entry/pickers,
  solver-boundary normalization, report unit-system disclosure,
  imports/exports, and rule-pack unit I/O; B3 still owns broader conversion
  witness and tolerance corpus coverage.

## 2026-06-12 - TP-UNITS-B2-REPORTUNITS-001 report unit-system disclosure

- Added the accepted DEC-018 unit-system reference
  `unit-system:dec-018-si-dual-display` to the rendered-report adapter's
  existing `model_input_summary.unit_system_ref`.
- Added Report Packet `unit_system_disclosure` with model units, distinct
  result-row units, entered-unit preservation posture, and
  `conversion_performed=false`.
- Added a visible Report Packet `Unit system` line so users can see the unit
  system reference and units used before exporting JSON.
- Evidence: `_run_records/WORKING_ITEMS_RUN_2026-06-12_report_unit_system_disclosure.md`;
  corresponding DEL-08-01 run record; `apps/desktop/SMOKE.md` TP-MAC-132.
- Validation: focused report/App Vitest 53/53; Playwright R2 smoke 2/2 after
  wasm engine build; full desktop Vitest 216/216; desktop build passed.
- Boundaries preserved: no unit conversion, solver-boundary normalization,
  unit picker, import/export unit conversion, rule-pack unit I/O, report
  schema expansion, fallback browser catalog, protected-content ingestion,
  private-data ingestion, professional approval, certification, sealing,
  authentication, release-readiness, or code-compliance claim.
- Residual handoffs: B2 still owns broader visible unit entry/pickers,
  solver-boundary normalization, report renderer body expansion beyond packet
  disclosure, imports/exports, and rule-pack unit I/O; B3 still owns broader
  conversion witness and tolerance corpus coverage.

## 2026-06-12 - TP-UNITS-B2-SOLVERNORM-001 solver-boundary unit normalization

- `core/product_physics` now depends on `core/units` and uses DEC-018 catalog
  dimension compatibility at the preview mechanics boundary.
- Compatible mixed-unit inputs are normalized to SI-canonical solver values
  before assembly: regression coverage includes material moduli in `MPa`,
  pipe dimensions in `mm`, and pressure loads in `kPa` solving to the same
  rounded result surface as the SI fixture.
- Incompatible unit dimensions remain blocking through `UNIT_INPUT_INVALID`;
  unexpected normalization failures use `UNIT_CONVERSION_UNAVAILABLE`.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-12_solver_boundary_unit_normalization.md`
  and corresponding DEL-07-07 run record; `apps/desktop/SMOKE.md` TP-MAC-133.
- Validation: product_physics cargo tests 25/25; Tauri Rust tests 32/32;
  headless runner cargo tests 11/11; focused desktop Vitest 56/56; Playwright
  R2 smoke 2/2; full desktop Vitest 216/216; desktop build passed.
- Boundaries preserved: no visible unit picker controls, report-time
  conversion, import/export unit conversion, rule-pack unit I/O, protected
  standards content, private data, release-readiness, professional approval,
  certification, sealing, authentication, or code-compliance claim.
- Residual handoffs: B2 still owns broader visible unit entry/pickers, report
  renderer body expansion beyond packet disclosure, imports/exports, and
  rule-pack unit I/O; B3 still owns broader conversion witness and tolerance
  corpus coverage.

## 2026-06-12 - TP-UNITS-B2-UNITPICKERS-001 Property Inspector unit selectors

- Added visible unit selectors to Property Inspector material and section
  create forms, backed by accepted DEC-018 catalog options in desktop mode and
  explicit model-metadata-only behavior in browser preview.
- `core/model_operations/operation_applier` now validates compatible
  create-section length units and create-material stress / thermal-expansion
  units through `core/units`, preserving entered units in the applied session
  model and blocking incompatible dimensions.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-12_property_inspector_unit_selectors.md`;
  corresponding DEL-07-02, DEL-16-02, and DEL-16-03 run records;
  `apps/desktop/SMOKE.md` TP-MAC-134.
- Validation: operation-applier cargo suites passed; Tauri Rust tests 32/32;
  focused desktop Vitest 165/165; full desktop Vitest 216/216; desktop build
  passed; Playwright R2 smoke 2/2.
- Residual handoffs: B2 still owns broader app unit entry/pickers outside
  material/section create forms, report renderer body expansion beyond packet
  disclosure, imports/exports, and rule-pack unit I/O.

## 2026-06-12 - TP-UNITS-B2-REPORTBODY-001 rendered report body unit disclosure

- Added optional `unit_display_summary` to the report-generator
  `ModelInputSummary` contract and `schemas/report_generator.schema.yaml`.
  The summary records entered-unit storage convention, sorted model unit map,
  distinct result-row units, report display policy, and
  `conversion_performed=false`.
- The frontend report adapter now populates the same unit-display summary used
  by the Report Packet disclosure, preventing packet/body vocabulary drift.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-12_report_body_unit_disclosure.md`;
  corresponding DEL-08-01 run record; `apps/desktop/SMOKE.md` TP-MAC-135.
- Validation: report-generator schema contract test passed; report-generator
  cargo tests 10/10; report-renderer cargo tests 8/8; Tauri Rust tests 32/32;
  focused report/App Vitest 53/53; full desktop Vitest 216/216; desktop build
  passed; Playwright R2 smoke 2/2.
- Boundaries preserved: no report-time unit conversion, project-wide
  unit-system picker, import/export unit conversion, rule-pack unit I/O,
  protected standards content, private project data, release-readiness,
  professional approval, certification, sealing, authentication, or
  code-compliance claim.
- Residual handoffs: B2 still owns broader app unit entry/pickers outside
  material/section create forms, imports/exports, and rule-pack unit I/O; B3
  still owns broader conversion witness and tolerance corpus coverage.

## 2026-06-12 - TP-UNITS-B2-LOADPICKERS-001 primitive-load unit selectors

- Added DEC-018-backed unit option filtering for Load Cases manager
  primitive-load creation by exposing `unitEntryMatchesDimension` from the
  frontend unit-catalog service.
- The primitive-load creation form now has a visible `Magnitude unit` selector
  and labels the magnitude field with the selected unit basis. Browser preview
  remains model-metadata-only; desktop/Tauri mode can use accepted catalog
  entries for the selected primitive-load dimension.
- `core/model_operations/operation_applier` now validates compatible entered
  units for created primitive loads and preserves those units in the applied
  session model.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-12_primitive_load_unit_selectors.md`;
  corresponding DEL-05-05, DEL-07-02, and DEL-16-02 run records;
  `apps/desktop/SMOKE.md` TP-MAC-136.
- Validation: operation-applier cargo suites passed 52 unit tests plus
  canonical-hash and contract-corpus tests; Tauri Rust tests 32/32; focused
  desktop Vitest 165/165; full desktop Vitest 216/216; desktop build passed;
  Playwright R2 smoke 2/2.
- Boundaries preserved: no report-time unit conversion, project-wide
  unit-system picker, existing primitive-load magnitude-edit unit handling,
  import/export unit conversion, rule-pack unit I/O, protected standards
  content, private project data, release-readiness, professional approval,
  certification, sealing, authentication, or code-compliance claim.
- Residual handoffs: B2 still owns broader app unit entry/pickers outside
  material/section/primitive-load create forms, imports/exports, rule-pack
  unit I/O, and existing primitive-load magnitude-edit unit handling; B3 still
  owns broader conversion witness and tolerance corpus coverage.

## 2026-06-12 - TP-UNITS-B2-PRIMEDITUNITS-001 primitive-load magnitude unit edits

- Extended the selected primitive-load magnitude edit path from value-only
  updates to atomic value+unit payloads.
- The Load Cases manager now exposes a `Magnitude unit` selector for existing
  primitive loads and labels the value field with the active unit basis.
- `core/model_operations/operation_applier` preserves legacy numeric-string
  primitive magnitude edits and also accepts `{ value, unit }` payloads that
  update the sibling `.unit` field when the entered unit matches the declared
  primitive-load dimension.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-12_primitive_load_magnitude_unit_edits.md`;
  corresponding DEL-05-05, DEL-07-02, and DEL-16-02 run records;
  `apps/desktop/SMOKE.md` TP-MAC-137.
- Validation: operation-applier cargo suites passed 53 unit tests plus
  canonical-hash and contract-corpus tests; Tauri Rust tests 32/32; focused
  desktop Vitest 165/165; full desktop Vitest 216/216; desktop build passed;
  Playwright R2 smoke 2/2.
- Boundaries preserved: no report-time unit conversion, project-wide
  unit-system picker, import/export unit conversion, rule-pack unit I/O,
  protected standards content, private project data, release-readiness,
  professional approval, certification, sealing, authentication, or
  code-compliance claim.
- Residual handoffs: B2 still owns broader app unit entry/pickers outside
  material/section/primitive-load create and edit forms, imports/exports, and
  rule-pack unit I/O; B3 still owns broader conversion witness and tolerance
  corpus coverage.

## 2026-06-12 - TP-UNITS-B2-INSPECTOREDITUNITS-001 Property Inspector quantity unit edits

- Extended B2 app unit I/O from creation forms and primitive-load edits into
  existing material and pipe-section quantity edits in the Property Inspector.
- Material modulus, material thermal-expansion, pipe outside-diameter, and pipe
  wall-thickness edits now carry an explicit selected unit beside the proposed
  value; node coordinate edits remain value-only because project-unit mutation
  is outside this tranche.
- `core/model_operations/operation_applier` now accepts generic sibling-unit
  `{ value, unit }` quantity-edit payloads and writes the sibling `.unit` field
  only after validating the entered unit against the declared dimension.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-12_property_inspector_quantity_unit_edits.md`;
  corresponding DEL-03-01, DEL-03-02, DEL-07-02, and DEL-16-02 run records;
  `apps/desktop/SMOKE.md` TP-MAC-138.
- Validation: operation-applier cargo suites passed 54 unit tests plus
  canonical-hash and contract-corpus tests; Tauri Rust tests 32/32; focused
  desktop Vitest 165/165; full desktop Vitest 216/216; desktop build passed;
  Playwright R2 smoke 2/2.
- Boundaries preserved: no project-wide unit-system picker, project unit
  mutation, report-time unit conversion, import/export unit conversion,
  rule-pack unit I/O, protected standards content, private project data,
  release-readiness, professional approval, certification, sealing,
  authentication, or code-compliance claim.
- Residual handoffs: B2 still owns broader app unit entry/pickers outside
  material/section/primitive-load create and edit forms, imports/exports, and
  rule-pack unit I/O; B3 still owns broader conversion witness and tolerance
  corpus coverage.

## 2026-06-12 - TP-UNITS-B2-EXPORTDISCLOSURE-001 export unit-system disclosure

- Extended B2 unit I/O into export-package contracts for PCF, CAEPIPE MBF,
  and stress-neutral packages.
- Desktop export JSON now carries `unit_system_disclosure` with DEC-018 unit
  system ref, entered-unit storage convention, source model units, target
  export units, result units where present, conversion policy/scope, and
  protected/private-content false flags.
- Strict export schemas and Python package builders now require and checksum
  `unit_system_disclosure.json` as a manifest package member.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-12_export_unit_disclosure.md`;
  corresponding DEL-17-04, DEL-17-06, and DEL-17-07 run records;
  `apps/desktop/SMOKE.md` TP-MAC-139.
- Validation: focused export-package tests passed 32/32; repository Python
  tests passed 356/356; full desktop Vitest 216/216; desktop build passed;
  Playwright R2 smoke 2/2.
- Boundaries preserved: no import round-trip claim, target compatibility
  claim, solver-deck validation claim, rule-pack unit I/O, protected standards
  content, private project data, release-readiness, professional approval,
  certification, sealing, authentication, or code-compliance claim.
- Residual handoffs: B2 still owns import round-trip unit I/O, target-format
  conversion witnesses beyond disclosure, broader app unit entry/pickers
  outside covered forms, and rule-pack unit I/O; B3 still owns broader
  conversion witness and tolerance corpus coverage.

## 2026-06-15 - TP-UNITS-B2-RULEPACKUNITS-001 rule-pack declaration unit selectors

- Extended B2 unit-aware I/O into the C2 rule-pack declarations form-builder:
  `required_inputs[].quantity_intent.unit_ref` and
  `value_slots[].quantity_intent.unit_ref` now use DEC-018-backed desktop unit
  selectors when Tauri `get_unit_catalog` is available.
- Browser preview keeps manual unit text entry and does not synthesize a
  fallback unit catalog; desktop selectors filter by declaration dimension,
  preserve out-of-catalog stored units, and avoid silent dimension/unit
  rewrites.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-15_TP-UNITS-B2-RULEPACKUNITS-001.md`;
  coupled DEL-07-03 memory addendum; `apps/desktop/SMOKE.md` TP-MAC-168.
- Validation: focused `DeclarationsEditor` Vitest 29/29; full desktop Vitest
  381/381; desktop build passed with the existing Vite chunk-size warning;
  Playwright e2e 10/10.
- Boundaries preserved: frontend-only; no schema/backend/evaluator/grammar
  change, protected standards content, private value embedding, browser
  fallback catalog, release-readiness, professional approval, certification,
  sealing, authentication, or code-compliance claim.
- Residual handoffs: B2 still owns rule-pack expression literal/table unit I/O
  and broader import/export conversion work outside covered forms; B3 still
  owns broader conversion-witness, incompatible-unit rejection, and DEC-026
  tolerance corpus coverage.

## 2026-06-15 - TP-UNITS-B2-RULEEXPRUNITS-001 rule-pack expression unit selectors

- Extended B2 unit-aware I/O into the C2 expression composer: literal
  `quantity.unit_ref`, table `argument_unit_ref`, and table `result_unit_ref`
  now use DEC-018-backed desktop unit selectors when Tauri `get_unit_catalog`
  is available.
- Browser preview keeps manual unit text entry and does not synthesize a
  fallback unit catalog; desktop selectors filter by authored expression/table
  dimension, preserve out-of-catalog stored units, and avoid silent
  dimension/unit rewrites.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-15_TP-UNITS-B2-RULEEXPRUNITS-001.md`;
  corresponding DEL-06-02 run record; `apps/desktop/SMOKE.md` TP-MAC-169.
- Validation: focused `ExpressionComposer` Vitest 19/19; full desktop Vitest
  384/384; desktop build passed with the existing Vite chunk-size warning;
  Playwright e2e 10/10.
- Boundaries preserved: frontend-only; no schema/backend/evaluator/grammar
  change, protected standards content, private value embedding, browser
  fallback catalog, release-readiness, professional approval, certification,
  sealing, authentication, or code-compliance claim.
- Residual handoffs: B2 still owns broader app unit entry/pickers outside
  material/section/primitive-load create/edit and rule-pack
  declaration/expression forms, import round-trip unit I/O, and target-format
  conversion witnesses beyond disclosure; B3 still owns broader
  conversion-witness, incompatible-unit rejection, and DEC-026 tolerance corpus
  coverage.

## 2026-06-16 - TP-UNITS-B2-LOADINSPECTORUNITS-001 load-case inspector primitive unit payloads

- Extended B2 app unit I/O into the Property Inspector load-case path:
  `primitive_loads.0.magnitude.value` now uses the unit-aware editor control
  and queues an explicit `{value, unit}` after-value payload.
- This is a frontend binding to the already-landed primitive-load magnitude
  operation seam; no units crate, schema, backend operation, solver,
  import/export, report, or rule-pack behavior changed.
- Evidence:
  `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2-LOADINSPECTORUNITS-001.md`;
  SMOKE TP-MAC-171; focused App Vitest 54/54; full desktop Vitest 386/386;
  desktop build passed; focused Playwright R2 smoke 2/2.
- Boundary unchanged: browser preview uses model metadata only and does not
  synthesize a DEC-018 fallback catalog; no protected content, private data,
  network/telemetry path, release-readiness claim, or professional/code-
  compliance claim changed.

## 2026-06-16 - TP-UNITS-B2-IMPORTRT-001 local project unit round-trip evidence

- Added a B2 import/open/save unit I/O witness: local project create/save/open
  summaries now expose a deterministic unit metadata signature for restored
  local project envelopes.
- The evidence includes unit refs from `project.units`, materials, sections,
  pipe segment section quantities, and primitive-load magnitudes. Project
  Storage Audit and Project Validation Preflight display/export the same
  `unit_round_trip_status`, `unit_round_trip_checked_ref_count`, and
  `unit_round_trip_signature` fields.
- Evidence:
  `execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-05_Project persistence and round-trip serialization/_run_records/WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2-IMPORTRT-001.md`;
  supporting run record in this deliverable; SMOKE TP-MAC-173; completion log
  entry.
- Validation passed: focused project service/App Vitest 61/61; full desktop
  Vitest 386/386; desktop build; rustfmt package check; focused Tauri store
  regression; in-app Browser visible-row verification. Initial direct
  Playwright execution found a missing local Chromium cache and browser-install
  CDN timeouts; the later DEC-025 sweep ran the updated Playwright smoke
  successfully (10/10 dev-server lane plus 1/1 production-dist lane).
- Boundary unchanged: no unit conversion, import target compatibility, solver
  behavior, protected content, private data, network/telemetry path,
  release-readiness claim, or professional/code-compliance claim changed.

## 2026-06-16 - TP-UNITS-B2B3-CAEPIPECONVWITNESS-001 CAEPIPE MBF conversion witnesses

- Added supporting B2/B3 unit evidence for desktop CAEPIPE MBF node-coordinate
  source-to-target conversion witnesses.
- Each witness records DEC-018/DEL-02-02 basis refs, source node coordinate
  value/unit/dimension, target MBF payload value/unit (`mm`), and conversion
  factor. The invented preview witness for `node:N-120` records
  `3.2 m -> 3200 mm`.
- Evidence:
  `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-04_CAEPIPE MBF export profile and deterministic writer/_run_records/WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2B3-CAEPIPECONVWITNESS-001.md`;
  supporting run record in this deliverable; SMOKE TP-MAC-174; completion log
  entry.
- Validation passed: focused `App.test.tsx` 54/54; full desktop Vitest
  386/386; desktop build with existing Vite chunk-size warning; targeted R2
  Playwright smoke spec 10/10 after wasm rebuild.
- Boundary unchanged: no unit catalog, conversion constant, schema, runtime
  solver, Python export-package contract, protected content, private data,
  target compatibility, release-readiness claim, or professional/code-
  compliance claim changed.

## 2026-06-16 - TP-UNITS-B2B3-STRESSNEUTRALUNITWITNESS-001 stress-neutral unit preservation witnesses

- Added supporting B2/B3 unit evidence for desktop stress-neutral per-row unit
  preservation witnesses.
- Each witness records DEC-018/DEL-02-02 basis refs, source result
  value/unit/dimension, target row value/unit/dimension, and
  `conversion_performed=false`. The package remains a unit-preserving review
  format, not a target conversion path.
- Evidence:
  `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-06_Stress-neutral CSV JSON package/_run_records/WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2B3-STRESSNEUTRALUNITWITNESS-001.md`;
  supporting run record in this deliverable; SMOKE TP-MAC-175; completion log
  entry.
- Validation passed: focused `App.test.tsx` 54/54; full desktop Vitest
  386/386; desktop build with existing Vite chunk-size warning; targeted R2
  Playwright smoke spec 10/10 after wasm rebuild.
- Boundary unchanged: no unit catalog, conversion constant, schema, runtime
  solver, Python export-package contract, protected content, private data,
  target compatibility, release-readiness claim, or professional/code-
  compliance claim changed.

## 2026-06-16 - TP-UNITS-B2B3-RULECHECKNORM-001 rule-check mixed-unit normalization

- Added supporting B2/B3 unit evidence for rule-pack checks: compatible
  DEC-018 units are normalized at the `core/rules/rule_check_runner` boundary
  to the rule-pack declaration unit before formula evaluation and
  acceptability comparison.
- New witnesses cover `MPa`/`kPa` stress values normalized to declared `Pa`
  and an incompatible `mm` stress input blocked with `UnitMismatch` /
  `RULE_INPUTS_INCOMPLETE`.
- Evidence:
  `execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/DEL-06-02_Sandboxed unit-aware expression evaluator/_run_records/WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2B3-RULECHECKNORM-001.md`;
  supporting run record in this deliverable; SMOKE TP-MAC-176; completion log
  entry.
- Validation passed: rule-check runner cargo tests (13 unit + 7 integration);
  desktop Tauri cargo tests 62/62; desktop Vitest 386/386; desktop build with
  existing Vite chunk-size warning.
- Boundary unchanged: no unit catalog constant, grammar, schema, parser,
  protected content, private data, release-readiness claim, or professional/
  code-compliance claim changed.

## 2026-06-16 - TP-UNITS-B2B3-REVIEWGEOMUNITWITNESS-001 review-geometry unit witnesses

- Added supporting B2/B3 unit evidence for the DEL-17-08 review-geometry glTF
  JSON preview packet.
- The packet now records `unit-system:dec-018-si-dual-display`, source model
  units, target coordinate unit `m`, `conversion_performed=false`, and the
  glTF +Y-up axis transform policy.
- The witness set covers 54 emitted coordinate components for pipe endpoints,
  node markers, support markers, and component markers in the invented preview
  fixture.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2B3-REVIEWGEOMUNITWITNESS-001.md`;
  primary DEL-17-08 run record; SMOKE TP-MAC-178; completion log entry.
- Validation passed: focused App Vitest 54/54; full desktop Vitest 386/386;
  desktop build with existing Vite chunk-size warning; final full desktop
  Playwright 10/10; DEC-025 sweep pass.
- Boundary unchanged: no unit catalog constant, conversion API, tolerance
  policy, schema dimension enum, protected content, private data,
  release-readiness claim, or professional/code-compliance claim changed.

## 2026-06-16 - TP-UNITS-B2B3-NATIVEUNITWITNESS-001 native JSON unit preservation witnesses

- Added supporting B2/B3 unit evidence for the desktop native JSON package
  review packet.
- The packet records the accepted
  `unit-system:dec-018-si-dual-display` basis and preserves project/model/
  result value+unit fields without conversion.
- The witness set covers 6 project unit declarations, 18 model quantity
  witnesses, and 739 result quantity witnesses in the invented preview
  fixture.
- Evidence:
  `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-03_Native open JSON export package/_run_records/WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2B3-NATIVEUNITWITNESS-001.md`;
  supporting run record in this deliverable; SMOKE TP-MAC-179; completion log
  entry.
- Validation passed: focused App Vitest 54/54; full desktop Vitest 386/386;
  desktop build with existing Vite chunk-size warning; full desktop
  Playwright 10/10; in-app Browser solve/export verification pass; DEC-025
  dirty-tree sweep pass.
- Boundary unchanged: no DEC-018 catalog constant, unit-conversion API,
  tolerance policy, schema dimension enum, protected content, private data,
  release-readiness claim, or professional/code-compliance claim changed.

## 2026-06-16 - TP-UNITS-B2-VIEWPORTDRAFTUNITS-001 viewport draft length-unit controls

- Added supporting B2/B3 unit evidence for viewport draft node and pipe
  creation forms.
- The viewport forms expose selected length units and visible basis text;
  browser preview records the model-metadata fallback, while Tauri-capable
  runs can use accepted DEC-018 length catalog entries.
- The operation seam validates compatible DEC-018 length units, normalizes
  node coordinate values back to `project.units.length`, and preserves entered
  pipe section units after dimension-compatible validation.
- Evidence:
  `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2-VIEWPORTDRAFTUNITS-001.md`;
  supporting run records in DEL-16-02 and this deliverable; SMOKE TP-MAC-181;
  completion log entry.
- Validation passed: operation-applier cargo focused/full suites, desktop
  Vitest focused/full suites, desktop build, focused/full Playwright, and
  in-app Browser viewport verification; DEC-025 dirty-tree sweep passed.
- Boundary unchanged: no DEC-018 catalog constant, schema dimension enum,
  tolerance policy, project-unit mutation, hidden unit fallback, protected
  content, private data, release-readiness claim, or professional/code-
  compliance claim changed.

## 2026-06-16 - TP-UNITS-B2-RULECHECKRUNUNITS-001 run-check runtime unit controls

- Added supporting B2/B3 unit evidence for C4 run-check runtime value binding
  controls.
- The GUI now presents DEC-018-backed desktop unit selectors for user-supplied
  value and value-slot bindings, filtered by declared dimension, with browser
  preview retaining explicit manual stored-unit text entry.
- The unit catalog is consumed as metadata for controls only; no catalog
  constant, schema dimension enum, tolerance policy, or evaluator behavior was
  changed.
- Evidence:
  `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-04_Missing-data warning and blocking UX/_run_records/WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2-RULECHECKRUNUNITS-001.md`;
  supporting run records in DEL-06-02 and this deliverable; SMOKE TP-MAC-182;
  completion log entry.
- Validation passed: focused RuleCheckRunPanel Vitest, full desktop Vitest,
  desktop build, focused/full Playwright, in-app Browser verification, and
  DEC-025 dirty-tree sweep.
- Boundary unchanged: no DEC-018 catalog constant, schema dimension enum,
  tolerance policy, hidden unit fallback, protected content, private data,
  release-readiness claim, or professional/code-compliance claim changed.

## 2026-06-17 - Lifecycle Housekeeping

- Housekeeping lifecycle reset: `_STATUS.md` current state set to `IN_PROGRESS` to reflect current code development in progress. This does not change review, issuance, release readiness, professional approval, certification, sealing, authentication, or code-compliance status.

## 2026-06-17 - TP-UNITS-BTAIL-RESULTEXPORTUNITWITNESS-001 supporting unit evidence

- Supporting role for DEL-08-04 result-export tranche: the schema-first
  result envelope now has optional unit preservation witness vocabulary for
  row-level value/unit/dimension preservation.
- The desktop result export preview records one witness per exported result
  row in the invented mechanics fixture, using the accepted DEC-018-style
  unit-system reference and `conversion_performed=false`.
- Validation passed: `python3 tests/test_results_schema.py`;
  `npm --prefix apps/desktop test -- App.test.tsx` (55/55);
  `npm test --workspace apps/desktop` (18/18 files, 391/391 tests);
  `npm run build --workspace apps/desktop` (existing Vite large-chunk
  warning); `git diff --check`.
- Boundary preserved: no DEC-018 catalog constant change, unit conversion API,
  tolerance policy, solver behavior, protected standards content, private
  data, lifecycle transition, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.

## 2026-06-17 - TP-UNITS-BTAIL-COMPTOLCORPUS-001 supporting unit evidence

- Supporting role for DEL-14-04/DEL-14-05 comparison tranche: analysis-run
  comparison now has focused mixed-unit evidence for caller-supplied
  conversion factors and DEC-026-style relative+absolute tolerance pairs.
- The corpus covers stress normalization from `kPa` to `Pa`, force
  normalization from `lbf` to `N`, absolute near-zero floor behavior, and
  blocking diagnostics when a required conversion path is omitted.
- Validation passed: `python3 tests/test_analysis_run_comparison.py`;
  `python3 tests/test_comparison_contracts.py`; adjacent comparison pytest
  subset passed 23/23; full Python suite passed 360/360; `git diff --check`.
- Boundary preserved: no DEC-018 catalog constant change, unit conversion API
  defaulting, hidden conversion path, release threshold, solver convergence
  policy, protected standards content, private data, lifecycle transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## 2026-06-17 - TP-UNITS-BTAIL-COMPLIBFIELDUNITS-001 supporting unit evidence

- Supporting role for DEL-07-03 component-library authoring tranche: the
  Private Library Manager now uses the DEC-018 unit catalog to filter a
  component-field draft selector by canonical dimension, with
  `linear_stiffness` mapped to accepted force-per-length units.
- Browser preview remains no-fallback: stored `N/m` metadata stays visible as
  model metadata and the route records the desktop-only catalog diagnostic.
- Validation passed: focused `LibraryManagerPanel` Vitest 11/11, full desktop
  Vitest 393/393, desktop production build, and focused Playwright
  library-manager smoke 2/2.
- Boundary preserved: no DEC-018 catalog constant change, schema dimension
  enum change, unit conversion API change, component mechanics, public
  component values, protected standards content, private data, lifecycle
  transition, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim changed.

## 2026-06-17 - TP-UNITS-BTAIL-ADAPTERFRAMEWORKUNITS-001 supporting unit evidence

- Supporting role for DEL-10-02 adapter-framework tranche: the format-neutral
  desktop adapter framework packet now records DEC-018 unit-policy evidence,
  entered-unit storage convention, source/result unit disclosure, and
  `conversion_performed=false`.
- The evidence confirms the framework requires unit validation before adapter
  payload exchange but does not perform target-format conversion or claim any
  target writer support.
- Validation passed: adapter framework contract test; App Vitest 55/55;
  focused R2 Playwright smoke 2/2; full desktop Vitest 397/397; desktop
  production build with the existing Vite large-chunk warning.
- Boundary preserved: no DEC-018 catalog constant change, schema dimension
  enum change, unit conversion API change, target-specific writer, protected
  standards content, private data, lifecycle transition, release-readiness
  claim, professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## 2026-06-17 - TP-UNITS-BTAIL-ADAPTERSDKUNITS-001 supporting unit evidence

- Supporting role for DEL-17-09 export-adapter SDK tranche: adapter admission
  packages now have explicit unit-policy evidence carrying DEC-018 basis,
  entered-unit storage convention, source/result/target unit disclosure, and
  `conversion_performed=false`.
- The evidence is metadata for candidate target admission only; it does not
  introduce a unit conversion API, target writer, runtime loader, compatibility
  claim, or target support claim.
- Validation passed: focused export-adapter SDK Python test; App Vitest 55/55;
  full desktop Vitest 393/393; desktop production build; focused R2 Playwright
  smoke 2/2.
- Boundary preserved: no DEC-018 catalog constant change, schema dimension
  enum change, unit conversion API change, target-specific writer, protected
  standards content, private data, lifecycle transition, release-readiness
  claim, professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## 2026-06-17 - TP-UNITS-BTAIL-MATLIBFIELDUNITS-001 supporting unit evidence

- Supporting role for DEL-07-03/DEL-03-01 material-library authoring tranche:
  the Private Library Manager now uses the DEC-018 unit catalog to filter a
  material-property draft selector by compatible dimension while writing the
  schema-native `unit_ref` and `dimension_id` payload.
- Browser preview remains no-fallback: the selected property default unit ref
  stays visible as model metadata and the route records the desktop-only
  catalog diagnostic.
- Validation passed: focused `LibraryManagerPanel` Vitest 13/13, full desktop
  Vitest 395/395, desktop production build, and focused Playwright
  library-manager smoke 2/2.
- Boundary preserved: no DEC-018 catalog constant change, schema dimension
  enum change, unit conversion API change, material engineering allowables,
  public material values, protected standards content, private data, lifecycle
  transition, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim changed.

## 2026-06-17 - TP-UNITS-BTAIL-SECLIBQTYUNITS-001 supporting unit evidence

- Supporting role for DEL-07-03/DEL-03-02 section-library authoring tranche:
  the Private Library Manager now uses the DEC-018 unit catalog to filter a
  section quantity draft selector by compatible dimension while writing
  schema-native `{ magnitude, unit, dimension }` values.
- Browser preview remains no-fallback: the selected section default unit stays
  visible as model metadata and the route records the desktop-only catalog
  diagnostic.
- Validation passed: focused `LibraryManagerPanel` Vitest 15/15, full desktop
  Vitest 397/397, desktop production build, and focused Playwright
  library-manager smoke 2/2.
- Boundary preserved: no DEC-018 catalog constant change, schema dimension
  enum change, unit conversion API change, section-property calculator, public
  section values, protected standards content, private data, lifecycle
  transition, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim changed.

## 2026-06-17 - TP-UNITS-BTAIL-HEADLESSRUNNERUNITWITNESS-001 supporting unit evidence

- Supporting role for DEL-10-05 headless-runner tranche: the desktop runner
  result-handoff envelope now records DEC-018 unit-system disclosure and
  per-result source value/unit/dimension preservation witnesses with
  `conversion_performed=false`.
- The evidence confirms the runner preview does not perform export-time unit
  conversion and does not silently drop unit metadata when handing off result
  references and audit metadata.
- Validation passed: focused App Vitest 55/55, focused R2 Playwright smoke
  2/2, full desktop Vitest 397/397, and desktop production build with the
  existing Vite large-chunk warning.
- Boundary preserved: no DEC-018 catalog constant change, schema dimension
  enum change, unit conversion API change, final CLI/process/network policy,
  protected standards content, private data, lifecycle transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## 2026-06-17 - TP-UNITS-BTAIL-CAEPIPEEXTERNALUNITWITNESS-001 supporting unit evidence

- Supporting role for DEL-17-05 CAEPIPE external parser tranche: the desktop
  parser package now records DEC-018 unit-system disclosure and per-parser-row
  value/unit/dimension preservation witnesses with `conversion_performed=false`.
- The evidence confirms the parser-only package preserves declared invented
  CSV units at the row boundary and does not perform external-run conversion.
- Validation passed: focused App Vitest 55/55, focused R2 Playwright smoke
  2/2, full desktop Vitest 397/397, desktop production build with the
  existing Vite large-chunk warning, and `git diff --check`.
- Boundary preserved: no DEC-018 catalog constant change, schema dimension
  enum change, unit conversion API change, external CAEPIPE execution,
  protected standards content, private data, lifecycle transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## 2026-06-17 - TP-UNITS-BTAIL-OPDIFFUNITWITNESS-001 supporting unit evidence

- Supporting role for DEL-16-02/DEL-16-03 operation diff tranche: the desktop
  Operation Diff Preview packet now records DEC-018 unit-system disclosure and
  per-unit-bearing-diff-row value/unit/dimension preservation witnesses with
  `conversion_performed=false`.
- The evidence confirms local operation previews do not perform hidden unit
  conversion and do not drop unit metadata before user acceptance.
- Validation passed: focused App Vitest 55/55, focused R2 Playwright smoke
  2/2, full desktop Vitest 397/397, desktop production build with the
  existing Vite large-chunk warning, and `git diff --check`.
- Boundary preserved: no DEC-018 catalog constant change, schema dimension
  enum change, unit conversion API change, operation application, protected
  standards content, private data, lifecycle transition, release-readiness
  claim, professional approval, certification, sealing, authentication, or
  code-compliance claim changed.
