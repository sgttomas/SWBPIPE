# R23 runtime-transport amendment V1

- Amendment ID: `R23-RTA-001`.
- Classification: non-consequential runtime transport correction under the
  adopted owner AMEND; objective, accepted basis, row/field set, write scope,
  authority, evidence, risk posture, and acceptance gates are unchanged.
- Attempt-01 fact: all five governed children had `apply_patch` but no filesystem
  read tool. Each stopped before writes. No consumer target file changed.
- Preservation: original N2 `LAUNCH_BRIEF.md` and `STATUS.md` files are retained
  without edit. Attempt-01 terminal status/return records are additive. Attempt-02
  is a new versioned attempt linked to the original instance and this amendment.
- Attempt-02 transport: before dispatch, persist the full prior brief verbatim,
  amendment/attempt IDs, exact target/pre-hash, literal guarded forward hunk,
  literal guarded inverse hunk, apply-patch-only tool fence, unchanged write
  target, acceptance conditions, and F-PIP fences. Relay that persisted content
  verbatim to the same actual child session.
- Child execution: the child may call only `apply_patch` against its named
  consumer register/index pair. Patch context is the old-text guard. Any context
  mismatch or tool error stops the attempt without rereading, improvisation,
  regenerated hunks, broader edits, or a second semantic attempt.
- Success posture: successful tool output means only
  `PATCH_APPLIED_PENDING_N5_VALIDATION`, never semantic PASS or COMPLETE.
- N5 ownership: PROJECT_SETUP's later serialized N5 is the sole Bash-bearing
  integration owner. It will independently verify post-hashes, exact permitted
  row/field deltas, byte identity elsewhere, evidence predicates, schemas/enums/
  IDs, five holds, anchor scope, and all existing acceptance conditions. N5 is
  not authorized in the present stage.
- Rollback: only exact inverse `apply_patch` hunks against actually applied
  post-text guards may be used later after N5 failure. Never use Git checkout,
  reset, or whole-file replacement. Record inverse tool output and post-rollback
  hash/semantic validation. If an inverse guard fails, stop and escalate.
- Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). F-PIP-1
  through F-PIP-5 remain unchanged. No receipt, DAG/DAG-009, `_LATEST`, decision/
  register, lifecycle, status/memory/product, commit, push, PR, or merge write is
  authorized.

