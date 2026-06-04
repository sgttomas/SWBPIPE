# Dependencies: DEL-03-01 Material library schema with provenance

## Extracted Dependency Register

- **Status:** REFRESHED_TP_DAG_004
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer Context:** RECONCILIATION
- **Source of Truth:** deliverable-local extraction from assigned folder plus `execution/_Decomposition/SOFTWARE_DECOMP.md`
- **Local Register:** `Dependencies.csv`
- **Rows:** 12 total; 12 ACTIVE; 0 RETIRED.
- **Generated:** 2026-05-10

| DependencyClass | Count |
|---|---:|
| ANCHOR | 2 |
| EXECUTION | 10 |

| Direction | Count |
|---|---:|
| UPSTREAM | 12 |
| DOWNSTREAM | 0 |

## Compact Active Rows

| DependencyID | Class | Type | Target | Status | Satisfaction |
|---|---|---|---|---|---|
| TP-DAG-004-DEL-03-01-A001 | ANCHOR | OTHER | PKG-03 | ACTIVE | NOT_APPLICABLE |
| TP-DAG-004-DEL-03-01-A002 | ANCHOR | OTHER | SOW-017 | ACTIVE | NOT_APPLICABLE |
| DAG-002-E0052 | EXECUTION | CONSTRAINT | DEL-00-01 | ACTIVE | SATISFIED |
| DAG-002-E0053 | EXECUTION | CONSTRAINT | DEL-00-02 | ACTIVE | SATISFIED |
| DAG-002-E0054 | EXECUTION | CONSTRAINT | DEL-00-04 | ACTIVE | SATISFIED |
| DAG-002-E0055 | EXECUTION | CONSTRAINT | DEL-00-06 | ACTIVE | SATISFIED |
| DAG-002-E0056 | EXECUTION | CONSTRAINT | DEL-00-07 | ACTIVE | SATISFIED |
| DAG-002-E0057 | EXECUTION | CONSTRAINT | DEL-00-08 | ACTIVE | SATISFIED |
| DAG-002-E0401 | EXECUTION | PREREQUISITE | DEL-02-01 | ACTIVE | TBD |
| DAG-002-E0402 | EXECUTION | PREREQUISITE | DEL-02-02 | ACTIVE | TBD |
| DAG-002-E0403 | EXECUTION | PREREQUISITE | DEL-01-02 | ACTIVE | TBD |
| DAG-002-E0404 | EXECUTION | PREREQUISITE | DEL-01-03 | ACTIVE | TBD |

## Run Notes

- Defaults applied: `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`.
- Chosen anchor evidence: `Datasheet.md`, `_CONTEXT.md`, and `SOFTWARE_DECOMP.md` for canonical package/scope labels.
- Chosen execution evidence: `_CONTEXT.md`, `Specification.md`, `Datasheet.md`, `Procedure.md`, and `Guidance.md`.
- Decomposition path: `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`.
- Decomposition status: revision 0.7, `current_basis`.
- No `[WARNING] MISSING_DECOMPOSITION`.
- Parent anchor check: PASS; exactly one ACTIVE `IMPLEMENTS_NODE` row.
- ID-format helper check: WARNING. `tools/validation/validate_id_format.sh` expects older three-digit patterns such as `PKG-003`, `DEL-003-01`, and `SOW-0017`, while the current decomposition uses canonical IDs `PKG-03`, `DEL-03-01`, and `SOW-017`. The refresh preserves decomposition IDs rather than rewriting identifiers to satisfy the stale helper.
- SCA-002 impact: revision 0.5 marks downstream registers stale, but conservative extraction found no new explicit DEL-03-01 dependency target created by SCA-002. Existing SCA-001 architecture-basis constraints remain applicable through the v0.5 decomposition notes.
- Enum refresh: prior local values outside the current v3.1 enum set were normalized (`Origin=EXTRACTED`, `Explicitness=IMPLICIT` where previously inferred, `SatisfactionStatus=TBD` where previously unknown, `AnchorType=NOT_APPLICABLE` for execution rows).
- Authority boundary: this local register is reconciliation evidence, not independent DAG/blocker authority.

## Lifecycle Summary

| Metric | Count |
|---|---:|
| ACTIVE rows | 12 |
| RETIRED rows | 0 |
| SATISFIED rows | 6 |
| TBD rows | 4 |
| NOT_APPLICABLE rows | 2 |

## Downstream Handoff Notes

- For `RECONCILIATION`, compare this refreshed local extraction against aggregate DAG and coordination surfaces because existing project-level DAG artifacts may still reflect pre-SCA-002 state.
- The new anchor rows are local Tree x DAG integrity evidence and should not be interpreted as additional execution blockers.
- The retained DEL-01/DEL-02 predecessor rows are conservative carry-forward edges from the prior local mirror, now backed by local deliverable evidence and v0.5 decomposition labels.
- No downstream consumer edge was added for DEL-03-07 or DEL-03-08 in this conservative refresh because the assigned sources do not explicitly name those deliverables as consumers of DEL-03-01.

## Run History

- 2026-04-30T09:55:57-0600 — `TASK_RUN_DEL-03-01_2026-04-30_0955_dependency-extract`; mode `UPDATE`; strictness `CONSERVATIVE`; produced prior local register and dependency index.
- 2026-05-03 — synchronized local register from `execution/_DAG/DAG-006/DependencyEdges.csv`; 10 ACTIVE rows.
- 2026-05-10T21:56:41-0600 — TP-DAG-004 refresh row; mode `UPDATE`; strictness `CONSERVATIVE`; consumer context `RECONCILIATION`; decomposition revision 0.5 at `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`; 12 ACTIVE rows; warnings: ID-format helper pattern mismatch.

## Authority Boundary

- Aggregate DAG/blocker computation remains outside this deliverable-local artifact.
- This local register is an evidence and reconciliation surface only.
- `PKG-00` architecture-basis rows are preserved here as injected context evidence; `PKG-00` does not receive local dependency registers.
