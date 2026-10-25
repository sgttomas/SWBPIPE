# Bundled demo results and the refused demo's envelopes

**The bundled demo.** `invented_demo_model.json` is the valid demo model behind the browser's bundled reference results. It is the invented utility loop with no legacy pressure primitives (the legacy pressure contract is retired product-wide), no expansion joint component:C-150 (its user-stiffness element is refused) and no tee component:C-120 (its node N-120 joins only two pipes, so the tee was malformed: `BRANCH_TOPOLOGY_INVALID`). It is derived from `core/product_physics/tests/fixtures/preview_physics_invented_model.json`; only its project id, name and description, its load-case labels (which named the removed pressure), its diagnostic's source path and the removed C-120 differ. Its pipes, nodes and supports are the source's. It is also the app's default session model.
- `invented_demo_result_preview_physics_1_sparse.json` and `_dense.json` are verbatim stdout from the product on that model, one per solver mode: solved, preview-physics-2 (the file names keep their earlier identity token). The browser loads them only through its explicit reference inspection, never as a solve for the current model. The Python preview service (`core/product_preview/service.py`) returns the sparse one as its frozen mechanics result. No historical-format (0.1.0) carrier is kept: that mechanics-result version is retired and refused by name (b7-plan E2).
- `demo_fixture_generation.json` records the generation: the input, the generator, the recipe, every participating source file and every output, with raw-byte SHA-256 hashes. It does not hash cargo's stderr, whose warnings carry the checkout's absolute paths, so the record is the same in every checkout.

**The refused demo.** `invented_preview_model.json` stays unchanged as the refused demo for the product's tests. It still carries legacy pressure primitives and the joint. `invented_mechanics_result_preview_physics_1_sparse.json` and `_dense.json` are the product's verbatim refusal envelopes for it, and `preview_physics_fixture_generation.json` is their record.

**The milestone model for the desktop app.** `rf_skew_t_cant_off_122_r1e-04.desktop_model.json` is the `model` of `rf_skew_t_cant_off_122_r1e-04.request.json` (the F2a milestone) with only the display fields the desktop shell needs added: `data_boundary` (from the bundled demo), a project name and description, labels equal to ids on materials, nodes, pipe segments and supports, empty `components` and `diagnostics`, and `status: "preview_only"` on its load case. Ids, values and `schema_version` are unchanged. `core/product_physics/tests/g10_desktop_milestone_model.rs` checks that difference and that the desktop's solve request publishes the milestone's protected ordinary bytes in both modes; `apps/desktop/src/services/g10DesktopMilestoneModel.test.ts` opens it through the model index and the input manifest.

The bundled results computed with the flawed joint and the legacy pressure (`invented_mechanics_result.json` and the precision-1 pair) were removed by the owner's decision on M07 (G10, D-3).

## Regenerating

From the Piping project root, with a product-manifest-specific `CARGO_TARGET_DIR` and `CARGO_BUILD_JOBS=2`:
- `npm run generate:product-preview-mechanics` regenerates the bundled demo and its record;
- `node tools/serialization/generate_product_preview_mechanics.mjs --preview-physics-1` regenerates the refused demo's envelopes and their record.

The recipe runs both solver modes, checks that the source, input and lock identities are unchanged, and replaces only its own mode's files after a successful capture. The demo mode refuses to install anything but a solved result. The recipe rejects redirected input and output paths and keeps recovery backups if a rollback cannot finish. It does not claim multi-file power-loss atomicity.

For direct observation from the repository root, run each mode with the product manifest, the installed matching toolchain and locked dependencies:

```
cargo run --offline --locked -j2 --manifest-path projects/chirality-piping/core/product_physics/Cargo.toml --example preview_result -- sparse_interactive invented_demo_model
cargo run --offline --locked -j2 --manifest-path projects/chirality-piping/core/product_physics/Cargo.toml --example preview_result -- dense_scrutiny invented_demo_model
```

Without a model argument the example runs `invented_preview_model`, and without any argument it runs that model in sparse mode. An unknown or extra argument exits 2.

Capture stdout only after a successful exit and source and input verification, and keep stderr separately as execution evidence. Do not edit generated numeric values, headers or qualification to make consumers pass. A source or code change requires regeneration and validation of the affected consumers. These files are reproducible example and transport data, not independently derived physical oracles or proof of general engineering correctness.
