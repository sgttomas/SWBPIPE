# MEMORY - DEL-14-02 Analysis Run Records

## 2026-07-12 - D-41 R5 T2A canonicalization-label fidelity

- Under `DEC-074` E1 and PDU-003, DEL-14-02 now emits
  `SORTED_COMPACT_JSON` checksum metadata and
  `sorted_compact_json_payload` canonical-truth metadata for its actual
  Python sorted-key compact JSON serializer with ASCII escaping.
- The serializer implementation and hash inputs are unchanged. The
  compatibility name `canonical_json` remains, but its documentation now
  explicitly states that it is not an RFC 8785/JCS implementation.
- Focused evidence proves exact serialized bytes and SHA-256, equivalent-map
  stability, existing result-value mutation sensitivity, exact emitted labels,
  schema alignment, and absence of a JCS claim in generated analysis-run
  envelopes: `10 passed`.
- The analysis-run schema continues to accept legacy `JCS` checksum metadata
  and `canonical_json_jcs_payload` truth metadata for backward compatibility;
  DEL-14-02 does not emit those labels.
- Fan-in confirmed that the PDU-003 DEL-02-02 declaration-currentness repair
  is present in the same diff. The temporary cross-owner `_STATUS.md`
  residual is therefore closed; the D-41 program bootstrap and lifecycle
  `IN_PROGRESS` remain preserved.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-07-12_D41-R5-T2A-PDU003.md`.

## 2026-06-18 - TP-UNITS-BTAIL-RUNAUDITLINTUNITS-001 supporting analysis-run evidence

- Supporting role for DEL-08-05 report-lint inventory: the desktop Report
  Content Lint inventory now includes the Run Audit unit traceability surface.
- The lint inventory records `run-audit-units` as an existing unit-policy
  surface and reports `unit_targets=28`, `conversion_witness_targets=2`, and
  `lint_conversion=false`. The Run Audit panel continues to derive sorted
  model units, solved result unit symbols, result row count, and
  `source=result_envelope` from the existing analysis-run/result envelope
  context without conversion.
- Validation passed: focused App Vitest workspace-render and local project
  round-trip selected tests, focused R2 Playwright smoke 2/2 configured
  project tests, full desktop Vitest 18/18 files and 399/399 tests, desktop
  production build with the existing Vite large-chunk warning, and
  single-worker R2/R3 Playwright smoke 18/18.
- Boundary preserved: no analysis-run schema, hash policy, solver behavior,
  result values, unit conversion API, DEC-018 catalog constant, schema
  dimension enum, protected standards content, private data, lifecycle
  transition, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim changed.

## 2026-06-17 - TP-UNITS-BTAIL-RUNAUDITUNITS-001 run-audit unit traceability

- Primary role for a bounded Phase B-tail analysis-run audit evidence slice:
  the desktop Run Audit panel now exposes a `Unit audit` row for solved runs.
- The row derives sorted model unit declarations from `model.project.units`,
  result row unit symbols from the bound result envelope, the result row
  count, `source=result_envelope`, and `conversion=false`.
- Validation passed: focused App Vitest 56/56; focused R2 Playwright 2/2;
  full desktop Vitest 399/399; desktop production build with the existing
  Vite large-chunk warning.
- Evidence: `_run_records/WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-RUNAUDITUNITS-001.md`;
  `apps/desktop/SMOKE.md` TP-MAC-222.
- Boundary preserved: no analysis-run schema, hash policy, solver behavior,
  result values, unit conversion API, lifecycle state, review disposition,
  protected/private data posture, release status, professional approval,
  certification, sealing, authentication, or code-compliance posture changed.

## Implementation Summary

2026-05-04: Added the schema-first analysis run record contract for DEV-001
revision `0.5` Tranche E.

The implementation records:

- `schemas/analysis_run.schema.json` as a strict JSON-syntax JSON Schema
  2020-12 contract for immutable analysis run records;
- `tests/test_analysis_run_schema.py` for focused stdlib structural checks;
- the sealed brief at
  `execution/_Coordination/DEV-001_REV05_SEALED_BRIEF_DEL-14-02.md`.

## Boundary Decisions

- Analysis runs bind to immutable model states, solver identity/version,
  settings, unit system, load basis, diagnostics, result references, rule-pack
  references, library references, hashes, and reproducibility metadata.
- Run records are read-only reproducibility records; changing the model state,
  solver/settings identity, load basis, result references, or hashes requires a
  distinct run record.
- Human acceptance remains an external hash-bound boundary.
- Physical project package/container details, commercial-tool ingest, external
  prover status, and final API/runtime behavior remain `TBD`.
- The schema does not bundle private library data, private rule-pack payloads,
  protected standards text, proprietary project values, or code-specific
  acceptance limits.
- Professional-boundary controls remain explicit and negative; the schema does
  not claim software compliance, certification, sealing, approval, or
  authentication.

## Verification

Implementation verification for this working-tree state:

- `python3 -m json.tool schemas/analysis_run.schema.json`
- `python3 tests/test_analysis_run_schema.py`
- adjacent Tranche E checks recorded in coordination handoff state.

## Remaining TBDs

- Analysis-run comparison remains downstream in `DEL-14-04`.
- Comparison mapping/tolerance/export contracts remain downstream in
  `DEL-14-05`.
- Handoff package consumption remains downstream in `PKG-15`.
- Shared `docs/SPEC.md` and `docs/TYPES.md` integration was held for a later
  ORCHESTRATOR/closeout gate.

## 2026-05-11 TP-RECON-01 Reconciliation

Evidence: dispatch row `DEL-14-02` / `PKG-14` identifies the deliverable as
`Analysis run records`, with allowed writes limited to this `MEMORY.md` and
`_STATUS.md`. Matrix evidence points to archived implementation/review state,
sealed brief, lifecycle snapshot, tranche proposals/closeouts, semantic
dependency review/precheck records, commit `002263b`, and implementation hints
for `schemas/analysis_run.schema.json`, `core/analysis_runs`,
`tests/test_analysis_run_records.py`, and TP result references/hashes.

Implemented history: the matrix row ties DEL-14-02 history to archived DEV-001
implementation evidence/status, the REV05 sealed brief, lifecycle snapshot,
tranche proposals/closeouts, semantic dependency review/refresh/precheck
records, and commit `002263b`. The row's code/schema/test hints identify the
analysis-run schema, `core/analysis_runs`, `tests/test_analysis_run_records.py`,
and TP result references/hashes as the implementation surface for later
inspection.

Verification: reconciliation checked the DEL-14-02 row against the governance
bundle. Applicable constraints remain: Type 2 work requires explicit
deliverable scope and write scope; agents must surface gaps rather than invent
scope, values, citations, or legal conclusions; protected/private data must not
be bundled as public defaults; software outputs must not claim certification,
approval, sealing, authentication, or engineering code compliance.

Deferred boundaries: no protected standards text, private rule-pack data,
proprietary project values, material allowables, SIF/flexibility tables,
code-specific acceptance criteria, legal conclusion, or human/professional
approval record was added. Archive-file inspection, code/schema/test inspection,
engineering validation, and any protected/private data review remain outside
this bounded write-only reconciliation.

## 2026-05-16 - DEV-001 downstream PKG-02 audit memory addendum

Durable context preserved after reconciliation review:
- DEV-001 package-worker audit reviewed this deliverable for downstream compatibility with the accepted PKG-02 foundation contracts.
- Local audit artifacts are `execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-02_Analysis run records/_REVIEW.md` and `execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-02_Analysis run records/Review_Findings.csv`.
- Package audit summary is `execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/_audit/PKG02_DOWNSTREAM_REVIEW_2026-05-16.md`; package run record is `execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/_run_records/TASK_RUN_2026-05-16_PKG14_PKG02_DOWNSTREAM_AUDIT.md`.
- This was audit evidence only. It did not change lifecycle state, authorize release, or make a professional, certification, sealing, approval, or code-compliance claim.
- The May 16 package-worker TASK run record did not fully preserve canonical per-deliverable TASK documentation context; this addendum preserves the durable deliverable-local pointer without modifying the completed run record.

## 2026-06-04 - TP-AUTHORITY-REFRESH-0_7-DAG006

- WORKING_ITEMS/TASK workforce current-authority refresh applied to active deliverable-local surfaces for `DEL-14-02`.
- Current authority basis is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7` plus approved `execution/_DAG/DAG-006/` active graph authority.
- Historical run records, historical DAG row IDs, review dispositions, lifecycle `_STATUS.md`, aggregate DAG artifacts, candidate edges, repo-level governance files, schemas, code, and tools were intentionally not changed by this refresh.
- Preserved historical references remain evidence of earlier work, not current authority claims.

