# Dependencies: DEL-14-04 Analysis-run comparison engine

## Generated Dependency Register
- **Status:** REFRESHED_FROM_LOCAL_EVIDENCE_AND_APPROVED_DAG002
- **Graph authority used:** `execution/_DAG/DAG-006/`
- **Local Register:** `Dependencies.csv`
- **Rows:** 17 total; 17 ACTIVE; 0 RETIRED.
- **Generated:** 2026-05-10

## Authority Boundary
- Aggregate `DAG-006` remains the approved graph authority for sequencing and blocker computation within its approval boundary.
- This local register is a refreshed evidence surface for later RECONCILIATION, not an independent graph approval.
- `DAG-003` was not used as graph authority and was not approved or promoted by this refresh.
- Downstream `ENABLES` rows are local reconciliation evidence only; they are not prerequisites blocking DEL-14-04 execution.
- `PKG-00` architecture-basis rows are preserved as injected context evidence; `PKG-00` does not receive local dependency registers in this tranche.

## Extracted Dependency Register

| Count basis | Count |
|---|---:|
| Total rows | 17 |
| ACTIVE | 17 |
| RETIRED | 0 |
| ANCHOR | 4 |
| EXECUTION | 13 |
| UPSTREAM | 15 |
| DOWNSTREAM | 2 |

### Active Rows

| DependencyID | Class | Direction | Type | Target | Evidence |
|---|---|---|---|---|---|
| DEL-14-04-A001 | ANCHOR | UPSTREAM | OTHER | PKG-14 | `_CONTEXT.md` Package Reference |
| DEL-14-04-A002 | ANCHOR | UPSTREAM | OTHER | SOW-073 | `_CONTEXT.md` Scope Coverage |
| DEL-14-04-A003 | ANCHOR | UPSTREAM | OTHER | SOW-072 | `_CONTEXT.md` Scope Coverage |
| DEL-14-04-A004 | ANCHOR | UPSTREAM | OTHER | OBJ-016 | `_CONTEXT.md` Objective Support |
| DAG-002-E0688 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-01 | `_CONTEXT.md` Architecture Basis Injection |
| DAG-002-E0689 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-02 | `_CONTEXT.md` Architecture Basis Injection |
| DAG-002-E0690 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-03 | `_CONTEXT.md` Architecture Basis Injection |
| DAG-002-E0691 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-04 | `_CONTEXT.md` Architecture Basis Injection |
| DAG-002-E0692 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-06 | `_CONTEXT.md` Architecture Basis Injection |
| DAG-002-E0693 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-07 | `_CONTEXT.md` Architecture Basis Injection |
| DAG-002-E0694 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-08 | `_CONTEXT.md` Architecture Basis Injection |
| DAG-002-E0795 | EXECUTION | UPSTREAM | INTERFACE | DEL-14-02 | `Procedure.md` Prerequisites |
| DAG-002-E0796 | EXECUTION | UPSTREAM | INTERFACE | DEL-14-05 | `Procedure.md` Prerequisites |
| DAG-002-E0797 | EXECUTION | UPSTREAM | INTERFACE | DEL-08-04 | `Procedure.md` Prerequisites |
| DAG-002-E0798 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-02-02 | `Procedure.md` Prerequisites |
| DEL-14-04-D001 | EXECUTION | DOWNSTREAM | ENABLES | DEL-07-08 | `DAG-002/DependencyEdges.csv` DAG-002-E0849 |
| DEL-14-04-D002 | EXECUTION | DOWNSTREAM | ENABLES | DEL-08-06 | `DAG-002/DependencyEdges.csv` DAG-002-E0864 |

## Run Notes
- **Mode:** UPDATE.
- **Strictness:** CONSERVATIVE.
- **Consumer context:** RECONCILIATION.
- **Scope:** DEL-14-04 only.
- **Decomposition path:** `execution/_Decomposition/SOFTWARE_DECOMP.md` located and used for PKG-14, SOW-072, SOW-073, OBJ-016, and DEL-14-04 label validation.
- **Graph authority:** `execution/_DAG/DAG-006/` used as approved graph authority. Historical `DAG-003` material was intentionally not used.
- **Anchor doc:** `Datasheet.md` selected by AUTO role heuristic; `_CONTEXT.md` supplied the strongest explicit anchor identifiers and was used as evidence for anchor rows.
- **Execution docs order:** `Procedure.md`, `Guidance.md`, `Specification.md`, `Datasheet.md`, `_CONTEXT.md`, `_REFERENCES.md`, `Dependencies.csv`.
- **Preserved rows:** 11 existing DAG-002 mirror rows retained with original `DependencyID` values.
- **Normalization:** Existing mirror rows used non-local enum values (`ARCHITECTURE_BASIS`, `PERSISTENCE_CONTRACT`, `REPORTING_PREDECESSOR`, `UNIT_CONTRACT`, `CONTEXT`, `GRAPH_REVIEW`, `DELIVERABLE` as `AnchorType`). These were normalized to canonical local v3.1 enums while preserving original meaning in `Notes`.
- **Additions:** 4 active anchor rows and 2 downstream `ENABLES` rows were added from explicit local/DAG-002 evidence.
- **Warnings:** No active cycles were introduced within this deliverable-local register. No uncertain items were promoted to active prerequisites. No protected standards content, private data, engineering defaults, or professional/code-compliance claims were added.
- **ID-format caveat:** `tools/validation/validate_id_format.sh` appears to enforce legacy three-digit package/deliverable patterns (`PKG-014`, `DEL-014-04`) and does not match current approved IDs such as `PKG-14` and `DEL-14-04`; this is a validator/tooling mismatch, not a local dependency evidence failure.

## Lifecycle Summary

| Status | Rows |
|---|---:|
| ACTIVE | 17 |
| RETIRED | 0 |

| SatisfactionStatus | Rows |
|---|---:|
| SATISFIED | 17 |

| DependencyClass | Rows |
|---|---:|
| ANCHOR | 4 |
| EXECUTION | 13 |

| DependencyType | Rows |
|---|---:|
| CONSTRAINT | 8 |
| ENABLES | 2 |
| INTERFACE | 3 |
| OTHER | 4 |

## Downstream Handoff Notes
- Reconciliation should treat this file as refreshed local evidence, not as aggregate DAG mutation.
- `DEL-14-04-D001` and `DEL-14-04-D002` restate approved DAG-006 consumers as downstream local evidence. They should not be interpreted as DEL-14-04 prerequisites.
- Open issue OI-014 remains relevant: comparison tolerance defaults and mapping workflows are still TBD pending upstream contract/human product decisions.
- Later aggregation must reconcile this refreshed local surface against DAG-002 and any preliminary DAG-003 differences without silently promoting candidate or uncertain edges.

## Run History
- 2026-05-03: synchronized local mirror from approved `DAG-006`; 11 total rows; 11 ACTIVE; 0 CANDIDATE.
- 2026-05-10: TP-DAG-004 dependency surface refresh in UPDATE / CONSERVATIVE / RECONCILIATION mode; decomposition found; DAG-002 authority used; 17 total rows; 17 ACTIVE; 0 RETIRED; warnings: legacy ID-format validator mismatch only.
