**FINDINGS — three P2 issues need repair before CI-strategy fan-in.**

Reviewed frozen `3861b614362d8da925e148d7506ecc9902c7f808..097decd6d4c89bacb0909c9c798933835ed0f27b`, including all four maintained CI files, strategy/tranche/notice, and worker evidence.

1. **P2 — instrument-directory reduction omits actual dependents.**  
   [e2e_plan.py:110](/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-wt2/projects/chirality-piping/tools/ci/e2e_plan.py:110) treats every `e2e/ui-foundation/**` change as instrument-only. However, `ui-foundation-workflows.ts` imports `installInstrumentation`, `routeModelFixture`, `settledRafState`, and `treeRowTestId` from that directory and reads its fixtures. These serve ordinary `ui-foundation.spec.ts` and `workspace-layout.spec.ts` journeys, which this mode omits. A shared helper or fixture regression can therefore pass reduced CI without exercising its affected journeys. Remove the directory-wide reduction or establish a verified dependency mapping; ROOT’s proposed full fallback for helpers/fixtures resolves this conservatively.

2. **P2 — PR-head checkout drops target-branch integration coverage.**  
   [piping-desktop-e2e.yml:67](/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-wt2/.github/workflows/piping-desktop-e2e.yml:67), repeated in every execution job, replaces the previous default merge checkout with `pull_request.head.sha`. If the target branch advances independently, the workflow tests the old head combination and can report green without testing its integration with that target. No gate requires the event’s target-base SHA to be an ancestor of the tested head; the routing test explicitly accepts divergence. Preserve synthetic-merge execution, or reject an unintegrated target base and retain its identity separately from the merge base. This checkout distinction is documented by [actions/checkout](https://github.com/actions/checkout#checkout-pull-request-head-commit-instead-of-merge-commit).

3. **P2 — required focused cases are not checked against collected tests.**  
   [e2e_plan.py:156](/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-wt2/projects/chirality-piping/tools/ci/e2e_plan.py:156) builds an OR filter, while execution at line 207 checks only process success. If one required title stops matching, the remaining titles can pass and conceal the missing coverage. An in-memory probe confirmed that an absent fourth title passes plan validation. Validate candidate-bound collected test identities for every required title/profile and nonempty selected file before execution. Retained historical listing evidence does not enforce this on future runs.

Other review results:

- All four output hashes and eleven supplied-source hashes match. Brief SHA-256 matches `547dc7…3a63c`.
- Independently verified the historical exact-once partition: **430 = 24 + 147 + 59 + 144 + 56**, with eight focused title/profile entries. This remains prior-source inventory.
- **500 pure aggregation combinations passed**; no additional finding in the status predicate, dependency gating, safe argument arrays, or full/reduced claim labeling.
- Composite setup is identical to the previous setup apart from required explicit Bash declarations.
- `git diff --check` reports a blank EOF in the composite and whitespace in retained evidence. Preserve raw evidence rather than rewriting it for hygiene.

Checks used read-only Git/file inspection, in-memory Python routing/aggregation probes, and historical JSON inventory comparison. No files, Git state, browsers, native sessions, or hosted jobs were changed or launched.

The worker’s 18 unit passes remain reported evidence. Final candidate inventory, repaired-policy checks, and actual hosted execution are still owed. Existing independent TASK/Astra-xhigh attribution and telemetry limits apply; no acceptance or final B3 merge suitability is established.
