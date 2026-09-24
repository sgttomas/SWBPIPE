# Clean-sweep diagnosis — two independent failures

Diagnosis completed without product/test/setup writes. Source hashes are in `DIAGNOSIS_SOURCES.json`. The c57 clean sweep is authoritative for the broad failure; later ROOT upstream-only document integration does not change these Piping source bytes. No Git mutation, server/browser/native process, CUA or native bundle action occurred. The selected software-defect-diagnosis skill and full TASK/Root/project instructions are recorded in `PROVENANCE.json`.

## Q1: compact queue feedback marker omitted

The unchanged App test `queues layout-grid cell edits as structured review intents` reproduces alone: exit 1, one failed test, 217 unselected tests, no popover unhandled errors (`_run_records/queue-reproduction.log`). Repeating with a larger diagnostic DOM limit confirms `data-compact-grid=true`, and the actual `span.compact-queued-message[role=status]` contains exactly `Queued 2 review intents from Grid mode.` The same complete message is also present in Details. Neither compact representation has `data-testid=entity-grid-queued-message`; that marker exists only on the conditional noncompact paragraph in ModelTree. Exact DOM is preserved in `_run_records/queue-full-dom.log`.

Cause, high confidence: a presentation branch changed the maintained test-facing status identity, while genuine compact feedback text/role exists. This is not evidence that queueing failed. The later unchanged App assertions for both structured intent IDs and unit payload were not reached because the marker assertion failed first; they must remain and run after repair.

Minimal proposed repair: put the existing canonical queued-message marker on the compact status span. Retain status text/role and the noncompact marker, whose branch is mutually exclusive. Do not remove, loosen or retarget the two operation/payload assertions to bypass feedback.

## P1: unsupported jsdom native-popover query during ordinary cleanup

The unchanged project-handler test `records both snapshot observations while retaining the newer edit after Save` reproduces independently: its assertion passes, but Vitest exits 1 with exactly two unhandled errors, one from EngineeringTable.closeInfo and one from ModelTree.closeCompactDetails (`_run_records/popover-reproduction.log`). That case edits/queues/applies a grid value during a pending Save; ordinary focus transitions invoke cleanup on mounted compact disclosure elements even though they were never opened. Both cleanup functions call `matches(":popover-open")` before checking whether a native hide API is available.

The standalone probe records jsdom's `showPopover` and `hidePopover` as undefined, while `matches(":popover-open")` throws SyntaxError (`_run_records/jsdom-popover-probe.json`). Test setup only prewarms WASM; it does not provide a native-popover adapter. The existing narrow EngineeringTable test passes with its per-element closed-popover selector spy (`_run_records/local-shim-control.log`), demonstrating why that focused component result did not cover App integration.

Cause, high confidence: the test environment lacks the native feature, and unconditional closed-disclosure cleanup evaluates an unsupported selector. This is separate from Q1. It does not establish a failure of supported Chromium or native WebKit popovers. A passing assertion with unhandled errors is still a failed check. Existing native/browser support evidence remains separately bound to its actual host and candidate.

Minimal proposed repair: in both cleanup functions, return false if the target lacks a callable `hidePopover`, before querying `:popover-open`. When the method exists, retain the native state query, hide call and current focus/event ordering. Do not catch arbitrary selector errors or claim a fallback disclosure works. This is a narrow cleanup capability boundary: an environment with no native popover has nothing native to dismiss. It also leaves unexpected failures observable when the native API is present.

Alternative: a tested jsdom presentation adapter could emulate the full native feature, or native lifecycle state could replace the predicate. Both introduce substantially more state/timing/test surface than this closed-cleanup failure needs. A blanket global matches override, an always-false selector patch, or suppressing unhandled errors would mask real behavior and is not proposed.

## Proposed proof and scope

Proposed maintained files: ModelTree.tsx, EngineeringTable.tsx and EngineeringTable.test.tsx, with App assertions preserved. Remove the prior closed-popover spy from the long-feedback test; explicitly assert absent-API cleanup does not evaluate the unsupported selector. Add narrow supported-API closed/open branch assertions with per-element doubles, then backcheck actual native-popover paths with existing real-browser Details/Info cases. No global setup shim is required.

After manager activation: rerun unchanged Q1, the five project-handler cases corresponding to the ten clean-sweep errors, focused component tests and TypeScript; run the relevant existing Details/Info Chromium journeys. ROOT retains clean full-sweep and native authority. A supported-API test double is only a branch-level component check, never a native-support claim. If another dependency is required, return it before broadening writes.

No source correction is activated by this diagnosis itself. Await manager's explicit bounded repair activation. All launched focused Vitest processes completed; no port was acquired.
