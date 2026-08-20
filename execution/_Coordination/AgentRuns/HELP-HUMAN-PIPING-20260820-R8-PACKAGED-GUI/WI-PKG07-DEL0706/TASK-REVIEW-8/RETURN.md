# Software code review 8 return — PASS

RUN_STATUS: `SUCCESS`

ReviewVerdict: `PASS / VALID_FOR_ADJACENT_CHANGE`

ControlSurface: `FILE`

TaskProfile: `NONE`

TaskSkill: `software-code-review`

ScopePath: `/Users/ryan/.codex/worktrees/4918/chirality/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-R8-PACKAGED-GUI/WI-PKG07-DEL0706/TASK-REVIEW-8`

ResolvedSkillPath: `/Users/ryan/.codex/worktrees/4918/chirality/skills/software-code-review`

ResolvedSkillVersion: `1`

ResolvedTaskProfileRequirement: `NONE`

CompanionFiles: `BRIEF_SCHEMA.md (found), TOOL_POLICY.md (found), QA_CHECKS.md (found)`

WriteAuthorization: `ALLOWED_WRITE_TARGETS — RETURN.md and STATUS.json only`

## Frozen identity, membership, and containment

- Basis: `a9b1fbef90f3bb9a894054c22f6fc77572fedd0d` on
  `codex/piping-product-20260820`.
- SHA-256: PASS, 19/19. Every recomputed digest equals the review-8
  manifest:
  - `3fd5c60009cf74da7ced6f841e58a796aa92535b38ee7d7ed3e66bddd7a4e727`
    — `apps/desktop/e2e/r2-smoke.spec.ts`
  - `9e308acdccab0cbf6311360730b6dcd363e30de548d6985bb5e7f84015d10357`
    — `apps/desktop/src/styles.css`
  - `3896d8f5d583e3f22ed98248ea58eb11f591d49535a4ca17d9b2eae2c04281cd`
    — DEL run record
  - `c9731359e5e643deaf9b9b14bd02913d8f20e7db193c250be37bb0d1902baf87`
    — `WORK_GRAPH.json`
  - `ef10400055e9f19d9e917b895f1537a217825e10fee0a03aa1b2510005b406bb`
    — `ADJACENT_REMEDIATION_AFFECTED_CHECKS.json`
  - `4ff34482dcf41a52584db6d0fa5a3bedd503156dd2601818ca74b7280f12f449`
    — `ADJACENT_REMEDIATION_REGISTERED_CHECKS.json`
  - `7d9ff261bcf471763c065fac5808b158279e6eb8b38967c97787bbc39dacdec3`
    — `AMENDMENT_7.md`
  - `a14c38c92583511dc143c23380f89c8fe3edeeca494be55d602470755ad4426d`
    — `AMENDMENT_8.md`
  - `31cc154daf5115a164f88ce93aff6537d8718587952ddb53cbf6a43cc28b907e`
    — `HANDOFF_STATE.md`
  - `a0e2522ac5e5e067916991b1687e53786417d4531546dbf257f675ccc9d78614`
    — `MANAGER_RETURN.md`
  - `c4fab5cd1535c425ffb8970cdc2cae0f918fc9fbec2acd4d6a54b62bf125a7cb`
    — `RUNTIME_EVENTS.jsonl`
  - `02c8fb692f468b77e5651a2261bb6537bdc39e36653b003acc61ef86c67908bb`
    — `RUNTIME_SUMMARY.json`
  - `8c6b4feb3af64496a258a15bc699b6901bed25fb362002fb36eaa2585c16599b`
    — review-7 `FROZEN_ADJACENT_DIFF.json`
  - `5f6f4849b91e8c646837dcba34dd2733ab467eab48a7ebe0d5239b7c107c6b69`
    — review-7 `LAUNCH_BRIEF.md`
  - `5e30ba8dfd77357d30f4cd1d9148bbfc2015a482b42c53f137c5758cc20a670d`
    — review-7 `RETURN.md`
  - `f597ad76da510dd935392619b17b73b7fa5c04a27d0b9d5f2ed6e581ddba6452`
    — review-7 `STATUS.json`
  - `d89d687738db5ff52f8e73423051d44bdb816f5de3466e839ed51ab3f936edea`
    — review-8 `LAUNCH_BRIEF.md`
  - `c66786be239b46f5d8fdc16c754c31f9eaff55de99f684a1b7d7970d4173e7df`
    — `SWEEP_20260820T214752Z_a9b1fbef90f3.json`
  - `79301f7651bce52a492562a001d65b959c368b8d02cdf896bc3927d911c3a078`
    — `SWEEP_20260820T214858Z_a9b1fbef90f3.json`
- Pre-output membership: PASS, exactly 20 paths after expanding untracked
  directories: the 19 manifest members plus the self-excluded review-8
  manifest. There were no hidden or unexpected paths. The only authorized
  post-review additions are this `RETURN.md` and `STATUS.json`.
