---
doc_id: R20-D06B-ORCHESTRATION-PLAN
doc_kind: coordination.orchestration_plan
status: TERMINAL_VERIFIED_RECEIPT_INTEGRATED
created: 2026-07-25
version: 1
---

# R20 D-06b O-B and App ID result integration plan

## Activation

- Parent: HELP_HUMAN.
- Manager: HELPS_HUMANS `HELPS-HUMANS-R20-INTEGRATION`.
- Frozen HEAD:
  `2f8d35ceb30da734ca6dff24dcab36dded8c9b35`.
- Branch: `codex/piping-candidate-briefs-20260725`.
- Owner direction: `OWNER_DIRECTION.md`.
- Accepted preparation input: R19 packet and terminal handoff.
- Existing dirty R18/R19 and unrelated working-tree state are protected input.

## Exact write fence

1. Create the dated D-06b ruling record.
2. Edit only the single D-06b register row from `AWAITING_RULING` to
   `RULED (O-B; 2026-07-25)`.
3. Append exactly one next-free decision-log entry, `DEC-089`, in
   `execution/_Decomposition/SOFTWARE_DECOMP.md` §12.
4. Write only this R20 managed subtree.

HELP_HUMAN performed the separately authorized Safari action and supplied its
observation to this manager. Recording that external result under item 4 does
not enlarge this manager's write fence.

## Work graph

| Node | Owner | Writes/effect | Gate |
|---|---|---|---|
| P0 | HELPS_HUMANS | R20 owner binding and plan | exact text/hash |
| P1 | HELPS_HUMANS | ruling + one register row + DEC-089 + R20 | semantics and containment |
| X1 | HELP_HUMAN | one exact Apple Developer Explicit App ID registration through Safari | owner direction + bounded action constraints |
| P2 | HELPS_HUMANS | preserve HELP_HUMAN's external observation in R20 | no claim of direct manager observation |
| V1 | fresh Agent 2 | read-only terminal verification | `PASS / COMMIT-SAFE` or `BLOCK` |
| I1 | HELPS_HUMANS | terminal R20 fan-in only | accepted V1 |

No receipt append is authorized in R20. Root retains receipt integration.

## Protected and excluded effects

The D-06b proposal packet, R19, Receipt-73 ledger, D-06, D-21,
`tauri.conf.json`, PRDs, DEL-10-04 status, `BUILD_AND_RELEASE.md`, DAG pointer
and approval, product/code/tests, other documentation/status, lifecycle,
certificates, profiles, keys, signing, notarization, packaging, release,
publication, Git, and all network actions by this manager remain unchanged.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
