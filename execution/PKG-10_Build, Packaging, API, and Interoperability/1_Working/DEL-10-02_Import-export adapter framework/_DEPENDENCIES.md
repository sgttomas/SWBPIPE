# Dependencies: DEL-10-02 Import/export adapter framework

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
- **Rows:** 18 total; 17 ACTIVE; 1 RETIRED.
- **Classes:** ANCHOR=5; EXECUTION=13.
- **Candidate/non-gating rows in register:** 1.

## Canonical Dependency Types
- `CONSTRAINT`: 1
- `ENABLES`: 1
- `INTERFACE`: 6
- `OTHER`: 5
- `PREREQUISITE`: 5

## Candidate / Non-Gating Handoff Notes
- `DEP-010-02-014` (RETIRED): DEL-10-02 enables later external handoff contracts by providing the import/export adapter framework shell while leaving concrete external formats TBD.

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
- 2026-06-16: Dependency semantic refresh for PKG-10 shard; rows added=0, rows retired=1, rows changed=1; PKG-00 rows 5 reviewed / 0 changed; validation passed.

## Lifecycle Summary
- ACTIVE rows: 17
- RETIRED rows: 1
- Closure-state breakdown: NOT_APPLICABLE=5; SATISFIED=9; TBD=4

## Validation Result
- PASS: `python3 tools/validation/validate_dependencies_schema.py Dependencies.csv` returned VALID for this deliverable.

## Downstream Handoff Notes
- This file is a deliverable-local derivative dependency register index for reconciliation; it is not aggregate graph authority.
- Later DAG fan-in must consume accepted snapshots and preserve retired candidate dispositions unless human graph authority promotes them.
