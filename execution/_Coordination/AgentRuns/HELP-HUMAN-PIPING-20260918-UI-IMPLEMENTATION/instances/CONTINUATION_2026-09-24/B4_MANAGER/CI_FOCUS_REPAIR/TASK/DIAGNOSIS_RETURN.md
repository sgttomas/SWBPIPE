# CI endpoint-focus diagnosis

Diagnosis completed on frozen product 01063c7f1cfb7a651261401f62a5a72a5daa94dc before repair writes. The full local DEC-025 pass remains valid local evidence; hosted run 35958768464 remains blocking with six classic-case failures. This return neither converts CI failures to passes nor attributes unrecorded events to the hosted trace.

## Recorded hosted facts

The representative byte-preserved trace identifies Linux HeadlessChrome 148.0.7778.96. The Earlier columns click is action call@80, resolved visible/enabled and performed at x854/y664. Its after-snapshot shows Earlier disabled, Later enabled and translateX(0px). The next evaluation (call@82) returns null for activeElement's aria-label. The trace does not record focus/blur event ordering or active-element tag identity. `CI_TRACE_EXTRACT.json` retains these exact bounded facts; the original trace/context and all-six failure log are preserved with SHA-256 references in PROVENANCE.

## Direct event reproduction and cause

The approved sequential probe runs the same product interaction and a minimal native button on two available macOS engines, without changing source. Installed Chrome 153.0.8010.54 passes; bundled Chromium 148.0.7778.96 reproduces the hosted null-label symptom. It is therefore not necessary to posit a Linux-only cause.

In bundled 148, the Earlier click first focuses the enabled button. During setAttribute(disabled), native blur/focusout occur synchronously, activeElement becomes BODY, and only then setAttribute returns. The layout effect consequently sees BODY, so its `pair.contains(document.activeElement)` guard is false and cannot recover the enabled sibling. The final active element remains BODY, while the draft value and disabled Undo are preserved. The status rail has the same vulnerability and already leaves BODY before the column action.

In installed 153, the disabled-after observation still identifies the disabled Earlier button as active. The existing layout effect can then focus Later. The minimal button reproduces the same immediate-disable distinction independently of React/table geometry. Confidence is high: source ordering and actual instrumented native focus events account for both observed outcomes. The hosted event timing is inferred from this matching-engine reproduction, not claimed to have been recorded on Linux.

The first probe records complete event descriptors and draft/history observations. A second attempt requesting Browser.getBrowserCommandLine failed because enable-automation was not set; that failure is retained. The follow-up tolerates only that unavailable diagnostic field and records actual executable launch logs plus active-element outerHTML. No product exception or focus behavior is suppressed.

## Minimal proposed repair and proof

Capture the initiating focused pan button, pair and owner identity before offset/scroll mutation. In the post-commit effect, retain the normal existing focused-disabled guard, plus recover the captured connected/visible still-active owner if it became disabled and focus is BODY or still that button. Clear the one-shot request. Refuse recovery after disconnection, hidden/inactive/changed pair ownership, or when another live control owns focus. Keep native disabled semantics, no timers/global listeners and unchanged six CI assertions.

Apply this to EngineeringTable column pan and OverflowRail tool/status pan. Focused component tests may reproduce synchronous blur-on-disable as an explicitly synthetic fixture and must cover intentional outside focus/no-steal. Real bundled-148 browser tests are decisive backchecks, with installed-153 regression, pointer/keyboard endpoints, unchanged draft/history and tool/status behavior. No styles, controllers, VirtualList, CI policy or native application changes are needed.

The manager activated this exact bounded repair after the causal return. Native bundle/CUA/PID9925 were untouched. Port5174 is reserved for this lane; the diagnostic browser processes close after each engine. ROOT retains Git, independent review, hosted rerun and native gates.