## 2026-06-06 - Analysis-run evidence hardening

- TASK worker verified the existing analysis-run schema and builder preserve
  model-state references, solver/settings/unit/load basis, result references,
  result hashes, diagnostics, reproducibility metadata, and explicit
  professional-boundary fields.
- Added generated-envelope schema validation and persistence-history evidence
  in `tests/test_analysis_run_records.py`.
- New persistence evidence records that an analysis-run record remains bound
  to its original run basis and run-history hash across unrelated model-payload
  revision changes and canonical JSON round trip.
- Verification passed:
  `python3 -m pytest tests/test_analysis_run_schema.py tests/test_analysis_run_records.py -q`
  (`9 passed`).
- No schema change, production code change, lifecycle update, review
  disposition, protected standards content, private/proprietary data,
  professional approval claim, certification claim, sealing claim,
  authentication claim, or code-compliance claim was introduced.

## 2026-06-07 - TP-PKG14-Remaining Checking Alignment

- Bounded TASK validation rechecked `schemas/analysis_run.schema.json` against
  DEL-14-02/SOW-072 evidence and confirmed the schema parses with
  `python3 -m json.tool schemas/analysis_run.schema.json`.
- Focused validation passed:
  `python3 -m pytest tests/test_analysis_run_schema.py tests/test_analysis_run_records.py -q`
  (`9 passed`).
