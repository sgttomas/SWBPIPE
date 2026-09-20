# C3 manager activation

ROOT activates retained WORKING_ITEMS /root/i1_manager, Astra/high, on verified
merged main 88c6c7463199d1d69caebde04678fa74facca78f. PR826 checks all passed after
the owner-UI merge; no future waiver follows. Fast-forward preserved clean wt4
from 045acae7 to this main, preserving all I1 work and evidence. Seal and launch
one fresh Astra/low TASK. Current preparation is _run_records/C3_PREPARATION_RETURN.md.

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

## Current coordination and delivery

Both managers acknowledged C3_SHARED_INTERFACE_DRAFT.md (SHA256 34815f05fe750db1ae36af3dff30a117d3e74d3ffde6522d32f64fb60671cea9). Use ViewportVisibilityProjection and deriveViewportVisibility as agreed, onIsolationSelectionChange for the snapshot and existing onClearVisibility for Show All. First freeze the small pure helper/type checkpoint and send its exact identity to ROOT for shell consumption. Continue disjoint viewport/material work. Shell freezes B3B, then writes the C3 bridge; receive it only at a clean source boundary before claiming connected behavior.

A new e2e/c3-viewport-visibility.spec.ts is in canvas scope for meaningful connected journeys; first-profile e2e/ui-foundation/** stays unchanged. The existing root-level ui-foundation.spec.ts and workspace-layout.spec.ts temporarily belong to CI economy work. Any later Isolate expectation amendment goes through ROOT to the shell writer after CI handback. Do not write App.tsx, workspaceSession.ts, selectionSessionState.ts, styles.css or shared diagnostics.

Owner approved effective CI/cadence reductions: focus while developing; one planned coherent B3B/C3 integration batch (plus CI economy if ready), one independent complete-batch review and one complete local sweep. No full validation per internal helper commit or evidence-only update. Preserve one concise canonical return/evidence set, meaningful failures and source/command bindings without redundant source copies or repeated manifests. No performance timing before owner profile freeze; mechanical resource and behavior checks remain.

Evidence only RUN/instances/C3-CODEX. Parent ROOT owns publication, graph, cross-scope contracts/resources. You may commit scoped work in wt4; no push/PR/main merge. TASK does not delegate. Host unrestricted; scopes instruction-enforced. No Runtime adoption, deliverable reconciliation, DAG rebuild or acceptance.
