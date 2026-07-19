BLOCK

- **Material activation-guard bypass.** The repeat-S5 packet requires every
  machine-readable verdict/binding line to be exact and unambiguous, and says
  any ambiguous line must keep the guard in fallback
  (`S5_REVIEW_PACKET.md:47-60`). The guard instead uses four independent
  presence tests (`WORKPLAN_2026-07-18_piping_loop.md:48-54`); it neither
  requires exactly one matching line nor rejects contradictory verdicts or
  duplicate bindings. I executed the exact guard body against an in-memory
  post-landing simulation in which all four paths were tracked at the same
  simulated HEAD, all worktree blobs equalled their HEAD blobs, and the plan,
  D-54, and DEC-087 bindings were correct. Results were:
  `valid -> DEC087_GUARD_ACTIVE`;
  `Verdict: BLOCK` plus `Verdict: COMMIT-SAFE -> DEC087_GUARD_ACTIVE`;
  and two `Verdict: COMMIT-SAFE` lines
  `-> DEC087_GUARD_ACTIVE`. Thus an ambiguous or expressly contradictory S5
  return can activate DEC-087, refuting the claimed fail-closed actual-verdict
  condition at plan lines 67-72. The same presence-only defect applies to
  duplicate binding fields. This blocks landing and repeat S5 until corrected
  and freshly verified.

- **Enumeration performed.** The live governed tranche contains exactly D-54,
  the appended D-54 register row, the appended DEC-087 row, the guarded
  `WORKPLAN_2026-07-18_piping_loop.md`, and 14 pre-return R5 files: seven
  top-level records (`ORCHESTRATION_PLAN.md`, `RETURN.md`,
  `S5_BLOCK_RETURN_01.md`, `S5_REVIEW_PACKET.md`, `HANDOFF_STATE.md`,
  `STATUS.json`, `WORK_GRAPH.json`); the integration launch brief; both v1
  verifier briefs and preserved returns; and both v2 launch briefs. Before this
  authorized write, both v2 return paths and `S5_REVIEW_RETURN_02.md` were
  absent. The v1 returns remain preserved and expressly superseded by S5 BLOCK.

- **Passing pre-landing and graph checks.** `LOOP_INIT.md` selects the newest
  filename, which is the 2026-07-18 candidate, but executing its exact first
  shell block in the current tree returned
  `DEC087_GUARD_FALLBACK: use WORKPLAN_2026-07-17_piping_loop.md; DEC-087 prohibited`.
  The candidate, D-54, and repeat-S5 return are not tracked at HEAD (the return
  is absent), so current effect is mechanically held. All eight graph node IDs
  are unique; every dependency resolves; N4 depends on `N1_INTEGRATION`; and N8
  depends on both v2 verifier nodes.

- **Passing authority/semantic checks.** D-54's owner payload recomputes to
  4,632 UTF-8 bytes / SHA-256
  `1cb500ce6a2e41038b1fbf4f2250ef80e650e8de8fb990658c4a68ac3e65fbc0`.
  D-54, its register row, DEC-087, the candidate Step 2, and the R5 packet
  consistently preserve agent-judgment attribution under owner standing
  approval, `OwnerCaseSelection=NONE`, governed-brief adoption as the owner's
  act, fast-reject owner gates, live accepted-artifact grounding, all four
  lenses, existing-authority-only application, bounded reversibility, concise
  rationale/material rejected alternatives, and independent refutation.

- **Passing preservation/effect checks.** Git enumeration shows no diff in
  app-dev, D-49 through D-53, DEC-082 through DEC-086 text, any older workplan,
  `LOOP_RECEIPTS.md`, or DEL-09-04 surfaces. The piping D-50 and app-dev
  D-APP-60 Shared-Block v1 payloads both recompute byte-identically to 5,108
  bytes / SHA-256
  `76438ab0e00dc70e5f6db751a32d0ff07b681c7b7fb12eeda338157c5ebe7668`.
  HEAD remains `756425eb53814f7a9f154fac5e2c139ef8ed5039`, the index is unstaged,
  `S5_REVIEW_RETURN_02.md` is absent, and STATUS/graph/handoff/packet remain
  `CORRECTION_IN_PROGRESS`, effect held, both v2 verifiers pending, repeat S5
  not ready. No receipt, DEL-09-04 change, refined-discretion effect, commit,
  merge, push, product/code, or external action is evidenced.
