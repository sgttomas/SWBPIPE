# N2 integrated independent code review — round 3, review v2

- RunID: `HELP-HUMAN-PIPING-20260821-VOCABULARY-R3`
- Role: fresh bounded read-only ephemeral Agent 2 software-code reviewer
- Task method: `software-code-review`
- Accepted basis: N1 commit `8ca1984db45a9a8f6f3111a905b07c7d3da47c33`
- Reviewed state: 100% of the refreshed complete five-product-file basis-to-current-working-blob diff after repair cycle 1
- Prior failed review: immutable `N2_REVIEW_V1.md`, SHA-256 `20c56ac4e25ff36ef38901d131a697b92f329b223721a015c65b573243ad9bc2`
- Verdict: **PASS — no actionable findings; N2 is valid for manager fan-in and commit**
- Mutation boundary: no product, Git, shared status/coverage/handoff/receipt, launch brief, return/check, or prior-review mutation; this review record is the only write.

## Findings

No actionable finding remains in the complete N2 diff. Repair cycle 1 closes
`N2-F1`; the expansion-joint path and all five accepted component-creation paths
are coherent across draft state, UI controls, structured intent, operation-applier
resolution, canonical application, and regression evidence.

## Repair-cycle-1 proof — N2-F1 closed

The repaired helper has explicit, kind-safe transition semantics:

1. **Entering `expansion_joint` clears every explicit expansion-joint input.**
   `componentDraftForKind` applies `emptyExpansionJointInputs`, which clears the
   selected pipe indirectly through the common non-bend role reset and clears all
   four unit selectors, area, movement, hardware/manufacturer/pressure-thrust
   references, all four stiffness values, both source references, and provenance
   (`componentIntent.ts:110-150`). A default or post-queue expansion-joint draft
   has the same empty state (`componentIntent.ts:52-107`).
2. **Leaving `expansion_joint` restores only the established common defaults.**
   For every accepted non-expansion kind, the helper restores exactly
   `geometrySourceReference = user_entered_component_form` and
   `provenance = user_entered_local_preview` (`componentIntent.ts:116-125`). It
   does not supply a pipe role, unit, reference, quantity, or engineering value.
3. **Bend behavior remains accepted.** Bend alone retains the established
   first-incident-pipe initialization on default, kind, node, and post-queue
   construction (`componentIntent.ts:62-69,115,154-161`). Direct Inspector and
   viewport transition regressions observe `pipe:P-100`, restore the two common
   defaults, complete the usual bend fields, queue, and apply
   (`App.test.tsx:15154-15225`).
4. **Every non-bend role stays empty until deliberate selection.** Kind and node
   changes clear primary and secondary roles for tee, reducer, valve, flange, and
   expansion joint; post-queue reconstruction does the same. The Inspector and
   viewport negative matrices cover every kind, while the transition regressions
   directly prove valve remains empty after leaving expansion-joint mode and only
   applies after `pipe:P-100` is selected (`App.test.tsx:15154-15355`).
5. **Inspector and viewport transitions apply.** Four direct repair regressions
   cover `expansion_joint -> bend` and `expansion_joint -> valve` through both
   product surfaces. Each checks restored common evidence, queue-disabled state,
   deliberate role behavior, queueing, and successful operation-applier-backed
   application. The independently rerun direct slice passed `5/5`, including the
   separate three-incident expansion-joint case.

The direct transition tests use bend and valve as representatives of the two
state families. The shared helper has no kind-specific restoration branch among
tee/reducer/valve/flange, and the affected matrix separately exercises every
accepted kind; therefore the fix is not collection-order- or representative-name
dependent.

## Expansion-joint acceptance matrix

