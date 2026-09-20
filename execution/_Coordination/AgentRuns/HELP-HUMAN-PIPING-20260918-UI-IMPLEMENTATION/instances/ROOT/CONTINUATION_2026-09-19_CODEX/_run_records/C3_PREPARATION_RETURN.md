C3 preparation is complete. I1 remains clean at `045acae749fe1acc95bce221cdb58c2db4a7064c`. No files changed, children launched, tests/builds run, or UI resources acquired. The preparation brief hash matches `76e20ba0ae299c7a9ead1520635aa673237d48a8f6e8585c5b9415118a9b22bc`.

I recommend implementing C3 as a presentation-state change with **dimmed geometry remaining pickable**. This follows UX_SPEC_V1 §4.6, §11 question 27, §12 items 1/9, OPERATIONS_MAP G-30 and design-system §6.6.

- Direct click and hover use the existing analytic geometry and ordering, including dimmed elements. A nearer dimmed element can therefore win over a farther bright element; no undocumented selection preference is introduced.
- Box selection includes dimmed elements, respecting the existing type filter and geometric rules.
- Table/tree selection remains available for both dimmed and explicitly hidden entities. Selecting an explicitly hidden entity does not reveal it.
- Isolate captures the selected valid entity keys when invoked. Subsequent selection changes retain that isolation until another Isolate or Show All. A newly selected dimmed entity receives the existing full-strength selection halo while its body stays dimmed. C2 already anticipates a full-strength hover halo over dimmed geometry.

The present coupling is explicit: `workspaceSession.ts:393` unions explicit Hide with the isolate complement; `App.tsx:395,711` sends that result to both tree and viewport; `viewportResource.ts:428` removes affected geometry; picking, labels, fitting and box selection consume the same hidden set. The HUD currently counts only selected hidden entities (`PipeViewport.tsx:508,2277`).

The proposed shell handoff is narrow:

| Shell-owned file | Requested change |
|---|---|
| `selectionSessionState.ts` | Replace `isolateHiddenEntityKeys` with `isolationSelectionKeys: ReadonlySet<EntityKey> \| null`; `null` means isolation inactive. Keep explicit Hide state. |
| `workspaceSession.ts` | Derive one presentation result from the existing model index, explicit Hide and isolation selection. Prune deleted keys; clear both visibility states at project replacement; retain them across ordinary view/stage changes. |
| `App.tsx` | Pass actual `hiddenKeys` to ModelTree and viewport; pass separate derived `dimmedKeys` and isolation state to the viewport. Wire Isolate to the selected-key snapshot and Show All to clearing both states. |
| `styles.css` | Add the scoped dimmed-label treatment at opacity `0.2`, preserving focus indication and existing placement. |
| Shell-owned tests | Update only identified Isolate/hidden-count presentation assertions and add session-reset/composition coverage. |

There must be no independently stored viewport mask. With valid drawable keys `V`, expanded explicit Hide `H`, and active isolation selection `I`, the derived dimmed set is `V − H − I`; otherwise it is empty. Hide wins over isolation. The hidden count is `|H ∩ V|`, independent of selection.

I recommend literal selected-entity membership for the new 20% rule. The old helper also exempts endpoint nodes and their sibling attachments as “authored context” (`viewportSelection.ts:246`); that exemption should not silently survive the adopted “everything but the selection” wording. Owned submeshes follow their entity, while separately identified nodes, supports and components retain their own membership.

Canvas implementation can remain independent after that interface is acknowledged:

- Add `viewportVisibility.ts` and focused tests for the pure projection/count.
- Update `PipeViewport.tsx` for Isolate dispatch, dimmed labels, truthful tooltips and the `n hidden · Show all` text button. Add unmodified I/H accelerators within viewport focus, sharing the pointer dispatcher and excluding editable controls; preserve shell ⌘I.
- Update `viewportResource.ts` and its tests for reversible full/dimmed presentation, theme changes, layer replacement and disposal.
- Use the existing `FigureMaterial` opacity support. Prototype normal/dimmed instanced batches **within existing spatial chunks**, rather than changing the renderer. Dimmed materials need opacity `0.2`, transparency, appropriate depth behavior and ordering before C2’s halos. Keep entity ownership disjoint between batches so halos are not duplicated.
- Keep `viewportSelection.ts`, its picking tests, `viewportSelectionPresentation.ts`, analytic primitives, geometric tolerances and first-profile instrument bytes unchanged for C3. The older visibility helpers can remain compatibility-tested while the session uses the new projection. No `modelIndex.ts` change is currently necessary.

Count and drawing semantics should be explicit:

- Hiding a node retains current attachment propagation to its support/component entities; connected pipes remain independent. Count unique hidden entity keys, not meshes.
- Load arrows and existing result graphics follow their owner’s hiding/dimming, without adding selectable entities or hidden-count entries. Preserve Current/Historical gates.
- Labels follow their entity’s opacity. Budget suppression, overlap, offscreen culling and occlusion do not mean an entity is hidden.
- Tree/table filters currently filter rows, not canvas geometry (`ModelTree.tsx:58`); they do not affect this count. A future “filter to presented rows” control requires an explicit composition input.
- Section-plane clipping is not implemented here. Future fully clipped entities must join the effective removal calculation; partially visible entities should not count as wholly hidden.
- Missing/invalid geometry stays separately diagnosed and excluded from the hidden count because Show All cannot restore it. Actual-OD centerline fallbacks remain valid displayed entities.

Verification should cover Hide → clear selection → persistent total count; Isolate without removal; dimmed click/hover/box/table selection; hidden-over-dimmed precedence; attachment ownership; Show All restoration; model-hash invariance; project replacement; both themes; and unchanged Historical-result behavior. Preserve all geometric picking tests byte-for-byte. Change the obsolete “other pipe is Hidden after Isolate” assertions in `e2e/ui-foundation.spec.ts:1337` explicitly, through the shell owner, and add material/resource opacity assertions rather than weakening visibility tests.

The batch approach adds draws and transparency work even if CPU updates occur only on state changes. A bounded guidance probe therefore belongs in the later activation; no performance result can be inferred from inspection. No benchmark or second-profile qualification is authorized by this preparation.

C4 still needs the shell’s pane/stage/view selectors, presented-row filter contract, selection-reset route and optional `labels.mode` diagnostic field; existing `labels.budget` and `renderedCount` remain. C2’s proposed `hover: { ref, changeSequence }` is a separate requested diagnostic addition, not required for this C3 handoff.

Proposed executor: one fresh TASK Astra/low, canvas paths only, no delegation. The shell manager remains sole writer of the shared paths while B3B proceeds; ROOT coordinates fan-in and independent review. Contrast, casing, silhouette shades, `canvas.edgeAlt`, final HUD regrouping, label-budget work and deformation appearance remain deferred. No new owner-held choice is needed for the recommended picking rule; second-profile freeze, casing and deformation decisions remain reserved.

