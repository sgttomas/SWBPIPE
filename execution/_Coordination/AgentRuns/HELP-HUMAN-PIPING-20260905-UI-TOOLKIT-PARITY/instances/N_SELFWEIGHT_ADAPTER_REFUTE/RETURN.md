# Self-weight adapter v1 design refutation

Verdict: REVISE for SA1 exact request shape. Otherwise the proposed shared thin adapter is compatible with existing hash and generator contracts. Reviewed design hash matches ADAPTER_DESIGN_SNAPSHOT_V1.json.

## SA1 — Typed request parsing currently silently drops unknown keys

SelfWeightRequest and SelfWeightGravity (product_physics/src/self_weight.rs:13–27) derive Deserialize without deny_unknown_fields. The candidate requires object shape and typed parsing but does not specify rejection of unknown request/gravity fields. An input containing a valid gravity.axis alongside an extra contradictory gravity.direction, or an unsupported generation option, would silently lose that field and produce a ready plan. This violates the intended explicit-input/no-hidden-fallback boundary.

Minimum correction: the adapter validates exact key allowlists before typed projection. Request requires only case_id,label,pipe_refs,gravity,provenance,source_model_hash; gravity requires only value,unit,axis. Reject missing, mistyped or unknown fields at either level with REQUEST-SHAPE diagnostic and plan=null. Implement locally in adapter if changing module structs is outside its fence. Test unknown top-level request and nested gravity keys as well as valid explicit values. Do not apply this allowlist to the original model: model unknown fields must remain in the authenticated full original hash even when PreviewModel does not consume them.

## Accepted design properties and remaining implementation checks

- Compute sha256 over shared canonical_json(original Value) before lossy PreviewModel deserialization. Require exact sha256:<64 lowercase hex> equality; never reconstruct original input from typed model. This matches operation-applier backend hashing and the amended module source_model_hash contract.
- Missing/nonstring/malformed/mismatched digest rejects without generation. Changed otherwise-ignored model fields invalidate the old digest. Reordered object keys preserve hash; array order remains significant. Use identical parsed Value inputs for native/Wasm parity. The canonical serializer documents integer precision outside JavaScript safe-integer range; no new normalization/fallback is justified.
- Native generate_plan and the feature-gated browser parser both use one function. The adapter contains no mass/force formulas or solver calls: it invokes the pure module generator, which reuses existing mass/unit helpers. Frame-kernel and sparse-direct crate manifests show ordinary Rust code without native-system dependencies, but an actual wasm-target build remains required evidence for the product_physics transitive graph.
- Parse/model/request failures return blocking structured diagnostics and plan=null, without panics, model echo, submitted request echo, partial plan, applied_model or acceptance. Generator failures preserve diagnostic code/message/affected refs and add remediation. ready is proposal readiness, not mechanics solve or professional/local-session acceptance.
- Source evidence and source_note/provenance remain unchanged from the pure module. PKG16 owns complete model-hash claim metadata, imported author/source envelope, transactional application and checkpoints. Adapter output must not synthesize those approvals or imply provider identity.

## Handoff

Return SA1 to HELP_HUMAN/N5 for explicit shape amendment and focused recheck. No source, test, dependency installation or delegated work performed. This is derivative candidate-design evidence, not source grant, implementation review or lifecycle acceptance. Preserve this record; rerun on amended adapter or relevant hash/module interface changes. Actual inherited model identifier unavailable.
