# Dependencies: DEL-05-02 Load-case algebra engine

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
- **Rows:** 19 total; 18 ACTIVE; 1 RETIRED.
- **Classes:** ANCHOR=10, EXECUTION=9.
- **Candidate rows moved to handoff/retired visibility this run:** 1.

| DependencyID | Class | Direction | Type | Target | Status | Evidence |
|---|---|---|---|---|---|---|
| `DEP-DEL-05-02-A001` | ANCHOR | UPSTREAM | OTHER | SOW-014 | ACTIVE | Datasheet.md / Identification / Scope items |
| `DEP-DEL-05-02-A002` | ANCHOR | UPSTREAM | OTHER | REQ-05-02-001 | ACTIVE | Specification.md / Requirements table |
| `DEP-DEL-05-02-A003` | ANCHOR | UPSTREAM | OTHER | REQ-05-02-002 | ACTIVE | Specification.md / Requirements table |
| `DEP-DEL-05-02-A004` | ANCHOR | UPSTREAM | OTHER | REQ-05-02-003 | ACTIVE | Specification.md / Requirements table |
| `DEP-DEL-05-02-A005` | ANCHOR | UPSTREAM | OTHER | REQ-05-02-004 | ACTIVE | Specification.md / Requirements table |
| `DEP-DEL-05-02-A006` | ANCHOR | UPSTREAM | OTHER | REQ-05-02-005 | ACTIVE | Specification.md / Requirements table |
| `DEP-DEL-05-02-A007` | ANCHOR | UPSTREAM | OTHER | REQ-05-02-006 | ACTIVE | Specification.md / Requirements table |
| `DEP-DEL-05-02-A008` | ANCHOR | UPSTREAM | OTHER | REQ-05-02-007 | ACTIVE | Specification.md / Requirements table |
| `DEP-DEL-05-02-A009` | ANCHOR | UPSTREAM | OTHER | REQ-05-02-008 | ACTIVE | Specification.md / Requirements table |
| `DEP-DEL-05-02-A010` | ANCHOR | UPSTREAM | OTHER | REQ-05-02-009 | ACTIVE | Specification.md / Requirements table |
| `DAG-002-E0135` | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-01 | ACTIVE | _CONTEXT.md / Architecture Basis Injection / Applicable Basis IDs |
| `DAG-002-E0136` | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-02 | ACTIVE | _CONTEXT.md / Architecture Basis Injection / Applicable Basis IDs |
| `DAG-002-E0137` | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-03 | ACTIVE | _CONTEXT.md / Architecture Basis Injection / Applicable Basis IDs |
| `DAG-002-E0138` | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-06 | ACTIVE | _CONTEXT.md / Architecture Basis Injection / Applicable Basis IDs |
| `DAG-002-E0139` | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-08 | ACTIVE | _CONTEXT.md / Architecture Basis Injection / Applicable Basis IDs |
| `DAG-002-E0451` | EXECUTION | UPSTREAM | PREREQUISITE | DEL-05-01 | ACTIVE | lib.rs / DEL-05-01 CHECKING status and load-case algebra validation |
| `DAG-002-E0452` | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-02 | ACTIVE | README.md / Algebra result boundary metadata; QuantityUnitMetadata; canonical dimension tests |
| `DAG-002-E0453` | EXECUTION | UPSTREAM | INTERFACE | DEL-05-04 | ACTIVE | analysis_status.schema.yaml / DEL-05-04 CHECKING status and analysis-status schema/API tests |
| `DAG-002-E0616` | EXECUTION | UPSTREAM | INTERFACE | DEL-06-02 | RETIRED | PKG05_BLOCKER_CLOSURE_RULING_PACKET_2026-06-05_2120.md / Human ruling: DEL-06-02 evaluator interface non-gating for this review cycle |

## Canonical Dependency Types
- `CONSTRAINT`: 5
- `INTERFACE`: 2
- `OTHER`: 10
- `PREREQUISITE`: 2

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
- **Warnings:** [WARNING] Retired low-confidence non-gating DEL-06-02 evaluator-interface proposal from ACTIVE graph; preserved as reconciliation handoff note.
- Core enum fields conform to the canonical Chirality dependency model.
- Legacy project-specific labels are preserved in `Notes` as `legacy_*` fields where present.
- This dependency refresh does not authorize implementation, lifecycle promotion, release claims, professional approval, certification, sealing, authentication, or code-compliance claims.

## Downstream Handoff Notes
- `DAG-002-E0616` is retained as a RETIRED row and reconciliation handoff only. It must not gate scheduling or execution unless a future human graph decision promotes evaluator-interface reuse after grammar/library selection.

## Run History
- 2026-06-16: `dependency-extract` semantic refresh for PKG-05 shard; mode `UPDATE`; strictness `CONSERVATIVE`; decomposition `execution/_Decomposition/SOFTWARE_DECOMP.md`; ACTIVE rows 18; RETIRED rows 1; warnings: [WARNING] Retired low-confidence non-gating DEL-06-02 evaluator-interface proposal from ACTIVE graph; preserved as reconciliation handoff note.

## Lifecycle Summary
- **ACTIVE rows:** 18
- **RETIRED rows:** 1
- **Satisfaction statuses:** NOT_APPLICABLE=10, SATISFIED=8, TBD=1
- **Closure note:** Dependency semantic refresh closed locally after schema validation; lifecycle acceptance remains outside this task.
