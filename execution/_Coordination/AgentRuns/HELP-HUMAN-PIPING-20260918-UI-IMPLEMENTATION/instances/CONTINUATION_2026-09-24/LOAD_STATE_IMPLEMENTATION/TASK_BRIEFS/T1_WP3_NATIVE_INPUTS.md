# TASK brief — T1_WP3_NATIVE_INPUTS (plain 0.4.0 input fields and the read-only resolved-state block)

Read `_T1_COMMON.md` first, including its **Wave 2** section. Return folder: `LSI/T1_WP3_NATIVE_INPUTS/`.

**Base.** The manager's integration of T1_WP2_DESKTOP_READERS, which the spawn request names. Use its types and readers; do not redefine them.

## Assignment (D2: plain fields, no layout or visual work)

**1. Inputs.** In the existing panels (`features/load-cases/LoadCaseManagerPanel.tsx` and the materials form `features/material-temperature/`, or wherever materials are edited), add plain fields so that a human can enter, on a 0.4.0 model:

- **Reference configuration.** Its member references, basis and fit (`Model.reference_configurations`).
- **Material expansion laws** (`Material.expansion_laws`).
- **Per load case `analysis_state`:**
  - element states: thermal and fit strain, or temperatures with law and material selection;
  - support states with `boundary_motion`;
  - `load_sources` with factors.

Rules for the fields:

- Every edit submits through the existing typed operations (`core/model_operations/operation_applier` `load_state_authoring.rs`, via the wasm engine or the native IPC exactly as the other panels do). They use Review/Apply, Undo/Redo and atomic batches.
- There is no client-side mutation of the model.
- No field has a default, placeholder quantity, library value or code value. Empty means absent. Units are shown and are explicit.
- On a pre-0.4 model the fields are not offered. Instead they show one line: load/reference state needs a 0.4.0 model. There is no upgrade operation (D3).
- Refusals from the operations (unknown field, null, dangling reference, wrong version) are shown as targeted diagnostics, and the model stays editable.

**2. A new 0.4.0 model.**

- Offer creating a blank 0.4.0 model beside the existing blank-model path (`services/projectService.ts` `buildBlankLocalModelDocument`), with the exact pressure contract (`2.0.0` / `exact_straight_pressure_v2`).
- It gets no engineering content: no materials, sections, cases or reference values.
- If this needs a native IPC change, stop and describe it to the manager.

**3. The read-only resolved-state block.**

- In the Solve result (`features/solve/SolvePanel.tsx`), show the published `contract_evidence.load_reference_states` record, as read by the WP2 readers.
- For each case, show the resolved element temperatures and strains, support motions and sources, with the case's numerical standing.
- Label the route:
  - ordinary `load-reference-1`;
  - joined `load-reference-source-1`, always shown as needing recompute (not Current) in T1.
- A sensitive result is never shown as Current.
- The block is read-only text or a plain table. It shows only published values; it computes none.

## Write boundary

- `apps/desktop/src/features/load-cases/**`, `features/material-temperature/**` (or the actual materials editor), `features/solve/**`, and `services/projectService.ts` (the blank-model function only), with their tests.
- Browser e2e specs under `apps/desktop/e2e/` (new files).
- Your return folder.

## Checks

- **Desktop vitest, the full suite.** Report the baseline and your count. Tests must cover:
  - operation submission for each field group;
  - refusal display;
  - no defaults;
  - pre-0.4 gating;
  - the resolved-state block for an ordinary result, a joined result and a sensitive result, using the committed raws in `fixtures/product_preview/load_reference*/`.
- `npm run build --workspace apps/desktop`.
- **Chromium Playwright** on the browser fixture. Author support motion, an element thermal state and a material selection through the fields. Review/Apply, then Undo/Redo, and show that the document bytes are restored. Solve in both modes and see the resolved-state block.
- Mutants: remove the pre-0.4 gating, the not-Current labelling of joined and sensitive results, and one operation submission. Every mutant must be killed.
- Native behaviour is not claimed. The owner's Mac witnesses 2–8 (`T1_PLAN.md` §6) stay outstanding.
