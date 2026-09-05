# Browser parity recommendation — design only
Read docs/architecture/adr/ADR-0001_operation_seam_engine_unification.md references, docs/BUILD_AND_RELEASE.md browser contract, operation_applier Cargo.toml, desktop build-wasm-engine.mjs and loadWasmEngine.ts, operationService.ts, and product_physics dependency graph. Current build exports only operation_applier; physics has no Wasm ABI.

Recommend a dedicated thin Wasm JSON wrapper using the identical product_physics::self_weight generator. Suggested new core/loads/self_weight_wasm Cargo crate (cdylib/rlib) depends on product_physics, serde_json and exact wasm-bindgen 0.2.123. Function accepts original model JSON plus request JSON, returns plan or structured diagnostics. No math or typed-model roundtrip mutation. Core primitive operation mutation remains exclusively operation_applier per DEC020; generator merely emits drafts.

Build integration: desktop build-wasm-engine.mjs runs separate wrapper build/glue step with atomic artifact replacement under public/self-weight-engine; N1 separate lazy loader/service reports explicit unavailable and no fixture/TypeScript math fallback. Build script, wrapper crate, generated-artifact ignore/docs/test ownership requires parent extension; original module child has none of these writes.

Pure-Rust product_physics path dependencies have no observed runtime FFI/thread/fs blocker; compilation feasibility and artifact size remain unverified. Thin wrapper does not imply small transitive dependency graph. Native-only readiness may be shown honestly temporarily but is not full browser toolkit parity; browser editing is accepted architecture/test lane and should eventually exercise same generator output + same atomic batch.

No dependency/build/source change for this proposal. Parent decides scope integration owner; existing generator work continues independently.