- Containment: PASS, 19/19. Every member matches one of graph-v9's seven
  declared targets. The two validation members match only their two exact
  immutable sweep-summary targets; no `validation/**` or other broad
  validation root was introduced.
- Frozen coverage: 100%. I read the full contents and adjacent diff of all 19
  members, the review-8 manifest, Amendments 1-6, reviewer histories 1-6, and
  the preserved review-7 FAIL artifacts.

## Reviewer-7 finding closure and product review

Reviewer 7's sole actionable finding is CLOSED. Graph v9 adds exactly the two
sweep-summary paths Amendment 7 authorized and nothing broader. Review-7's
FAIL, manifest, brief, and status are preserved in the new freeze.

No actionable product-code finding remains.

- Product/test drift from review 7: none. The Playwright and CSS hashes remain
  exactly `3fd5c600...` and `9e308acd...`; Amendment 8 changed control state
  only.
- The CSS change remains a structural viewport-budget repair, not a
  hit-testing bypass. At 1440x920, `clamp(230px, 40vh, 540px)` resolves to
  368px instead of the former 540px floor, returning 172px to the flex budget.
  The panes and dock retain their own scrolling; no pointer-event, visibility,
  or status-control operability rule changed. The later compact-height and
  compact-width rules retain their 210px/230px floors, while sufficiently tall
  displays still reach the 540px ceiling.
- The Playwright helper scrolls each real target into its own scroll surface,
  obtains independent target and status rectangles, and rejects geometric
  overlap. Both original `.click()` calls and their downstream result-detail
  and report-redaction assertions remain. There is no force-click, synthetic
  dispatch, hidden status surface, or tautological assertion.
- Amendments/reviewers 1-6 remain preserved. The adjacent CSS/test repair does
  not touch the solve job/model/generation/cancellation implementation and
  therefore does not reopen their closed findings.

## Affected checks and evidence reconciliation

Exact selector execution over all 19 project-relative paths and the project
JSON profile returned, in sorted order:

1. `desktop-build`
2. `desktop-test`
3. `evidence-sweep`
4. `harness-pytest`
5. `harness-self-check`
6. `piping-pytest`

This exactly matches `ADJACENT_REMEDIATION_AFFECTED_CHECKS.json`.

- `desktop-build`: recorded PASS; only the existing Vite chunk-size warning.
- `desktop-test`: recorded PASS, 29 files / 533 tests.
- Focused Playwright: recorded PASS, the two formerly failing desktop cases
  2/2. Full Playwright: recorded PASS, desktop and compact 22/22.
- `harness-pytest`: normalized registered evidence records exit zero and
  350/350 tests.
- `harness-self-check`: normalized registered evidence records exit zero; its
  repository-wide REVIEW/WARN findings are preserved and are not represented
  as new R8 failures.
- `piping-pytest`: accepting the clean basis-commit Python PASS is reasonable
  for the two immutable summary additions because the adjacent product change
  is CSS/Playwright-only and no Python implementation changed. The second
  commit-bound sweep records Python exit zero. The exact 902 count is manager
  record provenance rather than a field embedded in the summary schema.
- `evidence-sweep`: correctly deferred to CHANGE after the adjacent commit.
  The required proof is clean and commit-bound, so an uncommitted rerun could
  not substitute for it. This review validates the tree for that adjacent
  commit; it does not claim the deferred sweep has passed.

Both sweep summaries remain byte-identical and clean-bound to
`a9b1fbef90f3bb9a894054c22f6fc77572fedd0d`. The first records Python exit 2
with later surfaces not run; its missing-`jsonschema` classification depends
on the preserved run/manager record because the compact summary omits command
output. The second records Cargo, Python, and desktop Vitest PASS, Playwright
FAIL, build not run, and overall FAIL. No packaged native proof predicate or
packaged executable changed, and no packaged rerun is claimed.

## Findings, tool policy, and residual risk

Actionable findings: `none`.

ToolsUsed:

- `python3 tools/software_workflow/select_affected_checks.py`

ToolPolicyCompliance: `PASS`

No tests, builds, browsers, GUI tools, generators, formatters, installers,
network operations, Git mutations, or delegation were performed.

Residual risks are non-blocking and accurately fenced:

- the sweep summary schema records surface status/exit/duration rather than
  full stdout, so exact test counts and the first interpreter's module error
  rely on the preserved manager/run evidence;
- browser desktop/compact evidence covers the affected layout surface but does
  not claim a new native packaged journey;
- the next clean DEC-025 sweep remains a mandatory CHANGE-owned gate after the
  adjacent commit.

Outputs:

- `RETURN.md`
- `STATUS.json`

MISSING: `none`

NEEDS_HUMAN_RULING: `none`

DEPENDENCY_NOTES: `none`

Fan-in validity: `true` for the adjacent CHANGE proof-loop commit. Receipt,
push, and PR remain stopped until the subsequent clean commit-bound DEC-025
sweep passes.
