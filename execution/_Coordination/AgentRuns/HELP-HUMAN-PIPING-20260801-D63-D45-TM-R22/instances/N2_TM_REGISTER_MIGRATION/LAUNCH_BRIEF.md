---
doc_id: R22-N2-TM-REGISTER-MIGRATION-LAUNCH
doc_kind: coordination.launch_brief
status: SEALED
created: 2026-08-01
---

# Launch brief — N2 Task Management register migration

- Parent: `HELP_HUMAN`
- Managed role: `TASK_MANAGEMENT` (Agent 1)
- Run ID: `HELP-HUMAN-PIPING-20260801-D63-D45-TM-R22`
- Node: `N2_TM_REGISTER_MIGRATION`
- Working root: `{REPO_ROOT}/projects/chirality-piping`
- Frozen Git basis: `3c2e816f1072295de15fdcdf924c19b4b66497bc`
- Active branch: `codex/piping-d63-d45-rulings`
- Dependency: `N1_D63_RECORD` completed
- Delegation: none

## Authority

The owner ruled verbatim `1) D-63 ruling: Option A.` and previously directed
that adoption mint the Piping register and migrate the packet's exact linked
rows. The governing record is
`execution/_Coordination/_DECISIONS/D-63_RULING_2026-08-01.md`.

## Objective

Mint the Piping loop's schema-1.0 Task Management register and create exactly
23 linked rows in this deterministic order: `TM-ROOT-037`,
`TM-ROOT-077` through `TM-ROOT-097`, then `TM-ROOT-053`, mapped to
`TM-PIP-001` through `TM-PIP-023`.

## Write scope

- `execution/_Coordination/_TaskManagement/REGISTER.csv`
- this `instances/N2_TM_REGISTER_MIGRATION/` directory
- only the N2 and N3 `status` values in the R22 `WORK_GRAPH.json`, after the
  fan-in gate passes

No root register, decision record/register, D-45 or D-62 surface, notice,
receipt, `LOOP_INIT.md`, adoption packet, product/decomposition artifact, or
Git state may be written.

## Row contract

- Use the authoritative 25-column schema 1.0.
- Each local `SourceRef` cites the exact root `REGISTER.csv` row and each
  `SourceSha` is the live root-register blob SHA.
- Preserve source titles, concerns, domain lenses, associations, assignments,
  priorities, and status posture.
- Keep the 22 deferred rows deferred with their still-live activation
  conditions; remove only the already-satisfied migration clause from the 21
  TP-EXPORT-006 triggers.
- Keep the D-45 linked row `OPEN` / `HIGH`; N3, not N2, codifies the O-B
  ruling.
- Use `NoticeRef=NONE`; N5 owns the ordinary notice and closure updates.
- Rows contain no directives, create no accountable agent, and have no
  authority effect under K-TM-1..6.

## Acceptance checks

CSV parsing and exact header width; unique deterministic IDs; exact source-row
coverage and provenance SHA; 22 `DEFERRED` rows with nonempty triggers; one
`OPEN` / `HIGH` D-45 row; `NoticeRef=NONE`; no agent A or directive text;
`taskmgmt validate`; applicable `taskmgmt scan` without a tracked projection;
path containment; root-register immutability; protected-surface containment;
JSON parsing; and `git diff --check`.
