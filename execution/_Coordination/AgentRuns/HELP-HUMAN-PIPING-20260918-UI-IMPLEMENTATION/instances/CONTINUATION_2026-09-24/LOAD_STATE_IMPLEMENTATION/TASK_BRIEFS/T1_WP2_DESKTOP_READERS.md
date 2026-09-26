# TASK brief — T1_WP2_DESKTOP_READERS (desktop 0.4.0 types, load/reference result readers and saved-result standing)

Read `_T1_COMMON.md` first, including its **Wave 2** section. Return folder: `LSI/T1_WP2_DESKTOP_READERS/`.

## Assignment

The desktop must carry 0.4.0 documents losslessly and read 0.4.0 results truthfully. At present a `load-reference-1` or `load-reference-source-1` result dispatches as `unsupported` in TypeScript, although both are in the fresh set.

**1. Types (`apps/desktop/src/types.ts`).**

- Add lossless types for the 0.4.0 model additions: `reference_configurations`, `materials[].expansion_laws`, `load_cases[].analysis_state` with every closed variant, and the result evidence `contract_evidence.load_reference_states`.
- The Rust DTOs in `core/product_physics/src/case_state/input.rs` are the shape of record, together with `schemas/load_reference_state.schema.json`.
- There must be no lossy projection anywhere a model or result passes through the desktop (session state, persistence, the IPC and browser-fixture paths). Unknown or absent keys round-trip byte-exactly. Find every projection point and prove it with a test.

**2. Readers (`apps/desktop/src/features/results/`).**

- Add explicit dispatch in `sourceContract` for both identities, with their exact profiles:
  - `load-reference-1`: `resolved_straight_load_state_v1`, contract evidence required, no `source_block_recovery`;
  - `load-reference-source-1`: `resolved_straight_load_state_source_v1`, its receipt shape.
- Add evidence validation that mirrors the Python readers exactly: `core/analysis_runs/load_reference_evidence.py` for `load-reference-1`, and `core/analysis_runs/load_reference_source.py` for the joined identity. Rust `result_export` `load_reference*` is the other peer. Follow the existing TS pattern (`physicsResultEvidence.ts`, `previewPhysicsEvidence.ts`, `physicsSourceRecovery.ts`). A header never authenticates its claimed producer.
- **Standing.**
  - `load-reference-1` goes through T0R's existing generic standing unchanged.
  - `load-reference-source-1` gets the declared T1 early `needs_recompute`, placed after validation and after T0R's standing reason, exactly as in Rust and Python.
  - No other standing, rule-binding or export change. Where an export or report surface refuses unknown identities today, it must keep refusing or labelling these ones truthfully. Report what you find; do not open new export routes.

**3. Cross-language parity.**

- The Rust and Python reader suites record per-case outcomes: `rust_outcomes.json` in `result_export/tests/load_reference*_contract.rs`, and `python_outcomes.json` under `LOAD_REFERENCE_PARITY_OUT` and `LOAD_REFERENCE_SOURCE_PARITY_OUT`.
- Reproduce the same accept/refuse outcome for every case in TypeScript, over the committed raws in `fixtures/product_preview/load_reference*/` and the same mutation corpus.
- Add a test that fails on any disagreement. Record the three logs.

**4. Saved results.**

- The saved result keeps the raw result hash and the resolved evidence.
- On reopen, a result whose model or reference basis changed is not Current. T0R's hash-scope and session code (`analysisResultHashScope.ts`, `resultsSessionState.ts`, `HistoricalRunContext.tsx`) already establish the mechanism. Extend it to 0.4.0 only where needed, and prove it by test: edit a `reference_configurations` value, or a case's `analysis_state`, and the result is no longer Current.

## Write boundary

- `apps/desktop/src/**`, excluding the Load Case Manager and Materials panels (WP3 native inputs, a later TASK) and `src-tauri`.
- Your return folder.
- A fixture or test under `fixtures/` or `tests/` only if new, and say so.

## Checks

- Desktop vitest: the full suite, not only your files. Report the baseline count at the base commit and your count.
- `npm run build --workspace apps/desktop` (type check and production build).
- Mutants: remove each new reader refusal, the joined early return and each lossless-projection guard. Every mutant must be killed.
