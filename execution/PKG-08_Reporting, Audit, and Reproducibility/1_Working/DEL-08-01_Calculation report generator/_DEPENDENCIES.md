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
- **Closure-status refreshed:** 2026-07-22
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
- 2026-07-22 closure refresh runtime: `SOURCE_DOCS=AUTO`; `DOC_ROLE_MAP=DEFAULT`; `ANCHOR_DOC=AUTO`; `EXECUTION_DOC_ORDER=AUTO`; `MODE=UPDATE`; `STRICTNESS=CONSERVATIVE`; `CONSUMER_CONTEXT=RECONCILIATION`.
- 2026-07-22 evidence path: accepted R15 audit under `execution/_Evaluation/DEPENDENCY_READINESS_AUDIT_2026-07-21_R15/`, followed by the exact live provider/status and consumer-binding paths cited for `DAG-002-E0522` through `DAG-002-E0528`; no source re-extraction, row creation, retirement, or edge-meaning change was performed.
- `[WARNING] ID_VALIDATOR_GRAMMAR_MISMATCH`: `tools/validation/validate_id_format.sh` accepts three-digit PKG/DEL and four-digit SOW forms, while this accepted decomposition/register uses two-digit package/deliverable families and `SOW-024`; exact IDs were preserved and all target IDs were resolved against `execution/_Decomposition/SOFTWARE_DECOMP.md`.
- Warnings: one validator-grammar mismatch; no dependency-content warning.
- This dependency refresh does not authorize implementation, lifecycle promotion, release claims, professional approval, certification, sealing, authentication, or code-compliance claims.
- This deliverable-local register refresh is not DAG activation, dependency-based selection authority, lifecycle closure, or acceptance of a successor graph.

| Class | ACTIVE | RETIRED |
|---|---:|---:|
| ANCHOR | 3 | 0 |
| EXECUTION | 14 | 1 |

## Run History
- 2026-06-16 dependency semantic refresh: MODE=UPDATE; STRICTNESS=CONSERVATIVE; CONSUMER_CONTEXT=RECONCILIATION; ARCHITECTURE_BASIS_POLICY=PKG00_CONSISTENCY_TRACKERS; rows added 0, retired 0, changed 0; validation PASS.
- 2026-07-22 dependency satisfaction refresh: MODE=UPDATE; STRICTNESS=CONSERVATIVE; CONSUMER_CONTEXT=RECONCILIATION; decomposition `execution/_Decomposition/SOFTWARE_DECOMP.md` present; accepted R15 edge audit reconciled against unchanged current live evidence at frozen SHA `aeace2ac39cb0039f2076dadcfce980c9e327a86`; rows added 0, retired 0, changed 7 (`DAG-002-E0522` through `DAG-002-E0528`, `TBD` to `SATISFIED` at edge grain only); warnings=1 non-content tool-grammar limitation (`validate_id_format.sh` does not recognize this project's accepted short ID family); validation recorded in the TASK run record.

## Lifecycle Summary
- ACTIVE rows: 17.
- RETIRED rows: 1.
- Satisfaction status: NOT_APPLICABLE=3; SATISFIED=14; TBD=1.
- Closure state: dependency register valid for reconciliation handoff; seven refreshed rows are satisfied only at their recorded edge grain and `SEMANTIC_READY` maturity; no lifecycle issuance, implementation acceptance, graph activation, or selection authority implied.

## Downstream Handoff Notes
- Candidate/non-gating row retained as `RETIRED` with `candidate_disposition` notes for reconciliation visibility.
- PKG-00 rows remain architecture-consistency trackers only; they are not substitutes for decomposition truth.
- The seven R15-backed local satisfactions remain inputs to a separately validated immutable successor-DAG proposal; DAG-007 remains the approved graph authority until an owner separately accepts a successor and authorizes its pointer update.
