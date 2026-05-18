# Dependencies: DEL-10-04 Build, packaging, and CI/CD pipeline

## Declared Dependency Notes

No declared dependency rows are present in this deliverable-local register. This refresh preserves the previous aggregate-DAG rows as history where needed, but only source-supported rows remain `ACTIVE`.

## Extracted Dependency Register

- **Register schema:** v3.1
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer context:** RECONCILIATION
- **Rows:** 20 total; 16 ACTIVE; 4 RETIRED
- **ACTIVE classes:** 9 ANCHOR; 7 EXECUTION

| DependencyID | Class | Direction | Type | Target | Status | Evidence |
|---|---|---:|---|---|---|---|
| DEP-10-04-A001 | ANCHOR | UPSTREAM | OTHER | SOW-032 | ACTIVE | `_CONTEXT.md` Scope Coverage |
| DEP-10-04-A002..A009 | ANCHOR | UPSTREAM | OTHER | REQ-10-04-01..REQ-10-04-08 | ACTIVE | `Specification.md` Requirements |
| DEP-10-04-E001..E007 | EXECUTION | UPSTREAM | CONSTRAINT | AB-00-01/02/03/04/06/07/08 deliverables | ACTIVE | `_CONTEXT.md` Architecture Basis Injection |
| DAG-002-E0571..E0574 | EXECUTION | UPSTREAM | OTHER | Prior inferred DAG targets | RETIRED | Not re-extracted from assigned docs |

## Run Notes

- `SOURCE_DOCS=AUTO`; scanned assigned deliverable source documents: `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_CONTEXT.md`, `_REFERENCES.md`, existing `Dependencies.csv`, and existing `_DEPENDENCIES.md`.
- `ANCHOR_DOC=AUTO`; primary anchor signals came from `_CONTEXT.md`, `Datasheet.md`, and `Specification.md`.
- `EXECUTION_DOC_ORDER=AUTO`; explicit execution constraints came from `_CONTEXT.md` Architecture Basis Injection.
- `DECOMPOSITION_PATH=/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`; decomposition was available and used only to validate identifiers and canonical labels.
- Current enum validation accepts only `ANCHOR_TYPE=IMPLEMENTS_NODE|TRACES_TO_REQUIREMENT|NOT_APPLICABLE`, `DEPENDENCY_TYPE=PREREQUISITE|INTERFACE|HANDOVER|CONSTRAINT|ENABLES|OTHER`, `EXPLICITNESS=EXPLICIT|IMPLICIT`, and `ORIGIN=DECLARED|EXTRACTED`; historical aggregate-DAG values were normalized.
- [WARNING] Four prior aggregate-DAG rows were not explicit in assigned deliverable source documents under conservative strictness and were marked `RETIRED` rather than deleted.

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 16 |
| RETIRED | 4 |

| SatisfactionStatus | Count |
|---|---:|
| NOT_APPLICABLE | 9 |
| SATISFIED | 7 |
| TBD | 4 |

## Downstream Handoff Notes

For RECONCILIATION: ACTIVE rows now reflect only evidence available inside the assigned deliverable folder plus decomposition validation. The retired `DAG-002-E0571..E0574` rows may still be valid aggregate sequencing edges, but they need reconciliation or a declared/approved source because this conservative local refresh could not re-extract them from the assigned DEL-10-04 source documents.

## Run History

- 2026-05-10 23:12 MDT — TP-DAG-004 dependency-extract refresh; mode UPDATE; strictness CONSERVATIVE; consumer context RECONCILIATION; decomposition available; ACTIVE rows: 16; RETIRED rows: 4; warnings: 1.
