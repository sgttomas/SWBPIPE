# Accepted Stage C service wire

Bound to C_ACCEPTED_SNAPSHOT_V1.json and fresh RC_RETURN.md PASS. Operation engine/schema tests pass; native compilation passes; the full native suite has one inherited rule-pack notice assertion failure explicitly recorded in C_CHECKS.json. Runtime/browser parity remains a downstream check.

| Capability | Native command / arguments | Wasm export / positional JSON strings |
|---|---|---|
| Batch preview | validate_model_operation_batch / model, batch, claimedModelHash | validate_operation_batch_json(model_json,batch_json_input,claimed_model_hash_json) |
| Batch explicit apply | apply_model_operation_batch / model, batch, claimedModelHash | apply_operation_batch_json(model_json,batch_json_input,claimed_model_hash_json) |
| Numeric display only | convert_display_quantities / input | convert_display_quantities_json(input_json) |
| Selfweight planning only | generate_self_weight_operation_plan / model, request | separate self_weight_wasm artifact: generate_self_weight_plan_json(model_json,request_json) |

Native command arguments use Tauri camelCase mapping for claimed_model_hash. Batch input/output exact shape and controls are frozen in C1_RETURN.md; the operation Wasm wrapper marks application_route local_wasm_engine. Single-operation routes remain the existing APIs and accept the reviewed new operation kinds split_pipe_run and transform_pipe_run with mandatory complete hash evidence; geometry exact wire is C2_BRIEF.md and C2_RETURN.md. Boundary member creates use existing create_support inside one batch; association wire and canonical member constraints are C3_RETURN.md and PRIVATE_BATCH_CONTEXT_API_V2.md.

Display input `{items:[{id,value,from_unit,to_unit,dimension_id}]}` returns item-level converted/unavailable status through the accepted units catalog; see B2_UI_INTERFACE.md. Selfweight exact envelope and strict original-source hash semantics are accepted N5 ADAPTER_DESIGN_V2.md and SELF_WEIGHT_ACCEPTED_SNAPSHOT_V1.json; native calls the same pure generate_plan facade and never applies it.

Batch publication obligation: capture monotonic renderer generation and full original model hash before asynchronous invocation. At actual commit, verify captured generation and recomputed current authoritative hash against initial_model_hash.value, including a final generation check after any awaited hash calculation. Load/new/undo/redo/edit increments generation. Reject stale response without checkpoint. Only a successful explicit apply may publish applied_model and create one session checkpoint. Preview and rollback publish none.

Offline agent intent preserves complete asserted source/author/rationale metadata, requires explicit user apply, and never enables a provider or runtime binding. D58 remains held.
