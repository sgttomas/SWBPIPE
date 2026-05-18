# Dependencies: DEL-13-01 Design knowledge schema and provenance model

## Generated Dependency Register
- **Status:** REFRESHED_TP_DAG_004_CONSERVATIVE
- **Approved graph authority:** `execution/_DAG/DAG-002/`
- **Source of Truth:** `execution/_DAG/DAG-002/DependencyEdges.csv` for approved execution rows.
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
| DAG-002-E0639 | EXECUTION | UPSTREAM | ARCHITECTURE_BASIS | DEL-00-01 | ACTIVE | `execution/_Decomposition/SOFTWARE_DECOMP.md` |
| DAG-002-E0640 | EXECUTION | UPSTREAM | ARCHITECTURE_BASIS | DEL-00-02 | ACTIVE | `execution/_Decomposition/SOFTWARE_DECOMP.md` |
| DAG-002-E0641 | EXECUTION | UPSTREAM | ARCHITECTURE_BASIS | DEL-00-03 | ACTIVE | `execution/_Decomposition/SOFTWARE_DECOMP.md` |
| DAG-002-E0642 | EXECUTION | UPSTREAM | ARCHITECTURE_BASIS | DEL-00-04 | ACTIVE | `execution/_Decomposition/SOFTWARE_DECOMP.md` |
| DAG-002-E0643 | EXECUTION | UPSTREAM | ARCHITECTURE_BASIS | DEL-00-06 | ACTIVE | `execution/_Decomposition/SOFTWARE_DECOMP.md` |
| DAG-002-E0644 | EXECUTION | UPSTREAM | ARCHITECTURE_BASIS | DEL-00-07 | ACTIVE | `execution/_Decomposition/SOFTWARE_DECOMP.md` |
| DAG-002-E0645 | EXECUTION | UPSTREAM | ARCHITECTURE_BASIS | DEL-00-08 | ACTIVE | `execution/_Decomposition/SOFTWARE_DECOMP.md` |
| DAG-002-E0758 | EXECUTION | UPSTREAM | DOMAIN_MODEL | DEL-02-01 | ACTIVE | `execution/_DAG/DAG-002/DAG-002_EdgeDispositionReview.md` |
| DAG-002-E0759 | EXECUTION | UPSTREAM | UNIT_CONTRACT | DEL-02-02 | ACTIVE | `execution/_DAG/DAG-002/DAG-002_EdgeDispositionReview.md` |
| DEV-001-PKG13-DEL1301-DEL0205 | EXECUTION | UPSTREAM | PERSISTENCE_CONTRACT | DEL-02-05 | ACTIVE | `Dependencies.csv`; DEV-001 Stage 2 finding resolution |
| DAG-002-E0760 | EXECUTION | UPSTREAM | GOVERNANCE_PREDECESSOR | DEL-01-02 | ACTIVE | `execution/_DAG/DAG-002/DAG-002_EdgeDispositionReview.md` |
| DAG-002-E0761 | EXECUTION | UPSTREAM | GOVERNANCE_PREDECESSOR | DEL-01-04 | ACTIVE | `execution/_DAG/DAG-002/DAG-002_EdgeDispositionReview.md` |
| DEL-13-01-A001 | ANCHOR | UPSTREAM | OTHER | SOW-067 | ACTIVE | `_CONTEXT.md` |
| DEL-13-01-A002 | ANCHOR | UPSTREAM | OTHER | OBJ-014 | ACTIVE | `_CONTEXT.md` |

## Run Notes
- **TaskSkill:** `dependency-extract`
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer context:** RECONCILIATION
- **Run root:** `/Users/ryan/ai-env/projects/chirality-piping/execution`
- **Decomposition path:** `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`
- **Graph authority consulted:** `execution/_DAG/DAG-002/DependencyEdges.csv` and `execution/_DAG/DAG-002/DeliverableNodes.csv`
- **DAG-003 treatment:** not used as approved authority; not approved or promoted.
- **Anchor document:** `_CONTEXT.md`
- **Execution documents read:** `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, `_REFERENCES.md`
- **Preserved rows:** all 11 existing DAG-002 mirror execution rows remain ACTIVE.
- **Added rows:** two ACTIVE anchor rows for explicit SOW-067 and OBJ-014 traceability.
- **DEV-001 Stage 2 addendum:** added one package-local ACTIVE DEL-02-05 persistence/hash/round-trip evidence row for design knowledge records. This row is reconciliation evidence only and does not promote DAG-003 or lifecycle state.
- **Candidate treatment:** no new candidate rows were added. Downstream consumers DEL-13-02, DEL-13-03, DEL-13-04, DEL-16-01, and DEL-07-08 are already represented in `DAG-002` as rows from those deliverables to DEL-13-01; this local refresh does not duplicate them as new ACTIVE rows.
- **Warnings:** Existing DAG-002 mirror rows use graph-authority enum values outside the narrow local `validate_enum.py` enum set (`DELIVERABLE` as `AnchorType`, `ARCHITECTURE_BASIS`, `DOMAIN_MODEL`, `UNIT_CONTRACT`, `GOVERNANCE_PREDECESSOR`, `GRAPH_REVIEW`, and `INFERRED_DIRECT`). They were preserved because this run is conservative and `DAG-002` is the approved graph authority.
- **Validation warning:** `tools/validation/validate_id_format.sh` rejects current project IDs such as `DEL-13-01` and `PKG-13` because the helper expects legacy three-digit ID formats. Approved project identifiers were preserved.
- **Tree x DAG integrity:** one ACTIVE parent anchor found; no `FLOATING_NODE`; no `AMBIGUOUS_ANCHOR`.

## Run History

| Timestamp | Mode | Strictness | Decomposition | Graph authority | Active rows | Warnings |
|---|---|---|---|---|---:|---|
| 2026-05-03 | SYNCHRONIZE | N/A | `execution/_Decomposition/SOFTWARE_DECOMP.md` | `execution/_DAG/DAG-002/` | 11 | Local register synchronized from approved DAG-002 mirror. |
| 2026-05-10 23:38 MDT | UPDATE | CONSERVATIVE | `execution/_Decomposition/SOFTWARE_DECOMP.md` | `execution/_DAG/DAG-002/` | 13 | Preserved DAG-002 mirror enum values outside narrow validator, including execution-row `AnchorType=DELIVERABLE`; ID helper expects legacy three-digit IDs; no DAG-003 promotion. |
| 2026-05-16 MDT | DEV-001_STAGE2_FINDING_RESOLUTION | PACKAGE_SCOPED | accepted PKG-02 contract | local package evidence only | 14 | Added DEL-02-05 local dependency evidence for persistence/hash/round-trip coverage; HumanDisposition remains TBD; no DAG or lifecycle files changed. |

## Lifecycle Summary

| Status | Rows |
|---|---:|
| ACTIVE | 14 |
| RETIRED | 0 |
| CANDIDATE | 0 |

| SatisfactionStatus | Rows |
|---|---:|
| SATISFIED | 7 |
| UNKNOWN | 4 |
| PENDING | 2 |
| TBD | 1 |

## Downstream Handoff Notes
- RECONCILIATION should treat this file as a deliverable-local evidence refresh only.
- Approved DAG-002 execution edges remain the 11 preserved mirror rows; the DEV-001 DEL-02-05 row is package-local reconciliation evidence, and the two anchor rows are Tree traceability rather than new graph authority.
- Later graph reconciliation may compare the local anchor rows against decomposition/register traceability, but should not infer approval of DAG-003 from this refresh.
