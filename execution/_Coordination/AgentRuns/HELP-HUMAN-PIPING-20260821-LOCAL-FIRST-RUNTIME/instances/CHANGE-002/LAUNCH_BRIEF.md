# CHANGE-002 Launch Brief v1

- RequestedBy: `HELP_HUMAN`
- RunID: `HELP-HUMAN-PIPING-20260821-LOCAL-FIRST-RUNTIME`
- Objective: land the validated N1 product, tests, minimal DEL-12-01 truth, and current AgentRuns evidence as one scoped node commit.
- Branch: `codex/piping-local-first-runtime-20260821`
- Base: `1b375af4f1219ecfc00fc2755854aa7fd4220901`
- AcceptedEvidence: WI-PKG12-001 `RETURN.md`; review attempt 2 PASS over exact 13-path product/test hash inventory; pinned full Piping 913 PASS; host DEC-025 PASS; final harness 350 PASS plus self-check; `git diff --check` PASS.
- StageScope: only the current changed/untracked paths under `projects/chirality-piping/**` belonging to this RunID and DEL-12-01 node. Exclude `loop/LOOP_RECEIPTS.md` (not yet written), all unrelated project/root paths, and any generated dependency/cache output.
- CommitMessage: `feat(piping): enforce local-first export routes`
- Actions: verify branch and exact scope; stage scope; validate cached diff and containment; commit. Do not push, fetch, merge, rebase, open PR, or append the receipt.
- Return: commit SHA, staged inventory, post-commit status, and any blocker.
