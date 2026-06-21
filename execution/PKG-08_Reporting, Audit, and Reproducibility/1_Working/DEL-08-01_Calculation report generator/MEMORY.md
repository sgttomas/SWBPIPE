# MEMORY - DEL-08-01 Calculation Report Generator

## 2026-06-21 - TP-R4-D8-COMPPROVREPORT-001 component provenance in rendered reports

WORKING_ITEMS landed the DEL-08-01 renderer side of the R4 D8 component
provenance path. The desktop rendered-report input now passes component
provenance rows, geometry/modifier source references, and
missing-provenance warnings through `ReportSections`; the Rust hash-bound
HTML renderer has regression coverage proving those component rows appear in
the warnings/assumptions/provenance section.

Evidence: package run record
`execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/_run_records/WORKING_ITEMS_RUN_2026-06-21_TP-R4-D8-COMPPROVREPORT-001.md`.
Validation passed: report-renderer `cargo fmt --check`, report-renderer
`cargo test` 8/8, focused desktop rendered-report Vitest 8/8, desktop
production build, and `git diff --check`.

Boundary preserved: no protected standards content, proprietary catalog data,
private default write, lifecycle transition, release-readiness claim,
professional approval, certification, sealing, authentication, or
code-compliance claim changed.

## 2026-06-18 - TP-UNITS-BTAIL-RENDEREDREPORTLINTUNITS-001 supporting report-lint inventory evidence

- Supporting role for DEL-08-05 report-lint inventory: the desktop Report
  Content Lint inventory now includes the Rendered Report unit-basis surface.
- The lint inventory records `rendered-report-unit-basis` as an existing
  unit-policy surface and reports `unit_targets=32`,
  `conversion_witness_targets=2`, and `lint_conversion=false`. The Rendered
  Report panel continues to expose `unit_system`, model-unit summary,
  result-unit summary, `conversion=false`, and
  `source=renderable_report_input`.
- Validation passed: focused App Vitest workspace-render and local project
  round-trip selected tests, focused R2 Playwright smoke 2/2 configured
  project tests, full desktop Vitest 18/18 files and 399/399 tests, desktop
  production build with the existing Vite large-chunk warning, and
  single-worker R2/R3 Playwright smoke 18/18.
- Boundary preserved: no report renderer route, canonical hash policy,
  export-gate policy, report content, print/PDF behavior, protected standards
  content, private data, lifecycle transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## Implementation Record

2026-05-02:

- Implemented a bounded calculation-report contract under
  `schemas/report_generator.schema.yaml`.
- Added `core/reporting/report_generator/` as a standalone Rust support crate
  for in-memory report validation, template-slot validation, and deterministic
  neutral section ordering.
- Added an invented, non-engineering report fixture under
  `fixtures/reports/invented/calculation_report_fixture.json`.
- Added `tests/test_report_generator_contract.py` for stdlib schema and
  fixture checks.
- Updated `docs/SPEC.md` and `docs/TYPES.md` with the report-generator
  boundary.

## Source Basis

- Sealed dispatch brief:
  `execution/_Coordination/DEV-001_DISPATCH_DEL-08-01.md`.
- Deliverable context:
  `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-01_Calculation report generator/_CONTEXT.md`.
- Active upstreams preserved:
  `DEL-02-05`, `DEL-05-03`, `DEL-05-04`, `DEL-06-04`, `DEL-08-02`,
  `DEL-08-03`, and `DEL-01-04`.

## Guardrails Preserved

- No protected standards text, protected tables, protected examples,
  proprietary formulas, proprietary engineering values, private project data,
  private rule-pack payloads, private library content, or real secrets were
  introduced.
- No GUI presentation, CLI runtime, API transport, adapter behavior, local FEA
  handoff packaging, protected-content linter implementation, private-data
  redaction/export controls, package/CI/release change, lifecycle transition,
  dependency-register edit, evidence registration, blocker-queue refresh, or
  candidate-edge change was performed.
- Report output remains decision support and preserves human-review-required
  status without automatic code-compliance, certification, sealing, approval,
  authentication, endorsement, or professional-reliance claims.

## Verification

Run before closeout:

- `python3 tests/test_report_generator_contract.py`
- `cargo fmt --manifest-path core/reporting/report_generator/Cargo.toml -- --check`
- `cargo test --manifest-path core/reporting/report_generator/Cargo.toml`
- `python3 tests/test_report_sections_contract.py`
- `python3 tests/test_results_schema.py`
- `python3 tests/test_analysis_status_schema.py`
- `python3 tests/test_persistence_schema.py`
- `python3 tests/test_rule_pack_schema.py`
- `git diff --check`

## Remaining TBDs

- GUI presentation and report preview workflow.
- CLI runtime and command syntax.
- Public API transport.
- Adapter behavior.
- Private-data redaction/export controls.
- Protected-content linter integration.
- Release-template integration.
- Final report styling/layout policy.

## 2026-05-11 TP-RECON-01 Reconciliation

