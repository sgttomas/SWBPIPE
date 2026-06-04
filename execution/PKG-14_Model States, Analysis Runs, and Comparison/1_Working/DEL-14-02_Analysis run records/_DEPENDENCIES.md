# Dependencies: DEL-14-02 Analysis run records

## Generated Dependency Register
- **Status:** REFRESHED_TP_DAG_004_CONSERVATIVE
- **Approved graph authority:** `execution/_DAG/DAG-006/`
- **Source of Truth:** `execution/_DAG/DAG-006/DependencyEdges.csv` for approved execution rows.
- **Local Register:** `Dependencies.csv`
- **Rows:** 14 total; 14 ACTIVE; 0 RETIRED; 0 CANDIDATE.
- **Generated:** 2026-05-03
- **Refreshed:** 2026-05-10

## Authority Boundary
- Aggregate `DAG-002` remains the approved sequencing and blocker-computation authority within its approval boundary.
- This local register is a refreshed deliverable-local evidence surface, not an independent graph authority.
- `DAG-003` is not approved or promoted by this refresh.
- `PKG-00` architecture-basis rows are preserved here as injected context evidence; `PKG-00` does not receive local dependency registers.
- Potential downstream consumers already represented in `DAG-002` are recorded in handoff notes rather than duplicated as new ACTIVE local execution rows.

## Extracted Dependency Register

| Count | Scope |
|---:|---|
| 14 | Total rows |
| 14 | ACTIVE rows |
| 2 | ANCHOR rows |
| 12 | EXECUTION rows |
| 0 | RETIRED rows |
| 0 | CANDIDATE rows |

| DependencyID | Class | Direction | Type | Target | Status | Evidence |
|---|---|---|---|---|---|---|
| DAG-002-E0674 | EXECUTION | UPSTREAM | ARCHITECTURE_BASIS | DEL-00-01 | ACTIVE | `execution/_Decomposition/SOFTWARE_DECOMP.md` |
| DAG-002-E0675 | EXECUTION | UPSTREAM | ARCHITECTURE_BASIS | DEL-00-02 | ACTIVE | `execution/_Decomposition/SOFTWARE_DECOMP.md` |
| DAG-002-E0676 | EXECUTION | UPSTREAM | ARCHITECTURE_BASIS | DEL-00-03 | ACTIVE | `execution/_Decomposition/SOFTWARE_DECOMP.md` |
| DAG-002-E0677 | EXECUTION | UPSTREAM | ARCHITECTURE_BASIS | DEL-00-04 | ACTIVE | `execution/_Decomposition/SOFTWARE_DECOMP.md` |
| DAG-002-E0678 | EXECUTION | UPSTREAM | ARCHITECTURE_BASIS | DEL-00-06 | ACTIVE | `execution/_Decomposition/SOFTWARE_DECOMP.md` |
| DAG-002-E0679 | EXECUTION | UPSTREAM | ARCHITECTURE_BASIS | DEL-00-07 | ACTIVE | `execution/_Decomposition/SOFTWARE_DECOMP.md` |
| DAG-002-E0680 | EXECUTION | UPSTREAM | ARCHITECTURE_BASIS | DEL-00-08 | ACTIVE | `execution/_Decomposition/SOFTWARE_DECOMP.md` |
| DAG-002-E0783 | EXECUTION | UPSTREAM | PERSISTENCE_CONTRACT | DEL-14-01 | ACTIVE | `execution/_DAG/DAG-006/DAG-002_EdgeDispositionReview.md` |
| DAG-002-E0784 | EXECUTION | UPSTREAM | LOAD_STRESS_PREDECESSOR | DEL-05-04 | ACTIVE | `execution/_DAG/DAG-006/DAG-002_EdgeDispositionReview.md` |
| DAG-002-E0785 | EXECUTION | UPSTREAM | REPORTING_PREDECESSOR | DEL-08-02 | ACTIVE | `execution/_DAG/DAG-006/DAG-002_EdgeDispositionReview.md` |
| DAG-002-E0786 | EXECUTION | UPSTREAM | REPORTING_PREDECESSOR | DEL-08-04 | ACTIVE | `execution/_DAG/DAG-006/DAG-002_EdgeDispositionReview.md` |
| DAG-002-E0787 | EXECUTION | UPSTREAM | PERSISTENCE_CONTRACT | DEL-02-05 | ACTIVE | `execution/_DAG/DAG-006/DAG-002_EdgeDispositionReview.md` |
| DEL-14-02-A001 | ANCHOR | UPSTREAM | OTHER | SOW-072 | ACTIVE | `_CONTEXT.md` |
| DEL-14-02-A002 | ANCHOR | UPSTREAM | OTHER | OBJ-016 | ACTIVE | `_CONTEXT.md` |

