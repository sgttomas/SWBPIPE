# Actual producer example fixtures

The precision-1 sparse and dense JSON files are verbatim stdout from the selected Rust product on the unchanged invented_preview_model.json input. They preserve the actual producer header, numerical status, diagnostics, limitations and all rows. The historical invented_mechanics_result.json stays unchanged for legacy read/hash/rounding tests. These are reproducible example/transport data, not independently derived physical oracles or proof of general engineering correctness.

The maintained recipe runs both modes, checks unchanged source/input/lock identities, and replaces only the two precision fixtures plus their generation record after successful capture. From the Piping project root, run `npm run generate:product-preview-mechanics` with a product-manifest-specific `CARGO_TARGET_DIR` and `CARGO_BUILD_JOBS=2`. It rejects redirected input/output paths and retains recovery backups if rollback cannot finish. It does not claim multi-file power-loss atomicity.

For direct observation from the repository root, run each mode using the same product manifest and installed matching toolchain with locked dependencies:

```
cargo run --offline --locked -j2 --manifest-path projects/chirality-piping/core/product_physics/Cargo.toml --example preview_result -- sparse_interactive
cargo run --offline --locked -j2 --manifest-path projects/chirality-piping/core/product_physics/Cargo.toml --example preview_result -- dense_scrutiny
```

Capture stdout to the corresponding precision_1_sparse/dense JSON file only after successful exit and source/input verification; preserve stderr separately as execution evidence. Do not edit generated numeric values, headers or qualification to make consumers pass. An actual nonpassing producer status must remain nonpassing. The no-argument example keeps its original sparse default; unknown/extra mode arguments exit2.

precision_fixture_generation.json binds the exact input, generator, participating source files and both outputs with raw-byte SHA256 hashes. It is a generation record, not a new public result schema or an accepted method registry. Browser use is limited to the exact unchanged default model and requested mode; edited models require the native backend. The reader verifies the actual per-case mode rows for completed results and never invents a completed mode proof for a blocked empty output. Source/code changes require regeneration and affected consumer validation.
