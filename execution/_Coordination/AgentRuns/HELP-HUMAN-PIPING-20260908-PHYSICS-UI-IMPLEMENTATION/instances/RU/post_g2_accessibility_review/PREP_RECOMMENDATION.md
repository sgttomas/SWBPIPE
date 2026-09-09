# RU G2 accessibility repair — preparation recommendation

Status: PREPARATION_COMPLETE; terminal review withheld pending U7 final source hash and evidence.

## Confirmed preimage and acceptance condition

- `PipeViewport.tsx` preimage SHA-256: `ac3789a6b4e8c452fe398d062183b25d884f55f5763af877ac6f1b3d4cb9f2ad`.
- The reviewed Apply button is disabled by the unchanged predicate `!draftReview || draftReviewBusy || !onApplyDraft` and currently exposes no accessible disabled reason.
- `App.deadControls.test.tsx` accepts a disabled control only when it has a nonempty `title`, `aria-label`, or `aria-describedby` target text. Its initial-state failure is therefore causal and specific.
- A truthful reason must cover all three existing causes: unavailable Apply callback, an Add/Apply request in flight, and absence of a frozen review. Because Add can be busy while no review exists, the busy condition must take precedence over the missing-review reason after callback availability is considered.

## Minimum final review checks

1. Reconstruct and inspect the exact U7 one-path final delta.
2. Verify the disabled predicate, click handler, callback/state transitions, rendered control text, and all model/physics/route behavior are unchanged.
3. Verify the added accessible reason is nonempty and truthful for each disabled cause, including overlapping busy-plus-no-review state.
4. Bind U7 evidence for the unchanged dead-control audit and focused App behavior checks covering enabled Apply/one checkpoint, delayed Apply invalidation, and delayed Add invalidation.
5. Require the downstream full DEC-025 rerun from G0 after acceptance; RU will not execute it.

## Native evidence disposition criterion

If the final delta is limited to inert accessible metadata on the existing Apply button, the accepted native packet remains valid historical proof of the unchanged solver/route workflow at its prior source cut. The prior bundle must not be described as freshly built from, or exact source identity for, the repaired cut. An automatic native rebuild or re-witness is not causally warranted by a source hash change alone. A native repeat would become warranted if the final delta changes the disabled predicate, handler, state transitions, conditional rendering, visible layout text, callback wiring, focus/keyboard behavior used by the workflow, model/physics/service logic, or any other bundled behavior.
