# Dependencies: DEL-13-04 Physical-to-analytical transformation contract

## Coordination Mode
- **Mode:** FULL_GRAPH
- **Graph Authority:** `execution/_DAG/DAG-006/` is the approved legacy graph pending `DAG-007` canonical approval.
- **Authority Boundary:** Candidate/non-gating edges are not represented through `Status=CANDIDATE` in current canonical registers.

## Declared Upstream Dependencies
- None recorded.

## Declared Downstream Dependencies
- None recorded.

## Extracted Dependency Register
- **Local Register:** `Dependencies.csv`
- **Register schema version:** `v3.1`
- **Canonicalized:** 2026-06-16
- **Rows:** 20 total; 20 ACTIVE; 0 RETIRED.
- **Classes:** ANCHOR=2; EXECUTION=18.
- **Candidate rows moved to worklist:** 0.

### Compact Active Register

| DependencyID | Class | Direction | Type | Target | Evidence |
|---|---|---|---|---|---|
| DEL-13-04-A001 | ANCHOR | UPSTREAM | OTHER | SOW-066 Physical-to-analytical deterministic transform | `_CONTEXT.md` Scope Detail |
| DEL-13-04-A002 | ANCHOR | UPSTREAM | OTHER | OBJ-014 Schema-backed design model traceability | `Datasheet.md` Identification and Attributes |
| DAG-002-E0660 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-01 Architecture decision record baseline | `execution/_DAG/DAG-002/DependencyEdges.csv` DAG-002-E0660 |
| DAG-002-E0661 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-02 Repository and module boundary architecture | `execution/_DAG/DAG-002/DependencyEdges.csv` DAG-002-E0661 |
| DAG-002-E0662 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-03 Application service command-query-job model | `execution/_DAG/DAG-002/DependencyEdges.csv` DAG-002-E0662 |
| DAG-002-E0663 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-04 Persistence and schema versioning architecture | `execution/_DAG/DAG-002/DependencyEdges.csv` DAG-002-E0663 |
| DAG-002-E0664 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-06 Diagnostics, warning, and result-envelope contract | `execution/_DAG/DAG-002/DependencyEdges.csv` DAG-002-E0664 |
| DAG-002-E0665 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-07 API boundary and adapter contract map | `execution/_DAG/DAG-002/DependencyEdges.csv` DAG-002-E0665 |
| DAG-002-E0666 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-08 Layered software test and acceptance strategy | `execution/_DAG/DAG-002/DependencyEdges.csv` DAG-002-E0666 |
| DAG-002-E0772 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-01 Canonical domain model schema | `Procedure.md` Define Contract Inputs |
| DEV-001-PKG13-DEL1304-DEL0205 | EXECUTION | UPSTREAM | INTERFACE | DEL-02-05 Project persistence and round-trip serialization | `_DEPENDENCIES.md` DEV-001 Stage 2 finding resolution |
| DAG-002-E0773 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-13-01 Design knowledge schema and provenance model | `Procedure.md` Define Contract Inputs |
| DAG-002-E0774 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-13-02 Constraint entity and provenance model | `Procedure.md` Define Contract Inputs |
| DAG-002-E0775 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-13-03 Constraint validation engine | `Procedure.md` Define Contract Inputs |
| DAG-002-E0776 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-04-01 3D frame stiffness kernel | `Specification.md` DEL-13-04-REQ-007 |
| DAG-002-E0777 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-04-03 Linear support and restraint models | `Procedure.md` Define Contract Inputs |
| DAG-002-E0778 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-05-01 Primitive load case engine | `Procedure.md` Define Contract Inputs |
| DEL-13-04-D001 | EXECUTION | DOWNSTREAM | ENABLES | DEL-15-02 Target mapping and unsupported-behavior contract | `execution/_DAG/DAG-002/DependencyEdges.csv` DAG-002-E0809 |
| DEL-13-04-D002 | EXECUTION | DOWNSTREAM | ENABLES | DEL-15-03 Downstream modeling export workflow | `execution/_DAG/DAG-002/DependencyEdges.csv` DAG-002-E0816 |
| DEL-13-04-D003 | EXECUTION | DOWNSTREAM | ENABLES | DEL-07-08 Design-authoring state and comparison workspace | `execution/_DAG/DAG-002/DependencyEdges.csv` DAG-002-E0846 |

## Canonical Dependency Types
- `ENABLES`: 3
- `INTERFACE`: 1
- `OTHER`: 2
- `PREREQUISITE`: 14

## Run Notes
- Core enum fields conform to the canonical Chirality dependency model.
- Legacy project-specific labels are preserved in `Notes` as `legacy_*` fields.
- Candidate rows remain non-gating in the candidate worklist and require explicit human approval plus graph revalidation before promotion.
- This dependency refresh does not authorize implementation, lifecycle promotion, release claims, professional approval, certification, sealing, authentication, or code-compliance claims.
- Defaults applied: `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`, `MODE=UPDATE`, `STRICTNESS=CONSERVATIVE`, `CONSUMER_CONTEXT=RECONCILIATION`, `ARCHITECTURE_BASIS_POLICY=PKG00_CONSISTENCY_TRACKERS`.
- Decomposition path used: `execution/_Decomposition/SOFTWARE_DECOMP.md`; anchor validation resolved SOW-066 and OBJ-014.
- PKG-00 rows reviewed: 7; PKG-00 rows changed in `Dependencies.csv`: 7; retained as architecture-consistency trackers.
- Warnings: none.

## Run History

| Timestamp | Mode | Strictness | Decomposition | Warnings | ACTIVE Rows |
|---|---|---|---|---|---:|
| 2026-06-16 | UPDATE | CONSERVATIVE | `execution/_Decomposition/SOFTWARE_DECOMP.md` | None | 20 |

## Lifecycle Summary

| Lifecycle Field | Breakdown |
|---|---|
| Status | ACTIVE: 20; RETIRED: 0 |
| DependencyClass | ANCHOR: 2; EXECUTION: 18 |
| Direction | UPSTREAM: 17; DOWNSTREAM: 3 |
| SatisfactionStatus | NOT_APPLICABLE: 2; SATISFIED: 7; PENDING: 10; TBD: 1 |
| Origin | EXTRACTED: 10; DECLARED: 10 |

## Downstream Handoff Notes

- For RECONCILIATION, the three downstream `ENABLES` rows expose known consumers of the transform output; they are not new blocker authority.
- PRD-specific transform particulars, runtime GUI/API integration, and professional acceptance claims remain out of scope or `TBD` per local source evidence.
