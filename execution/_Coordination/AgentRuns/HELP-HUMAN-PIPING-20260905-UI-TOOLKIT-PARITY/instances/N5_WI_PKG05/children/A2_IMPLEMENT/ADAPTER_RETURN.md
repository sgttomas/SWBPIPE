# Adapter Agent2 return — ready for fresh combined review

Parent WORKING_ITEMS N5_WI_PKG05; PKG-05 DEL-05-01 SOW-013 OBJ-003. Execution permitted by parent after independent ADAPTER_DESIGN_V2 PASS_DESIGN_ONLY and source release. Sealed scope ADAPTER_LAUNCH_BRIEF.md. Agent2 ephemeral no delegation; runtime model attribution unavailable. Existing pure module and lib.rs remained read-only.

Implemented shared native `generate_plan(&serde_json::Value, &serde_json::Value) -> serde_json::Value` and feature wasm export `generate_self_weight_plan_json(&str, &str) -> String`. The JSON transport is available natively for shared parser tests. Full original model canonical SHA256 is authenticated before PreviewModel projection. Only hash identity, shape/strict option parsing, outcome envelope and diagnostic projection occur here; production mass math stays in existing module. Ready contains full serialized plan/evidence. Every blocked outcome has null plan; unauthenticated or shape-invalid model gives null response source hash. Input document and extensions are not reconstructed or returned. No receipt/model application.

Validation commands:
- `cargo test --offline --manifest-path projects/chirality-piping/core/loads/self_weight_wasm/Cargo.toml`: PASS 4 tests, 0 failures; doc tests 0.
- `cargo build --offline --manifest-path projects/chirality-piping/core/loads/self_weight_wasm/Cargo.toml --target wasm32-unknown-unknown --features wasm --release`: PASS.

Tests cover direct module plan parity; native JSON transport parity; original model and request immutability; stale hash when unknown model field changes; refreshed hash accepting retained extension; malformed/missing/wrong hash; malformed native/model/request and JSON inputs; strict unknown request and gravity fields; module diagnostic preservation; reordered key canonical identity. Every error checks null plan and absence of applied model/receipt/source model echo. Cached dependency resolution recorded in Cargo.lock; wasm-bindgen exactly 0.2.123, sha2 0.10.9, serde_json 1.0.151. No network or installation used.

Frozen source hashes:
- `core/loads/self_weight_wasm/Cargo.toml`: `7e758b467ffed0a16629f5666f6dd06810c9b96e8e838a2ae4a080c0d4fa4850`
- `core/loads/self_weight_wasm/Cargo.lock`: `e1254a5095d363697fe687a9eaf7327999ad11f86355f1864fb94b13c6dc5597`
- `core/loads/self_weight_wasm/src/lib.rs`: `61512d3c9d6a0343977a81fd3d8f93118d135ae1a25d204683a4fe9d4b93f8c8`

Scope: only the three allowed crate files plus own ADAPTER_RETURN.md written; cargo-generated ignored target artifacts are validation outputs. No root/native/UI/build script/shared-source modifications. Source ready for combined fresh independent review; parent owns acceptance snapshot and complete broader checks. N1 still owns Wasm glue/browser consumer integration; N2 owns native facade and batch metadata. Actual consumer native/Wasm comparison and atomic apply/rollback/checkpoint tests remain integration obligations. No lifecycle/decomposition closure asserted. Rerun tests/build/review on source changes. This is derivative evidence against accepted parent design and original SCA009/DAG010 basis.
