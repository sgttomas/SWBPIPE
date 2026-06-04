# Dependencies: DEL-17-05 CAEPIPE external run harness and CSV parser

## Coordination Mode
- **Mode:** FULL_GRAPH
- **Graph Authority:** `execution/_DAG/DAG-006/`.
- **Authority Boundary:** `DAG-006` is the approved active graph authority.

## Declared Upstream Dependencies
- `DEL-17-04`

## Declared Downstream Dependencies
- No active downstream dependency is declared in this local register unless listed in `Dependencies.csv`; historical DAG-005 extraction wording is superseded by DAG-006 active graph authority.

## Extracted Dependency Register
- **Register:** `Dependencies.csv`
- **Schema:** v3.1
- **Generated/Refreshed:** 2026-05-18 12:26 America/Edmonton
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer Context:** NONE
- **ACTIVE rows:** 12
- **RETIRED rows:** 0
- **Declared rows preserved:** 1

| DependencyID | Class | Direction | Type | TargetType | Target | Origin | Status |
|---|---|---|---|---|---|---|---|
| DEP-017-05-001 | ANCHOR | UPSTREAM | OTHER | WBS_NODE | PKG-17 Export Format Interoperability | EXTRACTED | ACTIVE |
| DEP-017-05-002 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | SOW-030 | EXTRACTED | ACTIVE |
| DEP-017-05-003 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | SOW-046 | EXTRACTED | ACTIVE |
| DEP-017-05-004 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | SOW-075 | EXTRACTED | ACTIVE |
| DEP-017-05-005 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | OBJ-007 | EXTRACTED | ACTIVE |
| DEP-017-05-006 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | OBJ-009 | EXTRACTED | ACTIVE |
| DEP-017-05-007 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | OBJ-017 | EXTRACTED | ACTIVE |
| DEP-017-05-008 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | OBJ-018 | EXTRACTED | ACTIVE |
| DEP-017-05-009 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-17-04 | DECLARED | ACTIVE |
| DEP-017-05-010 | EXECUTION | UPSTREAM | INTERFACE | DELIVERABLE | DEL-17-01 | EXTRACTED | ACTIVE |
| DEP-017-05-011 | EXECUTION | UPSTREAM | INTERFACE | DELIVERABLE | DEL-17-02 | EXTRACTED | ACTIVE |
| DEP-017-05-012 | EXECUTION | UPSTREAM | CONSTRAINT | EXTERNAL | CAEPIPE-EXECUTABLE | EXTRACTED | ACTIVE |

## Run Notes
- Applied runtime overrides: `SCOPE=DEL-17-05`, `RUN_ROOT=/Users/ryan/ai-env/projects/chirality-piping/execution`, `DECOMPOSITION_PATH=/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`, `MODE=UPDATE`, `STRICTNESS=CONSERVATIVE`, `CONSUMER_CONTEXT=NONE`.
- Source document selection used the skill defaults: `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`.
- Chosen anchor document: `Datasheet.md` because it contains explicit deliverable identity, package, scope items, objectives, and upstream dependency fields.
- Chosen execution document order: `Specification.md`, `Procedure.md`, `Guidance.md`, `Datasheet.md`, `_DEPENDENCIES.md`, `_REFERENCES.md`.
- Decomposition path resolved and used for package, deliverable, scope-item, and objective label checks: `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`.
- Existing declared upstream dependency `DEL-17-04` was preserved as `Origin=DECLARED` and not duplicated as a separate extracted row.
- Required maturity and satisfaction status for execution dependencies remain `TBD` where the four documents keep invocation profile, accepted MBF profile/writer tranche, parser coverage, or target behavior unresolved.
- No downstream dependencies were materialized because no explicit downstream consumer edge was stated in the DEL-17-05 source documents.
- No `[WARNING] FLOATING_NODE`: exactly one ACTIVE `IMPLEMENTS_NODE` parent anchor is present.

## Notes
- Candidate rows, if any, remain non-gating until explicit promotion and graph revalidation.
- Dependencies do not authorize implementation, lifecycle promotion, release claims, or professional claims by themselves.

## Run History
- 2026-05-18 12:26 America/Edmonton — `dependency-extract` UPDATE / CONSERVATIVE / CONSUMER_CONTEXT=NONE. Decomposition path resolved. Warnings: none. ACTIVE rows: 12 total; 8 ANCHOR; 4 EXECUTION; 1 DECLARED; 11 EXTRACTED.

## Lifecycle Summary
- ACTIVE: 12
- RETIRED: 0
- By dependency class: ANCHOR 8, EXECUTION 4
- By origin: DECLARED 1, EXTRACTED 11
- Closure state breakdown:
  - NOT_APPLICABLE: 8
  - TBD: 4
