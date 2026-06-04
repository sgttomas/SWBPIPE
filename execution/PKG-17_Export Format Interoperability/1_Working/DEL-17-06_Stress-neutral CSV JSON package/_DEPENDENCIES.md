# Dependencies: DEL-17-06 Stress-neutral CSV/JSON package

## Coordination Mode
- **Mode:** FULL_GRAPH
- **Graph Authority:** `execution/_DAG/DAG-006/`.
- **Authority Boundary:** `DAG-006` is the approved active graph authority.

## Declared Upstream Dependencies
- `DEL-17-02`
- `DEL-08-04`
- `DEL-14-02`
- `DEL-14-05`

## Declared Downstream Dependencies
- To be materialized by DAG-005 and later dependency-extract.

## Extracted Dependency Register
- Register: `Dependencies.csv`
- Schema: `v3.1`
- ACTIVE rows: 16
- RETIRED rows: 0
- ANCHOR rows: 11
- EXECUTION rows: 5
- DECLARED upstream execution rows preserved: 4

| DependencyID | Class | Direction | Type | Target | Origin | Status |
|---|---|---|---|---|---|---|
| DEL-17-06-A001 | ANCHOR | UPSTREAM | OTHER | PKG-17 Export Format Interoperability | EXTRACTED | ACTIVE |
| DEL-17-06-A002 | ANCHOR | UPSTREAM | OTHER | SOW-046 Result export review regression and downstream tooling scope | EXTRACTED | ACTIVE |
| DEL-17-06-A003 | ANCHOR | UPSTREAM | OTHER | SOW-074 Schema-compliant handoff package scope | EXTRACTED | ACTIVE |
| DEL-17-06-A004 | ANCHOR | UPSTREAM | OTHER | OBJ-007 Reproducible reports and result exports objective | EXTRACTED | ACTIVE |
| DEL-17-06-A005 | ANCHOR | UPSTREAM | OTHER | OBJ-017 Traceable handoff package objective | EXTRACTED | ACTIVE |
| DEL-17-06-A006 | ANCHOR | UPSTREAM | OTHER | OBJ-018 Professional and IP boundary objective | EXTRACTED | ACTIVE |
| DEL-17-06-A007 | ANCHOR | UPSTREAM | OTHER | DEL-17-06-REQ-003 Canonical identity preservation requirement | EXTRACTED | ACTIVE |
| DEL-17-06-A008 | ANCHOR | UPSTREAM | OTHER | DEL-17-06-REQ-004 Manifest requirement | EXTRACTED | ACTIVE |
| DEL-17-06-A009 | ANCHOR | UPSTREAM | OTHER | DEL-17-06-REQ-005 Loss report requirement | EXTRACTED | ACTIVE |
| DEL-17-06-A010 | ANCHOR | UPSTREAM | OTHER | DEL-17-06-REQ-006 Unit and dimensional metadata requirement | EXTRACTED | ACTIVE |
| DEL-17-06-A011 | ANCHOR | UPSTREAM | OTHER | DEL-17-06-REQ-009 Professional-boundary prohibition requirement | EXTRACTED | ACTIVE |
| DEL-17-06-D001 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-17-01 CAEPIPE and export-format source basis | EXTRACTED | ACTIVE |
| DEL-17-06-D002 | EXECUTION | UPSTREAM | INTERFACE | DEL-17-02 Export package, profile, and stable ID map contracts | DECLARED | ACTIVE |
| DEL-17-06-D003 | EXECUTION | UPSTREAM | INTERFACE | DEL-08-04 Result export format | DECLARED | ACTIVE |
| DEL-17-06-D004 | EXECUTION | UPSTREAM | INTERFACE | DEL-14-02 Analysis run records | DECLARED | ACTIVE |
| DEL-17-06-D005 | EXECUTION | UPSTREAM | INTERFACE | DEL-14-05 Comparison mapping, tolerance, and export contracts | DECLARED | ACTIVE |

## Run Notes
- Runtime overrides: `SCOPE=DEL-17-06`, `RUN_ROOT=/Users/ryan/ai-env/projects/chirality-piping/execution`, `DECOMPOSITION_PATH=/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`, `MODE=UPDATE`, `STRICTNESS=CONSERVATIVE`, `CONSUMER_CONTEXT=NONE`.
- Source-document defaults applied: `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`.
- Chosen anchor document: `Datasheet.md`.
- Chosen execution document order: `Procedure.md`, `Specification.md`, `Guidance.md`, `Datasheet.md`.
- Decomposition path used and resolved: `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`.
- Existing `Dependencies.csv` was absent at run start, so the register was created.
- Existing declared upstream list was preserved exactly: `DEL-17-02`, `DEL-08-04`, `DEL-14-02`, `DEL-14-05`.
- The explicit `DEL-17-01` source-basis prerequisite was added as `Origin=EXTRACTED` from `Procedure.md#Prerequisites`; it was not promoted into the declared dependency list.
- DAG-005 proposal rows were inspected read-only for context. Proposed architecture-basis edges were not materialized into the local register because the local declared list and four source documents do not declare those targets as execution inputs; `_CONTEXT.md` remains the applicable architecture-basis control surface.
- Tree/DAG integrity: one ACTIVE parent anchor row is present; no `FLOATING_NODE` or `AMBIGUOUS_ANCHOR` warning applies.
- ID-format validation warning: `tools/validation/validate_id_format.sh` expects three-digit package/deliverable formats such as `PKG-017` and `DEL-017-06`, while this project's accepted registers use canonical IDs such as `PKG-17` and `DEL-17-06`. Canonical project IDs were preserved.
- No downstream handoff section was added because `CONSUMER_CONTEXT=NONE`.

## Run History
- 2026-05-18 12:26 MDT - `MODE=UPDATE`, `STRICTNESS=CONSERVATIVE`, decomposition status resolved, warnings: ID-format validator/project-ID mismatch, ACTIVE rows: 16, RETIRED rows: 0, declared rows preserved: 4.

## Lifecycle Summary
- ACTIVE: 16
- RETIRED: 0
- Satisfaction status breakdown: `SATISFIED=11`, `TBD=5`.
- Origin breakdown: `EXTRACTED=12`, `DECLARED=4`.

## Notes
- Candidate rows, if any, remain non-gating until explicit promotion and graph revalidation.
- Dependencies do not authorize implementation, lifecycle promotion, release claims, or professional claims by themselves.
