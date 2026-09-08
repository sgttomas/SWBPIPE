# U7 implementation brief — native straight authoring

Date: 2026-09-08
State: concrete release candidate; product writes remain held pending root amendment and formal dependency disposition

## Objective

Implement the adopted U-A first release in the existing Tauri/React/Three desktop workspace: create an explicit node, create a straight segment to an existing or explicitly entered new endpoint, complete the invented one-case model through existing forms, solve it through the existing native mechanics route, save/reopen it, revise one selected force through an inline inspector Apply action, and undo/redo that revision.

The persistent Three canvas remains visible. The build surface remains compact and icon-led. Existing material, section, support, load-case, solve, save/open, operation-service, and result surfaces are reused.

## Exact source fence

Product implementation may modify only:

- `apps/desktop/src/App.tsx`
- `apps/desktop/src/App.test.tsx`
- `apps/desktop/src/features/viewport/PipeViewport.tsx`
- `apps/desktop/src/features/viewport/routeDraft.ts` (new)
- `apps/desktop/src/features/viewport/routeDraft.test.ts` (new)
- `apps/desktop/src/features/model-tree/PropertyInspector.tsx`
- `apps/desktop/src/features/model-tree/typedInspector.test.tsx`
- `apps/desktop/src/styles.css`
- `apps/desktop/e2e/linear-authoring.spec.ts` (new)

The exact fence excludes `types.ts`, `WorkspaceToolbar.tsx`, all operation/project services, all Rust and Tauri source, and the existing material/section/support/load forms. Any discovered need outside the fence stops implementation and returns a specific amendment request; it must not be expanded opportunistically.

## Interaction and state contract

### Straight route

The straight-route tool has a start node and exactly one end mode:

- existing endpoint: choose or pick an existing node ID;
- new endpoint: type the end node ID, label, X/Y/Z, coordinate unit, and provenance, with optional pointer values only as a draft aid.

The shown construction plane is named `XZ @ Y=0`. This exposes the plane already implemented by `raycastDraftPoint` in `PipeViewport.tsx:1487-1500`; it does not add arbitrary-plane selection. Typed coordinates remain authoritative and may contain zero. Pointer placement must never manufacture an omitted coordinate or semantic default.

The route card also shows the new pipe ID/label, selected existing material ID, inline OD, wall, length unit, y-reference, and provenance. It shows the exact operation members and endpoint IDs before acceptance.

`routeDraft.ts` is a pure builder/validator. It reserves IDs across the current model and pending operations. Its outputs are:

- existing-to-existing: one `connect_pipe_run` intent;
- existing-to-new: one batch whose exact order is `[create_node, connect_pipe_run]`.

It never emits `create_material`, `create_section`, `assign_section`, support, load, solve, component, or rule operations. The new-endpoint batch is submitted once through the existing App batch service and yields one checkpoint only after a complete `committed_as_one_batch` receipt.

### Add and Apply

Creation uses an explicit two-stage contract:

1. `Add` freezes the current revision, model hash, route draft, exact intent/batch, and displays the service-generated validation/diff.
2. `Apply` submits only that frozen intent/batch. It is enabled only for a passed, current validation result matching the displayed operation IDs and current revision/hash.

Any edit after Add, selection change affecting the draft, model commit, open/create, undo/redo, or Cancel invalidates the frozen review. Apply then remains disabled until Add is run again. While Add or Apply is in flight the controls are disabled. Repeated activation cannot submit a duplicate. A warning, changed diff, blocked member, missing receipt, stale callback, or hash/revision mismatch publishes no model and requires a fresh deliberate action.

Node-only creation follows the same explicit Add/Apply pattern and uses the existing single-operation service. It remains valid to save/reopen that incomplete model; solve readiness remains explicitly incomplete.

### Inspector revision

The current `Queue change` and `Validate` controls remain. `PropertyInspector` receives `onApplyIntent`; inline `Apply` sends the exact currently displayed intent to existing `App.handleApplyIntent`. It does not mutate the model or reinterpret values locally. The control is enabled only when the current intent is complete and operation work is not busy; an optional current validation outcome may be displayed, but App/service validation remains authoritative at Apply.

The revision scenario changes only `load:UI-A-FY` magnitude from `350 N` to `500 N`. It uses one existing single-operation checkpoint, then existing undo and redo. The old result is cleared by the existing commit path before a new solve. Queue remains available for users choosing the review dock.

## Stale and cancellation invariants

- Draft editing or Cancel increments a route-review generation and discards its validation outcome.
- App's model revision, request sequence, epoch, and model hash remain the authority for all async completion.
- A canceled/inactive route cannot publish when a delayed callback returns.
- Opening or creating a project invalidates every pending route/inspector callback and does not create an undo checkpoint from the stale result.
- Successful new-endpoint route application creates one undo checkpoint for both members; failure creates none.
- Continuation begins only from a successful committed end node. Cancel clears continuation and all transient end inputs.

## Invented complete input card

All values are synthetic, explicitly entered, and displayed before Apply.

