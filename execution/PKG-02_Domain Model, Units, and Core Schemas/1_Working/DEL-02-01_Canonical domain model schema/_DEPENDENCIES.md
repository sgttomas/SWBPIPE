# Dependencies: DEL-02-01 Canonical domain model schema

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
- **Semantic refresh:** 2026-06-16
- **Rows:** 13 total; 13 ACTIVE; 0 RETIRED.
- **Classes:** ANCHOR=6; EXECUTION=7.
- **Candidate rows moved to worklist:** 0.
- **PKG-00 architecture-basis trackers:** 7 reviewed; 7 retained ACTIVE; 0 changed during final audit pass.

## Canonical Dependency Types
- `OTHER`: 6
- `PREREQUISITE`: 7

## Run Notes
- TaskSkill: `dependency-extract`; MODE=`UPDATE`; STRICTNESS=`CONSERVATIVE`; CONSUMER_CONTEXT=`RECONCILIATION`; ARCHITECTURE_BASIS_POLICY=`PKG00_CONSISTENCY_TRACKERS`; ApplyEdits=`true`.
- Decomposition path used: `execution/_Decomposition/SOFTWARE_DECOMP.md` (present; anchor validation not degraded).
- Source docs reviewed: `Datasheet.md`, `_CONTEXT.md`, `Guidance.md`, `Procedure.md`, `Specification.md`, `_REFERENCES.md`.
- PKG-00 evidence reviewed for architecture trackers: `_CONTEXT.md` architecture-basis injection plus PKG-00 deliverable source excerpts for AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, and AB-00-08.
- Core enum fields conform to the canonical Chirality dependency model. Legacy project-specific labels are preserved in `Notes` as `legacy_*` fields.
- Parent anchor check: exactly one ACTIVE `IMPLEMENTS_NODE`; no FLOATING_NODE or AMBIGUOUS_ANCHOR warning.
- Candidate rows remain non-gating in the candidate worklist and require explicit human approval plus graph revalidation before promotion.
- This dependency refresh does not authorize implementation, lifecycle promotion, release claims, professional approval, certification, sealing, authentication, or code-compliance claims.

## Run History
- 2026-06-16: dependency semantic refresh under sealed PKG-02 shard brief. Rows added=0; rows newly retired=0; rows changed since HEAD=13 (LastSeen/notes/evidence normalization); PKG-00 rows reviewed=7, changed in final pass=0; validation PASS.

## Lifecycle Summary
- ACTIVE: 13
- RETIRED: 0
- SatisfactionStatus: SATISFIED=13; TBD=0; NOT_APPLICABLE=0; other=0.
