# Launch brief — D06B-TERMINAL-HANDOFF-VERIFIER

## Construction and permissions

- Form: fresh bounded ephemeral Agent 2 generalist.
- Parent: HELPS_HUMANS `HELPS-HUMANS-R19-INTEGRATION`.
- Objective: verify the post-receipt R19 terminal-title correction.
- Read scope: repository.
- Write scope: none.
- Forbidden: edits, delegation, Git mutation, network/external action,
  credentials, signing/notarization, implementation, release, or publication.

## Bound state

- Frozen HEAD:
  `2f8d35ceb30da734ca6dff24dcab36dded8c9b35`.
- Packet:
  `7f72f244a5cfd896e0f33137eafd32302f052adec75aa20bc56c85c123afee49`.
- Register:
  `f03b9ad9b4b7e7156cd6ec28d8a8933fb57ee28c7ecbc874f1405c763f53dd84`.
- Receipt ledger:
  `94577b97ff75e44ab8ada7ea9cd8d413d2a8a0dea689af149444996b86dc5f7c`.
- Pre-V3-verifier R19 manifest:
  `ea5a740c293c504ef87bdb46c5268b7680549298ceba5c48cad1c50a2a07e294`.
- Protected R18 manifest:
  `e1241d77647ce369ae9d8f1a13263055c27f49e6315b8127d302a0f689638c19`.

V1 and V2 histories must remain exact at the six hashes recorded in the
manager's terminal audit evidence; do not reinterpret or replace them.

## Mandatory checks

1. Recompute the packet, register, and receipt hashes above. Confirm
   Receipt-73 occurs exactly once, remains the last receipt, and the receipt
   ledger is byte-identical to its bound hash.
2. Confirm packet, register, and receipt have no post-terminal edit and no
   target outside R19 changed under this correction.
3. Confirm `HANDOFF_STATE.md` title is exactly `# R19 terminal handoff` and is
   semantically consistent with its terminal status/body and Receipt-73.
4. Recompute the pre-V3-verifier R19 manifest and all six V1/V2 history file
   hashes; confirm both histories are preserved.
5. Parse every R19 JSON; run R19 trailing-whitespace and `git diff --check`;
   enumerate correction containment as R19-only.
6. Recompute protected D-06, D-21, decomposition, current/archived PRD,
   DEL-10-04, `BUILD_AND_RELEASE.md`, DAG, and R18 hashes. Confirm no ruling,
   codification, successor, additional receipt, Git, network, or downstream
   effect.

## Return contract

Return terminal `PASS / COMMIT-SAFE` or `BLOCK` with concise evidence for all
six checks. Do not write a file; HELPS_HUMANS records the return.
