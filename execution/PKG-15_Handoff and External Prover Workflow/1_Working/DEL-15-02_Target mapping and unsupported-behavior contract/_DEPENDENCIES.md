# Dependencies: DEL-15-02 Target mapping and unsupported-behavior contract

## Generated Dependency Register
- **Status:** TP-DAG-004_REFRESHED_FROM_APPROVED_DAG002_PLUS_LOCAL_EVIDENCE
- **Source of Truth:** `execution/_DAG/DAG-006/DependencyEdges.csv`
- **Local Register:** `Dependencies.csv`
- **Rows:** 18 total; 18 ACTIVE; 0 RETIRED; 0 CANDIDATE.
- **Generated:** 2026-05-03
- **Refreshed:** 2026-05-10

## Authority Boundary
- Aggregate `APPROVED_DAG002` remains the sequencing and blocker-computation authority within its approval boundary.
- This local register is a refreshed evidence surface, not an independent graph authority.
- `CANDIDATE` rows remain non-gating until later RECONCILIATION plus CHANGE approval.
- `PKG-00` architecture-basis rows are preserved here as injected context evidence; `PKG-00` does not receive local dependency registers.
- `DAG-003` was not used as graph authority and was not approved or promoted by this refresh.

## Extracted Dependency Register

| Status | Class | Type | Count |
|---|---|---:|---:|
| ACTIVE | ANCHOR | OTHER | 3 |
| ACTIVE | EXECUTION | CONSTRAINT | 1 |
| ACTIVE | EXECUTION | HANDOVER | 2 |
| ACTIVE | EXECUTION | INTERFACE | 2 |
| ACTIVE | EXECUTION | PREREQUISITE | 10 |

| DependencyID | Class | Direction | Type | Target | Status | Evidence |
|---|---|---|---|---|---|---|
| DAG-002-E0709 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-01 | ACTIVE | `execution/_Decomposition/SOFTWARE_DECOMP.md` |
| DAG-002-E0710 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-02 | ACTIVE | `execution/_Decomposition/SOFTWARE_DECOMP.md` |
| DAG-002-E0711 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-03 | ACTIVE | `execution/_Decomposition/SOFTWARE_DECOMP.md` |
| DAG-002-E0712 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-04 | ACTIVE | `execution/_Decomposition/SOFTWARE_DECOMP.md` |
| DAG-002-E0713 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-06 | ACTIVE | `execution/_Decomposition/SOFTWARE_DECOMP.md` |
| DAG-002-E0714 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-07 | ACTIVE | `execution/_Decomposition/SOFTWARE_DECOMP.md` |
| DAG-002-E0715 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-08 | ACTIVE | `execution/_Decomposition/SOFTWARE_DECOMP.md` |
| DAG-002-E0805 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-15-01 | ACTIVE | `execution/_DAG/DAG-006/DAG-002_EdgeDispositionReview.md` |
| DAG-002-E0806 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-10-02 | ACTIVE | `execution/_DAG/DAG-006/DAG-002_EdgeDispositionReview.md` |
| DAG-002-E0807 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-10-03 | ACTIVE | `execution/_DAG/DAG-006/DAG-002_EdgeDispositionReview.md` |
| DAG-002-E0808 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-12-02 | ACTIVE | `execution/_DAG/DAG-006/DAG-002_EdgeDispositionReview.md` |
| DAG-002-E0809 | EXECUTION | UPSTREAM | INTERFACE | DEL-13-04 | ACTIVE | `execution/_DAG/DAG-006/DAG-002_EdgeDispositionReview.md` |
| DAG-002-E0810 | EXECUTION | UPSTREAM | INTERFACE | DEL-14-05 | ACTIVE | `execution/_DAG/DAG-006/DAG-002_EdgeDispositionReview.md` |
| DEL-15-02-A001 | ANCHOR | UPSTREAM | OTHER | PKG-15 | ACTIVE | `Datasheet.md` |
| DEL-15-02-A002 | ANCHOR | UPSTREAM | OTHER | SOW-074 | ACTIVE | `_CONTEXT.md` |
| DEL-15-02-A003 | ANCHOR | UPSTREAM | OTHER | OBJ-017 | ACTIVE | `_CONTEXT.md` |
| DEL-15-02-D001 | EXECUTION | DOWNSTREAM | HANDOVER | DEL-15-03 | ACTIVE | `execution/_DAG/DAG-006/DependencyEdges.csv` |
| DEL-15-02-D002 | EXECUTION | DOWNSTREAM | HANDOVER | DEL-15-04 | ACTIVE | `execution/_DAG/DAG-006/DependencyEdges.csv` |

## Run Notes

- `MODE=UPDATE`; `STRICTNESS=CONSERVATIVE`; `CONSUMER_CONTEXT=RECONCILIATION`.
- `RUN_ROOT=/Users/ryan/ai-env/projects/chirality-piping/execution`.
- `DECOMPOSITION_PATH=/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`; decomposition located and used for PKG-15, DEL-15-02, SOW-074, and OBJ-017 validation.
- `ANCHOR_DOC=AUTO` resolved to `Datasheet.md`; execution documents scanned were `Procedure.md`, `Specification.md`, `Guidance.md`, `_CONTEXT.md`, `_REFERENCES.md`, existing `Dependencies.csv`, and approved `execution/_DAG/DAG-002` evidence.
- Existing 13 active DAG-002 mirror rows were preserved as active evidence by ID, target, statement, evidence, and authority note. Local enum fields were normalized for `Dependencies.csv` v3.1 compatibility; original DAG-002 values are retained in each row `Notes`.
- Added 3 active anchor rows for PKG-15, SOW-074, and OBJ-017 because the deliverable identity, scope item, and objective are explicit in local source documents and decomposition.
- Added 2 active downstream handoff rows for DEL-15-03 and DEL-15-04 because approved DAG-006 records those deliverables as consumers of DEL-15-02.
- Historical `DAG-003` content was not used as graph authority.
- Enum validation passes for current CSV values after local normalization. Reconciliation should still recognize that original DAG-002 graph-review classifications are preserved in row notes, not as active enum values.
- [WARNING] ID_FORMAT_TOOL_STALE: `validate_id_format.sh` expects legacy three-digit package/deliverable formats and rejects project-current IDs such as `PKG-15` and `DEL-15-02`.
- No `FLOATING_NODE` warning: one active `IMPLEMENTS_NODE` anchor is present.

## Run History

- 2026-05-03: synchronized local dependency mirror from approved `execution/_DAG/DAG-006/DependencyEdges.csv`; 13 rows, all ACTIVE.
- 2026-05-10: TP-DAG-004 dependency surface refresh; mode UPDATE; strictness CONSERVATIVE; consumer RECONCILIATION; decomposition available; warning `ID_FORMAT_TOOL_STALE`; active counts: 3 ANCHOR, 15 EXECUTION.

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 18 |
| RETIRED | 0 |
| CANDIDATE | 0 |

| SatisfactionStatus | Count |
|---|---:|
| SATISFIED | 12 |
| TBD | 6 |

## Downstream Handoff Notes

- Reconciliation should treat the 13 preserved DAG-002 rows as approved graph-authority mirror evidence, not as newly promoted local findings.
- Reconciliation should note that the 2 downstream rows are local-surface restatements of active DAG-002 consumer edges, not a DAG-003 approval signal.
- ID-format warnings appear tool/schema-alignment related and should be handled centrally rather than by local row deletion or unauthorized graph mutation.
