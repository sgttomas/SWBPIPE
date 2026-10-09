# Bundled demo results and the refused demo's envelopes

**The bundled demo.** `invented_demo_model.json` is the valid demo model behind the browser's bundled reference results. It is the invented utility loop with no legacy pressure primitives (the legacy pressure contract is retired product-wide) and no expansion joint component:C-150 (its user-stiffness element is refused). It is derived from `core/product_physics/tests/fixtures/preview_physics_invented_model.json`; only its project id, name and description, its load-case labels (which named the removed pressure) and its diagnostic's source path differ. It is also the app's default session model.
- `invented_demo_result_preview_physics_1_sparse.json` and `_dense.json` are verbatim stdout from the product on that model, one per solver mode: solved, preview-physics-1. The browser loads them only through its explicit reference inspection, never as a solve for the current model.
- `invented_demo_result_legacy_0_1.json` is a historical-format carrier of the sparse output. The Python preview service (`core/product_preview/service.py`) returns it as its frozen mechanics result, and tests of readers of the legacy 0.1.0 result format read it. It is the sparse output's exact bytes with four changes:
  - its four 0.2.0 header members (`producer`, `numerical_quality`, `formulation_basis`, `contract_evidence`) are removed;
  - `schema_version` is set to `0.1.0`;
  - the rows whose kinds the historical 0.1.0 result semantics (`fixtures/results/semantic_contract_v0_2.json`) do not define are removed (five preview-physics-1 kinds, 112 rows);
  - the summary members that describe removed rows are set to null: the headline `max_open_formula_stress`, whose row was removed, so no reference dangles, and the count `component_stress_modifier_count`, which counts the removed intensified bending rows.

  No kept row, value, other summary member or diagnostic is changed. It is not producer output and not a legacy computation.
- `demo_fixture_generation.json` records the generation: the input, the generator, the recipe, every participating source file and every output, with raw-byte SHA-256 hashes, and the carrier's derivation. It does not hash cargo's stderr, whose warnings carry the checkout's absolute paths, so the record is the same in every checkout.

**The refused demo.** `invented_preview_model.json` stays unchanged as the refused demo for the product's tests. It still carries legacy pressure primitives and the joint. `invented_mechanics_result_preview_physics_1_sparse.json` and `_dense.json` are the product's verbatim refusal envelopes for it, and `preview_physics_fixture_generation.json` is their record.

The bundled results computed with the flawed joint and the legacy pressure (`invented_mechanics_result.json` and the precision-1 pair) were removed by the owner's decision on M07 (G10, D-3).

## Regenerating

From the Piping project root, with a product-manifest-specific `CARGO_TARGET_DIR` and `CARGO_BUILD_JOBS=2`:
- `npm run generate:product-preview-mechanics` regenerates the bundled demo, its carrier and their record;
- `node tools/serialization/generate_product_preview_mechanics.mjs --preview-physics-1` regenerates the refused demo's envelopes and their record.

The recipe runs both solver modes, checks that the source, input and lock identities are unchanged, and replaces only its own mode's files after a successful capture. The demo mode refuses to install anything but a solved result. The recipe rejects redirected input and output paths and keeps recovery backups if a rollback cannot finish. It does not claim multi-file power-loss atomicity.

For direct observation from the repository root, run each mode with the product manifest, the installed matching toolchain and locked dependencies:

```
cargo run --offline --locked -j2 --manifest-path projects/chirality-piping/core/product_physics/Cargo.toml --example preview_result -- sparse_interactive invented_demo_model
cargo run --offline --locked -j2 --manifest-path projects/chirality-piping/core/product_physics/Cargo.toml --example preview_result -- dense_scrutiny invented_demo_model
```

Without a model argument the example runs `invented_preview_model`, and without any argument it runs that model in sparse mode. An unknown or extra argument exits 2.

Capture stdout only after a successful exit and source and input verification, and keep stderr separately as execution evidence. Do not edit generated numeric values, headers or qualification to make consumers pass. A source or code change requires regeneration and validation of the affected consumers. These files are reproducible example and transport data, not independently derived physical oracles or proof of general engineering correctness.
