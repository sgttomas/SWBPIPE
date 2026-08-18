# Frozen Work Graph v7

Selection authority: Agent-0 frozen one-node graph under the human 2026-08-17 steer.

| Node | Deliverable | Objective | Reads | Writes | Return | Gate |
|---|---|---|---|---|---|---|
| N1 | DEL-09-04 | Prove an invented saved-project -> edited load data -> save/reopen -> solve/result-state workflow through representative Playwright and bind reproducible validation-manual evidence. | DEL-09-04 intake; desktop e2e/config/product fixtures; validation manual | `apps/desktop/**` excluding `src/**` absent a proved defect; `docs/validation_manual/**`; clean governed `validation/evidence/**`; DEL-09-04 status/memory/one run record; this run root | changed paths, exact checks, claim-calibrated evidence, residual disposition, blockers/reruns, requested CHANGE action | Playwright representative case; relevant Vitest; manual/evidence binding; offline/private-data fence; path containment; harness self-check + full practitioner harness; DEC-025 five-surface sweep |

Edges: `N1A -> N1` where `N1A` is the read-only execution-substrate retry
described below. Concurrency: not applicable. Integration owner: WORKING_ITEMS.
Escalation: any product-source repair activates a fresh read-only
`TASK + software-code-review` over 100% of the frozen product-source diff
before DEC-025; authority-changing or excluded work returns to Agent 0.

## Amendment v2 — execution-substrate retry

Detection: the manager provisioned the pinned Playwright runner and completed
the WASM build, but the managed shell sandbox denied Chromium's macOS Mach
rendezvous before test code began (`bootstrap_check_in ... Permission denied`).
The in-app Browser runtime also listed no available browser. This is a runner
substrate failure, not a test assertion or product failure.

`N1A` is a fresh read-only ephemeral Agent 2 retry over the frozen N1 test
candidate. It may run the focused Playwright case with existing or temporary
tooling, inspect failures, and return exact output. It may not edit, delegate,
commit, or broaden scope. N1 accepts the return only if both registered
viewports execute the actual test body and pass; a launch failure remains a
blocker.

## Amendment v3 — ruled desktop-smoke exception

After the independent retry reproduced the same pre-assertion Chromium launch
failure, Agent 0 routed the existing Computer Use desktop-smoke exception.
WORKING_ITEMS exercised N1's same invented-project workflow in Safari against a
loopback-only Vite server. This is supporting manual/computer-use smoke, not a
replacement for the required Playwright pass. The N1 acceptance gate and node
identity are unchanged.

## Amendment v4 — host assertion execution and case sequencing repair

A human host rerun launched both registered Chromium projects. Every assertion
through the initial solved-state checks passed, then both projects timed out at
the `queue-editor-intent` click because the still-open Solve dock intercepted
pointer events through `modeling-workspace` / `viewport-scale-bar`. This
classifies as test case sequencing: the case deliberately opened Solve and did
not restore the authoring layout, while the product exposes
`workspace-dock-close` and the equivalent visible Safari workflow completed.

N1 is amended in place to close the active dock through that visible control
and assert the dock is collapsed and the Solve section hidden before selecting
and editing the load. No bypass, timeout, product source, CSS, or assertion
change is authorized. Host rerun remains required.

## Amendment v5 — product layout repair and mandatory code review

Agent 0's escalated host run at instruction commit
`32360f2eb53936a526a98a41b0a571c7af287483` launched both Chromium projects
after installing pinned `wasm-bindgen-cli 0.2.123`. Both still timed out at the
enabled `queue-editor-intent` click with Solve closed. Trace evidence showed
the viewport workspace and its command/scale descendants intercepting pointer
events while the expanded inspector rendered in a zero-width grid track.

The corrected classification is a product layout defect. When both rails are
expanded, CSS hides `.workspace-pane-agent` but retained a zero-width third
track. Grid auto-placement therefore put the fourth DOM child, the inspector,
in that zero-width track and left the intended inspector track empty. N1 now
overrides the exact both-expanded selector with a three-column
tree/viewport/inspector template. Collapsed-rail selectors retain the original
four-track layout. The e2e case also asserts the inspector track is wider than
300 px before authoring.

`CODE-REVIEW-01 -> N1` is a fresh mandatory read-only `TASK +
software-code-review` review over 100% of the frozen product/test diff. N1 may
return to Agent 0 for host rerun only after the review reports PASS with no
blocking findings.

## Amendment v6 — review finding repair and second frozen review

`CODE-REVIEW-01` returned FAIL on one blocking e2e sequencing finding: the case
opened the fixed-position Issues drawer for the required diagnostic but did not
close it before opening the co-located Audit drawer. The CSS product repair was
accepted as coherent. N1 now closes Issues through its visible Close button,
asserts the drawer is removed, opens Audit, and asserts Audit is visible before
checking local network/telemetry evidence.

`CODE-REVIEW-02 -> N1` is a new fresh read-only `TASK +
software-code-review` review over the complete amended two-path diff. It
supersedes the failed review for fan-in; both returns remain durable.

## Amendment v7 — focused host acceptance

Agent 0 executed the exact frozen focused command under capability `host` at
HEAD `32360f2eb53936a526a98a41b0a571c7af287483`. The reviewed product/test
hashes remained `476afc6...1dc` and `2ae72d6...bb2`. Both registered projects
passed: `chromium-desktop` in 11.3 seconds and `chromium-compact` in 10.6
seconds; total result 2 passed in 22.9 seconds, exit 0.

N1 is accepted for CHANGE fan-in. The DEL-09-04 residual remains unchanged in
this tranche. The commit-bound full DEC-025 sweep must run at the clean N1
commit before the merge gate can be claimed.
