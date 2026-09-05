# Tier3 consumer preparation — no source release

Basis parent accepted design N2_WI_PKG16/B4_BATCH_DESIGN_V3.md; source review pending. This derivative preparation does not release Tier3 UI before M_EXISTING accepted.

Common batch service: native validate_model_operation_batch/apply_model_operation_batch; Wasm validate_operation_batch_json/apply_operation_batch_json. Input {batch_id,operations:[EditorOperationIntent,...]} + initial canonical model hash. Nonempty/unique IDs, strict proposed source/audit/professional controls. Offline agent metadata retained; explicit local user apply only, D58 live/provider integration held.

Outcome top level document_kind openpipestress.desktop.operation_batch_outcome; batch ID/mode/route/input_model_unchanged/validation/application/diagnostics; initial_model_hash complete evidence and initial_backend_hash; batch_hash; submitted_operations exact metadata; operation_outcomes are allowlist simulation projections with no intermediate model/hash/acceptance. Only fully successful apply returns final applied_model/hash and one local session acceptance. Validation/failure never returns model or acceptance; preceding hypothetical diffs labelled simulated/rolled back.

Renderer captures model generation and authoritative initial hash before invocation. After response, compare unchanged generation and recomputed CURRENT model hash to request/response initial basis, recheck generation following hash await. Load/new/undo/redo/edit invalidates. Stale response no receipt/checkpoint/queue mutation/model replacement. Successful batch exactly one checkpoint. B0 v2A applying equivalent guard to existing single apply.

Future consumer modules should return operation batches only; App integration owns one guarded execution seam. UI preview presents ordered hypothetical steps and entire-batch failure, never serial commits or false partial acceptance. Self-weight generator, manual imported hanger and transform forms await exact reviewed producers. Display batch API separate presentation-only path; no model edits/operation checkpoint.

## Producer relay preparation

Hanger import: LibraryKind hanger, metadata hanger_library, hanger_records array; record hanger_id/name/hanger/provenance; imported quantities {magnitude,unit,dimension,provenance}, explicit translational DOF and force/length/force_per_length dimensions. Selection converts shape only to product {value,unit}, preserving complete canonical selection provenance in atomic Support.configuration. Manual record choice; no sizing or defaults. N4 V2 exact schema/source pending.

Self-weight request {case_id,label,pipe_refs,gravity:{value,unit,axis},provenance,source_model_hash}; return plan drafts/source_evidence, selected_pipe_mass_only. Apply draft through batch on original document after preview, never reserialize typed physics model to working model. N2 wrapper exact contract awaited.

## Parent fence amendment — self-weight browser adapter (future only)

Parent accepted separate thin self-weight generator Wasm adapter sharing existing mass math, no fallback. Future source fence expands narrowly to apps/desktop/scripts/build-wasm-engine.mjs, its relevant script tests and exact generated-artifact .gitignore entries for public/self-weight-engine alongside operation Wasm. No edits authorized until M_EXISTING accepted and exact N5 crate/export/envelope frozen. Loader/service stays desktop src; native/browser same pure adapter validation, differing transport only; generator never mutates, application only through operation batch. Parent controls rebuild windows.

Parent fence clarification: only project-local .gitignore exact public/self-weight-engine and temporary/old generated output entries; root .gitignore excluded. Preserve operation artifact behavior and existing resolveWasmArtifact exported/tested interface when parameterizing crate filename.

## Self-weight adapter V2 exact interface

Parent accepted source design instances/N5_WI_PKG05/ADAPTER_DESIGN_V2.md. Crate open_pipe_stress_self_weight_wasm with wasm feature; export generate_self_weight_plan_json(model_json,request_json)->String. Native facade invokes same shared generate_plan Value. Envelope document_kind openpipestress.desktop.self_weight_plan_outcome, status ready|blocked, source_model_hash sha256:<hex>, plan|null, diagnostics, input_model_unchanged true, mutation_route structured_operations_only, requires_user_acceptance true. Build/loader fence conditionally released when crate compile-ready; UI still after M_EXISTING. No fixture fallback.
