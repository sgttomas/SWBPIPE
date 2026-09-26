# TASK brief — T1_WP1_JOINED_SCHEMAS (carrier schema branches for `load-reference-source-1`)

Read `_T1_COMMON.md` first. Return folder: `LSI/T1_WP1_JOINED_SCHEMAS/`.

## Assignment

Add additive `load-reference-source-1` branches to the three 0.3 carrier schemas:

- `schemas/results.v0.3.schema.yaml`;
- `schemas/analysis_run.v0.3.schema.json`;
- `schemas/stress_neutral_export.v0.3.schema.json`;
- plus `schemas/analysis_run.schema.json`, if its root dispatches versions as it did for load-reference-1.

Mirror how CP3 added the load-reference-1 branches (`LSI/CP3_SCHEMAS/RETURN.md`, `BRIEF.md`) and how the physics-source-1 branches are built.

- Each new branch pins producer ID `openpipestress.result_semantics/0.3.0/load-reference-source-1` and profile `resolved_straight_load_state_source_v1`.
- The results branch requires `source_block_recovery` with policy `LOAD-REFERENCE-SOURCE-1`, and `contract_evidence` with exactly `{pressure, connector, exact_cases, load_reference_states}`. The `load_reference_states` items use `schemas/load_reference_state.schema.json`, as the load-reference-1 branch does.
- The AnalysisRun and stress-neutral branches mirror physics-source-1 in their own files, for whatever each carries.
- Old documents must match the same branch as before. The pointer diff must show only appends.

**Tests.** Add a new `tests/test_load_reference_source_schema.py`:

- the ten committed joined raws (`fixtures/product_preview/load_reference_source/*.raw.json`) validate where the results schema applies to raw transport metadata;
- every existing load-reference-1, physics-source-1 and physics-1 fixture still validates against its unchanged branch;
- relabels are refused;
- schema-weakening mutations (removing a pin, the required receipt or an exact key) are each caught.

A parallel TASK, T1_WP1_JOINED_READERS, is producing the canonical documents and AnalysisRun records (`fixtures/results/load_reference_source_*`). If they exist when you finish, validate them too; otherwise say so, and the manager will integrate.

## Write boundary

- The four schema files above.
- `tests/test_load_reference_source_schema.py` (new).
- Your return folder.

## Checks

- Your new test file, plus `tests/test_load_reference_schema.py`. Its baseline is 527 passed, and it must be unchanged.
- The schema tests of physics-source-1 and physics-1, if present.
- The weakening mutations.
