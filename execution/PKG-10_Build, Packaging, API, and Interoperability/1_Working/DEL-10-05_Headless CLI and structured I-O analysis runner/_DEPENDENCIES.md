# Dependencies: DEL-10-05 Headless CLI and structured I/O analysis runner

## Coordination Mode
- **Mode:** FULL_GRAPH
- **Graph Authority:** `execution/_DAG/DAG-006/` is the approved legacy graph pending `DAG-007` canonical approval.
- **Authority Boundary:** Candidate/non-gating edges are not represented through `Status=CANDIDATE` in current canonical registers.

## Declared Upstream Dependencies
- None recorded.

## Declared Downstream Dependencies
- None recorded.

## Extracted Dependency Register
- **Local Register:** `Dependencies.csv`
- **Register schema version:** `v3.1`
- **Semantic refresh:** 2026-06-16
- **Rows:** 29 total; 16 ACTIVE; 13 RETIRED.
- **Classes:** ANCHOR=6; EXECUTION=23.
- **Candidate/non-gating rows in register:** 0.

## Canonical Dependency Types
- `CONSTRAINT`: 1
- `INTERFACE`: 6
- `OTHER`: 19
- `PREREQUISITE`: 3

## Candidate / Non-Gating Handoff Notes
- None.

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
- 2026-06-16: Dependency semantic refresh for PKG-10 shard; rows added=0, rows retired=0, rows changed=0; PKG-00 rows 7 reviewed / 0 changed; validation passed.

## Lifecycle Summary
- ACTIVE rows: 16
- RETIRED rows: 13
- Closure-state breakdown: SATISFIED=17; TBD=12

## Validation Result
- PASS: `python3 tools/validation/validate_dependencies_schema.py Dependencies.csv` returned VALID for this deliverable.

## Downstream Handoff Notes
- This file is a deliverable-local derivative dependency register index for reconciliation; it is not aggregate graph authority.
- Later DAG fan-in must consume accepted snapshots and preserve retired candidate dispositions unless human graph authority promotes them.
