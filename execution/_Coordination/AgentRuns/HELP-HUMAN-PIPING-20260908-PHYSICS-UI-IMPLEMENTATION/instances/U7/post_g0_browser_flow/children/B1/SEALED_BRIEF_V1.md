# U7/B1 sealed browser-flow maintenance brief

Status: SEALED FOR EXECUTION
Parent: U7, WORKING_ITEMS Agent 1
Executor: bounded Agent 2, `gpt-5.6-sol`, high reasoning, no delegation
Head: `7b73460c5e2d85a9f050344069d211fea4af7b3e`
Authority: owner direction `adfa58c038967c4a62fa490edc49139d33fb9442af99bb750ca66009ee3c4326`; ownership transfer `317732dcd0c786674ae58168b1bd0d056a69c9b06af50444b30bea19ead3a6b7`; actual F4/root release received after the transfer record.

## Objective

Update both affected journeys in `projects/chirality-piping/apps/desktop/e2e/r2-smoke.spec.ts` comprehensively for the accepted viewport Add -> frozen review -> Apply behavior. Preserve equivalent visible intent, receipt, unit, model, and solver assertions. Keep legacy queued-operation coverage for forms and placeholder tools that still use `editor-intent-N`.

The preimage SHA-256 is `be0650f640986a9282db43803107439f0c08d71a7c37639508e3758fa6c13248`. The two journeys are:

1. `R2 desktop preview smoke covers solve, results, report, and viewport overlay`.
2. `R2 from-blank GUI journey authors the A12 rehearsal script`.

## Required implementation

- Inspect each entire journey and related helpers before editing. Do not repair one failing assertion at a time.
- Add a focused helper for direct reviewed drafts, or equivalent clear code, that asserts the frozen review kind, operation IDs/diff, visible unit, validated model hash, Apply action, model publication, receipt route, acceptance boundary, and total applied count.
- Keep `applyQueuedIntent` for legacy forms, but separate its queued `editor-intent-N` sequence from total applied receipt sequence. Direct node/pipe applies increment applied receipts without consuming editor-intent IDs.
- Main journey: replace the obsolete queued-node flow with direct single-operation review/Apply. Preserve unit/model/receipt and solve-clearing evidence. Update the later legacy combination to its true queued-intent key and total receipt index.
- From-blank journey: use direct single-operation review/Apply for both nodes and the existing-end pipe. Update material, section, support, load-case, primitive-load, and combination queue keys and total receipt indexes without reducing their assertions.
- Preserve the separate `viewport gesture placeholders record unit validation` legacy queue test unchanged unless a same-file helper signature requires a mechanical call update.
- Do not change fixture values or the already-correct explicit provenance assertions.

Direct-review receipt IDs are based on `viewport-draft-review-N`; legacy queued receipt IDs retain `applied-N-editor-intent-M`. Review evidence exposes `Single operation` or `Atomic batch`, operation IDs, hash-bound validation, diff and `[m]`. Use current visible behavior rather than inventing product state.

## Write scope

- `projects/chirality-piping/apps/desktop/e2e/r2-smoke.spec.ts`
- `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/U7/post_g0_browser_flow/children/B1/**`
- `projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/PHYSICS_UI_IMPLEMENTATION_20260908/post_g0_browser_flow/children/B1/**`

All other paths are read-only. Do not run builds, native, Rust, WASM builds, physics, Python, full sweeps, or Git mutations.

## Validation and return

Run the two named journeys in both configured Playwright projects using the prepared engine artifacts and direct Playwright invocation. Preserve every failure as concise evidence and repair only this file. Once both journeys pass 4/4 on one source cut, stop editing and return:

- final test-file SHA-256 and unified diff against the sealed preimage;
- exact command and 4/4 project/journey result;
- an old-to-new assertion map for node/pipe direct review and downstream legacy queue/receipt numbering;
- containment confirmation and concise status/return records, LF-clean with one final newline.
