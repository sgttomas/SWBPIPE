# Dependencies: DEL-04-05 Sparse solver performance harness

## Generated Dependency Register
- **Status:** TP-DAG-004_REFRESHED_DAG002_MIRROR
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer Context:** RECONCILIATION
- **Source of Truth:** `execution/_DAG/DAG-006/DependencyEdges.csv` remains the approved coordination mirror source.
- **Local Register:** `Dependencies.csv`
- **Rows:** 7 total; 7 ACTIVE; 0 RETIRED; 0 CANDIDATE.
- **Refreshed:** 2026-05-10

## Extracted Dependency Register

| DependencyID | Class | Direction | Type | Target | Status | Satisfaction |
|---|---|---|---|---|---|---|
| DAG-002-E0120 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-01 Architecture decision record baseline | ACTIVE | SATISFIED |
| DAG-002-E0121 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-02 Repository and module boundary architecture | ACTIVE | SATISFIED |
| DAG-002-E0122 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-03 Application service command-query-job model | ACTIVE | SATISFIED |
| DAG-002-E0123 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-06 Diagnostics, warning, and result-envelope contract | ACTIVE | SATISFIED |
| DAG-002-E0124 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-08 Layered software test and acceptance strategy | ACTIVE | SATISFIED |
| DAG-002-E0444 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-04-01 3D frame stiffness kernel | ACTIVE | TBD |
| DAG-002-E0445 | EXECUTION | UPSTREAM | INTERFACE | DEL-04-06 Solver diagnostics and singularity detection | ACTIVE | TBD |

## Run Notes

- **Chosen anchor doc:** `Datasheet.md`.
- **Chosen execution docs:** `Specification.md`, `Procedure.md`, `Guidance.md`, `_CONTEXT.md`, `_REFERENCES.md`.
- **Decomposition path:** `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`.
- **Dispatch row:** TP-DAG-004 row for `DEL-04-05`, `PackageID=PKG-04`, `Mode=UPDATE`, `Strictness=CONSERVATIVE`, `ConsumerContext=RECONCILIATION`.
- Existing `Dependencies.csv` rows were treated as prior evidence from the approved `DAG-002` mirror. No aggregate DAG authority, lifecycle state, source document, status, memory, code, schema, test, or coordination artifact was changed.
- Source review confirms local evidence for SOW-035, OBJ-003, OBJ-008, five SCA-001 architecture-basis constraints, the solver-kernel predecessor, and the diagnostics/result-envelope predecessor.
- Conservative refresh preserved 7 ACTIVE mirror rows, updated `LastSeen` to 2026-05-10, and normalized non-canonical mirror enum values (`AnchorType`, `DependencyType`, `Explicitness`, `Origin`, `SatisfactionStatus`) to v3.1 write-form enums.
- Original mirror origin semantics were `CONTEXT` for SCA-001 architecture-basis rows and `DECOMPOSITION` for inferred solver predecessor rows; v3.1 write form records these as `Origin=EXTRACTED`.
- No protected standards content, private project data, engineering default values, benchmark thresholds, certification claims, or code-compliance claims were introduced.
- `[WARNING] FLOATING_NODE`: no ACTIVE `DependencyClass=ANCHOR` / `AnchorType=IMPLEMENTS_NODE` row is present because this local register is preserving the `DAG-002` execution-edge mirror for reconciliation.

## Run History

| Timestamp | Mode | Strictness | Consumer | Rows | Validation | Notes |
|---|---|---|---|---:|---|---|
| 2026-04-30 10:15 | UPDATE | CONSERVATIVE | NONE | 5 | PASS | Original dependency-extract run emitted local anchor/objective/execution setup rows. |
| 2026-05-03 | SYNC | N/A | DAG-002 | 7 | PASS | Local register synchronized from approved `DAG-006` mirror. |
| 2026-05-10 22:16 | UPDATE | CONSERVATIVE | RECONCILIATION | 7 | PASS | TP-DAG-004 refresh preserved active mirror rows, updated run notes/history, and kept candidate/lifecycle authority unchanged. |

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 7 |
| RETIRED | 0 |
| CANDIDATE | 0 |

| SatisfactionStatus | Count |
|---|---:|
| SATISFIED | 5 |
| TBD | 2 |

## Authority Boundary

- Aggregate `DAG-002` remains the sequencing and blocker-computation authority within its approval boundary.
- This local register is a synchronized mirror/evidence surface, not an independent graph authority.
- `CANDIDATE` rows remain non-gating until later RECONCILIATION plus CHANGE approval.
- `PKG-00` architecture-basis rows are preserved here as injected context evidence; `PKG-00` does not receive local dependency registers.

## Downstream Handoff Notes

- RECONCILIATION should treat this row as a refreshed `DAG002_MIRROR` surface with 7 ACTIVE execution rows and no candidate promotions.
- The two non-architecture predecessor rows remain `SatisfactionStatus=TBD` because local refresh did not verify implementation maturity of `DEL-04-01` or `DEL-04-06`.
- The missing parent-anchor warning is a local skill-integrity note, not a graph-change request.