## Run Notes
- **TaskSkill:** `dependency-extract`
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer context:** RECONCILIATION
- **Run root:** `/Users/ryan/ai-env/projects/chirality-piping/execution`
- **Decomposition path:** `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`
- **Graph authority consulted:** `execution/_DAG/DAG-006/DependencyEdges.csv` and `execution/_DAG/DAG-006/DeliverableNodes.csv`
- **DAG-003 treatment:** not used as approved authority; not approved or promoted.
- **Anchor document:** `_CONTEXT.md`
- **Execution documents read:** `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_SEMANTIC.md`, `_REFERENCES.md`
- **Preserved rows:** all 12 existing DAG-002 mirror execution rows remain ACTIVE.
- **Added rows:** two ACTIVE anchor rows for explicit SOW-072 and OBJ-016 traceability.
- **Candidate treatment:** no new candidate rows were added. Potential downstream consumers DEL-14-04, DEL-14-05, DEL-15-01, and DEL-08-06 are already represented in `DAG-002` as rows from those deliverables to DEL-14-02; this local refresh does not duplicate them as new ACTIVE rows.
- **Warnings:** Existing DAG-002 mirror rows use graph-authority enum values outside the narrow local `validate_enum.py` enum set (`DELIVERABLE` as `AnchorType`, `ARCHITECTURE_BASIS`, `PERSISTENCE_CONTRACT`, `LOAD_STRESS_PREDECESSOR`, `REPORTING_PREDECESSOR`, `GRAPH_REVIEW`, `CONTEXT`, `INFERRED_DIRECT`, and `UNKNOWN`). They were preserved because this run is conservative and `DAG-002` is the approved graph authority.
- **Validation warning:** `tools/validation/validate_id_format.sh` rejects current project IDs such as `DEL-14-02` and `PKG-14` because the helper expects legacy three-digit ID formats. Approved project identifiers were preserved.
- **Tree x DAG integrity:** one ACTIVE parent anchor found; no `FLOATING_NODE`; no `AMBIGUOUS_ANCHOR`.

## Run History

| Timestamp | Mode | Strictness | Decomposition | Graph authority | Active rows | Warnings |
|---|---|---|---|---|---:|---|
| 2026-05-03 | SYNCHRONIZE | N/A | `execution/_Decomposition/SOFTWARE_DECOMP.md` | `execution/_DAG/DAG-006/` | 12 | Local register synchronized from approved DAG-006 mirror. |
| 2026-05-10 23:46 MDT | UPDATE | CONSERVATIVE | `execution/_Decomposition/SOFTWARE_DECOMP.md` | `execution/_DAG/DAG-006/` | 14 | Preserved DAG-002 mirror enum values outside narrow validator, including execution-row `AnchorType=DELIVERABLE`; ID helper expects legacy three-digit IDs; no DAG-003 promotion. |

## Lifecycle Summary

| Status | Rows |
|---|---:|
| ACTIVE | 14 |
| RETIRED | 0 |
| CANDIDATE | 0 |

| SatisfactionStatus | Rows |
|---|---:|
| SATISFIED | 7 |
| UNKNOWN | 5 |
| PENDING | 2 |

## Downstream Handoff Notes
- RECONCILIATION should treat this file as a deliverable-local evidence refresh only.
- Approved execution edges remain the 12 preserved DAG-002 mirror rows; the two new anchor rows are Tree traceability, not new execution blockers.
- Later graph reconciliation may compare the local anchor rows against decomposition/register traceability, but should not infer approval of DAG-003 from this refresh.
