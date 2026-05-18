# Dependencies: DEL-07-01 3D viewport and centerline editor

## Extracted Dependency Register
- **Status:** TP-DAG-004_REFRESHED
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer Context:** RECONCILIATION
- **Schema:** Dependencies.csv v3.1
- **Source Surface:** Existing local `Dependencies.csv`, local deliverable source documents, and `execution/_Decomposition/SOFTWARE_DECOMP.md`
- **Local Register:** `Dependencies.csv`
- **Rows:** 15 total; 15 ACTIVE; 0 RETIRED; 0 CANDIDATE.
- **Refreshed:** 2026-05-10

| Class | Direction | Count | Notes |
|---|---:|---:|---|
| EXECUTION | UPSTREAM | 15 | Preserved active upstream dependency surface for DEL-07-01. |

## Run Notes
- Applied runtime defaults: `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`.
- Decomposition path used: `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`.
- Local source review confirmed DEL-07-01 remains a PKG-07 `UX_UI_SLICE` scoped to SOW-020 / OBJ-006, with accepted architecture basis IDs AB-00-01, AB-00-02, AB-00-03, AB-00-05, AB-00-06, AB-00-07, and AB-00-08.
- Conservative refresh preserved existing row identities and did not introduce new inferred execution edges.
- Enum normalization applied for v3.1 validation: legacy execution anchor/type/origin values were normalized to `AnchorType=NOT_APPLICABLE`, `DependencyType=PREREQUISITE`, `Explicitness=IMPLICIT` where inferred, `Origin=EXTRACTED`, and unresolved `SatisfactionStatus=TBD`.
- Existing architecture-basis rows remain evidence-backed by `_CONTEXT.md`; implementation/domain rows remain evidence-backed by the established local register statements.
- [WARNING] FLOATING_NODE: No ACTIVE parent anchor row with `DependencyClass=ANCHOR` and `AnchorType=IMPLEMENTS_NODE` is present in the existing local register. Not auto-added during this refresh because TP-DAG-004 requested a conservative update row and no aggregate graph authority was edited.
- No protected standards text, engineering code values, proprietary data, GUI source, schemas, tests, status files, memory files, DAG files, or coordination files were edited.

## Run History

| Timestamp | Mode | Strictness | Consumer Context | Decomposition | Active Rows | Warnings |
|---|---|---|---|---|---:|---|
| 2026-04-30 | UPDATE | CONSERVATIVE | NONE | `docs/_Decomposition/SOFTWARE_DECOMP.md` | 15 | None recorded in prior run record. |
| 2026-05-10_2243 | UPDATE | CONSERVATIVE | RECONCILIATION | `execution/_Decomposition/SOFTWARE_DECOMP.md` | 15 | FLOATING_NODE |

## Lifecycle Summary
- **ACTIVE:** 15
- **RETIRED:** 0
- **CANDIDATE:** 0
- **SatisfactionStatus:** 7 SATISFIED; 8 TBD
- **RequiredMaturity:** 15 SEMANTIC_READY
- **ProposedMaturity:** 15 SEMANTIC_READY

## Downstream Handoff Notes
- For RECONCILIATION, this register should be treated as a local dependency evidence surface, not an independent graph authority.
- The refresh intentionally preserves the existing `DAG-002-*` dependency identifiers and notes because this worker was not authorized to edit DAG or coordination artifacts.
- Reconciliation should decide whether the missing parent anchor belongs in local v3.1 registers or remains outside DAG-synchronized dependency surfaces.

## Authority Boundary
- Aggregate DAG artifacts remain the sequencing and blocker-computation authority within their approval boundary.
- This local register is a synchronized mirror/evidence surface, not an independent graph authority.
- `CANDIDATE` rows remain non-gating until later RECONCILIATION plus CHANGE approval.
- `PKG-00` architecture-basis rows are preserved here as injected context evidence; `PKG-00` does not receive local dependency registers.
