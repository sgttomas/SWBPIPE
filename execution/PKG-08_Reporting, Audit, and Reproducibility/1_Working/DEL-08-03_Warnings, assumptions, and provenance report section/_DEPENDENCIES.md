# Dependencies: DEL-08-03 Warnings, assumptions, and provenance report section

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
- **Rows:** 15 total; 15 ACTIVE; 0 RETIRED.
- **Classes:** ANCHOR=4; EXECUTION=11.
- **Candidate rows moved to worklist:** 0.

## Canonical Dependency Types
- `INTERFACE`: 1
- `OTHER`: 4
- `PREREQUISITE`: 10

## Run Notes
- Core enum fields conform to the canonical Chirality dependency model.
- Legacy project-specific labels are preserved in `Notes` as `legacy_*` fields.
- Candidate rows remain non-gating in the candidate worklist and require explicit human approval plus graph revalidation before promotion.
- Added 4 anchor rows to resolve the prior floating-node condition using explicit `_CONTEXT.md` identity, scope, and objective evidence.
- PKG-00 architecture-basis rows reviewed: 7; changed: 0.
- Anchor check: PASS; exactly one ACTIVE `IMPLEMENTS_NODE` row is present.
- Decomposition path used: `execution/_Decomposition/SOFTWARE_DECOMP.md`.
- Source documents used: `_CONTEXT.md`, `Datasheet.md`, `Specification.md`, `Procedure.md`, `Guidance.md`, and current `Dependencies.csv`.
- Warnings: none.
- This dependency refresh does not authorize implementation, lifecycle promotion, release claims, professional approval, certification, sealing, authentication, or code-compliance claims.

| Class | ACTIVE | RETIRED |
|---|---:|---:|
| ANCHOR | 4 | 0 |
| EXECUTION | 11 | 0 |

## Run History
- 2026-06-16 dependency semantic refresh: MODE=UPDATE; STRICTNESS=CONSERVATIVE; CONSUMER_CONTEXT=RECONCILIATION; ARCHITECTURE_BASIS_POLICY=PKG00_CONSISTENCY_TRACKERS; rows added 4, retired 0, changed 0; validation PASS.

## Lifecycle Summary
- ACTIVE rows: 15.
- RETIRED rows: 0.
- Satisfaction status: NOT_APPLICABLE=4; SATISFIED=7; TBD=4.
- Closure state: dependency register valid for reconciliation handoff; no lifecycle issuance or implementation acceptance implied.

## Downstream Handoff Notes
- No candidate rows are present in the canonical register.
- PKG-00 rows remain architecture-consistency trackers only; they are not substitutes for decomposition truth.
