# Sealed Agent 2 brief — owner-ruling fan-in verifier

## Identity and parentage

- Parent Agent 1: `/root/reconciliation_tm040_lost_ruling`, acting as
  `RECONCILIATION`.
- Child: one ephemeral non-delegating Agent 2 verifier.
- Frozen source state:
  `38801d299b19b36f40009714d2c7015db0bd6484`.
- Treatment commit in ancestry:
  `7c8cac7ae93204f5a5903f732755d60e65ab1a50`.
- Activation commit in ancestry:
  `3f00a351695ec3943be6d60a89643795a28f9220`.

## Objective

Independently verify that the new owner-ruling derivative faithfully records
the owner's treatment acceptances and TM-PIP-040 `LOST` personal-act outcome,
implements the accepted decision packet's on-ruling mechanism, preserves all
accepted and historical sources, and routes only the owner-directed future
TM-PIP-040 Task Management closure proposal without applying it.

## Required reads

Read this brief, the new `RUN_BASIS.md`, `OWNER_TREATMENT_RULINGS.md`,
`TM_PIP_040_LOST_OUTCOME_RECORD.md`, and
`TASK_MANAGEMENT_CLOSURE_HANDOFF.md`; the accepted parent treatment
`RUN_BASIS.md`, `RUN_RECORD.md`, `TREATMENT_VERDICTS.md`,
`VALIDATION_BACKCHECK.md`, `HANDOFF_STATE.md`, TM-PIP-040 decision packet and
provenance investigation; Receipt 96; and the read-only Task Management rows
`TM-PIP-038` through `TM-PIP-040`.

## Read and write scope

- Read scope: repository surfaces necessary for the checks above.
- Sole write:
  `OWNER_RULING_2026-08-09_TM_PIP_038_040/CHILD_RETURNS/FAN_IN_VERIFIER_RETURN.md`.
- No other file may be created or edited.
- No delegation, Git mutation, network operation, physical artifact
  operation, register disposition, or receipt append.

## Acceptance checks

1. Reproduce clean base and both required ancestor relationships.
2. Confirm the activation record and all 19 pre-existing accepted treatment
   files are byte-identical to `HEAD`.
3. Confirm the full owner direction in `OWNER_TREATMENT_RULINGS.md` is exact,
   including the required one-line personal-act statement byte-for-byte.
4. Confirm `LOST` is the only selected outcome and is grounded in the owner's
   personal act, not inferred from absence.
5. Confirm no exact date or command is invented; the earlier blocked
   orchestrator deletion is not recoded as the owner act.
6. Confirm the six-set population is exact and further recovery is declined
   without test/ledger invalidation.
7. Confirm the accepted blank packet remains blank and unchanged; the new
   outcome is a separate derivative.
8. Confirm only future `TM-PIP-040` closure as `RESOLVED_BY_DECISION` is
   proposed, not applied; no closure direction is invented for
   `TM-PIP-038` or `TM-PIP-039`.
9. Inspect tracked, untracked, and ignored state for exact containment and no
   physical restore/delete/copy/regeneration evidence.
10. Return `PASS` or `FAIL`, findings, exact observed manifest, reproduced
    hashes, and any blocker. Do not repair manager outputs.

## Return contract

The return must be a durable Markdown record containing identity, source
basis, methods, deterministic results, semantic verdict, changed-path
observation, limitations, and terminal status. `FAIL` must identify the exact
defect and leave repair to the parent.
