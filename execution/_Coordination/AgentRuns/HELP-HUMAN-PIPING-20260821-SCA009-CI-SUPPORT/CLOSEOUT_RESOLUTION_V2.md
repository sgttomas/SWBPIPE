# Closeout resolution v2 — owner-resolved all-node fan-in

## Authority

- Exact owner direction: “Good. I shouldn't have said that though. Try to close everything out. Resolve failures.”
- Effect: withdraws the earlier stop rule for ordinary bounded repair/closeout while preserving every failure record.

## Preserved failures and resolutions

- N1's first CHANGE closeout failure remains in `N1_CLOSEOUT_FAILURE.md`: six hash-pinned derivative files carried an extra terminal blank line. `N1_RESOLUTION_V2.md` records regeneration, exact one-newline normalization, manifest rebinding, and passing focused closeout evidence.
- N4's first registered-profile failure remains in `N4_CHECKS.json`, `N4_REVIEW.md`, and the historical sections of `N4_RETURN.md`. `N4_RESOLUTION_V2.md`, `N4_CHECKS_V2.json`, `N4_FROZEN_DIFF_V2.md`, and `N4_REVIEW_V2.md` record the resolved environment/revision-pin conditions and successful terminal review.

## Control-surface normalization

- All four launch briefs and `ORCHESTRATION_PLAN.md` were checked for terminal-newline form.
- `N1_LAUNCH_BRIEF.md` already had exactly one terminal newline.
- One extra terminal blank line was removed from `N2_LAUNCH_BRIEF.md`, `N3_LAUNCH_BRIEF.md`, `N4_LAUNCH_BRIEF.md`, and `ORCHESTRATION_PLAN.md` under the owner's ordinary closeout authorization.
- No launch scope, objective, acceptance criterion, exclusion, or model attribution changed.
- The first final self-check identified two machine-local Python paths in `N4_CHECKS_V2.json`. They were replaced with a host-local cached-Python placeholder while preserving the environment version, offline posture, command semantics, and check results; the follow-up self-check no longer reports that active-surface portability finding.

## Git/PR state

- Branch: `codex/piping-sca009-ci-support-20260821`.
- PR: [#599](https://github.com/sgttomas/chirality/pull/599) against `main`.
- Already committed and pushed: N2 `b988d9d0e4a7048ac28a73bbe53ce045c631dff8`; N3 `ffbc4834389f3095d22896e126b39085c3e00369`.
- Uncommitted final candidate: current N1 project/control records, N4 product/revision-pin records, DEL-04-03 and DEL-07-09 shared status/coverage, final AgentRuns fan-in, and Receipt-122.

No broad project tests or DEC-025 sweep were run by this final fan-in. Those remain with CHANGE after the final candidate is committed on an integrated clean tree.
