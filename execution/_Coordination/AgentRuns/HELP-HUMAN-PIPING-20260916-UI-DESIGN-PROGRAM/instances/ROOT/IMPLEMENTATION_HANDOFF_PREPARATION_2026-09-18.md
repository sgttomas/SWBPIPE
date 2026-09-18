# Implementation-ready handoff — preparation (not yet ready)

Status: ROOT record, 2026-09-18, successor ROOT. **The design work is not ready to hand to implementation, and this record authorizes nothing.** It fixes, now, the constraints and the shape the handoff will have, so that the revision pass and the owner's rulings fill it in rather than reshape it. This program's assignment is design only; product implementation needs the owner's separate authorization of an implementation tranche.

## 1. What must be true before the handoff is issued

| # | Prerequisite | State |
|---|---|---|
| P-1 | D-70's performance handoff condition | Satisfied ([reconciliation](PIPING_HANDOFF_RECONCILIATION_2026-09-18.md)) |
| P-2 | D-71 ruled, item by item; deferred items named as deferred | Ruled 2026-09-18, all nine items (`DEC-099` to `DEC-105`; `DEC-105` supersedes `DEC-099`: the maturity sentence is removed everywhere, and the claims lint's `MISSING_MATURITY_BANNER` anchor changes in the same tranche) |
| P-3 | Q-15 to Q-22 answered ([preparation](REVISION_PASS_PREPARATION_2026-09-18.md) §2) | Answered 2026-09-18; Q-20 amended |
| P-4 | Revision pass accepted: design system V1.2, UX specification V1.1, frames; operations map re-checked against the then-current `HEAD` | Running: DESIGN-SYSTEM-03 launched 2026-09-18 |
| P-5 | D-72 ruled (performance acceptance criteria), or the owner states that implementation may start before it and qualification waits for it | Ruled 2026-09-18, complete and frozen (ruling and addendum) |
| P-6 | If D-71 item 3(i) is ruled: the scope-change bundle for PRD §19.3 accepted separately, or the rename sequenced after the first tranche | SCA-010 accepted by the owner 2026-09-18; executed with the tranche that executes `DEC-101` |
| P-7 | The owner's decision to authorize the implementation tranche, naming the owning loop | Reserved to the owner |

## 2. Constraints every implementation brief carries

From the owner's handoff message ([activation record](SUCCESSOR_ACTIVATION_2026-09-18.md) §3, "Implementation-handoff constraints"), restated as checks a reviewer can apply:

1. **One mutation route.** Every engineering action, by table cell, canvas gesture, paste, accepted proposal or future harness, is a typed operation through the existing Rust applier (`applyModelOperation`, `applyOperationBatch`). No second route for the interface. Semantic equivalence between a human's and an agent's action is a test, not a convention.
2. **Projections, not rivals.** "The tables are the model" describes the table-first experience. Tables and canvas are projections and editors of the canonical model; neither holds state the model does not, and neither is independently mutable.
3. **Result integrity preserved.** Current and Historical designation, the exact solve-input basis, stale-response guards, reviewed application, undo and redo, and persistence compatibility are kept. A Historical record never acquires a current-model overlay or a readiness claim through a presentation change.
4. **Picking repair preserved.** The stable shared closest-point computation of PR #794 and its maintained regression tests stay. Pick tolerances and oracle expectations are never altered to obtain a benchmark result.
5. **Rendering foundation retained** unless separately justified and authorized: persistent renderer, instancing and chunking, invalidation scheduling, resource ownership, typed model index, selection and picking.
6. **Every control maps to an operation or a classed gap.** The operations map is the instrument: 277 rows, 29 gap entries (G-01 to G-29). A mock control does not establish an implemented operation and does not authorize a schema migration.
7. **Semantic changes are named as changes** (§4 below), never delivered as restyling.
8. **The benchmark stays a reusable regression instrument**: named setup and query actions, typed entity identities, real pointer and keyboard stimuli for interaction responsiveness.
9. **The demonstration is a comparison basis.** The redesigned canvas, geometry, labels and overlays change the workload; acceptance is under D-72's criteria once ruled.
10. **Separately scoped work stays separate**: pressure runtime, connector mechanics, sparse execution, export implementation. Shared solver and result contracts are coordinated; substantial builds, tests and other runtime activity stay clear of timed measurement.

## 3. Gap classes and the write scopes they imply

From the operations map's gap list. The classes have different owners and different authorization, which is why they are different tranches and not one.

| Class | Gaps | Write scope (relative to `{WORKING_ROOT}`) | Authorization it needs beyond the tranche |
|---|---|---|---|
| Interface only | G-27, G-28, G-29 and every provider row marked change kind, composed or derived | `apps/desktop/src/**`, `apps/desktop/e2e/**` | None beyond the tranche; independent review is mandatory for `apps/desktop/src/**` product behaviour |
| Rendering | G-16 | `apps/desktop/src/features/viewport/**` and its tests | None beyond the tranche; the foundation of constraint 5 is out of scope |
| Typed interface | G-05, G-07 to G-11, G-14, G-15, G-17, G-18, G-20, G-21, G-25, G-26 | `schemas/**`, `apps/desktop/src/types.ts`, `apps/desktop/src-tauri/**`, persistence | Each new record or field is a schema change with a persistence-compatibility obligation; authorized per gap, not by the mock that needs it |
| Engine | G-01 to G-04, G-06, G-12, G-13, G-22 to G-24 | `core/**` | New change kinds and result fields; independent review mandatory; coordinated with the separately scoped solver work |
| Host | G-19 | — | Held by `DEC-042` and `DEC-091`; a separate owner stage act. The design specifies the seam only |

A first tranche that stays inside the first two rows can deliver the shell, the three views, the table component, the docked inspector, the status bar and the results header over operations that exist today, with every gap-dependent control absent or disabled with its reason, never faked.

## 4. Proposed semantic changes already identified

From [RESEARCH-G](../RESEARCH/G_rendering_workload_classification.md) §2 and the operations map; completed by the revision pass. Source paths are relative to `{WORKING_ROOT}`.

| Change | Today | Proposed | Needs |
|---|---|---|---|
| Isolate | Hides: the complement goes to the same hidden-key mask as Hide, which removes geometry (`apps/desktop/src/features/viewport/viewportSelection.ts:246-276`; `viewportResource.ts:322-341`) | Dims everything else to 20 % opacity; Hide removes and the HUD shows the count | A decision that dimming replaces, or joins, the current behaviour |
| Camera and interface state | Not persisted with the project (G-17) | Per-stage view memory, camera, report-figure preset, label mode, hidden sets persisted | Saved-state requirement; G-17 |
| Results after a model change | Run cleared on any change (G-11) | Kept as Historical basis, hatched, chips dropped | Typed-interface change with result-integrity review under constraint 3 |
| Proposal review | One proposal slot, session receipts (G-18) | Per-row decisions, multi-row and batch, persisted receipts | G-18; live arrival is G-19 and held |
| Selection presentation | Replaces the instance colour (`apps/desktop/src/features/viewport/viewportResource.ts:981-985`), which would destroy result colour | A halo over unchanged colour | Rendering change; the selection and picking foundation itself is kept |
| Theme on the canvas | The switch changes background, gizmo and selection colour; model colours are literals | Every canvas colour a token with light and dark values, repainted live | Rendering change; G-16 |
| Checked mark | None | Human row tag outside the model payload | D-71 item 7 ruling; G-08 |
| Fitted camera state | None | Refit on dock only if unmoved since Fit (Q-20, if the owner agrees) | New interface state |
| Label mode | Cap of 80 | Budget of one per 3,600 square pixels, with All and Off | Rendering; workload change under D-72 item 3 |

## 5. Verification the handoff will require

Registered checks of the owning loop (`software-workflow.json`), the DEC-025 sweep including host-capability Playwright surfaces, the claims-language lint and its three anchors, the picking regression tests, the operation-equivalence tests of constraint 1, a Historical-record presentation test for constraint 3, WCAG 2.2 AA criteria for touched controls (D-68), a fresh read-only independent review over the complete frozen diff, and performance qualification under D-72 once ruled. The independent-usability holds PDU-045 and PDU-046 remain holds; passing the above does not close them.

## 6. The owner decision the handoff will ask for

One decision: authorize a named implementation tranche, in a named owning loop, with the write scope of §3's first two rows (or a wider one the owner chooses), under the constraints of §2, with qualification under the ruled D-72 criteria. The handoff will state the accepted design decisions by record and hash, the unresolved items, and the gaps left out of the tranche.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
