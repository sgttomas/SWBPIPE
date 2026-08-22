# N2 integrated independent code review — round 3, review v1

- RunID: `HELP-HUMAN-PIPING-20260821-VOCABULARY-R3`
- Role: fresh bounded read-only ephemeral Agent 2 software-code reviewer
- Task method: `software-code-review`
- Accepted basis: N1 commit `8ca1984db45a9a8f6f3111a905b07c7d3da47c33`
- Reviewed state: 100% of the complete five-product-file basis-to-current-working-blob diff
- Verdict: **FAIL — one actionable preservation regression; repair cycle 1 required**
- Mutation boundary: no product, Git, shared status/coverage/handoff/receipt, launch brief, return, checks, or prior-review mutation; this immutable review record is the only write.

## Blocking finding

### N2-F1 — leaving expansion-joint mode strands accepted kinds with empty common evidence

**Location:** `apps/desktop/src/features/component-creation/componentIntent.ts:110-143`

`componentDraftForKind` clears `geometrySourceReference` and `provenance` when
the selected kind becomes `expansion_joint`, through `emptyExpansionJointInputs`,
but it restores neither field when the user next selects bend, tee, reducer,
valve, or flange. Those fields are common validity gates at lines 167-168.
Consequently an otherwise complete accepted-kind draft remains invalid after the
sequence `accepted kind -> expansion_joint -> accepted kind`; both Inspector and
viewport queue buttons stay disabled until the user manually reconstructs values
that previously initialized to `user_entered_component_form` and
`user_entered_local_preview`.

This is a concrete regression against the N2 brief's requirement to preserve bend,
tee, reducer, valve, and flange behavior. The current affected matrix always starts
from a fresh default draft and therefore does not exercise this transition.

Independent read-only reproduction against the current helper:

```text
initial:   geometry=user_entered_component_form; provenance=user_entered_local_preview
expansion: geometry=""; provenance=""
bend:      geometry=""; provenance=""; selected incident pipe=pipe:P-1
completed bend draft valid=false
```

**Required repair:** make kind transition state kind-safe. Entering
`expansion_joint` must still clear every expansion-joint input, including its
explicit geometry source and provenance, but leaving `expansion_joint` must restore
the accepted non-expansion source/provenance initialization (without inferring any
pipe role or expansion-joint engineering value). Add direct Inspector and viewport
regressions that switch from a partially or fully entered expansion joint back to
representative accepted kinds, prove the accepted common evidence and deliberate
pipe-selection behavior are restored, and prove the resulting old-kind intent can
queue/apply. Preserve the existing expansion-joint kind/node/post-queue negatives.

No other actionable finding was found.

## Expansion-joint acceptance matrix

| Requirement | Result | Review evidence |
|---|---|---|
| UI/applier schema parity | PASS | Both sides use `geometry.expansion_joint_pipe_ref`, `effective_area`, `movement_limit`, `hardware_reference`, `manufacturer_reference`, `pressure_thrust_reference`, `expansion_joint_source_reference`; `modifiers` has the four exact `*_stiffness_user_value` keys plus `source_reference`; the resolver canonically reconstructs those same keys. |
| Structured-operation end-to-end application | PASS | Intent remains `insert_component_symbol` / `components`; applier allowlists `expansion_joint`, resolves geometry and modifiers, and inserts only the canonical resolved record through the existing copy-on-write apply branch. Browser-WASM-backed App positives apply the created component. |
| Deliberate incident pipe / no collection-order inference | PASS | Expansion-joint defaults, kind selection, node change, and post-queue reset leave `primaryPipeRef` empty. The three-incident App case selects `pipe:P-150`, verifies it in the serialized intent, applies it, and observes `pipe:P-150` on the created component rather than earlier collection members. |
| Explicit positive area and movement | PASS | UI requires non-empty positive numeric entries and explicit units; resolver requires finite positive quantities and checks `Area` / `Length`. Positive evidence preserves `0.018 m^2` and `0.045 m`. |
| Explicit four-axis stiffness with dimensions | PASS | Axial/lateral require `LinearStiffness`; angular/torsional require `RotationalStiffness`; every value must be finite and positive. Positive evidence preserves `3200000 N/m`, `900000 N/m`, `480000 N*m/rad`, and `620000 N*m/rad`. |
| Exact references and provenance | PASS | Hardware, manufacturer, pressure-thrust, geometry-source, stiffness-source, and common provenance are all required, trimmed, serialized, canonically reconstructed, and asserted across intent/applied-model evidence. |
| Blocking invalid or absent inputs | PASS | Missing pressure-thrust or axial data, unknown pipe, nonincident pipe, area dimension mismatch, angular dimension mismatch, and nonpositive torsional stiffness block; every negative asserts no `applied_model`. |
| No `mechanics_interface` or invented engineering default | PASS | UI payload contains only identity, geometry, modifiers, and explicit provenance. Applier reconstructs the same canonical shape and does not add `mechanics_interface`, pipe roles, quantities, units, references, or stiffness values. |
| Existing-kind preservation | **FAIL** | Direct fresh-draft and resolver regressions pass, but N2-F1 breaks the accepted-kind path after an expansion-joint kind transition. |

