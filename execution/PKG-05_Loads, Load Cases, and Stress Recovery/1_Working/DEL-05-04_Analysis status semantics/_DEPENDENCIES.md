# Dependencies: DEL-05-04 Analysis status semantics

## Coordination Mode
- **Mode:** FULL_GRAPH
- **Graph Authority:** `execution/_DAG/DAG-007/` is the current approved canonical graph authority.
- **Authority Boundary:** Candidate/non-gating edges are not represented through `Status=CANDIDATE` or ACTIVE proposal rows in current canonical registers.

## Declared Upstream Dependencies
- None declared outside the extracted register in this refresh.

## Declared Downstream Dependencies
- None declared outside the extracted register in this refresh.

## Extracted Dependency Register
- **Local Register:** `Dependencies.csv`
- **Register schema version:** `v3.1`
- **Semantic refresh:** 2026-06-16
- **Rows:** 10 total; 10 ACTIVE; 0 RETIRED.
- **Classes:** ANCHOR=4, EXECUTION=6.
- **Candidate rows moved to handoff/retired visibility this run:** 0.

| DependencyID | Class | Direction | Type | Target | Status | Evidence |
|---|---|---|---|---|---|---|
| `DEL-05-04-DEP-001` | ANCHOR | UPSTREAM | OTHER | DEL-05-04 | ACTIVE | Datasheet.md / Identification |
| `DEL-05-04-DEP-002` | ANCHOR | UPSTREAM | OTHER | SOW-047 | ACTIVE | _CONTEXT.md / Scope Coverage |
| `DEL-05-04-DEP-003` | ANCHOR | UPSTREAM | OTHER | OBJ-005 | ACTIVE | _CONTEXT.md / Objective Support |
| `DEL-05-04-DEP-004` | ANCHOR | UPSTREAM | OTHER | OBJ-011 | ACTIVE | _CONTEXT.md / Objective Support |
| `DAG-002-E0145` | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-01 | ACTIVE | _CONTEXT.md / Architecture Basis Injection / Applicable Basis IDs |
| `DAG-002-E0146` | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-02 | ACTIVE | _CONTEXT.md / Architecture Basis Injection / Applicable Basis IDs |
| `DAG-002-E0147` | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-03 | ACTIVE | _CONTEXT.md / Architecture Basis Injection / Applicable Basis IDs |
| `DAG-002-E0148` | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-06 | ACTIVE | _CONTEXT.md / Architecture Basis Injection / Applicable Basis IDs |
| `DAG-002-E0149` | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-08 | ACTIVE | _CONTEXT.md / Architecture Basis Injection / Applicable Basis IDs |
| `DAG-002-E0450` | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-03 | ACTIVE | analysis_status_semantics.md / Requirements / REQ-05-04-001; schema automatic-status separation; architecture boundary note |

## Canonical Dependency Types
- `CONSTRAINT`: 5
- `OTHER`: 4
- `PREREQUISITE`: 1

## Run Notes
- **TaskSkill:** `dependency-extract`
- **MODE:** `UPDATE`
- **STRICTNESS:** `CONSERVATIVE`
- **CONSUMER_CONTEXT:** `RECONCILIATION`
- **ARCHITECTURE_BASIS_POLICY:** `PKG00_CONSISTENCY_TRACKERS`
- **Decomposition path:** `execution/_Decomposition/SOFTWARE_DECOMP.md` located and used for anchor/PKG-00 basis validation.
- **Anchor doc selection:** `AUTO`; local datasheet/context/specification evidence used according to strongest explicit identifiers.
- **Execution doc order:** `AUTO`; local `_CONTEXT.md`, `Specification.md`, `Procedure.md`, `Guidance.md`, and cited upstream/downstream evidence reviewed as needed.
- **PKG-00 tracker review:** 5 rows reviewed; 0 rows changed. Supported architecture-basis rows retained as upstream `CONSTRAINT` execution dependencies.
- **Warnings:** None.
- Core enum fields conform to the canonical Chirality dependency model.
- Legacy project-specific labels are preserved in `Notes` as `legacy_*` fields where present.
- This dependency refresh does not authorize implementation, lifecycle promotion, release claims, professional approval, certification, sealing, authentication, or code-compliance claims.

## Downstream Handoff Notes
- No candidate or non-gating proposal rows identified during this semantic refresh.

## Run History
- 2026-06-16: `dependency-extract` semantic refresh for PKG-05 shard; mode `UPDATE`; strictness `CONSERVATIVE`; decomposition `execution/_Decomposition/SOFTWARE_DECOMP.md`; ACTIVE rows 10; RETIRED rows 0; warnings: None.

## Lifecycle Summary
- **ACTIVE rows:** 10
- **RETIRED rows:** 0
- **Satisfaction statuses:** NOT_APPLICABLE=4, SATISFIED=6
- **Closure note:** Dependency semantic refresh closed locally after schema validation; lifecycle acceptance remains outside this task.
