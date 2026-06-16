---
run-id: WORKING_ITEMS_RUN_2026-06-16_TP-R3UX-SHELL-001
timestamp: 2026-06-16T11:02:31-06:00
run-status: SUCCESS
persona: WORKING_ITEMS
primary-deliverable: DEL-07-06
tranche-ids:
  - TP-R3UX-SHELL-001
smoke-ids:
  - TP-MAC-185
---

# WORKING_ITEMS Run Record - C5 guided workbench shell

## Objective

Implement C5.3 from the active completion plan: add the default guided
workbench shell before A12/R3 journey-specific repair work. This tranche
addresses the C5 baseline finding that the shell was structurally stable but
still exposed developer/evidence state before the user knew the next action.

## Authority and scope

- Human instruction in-session: implement the R3 Exit Readiness + Guided
  Workbench Redesign Program as a revision to the current completion plan.
- `plans/PLAN_2026-06-10_prd_completion.md` Phase C5: guided workbench before
  any R3 exit review or Phase D selection.
- `DEC-035`: F-4 and the authoring-journey usability finding must close before
  any R3 exit review.
- `DEC-037`: writable expression text syntax remains out of scope; no parser
  or text-entry syntax is introduced by this tranche.
- DEL-07-06 is the evidence home because this is an accessibility/usability
  shell tranche.

## Outputs produced

- Added a default guided workbench below the persistent spatial core:
  model tree, 3D centerline viewport, and property inspector stay visible.
- Added a task rail for model edits, loads, private libraries, rule packs,
  solve/check, results, report, and save/reopen.
- Added a current-step panel with compact queue and R3 exit journey status.
- Moved evidence/debug-heavy surfaces behind the Review evidence detail drawer
  in Operation Apply:
  - editor contract and surface coverage;
  - operation diff preview;
  - operation ledger;
  - agent proposal context.
- Preserved existing panel test IDs after the detail drawer opens.
- Added stable C5 shell IDs including `guided-workbench`, `journey-step-*`,
  `journey-current-step`, `review-apply-drawer`,
  `review-apply-drawer-toggle`, and `r3-exit-journey-status`.
- Updated `apps/desktop/SMOKE.md`, the active completion plan row,
  `plans/PLAN_COMPLETION_LOG.md`, this DEL-07-06 memory/run record, and
  `NEXT_INSTANCE_PROMPT.md`.

## Visual evidence

Captured in-app browser screenshots and probe JSON:

- `assets/TP-R3UX-SHELL-001_1440x920_iab.png`
- `assets/TP-R3UX-SHELL-001_1280x800_iab.png`
- `assets/TP-R3UX-SHELL-001_iab_probe.json`

Probe summary:

| Viewport | Horizontal overflow | Clipped guided-workbench primary controls | Drawer default |
|---|---:|---:|---|
| 1440x920 | false | 0 | closed |
| 1280x800 | false | 0 | closed |

## Validation

- `npm test --workspace apps/desktop -- App.test.tsx -t "guided workbench"`
  - focused guided-workbench Vitest passed: 1 test file / 1 test.
- `npm test --workspace apps/desktop -- App.deadControls`
  - dead-control audit passed.
- `npm run test:e2e --workspace apps/desktop -- -g "guided workbench"`
  - focused Playwright e2e passed: 2/2 at 1440x920 and 1280x800.
- In-app browser visual verification against the local dev server captured the
  screenshots/probe listed above.
- `npm test --workspace apps/desktop`
  - full desktop Vitest passed: 18 test files / 386 tests.
- `npm run build --workspace apps/desktop`
  - desktop production build passed, retaining the existing Vite chunk-size
    warning.

## Boundary review

This tranche changed frontend shell/usability behavior only. It did not change
the rule-pack schema, evaluator grammar, solver, persistence, backend API,
rule-pack checksum, lifecycle state, packaging status, or release status.

No protected standards content, private project data, repository-default
private-data write, network/telemetry feature, professional approval,
certification, sealing, authentication, code-compliance, release-readiness, or
R3 exit-review claim is created.

## Residuals and next item

- F-4 remains open until a human completes the packaged GUI journey.
- The A3 authoring-journey usability finding remains open until the packaged
  successor journey is human-completable and recorded.
- Next unblocked C5 implementation tranche: `TP-R3UX-A12FLOW-001` / target
  SMOKE TP-MAC-186, the A12 authoring journey redesign.
