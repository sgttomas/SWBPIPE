# Software code review 7 return — FAIL

RUN_STATUS: `FAILED`

ReviewVerdict: `FAIL / ACTIONABLE_FINDINGS`

ControlSurface: `FILE`

TaskProfile: `NONE`

TaskSkill: `software-code-review`

ScopePath: `/Users/ryan/.codex/worktrees/4918/chirality/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-R8-PACKAGED-GUI/WI-PKG07-DEL0706/TASK-REVIEW-7`

ResolvedSkillPath: `/Users/ryan/.codex/worktrees/4918/chirality/skills/software-code-review`

ResolvedSkillVersion: `1`

ResolvedTaskProfileRequirement: `NONE`

CompanionFiles: `BRIEF_SCHEMA.md (found), TOOL_POLICY.md (found), QA_CHECKS.md (found)`

WriteAuthorization: `ALLOWED_WRITE_TARGETS — RETURN.md and STATUS.json only`

## Frozen identity and membership

- Basis: current `HEAD` is exactly
  `a9b1fbef90f3bb9a894054c22f6fc77572fedd0d` on
  `codex/piping-product-20260820`.
- SHA-256: PASS, 14/14. Every recomputed digest equals the manifest:
  - `3fd5c60009cf74da7ced6f841e58a796aa92535b38ee7d7ed3e66bddd7a4e727`
    — `apps/desktop/e2e/r2-smoke.spec.ts`
  - `9e308acdccab0cbf6311360730b6dcd363e30de548d6985bb5e7f84015d10357`
    — `apps/desktop/src/styles.css`
  - `369058517102448d44d659fab01ddca83102a619c64768fa54f9a96e323a6c88`
    — DEL run record
  - `3c47e557fcd955158045acf8f168a813df3f69b23e05ee09cb54937c48124b25`
    — `WORK_GRAPH.json`
  - `ef10400055e9f19d9e917b895f1537a217825e10fee0a03aa1b2510005b406bb`
    — `ADJACENT_REMEDIATION_AFFECTED_CHECKS.json`
  - `4ff34482dcf41a52584db6d0fa5a3bedd503156dd2601818ca74b7280f12f449`
    — `ADJACENT_REMEDIATION_REGISTERED_CHECKS.json`
  - `7d9ff261bcf471763c065fac5808b158279e6eb8b38967c97787bbc39dacdec3`
    — `AMENDMENT_7.md`
  - `9c2b8ce6e64bb6424c32895b1ddb05e682c6040fb4cfbac8164e0072b58e1b23`
    — `HANDOFF_STATE.md`
  - `502fb0e6ad83c5057be96d30ff6ff1ccc45dcb352aff0e88a679619f367ff844`
    — `MANAGER_RETURN.md`
  - `efa99f0f1d90b3eb49eae0fd3e73d3ee090fffcb61d3dceeb10014161e8ee402`
    — `RUNTIME_EVENTS.jsonl`
  - `2c4e3ef82c1aed38a86d1954e91a0e196d4d8799be29b35735e96c335532d2f0`
    — `RUNTIME_SUMMARY.json`
  - `5f6f4849b91e8c646837dcba34dd2733ab467eab48a7ebe0d5239b7c107c6b69`
    — review-7 `LAUNCH_BRIEF.md`
  - `c66786be239b46f5d8fdc16c754c31f9eaff55de99f684a1b7d7970d4173e7df`
    — `SWEEP_20260820T214752Z_a9b1fbef90f3.json`
  - `79301f7651bce52a492562a001d65b959c368b8d02cdf896bc3927d911c3a078`
    — `SWEEP_20260820T214858Z_a9b1fbef90f3.json`
- Membership: PASS. Before this return, expanded `git status -uall` contained
  exactly the 14 manifest members plus the self-excluded manifest. After this
  return, the only additional paths are the brief-authorized `RETURN.md` and
  `STATUS.json`; there are no hidden directory extras.
- Frozen coverage: 100%. I reviewed the complete adjacent diff/content of all
  14 members, not a sample, plus manifest structure, Amendments 1-6, and the
  committed reviewer 1-6 returns.

## Actionable finding

