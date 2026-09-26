# TASK S4 — Python readers, schemas, packaging and the generator mode

Read `_COMMON.md` first.

## Assignment
1. `P/core/analysis_runs/compatibility.py`: register `PREVIEW_PHYSICS_CONTRACT_ID` / sha256 `ae55503d…` / table path; dispatch in `_source_contract` (profile `product_preview_mechanics_v1`, `contract_evidence` required for this id only); the static `FRESH_CONTRACT_IDS`; `standing_reason`; `numerical_use_standing` → `needs_recompute` for non-fresh ids and for mixed ordinary source-blocks-1 (minimal hunk; T1 adds one early return for `load-reference-source-1` in the same function); a Python mirror of `rule_binding_refusal`. Enumeration hunks small (T1 edits the same lists).
2. New `P/core/analysis_runs/preview_physics_evidence.py` with every S1 §9 check (`validate_preview_physics_evidence`, and a transport-metadata variant if the physics module has one). `source_blocks.py`: the ordinary-case standing (non-composite only).
3. `P/core/handoff/stress_neutral/package_v0_3.py`: accept preview-physics-1 sources; the stress-neutral method CSV mapping and signature readers learn `arc_chord_frame` and `nominal_straight_beam_formula_on_arc_resultants` (and the other S1 §1 tokens) for this table only; existing tables keep their vocabularies. N-A: no notice or new field in any exported document or `manifest` (including `manifest.boundary_notes`); existing source-blocks-1 export bytes unchanged.
4. The three schemas `P/schemas/analysis_run.v0.3.schema.json`, `results.v0.3.schema.yaml`, `stress_neutral_export.v0.3.schema.json`: the new id and the preview `contract_evidence` shape wherever ids/evidence are enumerated; new vocabulary tokens where metadata enums exist.
5. `P/tools/serialization/generate_product_preview_mechanics.mjs`: a new mode that writes `P/fixtures/product_preview/invented_mechanics_result_preview_physics_1_{sparse,dense}.json` (and its own generation record beside `precision_fixture_generation.json`) from the unchanged invented demo model using the actual producer; the precision-1 pair and its record are never a write destination again (the default mode must refuse rather than overwrite them with preview-physics-1 output). Run the mode only after the manager tells you S2a has landed; those two outputs are yours to generate, and nothing else under `fixtures/`.
6. Python tests under `P/tests/`: new tests for all of the above, and updates to tests that enumerate identities or assert precision-1 as Current. Tamper tests as DESIGN §9.2 for the Python reader (same list as S3's), including the F-1 positive control on the invented-demo output (hanger, nonlinear and intensification diagnostics accepted) and a dangling `result:` ref refused.

## Write boundary
`P/core/analysis_runs/**`, `P/core/handoff/stress_neutral/package_v0_3.py`, the three schemas above, `P/tools/serialization/generate_product_preview_mechanics.mjs`, its two new outputs and new generation record, `P/tests/**` (Python only; add `P/tools/validation/*` only if a test proves it enumerates ids — ask first). Also `P/package.json` only if the new mode needs a script entry.

## Checks
From `P/`: `<venv>/bin/python -m pytest tests -q -x` (or the targeted subset, then the full `tests/` once) and any schema validation tests. Report counts and failures verbatim.