The accepted product-physics input types and validation were inspected directly.
They name the same geometry/modifier fields, require the expansion-joint pipe to be
incident to the component node, require positive geometry/stiffness quantities,
and do not require a creation-time `mechanics_interface`. No unresolved
geometry/connectivity contract question was found.

## Scope and refreshed hash inventory

The reviewed product scope is exactly five authorized paths, `+748/-23`. Scope
validation against the declared five paths returned `PASS` with no violations.
Run-local plan/brief/return/check records are outside the product diff and were read
as evidence only.

Binary-diff hashes are SHA-256 over
`git diff --binary 8ca1984db45a9a8f6f3111a905b07c7d3da47c33 -- <path>`.

| Path | Diff (+/-) | Basis content SHA-256 | Current content SHA-256 | Basis-to-current binary diff SHA-256 |
|---|---:|---|---|---|
| `core/model_operations/operation_applier/src/lib.rs` | `+413/-4` | `f6ecdc476482d8798591689770c4a87a318ec4765e0362b886ae46922b58be7f` | `13246f843fa9673b2de3fe8758695672d974f91bdd9be03e2e8b0698247f5bc8` | `6f2fb76572c079ffc5c5c2b1f12a05dc687602cc346124885150ec93c1ba109e` |
| `apps/desktop/src/features/component-creation/componentIntent.ts` | `+111/-3` | `19ebce4104c8b7900a004526c71769bc4ce8cfec0036342f5cdd643bb47e98d6` | `f522085b29ad3cb69052e6b7ce6ce1203901b0b349858fa535f28cff024ca538` | `0155b1113506c6edfb190e428d2193c0aab82e0ecd27ed5a38ff77472ae80d27` |
| `apps/desktop/src/features/model-tree/PropertyInspector.tsx` | `+48/-1` | `07162a3ba9d2a43b9a3a95f9b1af9b891559149d7fd408044250caa9a394b14f` | `ff95ef6b76fd9d2eac338a7f30ccbdc4285d70a0fe715cc5a4e03b88dac0c1e6` | `9c8d05b6ac69e13af1a4838a4fbb8f254b2cf1a023b0b1e7346cecbf34f197f8` |
| `apps/desktop/src/features/viewport/PipeViewport.tsx` | `+48/-1` | `ae5e770d512508859bf340401b49b13d322bbdf53071e0c940a58a217002bb4a` | `ad8f1ef826bbd153bcb8f9252afbc1262641a869cbacf4489b8e466dfc7f03ce` | `2b55b74f9c7ebd909c16db3aff904375255cf6613a93398898c53e6722e5f3de` |
| `apps/desktop/src/App.test.tsx` | `+128/-14` | `19d1aa176a447a3d6c491164cba007471a8e7cbc819b8141a485028e89a396cb` | `1794c4672ee2328eb0570add2a8820188f1b7265702dad2571ff09501eccc7c8` | `d5734193819d51b6c4ca9dcfb8ec281dec5ac1c6f5a2cf947a9571568c8af1e7` |

Aggregate basis-to-current binary-diff SHA-256:
`4c7d03f165a0a35cc0ec90b458c1feaf054c44935edbfacc62e2e8401f0ce4e9`.

All five basis/current/per-file diff hashes exactly reproduce the engine and UI
return inventories.

## Independently reproduced checks

| Check | Result |
|---|---|
| `cargo test --offline --manifest-path projects/chirality-piping/core/model_operations/operation_applier/Cargo.toml --lib expansion_joint_creation -- --nocapture` | PASS — `4 passed`, `0 failed` |
| `cargo test --offline --manifest-path projects/chirality-piping/core/model_operations/operation_applier/Cargo.toml --lib creation -- --nocapture` | PASS — `11 passed`, `0 failed`; bend, tee, reducer, valve, flange, and expansion-joint creation included |
| `npm exec vitest -- run src/App.test.tsx -t 'component\|tee\|reducer\|valve\|flange\|expansion'` from `apps/desktop` | PASS — `32 passed`, `65 skipped` |
| Exact unrelated `DeclarationsEditor` stored-unit retry | PASS — `1 passed`, `28 skipped` |
| `npm run build:desktop` from the Piping workspace | PASS — TypeScript and Vite build; standing large-chunk advisory only |
| `cargo fmt --manifest-path projects/chirality-piping/core/model_operations/operation_applier/Cargo.toml -- --check` | PASS |
| Scoped `git diff --check` over the five reviewed paths | PASS |

The UI lane's full-check evidence was inspected: the first exact desktop attempt
had one `DeclarationsEditor` stored-unit assertion failure (`568/569`); that exact
unrelated test then passed alone, the full suite passed `569/569`, and the
exact-final full suite passed `569/569` again. No code in the failed test's surface
or dependencies is in the five-file N2 product diff. The engine lane's full crate,
WASM release build, UI browser-WASM rebuild after engine fan-in, practitioner
harness (`350`), and self-check (exit `0`, standing unrelated findings) evidence
was also inspected and is coherent with the current hash inventory.

## Fan-in disposition

**FAIL.** The expansion-joint creation path is otherwise review-ready, and there
is no engine repair or domain decision to make. Apply one UI repair cycle for
N2-F1, retain this failed review and hash inventory immutably, rerun the affected
matrix and exact-final checks, then dispatch a fresh reviewer over 100% of the
refreshed five-file diff. N2 is not accepted for commit or final fan-in yet.
