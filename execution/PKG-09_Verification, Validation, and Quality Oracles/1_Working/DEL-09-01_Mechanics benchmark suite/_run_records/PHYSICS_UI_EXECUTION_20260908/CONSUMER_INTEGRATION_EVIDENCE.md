# Exact fixture-to-adapter execution evidence

Status: POST-FREEZE BOUNDED CONSUMER EVIDENCE; NO DEPENDENCY DISPOSITION.

This packet is separate from oracle authoring. `FREEZE_MANIFEST.json` binds the independent method and expected-result bytes before this production route was run. Neither the harness nor the production result files supplied an expected number.

## Exact runtime route exercised

The harness reads `fixtures/product_preview/invented_preview_model.json` at SHA-256 `986c055944776ca8d0d849d552678e1f2bfb6c4559bbb9ff8a1928157a4f871c`, applies only the transformations in `VARIANT_LEDGER.json`, deserializes the result as the public `PreviewModel`, and calls `run_linear_static_preview_with_mode` once in `DenseScrutiny` and once in `SparseInteractive` for each variant. Its source is `production_comparison_harness/src/main.rs`; all eight returned envelopes are in `production_outputs/`.

The called adapter route is concrete in production source:

- `core/product_physics/src/lib.rs:100` declares the input `PreviewModel` fields consumed from the fixture.
- `core/product_physics/src/lib.rs:826` is the public dense/sparse entry point; lines 838–876 validate, normalize units, and build the mechanics model.
- `core/product_physics/src/lib.rs:3279` builds nodes and the four straight-pipe frame elements; lines 3386–3391 insert each unchanged straight span into the frame.
- `core/product_physics/src/lib.rs:3397` builds C-150 user-stiffness elements; lines 3538–3632 show the current adapter mapping from the expansion-joint marker and P-130 endpoints into one relative user-stiffness element.
- `core/product_physics/src/lib.rs:3407` builds nonlinear supports; lines 3445–3462 map SH-140's UZ stiffness into a ground spring.
- `core/product_physics/src/lib.rs:1204` maps the selected L-100 primitives; lines 1257–1284 assemble the uniform element and thermal equivalent loads after the pressure primitives have been removed.
- `core/product_physics/src/lib.rs:1331` selects the nonlinear solve result; lines 1852–1921 pass frame elements, C-150 user stiffness when present, the one-way and friction supports, current-normal source, and SH-140 spring into the active-set solve.
- `core/product_physics/src/lib.rs:1993` emits derived friction-normal evidence; lines 2002 onward emit final nonlinear state, signed displacement, and reaction results.

The returned envelopes bind the selected `load:L-100` in each compared result. They contain the expected entity identities, including N-100 through N-140 displacement rows, P-100 through P-130 end and station rows, NL-140 UY state/displacement/reaction, NL-130-FRIC UZ state/displacement/reaction and current-normal reaction, and SH-140 support reaction. The literal-adapter outputs additionally emit C-150 axial/lateral/angular/torsional input rows. Every envelope reports `MECHANICS_SOLVED`; dense and sparse comparison values are identical at the product's emitted precision.

## What this evidence can and cannot support

This is exact consumer-route evidence for the transformed L-100 variants at the source SHA. It is suitable input to a later owner-run, edge-specific dependency revalidation. It does not itself alter or satisfy a dependency row.

For `DAG-002-E0532` (DEL-04-01 frame stiffness/coordinate transformation) and `DAG-002-E0533` (DEL-04-02 straight pipe element), an owning revalidation would need to combine accepted producer identity/maturity with this exact consumer route, the frozen independent expected vectors, the comparison results, and the retained D01/D02 limitations. R23 established producer maturity only; it did not establish this consumer integration. M9 does not override the R23 disposition or authorize a DAG pointer change.

For `TP-DAG-004-DEL-09-01-E001` (DEL-02-02 unit contract), this run proves only ordinary explicit-SI fixture calculations: metres, newtons, newton-metres, radians and pascals are declared in `INPUT_FREEZE.json`, and the product emits millimetres/newtons/newton-metres/radians for the compared results. That evidence is deliberately isolated from canonical conversion acceptance. The separate human disposition `PKG09-0901-PKG02-001` and accepted canonical unit/conversion basis remain required before the held unit edge can change.

No comparison threshold, engineering acceptance, C-150 physical adequacy, spring preload basis, or friction-history basis follows from successful execution.
