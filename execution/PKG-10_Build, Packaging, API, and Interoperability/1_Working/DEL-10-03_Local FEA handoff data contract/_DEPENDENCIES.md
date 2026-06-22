# Dependencies: DEL-10-03 Local FEA handoff data contract

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
- **Rows:** 20 total; 18 ACTIVE; 2 RETIRED.
- **Classes:** ANCHOR=3; EXECUTION=17.
- **Candidate/non-gating rows in register:** 2.

## Canonical Dependency Types
- `ENABLES`: 1
- `INTERFACE`: 5
- `OTHER`: 3
- `PREREQUISITE`: 11

## Candidate / Non-Gating Handoff Notes
- `DAG-002-E0618` (RETIRED): CANDIDATE: DEL-10-03 may need DEL-08-04 as a predecessor: local FEA handoff may reuse result export envelopes, but the handoff contract may define a separate package
- `TP-DAG-004-DEL-10-03-C0001` (RETIRED): DEL-10-03 likely enables DEL-15-01: local FEA handoff contract defines advisory handoff concepts that the canonical handoff package schema may need to preserve.

## Run Notes
- TaskSkill: dependency-extract; MODE=UPDATE; STRICTNESS=CONSERVATIVE; CONSUMER_CONTEXT=RECONCILIATION; ARCHITECTURE_BASIS_POLICY=PKG00_CONSISTENCY_TRACKERS.
- Decomposition path: `execution/_Decomposition/SOFTWARE_DECOMP.md`; located and used for anchor/target resolution.
- Anchor doc: `_CONTEXT.md`; execution docs reviewed as needed: `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, existing `Dependencies.csv`, and prior `_DEPENDENCIES.md`.
- PKG-00 architecture-basis rows were reviewed read-only where cited; supported rows were preserved and no PKG-00 files were changed.
- Core enum fields conform to the canonical Chirality dependency model.
- Legacy project-specific labels are preserved in `Notes` as `legacy_*` fields.
- Candidate/non-gating ideas require explicit human graph approval plus graph revalidation before promotion.
- This dependency refresh does not authorize implementation, lifecycle promotion, release claims, professional approval, certification, sealing, authentication, or code-compliance claims.

## Run History
- 2026-06-16: Dependency semantic refresh for PKG-10 shard; rows added=3, rows retired=2, rows changed=2; PKG-00 rows 7 reviewed / 0 changed; validation passed.

## Lifecycle Summary
- ACTIVE rows: 18
- RETIRED rows: 2
- Closure-state breakdown: NOT_APPLICABLE=3; SATISFIED=11; TBD=6

## Validation Result
- PASS: `python3 tools/validation/validate_dependencies_schema.py Dependencies.csv` returned VALID for this deliverable.

## Downstream Handoff Notes
- This file is a deliverable-local derivative dependency register index for reconciliation; it is not aggregate graph authority.
- Later DAG fan-in must consume accepted snapshots and preserve retired candidate dispositions unless human graph authority promotes them.
