# TASK RUN: DEL-06-05 dependency-extract refresh

## Dispatch
- **Task:** TP-DAG-004 dependency-extract refresh row
- **DeliverableID:** DEL-06-05
- **PackageID:** PKG-06
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **ConsumerContext:** RECONCILIATION
- **Timestamp:** 2026-05-10 22:36 MDT

## Read Scope Used
- `AGENTS.md`
- `docs/CONTRACT.md`
- `skills/dependency-extract/SKILL.md`
- `skills/dependency-extract/BRIEF_SCHEMA.md`
- `skills/dependency-extract/QA_CHECKS.md`
- `skills/dependency-extract/TOOL_POLICY.md`
- `execution/_Decomposition/SOFTWARE_DECOMP.md`
- Assigned deliverable folder:
  - `_CONTEXT.md`
  - `_REFERENCES.md`
  - `Datasheet.md`
  - `Specification.md`
  - `Guidance.md`
  - `Procedure.md`
  - existing `Dependencies.csv`
  - existing `_DEPENDENCIES.md`

## Write Scope Used
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_run_records/TASK_RUN_2026-05-10_2236_dependency-extract-refresh.md`

## Actions
- Normalized existing DAG mirror rows to canonical Dependencies.csv v3.1 enum values.
- Added one parent ANCHOR row for SOW-016.
- Added two trace ANCHOR rows for local requirement IDs `DEL-06-05-REQ-01` and `DEL-06-05-REQ-03`.
- Preserved active architecture-basis constraints as `DependencyType=CONSTRAINT`.
- Preserved active rule-pack schema and evaluator prerequisites as `DependencyType=PREREQUISITE`.
- Retired `DAG-002-E0476` and `DAG-002-E0477` conservatively because the allowed source evidence did not explicitly cite target deliverables `DEL-01-02` or `DEL-01-04`.

## Validation
- `python3 tools/validation/validate_dependencies_schema.py <deliverable>/Dependencies.csv`: PASS.
- Canonical enum validation for `DEPENDENCY_CLASS`, `ANCHOR_TYPE`, `DIRECTION`, `DEPENDENCY_TYPE`, `TARGET_TYPE`, `EXPLICITNESS`, `CONFIDENCE`, `ORIGIN`, `STATUS`, and `SATISFACTION_STATUS`: PASS.
- Parent anchor check: PASS; exactly one ACTIVE `IMPLEMENTS_NODE` row.
- ACTIVE row evidence check: PASS; every ACTIVE row has `EvidenceFile` and `SourceRef`.

## Closeout
- **Rows:** 14 total; 12 ACTIVE; 2 RETIRED.
- **ACTIVE class counts:** 3 ANCHOR; 9 EXECUTION.
- **Warnings:** none.
- **Handoff:** RECONCILIATION should decide whether retired governance-predecessor mappings remain aggregate DAG edges, become document constraints, or stay retired locally.
