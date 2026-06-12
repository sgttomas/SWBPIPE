# MEMORY - DEL-02-02

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
