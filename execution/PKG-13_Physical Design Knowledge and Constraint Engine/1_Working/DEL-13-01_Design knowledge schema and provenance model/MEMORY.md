# MEMORY - DEL-13-01 Design Knowledge Schema and Provenance Model

## 2026-06-17 - TP-UNITS-BTAIL-KNOWLEDGEUNITS-001 computed-unit context evidence

- Primary role for a bounded Phase B-tail review-surface evidence slice: the
  desktop Design Knowledge panel now exposes a computed-unit context row when
  mechanics results are present.
- The row derives the count of computed unit-bearing knowledge result refs,
  result unit symbols, `source=computed_preview_result`, and
  `conversion=false` from the existing result envelope.
- Validation passed: focused App Vitest 56/56; focused R2 Playwright 2/2;
  full desktop Vitest 399/399; desktop production build with the existing
  Vite large-chunk warning.
- Evidence: `_run_records/WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-KNOWLEDGEUNITS-001.md`;
  `apps/desktop/SMOKE.md` TP-MAC-223.
- Boundary preserved: no design-knowledge schema, result values, solver
  behavior, unit conversion API, lifecycle state, review disposition,
  protected/private data posture, release status, professional approval,
  certification, sealing, authentication, or code-compliance posture changed.

## Implementation Summary

2026-05-04: Added the schema-first design knowledge contract for DEV-001
revision `0.5` Tranche D.

The implementation records:

- `schemas/design_knowledge.schema.json` as a strict JSON-syntax JSON Schema
  2020-12 contract for user-supplied design knowledge;
- `tests/test_design_knowledge_schema.py` for focused stdlib structural
  checks;
- focused `docs/SPEC.md` and `docs/TYPES.md` entries;
- the sealed brief at
  `execution/_Coordination/DEV-001_REV05_SEALED_BRIEF_DEL-13-01.md`.

## Boundary Decisions

- Design knowledge is user/project supplied and provenance-marked.
- The schema covers endpoints, line data, routing corridors, zones, equipment
  interfaces, access/slope/drain/vent requirements, owner/project metadata,
  source notes, assumptions, diagnostics, privacy classification, and
  redistribution/review status.
- Unit-bearing values require explicit unit metadata.
- Public examples must be invented or otherwise cleared; this deliverable adds
  no public example payloads.
- The schema does not bundle owner standards, protected code criteria,
  proprietary project data, protected standards text, protected tables, vendor
  catalog values, private project data, or code-specific acceptance criteria.
- Professional-boundary controls remain explicit and negative; the schema does
  not claim software compliance, certification, sealing, approval, or
  authentication.

## Verification

Implementation verification for this working-tree state:

- `python3 tests/test_design_knowledge_schema.py`
- `python3 tests/test_model_schema.py`
- broader Tranche D checks recorded in coordination handoff state.

## Remaining TBDs

- Concrete GUI authoring behavior remains downstream in `PKG-07`.
- Constraint records and validation remain downstream in `DEL-13-02` and
  `DEL-13-03`.
- Physical-to-analytical transformation consumption remains downstream in
  `DEL-13-04`.
- Public example payload policy and fixture generation remain later governed
  work.
- Runtime persistence/API integration remains downstream.

## 2026-05-11 TP-RECON-01 Reconciliation

- Reconciled archived `DEV-001` revision `0.5` Tranche D evidence for
  `DEL-13-01` into this deliverable-local history.
- Evidence rows and closeout sources identify commit `dcdc1ac` (`schema: add
  design knowledge and model state contracts`) as committed implementation
  evidence for `schemas/design_knowledge.schema.json`,
  `tests/test_design_knowledge_schema.py`, focused `docs/SPEC.md` /
  `docs/TYPES.md` updates, this deliverable `MEMORY.md`, `_STATUS.md`, and the
  2026-05-04 run note.
- Archived verification recorded `python3 tests/test_design_knowledge_schema.py`,
  `python3 tests/test_model_schema.py`, adjacent schema checks,
  `git diff --check`, dependency validation, DAG audit, and focused
  protected/private/authority scans.
- Reconciliation preserves `CHECKING`; deferred scope remains GUI/runtime/API
  and persistence integration, constraint execution,
  physical-to-analytical consumption, fixture/example policy, and later
  governed project-container behavior. No lifecycle promotion beyond
  `CHECKING` or expansion of engineering authority is recorded.

