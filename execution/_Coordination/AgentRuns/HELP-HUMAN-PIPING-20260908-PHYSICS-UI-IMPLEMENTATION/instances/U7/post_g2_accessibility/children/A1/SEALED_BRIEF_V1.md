# U7/A1 sealed accessibility repair brief

Status: SEALED FOR EXECUTION
Parent: U7, WORKING_ITEMS Agent 1
Executor: bounded Agent 2, `gpt-5.6-sol`, high reasoning, no delegation
Head: `f67498d4fe8df4f5a7c0d6c279a8e62e1cc77634`

## Objective

Repair the unchanged `src/App.deadControls.test.tsx` failure at line 211 by giving `apply-reviewed-draft` a truthful accessible disabled-state reason. Preserve its exact existing disabled predicate `!draftReview || draftReviewBusy || !onApplyDraft`, enabled label `Apply`, click handler and all Add/review/Apply behavior.

The sole source is `projects/chirality-piping/apps/desktop/src/features/viewport/PipeViewport.tsx`, preimage SHA-256 `ac3789a6b4e8c452fe398d062183b25d884f55f5763af877ac6f1b3d4cb9f2ad`. Prefer the smallest `title` or accessible-description expression that distinguishes missing review, busy request, and unavailable Apply handler. Do not change enablement order, semantics, state, handlers, contracts, tests, model/physics, or UI flow.

## Write scope

- `projects/chirality-piping/apps/desktop/src/features/viewport/PipeViewport.tsx`
- `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/U7/post_g2_accessibility/children/A1/**`
- `projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/PHYSICS_UI_IMPLEMENTATION_20260908/post_g2_accessibility/children/A1/**`

All other paths are read-only. Do not edit the audit or other tests. Do not run builds, native, Rust, WASM, Python, physics, the full suite, global validators, or Git mutations.

## Validation and return

Run the unchanged focused dead-controls audit. Also run the existing relevant `App.test.tsx` draft-review tests needed to show enabled Apply and busy/stale behavior remain unchanged; use test-name filtering and report exact counts. Stop source edits after PASS.

Return the one-file diff and postimage SHA-256, exact commands/results, confirmation that the disabled predicate/click handler and App producer/consumer contracts are unchanged, and a brief native-impact assessment. Evidence must be portable, LF-clean, concise, and contain no raw unified-diff file; encode any required diff losslessly as base64 JSON or provide a whitespace-clean change map.
