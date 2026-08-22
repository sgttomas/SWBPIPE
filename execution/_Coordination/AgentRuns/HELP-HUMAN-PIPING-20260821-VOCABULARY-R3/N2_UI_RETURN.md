# N2 UI lane return — expansion-joint creation

- RunID: `HELP-HUMAN-PIPING-20260821-VOCABULARY-R3`
- Instance: `WORKING-ITEMS-VOCAB-R3-N2-UI`
- Role: `WORKING_ITEMS`; package `PKG-07`; deliverables `DEL-07-01`, `DEL-07-02`
- Accepted basis: N1 commit `8ca1984db45a9a8f6f3111a905b07c7d3da47c33`
- Runtime: inherited GPT-5.6 Codex; no substitution
- Verdict: **PASS — repair cycle 1 complete; ready for fresh integrated N2 review**

## Product result

The Inspector and viewport now create `expansion_joint` components through the structured-operation seam using the accepted product-physics shape: one deliberately selected incident pipe; positive effective area and movement limit; hardware, manufacturer, pressure-thrust, geometry-source, and stiffness-source references; positive axial/lateral/angular/torsional stiffness quantities with explicitly selected area/length/linear-stiffness/rotational-stiffness units; and explicit common provenance. The intent serializes `geometry` plus `modifiers` only, matching the engine resolver and accepted preview fixture.

No pipe or engineering value is inferred. Expansion-joint pipe, unit, value, source, and provenance inputs initialize empty on kind selection; the selected pipe clears on node change; the complete draft resets after queue while retaining the selected kind. Queueing remains disabled until all required inputs are present. Existing bend, tee, reducer, valve, and flange paths remain covered by the affected matrix.

## Repair cycle 1 — N2-F1

Immutable failed review `N2_REVIEW_V1.md` (SHA-256 `20c56ac4e25ff36ef38901d131a697b92f329b223721a015c65b573243ad9bc2`) identified `N2-F1`: leaving expansion-joint mode retained its intentionally empty common geometry-source/provenance fields and stranded accepted existing kinds. The repair changes only kind-transition state. Entering `expansion_joint` still clears every explicit expansion-joint input; leaving it restores the established existing-kind defaults `user_entered_component_form` and `user_entered_local_preview`. It does not restore or infer any pipe role or engineering value. Bend retains its accepted incident-pipe behavior; valve and the other non-bend kinds retain empty pipe selection until deliberate user choice.

Four direct applied regressions cover Inspector and viewport transitions from `expansion_joint` to bend and valve. Each proves restored common evidence, usual required-field completion, and successful structured-operation application; the valve cases additionally prove the realized-pipe selector remains empty until deliberately selected.

## Required evidence

- Browser WASM rebuilt after engine fan-in: PASS.
- Focused affected App matrix: PASS, `36 passed / 65 skipped`.
- Inspector and viewport kind/node/post-queue negatives: PASS.
- Three-incident node positive: PASS; explicit selection `pipe:P-150` (rather than earlier collection members `pipe:P-100` or `pipe:P-110`) serialized and applied.
- Positive application preserves exact entries: `0.018 m^2`, `0.045 m`, `3200000 N/m`, `900000 N/m`, `480000 N*m/rad`, `620000 N*m/rad`, all five source/reference inputs, and common provenance.
- Exact-final repair-cycle desktop suite: PASS, `29 files / 573 tests`.
- Desktop production build: PASS (standing Vite chunk-size warning only).
- Practitioner harness: PASS, `350 passed`.
- Harness self-check: exit `0`; standing unrelated REVIEW/WARN findings remain unchanged.
- Scoped `git diff --check`: PASS.

The first full desktop attempt recorded one unrelated `DeclarationsEditor` stored-unit assertion failure (`568/569`). The exact failed test immediately passed `1/1` in isolation; the full suite then passed `569/569`, and the exact-final full suite passed `569/569` again. No product repair outside this lane was made.

## Hash inventory

| File | Accepted-basis SHA-256 | Current SHA-256 | Basis-to-current diff SHA-256 |
|---|---|---|---|
| `apps/desktop/src/features/component-creation/componentIntent.ts` | `19ebce4104c8b7900a004526c71769bc4ce8cfec0036342f5cdd643bb47e98d6` | `9fe80410d9694f843cd7276029f98ec094302b3bddc3fd2131935a16a046e3b7` | `40e082fbc6bcd68376e2d0561c957631aa539bf668211b64631d7671bc4a77a9` |
| `apps/desktop/src/features/model-tree/PropertyInspector.tsx` | `07162a3ba9d2a43b9a3a95f9b1af9b891559149d7fd408044250caa9a394b14f` | `ff95ef6b76fd9d2eac338a7f30ccbdc4285d70a0fe715cc5a4e03b88dac0c1e6` | `9c8d05b6ac69e13af1a4838a4fbb8f254b2cf1a023b0b1e7346cecbf34f197f8` |
| `apps/desktop/src/features/viewport/PipeViewport.tsx` | `ae5e770d512508859bf340401b49b13d322bbdf53071e0c940a58a217002bb4a` | `ad8f1ef826bbd153bcb8f9252afbc1262641a869cbacf4489b8e466dfc7f03ce` | `2b55b74f9c7ebd909c16db3aff904375255cf6613a93398898c53e6722e5f3de` |
| `apps/desktop/src/App.test.tsx` | `19d1aa176a447a3d6c491164cba007471a8e7cbc819b8141a485028e89a396cb` | `d66cb3ccccb9ac621812d35e0c864719888ce395447a9921246fc7a2d8829f8c` | `e983f287218c33232a2a263f8d47289e986517ed07c86740ced1eee7cb95fb83` |

## Containment and handoff

Writes are contained to the four authorized UI/test paths plus this return and `N2_UI_CHECKS.json`. No resolver, shared status/coverage/handoff/receipt, or Git surface was modified by this lane. There is no geometry/connectivity contradiction or unresolved design question. Fresh read-only integrated review may proceed over the complete N2 diff.
