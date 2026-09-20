# B3 CI tooltip repair — bounded checks

Manager basis: reviewed PR002dff0f; ROOT owns CI strategy and final clean sweep.
No full source/dist repetition in this lane. All browser commands use run lock,
PLAYWRIGHT_WORKERS=1 and the installed Playwright Chromium executable override,
with actual version/path retained. Source/binary stays frozen during each run.

1. Implementer deterministic regression in b3-accessibility.spec.ts: Agent focused,
   pointer over visible reason, ordinary Close click. Retain original120s failure and
   center-hit evidence before product fix; repair must pass without cursor bypass.
   Add Escape consumption and fresh pointer/focus re-entry checks as applicable.
2. Focused source: complete b3-accessibility.spec.ts and workspace-layout.spec.ts in
   both configured profiles, plus gui-workflow-validation shared-drawer menu-overlap
   cases. This covers reason/Close, page focus/inert behavior, shell routing geometry,
   split/drawer reachability and shared Escape/Close priorities.
3. Focused dist: preflight production compact command groups; rail-clearance; outer
   overlays; below1280 focus-restoring slide-over; production appearance matrix.
   These exercise packaged styles, theme/density/window cases and adjacent layers.
   Keep exact grep/names/counts/durations in command evidence after final selection.
4. TypeScript/build and focused App.shell + workspaceSession.shell unit suites;
   any added tooltip-specific unit test. No Rust tests for unchanged Rust bytes.
5. Rebuilt actual Tauri: explicit fresh launch/source/binary/PID identity, known
   disposable or initial synthetic project without model mutations. Open a shell
   page, focus/hover Agent reason, ordinary pointer Close; re-open and Escape reason
   dismissal first, page still open, then intended page close. Verify tooltip visible
   and readable on hover/focus, no click interception, focus return. CUA only, no
   browser native-class substitution or injected diagnostics. Record full AX and
   screenshots/action ledger. Stop only owned app after ROOT handoff/release.

ROOT reserves resource transfers. Current worker owns browser only; manager requests
native after browser release. Independent review and final clean sweep/CI remain ROOT
work; native unavailable means a gate stays open, not a browser pass substitute.
