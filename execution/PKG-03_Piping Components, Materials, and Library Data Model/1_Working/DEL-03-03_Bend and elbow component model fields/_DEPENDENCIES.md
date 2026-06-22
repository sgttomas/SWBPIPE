# Dependencies: DEL-03-03 Bend and elbow component model fields

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
- **Rows:** 11 total; 11 ACTIVE; 0 RETIRED.
- **Classes:** ANCHOR=2; EXECUTION=9.
- **Candidate rows moved to worklist:** 0.

## Canonical Dependency Types
- `CONSTRAINT`: 6
- `OTHER`: 2
- `PREREQUISITE`: 3

## PKG-00 Architecture-Basis Review
- **Rows reviewed:** 6
- **Rows changed:** 0
- **Disposition:** Supported PKG-00 consistency tracker rows retained as ACTIVE execution dependencies; PKG-00 files were read for basis context and not written.

## Run Notes
- TaskSkill: dependency-extract; MODE=UPDATE; STRICTNESS=CONSERVATIVE; CONSUMER_CONTEXT=RECONCILIATION; ARCHITECTURE_BASIS_POLICY=PKG00_CONSISTENCY_TRACKERS.
- Anchor doc: `_CONTEXT.md` plus `Datasheet.md`/decomposition where already cited by existing rows.
- Execution docs: `_CONTEXT.md`, `Datasheet.md`, `Specification.md`, `Procedure.md`, and cited decomposition/register evidence already present in the register.
- Decomposition path: `execution/_Decomposition/SOFTWARE_DECOMP.md` located and used for PKG-03/SOW/OBJ support checks.
- Rows added in this semantic refresh: 0.
- Rows retired in this semantic refresh: 0.
- Warnings: none.
- This dependency refresh does not authorize implementation, lifecycle promotion, release claims, professional approval, certification, sealing, authentication, or code-compliance claims.

## Run History
- 2026-06-16: MODE=UPDATE; STRICTNESS=CONSERVATIVE; semantic refresh for PKG-03 shard; rows added=0; rows retired=0; PKG-00 rows reviewed=6; PKG-00 rows changed=0; warnings=none.

## Lifecycle Summary
- ACTIVE rows: 11
- RETIRED rows: 0
- SatisfactionStatus `SATISFIED`: 8
- SatisfactionStatus `TBD`: 3
