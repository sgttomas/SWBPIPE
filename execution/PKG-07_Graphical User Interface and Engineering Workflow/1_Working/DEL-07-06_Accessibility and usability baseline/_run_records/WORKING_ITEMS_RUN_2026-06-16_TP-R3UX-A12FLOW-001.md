---
run-id: WORKING_ITEMS_RUN_2026-06-16_TP-R3UX-A12FLOW-001
timestamp: 2026-06-16T11:43:27-06:00
run-status: SUCCESS
persona: WORKING_ITEMS
primary-deliverable: DEL-07-06
tranche-ids:
  - TP-R3UX-A12FLOW-001
smoke-ids:
  - TP-MAC-186
---

# WORKING_ITEMS Run Record - A12 guided authoring journey

## Objective

Implement C5.4 from the active completion plan: make the blank-model A12
authoring journey human-completable without reading developer state. The
journey must expose a clear next action, queue status, inline review/apply
affordance, and obvious navigation through node/material/section/pipe/support,
load, solve, report, save, and reopen work.

## Authority and scope

- `plans/PLAN_2026-06-10_prd_completion.md` Phase C5, item C5.4.
- `DEC-035`: F-4 and the authoring-usability finding must close before any R3
  exit review.
- `DEC-037`: writable expression text syntax remains out of scope.
- Human direction: guided workbench, not a wholesale product rewrite.
- Allowed implementation scope: frontend guided-workbench navigation/state and
  tests/evidence. No rule-pack schema, evaluator grammar, solver, persistence,
  or backend API changes were planned or made.

## Outputs produced

- Added an A12 authoring panel to the guided workbench.
- Added stable IDs:
  - `a12-authoring-journey`
  - `a12-next-action`
  - `a12-journey-progress`
  - `a12-journey-step-*`
  - `a12-queue-status`
  - `a12-next-action-button`
  - `a12-review-apply-button`
- The checklist is computed from the active model/session state:
  blank document, node count, material count, section count, pipe count,
  support count, load-case count, primitive-load count, combination count,
  solved result status, report readiness, and save/reopen state.
- The queue status names the created operation payload object when the queued
  intent carries a JSON `after.id`; otherwise it falls back to the target ref.
- `Apply queued` uses the existing `handleApplyIntent` path and therefore
  preserves validation/application semantics, receipt recording, undo
  checkpointing, computed-result clearing, and the local wasm engine route.
- A12 step buttons keep selected-step state so controls mapping to the same
  workspace section visibly respond and do not regress the dead-control audit.
- Compact-height CSS keeps the lower dock usable at the 1280x800 viewport.

## Visual evidence

Captured in-app browser screenshots and probe JSON:

- `assets/TP-R3UX-A12FLOW-001_1440x920_iab_blank.png`
- `assets/TP-R3UX-A12FLOW-001_1280x800_iab_queued.png`
- `assets/TP-R3UX-A12FLOW-001_1280x800_iab_applied.png`
- `assets/TP-R3UX-A12FLOW-001_iab_probe.json`

Probe summary:

| State | Viewport | Horizontal overflow | Primary A12 controls clipped | Key signal |
|---|---:|---:|---:|---|
| blank | 1440x920 | false | 0 | next action `Add two nodes in the viewport` |
| queued | 1280x800 | false | 0 | inline `Apply queued` visible in viewport |
| applied | 1280x800 | false | 0 | `route=local_wasm_engine`; queue cleared |

## Validation

- `npm test --workspace apps/desktop -- App.test.tsx -t "guided workbench"`
  - focused guided-workbench Vitest passed: 1 test file / 1 test.
- `npm run test:e2e --workspace apps/desktop -- -g "R2 from-blank GUI journey"`
  - focused A12 Playwright e2e passed: 2/2 at 1440x920 and 1280x800.
- `npm test --workspace apps/desktop -- App.deadControls.test.tsx`
  - permanent dead-control audit passed.
- `npm test --workspace apps/desktop`
  - full desktop Vitest passed: 18 test files / 386 tests.
- `npm run build --workspace apps/desktop`
  - desktop production build passed, retaining the existing Vite chunk-size
    warning.
- `npm run test:e2e --workspace apps/desktop`
  - full Playwright e2e passed: 12/12 across 1440x920 and 1280x800.
- In-app browser visual verification against the local dev server captured the
  screenshots/probe listed above.

## Boundary review

This tranche changed frontend guided-authoring usability only. It did not
change the rule-pack schema, evaluator grammar, solver, persistence, backend
API, project-store semantics, rule-pack checksum, lifecycle state, packaging
status, release status, private-data boundary, protected-content boundary,
network/telemetry posture, or professional boundary.

No protected standards content, private project data, repository-default
private-data write, network/telemetry feature, professional approval,
certification, sealing, authentication, code-compliance, release-readiness, or
R3 exit-review claim is created.

## Residuals and next item

- F-4 remains open until a human completes the packaged GUI journey.
- The authoring-usability finding remains open until the packaged successor
  journey is human-completable and recorded.
- Next unblocked C5 implementation tranche: `TP-R3UX-R3FLOW-001` / target
  SMOKE TP-MAC-187, the R3 rule-pack/private-library guided flow.