- Evidence in `tests/test_analysis_run_schema.py` and
  `tests/test_analysis_run_records.py` covers DEL-14-02/SOW-072 identity,
  model-state binding, solver/settings/unit/load basis, diagnostics, result
  references, result hashes, reproducibility, private-payload redaction, and
  professional-boundary rejection checks.
- Conservative deliverable-local consistency scan reported no missing core
  files, no missing four-document kit files, no identity mismatches, and no
  candidate unsourced numerics. The reported `TBD`/`ASSUMPTION` markers were
  classified as intentional deferred decisions or stale setup-stage wording
  superseded by current schema/test evidence, not blockers for CHECKING.
- `_STATUS.md` was moved to `CHECKING` for this bounded alignment only. No
  release, professional-approval, certification, sealing, authentication, or
  code-compliance claim was made.

## 2026-06-11 - Edited-model solve binding guard

- `TP-APP-R2-SOLVEBOUND-001` preserved analysis-run/result binding honesty for
  edited app-session models. Browser fixture mode now emits an incomplete
  result envelope bound to the edited `project.id` instead of carrying the
  bundled fixture's solved result rows forward after model mutation.
- Backend evidence verifies the authoritative Tauri solve paths
  (`run_preview_mechanics(Some(model))` and the solve-job registry) solve the
  supplied edited model payload and publish solved mechanics results bound to
  the edited `project.id`.
- Validation evidence: `_run_records/WORKING_ITEMS_RUN_2026-06-11_edited_model_solve_binding.md`;
  `apps/desktop/SMOKE.md` TP-MAC-88; desktop Vitest 31/31; Tauri Rust tests
  26/26; desktop build green.
- This addendum did not change schemas, lifecycle state, review disposition,
  protected/private data posture, release status, professional approval,
  certification, sealing, authentication, or code-compliance posture.

## 2026-06-11 - TP-APP-R2-PERSISTEDSOLVE-001 persisted edited-load solve regression

- WORKING_ITEMS app-integration tranche added backend evidence that a restored
  saved model, not the original bundled fixture, supplies the solve payload
  after an edited load-data operation.
- The restored solve binds to `project:edited-load-roundtrip`, reports
  `MECHANICS_SOLVED`, produces non-empty results, and changes
  `result:disp:node-N-140` relative to the baseline fixture solve.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_persisted_edited_load_solve_regression.md`
  and `apps/desktop/SMOKE.md` TP-MAC-107. Validation passed:
  src-tauri format check and src-tauri Rust tests 27/27.
- No analysis-run schema, hash policy, lifecycle state, review disposition,
  protected/private data posture, release status, professional approval,
  certification, sealing, authentication, or code-compliance posture changed.

## 2026-06-17 - Lifecycle Housekeeping

- Housekeeping lifecycle reset: `_STATUS.md` current state set to `IN_PROGRESS` to reflect current code development in progress. This does not change review, issuance, release readiness, professional approval, certification, sealing, authentication, or code-compliance status.
