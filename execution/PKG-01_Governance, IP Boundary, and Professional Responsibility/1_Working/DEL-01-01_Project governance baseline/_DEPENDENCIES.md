# Dependencies: DEL-01-01 Project governance baseline

## Generated Dependency Register
- **Status:** REFRESHED_FROM_LOCAL_EVIDENCE
- **Previous Source:** `execution/_DAG/DAG-002/DependencyEdges.csv`
- **Local Register:** `Dependencies.csv`
- **Rows:** 13 total; 13 ACTIVE; 0 RETIRED; 0 CANDIDATE.
- **Refreshed:** 2026-05-10

## Authority Boundary
- Aggregate `DAG-002` remains the sequencing and blocker-computation authority within its approval boundary.
- This local register is refreshed dependency evidence for later reconciliation, not independent aggregate graph authority.
- `DAG-003` remains preliminary and not approval-ready.
- `CANDIDATE` rows remain non-gating until later RECONCILIATION plus CHANGE approval.
- `PKG-00` architecture-basis rows are preserved here as injected context evidence; `PKG-00` does not receive local dependency registers in TP-DAG-004.

## Extracted Dependency Register

| Class | Active | Retired | Notes |
|---|---:|---:|---|
| ANCHOR / IMPLEMENTS_NODE | 1 | 0 | Parent anchor to DEL-01-01 decomposition node. |
| ANCHOR / TRACES_TO_REQUIREMENT | 4 | 0 | Explicit traces to SOW-001, SOW-048, OBJ-001, and OBJ-002. |
| EXECUTION / DELIVERABLE | 4 | 0 | Preserved PKG-00 architecture-basis prerequisites from `_CONTEXT.md`; normalized to v3.1 enum vocabulary. |
| EXECUTION / DOCUMENT | 4 | 0 | Explicit governing-document prerequisites from Procedure, Datasheet, and `_REFERENCES.md`. |

## Run Notes

- TaskSkill: `dependency-extract`
- ScopePath: `execution/PKG-01_Governance, IP Boundary, and Professional Responsibility/1_Working/DEL-01-01_Project governance baseline`
- DeliverableID: `DEL-01-01`
- PackageID: `PKG-01`
- Mode: `UPDATE`
- Strictness: `CONSERVATIVE`
- Consumer context: `RECONCILIATION`
- Decomposition path: `execution/_Decomposition/SOFTWARE_DECOMP.md` located and used for identifier/label validation.
- Source docs defaulted by role: `Datasheet.md` as anchor source; `Specification.md`, `Procedure.md`, `Guidance.md`, `_CONTEXT.md`, and `_REFERENCES.md` as execution/context sources.
- Existing `DAG-002` mirror rows were treated as prior local evidence. The four architecture-basis rows were preserved by `DependencyID`, refreshed as ACTIVE, and normalized from legacy mirror values into canonical v3.1 enums: `AnchorType=NOT_APPLICABLE`, `DependencyType=PREREQUISITE`, `Origin=EXTRACTED`.
- No `PKG-00` local register was created or edited.
- No source documents, lifecycle files, aggregate DAG files, schemas, tests, blocker queues, or coordination files were edited by this run.
- Warnings: `[WARNING] ID_FORMAT_TOOL_STALE`: `tools/validation/validate_id_format.sh` expects legacy `DEL-001-01` / `PKG-001` patterns and rejects current register identifiers such as `DEL-01-01` and `PKG-01`; IDs were preserved from the canonical project registers.

## Run History

- 2026-05-03: Local register synchronized from `DAG-002` mirror; 4 ACTIVE rows.
- 2026-05-10: TP-DAG-004 dependency-surface refresh in `UPDATE` / `CONSERVATIVE` / `RECONCILIATION` mode; 13 ACTIVE rows; 0 RETIRED rows; 0 candidate rows; decomposition available; warning recorded for stale ID-format helper patterns.

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 13 |
| RETIRED | 0 |

| SatisfactionStatus | Count |
|---|---:|
| SATISFIED | 13 |

## Downstream Handoff Notes

- This refresh is suitable as DEL-local evidence for a later graph reconciliation pass.
- The refreshed rows do not approve `DAG-003`, promote candidate edges, change lifecycle state, or authorize Type 2 product implementation dispatch.
- Later reconciliation should compare these local anchor/document/prerequisite rows against `DAG-002` and preliminary `DAG-003` without silently promoting non-gating evidence into aggregate graph authority.
