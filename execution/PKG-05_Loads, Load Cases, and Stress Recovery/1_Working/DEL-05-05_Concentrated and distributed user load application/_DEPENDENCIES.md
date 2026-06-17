# Dependencies: DEL-05-05 Concentrated and distributed user load application

## Coordination Mode
- **Mode:** FULL_GRAPH
- **Graph Authority:** `execution/_DAG/DAG-006/` is the approved legacy graph pending `DAG-007` canonical approval.
- **Authority Boundary:** Candidate/non-gating edges are not represented through `Status=CANDIDATE` or ACTIVE proposal rows in current canonical registers.

## Declared Upstream Dependencies
- None declared outside the extracted register in this refresh.

## Declared Downstream Dependencies
- None declared outside the extracted register in this refresh.

## Extracted Dependency Register
- **Local Register:** `Dependencies.csv`
- **Register schema version:** `v3.1`
- **Semantic refresh:** 2026-06-16
- **Rows:** 13 total; 13 ACTIVE; 0 RETIRED.
- **Classes:** ANCHOR=4, EXECUTION=9.
- **Candidate rows moved to handoff/retired visibility this run:** 0.

| DependencyID | Class | Direction | Type | Target | Status | Evidence |
|---|---|---|---|---|---|---|
| `DEL-05-05-A001` | ANCHOR | UPSTREAM | OTHER | SOW-052 | ACTIVE | Datasheet.md / Identification / Scope Items |
| `DEL-05-05-A002` | ANCHOR | UPSTREAM | OTHER | SOW-013 | ACTIVE | Guidance.md / Considerations |
| `DEL-05-05-A003` | ANCHOR | UPSTREAM | OTHER | OBJ-003 | ACTIVE | Datasheet.md / Identification / Objectives |
| `DEL-05-05-A004` | ANCHOR | UPSTREAM | OTHER | OBJ-012 | ACTIVE | Datasheet.md / Identification / Objectives |
| `DAG-002-E0150` | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-01 | ACTIVE | _CONTEXT.md / Architecture Basis Injection / Applicable Basis IDs |
| `DAG-002-E0151` | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-02 | ACTIVE | _CONTEXT.md / Architecture Basis Injection / Applicable Basis IDs |
| `DAG-002-E0152` | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-03 | ACTIVE | Procedure.md / Steps / Step 5 |
| `DAG-002-E0153` | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-06 | ACTIVE | Specification.md / Requirements / DEL-05-05-R6 |
| `DAG-002-E0154` | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-08 | ACTIVE | Specification.md / Requirements / DEL-05-05-R5 |
| `DAG-002-E0459` | EXECUTION | UPSTREAM | PREREQUISITE | DEL-05-01 | ACTIVE | lib.rs / DEL-05-01 CHECKING status and user-load primitive axial-effect bridge validation |
| `DAG-002-E0460` | EXECUTION | UPSTREAM | PREREQUISITE | DEL-04-01 | ACTIVE | lib.rs / DEL-04-01 CHECKING status and user-load frame DOF validation |
| `DAG-002-E0461` | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-02 | ACTIVE | README.md / User-load model-load boundary records; recovery-hook result records; explicit ForcePerLength TBD metadata test |
| `DEL-05-05-E001` | EXECUTION | DOWNSTREAM | INTERFACE | DEL-05-03 | ACTIVE | Procedure.md / Steps / Step 7 |

## Canonical Dependency Types
- `CONSTRAINT`: 5
- `INTERFACE`: 1
- `OTHER`: 4
- `PREREQUISITE`: 3

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
- 2026-06-16: `dependency-extract` semantic refresh for PKG-05 shard; mode `UPDATE`; strictness `CONSERVATIVE`; decomposition `execution/_Decomposition/SOFTWARE_DECOMP.md`; ACTIVE rows 13; RETIRED rows 0; warnings: None.

## Lifecycle Summary
- **ACTIVE rows:** 13
- **RETIRED rows:** 0
- **Satisfaction statuses:** SATISFIED=12, TBD=1
- **Closure note:** Dependency semantic refresh closed locally after schema validation; lifecycle acceptance remains outside this task.
