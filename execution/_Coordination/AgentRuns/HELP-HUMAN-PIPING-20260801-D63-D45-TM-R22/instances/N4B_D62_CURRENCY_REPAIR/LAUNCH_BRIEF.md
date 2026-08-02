---
doc_id: R22-N4B-D62-CURRENCY-REPAIR-LAUNCH
doc_kind: coordination.launch_brief
status: SEALED
created: 2026-08-01
---

# Launch brief — N4B D-62 register-currency repair

- Parent: `HELP_HUMAN`
- Managed role: `HELPS_HUMANS` (Agent 1)
- Run ID: `HELP-HUMAN-PIPING-20260801-D63-D45-TM-R22`
- Node: `N4B_D62_CURRENCY_REPAIR`
- Working root: `{REPO_ROOT}/projects/chirality-piping`
- Frozen Git basis: `3c2e816f1072295de15fdcdf924c19b4b66497bc`
- Active branch: `codex/piping-d63-d45-rulings`
- Dependency: `N4A_TM_D62_PROMOTION` completed with `PASS`
- Delegation: none

## Owner direction and objective

The owner directed verbatim:

> Then for the TM-CANDIDATE promote it as a Decisions-domain row, then
> perform a narrow D-62 register-currency correction without reopening or
> reinterpreting D-62.

N4B performs only the second serialized act. It replaces the stale assertion
in the D-62 row's `Ruling record` cell that three owner-return fences remain
empty and pending with the current facts already recorded by the live D-62
ruling: `status: RULED`, `selected_option: RATIFY-ALL-ENUMERATED`, and the
same ratification in all three fences.

## Accepted evidence

- `execution/_Coordination/_DECISIONS/_REGISTER.md`, D-62 row and its
  `Ruling record` cell; pre-N4B worktree blob
  `879374f40f6a27b06a64d608618e60da777a0ea4`
- `execution/_Coordination/_DECISIONS/D-62_od8_ratification_acceptance.md`;
  worktree blob `969bb3672b9c5e1f889b57da480a67e23f5cc311`; SHA-256
  `917e9c9a6b8c6fc5a4451597c084c959ff89e7a73d6172675475dd79a66db2b2`
- `loop/LOOP_RECEIPTS.md`, `Receipt-82`, which points to the completed D-62
  row and records the same canonical content SHA-256 for each fence
- N4A terminal return and `TM-PIP-024` promotion evidence

## Write scope and boundaries

- Modify only the D-62 row's `Ruling record` cell in
  `execution/_Coordination/_DECISIONS/_REGISTER.md`.
- Create only this N4B instance's `LAUNCH_BRIEF.md`, `STATUS.json`, and
  `RETURN.md`.
- On `PASS`, change only N4B from `READY` to `COMPLETED` and N5 from
  `PENDING` to `READY` in the R22 `WORK_GRAPH.json`.

Do not modify any other D-62 cell, decision row, D-62 ruling bytes, Task
Management row, receipt, notice, product, decomposition, lifecycle, root-loop,
or Git state. Preserve all ruling boundaries: no historical cure, no
reliance-hold, lifecycle, successor-mechanism, client-status, scope, or
blanket-approval effect. N5 alone owns evidence-bound disposition of
`TM-PIP-024` and the ordinary root-loop notice.

## Acceptance checks

Validate exact one-row/one-cell source change; unchanged non-ruling D-62
cells; unchanged every other decision row; live ruling `RULED` status,
`RATIFY-ALL-ENUMERATED` selection, three populated and byte-identical fences,
and canonical fence hash; preservation of every named boundary; Markdown
table integrity; no Task Management or ruling-record write; JSON parsing;
only the N4B/N5 graph status transition; declared-path containment; and
`git diff --check`.
