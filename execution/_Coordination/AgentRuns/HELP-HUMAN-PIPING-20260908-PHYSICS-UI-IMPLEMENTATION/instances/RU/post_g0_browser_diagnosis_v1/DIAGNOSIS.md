# RU post-G0 browser diagnosis

Verdict: **STALE TEST FLOW — NO PRODUCTION REGRESSION CONFIRMED**

## Exact cause

The failing interaction leaves one required node field empty:

1. apps/desktop/e2e/r2-smoke.spec.ts:546-551 clicks the canvas and proves that ID, label, and X/Y/Z were populated. It does not enter viewport-create-node-provenance.
2. apps/desktop/src/features/viewport/PipeViewport.tsx:747-760 routes that click through buildDraftNodeFromViewportPoint, whose returned draft explicitly sets provenance to an empty string at :1701-1716.
3. apps/desktop/src/features/viewport/routeDraft.ts:91-105,137-149 makes buildNodeCreationSubmission(...).ok false when provenance is blank (Enter node provenance).
4. PipeViewport.tsx:225-241 derives nodeDraftValid from that builder, and :1080-1085 disables Add while it is false.
5. Both captured Playwright projects resolved the same button as disabled 24 times over 10 seconds. That rules out a transient render wait. The empty field and the validation rule fully explain the observed state.

This is the intended product behavior. The current App test at apps/desktop/src/App.test.tsx:16045-16058 expressly requires blank initial provenance and a disabled Add control, while the accepted node flow at :16262-16280 enters provenance before Add. App.tsx:1954-1966 only receives a submission after the viewport gate passes; nothing in its Add/Apply wiring can make an invalid draft ready.

The six-path G0 oracle act did not introduce this behavior. Its patch changes only the deformation literal in r2-smoke.spec.ts (4.927109 to 4.927112); the failed readiness sequence was otherwise unchanged.

## Active-consumer scan

- apps/desktop/e2e/linear-authoring.spec.ts:34-40 already enters explicit node provenance before Add.
- In r2-smoke.spec.ts, the canvas-derived draft at :546-552 lacks provenance. Its later Add at :780 consumes that same prepared draft, so the first candidate insertion repairs both readiness and the eventual engine-route receipt.
- The shared fillNodeDraft helper at r2-smoke.spec.ts:1665-1672 omits provenance. The active from-blank journey calls it before Add at :901-912. Both consumed rehearsal payloads contain the nonempty value invented_a12_rehearsal_user_input, so adding the missing fill repairs the helper without inventing data.
- No other active browser spec uses queue-explicit-node-intent.

## Minimal correction

CANDIDATE_TEST_CORRECTION.diff is a one-file, unapplied candidate. It:

- asserts Add is disabled after the canvas has supplied geometry but before provenance is entered;
- fills explicit synthetic_ui_acceptance_input provenance and retains the existing enabled assertion;
- makes fillNodeDraft enter the producer-supplied payload.provenance.

This keeps the product prerequisite and strengthens the smoke test rather than bypassing validation.

## Required evidence after authorization

Have an independent reviewer verify the one-file patch and its pre/post binding. After applying the reviewed patch, run both affected active journeys in both configured projects:

    PLAYWRIGHT_WORKERS=1 ../../node_modules/.bin/playwright test e2e/r2-smoke.spec.ts --grep "R2 desktop preview smoke covers solve, results, report, and viewport overlay|R2 from-blank GUI journey authors the A12 rehearsal script" --workers=1

The run should prove the pre-provenance disabled assertion, post-provenance enabled assertion, the later engine-route receipt, and both from-blank node commits. Downstream registered browser/full-sweep checks remain the owning release evidence.

No browser, build, or test command was run during this diagnosis. The candidate was written only under the RU evidence root and is not applied to the live test.
