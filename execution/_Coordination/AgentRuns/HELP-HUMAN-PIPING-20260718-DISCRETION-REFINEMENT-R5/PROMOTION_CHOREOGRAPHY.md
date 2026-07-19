# Promotion Choreography — D-54 / DEC-087 Candidate to Active Plan

**Status:** `READY_FOR_DURABLE_LANDING — EFFECT HELD`

Candidate:
`WORKPLAN_CANDIDATE_2026-07-18_piping_loop.md`

Future active path:
`projects/chirality-piping/loop/WORKPLAN_2026-07-18_piping_loop.md`

CHANGE may perform promotion because the durable v7 semantic and carry-forward
returns are both `COMMIT-SAFE` and the owner clarified that no repeat sibling
review is required. `S5_REVIEW_RETURN_02.md` must not be invented.

Exact choreography:

1. Recompute and match the frozen candidate bytes, Git blob, and SHA-256 in
   `LANDING_MANIFEST.md`.
2. Confirm the future active path is absent from current HEAD, index, and
   worktree; confirm HEAD-only LOOP_INIT selects the committed 2026-07-17 plan.
3. Materialize the candidate bytes at the future active path without editing
   either copy.
4. Require `cmp -s <candidate> <active>` and equal `git hash-object` output.
5. Stage only the declared landing scope. Reconfirm HEAD-only discovery still
   selects 2026-07-17 because staging is not HEAD.
6. Commit candidate, byte-identical active copy, D-54/register/DEC-087,
   LOOP_INIT, and the complete R5 history atomically through CHANGE.
7. Post-commit, require the new HEAD entry to be a unique mode-100644 blob,
   candidate/active HEAD blobs equal, and HEAD-only discovery to select the
   committed 2026-07-18 plan.

Any failed step holds promotion. No active-path materialization occurs before
CHANGE executes this durable landing.
