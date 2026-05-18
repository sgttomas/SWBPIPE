---
doc_id: SCA-004-PROPAGATION-PLAN
doc_kind: scope_change.propagation_plan
status: accepted
created: 2026-05-18
---

# SCA-004 Propagation Plan

## ORCHESTRATOR Workflow

1. PREPARATION Task A creates `PKG-17 Export Format Interoperability`.
2. PREPARATION Task C creates `DEL-17-01` through `DEL-17-09` with `_CONTEXT.md`, `_DEPENDENCIES.md`, `_STATUS.md`, `_REFERENCES.md`, and `_SEMANTIC.md`.
3. PREPARATION Task B creates `PKG-17/0_References/_REFERENCE_INDEX.md` for official/public references and CAEPIPE developer-team question records.
4. TASK + `four-documents` `P1_P2` populates `DEL-17-01` first.
5. REVIEW evaluates the `DEL-17-01` source-basis dossier before target-specific contract or writer claims are drafted.
6. TASK + `four-documents`, `semantic-matrix-build`, `lens-register`, `four-documents P3_ONLY`, and `dependency-extract` populate `DEL-17-02` through `DEL-17-09` in DAG-005 dependency waves.

## Wave Intent

- Wave A: `DEL-17-01`.
- Wave B: `DEL-17-02`.
- Wave C: `DEL-17-03`, `DEL-17-04`.
- Wave D: `DEL-17-05`, `DEL-17-06`, `DEL-17-07`, `DEL-17-08`, `DEL-17-09` after their declared prerequisites are satisfied.

## Non-Writes

SCOPE_CHANGE does not write production deliverable content, code, schemas, tests, lifecycle promotion beyond PREPARATION `OPEN`, dependency-extract registers, commits, release claims, or professional claims.
