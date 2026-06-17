# Dependencies: DEL-05-01 Primitive load case engine

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
- **Rows:** 12 total; 8 ACTIVE; 4 RETIRED.
- **Classes:** ANCHOR=2, EXECUTION=10.
- **Candidate rows moved to handoff/retired visibility this run:** 0.

| DependencyID | Class | Direction | Type | Target | Status | Evidence |
|---|---|---|---|---|---|---|
| `TP-DAG-004-DEL-05-01-A001` | ANCHOR | UPSTREAM | OTHER | SOW-013 | ACTIVE | _CONTEXT.md / Scope Coverage |
| `TP-DAG-004-DEL-05-01-A002` | ANCHOR | UPSTREAM | OTHER | OBJ-003 | ACTIVE | _CONTEXT.md / Objective Support |
| `DAG-002-E0130` | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-01 | ACTIVE | _CONTEXT.md / Architecture Basis Injection / Applicable Basis IDs |
| `DAG-002-E0131` | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-02 | ACTIVE | _CONTEXT.md / Architecture Basis Injection / Applicable Basis IDs |
| `DAG-002-E0132` | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-03 | ACTIVE | _CONTEXT.md / Architecture Basis Injection / Applicable Basis IDs |
| `DAG-002-E0133` | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-06 | ACTIVE | _CONTEXT.md / Architecture Basis Injection / Applicable Basis IDs |
| `DAG-002-E0134` | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-08 | ACTIVE | _CONTEXT.md / Architecture Basis Injection / Applicable Basis IDs |
| `TP-DAG-004-DEL-05-01-E001` | EXECUTION | DOWNSTREAM | INTERFACE | DEL-05-02 | ACTIVE | Specification.md / Requirements / REQ-05-01-007 |
| `DAG-002-E0446` | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-01 | RETIRED | Deliverables.csv / Retired during TP-DAG-004 refresh; prior DAG inference not directly evidenced in assigned DEL-05-01 sources |
| `DAG-002-E0447` | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-02 | RETIRED | Deliverables.csv / Retired during TP-DAG-004 refresh; prior DAG inference not directly evidenced in assigned DEL-05-01 sources |
| `DAG-002-E0448` | EXECUTION | UPSTREAM | PREREQUISITE | DEL-04-01 | RETIRED | Deliverables.csv / Retired during TP-DAG-004 refresh; prior DAG inference not directly evidenced in assigned DEL-05-01 sources |
| `DAG-002-E0449` | EXECUTION | UPSTREAM | PREREQUISITE | DEL-04-03 | RETIRED | Deliverables.csv / Retired during TP-DAG-004 refresh; prior DAG inference not directly evidenced in assigned DEL-05-01 sources |

## Canonical Dependency Types
- `CONSTRAINT`: 5
- `INTERFACE`: 1
- `OTHER`: 2
- `PREREQUISITE`: 4

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
- **Warnings:** [WARNING] None. Four unsupported prior DAG-inference rows remain RETIRED from the existing refreshed register.
- Core enum fields conform to the canonical Chirality dependency model.
- Legacy project-specific labels are preserved in `Notes` as `legacy_*` fields where present.
- This dependency refresh does not authorize implementation, lifecycle promotion, release claims, professional approval, certification, sealing, authentication, or code-compliance claims.

## Downstream Handoff Notes
- Retired upstream predecessor hypotheses (`DAG-002-E0446` through `DAG-002-E0449`) remain visible as historical unsupported extracted rows and should not gate execution without renewed evidence.

## Run History
- 2026-06-16: `dependency-extract` semantic refresh for PKG-05 shard; mode `UPDATE`; strictness `CONSERVATIVE`; decomposition `execution/_Decomposition/SOFTWARE_DECOMP.md`; ACTIVE rows 8; RETIRED rows 4; warnings: [WARNING] None. Four unsupported prior DAG-inference rows remain RETIRED from the existing refreshed register.

## Lifecycle Summary
- **ACTIVE rows:** 8
- **RETIRED rows:** 4
- **Satisfaction statuses:** PENDING=1, SATISFIED=7, TBD=4
- **Closure note:** Dependency semantic refresh closed locally after schema validation; lifecycle acceptance remains outside this task.
