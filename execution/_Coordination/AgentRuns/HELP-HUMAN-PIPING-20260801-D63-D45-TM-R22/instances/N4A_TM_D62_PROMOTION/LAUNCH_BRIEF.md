---
doc_id: R22-N4A-TM-D62-PROMOTION-LAUNCH
doc_kind: coordination.launch_brief
status: SEALED
created: 2026-08-01
---

# Launch brief — N4A D-62 Task Management promotion

- Parent: `HELP_HUMAN`
- Managed role: `TASK_MANAGEMENT` (Agent 1)
- Run ID: `HELP-HUMAN-PIPING-20260801-D63-D45-TM-R22`
- Node: `N4A_TM_D62_PROMOTION`
- Working root: `{REPO_ROOT}/projects/chirality-piping`
- Frozen Git basis: `3c2e816f1072295de15fdcdf924c19b4b66497bc`
- Active branch: `codex/piping-d63-d45-rulings`
- Dependency: `N3_D45_CODIFICATION` completed
- Delegation: none

## Owner direction and objective

The owner directed verbatim:

> Then for the TM-CANDIDATE promote it as a Decisions-domain row, then
> perform a narrow D-62 register-currency correction without reopening or
> reinterpreting D-62.

N4A performs only the first serialized act: append exactly one new Piping
Action Item row, `TM-PIP-024`, recording the surfaced D-62 currency mismatch.
The row remains `OPEN`; `N4B_D62_CURRENCY_REPAIR`, owned by `HELPS_HUMANS`,
performs the repair.

## Evidence and exact row posture

- The current `_DECISIONS/_REGISTER.md` D-62 row is `RULED`, but its
  ruling-record cell says three owner-return fences are empty and pending.
- The live `D-62_od8_ratification_acceptance.md` has
  `selected_option: RATIFY-ALL-ENUMERATED` and three populated owner-ruling
  fences.
- Bind both current worktree byte streams with `git hash-object`-style source
  hashes in row order:
  - decision register: `879374f40f6a27b06a64d608618e60da777a0ea4`
  - D-62 ruling record: `969bb3672b9c5e1f889b57da480a67e23f5cc311`
- `DomainLenses=Decisions`, `Status=OPEN`, `Priority=LOW`, and the priority
  basis attributes the judgment to the owner's accepted recommendation in
  this session.
- `NoticeRef=NONE`, `ScaRef=NONE`, no agent is accountable, and CandidateRef
  identifies the surfaced `TM-CANDIDATE`.

## HOLD and authority boundary

This is record-currency correction only. It does not reopen or reinterpret
D-62 and creates no scope, reliance-hold, lifecycle, decision, approval,
priority-authority, or other authority effect. N4A must not write the decision
register, ruling record, any existing Task Management row, or any non-register
source.

## Write scope

- `execution/_Coordination/_TaskManagement/REGISTER.csv` — append only
  `TM-PIP-024`
- this `instances/N4A_TM_D62_PROMOTION/` directory
- only the N4A and N4B `status` values in the R22 `WORK_GRAPH.json`, after the
  fan-in gate passes

## Acceptance checks

Validate 24 total rows, 25 canonical columns, unique ID, exact one-row append,
source-hash reproduction from current live bytes, owner direction, no agent A,
K-TM-1..6, `taskmgmt validate`, JSON parsing, graph transition containment,
declared-path containment, and `git diff --check`. N4A passes only if N4A is
`COMPLETED`, N4B is `READY`, and every other graph status is unchanged.
