# C3 manager brief — preparation draft, not activation

ROOT preparation for retained WORKING_ITEMS /root/i1_manager, Astra/high. Activate
only after ROOT verifies the combined B3A/I1 merge and supplies its exact revision.
Preserve the lane and all I1 evidence. No child or implementation starts from this
draft. Current source-grounded preparation is _run_records/C3_PREPARATION_RETURN.md.

Adopted design scope is G-30: Isolate dims all valid drawable entities outside the
selected-key snapshot to 20 percent opacity; Hide still removes. Total hidden
count is independent of current selection and excludes dimmed, invalid, offscreen,
occluded or label-budget-suppressed entities. Owned submeshes follow their entity;
separately identified nodes/attachments retain separate isolation membership.
Preserve explicit Hide's existing node-to-attachment propagation. Show All clears
Hide and isolation. No engineering model/history/result mutation is introduced.

ROOT's proposed implementation disposition, under UX_SPEC_V1 question 27's explicit
implementation discretion: dimmed geometry remains pickable through unchanged
click/hover/box geometry and ordering; table selection remains available. A dimmed
nearer entity can win over a bright farther entity. Selecting a dimmed entity does
not silently change the isolation snapshot; its existing selection halo still
indicates selection. Explicitly hidden entities remain unpickable in the canvas.
This interpretation makes visible context usable and avoids an undocumented new
pick preference. State these semantics in help and meaningful tests.

Use one canonical session visibility projection, not a second stored viewport
mask. Proposed shell interface: explicit hidden set plus nullable isolation
selection snapshot; derive actual hidden keys, dimmed keys and hidden count from
the existing model index. Prune deleted keys; clear on project replacement; retain
across ordinary stage/view changes. The old first-profile visibility helpers and
geometric picking tests may remain compatibility-tested and byte-identical while
new pure projection code receives focused tests.

Canvas owns viewportVisibility.ts/tests, PipeViewport.tsx, viewportResource.ts and
bounded material/resource tests. Propose exact files after inventory. Shell is the
sole writer of selectionSessionState.ts, workspaceSession.ts, App.tsx, styles.css,
shared diagnostics and relevant App/source journey tests. Send an exact typed
interface/patch request through ROOT and obtain acknowledged writer handoff. B3B
may run concurrently in shell; no parallel edits to shared files. Existing legacy
Isolate-hides assertions may change only as named consequences of adopted G-30;
all picking tolerances/oracles, floors and first-profile bytes remain protected.

Prototype reversible normal/dimmed presentation within existing spatial chunks
and persistent resource ownership. Existing material opacity is available; normal/
dimmed instanced batches are a recommendation, not authority for a renderer rewrite.
Check layer replacement, theme repaint, load/result owner membership, edge/halo
ordering, empty layers, disposal and resource counts. No duplicate entity ownership
or halo. Return any foundational change before implementation. No current overlay
or readiness cue may be revived for Historical results.

Timing boundary: the preparation proposed a guidance probe, but the later owner
section-7 group 3(c) and orchestration amendment 1 retain the second-profile freeze
before performance timing runs. The earlier T1/edge and C2 exceptions are completed;
do not generalize them. C3 may implement and test behavior and resource/settling
mechanics, and prepare instrumentation, without claiming measured performance.
No new timed benchmark or profile qualification is activated here. Preserve the
full D-72 limits and carry the new rendering cost as an explicit open verification
item for the owner-frozen final candidate. Return a concrete conflict if proceeding
would require reversing any fixed criterion; silence is not approval.

Use one fresh bounded TASK Astra/low, no child delegation. Run picking checks
first/last, focused pure/resource tests and connected pointer/keyboard journeys
when operable: Isolate visibility and dimmed selection; Hide then clear selection
retains total count; Hide wins; Show All; project replacement; theme; model-hash
and history invariance; Current/Historical behavior. ROOT reserves browser/native
resources and owns independent complete-diff review, own look, clean validation,
CI and merge. Do not duplicate full lanes immediately before ROOT's sweep.

Keep controls named, keyboard/pointer usable, focused visibly, with truthful
disabled reasons. Contrast/fine spacing and final HUD regrouping remain for the
closing pass; list affected controls. C4 label budgeting/diagnostic fields, C5 HUD/
real-OD/fitted camera, owner casing/deformation choices and second profile remain
separate. No Runtime integration, DAG rebuild, deliverable reconciliation or release.
Standard F-PIP-2/DEC-081 claim fence applies.
