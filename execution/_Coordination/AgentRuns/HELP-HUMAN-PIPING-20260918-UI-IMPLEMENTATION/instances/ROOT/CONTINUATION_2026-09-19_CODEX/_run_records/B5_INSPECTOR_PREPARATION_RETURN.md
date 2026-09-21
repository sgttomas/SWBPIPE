**B5 should start with live single-node Geometry editing in Properties, sharing B4’s operation/apply route while preserving frozen Task behavior.** The inspector already has substantial editing and presentation machinery; rebuilding it or converting every Task into a direct cell editor would discard useful guarantees.

Read-only basis: `c8f8e503383e15b42b171c4957c6f458d3697150`. Actual executor: **TASK Type2, gpt-6-astra / low**. No dirty repair bytes, writes, tests, builds, UI or delegation. Brief hash verified.

### Capability and gap map

| Area | Implemented source path | Remaining distinction |
|---|---|---|
| Live Properties | `PropertyInspector.tsx:74–106,212–214` tracks `propertiesTarget` with single selection. Rich controls select their mutation target independently from Task. | Ordinary properties are primarily readouts under “All properties”; this is not yet the specified organized row-field editor. |
| Frozen Task | `:216–232,278–314` copies the target, retains field/value/unit/rationale and guards completion with `taskGenerationRef`. | Preserve this explicit workflow. Successful old Apply must not close a newer draft. |
| Project replacement | `:195–210` resets inspector/drafts on project-session generation. | New live edits must retain equivalent invalidation, including same-ID replacement. |
| Aggregate selection | `:386–443` shows counts/kinds and disables mutation while retaining frozen/rich drafts. | UX §5.1 specifies shared-field multi-row editing; current read-only aggregate is an actual gap, not completed B5 behavior. |
| Editing | Node X/Y/Z already map to `Node/set_field/position.{x,y,z}`, with model length units (`:1605–1640`). Task supports queue/validate/apply; rich forms support creation/configuration/assignment/removal. | No new engine operation is needed for the first Geometry increment. |
| Shell/routing | `App.tsx:760–790` hosts the inspector, Both-view close button and routing portal; passes existing queue/validate/apply callbacks. | Routing already lives in the inspector. Do not repeat the historical planned move as missing work. |
| Docking/Escape | Maintained cases include “Both inspector Escape closes…” and “Model inspector stays docked…” (`b3-accessibility.spec.ts:90,103`). | Source/tests establish intended behavior, not fresh native verification. |
| B4 integration | `App.tsx:403–409` wraps `handleApplyIntent` with an owned outcome carrier; `table/modelTableAdapter.ts:28` builds typed grid intents. | Inspector can share operation semantics and acknowledgement handling without reusing table keyboard mechanics or changing Task semantics. |

The adopted UX §5.1 specifies selection-specific sections, required markers, units, provenance disclosure, same row operation as table cells, and shared-field multi-selection. It also describes Checked/origin/load-set features beyond the smallest increment. Those should not be fabricated as decorative controls.

**Important interpretation:** Task freezes its typed target and entered draft, but its field options are recomputed from the current model (`PropertyInspector.tsx:77`). “Frozen Task” must not be advertised as an immutable original-model snapshot. Existing controller before-state/stale checks remain necessary.

### First bounded increment

Add a clear **Identity + Geometry** section for a single selected node in live Properties:

- Read typed ID and label; expose X/Y/Z with explicit source units.
- Commit valid edits through the existing `handleApplyIntent` route, producing one history checkpoint, truthful errors and result invalidation.
- Preserve entered drafts on validation failure; retire them on explicit cancellation or invalidated project/target basis.
- Keep Properties responsive to selection B while Task A retains its target/value.
- Leave multi-selection read-only in this increment, with its existing explanation. Record shared-field batch editing as a later B5 subincrement.
- Reuse the existing node `set_field` support and B4’s owned outcome handling. Factor a small shared adapter if necessary; do not introduce another model setter or copy the table controller.
- Retain Task Queue/Validate/Apply and rich-form disclosures. Do not route intentionally frozen Task drafts through a live-selection cell callback.

This is a recommendation within B5 preparation, not activation or a claim that the full inspector specification is satisfied.

### Ownership and handback

The **existing shell manager remains sole writer** for:

- `src/App.tsx`
- `src/features/workspace/workspaceSession.ts`
- `src/styles.css`
- `src/features/model-tree/PropertyInspector.tsx`
- Any shared extraction from `src/features/workspace/table/modelTableAdapter.ts`

Suggested bounded additions, also assigned through that writer:

