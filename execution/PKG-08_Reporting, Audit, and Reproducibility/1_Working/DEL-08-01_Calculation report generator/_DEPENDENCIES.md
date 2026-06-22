# Dependencies: DEL-08-01 Calculation report generator

## Coordination Mode
- **Mode:** FULL_GRAPH
- **Graph Authority:** `execution/_DAG/DAG-007/` is the current approved canonical graph authority.
- **Authority Boundary:** Candidate/non-gating edges are not represented through `Status=CANDIDATE` in current canonical registers.

## Declared Upstream Dependencies
- None separately declared by a human-owned section in this local artifact.

## Declared Downstream Dependencies
- None separately declared by a human-owned section in this local artifact.

## Extracted Dependency Register
- **Local Register:** `Dependencies.csv`
- **Register schema version:** `v3.1`
- **Canonicalized:** 2026-06-16
- **Rows:** 18 total; 17 ACTIVE; 1 RETIRED.
- **Classes:** ANCHOR=3; EXECUTION=15.
- **Candidate rows moved to worklist:** 1.

## Canonical Dependency Types
- `CONSTRAINT`: 1
- `OTHER`: 17

## Run Notes
- Core enum fields conform to the canonical Chirality dependency model.
- Legacy project-specific labels are preserved in `Notes` as `legacy_*` fields.
- Candidate rows remain non-gating in the candidate worklist and require explicit human approval plus graph revalidation before promotion.
- PKG-00 architecture-basis rows reviewed: 7; changed: 0.
- Anchor check: PASS; exactly one ACTIVE `IMPLEMENTS_NODE` row is present.
- Decomposition path used: `execution/_Decomposition/SOFTWARE_DECOMP.md`.
- Source documents used: `_CONTEXT.md`, `Datasheet.md`, `Specification.md`, `Procedure.md`, `Guidance.md`, and current `Dependencies.csv`.
- Warnings: none.
- This dependency refresh does not authorize implementation, lifecycle promotion, release claims, professional approval, certification, sealing, authentication, or code-compliance claims.

| Class | ACTIVE | RETIRED |
|---|---:|---:|
| ANCHOR | 3 | 0 |
| EXECUTION | 14 | 1 |

## Run History
- 2026-06-16 dependency semantic refresh: MODE=UPDATE; STRICTNESS=CONSERVATIVE; CONSUMER_CONTEXT=RECONCILIATION; ARCHITECTURE_BASIS_POLICY=PKG00_CONSISTENCY_TRACKERS; rows added 0, retired 0, changed 0; validation PASS.

## Lifecycle Summary
- ACTIVE rows: 17.
- RETIRED rows: 1.
- Satisfaction status: NOT_APPLICABLE=3; SATISFIED=7; TBD=8.
- Closure state: dependency register valid for reconciliation handoff; no lifecycle issuance or implementation acceptance implied.

## Downstream Handoff Notes
- Candidate/non-gating row retained as `RETIRED` with `candidate_disposition` notes for reconciliation visibility.
- PKG-00 rows remain architecture-consistency trackers only; they are not substitutes for decomposition truth.
