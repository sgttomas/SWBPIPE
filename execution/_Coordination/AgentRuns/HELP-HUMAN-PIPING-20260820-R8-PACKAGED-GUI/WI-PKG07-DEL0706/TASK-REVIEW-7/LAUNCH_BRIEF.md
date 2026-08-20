# Sealed brief — software code review 7

RunID: `HELP-HUMAN-PIPING-20260820-R8-PACKAGED-GUI`

Instance: `WI-PKG07-DEL0706-TASK-REVIEW-7`

Role: fresh Agent 2 software-code-review specialist; read-only and
non-delegating. The only permitted writes are this directory's `RETURN.md` and
`STATUS.json`. Do not modify any frozen member, source, test, evidence, status,
manager, telemetry, Git, receipt, register, PR, or dependency surface.

## Objective

Independently review 100% of the exact adjacent Amendment-7 remediation diff
against node commit `a9b1fbef90f3bb9a894054c22f6fc77572fedd0d`. Determine whether it is
valid for an adjacent CHANGE proof-loop commit and subsequent clean DEC-025
sweep. Return PASS only if there is no actionable finding.

## Required context and method

Read completely before review:

- repository `AGENTS.md`;
- `agents/AGENT_TASK.md`;
- `skills/software-code-review/SKILL.md`;
- `projects/chirality-piping/software-workflow.json`;
- `docs/SOFTWARE_WORKFLOW_PROFILE.md`;
- this brief and `FROZEN_ADJACENT_DIFF.json`.

Recompute every frozen SHA-256 and exact membership. The manifest excludes
only itself. Current Git paths must equal the manifest members plus the
self-excluded manifest and this review's eventual `RETURN.md`/`STATUS.json`.
Verify every path is contained by the graph's declared write roots. Review the
complete content/diff of every frozen member, not samples. Also read the
committed reviewer 1-6 returns and Amendments 1-6 to confirm their histories
are preserved and Amendment 7 has not reopened their findings.

Run the exact affected-check selector with
`PROFILE_PATH=projects/chirality-piping/software-workflow.json` and paths
relative to that project root. Reconcile all selected checks against
`ADJACENT_REMEDIATION_AFFECTED_CHECKS.json` and recorded evidence. In
particular, judge whether the accepted clean node-commit Python PASS is valid
for the immutable sweep-summary additions, and whether deferring the next
`evidence-sweep` to CHANGE after the adjacent commit faithfully follows the
explicit clean commit-bound proof requirement. Do not execute tests, builds,
browsers, GUI tools, generators, formatters, installers, or network calls.

## Review focus

- Smallest correct fix for the 1440x920 status-row click interception; no
  pointer-event bypass, hidden control, or layout regression.
- CSS behavior across the desktop and compact height rules, including whether
  `clamp(230px, 40vh, 540px)` actually resolves the viewport budget without
  creating an inaccessible pane.
- Playwright coverage must strengthen geometry evidence and retain both real
  clicks/downstream assertions; no test weakening or tautology.
- Truth and identity of both clean sweep summaries, including the first
  missing-`jsonschema` environment attempt and the second commit-bound
  20-pass/2-fail trigger.
- Consistency of Amendment 7, manager return, handoff, DEL run record,
  telemetry, graph, affected-check dispositions, and check evidence.
- No packaged edited-load/native solve predicate changed; no unsupported
  packaged rerun, release, accessibility-conformance, or professional-reliance
  claim.

## Return contract

Write `RETURN.md` with terminal `ReviewVerdict: PASS / VALID_FOR_ADJACENT_CHANGE`
or `FAIL / ACTIONABLE_FINDINGS`, exact hash/membership/containment/profile
results, 100% coverage statement, findings with file/line/remediation if any,
check-evidence reconciliation, prior-review preservation, and residual risks.
Write matching machine-readable `STATUS.json`. Do not delegate.