| Item | Exact input |
|---|---|
| Project units | length `m`; force `N`; angle `rad`; pressure/stress storage `Pa`; temperature `degC` |
| Material | ID `material:ui-phase-a-invented`; label `Invented carbon steel`; E `200000000000 Pa`; G `77000000000 Pa`; provenance `synthetic_ui_acceptance_input` |
| Section | ID `section:ui-phase-a-straight`; name `Invented straight pipe`; type `pipe`; OD `0.168 m`; wall `0.007 m`; provenance `synthetic_ui_acceptance_input` |
| Start node | ID `node:UI-A-100`; label `Anchor`; `(0, 0, 0) m`; provenance `synthetic_ui_acceptance_input` |
| End node | ID `node:UI-A-110`; label `Loaded end`; `(3.2, 0, 0) m`; provenance `synthetic_ui_acceptance_input` |
| Pipe | ID `pipe:UI-A-100`; label `Straight run`; from `node:UI-A-100`; to `node:UI-A-110`; inline OD/wall `0.168/0.007 m`; material `material:ui-phase-a-invented`; y-reference `(0, 0, 1)`; provenance `synthetic_ui_acceptance_input` |
| Named section assignment | Existing `assign_section` of `section:ui-phase-a-straight` to `pipe:UI-A-100`, performed separately after the straight-route batch |
| Support | ID `support:UI-A-100`; label `Anchor support`; node `node:UI-A-100`; restraints `UX, UY, UZ, RX, RY, RZ`; provenance `synthetic_ui_acceptance_input` |
| Load case | ID `load:UI-A`; label `Invented Phase A force`; kind `primitive_user_load`; status `preview_only`; provenance `synthetic_ui_acceptance_input` |
| Primitive load | ID `load:UI-A-FY`; category `concentrated_force`; target node `node:UI-A-110`; direction `global_y`; magnitude `350 N`; dimension `force`; provenance `synthetic_ui_acceptance_input` |
| Revision | Same primitive load, magnitude `500 N`, entered through selected inspector field |

The input card contains no combination, pressure, thermal, weight, spring, nonlinear behavior, zero-coordinate prohibition, section prerequisite for node creation, or global completeness rule.

## Walkthrough and acceptance sequence

1. Create a blank local project and record its initial model hash and incomplete analysis status.
2. Enter and Add/Apply `node:UI-A-100`; save; reopen; prove the node persists and the model remains explicitly incomplete. This is the node-only regression witness.
3. Create the material and section through existing forms.
4. Select `node:UI-A-100`; enter new endpoint `node:UI-A-110` and pipe card; Add to validate the exact ordered two-member batch; Apply once.
5. Prove both records appeared together, one receipt and one undo checkpoint exist, and no intermediate node-only state was published by that batch.
6. Assign the named section through existing `assign_section`; add the six-DOF anchor support; add `load:UI-A` and `load:UI-A-FY` through existing forms.
7. Inspect readiness. Missing-input diagnostics, if any, must be explicit; the UI must not invent a default or a new global gate.
8. Run the whole current model through the existing native mechanics path. Record actual Tauri route, job ID, run ID, submitted model hash, result model hash, backend hash evidence, terminal status, and visible result provenance.
9. Save and reopen; compare authored IDs, quantities, units, provenance, and the saved/reopened model hash.
10. Select `load:UI-A-FY`; edit `350 N` to `500 N`; inline Apply once; prove prior results clear; run again and record a separate job/run/hash set.
11. Undo once and prove `350 N`; redo once and prove `500 N`; save/reopen and prove the final selected value.
12. Cancel a prepared new-endpoint route and inject/delay its completion in tests; prove it cannot publish. Repeat Apply activation while busy; prove one application only.

Native job/run/hash evidence and the walkthrough record are separate from browser Playwright evidence. Browser fallback must not be labeled native.

## Meaningful test fence

- `routeDraft.test.ts`: exact intent payloads and IDs; existing endpoint one-member route; new endpoint exact two-member order; ID collision; missing material; invalid inline OD/wall/y-reference; zero coordinate accepted; typed-coordinate authority; draft invalidation.
- `typedInspector.test.tsx`: the exact selected intent reaches `onApplyIntent` once; disabled/busy/incomplete controls; Queue and Validate remain; text/unit/provenance display unchanged.
- `App.test.tsx`: two-member atomic Add/Apply produces one checkpoint; rollback publishes neither member; stale/canceled callback publishes nothing; double activation cannot duplicate; node-only create/save/reopen remains incomplete; inline force Apply clears results and supports undo/redo.
- `e2e/linear-authoring.spec.ts`: persistent canvas and compact palette; blank-to-complete invented card; existing/new endpoint interaction; Add/Apply diff; section/support/load reuse; browser solve/save/reopen/revision/undo/redo; explicit route label proves whether WASM/browser fallback was used.
- Native walkthrough record: packaged Tauri execution with actual native route plus job/run/model hashes for initial and revised solves.

Required registered checks after implementation are `harness-self-check`, `desktop-test`, `desktop-build`, and `harness-pytest` because the diff touches `apps/desktop/**` and `execution/**`. Run targeted tests first, then the registered checks once. Do not start a heavy Rust compile for this source fence.

After tests pass, obtain a fresh `software-code-review` over 100% of the implementation diff. H4/DEC-025 and all root/Owner release gates remain required before acceptance. No permanent baseline test is added solely for ceremony.

## Explicit exclusions

Phase B typed case viewing; selective solve; model slicing; bend/branch/rigid/expansion-joint creation; arbitrary construction planes; coincident-coordinate geometric inference; perpendicular-frame inference; new canonical or bridge contracts; new persistence or Rust operations; and any claim that all legacy dependency rows are already satisfied.
