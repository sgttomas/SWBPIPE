# Select popup repair — blocked prototype return

TASK Type2, parent B3-CODEX WORKING_ITEMS, native delegated-harness descendant; assigned GPT-6 Astra/low, no delegation. Scope instructional on unrestricted host. Prototype only, NOT ready for production acceptance. Manager instructed stop after native same/changed-value checks. No alternate mechanism implemented, no Git mutations.

## Candidate and contract

Only production source files written by this child: src/features/workspace/nativeSelectPopup.ts and nativeSelectPopup.test.ts under Piping desktop. Consumer changes in production are manager-owned. Helper interface createNativeSelectPopupGuard(window, getRoot) returns isCancellation(KeyboardEvent) and dispose. Capture observers record actual :open only; WeakMap retains one event's classification for React narrow nativeEvent and global bubble handlers. Consumers retain existing defaultPrevented priority and return without preventDefault for popup cancellation. No polling, debug instrumentation or blanket select Escape guard in production.

Module SHA256 1bd63f00d14c7f041fc188f7facd0ba77b35f11cacc25c3fd0ca3bc4b1384dc3. Test SHA256 2f1eb8bc759b89e02a0bbb1142c597017ee63e916e01736293d648a80a879351. 16 unit cases pass (unit-C.log). They prove modeled delivered-event behavior; they do not prove that native same-value commits emit those modeled events. Probe build passes (build-C.log). Initial npm test from repository root failed ENOENT because package.json is in desktop; corrected explicit desktop workdir run passed.

Probe detached base f5bc01f8beaa1c210b988fe28aea95573781f963. Existing variant B passive logging and native predefined responders unchanged. variant-C.patch retains tracked diagnostic source and temporary consumer integration; variant-C-nativeSelectPopup.ts/test.ts preserve untracked new files separately. launch-C.json binds PID16215, binary/source hashes and start. Prior A/B evidence unchanged.

## Observed result and causal limit

Pointer-open first Escape preserved Inspector, second closed it and restored opener focus. Space-open first Escape visually did the same. HOWEVER raw capture in that first Space cancellation delivered only Escape keyup, not keydown; this does not causally demonstrate the guard intercepting the original pre-keydown-close failure. Do not claim that original symptom repaired from this visual pass.

Space-open then Return committing unchanged m closes popup but delivers no Return keydown/keyup, input or change to DOM. Later Escape keydown sees :open=false and is incorrectly suppressed by remembered state; Inspector remains open. Pointer-open then click already-selected m has the same failure and no DOM commit event. These histories are indistinguishable to this helper from the previously observed genuine native cancellation: observed :open=true followed later by Escape keydown :open=false. Clearing only on events cannot clear an event that does not occur. Candidate fails required same-value lifecycle and must not be integrated/frozen as successful.

Changed pointer commit m→mm emits input/change, clears memory and allows next Escape to close Inspector. Direct closed-select Escape also closes Inspector and restores focus. Raw C-select-capture-extract.txt preserves line references into runtime-C.log. Full exact CUA actions and AX evidence are in C_ACTIONS.md. No applied model operation/save; Undo/Redo remained disabled.

An attempted Tab exit from an open native menu did not leave focus; it changed highlighted menu option. Escape dismissed it, committed unit remained mm. Screenshot API returned unavailable for explicit probe path. Outside-focus case therefore NOT established. No forced narrow resize, no native minimum repeat, no neighboring control or uninstrumented production acceptance. Scope stopped after known blocker and manager instruction.

## Next decision

A new reliable event or host signal is required, or an explicitly owner-approved interaction change. CSS :open transition/animation notifications are a source-level research option only: closure notifications would also occur on genuine Escape before DOM keydown, so ordering/classification still needs proof and must not be assumed. A public native popup commit/cancel callback would be another research option; no such available callback has been established in this task. No timing expiry, poll or blanket suppression is justified by this return.

Production prototype files remain for manager disposition; do not treat passing synthetic tests as native regression coverage. Manager can remove/park helper and consumer integration while retaining these evidence copies, or obtain revised bounded diagnostic authority.

## Process and UI handoff

PID13719 variant B identity/start/binary verified before termination. Initial CUA bundle-ID selection unexpectedly auto-launched production PID15926; no UI action was sent, exact path verified and terminated. C was then explicitly launched/bound by probe app path. At stop: disposable project project:blank-local-20260920t014507z, blank unchanged model, Node armed, Inspector open, coordinate draft unit mm, native popup and devtools closed, no save/delete. Final AX captured. PID16215 identity and binary hash verified again, then terminated to freeze runtime evidence. Native UI slot RELEASED; no running probe process remains from this child. No acceptance claim.
