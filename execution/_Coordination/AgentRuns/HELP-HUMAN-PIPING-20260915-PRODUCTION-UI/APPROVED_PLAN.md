PLEASE IMPLEMENT THIS PLAN:
# Agent 0 execution plan: professional modelling workspace foundation

## 1. Outcome and basis

Build the first production UI tranche from the accepted interface handoff, starting from verified `origin/main@28ad73cc`, following merged PR #788.

The agreed target is a **professional modelling workspace with multi-selection, box selection, hide/isolate, measurement and a responsive 10,000-pipe viewport and model tree**. Ship light and dark appearances, following the operating system by default.

Professional quality means precise entry, predictable selection, discoverable tools, clear units and targets, dependable history, and responsive navigation. Familiar piping-software conventions provide useful precedent; production task testing will determine whether this implementation meets that target. [AutoPIPE interaction documentation](https://bentleysystems.service-now.com/community?id=kb_article&sysparm_article=KB0116332)

Preserve the completed engineering and compatibility repairs. Pressure activation, connector mechanics, new modelling operations, CAEPIPE export and harness integration remain separate. UI development can proceed without completing that backlog.

## 2. Production workspace and interaction design

**Workspace composition**

- Make the 3D viewport dominant, with a searchable, collapsible model tree on the left, contextual properties and active-task tabs on the right, and a resizable review/results dock below.
- Use a compact command band grouped by the existing capability catalogue: Build, Supports, Properties, Loads, Edit, Select/View and Review. Provide command search, visible shortcuts and specific disabled reasons. Keep technical evidence accessible through Details.
- Use restrained neutral surfaces, consistent typography and spacing, tabular numeric values, labelled icons and distinct selection, focus and diagnostic states. Provide Comfortable and Compact densities.
- At widths of at least 1280 px, show both side rails. Below that, use independently accessible drawers while preserving the canvas and task action footer. Pending review opens its dock visibly.
- Provide System, Light and Dark theme settings. Persist theme, density and panel dimensions in a small versioned UI preference record; do not change project storage.

**Modelling controls**

Keep camera presets, Fit Visible, Fit Selection, orbit/pan/zoom, axes, units, construction plane and routing constraints together around the viewport. Preserve left-drag orbit, right-drag pan and wheel zoom.

Properties always identify the selected entity. An active task separately identifies its frozen target and retains entered values when panels change. Selection changes cannot silently retarget a draft. Add, Cancel, Review and Apply remain explicit.

Fresh Current results retain their existing overlay and inspection behavior. Historical results remain readable with their designation and integrity findings, without acquiring current-model overlays or readiness claims.

## 3. Selection, visibility and shared interfaces

Introduce internal typed selection, viewport-state and model-index interfaces. Keep existing `EntityRef`, engineering intents, batches, Rust application, result DTOs and persistence contracts compatible.

**Selection**

- Store an ordered selection set and primary entity using collision-safe typed keys. Empty selection adapts to the existing project inspector; explicit project selection is exclusive.
- Click replaces selection, Shift adds, and Ctrl/Cmd toggles. Tree range selection follows displayed row order, with keyboard focus separate from selection.
- Add an explicit Box Select tool so ordinary orbit gestures remain unchanged. Left-to-right selects fully contained centerlines/anchors; right-to-left selects intersecting centerlines/anchors.
- Box selection operates through depth on unhidden, valid entities inside the camera frustum, regardless of occlusion. Clip segments before screen-space tests. Provide All, Pipes, Nodes, Supports and Components filters.
- Preserve the existing drag threshold and cancellation guards. Active routing and endpoint capture take precedence over general selection.
- Every membership change synchronously advances the existing preparation epoch, including changes that leave the primary unchanged. Pending preparations and reviews become stale; selection alone does not invalidate solved model results.

Multiple selection enables aggregate inspection and view commands. Disable single-target mutation controls while multiple entities are selected. Existing transform and self-weight workflows gain an explicit **Use selected pipes** action that copies a validated target snapshot; later selection changes do not rewrite it. Preserve existing reviewed grid and batch routes without adding general bulk editing.

**Visibility**

Hide and isolate are presentation masks. Hidden entities remain selected and marked in the tree.

Each entity owns its labels, highlight and deformed graphics. Hiding a node also hides its attached symbols and nodal load arrows, while connected spans remain independently visible. Pipe-owned load and deformation graphics follow the pipe.

Isolating pipes includes their endpoint nodes and attached context; isolating nodes includes their attachments; isolating a support or component includes its anchor node. This expansion does not recursively include neighbouring pipes. Show All clears both masks. Picking and fitting use the resulting visible geometry.

**Measurement and geometry display**

Provide distance and ΔX/ΔY/ΔZ between two model nodes or a pipe’s endpoints, using existing unit conversion. Measurement uses authored coordinates, never displayed tube radius or deformed results.

Keep schematic centerline as the default. Add an optional **Actual OD** display for straight spans whose geometry, section binding, inline/shared-section consistency and units are valid. Components remain schematic. Invalid or unavailable dimensions retain centerline display with an explanation. Batch and cache conversions, rejecting responses from obsolete model generations.

**State lifetime**

New/Open/import/replacement advances a distinct project-session generation, including reopening the same project ID. It clears transient selection, visibility and measurement state and fits the new model. Ordinary edits and save normalization preserve view state while pruning invalid references. Undo/redo restores the existing primary checkpoint as a singleton selection.

All human engineering actions continue through shared typed intents and the Rust applier. New view controls use named internal commands that future harness integration can consume without depending on DOM gestures.

## 4. Rendering architecture and performance contract

Retain React, Three.js and Tauri. Repair the current renderer lifecycle before adding the full workspace controls.

- Maintain one renderer and WebGL context, with persistent camera and controls. Render the orientation gizmo through the same renderer with correct viewport/scissor restoration.
- Use shared geometry/materials and spatially chunked instancing for pipes and repeated markers. Maintain typed instance-to-entity maps and test chunk bounds before detailed picking. Selection and theme changes update display data without recreating the scene. [Three.js instancing](https://threejs.org/docs/pages/InstancedMesh.html)
- Render on invalidation and while controls are damping; stop animation-frame scheduling after settling. Include routing ghosts, resize, theme, measurement, visibility, conversion and result changes in invalidation. Suspend drawing while hidden. [Three.js rendering on demand](https://threejs.org/manual/en/rendering-on-demand.html)
- Define ownership and disposal for buffers, materials, textures and controls, including context loss/restoration and unmount/remount.
- Build one read-only typed model index per model revision. Virtualize the tree, editable grid and all large target pickers, including transform and self-weight panels.
- Keep drafts outside mounted rows, keyed by session, typed entity and field. Only the focused editing row may remain pinned outside the virtual window. Queueing filtered grid edits clears only those queued drafts.
- Suspend expensive inactive-panel computations while retaining draft state. Bound viewport labels to 80, prioritizing primary selection and hover.

Freeze deterministic benchmark fixtures before changing implementation: 1,000 and 10,000 pipes, with `N+1` nodes, `N/20` supports, `N/100` components and `N/10` nodal load arrows. Record exact layouts, dense-overlap camera views, selection sizes, result-overlay fixtures, queries and conversion-cache conditions. These are UI workloads, not solver-capacity claims.

Use five production-build runs on the current M5 Max/128 GiB reference host at 1440×920, DPR capped at 2. Per run, collect 200 point selections, 40 box/filter actions and 10 seconds of orbit samples after a two-second warm-up.

| Measure | Acceptance target |
|---|---|
| Model assignment to presented viewport and responsive tree | ≤2 seconds |
| Point selection to presented feedback, p95 | ≤100 ms |
| Box selection or tree filtering to feedback, p95 | ≤200 ms |
| Orbit frame interval, p95 | ≤16.7 ms centerline; ≤33.3 ms Actual OD |
| Settled viewport | No scheduled animation frames |
| Resource lifecycle | Stable owned-resource baseline after 20 replacements and repeated unmount/remount |

Measure baseline and candidate identically. Report cold startup, file loading, JavaScript heap, native RSS and renderer resources separately. These are proposed acceptance targets, not measured performance or minimum-hardware claims. Do not relax them silently after observing results.

## 5. Agent 0 execution and acceptance

| Assignment | Role | Model / reasoning |
|---|---|---|
| Contract decisions, orchestration and combined acceptance | HELP_HUMAN, Agent 0 | `gpt-6-astra` / high |
| Coupled UI implementation ownership | WORKING_ITEMS, Type 1 | `gpt-5.6-sol` / high |
| Product implementation and focused tests | TASK, Type 2 → manager | `gpt-5.6-sol` / high |
| Independent benchmarks and native witnesses | TASK, Type 2 → Agent 0 | `gpt-5.6-sol` / high |
| Fresh complete-diff review | TASK, Type 2 → Agent 0 | `gpt-6-astra` / high |
| Scoped Git and evidence closeout | TASK, Type 2 → Agent 0 | `gpt-5.6-sol` / medium |

Use one implementation manager and one product writer because selection, rendering, panels and drafts are coupled. Benchmark fixtures and verification may proceed concurrently in disjoint scopes. Keep at most six active instances; serialize builds, browser/native witnesses and Git integration. Type 2 instances do not delegate. Record actual configuration, parentage, context hashes, scopes and enforcement limitations. After two unsuccessful repair cycles without new evidence, assign fresh Astra/xhigh diagnosis.

Execute in five stages:

1. **Revalidate and bind.** Run Step 0 and inspect the committed loop, Remaining entries and accepted handoff. Anchor implementation in PKG07, with DEL-07-01 viewport ownership, DEL-07-06 accessibility acceptance and DEL-07-09 catalogue coverage. Seal a bounded file inventory that explicitly includes affected target selectors, preparation seams and inactive-panel computation.
2. **Freeze expectations and baseline.** Record selection/visibility contracts, benchmark manifests, visual tokens and acceptance targets before product changes.
3. **Build a working vertical slice.** Establish the persistent renderer, model index and selection state, then integrate the workspace, palette, properties and review dock. Exercise real native author/edit/review/apply behavior early.
4. **Qualify the complete experience.** Test selection membership ABA, cancellation, stale drafts, hidden attachments, typed-ID collisions, invalid geometry, delayed conversions, context recovery and filtered draft retention. Exercise both themes and densities at 1024×768, 1280×800 and 1440×920, including keyboard operation and screen-reader tree semantics.
5. **Freeze, review and integrate.** Run fresh independent review over the complete integrated diff, the registered DEC-025 sweep, practitioner harness, self-check and receipt validation. Verify a clean indexed checkout, then bind required CI to the actual PR candidate and merge one scoped PR under standing authority.

Adopt WCAG 2.2 AA criteria for touched controls, including visible focus, keyboard alternatives, 4.5:1 ordinary-text contrast, 3:1 required graphical contrast and applicable minimum target sizes. Preserve the separate held independent-usability work; tranche acceptance does not close it. [Text contrast](https://www.w3.org/WAI/WCAG22/Understanding/contrast-minimum.html), [non-text contrast](https://www.w3.org/WAI/WCAG22/Understanding/non-text-contrast.html), [target sizes](https://www.w3.org/WAI/WCAG22/Understanding/target-size-minimum.html)

Repeat the native 3.2 m cantilever lifecycle: 350 N → 500 N, analytical results, exact undo/redo hashes, attachments, save, normal quit/reopen, Historical designation and fresh re-solve. Preserve equivalent human/agent-intent hashes and busy, cancellation, late-response and native-error guards.

Close with updated Remaining entries, immutable evidence and a handoff identifying measured performance, supported production workflows and remaining functionality. Preserve the original local handoff and all accepted engineering history.
