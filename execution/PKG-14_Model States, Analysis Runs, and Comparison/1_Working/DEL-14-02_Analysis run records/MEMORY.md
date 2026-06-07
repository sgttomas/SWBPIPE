# MEMORY - DEL-14-02 Analysis Run Records

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
