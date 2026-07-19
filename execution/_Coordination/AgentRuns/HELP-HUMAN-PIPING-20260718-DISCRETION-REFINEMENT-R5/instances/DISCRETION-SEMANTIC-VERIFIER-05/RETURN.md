BLOCK

- **Material missing/wrong-object activation remains.** I ran the exact fenced
  guard in an isolated post-landing Git history containing the candidate plan,
  D-54, `SOFTWARE_DECOMP.md`, and an exact content-bound repeat-S5 return. The
  valid state returned `DEC087_GUARD_ACTIVE`. I then removed only the candidate
  plan's loose Git blob object, leaving its path and OID in the HEAD tree, the
  same OID in the stage-0 index entry, and worktree bytes whose `git
  hash-object` value was that OID. `git cat-file -e <plan-oid>` reported the
  object missing, but the exact guard still returned `DEC087_GUARD_ACTIVE`.
  `git rev-parse --verify HEAD:<path>` resolves the dangling tree-entry OID; it
  does not prove that the object exists or is a readable blob. The guard never
  otherwise reads or type-checks the candidate-plan object. Therefore its
  three OID comparisons can all pass while the claimed durable HEAD does not
  actually contain the guarded plan blob. This refutes the launch brief's
  wrong-object/fail-closed requirement and the plan's durable-HEAD containment
  claim.

- **The v4 index-only defect is closed, but does not cure this blocker.** In
  the same simulation I independently created `MM` index-only divergence for
  each of the candidate plan, D-54, decomposition, and S5-return paths while
  restoring its worktree bytes to HEAD; all four returned fallback. Replacing
  each HEAD/index entry with a non-blob gitlink object also returned fallback.
  Complete valid state returned active, and the live pre-landing tree returned
  exactly `DEC087_GUARD_FALLBACK: use WORKPLAN_2026-07-17_piping_loop.md;
  DEC-087 prohibited`.

- **History and scope remain held.** The enumerated R5 surfaces preserve v1
  local returns, actual S5 `BLOCK`, semantic/carry v2 `BLOCK` (including the
  late carry return outside the closed fan-in), semantic v3 `BLOCK`, carry v3
  interruption without return, semantic v4 interruption without return, and
  carry v4 `BLOCK`. The 20-node graph has unique IDs and resolved dependencies,
  including N4 to `N1_INTEGRATION`; N18/N19 remain pending and repeat S5 is not
  ready. D-54, the register, DEC-087, orchestration plan, packet, top-level
  return, handoff, status, graph, and interruption records consistently hold
  effect. Shared-Block/app-dev, the prior workplan, receipts, Step 3/fences,
  and DEL-09-04 remain outside the live diff from HEAD.

No repair, S5 return, DEC-087 effect, receipt, DEL-09-04 action, staging,
commit, merge, or push was performed. This verifier's only repository write is
this return.
