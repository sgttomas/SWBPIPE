# Dependencies: DEL-15-03 Downstream modeling export workflow
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
- **Semantic refreshed:** 2026-06-16
- **Rows:** 16 total; 16 ACTIVE; 0 RETIRED.
- **Classes:** ANCHOR=2; EXECUTION=14.
- **Candidate rows moved to worklist:** 0.
- **PKG-00 rows reviewed:** 7 PKG-00 architecture-basis rows reviewed; 0 changed; all retained ACTIVE as supported by _CONTEXT.md Architecture Basis Injection and PKG-00 _DEPENDENCIES.md basis notes.

## Canonical Dependency Types
- `OTHER`: 2
- `PREREQUISITE`: 14

## Run Notes
- MODE=UPDATE; STRICTNESS=CONSERVATIVE; CONSUMER_CONTEXT=RECONCILIATION; ARCHITECTURE_BASIS_POLICY=PKG00_CONSISTENCY_TRACKERS.
- Decomposition path: `/Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`; status: available and used for SOW/OBJ/PKG-00 support checks.
- Source docs reviewed: `_CONTEXT.md`, `_REFERENCES.md`, `Datasheet.md`, `Specification.md`, `Procedure.md`, `Guidance.md`, local `Dependencies.csv`, existing `_DEPENDENCIES.md`, and cited PKG-00 basis notes.
- PKG-00 review: 7 PKG-00 architecture-basis rows reviewed; 0 changed; all retained ACTIVE as supported by _CONTEXT.md Architecture Basis Injection and PKG-00 _DEPENDENCIES.md basis notes.
- Change: no row-level semantic edits required; supported rows retained.
- Core enum fields conform to the canonical Chirality dependency model; no `Status=CANDIDATE` rows emitted.
- Legacy project-specific labels are preserved in `Notes` as `legacy_*` fields where present.
- This dependency refresh does not authorize implementation, lifecycle promotion, release claims, professional approval, certification, sealing, authentication, or code-compliance claims.

## Warnings
- None.

## Run History
- 2026-06-16 18:25 MDT - dependency semantic refresh, MODE=UPDATE, STRICTNESS=CONSERVATIVE, decomposition available, warnings: None., ACTIVE rows: 16, RETIRED rows: 0.

## Lifecycle Summary
- **ACTIVE rows:** 16
- **RETIRED rows:** 0
- **Satisfaction status counts:** SATISFIED=9; TBD=7.