1. **HIGH / blocking control defect — the frozen adjacent diff is outside the
   graph's declared five-root write boundary.** `WORK_GRAPH.json:24-29`
   declares only the desktop tree, two exact DEL files, the DEL run-record
   tree, and the R8 AgentRuns tree. The manifest nevertheless includes both
   `validation/evidence/sweeps/SWEEP_20260820T214752Z_a9b1fbef90f3.json` and
   `SWEEP_20260820T214858Z_a9b1fbef90f3.json` at
   `FROZEN_ADJACENT_DIFF.json:25-26`. Semantic five-root validation therefore
   returns FAIL: 12/14 contained and these exact two violations. Amendment 7
   authorizes preserving the summaries, but `WORK_GRAPH.json` is the declared
   scope surface the sealed review requires and its version-8 update did not
   carry that expansion. The graph's fan-in predicate `writes remain
   contained` is consequently false, so CHANGE must not stage this freeze.
   Remediation: add only the two exact summary paths to the graph's declared
   write targets (or remove/reconcile them if they are not part of this
   tranche), update the graph/manifest hashes, refreeze exact membership, and
   run a fresh read-only review.

## Product and test review

No independent product-code finding was found.

- The CSS change is structural, not a hit-testing bypass: no pointer-event or
  hidden-control rule was added. At 1440x920, the new minimum resolves to
  368px instead of 540px, returning 172px to the workspace/dock budget. Pane
  and dock scrolling remain explicit. The existing `max-height: 840px` and
  later `max-width: 1320px` rules retain 210px/230px compact floors, and the
  clamp returns to the 540px ceiling on sufficiently tall displays.
- The Playwright helper scrolls the real target, compares independently
  obtained target/status rectangles, then the tests retain both original real
  `.click()` operations and downstream result/report assertions. It is not a
  forced click, event dispatch, pointer bypass, or tautological assertion.
- No packaged edited-load persistence, native solve, identity, cancellation,
  or result predicate changed. A native predicate rerun is not claimed.
- Amendments/reviewers 1-6 are byte-preserved committed history. Amendment 7
  does not touch the job/model/generation/cancellation implementation, so it
  does not reopen their closed findings.

## Check-evidence reconciliation

- Exact affected-check selection over all 14 project-relative paths returned
  `desktop-build`, `desktop-test`, `evidence-sweep`, `harness-pytest`,
  `harness-self-check`, and `piping-pytest`. This exactly matches
  `ADJACENT_REMEDIATION_AFFECTED_CHECKS.json`.
- Recorded results reconcile for desktop build, desktop Vitest 29 files / 533
  tests, focused Playwright 2/2, full Playwright 22/22, harness pytest 350, and
  harness self-check execution. The registered-check JSON parses and its two
  recorded harness commands both exited zero. Runtime telemetry also parses
  and reconciles to 33 events, eight sessions, and a four-event PASS attempt-3
  session.
- Accepting the clean node-commit Python result for these immutable summary
  additions is reasonable: the second summary is clean-bound to the exact
  basis commit and records Python exit zero, while the adjacent implementation
  changes no Python source. The summary artifact does not embed its console
  count, so the 902 count remains manager-record provenance rather than a
  field of the summary itself.
- Deferring `evidence-sweep` to CHANGE after the adjacent commit is faithful to
  the explicit clean commit-bound proof rule; an uncommitted rerun could not
  provide that proof. This disposition remains pending and does not cure the
  present write-boundary failure.
- Both sweep summaries are byte-identical to their declared hashes and record
  a clean basis tree at the exact commit. The first records bare-Python pytest
  exit 2 followed by no later surfaces; the missing-`jsonschema`
  classification is preserved by Amendment 7/manager records. The second
  records Cargo/Python/desktop-Vitest passes, Playwright exit 1, build not run,
  and overall fail. Exact 902 and 20/2 counts are manager-record detail rather
  than fields embedded in the summary schema.

## Tool policy and residual risk

ToolsUsed:

- `python3 tools/software_workflow/validate_change_scope.py`
- `python3 tools/software_workflow/select_affected_checks.py`

ToolPolicyCompliance: `PASS`

Residual risk after the blocking control defect is repaired: the sweep-summary
schema stores exit state and duration, not complete stdout/stderr, so its exact
test counts and the first interpreter's missing-module text depend on the
preserved manager/run records. The layout proof covers the affected browser
desktop/compact surface; it does not claim a new packaged native journey.

Outputs:

- `RETURN.md`
- `STATUS.json`

MISSING: `none`

NEEDS_HUMAN_RULING: `none`

DEPENDENCY_NOTES: `none`

Fan-in validity: `false`. CHANGE remains stopped pending scope reconciliation,
refreeze, and fresh review.
