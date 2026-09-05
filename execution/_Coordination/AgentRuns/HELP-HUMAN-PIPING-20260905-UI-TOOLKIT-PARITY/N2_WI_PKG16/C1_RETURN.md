# C1 atomic batch return

RUN_STATUS: SUCCESS — bounded implementation complete; governed project closure not asserted.
Parent: WORKING_ITEMS N2_WI_PKG16; role ephemeral Agent 2, native instruction-asserted, no delegation.
Scope: PKG-16 DEL-16-01/02/03; C1_BRIEF.md. All B3 accepted source hashes and B4_BATCH_DESIGN_V3.md accepted hash verified before edits; R3_RETURN.md and RECHECK_V3.md PASS read. Private boundary API V2 unchanged.
Method: software-bounded-implementation v1 under explicit read/write/bash/test authorization. Portable path anchors are {REPO_ROOT}, {WORKING_ROOT}, {INSTANCE}; no source authority expansion.
WriteAuthorization: only core/model_operations/operation_applier/src/atomic_batch.rs and C1_RETURN.md/C1_CHECKS.json. No lib/schema/Cargo/native edits. Parent supplies integration; C3 supplies boundary token.
ToolsUsed: bash targeted reads/search; Python module/evidence writes; rustfmt only owned module; offline Cargo crate tests after parent integration release.
ToolPolicyCompliance: PASS under explicit brief; no network/delegation/commits/receipt writes.

## Public functions and private dependencies

`pub fn validate_operation_batch(model:&Value,batch:&Value,claimed:Option<&Value>)->Value`
`pub fn apply_operation_batch(model:&Value,batch:&Value,claimed:Option<&Value>)->Value`

Required parent hooks: `crate::apply_operation_in_batch(&current,&intent,Some(&fresh_claim),&context)->OperationOutcome`, crate-private `boundary_association::preflight_batch(base,batch)->Result<ValidatedBoundaryBatch,BoundaryError>`, existing Checker/model_basis_evidence/check_claimed_model_hash/canonical_json/sha256_hex/constants. The private context is never caller input or serialized.

## Exact input and preflight

Batch has exactly `batch_id` nonempty string and `operations` nonempty array. Each entry has required EditorOperationIntent fields; optional queue_id/source only. Duplicate operation_id or change_id rejects entire batch before simulation. Type/status/flags and complete source shape are checked, including proposed status, author user|agent, all review/nonprofessional controls, direct mutation false and requires_user_acceptance true. Agent rationale must be nonempty and source_ref/source_channel/source_role mandatory nonempty strings. User source optional but complete when present. Source permits no undeclared keys. Submitted validation labels are strings, retained as untrusted input, never promoted into backend evidence.

Initial hash is mandatory; four existing required algorithm/canonicalization/payload_scope/value fields use existing authoritative checker. Optional payload_ref/hash_status are validated; unsupported/unknown fields reject. Null/missing/stale/malformed initial claims produce no replay. Initial request model hash is separate from final hash and incoming claim is echoed only under submitted_initial_model_hash.

## Exact output shape

- schema_version; document_kind `openpipestress.desktop.operation_batch_outcome`; batch_id; mode `validate_only|apply`; application_route `structured_operation_batch`; input_model_unchanged true.
- validation: schema_validation `passed|blocked`, batch_validation_status `passed|blocked`, diff_preview_status `generated|blocked_by_validation`, application_status `applied_to_session_model|blocked|not_applied`.
- simulation_disposition `rolled_back_no_model_published|committed_as_one_batch|validation_only_discarded`.
- diagnostics array and operation_outcomes array.
- initial_model_hash with algorithm sha256, canonicalization rfc8785_jcs, payload_scope model_payload, payload_ref model:local_batch_input, hash_status computed_local_preview, value prefixed SHA256 of ORIGINAL request model. initial_backend_hash and input_backend_hash equal that value. These are computed evidence, not accepted authority.
- batch_hash prefixed SHA256 of canonical submitted batch; submitted_operations preserves exact Value; submitted_operations_trust `untrusted_submitted_metadata_not_validation_evidence`; submitted_initial_model_hash echoes untrusted claim or null.
- applied_model, applied_model_backend_hash and acceptance are null unless every operation succeeded AND explicit apply API was used. Successful acceptance contains acceptance_basis user_initiated_apply_in_local_session, acceptance_is_professional_approval false, persistence_status session_state_only_not_yet_saved.
- top professional_boundary retains human review and all false software professional claims. audit_boundary records structured-only route, direct false, user acceptance required, input unchanged, whether new model returned, held_D58 runtime binding and not_performed_asserted_metadata_only source identity verification.

Every step is an explicit allowlist: index, operation_id, change_id, target_object_type, target_ref, change_kind; validation {schema_validation,reference_validation,unit_validation,before_state_validation,diff_preview_status}; diff_preview; diagnostics; simulation_status `validated_on_temporary_state|blocked`. No nested mode/application status/acceptance/model/model hash/model basis/professional or audit boundary is copied. Previous successful simulation rows remain labelled temporary even if a later failure causes rollback. No later operation runs after the first failed simulation.

## Verification

Final offline integrated Cargo run passed **132 unit +1 native contract +2 canonical hash tests**. Nine C1 tests cover dependent case/load creation; deterministic output/hash; validation with no model/acceptance; late reference rollback and replay stop; late stale before; missing/null/incomplete/malformed/stale hash; duplicate identities/empty batch/unknown context field; user-agent identical model and simulation results with exact attributed metadata retention; every mandatory control/source/rationale rejection; untrusted submitted validation labels cannot override backend failure. Module formatting completed. C1_CHECKS.json binds source hash and command template. Parent and C3 tests were concurrently integrated, explaining total suite growth from initial 126 to final 132.

## Handoff and remaining work

Module source released to parent for freeze/review. Implementation is derivative evidence under accepted B4 V3, not new decomposition authority. Parent owns fresh whole-diff review, native/Wasm/UI integration and wider registered checks. Renderer generation/current-hash publication guard remains required by V3; core cannot inspect later UI state and this return does not claim that guard tested. Successful UI publication must create one checkpoint only. D58 remains held; source attribution is unverified metadata and no provider or engineering approval is added.
MISSING: none for bounded C1 module.
NEEDS_HUMAN_RULING: none.
DEPENDENCY_NOTES: private C3 context and shared parent hooks integrated; parent must freeze exact combined source before independent review; no circular authorization introduced.
