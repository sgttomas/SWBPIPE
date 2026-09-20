# B4.1 existing bulk-edit compatibility disposition

ROOT HELP_HUMAN engineering disposition, 2026-09-20, inside the authorized incremental B4.1 scope. This is not a new owner ruling or a waiver of any protected test. Read with B4_1_MANAGER_BRIEF.md SHA25633090ae5e7cc98589685578c16a605158c29123b5cbd2b4054b98aea4bd64446. The original brief and worker launch payload remain unchanged.

## Evidence and decision

The adopted design-run UX_SPEC_V1.md §2.4 lines76–91 requires direct typed-cell engine apply on Apply, focus leaving or the named keyboard accelerators. §3.1 requires focus/edit states and a visible Apply/Cancel footer. Existing ModelTree.test.tsx:47–84 and e2e/ui-foundation.spec.ts:349–395 deliberately test a distinct bulk-review workflow: multiple retained positive drafts, Tree/family round-trips, and a filtered Queue action that clears only the visible draft. DisplayIntegration.test.tsx:24–46 also protects exact typed text across display-unit changes without mutating model/hash. ROOT inspected these source records at mergedfd195cf4; the manager supplied the complete seven-consumer impact inventory.

Authorize the proposed temporary coexistence for this internal B4.1 increment: primary “Node coordinates” EngineeringTable with immutable ID and entered-unit XYZ, plus a sibling collapsed native details disclosure “Review multiple changes” retaining the existing full Node bulk grid and Queue/Clear route. Existing labels and provenance remain available there. Other entity families retain their current route. Extract/reuse one intent builder within the already owned table adapter; no duplicate operation semantics or new model store.

This preserves a real existing capability while the primary table adopts the selected direct-edit behavior. It is a disclosed incremental departure from the final one-table anatomy, not the final B4/MVP interface. Record a retirement task in B4: migrate the remaining label/provenance and multi-change review capability into the common table/review grammar, then remove the duplicate grid only after equivalent behavioral coverage and independent review. Do not retain the duplicate surface merely to satisfy historical selectors.

## Conditions

- Keep existing bulk drafts in their lifted owner. Collapsing the disclosure or changing Tree/family views must not erase or automatically queue them. Show a concise retained-draft count when the disclosure is closed if drafts exist.
- Primary direct-edit focus-leave follows the adopted rule. Sequence a valid blur apply explicitly before navigation; preserve an invalid/rejected draft and its original typed target/before value/unit/generation for correction or Cancel. Disclosure transitions themselves never submit bulk drafts.
- Mounted inactive surfaces must be hidden/inert and excluded from Tab/accessibility; no duplicate active editor IDs or listeners. Test re-entry/focus recovery and actual filtered Queue behavior.
- Capture direct-edit identity, original before value, unit and generation at edit start. The preparation shorthand is corrected: the old grid draft key did not bind before value. Preserve the raw preparation; record the correction separately.
- The narrow internal outcome observer may report the owned engine outcome after existing guards. It must not be described as a live-controller publication acknowledgement or infer success from stale asynchronous state. No change to the existing boolean API, history/invalidation or reachable authoring during projectBusy.
- The seven identified legacy consumers may gain the explicit disclosure-open setup action. Preserve their numeric, target, unit, ID, model/hash, queue, history, stale-response and result-standing assertions. Prefer a small shared driver where already appropriate; do not weaken or delete an oracle to accommodate the new UI.
- The legacy-test additions are limited to ModelTree.test.tsx, DisplayIntegration.test.tsx, App.test.tsx, App.projectHandlers.test.tsx, e2e/ui-foundation.spec.ts, e2e/r2-smoke.spec.ts and e2e/b3a-session-status.spec.ts. Leave the pipe-grid dist case unchanged. Report a concrete additional consumer before expanding this set.

Proceed under the existing single shell writer. Focused new/affected unit and TypeScript checks first; request the browser/native slot once operable. No broad sweep, native/lib edits, live binding or extra PR is authorized by this disposition. ROOT will integrate/review the coherent next batch.