| Requirement | Result | Independent review evidence |
|---|---|---|
| UI/applier schema parity | PASS | Intent and resolver use the same `geometry` keys (`expansion_joint_pipe_ref`, area, movement, four named references) and the same `modifiers` keys (four exact `*_stiffness_user_value` quantities plus `source_reference`). The resolver reconstructs only that canonical shape. |
| Deliberate connectivity / no inferred role | PASS | Expansion-joint default, kind, node, and post-queue states leave the pipe empty. The three-incident case exposes `P-100`, `P-110`, and `P-150`, serializes the deliberately selected `P-150`, applies it, and observes `P-150` on the realized component (`App.test.tsx:15431-15504`). |
| Explicit effective area and movement | PASS | Both values must be finite and positive, both unit selectors start empty, UI unit dimensions are area/length, and the resolver independently enforces `Area`/`Length`. Evidence preserves `0.018 m^2` and `0.045 m`. |
| Explicit four-axis stiffness | PASS | Axial/lateral require positive `LinearStiffness`; angular/torsional require positive `RotationalStiffness`. Evidence preserves `3200000 N/m`, `900000 N/m`, `480000 N*m/rad`, and `620000 N*m/rad`. |
| Explicit references and provenance | PASS | Hardware, manufacturer, pressure-thrust, geometry source, stiffness source, and common provenance all gate queueing or applier resolution, are trimmed, and survive intent/application. |
| Blocking diagnostics / no applied model | PASS | Missing geometry/stiffness input, unknown pipe, nonincident pipe, area-dimension mismatch, angular-dimension mismatch, and nonpositive torsional stiffness all block; every regression asserts no `applied_model`. |
| No invented defaults or `mechanics_interface` | PASS | Expansion-joint units, values, references, sources, provenance, and pipe initialize empty. The intent emits only identity, `geometry`, `modifiers`, and provenance; the resolver canonicalizes the same fields and never adds `mechanics_interface`. |
| Existing bend preservation | PASS | Existing first-incident behavior, explicit bend quantities, structured application, and transition-from-expansion behavior pass in both UI surfaces and the resolver slice. |
| Existing tee/reducer/valve/flange preservation | PASS | Non-bend role resets remain explicit; exact old-kind intent/apply tests pass in the affected matrix, and the resolver creation slice includes all four kinds plus tee's distinct-role negative. |
| Geometry/connectivity contract settled | PASS | Accepted product-physics types and validation name the same geometry/modifier fields, require the selected pipe to be incident, and require positive geometry/stiffness quantities. No contradictory or missing creation choice was found. |

## Complete semantic review

- The UI forms expose every required expansion-joint input in both Inspector and
  viewport, including initially blank dimensioned unit selectors and editable
  common provenance (`PropertyInspector.tsx:96-104,776-830,2337-2348`;
  `PipeViewport.tsx:208-216,1164-1223,1382-1393`).
- `isComponentDraftValid` first enforces identity, node, geometry source,
  provenance, and duplicate-intent gates, then requires an incident selected pipe,
  positive quantities, explicit units, and all expansion-joint references
  (`componentIntent.ts:164-228`). Intent construction serializes those exact
  entries and adds modifiers only for expansion joints
  (`componentIntent.ts:259-290,341-391`).
- The operation applier allowlists `expansion_joint`, validates the node, resolves
  one explicit incident pipe, validates positive area/length and four stiffness
  quantities against their exact dimensions, requires all references, and builds a
  canonical record only after all blocking checks pass (`operation_applier/src/lib.rs:1835-1927,2191-2357`).
- Application remains copy-on-write through the existing structured-operation
  branch. Unknown or nonincident connectivity and invalid dimensions cannot reach
  `applied_model`. Extra/invented payload content is not copied into the canonical
  component.
- The accepted product-physics `ComponentGeometryInput` and
  `ComponentModifierInput` definitions, expansion-joint completeness/mapping/
  positivity validation, and the existing invented preview fixture were inspected
  directly. They agree with the implemented persisted shape and do not make a
  creation-time `mechanics_interface` mandatory.

## Scope and refreshed hash inventory

The product diff is exactly the five declared paths, `+844/-23`. Independent
`validate_change_scope.py` execution returned `PASS` with no violations.
Run-local plan, launch, return, check, and review records are evidence outside the
product diff.

Content hashes are SHA-256 of the accepted-basis blob and current working blob.
Binary-diff hashes are SHA-256 over
`git diff --binary 8ca1984db45a9a8f6f3111a905b07c7d3da47c33 -- <path>`.

