# Structured Return — TASK-DEL0904-GUI-REVIEW-01

## Terminal verdict

`FAIL — ONE ACTIONABLE FINDING`

The frozen integrated N1 diff is not valid for manager fan-in yet. One test
ordering defect allows the new post-edit solve assertions to pass against a
fully hidden panel, while the manual and deliverable state claim a visible
solve transition.

## Finding

### F1 — Post-edit solve-state checks target hidden mounted DOM

- Severity: blocking for this review gate.
- Location: `projects/chirality-piping/apps/desktop/e2e/gui-workflow-validation.spec.ts:151-158`.
- Impact: after `openWorkspaceSection(page, "operations")`, the active dock
  section is Operations. The test then applies the edit and checks
  `solve-job-summary` only with `toContainText`. Inactive dock sections remain
  mounted but receive `.workspace-dock-section.inactive { display: none; }`
  (`apps/desktop/src/App.tsx:1382-1406`,
  `apps/desktop/src/styles.css:765-768`). Playwright text assertions do not by
  themselves establish visibility, so lines 156-157 can pass without a user
  being able to see the reset `state=not_started; result_rows=0` solve state.
  This is a false-positive gap against the objective and the visibility claims
  in `docs/validation_manual/index.md:202`, `_STATUS.md:11`, and
  `MEMORY.md:19-20`.
- Evidence: the preceding navigation and live dock implementation prove the
  Solve section is inactive at the assertion point. The later Results-panel
  assertion proves result clearing, but it does not prove the claimed visible
  solve-status transition.
- Remediation direction: after applying the intent, open the Solve section and
  assert both the section and the reset summary are visible before checking
  `state=not_started` and `result_rows=0`; then continue to the Results section.
  Re-run the focused two-project Playwright proof and affected checks, then
  freeze a new manifest for a fresh independent review.

## Coverage and identity

- Reviewed 100% of the base-to-worktree content for all nine paths frozen by
  `FROZEN_DIFF_MANIFEST.md`, including the four tracked deltas and all five
  untracked implementation/coordination records.
- Traced every added Playwright selector and expectation through the live
  desktop source, fixture data, solve/result state handling, local persistence,
  warning construction, boundary drawer, viewport configuration, and CSS
  visibility behavior.
- Verified all nine SHA-256 identities twice, before and after review. Both
  passes exactly matched the manifest:
  - `adafac0b5758eb6b109b30793c534f1df3ea86bb9377ad95e27d6505857601e3`
  - `34bba622b5311f2b89739c23b89eef8555dc27c3d836262a1489c1e911118a2f`
  - `c971dfb0214d17ff578594e67e0f828f8cb0ac83d3e3b3227210cb92d7cd53db`
  - `723628f50aeb17ee2938da82eb643252d3aae9511d7a3ca9819b2cdd7bf3c8b9`
  - `5d0ebc2a8e5ff2d429527cd7a5662c90b8ad450935b390a61131d54ce7ea65cb`
  - `e8a309d4239ace33431a9bb95b17697d2664c35eed80b9ce864ed57670f91d93`
  - `038ee295bca5bb32ed1aa4e66f00462b8bb5b9b8c00d24e8664777926a72a1aa`
  - `88fa71a22b55e17f529b09a8fe0cc9ed9882a6eefc871267ce2eea92ecb2ce54`
  - `e3a0595f53a30ec37e829510e2c0dee7df68ef375964ecb2c05538e2f5eba2af`

## Scope and evidence sufficiency

- Explicit nine-path scope validation: `PASS`, zero violations.
- The affected-check selector required `desktop-test`, `desktop-build`,
  `harness-pytest`, and `harness-self-check`.
- Frozen implementation evidence records `desktop-test` `523/523 PASS` and
  `desktop-build PASS`.
- Manager fan-in evidence
  `MANAGER_REGISTERED_CHECKS.json` (SHA-256
  `f8b55e46adf613b6866c1380270a0cf3471b841fb5519ea254cd4162a16ea8d2`)
  closes the initial evidence gap with `harness-pytest` `350/350 PASS` and
  `harness-self-check` exit `0`.
- The implementation run record reports Playwright discovery as two tests in
  one file and focused host execution as `2/2 PASS` across
  `chromium-desktop` and `chromium-compact`; the configuration declares exactly
  those two projects. This is sufficient for the existing execution claim,
  but the proof must be rerun after F1 is repaired.
- Manual calibration otherwise passes: GUI evidence moves only to
  `DRAFT_EVIDENCE`; no case page becomes `MAINTAINER_REVIEWED`; no threshold,
  release, publication, acceptance, or professional-reliance decision is
  implied. `_STATUS.md` removes only the GUI-workflow clause and preserves
  `IN_PROGRESS` plus both owner-gated residuals.

## Residual risk and fan-in validity

- Residual risk after F1: normal browser/viewport timing risk remains bounded
  by the required two-project host rerun; the invented-fixture and no-external-
  request scope remain appropriate for `DRAFT_EVIDENCE`.
- Manager fan-in validity: `INVALID` until F1 is repaired and the amended diff
  receives a fresh terminal review with zero actionable findings.

## Bounded TASK report

RUN_STATUS: `FAILED`

ControlSurface: `FILE`

TaskProfile: `NONE`

TaskSkill: `software-code-review`

ScopePath: `/Users/ryan/.codex/worktrees/4918/chirality/projects/chirality-piping/execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton`

ResolvedSkillPath: `/Users/ryan/.codex/worktrees/4918/chirality/skills/software-code-review`

ResolvedSkillVersion: `1`

ResolvedTaskProfileRequirement: `NONE`

CompanionFiles: `BRIEF_SCHEMA.md (found), TOOL_POLICY.md (found), QA_CHECKS.md (found)`

AllowedTools: `python3 tools/software_workflow/select_affected_checks.py`,
`python3 tools/software_workflow/validate_change_scope.py`, structured compare
and generated-manifest tools when relevant; direct read-only source/diff/hash
inspection required by the brief.

RuntimeOverrides: `none`

ToolsUsed:

- `python3 tools/software_workflow/validate_change_scope.py`
- `python3 tools/software_workflow/select_affected_checks.py`
- direct read-only repository, diff, and SHA-256 inspection

ToolPolicyCompliance: `PASS` — scope validation ran before affected-check
selection; structured-compare and generated-manifest tools were not relevant;
no implementation checks, installs, network operations, or mutations ran.

WriteAuthorization: runtime-owned `RETURN.md` and `STATUS.json` only.

Outputs:

- this terminal review return
- `STATUS.json`

MISSING: none.

NEEDS_HUMAN_RULING: none.

DEPENDENCY_NOTES: none.

ProposedChanges:

- Apply the bounded F1 repair above; no broader product or governance change is
  proposed.