- A small inspector field adapter/component under `src/features/model-tree/`
- Focused inspector tests plus connected App/table tests

Before B5 writes, require B4.1’s frozen commit and handback documenting:

1. Final apply/outcome callback signature and admission behavior.
2. Keyboard commit/cancel/focus rules and validation feedback.
3. Shared adapter exports and supported quantity representation.
4. Any changed pane/minimum/scroll layout rules.
5. Remaining findings and independent review coverage.

Do not let a second writer “prepare” shared App/styles/session edits concurrently. No Rust/schema/native transport change appears necessary for this first increment; native work is verification of the shared route, not a new implementation lane.

### Useful connected checks

- Start Task on node A, enter a draft; select B and edit B in live Properties; A’s draft survives unchanged.
- Select several objects while A’s Task/rich draft exists; mutation remains disabled and drafts survive returning to one selection.
- Enter invalid/incompatible coordinates: model/history unchanged, entered value and diagnostic remain visible.
- Commit one supported coordinate: table, inspector and canvas agree; exactly one Undo checkpoint; prior result standing invalidated; Undo/Redo restore expected geometry.
- Replace/reopen the project during pending Apply, including same project/entity IDs; late completion must not affect the replacement.
- An older successful Apply must not close a newer edited Task.
- Both→Model→Table→Both and narrow slide-over transitions preserve appropriate drafts and restore focus.
- In the installed native app, verify coordinate application, Cmd-I/Escape responder behavior and popup Escape precedence. Browser mode cannot prove macOS responder/menu behavior.

Existing focused evidence to preserve includes `typedInspector.test.tsx:102,120,149,170,215,265,339` and `ui-foundation.spec.ts:1095,1164`.

### Design decisions and limits

No new owner decision is required merely to implement existing node-coordinate support through the shared operation route. Two boundaries need explicit treatment:

- **Aggregate editing:** adopted UX calls for it, current code deliberately disables it. A later bounded shared-field batch implementation needs its own exact field/batch semantics; do not silently remove the guard.
- **Geometry:** UX §5.1’s original “canvas shrinks/table never reflows” prose predates later B3 owner dispositions. Preserve the accepted B4/B3 layout and obtain the current layout handback rather than reinstating that historical sentence.

Casing, deformation view and second-profile freeze remain reserved elsewhere; this increment need not reopen them. No performance qualification, live-control activation or deliverable reconciliation follows.

### Consulted hashes

Root/Piping/TASK/loop instructions were verified unchanged from the previously read basis. Prefix `A/` means `projects/chirality-piping/apps/desktop/`; `R/` means the current UI implementation run.

```text
8922a669219546bc557c0718ce947b236936b0e55935d6a62389c1ac61765df6 R/WORK_GRAPH.json
2141c1e844109d4869287048c695e79677f423acae965d5dd0436b7f6aeff682 projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM/instances/UX-SPEC/UX_SPEC_V1.md
5b86c2783c5d460adf63b69dc3a49db7df672795f0feeb59079a2f93a82c0f4e R/instances/ROOT/OWNER_DIRECTION_2026-09-19_EIGHT_UX_ITEMS.md
c9049e543dd1a46b1fdcf3d7f23cd5551c89b24185c66c70d2d03eb77c2fee2e R/instances/ROOT/OWNER_DIRECTION_2026-09-19_SECTION7_DECISIONS.md
bc41f82680cfcb69a193ee3820cb0861f36fbe872fae970df1be1729f9a23268 R/instances/ROOT/OWNER_DIRECTION_2026-09-19_CONTROL_LAYER_FIRST.md
3120f29cb431fe649189a41048b95923487ff6140878dcb22bbad6b23a2d3f89 A/src/features/model-tree/PropertyInspector.tsx
12785a6391a6678fb0a48a4bf971d9142de3042c5da043556ea0c25d10958a0b A/src/features/model-tree/typedInspector.test.tsx
c3b783b943a6795cd77d88cccdce16b7cb1daf9bec230ab8de0e6210a11c5f5b A/src/App.tsx
cf0473c0cc4c201e63df4f754e38749b43860a5643c15b08eb26db81e9d04ce9 A/src/features/workspace/table/modelTableAdapter.ts
684be3d9ee42f32a3f0b2f7bf2ef8d1b690353e75a4154bf6c288773ade7992a A/e2e/b3-accessibility.spec.ts
626e790f992c28d398833d48abdf987c36823ae8d0547bb2cc233a4ecec465b6 A/e2e/ui-foundation.spec.ts
```
