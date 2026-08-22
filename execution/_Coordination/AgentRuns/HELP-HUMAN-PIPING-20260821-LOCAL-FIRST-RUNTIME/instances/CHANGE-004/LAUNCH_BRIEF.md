# CHANGE-004 Launch Brief

- RequestedBy: `HELP_HUMAN`
- Objective: publish the clean validated tranche branch and open its single ready-for-review PR.
- Branch: `codex/piping-local-first-runtime-20260821`
- BranchHead: `6258eb699d05878921c2ea49d9aee7bff15bf3e4`
- PRBase: `main`
- Dependency: none; there were no open PRs at Step 0 and the tranche branched from current `main@1b375af4f`.
- RequiredPreflight: clean tree; fetch `origin main`; verify `origin/main` remains `1b375af4f` or stop and report main advancement without syncing; verify local branch contains the node and proof commits.
- Actions: push branch with upstream; open one non-draft PR against `main`; do not merge.
- PRTitle: `piping: enforce local-first export routes`
- PRDescription: concise product outcome, node/proof commits, exact clean-head sweep path and PASS counts, prior P1 repair/re-review, base `main`, dependency `none`, and standard claim fence.
- Return: pushed ref, PR number/URL/base/head, and initial check listing if available.