- Reconciled DEL-08-01 history from the TP-RECON-01 dispatch row and archived
  DEV-001 evidence. Commit `9e21716` (`schema: add calculation report generator
  contract`, 2026-05-02) records the bounded report-generator schema, Rust
  support crate, invented fixture, contract tests, focused docs updates,
  deliverable memory/status updates, and coordination closeout.
- Evidence status remains `CHECKING`; the archived lifecycle snapshot and
  current `_STATUS.md` both carry `CHECKING`. This reconciliation carries
  forward implemented evidence only and does not add product-readiness,
  engineering-reliance, or standards-conformance conclusions.
- Verification evidence carried forward from the archived dispatch includes
  `python3 tests/test_report_generator_contract.py`, report/results/status/
  persistence/rule-pack schema checks, `cargo fmt --manifest-path
  core/reporting/report_generator/Cargo.toml -- --check`, `cargo test
  --manifest-path core/reporting/report_generator/Cargo.toml`, and
  `git diff --check`.
- Deferred scope remains GUI/report preview workflow, CLI/API/adapter runtime
  behavior, private-data redaction/export controls, protected-content linter
  integration, release-template integration, and final styling/layout policy.

## 2026-05-16 - DEV-001 downstream PKG-02 audit memory addendum

Durable context preserved after reconciliation review:
- DEV-001 package-worker audit reviewed this deliverable for downstream compatibility with the accepted PKG-02 foundation contracts.
- Local audit artifacts are `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-01_Calculation report generator/_REVIEW.md` and `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-01_Calculation report generator/Review_Findings.csv`.
- Package audit summary is `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/_audit/PKG02_DOWNSTREAM_REVIEW_2026-05-16.md`; package run record is `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/_run_records/TASK_RUN_2026-05-16_pkg08_pkg02_downstream_audit.md`.
- This was audit evidence only. It did not change lifecycle state, authorize release, or make a professional, certification, sealing, approval, or code-compliance claim.
- The May 16 package-worker TASK run record did not fully preserve canonical per-deliverable TASK documentation context; this addendum preserves the durable deliverable-local pointer without modifying the completed run record.

## 2026-06-04 - TP-AUTHORITY-REFRESH-0_7-DAG006

- WORKING_ITEMS/TASK workforce current-authority refresh applied to active deliverable-local surfaces for `DEL-08-01`.
- Current authority basis is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7` plus approved `execution/_DAG/DAG-006/` active graph authority.
- Historical run records, historical DAG row IDs, review dispositions, lifecycle `_STATUS.md`, aggregate DAG artifacts, candidate edges, repo-level governance files, schemas, code, and tools were intentionally not changed by this refresh.
- Preserved historical references remain evidence of earlier work, not current authority claims.

## 2026-06-06 - TASK hardening addendum

- Hardened `core/reporting/report_generator/src/lib.rs` deterministic in-memory
  report validation for all governed section kinds, duplicate template ordering
  indexes, incoming diagnostic completeness, result/audit/report-section
  reference boundary metadata, and rule-pack review/completeness gaps.
- Added crate-level regression coverage for missing audit/section kinds,
  duplicate ordering indexes, incomplete diagnostics, private/protected/TBD
  reference metadata warnings, and incomplete rule-pack review metadata.
- Preserved existing exclusions: no schema, fixture, GUI, CLI/API, adapter,
  redaction/export, protected-content linter, release-template, lifecycle,
  review-disposition, or professional/code-compliance claim changes.
- Validation run: `python3 tests/test_report_generator_contract.py`,
  `cargo test --manifest-path core/reporting/report_generator/Cargo.toml`,
  `cargo fmt --manifest-path core/reporting/report_generator/Cargo.toml -- --check`.
  `git diff --check` is recorded in the run record after final file updates.

## 2026-06-06 - WORKING_ITEMS fan-in validation

- Parent WORKING_ITEMS fan-in for the PKG-08 reporting hardening tranche found
  the DEL-08-01 changes inside declared scope.
- Fan-in validation passed: `python3 tests/test_report_generator_contract.py`,
  `cargo test --manifest-path core/reporting/report_generator/Cargo.toml`,
  `cargo fmt --manifest-path core/reporting/report_generator/Cargo.toml -- --check`,
  and `git diff --check`.
- Lifecycle state remains `IN_PROGRESS`; no review disposition, dependency,
  DAG, release, professional-approval, or code-compliance claim was changed.

## 2026-06-11 - TP-APP-R2-REPORTRENDER-001 (A7 rendered report, DEC-021)

- The D-10 ruling (`DEC-021`, SOFTWARE_DECOMP §12) is implemented: new
  `core/reporting/report_renderer` crate renders `CalculationReport` +
  `ReportSections` + caller result rows into a deterministic, scriptless,
  single-file HTML document; the SHA-256 of the bytes is the canonical
  hash-bound rendered-report evidence; three-point protected-content lint
  gating plus blocking validation diagnostics refuse export;
  `derived_print_view` emits the labeled non-hash-bound print/PDF view
  naming the canonical hash.
- `report_generator` and `report_sections` gained a default-off `serde`
  feature with spellings pinned to the schema contract (SCREAMING analysis
  statuses/diagnostic classes, `"TBD"`, envelope field `ref`), proven
  against `fixtures/reports/invented/calculation_report_fixture.json`.
- Desktop seam: `render_calculation_report` Tauri command, render service
  with explicit desktop-only browser route, session-envelope adapter with
  explicit TBD markers, Rendered Report panel refusing save/print while
  blocked.
- Local run record:
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_TP-APP-R2-REPORTRENDER-001.md`
  (evidence, boundary review, residuals: browser-mode render seam → A8,
  rule-pack refs → Phase C, report-hash persistence follow-up, D-10b PDF
  emitter at the R5 lead-up).