## 2026-05-16 - DEV-001 downstream PKG-02 audit memory addendum

Durable context preserved after reconciliation review:
- DEV-001 package-worker audit reviewed this deliverable for downstream compatibility with the accepted PKG-02 foundation contracts.
- Local audit artifacts are `execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/DEL-13-01_Design knowledge schema and provenance model/_REVIEW.md` and `execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/DEL-13-01_Design knowledge schema and provenance model/Review_Findings.csv`.
- Package audit summary is `execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/_audit/PKG02_DOWNSTREAM_REVIEW_2026-05-16.md`; package run record is `execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/_run_records/TASK_RUN_2026-05-16_PKG13_PKG02_DOWNSTREAM_PACKAGE_AUDIT.md`.
- This was audit evidence only. It did not change lifecycle state, authorize release, or make a professional, certification, sealing, approval, or code-compliance claim.
- The May 16 package-worker TASK run record did not fully preserve canonical per-deliverable TASK documentation context; this addendum preserves the durable deliverable-local pointer without modifying the completed run record.

## 2026-05-16 - DEV-001 PKG-02 grounded finding-resolution memory addendum

Durable context preserved after PKG-02 grounded finding resolution:
- Stage 2 technical resolution used the accepted PKG-02 contract as the governing source for this deliverable's downstream compatibility evidence.
- Original audit finding count for this deliverable: 2 (BLOCKER=1, WARNING=1). Current technical status count in the resolution matrix: TECHNICALLY_ADDRESSED_PENDING_HUMAN=2.
- Resolution evidence is indexed in `execution/_Reconciliation/Reviews/DEV001_FINDING_RESOLUTION_PKG02_GROUNDED_2026-05-16/RESOLUTION_MATRIX.csv`; validation evidence is summarized in `execution/_Reconciliation/Reviews/DEV001_FINDING_RESOLUTION_PKG02_GROUNDED_2026-05-16/VALIDATION_SUMMARY.md`.
- Local `Review_Findings.csv` entries remain subject to the human disposition gate. `HumanDisposition` stays `TBD` until review, and `Status` must not be changed to `RESOLVED` automatically.
- No lifecycle promotion, release claim, or professional/code-compliance claim is implied by the technical closeout.

## 2026-06-04 - TP-AUTHORITY-REFRESH-0_7-DAG006

- WORKING_ITEMS/TASK workforce current-authority refresh applied to active deliverable-local surfaces for `DEL-13-01`.
- Current authority basis is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7` plus approved `execution/_DAG/DAG-006/` active graph authority.
- Historical run records, historical DAG row IDs, review dispositions, lifecycle `_STATUS.md`, aggregate DAG artifacts, candidate edges, repo-level governance files, schemas, code, and tools were intentionally not changed by this refresh.
- Preserved historical references remain evidence of earlier work, not current authority claims.

## 2026-06-06 - TASK stale four-document evidence refresh

- Corrected `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` text that still described the design knowledge schema and tests as absent, future, or `TBD`.
- Current implementation evidence remains `schemas/design_knowledge.schema.json`, `tests/test_design_knowledge_schema.py`, `RUN_2026-05-04_IMPLEMENTATION.md`, and this memory file. The schema records JSON Schema 2020-12 identity, design knowledge record kinds, provenance/privacy/review fields, unit-aware `Quantity`, diagnostics/assumptions/source notes, and negative professional-boundary flags.
- Validation evidence for this refresh: `python3 tests/test_design_knowledge_schema.py` passed on 2026-06-06.
- Deferred scope remains unchanged: GUI authoring behavior, constraint records/validation, physical-to-analytical transform consumption, runtime persistence/API integration, and public example/fixture policy closure remain downstream or `TBD`.
- No schema/test edits, lifecycle transition, DAG/coordination edit, review-disposition change, blocker-queue update, commit, release claim, or professional/code-compliance claim was made.

## 2026-06-17 - Lifecycle Housekeeping

- Housekeeping lifecycle reset: `_STATUS.md` current state set to `IN_PROGRESS` to reflect current code development in progress. This does not change review, issuance, release readiness, professional approval, certification, sealing, authentication, or code-compliance status.