| Path | Diff (+/-) | Basis content SHA-256 | Current content SHA-256 | Basis-to-current binary diff SHA-256 |
|---|---:|---|---|---|
| `core/model_operations/operation_applier/src/lib.rs` | `+413/-4` | `f6ecdc476482d8798591689770c4a87a318ec4765e0362b886ae46922b58be7f` | `13246f843fa9673b2de3fe8758695672d974f91bdd9be03e2e8b0698247f5bc8` | `6f2fb76572c079ffc5c5c2b1f12a05dc687602cc346124885150ec93c1ba109e` |
| `apps/desktop/src/features/component-creation/componentIntent.ts` | `+118/-3` | `19ebce4104c8b7900a004526c71769bc4ce8cfec0036342f5cdd643bb47e98d6` | `9fe80410d9694f843cd7276029f98ec094302b3bddc3fd2131935a16a046e3b7` | `40e082fbc6bcd68376e2d0561c957631aa539bf668211b64631d7671bc4a77a9` |
| `apps/desktop/src/features/model-tree/PropertyInspector.tsx` | `+48/-1` | `07162a3ba9d2a43b9a3a95f9b1af9b891559149d7fd408044250caa9a394b14f` | `ff95ef6b76fd9d2eac338a7f30ccbdc4285d70a0fe715cc5a4e03b88dac0c1e6` | `9c8d05b6ac69e13af1a4838a4fbb8f254b2cf1a023b0b1e7346cecbf34f197f8` |
| `apps/desktop/src/features/viewport/PipeViewport.tsx` | `+48/-1` | `ae5e770d512508859bf340401b49b13d322bbdf53071e0c940a58a217002bb4a` | `ad8f1ef826bbd153bcb8f9252afbc1262641a869cbacf4489b8e466dfc7f03ce` | `2b55b74f9c7ebd909c16db3aff904375255cf6613a93398898c53e6722e5f3de` |
| `apps/desktop/src/App.test.tsx` | `+217/-14` | `19d1aa176a447a3d6c491164cba007471a8e7cbc819b8141a485028e89a396cb` | `d66cb3ccccb9ac621812d35e0c864719888ce395447a9921246fc7a2d8829f8c` | `e983f287218c33232a2a263f8d47289e986517ed07c86740ced1eee7cb95fb83` |

Aggregate basis-to-current five-file binary-diff SHA-256:
`d0379aee6a142b7add00406a1117c852b0beb3c15496cd9d9d1e5d4f5f011df3`.

Every current and per-file diff hash reproduces the engine/UI repair-cycle return
inventories. Relative to failed review v1, only the intended shared helper and
transition-regression test blob changed; the applier and both form blobs remain at
their reviewed v1 hashes. `N2_REVIEW_V1.md` was rehashed before and after review at
`20c56ac4e25ff36ef38901d131a697b92f329b223721a015c65b573243ad9bc2`.

## Independently reproduced checks

| Check | Result |
|---|---|
| Focused Rust `expansion_joint_creation` slice | PASS — `4 passed`, `0 failed` |
| Focused Rust `creation` slice | PASS — `11 passed`, `0 failed`; bend, tee, reducer, valve, flange, expansion joint, and blocking tee equality included |
| Focused App matrix `component\|tee\|reducer\|valve\|flange\|expansion` | PASS — `36 passed`, `65 skipped` |
| Direct repair/three-incident slice | PASS — `5 passed`, `96 skipped`; four N2-F1 transition cases plus exact selected expansion-joint pipe |
| Desktop production build | PASS — TypeScript and Vite completed; standing large-chunk advisory only |
| Rust formatting check | PASS |
| Scoped five-file `git diff --check` | PASS |

The exact-final lane evidence was inspected against the current hashes:
`npm run test:desktop` passed `29 files / 573 tests`. The recorded browser-WASM
rebuild after engine fan-in, full operation-applier crate, practitioner harness
(`350 passed`), and self-check (exit `0`, standing unrelated findings only) are
coherent. The earlier unrelated `DeclarationsEditor` transient failure is preserved
in the lane record; it passed immediately in isolation and in subsequent full runs,
and its surface is outside this five-file diff.

## Fan-in disposition

**PASS.** Repair cycle 1 closes the only failed-review finding without weakening
the no-inferred-role discipline or adding engineering defaults. Expansion-joint
creation is valid for N2 manager fan-in and commit. This review does not itself
close row 14, update shared state, or perform lifecycle/release acceptance.
