# Self-weight adapter V2 recheck

Verdict: PASS_DESIGN_ONLY. SA1 is closed at candidate contract level; no residual design blocker found.

V2 explicitly restricts request keys to case_id,label,pipe_refs,gravity,provenance,source_model_hash and gravity keys to value,unit,axis. Both module Deserialize structs must use deny_unknown_fields, so the native/browser shared adapter and direct module JSON parsing cannot silently discard unknown generation options. Unknown original model fields remain intentionally retained in the raw Value and included in the canonical hash before PreviewModel projection. Required regressions cover both request nesting levels and ignored-model-field stale-hash detection.

Full-original canonical hash verification, exact sha256-prefixed lowercase digest, identical native/Wasm generate_plan logic, no mass-math duplication and no acceptance/mutation outputs remain unchanged from V1. The frozen snapshot digest matches the reviewed V2 file.

This does not claim that assigned struct repairs, adapter implementation, Wasm build or parity tests have completed. Parent controls source release; fresh full source review and required integration checks remain necessary. No source edits/tests/delegation; inherited exact model identifier unavailable. Prior V1 records remain immutable. Rerun on design/module/hash-interface change or implementation finding.
