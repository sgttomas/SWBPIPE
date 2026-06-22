# Dependencies: DEL-08-06 State, comparison, and handoff report sections

## Coordination Mode
- **Mode:** FULL_GRAPH
- **Graph Authority:** `execution/_DAG/DAG-007/` is the current approved canonical graph authority.
- **Authority Boundary:** Candidate/non-gating edges are not represented through `Status=CANDIDATE` in current canonical registers.

## Declared Upstream Dependencies
- None recorded.

## Declared Downstream Dependencies
- None recorded.

## Extracted Dependency Register
- **Local Register:** `Dependencies.csv`
- **Register schema version:** `v3.1`
- **Canonicalized:** 2026-06-16
- **Rows:** 29 total; 28 ACTIVE; 1 RETIRED.
- **Classes:** ANCHOR=6; EXECUTION=23.
- **Candidate rows moved to worklist:** 1.

## Canonical Dependency Types
- `INTERFACE`: 5
- `OTHER`: 6
- `PREREQUISITE`: 18

## Run Notes
- Core enum fields conform to the canonical Chirality dependency model.
- Legacy project-specific labels are preserved in `Notes` as `legacy_*` fields.
- Candidate rows remain non-gating in the candidate worklist and require explicit human approval plus graph revalidation before promotion.
- Added 6 anchor rows to resolve the prior floating-node condition using explicit `_CONTEXT.md` identity, scope, and objective evidence.
- PKG-00 architecture-basis rows reviewed: 7; changed: 0.
- Anchor check: PASS; exactly one ACTIVE `IMPLEMENTS_NODE` row is present.
- Decomposition path used: `execution/_Decomposition/SOFTWARE_DECOMP.md`.
- Source documents used: `_CONTEXT.md`, `Datasheet.md`, `Specification.md`, `Procedure.md`, `Guidance.md`, and current `Dependencies.csv`.
- Warnings: none.
- This dependency refresh does not authorize implementation, lifecycle promotion, release claims, professional approval, certification, sealing, authentication, or code-compliance claims.

| Class | ACTIVE | RETIRED |
|---|---:|---:|
| ANCHOR | 6 | 0 |
| EXECUTION | 22 | 1 |

## Run History
- 2026-06-16 dependency semantic refresh: MODE=UPDATE; STRICTNESS=CONSERVATIVE; CONSUMER_CONTEXT=RECONCILIATION; ARCHITECTURE_BASIS_POLICY=PKG00_CONSISTENCY_TRACKERS; rows added 6, retired 0, changed 0; validation PASS.

## Lifecycle Summary
- ACTIVE rows: 28.
- RETIRED rows: 1.
- Satisfaction status: NOT_APPLICABLE=6; SATISFIED=7; TBD=16.
- Closure state: dependency register valid for reconciliation handoff; no lifecycle issuance or implementation acceptance implied.

## Downstream Handoff Notes
- Candidate/non-gating `DEL-15-02` row remains `RETIRED` with candidate-disposition notes pending graph authority.
- PKG-00 rows remain architecture-consistency trackers only; they are not substitutes for decomposition truth.
