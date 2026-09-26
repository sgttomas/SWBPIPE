# TASK brief — T1_WP4_HEADLESS (0.4.0 through the headless runner and CLI)

Read `_T1_COMMON.md` first, including its **Wave 2** section. Return folder: `LSI/T1_WP4_HEADLESS/`.

## Assignment

Add tests of the 0.4.0 path through `core/runner/headless`: the `openpipestress-runner solve` CLI, and the library value route. Run each in both solver modes. This is tests-first work: **no product semantics change**. If the binding or runner needs a seam, stop and describe it to the manager; do not add it on your own.

Cover:

1. **`load-reference-1`.**
   - The `ControlledExport` wrapper.
   - The canonical results document built by `result_envelope_binding` from an actual solve of `fixtures/product_preview/load_reference/{connected,pressure}.request.json`. It must validate against `schemas/results.v0.3.schema.yaml` on the `load-reference-1` branch.
   - Its AnalysisRun record: `core/analysis_runs` `build_analysis_run_v0_3` must verify with `match`.
   - `QualifiedPreviewEvidence` behaviour as the binding defines it for this identity. Characterize what exists; do not extend it.
2. **`load-reference-source-1`**, from an actual solve of the committed joined witnesses (`fixtures/product_preview/load_reference_source/*.request.json`).
   - The receipt is retained and bound to the actual invocation, as in the source-blocks tests in `result_envelope_binding.rs`.
   - The canonical document selects the joined branch.
   - Standing is `needs_recompute` in every reader.
   - Reparsed public receipt bytes are not the library's opaque proof.
3. **Refusal controls through the runner.** Explicit null on a 0.4.0 key, an unknown field, and a pre-0.4 document carrying 0.4.0 keys. Each gets its targeted code, and nothing is published as solved.
4. **Blocked 0.4.0 envelopes.** A 0.4.0 document without the exact pressure contract gives a blocked envelope on `load-reference-1`, never `preview-physics-1` (`CHECKPOINT_6.md`). Check this through the runner.
5. **SF-1 fallback through the runner.** The budget-cliff request from `core/product_physics/src/source_receipt/load_state_fallback_tests.rs` publishes on the ordinary route with its diagnostic. It must not be a blocked result.
6. **Actual artifacts.** Follow the existing pattern (`HEADLESS_SOURCE_BLOCK_OUTPUT_DIR`, `HEADLESS_PHYSICS_OUTPUT_DIR`): write the actual raw, document, request, invocation and manifest files for (1) and (2) when an env var is set, and add the Python consumer tests that read them. Those tests must not be left skipped in your recorded run: run them with the variable set.

## Write boundary

- `core/runner/headless/**` (tests; production code only for artifact emission behind an env var, as the existing lanes do).
- New Python tests in `tests/`.
- Your return folder.

## Checks

- `core/runner/headless` `cargo test`. Report the baseline at the base commit and your count.
- The new Python tests, and `tests/test_load_reference_readers.py` and `tests/test_load_reference_source_readers.py`, with the artifact variables set.
- Mutants: break the identity routing, the receipt binding and each refusal at the runner seam. Every mutant must be killed.