## 2026-06-12 - TP-UNITS-B2-REPORTUNITS-001 report unit-system disclosure

- WORKING_ITEMS B2 app-integration tranche added explicit unit-system
  disclosure to the frontend Report Packet panel and JSON export.
- The report packet now names `unit-system:dec-018-si-dual-display`, exports
  the model unit map, exports distinct result-row units, records
  entered-unit preservation, and records `conversion_performed=false`.
- The rendered-report adapter now uses the DEC-018 unit-system reference in
  the existing strict `model_input_summary.unit_system_ref` field. The
  report-generator schema was not widened.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-12_report_unit_system_disclosure.md`,
  the corresponding DEL-02-02 run record, and `apps/desktop/SMOKE.md`
  TP-MAC-132. Validation passed: focused report/App Vitest 53/53, Playwright
  R2 smoke 2/2, full desktop Vitest 216/216, and desktop build.
- No lifecycle state, review finding status, dependency authority, release
  readiness, professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards data, private project data,
  network path, telemetry path, unit conversion, or report-schema expansion
  changed.

## 2026-06-12 - TP-UNITS-B2-REPORTBODY-001 rendered report body unit disclosure

- WORKING_ITEMS B2 app-integration tranche expanded the hash-bound rendered
  report body so users can see unit evidence in the `Model Input Summary`
  section, not only in the Report Packet JSON.
- `report_generator::ModelInputSummary` and
  `schemas/report_generator.schema.yaml` now support optional
  `unit_display_summary`; existing inputs remain valid when the field is
  absent.
- `core/reporting/report_renderer` renders unit storage convention, model
  units, result units, quantity display policy, and report-time conversion.
  The shared invented report fixture and Tauri renderer-command regression
  assert those rows.
- The A12 from-blank rehearsal helper now overwrites the shared fixture's unit
  summary with the actual authored model units and solved result units before
  rendering, so the full author -> solve -> report regression carries the
  same evidence.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-12_report_body_unit_disclosure.md`,
  the corresponding DEL-02-02 run record, and `apps/desktop/SMOKE.md`
  TP-MAC-135. Validation passed: report-generator schema contract test,
  report-generator cargo tests 10/10, report-renderer cargo tests 8/8, Tauri
  Rust tests 32/32, focused report/App Vitest 53/53, full desktop Vitest
  216/216, desktop build, and Playwright R2 smoke 2/2.
- No lifecycle state, review finding status, dependency authority, release
  readiness, professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards data, private project data,
  network path, telemetry path, report-time conversion, import/export unit
  conversion, rule-pack unit I/O, or browser fallback catalog changed.

## 2026-06-17 - Lifecycle Housekeeping

- Housekeeping lifecycle reset: `_STATUS.md` current state set to `IN_PROGRESS` to reflect current code development in progress. This does not change review, issuance, release readiness, professional approval, certification, sealing, authentication, or code-compliance status.

## 2026-06-17 - TP-UNITS-BTAIL-RENDEREDREPORTUNITS-001 rendered-report unit basis visibility

- Primary role for Phase B-tail Rendered Report unit-basis visibility: the
  desktop Rendered Report panel now exposes the render-input unit basis before
  invoking the desktop-only renderer route.
- `data-testid="rendered-report-unit-basis"` reports the DEC-018 unit-system
  reference, sorted model units, solved result units or `results=none`,
  `conversion=false`, and `source=renderable_report_input`.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-RENDEREDREPORTUNITS-001.md`;
  `apps/desktop/SMOKE.md` TP-MAC-232; completion log entry; supporting
  DEL-02-02 run record.
- Validation passed: focused rendered-report Vitest 8/8 tests, focused
  Playwright 2/2 Chromium desktop tests, full desktop Vitest 399/399,
  single-worker R2/R3 Playwright smoke 18/18, `git diff --check`, and
  desktop production build with the existing Vite large-chunk warning.
- Boundary preserved: no renderer command, report schema, canonical hash
  behavior, save/print gate, report-time conversion, unit-conversion API,
  DEC-018 catalog constant, schema dimension enum, protected standards
  content, private data, lifecycle transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim changed.
