# Dependency tracking modes and `_DEPENDENCIES.md` schema (D-GOV-46)

Owner-directed Root tranche `ROOT-DEPENDENCY-SCHEMA-D-GOV-46-20260926` (record `docs/governance_harness/_DECISIONS/D-GOV-46_dependency_tracking_modes_schema.md`) makes one mode vocabulary and one `_DEPENDENCIES.md` heading schema authoritative:
- `docs/SPEC.md` §5.3 modes are `NOT_TRACKED`, `DECLARED` and `FULL_GRAPH`, with the alignment manual §6 meanings. `NOT_TRACKED` reports no computed ready/blocked judgment. `DECLARED` is a partial view of recorded critical edges and may use extraction when the confirmed dependency rules call for it. `FULL_GRAPH` computes blockers only after closure audit and cycle treatment. A legacy `TRACKED` value is read as `FULL_GRAPH`.
- `docs/SPEC.md` §5.2 is the single heading schema: `## Dependency Tracking Mode`, `## Declared Upstream (I need these before I can proceed)`, `## Declared Downstream (These need me)`, `## Extracted Dependency Register`, `## Lifecycle Summary`, `## Run Notes`, `## Run History`, and `## Downstream Handoff Notes` (only when `CONSUMER_CONTEXT` is not `NONE`). A legacy-heading table maps older headings to these sections.
- The `preparation` skeleton, `dependency-extract` Function 4, `project-setup`, `content-digest` and `tools/validation/validate_enum.py` (`TRACKING_MODE`) now match.

Existing `_DEPENDENCIES.md` files keep their headings and mode values. `dependency-extract` preserves and reads legacy headings, and refreshes agent-owned sections under the heading a file already uses. New files use the amended schema.

Historical runs are not rewritten. This loop decides its own adoption; this source tranche grants no release.
