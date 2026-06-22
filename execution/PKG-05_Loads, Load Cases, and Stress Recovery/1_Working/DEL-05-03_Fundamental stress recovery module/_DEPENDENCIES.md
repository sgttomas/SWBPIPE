# Dependencies: DEL-05-03 Fundamental stress recovery module

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
- **Rows:** 12 total; 12 ACTIVE; 0 RETIRED.
- **Classes:** ANCHOR=2, EXECUTION=10.
- **Candidate rows moved to handoff/retired visibility this run:** 0.

| DependencyID | Class | Direction | Type | Target | Status | Evidence |
|---|---|---|---|---|---|---|
| `DEP-DEL-05-03-A001` | ANCHOR | UPSTREAM | OTHER | SOW-015 | ACTIVE | _CONTEXT.md / Scope Coverage |
| `DEP-DEL-05-03-A002` | ANCHOR | UPSTREAM | OTHER | OBJ-003 | ACTIVE | _CONTEXT.md / Objective Support |
| `DAG-002-E0140` | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-01 | ACTIVE | _CONTEXT.md / Architecture Basis Injection / Applicable Basis IDs |
| `DAG-002-E0141` | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-02 | ACTIVE | _CONTEXT.md / Architecture Basis Injection / Applicable Basis IDs |
| `DAG-002-E0142` | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-03 | ACTIVE | _CONTEXT.md / Architecture Basis Injection / Applicable Basis IDs |
| `DAG-002-E0143` | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-06 | ACTIVE | _CONTEXT.md / Architecture Basis Injection / Applicable Basis IDs |
| `DAG-002-E0144` | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-08 | ACTIVE | _CONTEXT.md / Architecture Basis Injection / Applicable Basis IDs |
| `DAG-002-E0454` | EXECUTION | UPSTREAM | INTERFACE | DEL-04-02 | ACTIVE | lib.rs / DEL-04-02 CHECKING status and straight-pipe/resultant interface validation |
| `DAG-002-E0455` | EXECUTION | UPSTREAM | INTERFACE | DEL-03-08 | ACTIVE | lib.rs / DEL-03-08 CHECKING status and stress-recovery section-property validation |
| `DAG-002-E0456` | EXECUTION | UPSTREAM | INTERFACE | DEL-05-01 | ACTIVE | lib.rs / DEL-05-01 CHECKING status and stress-resultant boundary validation |
| `DAG-002-E0457` | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-02 | ACTIVE | README.md / Stress input unit metadata validation; recovered stress boundary records; canonical dimension tests |
| `DAG-002-E0458` | EXECUTION | UPSTREAM | INTERFACE | DEL-05-04 | ACTIVE | analysis_status.schema.yaml / DEL-05-04 CHECKING status and analysis-status boundary tests |

## Canonical Dependency Types
- `CONSTRAINT`: 5
- `INTERFACE`: 4
- `OTHER`: 2
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
- 2026-06-16: `dependency-extract` semantic refresh for PKG-05 shard; mode `UPDATE`; strictness `CONSERVATIVE`; decomposition `execution/_Decomposition/SOFTWARE_DECOMP.md`; ACTIVE rows 12; RETIRED rows 0; warnings: None.

## Lifecycle Summary
- **ACTIVE rows:** 12
- **RETIRED rows:** 0
- **Satisfaction statuses:** SATISFIED=12
- **Closure note:** Dependency semantic refresh closed locally after schema validation; lifecycle acceptance remains outside this task.
